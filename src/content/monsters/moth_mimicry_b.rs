//! Moth Mimicry B — QingLong ranged family (poison + crowd-stress).
//!
//! DDGC reference: Eldritch-type ranged monster from the QingLong dungeon.
//! Tier 1 base stats: HP 63, DEF 5%, PROT 0%, SPD 6.
//! Skills: poison (ranged blight), stress (ranged + stress),
//! stress_crowd (AoE stress).
//!
//! This family is distinct from moth_mimicry_A: it focuses on
//! blight + AoE stress crowd control rather than blight + stress-poison pressure.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Moth Mimicry B base archetype — tier 1 stats from DDGC data.
///
/// HP 63, weapon damage derived from poison skill (9–12 avg 10.5),
/// speed 6, dodge 5%, crit 0% (no skill has nonzero crit).
/// Defense 5% mapped to `defense` field as 0.05.
/// Ranged role: applies blight and AoE stress from back ranks.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Moth Mimicry B"),
        side: CombatSide::Enemy,
        health: 63.0,
        max_health: 63.0,
        attack: 10.5,
        defense: 0.05,
        speed: 6.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.0,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Moth Mimicry B Skills ──────────────────────────────────────────────────

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

/// Stress — ranged low-damage attack applying stress to a single target.
///
/// DDGC reference: dmg 2–3 (avg 2.5), applies "Stress 2",
/// atk 82.5%, crit 0%, launch ranks 3–4, target ranks 1–4.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn stress() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("stress"),
        vec![
            EffectNode::damage(2.5),
            EffectNode::apply_status("stress", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Stress Crowd — AoE stress attack targeting all enemy ranks.
///
/// DDGC reference: dmg 0–0, applies "Stress 0",
/// atk 82.5%, crit 0%, launch ranks 3–4, target ~1234 (AoE all 4 positions).
/// Game-gap: position-based targeting and AoE vs single-target distinction
/// not modeled — targets AllEnemies; Stress 0 preserved as identity marker.
pub fn stress_crowd() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("stress_crowd"),
        vec![EffectNode::apply_status("stress", Some(0))],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 3 Moth Mimicry B skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![poison(), stress(), stress_crowd()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moth_mimicry_b_archetype_is_enemy_eldritch() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Moth Mimicry B");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 63.0);
        assert_eq!(arch.max_health, 63.0);
        assert_eq!(arch.speed, 6.0);
        assert_eq!(arch.defense, 0.05, "moth_mimicry_B has 5% defense");
        assert_eq!(arch.crit_chance, 0.0, "moth_mimicry_B has 0% crit");
    }

    #[test]
    fn moth_mimicry_b_poison_applies_blight() {
        let skill = poison();
        assert_eq!(skill.id.0, "poison");
        assert!(
            skill.effects.len() >= 2,
            "poison should have damage + blight status"
        );
    }

    #[test]
    fn moth_mimicry_b_stress_applies_stress() {
        let skill = stress();
        assert_eq!(skill.id.0, "stress");
        assert!(
            skill.effects.len() >= 2,
            "stress should have damage + stress status"
        );
    }

    #[test]
    fn moth_mimicry_b_stress_crowd_applies_stress() {
        let skill = stress_crowd();
        assert_eq!(skill.id.0, "stress_crowd");
        assert!(
            !skill.effects.is_empty(),
            "stress_crowd should have stress status"
        );
    }

    #[test]
    fn moth_mimicry_b_skill_pack_has_three_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 3);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"poison"), "missing poison skill");
        assert!(ids.contains(&"stress"), "missing stress skill");
        assert!(ids.contains(&"stress_crowd"), "missing stress_crowd skill");
    }

    #[test]
    fn moth_mimicry_b_poison_plus_crowd_stress_identity() {
        // The core identity of moth_mimicry_B is poison (ranged blight)
        // plus stress_crowd (AoE stress). This test preserves that identity.
        let pack = skill_pack();
        let has_poison_blight = pack.iter().any(|s| {
            s.id.0 == "poison"
                && s.effects.iter().any(|e| e.status_kind.as_deref() == Some("blight"))
        });
        let has_crowd_stress = pack.iter().any(|s| {
            s.id.0 == "stress_crowd"
                && s.effects.iter().any(|e| e.status_kind.as_deref() == Some("stress"))
        });
        assert!(
            has_poison_blight,
            "moth_mimicry_B must have poison skill with blight"
        );
        assert!(
            has_crowd_stress,
            "moth_mimicry_B must have stress_crowd skill with stress"
        );
    }
}
