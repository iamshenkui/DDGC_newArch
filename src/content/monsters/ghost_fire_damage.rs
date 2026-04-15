//! Ghost Fire Damage — ZhuQue ranged damage family (burn + stress + self-split on death).
//!
//! DDGC reference: Eldritch-type damage monster from the ZhuQue dungeon.
//! Tier 1 base stats: HP 72, DEF 7.5%, PROT 0.3, SPD 6, crit 3%.
//! Skills: stress (damage + stress), burn_attack (damage + burn), ghost_fire_split (summon).
//!
//! This family's defining identity is a ranged damage unit that combines
//! burn and stress pressure, and can split into a new ghost fire on death.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Ghost Fire Damage base archetype — tier 1 stats from DDGC data.
///
/// HP 72, weapon damage derived from stress/burn_attack skill (9–15 avg 12),
/// speed 6, defense 0.075 (7.5% dodge).
/// Ranged role: burn + stress damage dealer with PROT 0.3 (not modeled in Archetype).
/// Crit 3% — burn_attack has 3% crit at tier 1.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Ghost Fire Damage"),
        side: CombatSide::Enemy,
        health: 72.0,
        max_health: 72.0,
        attack: 12.0,
        defense: 0.075,
        speed: 6.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.03,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Ghost Fire Damage Skills ─────────────────────────────────────────────

/// Stress — ranged attack that deals damage and applies stress.
///
/// DDGC reference: dmg 9–15 (avg 12), atk 82.5%, crit 0%,
/// target 1234 (any rank), effect "Stress Range 12-16".
/// Stress value averaged: (12+16)/2 = 14.
/// Game-gap: position-based targeting (.launch 1234, .target 1234) is
/// approximated as AllEnemies.
pub fn stress() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("stress"),
        vec![
            EffectNode::damage(12.0),
            EffectNode::apply_status("stress", Some(14)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Burn Attack — ranged attack that deals damage and applies burn.
///
/// DDGC reference: dmg 9–15 (avg 12), atk 82.5%, crit 3%,
/// target 1234 (any rank), effect "New Burn 1".
/// Game-gap: position-based targeting (.launch 1234, .target 1234) is
/// approximated as AllEnemies.
pub fn burn_attack() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("burn_attack"),
        vec![
            EffectNode::damage(12.0),
            EffectNode::apply_status("burn", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Ghost Fire Split — zero-damage summon skill.
///
/// DDGC reference: dmg 0–0, atk 100%, crit 0%,
/// effect "Ghost Fire Summon 1": 50/50 chance to summon either
/// ghost_fire_assist_1 or ghost_fire_damage_1, apply_once true.
/// 3-round cooldown in AI brain (space_skill type).
/// Modeled as a self-applied "split" status marker to preserve identity.
/// Game-gap: the actual summon mechanic (spawning a new ghost fire unit
/// of either type) is not modeled in the framework.
pub fn ghost_fire_split() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("ghost_fire_split"),
        vec![EffectNode::apply_status("split", Some(1))],
        TargetSelector::SelfOnly,
        1,
        Some(3),
    )
}

/// All 3 Ghost Fire Damage skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![stress(), burn_attack(), ghost_fire_split()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ghost_fire_damage_archetype_is_enemy_eldritch_ranged() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Ghost Fire Damage");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 72.0);
        assert_eq!(arch.max_health, 72.0);
        assert_eq!(arch.speed, 6.0);
        assert_eq!(arch.defense, 0.075, "ghost_fire_damage has 7.5% defense");
        assert_eq!(arch.crit_chance, 0.03, "ghost_fire_damage has 3% crit");
        assert_eq!(arch.attack, 12.0, "ghost_fire_damage attack from stress/burn_attack avg");
    }

    #[test]
    fn ghost_fire_damage_stress_deals_damage_and_applies_stress() {
        let skill = stress();
        assert_eq!(skill.id.0, "stress");
        assert!(
            skill.effects.len() >= 2,
            "stress should have damage + stress effects"
        );
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        assert!(has_damage, "stress must have damage effect");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "stress must apply stress status");
    }

    #[test]
    fn ghost_fire_damage_burn_attack_deals_damage_and_applies_burn() {
        let skill = burn_attack();
        assert_eq!(skill.id.0, "burn_attack");
        assert!(
            skill.effects.len() >= 2,
            "burn_attack should have damage + burn effects"
        );
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        assert!(has_damage, "burn_attack must have damage effect");
        let has_burn = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("burn")
        });
        assert!(has_burn, "burn_attack must apply burn status");
    }

    #[test]
    fn ghost_fire_damage_ghost_fire_split_applies_split_status() {
        let skill = ghost_fire_split();
        assert_eq!(skill.id.0, "ghost_fire_split");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "ghost_fire_split targets self"
        );
        let has_split = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("split")
        });
        assert!(has_split, "ghost_fire_split must apply split status");
        assert_eq!(skill.cooldown, Some(3), "ghost_fire_split has 3-round cooldown");
    }

    #[test]
    fn ghost_fire_damage_skill_pack_has_three_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 3);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"stress"), "missing stress skill");
        assert!(ids.contains(&"burn_attack"), "missing burn_attack skill");
        assert!(ids.contains(&"ghost_fire_split"), "missing ghost_fire_split skill");
    }

    #[test]
    fn ghost_fire_damage_burn_plus_stress_plus_split_identity() {
        // The core identity of ghost_fire_damage is burn + stress damage plus split-on-death.
        // This test preserves that identity.
        let pack = skill_pack();
        let has_burn = pack.iter().any(|s| {
            s.id.0 == "burn_attack"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("burn")
                })
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Damage)
                })
        });
        let has_stress = pack.iter().any(|s| {
            s.id.0 == "stress"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stress")
                })
        });
        let has_split = pack.iter().any(|s| {
            s.id.0 == "ghost_fire_split"
                && format!("{:?}", s.target_selector) == "SelfOnly"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("split")
                })
        });
        assert!(
            has_burn,
            "ghost_fire_damage must have burn_attack with damage and burn status"
        );
        assert!(
            has_stress,
            "ghost_fire_damage must have stress skill with stress status"
        );
        assert!(
            has_split,
            "ghost_fire_damage must have ghost_fire_split self-applied status"
        );
    }
}
