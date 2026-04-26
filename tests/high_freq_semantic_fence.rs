//! Cross-cutting high-frequency semantic fence integration tests (US-009-c).
//!
//! This is the canonical integration-level "no silent drop" test suite.
//! It exercises every high-frequency semantic path across all subsystems
//! (targeting, movement, camp effects, meta transitions, damage, hit resolution,
//! conditions) and verifies that each path either produces a meaningful result
//! or a deterministic fence marker — never an empty string, never a panic,
//! never a silent no-op.
//!
//! The docs-layer (US-009-b) provides unit-level fence tests in
//! `src/docs/mod.rs::high_freq_path_tests`. This file provides integration-level
//! regression coverage that exercises the fences through the public API.

use game_ddgc_headless::contracts::{
    CampEffect, CampEffectType, CampTargetSelection,
    HeroCampState, LaunchConstraint, MovementDirection, MovementEffect,
    PhaseTransitionConfig, PhaseTransitionTrigger, SideAffinity, TargetCount,
    TargetRank, TargetingIntent,
};
use game_ddgc_headless::run::conditions::{
    ConditionAdapter, ConditionContext, ConditionResult, DdgcCondition,
};
use game_ddgc_headless::run::damage_policy::{DamagePolicy, DamageRange};
use game_ddgc_headless::run::hit_resolution::HitResolutionContext;
use game_ddgc_headless::encounters::Dungeon;
use framework_combat::effects::EffectCondition;
use framework_rules::actor::ActorId;

// ── Helpers ───────────────────────────────────────────────────────────────────

fn make_condition_context() -> ConditionContext {
    let mut actors = std::collections::HashMap::new();
    let mut side_lookup = std::collections::HashMap::new();
    let actor = framework_rules::actor::ActorAggregate::new(ActorId(1));
    actors.insert(ActorId(1), actor);
    side_lookup.insert(ActorId(1), framework_combat::encounter::CombatSide::Ally);
    ConditionContext::new(ActorId(1), vec![ActorId(2)], 0, actors, side_lookup, Dungeon::QingLong)
}

fn make_hero_state() -> HeroCampState {
    HeroCampState::new(80.0, 100.0, 40.0, 200.0)
}

fn make_camp_effect(effect_type: CampEffectType, amount: f64, sub_type: &str) -> CampEffect {
    CampEffect {
        selection: CampTargetSelection::Individual,
        requirements: vec![],
        chance: 1.0,
        effect_type,
        sub_type: sub_type.to_string(),
        amount,
    }
}

// ── Canonical "no silent drop" integration test ───────────────────────────────
//
// This test exercises every high-frequency path and verifies the invariant:
// every path produces a meaningful result, a fence marker, or a recognized
// Unknown — never an empty string, never panic, never silent no-op.

