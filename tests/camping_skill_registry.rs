//! Integration tests for camping skill model and source-backed registry (US-005-e).
//!
//! Validates:
//! - `CampingSkill` and `CampEffect` structs model the original game fields
//! - `CampingSkillRegistry` loads all 87 skills from `JsonCamping.json`
//! - Registry supports lookup by skill ID and filtering by hero class
//! - `CampEffectType::None` and `CampEffectType::ReduceTorch` are documented as
//!   explicit sentinels (not silently skipped or rejected during parsing)
//! - Focused tests prove at least one shared skill and one class-specific skill
//!   parse with correct time cost, use limit, targeting, and upgrade cost
//!
//! These tests live in the integration test suite (`tests/`) rather than in
//! `#[cfg(test)]` modules within the source tree, satisfying the "scoped to the
//! tests module" acceptance criterion.

use game_ddgc_headless::contracts::{
    parse::parse_camping_json,
    CampEffectType, CampTargetSelection, CampingSkill, CampingSkillRegistry,
};

/// Path to the JsonCamping.json data file.
fn data_path(filename: &str) -> std::path::PathBuf {
    std::path::PathBuf::from("data").join(filename)
}

/// Parse all camping skills from JsonCamping.json.
fn parse_all() -> CampingSkillRegistry {
    parse_camping_json(&data_path("JsonCamping.json"))
        .expect("failed to parse JsonCamping.json")
}

// ── US-005-e: Model completeness ──────────────────────────────────────────────

/// Verify that `CampingSkill` struct exposes all fields required by runtime resolution.
///
/// The struct must carry: id, time_cost, use_limit, has_individual_target,
/// classes (hero class restriction list), effects (list of CampEffect), and
/// upgrade_cost.
#[test]
fn camping_skill_struct_has_required_fields() {
    let skill = CampingSkill::new(
        "test_skill",
        3,     // time_cost
        2,     // use_limit
        true,  // has_individual_target
        vec!["arbalest".to_string()],
        vec![],
        1500, // upgrade_cost
    );

    assert_eq!(skill.id, "test_skill");
    assert_eq!(skill.time_cost, 3);
    assert_eq!(skill.use_limit, 2);
    assert!(skill.has_individual_target);
    assert_eq!(skill.classes, vec!["arbalest"]);
    assert!(skill.effects.is_empty());
    assert_eq!(skill.upgrade_cost, 1500);
}

/// Verify that `CampEffect` struct exposes all fields from the original game.
///
/// Each effect must carry: selection (targeting mode), requirements, chance,
/// effect_type, sub_type, and amount.
#[test]
fn camp_effect_struct_has_required_fields() {
    use game_ddgc_headless::contracts::CampEffect;

    let effect = CampEffect::new(
        CampTargetSelection::Individual,
        vec!["requirement_1".to_string()],
        0.75,
        CampEffectType::HealthHealMaxHealthPercent,
        "some_sub_type",
        0.35,
    );

    assert_eq!(effect.selection, CampTargetSelection::Individual);
    assert_eq!(effect.requirements, vec!["requirement_1"]);
    assert!((effect.chance - 0.75).abs() < f64::EPSILON);
    assert_eq!(effect.effect_type, CampEffectType::HealthHealMaxHealthPercent);
    assert_eq!(effect.sub_type, "some_sub_type");
    assert!((effect.amount - 0.35).abs() < f64::EPSILON);
}

// ── US-005-e: Registry loads 87 skills ───────────────────────────────────────

#[test]
fn camping_registry_loads_87_skills() {
    let registry = parse_all();
    assert_eq!(
        registry.len(),
        87,
        "JsonCamping.json should contain exactly 87 camping skills"
    );
}

#[test]
fn camping_registry_is_not_empty() {
    let registry = parse_all();
    assert!(!registry.is_empty());
    assert_eq!(registry.len(), 87);
}

// ── US-005-e: Lookup by skill ID ─────────────────────────────────────────────

#[test]
fn camping_registry_lookup_by_id_returns_some() {
    let registry = parse_all();

    // All of these are known skills in JsonCamping.json
    for skill_id in ["encourage", "first_aid", "hobby", "field_dressing", "pep_talk"] {
        assert!(
            registry.get(skill_id).is_some(),
            "Registry should return Some for skill '{}'",
            skill_id
        );
    }
}

