//! DDGC targeting intent model — game-layer targeting semantics that bridge DDGC skill
//! definitions to the framework's TargetSelector.
//!
//! DDGC targeting is more expressive than the framework's TargetSelector alone:
//! - Launch constraints specify which slots an actor must occupy to use a skill
//! - Target rank specifies which rows (front/back) are valid targets
//! - Side affinity specifies whether the skill targets allies, enemies, or both
//! - Target count specifies single vs multiple targeting intent
//!
//! This module provides TargetingIntent as a game-layer abstraction that can be
//! used to configure skill targeting and resolve it deterministically against
//! the framework's formation and actor state.

use framework_combat::encounter::CombatSide;
use framework_combat::formation::{FormationLayout, Lane, SlotIndex};
use framework_rules::actor::ActorId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Launch constraint — specifies where an actor must be positioned to use a skill.
///
/// DDGC skills often have positional requirements (e.g., ".launch 0" means the actor
/// must be in the front row). This constraint captures that intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LaunchConstraint {
    /// No launch constraint — the actor can use the skill from any position.
    Any,
    /// The actor must be in the front row (lane 0).
    FrontRow,
    /// The actor must be in the back row (lane > 0).
    BackRow,
    /// The actor must be in a specific lane.
    SpecificLane(Lane),
    /// The actor must be in a slot within the given range [min, max].
    SlotRange { min: u32, max: u32 },
}

impl LaunchConstraint {
    /// Check if an actor at the given slot satisfies this launch constraint.
    pub fn is_satisfied(&self, slot: SlotIndex, formation: &FormationLayout) -> bool {
        match self {
            LaunchConstraint::Any => true,
            LaunchConstraint::FrontRow => slot.0 < formation.slots_per_lane,
            LaunchConstraint::BackRow => slot.0 >= formation.slots_per_lane,
            LaunchConstraint::SpecificLane(lane) => formation
                .slots
                .get(&slot)
                .is_some_and(|s| s.lane == *lane),
            LaunchConstraint::SlotRange { min, max } => slot.0 >= *min && slot.0 <= *max,
        }
    }
}

/// Target rank — specifies which ranks (rows) are valid targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetRank {
    /// Any rank is a valid target.
    Any,
    /// Only the front row (lane 0) is a valid target.
    Front,
    /// Only the back row (lane > 0) is a valid target.
    Back,
    /// Both front and back rows are valid targets.
    FrontAndBack,
}

impl TargetRank {
    /// Check if a slot in the given lane satisfies this target rank.
    pub fn is_satisfied(&self, lane: Lane, _slots_per_lane: u32) -> bool {
        match self {
            TargetRank::Any => true,
            TargetRank::Front => lane.0 == 0,
            TargetRank::Back => lane.0 > 0,
            TargetRank::FrontAndBack => true,
        }
    }
}

/// Side affinity — specifies which sides can be targeted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SideAffinity {
    /// Only enemies can be targeted.
    Enemy,
    /// Only allies (including self) can be targeted.
    Ally,
    /// Both allies and enemies can be targeted.
    Any,
}

impl SideAffinity {
    /// Check if the given side matches this affinity relative to the actor's side.
    pub fn matches(&self, actor_side: CombatSide, target_side: CombatSide) -> bool {
        match self {
            SideAffinity::Enemy => target_side != actor_side,
            SideAffinity::Ally => target_side == actor_side,
            SideAffinity::Any => true,
        }
    }
}

/// Target count — specifies whether single or multiple targeting is intended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetCount {
    /// Exactly one target must be selected.
    Single,
    /// Multiple targets are allowed.
    Multiple,
}

/// DDGC targeting intent — captures the full targeting semantics of a DDGC skill.
///
/// This struct combines launch constraints, target rank, side affinity, and target
/// count into a single intent that can be resolved deterministically against the
/// current encounter state.
///
/// # Example
///
/// ```ignore
/// // A skill that can only be used by front-row actors, targets back-row enemies
/// TargetingIntent {
///     launch_constraint: LaunchConstraint::FrontRow,
///     target_rank: TargetRank::Back,
///     side_affinity: SideAffinity::Enemy,
///     target_count: TargetCount::Single,
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetingIntent {
    /// Where the actor must be to use this skill.
    pub launch_constraint: LaunchConstraint,
    /// Which ranks (rows) are valid targets.
    pub target_rank: TargetRank,
    /// Which sides are valid targets.
    pub side_affinity: SideAffinity,
    /// Whether single or multiple targets are intended.
    pub target_count: TargetCount,
}

