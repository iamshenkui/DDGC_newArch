//! Captor and release runtime state tracking.
//!
//! Some DDGC bosses have captor mechanics where heroes are captured into
//! cauldron-type units (egg_membrane). While captured, heroes:
//! - Stop acting as normal actors (removed from turn order)
//! - Take passive damage from the captor each round
//! - Are released when the captor dies or when they reach death's door
//!
//! ## Necrodrake Embryosac Captor Pattern
//!
//! The necrodrake_embryosac boss uses `untimely_progeny` to capture heroes
//! into egg_membrane_empty cauldrons. The captor transitions:
//! - `egg_membrane_empty` → `egg_membrane_full` (on capture)
//! - `egg_membrane_full` → `egg_membrane_empty` (on captor death, releasing prisoner)
//!
//! ## Implementation
//!
//! `CaptorTracker` maintains the capture relationships:
//! - `captive_of: HashMap<ActorId, ActorId>` — hero ID → captor actor ID
//! - `captor_holds: HashMap<ActorId, ActorId>` — captor actor ID → hero ID
//!
//! Captured heroes remain in the `actors` HashMap but are marked as
//! non-acting and removed from the turn order queue.

use std::collections::HashMap;

use framework_rules::actor::{ActorAggregate, ActorId};
use framework_rules::attributes::{AttributeKey, AttributeValue, ATTR_HEALTH};

/// Tracks captor state for encounter battles.
///
/// For necrodrake_embryosac encounters, egg_membrane_empty actors serve
/// as captor vessels. When a hero is captured, they enter the captive state
/// and the egg_membrane_empty transforms to egg_membrane_full.
#[derive(Debug, Clone, Default)]
pub struct CaptorTracker {
    /// Maps captured hero ID to the captor actor ID holding them.
    captive_of: HashMap<ActorId, ActorId>,
    /// Maps captor actor ID to the captured hero ID.
    captor_holds: HashMap<ActorId, ActorId>,
    /// Actor IDs that are currently in captive state (non-acting).
    captive_actors: HashMap<ActorId, CaptiveInfo>,
}

/// Additional info about a captive actor.
#[derive(Debug, Clone)]
pub struct CaptiveInfo {
    /// The captor actor ID holding this hero.
    pub captor_id: ActorId,
    /// The turn number when capture occurred.
    pub capture_turn: u32,
    /// The hero's HP at the time of capture (for potential restoration).
    pub hp_at_capture: f64,
}

impl CaptorTracker {
    /// Create a new empty captor tracker.
    pub fn new() -> Self {
        CaptorTracker::default()
    }

    /// Check if an actor is currently in captive state.
    pub fn is_captive(&self, actor_id: ActorId) -> bool {
        self.captive_actors.contains_key(&actor_id)
    }

    /// Get the captor ID for a captive actor, if any.
    pub fn captor_for(&self, actor_id: ActorId) -> Option<ActorId> {
        self.captive_of.get(&actor_id).copied()
    }

    /// Get the captive ID for a captor actor, if any.
    pub fn captive_of_captor(&self, captor_id: ActorId) -> Option<ActorId> {
        self.captor_holds.get(&captor_id).copied()
    }

    /// Check if an actor is a captor holding a captive.
    pub fn is_captor(&self, actor_id: ActorId) -> bool {
        self.captor_holds.contains_key(&actor_id)
    }

    /// Place a hero into captive state.
    ///
    /// The hero is removed from normal play and placed under the control
    /// of the specified captor actor.
    ///
    /// Returns the `CaptiveInfo` for the newly captured actor.
    pub fn capture(
        &mut self,
        hero_id: ActorId,
        captor_id: ActorId,
        capture_turn: u32,
        hero_hp: f64,
    ) -> CaptiveInfo {
        let info = CaptiveInfo {
            captor_id,
            capture_turn,
            hp_at_capture: hero_hp,
        };

        self.captive_of.insert(hero_id, captor_id);
        self.captor_holds.insert(captor_id, hero_id);
        self.captive_actors.insert(hero_id, info.clone());

        info
    }

    /// Release a hero from captive state.
    ///
    /// This is called when the captor dies or when the captive reaches
    /// death's door and must be released.
    ///
    /// Returns the ActorId of the released hero, if any.
    pub fn release(&mut self, hero_id: ActorId) -> Option<ActorId> {
        if let Some(captor_id) = self.captive_of.remove(&hero_id) {
            self.captor_holds.remove(&captor_id);
            self.captive_actors.remove(&hero_id);
            Some(hero_id)
        } else {
            None
        }
    }

    /// Release the captive held by a captor (when captor dies).
    ///
    /// Returns the ActorId of the released hero, if any.
    pub fn release_captive_of(&mut self, captor_id: ActorId) -> Option<ActorId> {
        if let Some(hero_id) = self.captor_holds.remove(&captor_id) {
            self.captive_of.remove(&hero_id);
            self.captive_actors.remove(&hero_id);
            Some(hero_id)
        } else {
            None
        }
    }

    /// Get all currently captive actor IDs.
    pub fn all_captives(&self) -> Vec<ActorId> {
        self.captive_actors.keys().copied().collect()
    }

    /// Get captive info for an actor.
    pub fn captive_info(&self, actor_id: ActorId) -> Option<&CaptiveInfo> {
        self.captive_actors.get(&actor_id)
    }

    /// Number of currently captive actors.
    pub fn captive_count(&self) -> usize {
        self.captive_actors.len()
    }
}

