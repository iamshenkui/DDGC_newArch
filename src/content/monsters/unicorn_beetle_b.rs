//! Unicorn Beetle B — BaiHu ranged family (bleed + stress).
//!
//! DDGC reference: Eldritch-type ranged monster from the BaiHu dungeon.
//! Tier 1 base stats: HP 62, DEF 15%, PROT 0.6, SPD 6.
//! Skills: normal_attack (ranged, ignore def), bleed (ranged + Bleed 1),
//! stress (AoE + Stress 0), move.
//!
//! This family is distinct from unicorn_beetle_A: it swaps bleed_crowd
//! for stress, combining bleed pressure with AoE stress instead of AoE bleed.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Unicorn Beetle B base archetype — tier 1 stats from DDGC data.
///
/// HP 62, weapon damage derived from normal_attack skill (8–20 avg 14),
/// speed 6, defense 15%, crit 12%.
/// Defense 15% mapped to `defense` field as 0.15.
/// Ranged role: applies bleed + AoE stress from back ranks.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Unicorn Beetle B"),
        side: CombatSide::Enemy,
        health: 62.0,
        max_health: 62.0,
        attack: 14.0,
        defense: 0.15,
        speed: 6.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.12,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Unicorn Beetle B Skills ──────────────────────────────────────────────────

/// Normal Attack — ranged attack that ignores defense, targeting ranks 3–4.
///
/// DDGC reference: dmg 8–20 (avg 14), atk 82.5%, crit 12%,
/// launch ranks 3–4, target ranks 3–4, is_ignore_def True.
/// Game-gap: position-based targeting not modeled — targets AllEnemies;
/// armor-piercing semantic captured by skill name only.
pub fn normal_attack() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("normal_attack"),
        vec![EffectNode::damage(14.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Bleed — ranged attack with bleed status targeting all enemy ranks.
///
/// DDGC reference: dmg 8–20 (avg 14), applies "New Bleed 1",
/// atk 82.5%, crit 9%, launch ranks 3–4, target ranks 1–4.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn bleed() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("bleed"),
        vec![
            EffectNode::damage(14.0),
            EffectNode::apply_status("bleed", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Stress — AoE ranged attack applying stress to all enemy ranks.
///
/// DDGC reference: dmg 0–0, applies "Stress 0",
/// atk 82.5%, crit 0%, launch ranks 3–4, target ~1234 (AoE all 4 positions).
/// Game-gap: AoE vs single-target distinction not modeled — targets AllEnemies.
/// Stress 0 is preserved as an identity marker even though value is zero.
pub fn stress() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("stress"),
        vec![EffectNode::apply_status("stress", Some(0))],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Move — self-positioning skill.
///
/// DDGC reference: dmg 0–0, launch ranks 1–2, target @23, move 1 0.
/// Game-gap: position-based movement not modeled — approximated as
/// EffectNode::push(0) with SelfOnly target.
pub fn move_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("move"),
        vec![EffectNode::push(0)],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// All 4 Unicorn Beetle B skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![normal_attack(), bleed(), stress(), move_skill()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unicorn_beetle_b_archetype_is_enemy_eldritch() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Unicorn Beetle B");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 62.0);
        assert_eq!(arch.max_health, 62.0);
        assert_eq!(arch.speed, 6.0);
        assert_eq!(arch.defense, 0.15, "unicorn_beetle_B has 15% defense");
        assert_eq!(arch.crit_chance, 0.12, "unicorn_beetle_B has 12% crit");
    }

    #[test]
    fn unicorn_beetle_b_normal_attack_deals_damage() {
        let skill = normal_attack();
        assert_eq!(skill.id.0, "normal_attack");
        assert!(!skill.effects.is_empty(), "normal_attack should have effects");
    }

    #[test]
    fn unicorn_beetle_b_bleed_applies_bleed() {
        let skill = bleed();
        assert_eq!(skill.id.0, "bleed");
        assert!(skill.effects.len() >= 2, "bleed should have damage + bleed status");
    }

    #[test]
    fn unicorn_beetle_b_stress_applies_stress() {
        let skill = stress();
        assert_eq!(skill.id.0, "stress");
        assert!(
            skill.effects.iter().any(|e| e.status_kind.as_deref() == Some("stress")),
            "stress skill should apply stress status"
        );
    }

    #[test]
    fn unicorn_beetle_b_skill_pack_has_four_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 4);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"normal_attack"), "missing normal_attack skill");
        assert!(ids.contains(&"bleed"), "missing bleed skill");
        assert!(ids.contains(&"stress"), "missing stress skill");
        assert!(ids.contains(&"move"), "missing move skill");
    }

    #[test]
    fn unicorn_beetle_b_bleed_plus_stress_identity() {
        // The core identity of unicorn_beetle_B is bleed (ranged bleed)
        // plus stress (AoE stress). This test preserves that identity.
        let pack = skill_pack();
        let has_bleed = pack.iter().any(|s| {
            s.id.0 == "bleed"
                && s.effects.iter().any(|e| {
                    e.status_kind.as_deref() == Some("bleed")
                })
        });
        let has_stress = pack.iter().any(|s| {
            s.id.0 == "stress"
                && s.effects.iter().any(|e| {
                    e.status_kind.as_deref() == Some("stress")
                })
        });
        assert!(has_bleed, "unicorn_beetle_B must have bleed skill with bleed status");
        assert!(has_stress, "unicorn_beetle_B must have stress skill with stress status");
    }
}
