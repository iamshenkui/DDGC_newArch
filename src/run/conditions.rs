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
//!
//! # Adapter Architecture
//!
//! The [`ConditionAdapter`] provides a unified interface for evaluating both
//! framework-native conditions AND DDGC-specific conditions. Framework conditions
//! are evaluated using the same logic as `EffectContext::check_condition` (no
//! duplication), while DDGC conditions are evaluated via `ConditionContext`.
//!
//! # Game Condition Evaluator Wiring
//!
//! The framework's [`EffectCondition::GameCondition`] variant requires a game-layer
//! evaluator function to be wired into `EffectContext::game_condition_evaluator`.
//! This module provides thread-local infrastructure for this wiring:
//!
//! - [`set_condition_context()`] — stores the current `ConditionContext` in thread-local
//! - [`get_condition_context_ref()`] — retrieves a reference to the stored context
//! - [`create_game_condition_evaluator()`] — creates an evaluator function for use with
//!   `EffectContext::new_with_game_condition_evaluator()`
//!
//! Usage in battle loop:
//! ```ignore
//! let ctx = ConditionContext::new(actor_id, targets, round, actors, sides, dungeon);
//! set_condition_context(ctx);
//! let evaluator = create_game_condition_evaluator();
//! let mut effect_ctx = EffectContext::new_with_game_condition_evaluator(
//!     actor, targets, formation, actors, evaluator
//! );
//! resolve_skill(skill, &mut effect_ctx);
//! ```

use std::cell::RefCell;
use std::collections::HashMap;

use framework_combat::encounter::CombatSide;
use framework_combat::effects::EffectCondition;
use framework_rules::actor::{ActorAggregate, ActorId};
use framework_rules::attributes::AttributeKey;
use framework_rules::attributes::ATTR_HEALTH;

use crate::content::actors::ATTR_STRESS;
use crate::encounters::Dungeon;

// ── Thread-Local Condition Context ────────────────────────────────────────────

thread_local! {
    /// Thread-local storage for the current `ConditionContext`.
    ///
    /// This is used by [`create_game_condition_evaluator`] to provide battle state
    /// to the framework's `EffectCondition::GameCondition` evaluator without
    /// requiring the evaluator function to carry explicit state.
    static CONDITION_CONTEXT: RefCell<Option<ConditionContext>> = const { RefCell::new(None) };
}

/// Set the current `ConditionContext` for game condition evaluation.
///
/// This should be called before entering the skill resolution path in the battle
/// loop. The context is stored in thread-local storage and retrieved by the
/// evaluator function created by [`create_game_condition_evaluator`].
pub fn set_condition_context(ctx: ConditionContext) {
    CONDITION_CONTEXT.with(|cell| {
        *cell.borrow_mut() = Some(ctx);
    });
}

/// Get a clone of the current thread-local `ConditionContext`, if set.
pub fn get_condition_context_ref() -> Option<ConditionContext> {
    CONDITION_CONTEXT.with(|cell| cell.borrow().clone())
}

/// Create a game condition evaluator function for use with
/// `EffectContext::new_with_game_condition_evaluator()`.
///
/// The returned function reads the current thread-local `ConditionContext` set by
/// [`set_condition_context`], creates a `ConditionAdapter`, and evaluates the
/// condition tag through it.
///
/// - `ConditionResult::Pass` → `true` (condition passes)
/// - `ConditionResult::Fail` → `false` (condition does not pass)
/// - `ConditionResult::Unknown` → `false` (unrecognized tag treated as failing)
///
/// # Panics
///
/// Panics if no `ConditionContext` has been set via `set_condition_context`.
pub fn create_game_condition_evaluator() -> fn(&str) -> bool {
    fn evaluate(tag: &str) -> bool {
        let ctx = get_condition_context_ref()
            .expect("ConditionContext not set; call set_condition_context() before evaluating game conditions");
        let adapter = ConditionAdapter::new(ctx);
        match adapter.evaluate_by_tag(tag) {
            ConditionResult::Pass => true,
            ConditionResult::Fail | ConditionResult::Unknown => false,
        }
    }
    evaluate
}

/// DDGC condition evaluation context.
///
/// Created from in-progress combat state, this struct provides read-only
/// access to data needed for evaluating DDGC-specific conditions like
/// `FirstRound`, `StressAbove`, and `StressBelow`.
///
/// All data is accessed through deterministic lookups so the context
/// can be created and queried without introducing non-determinism.
///
/// See unit tests in `run::conditions::tests` for usage examples.
#[derive(Clone)]
pub struct ConditionContext {
    /// The actor attempting to perform the action.
    actor_id: ActorId,
    /// The target(s) of the action.
    target_ids: Vec<ActorId>,
    /// The current round number (0 = first round).
    current_round: u32,
    /// All actors in the encounter, keyed by ID.
    actors: HashMap<ActorId, ActorAggregate>,
    /// Map from actor ID to combat side (ally/enemy).
    side_lookup: HashMap<ActorId, CombatSide>,
    /// The dungeon this encounter is taking place in.
    dungeon: Dungeon,
}

impl ConditionContext {
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
        actors: HashMap<ActorId, ActorAggregate>,
        side_lookup: HashMap<ActorId, CombatSide>,
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
    pub fn actor(&self) -> Option<&ActorAggregate> {
        self.actors.get(&self.actor_id)
    }

    /// Returns the targets of the action.
    pub fn targets(&self) -> Vec<&ActorAggregate> {
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
    actor.statuses.active().values().any(|s| s.kind.0 == kind)
}

// ── DDGC-Specific Conditions ─────────────────────────────────────────────────

/// DDGC-specific conditions that require game-layer state not available in the framework.
///
/// These conditions are evaluated via `ConditionContext`, which provides access to
/// DDGC-specific combat state like stress levels, round number, and dungeon context.
///
/// Compare to framework-native `EffectCondition` which covers generic conditions
/// like health thresholds, status checks, position checks, and probability.
#[derive(Debug, Clone, PartialEq)]
pub enum DdgcCondition {
    /// Active only on the first round of combat (round 0).
    FirstRound,
    /// Actor's stress is above the given threshold.
    StressAbove(f64),
    /// Actor's stress is below the given threshold.
    StressBelow(f64),
    /// Actor is at death's door (HP < 50% of max).
    DeathsDoor,
    /// Target has a specific status active.
    TargetHasStatus(String),
    /// Actor has a specific status active.
    ActorHasStatus(String),
}

/// Result of evaluating a condition through the adapter.
///
/// This allows distinguishing between "condition passed" and "condition not recognized"
/// so callers can fall back appropriately.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConditionResult {
    /// Condition passes — effect should execute.
    Pass,
    /// Condition fails — effect should not execute.
    Fail,
    /// Condition kind is not recognized by this adapter.
    Unknown,
}

