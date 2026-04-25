//! Camping phase resolution — runner-layer camping skill execution.
//!
//! Implements the camping phase that occurs during dungeon runs (not in town).
//! Heroes can use camping skills for healing, stress relief, buffs, and debuff
//! removal. All behavior derived from original DDGC `CampingSkill.cs`,
//! `CampController.cs`, and `JsonCamping.json`.
//!
//! ## Time Budget
//!
//! Camping skills consume time from a shared pool (default 12 points).
//! Each skill has a `time_cost` that is deducted when the skill is used.
//! The phase tracks `time_spent` to enforce the budget.
//!
//! ## Use Limits
//!
//! Each skill has a `use_limit` specifying how many times it can be used
//! per camp. The phase tracks `skill_uses` to enforce these limits.
//!
//! ## Class Eligibility
//!
//! Skills can be class-specific (e.g., Arbalest's `field_dressing`) or
//! generic (available to all classes). Class-specific skills use the
//! `classes` list to restrict which heroes can use them.
//!
//! ## Targeting
//!
//! Camping skills use DDGC's original targeting semantics:
//! - `SelfTarget`: target must be the hero performing the skill
//! - `Individual`: any single valid hero
//! - `PartyOther`: any hero except the performer
//! - `PartyAll`: no individual target needed

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Default time budget for camping phase (in time points).
pub const DEFAULT_CAMP_TIME_BUDGET: u32 = 12;

/// A camping skill definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampingSkill {
    /// Unique skill identifier.
    pub id: String,
    /// Time cost in points.
    pub time_cost: u32,
    /// Maximum uses per camp.
    pub use_limit: u32,
    /// Whether this skill targets individuals.
    pub has_individual_target: bool,
    /// Hero classes that can use this skill (empty = generic/available to all).
    pub classes: Vec<String>,
    /// Effects applied by this skill.
    pub effects: Vec<CampEffect>,
}

/// A single effect within a camping skill.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampEffect {
    /// Target selection type.
    pub selection: CampTargetSelection,
    /// Prerequisites (usually empty in data).
    pub requirements: Vec<String>,
    /// Probability of effect triggering (1.0 = guaranteed).
    pub chance: f64,
    /// Effect type.
    pub effect_type: CampEffectType,
    /// Buff ID when effect_type is Buff.
    pub sub_type: String,
    /// Numeric parameter (heal amount, percent, etc.).
    pub amount: f64,
}

/// Target selection for camping skills.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CampTargetSelection {
    /// Target must be self (the hero performing the skill).
    SelfTarget,
    /// Any valid individual target.
    Individual,
    /// Any hero except the performer.
    PartyOther,
    /// All party members (no individual target needed).
    PartyAll,
}

/// Effect types for camping skills.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CampEffectType {
    StressHealAmount,
    HealthHealMaxHealthPercent,
    RemoveBleed,
    RemovePoison,
    Buff,
    RemoveDeathRecovery,
    ReduceAmbushChance,
    RemoveDisease,
    StressDamageAmount,
    Loot,
    HealthDamageMaxHealthPercent,
    RemoveBurn,
    RemoveFrozen,
    StressHealPercent,
    RemoveDebuff,
    RemoveAllDebuff,
    HealthHealRange,
    HealthHealAmount,
    ReduceTurbulenceChance,
    ReduceRiptideChance,
}

/// A hero participating in the camp.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeroInCamp {
    /// Unique hero identifier.
    pub hero_id: String,
    /// Hero class ID (e.g., "alchemist", "arbalest").
    pub class_id: String,
    /// Current health.
    pub health: f64,
    /// Maximum health.
    pub max_health: f64,
    /// Current stress level.
    pub stress: f64,
    /// Maximum stress level.
    pub max_stress: f64,
    /// Active buff IDs.
    pub active_buffs: Vec<String>,
    /// Whether the hero can use camping skills (activity lock).
    pub can_use_skills: bool,
}

impl HeroInCamp {
    /// Create a new hero in camp.
    pub fn new(
        hero_id: &str,
        class_id: &str,
        health: f64,
        max_health: f64,
        stress: f64,
        max_stress: f64,
    ) -> Self {
        HeroInCamp {
            hero_id: hero_id.to_string(),
            class_id: class_id.to_string(),
            health,
            max_health,
            stress,
            max_stress,
            active_buffs: Vec::new(),
            can_use_skills: true,
        }
    }

