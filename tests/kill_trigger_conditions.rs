//! Integration tests for kill-trigger condition evaluation (US-804-d).
//!
//! Validates that `ddgc_on_kill` condition tag is correctly parsed and
//! evaluated through the ConditionAdapter public API, and that the
//! game-layer kill event tracker correctly influences condition outcomes.

use game_ddgc_headless::run::conditions::{
    ConditionAdapter, ConditionContext, ConditionResult, DdgcCondition,
    create_game_condition_evaluator, set_condition_context,
};
use framework_combat::effects::EffectCondition;
use framework_combat::encounter::CombatSide;
use framework_rules::actor::{ActorAggregate, ActorId};
use game_ddgc_headless::encounters::Dungeon;

fn make_context_with_kills(kills: Vec<ActorId>) -> ConditionContext {
    let mut actors: std::collections::HashMap<ActorId, ActorAggregate> =
        std::collections::HashMap::new();
    let mut side_lookup: std::collections::HashMap<ActorId, CombatSide> =
        std::collections::HashMap::new();

    let actor = ActorAggregate::new(ActorId(1));
    actors.insert(ActorId(1), actor);
    side_lookup.insert(ActorId(1), CombatSide::Ally);

    ConditionContext::new_with_kills(
        ActorId(1),
        vec![],
        0,
        actors,
        side_lookup,
        Dungeon::QingLong,
        kills,
    )
}

// ── OnKill passes when kill occurred ─────────────────────────────────────────

#[test]
fn on_kill_passes_when_actor_killed_enemy_previous_turn() {
    let ctx = make_context_with_kills(vec![ActorId(2)]);
    let adapter = ConditionAdapter::new(ctx);

    assert_eq!(
        adapter.evaluate_by_tag("ddgc_on_kill"),
        ConditionResult::Pass,
        "OnKill should pass when actor killed an enemy on previous turn"
    );
}

// ── OnKill fails when no kill ─────────────────────────────────────────────────

#[test]
fn on_kill_fails_when_no_kill_previous_turn() {
    let ctx = make_context_with_kills(vec![]);
    let adapter = ConditionAdapter::new(ctx);

    assert_eq!(
        adapter.evaluate_by_tag("ddgc_on_kill"),
        ConditionResult::Fail,
        "OnKill should fail when actor did not kill on previous turn"
    );
}

// ── Determinism: multiple kills still pass ────────────────────────────────────

#[test]
fn on_kill_passes_when_actor_killed_multiple_enemies() {
    let ctx = make_context_with_kills(vec![ActorId(2), ActorId(3), ActorId(4)]);
    let adapter = ConditionAdapter::new(ctx);

    assert_eq!(
        adapter.evaluate_by_tag("ddgc_on_kill"),
        ConditionResult::Pass,
        "OnKill should pass when actor killed multiple enemies"
    );
}

// ── Boundary outcome change ───────────────────────────────────────────────────

#[test]
fn on_kill_condition_changes_outcome_based_on_kill() {
    let ctx_with_kill = make_context_with_kills(vec![ActorId(2)]);
    let ctx_without_kill = make_context_with_kills(vec![]);

    let result_with = ConditionAdapter::new(ctx_with_kill).evaluate_by_tag("ddgc_on_kill");
    let result_without = ConditionAdapter::new(ctx_without_kill).evaluate_by_tag("ddgc_on_kill");

    assert_eq!(result_with, ConditionResult::Pass);
    assert_eq!(result_without, ConditionResult::Fail);
    assert_ne!(result_with, result_without,
        "OnKill outcome must differ based on kill state");
}

// ── Tag parsing ──────────────────────────────────────────────────────────────

#[test]
fn parse_condition_tag_handles_ddgc_on_kill() {
    assert!(matches!(
        ConditionAdapter::parse_condition_tag("ddgc_on_kill"),
        Some(DdgcCondition::OnKill)
    ));
}

// ── Game condition evaluator wiring ──────────────────────────────────────────

#[test]
fn game_evaluator_wires_on_kill_correctly() {
    let ctx = make_context_with_kills(vec![ActorId(2)]);
    set_condition_context(ctx);
    let evaluator = create_game_condition_evaluator();

    assert!(evaluator("ddgc_on_kill"),
        "Evaluator should return true when actor killed on previous turn");
}

#[test]
fn game_evaluator_returns_false_when_no_kill() {
    let ctx = make_context_with_kills(vec![]);
    set_condition_context(ctx);
    let evaluator = create_game_condition_evaluator();

    assert!(!evaluator("ddgc_on_kill"),
        "Evaluator should return false when actor did not kill on previous turn");
}

// ── Fixture skill uses OnKill condition ───────────────────────────────────────

#[test]
fn executioner_strike_skill_uses_on_kill_condition() {
    // Verify that the executioner_strike fixture skill registers through
    // the skill pack and contains a ddgc_on_kill condition tag.
    let pack = game_ddgc_headless::content::heroes::hunter::skill_pack();
    let skill = pack.iter().find(|s| s.id.0.as_str() == "executioner_strike");
    assert!(skill.is_some(), "executioner_strike should be in Hunter skill_pack");

    let skill = skill.unwrap();

    // Verify the condition tag is parseable by the adapter
    assert!(
        ConditionAdapter::parse_condition_tag("ddgc_on_kill").is_some(),
        "ddgc_on_kill should parse as a valid OnKill condition"
    );

    // Verify the skill's effect nodes actually wire the GameCondition
    let has_on_kill_condition = skill.effects.iter().any(|e| {
        e.conditions.iter().any(|c| matches!(c, EffectCondition::GameCondition(tag) if tag == "ddgc_on_kill"))
    });
    assert!(
        has_on_kill_condition,
        "executioner_strike should have an effect node with GameCondition(\"ddgc_on_kill\")"
    );
}
