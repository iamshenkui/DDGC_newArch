//! DDGC condition evaluation context — game-layer condition checks for DDGC-specific rules.
//!
//! This module provides a `ConditionContext` that exposes DDGC-specific condition
//! evaluation data (FirstRound, StressAbove, StressBelow, etc.) to the game layer.
//! The context is created from in-progress combat state and provides read-only
//! access to actor, target, turn-state, and encounter-state data.
//!
//! Framework-native conditions (HpBelow, Probability, etc.) are handled by the
//! framework's `EffectCondition` system. This module handles DDGC-only conditions
//! that require game-layer state not available in the framework.

use std::collections::HashMap;

use framework_combat::encounter::CombatSide;
use framework_rules::actor::{ActorAggregate, ActorId};
use framework_rules::attributes::AttributeKey;

use crate::content::actors::ATTR_STRESS;
use crate::encounters::Dungeon;

/// DDGC condition evaluation context.
///
/// Created from in-progress combat state, this struct provides read-only
/// access to data needed for evaluating DDGC-specific conditions like
/// `FirstRound`, `StressAbove`, and `StressBelow`.
///
/// All data is accessed through deterministic lookups so the context
/// can be created and queried without introducing non-determinism.
///
/// # Example
///
/// ```
/// let ctx = ConditionContext::new(
///     actor_id,
///     target_ids,
///     0,                           // current round
///     &actors,
///     &side_lookup,                // map from ActorId to CombatSide
///     Dungeon::QingLong,
/// );
///
/// if ctx.is_first_round() {
///     // First-round-only effect applies
/// }
///
/// if ctx.actor_stress_above(50.0) {
///     // High-stress effect applies
/// }
/// ```
pub struct ConditionContext<'a> {
    /// The actor attempting to perform the action.
    actor_id: ActorId,
    /// The target(s) of the action.
    target_ids: Vec<ActorId>,
    /// The current round number (0 = first round).
    current_round: u32,
    /// All actors in the encounter, keyed by ID.
    actors: &'a HashMap<ActorId, ActorAggregate>,
    /// Map from actor ID to combat side (ally/enemy).
    side_lookup: &'a HashMap<ActorId, CombatSide>,
    /// The dungeon this encounter is taking place in.
    dungeon: Dungeon,
}

impl<'a> ConditionContext<'a> {
    /// Create a new condition context from combat state.
    ///
    /// All parameters must come from deterministic combat state so the
    /// context itself is deterministic.
    ///
    /// # Arguments
    ///
    /// * `actor_id` — the actor attempting the action
    /// * `target_ids` — the targets of the action
    /// * `current_round` — the current round number (0 = first round)
    /// * `actors` — all actors in the encounter
    /// * `side_lookup` — map from actor ID to combat side
    /// * `dungeon` — the dungeon this encounter is in
    pub fn new(
        actor_id: ActorId,
        target_ids: Vec<ActorId>,
        current_round: u32,
        actors: &'a HashMap<ActorId, ActorAggregate>,
        side_lookup: &'a HashMap<ActorId, CombatSide>,
        dungeon: Dungeon,
    ) -> Self {
        ConditionContext {
            actor_id,
            target_ids,
            current_round,
            actors,
            side_lookup,
            dungeon,
        }
    }

    /// Returns the actor ID.
    pub fn actor_id(&self) -> ActorId {
        self.actor_id
    }

    /// Returns the target IDs.
    pub fn target_ids(&self) -> &[ActorId] {
        &self.target_ids
    }