    /// Check if this hero is the same as another.
    pub fn is_hero(&self, hero_id: &str) -> bool {
        self.hero_id == hero_id
    }
}

/// A record of a single camping activity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampActivityRecord {
    /// Skill ID that was used.
    pub skill_id: String,
    /// Hero who performed the skill.
    pub performer_id: String,
    /// Target hero ID (None for PartyAll skills).
    pub target_id: Option<String>,
    /// Time cost consumed.
    pub time_cost: u32,
    /// Whether the skill was successfully executed.
    pub success: bool,
    /// Effects that were applied.
    pub effects_applied: Vec<CampEffectResult>,
}

/// Result of a single effect within a camping skill.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampEffectResult {
    /// Effect type.
    pub effect_type: CampEffectType,
    /// Target hero ID.
    pub target_id: String,
    /// Whether the effect triggered (based on chance roll).
    pub triggered: bool,
    /// Actual amount applied (may differ from base amount due to chance roll).
    pub amount: f64,
}

/// The camping phase state.
///
/// Tracks the shared time budget, per-skill usage counts, participating heroes,
/// and an activity trace for debugging and regression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampingPhase {
    /// Shared time budget (default 12 points).
    pub time_budget: u32,
    /// Time spent so far.
    pub time_spent: u32,
    /// Per-skill usage counts for this camp.
    pub skill_uses: HashMap<String, u32>,
    /// Heroes participating in this camp.
    pub heroes: Vec<HeroInCamp>,
    /// Activity trace of all skill uses.
    pub trace: Vec<CampActivityRecord>,
}

impl CampingPhase {
    /// Create a new camping phase with default time budget.
    pub fn new(heroes: Vec<HeroInCamp>) -> Self {
        CampingPhase {
            time_budget: DEFAULT_CAMP_TIME_BUDGET,
            time_spent: 0,
            skill_uses: HashMap::new(),
            heroes,
            trace: Vec::new(),
        }
    }

    /// Create a camping phase with custom time budget.
    pub fn with_budget(heroes: Vec<HeroInCamp>, time_budget: u32) -> Self {
        CampingPhase {
            time_budget,
            time_spent: 0,
            skill_uses: HashMap::new(),
            heroes,
            trace: Vec::new(),
        }
    }

    /// Get remaining time budget.
    pub fn remaining_time(&self) -> u32 {
        self.time_budget.saturating_sub(self.time_spent)
    }

    /// Check if there is enough time for a skill.
    pub fn has_time_for(&self, time_cost: u32) -> bool {
        self.time_spent + time_cost <= self.time_budget
    }

    /// Get the number of times a skill has been used this camp.
    pub fn skill_use_count(&self, skill_id: &str) -> u32 {
        self.skill_uses.get(skill_id).copied().unwrap_or(0)
    }

    /// Get a hero by ID.
    pub fn get_hero(&self, hero_id: &str) -> Option<&HeroInCamp> {
        self.heroes.iter().find(|h| h.hero_id == hero_id)
    }

    /// Get a mutable hero by ID.
    pub fn get_hero_mut(&mut self, hero_id: &str) -> Option<&mut HeroInCamp> {
        self.heroes.iter_mut().find(|h| h.hero_id == hero_id)
    }

    /// Check if a hero exists and can use skills.
    pub fn hero_can_act(&self, hero_id: &str) -> bool {
        self.get_hero(hero_id)
            .map(|h| h.can_use_skills)
            .unwrap_or(false)
    }
}

/// Result of performing a camping skill.
#[derive(Debug, Clone, PartialEq)]
pub struct CampingSkillResult {
    /// Whether the skill was successfully executed.
    pub success: bool,
    /// Error message if the skill failed.
    pub error: Option<String>,
    /// Activity record (always present, success field indicates actual execution).
    pub record: CampActivityRecord,
}