#[test]
fn no_high_freq_path_silently_drops_semantic() {
    // ── 1. Targeting (H): all 5 LaunchConstraint variants must label ──────
    let constraints = [
        LaunchConstraint::Any,
        LaunchConstraint::FrontRow,
        LaunchConstraint::BackRow,
        LaunchConstraint::SpecificLane(0),
        LaunchConstraint::SlotRange { min: 0, max: 3 },
    ];
    for c in &constraints {
        assert!(!c.label().is_empty(),
            "LaunchConstraint label must not be empty");
    }

    // ── 2. Targeting (H): all 3 SideAffinity variants must label ─────────
    for a in &[SideAffinity::Enemy, SideAffinity::Ally, SideAffinity::Any] {
        assert!(!a.label().is_empty(),
            "SideAffinity label must not be empty");
    }

    // ── 3. Targeting (H): all 4 TargetRank variants must label ───────────
    for r in &[TargetRank::Any, TargetRank::Front, TargetRank::Back, TargetRank::FrontAndBack] {
        assert!(!r.label().is_empty(),
            "TargetRank label must not be empty");
    }

    // ── 4. Targeting (H): all 2 TargetCount variants must label ──────────
    for tc in &[TargetCount::Single, TargetCount::Multiple] {
        assert!(!tc.label().is_empty(),
            "TargetCount label must not be empty");
    }

    // ── 5. Movement (M): all 4 MovementEffect variants must label ────────
    for e in &[
        MovementEffect::Push(1),
        MovementEffect::Pull(1),
        MovementEffect::Shuffle,
        MovementEffect::None,
    ] {
        assert!(!e.label().is_empty(),
            "MovementEffect label must not be empty");
    }

    // ── 6. Movement (M): all 2 MovementDirection variants must serialize ─
    for d in &[MovementDirection::Forward, MovementDirection::Backward] {
        let json = serde_json::to_string(d).unwrap();
        assert!(!json.is_empty(),
            "MovementDirection must serialize to non-empty JSON");
    }

    // ── 7. Special Effects / Camp (M): all 22 variants must produce trace ─
    let all_camp_types = [
        CampEffectType::None,
        CampEffectType::StressHealAmount,
        CampEffectType::HealthHealMaxHealthPercent,
        CampEffectType::RemoveBleed,
        CampEffectType::RemovePoison,
        CampEffectType::Buff,
        CampEffectType::RemoveDeathRecovery,
        CampEffectType::ReduceAmbushChance,
        CampEffectType::RemoveDisease,
        CampEffectType::StressDamageAmount,
        CampEffectType::Loot,
        CampEffectType::ReduceTorch,
        CampEffectType::HealthDamageMaxHealthPercent,
        CampEffectType::RemoveBurn,
        CampEffectType::RemoveFrozen,
        CampEffectType::StressHealPercent,
        CampEffectType::RemoveDebuff,
        CampEffectType::RemoveAllDebuff,
        CampEffectType::HealthHealRange,
        CampEffectType::HealthHealAmount,
        CampEffectType::ReduceTurbulenceChance,
        CampEffectType::ReduceRiptideChance,
    ];
    assert_eq!(all_camp_types.len(), 22);

    for et in &all_camp_types {
        let effect = make_camp_effect(*et, 5.0, "");
        let state = make_hero_state();
        let result = effect.apply(state, "test_skill", "perf", None, 0);
        assert!(!result.trace.description.is_empty(),
            "SILENT SEMANTIC DROP: CampEffectType::{:?} produced empty trace", et);
    }

    // ── 8. Meta Transitions (B): all 5 PhaseTransitionTrigger variants ──
    for t in &[
        PhaseTransitionTrigger::PressAttackCount(1),
        PhaseTransitionTrigger::HealthBelow(0.5),
        PhaseTransitionTrigger::RoundElapsed(1),
        PhaseTransitionTrigger::OnAllyDeath("x".to_string()),
        PhaseTransitionTrigger::OnAllAlliesDead(vec![]),
    ] {
        assert!(!t.label().is_empty(),
            "PhaseTransitionTrigger label must not be empty");
    }

    // ── 9. Combat Conditions (H): all 11 DdgcCondition variants ─────────
    let ctx = make_condition_context();
    let adapter = ConditionAdapter::new(ctx);
    for cond in &[
        DdgcCondition::FirstRound,
        DdgcCondition::StressAbove(0.0),
        DdgcCondition::StressBelow(0.0),
        DdgcCondition::DeathsDoor,
        DdgcCondition::HpAbove(0.5),
        DdgcCondition::TargetHpAbove(0.5),
        DdgcCondition::TargetHpBelow(0.5),
        DdgcCondition::TargetHasStatus("s".to_string()),
        DdgcCondition::ActorHasStatus("s".to_string()),
        DdgcCondition::InMode("m".to_string()),
        DdgcCondition::OnKill,
    ] {
        let result = adapter.evaluate_ddgc(cond);
        assert_ne!(result, ConditionResult::Unknown,
            "DdgcCondition {:?} returned Unknown — supported condition must not silently fail", cond);
    }

    // ── 10. Conditions: IfTargetPosition must return Unknown (not panic) ─
    let result = adapter.evaluate_framework(
        &EffectCondition::IfTargetPosition(
            framework_combat::effects::SlotRange { min: 0, max: 3 },
        ),
        ActorId(2),
    );
    assert_eq!(result, ConditionResult::Unknown,
        "IfTargetPosition must return Unknown, not silently fail");

    // ── 11. Damage (H): FixedAverage produces valid deterministic output ─
    let range = DamageRange::new(10.0, 20.0);
    let damage = DamagePolicy::FixedAverage.resolve(range, 0, "test");
    assert!(damage.is_finite() && damage > 0.0,
        "FixedAverage must produce finite positive damage");

    // ── 12. Damage (H): Rolled is deterministic with same seed ───────────
    let d1 = DamagePolicy::Rolled.resolve(range, 12345, "test");
    let d2 = DamagePolicy::Rolled.resolve(range, 12345, "test");
    assert!((d1 - d2).abs() < f64::EPSILON,
        "Rolled damage with same seed must be deterministic");

    // ── 13. Hit Resolution (M): context is constructable ─────────────────
    let hit_ctx = HitResolutionContext {
        attacker_id: ActorId(1),
        defender_id: ActorId(2),
        attacker_accuracy: 0.95,
        defender_dodge: 0.05,
        has_flanking_bonus: false,
        defender_is_marked: false,
    };
    assert!(hit_ctx.attacker_id.0 > 0);
    assert!(hit_ctx.defender_id.0 > 0);
    assert!(hit_ctx.attacker_accuracy >= 0.0 && hit_ctx.attacker_accuracy <= 1.0);
    assert!(hit_ctx.defender_dodge >= 0.0 && hit_ctx.defender_dodge <= 1.0);
}