impl TargetingIntent {
    /// Create a new targeting intent with the given parameters.
    pub fn new(
        launch_constraint: LaunchConstraint,
        target_rank: TargetRank,
        side_affinity: SideAffinity,
        target_count: TargetCount,
    ) -> Self {
        TargetingIntent {
            launch_constraint,
            target_rank,
            side_affinity,
            target_count,
        }
    }

    /// Create a targeting intent for a skill that targets all enemies (no constraints).
    ///
    /// This is the default DDGC behavior for most offensive skills.
    pub fn all_enemies() -> Self {
        TargetingIntent {
            launch_constraint: LaunchConstraint::Any,
            target_rank: TargetRank::Any,
            side_affinity: SideAffinity::Enemy,
            target_count: TargetCount::Multiple,
        }
    }

    /// Create a targeting intent for a skill that targets all allies (no constraints).
    ///
    /// This is the default DDGC behavior for most support/heal skills.
    pub fn all_allies() -> Self {
        TargetingIntent {
            launch_constraint: LaunchConstraint::Any,
            target_rank: TargetRank::Any,
            side_affinity: SideAffinity::Ally,
            target_count: TargetCount::Multiple,
        }
    }

    /// Create a targeting intent for a self-target skill.
    pub fn self_only() -> Self {
        TargetingIntent {
            launch_constraint: LaunchConstraint::Any,
            target_rank: TargetRank::Any,
            side_affinity: SideAffinity::Ally,
            target_count: TargetCount::Single,
        }
    }

    /// Resolve this targeting intent against the current encounter state.
    ///
    /// Returns the list of valid target actor IDs that satisfy all targeting constraints:
    /// - The actor must satisfy the launch constraint
    /// - Target actors must be in a valid rank (per target_rank)
    /// - Target actors must match the side affinity
    ///
    /// The results are sorted by ActorId for deterministic ordering.
    pub fn resolve(
        &self,
        actor: ActorId,
        formation: &FormationLayout,
        actors: &HashMap<ActorId, framework_rules::actor::ActorAggregate>,
        side_lookup: &HashMap<ActorId, CombatSide>,
    ) -> Vec<ActorId> {
        // First, find the actor's slot to check launch constraint
        let actor_slot = formation.find_actor(actor);
        let actor_side = side_lookup.get(&actor).copied();

        // If actor has no slot or no side, no valid targets
        let Some(actor_slot) = actor_slot else {
            return vec![];
        };
        let Some(actor_side) = actor_side else {
            return vec![];
        };

        // Check launch constraint
        if !self.launch_constraint.is_satisfied(actor_slot, formation) {
            return vec![];
        }

        // Collect candidates that satisfy rank and side affinity
        let mut candidates: Vec<ActorId> = Vec::new();
        for &id in actors.keys() {
            // Check side affinity first
            let Some(&target_side) = side_lookup.get(&id) else {
                continue;
            };
            if !self.side_affinity.matches(actor_side, target_side) {
                continue;
            }

            // Check rank constraint
            if let Some(slot) = formation.find_actor(id) {
                if let Some(formation_slot) = formation.slots.get(&slot) {
                    if !self.target_rank.is_satisfied(formation_slot.lane, formation.slots_per_lane) {
                        continue;
                    }
                }
            }

            candidates.push(id);
        }

        // Sort for deterministic output
        candidates.sort_by_key(|id| id.0);

        // Apply target count limit
        if matches!(self.target_count, TargetCount::Single) && !candidates.is_empty() {
            candidates.truncate(1);
        }

        candidates
    }
}

/// A targeting context built from an in-progress encounter state.
///
/// This holds all the information needed to resolve targeting decisions
/// deterministically.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetingContext {
    /// The actor attempting to use the skill.
    pub actor: ActorId,
    /// The actor's current slot in the formation.
    pub actor_slot: SlotIndex,
    /// The actor's combat side.
    pub actor_side: CombatSide,
    /// The formation layout.
    pub formation: FormationLayout,
    /// The side lookup map for all actors.
    side_lookup: HashMap<ActorId, CombatSide>,
    /// The targeting intent for the skill being used.
    pub intent: TargetingIntent,
}