/// Deterministic pseudo-random roll for effect chance.
///
/// Uses a simple hash of the skill_id, performer_id, target_id, and effect index
/// to produce a value in [0, 1). This ensures deterministic outcomes for the same inputs.
fn deterministic_chance_roll(skill_id: &str, performer_id: &str, target_id: Option<&str>, effect_idx: usize) -> f64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    skill_id.hash(&mut hasher);
    performer_id.hash(&mut hasher);
    if let Some(tid) = target_id {
        tid.hash(&mut hasher);
    }
    effect_idx.hash(&mut hasher);
    let hash = hasher.finish();
    (hash as f64) / (u64::MAX as f64)
}

/// Validate that a target is legal for a camping skill.
///
/// Uses DDGC's original targeting semantics:
/// - `SelfTarget`: target must be the performer
/// - `Individual`: any valid hero
/// - `PartyOther`: any hero except the performer
/// - `PartyAll`: no individual target needed
fn validate_target(
    selection: CampTargetSelection,
    performer_id: &str,
    target_id: Option<&str>,
    heroes: &[HeroInCamp],
) -> Result<(), String> {
    match selection {
        CampTargetSelection::SelfTarget => {
            let target = target_id.ok_or("SelfTarget skill requires a target")?;
            if target != performer_id {
                return Err(format!(
                    "SelfTarget skill must target self (performer={}, target={})",
                    performer_id, target
                ));
            }
            // Verify target exists in heroes
            if !heroes.iter().any(|h| h.hero_id == target) {
                return Err(format!("Target hero '{}' not found in camp", target));
            }
        }
        CampTargetSelection::Individual => {
            let target = target_id.ok_or("Individual skill requires a target")?;
            // Verify target exists in heroes
            if !heroes.iter().any(|h| h.hero_id == target) {
                return Err(format!("Target hero '{}' not found in camp", target));
            }
        }
        CampTargetSelection::PartyOther => {
            let target = target_id.ok_or("PartyOther skill requires a target")?;
            if target == performer_id {
                return Err(format!(
                    "PartyOther skill cannot target self (performer={})",
                    performer_id
                ));
            }
            // Verify target exists in heroes
            if !heroes.iter().any(|h| h.hero_id == target) {
                return Err(format!("Target hero '{}' not found in camp", target));
            }
        }
        CampTargetSelection::PartyAll => {
            // No individual target needed for PartyAll - target is ignored if provided
            // Just verify the target exists if one is provided
            if let Some(target) = target_id {
                if !heroes.iter().any(|h| h.hero_id == target) {
                    return Err(format!("Target hero '{}' not found in camp", target));
                }
            }
        }
    }
    Ok(())
}

/// Check if a hero's class is eligible to use a skill.
///
/// Generic skills (classes list is empty) are available to all classes.
/// Class-specific skills require the hero's class to be in the classes list.
fn check_class_eligibility(hero_class: &str, skill_classes: &[String]) -> bool {
    // Empty classes list means generic (available to all)
    if skill_classes.is_empty() {
        return true;
    }
    skill_classes.iter().any(|c| c == hero_class)
}

