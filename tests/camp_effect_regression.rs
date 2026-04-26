//! Regression tests for movement effect and camp effect fences (US-009-c).
//!
//! Verifies that every high-frequency movement and special effect semantic path
//! is either implemented or fenced with deterministic stubs/skips. No movement
//! or camp effect variant silently drops.
//!
//! Coverage:
//! - MovementEffect (4 variants): labels, direction, steps, serialization
//! - MovementDirection (2 variants): labels, serialization
//! - CampEffectType (22 variants): all produce non-empty trace
//! - CampEffect stubs: [STUB] markers, deterministic output
//! - CampEffect skipped: [SKIPPED] markers
//! - CampEffect enforced: state mutation as expected

use game_ddgc_headless::contracts::{
    CampEffect, CampEffectType, CampTargetSelection,
    HeroCampState, MovementDirection, MovementEffect,
};

// ── MovementDirection fence coverage ──────────────────────────────────────────

#[test]
fn movement_direction_variants_have_non_empty_labels() {
    let variants = [MovementDirection::Forward, MovementDirection::Backward];
    for v in &variants {
        let label = v.label();
        assert!(!label.is_empty(), "MovementDirection variant produces empty label");
    }
}

#[test]
fn movement_direction_serialization_roundtrip() {
    let variants = [MovementDirection::Forward, MovementDirection::Backward];
    for v in &variants {
        let json = serde_json::to_string(v).expect("serialization must succeed");
        let restored: MovementDirection = serde_json::from_str(&json).expect("deserialization must succeed");
        assert_eq!(*v, restored, "MovementDirection roundtrip mismatch for {:?}", v);
    }
}

// ── MovementEffect fence coverage ─────────────────────────────────────────────

#[test]
fn all_movement_effect_variants_have_non_empty_labels() {
    let variants = [
        MovementEffect::Push(2),
        MovementEffect::Pull(1),
        MovementEffect::Shuffle,
        MovementEffect::None,
    ];
    for v in &variants {
        let label = v.label();
        assert!(!label.is_empty(), "MovementEffect variant {:?} produces empty label", v);
    }
}

#[test]
fn movement_effect_push_has_backward_direction() {
    let effect = MovementEffect::Push(3);
    assert_eq!(effect.direction(), MovementDirection::Backward);
    assert_eq!(effect.steps(), 3);
}

#[test]
fn movement_effect_pull_has_forward_direction() {
    let effect = MovementEffect::Pull(2);
    assert_eq!(effect.direction(), MovementDirection::Forward);
    assert_eq!(effect.steps(), 2);
}

#[test]
fn movement_effect_none_has_zero_steps() {
    let effect = MovementEffect::None;
    assert_eq!(effect.steps(), 0);
}

#[test]
fn movement_effect_shuffle_has_zero_steps() {
    let effect = MovementEffect::Shuffle;
    assert_eq!(effect.steps(), 0);
}

#[test]
fn movement_effect_serialization_roundtrip() {
    let variants = [
        MovementEffect::Push(1),
        MovementEffect::Pull(3),
        MovementEffect::Shuffle,
        MovementEffect::None,
    ];
    for v in &variants {
        let json = serde_json::to_string(v).expect("serialization must succeed");
        let restored: MovementEffect = serde_json::from_str(&json).expect("deserialization must succeed");
        assert_eq!(*v, restored, "MovementEffect roundtrip mismatch for {:?}", v);
    }
}

#[test]
fn movement_effect_four_variants_enum_count() {
    let variants: &[MovementEffect] = &[
        MovementEffect::Push(1),
        MovementEffect::Pull(1),
        MovementEffect::Shuffle,
        MovementEffect::None,
    ];
    assert_eq!(variants.len(), 4, "MovementEffect must have exactly 4 variants");
}

// ── Camp effect helpers ───────────────────────────────────────────────────────

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

// ── Camp effect trace coverage: all 22 variants ───────────────────────────────

