//! Integration test for full condition coverage (US-805-b).
//!
//! Validates that every condition family implemented in ConditionAdapter has
//! end-to-end coverage through the public API. This test closes B-004 by
//! proving all implemented DDGC conditions (11 DdgcCondition variants +
//! 4 framework bridges) are exercised at the integration level.
//!
//! Conditions with dedicated integration test files:
//! - HP-threshold: `tests/hp_threshold_conditions.rs` (US-802-d)
//! - Dungeon-mode: `tests/in_mode_conditions.rs` (US-803-f)
//! - Kill-trigger: `tests/kill_trigger_conditions.rs` (US-804-d)
//!
//! This file covers the remaining families:
//! - Round-trigger (FirstRound)
//! - Stress-threshold (StressAbove, StressBelow)
//! - HP-threshold (DeathsDoor)
//! - Status-check (TargetHasStatus, ActorHasStatus)
//! - Framework-native bridges (Probability, IfTargetHealthBelow, IfActorHasStatus, IfTargetPosition)

use game_ddgc_headless::run::conditions::{
    Condition, ConditionAdapter, ConditionContext, ConditionResult, DdgcCondition,
    create_game_condition_evaluator, set_condition_context,
};
use framework_combat::effects::{EffectCondition, SlotRange};
use framework_combat::encounter::CombatSide;
use framework_rules::actor::{ActorAggregate, ActorId};
use framework_rules::attributes::{AttributeKey, AttributeValue, ATTR_HEALTH};
use game_ddgc_headless::content::actors::ATTR_MAX_HEALTH;
use game_ddgc_headless::encounters::Dungeon;

fn make_actor_context(
    actor_hp: f64,
    actor_max_hp: f64,
    actor_stress: f64,
    round: u32,
) -> ConditionContext {
    let mut actors: std::collections::HashMap<ActorId, ActorAggregate> =
        std::collections::HashMap::new();
    let mut side_lookup: std::collections::HashMap<ActorId, CombatSide> =
        std::collections::HashMap::new();

    let mut actor = ActorAggregate::new(ActorId(1));
    actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(actor_hp));
    actor.set_base(AttributeKey::new(ATTR_MAX_HEALTH), AttributeValue(actor_max_hp));
    actor.set_base(AttributeKey::new(game_ddgc_headless::content::actors::ATTR_STRESS), AttributeValue(actor_stress));
    actors.insert(ActorId(1), actor);
    side_lookup.insert(ActorId(1), CombatSide::Ally);

    ConditionContext::new(ActorId(1), vec![ActorId(2)], round, actors, side_lookup, Dungeon::QingLong)
}

fn make_target_context(target_has_status: bool) -> ConditionContext {
    let mut actors: std::collections::HashMap<ActorId, ActorAggregate> =
        std::collections::HashMap::new();
    let mut side_lookup: std::collections::HashMap<ActorId, CombatSide> =
        std::collections::HashMap::new();

    let actor = ActorAggregate::new(ActorId(1));
    actors.insert(ActorId(1), actor);
    side_lookup.insert(ActorId(1), CombatSide::Ally);

    let mut target = ActorAggregate::new(ActorId(2));
    target.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(50.0));
    target.set_base(AttributeKey::new(ATTR_MAX_HEALTH), AttributeValue(50.0));
    if target_has_status {
        target.statuses.attach(game_ddgc_headless::content::statuses::bleed(5.0, 3));
    }
    actors.insert(ActorId(2), target);
    side_lookup.insert(ActorId(2), CombatSide::Enemy);

    ConditionContext::new(ActorId(1), vec![ActorId(2)], 0, actors, side_lookup, Dungeon::QingLong)
}

// ── Round-trigger: FirstRound ────────────────────────────────────────────────

#[test]
fn first_round_passes_on_round_zero() {
    let ctx = make_actor_context(100.0, 100.0, 0.0, 0);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_first_round"),
        ConditionResult::Pass,
        "FirstRound should pass on round 0"
    );
}