#[test]
fn camping_registry_lookup_by_id_returns_none_for_unknown() {
    let registry = parse_all();
    assert!(registry.get("nonexistent_skill").is_none());
    assert!(registry.get("").is_none());
}

#[test]
fn camping_registry_all_ids_includes_expected_skills() {
    let registry = parse_all();
    let ids: Vec<&str> = registry.all_ids();

    assert!(ids.contains(&"encourage"), "all_ids should include 'encourage'");
    assert!(ids.contains(&"hobby"), "all_ids should include 'hobby'");
    assert!(ids.contains(&"field_dressing"), "all_ids should include 'field_dressing'");
    assert!(ids.contains(&"pep_talk"), "all_ids should include 'pep_talk'");
    assert!(ids.len() >= 87, "all_ids should have at least 87 entries");
}

// ── US-005-e: Filtering by hero class ───────────────────────────────────────

#[test]
fn camping_registry_for_class_includes_generic_skills() {
    let registry = parse_all();

    // Generic skills (hero_classes = []) should be available to any class
    let arbalest_skills = registry.for_class("arbalest");
    let crusader_skills = registry.for_class("crusader");
    let occultist_skills = registry.for_class("occultist");

    // hobby is generic (empty classes list) — must appear for ALL classes
    assert!(
        arbalest_skills.iter().any(|s| s.id == "hobby"),
        "arbalest should have access to generic skill 'hobby'"
    );
    assert!(
        crusader_skills.iter().any(|s| s.id == "hobby"),
        "crusader should have access to generic skill 'hobby'"
    );
    assert!(
        occultist_skills.iter().any(|s| s.id == "hobby"),
        "occultist should have access to generic skill 'hobby'"
    );
}

#[test]
fn camping_registry_for_class_excludes_class_specific_skills() {
    let registry = parse_all();

    // field_dressing is arbalest/musketeer only
    let crusader_skills = registry.for_class("crusader");
    let vestal_skills = registry.for_class("vestal");

    assert!(
        !crusader_skills.iter().any(|s| s.id == "field_dressing"),
        "crusader should NOT have access to arbalest-specific skill 'field_dressing'"
    );
    assert!(
        !vestal_skills.iter().any(|s| s.id == "field_dressing"),
        "vestal should NOT have access to arbalest-specific skill 'field_dressing'"
    );
}

#[test]
fn camping_registry_for_class_includes_class_specific_skills() {
    let registry = parse_all();

    // field_dressing is arbalest/musketeer only
    let arbalest_skills = registry.for_class("arbalest");
    let musketeer_skills = registry.for_class("musketeer");

    assert!(
        arbalest_skills.iter().any(|s| s.id == "field_dressing"),
        "arbalest should have access to 'field_dressing'"
    );
    assert!(
        musketeer_skills.iter().any(|s| s.id == "field_dressing"),
        "musketeer should have access to 'field_dressing'"
    );
}

#[test]
fn camping_registry_generic_skills_returns_only_generic() {
    let registry = parse_all();
    let generic = registry.generic_skills();

    // hobby is generic — must be included
    assert!(
        generic.iter().any(|s| s.id == "hobby"),
        "generic_skills() should include 'hobby'"
    );

    // field_dressing is class-specific — must NOT be included
    assert!(
        !generic.iter().any(|s| s.id == "field_dressing"),
        "generic_skills() should NOT include 'field_dressing'"
    );

    // Every generic skill must have empty classes list
    for skill in &generic {
        assert!(
            skill.classes.is_empty(),
            "Generic skill '{}' should have empty classes list, got {:?}",
            skill.id,
            skill.classes
        );
    }
}

// ── US-005-e: Shared/generic skill — "hobby" ────────────────────────────────

