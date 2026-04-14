//! Paired boss state tracking for HP-averaging mechanics.
//!
//! Some DDGC bosses have paired units that share HP via the crimson_duet skill.
//! When the assassin uses crimson_duet, both the assassin and shadow have their
//! HP averaged together.
//!
//! ## Bloodthirsty Paired Boss Pattern
//!
//! The bloodthirsty_assassin + bloodthirsty_shadow pair have a unique mechanic
//! where crimson_duet averages HP between the two. This keeps both units at
//! roughly equal HP, creating a more interesting fight where you need to pressure
//! both targets.
//!
//! ## Implementation
//!
//! `PairedBossTracker` maintains the paired relationships:
//! - `paired_with: HashMap<ActorId, ActorId>` — actor ID → partner actor ID
//!
//! HP averaging is triggered when crimson_duet skill is resolved. The tracker
//! computes the average HP of both paired actors and sets both to that average.

use std::collections::HashMap;

use framework_rules::actor::{ActorAggregate, ActorId};
use framework_rules::attributes::{AttributeKey, AttributeValue, ATTR_HEALTH};

/// Tracks paired boss relationships for HP-averaging mechanics.
///
/// For bloodthirsty_assassin + bloodthirsty_shadow encounters, crimson_duet
/// skill triggers HP averaging between the paired units.
#[derive(Debug, Clone, Default)]
pub struct PairedBossTracker {
    /// Maps each actor to their paired partner.
    paired_with: HashMap<ActorId, ActorId>,
    /// Track if averaging was already triggered this turn (prevent double-trigger)
    averaging_done_this_turn: HashMap<ActorId, bool>,
}

impl PairedBossTracker {
    /// Create a new empty tracker.
    pub fn new() -> Self {
        PairedBossTracker::default()
    }

    /// Initialize paired relationship between two actors.
    pub fn establish_pair(&mut self, actor_a: ActorId, actor_b: ActorId) {
        self.paired_with.insert(actor_a, actor_b);
        self.paired_with.insert(actor_b, actor_a);
    }

    /// Get the paired partner for an actor, if any.
    pub fn partner_of(&self, actor_id: ActorId) -> Option<ActorId> {
        self.paired_with.get(&actor_id).copied()
    }

    /// Check if an actor has a paired partner.
    pub fn is_paired(&self, actor_id: ActorId) -> bool {
        self.paired_with.contains_key(&actor_id)
    }

    /// Check if HP averaging was already done for this actor this turn.
    /// Used to prevent double-triggering when multiple crimson_duet effects fire.
    pub fn averaging_done(&self, actor_id: ActorId) -> bool {
        self.averaging_done_this_turn.get(&actor_id).copied().unwrap_or(false)
    }

    /// Mark HP averaging as done for this actor this turn.
    pub fn mark_averaging_done(&mut self, actor_id: ActorId) {
        self.averaging_done_this_turn.insert(actor_id, true);
        // Also mark the partner
        if let Some(partner) = self.partner_of(actor_id) {
            self.averaging_done_this_turn.insert(partner, true);
        }
    }

    /// Reset averaging flag at the start of a new round.
    /// Call this when a new round begins.
    pub fn reset_round(&mut self) {
        self.averaging_done_this_turn.clear();
    }
}

/// Execute HP averaging between paired actors.
///
/// When crimson_duet is used, both the actor and their partner have their
/// HP averaged. The average is computed from current HP values.
///
/// Returns the average HP value, or None if the actor has no partner.
pub fn execute_hp_averaging(
    actor_id: ActorId,
    actors: &mut HashMap<ActorId, ActorAggregate>,
    tracker: &PairedBossTracker,
) -> Option<(ActorId, ActorId, f64)> {
    let partner_id = tracker.partner_of(actor_id)?;

    // Get current HP of both actors
    let hp_a = actors
        .get(&actor_id)
        .map(|a| a.effective_attribute(&AttributeKey::new(ATTR_HEALTH)).0)?;

    let hp_b = actors
        .get(&partner_id)
        .map(|a| a.effective_attribute(&AttributeKey::new(ATTR_HEALTH)).0)?;

    // Compute average HP
    let avg_hp = (hp_a + hp_b) / 2.0;

    // Set both actors to the average HP
    if let Some(actor) = actors.get_mut(&actor_id) {
        actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(avg_hp));
    }
    if let Some(actor) = actors.get_mut(&partner_id) {
        actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(avg_hp));
    }

    Some((actor_id, partner_id, avg_hp))
}

