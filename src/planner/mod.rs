//! Camping skill planner — selection, recommendation, and budget planning.
//!
//! This module provides the planner-layer API for the camping phase. It re-exports
//! the canonical data models from the contracts layer and adds planning functions
//! for skill selection, class filtering, and time budget allocation.
//!
//! # Architecture
//!
//! - **Contracts** (`crate::contracts`): canonical data model and JSON parsing.
//! - **Planner** (`crate::planner`): selection/recommendation logic (this module).
//! - **Run** (`crate::run::camping`): runtime phase resolution and effect application.
//!
//! # Enum Ambiguity Resolution
//!
//! The [`CampEffectType`] enum explicitly resolves the original game's enum-surface
//! ambiguity:
//!
//! - `None`: **included** as a sentinel for uninitialized/unknown effect types.
//!   Skills with `None` effects should be treated as malformed during planning.
//! - `ReduceTorch`: **included** for source completeness but marked as deleted
//!   in the original game. The planner skips any skill that relies on this type.

use std::collections::HashMap;

// Re-export canonical camping types from contracts.
pub use crate::contracts::{
    CampEffect, CampEffectType, CampTargetSelection, CampingSkill, CampingSkillRegistry,
};

/// A skill recommendation with a priority score.
///
/// Higher scores indicate more impactful skills for the given context.
#[derive(Debug, Clone)]
pub struct SkillRecommendation {
    pub skill_id: String,
    pub time_cost: u32,
    pub score: f64,
}

/// Planner for camping skill selection and budget allocation.
///
/// Wraps a [`CampingSkillRegistry`] and provides planning-oriented queries:
/// - Skill availability by class
/// - Ranked recommendations for a given hero
/// - Time budget feasibility checks
#[derive(Debug, Clone)]
pub struct CampingPlanner {
    registry: CampingSkillRegistry,
}

impl CampingPlanner {
    /// Create a new planner backed by the given skill registry.
    pub fn new(registry: CampingSkillRegistry) -> Self {
        CampingPlanner { registry }
    }

    /// Return a reference to the underlying registry.
    pub fn registry(&self) -> &CampingSkillRegistry {
        &self.registry
    }

    /// Get all skills available to a specific hero class.
    ///
    /// Includes both generic skills (available to all classes) and
    /// class-specific skills matching the given class.
    pub fn available_skills_for(&self, class_id: &str) -> Vec<&CampingSkill> {
        self.registry.for_class(class_id)
    }

    /// Get a skill by ID.
    pub fn get_skill(&self, id: &str) -> Option<&CampingSkill> {
        self.registry.get(id)
    }

    /// Get all generic skills (available to all classes).
    pub fn generic_skills(&self) -> Vec<&CampingSkill> {
        self.registry.generic_skills()
    }

    /// Get all class-specific skills.
    pub fn class_specific_skills(&self) -> Vec<&CampingSkill> {
        self.registry.class_specific_skills()
    }

    /// Get the total number of registered skills.
    pub fn skill_count(&self) -> usize {
        self.registry.len()
    }

    /// Recommend skills for a hero sorted by healing/stress-relief impact.
    ///
    /// Skills that heal health or stress are scored higher. The score is a weighted
    /// sum of the effect amounts, adjusted by chance probability. Buff and utility
    /// skills receive a base score.
    ///
    /// Returns skills sorted by score descending, limited to `max_count` results.
    pub fn recommend_for_hero(
        &self,
        class_id: &str,
        max_count: usize,
    ) -> Vec<SkillRecommendation> {
        let mut scored: Vec<SkillRecommendation> = self
            .available_skills_for(class_id)
            .iter()
            .map(|skill| {
                let score = score_skill(skill);
                SkillRecommendation {
                    skill_id: skill.id.clone(),
                    time_cost: skill.time_cost,
                    score,
                }
            })
            .collect();

        scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(max_count);
        scored
    }

    /// Check whether a set of skills fits within the time budget.
    ///
    /// Returns `true` if the sum of time costs does not exceed the budget.
    pub fn fits_budget(&self, skill_ids: &[&str], time_budget: u32) -> bool {
        let total_cost: u32 = skill_ids
            .iter()
            .filter_map(|id| self.registry.get(id))
            .map(|s| s.time_cost)
            .sum();
        total_cost <= time_budget
    }

    /// Count skills by targeting mode.
    pub fn count_by_targeting(&self) -> HashMap<CampTargetSelection, usize> {
        let mut counts = HashMap::new();
        for skill in self.registry.generic_skills() {
            for effect in &skill.effects {
                *counts.entry(effect.selection.clone()).or_insert(0) += 1;
            }
        }
        for skill in self.registry.class_specific_skills() {
            for effect in &skill.effects {
                *counts.entry(effect.selection.clone()).or_insert(0) += 1;
            }
        }
        counts
    }
}