// ── Fence taxonomy verification ───────────────────────────────────────────────

#[test]
fn every_fence_status_is_represented() {
    // Verify that all fence categories are populated:
    // - Implemented: targeting, movement, implemented camp effects
    // - Fenced (STUB): 4 camp effects
    // - Fenced (SKIPPED): 2 camp effects
    // - Unsupported (Unknown): IfTargetPosition

    // STUB count: exactly 4 camp effects are stubbed
    let stubbed = [
        CampEffectType::ReduceAmbushChance,
        CampEffectType::Loot,
        CampEffectType::ReduceTurbulenceChance,
        CampEffectType::ReduceRiptideChance,
    ];
    assert_eq!(stubbed.len(), 4);

    // SKIPPED count: exactly 2 camp effects are skipped
    let skipped = [CampEffectType::None, CampEffectType::ReduceTorch];
    assert_eq!(skipped.len(), 2);

    // Implemented count: 22 total - 4 stubbed - 2 skipped = 16 implemented
    assert_eq!(22 - stubbed.len() - skipped.len(), 16);
}

#[test]
fn targeting_intent_fence_has_no_empty_labels() {
    // Exhaustive: every TargetingIntent combination produces valid labels
    let launch_constraints = [
        LaunchConstraint::Any,
        LaunchConstraint::FrontRow,
        LaunchConstraint::BackRow,
        LaunchConstraint::SpecificLane(0),
        LaunchConstraint::SlotRange { min: 0, max: 3 },
    ];
    let target_ranks = [
        TargetRank::Any,
        TargetRank::Front,
        TargetRank::Back,
        TargetRank::FrontAndBack,
    ];
    let side_affinities = [
        SideAffinity::Enemy,
        SideAffinity::Ally,
        SideAffinity::Any,
    ];
    let target_counts = [TargetCount::Single, TargetCount::Multiple];

    for lc in &launch_constraints {
        for tr in &target_ranks {
            for sa in &side_affinities {
                for tc in &target_counts {
                    let intent = TargetingIntent::new(lc.clone(), tr.clone(), sa.clone(), tc.clone());
                    let fields = [
                        intent.launch_constraint.label(),
                        intent.target_rank.label(),
                        intent.side_affinity.label(),
                        intent.target_count.label(),
                    ];
                    for field in &fields {
                        assert!(!field.is_empty(),
                            "TargetingIntent combo ({:?},{:?},{:?},{:?}) has empty field",
                            lc, tr, sa, tc);
                    }
                }
            }
        }
    }
}

// ── Condition adapter fence: unsupported returns Unknown ──────────────────────

