//! Shaman hero — magic DPS class family base variant.
//!
//! DDGC reference: pure magic damage dealer with DoT variety (frozen, burn, bleed),
//! two direct-hit spells, stun, and self-buff. Lowest HP pool.
//! Base magic damage at level 0: 32–45, averaged to 39.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Shaman base archetype — level 0 stats from DDGC data.
///
/// HP 135, magic damage 32–45 (avg 39), speed 5, dodge 0%, crit 3%.
/// Lowest HP pool — glass cannon magic DPS.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Shaman"),
        side: CombatSide::Ally,
        health: 135.0,
        max_health: 135.0,
        attack: 39.0,
        defense: 0.0,
        speed: 5.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.03,
        dodge: 0.00,
    }
}

// ── Shaman Skills ──────────────────────────────────────────────────────────

/// Frozen Skill — magic damage + frozen DoT.
///
/// DDGC reference: -75% magic damage (avg 10) + frozen DoT 12/round for 3 rounds.
pub fn frozen_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("frozen_skill"),
        vec![
            EffectNode::damage(10.0),
            EffectNode::apply_status("frozen", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Burn Skill — AoE magic damage + burn DoT.
///
/// DDGC reference: -75% magic damage (avg 10) to all enemies + burn DoT 12/round
/// for 3 rounds.
pub fn burn_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("burn_skill"),
        vec![
            EffectNode::damage(10.0),
            EffectNode::apply_status("burn", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Bleed Skill — magic damage + bleed DoT.
///
/// DDGC reference: -75% magic damage (avg 10) + bleed DoT 12/round for 3 rounds.
pub fn bleed_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("bleed_skill"),
        vec![
            EffectNode::damage(10.0),
            EffectNode::apply_status("bleed", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Direct Hit 1 — full-power magic damage.
///
/// DDGC reference: 0% damage mod (full magic damage, avg 39), no additional effects.
/// Shaman's highest-damage single-target attack.
pub fn direct_hit_1() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("direct_hit_1"),
        vec![EffectNode::damage(39.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Direct Hit 2 — full magic damage + self-buff.
///
/// DDGC reference: 0% damage mod (full magic damage, avg 39) + dodge +5% and
/// speed +5% self-buff for 1 round.
/// Game-gap: self-buff not modeled — damage only.
pub fn direct_hit_2() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("direct_hit_2"),
        vec![EffectNode::damage(39.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Stun Skill — reduced magic damage + stun.
///
/// DDGC reference: -67% magic damage (avg 13) + guaranteed stun on hit.
/// Game-gap: rank 2 launch restriction not modeled.
pub fn stun_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("stun_skill"),
        vec![
            EffectNode::damage(13.0),
            EffectNode::apply_status("stun", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Buff Self — accuracy + crit self-buff.
///
/// DDGC reference: ACC +10% and crit chance +5% self-buff for 1 round.
/// Game-gap: buff not modeled — minimal heal placeholder.
pub fn buff_self() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("buff_self"),
        vec![EffectNode::heal(1.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// All 7 Shaman base skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        frozen_skill(),
        burn_skill(),
        bleed_skill(),
        direct_hit_1(),
        direct_hit_2(),
        stun_skill(),
        buff_self(),
    ]
}
