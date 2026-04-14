//! Black (+2) variant hero content for all DDGC recruitable hero class families.
//!
//! Black variants activate when chaos stored value < 50 (negative chaos).
//! They share the same base stats as their base counterparts but have
//! variant-specific skill effect chains. See HERO_CLASS_FAMILIES.md for
//! the full variant mapping and skill difference table.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

// ── Alchemist Black (+2) ────────────────────────────────────────────────────

/// Alchemist black variant archetype — same stats as base, black variant identity.
///
/// DDGC reference: identical weapon/armour stats to base. Black variant differences
/// are in skill effects only (stress healing on multi-heal, stress receive buffs).
pub fn alchemist_archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Alchemist (Black)"),
        side: CombatSide::Ally,
        health: 139.0,
        max_health: 139.0,
        attack: 26.0,
        defense: 0.0,
        speed: 5.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.02,
        accuracy: 0.95,
        dodge: 0.00,
    }
}

/// Black heal_multi — same HP heal + stress heal.
///
/// DDGC reference: heals 9–11 HP (same as base) + "Heal Stress 2/2/3/3/4" effect.
/// Stress heal component approximated as additional minor heal.
pub fn alchemist_heal_multi() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_alchemist_heal_multi"),
        vec![EffectNode::heal(10.0), EffectNode::heal(2.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Black heal_single — same heal + dodge buff.
///
/// DDGC reference: same heal values + "Target Dodge 2" series effect.
/// Game-gap: dodge buff not modeled — approximated as minor heal.
pub fn alchemist_heal_single() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_alchemist_heal_single"),
        vec![EffectNode::heal(28.0), EffectNode::heal(2.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Black miss_single — dodge + speed + crit buff.
///
/// DDGC reference: "Target Dodge 2" + "Target Speed 1-5" + "Target Crit Chance 1-5".
/// Game-gap: buff effects not modeled — improved heal placeholder.
pub fn alchemist_miss_single() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_alchemist_miss_single"),
        vec![EffectNode::heal(3.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Black stress_multi — stress heal + dodge buff.
///
/// DDGC reference: "Heal Stress 2/2/3/3/4" + "Target Dodge 1" series.
/// Game-gap: dodge buff not modeled — slightly improved heal.
pub fn alchemist_stress_multi() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_alchemist_stress_multi"),
        vec![EffectNode::heal(5.0), EffectNode::heal(1.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Black burn_skill — damage + burn + self-dodge buff.
///
/// DDGC reference: same damage + burn + adds "Self Dodge 1-5" effect.
/// Game-gap: self-dodge buff approximated as minor self-heal.
pub fn alchemist_burn_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_alchemist_burn_skill"),
        vec![
            EffectNode::damage(26.0),
            EffectNode::apply_status("burn", Some(3)),
            EffectNode::heal(2.0),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black push_skill — reduced damage + push + damage receive debuff.
///
/// DDGC reference: push + "Damage Receive Percent 1-5" effect on target.
/// Game-gap: push and damage receive debuff not directly modeled.
/// Approximated as tagged status (damage vulnerability marker).
pub fn alchemist_push_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_alchemist_push_skill"),
        vec![
            EffectNode::damage(9.0),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black push_self — self-reposition + stress heal receive buff.
///
/// DDGC reference: moves forward + "Stress Heal Receive Percent 1-5" effect.
/// Game-gap: movement not modeled — stress heal receive approximated as heal.
pub fn alchemist_push_self() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_alchemist_push_self"),
        vec![EffectNode::heal(3.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// All 7 Alchemist black variant skills.
pub fn alchemist_skill_pack() -> Vec<SkillDefinition> {
    vec![
        alchemist_heal_multi(),
        alchemist_heal_single(),
        alchemist_miss_single(),
        alchemist_stress_multi(),
        alchemist_burn_skill(),
        alchemist_push_skill(),
        alchemist_push_self(),
    ]
}

// ── Diviner Black (+2) ──────────────────────────────────────────────────────

/// Diviner black variant archetype — same stats as base, black variant identity.
///
/// DDGC reference: identical stats to base. Black variant differences are
/// in skill effects (anti-divination effects, debuff removal on evasion).
pub fn diviner_archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Diviner (Black)"),
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

/// Black duality_fate — damage + anti-divination.
///
/// DDGC reference: adds "Ask God Not Trigger Divination" effect — prevents
/// divine oracle from triggering on this target. Approximated as tagged status
/// (anti-divine marker).
pub fn diviner_duality_fate() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_diviner_duality_fate"),
        vec![
            EffectNode::damage(9.0),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black repel — reduced damage + push + anti-divination.
///
/// DDGC reference: push + "Ask God Not Trigger Divination" effect.
/// Approximated as tagged status (anti-divine marker).
pub fn diviner_repel() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_diviner_repel"),
        vec![
            EffectNode::damage(12.0),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black blessed_evasion — damage + pull + divine debuff removal.
///
/// DDGC reference: pull + "Divine Bad Remove Random Debuff" — removes a random
/// debuff from the target via divine. Approximated as tagged status.
pub fn diviner_blessed_evasion() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_diviner_blessed_evasion"),
        vec![
            EffectNode::damage(18.0),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black pull_skill — pull + anti-divination + stun.
///
/// DDGC reference: pull + "Ask God Not Trigger Divination" + "Divine Result
/// Clear Corpses" + Stun 1. Anti-divine and corpse clear approximated as tagged;
/// stun modeled directly.
pub fn diviner_pull_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_diviner_pull_skill"),
        vec![
            EffectNode::damage(9.0),
            EffectNode::apply_status("tagged", Some(2)),
            EffectNode::apply_status("stun", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black survive — heal + ACC buff + stress heal.
///
/// DDGC reference: heal 0–9 HP + ACC 1-5 + "Strong Heal Stress Range".
/// Game-gap: ACC buff not modeled — heal + stress heal approximated.
pub fn diviner_survive() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_diviner_survive"),
        vec![EffectNode::heal(4.0), EffectNode::heal(2.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Black draw_stick — anti-divination oracle.
///
/// DDGC reference: "Ask God Twice Consume Bad Count Result Must Lucky" —
/// consumes bad divination count for guaranteed lucky result. Better oracle
/// outcome than base. Approximated as slightly better heal.
pub fn diviner_draw_stick() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_diviner_draw_stick"),
        vec![EffectNode::heal(2.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Black karmic_cycle — damage + divine clearing.
///
/// DDGC reference: "Divine Extra Multi Damage" + "Divine Clear Lucky Bad Count" —
/// deals extra damage per divine stack and clears accumulated counts.
/// Approximated as damage + tagged (divine tracker).
pub fn diviner_karmic_cycle() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_diviner_karmic_cycle"),
        vec![
            EffectNode::damage(18.0),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 7 Diviner black variant skills.
pub fn diviner_skill_pack() -> Vec<SkillDefinition> {
    vec![
        diviner_duality_fate(),
        diviner_repel(),
        diviner_blessed_evasion(),
        diviner_pull_skill(),
        diviner_survive(),
        diviner_draw_stick(),
        diviner_karmic_cycle(),
    ]
}

// ── Hunter Black (+2) ───────────────────────────────────────────────────────

/// Hunter black variant archetype — same stats as base, black variant identity.
///
/// DDGC reference: identical stats to base. Black variant differences are
/// in skill effects (mark queue disruption, marked-dependent effects).
pub fn hunter_archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Hunter (Black)"),
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

/// Black mark_skill — mark with queue disruption.
///
/// DDGC reference: "Hunter Mark Queue False 2" — mark does not queue (refresh
/// is blocked) + "Hunter Mark 2/2/2/3/3". Mark queue false changes mark behavior
/// from queueing to overwriting. Approximated as standard tagged status.
pub fn hunter_mark_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_hunter_mark_skill"),
        vec![
            EffectNode::damage(4.0),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black pull_skill — pull + marked move chance.
///
/// DDGC reference: pull + "Marked Move Chance 1-5" — pull has chance to move
/// marked targets further. Approximated as tagged status.
pub fn hunter_pull_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_hunter_pull_skill"),
        vec![
            EffectNode::damage(20.0),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black aoe_skill — AoE + dodge debuff + marked damage.
///
/// DDGC reference: "D2 Target Dodge -2" series + "Marked Damage +0.5" — AoE
/// applies dodge debuff and extra damage to marked targets.
/// Approximated as tagged status (dodge debuff + marked damage marker).
pub fn hunter_aoe_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_hunter_aoe_skill"),
        vec![
            EffectNode::damage(20.0),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black stun_skill — stun + marked stun chance.
///
/// DDGC reference: stun + "Marked Stun Chance 1-5" — stun has bonus chance
/// against marked targets. Approximated as stun + tagged (marked interaction).
pub fn hunter_stun_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_hunter_stun_skill"),
        vec![
            EffectNode::damage(20.0),
            EffectNode::apply_status("stun", Some(1)),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black ignore_def_skill — ignore defense on marked + marked damage.
///
/// DDGC reference: "Marked Ignore Def" + "Marked Damage +1" — ignores defense
/// on marked targets and deals extra damage. Approximated as damage + tagged.
pub fn hunter_ignore_def_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_hunter_ignore_def_skill"),
        vec![
            EffectNode::damage(40.0),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black bleed_skill — bleed with marked multiplier.
///
/// DDGC reference: "Marked Bleed 150 1-5" — bleed is 150% effective on marked
/// targets. Approximated as stronger bleed (longer duration to represent
/// the 150% multiplier).
pub fn hunter_bleed_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_hunter_bleed_skill"),
        vec![
            EffectNode::damage(20.0),
            EffectNode::apply_status("bleed", Some(4)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black buff_skill — ACC buff + stress damage reduction.
///
/// DDGC reference: ACC 1-5 + "Stress Damage Receive Percent" (reduces incoming
/// stress damage). Game-gap: buff and stress damage reduction not modeled —
/// improved heal placeholder.
pub fn hunter_buff_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_hunter_buff_skill"),
        vec![EffectNode::heal(3.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// All 7 Hunter black variant skills.
pub fn hunter_skill_pack() -> Vec<SkillDefinition> {
    vec![
        hunter_mark_skill(),
        hunter_pull_skill(),
        hunter_aoe_skill(),
        hunter_stun_skill(),
        hunter_ignore_def_skill(),
        hunter_bleed_skill(),
        hunter_buff_skill(),
    ]
}

// ── Shaman Black (+2) ───────────────────────────────────────────────────────

/// Shaman black variant archetype — same stats as base, black variant identity.
///
/// DDGC reference: identical stats to base. Black variant differences are
/// in skill effects (reduced DoT damage penalty -37.5% vs base -75%, self-weak
/// effects on direct_hit_2, stun without DoT).
pub fn shaman_archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Shaman (Black)"),
        side: CombatSide::Ally,
        health: 135.0,
        max_health: 135.0,
        attack: 39.0,
        defense: 0.0,
        speed: 5.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.03,
        accuracy: 0.95,
        dodge: 0.00,
    }
}

/// Black frozen_skill — higher damage + frozen DoT.
///
/// DDGC reference: magic_dmg -37.5% (avg 24) + frozen 1-5. Base has -75% (avg 10).
/// Black variant deals 2.4x more damage on DoT skills than base.
pub fn shaman_frozen_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_shaman_frozen_skill"),
        vec![
            EffectNode::damage(24.0),
            EffectNode::apply_status("frozen", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black burn_skill — higher damage + burn DoT.
///
/// DDGC reference: magic_dmg -37.5% (avg 24) + burn 1-5. Base has -75% (avg 10).
pub fn shaman_burn_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_shaman_burn_skill"),
        vec![
            EffectNode::damage(24.0),
            EffectNode::apply_status("burn", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black bleed_skill — higher damage + bleed DoT.
///
/// DDGC reference: magic_dmg -37.5% (avg 24) + bleed 1-5. Base has -75% (avg 10).
pub fn shaman_bleed_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_shaman_bleed_skill"),
        vec![
            EffectNode::damage(24.0),
            EffectNode::apply_status("bleed", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black direct_hit_1 — full-power magic damage, ignores defense.
///
/// DDGC reference: magic_dmg 0% (avg 39) + is_ignore_def. Same damage as base
/// but with defense-piercing property. Game-gap: ignore_def not modeled.
pub fn shaman_direct_hit_1() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_shaman_direct_hit_1"),
        vec![EffectNode::damage(39.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black direct_hit_2 — heavy magic damage + self-weak effects.
///
/// DDGC reference: magic_dmg +50% (avg 59) + "Weak Dodge 1-5" + "Weak Self
/// Speed 1-5". Base has 0% mod (avg 39). The self-weak effects reduce dodge
/// and speed. Game-gap: self-weak not modeled — tagged as side-effect marker.
pub fn shaman_direct_hit_2() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_shaman_direct_hit_2"),
        vec![
            EffectNode::damage(59.0),
            EffectNode::apply_status("tagged", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black stun_skill — higher damage + stun.
///
/// DDGC reference: magic_dmg -33.5% (avg 26) + stun. Base has -67% (avg 13).
/// Black variant does double the base stun skill damage.
pub fn shaman_stun_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_shaman_stun_skill"),
        vec![
            EffectNode::damage(26.0),
            EffectNode::apply_status("stun", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black buff_self — accuracy + crit + damage percent buff.
///
/// DDGC reference: ACC 1-5 + Crit Chance 1-5 + "Damage Percent 1" (constant).
/// Game-gap: buff not modeled — improved heal placeholder.
pub fn shaman_buff_self() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_shaman_buff_self"),
        vec![EffectNode::heal(2.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// All 7 Shaman black variant skills.
pub fn shaman_skill_pack() -> Vec<SkillDefinition> {
    vec![
        shaman_frozen_skill(),
        shaman_burn_skill(),
        shaman_bleed_skill(),
        shaman_direct_hit_1(),
        shaman_direct_hit_2(),
        shaman_stun_skill(),
        shaman_buff_self(),
    ]
}

// ── Tank Black (+2) ─────────────────────────────────────────────────────────

/// Tank black variant archetype — same stats as base, black variant identity.
///
/// DDGC reference: identical stats to base. Black variant differences are
/// in skill effects (remove DoT on guard/stun, self-mark mechanics, conditional
/// damage, self-bleed on blood oath).
pub fn tank_archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Tank (Black)"),
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

/// Black protect_skill — guard + damage reduction + remove DoT.
///
/// DDGC reference: Guard 3 + "Tank Damage Receive Percent 1-5" + "Remove Random
/// Dot" — guarding also reduces ally damage and removes a DoT. Damage receive
/// and DoT removal approximated as tagged status.
pub fn tank_protect_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_tank_protect_skill"),
        vec![
            EffectNode::apply_status("guard", Some(3)),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Black attack_reduce — damage + self-mark + damage buff + dodge.
///
/// DDGC reference: "Mark performer" + "Tank Damage Percent 1-5" + "Tank Dodge
/// 1-5" — self-mark enables damage/dodge bonuses. Game-gap: self-mark and
/// buffs not modeled — approximated as damage + tagged.
pub fn tank_attack_reduce() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_tank_attack_reduce"),
        vec![
            EffectNode::damage(16.0),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black taunt_skill — full damage + self-mark + PROT + mark target.
///
/// DDGC reference: "Mark performer" + "Tank Self PROT 6-10" + "Mark Target" —
/// taunting also marks self for PROT bonus and marks the target.
/// Approximated as damage + tagged (combined self-mark/target-mark).
pub fn tank_taunt_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_tank_taunt_skill"),
        vec![
            EffectNode::damage(31.0),
            EffectNode::apply_status("tagged", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black active_riposte — reduced damage + mark target + riposte + self-mark + PROT.
///
/// DDGC reference: "Mark Target" + "Tank Riposte 1" + "Mark performer" + "Tank
/// Self PROT 1-5". Game-gap: riposte and PROT buff not modeled — damage +
/// tagged for mark effects.
pub fn tank_active_riposte() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_tank_active_riposte"),
        vec![
            EffectNode::damage(16.0),
            EffectNode::apply_status("tagged", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black blood_oath — damage + self-conditional bleed + target conditional bleed.
///
/// DDGC reference: "Tank Self Tag Condi Bleed 1" + "Tank Tag Condi Bleed 1-5" —
/// both self and target receive conditional bleed effects. Self-conditional
/// bleed approximated as tagged status alongside target bleed.
pub fn tank_blood_oath() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_tank_blood_oath"),
        vec![
            EffectNode::damage(16.0),
            EffectNode::apply_status("bleed", Some(3)),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black stun_skill — reduced damage + stun + remove DoT.
///
/// DDGC reference: stun + "Marked Self Remove Random Dot" — stunning removes
/// a random DoT from the Tank if marked. Approximated as stun + tagged
/// (DoT removal marker).
pub fn tank_stun_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_tank_stun_skill"),
        vec![
            EffectNode::damage(16.0),
            EffectNode::apply_status("stun", Some(1)),
            EffectNode::apply_status("tagged", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Black regression — small damage on marked target + self stress heal.
///
/// DDGC reference: "SmallDmg Marked Target" + "HealSelfStress 1/1/2/2/3" —
/// deals small damage to marked targets and heals self stress. Stress heal
/// approximated as HP heal.
pub fn tank_regression() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("b_tank_regression"),
        vec![EffectNode::damage(10.0), EffectNode::heal(2.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 7 Tank black variant skills.
pub fn tank_skill_pack() -> Vec<SkillDefinition> {
    vec![
        tank_protect_skill(),
        tank_attack_reduce(),
        tank_taunt_skill(),
        tank_active_riposte(),
        tank_blood_oath(),
        tank_stun_skill(),
        tank_regression(),
    ]
}