    /// Returns the actor attempting the action.
    pub fn actor(&self) -> Option<&'a ActorAggregate> {
        self.actors.get(&self.actor_id)
    }

    /// Returns the targets of the action.
    pub fn targets(&self) -> Vec<&'a ActorAggregate> {
        self.target_ids
            .iter()
            .filter_map(|id| self.actors.get(id))
            .collect()
    }

    /// Returns whether the current round is the first round of combat (round 0).
    ///
    /// DDGC reference: `FirstRound` condition is active only on the opening round.
    /// This method checks if `current_round == 0`.
    pub fn is_first_round(&self) -> bool {
        self.current_round == 0
    }

    /// Returns the actor's current stress level.
    ///
    /// DDGC reference: heroes have stress, monsters do not (stress is always 0
    /// for enemies). Returns 0.0 for actors without a stress attribute.
    pub fn actor_stress(&self) -> f64 {
        if let Some(actor) = self.actors.get(&self.actor_id) {
            actor
                .effective_attribute(&AttributeKey::new(ATTR_STRESS))
                .0
        } else {
            0.0
        }
    }

    /// Returns whether the actor's stress is above the given threshold.
    ///
    /// DDGC reference: `StressAbove(threshold)` — only applies to heroes.
    /// Monsters always return `false` since they don't have stress.
    ///
    /// Returns `false` if the actor has no stress attribute.
    pub fn actor_stress_above(&self, threshold: f64) -> bool {
        let actor = match self.actors.get(&self.actor_id) {
            Some(a) => a,
            None => return false,
        };

        // Only heroes have stress; monsters always fail stress conditions
        if self.side_lookup.get(&self.actor_id) != Some(&CombatSide::Ally) {
            return false;
        }

        let stress = actor
            .effective_attribute(&AttributeKey::new(ATTR_STRESS))
            .0;
        stress > threshold
    }

    /// Returns whether the actor's stress is below the given threshold.
    ///
    /// DDGC reference: `StressBelow(threshold)` — only applies to heroes.
    /// Monsters always return `false` since they don't have stress.
    ///
    /// Returns `false` if the actor has no stress attribute.
    pub fn actor_stress_below(&self, threshold: f64) -> bool {
        let actor = match self.actors.get(&self.actor_id) {
            Some(a) => a,
            None => return false,
        };

        // Only heroes have stress; monsters always fail stress conditions
        if self.side_lookup.get(&self.actor_id) != Some(&CombatSide::Ally) {
            return false;
        }

        let stress = actor
            .effective_attribute(&AttributeKey::new(ATTR_STRESS))
            .0;
        stress < threshold
    }

    /// Returns whether any target has a specific status kind active.
    ///
    /// DDGC reference: `Status("buff_name")` — checks if target has the status.
    pub fn target_has_status(&self, status_kind: &str) -> bool {
        for target_id in &self.target_ids {
            if let Some(target) = self.actors.get(target_id) {
                if has_status_kind(target, status_kind) {
                    return true;
                }
            }
        }
        false
    }

    /// Returns whether the actor has a specific status kind active.
    ///
    /// DDGC reference: `ActorStatus("buff_name")` — checks if actor has the status.
    pub fn actor_has_status(&self, status_kind: &str) -> bool {
        if let Some(actor) = self.actors.get(&self.actor_id) {
            has_status_kind(actor, status_kind)
        } else {
            false
        }
    }

    /// Returns the actor's current HP fraction (0.0 to 1.0+).
    ///
    /// DDGC reference: used for `HpBelow`, `HpAbove`, and `DeathsDoor` conditions.
    /// Returns 1.0 if the actor has no HP attribute.
    pub fn actor_hp_fraction(&self) -> f64 {
        let actor = match self.actors.get(&self.actor_id) {
            Some(a) => a,
            None => return 1.0,
        };
        let hp = actor
            .effective_attribute(&AttributeKey::new(framework_rules::attributes::ATTR_HEALTH))
            .0;
        let max_hp = actor
            .effective_attribute(&AttributeKey::new(crate::content::actors::ATTR_MAX_HEALTH))
            .0;
        if max_hp > 0.0 {
            hp / max_hp
        } else {
            1.0
        }
    }

    /// Returns whether the actor is at death's door (HP < 50% of max).
    ///
    /// DDGC reference: `DeathsDoor` condition.
    /// Returns `false` if the actor has no HP attribute.
    pub fn actor_at_deaths_door(&self) -> bool {
        let hp_frac = self.actor_hp_fraction();
        hp_frac < 0.5 && hp_frac > 0.0
    }

    /// Returns the first target's HP fraction (0.0 to 1.0+).
    ///
    /// DDGC reference: `TargetHpBelow`, `TargetHpAbove` conditions.
    /// Returns 1.0 if there are no targets or no HP attribute.
    pub fn target_hp_fraction(&self) -> f64 {
        if self.target_ids.is_empty() {
            return 1.0;
        }
        let target = match self.actors.get(&self.target_ids[0]) {
            Some(t) => t,
            None => return 1.0,
        };
        let hp = target
            .effective_attribute(&AttributeKey::new(framework_rules::attributes::ATTR_HEALTH))
            .0;
        let max_hp = target
            .effective_attribute(&AttributeKey::new(crate::content::actors::ATTR_MAX_HEALTH))
            .0;
        if max_hp > 0.0 {
            hp / max_hp
        } else {
            1.0
        }
    }

    /// Returns the current round number.
    ///
    /// DDGC reference: round 0 is the first round of combat.
    pub fn current_round(&self) -> u32 {
        self.current_round
    }

    /// Returns the dungeon this encounter is in.
    pub fn dungeon(&self) -> Dungeon {
        self.dungeon
    }

    /// Returns the actor's combat side (ally or enemy).
    ///
    /// Returns `None` if the actor ID is not in the side lookup.
    pub fn actor_side(&self) -> Option<CombatSide> {
        self.side_lookup.get(&self.actor_id).copied()
    }
}