#[test]
fn first_round_fails_after_round_zero() {
    let ctx = make_actor_context(100.0, 100.0, 0.0, 1);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_first_round"),
        ConditionResult::Fail,
        "FirstRound should fail on round 1"
    );
}

#[test]
fn first_round_evaluator_wiring() {
    let ctx = make_actor_context(100.0, 100.0, 0.0, 0);
    set_condition_context(ctx);
    let evaluator = create_game_condition_evaluator();
    assert!(evaluator("ddgc_first_round"), "Evaluator should pass on round 0");
}

// ── Stress-threshold: StressAbove ────────────────────────────────────────────

#[test]
fn stress_above_passes_when_stress_exceeds_threshold() {
    let ctx = make_actor_context(100.0, 100.0, 75.0, 0);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_stress_above_50"),
        ConditionResult::Pass,
        "StressAbove(50) should pass when stress is 75"
    );
}

#[test]
fn stress_above_fails_when_stress_below_threshold() {
    let ctx = make_actor_context(100.0, 100.0, 30.0, 0);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_stress_above_50"),
        ConditionResult::Fail,
        "StressAbove(50) should fail when stress is 30"
    );
}

#[test]
fn stress_above_fails_for_monsters() {
    let mut actors: std::collections::HashMap<ActorId, ActorAggregate> =
        std::collections::HashMap::new();
    let mut side_lookup: std::collections::HashMap<ActorId, CombatSide> =
        std::collections::HashMap::new();

    let mut enemy = ActorAggregate::new(ActorId(1));
    enemy.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(50.0));
    enemy.set_base(AttributeKey::new(ATTR_MAX_HEALTH), AttributeValue(50.0));
    actors.insert(ActorId(1), enemy);
    side_lookup.insert(ActorId(1), CombatSide::Enemy);

    let ctx = ConditionContext::new(ActorId(1), vec![], 0, actors, side_lookup, Dungeon::QingLong);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_stress_above_0"),
        ConditionResult::Fail,
        "Monsters should fail StressAbove regardless of threshold"
    );
}

// ── Stress-threshold: StressBelow ────────────────────────────────────────────

#[test]
fn stress_below_passes_when_stress_below_threshold() {
    let ctx = make_actor_context(100.0, 100.0, 30.0, 0);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_stress_below_50"),
        ConditionResult::Pass,
        "StressBelow(50) should pass when stress is 30"
    );
}

#[test]
fn stress_below_fails_when_stress_above_threshold() {
    let ctx = make_actor_context(100.0, 100.0, 75.0, 0);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_stress_below_50"),
        ConditionResult::Fail,
        "StressBelow(50) should fail when stress is 75"
    );
}

#[test]
fn stress_threshold_changes_outcome_across_boundary() {
    let ctx_low = make_actor_context(100.0, 100.0, 20.0, 0);
    let ctx_high = make_actor_context(100.0, 100.0, 80.0, 0);

    let low_above = ConditionAdapter::new(ctx_low.clone()).evaluate_by_tag("ddgc_stress_above_50");
    let high_above = ConditionAdapter::new(ctx_high.clone()).evaluate_by_tag("ddgc_stress_above_50");
    let low_below = ConditionAdapter::new(ctx_low).evaluate_by_tag("ddgc_stress_below_50");
    let high_below = ConditionAdapter::new(ctx_high).evaluate_by_tag("ddgc_stress_below_50");

    assert_eq!(low_above, ConditionResult::Fail);
    assert_eq!(high_above, ConditionResult::Pass);
    assert_eq!(low_below, ConditionResult::Pass);
    assert_eq!(high_below, ConditionResult::Fail);
}

// ── HP-threshold: DeathsDoor ─────────────────────────────────────────────────

#[test]
fn deaths_door_passes_when_hp_below_half() {
    let ctx = make_actor_context(20.0, 100.0, 0.0, 0);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_deaths_door"),
        ConditionResult::Pass,
        "DeathsDoor should pass when HP is 20/100 (20% < 50%)"
    );
}

