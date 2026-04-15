//! Diviner hero — utility/crowd-control class family base variant.
//!
//! DDGC reference: manipulation-focused hero with push/pull, mark, and
//! divine oracle system. Most skills deal reduced damage (-50% to -75%).
//! Base weapon damage at level 0: 27–45, averaged to 36.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Diviner base archetype — level 0 stats from DDGC data.
///
/// HP 160, weapon damage 27–45 (avg 36), speed 5, dodge 5%, crit 2%.
/// Utility-focused — most skills have heavy damage penalties.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Diviner"),
        side: CombatSide::Ally,
        health: 160.0,
        max_health: 160.0,
        attack: 36.0,
        defense: 0.0,
        speed: 5.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.02,
        accuracy: 0.95,
        dodge: 0.05,
    }
}

// ── Diviner Skills ─────────────────────────────────────────────────────────

/// Duality Fate — ranged attack on back ranks, ignore defense.
///
/// DDGC reference: -75% damage (avg 9), ignores defense rating, pulls target
/// forward 1 position. High base crit (5.4%).
/// Game-gap: ignore defense, pull, and rank targeting not modeled.
pub fn duality_fate() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("duality_fate"),
        vec![EffectNode::damage(9.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Repel — reduced damage + push.
///
/// DDGC reference: -67% damage (avg 12) + push enemy back 1 position.
/// Game-gap: push effect not modeled — damage only.
pub fn repel() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("repel"),
        vec![EffectNode::damage(12.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Blessed Evasion — reduced damage + pull.
///
/// DDGC reference: -50% damage (avg 18) + pull enemy forward 1 position.
/// Game-gap: pull effect not modeled — damage only.
pub fn blessed_evasion() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("blessed_evasion"),
        vec![EffectNode::damage(18.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Pull Skill — target rearmost enemy, pull 2 positions.
///
/// DDGC reference: -75% damage (avg 9) + pull target 2 positions forward.
/// Game-gap: rank targeting and pull not modeled — damage only.
pub fn pull_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("pull_skill"),
        vec![EffectNode::damage(9.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Survive — self-reposition + accuracy buff.
///
/// DDGC reference: moves forward 1 rank + ACC +10% for 1 round.
/// Game-gap: movement and buff not modeled — minimal heal placeholder.
pub fn survive() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("survive"),
        vec![EffectNode::heal(1.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Draw Stick — divine oracle system.
///
/// DDGC reference: 60/40 gamble — Lucky grants party buffs, Bad applies debuffs.
/// Consumes 2 bad results to force Lucky outcome.
/// Game-gap: oracle system not modeled — minimal heal placeholder.
pub fn draw_stick() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("draw_stick"),
        vec![EffectNode::heal(1.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Karmic Cycle — attack with multi-hit scaling from divine stacks.
///
/// DDGC reference: -50% damage (avg 18), extra hits scale with divine stack count.
/// Game-gap: divine stack multi-hit not modeled — single-hit damage only.
pub fn karmic_cycle() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("karmic_cycle"),
        vec![EffectNode::damage(18.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 7 Diviner base skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        duality_fate(),
        repel(),
        blessed_evasion(),
        pull_skill(),
        survive(),
        draw_stick(),
        karmic_cycle(),
    ]
}