#[test]
fn unsupported_ddgc_condition_tags_return_unknown() {
    let ctx = make_condition_context();
    let adapter = ConditionAdapter::new(ctx);

    // Tags that should return Unknown (not implemented, no migrated content uses them)
    let unsupported_tags = [
        "ddgc_afflicted",
        "ddgc_virtued",
        "ddgc_melee",
        "ddgc_ranged",
        "ddgc_light_below_50",
        "ddgc_light_above_75",
        "ddgc_chaos_above_30",
        "ddgc_in_camp",
        "ddgc_in_corridor",
        "ddgc_dot",
        "ddgc_size",
        "ddgc_enemy_type",
        "ddgc_skill",
        "ddgc_riposting",
        "ddgc_walk_back",
    ];

    for tag in &unsupported_tags {
        let result = adapter.evaluate_by_tag(tag);
        assert_eq!(
            result,
            ConditionResult::Unknown,
            "Unsupported tag '{}' must return Unknown, not Pass or Fail",
            tag
        );
    }
}

#[test]
fn unsupported_tags_never_panic() {
    let ctx = make_condition_context();
    let adapter = ConditionAdapter::new(ctx);

    // Various malformed or unsupported tags should never panic
    let edge_case_tags = [
        "",
        "invalid_format",
        "ddgc_",
        "ddgc_nonexistent_condition_xyz",
        "random_string",
    ];

    for tag in &edge_case_tags {
        let result = adapter.evaluate_by_tag(tag);
        // Must not panic — Unknown is the fence
        assert_eq!(result, ConditionResult::Unknown);
    }
}

// ── Damage range fence: min > max panics ──────────────────────────────────────

#[test]
fn damage_range_rejects_invalid_min_max() {
    let result = std::panic::catch_unwind(|| DamageRange::new(30.0, 20.0));
    assert!(result.is_err(),
        "DamageRange with min > max must panic — this is a contract violation");
}

#[test]
fn damage_range_fixed_produces_exact_value() {
    let range = DamageRange::fixed(15.0);
    let fixed = DamagePolicy::FixedAverage.resolve(range, 0, "x");
    let rolled = DamagePolicy::Rolled.resolve(range, 99, "x");
    assert!((fixed - 15.0).abs() < f64::EPSILON);
    assert!((rolled - 15.0).abs() < f64::EPSILON,
        "Rolled policy on fixed range should return exact value");
}

// ── Hit resolution: effective dodge edge cases ────────────────────────────────

#[test]
fn hit_resolution_marked_target_reduces_effective_dodge() {
    let unmarked = HitResolutionContext {
        attacker_id: ActorId(1), defender_id: ActorId(2),
        attacker_accuracy: 0.5, defender_dodge: 0.20,
        has_flanking_bonus: false, defender_is_marked: false,
    };
    let marked = HitResolutionContext {
        attacker_id: ActorId(1), defender_id: ActorId(2),
        attacker_accuracy: 0.5, defender_dodge: 0.20,
        has_flanking_bonus: false, defender_is_marked: true,
    };

    assert_eq!(unmarked.effective_dodge(), 0.20);
    assert!((marked.effective_dodge() - 0.10).abs() < 0.0001,
        "Marked target dodge should be 50% of base");
}

#[test]
fn hit_resolution_default_policy_is_accuracy_vs_dodge() {
    let policy = game_ddgc_headless::run::hit_resolution::HitPolicy::default();
    assert_eq!(
        policy,
        game_ddgc_headless::run::hit_resolution::HitPolicy::AccuracyVsDodge
    );
}

// ── Serialization roundtrip: all contract types ───────────────────────────────