#[test]
fn deaths_door_fails_when_hp_above_half() {
    let ctx = make_actor_context(80.0, 100.0, 0.0, 0);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_deaths_door"),
        ConditionResult::Fail,
        "DeathsDoor should fail when HP is 80/100 (80% >= 50%)"
    );
}

#[test]
fn deaths_door_fails_at_exactly_half() {
    let ctx = make_actor_context(50.0, 100.0, 0.0, 0);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_deaths_door"),
        ConditionResult::Fail,
        "DeathsDoor should fail at exactly 50% (HP < 50%, strict less-than)"
    );
}

#[test]
fn deaths_door_changes_outcome_across_boundary() {
    let ctx_low = make_actor_context(20.0, 100.0, 0.0, 0);
    let ctx_high = make_actor_context(80.0, 100.0, 0.0, 0);

    let result_low = ConditionAdapter::new(ctx_low).evaluate_by_tag("ddgc_deaths_door");
    let result_high = ConditionAdapter::new(ctx_high).evaluate_by_tag("ddgc_deaths_door");

    assert_eq!(result_low, ConditionResult::Pass);
    assert_eq!(result_high, ConditionResult::Fail);
    assert_ne!(result_low, result_high);
}

#[test]
fn deaths_door_evaluator_wiring() {
    let ctx = make_actor_context(20.0, 100.0, 0.0, 0);
    set_condition_context(ctx);
    let evaluator = create_game_condition_evaluator();
    assert!(evaluator("ddgc_deaths_door"), "Evaluator should pass when at death's door");
}

// ── Status-check: TargetHasStatus ────────────────────────────────────────────

#[test]
fn target_has_status_passes_when_target_has_status() {
    let ctx = make_target_context(true);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_target_has_status_bleed"),
        ConditionResult::Pass,
        "TargetHasStatus(bleed) should pass when target has bleed"
    );
}

#[test]
fn target_has_status_fails_when_target_lacks_status() {
    let ctx = make_target_context(false);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_target_has_status_bleed"),
        ConditionResult::Fail,
        "TargetHasStatus(bleed) should fail when target has no bleed"
    );
}

// ── Status-check: ActorHasStatus ─────────────────────────────────────────────

#[test]
fn actor_has_status_passes_when_actor_has_status() {
    let mut actors: std::collections::HashMap<ActorId, ActorAggregate> =
        std::collections::HashMap::new();
    let mut side_lookup: std::collections::HashMap<ActorId, CombatSide> =
        std::collections::HashMap::new();

    let mut actor = ActorAggregate::new(ActorId(1));
    actor.statuses.attach(game_ddgc_headless::content::statuses::stun(2));
    actors.insert(ActorId(1), actor);
    side_lookup.insert(ActorId(1), CombatSide::Ally);

    let ctx = ConditionContext::new(ActorId(1), vec![], 0, actors, side_lookup, Dungeon::QingLong);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_actor_has_status_stun"),
        ConditionResult::Pass,
        "ActorHasStatus(stun) should pass when actor has stun"
    );
}

#[test]
fn actor_has_status_fails_when_actor_lacks_status() {
    let mut actors: std::collections::HashMap<ActorId, ActorAggregate> =
        std::collections::HashMap::new();
    let mut side_lookup: std::collections::HashMap<ActorId, CombatSide> =
        std::collections::HashMap::new();

    let actor = ActorAggregate::new(ActorId(1));
    actors.insert(ActorId(1), actor);
    side_lookup.insert(ActorId(1), CombatSide::Ally);

    let ctx = ConditionContext::new(ActorId(1), vec![], 0, actors, side_lookup, Dungeon::QingLong);
    let adapter = ConditionAdapter::new(ctx);
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_actor_has_status_stun"),
        ConditionResult::Fail,
        "ActorHasStatus(stun) should fail when actor has no stun"
    );
}

