//! Rotten Fruit A — XuanWu boss part (absorb-heal minion that sacrifices self to heal boss).
//!
//! DDGC reference: Eldritch-type boss part from the XuanWu dungeon.
//! Tier 1 base stats: HP 30, DEF 0%, SPD 0, 1 turn/round.
//! Skills: absorbed (kill_self + heal_ally 15).
//!
//! This minion's identity is a sacrificial healer — after 2 rounds alive,
//! it uses `absorbed` to heal the Rotvine Wraith for 15 HP and then dies.
//! The AI brain triggers this when `rounds_alive >= 2`.
//!
//! Game-gaps:
//! - Self-destruct after 2 rounds (absorbed skill) modeled as kill_self status marker
//! - Heal 15 HP modeled as heal_ally status marker (actual heal resolution is game-gap)
//! - rounds_alive AI trigger not modeled
//! - ally_class_target (specifically targets rotvine_wraith) not modeled
//! - All 200% move resist (immune) not modeled in Archetype
//! - Poison Resist 85%, Bleed Resist 100% (immune), Burn Resist 25%,
//!   Frozen Resist 25%, MAGIC_PROT 0 — not modeled in Archetype

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Rotten Fruit A base archetype — tier 1 boss part stats from DDGC data.
///
/// HP 30, no attack (absorbed deals 0 damage), speed 0 (never acts first),
/// defense 0.0 (0% dodge).
/// Support role: heals the Rotvine Wraith then self-destructs.
/// Move Resist 200% (immune), Poison Resist 85%, Bleed Resist 100% (immune) —
/// not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Rotten Fruit A"),
        side: CombatSide::Enemy,
        health: 30.0,
        max_health: 30.0,
        attack: 0.0,
        defense: 0.0,
        speed: 0.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.0,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Rotten Fruit A Skills ────────────────────────────────────────────────────

/// Absorbed — heal ally for 15 HP then self-destruct.
///
/// DDGC reference: dmg 0–0, atk 100%, crit 0%,
/// launch any rank (1234), target @1234 (any ally including rotvine_wraith),
/// heal 15–15, effect "kill_self".
/// AI behavior: fires when `rounds_alive >= 2`.
/// Game-gap: heal 15 HP modeled as heal_ally status marker.
/// Game-gap: kill_self mechanic modeled as status marker.
/// Game-gap: rounds_alive AI trigger not modeled.
/// Game-gap: ally_class_target (specifically rotvine_wraith) not modeled.
pub fn absorbed() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("absorbed"),
        vec![
            EffectNode::apply_status("heal_ally", Some(15)),
            EffectNode::apply_status("kill_self", Some(1)),
        ],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// All 1 Rotten Fruit A skill.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![absorbed()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotten_fruit_a_archetype_is_enemy_eldritch_support() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Rotten Fruit A");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 30.0);
        assert_eq!(arch.max_health, 30.0);
        assert_eq!(arch.speed, 0.0, "rotten_fruit_A has SPD 0 (slow minion)");
        assert_eq!(arch.attack, 0.0, "rotten_fruit_A has no damage skill");
        assert_eq!(arch.defense, 0.0, "rotten_fruit_A has 0% defense");
    }

    #[test]
    fn rotten_fruit_a_absorbed_heals_and_kills_self() {
        let skill = absorbed();
        assert_eq!(skill.id.0, "absorbed");
        let has_heal = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("heal_ally")
        });
        assert!(has_heal, "absorbed must apply heal_ally status");
        let has_kill = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("kill_self")
        });
        assert!(has_kill, "absorbed must apply kill_self status");
    }

    #[test]
    fn rotten_fruit_a_skill_pack_has_one_skill() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 1);
        assert_eq!(pack[0].id.0, "absorbed");
    }

    #[test]
    fn rotten_fruit_a_heal_plus_sacrifice_identity() {
        // The core identity of rotten_fruit_A is a sacrificial healer:
        // it heals the boss for 15 HP then self-destructs.
        let pack = skill_pack();

        let has_heal_and_sacrifice = pack.iter().any(|s| {
            s.id.0 == "absorbed"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("heal_ally")
                })
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("kill_self")
                })
        });

        assert!(has_heal_and_sacrifice, "rotten_fruit_A must heal ally and kill self");
    }
}