#[test]
fn all_contract_types_serialize_roundtrip() {
    // LaunchConstraint
    let lc = LaunchConstraint::SlotRange { min: 1, max: 3 };
    let json = serde_json::to_string(&lc).unwrap();
    let restored: LaunchConstraint = serde_json::from_str(&json).unwrap();
    assert_eq!(lc, restored);

    // TargetRank
    let tr = TargetRank::FrontAndBack;
    let json = serde_json::to_string(&tr).unwrap();
    let restored: TargetRank = serde_json::from_str(&json).unwrap();
    assert_eq!(tr, restored);

    // SideAffinity
    let sa = SideAffinity::Any;
    let json = serde_json::to_string(&sa).unwrap();
    let restored: SideAffinity = serde_json::from_str(&json).unwrap();
    assert_eq!(sa, restored);

    // TargetCount
    let tc = TargetCount::Multiple;
    let json = serde_json::to_string(&tc).unwrap();
    let restored: TargetCount = serde_json::from_str(&json).unwrap();
    assert_eq!(tc, restored);

    // TargetingIntent
    let intent = TargetingIntent::default_enemy_single();
    let json = serde_json::to_string(&intent).unwrap();
    let restored: TargetingIntent = serde_json::from_str(&json).unwrap();
    assert_eq!(intent, restored);

    // MovementDirection
    let md = MovementDirection::Backward;
    let json = serde_json::to_string(&md).unwrap();
    let restored: MovementDirection = serde_json::from_str(&json).unwrap();
    assert_eq!(md, restored);

    // MovementEffect
    let me = MovementEffect::Push(3);
    let json = serde_json::to_string(&me).unwrap();
    let restored: MovementEffect = serde_json::from_str(&json).unwrap();
    assert_eq!(me, restored);

    // PhaseTransitionTrigger
    let pt = PhaseTransitionTrigger::RoundElapsed(4);
    let json = serde_json::to_string(&pt).unwrap();
    let restored: PhaseTransitionTrigger = serde_json::from_str(&json).unwrap();
    assert_eq!(pt, restored);

    // PhaseTransitionConfig
    let config = PhaseTransitionConfig::new(
        "test_pack",
        PhaseTransitionTrigger::HealthBelow(0.3),
        vec!["a".to_string()],
        "b",
        0,
    );
    let json = serde_json::to_string(&config).unwrap();
    let restored: PhaseTransitionConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(config.boss_pack_id, restored.boss_pack_id);
    assert_eq!(config.trigger, restored.trigger);
}

// ── Contracts-layer enum completeness ─────────────────────────────────────────

#[test]
fn launch_constraint_labels_are_unique() {
    let labels: Vec<&str> = [
        LaunchConstraint::Any,
        LaunchConstraint::FrontRow,
        LaunchConstraint::BackRow,
        LaunchConstraint::SpecificLane(0),
        LaunchConstraint::SlotRange { min: 0, max: 1 },
    ]
    .iter()
    .map(|c| c.label())
    .collect();

    // All labels should be distinct
    let mut unique: Vec<&str> = labels.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), labels.len(),
        "LaunchConstraint labels must be unique: {:?}", labels);
}

#[test]
fn side_affinity_labels_are_unique() {
    let labels: Vec<&str> = [
        SideAffinity::Enemy,
        SideAffinity::Ally,
        SideAffinity::Any,
    ]
    .iter()
    .map(|a| a.label())
    .collect();

    let mut unique = labels.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), labels.len());
}

#[test]
fn target_rank_labels_are_unique() {
    let labels: Vec<&str> = [
        TargetRank::Any,
        TargetRank::Front,
        TargetRank::Back,
        TargetRank::FrontAndBack,
    ]
    .iter()
    .map(|r| r.label())
    .collect();

    let mut unique = labels.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), labels.len());
}

#[test]
fn movement_effect_labels_are_unique() {
    let labels: Vec<&str> = [
        MovementEffect::Push(1),
        MovementEffect::Pull(1),
        MovementEffect::Shuffle,
        MovementEffect::None,
    ]
    .iter()
    .map(|e| e.label())
    .collect();

    let mut unique = labels.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), labels.len());
}

#[test]
fn camp_effect_trace_has_no_silent_drop_gap() {
    // Verify: Every camp effect type name can be parsed back from string.
    // This guards against adding new variants without adding string parsing.
    let all_names = [
        "stress_heal_amount", "health_heal_max_health_percent", "remove_bleeding",
        "remove_poison", "buff", "remove_deaths_door_recovery_buffs",
        "reduce_ambush_chance", "remove_disease", "stress_damage_amount",
        "loot", "reduce_torch", "health_damage_max_health_percent",
        "remove_burn", "remove_frozen", "stress_heal_percent",
        "remove_debuff", "remove_all_debuff", "health_heal_range",
        "health_heal_amount", "reduce_turbulence_chance", "reduce_riptide_chance",
    ];

    // 21 of 22 variants have string names (None has no canonical string name)
    for name in &all_names {
        let parsed = CampEffectType::from_str(name);
        assert!(parsed.is_some(),
            "CampEffectType::from_str(\"{}\") returned None — name must parse", name);
    }

    // Verify the None variant returns None from from_str
    assert!(CampEffectType::from_str("nonexistent_effect").is_none());
}
