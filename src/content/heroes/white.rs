//! White (+1) variant hero content for all DDGC recruitable hero class families.
//!
//! White variants activate when chaos stored value >= 150 (positive chaos).
//! They share the same base stats as their base counterparts but have
//! variant-specific skill effect chains. See HERO_CLASS_FAMILIES.md for
//! the full variant mapping and skill difference table.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

// ── Alchemist White (+1) ────────────────────────────────────────────────────

/// Alchemist white variant archetype — same stats as base, white variant identity.
///
/// DDGC reference: identical weapon/armour stats to base. White variant differences
/// are in skill effects only (stronger heals, self-sustain on burn/push_self).
pub fn alchemist_archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Alchemist (White)"),
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

/// White heal_multi — significantly higher heal values than base.
///
/// DDGC reference: heals 12–24 HP (vs base 9–11), averaged to 18.
pub fn alchemist_heal_multi() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_alchemist_heal_multi"),
        vec![EffectNode::heal(18.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// White heal_single — same heal + stress damage receive effect.
///
/// DDGC reference: same heal values + "Stress Damage Receive Percent" effect.
/// Approximated as additional minor heal for the stress component.
pub fn alchemist_heal_single() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_alchemist_heal_single"),
        vec![EffectNode::heal(28.0), EffectNode::heal(5.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// White miss_single — dodge/speed buff + strong stress damage receive.
///
/// DDGC reference: adds "Strong Stress Damage Receive Percent" effect.
/// Game-gap: buff effects not modeled — slightly improved heal placeholder.
pub fn alchemist_miss_single() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_alchemist_miss_single"),
        vec![EffectNode::heal(2.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// White stress_multi — upgraded "Strong" stress heal variant.
///
/// DDGC reference: "Strong Heal Stress" (vs base "Heal Stress"), higher values.
pub fn alchemist_stress_multi() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_alchemist_stress_multi"),
        vec![EffectNode::heal(7.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// White burn_skill — damage + burn + self-heal.
///
/// DDGC reference: same damage + burn + adds "Heal Self Range" (5–7 HP).
pub fn alchemist_burn_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_alchemist_burn_skill"),
        vec![
            EffectNode::damage(26.0),
            EffectNode::apply_status("burn", Some(3)),
            EffectNode::heal(5.0),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White push_skill — reduced damage + push + dodge debuff.
///
/// DDGC reference: push + "Target Dodge" debuff on hit.
/// Game-gap: dodge debuff approximated as tagged status.
pub fn alchemist_push_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_alchemist_push_skill"),
        vec![
            EffectNode::damage(9.0),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White push_self — self-repositioning + self-heal.
///
/// DDGC reference: moves forward + heals 12–16 HP (base has no heal).
/// Game-gap: movement not modeled — heal only.
pub fn alchemist_push_self() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_alchemist_push_self"),
        vec![EffectNode::heal(12.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// All 7 Alchemist white variant skills.
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

// ── Diviner White (+1) ──────────────────────────────────────────────────────

/// Diviner white variant archetype — same stats as base, white variant identity.
///
/// DDGC reference: identical stats to base. White variant differences are
/// in skill effects (divine bad count generation, karma engine upgrades).
pub fn diviner_archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Diviner (White)"),
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

/// White duality_fate — damage + divine bad count generation.
///
/// DDGC reference: adds "Divine Bad Count 1" effect.
/// Approximated as tagged status (divine tracker).
pub fn diviner_duality_fate() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_diviner_duality_fate"),
        vec![
            EffectNode::damage(9.0),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White repel — reduced damage + push + divine bad count.
///
/// DDGC reference: push + "Divine Bad Count 1" effect.
pub fn diviner_repel() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_diviner_repel"),
        vec![
            EffectNode::damage(12.0),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White blessed_evasion — reduced damage + pull + divine lucky bleed.
///
/// DDGC reference: adds "Divine Lucky Bleed" effect on hit.
pub fn diviner_blessed_evasion() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_diviner_blessed_evasion"),
        vec![
            EffectNode::damage(18.0),
            EffectNode::apply_status("bleed", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White pull_skill — pull + divine result clear corpses.
///
/// DDGC reference: expanded targeting (any rank) + "Divine Result Clear Corpses".
/// Game-gap: expanded targeting and corpse clearing not modeled.
pub fn diviner_pull_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_diviner_pull_skill"),
        vec![
            EffectNode::damage(9.0),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White survive — self-reposition + HP heal + stress heal.
///
/// DDGC reference: adds heal (2–5 HP) + "Heal Stress" (2–6 stress).
/// Game-gap: movement not modeled — heal + stress heal approximated.
pub fn diviner_survive() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_diviner_survive"),
        vec![EffectNode::heal(3.0), EffectNode::heal(3.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// White draw_stick — divine oracle with regain.
///
/// DDGC reference: "Ask God Consume Bad Count Result Must Lucky Regain" —
/// resources are refunded/regained on use. Approximated as slightly better heal.
pub fn diviner_draw_stick() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_diviner_draw_stick"),
        vec![EffectNode::heal(2.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// White karmic_cycle — attack with leveled divine clearing.
///
/// DDGC reference: "Divine Clear Lucky Bad Count 1/2/3/4/5" (leveled per upgrade).
/// Approximated as damage + tagged (divine tracker).
pub fn diviner_karmic_cycle() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_diviner_karmic_cycle"),
        vec![
            EffectNode::damage(18.0),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 7 Diviner white variant skills.
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

// ── Hunter White (+1) ───────────────────────────────────────────────────────

/// Hunter white variant archetype — same stats as base, white variant identity.
///
/// DDGC reference: identical stats to base. White variant differences are
/// in skill effects (mark-refresh loop, minor marks on AoE/pull).
pub fn hunter_archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Hunter (White)"),
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

/// White mark_skill — tag enemy + anyone mark.
///
/// DDGC reference: adds "Hunter Anyone Mark" effect — marks apply to any target
/// instead of just the primary. Approximated as longer tagged duration.
pub fn hunter_mark_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_hunter_mark_skill"),
        vec![
            EffectNode::damage(4.0),
            EffectNode::apply_status("tagged", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White pull_skill — pull + minor mark.
///
/// DDGC reference: adds "Hunter Minor Mark 1-5" — pull also marks the target.
pub fn hunter_pull_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_hunter_pull_skill"),
        vec![
            EffectNode::damage(20.0),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White aoe_skill — AoE attack + minor mark on all targets.
///
/// DDGC reference: adds "Hunter Minor Mark 1-5" — AoE also applies minor marks.
pub fn hunter_aoe_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_hunter_aoe_skill"),
        vec![
            EffectNode::damage(20.0),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White stun_skill — stun + refresh marked target's mark.
///
/// DDGC reference: adds "Marked Refresh Marked" — stunning a marked target
/// refreshes the mark duration. Approximated as tagged alongside stun.
pub fn hunter_stun_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_hunter_stun_skill"),
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

/// White ignore_def_skill — defense-ignoring attack + refresh mark.
///
/// DDGC reference: adds "Marked Refresh Marked 1-5" — hitting marked target
/// refreshes the mark. Approximated as tagged effect.
pub fn hunter_ignore_def_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_hunter_ignore_def_skill"),
        vec![
            EffectNode::damage(40.0),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White bleed_skill — bleed + refresh mark.
///
/// DDGC reference: adds "Marked Refresh Marked" — bleed attacks also refresh marks.
pub fn hunter_bleed_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_hunter_bleed_skill"),
        vec![
            EffectNode::damage(20.0),
            EffectNode::apply_status("bleed", Some(3)),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White buff_skill — self-ACC buff + heal (can now crit).
///
/// DDGC reference: is_crit_valid changes from False to True, adds heal 15–37 HP.
/// Game-gap: crit validity not modeled — heal added instead of placeholder.
pub fn hunter_buff_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_hunter_buff_skill"),
        vec![EffectNode::heal(20.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// All 7 Hunter white variant skills.
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

// ── Shaman White (+1) ───────────────────────────────────────────────────────

/// Shaman white variant archetype — same stats as base, white variant identity.
///
/// DDGC reference: identical stats to base. White variant differences are
/// in skill effects (cross-DoT chaos, self-heal on direct hits).
pub fn shaman_archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Shaman (White)"),
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

/// White frozen_skill — magic damage + frozen + burn (cross-DoT).
///
/// DDGC reference: adds "Ch75 Burn 1-5" — 75% chance to also apply burn.
/// Approximated as guaranteed burn application (chance not modeled).
pub fn shaman_frozen_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_shaman_frozen_skill"),
        vec![
            EffectNode::damage(10.0),
            EffectNode::apply_status("frozen", Some(3)),
            EffectNode::apply_status("burn", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White burn_skill — magic damage + burn + bleed (cross-DoT).
///
/// DDGC reference: adds "Ch75 Bleed 1-5" — 75% chance to also apply bleed.
/// Approximated as guaranteed bleed application.
pub fn shaman_burn_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_shaman_burn_skill"),
        vec![
            EffectNode::damage(10.0),
            EffectNode::apply_status("burn", Some(3)),
            EffectNode::apply_status("bleed", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White bleed_skill — magic damage + bleed + frozen (cross-DoT).
///
/// DDGC reference: adds "Ch75 Frozen 1-5" — 75% chance to also apply freeze.
/// Approximated as guaranteed frozen application.
pub fn shaman_bleed_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_shaman_bleed_skill"),
        vec![
            EffectNode::damage(10.0),
            EffectNode::apply_status("bleed", Some(3)),
            EffectNode::apply_status("frozen", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White direct_hit_1 — full-power magic damage + self-heal.
///
/// DDGC reference: adds "Heal Self Range 1-5" — direct hits now heal the Shaman.
pub fn shaman_direct_hit_1() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_shaman_direct_hit_1"),
        vec![EffectNode::damage(39.0), EffectNode::heal(5.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White direct_hit_2 — full magic damage + self-heal.
///
/// DDGC reference: adds "Heal Self Range 1-5" alongside existing debuffs.
pub fn shaman_direct_hit_2() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_shaman_direct_hit_2"),
        vec![EffectNode::damage(39.0), EffectNode::heal(5.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White stun_skill — reduced magic damage + stun + frozen.
///
/// DDGC reference: adds "D1 Frozen 1-5" — stun now also applies a 1-turn frozen DoT.
pub fn shaman_stun_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_shaman_stun_skill"),
        vec![
            EffectNode::damage(13.0),
            EffectNode::apply_status("stun", Some(1)),
            EffectNode::apply_status("frozen", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White buff_self — accuracy + crit + weak dodge.
///
/// DDGC reference: adds "Weak Dodge 6" — self-buff now also applies dodge effect.
/// Game-gap: dodge buff not modeled — improved heal placeholder.
pub fn shaman_buff_self() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_shaman_buff_self"),
        vec![EffectNode::heal(2.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// All 7 Shaman white variant skills.
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

// ── Tank White (+1) ─────────────────────────────────────────────────────────

/// Tank white variant archetype — same stats as base, white variant identity.
///
/// DDGC reference: identical stats to base. White variant differences are
/// in skill effects (aggressive guardian: crit on taunt/stun, ignore def on
/// riposte, full damage on regression).
pub fn tank_archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Tank (White)"),
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

/// White protect_skill — guard + tank damage bonus.
///
/// DDGC reference: adds "Tank Damage 1-5" — Tank deals bonus damage while guarding.
/// Approximated as self-tagged (damage bonus marker).
pub fn tank_protect_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_tank_protect_skill"),
        vec![
            EffectNode::apply_status("guard", Some(3)),
            EffectNode::apply_status("tagged", Some(2)),
        ],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// White attack_reduce — damage + self-damage buff.
///
/// DDGC reference: adds "Tank Self Damage Percent 1-5" — this skill also buffs
/// the Tank's own damage output. Approximated as higher damage value.
pub fn tank_attack_reduce() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_tank_attack_reduce"),
        vec![EffectNode::damage(20.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White taunt_skill — full damage + crit chance bonus.
///
/// DDGC reference: adds "Crit Chance 1-5" — taunting also grants crit chance.
/// Approximated as higher damage value (crit bonus → more damage).
pub fn tank_taunt_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_tank_taunt_skill"),
        vec![EffectNode::damage(35.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White active_riposte — reduced damage + ignore defense + mark enemy.
///
/// DDGC reference: adds "is_ignore_def True" — riposte setup attack ignores defense.
/// Approximated as higher damage (ignore defense = more effective damage).
pub fn tank_active_riposte() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_tank_active_riposte"),
        vec![
            EffectNode::damage(20.0),
            EffectNode::apply_status("tagged", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White blood_oath — damage + stronger bleed.
///
/// DDGC reference: uses "Tank Tag Condi Bleed 6-10" (vs base 1-5) — stronger
/// bleed application. Approximated as longer bleed duration.
pub fn tank_blood_oath() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_tank_blood_oath"),
        vec![
            EffectNode::damage(16.0),
            EffectNode::apply_status("bleed", Some(4)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White stun_skill — reduced damage + stun + crit bonus.
///
/// DDGC reference: adds "Crit Chance 1-5" + "Tank Marked Stun Chance 1-5" —
/// stun attacks benefit from crit and bonus stun on marked targets.
/// Approximated as higher damage value.
pub fn tank_stun_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_tank_stun_skill"),
        vec![
            EffectNode::damage(18.0),
            EffectNode::apply_status("stun", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// White regression — full damage against marked target.
///
/// DDGC reference: effect changes from "SmallDmg Marked Target" to
/// "Damage Marked Target" — upgraded to full damage (2x the base amount).
pub fn tank_regression() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("w_tank_regression"),
        vec![EffectNode::damage(20.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 7 Tank white variant skills.
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
