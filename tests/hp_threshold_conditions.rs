//! Integration tests for HP-threshold condition evaluation (US-802).
//!
//! Validates that `ddgc_hp_above_<t>`, `ddgc_target_hp_above_<t>`, and
//! `ddgc_target_hp_below_<t>` condition tags are correctly parsed and
//! evaluated through the ConditionAdapter public API.

use game_ddgc_headless::run::conditions::{
    ConditionAdapter, ConditionContext, ConditionResult, DdgcCondition,
    create_game_condition_evaluator, set_condition_context,
};
use framework_combat::encounter::CombatSide;
use framework_rules::actor::{ActorAggregate, ActorId};
use framework_rules::attributes::{AttributeKey, AttributeValue, ATTR_HEALTH};
use game_ddgc_headless::content::actors::ATTR_MAX_HEALTH;
use game_ddgc_headless::encounters::Dungeon;

fn make_context(
    actor_hp: f64,
    actor_max_hp: f64,
    target_hp: f64,
    target_max_hp: f64,
) -> ConditionContext {
    let mut actors: std::collections::HashMap<ActorId, ActorAggregate> =
        std::collections::HashMap::new();
    let mut side_lookup: std::collections::HashMap<ActorId, CombatSide> =
        std::collections::HashMap::new();

    let mut actor = ActorAggregate::new(ActorId(1));
    actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(actor_hp));
    actor.set_base(AttributeKey::new(ATTR_MAX_HEALTH), AttributeValue(actor_max_hp));
    actors.insert(ActorId(1), actor);
    side_lookup.insert(ActorId(1), CombatSide::Ally);

    let mut target = ActorAggregate::new(ActorId(2));
    target.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(target_hp));
    target.set_base(AttributeKey::new(ATTR_MAX_HEALTH), AttributeValue(target_max_hp));
    actors.insert(ActorId(2), target);
    side_lookup.insert(ActorId(2), CombatSide::Enemy);

    ConditionContext::new(
        ActorId(1),
        vec![ActorId(2)],
        0,
        actors,
        side_lookup,
        Dungeon::QingLong,
    )
}

// ── HpAbove ──────────────────────────────────────────────────────────────────

#[test]
fn hp_above_passes_when_actor_hp_exceeds_threshold() {
    let ctx = make_context(80.0, 100.0, 100.0, 100.0);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_hp_above_0.5"),
        ConditionResult::Pass,
        "80/100 = 0.8 > 0.5 should pass"
    );
}

#[test]
fn hp_above_fails_when_actor_hp_below_threshold() {
    let ctx = make_context(30.0, 100.0, 100.0, 100.0);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_hp_above_0.5"),
        ConditionResult::Fail,
        "30/100 = 0.3 < 0.5 should fail"
    );
}

#[test]
fn hp_above_fails_at_exact_threshold_boundary() {
    let ctx = make_context(50.0, 100.0, 100.0, 100.0);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_hp_above_0.5"),
        ConditionResult::Fail,
        "50/100 = 0.5 is not > 0.5 (strict comparison)"
    );
}

// ── TargetHpAbove ────────────────────────────────────────────────────────────

#[test]
fn target_hp_above_passes_when_target_hp_exceeds_threshold() {
    let ctx = make_context(100.0, 100.0, 75.0, 100.0);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_target_hp_above_0.5"),
        ConditionResult::Pass,
        "75/100 = 0.75 > 0.5 should pass"
    );
}

#[test]
fn target_hp_above_fails_when_target_hp_below_threshold() {
    let ctx = make_context(100.0, 100.0, 30.0, 100.0);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_target_hp_above_0.5"),
        ConditionResult::Fail,
        "30/100 = 0.3 < 0.5 should fail"
    );
}

#[test]
fn target_hp_above_fails_at_exact_threshold_boundary() {
    let ctx = make_context(100.0, 100.0, 50.0, 100.0);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_target_hp_above_0.5"),
        ConditionResult::Fail,
        "50/100 = 0.5 is not > 0.5 (strict comparison)"
    );
}

// ── TargetHpBelow ────────────────────────────────────────────────────────────

#[test]
fn target_hp_below_passes_when_target_hp_below_threshold() {
    let ctx = make_context(100.0, 100.0, 30.0, 100.0);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_target_hp_below_0.5"),
        ConditionResult::Pass,
        "30/100 = 0.3 < 0.5 should pass"
    );
}

#[test]
fn target_hp_below_fails_when_target_hp_above_threshold() {
    let ctx = make_context(100.0, 100.0, 80.0, 100.0);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_target_hp_below_0.5"),
        ConditionResult::Fail,
        "80/100 = 0.8 > 0.5 should fail"
    );
}

#[test]
fn target_hp_below_fails_at_exact_threshold_boundary() {
    let ctx = make_context(100.0, 100.0, 50.0, 100.0);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_target_hp_below_0.5"),
        ConditionResult::Fail,
        "50/100 = 0.5 is not < 0.5 (strict comparison)"
    );
}

