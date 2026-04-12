//! Pearlkin Flawed — XuanWu boss part (pull + stress).
//!
//! DDGC reference: Eldritch-type boss minion from the XuanWu dungeon (珍珠人•瑕).
//! Tier 1 base stats: HP 30, DEF 15%, PROT 0.5, SPD 5.
//! Skills: fracture_lure (pull + no damage), shattered_revelation (damage + stress).
//!
//! The Flawed variant pulls heroes from back ranks and applies stress.
//! It is summoned by the Frostvein Clam via nacreous_homunculus and also
//! pre-placed in the boss encounter pack.
//!
//! Game-gaps:
//! - fracture_lure "Pull 2A" modeled as EffectNode::pull(2) — pull is
//!   natively supported by the framework
//! - ~34 targeting (back 2 ranks) approximated as AllEnemies
//! - ~1234 AoE targeting approximated as AllEnemies
//! - PROT (0.5), MAGIC_PROT (0.7), Stun Resist 25%, Poison Resist 100% (immune),
//!   Bleed Resist 100% (immune), Debuff Resist 40%, Move Resist 50%,
//!   Burn Resist 25%, Frozen Resist 100% (immune) not modeled

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Pearlkin Flawed base archetype — boss minion stats from DDGC data.
///
/// HP 30, attack derived from shattered_revelation skill (magic_dmg 2–4 avg 3.0),
/// speed 5, defense 0.15 (15% dodge).
/// Controller role: pulls heroes from back ranks and applies stress.
/// Crit 5% from shattered_revelation.
/// PROT 0.5, MAGIC_PROT 0.7 — not modeled.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Pearlkin Flawed"),
        side: CombatSide::Enemy,
        health: 30.0,
        max_health: 30.0,
        attack: 3.0,
        defense: 0.15,
        speed: 5.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.05,
        dodge: 0.0,
    }
}

// ── Pearlkin Flawed Skills ──────────────────────────────────────────────────

/// Fracture Lure — ranged pull (no damage).
///
/// DDGC reference: magic_dmg 0–0, atk 85%, crit 0%,
/// launch ranks 1,2, target ~34 (back 2 ranks),
/// effect "Pull 2A" (pull target forward 2 positions).
/// Game-gap: ~34 targeting (back 2 ranks) approximated as AllEnemies.
pub fn fracture_lure() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("fracture_lure"),
        vec![EffectNode::pull(2)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Shattered Revelation — ranged magic damage + stress.
///
/// DDGC reference: magic_dmg 2–4, atk 85%, crit 5%,
/// launch ranks 1,2, target ~1234 (AoE all ranks),
/// effect "Stress 2".
/// Game-gap: ~1234 AoE targeting approximated as AllEnemies.
pub fn shattered_revelation() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("shattered_revelation"),
        vec![
            EffectNode::damage(3.0),
            EffectNode::apply_status("stress", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 2 Pearlkin Flawed skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![fracture_lure(), shattered_revelation()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pearlkin_flawed_archetype_is_enemy_eldritch_controller() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Pearlkin Flawed");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 30.0);
        assert_eq!(arch.max_health, 30.0);
        assert_eq!(arch.speed, 5.0);
        assert_eq!(arch.defense, 0.15, "pearlkin_flawed has 15% defense");
        assert_eq!(arch.attack, 3.0, "attack from shattered_revelation avg 2-4");
        assert_eq!(arch.crit_chance, 0.05, "crit 5% from shattered_revelation");
    }

    #[test]
    fn pearlkin_flawed_fracture_lure_applies_pull() {
        let skill = fracture_lure();
        assert_eq!(skill.id.0, "fracture_lure");
        let has_pull = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Pull)
        });
        assert!(has_pull, "fracture_lure must apply pull");
    }

    #[test]
    fn pearlkin_flawed_shattered_revelation_applies_damage_and_stress() {
        let skill = shattered_revelation();
        assert_eq!(skill.id.0, "shattered_revelation");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "shattered_revelation must deal damage");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "shattered_revelation must apply stress status");
    }

    #[test]
    fn pearlkin_flawed_skill_pack_has_two_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 2);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"fracture_lure"), "missing fracture_lure");
        assert!(ids.contains(&"shattered_revelation"), "missing shattered_revelation");
    }
}