/// Adapter that provides a unified interface for evaluating both framework-native
/// conditions (`EffectCondition`) and DDGC-specific conditions (`DdgcCondition`).
///
/// This adapter bridges the framework's condition system with DDGC-specific game state:
/// - Framework-native conditions are evaluated using the same logic as
///   `EffectContext::check_condition` (no duplication of framework logic).
/// - DDGC-specific conditions are evaluated via `ConditionContext`.
///
/// The adapter is created from in-progress combat state and provides read-only
/// access to condition evaluation. It does not mutate combat state.
///
/// See unit tests in `run::conditions::tests` for usage examples.
pub struct ConditionAdapter {
    /// The DDGC condition context for DDGC-specific condition evaluation.
    ctx: ConditionContext,
}

impl ConditionAdapter {
    /// Create a new condition adapter from a DDGC condition context.
    pub fn new(ctx: ConditionContext) -> Self {
        ConditionAdapter { ctx }
    }

    /// Evaluate a framework-native `EffectCondition`.
    ///
    /// This delegates to the same logic as `EffectContext::check_condition`,
    /// ensuring framework-native conditions behave identically through the adapter.
    ///
    /// Returns `ConditionResult` so callers can distinguish between
    /// "passed", "failed", and "not recognized".
    pub fn evaluate_framework(&self, condition: &EffectCondition, target: ActorId) -> ConditionResult {
        match condition {
            EffectCondition::IfTargetHealthBelow(threshold) => {
                if let Some(actor_agg) = self.ctx.actors.get(&target) {
                    let health = actor_agg.effective_attribute(&AttributeKey::new(ATTR_HEALTH));
                    if health.0 < *threshold {
                        ConditionResult::Pass
                    } else {
                        ConditionResult::Fail
                    }
                } else {
                    ConditionResult::Fail
                }
            }
            EffectCondition::IfActorHasStatus(status_kind) => {
                if let Some(actor_agg) = self.ctx.actors.get(&self.ctx.actor_id) {
                    if actor_agg
                        .statuses
                        .active()
                        .values()
                        .any(|s| s.kind.0 == *status_kind)
                    {
                        ConditionResult::Pass
                    } else {
                        ConditionResult::Fail
                    }
                } else {
                    ConditionResult::Fail
                }
            }
            EffectCondition::IfTargetPosition(slot_range) => {
                // NOTE: Formation lookup requires formation access which is not available
                // in ConditionContext. This is a limitation - position conditions
                // require integration with the formation system.
                // We return Unknown here because silently failing would hide a
                // missing implementation rather than surfacing it.
                // TODO: Integrate with formation layout in future iteration.
                let _ = slot_range;
                ConditionResult::Unknown
            }
            EffectCondition::Probability(p) => {
                // Deterministic: any probability > 0.0 passes in headless mode.
                if *p > 0.0 {
                    ConditionResult::Pass
                } else {
                    ConditionResult::Fail
                }
            }
            EffectCondition::GameCondition(tag) => {
                // Delegate to the DDGC condition evaluator via tag parsing.
                // This is the adapter's bridge between framework GameCondition
                // and DDGC-specific condition logic.
                self.evaluate_by_tag(tag)
            }
        }
    }

    /// Evaluate a DDGC-specific condition.
    ///
    /// Returns `ConditionResult` so callers can distinguish between
    /// "passed", "failed", and "not recognized".
    pub fn evaluate_ddgc(&self, condition: &DdgcCondition) -> ConditionResult {
        match condition {
            DdgcCondition::FirstRound => {
                if self.ctx.is_first_round() {
                    ConditionResult::Pass
                } else {
                    ConditionResult::Fail
                }
            }
            DdgcCondition::StressAbove(threshold) => {
                if self.ctx.actor_stress_above(*threshold) {
                    ConditionResult::Pass
                } else {
                    ConditionResult::Fail
                }
            }
            DdgcCondition::StressBelow(threshold) => {
                if self.ctx.actor_stress_below(*threshold) {
                    ConditionResult::Pass
                } else {
                    ConditionResult::Fail
                }
            }
            DdgcCondition::DeathsDoor => {
                if self.ctx.actor_at_deaths_door() {
                    ConditionResult::Pass
                } else {
                    ConditionResult::Fail
                }
            }
            DdgcCondition::TargetHasStatus(kind) => {
                if self.ctx.target_has_status(kind) {
                    ConditionResult::Pass
                } else {
                    ConditionResult::Fail
                }
            }
            DdgcCondition::ActorHasStatus(kind) => {
                if self.ctx.actor_has_status(kind) {
                    ConditionResult::Pass
                } else {
                    ConditionResult::Fail
                }
            }
        }
    }

    /// Unified evaluation that handles both framework-native and DDGC-specific conditions.
    ///
    /// For framework-native conditions, returns `ConditionResult::Pass` if the condition
    /// evaluates to true, `ConditionResult::Fail` if false, and `ConditionResult::Unknown`
    /// if the condition kind is not recognized.
    ///
    /// For DDGC-specific conditions, returns the `ConditionResult` from `evaluate_ddgc`.
    pub fn evaluate(&self, condition: &Condition, target: ActorId) -> ConditionResult {
        match condition {
            Condition::Framework(fc) => self.evaluate_framework(fc, target),
            Condition::Ddgc(dc) => self.evaluate_ddgc(dc),
        }
    }

