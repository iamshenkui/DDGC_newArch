//! DDGC game state — loaded content datasets for runtime use.
//!
//! This module holds in-memory game state loaded from DDGC data files.
//! It provides a bridge between the contracts-layer parsing and the
//! run-layer execution, exposing canonical content datasets as typed state.
//!
//! # Architecture
//!
//! - **Contracts** (`crate::contracts`): data model and JSON parsing.
//! - **State** (`crate::state`): loaded content datasets (this module).
//! - **Planner** (`crate::planner`): selection/recommendation logic.
//! - **Run** (`crate::run`): runtime phase resolution and effect application.

use std::path::{Path, PathBuf};

use crate::contracts::{
    CampingSkillRegistry,
    parse::parse_camping_json,
};

/// Full game state loaded from DDGC data files.
///
/// Holds all parsed content datasets needed at runtime.
/// Construct via [`GameState::load`] or [`GameState::load_from`].
#[derive(Debug, Clone)]
pub struct GameState {
    /// All 87 camping skill definitions from JsonCamping.json.
    pub camping_skills: CampingSkillRegistry,
    /// Path to the data directory used for loading.
    pub data_dir: PathBuf,
}

impl GameState {
    /// Load game state from the default data directory.
    ///
    /// The default data directory is `<project_root>/data/`.
    /// Project root is determined by the `CARGO_MANIFEST_DIR` environment
    /// variable at compile time, or the current working directory at runtime
    /// as a fallback.
    pub fn load() -> Result<Self, String> {
        let data_dir = Self::default_data_dir()?;
        Self::load_from(&data_dir)
    }

    /// Load game state from a specific data directory.
    pub fn load_from(data_dir: &Path) -> Result<Self, String> {
        let camping_path = data_dir.join("JsonCamping.json");
        if !camping_path.exists() {
            return Err(format!(
                "JsonCamping.json not found at {}",
                camping_path.display()
            ));
        }

        let camping_skills = parse_camping_json(&camping_path)?;

        Ok(GameState {
            camping_skills,
            data_dir: data_dir.to_path_buf(),
        })
    }

