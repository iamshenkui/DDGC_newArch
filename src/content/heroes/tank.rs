//! Tank hero — melee frontline/protector class family base variant.
//!
//! DDGC reference: highest HP pool, frontline guardian with self-mark mechanics,
//! riposte, taunt, and bleed synergy. Fastest hero at base speed 7.
//! Base weapon damage at level 0: 27–35, averaged to 31.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Tank base archetype — level 0 stats from DDGC data.
///
/// HP 192, weapon damage 27–35 (avg 31), speed 7, dodge 0%, crit 3%.
/// Highest HP pool and highest base speed — dedicated frontline protector.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Tank"),
        side: CombatSide::Ally,
        health: 192.0,
        max_health: 192.0,
        attack: 31.0,
        defense: 0.0,
        speed: 7.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.03,
        accuracy: 0.95,
        dodge: 0.00,
    }
}

// ── Tank Skills ────────────────────────────────────────────────────────────

/// Protect Skill — guard ally + damage reduction.
///
/// DDGC reference: guard an ally for 3 rounds + reduce their incoming damage by 20%.
/// Game-gap: single-ally targeting not supported — guard applied to all allies.
pub fn protect_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("protect_skill"),
        vec![EffectNode::apply_status("guard", Some(3))],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Attack Reduce — damage + enemy damage debuff.
///
/// DDGC reference: -50% damage (avg 16) + enemy damage -20% for 3 rounds +
/// self-tag Tank for 3 rounds.
/// Game-gap: enemy debuff and self-mark not modeled — damage only.
pub fn attack_reduce() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("attack_reduce"),
        vec![EffectNode::damage(16.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Taunt Skill — full damage + self-mark + protection buff.
///
/// DDGC reference: full weapon damage (0% mod, avg 31) + self-tagged for 3 rounds
/// + self protection +10% buff.
///   Game-gap: self-mark and prot buff not modeled — damage only.
pub fn taunt_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("taunt_skill"),
        vec![EffectNode::damage(31.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Active Riposte — reduced damage + mark enemy + riposte stance.
///
/// DDGC reference: -50% damage (avg 16) + mark enemy as tagged for 3 rounds
/// + activate riposte counter for 3 rounds.
///   Game-gap: riposte self-buff not modeled — damage + enemy tag only.
pub fn active_riposte() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("active_riposte"),
        vec![
            EffectNode::damage(16.0),
            EffectNode::apply_status("tagged", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Blood Oath — damage + bleed (stronger if tagged).
///
/// DDGC reference: -50% damage (avg 16) + bleed on target (stronger if target
/// tagged) + self-bleed (shorter if self tagged).
/// Simplification: bleed applied unconditionally at base strength.
pub fn blood_oath() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("blood_oath"),
        vec![
            EffectNode::damage(16.0),
            EffectNode::apply_status("bleed", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Stun Skill — reduced damage + stun.
///
/// DDGC reference: -50% damage (avg 16) + guaranteed stun on hit.
pub fn stun_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("stun_skill"),
        vec![
            EffectNode::damage(16.0),
            EffectNode::apply_status("stun", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Regression — ranged strike on rearmost enemy.
///
/// DDGC reference: -67% damage (avg 10) + damage doubles if Tank is self-tagged.
/// Game-gap: conditional damage doubling not modeled.
pub fn regression() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("regression"),
        vec![EffectNode::damage(10.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 7 Tank base skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        protect_skill(),
        attack_reduce(),
        taunt_skill(),
        active_riposte(),
        blood_oath(),
        stun_skill(),
        regression(),
    ]
}
