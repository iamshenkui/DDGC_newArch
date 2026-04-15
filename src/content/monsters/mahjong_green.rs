//! Mahjong Green — ZhuQue boss minion (ally-buff + AoE damage).
//!
//! DDGC reference: Eldritch-type minion from the ZhuQue dungeon.
//! Stats: HP 20, DEF 10%, PROT 0, SPD 5, 1 turn/round.
//! Skills: fa_cai_blessing, high_roller.
//!
//! The Mahjong Green is a summoned minion that can buff allies with
//! `fa_cai_blessing` ($1234 — targets any ally including self) and deal
//! AoE damage with `high_roller` (~1234). Its AI brain equally weights
//! both skills (0.5 each). It is immune to bleed and poison.
//!
//! Game-gaps:
//! - $1234 targeting (any ally, including self) approximated as AllAllies
//! - ~1234 AoE targeting approximated as AllEnemies
//! - fa_cai_blessing has no explicit effect in the .txt — modeled as
//!   "ally_buff" status marker on AllAllies target
//! - high_roller has no explicit effect in the .txt — modeled as damage only
//! - PROT 0, MAGIC_PROT 0.5 not modeled in Archetype
//! - Stun Resist 50%, Poison Resist 100% (immune), Bleed Resist 100% (immune),
//!   Debuff Resist 40%, Move Resist 70%, Burn Resist 35%, Frozen Resist 35%
//!   not modeled

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Mahjong Green base archetype — minion stats from DDGC data.
///
/// HP 20, weapon damage derived from fa_cai_blessing skill (dmg 4–6 avg 5),
/// speed 5, defense 0.10 (10% dodge).
/// Support role: buffs allies and deals AoE damage.
/// Crit 5% from both skills.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Mahjong Green"),
        side: CombatSide::Enemy,
        health: 20.0,
        max_health: 20.0,
        attack: 5.0,
        defense: 0.10,
        speed: 5.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.05,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Mahjong Green Skills ─────────────────────────────────────────────────────

/// Fa Cai Blessing — damage + ally buff.
///
/// DDGC reference: dmg 4–6, atk 85%, crit 5%,
/// launch ranks 1,2,3,4, target $1234 (any ally including self),
/// no explicit effect listed in .txt — inferred ally buff.
/// Game-gap: $1234 targeting (any ally including self) approximated as AllAllies.
/// Game-gap: Ally buff effect modeled as "ally_buff" status marker only.
pub fn fa_cai_blessing() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("fa_cai_blessing"),
        vec![
            EffectNode::damage(5.0),
            EffectNode::apply_status("ally_buff", Some(1)),
        ],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// High Roller — AoE damage to all enemies.
///
/// DDGC reference: dmg 4–6, atk 85%, crit 5%,
/// launch ranks 1,2,3,4, target ~1234 (AoE all ranks),
/// no explicit effect listed in .txt.
/// Game-gap: ~1234 AoE targeting approximated as AllEnemies.
pub fn high_roller() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("high_roller"),
        vec![EffectNode::damage(5.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 2 Mahjong Green skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![fa_cai_blessing(), high_roller()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mahjong_green_archetype_is_enemy_eldritch_support() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Mahjong Green");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 20.0);
        assert_eq!(arch.max_health, 20.0);
        assert_eq!(arch.speed, 5.0, "mahjong_green has SPD 5");
        assert_eq!(arch.defense, 0.10, "mahjong_green has 10% defense");
        assert_eq!(arch.attack, 5.0, "attack from fa_cai_blessing avg 4-6");
        assert_eq!(arch.crit_chance, 0.05, "crit 5% from all skills");
    }

    #[test]
    fn mahjong_green_fa_cai_blessing_applies_ally_buff() {
        let skill = fa_cai_blessing();
        assert_eq!(skill.id.0, "fa_cai_blessing");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "fa_cai_blessing must deal damage");
        let has_buff = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("ally_buff")
        });
        assert!(has_buff, "fa_cai_blessing must apply ally_buff status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllAllies",
            "fa_cai_blessing targets all allies ($1234)"
        );
    }

    #[test]
    fn mahjong_green_high_roller_deals_aoe_damage() {
        let skill = high_roller();
        assert_eq!(skill.id.0, "high_roller");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "high_roller must deal damage");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllEnemies",
            "high_roller targets all enemies (AoE ~1234)"
        );
    }

    #[test]
    fn mahjong_green_skill_pack_has_two_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 2);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"fa_cai_blessing"), "missing fa_cai_blessing");
        assert!(ids.contains(&"high_roller"), "missing high_roller");
    }

    #[test]
    fn mahjong_green_ally_buff_plus_aoe_damage_identity() {
        // The core identity of mahjong_green is a support minion that
        // buffs allies and deals AoE damage.
        let pack = skill_pack();

        let has_ally_buff = pack.iter().any(|s| {
            s.id.0 == "fa_cai_blessing"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("ally_buff")
                })
        });

        let has_aoe_damage = pack.iter().any(|s| {
            s.id.0 == "high_roller"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Damage)
                })
        });

        assert!(has_ally_buff, "mahjong_green must have ally_buff skill");
        assert!(has_aoe_damage, "mahjong_green must have AoE damage skill");
    }
}
