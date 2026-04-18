//! Hunter hero — ranged physical DPS/debuffer class family base variant.
//!
//! DDGC reference: highest base physical damage, mark synergy, and strong
//! conditional effects against tagged targets.
//! Base weapon damage at level 0: 35–45, averaged to 40.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Hunter base archetype — level 0 stats from DDGC data.
///
/// HP 152, weapon damage 35–45 (avg 40), speed 6, dodge 5%, crit 2%.
/// Highest base physical damage — primary DPS role.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Hunter"),
        side: CombatSide::Ally,
        health: 152.0,
        max_health: 152.0,
        attack: 40.0,
        defense: 0.0,
        speed: 6.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.02,
        accuracy: 0.95,
        dodge: 0.05,
    }
}

// ── Hunter Skills ──────────────────────────────────────────────────────────

/// Mark Skill — tag enemy for conditional effects.
///
/// DDGC reference: -90% damage (avg 4) + apply tagged status for 2 rounds.
/// Tagged enables Hunter's conditional effects (bleed on marked, ignore def on marked).
pub fn mark_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("mark_skill"),
        vec![
            EffectNode::damage(4.0),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Pull Skill — pull back-rank enemy forward.
///
/// DDGC reference: -50% damage (avg 20) + pull target 2 positions forward.
/// Game-gap: rank targeting and pull not modeled — damage only.
pub fn pull_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("pull_skill"),
        vec![EffectNode::damage(20.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// AoE Skill — AoE attack + dodge debuff.
///
/// DDGC reference: -50% damage (avg 20) to all enemies + dodge debuff -10% for 2 rounds.
/// Game-gap: dodge debuff not modeled — damage only.
pub fn aoe_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("aoe_skill"),
        vec![EffectNode::damage(20.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Stun Skill — stun front-rank enemy.
///
/// DDGC reference: -50% damage (avg 20) + guaranteed stun on hit.
/// Game-gap: rank targeting not modeled.
pub fn stun_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("stun_skill"),
        vec![
            EffectNode::damage(20.0),
            EffectNode::apply_status("stun", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Ignore Def Skill — full damage, bypass defense if target tagged.
///
/// DDGC reference: full weapon damage (0% mod, avg 40) + ignore defense if target
/// is tagged + self moves forward 1 rank.
/// Game-gap: ignore defense conditional and movement not modeled.
pub fn ignore_def_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("ignore_def_skill"),
        vec![EffectNode::damage(40.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Bleed Skill — reduced damage + bleed on marked target.
///
/// DDGC reference: -50% damage (avg 20) + apply bleed DoT 12/round for 3 rounds
/// if target has tagged status.
/// Simplification: bleed applied unconditionally (tagged condition not modeled).
pub fn bleed_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("bleed_skill"),
        vec![
            EffectNode::damage(20.0),
            EffectNode::apply_status("bleed", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Buff Skill — self-buff + reposition backward.
///
/// DDGC reference: self moves back 1 rank + ACC +10% for 1 round.
/// Game-gap: movement and buff not modeled — minimal heal placeholder.
pub fn buff_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("buff_skill"),
        vec![EffectNode::heal(1.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Opening Strike — first-round bonus attack.
///
/// DDGC reference: full weapon damage (avg 40) on first round, reduced damage (avg 20) after.
/// This skill demonstrates the FirstRound DDGC condition: the bonus damage effect
/// is only applied when the battle is on round 1.
///
/// Implementation: two effect nodes - normal damage always applies, bonus damage
/// only applies on first round (via GameCondition with ddgc_first_round tag).
pub fn opening_strike() -> SkillDefinition {
    // Normal damage effect (always applies)
    let normal_damage = EffectNode::damage(20.0);

    // Bonus damage effect (only on first round) - uses DDGC GameCondition
    // The condition tag "ddgc_first_round" is evaluated by the game-layer
    // ConditionAdapter via the game_condition_evaluator set on EffectContext.
    let bonus_damage = EffectNode::damage(20.0).with_game_condition("ddgc_first_round");

    SkillDefinition::new(
        SkillId::new("opening_strike"),
        vec![normal_damage, bonus_damage],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Desperate Strike — bonus damage when at death's door.
///
/// DDGC reference: bonus damage when actor is near death (HP < 50%).
/// This skill demonstrates the DeathsDoor DDGC condition: the bonus damage effect
/// is only applied when the actor's HP drops below 50%.
///
/// Implementation: two effect nodes - normal damage always applies, bonus damage
/// only applies when at deaths door (via GameCondition with ddgc_deaths_door tag).
pub fn desperate_strike() -> SkillDefinition {
    // Normal damage effect (always applies)
    let normal_damage = EffectNode::damage(15.0);

    // Bonus damage effect (only when at deaths door) - uses DDGC GameCondition
    // The condition tag "ddgc_deaths_door" is evaluated by the game-layer
    // ConditionAdapter via the game_condition_evaluator set on EffectContext.
    let bonus_damage = EffectNode::damage(25.0).with_game_condition("ddgc_deaths_door");

    SkillDefinition::new(
        SkillId::new("desperate_strike"),
        vec![normal_damage, bonus_damage],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 8 Hunter base skills (DDGC template + Opening Strike).
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        mark_skill(),
        pull_skill(),
        aoe_skill(),
        stun_skill(),
        ignore_def_skill(),
        bleed_skill(),
        buff_skill(),
        opening_strike(),
    ]
}
