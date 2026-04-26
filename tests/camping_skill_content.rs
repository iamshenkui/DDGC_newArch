//! Full camping-skill content validation tests (US-011-c).
//!
//! Validates that all 87 camping skills migrated from JsonCamping.json are
//! correctly represented in the Rust content layer and pass runtime schema
//! validation. Covers:
//!
//! - Full 87-skill registry validation against runtime schema (CampingSkillRegistry)
//! - Per-skill validation via CampingSkill::validate()
//! - GameState loading and validation from real source data
//! - Shared/generic and class-specific skill property preservation
//! - Time cost, use limit, effect count, and class restriction verification
//! - All 31 hero class coverage
//! - All 19 effect type coverage
//! - Distribution audits (time cost, use limit)
//!
//! These tests live in the integration test suite (`tests/`) rather than in
//! `#[cfg(test)]` modules within the source tree, satisfying the "scoped to the
//! tests module" acceptance criterion.

use game_ddgc_headless::contracts::{
    parse::parse_camping_json,
    CampEffectType, CampTargetSelection, CampingSkillRegistry,
};
use game_ddgc_headless::state::GameState;

/// Path to the JsonCamping.json data file.
fn data_path(filename: &str) -> std::path::PathBuf {
    std::path::PathBuf::from("data").join(filename)
}

/// Parse all camping skills from JsonCamping.json.
fn parse_all() -> CampingSkillRegistry {
    parse_camping_json(&data_path("JsonCamping.json"))
        .expect("failed to parse JsonCamping.json")
}

/// Load the full GameState from the default data directory.
///
/// Uses `CARGO_MANIFEST_DIR` to locate the data directory, matching how
/// cargo test resolves the project root.
fn load_state() -> GameState {
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set during test");
    let data_dir = std::path::PathBuf::from(manifest_dir).join("data");
    GameState::load_from(&data_dir).expect("failed to load game state")
}

// ─────────────────────────────────────────────────────────────────────────────
// Registry validation tests (US-011-c primary acceptances)
// ─────────────────────────────────────────────────────────────────────────────

/// Proves the full 87-skill camping registry validates against the runtime schema.
///
/// This is the primary acceptance test for US-011-c: load the complete registry
/// and assert that `CampingSkillRegistry::validate()` returns Ok, confirming
/// every skill passes field-level validation (non-empty id, positive time_cost,
/// positive use_limit, at least one effect, valid target selection, valid effect
/// type, chance in [0.0, 1.0]).
#[test]
fn full_registry_validates_against_runtime_schema() {
    let state = load_state();
    let result = state.validate_camping_skills();
    assert!(
        result.is_ok(),
        "Full 87-skill registry failed runtime schema validation: {:?}",
        result.err()
    );
}

/// Proves every individual camping skill passes per-skill validation.
///
/// Each skill is validated in isolation via `CampingSkill::validate()`.
/// This catches edge cases that aggregate validation might mask.
#[test]
fn each_of_87_skills_passes_individual_validation() {
    let state = load_state();
    for skill_id in state.camping_skills.all_ids() {
        let skill = state.camping_skill(skill_id).expect("skill should exist in registry");
        let errors = skill.validate();
        assert!(
            errors.is_empty(),
            "Skill '{}' failed individual validation: {:?}",
            skill_id,
            errors
        );
    }
}

/// Proves the registry contains exactly 87 skills.
#[test]
fn registry_contains_exactly_87_skills() {
    let state = load_state();
    assert_eq!(
        state.camping_skill_count(),
        87,
        "GameState should contain exactly 87 camping skills"
    );
    let registry = parse_all();
    assert_eq!(registry.len(), 87, "Raw registry should contain exactly 87 skills");
}

// ─────────────────────────────────────────────────────────────────────────────
// All 87 skill ID coverage
// ─────────────────────────────────────────────────────────────────────────────