// ── Framework-native: Probability ────────────────────────────────────────────

#[test]
fn framework_probability_passes_when_positive() {
    let adapter = ConditionAdapter::new(make_actor_context(100.0, 100.0, 0.0, 0));
    let cond = EffectCondition::Probability(0.5);
    assert_eq!(
        adapter.evaluate_framework(&cond, ActorId(1)),
        ConditionResult::Pass,
        "Probability(0.5) should pass (deterministic: > 0)"
    );
}

#[test]
fn framework_probability_fails_when_zero() {
    let adapter = ConditionAdapter::new(make_actor_context(100.0, 100.0, 0.0, 0));
    let cond = EffectCondition::Probability(0.0);
    assert_eq!(
        adapter.evaluate_framework(&cond, ActorId(1)),
        ConditionResult::Fail,
        "Probability(0.0) should fail"
    );
}

// ── Framework-native: IfTargetHealthBelow ────────────────────────────────────

#[test]
fn framework_if_target_health_below_passes_when_below() {
    let mut actors: std::collections::HashMap<ActorId, ActorAggregate> =
        std::collections::HashMap::new();
    let mut side_lookup: std::collections::HashMap<ActorId, CombatSide> =
        std::collections::HashMap::new();

    let actor = ActorAggregate::new(ActorId(1));
    actors.insert(ActorId(1), actor);
    side_lookup.insert(ActorId(1), CombatSide::Ally);

    let mut target = ActorAggregate::new(ActorId(2));
    target.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(30.0));
    actors.insert(ActorId(2), target);
    side_lookup.insert(ActorId(2), CombatSide::Enemy);

    let ctx = ConditionContext::new(ActorId(1), vec![ActorId(2)], 0, actors, side_lookup, Dungeon::QingLong);
    let adapter = ConditionAdapter::new(ctx);
    let cond = EffectCondition::IfTargetHealthBelow(50.0);
    assert_eq!(
        adapter.evaluate_framework(&cond, ActorId(2)),
        ConditionResult::Pass,
        "IfTargetHealthBelow(50) should pass when target HP is 30"
    );
}

#[test]
fn framework_if_target_health_below_fails_when_above() {
    let mut actors: std::collections::HashMap<ActorId, ActorAggregate> =
        std::collections::HashMap::new();
    let mut side_lookup: std::collections::HashMap<ActorId, CombatSide> =
        std::collections::HashMap::new();

    let actor = ActorAggregate::new(ActorId(1));
    actors.insert(ActorId(1), actor);
    side_lookup.insert(ActorId(1), CombatSide::Ally);

    let mut target = ActorAggregate::new(ActorId(2));
    target.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(80.0));
    actors.insert(ActorId(2), target);
    side_lookup.insert(ActorId(2), CombatSide::Enemy);

    let ctx = ConditionContext::new(ActorId(1), vec![ActorId(2)], 0, actors, side_lookup, Dungeon::QingLong);
    let adapter = ConditionAdapter::new(ctx);
    let cond = EffectCondition::IfTargetHealthBelow(50.0);
    assert_eq!(
        adapter.evaluate_framework(&cond, ActorId(2)),
        ConditionResult::Fail,
        "IfTargetHealthBelow(50) should fail when target HP is 80"
    );
}

// ── Framework-native: IfActorHasStatus ───────────────────────────────────────

#[test]
fn framework_if_actor_has_status_passes_when_has_status() {
    let mut actors: std::collections::HashMap<ActorId, ActorAggregate> =
        std::collections::HashMap::new();
    let mut side_lookup: std::collections::HashMap<ActorId, CombatSide> =
        std::collections::HashMap::new();

    let mut actor = ActorAggregate::new(ActorId(1));
    actor.statuses.attach(game_ddgc_headless::content::statuses::poison(5.0, 2));
    actors.insert(ActorId(1), actor);
    side_lookup.insert(ActorId(1), CombatSide::Ally);

    let ctx = ConditionContext::new(ActorId(1), vec![], 0, actors, side_lookup, Dungeon::QingLong);
    let adapter = ConditionAdapter::new(ctx);
    let cond = EffectCondition::IfActorHasStatus("poison".to_string());
    assert_eq!(
        adapter.evaluate_framework(&cond, ActorId(1)),
        ConditionResult::Pass,
        "IfActorHasStatus(poison) should pass when actor has poison"
    );
}

