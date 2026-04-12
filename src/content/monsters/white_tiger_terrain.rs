//! White Tiger Terrain — BaiHu boss terrain placeholder (immobile positional anchor).
//!
//! DDGC reference: Corpse-type terrain unit from the BaiHu dungeon.
//! Stats: HP 15, DEF 0%, PROT 1.0 (immune to physical), SPD 0,
//! 0 turns/round, all resists 200% (fully immune).
//!
//! This unit is a purely positional anchor — the tiger's `jump` skill
//! repositions to the terrain tile's rank slot. It cannot be targeted by
//! enemies or randomly selected, and never acts (SPD 0, 0 turns/round).
//!
//! Game-gaps:
//! - All 200% resists (stun/poison/bleed/debuff/move/burn/frozen) not modeled
//! - PROT 1.0 (immune to physical) not modeled
//! - MAGIC_PROT 1.0 (immune to magic) not modeled
//! - `can_be_random_target False` not modeled
//! - `is_valid_enemy_target False` not modeled
//! - `is_valid_friendly_target True` not modeled
//! - `can_be_summon_rank True` not modeled
//! - `does_count_as_monster_size_for_monster_brain False` not modeled

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// White Tiger Terrain base archetype — positional anchor stats from DDGC data.
///
/// HP 15, no attack (SPD 0, never acts), defense 0.0 (0% dodge).
/// This unit exists only as a positional anchor for the tiger's jump skill.
/// All resistances 200% (immune to everything) — not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("White Tiger Terrain"),
        side: CombatSide::Enemy,
        health: 15.0,
        max_health: 15.0,
        attack: 0.0,
        defense: 0.0,
        speed: 0.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.0,
        dodge: 0.0,
    }
}

// ── White Tiger Terrain Skills ────────────────────────────────────────────

/// Occupy — placeholder skill for terrain unit (never acts, never uses skills).
///
/// The terrain unit has no active skills in DDGC (0 turns/round).
/// This placeholder skill exists to satisfy the framework's requirement
/// that every family has at least one skill ID in the registry and
/// that skills have at least one effect. The "occupy" status marker
/// carries no game-mechanical effect.
pub fn occupy() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("occupy"),
        vec![EffectNode::apply_status("occupy", None)],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// All 1 White Tiger Terrain skill (placeholder).
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![occupy()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_tiger_terrain_archetype_is_enemy_corpse_support() {
        let arch = archetype();
        assert_eq!(arch.name.0, "White Tiger Terrain");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 15.0);
        assert_eq!(arch.max_health, 15.0);
        assert_eq!(arch.speed, 0.0, "terrain never acts (SPD 0)");
        assert_eq!(arch.attack, 0.0, "terrain has no attack");
        assert_eq!(arch.defense, 0.0, "terrain has 0% defense");
    }

    #[test]
    fn white_tiger_terrain_occupy_is_placeholder_skill() {
        let skill = occupy();
        assert_eq!(skill.id.0, "occupy");
        // occupy has a single status marker to satisfy validation
        assert_eq!(skill.effects.len(), 1, "occupy should have 1 placeholder effect");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "occupy targets self only"
        );
    }

    #[test]
    fn white_tiger_terrain_skill_pack_has_one_skill() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 1);
        assert_eq!(pack[0].id.0, "occupy");
    }

    #[test]
    fn white_tiger_terrain_immobile_anchor_identity() {
        // The core identity of white_tiger_terrain is a purely positional
        // anchor — it never acts and has no combat skills.
        let arch = archetype();
        assert_eq!(arch.speed, 0.0, "terrain must have 0 speed (never acts)");
        assert_eq!(arch.attack, 0.0, "terrain must have 0 attack (no combat)");

        let pack = skill_pack();
        assert_eq!(pack.len(), 1, "terrain must have exactly 1 placeholder skill");
        assert_eq!(pack[0].effects.len(), 1, "terrain skill must have 1 placeholder effect");
    }
}