/// Apply passive damage from captors to their captives at end of round.
///
/// For each egg_membrane_full captor, deals 5-6 damage to the captured hero.
/// The captive is released if they reach death's door (HP <= 0).
///
/// Returns a list of (captor_id, captive_id, damage_dealt) for trace recording,
/// plus any captives that were released due to death's door.
pub fn apply_captor_dot(
    tracker: &mut CaptorTracker,
    actors: &mut HashMap<ActorId, ActorAggregate>,
    round: u32,
) -> Vec<(ActorId, ActorId, f64, bool)> {
    // Deterministic damage: use round number to seed the damage calculation
    // 5-6 damage range, using round % 2 to alternate
    let base_damage = 5.0;
    let damage_variance = if round.is_multiple_of(2) { 0.0 } else { 1.0 };

    let mut results = Vec::new();
    let mut to_release = Vec::new();

    for (&captor_id, &captive_id) in tracker.captor_holds.iter() {
        let damage = base_damage + damage_variance;

        // Apply damage to the captive
        if let Some(captive) = actors.get_mut(&captive_id) {
            let current_hp = captive.effective_attribute(&AttributeKey::new(ATTR_HEALTH)).0;
            let new_hp = (current_hp - damage).max(0.0);
            captive.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(new_hp));

            // Check if captive reached death's door (HP <= 0) — release them
            let released = new_hp <= 0.0;
            results.push((captor_id, captive_id, damage, released));

            if released {
                to_release.push(captive_id);
            }
        }
    }

    // Release captives who reached death's door
    for captive_id in to_release {
        tracker.release(captive_id);
    }

    results
}

/// Check if an actor ID is a captor actor (egg_membrane_empty/egg_membrane_full).
///
/// The captor detection is based on family ID prefix matching.
/// This is used to determine if an actor can capture heroes.
pub fn is_captor_family(family_id: &str) -> bool {
    family_id.starts_with("egg_membrane")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captor_tracker_starts_empty() {
        let tracker = CaptorTracker::new();
        assert_eq!(tracker.captive_count(), 0);
        assert!(tracker.all_captives().is_empty());
    }

    #[test]
    fn captor_tracker_capture_and_release() {
        let mut tracker = CaptorTracker::new();

        let hero = ActorId(1);
        let captor = ActorId(10);

        // Initially not captive
        assert!(!tracker.is_captive(hero));
        assert!(!tracker.is_captor(captor));

        // Capture
        let info = tracker.capture(hero, captor, 1, 150.0);
        assert_eq!(info.captor_id, captor);
        assert_eq!(info.capture_turn, 1);
        assert_eq!(info.hp_at_capture, 150.0);

        assert!(tracker.is_captive(hero));
        assert!(tracker.is_captor(captor));
        assert_eq!(tracker.captor_for(hero), Some(captor));
        assert_eq!(tracker.captive_of_captor(captor), Some(hero));
        assert_eq!(tracker.captive_count(), 1);

        // Release
        let released = tracker.release(hero);
        assert_eq!(released, Some(hero));
        assert!(!tracker.is_captive(hero));
        assert!(!tracker.is_captor(captor));
        assert_eq!(tracker.captive_count(), 0);
    }

    #[test]
    fn captor_tracker_release_by_captor() {
        let mut tracker = CaptorTracker::new();

        let hero = ActorId(1);
        let captor = ActorId(10);

        tracker.capture(hero, captor, 1, 150.0);

        // Release via captor (when captor dies)
        let released = tracker.release_captive_of(captor);
        assert_eq!(released, Some(hero));
        assert!(!tracker.is_captive(hero));
        assert_eq!(tracker.captive_count(), 0);
    }

    #[test]
    fn captor_tracker_multiple_captives() {
        let mut tracker = CaptorTracker::new();

        let hero1 = ActorId(1);
        let hero2 = ActorId(2);
        let captor1 = ActorId(10);
        let captor2 = ActorId(11);

        tracker.capture(hero1, captor1, 1, 150.0);
        tracker.capture(hero2, captor2, 2, 120.0);

        assert_eq!(tracker.captive_count(), 2);
        assert!(tracker.is_captive(hero1));
        assert!(tracker.is_captive(hero2));
        assert!(tracker.is_captor(captor1));
        assert!(tracker.is_captor(captor2));

        let all = tracker.all_captives();
        assert!(all.contains(&hero1));
        assert!(all.contains(&hero2));
    }

    #[test]
    fn is_captor_family_egg_membrane() {
        assert!(is_captor_family("egg_membrane_empty"));
        assert!(is_captor_family("egg_membrane_full"));
        assert!(!is_captor_family("necrodrake_embryosac"));
        assert!(!is_captor_family("azure_dragon"));
    }

    #[test]
    fn apply_captor_dot_releases_at_deaths_door() {
        use framework_rules::actor::ActorId;
        use crate::content::actors::Archetype;
        use framework_combat::encounter::CombatSide;

        let mut tracker = CaptorTracker::new();
        let hero = ActorId(1);
        let captor = ActorId(10);

        // Create a mock actor with 5 HP (will die from 5-6 damage)
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let arch = Archetype {
            name: crate::content::actors::ArchetypeName::new("Tank"),
            side: CombatSide::Ally,
            health: 5.0,
            max_health: 150.0,
            attack: 10.0,
            defense: 0.0,
            speed: 5.0,
            stress: 0.0,
            max_stress: 200.0,
            crit_chance: 0.0,
            accuracy: 0.95,
            dodge: 0.0,
        };
        actors.insert(hero, arch.create_actor(hero));

        // Capture the hero
        tracker.capture(hero, captor, 1, 150.0);

        // Apply DoT — should release the hero since they reach 0 HP
        let results = apply_captor_dot(&mut tracker, &mut actors, 1);
        assert!(!results.is_empty());
        let (_, _, damage, released) = &results[0];
        assert!(*damage >= 5.0 && *damage <= 6.0);
        assert!(*released); // Should be released due to death's door

        // Hero should no longer be captive
        assert!(!tracker.is_captive(hero));
    }
}