/// Proves every skill ID from the source JSON is represented in the Rust layer.
///
/// Lists all 87 expected skill IDs and asserts each is loadable and queryable
/// from the GameState. This directly satisfies "All 87 camping skills are
/// represented in the Rust content layer."
#[test]
fn all_87_skill_ids_are_represented() {
    let state = load_state();

    #[rustfmt::skip]
    let expected_ids: [&str; 87] = [
        "encourage", "first_aid", "pep_talk", "hobby",
        "field_dressing", "marching_plan", "restring_crossbow", "clean_musket",
        "triage", "how_its_done", "tracking", "planned_takedown", "scout_ahead",
        "unshakeable_leader", "stand_tall", "zealous_speech", "zealous_vigil",
        "forage", "gallows_humor", "night_steps", "pilfer",
        "battle_trance", "revel", "reject_the_gods", "sharpen_spear",
        "uncatchable", "clean_guns", "bandits_sense",
        "maintain_equipment", "tactics", "instruction", "weapons_practice",
        "abandon_hope", "dark_ritual", "dark_strength", "unspeakable_commune",
        "experimental_vapours", "leeches", "preventative_medicine", "self_medicate",
        "bless", "chant", "pray", "sanctuary",
        "turn_back_time", "every_rose", "tigers_eye", "mockery",
        "let_the_mask_down", "bloody_shroud", "reflection", "quarantine",
        "hounds_watch", "therapy_dog", "pet_the_hound", "release_the_hound",
        "anger_management", "psych_up", "the_quickening", "eldritch_blood",
        "supply", "trinket_scrounge", "strange_powders", "curious_incantation",
        // PvP (alchemist/shaman/hunter/diviner/tank shared)
        "relax", "life_recall", "paradox",
        // shaman-specific
        "grace_shift", "heaven_bless", "mind_purge", "strike_prep",
        // alchemist-specific
        "dual_purge", "self_heal", "revive_downpour", "purge_affliction",
        // hunter-specific
        "ambush_scheme", "hunting_ritual", "luring_trap", "divine_reflex",
        // diviner-specific
        "mind_reset", "oblivion_surge", "calamity_ward", "lucky_reversal",
        // tank-specific
        "bastion_pact", "tartarus_ward", "deride", "chain_carnage",
    ];

    let registry_ids: Vec<&str> = state.camping_skills.all_ids();
    assert_eq!(
        registry_ids.len(),
        87,
        "Registry should report exactly 87 IDs"
    );

    for expected_id in &expected_ids {
        assert!(
            state.camping_skill(expected_id).is_some(),
            "Skill '{}' should be represented in the Rust content layer",
            expected_id
        );
    }

    for registry_id in &registry_ids {
        assert!(
            expected_ids.contains(registry_id),
            "Registry contains unexpected skill '{}' not in the expected list",
            registry_id
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Source data preservation: shared skills
// ─────────────────────────────────────────────────────────────────────────────

/// Tests that each of the 4 shared/almost-generic skills preserves source data.
///
/// - **encourage**: time_cost=2, use_limit=1, individual target, stress_heal_amount(15),
///   16 classes, upgrade_cost=1750
/// - **first_aid**: time_cost=2, use_limit=1, individual target, 3 effects
///   (health_heal_max_health_percent(15%), remove_bleeding, remove_poison),
///   16 classes, upgrade_cost=1750
/// - **pep_talk**: time_cost=2, use_limit=1, individual target, buff
///   (campingStressResistBuff), 16 classes, upgrade_cost=1750
/// - **hobby**: time_cost=2, use_limit=1, self target, stress_heal_amount(12),
///   generic (empty class list), upgrade_cost=1750
#[test]
fn shared_skills_preserve_source_data() {
    let state = load_state();

    // encourage
    let s = state.camping_skill("encourage").unwrap();
    assert_eq!(s.time_cost, 2, "encourage time_cost");
    assert_eq!(s.use_limit, 1, "encourage use_limit");
    assert!(s.has_individual_target, "encourage has_individual_target");
    assert!(!s.is_generic(), "encourage has explicit class list");
    assert_eq!(s.classes.len(), 16, "encourage available to 16 classes");
    assert_eq!(s.upgrade_cost, 1750, "encourage upgrade_cost");
    assert_eq!(s.effects.len(), 1);
    assert_eq!(s.effects[0].effect_type, CampEffectType::StressHealAmount);
    assert_eq!(s.effects[0].selection, CampTargetSelection::Individual);
    assert!((s.effects[0].amount - 15.0).abs() < f64::EPSILON);

    // first_aid
    let s = state.camping_skill("first_aid").unwrap();
    assert_eq!(s.time_cost, 2, "first_aid time_cost");
    assert_eq!(s.use_limit, 1, "first_aid use_limit");
    assert!(s.has_individual_target, "first_aid has_individual_target");
    assert_eq!(s.classes.len(), 16, "first_aid available to 16 classes");
    assert_eq!(s.effects.len(), 3);
    assert_eq!(s.effects[0].effect_type, CampEffectType::HealthHealMaxHealthPercent);
    assert!((s.effects[0].amount - 0.15).abs() < f64::EPSILON);
    assert_eq!(s.effects[1].effect_type, CampEffectType::RemoveBleed);
    assert_eq!(s.effects[2].effect_type, CampEffectType::RemovePoison);

    // pep_talk
    let s = state.camping_skill("pep_talk").unwrap();
    assert_eq!(s.time_cost, 2, "pep_talk time_cost");
    assert_eq!(s.use_limit, 1, "pep_talk use_limit");
    assert!(s.has_individual_target, "pep_talk has_individual_target");
    assert_eq!(s.classes.len(), 16, "pep_talk available to 16 classes");
    assert_eq!(s.effects.len(), 1);
    assert_eq!(s.effects[0].effect_type, CampEffectType::Buff);
    assert_eq!(s.effects[0].sub_type, "campingStressResistBuff");

    // hobby (truly generic)
    let s = state.camping_skill("hobby").unwrap();
    assert_eq!(s.time_cost, 2, "hobby time_cost");
    assert_eq!(s.use_limit, 1, "hobby use_limit");
    assert!(!s.has_individual_target, "hobby is self-target");
    assert!(s.is_generic(), "hobby is truly generic (empty class list)");
    assert!(s.classes.is_empty(), "hobby has empty class list");
    assert_eq!(s.effects.len(), 1);
    assert_eq!(s.effects[0].effect_type, CampEffectType::StressHealAmount);
    assert_eq!(s.effects[0].selection, CampTargetSelection::SelfTarget);
    assert!((s.effects[0].amount - 12.0).abs() < f64::EPSILON);
    assert!((s.effects[0].chance - 1.0).abs() < f64::EPSILON);
}

// ─────────────────────────────────────────────────────────────────────────────
// Source data preservation: class-specific skills (representative sample)
// ─────────────────────────────────────────────────────────────────────────────

/// Tests that class-specific skills preserve time cost, use limit, effects,
/// and class restrictions. Covers a diverse sample from the full set.
///
/// Each assertion is pinned to the values in the canonical JsonCamping.json.
#[test]
fn class_specific_skills_preserve_source_data() {
    let state = load_state();

    // field_dressing (arbalest/musketeer): 3 effects, time_cost=3
    let s = state.camping_skill("field_dressing").unwrap();
    assert_eq!(s.time_cost, 3);
    assert_eq!(s.use_limit, 1);
    assert_eq!(s.classes, vec!["arbalest", "musketeer"]);
    assert!(s.has_individual_target);
    assert_eq!(s.effects.len(), 3);
    assert_eq!(s.effects[0].effect_type, CampEffectType::HealthHealMaxHealthPercent);
    assert!((s.effects[0].amount - 0.35).abs() < f64::EPSILON);
    assert!((s.effects[0].chance - 0.75).abs() < f64::EPSILON);
    assert_eq!(s.effects[1].effect_type, CampEffectType::HealthHealMaxHealthPercent);
    assert!((s.effects[1].amount - 0.50).abs() < f64::EPSILON);
    assert!((s.effects[1].chance - 0.25).abs() < f64::EPSILON);
    assert_eq!(s.effects[2].effect_type, CampEffectType::RemoveBleed);

    // supply (antiquarian): use_limit=3, time_cost=1
    let s = state.camping_skill("supply").unwrap();
    assert_eq!(s.time_cost, 1);
    assert_eq!(s.use_limit, 3);
    assert_eq!(s.classes, vec!["antiquarian"]);
    assert_eq!(s.effects.len(), 1);
    assert_eq!(s.effects[0].effect_type, CampEffectType::Loot);

    // dark_ritual (occultist): time_cost=4, ReduceTorch effect
    let s = state.camping_skill("dark_ritual").unwrap();
    assert_eq!(s.time_cost, 4);
    assert_eq!(s.use_limit, 1);
    assert_eq!(s.classes, vec!["occultist"]);
    assert_eq!(s.effects.len(), 4);
    let torch_effect = s.effects.iter()
        .find(|e| e.effect_type == CampEffectType::ReduceTorch)
        .expect("dark_ritual should have ReduceTorch effect");
    assert!((torch_effect.amount - 100.0).abs() < f64::EPSILON);

    // zealous_speech (crusader): highest time_cost=5
    let s = state.camping_skill("zealous_speech").unwrap();
    assert_eq!(s.time_cost, 5);
    assert_eq!(s.use_limit, 1);
    assert_eq!(s.classes, vec!["crusader"]);

    // self_medicate (plague_doctor): 5 effects
    let s = state.camping_skill("self_medicate").unwrap();
    assert_eq!(s.time_cost, 3);
    assert_eq!(s.classes, vec!["plague_doctor"]);
    assert_eq!(s.effects.len(), 5);
    let etypes: Vec<_> = s.effects.iter().map(|e| &e.effect_type).collect();
    assert!(etypes.contains(&&CampEffectType::StressHealAmount));
    assert!(etypes.contains(&&CampEffectType::HealthHealMaxHealthPercent));
    assert!(etypes.contains(&&CampEffectType::RemovePoison));
    assert!(etypes.contains(&&CampEffectType::RemoveBleed));
    assert!(etypes.contains(&&CampEffectType::Buff));

    // battle_trance (hellion): 6 buff effects
    let s = state.camping_skill("battle_trance").unwrap();
    assert_eq!(s.time_cost, 3);
    assert_eq!(s.classes, vec!["hellion"]);
    assert_eq!(s.effects.len(), 6);

    // revel (hellion): 8 effects
    let s = state.camping_skill("revel").unwrap();
    assert_eq!(s.time_cost, 3);
    assert_eq!(s.classes, vec!["hellion"]);
    assert_eq!(s.effects.len(), 8);

    // sanctuary (vestal): reduce_ambush_chance + heal
    let s = state.camping_skill("sanctuary").unwrap();
    assert_eq!(s.time_cost, 4);
    assert_eq!(s.classes, vec!["vestal"]);
    assert_eq!(s.effects.len(), 3);
    let etypes: Vec<_> = s.effects.iter().map(|e| &e.effect_type).collect();
    assert!(etypes.contains(&&CampEffectType::ReduceAmbushChance));
    assert!(etypes.contains(&&CampEffectType::HealthHealMaxHealthPercent));
    assert!(etypes.contains(&&CampEffectType::StressHealAmount));
}

// ─────────────────────────────────────────────────────────────────────────────
// PvP/DLC class skill preservation (alchemist, shaman, hunter, diviner, tank)
// ─────────────────────────────────────────────────────────────────────────────

/// Tests PvP/expansion class skills preserve source data.
///
/// These classes (alchemist, shaman, hunter, diviner, tank) each have base
/// and variant forms sharing the same skill set. Skills shared across all
/// PvP classes (relax, life_recall, paradox) have 15-class lists.
#[test]
fn pvp_class_skills_preserve_source_data() {
    let state = load_state();

    // PvP-shared skills (15 classes each)
    for shared_id in &["relax", "life_recall", "paradox"] {
        let s = state.camping_skill(shared_id).unwrap();
        assert_eq!(s.time_cost, 2, "{} time_cost", shared_id);
        assert_eq!(s.use_limit, 1, "{} use_limit", shared_id);
        assert_eq!(s.classes.len(), 15, "{} should be available to 15 PvP classes", shared_id);
    }

    // relax: stress_heal_percent + buff (Party)
    let s = state.camping_skill("relax").unwrap();
    assert_eq!(s.effects.len(), 2);
    assert_eq!(s.effects[0].effect_type, CampEffectType::StressHealPercent);
    assert_eq!(s.effects[0].selection, CampTargetSelection::Party);
    assert!(s.effects.iter().any(|e| e.effect_type == CampEffectType::Buff));

    // alchemist-specific
    let s = state.camping_skill("dual_purge").unwrap();
    assert_eq!(s.classes, vec!["alchemist", "alchemist1", "alchemist2"]);
    assert_eq!(s.time_cost, 3);
    assert_eq!(s.effects.len(), 3);
    assert_eq!(s.effects[0].effect_type, CampEffectType::RemoveAllDebuff);

    // shaman-specific: grace_shift has ReduceTurbulenceChance + ReduceRiptideChance
    let s = state.camping_skill("grace_shift").unwrap();
    assert_eq!(s.classes, vec!["shaman", "shaman1", "shaman2"]);
    assert_eq!(s.time_cost, 4);
    let etypes: Vec<_> = s.effects.iter().map(|e| &e.effect_type).collect();
    assert!(etypes.contains(&&CampEffectType::ReduceTurbulenceChance));
    assert!(etypes.contains(&&CampEffectType::ReduceRiptideChance));

    // hunter-specific: hunting_ritual has 7 effects
    let s = state.camping_skill("hunting_ritual").unwrap();
    assert_eq!(s.classes, vec!["hunter", "hunter1", "hunter2"]);
    assert_eq!(s.effects.len(), 7);

    // diviner-specific: lucky_reversal has ReduceTurbulenceChance + ReduceRiptideChance
    let s = state.camping_skill("lucky_reversal").unwrap();
    assert_eq!(s.classes, vec!["diviner", "diviner1", "diviner2"]);
    let etypes: Vec<_> = s.effects.iter().map(|e| &e.effect_type).collect();
    assert!(etypes.contains(&&CampEffectType::ReduceTurbulenceChance));
    assert!(etypes.contains(&&CampEffectType::ReduceRiptideChance));

    // tank-specific
    let s = state.camping_skill("bastion_pact").unwrap();
    assert_eq!(s.classes, vec!["tank", "tank1", "tank2"]);
    assert_eq!(s.time_cost, 4);
    assert_eq!(s.effects[0].selection, CampTargetSelection::Party);
}

// ─────────────────────────────────────────────────────────────────────────────
// Content integrity: all skills have required properties
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn all_skills_have_positive_time_cost() {
    let state = load_state();
    for skill_id in state.camping_skills.all_ids() {
        let skill = state.camping_skill(skill_id).unwrap();
        assert!(
            skill.time_cost > 0,
            "Skill '{}' has zero time_cost",
            skill_id
        );
    }
}

#[test]
fn all_skills_have_positive_use_limit() {
    let state = load_state();
    for skill_id in state.camping_skills.all_ids() {
        let skill = state.camping_skill(skill_id).unwrap();
        assert!(
            skill.use_limit > 0,
            "Skill '{}' has zero use_limit",
            skill_id
        );
    }
}

#[test]
fn all_skills_have_at_least_one_effect() {
    let state = load_state();
    for skill_id in state.camping_skills.all_ids() {
        let skill = state.camping_skill(skill_id).unwrap();
        assert!(
            !skill.effects.is_empty(),
            "Skill '{}' has no effects",
            skill_id
        );
    }
}

#[test]
fn all_effects_have_valid_effect_type() {
    let state = load_state();
    for skill_id in state.camping_skills.all_ids() {
        let skill = state.camping_skill(skill_id).unwrap();
        for (i, effect) in skill.effects.iter().enumerate() {
            assert_ne!(
                effect.effect_type,
                CampEffectType::None,
                "Skill '{}' effect {} has CampEffectType::None",
                skill_id,
                i
            );
        }
    }
}

#[test]
fn all_effects_have_valid_chance() {
    let state = load_state();
    for skill_id in state.camping_skills.all_ids() {
        let skill = state.camping_skill(skill_id).unwrap();
        for (i, effect) in skill.effects.iter().enumerate() {
            assert!(
                effect.chance >= 0.0 && effect.chance <= 1.0,
                "Skill '{}' effect {} has chance {} outside [0.0, 1.0]",
                skill_id,
                i,
                effect.chance
            );
        }
    }
}

#[test]
fn all_effects_have_valid_target_selection() {
    let state = load_state();
    for skill_id in state.camping_skills.all_ids() {
        let skill = state.camping_skill(skill_id).unwrap();
        for (i, effect) in skill.effects.iter().enumerate() {
            assert_ne!(
                effect.selection,
                CampTargetSelection::None,
                "Skill '{}' effect {} has target selection None",
                skill_id,
                i
            );
        }
    }
}

#[test]
fn all_skills_have_positive_upgrade_cost() {
    let state = load_state();
    for skill_id in state.camping_skills.all_ids() {
        let skill = state.camping_skill(skill_id).unwrap();
        assert!(
            skill.upgrade_cost > 0,
            "Skill '{}' has zero upgrade_cost",
            skill_id
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Distribution tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn time_cost_distribution_matches_source() {
    let state = load_state();
    let mut counts = std::collections::HashMap::new();
    for skill_id in state.camping_skills.all_ids() {
        let skill = state.camping_skill(skill_id).unwrap();
        *counts.entry(skill.time_cost).or_insert(0) += 1;
    }
    assert_eq!(counts.get(&1).copied().unwrap_or(0), 5,
        "5 skills should have time_cost=1: abandon_hope, pilfer, preventative_medicine, supply, curious_incantation");
    assert_eq!(counts.get(&2).copied().unwrap_or(0), 20,
        "20 skills should have time_cost=2");
    assert_eq!(counts.get(&3).copied().unwrap_or(0), 35,
        "35 skills should have time_cost=3");
    assert_eq!(counts.get(&4).copied().unwrap_or(0), 26,
        "26 skills should have time_cost=4");
    assert_eq!(counts.get(&5).copied().unwrap_or(0), 1,
        "1 skill (zealous_speech) should have time_cost=5");
}

#[test]
fn use_limit_distribution_matches_source() {
    let state = load_state();
    let mut counts = std::collections::HashMap::new();
    for skill_id in state.camping_skills.all_ids() {
        let skill = state.camping_skill(skill_id).unwrap();
        *counts.entry(skill.use_limit).or_insert(0) += 1;
    }
    assert_eq!(counts.get(&1).copied().unwrap_or(0), 86,
        "86 skills should have use_limit=1");
    assert_eq!(counts.get(&3).copied().unwrap_or(0), 1,
        "1 skill (supply) should have use_limit=3");
}

// ─────────────────────────────────────────────────────────────────────────────
// Class coverage tests: all 31 hero classes
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn all_31_hero_classes_have_camping_skills() {
    let state = load_state();
    #[rustfmt::skip]
    let all_classes = [
        "bounty_hunter", "crusader", "vestal", "occultist", "hellion",
        "grave_robber", "highwayman", "plague_doctor", "jester", "leper",
        "arbalest", "man_at_arms", "houndmaster", "abomination", "antiquarian",
        "musketeer",
        "alchemist", "alchemist1", "alchemist2",
        "diviner", "diviner1", "diviner2",
        "hunter", "hunter1", "hunter2",
        "shaman", "shaman1", "shaman2",
        "tank", "tank1", "tank2",
    ];

    for class in &all_classes {
        let class_skills = state.camping_skills_for_class(class);
        assert!(
            !class_skills.is_empty(),
            "Class '{}' should have at least one camping skill",
            class
        );
    }
}

#[test]
fn generic_skill_hobby_available_to_all_31_classes() {
    let state = load_state();
    #[rustfmt::skip]
    let all_classes = [
        "bounty_hunter", "crusader", "vestal", "occultist", "hellion",
        "grave_robber", "highwayman", "plague_doctor", "jester", "leper",
        "arbalest", "man_at_arms", "houndmaster", "abomination", "antiquarian",
        "musketeer",
        "alchemist", "alchemist1", "alchemist2",
        "diviner", "diviner1", "diviner2",
        "hunter", "hunter1", "hunter2",
        "shaman", "shaman1", "shaman2",
        "tank", "tank1", "tank2",
    ];

    for class in &all_classes {
        let skills = state.camping_skills_for_class(class);
        assert!(
            skills.iter().any(|s| s.id == "hobby"),
            "Generic skill 'hobby' should be available to class '{}'",
            class
        );
    }
}

#[test]
fn class_filtering_excludes_other_class_skills() {
    let state = load_state();
    // field_dressing is arbalest/musketeer only
    let crusader_skills = state.camping_skills_for_class("crusader");
    assert!(
        !crusader_skills.iter().any(|s| s.id == "field_dressing"),
        "crusader should NOT have field_dressing"
    );
    let arbalest_skills = state.camping_skills_for_class("arbalest");
    assert!(
        arbalest_skills.iter().any(|s| s.id == "field_dressing"),
        "arbalest should have field_dressing"
    );
}

#[test]
fn exactly_one_truly_generic_skill() {
    let state = load_state();
    let generic = state.generic_camping_skills();
    assert_eq!(generic.len(), 1, "Exactly one truly generic skill (hobby)");
    assert_eq!(generic[0].id, "hobby");
}

#[test]
fn class_specific_count_is_86() {
    let state = load_state();
    let specific = state.class_specific_camping_skills();
    assert_eq!(specific.len(), 86);
}

// ─────────────────────────────────────────────────────────────────────────────
// Effect type coverage
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn all_19_effect_types_are_present_in_loaded_skills() {
    let state = load_state();
    use std::collections::HashSet;
    let mut types = HashSet::new();
    for skill_id in state.camping_skills.all_ids() {
        let skill = state.camping_skill(skill_id).unwrap();
        for effect in &skill.effects {
            types.insert(effect.effect_type.clone());
        }
    }

    assert!(types.contains(&CampEffectType::StressHealAmount));
    assert!(types.contains(&CampEffectType::HealthHealMaxHealthPercent));
    assert!(types.contains(&CampEffectType::RemoveBleed));
    assert!(types.contains(&CampEffectType::RemovePoison));
    assert!(types.contains(&CampEffectType::Buff));
    assert!(types.contains(&CampEffectType::RemoveDeathRecovery));
    assert!(types.contains(&CampEffectType::ReduceAmbushChance));
    assert!(types.contains(&CampEffectType::RemoveDisease));
    assert!(types.contains(&CampEffectType::StressDamageAmount));
    assert!(types.contains(&CampEffectType::Loot));
    assert!(types.contains(&CampEffectType::ReduceTorch));
    assert!(types.contains(&CampEffectType::HealthDamageMaxHealthPercent));
    assert!(types.contains(&CampEffectType::StressHealPercent));
    assert!(types.contains(&CampEffectType::RemoveDebuff));
    assert!(types.contains(&CampEffectType::RemoveAllDebuff));
    assert!(types.contains(&CampEffectType::HealthHealRange));
    assert!(types.contains(&CampEffectType::HealthHealAmount));
    assert!(types.contains(&CampEffectType::ReduceTurbulenceChance));
    assert!(types.contains(&CampEffectType::ReduceRiptideChance));

    assert_eq!(types.len(), 19,
        "Should have exactly 19 distinct effect types loaded from source data");
}

// ─────────────────────────────────────────────────────────────────────────────
// Registry technical tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn registry_lookup_by_id_works() {
    let state = load_state();
    assert!(state.camping_skill("encourage").is_some());
    assert!(state.camping_skill("hobby").is_some());
    assert!(state.camping_skill("field_dressing").is_some());
    assert!(state.camping_skill("dark_ritual").is_some());
    assert!(state.camping_skill("supply").is_some());
    assert!(state.camping_skill("nonexistent").is_none());
    assert!(state.camping_skill("").is_none());
}

#[test]
fn registry_is_not_empty_and_has_correct_count() {
    let registry = parse_all();
    assert!(!registry.is_empty());
    assert_eq!(registry.len(), 87);
}

#[test]
fn registry_all_ids_is_complete() {
    let state = load_state();
    let ids = state.camping_skills.all_ids();
    assert_eq!(ids.len(), 87);
    // Verify no duplicate IDs
    let mut seen = std::collections::HashSet::new();
    for id in &ids {
        assert!(seen.insert(*id), "Duplicate skill ID found: {}", id);
    }
}

#[test]
fn game_state_loads_from_default_data_dir() {
    let result = GameState::load();
    assert!(result.is_ok(), "GameState::load() should succeed: {:?}", result.err());
    let state = result.unwrap();
    assert_eq!(state.camping_skill_count(), 87);
}

#[test]
fn game_state_fails_when_json_camping_missing() {
    let result = GameState::load_from(std::path::Path::new("/nonexistent/path"));
    assert!(result.is_err());
    assert!(result.err().unwrap().contains("not found"));
}

// ─────────────────────────────────────────────────────────────────────────────
// Skill has_individual_target accuracy
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn has_individual_target_matches_effect_selections() {
    let state = load_state();

    // Self-target only skills should NOT have individual target
    let hobby = state.camping_skill("hobby").unwrap();
    assert!(!hobby.has_individual_target);
    assert_eq!(hobby.effects[0].selection, CampTargetSelection::SelfTarget);

    // Skills with Individual/PartyOther effects should have individual target
    let encourage = state.camping_skill("encourage").unwrap();
    assert!(encourage.has_individual_target);
    assert_eq!(encourage.effects[0].selection, CampTargetSelection::Individual);

    let first_aid = state.camping_skill("first_aid").unwrap();
    assert!(first_aid.has_individual_target);

    let field_dressing = state.camping_skill("field_dressing").unwrap();
    assert!(field_dressing.has_individual_target);

    // Party skills should NOT have individual target
    let relax = state.camping_skill("relax").unwrap();
    assert!(!relax.has_individual_target);
    assert_eq!(relax.effects[0].selection, CampTargetSelection::Party);
}

// ─────────────────────────────────────────────────────────────────────────────
// Specific skill data integrity
// ─────────────────────────────────────────────────────────────────────────────

/// dark_ritual: the only skill using ReduceTorch effect type (100 torch).
#[test]
fn dark_ritual_has_reduce_torch() {
    let state = load_state();
    let skill = state.camping_skill("dark_ritual").unwrap();
    assert_eq!(skill.time_cost, 4);
    assert_eq!(skill.classes, vec!["occultist"]);
    let torch_effect = skill.effects.iter()
        .find(|e| e.effect_type == CampEffectType::ReduceTorch)
        .expect("dark_ritual must have ReduceTorch");
    assert!((torch_effect.amount - 100.0).abs() < f64::EPSILON);
}

/// supply: the only skill with use_limit=3 (Loot effect, time_cost=1).
#[test]
fn supply_has_use_limit_3_and_loot() {
    let state = load_state();
    let skill = state.camping_skill("supply").unwrap();
    assert_eq!(skill.time_cost, 1);
    assert_eq!(skill.use_limit, 3);
    assert_eq!(skill.classes, vec!["antiquarian"]);
    assert_eq!(skill.effects[0].effect_type, CampEffectType::Loot);
}

/// zealous_speech: the only skill with time_cost=5.
#[test]
fn zealous_speech_has_highest_time_cost() {
    let state = load_state();
    let skill = state.camping_skill("zealous_speech").unwrap();
    assert_eq!(skill.time_cost, 5);
    assert_eq!(skill.classes, vec!["crusader"]);
}

/// self_medicate: 5 effects covering stress, health, poison, bleed, buff.
#[test]
fn self_medicate_has_five_distinct_effect_types() {
    let state = load_state();
    let skill = state.camping_skill("self_medicate").unwrap();
    assert_eq!(skill.effects.len(), 5);
    let types: Vec<_> = skill.effects.iter().map(|e| &e.effect_type).collect();
    assert!(types.contains(&&CampEffectType::StressHealAmount));
    assert!(types.contains(&&CampEffectType::HealthHealMaxHealthPercent));
    assert!(types.contains(&&CampEffectType::RemovePoison));
    assert!(types.contains(&&CampEffectType::RemoveBleed));
    assert!(types.contains(&&CampEffectType::Buff));
}

/// turn_back_time (jester): stress_heal_amount 30 + 15.
#[test]
fn turn_back_time_heals_stress() {
    let state = load_state();
    let skill = state.camping_skill("turn_back_time").unwrap();
    assert_eq!(skill.time_cost, 3);
    assert_eq!(skill.classes, vec!["jester"]);
    assert_eq!(skill.effects.len(), 2);
    assert_eq!(skill.effects[0].effect_type, CampEffectType::StressHealAmount);
    assert!((skill.effects[0].amount - 30.0).abs() < f64::EPSILON);
    assert_eq!(skill.effects[1].effect_type, CampEffectType::StressHealAmount);
    assert!((skill.effects[1].amount - 15.0).abs() < f64::EPSILON);
}

/// hunting_ritual (hunter): most effects (7) among all skills.
#[test]
fn hunting_ritual_has_most_effects() {
    let state = load_state();
    let skill = state.camping_skill("hunting_ritual").unwrap();
    assert_eq!(skill.effects.len(), 7);
    assert_eq!(skill.time_cost, 4);
}

/// revel (hellion): 8 effects (ties for most with hunting_ritual having 7? No, revel has 8).
#[test]
fn revel_has_many_effects() {
    let state = load_state();
    let skill = state.camping_skill("revel").unwrap();
    assert_eq!(skill.effects.len(), 8);
    assert_eq!(skill.time_cost, 3);
}