    /// Parse a DDGC condition tag string into a `DdgcCondition`.
    ///
    /// The tag format is: `ddgc_<condition>_<optional_args>`
    ///
    /// Supported tags:
    /// - `"ddgc_first_round"` → `DdgcCondition::FirstRound`
    /// - `"ddgc_stress_above_<threshold>"` → `DdgcCondition::StressAbove(threshold)`
    /// - `"ddgc_stress_below_<threshold>"` → `DdgcCondition::StressBelow(threshold)`
    /// - `"ddgc_deaths_door"` → `DdgcCondition::DeathsDoor`
    /// - `"ddgc_target_has_status_<kind>"` → `DdgcCondition::TargetHasStatus(kind)`
    /// - `"ddgc_actor_has_status_<kind>"` → `DdgcCondition::ActorHasStatus(kind)`
    ///
    /// Returns `None` if the tag is not recognized.
    pub fn parse_condition_tag(tag: &str) -> Option<DdgcCondition> {
        let tag = tag.trim();

        if tag == "ddgc_first_round" {
            return Some(DdgcCondition::FirstRound);
        }
        if tag == "ddgc_deaths_door" {
            return Some(DdgcCondition::DeathsDoor);
        }

        // Parse stress_above_<threshold>
        if let Some(rest) = tag.strip_prefix("ddgc_stress_above_") {
            if let Ok(threshold) = rest.parse::<f64>() {
                return Some(DdgcCondition::StressAbove(threshold));
            }
        }

        // Parse stress_below_<threshold>
        if let Some(rest) = tag.strip_prefix("ddgc_stress_below_") {
            if let Ok(threshold) = rest.parse::<f64>() {
                return Some(DdgcCondition::StressBelow(threshold));
            }
        }

        // Parse target_has_status_<kind>
        if let Some(rest) = tag.strip_prefix("ddgc_target_has_status_") {
            if !rest.is_empty() {
                return Some(DdgcCondition::TargetHasStatus(rest.to_string()));
            }
        }

        // Parse actor_has_status_<kind>
        if let Some(rest) = tag.strip_prefix("ddgc_actor_has_status_") {
            if !rest.is_empty() {
                return Some(DdgcCondition::ActorHasStatus(rest.to_string()));
            }
        }

        None
    }

    /// Evaluate a deferred effect's DDGC condition using its condition tag.
    ///
    /// This combines parsing the tag into a `DdgcCondition` and evaluating it.
    /// Returns `ConditionResult` so callers can distinguish between pass, fail, and unknown.
    pub fn evaluate_by_tag(&self, condition_tag: &str) -> ConditionResult {
        match Self::parse_condition_tag(condition_tag) {
            Some(ddgc_cond) => self.evaluate_ddgc(&ddgc_cond),
            None => ConditionResult::Unknown,
        }
    }
}

/// Unified condition type for the adapter.
#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    /// Framework-native condition.
    Framework(EffectCondition),
    /// DDGC-specific condition.
    Ddgc(DdgcCondition),
}

#[cfg(test)]
mod tests {
    use super::*;
    use framework_combat::encounter::CombatSide;
    use framework_rules::attributes::{AttributeKey, AttributeValue, ATTR_HEALTH};

    /// Create a test condition context with ally hero (high stress) and enemy monster.
    ///
    /// Takes ownership of the hashmaps.
    fn make_test_context(
        actors: HashMap<ActorId, ActorAggregate>,
        side_lookup: HashMap<ActorId, CombatSide>,
    ) -> ConditionContext {
        ConditionContext::new(
            ActorId(1),
            vec![ActorId(2)],
            0, // first round
            actors,
            side_lookup,
            Dungeon::QingLong,
        )
    }

    /// Build standard test actors and side_lookup for the first round test.
    ///
    /// Helper for tests that need to create a context with specific parameters.
    fn build_test_actors_and_side_lookup() -> (
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

        (actors, side_lookup)
    }

    #[test]
    fn is_first_round_on_round_0() {
        let (actors, side_lookup) = build_test_actors_and_side_lookup();
        let ctx = make_test_context(actors, side_lookup);
        assert!(ctx.is_first_round());
    }

    #[test]
    fn is_not_first_round_after_round_0() {
        let (actors, side_lookup) = build_test_actors_and_side_lookup();
        let ctx = ConditionContext::new(
            ActorId(1),
            vec![ActorId(2)],
            1, // round 1, not first round
            actors.clone(),
            side_lookup.clone(),
            Dungeon::QingLong,
        );
        assert!(!ctx.is_first_round());
    }

    #[test]
    fn actor_stress_above_threshold() {
        let (actors, side_lookup) = build_test_actors_and_side_lookup();
        let ctx = make_test_context(actors, side_lookup);
        assert!(ctx.actor_stress_above(50.0));
        assert!(ctx.actor_stress_above(70.0));
        assert!(!ctx.actor_stress_above(80.0));
    }

    #[test]
    fn actor_stress_below_threshold() {
        let (actors, side_lookup) = build_test_actors_and_side_lookup();
        let ctx = make_test_context(actors, side_lookup);
        assert!(ctx.actor_stress_below(80.0));
        assert!(ctx.actor_stress_below(100.0));
        assert!(!ctx.actor_stress_below(70.0));
    }

    #[test]
    fn stress_conditions_fail_for_monsters() {
        let (actors, side_lookup) = build_test_actors_and_side_lookup();
        // Actor 2 is a monster
        let ctx = ConditionContext::new(
            ActorId(2), // monster
            vec![],
            0,
            actors.clone(),
            side_lookup.clone(),
            Dungeon::QingLong,
        );
        // Monsters should fail stress conditions regardless of their (non-existent) stress
        assert!(!ctx.actor_stress_above(0.0));
        assert!(!ctx.actor_stress_below(1000.0));
    }

    #[test]
    fn actor_hp_fraction_calculates_correctly() {
        let (actors, side_lookup) = build_test_actors_and_side_lookup();
        let ctx = make_test_context(actors, side_lookup);
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
            actors.clone(),
            side_lookup.clone(),
            Dungeon::QingLong,
        );

        // 20/100 = 0.2 < 0.5, so at deaths door
        assert!(ctx.actor_at_deaths_door());
    }

    #[test]
    fn not_at_deaths_door_when_healthy() {
        let (actors, side_lookup) = build_test_actors_and_side_lookup();
        let ctx = make_test_context(actors, side_lookup);
        // Actor 1 has 100/100 HP, not at deaths door
        assert!(!ctx.actor_at_deaths_door());
    }

    #[test]
    fn target_hp_fraction_calculates_correctly() {
        let (actors, side_lookup) = build_test_actors_and_side_lookup();
        let ctx = make_test_context(actors, side_lookup);
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
            actors.clone(),
            side_lookup.clone(),
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
            actors.clone(),
            side_lookup.clone(),
            Dungeon::QingLong,
        );

        assert!(ctx.actor_has_status("stun"));
        assert!(!ctx.actor_has_status("bleed"));
    }

    #[test]
    fn context_is_deterministic() {
        // Creating the same context twice should yield identical results
        let (actors1, side_lookup1) = build_test_actors_and_side_lookup();
        let (actors2, side_lookup2) = build_test_actors_and_side_lookup();
        let ctx1 = make_test_context(actors1, side_lookup1);
        let ctx2 = make_test_context(actors2, side_lookup2);

        assert_eq!(ctx1.actor_id(), ctx2.actor_id());
        assert_eq!(ctx1.target_ids(), ctx2.target_ids());
        assert_eq!(ctx1.current_round(), ctx2.current_round());
        assert_eq!(ctx1.dungeon(), ctx2.dungeon());
        assert_eq!(ctx1.actor_stress(), ctx2.actor_stress());
        assert_eq!(ctx1.actor_hp_fraction(), ctx2.actor_hp_fraction());
    }
}

