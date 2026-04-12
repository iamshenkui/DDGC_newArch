//! Moth Fire — ZhuQue ranged controller family (stress + cocoon cycle + fire dive).
//!
//! DDGC reference: Eldritch-type ranged monster from the ZhuQue dungeon.
//! Tier 1 base stats: HP 65, DEF 7.5%, PROT 0.4, SPD 6, crit 0%.
//! Skills: stress_attack (stress), cocoon (defend + self-heal), fly_into_fire (burn + stress AoE).
//!
//! This family's defining identity is a cocoon cycle: the AI alternates between
//! stress pressure and a defensive cocoon (defender + self-heal), with a
//! guaranteed fire dive after emerging from cocoon.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Moth Fire base archetype — tier 1 stats from DDGC data.
///
/// HP 65, weapon damage derived from fly_into_fire skill (15–24 avg 19.5),
/// speed 6, defense 0.075 (7.5% dodge).
/// Ranged role: stress + cocoon defense cycle + fire dive with PROT 0.4 (not modeled in Archetype).
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Moth Fire"),
        side: CombatSide::Enemy,
        health: 65.0,
        max_health: 65.0,
        attack: 19.5,
        defense: 0.075,
        speed: 6.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.0,
        dodge: 0.0,
    }
}

// ── Moth Fire Skills ────────────────────────────────────────────────────────

/// Stress Attack — ranged attack that deals low damage and applies stress.
///
/// DDGC reference: dmg 2–3 (avg 2.5), atk 82.5%, crit 0%,
/// effect "Stress 2", launch 1234, target 1234.
/// Game-gap: position-based targeting (.launch 1234, .target 1234) is
/// approximated as AllEnemies.
pub fn stress_attack() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("stress_attack"),
        vec![
            EffectNode::damage(2.5),
            EffectNode::apply_status("stress", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Cocoon — zero-damage self-buff that applies defender and self-heal.
///
/// DDGC reference: dmg 0–0, atk 100%, crit 0%,
/// effects "Defender 2" and "HealSelf Percent 50",
/// launch 1234, target (empty = self).
/// Game-gap: "HealSelf Percent 50" is modeled as a status marker; actual
/// heal percentage resolution is not implemented.
/// Game-gap: cocoon's AI brain triggers fly_into_fire after cocoon (guaranteed
/// follow-up) — this combo sequencing is not modeled.
pub fn cocoon() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("cocoon"),
        vec![
            EffectNode::apply_status("defender", Some(2)),
            EffectNode::apply_status("heal_self", Some(50)),
        ],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// Fly Into Fire — AoE ranged attack that deals high damage, applies burn and stress.
///
/// DDGC reference: dmg 15–24 (avg 19.5), atk 82.5%, crit 0%,
/// effects "New Burn 1" and "Stress Range 3-5" (avg 4),
/// launch 1234, target ~1234.
/// Game-gap: ~1234 (AoE all enemies) is approximated as AllEnemies.
/// Game-gap: "Stress Range 3-5" is averaged to 4; random range is not modeled.
pub fn fly_into_fire() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("fly_into_fire"),
        vec![
            EffectNode::damage(19.5),
            EffectNode::apply_status("burn", Some(1)),
            EffectNode::apply_status("stress", Some(4)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 3 Moth Fire skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![stress_attack(), cocoon(), fly_into_fire()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moth_fire_archetype_is_enemy_eldritch_ranged() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Moth Fire");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 65.0);
        assert_eq!(arch.max_health, 65.0);
        assert_eq!(arch.speed, 6.0);
        assert_eq!(arch.defense, 0.075, "moth_fire has 7.5% defense");
        assert_eq!(arch.crit_chance, 0.0, "moth_fire has 0% crit");
        assert_eq!(arch.attack, 19.5, "moth_fire attack from fly_into_fire avg");
    }

    #[test]
    fn moth_fire_stress_attack_deals_damage_and_applies_stress() {
        let skill = stress_attack();
        assert_eq!(skill.id.0, "stress_attack");
        assert!(
            skill.effects.len() >= 2,
            "stress_attack should have damage + stress effects"
        );
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        assert!(has_damage, "stress_attack must have damage effect");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "stress_attack must apply stress status");
    }

    #[test]
    fn moth_fire_cocoon_applies_defender_and_heal_self() {
        let skill = cocoon();
        assert_eq!(skill.id.0, "cocoon");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "cocoon targets self"
        );
        let has_defender = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("defender")
        });
        assert!(has_defender, "cocoon must apply defender status");
        let has_heal_self = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("heal_self")
        });
        assert!(has_heal_self, "cocoon must apply heal_self status");
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        assert!(!has_damage, "cocoon should not deal damage");
    }

    #[test]
    fn moth_fire_fly_into_fire_deals_damage_burn_and_stress() {
        let skill = fly_into_fire();
        assert_eq!(skill.id.0, "fly_into_fire");
        assert!(
            skill.effects.len() >= 3,
            "fly_into_fire should have damage + burn + stress effects"
        );
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        assert!(has_damage, "fly_into_fire must have damage effect");
        let has_burn = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("burn")
        });
        assert!(has_burn, "fly_into_fire must apply burn status");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "fly_into_fire must apply stress status");
    }

    #[test]
    fn moth_fire_skill_pack_has_three_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 3);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"stress_attack"), "missing stress_attack skill");
        assert!(ids.contains(&"cocoon"), "missing cocoon skill");
        assert!(ids.contains(&"fly_into_fire"), "missing fly_into_fire skill");
    }

    #[test]
    fn moth_fire_cocoon_cycle_plus_fire_pressure_identity() {
        // The core identity of moth_fire is stress + cocoon (defend+heal) + fire dive.
        // This test preserves that identity.
        let pack = skill_pack();
        let has_stress = pack.iter().any(|s| {
            s.id.0 == "stress_attack"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stress")
                })
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Damage)
                })
        });
        let has_cocoon = pack.iter().any(|s| {
            s.id.0 == "cocoon"
                && format!("{:?}", s.target_selector) == "SelfOnly"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("defender")
                })
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("heal_self")
                })
        });
        let has_fire_dive = pack.iter().any(|s| {
            s.id.0 == "fly_into_fire"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Damage)
                })
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("burn")
                })
        });
        assert!(
            has_stress,
            "moth_fire must have stress_attack with damage and stress status"
        );
        assert!(
            has_cocoon,
            "moth_fire must have cocoon with self-target defender and heal_self"
        );
        assert!(
            has_fire_dive,
            "moth_fire must have fly_into_fire with damage and burn status"
        );
    }
}
