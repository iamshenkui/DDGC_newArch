//! Vegetable — XuanWu boss part (summoned minion of Skeletal Tiller).
//!
//! DDGC reference: Eldritch-type summoned minion from the XuanWu dungeon.
//! Tier 1 base stats: HP 20, DEF 10%, PROT 0.5, SPD 6, 1 turn/round.
//! Skills: briar_intimidation, crop_rot_claw, move.
//!
//! Vegetables are low-HP minions summoned by the Skeletal Tiller that
//! apply frozen and speed debuffs while the boss re-summons them
//! after they die.
//!
//! Game-gaps:
//! - Position-based targeting (launch 12, target 12/1234/@23)
//!   approximated as AllEnemies/SelfOnly
//! - magic_dmg on briar_intimidation not modeled (treated as normal damage)
//! - "Target Speed -1_2" from briar_intimidation modeled as
//!   apply_status("speed_down", Some(2)) — the specific speed reduction is a game-gap
//! - Push 2A on briar_intimidation modeled as push(2)
//! - Self-movement on move skill (move 0 1) approximated as push(1)
//! - PROT (0.5), MAGIC_PROT (0.5) not modeled in Archetype
//! - Stun Resist 100% (immune), Poison Resist 100% (immune),
//!   Bleed Resist 100% (immune), Debuff Resist 40%, Move Resist 60%,
//!   Burn Resist 35%, Frozen Resist 35% not modeled
//! - skill name "crop_rot_claw" prefixed with "vegetable_" to avoid
//!   ContentPack collision with skeletal_tiller's crop_rot_claw

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Vegetable base archetype — tier 1 boss part stats from DDGC data.
///
/// HP 20, weapon damage derived from briar_intimidation skill (4–8 avg 6),
/// speed 6, defense 0.10 (10% dodge).
/// Skirmisher role: applies speed debuffs and frozen while being disposable.
/// Crit 5% from briar_intimidation/crop_rot_claw.
/// PROT 0.5, MAGIC_PROT 0.5, Stun Resist 100% (immune),
/// Poison Resist 100% (immune), Bleed Resist 100% (immune),
/// Debuff Resist 40%, Move Resist 60%, Burn Resist 35%, Frozen Resist 35%
/// — all not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Vegetable"),
        side: CombatSide::Enemy,
        health: 20.0,
        max_health: 20.0,
        attack: 6.0,
        defense: 0.10,
        speed: 6.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.05,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Vegetable Skills ──────────────────────────────────────────────────────────

/// Briar Intimidation — magic damage + speed debuff + push.
///
/// DDGC reference: magic_dmg 4–8, atk 85%, crit 5%,
/// launch 12 (front ranks), target 12 (enemy front ranks),
/// effects "Target Speed -1_2" + "Push 2A".
/// AI behavior: 50% selection weight (equal to crop_rot_claw).
/// Game-gap: magic damage type not modeled — treated as normal damage.
/// Game-gap: "Target Speed -1_2" modeled as apply_status("speed_down", Some(2)).
/// Game-gap: position-based targeting (ranks 1-2) approximated as AllEnemies.
pub fn briar_intimidation() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("briar_intimidation"),
        vec![
            EffectNode::damage(6.0),
            EffectNode::apply_status("speed_down", Some(2)),
            EffectNode::push(2),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Crop Rot Claw — melee damage + frozen.
///
/// DDGC reference: dmg 2–4, atk 85%, crit 5%,
/// launch 12 (front ranks), target 1234 (any enemy rank),
/// effect "Frozen 2".
/// AI behavior: 50% selection weight (equal to briar_intimidation).
/// Note: skill name prefixed with "vegetable_" to avoid ContentPack collision
/// with skeletal_tiller's crop_rot_claw (registered as "tiller_crop_rot_claw").
/// Game-gap: position-based targeting approximated as AllEnemies.
pub fn crop_rot_claw() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("vegetable_crop_rot_claw"),
        vec![
            EffectNode::damage(3.0),
            EffectNode::apply_status("frozen", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Move — self-reposition skill.
///
/// DDGC reference: dmg 0–0, atk 0%, crit 0%,
/// launch 34 (back ranks), target @23 (friendly ranks 2-3), .move 0 1.
/// Game-gap: position-based movement approximated as push(1).
pub fn move_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("vegetable_move"),
        vec![EffectNode::push(1)],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// All 3 Vegetable skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        briar_intimidation(),
        crop_rot_claw(),
        move_skill(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vegetable_archetype_is_enemy_eldritch_skirmisher() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Vegetable");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 20.0);
        assert_eq!(arch.max_health, 20.0);
        assert_eq!(arch.speed, 6.0, "vegetable has SPD 6");
        assert_eq!(arch.defense, 0.10, "vegetable has 10% defense");
        assert_eq!(arch.attack, 6.0, "attack from briar_intimidation avg 4-8");
        assert_eq!(arch.crit_chance, 0.05, "crit 5% from briar_intimidation/crop_rot_claw");
    }

    #[test]
    fn vegetable_briar_intimidation_applies_speed_down_and_push() {
        let skill = briar_intimidation();
        assert_eq!(skill.id.0, "briar_intimidation");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "briar_intimidation must deal damage");
        let has_speed_down = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("speed_down")
        });
        assert!(has_speed_down, "briar_intimidation must apply speed_down status");
        let has_push = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Push)
        });
        assert!(has_push, "briar_intimidation must push");
    }

    #[test]
    fn vegetable_crop_rot_claw_applies_frozen() {
        let skill = crop_rot_claw();
        assert_eq!(skill.id.0, "vegetable_crop_rot_claw");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "crop_rot_claw must deal damage");
        let has_frozen = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("frozen")
        });
        assert!(has_frozen, "crop_rot_claw must apply frozen status");
    }

    #[test]
    fn vegetable_skill_pack_has_three_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 3);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"briar_intimidation"), "missing briar_intimidation");
        assert!(ids.contains(&"vegetable_crop_rot_claw"), "missing vegetable_crop_rot_claw");
        assert!(ids.contains(&"vegetable_move"), "missing vegetable_move");
    }

    #[test]
    fn vegetable_frozen_plus_speed_debuff_identity() {
        // The core identity of vegetable is a disposable minion that
        // applies frozen and speed debuffs while being cheap to re-summon.
        let pack = skill_pack();

        let has_frozen = pack.iter().any(|s| {
            s.effects.iter().any(|e| {
                matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                    && e.status_kind.as_deref() == Some("frozen")
            })
        });

        let has_speed_down = pack.iter().any(|s| {
            s.effects.iter().any(|e| {
                matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                    && e.status_kind.as_deref() == Some("speed_down")
            })
        });

        assert!(has_frozen, "vegetable must have frozen skill");
        assert!(has_speed_down, "vegetable must have speed_down skill");
    }
}