    /// Determine the default data directory.
    ///
    /// Uses `CARGO_MANIFEST_DIR` (set by cargo during build) to locate the
    /// project root. Falls back to the current working directory if the env
    /// var is not set.
    fn default_data_dir() -> Result<PathBuf, String> {
        if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
            Ok(PathBuf::from(manifest_dir).join("data"))
        } else {
            // Fallback: try current working directory
            let cwd = std::env::current_dir()
                .map_err(|e| format!("cannot determine current directory: {}", e))?;
            let data_dir = cwd.join("data");
            if data_dir.exists() {
                Ok(data_dir)
            } else {
                Err("CARGO_MANIFEST_DIR not set and data/ not found in CWD".to_string())
            }
        }
    }

    /// Get the total number of camping skills loaded.
    pub fn camping_skill_count(&self) -> usize {
        self.camping_skills.len()
    }

    /// Get a camping skill by ID.
    pub fn camping_skill(&self, id: &str) -> Option<&crate::contracts::CampingSkill> {
        self.camping_skills.get(id)
    }

    /// Validate all camping skills in the loaded state.
    ///
    /// Returns Ok if all 87 skills pass the runtime schema validation.
    pub fn validate_camping_skills(&self) -> Result<(), Vec<String>> {
        self.camping_skills.validate()
    }

    /// Get all camping skills available to a specific hero class.
    pub fn camping_skills_for_class(&self, class_id: &str) -> Vec<&crate::contracts::CampingSkill> {
        self.camping_skills.for_class(class_id)
    }

    /// Get all generic camping skills (available to all classes).
    pub fn generic_camping_skills(&self) -> Vec<&crate::contracts::CampingSkill> {
        self.camping_skills.generic_skills()
    }

    /// Get all class-specific camping skills.
    pub fn class_specific_camping_skills(&self) -> Vec<&crate::contracts::CampingSkill> {
        self.camping_skills.class_specific_skills()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::CampEffectType;

    /// Helper: resolve the data directory for tests.
    ///
    /// Uses `CARGO_MANIFEST_DIR` at test time (set by cargo test).
    fn test_data_dir() -> PathBuf {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR must be set during cargo test");
        PathBuf::from(manifest_dir).join("data")
    }

    /// Helper: load the real camping skill registry for tests.
    fn load_real_state() -> GameState {
        let data_dir = test_data_dir();
        GameState::load_from(&data_dir).expect("failed to load game state from data dir")
    }

    // ───────────────────────────────────────────────────────────────
    // State loading tests
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn state_loads_all_87_camping_skills() {
        let state = load_real_state();
        assert_eq!(state.camping_skill_count(), 87);
    }

    #[test]
    fn state_loads_from_default_data_dir() {
        let state = GameState::load();
        assert!(
            state.is_ok(),
            "GameState::load() should succeed from default data dir"
        );
        let state = state.unwrap();
        assert_eq!(state.camping_skill_count(), 87);
    }

    #[test]
    fn state_fails_when_json_camping_missing() {
        let result = GameState::load_from(Path::new("/nonexistent/path"));
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("not found"));
    }

    #[test]
    fn state_preserves_data_dir() {
        let state = load_real_state();
        assert!(state.data_dir.exists());
        assert!(state.data_dir.join("JsonCamping.json").exists());
    }

    // ───────────────────────────────────────────────────────────────
    // Full registry validation tests
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn full_camping_registry_validates_against_runtime_schema() {
        let state = load_real_state();
        let result = state.validate_camping_skills();
        assert!(
            result.is_ok(),
            "camping skill validation failed: {:?}",
            result.err()
        );
    }

    #[test]
    fn every_individual_camping_skill_passes_validation() {
        let state = load_real_state();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).expect("skill should exist");
            let errors = skill.validate();
            assert!(
                errors.is_empty(),
                "skill '{}' failed validation: {:?}",
                skill_id,
                errors
            );
        }
    }

    // ───────────────────────────────────────────────────────────────
    // Content integrity tests
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn all_skills_have_positive_time_cost() {
        let state = load_real_state();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).unwrap();
            assert!(
                skill.time_cost > 0,
                "skill '{}' has zero time_cost",
                skill_id
            );
        }
    }

    #[test]
    fn all_skills_have_positive_use_limit() {
        let state = load_real_state();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).unwrap();
            assert!(
                skill.use_limit > 0,
                "skill '{}' has zero use_limit",
                skill_id
            );
        }
    }

    #[test]
    fn all_skills_have_at_least_one_effect() {
        let state = load_real_state();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).unwrap();
            assert!(
                !skill.effects.is_empty(),
                "skill '{}' has no effects",
                skill_id
            );
        }
    }

    #[test]
    fn all_effects_have_valid_type() {
        let state = load_real_state();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).unwrap();
            for (i, effect) in skill.effects.iter().enumerate() {
                assert_ne!(
                    effect.effect_type,
                    CampEffectType::None,
                    "skill '{}' effect {} has None type",
                    skill_id,
                    i
                );
            }
        }
    }

    #[test]
    fn all_effects_have_valid_chance() {
        let state = load_real_state();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).unwrap();
            for (i, effect) in skill.effects.iter().enumerate() {
                assert!(
                    effect.chance >= 0.0 && effect.chance <= 1.0,
                    "skill '{}' effect {} has invalid chance {}",
                    skill_id,
                    i,
                    effect.chance
                );
            }
        }
    }

    // ───────────────────────────────────────────────────────────────
    // Class filtering tests
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn exactly_one_generic_skill() {
        let state = load_real_state();
        let generic = state.generic_camping_skills();
        assert_eq!(generic.len(), 1);
        assert_eq!(generic[0].id, "hobby");
    }

    #[test]
    fn generic_skill_hobby_preserves_source_data() {
        let state = load_real_state();
        let skill = state.camping_skill("hobby").unwrap();
        assert_eq!(skill.time_cost, 2);
        assert_eq!(skill.use_limit, 1);
        assert!(skill.classes.is_empty());
        assert!(skill.is_generic());
        assert!(!skill.has_individual_target);
        assert_eq!(skill.effects.len(), 1);
        assert_eq!(skill.effects[0].effect_type, CampEffectType::StressHealAmount);
        assert!((skill.effects[0].amount - 12.0).abs() < f64::EPSILON);
        assert!((skill.effects[0].chance - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn class_specific_count_is_86() {
        let state = load_real_state();
        let specific = state.class_specific_camping_skills();
        assert_eq!(specific.len(), 86);
    }

    #[test]
    fn encourage_preserves_source_data() {
        let state = load_real_state();
        let skill = state.camping_skill("encourage").unwrap();
        assert_eq!(skill.time_cost, 2);
        assert_eq!(skill.use_limit, 1);
        assert!(skill.has_individual_target);
        assert_eq!(skill.effects.len(), 1);
        assert_eq!(skill.effects[0].effect_type, CampEffectType::StressHealAmount);
        assert!((skill.effects[0].amount - 15.0).abs() < f64::EPSILON);
        assert_eq!(skill.classes.len(), 16);
    }

    #[test]
    fn field_dressing_preserves_source_data() {
        let state = load_real_state();
        let skill = state.camping_skill("field_dressing").unwrap();
        assert_eq!(skill.time_cost, 3);
        assert_eq!(skill.use_limit, 1);
        assert!(skill.has_individual_target);
        assert_eq!(skill.classes, vec!["arbalest", "musketeer"]);
        assert_eq!(skill.effects.len(), 3);

        // Effect 0: 35% heal, 75% chance
        assert_eq!(skill.effects[0].effect_type, CampEffectType::HealthHealMaxHealthPercent);
        assert!((skill.effects[0].amount - 0.35).abs() < f64::EPSILON);
        assert!((skill.effects[0].chance - 0.75).abs() < f64::EPSILON);

        // Effect 1: 50% heal, 25% chance
        assert_eq!(skill.effects[1].effect_type, CampEffectType::HealthHealMaxHealthPercent);
        assert!((skill.effects[1].amount - 0.50).abs() < f64::EPSILON);
        assert!((skill.effects[1].chance - 0.25).abs() < f64::EPSILON);

        // Effect 2: remove bleed, 100% chance
        assert_eq!(skill.effects[2].effect_type, CampEffectType::RemoveBleed);
        assert!((skill.effects[2].chance - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn supply_has_use_limit_3() {
        let state = load_real_state();
        let skill = state.camping_skill("supply").unwrap();
        assert_eq!(skill.use_limit, 3);
        assert_eq!(skill.time_cost, 1);
        assert_eq!(skill.classes, vec!["antiquarian"]);
        assert_eq!(skill.effects.len(), 1);
        assert_eq!(skill.effects[0].effect_type, CampEffectType::Loot);
    }

    #[test]
    fn dark_ritual_preserves_reduce_torch_effect() {
        let state = load_real_state();
        let skill = state.camping_skill("dark_ritual").unwrap();
        assert_eq!(skill.time_cost, 4);
        assert_eq!(skill.use_limit, 1);
        assert_eq!(skill.classes, vec!["occultist"]);
        assert_eq!(skill.effects.len(), 4);
        let torch_effect = skill
            .effects
            .iter()
            .find(|e| e.effect_type == CampEffectType::ReduceTorch)
            .expect("dark_ritual should have a reduce_torch effect");
        assert!((torch_effect.amount - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn zealous_speech_has_highest_time_cost_5() {
        let state = load_real_state();
        let skill = state.camping_skill("zealous_speech").unwrap();
        assert_eq!(skill.time_cost, 5);
        assert_eq!(skill.use_limit, 1);
        assert_eq!(skill.classes, vec!["crusader"]);
    }

    #[test]
    fn self_medicate_has_five_effects() {
        let state = load_real_state();
        let skill = state.camping_skill("self_medicate").unwrap();
        assert_eq!(skill.time_cost, 3);
        assert_eq!(skill.classes, vec!["plague_doctor"]);
        assert_eq!(skill.effects.len(), 5);
        let types: Vec<_> = skill.effects.iter().map(|e| &e.effect_type).collect();
        assert!(types.contains(&&CampEffectType::StressHealAmount));
        assert!(types.contains(&&CampEffectType::HealthHealMaxHealthPercent));
        assert!(types.contains(&&CampEffectType::RemovePoison));
        assert!(types.contains(&&CampEffectType::RemoveBleed));
        assert!(types.contains(&&CampEffectType::Buff));
    }

    #[test]
    fn first_aid_heals_and_cleanses() {
        let state = load_real_state();
        let skill = state.camping_skill("first_aid").unwrap();
        assert_eq!(skill.time_cost, 2);
        assert_eq!(skill.effects.len(), 3);
        let types: Vec<_> = skill.effects.iter().map(|e| &e.effect_type).collect();
        assert!(types.contains(&&CampEffectType::HealthHealMaxHealthPercent));
        assert!(types.contains(&&CampEffectType::RemoveBleed));
        assert!(types.contains(&&CampEffectType::RemovePoison));
    }

    // ───────────────────────────────────────────────────────────────
    // Effect type coverage test
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn effect_type_coverage_matches_source() {
        let state = load_real_state();
        use std::collections::HashSet;
        let mut types = HashSet::new();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).unwrap();
            for effect in &skill.effects {
                types.insert(effect.effect_type.clone());
            }
        }
        // All 19 effect types used in JsonCamping.json should be present
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
        assert_eq!(types.len(), 19);
    }

    // ───────────────────────────────────────────────────────────────
    // Distribution tests
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn time_cost_distribution_matches_source() {
        let state = load_real_state();
        let mut counts = std::collections::HashMap::new();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).unwrap();
            *counts.entry(skill.time_cost).or_insert(0) += 1;
        }
        assert_eq!(counts.get(&1).copied().unwrap_or(0), 5);
        assert_eq!(counts.get(&2).copied().unwrap_or(0), 20);
        assert_eq!(counts.get(&3).copied().unwrap_or(0), 35);
        assert_eq!(counts.get(&4).copied().unwrap_or(0), 26);
        assert_eq!(counts.get(&5).copied().unwrap_or(0), 1);
    }

    #[test]
    fn use_limit_distribution_matches_source() {
        let state = load_real_state();
        let mut counts = std::collections::HashMap::new();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).unwrap();
            *counts.entry(skill.use_limit).or_insert(0) += 1;
        }
        assert_eq!(counts.get(&1).copied().unwrap_or(0), 86);
        assert_eq!(counts.get(&3).copied().unwrap_or(0), 1);
    }

    // ───────────────────────────────────────────────────────────────
    // Class coverage tests
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn all_31_hero_classes_have_skills() {
        let state = load_real_state();
        let all_classes = [
            "bounty_hunter", "crusader", "vestal", "occultist", "hellion",
            "grave_robber", "highwayman", "plague_doctor", "jester", "leper",
            "arbalest", "man_at_arms", "houndmaster", "abomination", "antiquarian",
            "musketeer", "alchemist", "alchemist1", "alchemist2",
            "diviner", "diviner1", "diviner2",
            "hunter", "hunter1", "hunter2",
            "shaman", "shaman1", "shaman2",
            "tank", "tank1", "tank2",
        ];
        for class in &all_classes {
            let class_skills = state.camping_skills_for_class(class);
            assert!(
                !class_skills.is_empty(),
                "class '{}' should have at least one skill",
                class
            );
        }
    }

    #[test]
    fn class_filtering_includes_generic_skills() {
        let state = load_real_state();
        for class in &["alchemist", "crusader", "arbalest"] {
            let skills = state.camping_skills_for_class(class);
            assert!(
                skills.iter().any(|s| s.id == "hobby"),
                "class '{}' should have generic skill 'hobby'",
                class
            );
        }
    }

    #[test]
    fn class_filtering_excludes_other_class_specific_skills() {
        let state = load_real_state();
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

    // ───────────────────────────────────────────────────────────────
    // Registry technical tests
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn camping_skill_lookup_by_id() {
        let state = load_real_state();
        assert!(state.camping_skill("encourage").is_some());
        assert!(state.camping_skill("hobby").is_some());
        assert!(state.camping_skill("field_dressing").is_some());
        assert!(state.camping_skill("dark_ritual").is_some());
        assert!(state.camping_skill("supply").is_some());
        assert!(state.camping_skill("nonexistent_skill").is_none());
    }

    #[test]
    fn all_skills_have_upgrade_cost() {
        let state = load_real_state();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).unwrap();
            assert!(
                skill.upgrade_cost > 0,
                "skill '{}' has zero upgrade_cost",
                skill_id
            );
        }
    }

    #[test]
    fn skill_with_individual_target_is_flagged() {
        let state = load_real_state();
        let field_dressing = state.camping_skill("field_dressing").unwrap();
        assert!(field_dressing.has_individual_target);
        let encourage = state.camping_skill("encourage").unwrap();
        assert!(encourage.has_individual_target);
        let hobby = state.camping_skill("hobby").unwrap();
        assert!(!hobby.has_individual_target);
    }

    #[test]
    fn registry_is_not_empty() {
        let state = load_real_state();
        assert!(!state.camping_skills.is_empty());
        assert_eq!(state.camping_skill_count(), 87);
    }
}
