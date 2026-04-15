//! Multi-phase boss transition tracking.
//!
//! Some DDGC bosses have multiple phases where the boss transitions from
//! one form to another based on game conditions. This module tracks those
//! transitions and executes them.
//!
//! ## White Tiger Phase Transition
//!
//! White Tiger starts with two clone forms (A and B). After the clone
//! group has been pressed (hit) a certain number of times, both clones
//! are destroyed and the final form (C) is summoned.
//!
//! Implementation:
//! - `PhaseTransitionTracker` tracks press_attack_count for clone groups
//! - When count reaches the threshold, the transition is triggered
//! - Clone actors are removed from the encounter
//! - The final form actor is materialized into the encounter
//!
//! ## Design
//!
//! The seam is intentionally narrow: `PhaseTransitionTracker` takes encounter
//! state and mutates it directly when a transition triggers. The tracker is
//! called in the battle loop after damage resolution.

use std::collections::HashMap;

use framework_combat::encounter::{CombatSide, Encounter};
use framework_combat::formation::{FormationLayout, SlotIndex};
use framework_rules::actor::{ActorAggregate, ActorId};

use crate::content::ContentPack;
use crate::monsters::MonsterFamilyRegistry;

/// A phase transition event — signals that a boss should transition to a new phase.
#[derive(Debug, Clone)]
pub struct PhaseTransitionEvent {
    /// The actor IDs that should be removed (clone forms).
    pub remove_actors: Vec<ActorId>,
    /// The family ID of the actor to summon for the new phase.
    pub summon_family_id: String,
    /// The slot index where the new phase actor should be placed.
    pub placement_slot: SlotIndex,
}

/// Tracks phase transitions for multi-phase bosses.
///
/// Currently supports:
/// - White Tiger: A+B clones → C final form (triggered by press_attack_count)
#[derive(Debug, Clone, Default)]
pub struct PhaseTransitionTracker {
    /// Track press attacks on clone groups.
    /// Key: (boss_pack_id), Value: current press count.
    press_counts: HashMap<String, u32>,
    /// Whether a transition has already occurred for a given pack.
    transitioned: HashMap<String, bool>,
    /// Actor IDs that are part of the clone group (for white_tiger).
    clone_actor_ids: HashMap<String, Vec<ActorId>>,
}

impl PhaseTransitionTracker {
    /// Creates a new empty tracker.
    pub fn new() -> Self {
        PhaseTransitionTracker::default()
    }

    /// Initialize tracking for a specific boss pack.
    ///
    /// This sets up the clone group actor IDs and the threshold for phase transitions.
    pub fn init_for_pack(&mut self, pack_id: &str, clone_actor_ids: Vec<ActorId>) {
        self.press_counts.insert(pack_id.to_string(), 0);
        self.transitioned.insert(pack_id.to_string(), false);
        self.clone_actor_ids.insert(pack_id.to_string(), clone_actor_ids);
    }

    /// Check if a pack has phase transition tracking initialized.
    pub fn is_tracked(&self, pack_id: &str) -> bool {
        self.press_counts.contains_key(pack_id)
    }

    /// Check if a pack has already transitioned.
    pub fn has_transitioned(&self, pack_id: &str) -> bool {
        self.transitioned.get(pack_id).copied().unwrap_or(false)
    }

    /// Record that a clone actor was hit (pressed).
    ///
    /// This increments the press_attack_count for the pack.
    /// Returns the new count.
    pub fn record_press(&mut self, pack_id: &str, actor_id: ActorId) -> u32 {
        // Only count if:
        // 1. Pack is tracked
        // 2. Transition hasn't occurred yet
        // 3. Actor is part of the clone group
        if !self.is_tracked(pack_id) || self.has_transitioned(pack_id) {
            return 0;
        }

        let clone_ids = match self.clone_actor_ids.get(pack_id) {
            Some(ids) => ids,
            None => return 0,
        };

        if !clone_ids.contains(&actor_id) {
            return 0;
        }

        let count = self.press_counts.entry(pack_id.to_string()).or_insert(0);
        *count += 1;
        *count
    }

