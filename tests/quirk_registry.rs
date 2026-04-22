//! Integration test for quirk registry (US-016).
//!
//! Validates:
//! - QuirkRegistry holds all DDGC quirk definitions parsed from JsonQuirks.json
//! - At least 10 quirks (mix of positive, negative, disease) are parsed
//! - Focused test proves quirk lookup by ID works
//! - Focused test proves positive/negative/disease classification
//! - Focused test proves buff resolution produces correct modifiers

use game_ddgc_headless::contracts::{
    parse::parse_quirks_json,
    BuffRegistry, QuirkClassification, QuirkRegistry,
};

fn data_path(filename: &str) -> std::path::PathBuf {
    std::path::PathBuf::from("data").join(filename)
}

fn parse_all() -> QuirkRegistry {
    parse_quirks_json(&data_path("JsonQuirks.json"))
        .expect("failed to parse JsonQuirks.json")
}

// ── US-016: Registry lookup by ID ───────────────────────────────────────────

#[test]
fn quirk_registry_lookup_by_id_works() {
    let quirks = parse_all();

    // Verify specific quirk IDs exist
    assert!(quirks.get("quick_reflexes").is_some(), "quick_reflexes should exist");
    assert!(quirks.get("iron_will").is_some(), "iron_will should exist");
    assert!(quirks.get("consumptive").is_some(), "consumptive should exist");
    assert!(quirks.get("bloated").is_some(), "bloated should exist");
}

#[test]
fn quirk_registry_returns_none_for_unknown_id() {
    let quirks = parse_all();

    assert!(quirks.get("nonexistent_quirk").is_none(), "unknown quirk should return None");
}

// ── US-016: Positive/Negative/Disease classification ────────────────────────

#[test]
fn quirk_registry_positive_quirks_filter() {
    let quirks = parse_all();

    let positive = quirks.positive_quirks();
    assert!(
        positive.len() >= 5,
        "At least 5 positive quirks should exist, got {}",
        positive.len()
    );

    // All returned quirks should be positive
    for quirk in &positive {
        assert!(
            quirk.is_positive,
            "Quirk {} should be positive",
            quirk.id
        );
    }
}

#[test]
fn quirk_registry_negative_quirks_filter() {
    let quirks = parse_all();

    let negative = quirks.negative_quirks();
    assert!(
        negative.len() >= 5,
        "At least 5 negative quirks should exist, got {}",
        negative.len()
    );

    // All returned quirks should be negative
    for quirk in &negative {
        assert!(
            !quirk.is_positive,
            "Quirk {} should be negative",
            quirk.id
        );
    }
}

#[test]
fn quirk_registry_diseases_filter() {
    let quirks = parse_all();

    let diseases = quirks.diseases();
    assert!(
        diseases.len() >= 2,
        "At least 2 diseases should exist, got {}",
        diseases.len()
    );

    // All returned quirks should be diseases
    for quirk in &diseases {
        assert!(
            quirk.is_disease,
            "Quirk {} should be a disease",
            quirk.id
        );
        assert!(
            !quirk.is_positive,
            "Disease {} should be negative",
            quirk.id
        );
    }
}

#[test]
fn quirk_classification_by_type() {
    let quirks = parse_all();

    // Check specific quirks have correct classification
    let quick_reflexes = quirks.get("quick_reflexes").unwrap();
    assert_eq!(quick_reflexes.classification, QuirkClassification::Talent);
    assert!(quick_reflexes.is_positive);
    assert!(!quick_reflexes.is_disease);

    let consumptive = quirks.get("consumptive").unwrap();
    assert_eq!(consumptive.classification, QuirkClassification::Disease);
    assert!(!consumptive.is_positive);
    assert!(consumptive.is_disease);

    let weak_will = quirks.get("weak_will").unwrap();
    assert_eq!(weak_will.classification, QuirkClassification::Personality);
    assert!(!weak_will.is_positive);
    assert!(!weak_will.is_disease);
}

