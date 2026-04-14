//! Regression tests for Phase 1 reactive hooks and usage limits.
//!
//! This test suite verifies that the Phase 1 mechanics (riposte, guard,
//! per-turn limits, per-battle limits) remain functional after changes.
//!
//! All tests use game_ddgc_headless public APIs (run module) rather than
//! framework internals.

use game_ddgc_headless::run::usage_counters::{SkillUsageCounters, UsageLimit, UsageScope};
use game_ddgc_headless::run::usage_limits::get_usage_limit;
use framework_combat::skills::SkillId;
use framework_rules::actor::ActorId;

// ── Per-turn limit regression tests ──────────────────────────────────────────

#[test]
fn per_turn_limit_is_enforced() {
    let mut counters = SkillUsageCounters::new();
    let actor = ActorId(1);
    let skill = SkillId::new("direct_hit_1");

    // Get the limit from DDGC metadata
    let limit = get_usage_limit(&skill).expect("direct_hit_1 should have a limit");
    assert_eq!(limit.scope, UsageScope::Turn, "direct_hit_1 should be per-turn");
    assert_eq!(limit.max_uses, 2, "direct_hit_1 should have limit of 2");

    // Use up to limit
    assert!(counters.can_use(actor, &skill, limit));
    counters.record_usage(actor, skill.clone(), UsageScope::Turn);

    assert!(counters.can_use(actor, &skill, limit));
    counters.record_usage(actor, skill.clone(), UsageScope::Turn);

    // Over limit - should be blocked
    assert!(!counters.can_use(actor, &skill, limit));
}

#[test]
fn per_turn_limit_resets_after_turn_boundary() {
    let mut counters = SkillUsageCounters::new();
    let actor = ActorId(1);
    let skill = SkillId::new("direct_hit_1");
    let limit = get_usage_limit(&skill).expect("direct_hit_1 should have a limit");

    // Use up to limit
    counters.record_usage(actor, skill.clone(), UsageScope::Turn);
    counters.record_usage(actor, skill.clone(), UsageScope::Turn);
    assert!(!counters.can_use(actor, &skill, limit));

    // Turn boundary resets turn-scoped counters
    counters.reset_turn_scope(actor);

    // Should be available again
    assert!(counters.can_use(actor, &skill, limit));
}

// ── Per-battle limit regression tests ────────────────────────────────────────

#[test]
fn per_battle_limit_is_enforced() {
    let mut counters = SkillUsageCounters::new();
    let actor = ActorId(1);
    let skill = SkillId::new("duality_fate");

    // Get the limit from DDGC metadata
    let limit = get_usage_limit(&skill).expect("duality_fate should have a limit");
    assert_eq!(limit.scope, UsageScope::Battle, "duality_fate should be per-battle");
    assert_eq!(limit.max_uses, 1, "duality_fate should have limit of 1");

    // Use once
    assert!(counters.can_use(actor, &skill, limit));
    counters.record_usage(actor, skill.clone(), UsageScope::Battle);

    // Over limit - should be blocked
    assert!(!counters.can_use(actor, &skill, limit));
}

#[test]
fn per_battle_limit_survives_turn_reset() {
    let mut counters = SkillUsageCounters::new();
    let actor = ActorId(1);
    let skill = SkillId::new("duality_fate");
    let limit = get_usage_limit(&skill).expect("duality_fate should have a limit");

    // Use up to limit
    counters.record_usage(actor, skill.clone(), UsageScope::Battle);
    assert!(!counters.can_use(actor, &skill, limit));

    // Turn boundary should NOT reset battle-scoped counters
    counters.reset_turn_scope(actor);

    // Should still be blocked
    assert!(!counters.can_use(actor, &skill, limit));
}

#[test]
fn per_battle_limit_resets_at_battle_boundary() {
    let mut counters = SkillUsageCounters::new();
    let actor = ActorId(1);
    let skill = SkillId::new("duality_fate");
    let limit = get_usage_limit(&skill).expect("duality_fate should have a limit");

    // Use up to limit
    counters.record_usage(actor, skill.clone(), UsageScope::Battle);
    assert!(!counters.can_use(actor, &skill, limit));

    // Battle boundary resets battle-scoped counters
    counters.reset_battle_scope(actor);

    // Should be available again
    assert!(counters.can_use(actor, &skill, limit));
}

// ── Cross-limit regression tests ──────────────────────────────────────────────

#[test]
fn turn_and_battle_limits_are_independent() {
    let mut counters = SkillUsageCounters::new();
    let actor = ActorId(1);
    let skill = SkillId::new("fireball");

    let turn_limit = UsageLimit::per_turn(2);
    let battle_limit = UsageLimit::per_battle(1);

    // Use turn limit (2) and battle limit (1)
    counters.record_usage(actor, skill.clone(), UsageScope::Turn);
    counters.record_usage(actor, skill.clone(), UsageScope::Turn);
    counters.record_usage(actor, skill.clone(), UsageScope::Battle);

    // Both at limit
    assert!(!counters.can_use(actor, &skill, turn_limit), "Turn limit should be reached");
    assert!(!counters.can_use(actor, &skill, battle_limit), "Battle limit should be reached");

    // Turn reset restores turn only
    counters.reset_turn_scope(actor);
    assert!(counters.can_use(actor, &skill, turn_limit), "Turn should be available after reset");
    assert!(!counters.can_use(actor, &skill, battle_limit), "Battle should still be blocked");

    // Battle reset restores battle
    counters.reset_battle_scope(actor);
    assert!(counters.can_use(actor, &skill, turn_limit), "Turn should still be available");
    assert!(counters.can_use(actor, &skill, battle_limit), "Battle should be available after reset");
}

#[test]
fn different_actors_have_independent_limits() {
    let mut counters = SkillUsageCounters::new();
    let actor1 = ActorId(1);
    let actor2 = ActorId(2);
    let skill = SkillId::new("direct_hit_1");
    let limit = get_usage_limit(&skill).expect("direct_hit_1 should have a limit");

    // Actor 1 uses up limit
    counters.record_usage(actor1, skill.clone(), UsageScope::Turn);
    counters.record_usage(actor1, skill.clone(), UsageScope::Turn);
    assert!(!counters.can_use(actor1, &skill, limit));

    // Actor 2 should still have full uses
    assert!(counters.can_use(actor2, &skill, limit));
}

// ── Skill isolation regression tests ─────────────────────────────────────────

#[test]
fn each_skill_has_independent_limit() {
    let mut counters = SkillUsageCounters::new();
    let actor = ActorId(1);

    let direct_hit_limit = get_usage_limit(&SkillId::new("direct_hit_1"))
        .expect("direct_hit_1 should have a limit");
    let duality_limit = get_usage_limit(&SkillId::new("duality_fate"))
        .expect("duality_fate should have a limit");

    // Use direct_hit_1 up to limit
    counters.record_usage(actor, SkillId::new("direct_hit_1"), UsageScope::Turn);
    counters.record_usage(actor, SkillId::new("direct_hit_1"), UsageScope::Turn);
    assert!(!counters.can_use(actor, &SkillId::new("direct_hit_1"), direct_hit_limit));

    // duality_fate should be unaffected
    assert!(counters.can_use(actor, &SkillId::new("duality_fate"), duality_limit));
}