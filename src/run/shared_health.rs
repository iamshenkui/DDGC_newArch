//! Shared health pool tracking for multi-body boss encounters.
//!
//! Some DDGC bosses have multiple actor bodies that share a single health pool.
//! When any linked body takes damage, the shared pool decreases, and when the
//! pool reaches zero, all linked bodies die simultaneously.
//!
//! Supported boss families:
//! - Azure Dragon: 3 balls share one pooled HP (main body + thunder ball + wind ball)
//! - Vermilion Bird: main body + 2 tails share one HP pool
//!
//! Implementation model:
//! - Pool HP is stored as the TOTAL (sum of all member HP at creation)
//! - Pre-damage: all members at their individual HP values, pool = sum
//! - Framework applies damage to individual members
//! - Post-damage: we compute actual damage = pre_sum - post_sum
//! - Pool HP decreases by actual damage
//! - Each member's HP is then re-synced to their proportional share of the pool

use std::collections::HashMap;

use framework_rules::actor::{ActorAggregate, ActorId};
use framework_rules::attributes::{AttributeKey, AttributeValue, ATTR_HEALTH};

/// A shared health pool — multiple actors draw from the same HP pool.
#[derive(Debug, Clone)]
pub struct SharedHealthPool {
    /// Unique identifier for this pool.
    pub id: u64,
    /// Actor IDs that share this pool.
    pub member_ids: Vec<ActorId>,
    /// Current pool HP (sum of all member HP).
    current_hp: f64,
    /// Per-member max HP values for proportional redistribution.
    member_max_hps: HashMap<ActorId, f64>,
}

impl SharedHealthPool {
    /// Create a new shared health pool from a list of member actors.
    ///
    /// Pool HP is initialized as the sum of all member HP values.
    fn new(id: u64, members: Vec<ActorId>, actors: &HashMap<ActorId, ActorAggregate>) -> Self {
        let current_hp = members
            .iter()
            .filter_map(|mid| actors.get(mid))
            .map(|a| a.effective_attribute(&AttributeKey::new(ATTR_HEALTH)).0)
            .sum();

        // Store per-member max HP for proportional redistribution
        let mut member_max_hps = HashMap::new();
        for &mid in &members {
            if let Some(actor) = actors.get(&mid) {
                let max_hp = actor.effective_attribute(&AttributeKey::new(ATTR_HEALTH)).0;
                member_max_hps.insert(mid, max_hp);
            }
        }

        SharedHealthPool {
            id,
            member_ids: members,
            current_hp,
            member_max_hps,
        }
    }

    /// Current pool HP (sum of all member HP).
    #[allow(dead_code)]
    fn current_hp_fn(&self) -> f64 {
        self.current_hp
    }

    /// Apply damage to the pool. Returns the actual damage applied (capped at current HP).
    fn apply_damage(&mut self, damage: f64) -> f64 {
        let actual = damage.min(self.current_hp);
        self.current_hp = (self.current_hp - actual).max(0.0);
        actual
    }

    /// Check if the pool is exhausted.
    fn is_exhausted(&self) -> bool {
        self.current_hp <= 0.0
    }

    /// Redistribute pool HP evenly across members based on their max HP proportions.
    ///
    /// Each member's HP becomes: pool_hp * (member_max / total_max)
    /// For equal-max members, this is pool_hp / member_count.
    fn redistribute_to_members(&self, actors: &mut HashMap<ActorId, ActorAggregate>) {
        let total_max: f64 = self.member_max_hps.values().sum();

        for &mid in &self.member_ids {
            if let Some(actor) = actors.get_mut(&mid) {
                let member_max = self.member_max_hps.get(&mid).copied().unwrap_or(0.0);
                let share = if total_max > 0.0 {
                    self.current_hp * (member_max / total_max)
                } else {
                    0.0
                };
                actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(share));
            }
        }
    }
}

/// Tracks all shared health pools in an encounter.
#[derive(Debug, Default)]
pub struct SharedHealthTracker {
    pools: HashMap<u64, SharedHealthPool>,
    actor_to_pool: HashMap<ActorId, u64>,
    pre_damage_snapshots: HashMap<ActorId, f64>,
}

impl SharedHealthTracker {
    /// Create a new empty tracker.
    pub fn new() -> Self {
        SharedHealthTracker::default()
    }