    /// Get the current press count for a pack.
    pub fn press_count(&self, pack_id: &str) -> u32 {
        self.press_counts.get(pack_id).copied().unwrap_or(0)
    }

    /// Get the threshold for triggering a phase transition.
    ///
    /// Currently hardcoded to 2 for white_tiger.
    /// In the future, this could be configured per-boss.
    pub fn threshold(&self, _pack_id: &str) -> u32 {
        2
    }

    /// Check if a transition should be triggered.
    pub fn should_transition(&self, pack_id: &str) -> bool {
        if !self.is_tracked(pack_id) || self.has_transitioned(pack_id) {
            return false;
        }
        self.press_count(pack_id) >= self.threshold(pack_id)
    }

    /// Mark a pack as transitioned (prevents double-triggering).
    pub fn mark_transitioned(&mut self, pack_id: &str) {
        self.transitioned.insert(pack_id.to_string(), true);
    }

    /// Get the actor IDs that are part of the clone group.
    pub fn clone_actor_ids(&self, pack_id: &str) -> Vec<ActorId> {
        self.clone_actor_ids.get(pack_id).cloned().unwrap_or_default()
    }

    /// Get the placement slot for the final form.
    ///
    /// Returns the slot of the first clone actor, or None if no clones exist.
    pub fn placement_slot(&self, pack_id: &str, formation: &FormationLayout) -> Option<SlotIndex> {
        let clone_ids = self.clone_actor_ids(pack_id);
        for &actor_id in &clone_ids {
            if let Some(slot) = formation.find_actor(actor_id) {
                return Some(slot);
            }
        }
        None
    }
}

/// Execute a phase transition: remove clone actors and materialize the final form.
///
/// Returns the ActorId of the newly created final form actor, if successful.
#[allow(clippy::too_many_arguments)]
pub fn execute_phase_transition(
    event: &PhaseTransitionEvent,
    actors: &mut HashMap<ActorId, ActorAggregate>,
    formation: &mut FormationLayout,
    encounter: &mut Encounter,
    content_pack: &ContentPack,
    monster_registry: &MonsterFamilyRegistry,
    next_enemy_id: &mut u64,
) -> Option<ActorId> {
    // Remove clone actors
    for &actor_id in &event.remove_actors {
        // Remove from actors
        actors.remove(&actor_id);
        // Remove from encounter
        encounter.remove_actor(actor_id);
        if let Some(ref mut to) = encounter.turn_order {
            to.remove(actor_id);
        }
    }

    // Look up the final form family
    let family = monster_registry.get(&event.summon_family_id)?;

    // Get the archetype
    let archetype = content_pack.get_archetype(&family.archetype_name)?;

    // Create the new actor
    let actor_id = ActorId(*next_enemy_id);
    let actor = archetype.create_actor(actor_id);

    // Place in formation at the clone's slot
    if formation.place(actor_id, event.placement_slot).is_err() {
        return None;
    }

    // Add to actors
    actors.insert(actor_id, actor);

    // Add to encounter's enemy side
    encounter
        .sides
        .entry(CombatSide::Enemy)
        .or_default()
        .push(actor_id);

    // Add to turn order
    if let Some(ref mut turn_order) = encounter.turn_order {
        turn_order.queue.push_back(actor_id);
        turn_order.original_order.push(actor_id);
    }

    *next_enemy_id += 1;

    Some(actor_id)
}

