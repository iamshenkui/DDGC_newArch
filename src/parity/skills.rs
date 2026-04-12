//! Skill semantic parity fixtures — expectations for all 6 migrated skills.
//!
//! These fixtures capture the original DDGC targeting semantics, position
//! constraints, effect chains, and usage restrictions. Parity tests use these
//! fixtures to verify the migration preserves skill identity rather than
//! flattening skills into generic damage/heal templates.

/// Parity expectations for a single skill.
#[derive(Debug, Clone)]
pub struct SkillExpectation {
    pub name: &'static str,
    pub target_selector: TargetSelectorPattern,
    pub action_cost: u32,
    pub cooldown: Option<u32>,
    pub effect_count: usize,
    pub effect_chain: Vec<EffectExpectation>,
}

/// Pattern matching for TargetSelector — avoids requiring exact enum equality
/// since some variants carry data that's hard to compare.
#[derive(Debug, Clone)]
pub enum TargetSelectorPattern {
    AllEnemies,
    AllAllies,
}

/// Expectation for a single effect in a skill's effect chain.
#[derive(Debug, Clone)]
pub enum EffectExpectation {
    Damage,
    Heal,
    ApplyStatus,
    ConditionalApplyStatus,
}

/// Fixture bundle containing parity expectations for all 6 migrated skills.
pub struct SkillParityFixture {
    pub crusading_strike: SkillExpectation,
    pub holy_lance: SkillExpectation,
    pub divine_grace: SkillExpectation,
    pub rend: SkillExpectation,
    pub skull_bash: SkillExpectation,
    pub grave_bash: SkillExpectation,
}

impl SkillParityFixture {
    pub fn new() -> Self {
        SkillParityFixture {
            crusading_strike: SkillExpectation {
                name: "crusading_strike",
                target_selector: TargetSelectorPattern::AllEnemies,
                action_cost: 1,
                cooldown: None,
                effect_count: 1,
                effect_chain: vec![EffectExpectation::Damage],
            },
            holy_lance: SkillExpectation {
                name: "holy_lance",
                target_selector: TargetSelectorPattern::AllEnemies,
                action_cost: 1,
                cooldown: Some(2),
                effect_count: 2,
                effect_chain: vec![EffectExpectation::Damage, EffectExpectation::Heal],
            },
            divine_grace: SkillExpectation {
                name: "divine_grace",
                target_selector: TargetSelectorPattern::AllAllies,
                action_cost: 1,
                cooldown: None,
                effect_count: 1,
                effect_chain: vec![EffectExpectation::Heal],
            },
            rend: SkillExpectation {
                name: "rend",
                target_selector: TargetSelectorPattern::AllEnemies,
                action_cost: 1,
                cooldown: None,
                effect_count: 2,
                effect_chain: vec![EffectExpectation::Damage, EffectExpectation::ApplyStatus],
            },
            skull_bash: SkillExpectation {
                name: "skull_bash",
                target_selector: TargetSelectorPattern::AllEnemies,
                action_cost: 1,
                cooldown: Some(3),
                effect_count: 2,
                effect_chain: vec![EffectExpectation::Damage, EffectExpectation::ConditionalApplyStatus],
            },
            grave_bash: SkillExpectation {
                name: "grave_bash",
                target_selector: TargetSelectorPattern::AllEnemies,
                action_cost: 1,
                cooldown: None,
                effect_count: 2,
                effect_chain: vec![EffectExpectation::Damage, EffectExpectation::Damage],
            },
        }
    }

    /// Returns all skill expectations as a slice for iteration.
    pub fn all(&self) -> [&SkillExpectation; 6] {
        [
            &self.crusading_strike,
            &self.holy_lance,
            &self.divine_grace,
            &self.rend,
            &self.skull_bash,
            &self.grave_bash,
        ]
    }
}

impl Default for SkillParityFixture {
    fn default() -> Self {
        Self::new()
    }
}
