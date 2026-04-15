//! Rotten Fruit C — XuanWu boss part (AoE stress explosion minion).
//!
//! DDGC reference: Eldritch-type boss part from the XuanWu dungeon.
//! Tier 1 base stats: HP 30, DEF 0%, SPD 0, 1 turn/round.
//! Skills: fruit_stress_explosion (AoE stress 2 + kill_self).
//!
//! This minion's identity is a stress bomb — after 2 rounds alive,
//! it explodes inflicting Stress 2 on all heroes and then dies.
//! Shares the rotten_fruit_B AI brain (rounds_alive >= 2 trigger).
//!
//! Game-gaps:
//! - Self-destruct after 2 rounds modeled as kill_self status marker
//! - rounds_alive AI trigger not modeled
//! - random_target AI targeting not modeled
//! - All 200% move resist (immune) not modeled in Archetype
//! - Poison Resist 85%, Bleed Resist 100% (immune), Burn Resist 25%,
//!   Frozen Resist 25%, MAGIC_PROT 0 — not modeled in Archetype

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Rotten Fruit C base archetype — tier 1 boss part stats from DDGC data.
///
/// HP 30, no attack (fruit_stress_explosion deals 0 damage),
/// speed 0 (never acts first), defense 0.0 (0% dodge).
/// Controller role: one-time AoE stress explosion that then dies.
/// Move Resist 200% (immune), Bleed Resist 100% (immune) — not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Rotten Fruit C"),
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

// ── Rotten Fruit C Skills ─────────────────────────────────────────────────────

/// Fruit Stress Explosion — AoE stress then self-destruct.
///
/// DDGC reference: dmg 0–0, atk 100%, crit 0%,
/// launch any rank (1234), target ~1234 (AoE all enemy ranks),
/// effects "Stress 2" + "kill_self".
/// AI behavior: fires when `rounds_alive >= 2` (shares rotten_fruit_B brain).
/// Game-gap: kill_self mechanic modeled as status marker.
/// Game-gap: rounds_alive AI trigger not modeled.
pub fn fruit_stress_explosion() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("fruit_stress_explosion"),
        vec![
            EffectNode::apply_status("stress", Some(2)),
            EffectNode::apply_status("kill_self", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 1 Rotten Fruit C skill.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![fruit_stress_explosion()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotten_fruit_c_archetype_is_enemy_eldritch_controller() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Rotten Fruit C");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 30.0);
        assert_eq!(arch.max_health, 30.0);
        assert_eq!(arch.speed, 0.0, "rotten_fruit_C has SPD 0 (slow minion)");
        assert_eq!(arch.attack, 0.0, "rotten_fruit_C has no damage");
        assert_eq!(arch.defense, 0.0, "rotten_fruit_C has 0% defense");
    }

    #[test]
    fn rotten_fruit_c_fruit_stress_explosion_applies_stress_and_kills_self() {
        let skill = fruit_stress_explosion();
        assert_eq!(skill.id.0, "fruit_stress_explosion");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "fruit_stress_explosion must apply stress status");
        let has_kill = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("kill_self")
        });
        assert!(has_kill, "fruit_stress_explosion must apply kill_self status");
    }

    #[test]
    fn rotten_fruit_c_skill_pack_has_one_skill() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 1);
        assert_eq!(pack[0].id.0, "fruit_stress_explosion");
    }

    #[test]
    fn rotten_fruit_c_stress_plus_sacrifice_identity() {
        // The core identity of rotten_fruit_C is a stress bomb:
        // it inflicts AoE stress on all heroes then self-destructs.
        let pack = skill_pack();

        let has_stress_and_sacrifice = pack.iter().any(|s| {
            s.id.0 == "fruit_stress_explosion"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stress")
                })
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("kill_self")
                })
        });

        assert!(has_stress_and_sacrifice, "rotten_fruit_C must apply AoE stress and kill self");
    }
}