/// Perform a camping skill during the camping phase.
///
/// This function:
/// 1. Validates time budget before execution
/// 2. Validates use limits per camp
/// 3. Validates class eligibility and target legality
/// 4. On success: deducts time, records usage, appends trace record
///
/// Returns a result indicating success or failure with an activity record.
pub fn perform_camping_skill(
    phase: &mut CampingPhase,
    skill: &CampingSkill,
    performer_id: &str,
    target_id: Option<&str>,
) -> CampingSkillResult {
    let skill_id = &skill.id;
    let time_cost = skill.time_cost;

    // 1. Validate performer exists and can act
    let performer = match phase.get_hero(performer_id) {
        Some(h) => h,
        None => {
            return CampingSkillResult {
                success: false,
                error: Some(format!("Performer hero '{}' not found in camp", performer_id)),
                record: CampActivityRecord {
                    skill_id: skill_id.clone(),
                    performer_id: performer_id.to_string(),
                    target_id: target_id.map(String::from),
                    time_cost,
                    success: false,
                    effects_applied: Vec::new(),
                },
            };
        }
    };

    if !performer.can_use_skills {
        return CampingSkillResult {
            success: false,
            error: Some(format!(
                "Hero '{}' cannot use skills (activity locked)",
                performer_id
            )),
            record: CampActivityRecord {
                skill_id: skill_id.clone(),
                performer_id: performer_id.to_string(),
                target_id: target_id.map(String::from),
                time_cost,
                success: false,
                effects_applied: Vec::new(),
            },
        };
    }

    // 2. Validate time budget
    if !phase.has_time_for(time_cost) {
        return CampingSkillResult {
            success: false,
            error: Some(format!(
                "Insufficient time: need {}, have {}",
                time_cost,
                phase.remaining_time()
            )),
            record: CampActivityRecord {
                skill_id: skill_id.clone(),
                performer_id: performer_id.to_string(),
                target_id: target_id.map(String::from),
                time_cost,
                success: false,
                effects_applied: Vec::new(),
            },
        };
    }

    // 3. Validate use limit
    let current_uses = phase.skill_use_count(skill_id);
    if current_uses >= skill.use_limit {
        return CampingSkillResult {
            success: false,
            error: Some(format!(
                "Skill '{}' use limit reached: {}/{}",
                skill_id, current_uses, skill.use_limit
            )),
            record: CampActivityRecord {
                skill_id: skill_id.clone(),
                performer_id: performer_id.to_string(),
                target_id: target_id.map(String::from),
                time_cost,
                success: false,
                effects_applied: Vec::new(),
            },
        };
    }

    // 4. Validate class eligibility
    if !check_class_eligibility(&performer.class_id, &skill.classes) {
        return CampingSkillResult {
            success: false,
            error: Some(format!(
                "Hero class '{}' cannot use skill '{}'",
                performer.class_id, skill_id
            )),
            record: CampActivityRecord {
                skill_id: skill_id.clone(),
                performer_id: performer_id.to_string(),
                target_id: target_id.map(String::from),
                time_cost,
                success: false,
                effects_applied: Vec::new(),
            },
        };
    }

    // 5. Validate target legality using original targeting semantics
    // Use the first effect's selection (all effects in a skill share the same selection)
    let selection = skill.effects.first()
        .map(|e| e.selection)
        .unwrap_or(CampTargetSelection::Individual);

    if let Err(e) = validate_target(selection, performer_id, target_id, &phase.heroes) {
        return CampingSkillResult {
            success: false,
            error: Some(e),
            record: CampActivityRecord {
                skill_id: skill_id.clone(),
                performer_id: performer_id.to_string(),
                target_id: target_id.map(String::from),
                time_cost,
                success: false,
                effects_applied: Vec::new(),
            },
        };
    }

    // 6. Skill is valid — apply effects and record
    let mut effects_applied = Vec::new();

    // For PartyAll skills, the target is all heroes
    // For other skills, the target is the specified hero
    // Collect as owned strings to avoid borrow issues
    let target_hero_ids: Vec<String> = match selection {
        CampTargetSelection::PartyAll => phase.heroes.iter().map(|h| h.hero_id.clone()).collect(),
        _ => {
            if let Some(tid) = target_id {
                vec![tid.to_string()]
            } else {
                Vec::new()
            }
        }
    };

    for (effect_idx, effect) in skill.effects.iter().enumerate() {
        // Roll for chance
        let roll = deterministic_chance_roll(skill_id, performer_id, target_id, effect_idx);
        let triggered = roll < effect.chance;

        for target_hero_id in &target_hero_ids {
            let result = CampEffectResult {
                effect_type: effect.effect_type,
                target_id: target_hero_id.clone(),
                triggered,
                amount: if triggered { effect.amount } else { 0.0 },
            };
            effects_applied.push(result);

            // Apply effect to hero state (if triggered)
            if triggered {
                apply_effect_to_hero(phase, target_hero_id, effect);
            }
        }
    }

    // 7. Deduct time and increment use counter
    phase.time_spent += time_cost;
    *phase.skill_uses.entry(skill_id.clone()).or_insert(0) += 1;

    // 8. Record activity in trace
    let record = CampActivityRecord {
        skill_id: skill_id.clone(),
        performer_id: performer_id.to_string(),
        target_id: target_id.map(String::from),
        time_cost,
        success: true,
        effects_applied,
    };

    phase.trace.push(record.clone());

    CampingSkillResult {
        success: true,
        error: None,
        record,
    }
}

