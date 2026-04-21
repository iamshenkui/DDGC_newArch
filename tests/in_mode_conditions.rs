//! Integration tests for dungeon-mode condition evaluation (US-803-f).
//!
//! Validates that `ddgc_in_mode_<mode>` condition tag is correctly parsed and
//! evaluated through the ConditionAdapter public API, and that the contracts
//! layer provides the canonical mode name resolution.

use game_ddgc_headless::run::conditions::{
    ConditionAdapter, ConditionContext, ConditionResult, DdgcCondition,
    create_game_condition_evaluator, set_condition_context,
};
use framework_combat::effects::EffectCondition;
use framework_combat::encounter::CombatSide;
use framework_rules::actor::{ActorAggregate, ActorId};
use game_ddgc_headless::encounters::Dungeon;

fn make_context(dungeon: Dungeon) -> ConditionContext {
    let mut actors: std::collections::HashMap<ActorId, ActorAggregate> =
        std::collections::HashMap::new();
    let mut side_lookup: std::collections::HashMap<ActorId, CombatSide> =
        std::collections::HashMap::new();

    let actor = ActorAggregate::new(ActorId(1));
    actors.insert(ActorId(1), actor);
    side_lookup.insert(ActorId(1), CombatSide::Ally);

    ConditionContext::new(
        ActorId(1),
        vec![ActorId(2)],
        0,
        actors,
        side_lookup,
        dungeon,
    )
}

// ── InMode passes when dungeon matches ───────────────────────────────────────

#[test]
fn in_mode_passes_when_dungeon_matches() {
    let ctx = make_context(Dungeon::XuanWu);
    let adapter = ConditionAdapter::new(ctx);

    assert_eq!(
        adapter.evaluate_by_tag("ddgc_in_mode_xuanwu"),
        ConditionResult::Pass,
        "InMode should pass when dungeon matches the mode tag"
    );
}

// ── InMode fails when dungeon differs ────────────────────────────────────────

#[test]
fn in_mode_fails_when_dungeon_differs() {
    let ctx = make_context(Dungeon::QingLong);
    let adapter = ConditionAdapter::new(ctx);

    assert_eq!(
        adapter.evaluate_by_tag("ddgc_in_mode_xuanwu"),
        ConditionResult::Fail,
        "InMode should fail when dungeon does not match the mode tag"
    );
}

// ── InMode works for all dungeon variants ────────────────────────────────────

#[test]
fn in_mode_works_for_all_dungeon_variants() {
    for (dungeon, mode) in [
        (Dungeon::QingLong, "qinglong"),
        (Dungeon::BaiHu, "baihu"),
        (Dungeon::ZhuQue, "zhuque"),
        (Dungeon::XuanWu, "xuanwu"),
        (Dungeon::Cross, "cross"),
    ] {
        let ctx = make_context(dungeon);
        let adapter = ConditionAdapter::new(ctx);
        let tag = format!("ddgc_in_mode_{}", mode);

        assert_eq!(
            adapter.evaluate_by_tag(&tag),
            ConditionResult::Pass,
            "InMode({}) should pass in {:?} dungeon",
            mode,
            dungeon
        );
    }
}

// ── Tag parsing ──────────────────────────────────────────────────────────────

#[test]
fn parse_condition_tag_handles_ddgc_in_mode() {
    assert!(matches!(
        ConditionAdapter::parse_condition_tag("ddgc_in_mode_qinglong"),
        Some(DdgcCondition::InMode(mode)) if mode == "qinglong"
    ));
    assert!(matches!(
        ConditionAdapter::parse_condition_tag("ddgc_in_mode_xuanwu"),
        Some(DdgcCondition::InMode(mode)) if mode == "xuanwu"
    ));
}

#[test]
fn parse_condition_tag_rejects_empty_mode() {
    assert_eq!(
        ConditionAdapter::parse_condition_tag("ddgc_in_mode_"),
        None,
        "Empty mode tag should not parse"
    );
}

// ── Game condition evaluator wiring ──────────────────────────────────────────

#[test]
fn game_evaluator_wires_in_mode_correctly() {
    let ctx = make_context(Dungeon::ZhuQue);
    set_condition_context(ctx);
    let evaluator = create_game_condition_evaluator();

    assert!(
        evaluator("ddgc_in_mode_zhuque"),
        "Evaluator should return true for matching dungeon mode"
    );
    assert!(
        !evaluator("ddgc_in_mode_qinglong"),
        "Evaluator should return false for non-matching dungeon mode"
    );
}

// ── Fixture skill uses InMode condition ──────────────────────────────────────

#[test]
fn xuanwu_strike_skill_uses_in_mode_condition() {
    let pack = game_ddgc_headless::content::heroes::hunter::skill_pack();
    let skill = pack.iter().find(|s| s.id.0.as_str() == "xuanwu_strike");
    assert!(skill.is_some(), "xuanwu_strike should be in Hunter skill_pack");

    let skill = skill.unwrap();

    // Verify the condition tag is parseable by the adapter
    assert!(
        ConditionAdapter::parse_condition_tag("ddgc_in_mode_xuanwu").is_some(),
        "ddgc_in_mode_xuanwu should parse as a valid InMode condition"
    );

    // Verify the skill's effect nodes actually wire the GameCondition
    let has_in_mode_condition = skill.effects.iter().any(|e| {
        e.conditions.iter().any(|c| matches!(c, EffectCondition::GameCondition(tag) if tag == "ddgc_in_mode_xuanwu"))
    });
    assert!(
        has_in_mode_condition,
        "xuanwu_strike should have an effect node with GameCondition(\"ddgc_in_mode_xuanwu\")"
    );
}