// ── US-016: Buff resolution ─────────────────────────────────────────────────

#[test]
fn quirk_registry_buff_resolution_quick_reflexes() {
    let quirks = parse_all();
    let buff_registry = BuffRegistry::new();

    // quick_reflexes: SPD+5, DODGE+8
    let modifiers = quirks.resolve_quirk_buffs("quick_reflexes", &buff_registry);
    assert_eq!(modifiers.len(), 2);

    let spd = modifiers.iter().find(|m| m.attribute_key == "SPD").unwrap();
    assert_eq!(spd.value, 5.0);

    let dodge = modifiers.iter().find(|m| m.attribute_key == "DODGE").unwrap();
    assert_eq!(dodge.value, 8.0);
}

#[test]
fn quirk_registry_buff_resolution_iron_will() {
    let quirks = parse_all();
    let buff_registry = BuffRegistry::new();

    // iron_will: RESIST_STUN+15, RESIST_BLIND+15, STRESSRES+10
    let modifiers = quirks.resolve_quirk_buffs("iron_will", &buff_registry);
    assert_eq!(modifiers.len(), 3);

    let resist_stun = modifiers.iter().find(|m| m.attribute_key == "RESIST_STUN").unwrap();
    assert_eq!(resist_stun.value, 15.0);

    let resist_blind = modifiers.iter().find(|m| m.attribute_key == "RESIST_BLIND").unwrap();
    assert_eq!(resist_blind.value, 15.0);
}

#[test]
fn quirk_registry_buff_resolution_consumptive() {
    let quirks = parse_all();
    let buff_registry = BuffRegistry::new();

    // consumptive: MAXHP-20, DEF-5, SPD-3
    let modifiers = quirks.resolve_quirk_buffs("consumptive", &buff_registry);
    assert_eq!(modifiers.len(), 3);

    let maxhp = modifiers.iter().find(|m| m.attribute_key == "MAXHP").unwrap();
    assert_eq!(maxhp.value, -20.0);

    let def = modifiers.iter().find(|m| m.attribute_key == "DEF").unwrap();
    assert_eq!(def.value, -5.0);

    let spd = modifiers.iter().find(|m| m.attribute_key == "SPD").unwrap();
    assert_eq!(spd.value, -3.0);
}

#[test]
fn quirk_registry_buff_resolution_unknown_quirk_returns_empty() {
    let quirks = parse_all();
    let buff_registry = BuffRegistry::new();

    let modifiers = quirks.resolve_quirk_buffs("nonexistent_quirk", &buff_registry);
    assert!(modifiers.is_empty());
}

// ── US-016: Incompatible quirks ─────────────────────────────────────────────

#[test]
fn quirk_registry_incompatible_quirks_preserved() {
    let quirks = parse_all();

    // quick_reflexes incompatible with clumsy
    let quick_reflexes = quirks.get("quick_reflexes").unwrap();
    assert!(quick_reflexes.incompatible_quirks.contains(&"clumsy".to_string()));

    // clumsy incompatible with quick_reflexes
    let clumsy = quirks.get("clumsy").unwrap();
    assert!(clumsy.incompatible_quirks.contains(&"quick_reflexes".to_string()));
}

// ── US-016: Curio tag ───────────────────────────────────────────────────────

#[test]
fn quirk_registry_curio_tag_preserved() {
    let quirks = parse_all();

    let quick_reflexes = quirks.get("quick_reflexes").unwrap();
    assert_eq!(quick_reflexes.curio_tag, "mirror");

    let consumptive = quirks.get("consumptive").unwrap();
    assert_eq!(consumptive.curio_tag, "咳嗽");
}

// ── US-016: Registry size ───────────────────────────────────────────────────

#[test]
fn quirk_registry_has_at_least_10_quirks() {
    let quirks = parse_all();
    assert!(
        quirks.len() >= 10,
        "At least 10 quirks should be parsed, got {}",
        quirks.len()
    );
}