    /// Register a new shared health pool.
    pub fn register_pool(
        &mut self,
        pool_id: u64,
        member_ids: Vec<ActorId>,
        actors: &HashMap<ActorId, ActorAggregate>,
    ) {
        let pool = SharedHealthPool::new(pool_id, member_ids.clone(), actors);
        for mid in &member_ids {
            self.actor_to_pool.insert(*mid, pool_id);
        }
        self.pools.insert(pool_id, pool);
    }

    /// Check if an actor is part of a shared health pool.
    pub fn is_pool_member(&self, actor_id: ActorId) -> bool {
        self.actor_to_pool.contains_key(&actor_id)
    }

    /// Get the pool ID for an actor, if any.
    pub fn pool_id_for(&self, actor_id: ActorId) -> Option<u64> {
        self.actor_to_pool.get(&actor_id).copied()
    }

    /// Snapshot the current HP of all pool members before damage is applied.
    pub fn snapshot_pre_damage(&mut self, actors: &HashMap<ActorId, ActorAggregate>) {
        self.pre_damage_snapshots.clear();
        for (&actor_id, actor) in actors.iter() {
            if self.actor_to_pool.contains_key(&actor_id) {
                let hp = actor.effective_attribute(&AttributeKey::new(ATTR_HEALTH)).0;
                self.pre_damage_snapshots.insert(actor_id, hp);
            }
        }
    }

    /// Process damage for all pool members after the framework has applied damage.
    ///
    /// Returns the set of pool IDs that are now exhausted.
    pub fn process_post_damage(
        &mut self,
        actors: &mut HashMap<ActorId, ActorAggregate>,
    ) -> Vec<u64> {
        let mut exhausted_pools = Vec::new();

        for (pool_id, pool) in self.pools.iter_mut() {
            // Compute total damage dealt to this pool by summing member HP changes
            let mut total_damage = 0.0;
            for &mid in &pool.member_ids {
                if let Some(actor) = actors.get(&mid) {
                    let current_hp = actor.effective_attribute(&AttributeKey::new(ATTR_HEALTH)).0;
                    if let Some(&pre_hp) = self.pre_damage_snapshots.get(&mid) {
                        let damage = (pre_hp - current_hp).max(0.0);
                        total_damage += damage;
                    }
                }
            }

            // Apply damage to pool HP
            if total_damage > 0.0 {
                pool.apply_damage(total_damage);
                pool.redistribute_to_members(actors);
            }

            if pool.is_exhausted() {
                exhausted_pools.push(*pool_id);
            }
        }

        self.pre_damage_snapshots.clear();
        exhausted_pools
    }

    /// Get all actor IDs that belong to an exhausted pool.
    #[allow(dead_code)]
    pub fn members_of_exhausted_pool(&self, pool_id: u64) -> Vec<ActorId> {
        self.pools
            .get(&pool_id)
            .map(|p| p.member_ids.clone())
            .unwrap_or_default()
    }

    /// Number of registered pools.
    #[allow(dead_code)]
    pub fn pool_count(&self) -> usize {
        self.pools.len()
    }

    /// Get pool HP for testing.
    #[cfg(test)]
    pub fn get_pool_hp(&self, pool_id: u64) -> Option<f64> {
        self.pools.get(&pool_id).map(|p| p.current_hp_fn())
    }
}

// ── Boss shared health pool definitions ─────────────────────────────────────

/// Shared health pool definitions for boss encounters.
/// Format: (pool_id, member_family_ids...)
const SHARED_HEALTH_POOLS: &[(u64, &[&str])] = &[
    // Azure Dragon: main body + thunder ball + wind ball share one HP pool
    (1, &["azure_dragon", "azure_dragon_ball_thunder", "azure_dragon_ball_wind"]),
    // Vermilion Bird: main body + tail A + tail B share one HP pool
    (2, &["vermilion_bird", "vermilion_bird_tail_A", "vermilion_bird_tail_B"]),
];

