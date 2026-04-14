//! Gambler — ZhuQue boss (summon-mahjong + random-debuff + stress-pressure).
//!
//! DDGC reference: Eldritch-type boss from the ZhuQue dungeon.
//! Tier 1 base stats: HP 150, DEF 10%, PROT 0.6, SPD 5, 2 turns/round.
//! Skills: dice_thousand, hollow_victory, card_doomsday, jackpot_requiem,
//! summon_mahjong.
//!
//! The Gambler is a summon-control boss that summons mahjong tile minions
//! (red/green/white) and pressures heroes with random debuffs, tag-based
//! mechanics, stress, and bleed. Its AI brain prioritizes summon_mahjong
//! (weight 1000) over all other skills (each weight 0.25).
//!
//! Game-gaps:
//! - Summon Two Mahjong effect modeled as status marker only
//! - Mahjong summon pool (red/green/white, 1 each max, equal chance) not modeled
//! - Gambler Random Debuff (dice_thousand) picks from CRIT-3/DEF-10/SPD-3/DMG_PERCENT-20
//!   — modeled as generic "random_debuff" status marker
//! - Gambler Tag Debuff (hollow_victory) applies tag + -20% protection
//!   — modeled as "tag" + "prot_debuff" status markers
//! - Gambler Tag Stress (jackpot_requiem) applies Stress 1 only if target is tagged
//!   — modeled as "tag_stress" status marker; conditional trigger not modeled
//! - Position-based targeting (launch 234, target ~1234/~34/1234)
//!   approximated as AllEnemies
//! - ~1234 AoE inverse targeting approximated as AllEnemies
//! - ~34 targeting (back 2 ranks) approximated as AllEnemies
//! - Two turns per round not modeled in Archetype
//! - PROT (0.6), MAGIC_PROT (0.5) not modeled in Archetype
//! - Stun Resist 50%, Poison Resist 100% (immune), Bleed Resist 35%,
//!   Debuff Resist 40%, Move Resist 100% (immune), Burn Resist 70%,
//!   Frozen Resist 35% not modeled
//! - 5% crit rate on all skills — modeled as archetype crit_chance

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Gambler base archetype — tier 1 boss stats from DDGC data.
///
/// HP 150, weapon damage derived from dice_thousand skill (dmg 1–12 avg 6.5),
/// speed 5, defense 0.10 (10% dodge).
/// Summoner role: summons mahjong tiles (red/green/white) and pressures
/// with random debuffs, tag mechanics, stress, and bleed.
/// Crit 5% from all skills.
/// PROT 0.6, MAGIC_PROT 0.5, Stun Resist 50%, Poison Resist 100% (immune),
/// Bleed Resist 35%, Debuff Resist 40%, Move Resist 100% (immune),
/// Burn Resist 70%, Frozen Resist 35% — all not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Gambler"),
        side: CombatSide::Enemy,
        health: 150.0,
        max_health: 150.0,
        attack: 6.5,
        defense: 0.10,
        speed: 5.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.05,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Gambler Skills ───────────────────────────────────────────────────────────