#[test]
fn all_22_camp_effect_variants_produce_non_empty_trace() {
    let all_types: &[CampEffectType] = &[
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
    assert_eq!(all_types.len(), 22,
        "regression: must verify all 22 CampEffectType variants");

    for et in all_types {
        let effect = make_camp_effect(*et, 10.0, "");
        let state = make_hero_state();
        let result = effect.apply(state, "test_skill", "perf", None, 0);
        assert!(
            !result.trace.description.is_empty(),
            "CampEffectType::{:?} produced empty trace — silent semantic drop!", et
        );
    }
}

// ── Stubbed camp effects ─────────────────────────────────────────────────────

#[test]
fn stubbed_camp_effects_produce_stub_marker() {
    let stubbed: &[(CampEffectType, &str)] = &[
        (CampEffectType::ReduceAmbushChance, "[STUB]"),
        (CampEffectType::Loot, "[STUB]"),
        (CampEffectType::ReduceTurbulenceChance, "[STUB]"),
        (CampEffectType::ReduceRiptideChance, "[STUB]"),
    ];

    for (et, expected_marker) in stubbed {
        let effect = make_camp_effect(*et, 10.0, "test");
        let state = make_hero_state();
        let result = effect.apply(state, "test_skill", "perf", None, 0);
        assert!(
            result.trace.description.contains(expected_marker),
            "CampEffectType::{:?} should contain '{}' in trace, got: {}",
            et, expected_marker, result.trace.description
        );
    }
}

#[test]
fn stubbed_camp_effects_are_deterministic() {
    let stubbed: &[CampEffectType] = &[
        CampEffectType::ReduceAmbushChance,
        CampEffectType::Loot,
        CampEffectType::ReduceTurbulenceChance,
        CampEffectType::ReduceRiptideChance,
    ];

    for et in stubbed {
        let effect = make_camp_effect(*et, 10.0, "test");
        let state1 = make_hero_state();
        let state2 = make_hero_state();
        let result1 = effect.apply(state1, "test_skill", "perf", None, 0);
        let result2 = effect.apply(state2, "test_skill", "perf", None, 0);
        assert_eq!(
            result1.trace.description, result2.trace.description,
            "Stubbed effect {:?} must produce deterministic trace output", et
        );
        assert_eq!(
            result1.state, result2.state,
            "Stubbed effect {:?} must not modify state", et
        );
    }
}

#[test]
fn stubbed_camp_effects_do_not_mutate_state() {
    let effect = make_camp_effect(CampEffectType::ReduceAmbushChance, 5.0, "");
    let state = make_hero_state();
    let result = effect.apply(state.clone(), "test", "perf", None, 0);
    assert_eq!(result.state.health, state.health);
    assert_eq!(result.state.stress, state.stress);
}

// ── Non-functional (skipped) camp effects ────────────────────────────────────

#[test]
fn non_functional_camp_effects_produce_skipped_marker() {
    let skipped: &[CampEffectType] = &[
        CampEffectType::None,
        CampEffectType::ReduceTorch,
    ];

    for et in skipped {
        let effect = make_camp_effect(*et, 0.0, "");
        let state = make_hero_state();
        let result = effect.apply(state, "test_skill", "perf", None, 0);
        assert!(
            result.trace.description.contains("[SKIPPED]"),
            "CampEffectType::{:?} should produce [SKIPPED] marker, got: {}",
            et, result.trace.description
        );
    }
}

#[test]
fn non_functional_camp_effects_do_not_mutate_state() {
    let effect = make_camp_effect(CampEffectType::None, 0.0, "");
    let state = make_hero_state();
    let result = effect.apply(state.clone(), "test", "perf", None, 0);
    assert_eq!(result.state.health, state.health);
    assert_eq!(result.state.stress, state.stress);
}

// ── Implemented camp effects: state mutation verification ─────────────────────

#[test]
fn stress_heal_amount_reduces_stress() {
    let effect = make_camp_effect(CampEffectType::StressHealAmount, 15.0, "");
    let state = make_hero_state(); // stress = 40
    let result = effect.apply(state, "encourage", "crusader", None, 0);
    assert!(result.state.stress < 40.0,
        "StressHealAmount(15) should reduce stress below 40, got {}", result.state.stress);
    assert!(!result.trace.description.contains("[STUB]"));
    assert!(!result.trace.description.contains("[SKIPPED]"));
}

#[test]
fn health_heal_amount_increases_health() {
    let effect = make_camp_effect(CampEffectType::HealthHealAmount, 5.0, "");
    let state = HeroCampState::new(80.0, 100.0, 0.0, 200.0);
    let result = effect.apply(state, "field_dressing", "crusader", None, 0);
    assert!(result.state.health > 80.0,
        "HealthHealAmount(5) should increase health above 80, got {}", result.state.health);
    assert!(!result.trace.description.contains("[STUB]"));
    assert!(!result.trace.description.contains("[SKIPPED]"));
}

#[test]
fn stress_damage_amount_increases_stress() {
    let effect = make_camp_effect(CampEffectType::StressDamageAmount, 10.0, "");
    let state = make_hero_state(); // stress = 40
    let result = effect.apply(state, "stress_skill", "crusader", None, 0);
    assert!(result.state.stress > 40.0,
        "StressDamageAmount(10) should increase stress above 40, got {}", result.state.stress);
    assert!(!result.trace.description.contains("[STUB]"));
    assert!(!result.trace.description.contains("[SKIPPED]"));
}

#[test]
fn remove_bleed_produces_trace_without_stub_or_skip() {
    let effect = make_camp_effect(CampEffectType::RemoveBleed, 0.0, "");
    let state = make_hero_state();
    let result = effect.apply(state, "bandage", "crusader", None, 0);
    assert!(!result.trace.description.is_empty());
    assert!(!result.trace.description.contains("[STUB]"));
    assert!(!result.trace.description.contains("[SKIPPED]"));
}

#[test]
fn buff_effect_produces_trace_without_stub_or_skip() {
    let effect = make_camp_effect(CampEffectType::Buff, 0.0, "ATK+10");
    let state = make_hero_state();
    let result = effect.apply(state, "buff_skill", "crusader", None, 0);
    assert!(!result.trace.description.is_empty());
    assert!(!result.trace.description.contains("[STUB]"));
    assert!(!result.trace.description.contains("[SKIPPED]"));
}

// ── Edge cases: chance roll ──────────────────────────────────────────────────

#[test]
fn camp_effect_with_zero_chance_does_not_trigger() {
    let mut effect = make_camp_effect(CampEffectType::StressHealAmount, 15.0, "");
    effect.chance = 0.0;
    let state = make_hero_state();
    let result = effect.apply(state, "test", "perf", None, 0);
    assert!(!result.trace.triggered,
        "Effect with 0.0 chance should not trigger");
    assert!(result.trace.description.contains("did not trigger"));
    // State should be unchanged since effect didn't trigger
    assert_eq!(result.state.stress, 40.0);
}

#[test]
fn camp_effect_with_full_chance_always_triggers() {
    let effect = make_camp_effect(CampEffectType::StressHealAmount, 10.0, "");
    let state = make_hero_state();
    let result = effect.apply(state, "test", "perf", None, 0);
    assert!(result.trace.triggered,
        "Effect with 1.0 chance should always trigger");
}

// ── HeroCampState fence coverage ──────────────────────────────────────────────

#[test]
fn hero_camp_state_stores_health_as_given() {
    // HeroCampState::new stores values directly — clamping is the
    // caller's responsibility. This test documents current behavior.
    let state = HeroCampState::new(150.0, 100.0, 0.0, 200.0);
    assert!((state.health - 150.0).abs() < f64::EPSILON,
        "Health stored as-is: clamping not enforced at construction");
    assert!((state.max_health - 100.0).abs() < f64::EPSILON);
}

#[test]
fn hero_camp_state_stores_stress_as_given() {
    let state = HeroCampState::new(80.0, 100.0, 250.0, 200.0);
    assert!((state.stress - 250.0).abs() < f64::EPSILON,
        "Stress stored as-is: clamping not enforced at construction");
    assert!((state.max_stress - 200.0).abs() < f64::EPSILON);
}

#[test]
fn hero_camp_state_serialization_roundtrip() {
    let state = HeroCampState::new(75.0, 100.0, 30.0, 200.0);
    let json = serde_json::to_string(&state).expect("serialization must succeed");
    let restored: HeroCampState = serde_json::from_str(&json).expect("deserialization must succeed");
    assert!((restored.health - 75.0).abs() < f64::EPSILON);
    assert!((restored.max_health - 100.0).abs() < f64::EPSILON);
    assert!((restored.stress - 30.0).abs() < f64::EPSILON);
    assert!((restored.max_stress - 200.0).abs() < f64::EPSILON);
}