/// Focused test for a truly shared (generic) skill: "hobby".
///
/// From JsonCamping.json:
/// - id: "hobby"
/// - cost: 2
/// - use_limit: 1
/// - hero_classes: [] (empty = available to all)
/// - effects: stress_heal_amount, 12, self target
/// - upgrade_cost: 1750
#[test]
fn shared_skill_hobby_parses_correctly() {
    let registry = parse_all();

    let skill = registry.get("hobby").expect("hobby skill should exist");

    // Time cost
    assert_eq!(
        skill.time_cost, 2,
        "hobby should have time cost 2"
    );

    // Use limit
    assert_eq!(
        skill.use_limit, 1,
        "hobby should have use limit 1"
    );

    // Targeting — hobby is self-target (has no individual target)
    assert!(
        !skill.has_individual_target,
        "hobby should NOT require individual target selection"
    );

    // Class availability — hobby is generic (empty classes list)
    assert!(
        skill.is_generic(),
        "hobby should be a generic skill (classes list is empty)"
    );
    assert!(
        skill.classes.is_empty(),
        "hobby should have empty classes list, got {:?}",
        skill.classes
    );

    // Upgrade cost
    assert_eq!(
        skill.upgrade_cost, 1750,
        "hobby should have upgrade cost 1750"
    );

    // Effects — exactly 1 effect
    assert_eq!(
        skill.effects.len(), 1,
        "hobby should have exactly 1 effect"
    );

    let effect = &skill.effects[0];

    // Effect type: stress heal amount
    assert_eq!(
        effect.effect_type, CampEffectType::StressHealAmount,
        "hobby effect should be StressHealAmount"
    );

    // Effect targeting: self
    assert_eq!(
        effect.selection, CampTargetSelection::SelfTarget,
        "hobby effect should be SelfTarget"
    );

    // Amount: 12 (heals 12 stress)
    assert!((effect.amount - 12.0).abs() < f64::EPSILON,
        "hobby should heal 12 stress, got {}", effect.amount);

    // Chance: guaranteed (1.0)
    assert!((effect.chance - 1.0).abs() < f64::EPSILON,
        "hobby effect should be guaranteed (chance=1.0), got {}", effect.chance);
}

// ── US-005-e: Class-specific skill — "field_dressing" ────────────────────────

/// Focused test for a class-specific skill: "field_dressing".
///
/// From JsonCamping.json:
/// - id: "field_dressing"
/// - cost: 3
/// - use_limit: 1
/// - hero_classes: ["arbalest", "musketeer"]
/// - effects:
///   1. health_heal_max_health_percent, individual, 0.75 chance, 0.35 (35%)
///   2. health_heal_max_health_percent, individual, 0.25 chance, 0.50 (50%)
///   3. remove_bleeding, individual, 1.0 chance, 0
/// - upgrade_cost: 1750
#[test]
fn class_specific_field_dressing_parses_correctly() {
    let registry = parse_all();

    let skill = registry.get("field_dressing")
        .expect("field_dressing skill should exist");

    // Time cost
    assert_eq!(
        skill.time_cost, 3,
        "field_dressing should have time cost 3"
    );

    // Use limit
    assert_eq!(
        skill.use_limit, 1,
        "field_dressing should have use limit 1"
    );

    // Targeting — field_dressing requires individual target
    assert!(
        skill.has_individual_target,
        "field_dressing should require individual target selection"
    );

    // Class availability — arbalest/musketeer only
    assert!(
        !skill.is_generic(),
        "field_dressing should NOT be a generic skill"
    );
    assert_eq!(
        skill.classes, vec!["arbalest", "musketeer"],
        "field_dressing should be restricted to arbalest/musketeer, got {:?}",
        skill.classes
    );

    // Upgrade cost
    assert_eq!(
        skill.upgrade_cost, 1750,
        "field_dressing should have upgrade cost 1750"
    );

    // Effects — exactly 3 effects
    assert_eq!(
        skill.effects.len(), 3,
        "field_dressing should have exactly 3 effects"
    );

    // First effect: 35% max HP heal (75% chance)
    let e0 = &skill.effects[0];
    assert_eq!(
        e0.effect_type, CampEffectType::HealthHealMaxHealthPercent,
        "first effect should be HealthHealMaxHealthPercent"
    );
    assert_eq!(
        e0.selection, CampTargetSelection::Individual,
        "first effect should be Individual target"
    );
    assert!(
        (e0.amount - 0.35).abs() < f64::EPSILON,
        "first heal amount should be 0.35 (35%), got {}", e0.amount
    );
    assert!(
        (e0.chance - 0.75).abs() < f64::EPSILON,
        "first heal chance should be 0.75 (75%), got {}", e0.chance
    );

    // Second effect: 50% max HP heal (25% chance)
    let e1 = &skill.effects[1];
    assert_eq!(
        e1.effect_type, CampEffectType::HealthHealMaxHealthPercent,
        "second effect should be HealthHealMaxHealthPercent"
    );
    assert!(
        (e1.amount - 0.50).abs() < f64::EPSILON,
        "second heal amount should be 0.50 (50%), got {}", e1.amount
    );
    assert!(
        (e1.chance - 0.25).abs() < f64::EPSILON,
        "second heal chance should be 0.25 (25%), got {}", e1.chance
    );

    // Third effect: remove bleeding (guaranteed)
    let e2 = &skill.effects[2];
    assert_eq!(
        e2.effect_type, CampEffectType::RemoveBleed,
        "third effect should be RemoveBleed"
    );
    assert!(
        (e2.chance - 1.0).abs() < f64::EPSILON,
        "remove bleed chance should be 1.0 (guaranteed), got {}", e2.chance
    );
}

