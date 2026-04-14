//! Moth Mimicry A — QingLong ranged family (poison + stress-poison).
//!
//! DDGC reference: Eldritch-type ranged monster from the QingLong dungeon.
//! Tier 1 base stats: HP 63, DEF 5%, PROT 0%, SPD 6.
//! Skills: normal_attack (ranged + stress), poison (ranged blight),
//! stress_poison (AoE blight + stress).
//!
//! This family is distinct from moth_mimicry_B: it focuses on
//! blight + stress-poison pressure rather than AoE stress crowd control.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Moth Mimicry A base archetype — tier 1 stats from DDGC data.
///
/// HP 63, weapon damage derived from normal_attack skill (18–24 avg 21),
/// speed 6, dodge 5%, crit 12%.
/// Defense 5% mapped to `defense` field as 0.05.
/// Ranged role: applies blight and stress from back ranks.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Moth Mimicry A"),
        side: CombatSide::Enemy,
        health: 63.0,
        max_health: 63.0,
        attack: 21.0,
        defense: 0.05,
        speed: 6.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.12,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Moth Mimicry A Skills ──────────────────────────────────────────────────

/// Normal Attack — ranged attack with stress targeting all enemy ranks.
///
/// DDGC reference: dmg 18–24 (avg 21), applies "Stress 2",
/// atk 82.5%, crit 12%, launch ranks 1–4, target ranks 1–4.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn normal_attack() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("normal_attack"),
        vec![
            EffectNode::damage(21.0),
            EffectNode::apply_status("stress", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Poison — ranged blight attack targeting all enemy ranks.
///
/// DDGC reference: dmg 9–12 (avg 10.5), applies "New Blight 1",
/// atk 82.5%, crit 0%, launch ranks 1–4, target ranks 1–4.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn poison() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("poison"),
        vec![
            EffectNode::damage(10.5),
            EffectNode::apply_status("blight", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Stress Poison — AoE blight + stress attack targeting all enemy ranks.
///
/// DDGC reference: dmg 0–0, applies "New Blight 1" and "Stress 0",
/// atk 82.5%, crit 0%, launch ranks 1–4, target #1234 (AoE all 4 positions).
/// Game-gap: position-based targeting and AoE vs single-target distinction
/// not modeled — targets AllEnemies; Stress 0 preserved as identity marker.
pub fn stress_poison() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("stress_poison"),
        vec![
            EffectNode::apply_status("blight", Some(1)),
            EffectNode::apply_status("stress", Some(0)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 3 Moth Mimicry A skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![normal_attack(), poison(), stress_poison()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moth_mimicry_a_archetype_is_enemy_eldritch() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Moth Mimicry A");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 63.0);
        assert_eq!(arch.max_health, 63.0);
        assert_eq!(arch.speed, 6.0);
        assert_eq!(arch.defense, 0.05, "moth_mimicry_A has 5% defense");
        assert_eq!(arch.crit_chance, 0.12, "moth_mimicry_A has 12% crit");
    }

    #[test]
    fn moth_mimicry_a_normal_attack_applies_stress() {
        let skill = normal_attack();
        assert_eq!(skill.id.0, "normal_attack");
        assert!(skill.effects.len() >= 2, "normal_attack should have damage + stress");
    }

    #[test]
    fn moth_mimicry_a_poison_applies_blight() {
        let skill = poison();
        assert_eq!(skill.id.0, "poison");
        assert!(skill.effects.len() >= 2, "poison should have damage + blight status");
    }

    #[test]
    fn moth_mimicry_a_stress_poison_applies_blight_and_stress() {
        let skill = stress_poison();
        assert_eq!(skill.id.0, "stress_poison");
        // Must have blight + stress (even if stress value is 0)
        assert!(skill.effects.len() >= 2, "stress_poison should have blight + stress");
    }

    #[test]
    fn moth_mimicry_a_skill_pack_has_three_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 3);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"normal_attack"), "missing normal_attack skill");
        assert!(ids.contains(&"poison"), "missing poison skill");
        assert!(ids.contains(&"stress_poison"), "missing stress_poison skill");
    }

    #[test]
    fn moth_mimicry_a_poison_plus_stress_poison_identity() {
        // The core identity of moth_mimicry_A is poison (ranged blight)
        // plus stress_poison (AoE blight + stress). This test preserves
        // that identity.
        let pack = skill_pack();
        let has_poison_blight = pack.iter().any(|s| {
            s.id.0 == "poison"
                && s.effects.iter().any(|e| {
                    e.status_kind.as_deref() == Some("blight")
                })
        });
        let has_stress_poison = pack.iter().any(|s| {
            s.id.0 == "stress_poison"
                && s.effects.iter().any(|e| {
                    e.status_kind.as_deref() == Some("blight")
                })
                && s.effects.iter().any(|e| {
                    e.status_kind.as_deref() == Some("stress")
                })
        });
        assert!(has_poison_blight, "moth_mimicry_A must have poison skill with blight");
        assert!(has_stress_poison, "moth_mimicry_A must have stress_poison skill with blight + stress");
    }
}