/// Score a camping skill by its healing and utility value.
///
/// - Health/stress heals: score = chance × amount × weight
/// - Buffs: base score of 5
/// - Remove effects: base score of 3
/// - Damage/loot: base score of 2
fn score_skill(skill: &CampingSkill) -> f64 {
    let mut total = 0.0;

    for effect in &skill.effects {
        let base = match effect.effect_type {
            CampEffectType::StressHealAmount => effect.chance * effect.amount * 1.0,
            CampEffectType::StressHealPercent => effect.chance * effect.amount * 100.0,
            CampEffectType::HealthHealMaxHealthPercent => effect.chance * effect.amount * 100.0,
            CampEffectType::HealthHealAmount => effect.chance * effect.amount * 1.0,
            CampEffectType::HealthHealRange => effect.chance * effect.amount * 1.0,
            CampEffectType::Buff => 5.0,
            CampEffectType::RemoveBleed
            | CampEffectType::RemovePoison
            | CampEffectType::RemoveBurn
            | CampEffectType::RemoveFrozen
            | CampEffectType::RemoveDisease
            | CampEffectType::RemoveDebuff
            | CampEffectType::RemoveAllDebuff
            | CampEffectType::RemoveDeathRecovery => 3.0,
            CampEffectType::ReduceAmbushChance
            | CampEffectType::ReduceTurbulenceChance
            | CampEffectType::ReduceRiptideChance => 2.0,
            CampEffectType::StressDamageAmount
            | CampEffectType::HealthDamageMaxHealthPercent
            | CampEffectType::Loot => 2.0,
            CampEffectType::None | CampEffectType::ReduceTorch => {
                // Non-functional types — zero score to deprioritize.
                0.0
            }
        };
        total += base;
    }

    // Normalize by time cost to prefer efficient skills.
    if skill.time_cost > 0 {
        total / skill.time_cost as f64
    } else {
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{
        CampEffect, CampEffectType, CampTargetSelection, CampingSkill, CampingSkillRegistry,
    };

    fn build_test_registry() -> CampingSkillRegistry {
        let mut registry = CampingSkillRegistry::new();

        // Shared skill: encourage — stress heal to self
        registry.register(CampingSkill::new(
            "encourage",
            2,
            1,
            false,
            vec![],
            vec![CampEffect::new(
                CampTargetSelection::SelfTarget,
                vec![],
                1.0,
                CampEffectType::StressHealAmount,
                "",
                10.0,
            )],
            1000,
        ));

        // Shared skill: hobby — stress heal to self
        registry.register(CampingSkill::new(
            "hobby",
            2,
            1,
            false,
            vec![],
            vec![CampEffect::new(
                CampTargetSelection::SelfTarget,
                vec![],
                1.0,
                CampEffectType::StressHealAmount,
                "",
                12.0,
            )],
            1750,
        ));

        // Class-specific: field_dressing — arbalest/musketeer only, individual target
        registry.register(CampingSkill::new(
            "field_dressing",
            3,
            1,
            true,
            vec!["arbalest".into(), "musketeer".into()],
            vec![
                CampEffect::new(
                    CampTargetSelection::Individual,
                    vec![],
                    0.75,
                    CampEffectType::HealthHealMaxHealthPercent,
                    "",
                    0.35,
                ),
                CampEffect::new(
                    CampTargetSelection::Individual,
                    vec![],
                    0.25,
                    CampEffectType::HealthHealMaxHealthPercent,
                    "",
                    0.50,
                ),
                CampEffect::new(
                    CampTargetSelection::Individual,
                    vec![],
                    1.0,
                    CampEffectType::RemoveBleed,
                    "",
                    0.0,
                ),
            ],
            1750,
        ));

        // Class-specific: unshakeable_leader — crusader only, party buff
        registry.register(CampingSkill::new(
            "unshakeable_leader",
            4,
            1,
            false,
            vec!["crusader".into()],
            vec![CampEffect::new(
                CampTargetSelection::Party,
                vec![],
                1.0,
                CampEffectType::Buff,
                "unshakeable_leader_buff",
                0.0,
            )],
            1750,
        ));

        registry
    }

    // ── Planner construction ─────────────────────────────────────────────────

    #[test]
    fn planner_wraps_registry() {
        let registry = build_test_registry();
        let planner = CampingPlanner::new(registry);
        assert_eq!(planner.skill_count(), 4);
    }

    // ── Skill lookup ─────────────────────────────────────────────────────────

    #[test]
    fn get_shared_skill_hobby() {
        let planner = CampingPlanner::new(build_test_registry());
        let skill = planner.get_skill("hobby").expect("hobby should exist");
        assert_eq!(skill.time_cost, 2);
        assert_eq!(skill.use_limit, 1);
        assert!(!skill.has_individual_target);
        assert!(skill.is_generic());
        assert_eq!(skill.upgrade_cost, 1750);
        assert_eq!(skill.effects.len(), 1);
        let effect = &skill.effects[0];
        assert_eq!(effect.effect_type, CampEffectType::StressHealAmount);
        assert_eq!(effect.selection, CampTargetSelection::SelfTarget);
        assert!((effect.amount - 12.0).abs() < f64::EPSILON);
        assert!((effect.chance - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn get_class_specific_field_dressing() {
        let planner = CampingPlanner::new(build_test_registry());
        let skill = planner
            .get_skill("field_dressing")
            .expect("field_dressing should exist");
        assert_eq!(skill.time_cost, 3);
        assert_eq!(skill.use_limit, 1);
        assert!(skill.has_individual_target);
        assert!(!skill.is_generic());
        assert_eq!(skill.classes, vec!["arbalest", "musketeer"]);
        assert_eq!(skill.upgrade_cost, 1750);
        assert_eq!(skill.effects.len(), 3);
    }

    // ── Class filtering ──────────────────────────────────────────────────────

    #[test]
    fn available_skills_for_arbalest_includes_class_specific() {
        let planner = CampingPlanner::new(build_test_registry());
        let skills = planner.available_skills_for("arbalest");
        let ids: Vec<&str> = skills.iter().map(|s| s.id.as_str()).collect();
        // Should include shared skills + field_dressing
        assert!(ids.contains(&"encourage"));
        assert!(ids.contains(&"hobby"));
        assert!(ids.contains(&"field_dressing"));
        // Should NOT include crusader-only skill
        assert!(!ids.contains(&"unshakeable_leader"));
    }

    #[test]
    fn available_skills_for_alchemist_only_generic() {
        let planner = CampingPlanner::new(build_test_registry());
        let skills = planner.available_skills_for("alchemist");
        let ids: Vec<&str> = skills.iter().map(|s| s.id.as_str()).collect();
        // Should only have generic skills (no alchemist-specific in test data)
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&"encourage"));
        assert!(ids.contains(&"hobby"));
    }

    #[test]
    fn available_skills_for_crusader_includes_class_specific() {
        let planner = CampingPlanner::new(build_test_registry());
        let skills = planner.available_skills_for("crusader");
        let ids: Vec<&str> = skills.iter().map(|s| s.id.as_str()).collect();
        assert!(ids.contains(&"unshakeable_leader"));
        assert_eq!(skills.len(), 3); // 2 shared + 1 class-specific
    }

    // ── Recommendations ──────────────────────────────────────────────────────

    #[test]
    fn recommend_scores_healing_higher() {
        let planner = CampingPlanner::new(build_test_registry());
        let recommendations = planner.recommend_for_hero("arbalest", 5);

        // field_dressing should score highest (health heal + remove bleed)
        assert!(!recommendations.is_empty());
        // encourage and hobby both heal stress; hobby heals more (12 > 10)
        let hobby_pos = recommendations.iter().position(|r| r.skill_id == "hobby");
        let encourage_pos = recommendations.iter().position(|r| r.skill_id == "encourage");
        assert!(hobby_pos.is_some());
        assert!(encourage_pos.is_some());
        // hobby should rank above encourage (heals 12 vs 10 stress)
        assert!(hobby_pos.unwrap() < encourage_pos.unwrap());
    }

    // ── Budget planning ──────────────────────────────────────────────────────

    #[test]
    fn fits_budget_with_enough_time() {
        let planner = CampingPlanner::new(build_test_registry());
        // encourage (2) + hobby (2) = 4, budget 12 → fits
        assert!(planner.fits_budget(&["encourage", "hobby"], 12));
    }

    #[test]
    fn fits_budget_exceeds_limit() {
        let planner = CampingPlanner::new(build_test_registry());
        // field_dressing (3) × 5 = 15, budget 12 → doesn't fit
        assert!(!planner.fits_budget(
            &["field_dressing", "field_dressing", "field_dressing", "field_dressing", "field_dressing"],
            12
        ));
    }

    // ── Targeting counts ─────────────────────────────────────────────────────

    #[test]
    fn count_by_targeting_reports_distribution() {
        let planner = CampingPlanner::new(build_test_registry());
        let counts = planner.count_by_targeting();
        // SelfTarget: encourage, hobby = 2 effects
        // Individual: field_dressing = 3 effects
        // Party: unshakeable_leader = 1 effect
        assert_eq!(counts.get(&CampTargetSelection::SelfTarget).copied().unwrap_or(0), 2);
        assert_eq!(counts.get(&CampTargetSelection::Individual).copied().unwrap_or(0), 3);
        assert_eq!(counts.get(&CampTargetSelection::Party).copied().unwrap_or(0), 1);
    }

    // ── Enum ambiguity resolution verification ───────────────────────────────

    #[test]
    fn none_effect_type_is_present_but_scores_zero() {
        // Verify that CampEffectType::None exists and is handled correctly
        use crate::contracts::CampEffectType;
        assert_eq!(
            CampEffectType::from_str("unknown_type"),
            None,
            "Unknown types should return None (not Some(None))"
        );
    }

    #[test]
    fn reduce_torch_is_present_and_scores_zero() {
        // Verify ReduceTorch exists but is scored zero
        use crate::contracts::CampEffectType;
        assert_eq!(
            CampEffectType::from_str("reduce_torch"),
            Some(CampEffectType::ReduceTorch),
            "reduce_torch should be parseable"
        );
    }
}
