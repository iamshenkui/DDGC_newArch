//! Rotvine Wraith — XuanWu boss (summon-fruit + burn-mark-control).
//!
//! DDGC reference: Eldritch-type boss from the XuanWu dungeon.
//! Tier 1 base stats: HP 150, DEF 10%, PROT 0.6, SPD 5, 2 turns/round.
//! Skills: cadaver_bloom, rotvine_snare, sepsis_strangulate,
//! telluric_resurrect, carrion_sowing, move.
//!
//! The Rotvine Wraith is a summon-control boss that pressures heroes with
//! burn and bleed DoT, marks targets for increased damage, and continuously
//! re-summons rotten_fruit minions that either heal the boss (A) or explode
//! for AoE damage (B/C). Its AI prioritizes re-summoning dead fruit over
//! all other actions.
//!
//! Game-gaps:
//! - Summon mechanic (carrion_sowing) modeled as status marker only
//! - AI priority system (ally_dead_skill with 10M base_chance) not modeled
//! - Position-based targeting (launch 2-4, target 234/#1234/~1234/@#3)
//!   approximated as AllEnemies/AllAllies
//! - $1234 conditional targeting (marked heroes) approximated as AllEnemies
//! - Mark Target effect modeled as status marker only
//! - Two turns per round not modeled in Archetype
//! - PROT (0.6), MAGIC_PROT (0.5) not modeled in Archetype
//! - Stun Resist 70%, Poison Resist 100% (immune), Bleed Resist 100% (immune),
//!   Debuff Resist 40%, Move Resist 100% (immune) not modeled
//! - Burn Resist 25%, Frozen Resist 25% not modeled
//! - Self-heal + ally-buff in telluric_resurrect approximated as AllAllies
//! - Cooldown on sepsis_strangulate preserved (3 rounds)
//! - Self-movement on cadaver_bloom/rotvine_snare dropped as game-gap
//! - magic_dmg on cadaver_bloom not modeled (treated as normal damage)

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Rotvine Wraith base archetype — tier 1 boss stats from DDGC data.
///
/// HP 150, weapon damage derived from rotvine_snare skill (10–12 avg 11),
/// speed 5, defense 0.10 (10% dodge).
/// Summoner role: summons rotten_fruit minions and controls with burn/mark/bleed.
/// Crit 5% from cadaver_bloom/rotvine_snare.
/// PROT 0.6, MAGIC_PROT 0.5, Stun Resist 70%, Poison Resist 100% (immune),
/// Bleed Resist 100% (immune), Debuff Resist 40%, Move Resist 100% (immune),
/// Burn Resist 25%, Frozen Resist 25% — all not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Rotvine Wraith"),
        side: CombatSide::Enemy,
        health: 150.0,
        max_health: 150.0,
        attack: 11.0,
        defense: 0.10,
        speed: 5.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.05,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Rotvine Wraith Skills ────────────────────────────────────────────────────