/// Check if an actor has an active status of the given kind.
fn has_status_kind(actor: &ActorAggregate, kind: &str) -> bool {
    actor.statuses.active().iter().any(|s| s.kind.0 == kind)
}

#[cfg(test)]
mod tests {
    use super::*;
    use framework_combat::encounter::CombatSide;
    use framework_rules::attributes::{AttributeKey, AttributeValue, ATTR_HEALTH};

    fn make_test_context() -> (
        ConditionContext<'static>,
        HashMap<ActorId, ActorAggregate>,
        HashMap<ActorId, CombatSide>,
    ) {
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        // Ally hero with high stress
        let mut ally = ActorAggregate::new(ActorId(1));
        ally.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
        ally.set_base(
            AttributeKey::new(crate::content::actors::ATTR_MAX_HEALTH),
            AttributeValue(100.0),
        );
        ally.set_base(
            AttributeKey::new(ATTR_STRESS),
            AttributeValue(75.0),
        );
        actors.insert(ActorId(1), ally);
        side_lookup.insert(ActorId(1), CombatSide::Ally);

        // Enemy monster with low stress
        let mut enemy = ActorAggregate::new(ActorId(2));
        enemy.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(50.0));
        enemy.set_base(
            AttributeKey::new(crate::content::actors::ATTR_MAX_HEALTH),
            AttributeValue(50.0),
        );
        actors.insert(ActorId(2), enemy);
        side_lookup.insert(ActorId(2), CombatSide::Enemy);

        let ctx = ConditionContext::new(
            ActorId(1),
            vec![ActorId(2)],
            0, // first round
            &actors,
            &side_lookup,
            Dungeon::QingLong,
        );

