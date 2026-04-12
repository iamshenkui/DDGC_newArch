//! Azure Dragon Ball Wind — QingLong boss part (wind ball buff + shuffle).
//!
//! DDGC reference: Eldritch-type boss part from the QingLong dungeon.
//! Tier 1 base stats: HP 55, DEF 40%, PROT 0.8, SPD 10, 1 turn/round.
//! Skills: wind_buff_acc (ACC buff), wind_shuffle (shuffle positions),
//! wind_buff_physic (physical dmg buff).
//!
//! This part's identity is a wind-type support ball that buffs the main
//! boss's accuracy and physical damage, and can shuffle hero positions.
//! It shares a health pool with azure_dragon_ball_thunder (the thunder ball).
//!
//! Game-gaps:
//! - Shared health pool with thunder ball not modeled
//! - Ball-type activation effects not modeled
//! - PROT (0.8), MAGIC_PROT (0.2) not modeled in Archetype
//! - Stun/Move immune (200% resist) not modeled
//! - Position-based targeting (launch 14, target ~@23) not modeled
//! - Shuffle effect (50% chance, repositions heroes) modeled as status marker

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Azure Dragon Ball Wind base archetype — tier 1 boss part stats from DDGC data.
///
/// HP 55, no damage skills (attack 0), speed 10 (faster than main body),
/// defense 0.40 (40% dodge).
/// Support role: buffs main boss accuracy and physical damage, shuffles heroes.
/// PROT 0.8, MAGIC_PROT 0.2, Stun Resist 200% (immune),
/// Move Resist 200% (immune) — all not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Azure Dragon Ball Wind"),
        side: CombatSide::Enemy,
        health: 55.0,
        max_health: 55.0,
        attack: 0.0,
        defense: 0.40,
        speed: 10.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.0,
        dodge: 0.0,
    }
}

// ── Azure Dragon Ball Wind Skills ────────────────────────────────────────

/// Wind Buff ACC — buffs ally accuracy.
///
/// DDGC reference: dmg 0-0, atk 100%, crit 0%,
/// launch ranks 1,4, target ~@23 (AoE ally ranks 2-3),
/// effect "Azure Dragon Ball Buff 1" (ACC5, 2 rounds).
/// Game-gap: ally-targeting simplified to AllAllies.
/// Game-gap: actual +5 ACC modifier modeled as status marker.
pub fn wind_buff_acc() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("wind_buff_acc"),
        vec![EffectNode::apply_status("acc_up", Some(2))],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Wind Shuffle — shuffles enemy positions.
///
/// DDGC reference: dmg 0-0, atk 100%, crit 0%,
/// launch ranks 1,4, target #1234 (AoE enemy ranks 1-4),
/// effect "Azure Dragon Ball Shuffle" (50% chance shuffle target position).
/// Game-gap: shuffle mechanic modeled as status marker only.
/// Game-gap: 50% chance not modeled — status is always applied.
/// Game-gap: AoE vs single-target distinction not modeled — targets AllEnemies.
pub fn wind_shuffle() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("wind_shuffle"),
        vec![EffectNode::apply_status("shuffle", Some(1))],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Wind Buff Physic — buffs ally physical damage.
///
/// DDGC reference: dmg 0-0, atk 100%, crit 0%,
/// launch ranks 1,4, target ~@23 (AoE ally ranks 2-3),
/// effect "Azure Dragon Ball Buff 2" (+10% physical damage, 2 rounds).
/// Game-gap: ally-targeting simplified to AllAllies.
/// Game-gap: actual +10% physical damage modifier modeled as status marker.
pub fn wind_buff_physic() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("wind_buff_physic"),
        vec![EffectNode::apply_status("physic_dmg_up", Some(2))],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// All 3 Azure Dragon Ball Wind skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        wind_buff_acc(),
        wind_shuffle(),
        wind_buff_physic(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn azure_dragon_ball_wind_archetype_is_enemy_eldritch_support() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Azure Dragon Ball Wind");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 55.0);
        assert_eq!(arch.max_health, 55.0);
        assert_eq!(arch.speed, 10.0, "ball_wind is faster than main body (SPD 10)");
        assert_eq!(arch.defense, 0.40, "ball_wind has 40% defense");
        assert_eq!(arch.attack, 0.0, "ball_wind has no damage skills");
    }

    #[test]
    fn azure_dragon_ball_wind_buff_acc_applies_acc_up() {
        let skill = wind_buff_acc();
        assert_eq!(skill.id.0, "wind_buff_acc");
        let has_buff = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("acc_up")
        });
        assert!(has_buff, "wind_buff_acc must apply acc_up status");
    }

    #[test]
    fn azure_dragon_ball_wind_shuffle_applies_shuffle() {
        let skill = wind_shuffle();
        assert_eq!(skill.id.0, "wind_shuffle");
        let has_shuffle = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("shuffle")
        });
        assert!(has_shuffle, "wind_shuffle must apply shuffle status");
    }

    #[test]
    fn azure_dragon_ball_wind_buff_physic_applies_physic_dmg_up() {
        let skill = wind_buff_physic();
        assert_eq!(skill.id.0, "wind_buff_physic");
        let has_buff = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("physic_dmg_up")
        });
        assert!(has_buff, "wind_buff_physic must apply physic_dmg_up status");
    }

    #[test]
    fn azure_dragon_ball_wind_skill_pack_has_three_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 3);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"wind_buff_acc"), "missing wind_buff_acc");
        assert!(ids.contains(&"wind_shuffle"), "missing wind_shuffle");
        assert!(ids.contains(&"wind_buff_physic"), "missing wind_buff_physic");
    }

    #[test]
    fn azure_dragon_ball_wind_buff_plus_shuffle_identity() {
        // The core identity of ball_wind is a support that buffs
        // the main boss's accuracy/physical damage and shuffles heroes.
        let pack = skill_pack();

        let has_acc_buff = pack.iter().any(|s| {
            s.id.0 == "wind_buff_acc"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("acc_up")
                })
        });

        let has_shuffle = pack.iter().any(|s| {
            s.id.0 == "wind_shuffle"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("shuffle")
                })
        });

        let has_physic_buff = pack.iter().any(|s| {
            s.id.0 == "wind_buff_physic"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("physic_dmg_up")
                })
        });

        assert!(has_acc_buff, "ball_wind must have ACC buff");
        assert!(has_shuffle, "ball_wind must have shuffle skill");
        assert!(has_physic_buff, "ball_wind must have physical dmg buff");
    }
}