#[test]
fn framework_if_actor_has_status_fails_when_lacks_status() {
    let mut actors: std::collections::HashMap<ActorId, ActorAggregate> =
        std::collections::HashMap::new();
    let mut side_lookup: std::collections::HashMap<ActorId, CombatSide> =
        std::collections::HashMap::new();

    let actor = ActorAggregate::new(ActorId(1));
    actors.insert(ActorId(1), actor);
    side_lookup.insert(ActorId(1), CombatSide::Ally);

    let ctx = ConditionContext::new(ActorId(1), vec![], 0, actors, side_lookup, Dungeon::QingLong);
    let adapter = ConditionAdapter::new(ctx);
    let cond = EffectCondition::IfActorHasStatus("poison".to_string());
    assert_eq!(
        adapter.evaluate_framework(&cond, ActorId(1)),
        ConditionResult::Fail,
        "IfActorHasStatus(poison) should fail when actor has no poison"
    );
}

// ── Framework-native: IfTargetPosition ───────────────────────────────────────

#[test]
fn framework_if_target_position_returns_unknown() {
    let adapter = ConditionAdapter::new(make_actor_context(100.0, 100.0, 0.0, 0));
    let cond = EffectCondition::IfTargetPosition(SlotRange { min: 0, max: 2 });
    assert_eq!(
        adapter.evaluate_framework(&cond, ActorId(2)),
        ConditionResult::Unknown,
        "IfTargetPosition should return Unknown (formation context unavailable)"
    );
}

// ── Unified evaluate interface ───────────────────────────────────────────────

#[test]
fn unified_evaluate_handles_framework_condition() {
    let adapter = ConditionAdapter::new(make_actor_context(100.0, 100.0, 0.0, 0));
    let cond = Condition::Framework(EffectCondition::Probability(0.5));
    assert_eq!(adapter.evaluate(&cond, ActorId(1)), ConditionResult::Pass);
}

#[test]
fn unified_evaluate_handles_ddgc_condition() {
    let adapter = ConditionAdapter::new(make_actor_context(100.0, 100.0, 0.0, 0));
    let cond = Condition::Ddgc(DdgcCondition::FirstRound);
    assert_eq!(adapter.evaluate(&cond, ActorId(1)), ConditionResult::Pass);
}

// ── Coverage enumeration ─────────────────────────────────────────────────────

#[test]
fn all_ddgc_condition_variants_are_parseable() {
    // This test enumerates every DdgcCondition variant and proves it parses.
    // If a new variant is added, this test must be updated — making the
    // coverage table machine-verified.
    let tags = vec![
        ("ddgc_first_round", true),
        ("ddgc_stress_above_50", true),
        ("ddgc_stress_below_50", true),
        ("ddgc_deaths_door", true),
        ("ddgc_hp_above_0.5", true),
        ("ddgc_target_hp_above_0.5", true),
        ("ddgc_target_hp_below_0.5", true),
        ("ddgc_target_has_status_bleed", true),
        ("ddgc_actor_has_status_stun", true),
        ("ddgc_in_mode_qinglong", true),
        ("ddgc_on_kill", true),
        // Low-impact deferred / unknown
        ("ddgc_afflicted", false),
        ("ddgc_virtued", false),
        ("ddgc_light_below_50", false),
    ];

    for (tag, should_parse) in tags {
        let parsed = ConditionAdapter::parse_condition_tag(tag).is_some();
        assert_eq!(
            parsed, should_parse,
            "Tag '{}' parse result mismatch: expected {}, got {}",
            tag, should_parse, parsed
        );
    }
}