/// Apply a camping effect to a hero.
fn apply_effect_to_hero(phase: &mut CampingPhase, hero_id: &str, effect: &CampEffect) {
    // Import the effect type to use in the match
    use CampEffectType::*;

    let hero = match phase.get_hero_mut(hero_id) {
        Some(h) => h,
        None => return,
    };

    match effect.effect_type {
        StressHealAmount => {
            hero.stress = (hero.stress - effect.amount).max(0.0);
        }
        HealthHealMaxHealthPercent => {
            let heal_amount = hero.max_health * effect.amount;
            hero.health = (hero.health + heal_amount).min(hero.max_health);
        }
        RemoveBleed => {
            // Stub: would remove "bleed" status
            hero.active_buffs.retain(|b| b != "bleed");
        }
        RemovePoison => {
            // Stub: would remove "poison" status
            hero.active_buffs.retain(|b| b != "poison");
        }
        Buff => {
            if !hero.active_buffs.contains(&effect.sub_type) {
                hero.active_buffs.push(effect.sub_type.clone());
            }
        }
        RemoveDeathRecovery => {
            // Stub: would remove death's door recovery debuff
            hero.active_buffs.retain(|b| b != "death_recovery");
        }
        ReduceAmbushChance => {
            // Stub: would set ambush chance modifier
        }
        RemoveDisease => {
            // Stub: would remove disease
            hero.active_buffs.retain(|b| !b.starts_with("disease_"));
        }
        StressDamageAmount => {
            hero.stress += effect.amount;
        }
        Loot => {
            // Stub: would add loot to party inventory
        }
        HealthDamageMaxHealthPercent => {
            let damage = hero.max_health * effect.amount;
            hero.health = (hero.health - damage).max(0.0);
        }
        RemoveBurn => {
            // Stub: would remove "burning" status
            hero.active_buffs.retain(|b| b != "burning");
        }
        RemoveFrozen => {
            // Stub: would remove "frozen" status
            hero.active_buffs.retain(|b| b != "frozen");
        }
        StressHealPercent => {
            let heal_amount = hero.max_stress * effect.amount;
            hero.stress = (hero.stress - heal_amount).max(0.0);
        }
        RemoveDebuff => {
            // Stub: would remove one debuff
            if !hero.active_buffs.is_empty() {
                hero.active_buffs.remove(0);
            }
        }
        RemoveAllDebuff => {
            // Stub: would remove all debuffs
            hero.active_buffs.retain(|b| !b.starts_with("debuff_"));
        }
        HealthHealRange => {
            // Stub: would heal random amount (requires RNG seeding)
            // For deterministic testing, use a fixed amount
            let heal_amount = effect.amount * 0.5; // deterministic fallback
            hero.health = (hero.health + heal_amount).min(hero.max_health);
        }
        HealthHealAmount => {
            hero.health = (hero.health + effect.amount).min(hero.max_health);
        }
        ReduceTurbulenceChance => {
            // Stub: would set turbulence modifier
        }
        ReduceRiptideChance => {
            // Stub: would set riptide modifier
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: create a simple camping skill.
    fn make_skill(
        id: &str,
        time_cost: u32,
        use_limit: u32,
        classes: Vec<&str>,
        selection: CampTargetSelection,
    ) -> CampingSkill {
        CampingSkill {
            id: id.to_string(),
            time_cost,
            use_limit,
            has_individual_target: matches!(selection, CampTargetSelection::Individual | CampTargetSelection::PartyOther),
            classes: classes.iter().map(|s| s.to_string()).collect(),
            effects: vec![CampEffect {
                selection,
                requirements: Vec::new(),
                chance: 1.0,
                effect_type: CampEffectType::StressHealAmount,
                sub_type: String::new(),
                amount: 10.0,
            }],
        }
    }

    /// Helper: create a simple hero in camp.
    fn make_hero(hero_id: &str, class_id: &str) -> HeroInCamp {
        HeroInCamp::new(hero_id, class_id, 100.0, 100.0, 50.0, 200.0)
    }

    // ── CampingPhase construction tests ────────────────────────────────────────

    #[test]
    fn camping_phase_new_has_default_budget() {
        let heroes = vec![make_hero("h1", "alchemist")];
        let phase = CampingPhase::new(heroes);
        assert_eq!(phase.time_budget, DEFAULT_CAMP_TIME_BUDGET);
        assert_eq!(phase.time_spent, 0);
        assert!(phase.skill_uses.is_empty());
    }

    #[test]
    fn camping_phase_with_custom_budget() {
        let heroes = vec![make_hero("h1", "alchemist")];
        let phase = CampingPhase::with_budget(heroes, 20);
        assert_eq!(phase.time_budget, 20);
        assert_eq!(phase.time_spent, 0);
    }

    // ── Time budget tests ───────────────────────────────────────────────────────

    #[test]
    fn has_time_for_returns_true_when_enough_time() {
        let heroes = vec![make_hero("h1", "alchemist")];
        let phase = CampingPhase::with_budget(heroes, 12);
        assert!(phase.has_time_for(5));
        assert!(phase.has_time_for(12));
    }

    #[test]
    fn has_time_for_returns_false_when_not_enough_time() {
        let heroes = vec![make_hero("h1", "alchemist")];
        let phase = CampingPhase::with_budget(heroes, 12);
        assert!(!phase.has_time_for(13));
    }

    #[test]
    fn remaining_time_decreases_as_time_spent() {
        let heroes = vec![make_hero("h1", "alchemist")];
        let mut phase = CampingPhase::with_budget(heroes, 12);
        assert_eq!(phase.remaining_time(), 12);
        phase.time_spent = 5;
        assert_eq!(phase.remaining_time(), 7);
    }

    #[test]
    fn time_budget_exhaustion_blocks_skill() {
        let heroes = vec![make_hero("h1", "alchemist")];
        let mut phase = CampingPhase::with_budget(heroes, 12);

        // Use skills until time runs out
        let skill = make_skill("test_skill", 5, 10, vec![], CampTargetSelection::SelfTarget);

        // First use: 12 - 5 = 7 remaining
        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
        assert!(result.success);

        // Second use: 7 - 5 = 2 remaining
        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
        assert!(result.success);

        // Third use: 2 - 5 = not enough
        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
        assert!(!result.success);
        assert!(result.error.unwrap().contains("Insufficient time"));
    }

    #[test]
    fn time_cost_deducted_on_success() {
        let heroes = vec![make_hero("h1", "alchemist")];
        let mut phase = CampingPhase::with_budget(heroes, 12);

        let skill = make_skill("test_skill", 3, 10, vec![], CampTargetSelection::SelfTarget);
        assert_eq!(phase.time_spent, 0);

        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
        assert!(result.success);
        assert_eq!(phase.time_spent, 3);

        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
        assert!(result.success);
        assert_eq!(phase.time_spent, 6);
    }

    // ── Use limit tests ─────────────────────────────────────────────────────────

    #[test]
    fn use_limit_enforced_per_skill() {
        let heroes = vec![make_hero("h1", "alchemist")];
        let mut phase = CampingPhase::with_budget(heroes, 100);

        let skill = make_skill("limited_skill", 1, 2, vec![], CampTargetSelection::SelfTarget);

        // First use: allowed
        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
        assert!(result.success);
        assert_eq!(phase.skill_use_count("limited_skill"), 1);

        // Second use: allowed (at limit)
        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
        assert!(result.success);
        assert_eq!(phase.skill_use_count("limited_skill"), 2);

        // Third use: blocked
        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
        assert!(!result.success);
        assert!(result.error.unwrap().contains("use limit reached"));
    }

    #[test]
    fn different_skills_have_independent_limits() {
        let heroes = vec![make_hero("h1", "alchemist")];
        let mut phase = CampingPhase::with_budget(heroes, 100);

        let skill_a = make_skill("skill_a", 1, 1, vec![], CampTargetSelection::SelfTarget);
        let skill_b = make_skill("skill_b", 1, 1, vec![], CampTargetSelection::SelfTarget);

        // Use skill_a
        let result = perform_camping_skill(&mut phase, &skill_a, "h1", Some("h1"));
        assert!(result.success);

        // skill_a is exhausted, but skill_b should still work
        let result = perform_camping_skill(&mut phase, &skill_a, "h1", Some("h1"));
        assert!(!result.success);

        let result = perform_camping_skill(&mut phase, &skill_b, "h1", Some("h1"));
        assert!(result.success);
    }

    // ── Class restriction tests ─────────────────────────────────────────────────

    #[test]
    fn class_specific_skill_blocks_wrong_class() {
        let heroes = vec![make_hero("h1", "alchemist")];
        let mut phase = CampingPhase::new(heroes);

        // Arbalest-only skill
        let skill = make_skill("arbalest_skill", 2, 10, vec!["arbalest"], CampTargetSelection::SelfTarget);

        // Alchemist cannot use arbalest skill
        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
        assert!(!result.success);
        assert!(result.error.unwrap().contains("cannot use skill"));
    }

    #[test]
    fn class_specific_skill_allows_correct_class() {
        let heroes = vec![make_hero("h1", "arbalest")];
        let mut phase = CampingPhase::new(heroes);

        // Arbalest-only skill
        let skill = make_skill("arbalest_skill", 2, 10, vec!["arbalest"], CampTargetSelection::SelfTarget);

        // Arbalest can use arbalest skill
        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
        assert!(result.success);
    }

    #[test]
    fn generic_skill_available_to_all_classes() {
        let classes = ["alchemist", "arbalest", "hunter", "crusader"];
        for class_id in classes {
            let heroes = vec![make_hero("h1", class_id)];
            let mut phase = CampingPhase::new(heroes);

            // Generic skill (empty classes list)
            let skill = make_skill("generic_skill", 2, 10, vec![], CampTargetSelection::SelfTarget);

            let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
            assert!(result.success, "Generic skill should be available to class '{}'", class_id);
        }
    }

    #[test]
    fn multi_class_skill_allows_any_listed_class() {
        let allowed_classes = vec!["arbalest", "musketeer"];
        for class_id in &allowed_classes {
            let heroes = vec![make_hero("h1", class_id)];
            let mut phase = CampingPhase::new(heroes);

            let skill = make_skill("multi_class_skill", 2, 10, allowed_classes.clone(), CampTargetSelection::SelfTarget);

            let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
            assert!(result.success, "Class '{}' should be allowed for multi-class skill", class_id);
        }
    }

    // ── Targeting validation tests ──────────────────────────────────────────────

    #[test]
    fn self_target_requires_performer_as_target() {
        let heroes = vec![make_hero("h1", "alchemist"), make_hero("h2", "alchemist")];
        let mut phase = CampingPhase::new(heroes);

        let skill = make_skill("self_skill", 2, 10, vec![], CampTargetSelection::SelfTarget);

        // Targeting self is allowed
        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
        assert!(result.success);

        // Targeting other is not allowed for SelfTarget
        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h2"));
        assert!(!result.success);
        assert!(result.error.unwrap().contains("must target self"));
    }

    #[test]
    fn party_other_cannot_target_self() {
        let heroes = vec![make_hero("h1", "alchemist"), make_hero("h2", "alchemist")];
        let mut phase = CampingPhase::new(heroes);

        let skill = make_skill("party_other_skill", 2, 10, vec![], CampTargetSelection::PartyOther);

        // Cannot target self
        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
        assert!(!result.success);
        assert!(result.error.unwrap().contains("cannot target self"));
    }

    #[test]
    fn party_other_allows_other_targets() {
        let heroes = vec![make_hero("h1", "alchemist"), make_hero("h2", "alchemist")];
        let mut phase = CampingPhase::new(heroes);

        let skill = make_skill("party_other_skill", 2, 10, vec![], CampTargetSelection::PartyOther);

        // Targeting other hero is allowed
        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h2"));
        assert!(result.success);
    }

    #[test]
    fn party_all_ignores_target() {
        let heroes = vec![make_hero("h1", "alchemist"), make_hero("h2", "alchemist")];
        let mut phase = CampingPhase::new(heroes);

        let skill = make_skill("party_all_skill", 2, 10, vec![], CampTargetSelection::PartyAll);

        // PartyAll should succeed even without a target
        let result = perform_camping_skill(&mut phase, &skill, "h1", None);
        assert!(result.success);

        // PartyAll should also succeed with a target (target is ignored)
        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h2"));
        assert!(result.success);
    }

    #[test]
    fn individual_target_accepts_any_hero() {
        let heroes = vec![make_hero("h1", "alchemist"), make_hero("h2", "alchemist")];
        let mut phase = CampingPhase::new(heroes);

        let skill = make_skill("individual_skill", 2, 10, vec![], CampTargetSelection::Individual);

        // Can target self
        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
        assert!(result.success);

        // Can target other
        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h2"));
        assert!(result.success);
    }

    // ── Activity trace tests ────────────────────────────────────────────────────

    #[test]
    fn successful_skill_appends_trace_record() {
        let heroes = vec![make_hero("h1", "alchemist")];
        let mut phase = CampingPhase::new(heroes);

        let skill = make_skill("trace_test", 2, 10, vec![], CampTargetSelection::SelfTarget);
        assert!(phase.trace.is_empty());

        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
        assert!(result.success);
        assert_eq!(phase.trace.len(), 1);
        assert_eq!(phase.trace[0].skill_id, "trace_test");
        assert_eq!(phase.trace[0].performer_id, "h1");
        assert!(phase.trace[0].success);
    }

    #[test]
    fn failed_skill_does_not_append_trace() {
        let heroes = vec![make_hero("h1", "alchemist")];
        let mut phase = CampingPhase::new(heroes);

        let skill = make_skill("trace_test", 100, 10, vec![], CampTargetSelection::SelfTarget);
        assert!(phase.trace.is_empty());

        // This will fail due to time budget
        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
        assert!(!result.success);
        // Failed skills still have a record in the result, but don't modify phase.trace
        assert!(phase.trace.is_empty());
    }

    #[test]
    fn trace_record_contains_skill_and_target() {
        let heroes = vec![make_hero("h1", "alchemist"), make_hero("h2", "alchemist")];
        let mut phase = CampingPhase::new(heroes);

        let skill = make_skill("targeted_skill", 2, 10, vec![], CampTargetSelection::PartyOther);

        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h2"));
        assert!(result.success);

        let record = &phase.trace[0];
        assert_eq!(record.skill_id, "targeted_skill");
        assert_eq!(record.performer_id, "h1");
        assert_eq!(record.target_id.as_deref(), Some("h2"));
        assert_eq!(record.time_cost, 2);
    }

    #[test]
    fn deterministic_trace_for_same_inputs() {
        let heroes = vec![make_hero("h1", "alchemist")];
        let mut phase1 = CampingPhase::new(heroes);
        let heroes2 = vec![make_hero("h1", "alchemist")];
        let mut phase2 = CampingPhase::new(heroes2);

        let skill = make_skill("det_skill", 2, 10, vec![], CampTargetSelection::SelfTarget);

        let result1 = perform_camping_skill(&mut phase1, &skill, "h1", Some("h1"));
        let result2 = perform_camping_skill(&mut phase2, &skill, "h1", Some("h1"));

        assert_eq!(result1.success, result2.success);
        assert_eq!(result1.record.time_cost, result2.record.time_cost);
    }

    // ── Hero can_act tests ──────────────────────────────────────────────────────

    #[test]
    fn activity_locked_hero_cannot_act() {
        let mut heroes = vec![make_hero("h1", "alchemist")];
        heroes[0].can_use_skills = false;

        let mut phase = CampingPhase::new(heroes);

        let skill = make_skill("test_skill", 2, 10, vec![], CampTargetSelection::SelfTarget);
        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
        assert!(!result.success);
        assert!(result.error.unwrap().contains("cannot use skills"));
    }

    #[test]
    fn nonexistent_hero_fails() {
        let heroes = vec![make_hero("h1", "alchemist")];
        let mut phase = CampingPhase::new(heroes);

        let skill = make_skill("test_skill", 2, 10, vec![], CampTargetSelection::SelfTarget);
        let result = perform_camping_skill(&mut phase, &skill, "nonexistent", Some("nonexistent"));
        assert!(!result.success);
        assert!(result.error.unwrap().contains("not found in camp"));
    }
}