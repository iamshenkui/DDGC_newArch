//! Lantern — ZhuQue ranged pressure family (stress + burn).
//!
//! DDGC reference: Eldritch-type ranged monster from the ZhuQue dungeon.
//! Tier 1 base stats: HP 70, DEF 7.5%, PROT 0.3, SPD 6, crit 3%.
//! Skills: stress (low dmg + high stress), burn_attack (magic dmg + burn DoT).
//!
//! This family's defining identity is dual pressure: stress delivery (7–8 per hit)
//! paired with high burn DoT (12/turn for 3 turns). The lantern has no movement
//! skill and equal 50/50 AI weighting between its two skills.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Lantern base archetype — tier 1 stats from DDGC data.
///
/// HP 70, weapon damage derived from burn_attack skill (9–15 avg 12),
/// speed 6, defense 0.075 (7.5% dodge), crit 3% (from burn_attack).
/// Ranged role: stress + burn pressure with PROT 0.3 and MAGIC_PROT 0.6 (not modeled in Archetype).
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Lantern"),
        side: CombatSide::Enemy,
        health: 70.0,
        max_health: 70.0,
        attack: 12.0,
        defense: 0.075,
        speed: 6.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.03,
        dodge: 0.0,
    }
}

// ── Lantern Skills ────────────────────────────────────────────────────────

/// Stress — ranged attack that deals low damage and applies high stress.
///
/// DDGC reference: dmg 1–2 (avg 1.5), atk 82.5%, crit 0%,
/// effect "Stress Range 7-8" (avg 7.5), launch 1234, target ~1234.
/// Game-gap: ~1234 (AoE any enemy rank) is approximated as AllEnemies.
/// Game-gap: "Stress Range 7-8" is averaged to 7.5; random range is not modeled.
pub fn stress() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("stress"),
        vec![
            EffectNode::damage(1.5),
            EffectNode::apply_status("stress", Some(8)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Burn Attack — ranged magic attack that deals moderate damage and applies burn DoT.
///
/// DDGC reference: magic dmg 9–15 (avg 12), atk 82.5%, crit 3%,
/// effect "New Burn 1" (12 burn/turn for 3 turns), launch 1234, target 1234.
/// Game-gap: .magic_dmg flag is not modeled — damage is treated as normal damage.
/// Game-gap: "New Burn 1" burn value (12/turn) is captured as a status marker;
/// actual burn DoT resolution is not implemented.
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

/// All 2 Lantern skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![stress(), burn_attack()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lantern_archetype_is_enemy_eldritch_ranged() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Lantern");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 70.0);
        assert_eq!(arch.max_health, 70.0);
        assert_eq!(arch.speed, 6.0);
        assert_eq!(arch.defense, 0.075, "lantern has 7.5% defense");
        assert_eq!(arch.crit_chance, 0.03, "lantern has 3% crit from burn_attack");
        assert_eq!(arch.attack, 12.0, "lantern attack from burn_attack avg");
    }

    #[test]
    fn lantern_stress_deals_damage_and_applies_stress() {
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
    fn lantern_burn_attack_deals_damage_and_applies_burn() {
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
    fn lantern_skill_pack_has_two_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 2);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"stress"), "missing stress skill");
        assert!(ids.contains(&"burn_attack"), "missing burn_attack skill");
    }

    #[test]
    fn lantern_burn_plus_stress_identity() {
        // The core identity of lantern is burn + stress dual pressure.
        // This test preserves that identity.
        let pack = skill_pack();
        let has_stress = pack.iter().any(|s| {
            s.id.0 == "stress"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stress")
                })
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Damage)
                })
        });
        let has_burn = pack.iter().any(|s| {
            s.id.0 == "burn_attack"
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
            "lantern must have stress skill with damage and stress status"
        );
        assert!(
            has_burn,
            "lantern must have burn_attack skill with damage and burn status"
        );
    }
}