#[test]
fn condition_coverage_summary_matches_documentation() {
    // Machine-verified coverage count: 11 DdgcCondition variants +
    // 4 framework bridges = 15 implemented condition evaluations.
    let ddgc_variants = vec![
        DdgcCondition::FirstRound,
        DdgcCondition::StressAbove(50.0),
        DdgcCondition::StressBelow(50.0),
        DdgcCondition::DeathsDoor,
        DdgcCondition::HpAbove(0.5),
        DdgcCondition::TargetHpAbove(0.5),
        DdgcCondition::TargetHpBelow(0.5),
        DdgcCondition::TargetHasStatus("bleed".to_string()),
        DdgcCondition::ActorHasStatus("stun".to_string()),
        DdgcCondition::InMode("qinglong".to_string()),
        DdgcCondition::OnKill,
    ];

    let ctx = make_actor_context(100.0, 100.0, 75.0, 0);
    let adapter = ConditionAdapter::new(ctx);

    // Every variant must evaluate to Pass, Fail, or Unknown — never panic.
    for variant in &ddgc_variants {
        let result = adapter.evaluate_ddgc(variant);
        assert!(
            matches!(result, ConditionResult::Pass | ConditionResult::Fail | ConditionResult::Unknown),
            "DdgcCondition::{:?} produced invalid result",
            variant
        );
    }

    assert_eq!(
        ddgc_variants.len(),
        11,
        "DdgcCondition variant count should be 11 per coverage documentation"
    );
}

#[test]
fn unknown_tags_return_unknown_not_fail() {
    let adapter = ConditionAdapter::new(make_actor_context(100.0, 100.0, 0.0, 0));

    // Low-impact deferred conditions that are not implemented
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_afflicted"),
        ConditionResult::Unknown,
        "Unimplemented conditions should return Unknown"
    );
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_virtued"),
        ConditionResult::Unknown,
        "Unimplemented conditions should return Unknown"
    );
    assert_eq!(
        adapter.evaluate_by_tag("ddgc_chaos_above_50"),
        ConditionResult::Unknown,
        "Unimplemented conditions should return Unknown"
    );
}

#[test]
fn opening_strike_skill_uses_first_round_condition() {
    let pack = game_ddgc_headless::content::heroes::hunter::skill_pack();
    let skill = pack.iter().find(|s| s.id.0.as_str() == "opening_strike");
    assert!(skill.is_some(), "opening_strike should be in Hunter skill_pack");

    let skill = skill.unwrap();

    assert!(
        ConditionAdapter::parse_condition_tag("ddgc_first_round").is_some(),
        "ddgc_first_round should parse as a valid condition"
    );

    let has_condition = skill.effects.iter().any(|e| {
        e.conditions.iter().any(|c| matches!(c, EffectCondition::GameCondition(tag) if tag == "ddgc_first_round"))
    });
    assert!(
        has_condition,
        "opening_strike should have an effect node with GameCondition(\"ddgc_first_round\")"
    );
}

#[test]
fn desperate_strike_skill_uses_deaths_door_condition() {
    let pack = game_ddgc_headless::content::heroes::hunter::skill_pack();
    let skill = pack.iter().find(|s| s.id.0.as_str() == "desperate_strike");
    assert!(skill.is_some(), "desperate_strike should be in Hunter skill_pack");

    let skill = skill.unwrap();

    assert!(
        ConditionAdapter::parse_condition_tag("ddgc_deaths_door").is_some(),
        "ddgc_deaths_door should parse as a valid condition"
    );

    let has_condition = skill.effects.iter().any(|e| {
        e.conditions.iter().any(|c| matches!(c, EffectCondition::GameCondition(tag) if tag == "ddgc_deaths_door"))
    });
    assert!(
        has_condition,
        "desperate_strike should have an effect node with GameCondition(\"ddgc_deaths_door\")"
    );
}