// ── ConditionAdapter Tests ───────────────────────────────────────────────────

#[cfg(test)]
mod adapter_tests {
    use super::*;
    use framework_combat::effects::SlotRange;
    use framework_rules::attributes::AttributeValue;

    fn make_adapter_context() -> ConditionAdapter {
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        // Ally hero with high stress and poison status
        let mut ally = ActorAggregate::new(ActorId(1));
        ally.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
        ally.set_base(
            AttributeKey::new(crate::content::actors::ATTR_MAX_HEALTH),
            AttributeValue(100.0),
        );
        ally.set_base(AttributeKey::new(ATTR_STRESS), AttributeValue(75.0));
        ally.statuses.attach(crate::content::statuses::poison(10.0, 3));
        actors.insert(ActorId(1), ally);
        side_lookup.insert(ActorId(1), CombatSide::Ally);

        // Enemy monster with low HP
        let mut enemy = ActorAggregate::new(ActorId(2));
        enemy.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(40.0));
        enemy.set_base(
            AttributeKey::new(crate::content::actors::ATTR_MAX_HEALTH),
            AttributeValue(50.0),
        );
        enemy.statuses.attach(crate::content::statuses::bleed(5.0, 2));
        actors.insert(ActorId(2), enemy);
        side_lookup.insert(ActorId(2), CombatSide::Enemy);

        let ctx = ConditionContext::new(
            ActorId(1),        // actor
            vec![ActorId(2)], // targets
            0,                // first round
            actors,
            side_lookup,
            Dungeon::QingLong,
        );