/// Dice Thousand — AoE random debuff + low damage.
///
/// DDGC reference: dmg 1–12, atk 85%, crit 5%,
/// launch ranks 2,3,4, target ~1234 (AoE all ranks),
/// effect "Gambler Random Debuff" (picks from CRIT-3, DEF-10, SPD-3, DMG_PERCENT-20).
/// Game-gap: ~1234 AoE targeting approximated as AllEnemies.
/// Game-gap: Random debuff selection modeled as generic "random_debuff" status marker.
pub fn dice_thousand() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("dice_thousand"),
        vec![
            EffectNode::damage(6.5),
            EffectNode::apply_status("random_debuff", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Hollow Victory — tag target + protection debuff + low damage.
///
/// DDGC reference: dmg 4–6, atk 85%, crit 5%,
/// launch ranks 2,3,4, target ~34 (back 2 ranks),
/// effect "Gambler Tag Debuff" (tags target + -20% protection).
/// Game-gap: ~34 targeting (back 2 ranks) approximated as AllEnemies.
/// Game-gap: Tag mechanic modeled as "tag" status marker only.
/// Game-gap: -20% protection modeled as "prot_debuff" status marker only.
pub fn hollow_victory() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("hollow_victory"),
        vec![
            EffectNode::damage(5.0),
            EffectNode::apply_status("tag", Some(1)),
            EffectNode::apply_status("prot_debuff", Some(20)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Card Doomsday — AoE high stress, no damage.
///
/// DDGC reference: dmg 0–0, atk 85%, crit 5%,
/// launch ranks 2,3,4, target 1234 (any rank),
/// effect "Stress Range 7-10".
/// Game-gap: Stress Range 7-10 averaged to 8.
pub fn card_doomsday() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("card_doomsday"),
        vec![EffectNode::apply_status("stress", Some(8))],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Jackpot Requiem — damage + bleed + conditional tag stress.
///
/// DDGC reference: dmg 8–12, atk 85%, crit 5%,
/// launch ranks 2,3,4, target ~34 (back 2 ranks),
/// effects "Bleed 1" + "Gambler Tag Stress" (Stress 1 only if target is tagged).
/// Game-gap: ~34 targeting (back 2 ranks) approximated as AllEnemies.
/// Game-gap: Tag-stress conditional (only applies Stress 1 to tagged targets)
/// modeled as "tag_stress" status marker — conditional trigger not modeled.
pub fn jackpot_requiem() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("jackpot_requiem"),
        vec![
            EffectNode::damage(10.0),
            EffectNode::apply_status("bleed", Some(1)),
            EffectNode::apply_status("tag_stress", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Summon Mahjong — summon two mahjong tile minions.
///
/// DDGC reference: dmg 2–4, atk 85%, crit 5%,
/// launch rank 1, target self (empty .target),
/// effect "Summon Two Mahjong" (summons 2 from pool: mahjong_red,
/// mahjong_green, mahjong_white; 1 each max, equal 1.0 chance).
/// Game-gap: Summon mechanic modeled as "summon_mahjong" status marker only.
/// Game-gap: Mahjong selection pool and limits not modeled.
/// Game-gap: AI brain prioritizes this skill with weight 1000 — not modeled.
pub fn summon_mahjong() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("summon_mahjong"),
        vec![
            EffectNode::damage(3.0),
            EffectNode::apply_status("summon_mahjong", Some(2)),
        ],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// All 5 Gambler skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        dice_thousand(),
        hollow_victory(),
        card_doomsday(),
        jackpot_requiem(),
        summon_mahjong(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gambler_archetype_is_enemy_eldritch_summoner() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Gambler");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 150.0);
        assert_eq!(arch.max_health, 150.0);
        assert_eq!(arch.speed, 5.0, "gambler has SPD 5");
        assert_eq!(arch.defense, 0.10, "gambler has 10% defense");
        assert_eq!(arch.attack, 6.5, "attack from dice_thousand avg 1-12");
        assert_eq!(arch.crit_chance, 0.05, "crit 5% from all skills");
    }

    #[test]
    fn gambler_dice_thousand_applies_damage_and_random_debuff() {
        let skill = dice_thousand();
        assert_eq!(skill.id.0, "dice_thousand");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "dice_thousand must deal damage");
        let has_debuff = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("random_debuff")
        });
        assert!(has_debuff, "dice_thousand must apply random_debuff status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllEnemies",
            "dice_thousand targets all enemies (AoE ~1234)"
        );
    }

    #[test]
    fn gambler_hollow_victory_applies_tag_and_prot_debuff() {
        let skill = hollow_victory();
        assert_eq!(skill.id.0, "hollow_victory");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "hollow_victory must deal damage");
        let has_tag = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("tag")
        });
        assert!(has_tag, "hollow_victory must apply tag status");
        let has_prot_debuff = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("prot_debuff")
        });
        assert!(has_prot_debuff, "hollow_victory must apply prot_debuff status");
    }

    #[test]
    fn gambler_card_doomsday_applies_high_stress() {
        let skill = card_doomsday();
        assert_eq!(skill.id.0, "card_doomsday");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "card_doomsday must apply stress status");
        let stress_effect = skill.effects.iter().find(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(stress_effect.is_some(), "card_doomsday must have stress effect");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllEnemies",
            "card_doomsday targets all enemies (1234)"
        );
    }

    #[test]
    fn gambler_jackpot_requiem_applies_bleed_and_tag_stress() {
        let skill = jackpot_requiem();
        assert_eq!(skill.id.0, "jackpot_requiem");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "jackpot_requiem must deal damage");
        let has_bleed = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("bleed")
        });
        assert!(has_bleed, "jackpot_requiem must apply bleed status");
        let has_tag_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("tag_stress")
        });
        assert!(has_tag_stress, "jackpot_requiem must apply tag_stress status");
    }

    #[test]
    fn gambler_summon_mahjong_applies_summon_marker() {
        let skill = summon_mahjong();
        assert_eq!(skill.id.0, "summon_mahjong");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "summon_mahjong must deal damage");
        let has_summon = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("summon_mahjong")
        });
        assert!(has_summon, "summon_mahjong must apply summon_mahjong status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "summon_mahjong targets self (empty .target in DDGC)"
        );
    }

    #[test]
    fn gambler_skill_pack_has_five_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 5);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"dice_thousand"), "missing dice_thousand");
        assert!(ids.contains(&"hollow_victory"), "missing hollow_victory");
        assert!(ids.contains(&"card_doomsday"), "missing card_doomsday");
        assert!(ids.contains(&"jackpot_requiem"), "missing jackpot_requiem");
        assert!(ids.contains(&"summon_mahjong"), "missing summon_mahjong");
    }

    #[test]
    fn gambler_summon_plus_random_debuff_plus_stress_identity() {
        // The core identity of gambler is a summon-control boss that
        // summons mahjong tiles while pressuring with random debuffs,
        // tag mechanics, stress, and bleed.
        let pack = skill_pack();

        let has_summon = pack.iter().any(|s| {
            s.id.0 == "summon_mahjong"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("summon_mahjong")
                })
        });

        let has_random_debuff = pack.iter().any(|s| {
            s.id.0 == "dice_thousand"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("random_debuff")
                })
        });

        let has_stress = pack.iter().any(|s| {
            s.id.0 == "card_doomsday"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stress")
                })
        });

        assert!(has_summon, "gambler must have summon_mahjong skill");
        assert!(has_random_debuff, "gambler must have random_debuff skill");
        assert!(has_stress, "gambler must have high-stress skill");
    }
}