        (ctx, actors, side_lookup)
    }

    #[test]
    fn is_first_round_on_round_0() {
        let (ctx, _, _) = make_test_context();
        assert!(ctx.is_first_round());
    }

    #[test]
    fn is_not_first_round_after_round_0() {
        let (ctx, actors, side_lookup) = make_test_context();
        let ctx = ConditionContext::new(
            ActorId(1),
            vec![ActorId(2)],
            1, // round 1, not first round
            &actors,
            &side_lookup,
            Dungeon::QingLong,
        );
        assert!(!ctx.is_first_round());
    }

    #[test]
    fn actor_stress_above_threshold() {
        let (ctx, _, _) = make_test_context();
        assert!(ctx.actor_stress_above(50.0));
        assert!(ctx.actor_stress_above(70.0));
        assert!(!ctx.actor_stress_above(80.0));
    }

    #[test]
    fn actor_stress_below_threshold() {
        let (ctx, _, _) = make_test_context();
        assert!(ctx.actor_stress_below(80.0));
        assert!(ctx.actor_stress_below(100.0));
        assert!(!ctx.actor_stress_below(70.0));
    }

    #[test]
    fn stress_conditions_fail_for_monsters() {
        let (ctx, actors, side_lookup) = make_test_context();
        // Actor 2 is a monster
        let ctx = ConditionContext::new(
            ActorId(2), // monster
            vec![],
            0,
            &actors,
            &side_lookup,
            Dungeon::QingLong,
        );
        // Monsters should fail stress conditions regardless of their (non-existent) stress
        assert!(!ctx.actor_stress_above(0.0));
        assert!(!ctx.actor_stress_below(1000.0));
    }

    #[test]
    fn actor_hp_fraction_calculates_correctly() {
        let (ctx, _, _) = make_test_context();
        // Actor 1 has 100/100 HP = 1.0
        assert!((ctx.actor_hp_fraction() - 1.0).abs() < 0.001);
    }

    #[test]
    fn actor_at_deaths_door_when_low_hp() {
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        let mut actor = ActorAggregate::new(ActorId(1));
        actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(20.0));
        actor.set_base(
            AttributeKey::new(crate::content::actors::ATTR_MAX_HEALTH),
            AttributeValue(100.0),
        );
        actors.insert(ActorId(1), actor);
        side_lookup.insert(ActorId(1), CombatSide::Ally);

        let ctx = ConditionContext::new(
            ActorId(1),
            vec![],
            0,
            &actors,
            &side_lookup,
            Dungeon::QingLong,
        );

        // 20/100 = 0.2 < 0.5, so at deaths door
        assert!(ctx.actor_at_deaths_door());
    }

    #[test]
    fn not_at_deaths_door_when_healthy() {
        let (ctx, _, _) = make_test_context();
        // Actor 1 has 100/100 HP, not at deaths door
        assert!(!ctx.actor_at_deaths_door());
    }

    #[test]
    fn target_hp_fraction_calculates_correctly() {
        let (ctx, _, _) = make_test_context();
        // Target 2 has 50/50 HP = 1.0
        assert!((ctx.target_hp_fraction() - 1.0).abs() < 0.001);
    }

    #[test]
    fn target_has_status_checks_target_status() {
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        // Target has bleed status
        let mut target = ActorAggregate::new(ActorId(2));
        target.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(50.0));
        target
            .statuses
            .attach(crate::content::statuses::bleed(5.0, 3));
        actors.insert(ActorId(2), target);
        side_lookup.insert(ActorId(2), CombatSide::Enemy);

        // Actor is the ally
        let mut actor = ActorAggregate::new(ActorId(1));
        actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
        actors.insert(ActorId(1), actor);
        side_lookup.insert(ActorId(1), CombatSide::Ally);

        let ctx = ConditionContext::new(
            ActorId(1),
            vec![ActorId(2)], // targeting the enemy with bleed
            0,
            &actors,
            &side_lookup,
            Dungeon::QingLong,
        );

        assert!(ctx.target_has_status("bleed"));
        assert!(!ctx.target_has_status("stun"));
    }

    #[test]
    fn actor_has_status_checks_actor_status() {
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        let mut actor = ActorAggregate::new(ActorId(1));
        actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
        actor.statuses.attach(crate::content::statuses::stun(2));
        actors.insert(ActorId(1), actor);
        side_lookup.insert(ActorId(1), CombatSide::Ally);

        let ctx = ConditionContext::new(
            ActorId(1),
            vec![],
            0,
            &actors,
            &side_lookup,
            Dungeon::QingLong,
        );

        assert!(ctx.actor_has_status("stun"));
        assert!(!ctx.actor_has_status("bleed"));
    }

    #[test]
    fn context_is_deterministic() {
        // Creating the same context twice should yield identical results
        let (ctx1, _, _) = make_test_context();
        let (ctx2, _, _) = make_test_context();

        assert_eq!(ctx1.actor_id(), ctx2.actor_id());
        assert_eq!(ctx1.target_ids(), ctx2.target_ids());
        assert_eq!(ctx1.current_round(), ctx2.current_round());
        assert_eq!(ctx1.dungeon(), ctx2.dungeon());
        assert_eq!(ctx1.actor_stress(), ctx2.actor_stress());
        assert_eq!(ctx1.actor_hp_fraction(), ctx2.actor_hp_fraction());
    }
}