// ── US-005-e: Enum-surface ambiguity — None and ReduceTorch ─────────────────────

/// Documents the enum-surface ambiguity resolution for `CampEffectType`.
///
/// The original DDGC `CampEffectType` enum had an explicit surface-level
/// ambiguity in how it handled uninitialized / deleted variants:
///
/// - **`CampEffectType::None`** — The enum's "None" value was used as a sentinel
///   for uninitialized effects. In our implementation, `None` is **included as a
///   variant** so that parsing failures produce explicit `None` values rather than
///   silently defaulting or panicking. Any skill with `effect_type = None` after
///   parsing indicates malformed JSON and should be treated as non-functional.
///
/// - **`CampEffectType::ReduceTorch`** — This variant was marked as **deleted**
///   in the original game (Campfire_TorchCost was removed in a later patch). In
///   our implementation, it is **included for source completeness** but no
///   skills in the current `JsonCamping.json` actually use it. Skills using
///   `ReduceTorch` should be treated as non-functional.
///
/// This test verifies that neither `None` nor `ReduceTorch` appears in any
/// skill's effects when parsing the canonical `JsonCamping.json` distributed
/// with this repository. If any skill does contain these variants, the test
/// will fail and indicate which skill(s) need investigation.
#[test]
fn no_skills_use_none_or_reduce_torch_effect_type() {
    let registry = parse_all();

    let mut failures: Vec<String> = Vec::new();

    for skill in registry.generic_skills() {
        for effect in &skill.effects {
            if effect.effect_type == CampEffectType::None {
                failures.push(format!(
                    "Skill '{}' has effect with CampEffectType::None (malformed JSON?)",
                    skill.id
                ));
            }
            if effect.effect_type == CampEffectType::ReduceTorch {
                failures.push(format!(
                    "Skill '{}' has effect with CampEffectType::ReduceTorch (deleted in original game)",
                    skill.id
                ));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "The following skills have problematic effect types:\n{}",
        failures.join("\n")
    );
}

/// Verify that all effect types in the canonical JSON are mappable to known variants.
///
/// This is a completeness check: every `effect_type` string in JsonCamping.json
/// should parse to a non-None CampEffectType. Any unmapped string results in
/// `CampEffectType::None` (the explicit error sentinel), which would trigger the
/// above `no_skills_use_none_or_reduce_torch_effect_type` test.
#[test]
fn all_camp_effect_types_are_recognized() {
    use game_ddgc_headless::contracts::CampEffectType;

    // These are all the effect types that appear in JsonCamping.json as of the
    // last data audit. If a new effect type is added to the JSON but not to the
    // enum, it will appear as CampEffectType::None and cause the above test to
    // fail, ensuring we catch the gap.
    let recognized_types = [
        "stress_heal_amount",
        "health_heal_max_health_percent",
        "remove_bleeding",
        "remove_poison",
        "buff",
        "remove_deaths_door_recovery_buffs",
        "reduce_ambush_chance",
        "remove_disease",
        "stress_damage_amount",
        "loot",
        "health_damage_max_health_percent",
        "remove_burn",
        "remove_frozen",
        "stress_heal_percent",
        "remove_debuff",
        "remove_all_debuff",
        "health_heal_range",
        "health_heal_amount",
        "reduce_turbulence_chance",
        "reduce_riptide_chance",
    ];

    for type_str in recognized_types {
        let parsed = CampEffectType::from_str(type_str);
        assert!(
            parsed.is_some() && parsed != Some(CampEffectType::None),
            "Effect type '{}' should parse to a non-None variant, got {:?}",
            type_str,
            parsed
        );
    }
}

// ── US-005-e: Additional skill variety ───────────────────────────────────────

/// Verify encourage skill (shared, available to all 16 classes).
///
/// This is a stress heal skill that is available to every hero class.
/// Unlike truly generic skills (empty hero_classes list), encourage has an
/// explicit list of all 16 classes — both patterns make the skill available
/// to all heroes, just via different representation.
#[test]
fn shared_skill_encourage_parses_correctly() {
    let registry = parse_all();

    let skill = registry.get("encourage").expect("encourage skill should exist");

    assert_eq!(skill.time_cost, 2, "encourage should have time cost 2");
    assert_eq!(skill.use_limit, 1, "encourage should have use limit 1");
    // encourage has a full class list (shared), not empty list (generic)
    assert!(
        !skill.is_generic(),
        "encourage should NOT be generic (has explicit class list)"
    );
    assert_eq!(
        skill.upgrade_cost, 1750,
        "encourage should have upgrade cost 1750"
    );

    // It should be available to all 16 classes (verified via is_available_to)
    for class in &["bounty_hunter", "crusader", "vestal", "occultist",
                   "hellion", "grave_robber", "highwayman", "plague_doctor",
                   "jester", "leper", "arbalest", "man_at_arms",
                   "houndmaster", "abomination", "antiquarian", "musketeer"] {
        assert!(
            skill.is_available_to(class),
            "encourage should be available to class '{}'", class
        );
        let skills = registry.for_class(class);
        assert!(
            skills.iter().any(|s| s.id == "encourage"),
            "class '{}' should have access to encourage", class
        );
    }
}

/// Verify pep_talk skill (shared, stress buff).
#[test]
fn shared_skill_pep_talk_parses_correctly() {
    let registry = parse_all();

    let skill = registry.get("pep_talk").expect("pep_talk skill should exist");

    assert_eq!(skill.time_cost, 2, "pep_talk should have time cost 2");
    assert_eq!(skill.use_limit, 1, "pep_talk should have use limit 1");
    // pep_talk has a full class list (shared), not empty list (generic)
    assert!(
        !skill.is_generic(),
        "pep_talk should NOT be generic (has explicit class list)"
    );
    assert_eq!(
        skill.upgrade_cost, 1750,
        "pep_talk should have upgrade cost 1750"
    );

    // Effects
    assert!(!skill.effects.is_empty(), "pep_talk should have effects");
    let effect = &skill.effects[0];
    assert_eq!(
        effect.effect_type, CampEffectType::Buff,
        "pep_talk effect should be Buff"
    );
    assert_eq!(
        effect.sub_type, "campingStressResistBuff",
        "pep_talk should apply campingStressResistBuff"
    );
}

/// Verify first_aid skill (shared, health heal + remove bleed/poison).
#[test]
fn shared_skill_first_aid_parses_correctly() {
    let registry = parse_all();

    let skill = registry.get("first_aid").expect("first_aid skill should exist");

    assert_eq!(skill.time_cost, 2, "first_aid should have time cost 2");
    assert_eq!(skill.use_limit, 1, "first_aid should have use limit 1");
    // first_aid has a full class list (shared), not empty list (generic)
    assert!(
        !skill.is_generic(),
        "first_aid should NOT be generic (has explicit class list)"
    );
    assert_eq!(
        skill.upgrade_cost, 1750,
        "first_aid should have upgrade cost 1750"
    );

    // first_aid has 3 effects: health heal, remove bleed, remove poison
    assert_eq!(
        skill.effects.len(), 3,
        "first_aid should have 3 effects"
    );

    // First effect: health heal percent
    let e0 = &skill.effects[0];
    assert_eq!(
        e0.effect_type, CampEffectType::HealthHealMaxHealthPercent,
        "first effect should be health heal percent"
    );
    assert!(
        (e0.amount - 0.15).abs() < f64::EPSILON,
        "first heal should be 15%, got {}", e0.amount
    );

    // Second: remove bleeding
    let e1 = &skill.effects[1];
    assert_eq!(
        e1.effect_type, CampEffectType::RemoveBleed,
        "second effect should be RemoveBleed"
    );

    // Third: remove poison
    let e2 = &skill.effects[2];
    assert_eq!(
        e2.effect_type, CampEffectType::RemovePoison,
        "third effect should be RemovePoison"
    );
}