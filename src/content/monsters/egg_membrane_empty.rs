//! Egg Membrane Empty — XuanWu boss part (captor vessel, empty, life-linked).
//!
//! DDGC reference: Cauldron-type boss part from the XuanWu dungeon.
//! Stats: HP 210, DEF 20%, PROT 1.0 (immune to physical), SPD 2,
//! 0 turns/round, all resists 245% (fully immune).
//!
//! This unit is a captor vessel that starts empty. When the Necrodrake
//! Embryosac uses `untimely_progeny`, a hero is captured and this unit
//! transforms into `egg_membrane_full`. It never acts (0 turns/round) and
//! is life-linked to the Necrodrake Embryosac.
//!
//! Game-gaps:
//! - All 245% resists (stun/poison/bleed/debuff/move/burn/frozen) not modeled
//! - PROT 1.0 (immune to physical) not modeled
//! - MAGIC_PROT 1.0 (immune to magic) not modeled
//! - captor_empty mechanic (transforms to egg_membrane_full on capture) not modeled
//! - life_link to necrodrake_embryosac not modeled
//! - can_be_missed False not modeled
//! - 0 turns per round not modeled in Archetype

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Egg Membrane Empty base archetype — captor vessel stats from DDGC data.
///
/// HP 210, no attack (never acts), speed 2 (very slow),
/// defense 0.20 (20% dodge).
/// This unit exists as a captor vessel that holds no hero yet.
/// All resistances 245% (immune to everything) — not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Egg Membrane Empty"),
        side: CombatSide::Enemy,
        health: 210.0,
        max_health: 210.0,
        attack: 0.0,
        defense: 0.20,
        speed: 2.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.0,
        dodge: 0.0,
    }
}

// ── Egg Membrane Empty Skills ────────────────────────────────────────────────

/// Captor Empty — placeholder skill for captor vessel (never acts, no combat skills).
///
/// The egg_membrane_empty has no active combat skills in DDGC (0 turns/round).
/// This placeholder skill exists to satisfy the framework's requirement
/// that every family has at least one skill ID in the registry and
/// that skills have at least one effect. The "captor_empty" status marker
/// carries no game-mechanical effect.
pub fn captor_empty() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("captor_empty"),
        vec![EffectNode::apply_status("captor_empty", None)],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// All 1 Egg Membrane Empty skill (placeholder).
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![captor_empty()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn egg_membrane_empty_archetype_is_enemy_cauldron() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Egg Membrane Empty");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 210.0);
        assert_eq!(arch.max_health, 210.0);
        assert_eq!(arch.speed, 2.0, "egg_membrane_empty has SPD 2");
        assert_eq!(arch.attack, 0.0, "egg_membrane_empty has no attack");
        assert_eq!(arch.defense, 0.20, "egg_membrane_empty has 20% defense");
    }

    #[test]
    fn egg_membrane_empty_captor_empty_is_placeholder_skill() {
        let skill = captor_empty();
        assert_eq!(skill.id.0, "captor_empty");
        assert_eq!(skill.effects.len(), 1, "captor_empty should have 1 placeholder effect");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "captor_empty targets self only"
        );
    }

    #[test]
    fn egg_membrane_empty_skill_pack_has_one_skill() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 1);
        assert_eq!(pack[0].id.0, "captor_empty");
    }

    #[test]
    fn egg_membrane_empty_captor_vessel_identity() {
        // The core identity of egg_membrane_empty is a captor vessel that
        // starts empty and transforms to egg_membrane_full when a hero is captured.
        let arch = archetype();
        assert_eq!(arch.attack, 0.0, "captor vessel must have 0 attack (no combat)");

        let pack = skill_pack();
        assert_eq!(pack.len(), 1, "captor vessel must have exactly 1 placeholder skill");
        assert_eq!(pack[0].effects.len(), 1, "captor vessel skill must have 1 placeholder effect");
    }
}