/// Check if a pack ID represents a multi-phase boss.
pub fn is_multi_phase_boss(pack_id: &str) -> bool {
    pack_id == "baihu_boss_white_tiger"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_transition_tracker_new_is_empty() {
        let tracker = PhaseTransitionTracker::new();
        assert!(tracker.press_counts.is_empty());
        assert!(tracker.transitioned.is_empty());
    }

    #[test]
    fn phase_transition_tracker_init_for_pack() {
        let mut tracker = PhaseTransitionTracker::new();
        tracker.init_for_pack(
            "baihu_boss_white_tiger",
            vec![ActorId(10), ActorId(11)],
        );

        assert!(tracker.is_tracked("baihu_boss_white_tiger"));
        assert!(!tracker.has_transitioned("baihu_boss_white_tiger"));
        assert_eq!(tracker.press_count("baihu_boss_white_tiger"), 0);
    }

    #[test]
    fn phase_transition_tracker_record_press() {
        let mut tracker = PhaseTransitionTracker::new();
        tracker.init_for_pack(
            "baihu_boss_white_tiger",
            vec![ActorId(10), ActorId(11)],
        );

        // Record press for clone actor
        let count = tracker.record_press("baihu_boss_white_tiger", ActorId(10));
        assert_eq!(count, 1);

        // Record press for second clone
        let count = tracker.record_press("baihu_boss_white_tiger", ActorId(11));
        assert_eq!(count, 2);
    }

    #[test]
    fn phase_transition_tracker_ignores_non_clone_actors() {
        let mut tracker = PhaseTransitionTracker::new();
        tracker.init_for_pack(
            "baihu_boss_white_tiger",
            vec![ActorId(10), ActorId(11)],
        );

        // Record press for non-clone actor (hero)
        let count = tracker.record_press("baihu_boss_white_tiger", ActorId(1));
        assert_eq!(count, 0);
    }

    #[test]
    fn phase_transition_tracker_threshold() {
        let mut tracker = PhaseTransitionTracker::new();
        tracker.init_for_pack(
            "baihu_boss_white_tiger",
            vec![ActorId(10), ActorId(11)],
        );

        assert_eq!(tracker.threshold("baihu_boss_white_tiger"), 2);
    }

    #[test]
    fn phase_transition_tracker_should_transition() {
        let mut tracker = PhaseTransitionTracker::new();
        tracker.init_for_pack(
            "baihu_boss_white_tiger",
            vec![ActorId(10), ActorId(11)],
        );

        // Should not transition yet
        assert!(!tracker.should_transition("baihu_boss_white_tiger"));

        // Record first press
        tracker.record_press("baihu_boss_white_tiger", ActorId(10));
        assert!(!tracker.should_transition("baihu_boss_white_tiger"));

        // Record second press
        tracker.record_press("baihu_boss_white_tiger", ActorId(11));
        assert!(tracker.should_transition("baihu_boss_white_tiger"));
    }

    #[test]
    fn phase_transition_tracker_prevents_double_transition() {
        let mut tracker = PhaseTransitionTracker::new();
        tracker.init_for_pack(
            "baihu_boss_white_tiger",
            vec![ActorId(10), ActorId(11)],
        );

        // Trigger transition
        tracker.record_press("baihu_boss_white_tiger", ActorId(10));
        tracker.record_press("baihu_boss_white_tiger", ActorId(11));
        assert!(tracker.should_transition("baihu_boss_white_tiger"));

        // Mark as transitioned
        tracker.mark_transitioned("baihu_boss_white_tiger");
        assert!(tracker.has_transitioned("baihu_boss_white_tiger"));
        assert!(!tracker.should_transition("baihu_boss_white_tiger"));
    }

    #[test]
    fn phase_transition_tracker_clone_ids() {
        let mut tracker = PhaseTransitionTracker::new();
        tracker.init_for_pack(
            "baihu_boss_white_tiger",
            vec![ActorId(10), ActorId(11)],
        );

        let ids = tracker.clone_actor_ids("baihu_boss_white_tiger");
        assert_eq!(ids, vec![ActorId(10), ActorId(11)]);
    }

    #[test]
    fn is_multi_phase_boss_white_tiger() {
        assert!(is_multi_phase_boss("baihu_boss_white_tiger"));
    }

    #[test]
    fn is_multi_phase_boss_other() {
        assert!(!is_multi_phase_boss("qinglong_boss_azure_dragon"));
        assert!(!is_multi_phase_boss("zhuque_boss_vermilion_bird"));
    }
}