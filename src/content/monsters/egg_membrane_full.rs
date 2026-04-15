//! Egg Membrane Full — XuanWu boss part (captor vessel, holding hero, life-linked).
//!
//! DDGC reference: Cauldron-type boss part from the XuanWu dungeon.
//! Stats: HP 10, DEF 22.5%, PROT 0, SPD 2, 0 turns/round,
//! all resists 245% (fully immune to status), MAGIC_PROT 0.
//!
//! This unit is a captor vessel that holds a captured hero. It is
//! intentionally fragile (HP 10) so heroes can destroy it to release
//! their captured ally. It deals 5–6 damage per turn to the captured
//! hero passively. It transforms back to egg_membrane_empty on death
//! or when the captured hero reaches death's door.
//!
//! Game-gaps:
//! - All 245% resists (stun/poison/bleed/debuff/move/burn/frozen) not modeled
//! - PROT 0 (vulnerable to physical) — implicitly modeled (no PROT override)
//! - MAGIC_PROT 0 (vulnerable to magic) — implicitly modeled
//! - captor_full mechanic (transforms back to egg_membrane_empty on death) not modeled
//! - release_on_death not modeled
//! - release_on_prisoner_at_deaths_door not modeled
//! - per_turn_damage_range 5–6 (passive DoT to captured hero) not modeled
//! - life_link to necrodrake_embryosac not modeled
//! - can_be_missed False not modeled
//! - DurationBrokenCapture / DurationUnBrokenCapture effects not modeled
//! - 0 turns per round not modeled in Archetype

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Egg Membrane Full base archetype — captor vessel (holding hero) stats from DDGC data.
///
/// HP 10 (intentionally fragile — killing it releases the captured hero),
/// no attack (never acts), speed 2 (very slow),
/// defense 0.225 (22.5% dodge).
/// This unit holds a captured hero and deals 5–6 passive damage per turn.
/// All resistances 245% (immune to status) — not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Egg Membrane Full"),
        side: CombatSide::Enemy,
        health: 10.0,
        max_health: 10.0,
        attack: 0.0,
        defense: 0.225,
        speed: 2.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.0,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Egg Membrane Full Skills ─────────────────────────────────────────────────

/// Captor Full — placeholder skill for captor vessel holding a hero.
///
/// The egg_membrane_full has no active combat skills in DDGC (0 turns/round).
/// This placeholder skill exists to satisfy the framework's requirement
/// that every family has at least one skill ID in the registry and
/// that skills have at least one effect. The "captor_full" status marker
/// carries no game-mechanical effect but captures the identity of this
/// unit as a captor vessel holding a hero.
///
/// The passive DoT (5–6 damage per turn to captured hero) is not modeled
/// as a skill effect — it is a DDGC engine-level mechanic triggered by
/// the `per_turn_damage_range` field.
pub fn captor_full() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("captor_full"),
        vec![EffectNode::apply_status("captor_full", None)],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// All 1 Egg Membrane Full skill (placeholder).
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![captor_full()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn egg_membrane_full_archetype_is_enemy_cauldron() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Egg Membrane Full");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 10.0, "egg_membrane_full has 10 HP (fragile)");
        assert_eq!(arch.max_health, 10.0);
        assert_eq!(arch.speed, 2.0, "egg_membrane_full has SPD 2");
        assert_eq!(arch.attack, 0.0, "egg_membrane_full has no attack");
        assert_eq!(arch.defense, 0.225, "egg_membrane_full has 22.5% defense");
    }

    #[test]
    fn egg_membrane_full_captor_full_is_placeholder_skill() {
        let skill = captor_full();
        assert_eq!(skill.id.0, "captor_full");
        assert_eq!(skill.effects.len(), 1, "captor_full should have 1 placeholder effect");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "captor_full targets self only"
        );
    }

    #[test]
    fn egg_membrane_full_skill_pack_has_one_skill() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 1);
        assert_eq!(pack[0].id.0, "captor_full");
    }

    #[test]
    fn egg_membrane_full_fragile_captor_identity() {
        // The core identity of egg_membrane_full is a fragile captor vessel
        // that holds a captured hero — it has only 10 HP so destroying it
        // releases the hero.
        let arch = archetype();
        assert_eq!(arch.health, 10.0, "captor vessel (full) must be fragile (10 HP)");
        assert_eq!(arch.attack, 0.0, "captor vessel must have 0 attack (no combat)");

        let pack = skill_pack();
        assert_eq!(pack.len(), 1, "captor vessel must have exactly 1 placeholder skill");
        assert_eq!(pack[0].effects.len(), 1, "captor vessel skill must have 1 placeholder effect");
    }
}