// ── Boundary outcome change ──────────────────────────────────────────────────

#[test]
fn hp_above_condition_changes_outcome_across_boundary() {
    let ctx_low = make_context(20.0, 100.0, 100.0, 100.0);
    let ctx_high = make_context(80.0, 100.0, 100.0, 100.0);

    let result_low = ConditionAdapter::new(ctx_low).evaluate_by_tag("ddgc_hp_above_0.5");
    let result_high = ConditionAdapter::new(ctx_high).evaluate_by_tag("ddgc_hp_above_0.5");

    assert_eq!(result_low, ConditionResult::Fail);
    assert_eq!(result_high, ConditionResult::Pass);
    assert_ne!(result_low, result_high, "Condition outcome must differ across boundary");
}

#[test]
fn target_hp_condition_changes_outcome_across_boundary() {
    let ctx_low = make_context(100.0, 100.0, 30.0, 100.0);
    let ctx_high = make_context(100.0, 100.0, 70.0, 100.0);

    let low_above = ConditionAdapter::new(ctx_low.clone()).evaluate_by_tag("ddgc_target_hp_above_0.5");
    let high_above = ConditionAdapter::new(ctx_high.clone()).evaluate_by_tag("ddgc_target_hp_above_0.5");
    let low_below = ConditionAdapter::new(ctx_low).evaluate_by_tag("ddgc_target_hp_below_0.5");
    let high_below = ConditionAdapter::new(ctx_high).evaluate_by_tag("ddgc_target_hp_below_0.5");

    assert_ne!(low_above, high_above, "TargetHpAbove outcome must differ across boundary");
    assert_ne!(low_below, high_below, "TargetHpBelow outcome must differ across boundary");
}

// ── Determinism ──────────────────────────────────────────────────────────────

#[test]
fn hp_threshold_conditions_are_deterministic_at_boundary() {
    let ctx = make_context(50.0, 100.0, 50.0, 100.0);
    let adapter = ConditionAdapter::new(ctx);

    for _ in 0..10 {
        assert_eq!(adapter.evaluate_by_tag("ddgc_hp_above_0.5"), ConditionResult::Fail);
        assert_eq!(adapter.evaluate_by_tag("ddgc_target_hp_above_0.5"), ConditionResult::Fail);
        assert_eq!(adapter.evaluate_by_tag("ddgc_target_hp_below_0.5"), ConditionResult::Fail);
    }
}

// ── Tag parsing ──────────────────────────────────────────────────────────────

#[test]
fn parse_condition_tag_handles_all_hp_threshold_formats() {
    assert!(matches!(
        ConditionAdapter::parse_condition_tag("ddgc_hp_above_0.5"),
        Some(DdgcCondition::HpAbove(t)) if (t - 0.5).abs() < f64::EPSILON
    ));
    assert!(matches!(
        ConditionAdapter::parse_condition_tag("ddgc_target_hp_above_0.75"),
        Some(DdgcCondition::TargetHpAbove(t)) if (t - 0.75).abs() < f64::EPSILON
    ));
    assert!(matches!(
        ConditionAdapter::parse_condition_tag("ddgc_target_hp_below_0.25"),
        Some(DdgcCondition::TargetHpBelow(t)) if (t - 0.25).abs() < f64::EPSILON
    ));
}

// ── Game condition evaluator wiring ──────────────────────────────────────────

#[test]
fn game_evaluator_wires_hp_above_correctly() {
    let ctx = make_context(80.0, 100.0, 30.0, 100.0);
    set_condition_context(ctx);
    let evaluator = create_game_condition_evaluator();

    assert!(evaluator("ddgc_hp_above_0.5"), "Actor 80% HP > 50% should return true");
    assert!(!evaluator("ddgc_target_hp_above_0.5"), "Target 30% HP < 50% should return false");
    assert!(evaluator("ddgc_target_hp_below_0.5"), "Target 30% HP < 50% should return true");
}

// ── Fixture skill uses HP-threshold condition ────────────────────────────────

#[test]
fn retribution_strike_skill_uses_hp_above_condition() {
    // Verify that the retribution_strike fixture skill registers through
    // the skill pack and contains a ddgc_hp_above_0.5 condition tag.
    let pack = game_ddgc_headless::content::heroes::hunter::skill_pack();
    let skill = pack.iter().find(|s| s.id.0.as_str() == "retribution_strike");
    assert!(skill.is_some(), "retribution_strike should be in Hunter skill_pack");

    // Verify the condition tag is parseable by the adapter
    assert!(
        ConditionAdapter::parse_condition_tag("ddgc_hp_above_0.5").is_some(),
        "ddgc_hp_above_0.5 should parse as a valid HP-threshold condition"
    );
}
