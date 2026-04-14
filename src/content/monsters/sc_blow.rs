//! SC Blow — XuanWu boss minion (wind instrument, stress pressure).
//!
//! DDGC reference: Eldritch-type minion from the XuanWu dungeon.
//! Tier 1 base stats: HP 20, DEF 10%, PROT 0.5, SPD 5, 1 turn/round.
//! Skills: grindbone_lament, move.
//!
//! SC Blow is a wind-instrument minion summoned by the Scorchthroat Chanteuse.
//! It applies stress to heroes from ranged positions with grindbone_lament.
//!
//! Game-gaps:
//! - Position-based targeting (launch 3,4, target 1234) approximated as AllEnemies
//! - Movement skill (move 1→0) approximated as push(1)
//! - PROT (0.5), MAGIC_PROT (0.5) not modeled in Archetype
//! - Stun Resist 50%, Poison Resist 25%, Bleed Resist 50%, Debuff Resist 40%,
//!   Move Resist 50%, Burn Resist 75%, Frozen Resist 25% not modeled

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// SC Blow base archetype — tier 1 boss minion stats from DDGC data.
///
/// HP 20, weapon damage 0 (stress-only attacker),
/// speed 5, defense 0.10 (10% dodge).
/// Skirmisher role: applies stress from ranged positions.
/// No crit chance (stress-only skill).
/// PROT 0.5, MAGIC_PROT 0.5 — not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("SC Blow"),
        side: CombatSide::Enemy,
        health: 20.0,
        max_health: 20.0,
        attack: 0.0,
        defense: 0.10,
        speed: 5.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.0,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── SC Blow Skills ───────────────────────────────────────────────────────────

/// Grindbone Lament — ranged stress attack.
///
/// DDGC reference: dmg 0–0, atk 85%, crit 0%,
/// launch ranks 3,4, target 1234 (any enemy),
/// effects "Stress 2".
/// Game-gap: position-based targeting approximated as AllEnemies.
pub fn grindbone_lament() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("grindbone_lament"),
        vec![EffectNode::apply_status("stress", Some(2))],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Move — self-reposition skill.
///
/// DDGC reference: dmg 0–0, atk 0%, crit 0%,
/// launch ranks 1,2, target @23 (ally ranks 2-3), .move 1 0.
/// Game-gap: position-based movement approximated as push(1).
pub fn move_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("move"),
        vec![EffectNode::push(1)],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// All 2 SC Blow skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![grindbone_lament(), move_skill()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sc_blow_archetype_is_enemy_eldritch_skirmisher() {
        let arch = archetype();
        assert_eq!(arch.name.0, "SC Blow");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 20.0);
        assert_eq!(arch.max_health, 20.0);
        assert_eq!(arch.speed, 5.0, "sc_blow has SPD 5");
        assert_eq!(arch.defense, 0.10, "sc_blow has 10% defense");
        assert_eq!(arch.attack, 0.0, "sc_blow has no weapon damage (stress-only)");
    }

    #[test]
    fn sc_blow_grindbone_lament_applies_stress() {
        let skill = grindbone_lament();
        assert_eq!(skill.id.0, "grindbone_lament");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "grindbone_lament must apply stress status");
    }

    #[test]
    fn sc_blow_skill_pack_has_two_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 2);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"grindbone_lament"), "missing grindbone_lament");
        assert!(ids.contains(&"move"), "missing move");
    }

    #[test]
    fn sc_blow_stress_identity() {
        let pack = skill_pack();
        let has_stress = pack.iter().any(|s| {
            s.effects.iter().any(|e| {
                matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                    && e.status_kind.as_deref() == Some("stress")
            })
        });
        assert!(has_stress, "sc_blow must have stress skill");
    }
}
