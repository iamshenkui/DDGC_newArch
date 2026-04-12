//! Mahjong White — ZhuQue boss minion (stress + pull control).
//!
//! DDGC reference: Eldritch-type minion from the ZhuQue dungeon.
//! Stats: HP 20, DEF 10%, PROT 0, SPD 5, 1 turn/round.
//! Skills: joyful_bonus, triple_tile_invite.
//!
//! The Mahjong White is a summoned minion that applies stress with
//! `joyful_bonus` (Stress 2, 0 damage) and pulls targets with
//! `triple_tile_invite` (Pull 1 from positions 2/3/4). Its AI brain
//! heavily prioritizes `triple_tile_invite` against marked targets
//! (weight 1000), and equally weights both skills against non-marked
//! targets (0.5 each). It is immune to bleed and poison.
//!
//! Game-gaps:
//! - Stress effect (joyful_bonus) modeled as "stress" status marker only
//! - Pull 1A effect (triple_tile_invite) modeled as pull(1) on AllEnemies
//!   — position-based targeting (launch 1234, target 234) approximated
//! - AI brain's marked-target priority (weight 1000 for triple_tile_invite)
//!   not modeled
//! - 0% crit on both skills (vs 5% on other mahjong) modeled as 0% crit_chance
//! - PROT 0, MAGIC_PROT 0.5 not modeled in Archetype
//! - Stun Resist 50%, Poison Resist 100% (immune), Bleed Resist 100% (immune),
//!   Debuff Resist 40%, Move Resist 70%, Burn Resist 35%, Frozen Resist 35%
//!   not modeled

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Mahjong White base archetype — minion stats from DDGC data.
///
/// HP 20, no attack (both skills deal 0 damage), speed 5,
/// defense 0.10 (10% dodge).
/// Controller role: applies stress and pulls targets to disrupt formations.
/// Crit 0% — both skills have 0% crit (unlike mahjong_red/green).
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Mahjong White"),
        side: CombatSide::Enemy,
        health: 20.0,
        max_health: 20.0,
        attack: 0.0,
        defense: 0.10,
        speed: 5.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.0,
        dodge: 0.0,
    }
}

// ── Mahjong White Skills ─────────────────────────────────────────────────────

/// Joyful Bonus — stress to any position, no damage.
///
/// DDGC reference: dmg 0–0, atk 85%, crit 0%,
/// launch ranks 1,2,3,4, target 1234 (any rank),
/// effect "Stress 2".
pub fn joyful_bonus() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("joyful_bonus"),
        vec![EffectNode::apply_status("stress", Some(2))],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Triple Tile Invite — pull target forward.
///
/// DDGC reference: dmg 0–0, atk 85%, crit 0%,
/// launch ranks 1,2,3,4, target 234 (front 3 ranks),
/// effect "Pull 1A".
/// Game-gap: target 234 (front 3 ranks) approximated as AllEnemies.
pub fn triple_tile_invite() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("triple_tile_invite"),
        vec![EffectNode::pull(1)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 2 Mahjong White skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![joyful_bonus(), triple_tile_invite()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mahjong_white_archetype_is_enemy_eldritch_controller() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Mahjong White");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 20.0);
        assert_eq!(arch.max_health, 20.0);
        assert_eq!(arch.speed, 5.0, "mahjong_white has SPD 5");
        assert_eq!(arch.defense, 0.10, "mahjong_white has 10% defense");
        assert_eq!(arch.attack, 0.0, "mahjong_white has 0 attack (both skills deal 0 dmg)");
        assert_eq!(arch.crit_chance, 0.0, "crit 0% from all skills");
    }

    #[test]
    fn mahjong_white_joyful_bonus_applies_stress() {
        let skill = joyful_bonus();
        assert_eq!(skill.id.0, "joyful_bonus");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "joyful_bonus must apply stress status");
        let stress_effect = skill.effects.iter().find(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(stress_effect.is_some(), "joyful_bonus must have stress effect");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllEnemies",
            "joyful_bonus targets all enemies (1234)"
        );
    }

    #[test]
    fn mahjong_white_triple_tile_invite_applies_pull() {
        let skill = triple_tile_invite();
        assert_eq!(skill.id.0, "triple_tile_invite");
        let has_pull = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Pull)
        });
        assert!(has_pull, "triple_tile_invite must apply pull");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllEnemies",
            "triple_tile_invite targets all enemies (234 approx)"
        );
    }

    #[test]
    fn mahjong_white_skill_pack_has_two_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 2);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"joyful_bonus"), "missing joyful_bonus");
        assert!(ids.contains(&"triple_tile_invite"), "missing triple_tile_invite");
    }

    #[test]
    fn mahjong_white_stress_plus_pull_identity() {
        // The core identity of mahjong_white is a control minion that
        // applies stress and pulls targets to disrupt formations.
        let pack = skill_pack();

        let has_stress = pack.iter().any(|s| {
            s.id.0 == "joyful_bonus"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stress")
                })
        });

        let has_pull = pack.iter().any(|s| {
            s.id.0 == "triple_tile_invite"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Pull)
                })
        });

        assert!(has_stress, "mahjong_white must have stress skill");
        assert!(has_pull, "mahjong_white must have pull skill");
    }
}