/// Initialize shared health pools for a boss encounter based on pack composition.
///
/// The `actor_families` parameter maps ActorIds to their family IDs, built during
/// actor creation in the encounter loop.
pub fn init_shared_health_for_pack(
    _pack_id: &str,
    actors: &HashMap<ActorId, ActorAggregate>,
    actor_families: &HashMap<ActorId, String>,
) -> SharedHealthTracker {
    let mut tracker = SharedHealthTracker::new();

    // For each shared health pool definition, check if all members are present
    for &(pool_id, member_families) in SHARED_HEALTH_POOLS {
        // Match actors to pool members using family_id as a prefix of the archetype name
        // e.g., "azure_dragon" matches archetype "Azure Dragon" and "Azure Dragon Ball Thunder"
        let all_members: Vec<ActorId> = actor_families
            .iter()
            .filter(|(_, family)| {
                member_families.iter().any(|mf| {
                    // Normalize both for comparison (remove underscores, spaces, lowercase)
                    let normalized_mf = mf.replace('_', " ").replace(' ', "").to_lowercase();
                    let normalized_family = family.replace('_', " ").replace(' ', "").to_lowercase();
                    // Check if family starts with the member family prefix
                    normalized_family.starts_with(&normalized_mf)
                        || normalized_mf.starts_with(&normalized_family)
                        || normalized_family == normalized_mf
                })
            })
            .map(|(&actor_id, _)| actor_id)
            .collect();

        if all_members.len() == member_families.len() {
            tracker.register_pool(pool_id, all_members, actors);
        }
    }

    tracker
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a mock ActorAggregate for testing
    fn mock_actor(actor_id: ActorId, hp: f64) -> ActorAggregate {
        use crate::content::actors::Archetype;
        use framework_combat::encounter::CombatSide;
        let arch = Archetype {
            name: crate::content::actors::ArchetypeName::new("TestActor"),
            side: CombatSide::Enemy,
            health: hp,
            max_health: hp,
            attack: 10.0,
            defense: 0.0,
            speed: 5.0,
            stress: 0.0,
            max_stress: 200.0,
            crit_chance: 0.05,
            accuracy: 0.95,
            dodge: 0.0,
        };
        arch.create_actor(actor_id)
    }

    #[test]
    fn shared_health_pool_creation() {
        let id1 = ActorId(10);
        let id2 = ActorId(11);
        let id3 = ActorId(12);

        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        actors.insert(id1, mock_actor(id1, 100.0));
        actors.insert(id2, mock_actor(id2, 100.0));
        actors.insert(id3, mock_actor(id3, 100.0));

        let mut tracker = SharedHealthTracker::new();
        tracker.register_pool(1, vec![id1, id2, id3], &actors);

        assert_eq!(tracker.pool_count(), 1);
        assert!(tracker.is_pool_member(id1));
        assert!(tracker.is_pool_member(id2));
        assert!(tracker.is_pool_member(id3));
        assert_eq!(tracker.pool_id_for(id1), Some(1));
    }

    #[test]
    fn shared_health_damage_sharing() {
        let id1 = ActorId(10);
        let id2 = ActorId(11);
        let id3 = ActorId(12);

        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        // All equal max HP = 100 each, total = 300
        actors.insert(id1, mock_actor(id1, 100.0));
        actors.insert(id2, mock_actor(id2, 100.0));
        actors.insert(id3, mock_actor(id3, 100.0));

        let mut tracker = SharedHealthTracker::new();
        tracker.register_pool(1, vec![id1, id2, id3], &actors);

        // Initial pool HP = 300 (sum of 100+100+100)
        assert_eq!(tracker.get_pool_hp(1), Some(300.0));

        // Snapshot before damage
        tracker.snapshot_pre_damage(&actors);

        // Framework applies 60 damage to id1 (100 -> 40)
        if let Some(actor) = actors.get_mut(&id1) {
            actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(40.0));
        }

        // Process post-damage
        let exhausted = tracker.process_post_damage(&mut actors);

        // Pool should not be exhausted
        assert!(exhausted.is_empty());

        // Pool HP should now be 240 (300 - 60 damage)
        assert_eq!(tracker.get_pool_hp(1), Some(240.0));

        // All members should have HP = 240 * (100/300) = 80 each
        assert_eq!(
            actors[&id1].effective_attribute(&AttributeKey::new(ATTR_HEALTH)).0,
            80.0
        );
        assert_eq!(
            actors[&id2].effective_attribute(&AttributeKey::new(ATTR_HEALTH)).0,
            80.0
        );
        assert_eq!(
            actors[&id3].effective_attribute(&AttributeKey::new(ATTR_HEALTH)).0,
            80.0
        );
    }

    #[test]
    fn shared_health_pool_not_exhausted_when_partial_damage() {
        // Test that pool is NOT exhausted when only partial damage is dealt
        let id1 = ActorId(10);
        let id2 = ActorId(11);

        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        actors.insert(id1, mock_actor(id1, 100.0));
        actors.insert(id2, mock_actor(id2, 100.0));

        let mut tracker = SharedHealthTracker::new();
        tracker.register_pool(1, vec![id1, id2], &actors);

        // Initial pool HP = 200
        assert_eq!(tracker.get_pool_hp(1), Some(200.0));

        tracker.snapshot_pre_damage(&actors);

        // Framework applies 100 damage to id1 (100 -> 0)
        if let Some(actor) = actors.get_mut(&id1) {
            actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(0.0));
        }

        let exhausted = tracker.process_post_damage(&mut actors);

        // Pool should NOT be exhausted (100 damage < 200 pool HP)
        assert!(exhausted.is_empty());
        assert_eq!(tracker.get_pool_hp(1), Some(100.0));
    }

    #[test]
    fn shared_health_unequal_max_members() {
        // Test with members having different max HP
        let id1 = ActorId(10); // max = 150
        let id2 = ActorId(11); // max = 100
        let id3 = ActorId(12); // max = 50
        // Total max = 300

        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        actors.insert(id1, mock_actor(id1, 150.0)); // 50% of pool
        actors.insert(id2, mock_actor(id2, 100.0)); // 33% of pool
        actors.insert(id3, mock_actor(id3, 50.0)); // 17% of pool

        let mut tracker = SharedHealthTracker::new();
        tracker.register_pool(1, vec![id1, id2, id3], &actors);

        // Initial pool HP = 150 + 100 + 50 = 300
        assert_eq!(tracker.get_pool_hp(1), Some(300.0));

        tracker.snapshot_pre_damage(&actors);

        // Framework applies 60 damage to id1 (150 -> 90)
        if let Some(actor) = actors.get_mut(&id1) {
            actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(90.0));
        }

        tracker.process_post_damage(&mut actors);

        // Pool HP = 300 - 60 = 240
        assert_eq!(tracker.get_pool_hp(1), Some(240.0));

        // id1 share: 240 * (150/300) = 120
        assert_eq!(
            actors[&id1].effective_attribute(&AttributeKey::new(ATTR_HEALTH)).0,
            120.0
        );
        // id2 share: 240 * (100/300) = 80
        assert_eq!(
            actors[&id2].effective_attribute(&AttributeKey::new(ATTR_HEALTH)).0,
            80.0
        );
        // id3 share: 240 * (50/300) = 40
        assert_eq!(
            actors[&id3].effective_attribute(&AttributeKey::new(ATTR_HEALTH)).0,
            40.0
        );
    }

    #[test]
    fn shared_health_pool_exhausted_when_damage_exceeds_pool() {
        // For pool exhaustion, damage must be >= pool HP
        // Pool HP = 200 (id1=100 + id2=100)
        // We need to deal 200+ damage to exhaust
        // If id1 goes from 100 -> 0 (100 damage) and id2 goes from 100 -> 0 (100 damage), total = 200, pool exhausted
        let id1 = ActorId(10);
        let id2 = ActorId(11);

        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        actors.insert(id1, mock_actor(id1, 100.0));
        actors.insert(id2, mock_actor(id2, 100.0));

        let mut tracker = SharedHealthTracker::new();
        tracker.register_pool(1, vec![id1, id2], &actors);

        // Initial pool HP = 200
        assert_eq!(tracker.get_pool_hp(1), Some(200.0));

        tracker.snapshot_pre_damage(&actors);

        // Framework applies 200 total damage: id1 (100 -> 0) + id2 (100 -> 0)
        if let Some(actor) = actors.get_mut(&id1) {
            actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(0.0));
        }
        if let Some(actor) = actors.get_mut(&id2) {
            actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(0.0));
        }

        let exhausted = tracker.process_post_damage(&mut actors);

        // Pool should be exhausted (200 damage >= 200 pool HP)
        assert_eq!(exhausted.len(), 1);
        assert_eq!(exhausted[0], 1);
        assert_eq!(tracker.get_pool_hp(1), Some(0.0));
    }
}