/// Check if a pack ID represents a paired boss encounter.
pub fn is_paired_boss_pack(pack_id: &str) -> bool {
    pack_id == "cross_boss_bloodthirsty_assassin"
}

/// Initialize paired boss tracking for a pack.
///
/// For paired boss packs, establishes the paired relationship between
/// the boss actors based on their family IDs.
pub fn init_paired_boss_for_pack(
    pack_id: &str,
    actor_families: &HashMap<ActorId, String>,
) -> PairedBossTracker {
    let mut tracker = PairedBossTracker::new();

    if !is_paired_boss_pack(pack_id) {
        return tracker;
    }

    // Find bloodthirsty_assassin and bloodthirsty_shadow actors
    let mut assassin_id: Option<ActorId> = None;
    let mut shadow_id: Option<ActorId> = None;

    for (&actor_id, family) in actor_families.iter() {
        if family.contains("bloodthirsty_assassin") && !family.contains("shadow") {
            assassin_id = Some(actor_id);
        }
        if family.contains("bloodthirsty_shadow") {
            shadow_id = Some(actor_id);
        }
    }

    // Establish the pair if both actors are present
    if let (Some(a_id), Some(s_id)) = (assassin_id, shadow_id) {
        tracker.establish_pair(a_id, s_id);
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
            dodge: 0.0,
        };
        arch.create_actor(actor_id)
    }

    #[test]
    fn paired_boss_tracker_new_is_empty() {
        let tracker = PairedBossTracker::new();
        assert!(tracker.paired_with.is_empty());
    }

    #[test]
    fn paired_boss_tracker_establish_pair() {
        let mut tracker = PairedBossTracker::new();
        let id1 = ActorId(10);
        let id2 = ActorId(11);

        tracker.establish_pair(id1, id2);

        assert!(tracker.is_paired(id1));
        assert!(tracker.is_paired(id2));
        assert_eq!(tracker.partner_of(id1), Some(id2));
        assert_eq!(tracker.partner_of(id2), Some(id1));
    }

    #[test]
    fn paired_boss_tracker_no_partner() {
        let tracker = PairedBossTracker::new();
        let id1 = ActorId(10);

        assert!(!tracker.is_paired(id1));
        assert_eq!(tracker.partner_of(id1), None);
    }

    #[test]
    fn execute_hp_averaging_both_actors_equal() {
        // When both actors have same HP, averaging should keep them the same
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let id1 = ActorId(10);
        let id2 = ActorId(11);

        actors.insert(id1, mock_actor(id1, 100.0));
        actors.insert(id2, mock_actor(id2, 100.0));

        let mut tracker = PairedBossTracker::new();
        tracker.establish_pair(id1, id2);

        let result = execute_hp_averaging(id1, &mut actors, &tracker);

        assert!(result.is_some());
        let (_, _, avg_hp) = result.unwrap();
        assert_eq!(avg_hp, 100.0);

        // Both should now have 100 HP
        let hp1 = actors[&id1].effective_attribute(&AttributeKey::new(ATTR_HEALTH)).0;
        let hp2 = actors[&id2].effective_attribute(&AttributeKey::new(ATTR_HEALTH)).0;
        assert_eq!(hp1, 100.0);
        assert_eq!(hp2, 100.0);
    }

    #[test]
    fn execute_hp_averaging_both_actors_different() {
        // When actors have different HP, averaging should set both to the average
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let id1 = ActorId(10);
        let id2 = ActorId(11);

        actors.insert(id1, mock_actor(id1, 120.0));
        actors.insert(id2, mock_actor(id2, 80.0));

        let mut tracker = PairedBossTracker::new();
        tracker.establish_pair(id1, id2);

        let result = execute_hp_averaging(id1, &mut actors, &tracker);

        assert!(result.is_some());
        let (_, _, avg_hp) = result.unwrap();
        assert_eq!(avg_hp, 100.0); // (120 + 80) / 2 = 100

        // Both should now have 100 HP
        let hp1 = actors[&id1].effective_attribute(&AttributeKey::new(ATTR_HEALTH)).0;
        let hp2 = actors[&id2].effective_attribute(&AttributeKey::new(ATTR_HEALTH)).0;
        assert_eq!(hp1, 100.0);
        assert_eq!(hp2, 100.0);
    }

    #[test]
    fn execute_hp_averaging_no_partner() {
        // When actor has no partner, should return None
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let id1 = ActorId(10);

        actors.insert(id1, mock_actor(id1, 100.0));

        let tracker = PairedBossTracker::new();

        let result = execute_hp_averaging(id1, &mut actors, &tracker);

        assert!(result.is_none());
    }

    #[test]
    fn averaging_done_flag() {
        let mut tracker = PairedBossTracker::new();
        let id1 = ActorId(10);
        let id2 = ActorId(11);

        tracker.establish_pair(id1, id2);

        assert!(!tracker.averaging_done(id1));
        assert!(!tracker.averaging_done(id2));

        tracker.mark_averaging_done(id1);

        assert!(tracker.averaging_done(id1));
        assert!(tracker.averaging_done(id2)); // Partner also marked
    }

    #[test]
    fn reset_round_clears_flags() {
        let mut tracker = PairedBossTracker::new();
        let id1 = ActorId(10);

        tracker.mark_averaging_done(id1);
        assert!(tracker.averaging_done(id1));

        tracker.reset_round();

        assert!(!tracker.averaging_done(id1));
    }

    #[test]
    fn is_paired_boss_pack_bloodthirsty() {
        assert!(is_paired_boss_pack("cross_boss_bloodthirsty_assassin"));
    }

    #[test]
    fn is_paired_boss_pack_other() {
        assert!(!is_paired_boss_pack("qinglong_boss_azure_dragon"));
        assert!(!is_paired_boss_pack("baihu_boss_white_tiger"));
        assert!(!is_paired_boss_pack("zhuque_boss_vermilion_bird"));
    }

    #[test]
    fn init_paired_boss_for_pack_bloodthirsty() {
        let mut actor_families: HashMap<ActorId, String> = HashMap::new();
        actor_families.insert(ActorId(10), "bloodthirsty_assassin".to_string());
        actor_families.insert(ActorId(11), "bloodthirsty_shadow".to_string());

        let tracker = init_paired_boss_for_pack("cross_boss_bloodthirsty_assassin", &actor_families);

        assert!(tracker.is_paired(ActorId(10)));
        assert!(tracker.is_paired(ActorId(11)));
        assert_eq!(tracker.partner_of(ActorId(10)), Some(ActorId(11)));
        assert_eq!(tracker.partner_of(ActorId(11)), Some(ActorId(10)));
    }

    #[test]
    fn init_paired_boss_for_pack_other() {
        let mut actor_families: HashMap<ActorId, String> = HashMap::new();
        actor_families.insert(ActorId(10), "azure_dragon".to_string());
        actor_families.insert(ActorId(11), "white_tiger".to_string());

        let tracker = init_paired_boss_for_pack("qinglong_boss_azure_dragon", &actor_families);

        // No pairs established for non-paired boss packs
        assert!(!tracker.is_paired(ActorId(10)));
        assert!(!tracker.is_paired(ActorId(11)));
    }
}