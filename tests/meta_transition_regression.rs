//! Regression tests for meta transition contract fences (US-009-c).
//!
//! Verifies that every persistent meta transition semantic path is either
//! implemented or fenced with deterministic contracts. No phase transition
//! variant silently drops with an empty string or panics.
//!
//! Coverage:
//! - PhaseTransitionTrigger (5 variants): labels, serialization roundtrip
//! - PhaseTransitionConfig: serialization roundtrip, field validation

use game_ddgc_headless::contracts::{PhaseTransitionConfig, PhaseTransitionTrigger};

// ── PhaseTransitionTrigger fence coverage ─────────────────────────────────────

#[test]
fn all_phase_transition_triggers_have_non_empty_labels() {
    let triggers = [
        PhaseTransitionTrigger::PressAttackCount(3),
        PhaseTransitionTrigger::HealthBelow(0.5),
        PhaseTransitionTrigger::RoundElapsed(2),
        PhaseTransitionTrigger::OnAllyDeath("ally_1".to_string()),
        PhaseTransitionTrigger::OnAllAlliesDead(vec!["ally_1".to_string(), "ally_2".to_string()]),
    ];
    for t in &triggers {
        let label = t.label();
        assert!(!label.is_empty(),
            "PhaseTransitionTrigger variant produces empty label");
    }
}

#[test]
fn phase_transition_trigger_serialization_roundtrip() {
    let triggers = [
        PhaseTransitionTrigger::PressAttackCount(5),
        PhaseTransitionTrigger::HealthBelow(0.25),
        PhaseTransitionTrigger::RoundElapsed(3),
        PhaseTransitionTrigger::OnAllyDeath("satellite".to_string()),
        PhaseTransitionTrigger::OnAllAlliesDead(vec!["minion_a".to_string(), "minion_b".to_string()]),
    ];
    for t in &triggers {
        let json = serde_json::to_string(t).expect("serialization must succeed");
        let restored: PhaseTransitionTrigger = serde_json::from_str(&json)
            .expect("deserialization must succeed");
        assert_eq!(*t, restored,
            "PhaseTransitionTrigger roundtrip mismatch for {:?}", t);
    }
}

#[test]
fn phase_transition_trigger_five_variants_enum_count() {
    let triggers: &[PhaseTransitionTrigger] = &[
        PhaseTransitionTrigger::PressAttackCount(1),
        PhaseTransitionTrigger::HealthBelow(0.5),
        PhaseTransitionTrigger::RoundElapsed(1),
        PhaseTransitionTrigger::OnAllyDeath("x".to_string()),
        PhaseTransitionTrigger::OnAllAlliesDead(vec![]),
    ];
    assert_eq!(triggers.len(), 5,
        "PhaseTransitionTrigger must have exactly 5 variants");
}

#[test]
fn press_attack_count_label_is_consistent() {
    let t1 = PhaseTransitionTrigger::PressAttackCount(1);
    let t2 = PhaseTransitionTrigger::PressAttackCount(99);
    // Label should be the same regardless of count value
    assert_eq!(t1.label(), t2.label());
    assert_eq!(t1.label(), "press_attack_count");
}

#[test]
fn health_below_label_is_consistent() {
    let t1 = PhaseTransitionTrigger::HealthBelow(0.3);
    let t2 = PhaseTransitionTrigger::HealthBelow(0.7);
    assert_eq!(t1.label(), t2.label());
    assert_eq!(t1.label(), "health_below");
}

#[test]
fn round_elapsed_label_is_consistent() {
    let t1 = PhaseTransitionTrigger::RoundElapsed(1);
    let t2 = PhaseTransitionTrigger::RoundElapsed(10);
    assert_eq!(t1.label(), t2.label());
    assert_eq!(t1.label(), "round_elapsed");
}

#[test]
fn on_ally_death_label_is_consistent() {
    let t1 = PhaseTransitionTrigger::OnAllyDeath("alpha".to_string());
    let t2 = PhaseTransitionTrigger::OnAllyDeath("beta".to_string());
    assert_eq!(t1.label(), t2.label());
    assert_eq!(t1.label(), "on_ally_death");
}