impl TargetingContext {
    /// Build a targeting context from an actor and skill targeting intent.
    ///
    /// Returns None if the actor is not in the formation or has no side assignment.
    pub fn from_actor_and_intent(
        actor: ActorId,
        formation: &FormationLayout,
        _actors: &HashMap<ActorId, framework_rules::actor::ActorAggregate>,
        side_lookup: &HashMap<ActorId, CombatSide>,
        intent: TargetingIntent,
    ) -> Option<Self> {
        let actor_slot = formation.find_actor(actor)?;
        let actor_side = side_lookup.get(&actor).copied()?;

        Some(TargetingContext {
            actor,
            actor_slot,
            actor_side,
            formation: formation.clone(),
            side_lookup: side_lookup.clone(),
            intent,
        })
    }

    /// Resolve the valid targets for this context.
    pub fn resolve_targets(
        &self,
        actors: &HashMap<ActorId, framework_rules::actor::ActorAggregate>,
    ) -> Vec<ActorId> {
        self.intent
            .resolve(self.actor, &self.formation, actors, &self.side_lookup)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use framework_rules::actor::ActorAggregate;
    use framework_rules::attributes::{AttributeKey, AttributeValue, ATTR_HEALTH};

    fn setup_formation_2x2() -> (FormationLayout, HashMap<ActorId, ActorAggregate>, HashMap<ActorId, CombatSide>) {
        let mut formation = FormationLayout::new(2, 2);
        // Formation layout: 2 lanes, 2 slots per lane
        // Lane 0 = front row (slots 0-1), Lane 1 = back row (slots 2-3)
        formation.place(ActorId(1), SlotIndex(0)).unwrap(); // Ally front row
        formation.place(ActorId(2), SlotIndex(2)).unwrap(); // Ally back row
        formation.place(ActorId(10), SlotIndex(1)).unwrap(); // Enemy front row
        formation.place(ActorId(20), SlotIndex(3)).unwrap(); // Enemy back row

        let mut actors = HashMap::new();
        for id in [ActorId(1), ActorId(2), ActorId(10), ActorId(20)] {
            let mut a = ActorAggregate::new(id);
            a.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
            actors.insert(id, a);
        }

        let mut side_lookup = HashMap::new();
        side_lookup.insert(ActorId(1), CombatSide::Ally);
        side_lookup.insert(ActorId(2), CombatSide::Ally);
        side_lookup.insert(ActorId(10), CombatSide::Enemy);
        side_lookup.insert(ActorId(20), CombatSide::Enemy);

        (formation, actors, side_lookup)
    }

    #[test]
    fn targeting_intent_all_enemies_targets_all_enemies() {
        let (formation, actors, side_lookup) = setup_formation_2x2();

        let intent = TargetingIntent::all_enemies();
        let targets = intent.resolve(ActorId(1), &formation, &actors, &side_lookup);

        assert_eq!(targets.len(), 2);
        assert!(targets.contains(&ActorId(10)));
        assert!(targets.contains(&ActorId(20)));
    }

    #[test]
    fn targeting_intent_all_allies_targets_all_allies() {
        let (formation, actors, side_lookup) = setup_formation_2x2();

        let intent = TargetingIntent::all_allies();
        let targets = intent.resolve(ActorId(1), &formation, &actors, &side_lookup);

        // Includes self and other ally
        assert_eq!(targets.len(), 2);
        assert!(targets.contains(&ActorId(1)));
        assert!(targets.contains(&ActorId(2)));
    }

    #[test]
    fn targeting_intent_launch_constraint_front_row() {
        let (formation, actors, side_lookup) = setup_formation_2x2();

        let intent = TargetingIntent {
            launch_constraint: LaunchConstraint::FrontRow,
            target_rank: TargetRank::Any,
            side_affinity: SideAffinity::Enemy,
            target_count: TargetCount::Multiple,
        };

        // Actor 1 is at slot 0 (front row), should have valid targets
        let targets = intent.resolve(ActorId(1), &formation, &actors, &side_lookup);
        assert_eq!(targets.len(), 2);

        // Actor 2 is at slot 2 (back row), should have no valid targets
        let targets = intent.resolve(ActorId(2), &formation, &actors, &side_lookup);
        assert!(targets.is_empty());
    }

    #[test]
    fn targeting_intent_target_rank_front() {
        let (formation, actors, side_lookup) = setup_formation_2x2();

        let intent = TargetingIntent {
            launch_constraint: LaunchConstraint::Any,
            target_rank: TargetRank::Front,
            side_affinity: SideAffinity::Enemy,
            target_count: TargetCount::Multiple,
        };

        // Actor 1 at slot 0 (front row) targeting front rank enemies
        // Enemy at slot 1 is in lane 0 (front row), enemy at slot 3 is in lane 1 (back row)
        // So only enemy at slot 1 (ActorId 10) should be targeted
        let targets = intent.resolve(ActorId(1), &formation, &actors, &side_lookup);
        assert_eq!(targets.len(), 1);
        assert!(targets.contains(&ActorId(10)));
    }

    #[test]
    fn targeting_intent_target_rank_back() {
        let (formation, actors, side_lookup) = setup_formation_2x2();

        let intent = TargetingIntent {
            launch_constraint: LaunchConstraint::Any,
            target_rank: TargetRank::Back,
            side_affinity: SideAffinity::Enemy,
            target_count: TargetCount::Multiple,
        };

        // Actor 1 at slot 0 (front row) targeting back rank enemies
        // Enemy at slot 3 is in lane 1 (back row)
        let targets = intent.resolve(ActorId(1), &formation, &actors, &side_lookup);
        assert_eq!(targets.len(), 1);
        assert!(targets.contains(&ActorId(20)));
    }

    #[test]
    fn targeting_intent_single_target_returns_one() {
        let (formation, actors, side_lookup) = setup_formation_2x2();

        let intent = TargetingIntent {
            launch_constraint: LaunchConstraint::Any,
            target_rank: TargetRank::Any,
            side_affinity: SideAffinity::Enemy,
            target_count: TargetCount::Single,
        };

        let targets = intent.resolve(ActorId(1), &formation, &actors, &side_lookup);
        assert_eq!(targets.len(), 1);
    }

    #[test]
    fn targeting_context_builds_from_actor_and_intent() {
        let (formation, actors, side_lookup) = setup_formation_2x2();

        let intent = TargetingIntent::all_enemies();
        let context = TargetingContext::from_actor_and_intent(
            ActorId(1),
            &formation,
            &actors,
            &side_lookup,
            intent.clone(),
        );

        assert!(context.is_some());
        let ctx = context.unwrap();
        assert_eq!(ctx.actor, ActorId(1));
        assert_eq!(ctx.intent, intent);
    }

    #[test]
    fn targeting_context_returns_none_for_invalid_actor() {
        let (formation, actors, side_lookup) = setup_formation_2x2();

        let intent = TargetingIntent::all_enemies();
        // ActorId(999) is not in the formation
        let context = TargetingContext::from_actor_and_intent(
            ActorId(999),
            &formation,
            &actors,
            &side_lookup,
            intent,
        );

        assert!(context.is_none());
    }

    #[test]
    fn targets_are_sorted_deterministically() {
        let (formation, actors, side_lookup) = setup_formation_2x2();

        let intent = TargetingIntent::all_enemies();

        // Run multiple times and verify same order
        for _ in 0..10 {
            let targets = intent.resolve(ActorId(1), &formation, &actors, &side_lookup);
            assert_eq!(targets, vec![ActorId(10), ActorId(20)]);
        }
    }

    #[test]
    fn targeting_context_resolve_targets_uses_stored_context() {
        let (formation, actors, side_lookup) = setup_formation_2x2();

        let intent = TargetingIntent::all_enemies();
        let context = TargetingContext::from_actor_and_intent(
            ActorId(1),
            &formation,
            &actors,
            &side_lookup,
            intent,
        )
        .unwrap();

        // resolve_targets should use the stored context, not need external side_lookup
        let targets = context.resolve_targets(&actors);
        assert_eq!(targets.len(), 2);
        assert!(targets.contains(&ActorId(10)));
        assert!(targets.contains(&ActorId(20)));
    }
}