/// Cadaver Bloom — ranged burn + mark.
///
/// DDGC reference: dmg 5–6 (magic), atk 80%, crit 5%,
/// launch ranks 3,4, target #1234 (any marked hero, AoE),
/// effects "Super Burn 1" + "Mark Target".
/// Game-gap: mark-targeting (#1234) approximated as AllEnemies.
/// Game-gap: magic damage type not modeled — treated as normal damage.
/// Game-gap: Mark Target mechanic modeled as status marker only.
pub fn cadaver_bloom() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("cadaver_bloom"),
        vec![
            EffectNode::damage(5.5),
            EffectNode::apply_status("burn", Some(1)),
            EffectNode::apply_status("mark", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Rotvine Snare — ranged stun + mark.
///
/// DDGC reference: dmg 10–12, atk 85%, crit 5%,
/// launch ranks 2,3,4, target 234 (ranks 2-4),
/// effects "Stun 1" + "Mark Target".
/// Game-gap: position-based targeting (ranks 2-4) approximated as AllEnemies.
/// Game-gap: Mark Target mechanic modeled as status marker only.
pub fn rotvine_snare() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("rotvine_snare"),
        vec![
            EffectNode::damage(11.0),
            EffectNode::apply_status("stun", Some(1)),
            EffectNode::apply_status("mark", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Sepsis Strangulate — AoE bleed + stress (3-round cooldown).
///
/// DDGC reference: dmg 5–6, atk 85%, crit 5%,
/// launch ranks 3,4, target ~1234 (AoE all ranks, prefers marked),
/// effects "Strong 100 Bleed 1" + "Stress 2".
/// Cooldown: 3 rounds.
/// Game-gap: $1234 conditional targeting (marked heroes) approximated as AllEnemies.
pub fn sepsis_strangulate() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("sepsis_strangulate"),
        vec![
            EffectNode::damage(5.5),
            EffectNode::apply_status("bleed", Some(1)),
            EffectNode::apply_status("stress", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        Some(3),
    )
}

/// Telluric Resurrect — heal ally + buff PROT.
///
/// DDGC reference: dmg 0–0, atk 100%, crit 0%,
/// launch ranks 3,4, target @#3 (friendly ally at rank 3),
/// effects "Heal Percent 10" + "Prot 2".
/// Game-gap: rank-based ally targeting (@#3) approximated as AllAllies.
/// Game-gap: Heal Percent 10 (heal 10% of max HP) modeled as status marker.
/// Game-gap: Prot 2 (+2 PROT) modeled as status marker.
pub fn telluric_resurrect() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("telluric_resurrect"),
        vec![
            EffectNode::apply_status("heal_ally", Some(10)),
            EffectNode::apply_status("prot", Some(2)),
        ],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Carrion Sowing — summon rotten_fruit.
///
/// DDGC reference: dmg 0–0, atk 100%, crit 0%,
/// launch rank 1, target ~@1 (friendly rank 1, AoE),
/// effect "Rotten Fruit Summon".
/// AI behavior: fires with overwhelming priority (10M base_chance)
/// whenever any rotten_fruit ally is dead.
/// Game-gap: summon mechanic modeled as status marker only.
/// Game-gap: AI ally_dead_skill priority not modeled.
pub fn carrion_sowing() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("carrion_sowing"),
        vec![EffectNode::apply_status("summon_rotten_fruit", Some(1))],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// Move — self-reposition skill.
///
/// DDGC reference: dmg 0–0, atk 0%, crit 0%,
/// launch ranks 1,2, target @23 (ally ranks 2-3), .move 2 0.
/// Game-gap: position-based movement approximated as push(2).
pub fn move_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("move"),
        vec![EffectNode::push(2)],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// All 6 Rotvine Wraith skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        cadaver_bloom(),
        rotvine_snare(),
        sepsis_strangulate(),
        telluric_resurrect(),
        carrion_sowing(),
        move_skill(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotvine_wraith_archetype_is_enemy_eldritch_summoner() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Rotvine Wraith");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 150.0);
        assert_eq!(arch.max_health, 150.0);
        assert_eq!(arch.speed, 5.0, "rotvine_wraith has SPD 5");
        assert_eq!(arch.defense, 0.10, "rotvine_wraith has 10% defense");
        assert_eq!(arch.attack, 11.0, "attack from rotvine_snare avg 10-12");
        assert_eq!(arch.crit_chance, 0.05, "crit 5% from cadaver_bloom/rotvine_snare");
    }

    #[test]
    fn rotvine_wraith_cadaver_bloom_applies_burn_and_mark() {
        let skill = cadaver_bloom();
        assert_eq!(skill.id.0, "cadaver_bloom");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "cadaver_bloom must deal damage");
        let has_burn = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("burn")
        });
        assert!(has_burn, "cadaver_bloom must apply burn status");
        let has_mark = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("mark")
        });
        assert!(has_mark, "cadaver_bloom must apply mark status");
    }

    #[test]
    fn rotvine_wraith_rotvine_snare_applies_stun_and_mark() {
        let skill = rotvine_snare();
        assert_eq!(skill.id.0, "rotvine_snare");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "rotvine_snare must deal damage");
        let has_stun = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stun")
        });
        assert!(has_stun, "rotvine_snare must apply stun status");
        let has_mark = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("mark")
        });
        assert!(has_mark, "rotvine_snare must apply mark status");
    }

    #[test]
    fn rotvine_wraith_sepsis_strangulate_has_cooldown() {
        let skill = sepsis_strangulate();
        assert_eq!(skill.id.0, "sepsis_strangulate");
        assert_eq!(skill.cooldown, Some(3), "sepsis_strangulate has 3-round cooldown");
        let has_bleed = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("bleed")
        });
        assert!(has_bleed, "sepsis_strangulate must apply bleed status");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "sepsis_strangulate must apply stress status");
    }

    #[test]
    fn rotvine_wraith_telluric_resurrect_heals_and_buffs() {
        let skill = telluric_resurrect();
        assert_eq!(skill.id.0, "telluric_resurrect");
        let has_heal = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("heal_ally")
        });
        assert!(has_heal, "telluric_resurrect must apply heal_ally status");
        let has_prot = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("prot")
        });
        assert!(has_prot, "telluric_resurrect must apply prot status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllAllies",
            "telluric_resurrect targets allies"
        );
    }

    #[test]
    fn rotvine_wraith_carrion_sowing_is_summon() {
        let skill = carrion_sowing();
        assert_eq!(skill.id.0, "carrion_sowing");
        let has_summon = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("summon_rotten_fruit")
        });
        assert!(has_summon, "carrion_sowing must apply summon_rotten_fruit status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "carrion_sowing targets self (summon for own team)"
        );
    }

    #[test]
    fn rotvine_wraith_skill_pack_has_six_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 6);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"cadaver_bloom"), "missing cadaver_bloom");
        assert!(ids.contains(&"rotvine_snare"), "missing rotvine_snare");
        assert!(ids.contains(&"sepsis_strangulate"), "missing sepsis_strangulate");
        assert!(ids.contains(&"telluric_resurrect"), "missing telluric_resurrect");
        assert!(ids.contains(&"carrion_sowing"), "missing carrion_sowing");
        assert!(ids.contains(&"move"), "missing move");
    }

    #[test]
    fn rotvine_wraith_summon_plus_burn_plus_mark_identity() {
        // The core identity of rotvine_wraith is a summon-control boss that
        // pressures with burn, mark, bleed, and stun while re-summoning
        // rotten_fruit minions that heal the boss or explode.
        let pack = skill_pack();

        let has_summon = pack.iter().any(|s| {
            s.id.0 == "carrion_sowing"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("summon_rotten_fruit")
                })
        });

        let has_burn = pack.iter().any(|s| {
            s.id.0 == "cadaver_bloom"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("burn")
                })
        });

        let has_mark = pack.iter().any(|s| {
            s.effects.iter().any(|e| {
                matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                    && e.status_kind.as_deref() == Some("mark")
            })
        });

        assert!(has_summon, "rotvine_wraith must have summon skill");
        assert!(has_burn, "rotvine_wraith must have burn skill");
        assert!(has_mark, "rotvine_wraith must have mark skill");
    }
}