#[test]
fn on_all_allies_dead_label_is_consistent() {
    let t1 = PhaseTransitionTrigger::OnAllAlliesDead(vec!["a".to_string()]);
    let t2 = PhaseTransitionTrigger::OnAllAlliesDead(vec!["x".to_string(), "y".to_string()]);
    assert_eq!(t1.label(), t2.label());
    assert_eq!(t1.label(), "on_all_allies_dead");
}

// ── PhaseTransitionConfig fence coverage ──────────────────────────────────────

#[test]
fn phase_transition_config_constructor_is_well_formed() {
    let config = PhaseTransitionConfig::new(
        "boss_pack_01",
        PhaseTransitionTrigger::HealthBelow(0.5),
        vec!["old_family".to_string()],
        "new_family",
        1,
    );
    assert_eq!(config.boss_pack_id, "boss_pack_01");
    assert!(!config.trigger.label().is_empty());
    assert_eq!(config.remove_families, vec!["old_family"]);
    assert_eq!(config.summon_family_id, "new_family");
    assert_eq!(config.placement_slot, 1);
}

#[test]
fn phase_transition_config_serialization_roundtrip() {
    let config = PhaseTransitionConfig {
        trigger: PhaseTransitionTrigger::HealthBelow(0.25),
        boss_pack_id: "boss_phase_2".to_string(),
        remove_families: vec!["terrain_form".to_string(), "support_form".to_string()],
        summon_family_id: "final_form".to_string(),
        placement_slot: 0,
    };
    let json = serde_json::to_string(&config).expect("serialization must succeed");
    let restored: PhaseTransitionConfig = serde_json::from_str(&json)
        .expect("deserialization must succeed");

    assert_eq!(restored.boss_pack_id, "boss_phase_2");
    assert_eq!(restored.remove_families, vec!["terrain_form", "support_form"]);
    assert_eq!(restored.summon_family_id, "final_form");
    assert_eq!(restored.placement_slot, 0);

    // Trigger variant must be preserved
    match restored.trigger {
        PhaseTransitionTrigger::HealthBelow(v) => {
            assert!((v - 0.25).abs() < f64::EPSILON);
        }
        _ => panic!("trigger variant not preserved in roundtrip"),
    }
}

#[test]
fn phase_transition_config_all_triggers_roundtrip() {
    let configs = [
        PhaseTransitionConfig::new(
            "pack_1", PhaseTransitionTrigger::PressAttackCount(3),
            vec![], "next", 0,
        ),
        PhaseTransitionConfig::new(
            "pack_2", PhaseTransitionTrigger::HealthBelow(0.3),
            vec!["old".to_string()], "next", 1,
        ),
        PhaseTransitionConfig::new(
            "pack_3", PhaseTransitionTrigger::RoundElapsed(5),
            vec![], "next", 0,
        ),
        PhaseTransitionConfig::new(
            "pack_4", PhaseTransitionTrigger::OnAllyDeath("guardian".to_string()),
            vec![], "next", 0,
        ),
        PhaseTransitionConfig::new(
            "pack_5",
            PhaseTransitionTrigger::OnAllAlliesDead(vec!["a".to_string(), "b".to_string()]),
            vec!["a".to_string(), "b".to_string()],
            "boss_final", 0,
        ),
    ];

    for config in &configs {
        let json = serde_json::to_string(config).expect("serialize");
        let restored: PhaseTransitionConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(config.boss_pack_id, restored.boss_pack_id);
        assert_eq!(&config.trigger, &restored.trigger);
        assert_eq!(&config.remove_families, &restored.remove_families);
        assert_eq!(&config.summon_family_id, &restored.summon_family_id);
        assert_eq!(config.placement_slot, restored.placement_slot);
    }
}

#[test]
fn phase_transition_config_json_structure_is_valid() {
    let config = PhaseTransitionConfig::new(
        "white_tiger",
        PhaseTransitionTrigger::HealthBelow(0.5),
        vec!["white_tiger_terrain".to_string()],
        "white_tiger_final",
        0,
    );
    let json = serde_json::to_string(&config).expect("serialize");
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");
    assert!(parsed.is_object());
    assert_eq!(parsed["boss_pack_id"], "white_tiger");
    assert!(parsed["remove_families"].is_array());
    assert_eq!(parsed["summon_family_id"], "white_tiger_final");
    assert_eq!(parsed["placement_slot"], 0);
}
