//! Rotten Fruit B — XuanWu boss part (AoE explosion minion).
//!
//! DDGC reference: Eldritch-type boss part from the XuanWu dungeon.
//! Tier 1 base stats: HP 30, DEF 0%, SPD 0, 1 turn/round.
//! Skills: fruit_explosion (AoE damage 7–10 + kill_self).
//!
//! This minion's identity is a living bomb — after 2 rounds alive,
//! it explodes dealing 7–10 AoE physical damage to all heroes and then dies.
//! The AI brain triggers this when `rounds_alive >= 2`.
//!
//! Game-gaps:
//! - Self-destruct after 2 rounds (fruit_explosion skill) modeled as kill_self status marker
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

/// Rotten Fruit B base archetype — tier 1 boss part stats from DDGC data.
///
/// HP 30, attack derived from fruit_explosion damage (7–10 avg 8.5),
/// speed 0 (never acts first), defense 0.0 (0% dodge).
/// Skirmisher role: one-time AoE explosion that damages all heroes then dies.
/// Move Resist 200% (immune), Bleed Resist 100% (immune) — not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Rotten Fruit B"),
        side: CombatSide::Enemy,
        health: 30.0,
        max_health: 30.0,
        attack: 8.5,
        defense: 0.0,
        speed: 0.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.0,
        dodge: 0.0,
    }
}

// ── Rotten Fruit B Skills ─────────────────────────────────────────────────────

/// Fruit Explosion — AoE damage then self-destruct.
///
/// DDGC reference: dmg 7–10, atk 100%, crit 0%,
/// launch any rank (1234), target ~1234 (AoE all enemy ranks),
/// effect "kill_self".
/// AI behavior: fires when `rounds_alive >= 2`.
/// Game-gap: kill_self mechanic modeled as status marker.
/// Game-gap: rounds_alive AI trigger not modeled.
pub fn fruit_explosion() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("fruit_explosion"),
        vec![
            EffectNode::damage(8.5),
            EffectNode::apply_status("kill_self", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 1 Rotten Fruit B skill.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![fruit_explosion()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotten_fruit_b_archetype_is_enemy_eldritch_skirmisher() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Rotten Fruit B");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 30.0);
        assert_eq!(arch.max_health, 30.0);
        assert_eq!(arch.speed, 0.0, "rotten_fruit_B has SPD 0 (slow minion)");
        assert_eq!(arch.attack, 8.5, "attack from fruit_explosion avg 7-10");
        assert_eq!(arch.defense, 0.0, "rotten_fruit_B has 0% defense");
    }

    #[test]
    fn rotten_fruit_b_fruit_explosion_deals_damage_and_kills_self() {
        let skill = fruit_explosion();
        assert_eq!(skill.id.0, "fruit_explosion");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "fruit_explosion must deal damage");
        let has_kill = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("kill_self")
        });
        assert!(has_kill, "fruit_explosion must apply kill_self status");
    }

    #[test]
    fn rotten_fruit_b_skill_pack_has_one_skill() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 1);
        assert_eq!(pack[0].id.0, "fruit_explosion");
    }

    #[test]
    fn rotten_fruit_b_explosion_plus_sacrifice_identity() {
        // The core identity of rotten_fruit_B is a living bomb:
        // it explodes dealing AoE damage then self-destructs.
        let pack = skill_pack();

        let has_explosion = pack.iter().any(|s| {
            s.id.0 == "fruit_explosion"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Damage)
                })
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("kill_self")
                })
        });

        assert!(has_explosion, "rotten_fruit_B must deal AoE damage and kill self");
    }
}