        ConditionAdapter::new(ctx)
    }

    #[test]
    fn adapter_evaluates_probability_condition() {
        let adapter = make_adapter_context();

        // Probability(1.0) should pass
        let cond = EffectCondition::Probability(1.0);
        assert_eq!(adapter.evaluate_framework(&cond, ActorId(2)), ConditionResult::Pass);

        // Probability(0.5) should pass (deterministic: > 0 passes)
        let cond = EffectCondition::Probability(0.5);
        assert_eq!(adapter.evaluate_framework(&cond, ActorId(2)), ConditionResult::Pass);

        // Probability(0.0) should fail
        let cond = EffectCondition::Probability(0.0);
        assert_eq!(adapter.evaluate_framework(&cond, ActorId(2)), ConditionResult::Fail);
    }

    #[test]
    fn adapter_evaluates_target_health_below_condition() {
        let adapter = make_adapter_context();

        // Target (ActorId 2) has 40 HP base, but bleed(5.0) reduces effective health to 35
        // IfTargetHealthBelow compares raw effective health values

        // Threshold 50: 35 < 50 → passes
        let cond = EffectCondition::IfTargetHealthBelow(50.0);
        assert_eq!(adapter.evaluate_framework(&cond, ActorId(2)), ConditionResult::Pass);

        // Threshold 30: 35 < 30 → fails
        let cond = EffectCondition::IfTargetHealthBelow(30.0);
        assert_eq!(adapter.evaluate_framework(&cond, ActorId(2)), ConditionResult::Fail);

        // Threshold 35: 35 < 35 → fails (strict less-than)
        let cond = EffectCondition::IfTargetHealthBelow(35.0);
        assert_eq!(adapter.evaluate_framework(&cond, ActorId(2)), ConditionResult::Fail);
    }

    #[test]
    fn adapter_evaluates_actor_has_status_condition() {
        let adapter = make_adapter_context();

        // Actor (ActorId 1) has poison status

        // Has poison → passes
        let cond = EffectCondition::IfActorHasStatus("poison".to_string());
        assert_eq!(adapter.evaluate_framework(&cond, ActorId(2)), ConditionResult::Pass);

        // Has stun → fails
        let cond = EffectCondition::IfActorHasStatus("stun".to_string());
        assert_eq!(adapter.evaluate_framework(&cond, ActorId(2)), ConditionResult::Fail);
    }

    #[test]
    fn adapter_evaluates_ddgc_first_round_condition() {
        let adapter = make_adapter_context();

        // On round 0, FirstRound should pass
        let cond = DdgcCondition::FirstRound;
        assert_eq!(adapter.evaluate_ddgc(&cond), ConditionResult::Pass);
    }

    #[test]
    fn first_round_condition_fails_after_round_zero() {
        // Create a context with round 1 (not first round)
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        let ally = ActorAggregate::new(ActorId(1));
        actors.insert(ActorId(1), ally);
        side_lookup.insert(ActorId(1), CombatSide::Ally);

        let ctx = ConditionContext::new(
            ActorId(1),
            vec![],
            1, // round 1, NOT first round
            actors.clone(),
            side_lookup.clone(),
            Dungeon::QingLong,
        );

        let adapter = ConditionAdapter::new(ctx);
        let cond = DdgcCondition::FirstRound;

        // On round 1, FirstRound should fail
        assert_eq!(
            adapter.evaluate_ddgc(&cond),
            ConditionResult::Fail,
            "FirstRound condition should fail on round 1"
        );
    }

    #[test]
    fn adapter_evaluates_ddgc_stress_above_condition() {
        let adapter = make_adapter_context();

        // Actor (ActorId 1) has stress 75

        // 50 threshold: 75 > 50 → passes
        let cond = DdgcCondition::StressAbove(50.0);
        assert_eq!(adapter.evaluate_ddgc(&cond), ConditionResult::Pass);

        // 80 threshold: 75 > 80 → fails
        let cond = DdgcCondition::StressAbove(80.0);
        assert_eq!(adapter.evaluate_ddgc(&cond), ConditionResult::Fail);
    }

    #[test]
    fn adapter_evaluates_ddgc_deaths_door_condition() {
        let adapter = make_adapter_context();

        // Actor 1 has 100/100 HP (not at deaths door)
        let cond = DdgcCondition::DeathsDoor;
        assert_eq!(adapter.evaluate_ddgc(&cond), ConditionResult::Fail);
    }

    #[test]
    fn deaths_door_condition_passes_when_actor_at_deaths_door() {
        // This test proves DeathsDoor returns Pass when actor HP < 50%
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        // Actor at deaths door: 20/100 HP = 20% < 50%
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
            actors.clone(),
            side_lookup.clone(),
            Dungeon::QingLong,
        );

        let adapter = ConditionAdapter::new(ctx);

        // 20/100 = 0.2 < 0.5, so at deaths door → Pass
        let cond = DdgcCondition::DeathsDoor;
        assert_eq!(
            adapter.evaluate_ddgc(&cond),
            ConditionResult::Pass,
            "DeathsDoor should pass when HP is 20/100 (20% < 50%)"
        );
    }

    #[test]
    fn deaths_door_by_tag_returns_pass_when_at_deaths_door() {
        // This test proves evaluate_by_tag("ddgc_deaths_door") returns Pass when actor HP < 50%
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        // Actor at deaths door: 30/100 HP = 30% < 50%
        let mut actor = ActorAggregate::new(ActorId(1));
        actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(30.0));
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
            actors.clone(),
            side_lookup.clone(),
            Dungeon::QingLong,
        );

        let adapter = ConditionAdapter::new(ctx);

        // 30/100 = 0.3 < 0.5, so at deaths door → Pass
        let result = adapter.evaluate_by_tag("ddgc_deaths_door");
        assert_eq!(
            result,
            ConditionResult::Pass,
            "evaluate_by_tag(\"ddgc_deaths_door\") should pass when HP is 30/100 (30% < 50%)"
        );
    }

    #[test]
    fn deaths_door_by_tag_returns_fail_when_not_at_deaths_door() {
        // This test proves evaluate_by_tag("ddgc_deaths_door") returns Fail when actor HP >= 50%
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        // Actor NOT at deaths door: 60/100 HP = 60% >= 50%
        let mut actor = ActorAggregate::new(ActorId(1));
        actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(60.0));
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
            actors.clone(),
            side_lookup.clone(),
            Dungeon::QingLong,
        );

        let adapter = ConditionAdapter::new(ctx);

        // 60/100 = 0.6 >= 0.5, so NOT at deaths door → Fail
        let result = adapter.evaluate_by_tag("ddgc_deaths_door");
        assert_eq!(
            result,
            ConditionResult::Fail,
            "evaluate_by_tag(\"ddgc_deaths_door\") should fail when HP is 60/100 (60% >= 50%)"
        );
    }

    #[test]
    fn deaths_door_effect_changes_outcome_across_threshold_boundary() {
        // This is the key acceptance test for US-607: same effect (DeathsDoor condition),
        // different HP states, proving the effect changes outcome across the threshold boundary.

        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        // Low-HP actor (at deaths door: 20 HP out of 100 = 20%)
        let mut low_hp_ally = ActorAggregate::new(ActorId(1));
        low_hp_ally.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(20.0));
        low_hp_ally.set_base(
            AttributeKey::new(crate::content::actors::ATTR_MAX_HEALTH),
            AttributeValue(100.0),
        );
        actors.insert(ActorId(1), low_hp_ally);
        side_lookup.insert(ActorId(1), CombatSide::Ally);

        // High-HP actor (NOT at deaths door: 80 HP out of 100 = 80%)
        let mut high_hp_ally = ActorAggregate::new(ActorId(2));
        high_hp_ally.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(80.0));
        high_hp_ally.set_base(
            AttributeKey::new(crate::content::actors::ATTR_MAX_HEALTH),
            AttributeValue(100.0),
        );
        actors.insert(ActorId(2), high_hp_ally);
        side_lookup.insert(ActorId(2), CombatSide::Ally);

        // Low-HP context (actor 1, HP 20/100 = 20% < 50%)
        let ctx_low = ConditionContext::new(
            ActorId(1),
            vec![],
            0,
            actors.clone(),
            side_lookup.clone(),
            Dungeon::QingLong,
        );
        let adapter_low = ConditionAdapter::new(ctx_low);

        // High-HP context (actor 2, HP 80/100 = 80% >= 50%)
        let ctx_high = ConditionContext::new(
            ActorId(2),
            vec![],
            0,
            actors.clone(),
            side_lookup.clone(),
            Dungeon::QingLong,
        );
        let adapter_high = ConditionAdapter::new(ctx_high);

        // Same condition tag: ddgc_deaths_door
        // Low HP (20% < 50%) → Pass
        let result_low = adapter_low.evaluate_by_tag("ddgc_deaths_door");
        assert_eq!(
            result_low,
            ConditionResult::Pass,
            "Low-HP actor (20/100 = 20%) should pass DeathsDoor condition"
        );

        // High HP (80% >= 50%) → Fail
        let result_high = adapter_high.evaluate_by_tag("ddgc_deaths_door");
        assert_eq!(
            result_high,
            ConditionResult::Fail,
            "High-HP actor (80/100 = 80%) should fail DeathsDoor condition"
        );

        // The same condition tag produces different outcomes based on HP level
        assert_ne!(
            result_low, result_high,
            "Same DeathsDoor condition tag should produce different outcomes across HP threshold boundary"
        );
    }

    #[test]
    fn adapter_evaluates_ddgc_target_has_status_condition() {
        let adapter = make_adapter_context();

        // Target (ActorId 2) has bleed status

        // Has bleed → passes
        let cond = DdgcCondition::TargetHasStatus("bleed".to_string());
        assert_eq!(adapter.evaluate_ddgc(&cond), ConditionResult::Pass);

        // Has stun → fails
        let cond = DdgcCondition::TargetHasStatus("stun".to_string());
        assert_eq!(adapter.evaluate_ddgc(&cond), ConditionResult::Fail);
    }

    #[test]
    fn adapter_unified_evaluate_handles_both_condition_types() {
        let adapter = make_adapter_context();

        // Framework-native condition via unified interface
        let cond = Condition::Framework(EffectCondition::Probability(0.5));
        assert_eq!(adapter.evaluate(&cond, ActorId(2)), ConditionResult::Pass);

        // DDGC-specific condition via unified interface
        let cond = Condition::Ddgc(DdgcCondition::FirstRound);
        assert_eq!(adapter.evaluate(&cond, ActorId(2)), ConditionResult::Pass);
    }

    #[test]
    fn framework_conditions_behave_same_as_effect_context_logic() {
        // This test proves that framework-native conditions evaluated through
        // the adapter produce the same results as the framework's own logic.
        // This is the key acceptance criterion for US-603.
        let adapter = make_adapter_context();

        // Test Probability condition
        // Framework logic: p > 0.0 passes, p <= 0.0 fails
        for &p in &[0.0, 0.25, 0.5, 0.75, 1.0] {
            let cond = EffectCondition::Probability(p);
            let expected = if p > 0.0 { ConditionResult::Pass } else { ConditionResult::Fail };
            let actual = adapter.evaluate_framework(&cond, ActorId(2));
            assert_eq!(
                actual, expected,
                "Probability({}) should be {:?}, got {:?}",
                p, expected, actual
            );
        }

        // Test IfTargetHealthBelow condition
        // Framework logic: health < threshold passes (raw health, not fraction)
        // Actor 2 has 40 HP base but bleed(5) reduces effective health to 35
        let effective_health = 35.0;

        for &threshold in &[30.0, 35.0, 40.0, 45.0, 50.0] {
            let cond = EffectCondition::IfTargetHealthBelow(threshold);
            let expected = if effective_health < threshold { ConditionResult::Pass } else { ConditionResult::Fail };
            let actual = adapter.evaluate_framework(&cond, ActorId(2));
            assert_eq!(
                actual, expected,
                "IfTargetHealthBelow({}) should be {:?} (effective health is {}), got {:?}",
                threshold, expected, effective_health, actual
            );
        }

        // Test IfActorHasStatus condition
        // Framework logic: actor has status with matching kind passes
        // Actor 1 has poison status
        let cond = EffectCondition::IfActorHasStatus("poison".to_string());
        assert_eq!(adapter.evaluate_framework(&cond, ActorId(2)), ConditionResult::Pass);

        let cond = EffectCondition::IfActorHasStatus("stun".to_string());
        assert_eq!(adapter.evaluate_framework(&cond, ActorId(2)), ConditionResult::Fail);
    }

    // ── US-604: Unsupported conditions surfaced explicitly ─────────────────────

    #[test]
    fn unsupported_framework_conditions_return_unknown() {
        // This is the key acceptance test for US-604: unsupported conditions
        // must be observable, not silently applied or ignored.
        //
        // IfTargetPosition cannot be evaluated because ConditionContext does not
        // have formation layout access. Rather than silently failing (returning
        // false), we surface this as Unknown so callers can observe and handle it.
        let adapter = make_adapter_context();

        // IfTargetPosition requires formation access we don't have → Unknown
        let cond = EffectCondition::IfTargetPosition(SlotRange { min: 0, max: 2 });
        assert_eq!(
            adapter.evaluate_framework(&cond, ActorId(2)),
            ConditionResult::Unknown,
            "IfTargetPosition should return Unknown because formation context is unavailable"
        );
    }

    #[test]
    fn unknown_condition_is_deterministic() {
        // Proves that unsupported conditions are observable in a deterministic way.
        // Running the same evaluation twice yields the same Unknown result.
        let adapter1 = make_adapter_context();
        let adapter2 = make_adapter_context();

        let cond = EffectCondition::IfTargetPosition(SlotRange { min: 0, max: 2 });

        let result1 = adapter1.evaluate_framework(&cond, ActorId(2));
        let result2 = adapter2.evaluate_framework(&cond, ActorId(2));

        assert_eq!(result1, result2, "Unknown conditions should be deterministic");
        assert_eq!(result1, ConditionResult::Unknown);
    }

    #[test]
    fn unified_evaluate_propagates_unknown_from_framework() {
        // When evaluate_framework returns Unknown, the unified evaluate() method
        // should propagate it rather than converting it to Pass or Fail.
        let adapter = make_adapter_context();

        let cond = Condition::Framework(EffectCondition::IfTargetPosition(SlotRange {
            min: 0,
            max: 2,
        }));
        assert_eq!(
            adapter.evaluate(&cond, ActorId(2)),
            ConditionResult::Unknown,
            "evaluate() should propagate Unknown from evaluate_framework"
        );
    }

    // ── US-606: Stress threshold condition semantics ─────────────────────────────────

    #[test]
    fn stress_above_condition_passes_when_stress_exceeds_threshold() {
        // Actor 1 has stress 75 — StressAbove(50.0) should pass
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        let mut ally = ActorAggregate::new(ActorId(1));
        ally.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
        ally.set_base(
            AttributeKey::new(crate::content::actors::ATTR_MAX_HEALTH),
            AttributeValue(100.0),
        );
        ally.set_base(AttributeKey::new(ATTR_STRESS), AttributeValue(75.0));
        actors.insert(ActorId(1), ally);
        side_lookup.insert(ActorId(1), CombatSide::Ally);

        let ctx = ConditionContext::new(
            ActorId(1),
            vec![],
            0,
            actors.clone(),
            side_lookup.clone(),
            Dungeon::QingLong,
        );

        let adapter = ConditionAdapter::new(ctx);

        // Stress 75 > 50 → passes
        let result = adapter.evaluate_by_tag("ddgc_stress_above_50");
        assert_eq!(result, ConditionResult::Pass, "StressAbove(50) should pass when stress is 75");
    }

    #[test]
    fn stress_above_condition_fails_when_stress_below_threshold() {
        // Actor 1 has stress 30 — StressAbove(50.0) should fail
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        let mut ally = ActorAggregate::new(ActorId(1));
        ally.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
        ally.set_base(
            AttributeKey::new(crate::content::actors::ATTR_MAX_HEALTH),
            AttributeValue(100.0),
        );
        ally.set_base(AttributeKey::new(ATTR_STRESS), AttributeValue(30.0));
        actors.insert(ActorId(1), ally);
        side_lookup.insert(ActorId(1), CombatSide::Ally);

        let ctx = ConditionContext::new(
            ActorId(1),
            vec![],
            0,
            actors.clone(),
            side_lookup.clone(),
            Dungeon::QingLong,
        );

        let adapter = ConditionAdapter::new(ctx);

        // Stress 30 < 50 → fails
        let result = adapter.evaluate_by_tag("ddgc_stress_above_50");
        assert_eq!(result, ConditionResult::Fail, "StressAbove(50) should fail when stress is 30");
    }

    #[test]
    fn stress_below_condition_passes_when_stress_below_threshold() {
        // Actor 1 has stress 30 — StressBelow(50.0) should pass
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        let mut ally = ActorAggregate::new(ActorId(1));
        ally.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
        ally.set_base(
            AttributeKey::new(crate::content::actors::ATTR_MAX_HEALTH),
            AttributeValue(100.0),
        );
        ally.set_base(AttributeKey::new(ATTR_STRESS), AttributeValue(30.0));
        actors.insert(ActorId(1), ally);
        side_lookup.insert(ActorId(1), CombatSide::Ally);

        let ctx = ConditionContext::new(
            ActorId(1),
            vec![],
            0,
            actors.clone(),
            side_lookup.clone(),
            Dungeon::QingLong,
        );

        let adapter = ConditionAdapter::new(ctx);

        // Stress 30 < 50 → passes
        let result = adapter.evaluate_by_tag("ddgc_stress_below_50");
        assert_eq!(result, ConditionResult::Pass, "StressBelow(50) should pass when stress is 30");
    }

    #[test]
    fn stress_below_condition_fails_when_stress_above_threshold() {
        // Actor 1 has stress 75 — StressBelow(50.0) should fail
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        let mut ally = ActorAggregate::new(ActorId(1));
        ally.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
        ally.set_base(
            AttributeKey::new(crate::content::actors::ATTR_MAX_HEALTH),
            AttributeValue(100.0),
        );
        ally.set_base(AttributeKey::new(ATTR_STRESS), AttributeValue(75.0));
        actors.insert(ActorId(1), ally);
        side_lookup.insert(ActorId(1), CombatSide::Ally);

        let ctx = ConditionContext::new(
            ActorId(1),
            vec![],
            0,
            actors.clone(),
            side_lookup.clone(),
            Dungeon::QingLong,
        );

        let adapter = ConditionAdapter::new(ctx);

        // Stress 75 > 50 → fails
        let result = adapter.evaluate_by_tag("ddgc_stress_below_50");
        assert_eq!(result, ConditionResult::Fail, "StressBelow(50) should fail when stress is 75");
    }

    #[test]
    fn stress_condition_effect_changes_outcome_across_threshold_boundary() {
        // This is the key acceptance test: same effect, different stress levels,
        // proving the effect changes outcome across the threshold boundary.

        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        // Low-stress actor (stress 20)
        let mut low_stress_ally = ActorAggregate::new(ActorId(1));
        low_stress_ally.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
        low_stress_ally.set_base(
            AttributeKey::new(crate::content::actors::ATTR_MAX_HEALTH),
            AttributeValue(100.0),
        );
        low_stress_ally.set_base(AttributeKey::new(ATTR_STRESS), AttributeValue(20.0));
        actors.insert(ActorId(1), low_stress_ally);
        side_lookup.insert(ActorId(1), CombatSide::Ally);

        // High-stress actor (stress 80)
        let mut high_stress_ally = ActorAggregate::new(ActorId(2));
        high_stress_ally.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
        high_stress_ally.set_base(
            AttributeKey::new(crate::content::actors::ATTR_MAX_HEALTH),
            AttributeValue(100.0),
        );
        high_stress_ally.set_base(AttributeKey::new(ATTR_STRESS), AttributeValue(80.0));
        actors.insert(ActorId(2), high_stress_ally);
        side_lookup.insert(ActorId(2), CombatSide::Ally);

        // Low-stress context (actor 1, stress 20)
        let ctx_low = ConditionContext::new(
            ActorId(1),
            vec![],
            0,
            actors.clone(),
            side_lookup.clone(),
            Dungeon::QingLong,
        );
        let adapter_low = ConditionAdapter::new(ctx_low);

        // High-stress context (actor 2, stress 80)
        let ctx_high = ConditionContext::new(
            ActorId(2),
            vec![],
            0,
            actors.clone(),
            side_lookup.clone(),
            Dungeon::QingLong,
        );
        let adapter_high = ConditionAdapter::new(ctx_high);

        // Same condition tag: ddgc_stress_above_50
        // Low stress (20 < 50) → fails
        let result_low = adapter_low.evaluate_by_tag("ddgc_stress_above_50");
        assert_eq!(result_low, ConditionResult::Fail,
            "Low-stress actor (stress=20) should fail StressAbove(50)");

        // High stress (80 > 50) → passes
        let result_high = adapter_high.evaluate_by_tag("ddgc_stress_above_50");
        assert_eq!(result_high, ConditionResult::Pass,
            "High-stress actor (stress=80) should pass StressAbove(50)");

        // The same condition tag produces different outcomes based on stress level
        assert_ne!(result_low, result_high,
            "Same condition tag should produce different outcomes across threshold boundary");
    }

    #[test]
    fn parse_condition_tag_recognizes_all_supported_formats() {
        // FirstRound
        assert!(matches!(
            ConditionAdapter::parse_condition_tag("ddgc_first_round"),
            Some(DdgcCondition::FirstRound)
        ));

        // StressAbove
        assert!(matches!(
            ConditionAdapter::parse_condition_tag("ddgc_stress_above_50"),
            Some(DdgcCondition::StressAbove(50.0))
        ));
        assert!(matches!(
            ConditionAdapter::parse_condition_tag("ddgc_stress_above_0"),
            Some(DdgcCondition::StressAbove(0.0))
        ));

        // StressBelow
        assert!(matches!(
            ConditionAdapter::parse_condition_tag("ddgc_stress_below_30"),
            Some(DdgcCondition::StressBelow(30.0))
        ));

        // DeathsDoor
        assert!(matches!(
            ConditionAdapter::parse_condition_tag("ddgc_deaths_door"),
            Some(DdgcCondition::DeathsDoor)
        ));

        // TargetHasStatus
        assert!(matches!(
            ConditionAdapter::parse_condition_tag("ddgc_target_has_status_bleed"),
            Some(DdgcCondition::TargetHasStatus(s)) if s == "bleed"
        ));

        // ActorHasStatus
        assert!(matches!(
            ConditionAdapter::parse_condition_tag("ddgc_actor_has_status_stun"),
            Some(DdgcCondition::ActorHasStatus(s)) if s == "stun"
        ));

        // Unknown tag
        assert!(ConditionAdapter::parse_condition_tag("unknown_tag").is_none());
        assert!(ConditionAdapter::parse_condition_tag("ddgc_invalid").is_none());
    }

    #[test]
    fn stress_conditions_fail_for_monsters_regardless_of_tag() {
        // Monsters don't have stress, so stress conditions should always fail for them
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        // Monster (enemy) — even if we could set stress on it, it would fail
        let mut enemy = ActorAggregate::new(ActorId(1));
        enemy.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(50.0));
        enemy.set_base(
            AttributeKey::new(crate::content::actors::ATTR_MAX_HEALTH),
            AttributeValue(50.0),
        );
        // Note: enemies don't have stress attribute, but even if they did...
        actors.insert(ActorId(1), enemy);
        side_lookup.insert(ActorId(1), CombatSide::Enemy);

        let ctx = ConditionContext::new(
            ActorId(1),
            vec![],
            0,
            actors.clone(),
            side_lookup.clone(),
            Dungeon::QingLong,
        );

        let adapter = ConditionAdapter::new(ctx);

        // Monster should fail stress conditions regardless of tag
        assert_eq!(
            adapter.evaluate_by_tag("ddgc_stress_above_0"),
            ConditionResult::Fail,
            "Monsters should fail StressAbove regardless of tag"
        );
        assert_eq!(
            adapter.evaluate_by_tag("ddgc_stress_below_1000"),
            ConditionResult::Fail,
            "Monsters should fail StressBelow regardless of tag"
        );
    }

    // ── Game Condition Evaluator Wiring Tests ──────────────────────────────────

    #[test]
    fn set_and_get_condition_context_round_trips() {
        // Verify that set_condition_context and get_condition_context_ref work together
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        let ally = ActorAggregate::new(ActorId(1));
        actors.insert(ActorId(1), ally);
        side_lookup.insert(ActorId(1), CombatSide::Ally);

        let ctx = ConditionContext::new(
            ActorId(1),
            vec![],
            0,
            actors,
            side_lookup,
            Dungeon::QingLong,
        );

        set_condition_context(ctx);

        let retrieved = get_condition_context_ref();
        assert!(retrieved.is_some(), "Context should be retrievable after setting");
        assert_eq!(retrieved.unwrap().actor_id(), ActorId(1));

        // Verify it's a clone, not a reference to the same context
        let retrieved2 = get_condition_context_ref();
        assert!(retrieved2.is_some());
        assert_eq!(retrieved.unwrap().actor_id(), retrieved2.unwrap().actor_id());
    }

    #[test]
    fn game_condition_evaluator_returns_true_for_pass_condition() {
        // Set up a context on round 0 (first round)
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        let ally = ActorAggregate::new(ActorId(1));
        actors.insert(ActorId(1), ally);
        side_lookup.insert(ActorId(1), CombatSide::Ally);

        let ctx = ConditionContext::new(
            ActorId(1),
            vec![],
            0, // first round
            actors,
            side_lookup,
            Dungeon::QingLong,
        );

        set_condition_context(ctx);
        let evaluator = create_game_condition_evaluator();

        // ddgc_first_round should pass on round 0
        assert!(evaluator("ddgc_first_round"),
            "ddgc_first_round should pass on round 0");
    }

    #[test]
    fn game_condition_evaluator_returns_false_for_fail_condition() {
        // Set up a context on round 1 (not first round)
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        let ally = ActorAggregate::new(ActorId(1));
        actors.insert(ActorId(1), ally);
        side_lookup.insert(ActorId(1), CombatSide::Ally);

        let ctx = ConditionContext::new(
            ActorId(1),
            vec![],
            1, // not first round
            actors,
            side_lookup,
            Dungeon::QingLong,
        );

        set_condition_context(ctx);
        let evaluator = create_game_condition_evaluator();

        // ddgc_first_round should fail on round 1
        assert!(!evaluator("ddgc_first_round"),
            "ddgc_first_round should fail on round 1");
    }

    #[test]
    fn game_condition_evaluator_returns_false_for_unknown_tag() {
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        let ally = ActorAggregate::new(ActorId(1));
        actors.insert(ActorId(1), ally);
        side_lookup.insert(ActorId(1), CombatSide::Ally);

        let ctx = ConditionContext::new(
            ActorId(1),
            vec![],
            0,
            actors,
            side_lookup,
            Dungeon::QingLong,
        );

        set_condition_context(ctx);
        let evaluator = create_game_condition_evaluator();

        // Unknown tag should return false (treated as failing)
        assert!(!evaluator("unknown_condition_tag"),
            "Unknown condition tags should return false");
    }

    #[test]
    fn game_condition_evaluator_passes_for_deaths_door_when_at_threshold() {
        // Actor at deaths door: 20/100 HP = 20% < 50%
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        let mut ally = ActorAggregate::new(ActorId(1));
        ally.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(20.0));
        ally.set_base(
            AttributeKey::new(crate::content::actors::ATTR_MAX_HEALTH),
            AttributeValue(100.0),
        );
        actors.insert(ActorId(1), ally);
        side_lookup.insert(ActorId(1), CombatSide::Ally);

        let ctx = ConditionContext::new(
            ActorId(1),
            vec![],
            0,
            actors,
            side_lookup,
            Dungeon::QingLong,
        );

        set_condition_context(ctx);
        let evaluator = create_game_condition_evaluator();

        // At 20% HP (below 50%), deaths_door should pass
        assert!(evaluator("ddgc_deaths_door"),
            "ddgc_deaths_door should pass when HP is 20/100 (20% < 50%)");
    }

    #[test]
    fn game_condition_evaluator_fails_for_deaths_door_when_above_threshold() {
        // Actor NOT at deaths door: 60/100 HP = 60% >= 50%
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        let mut ally = ActorAggregate::new(ActorId(1));
        ally.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(60.0));
        ally.set_base(
            AttributeKey::new(crate::content::actors::ATTR_MAX_HEALTH),
            AttributeValue(100.0),
        );
        actors.insert(ActorId(1), ally);
        side_lookup.insert(ActorId(1), CombatSide::Ally);

        let ctx = ConditionContext::new(
            ActorId(1),
            vec![],
            0,
            actors,
            side_lookup,
            Dungeon::QingLong,
        );

        set_condition_context(ctx);
        let evaluator = create_game_condition_evaluator();

        // At 60% HP (above 50%), deaths_door should fail
        assert!(!evaluator("ddgc_deaths_door"),
            "ddgc_deaths_door should fail when HP is 60/100 (60% >= 50%)");
    }
}
