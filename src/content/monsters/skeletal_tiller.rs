//! Skeletal Tiller — XuanWu boss (summon-vegetable + frozen-stress control).
//!
//! DDGC reference: Eldritch-type boss from the XuanWu dungeon.
//! Tier 1 base stats: HP 150, DEF 10%, PROT 0.6, SPD 6, 2 turns/round.
//! Skills: bone_reforge, famine_reaping, scarecrow_shriek, grave_tug, crop_rot_claw.
//!
//! The Skeletal Tiller is a summon-control boss that continuously re-summons
//! vegetable minions while pressuring heroes with frozen debuffs, stress,
//! and heavy melee damage. Its AI prioritizes re-summoning dead vegetables
//! over all other actions.
//!
//! Game-gaps:
//! - Summon mechanic (bone_reforge) modeled as status marker only
//! - AI priority system (ally_dead_skill with 1M base_chance) not modeled
//! - Position-based targeting (launch 1234, target ~@12/#1234/~1234/34)
//!   approximated as AllEnemies/AllAllies/SelfOnly
//! - #1234 AoE targeting approximated as AllEnemies
//! - ~1234 inverse targeting approximated as AllEnemies
//! - ~@12 friendly inverse targeting approximated as SelfOnly (summon for own team)
//! - Two turns per round not modeled in Archetype
//! - PROT (0.6), MAGIC_PROT (0.7) not modeled in Archetype
//! - Stun Resist 50%, Poison Resist 50%, Bleed Resist 100% (immune),
//!   Debuff Resist 40%, Move Resist 70%, Burn Resist 35%, Frozen Resist 35%
//!   not modeled
//! - "Debuff Frozen-30_2" from famine_reaping approximated as
//!   apply_status("frozen", Some(2)) — the specific 30% resist reduction is a game-gap
//! - "Stress Range 7-15" from scarecrow_shriek averaged to 11
//! - Self-movement on bone_reforge dropped as game-gap
//! - crop_rot_claw is not in the AI brain's active rotation (only used as fallback)

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Skeletal Tiller base archetype — tier 1 boss stats from DDGC data.
///
/// HP 150, weapon damage derived from famine_reaping skill (25–36 avg 30.5),
/// speed 6, defense 0.10 (10% dodge).
/// Summoner role: summons vegetable minions and controls with frozen/stress.
/// Crit 5% from famine_reaping/crop_rot_claw.
/// PROT 0.6, MAGIC_PROT 0.7, Stun Resist 50%, Poison Resist 50%,
/// Bleed Resist 100% (immune), Debuff Resist 40%, Move Resist 70%,
/// Burn Resist 35%, Frozen Resist 35% — all not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Skeletal Tiller"),
        side: CombatSide::Enemy,
        health: 150.0,
        max_health: 150.0,
        attack: 30.5,
        defense: 0.10,
        speed: 6.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.05,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Skeletal Tiller Skills ────────────────────────────────────────────────────

/// Bone Reforge — summon vegetable.
///
/// DDGC reference: dmg 0–0, atk 100%, crit 0%,
/// launch 1234, target ~@12 (friendly front rows),
/// effect "Summon Vegetable".
/// AI behavior: fires with overwhelming priority (1M base_chance)
/// whenever any vegetable ally is dead.
/// Game-gap: summon mechanic modeled as status marker only.
/// Game-gap: AI ally_dead_skill priority not modeled.
pub fn bone_reforge() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("bone_reforge"),
        vec![EffectNode::apply_status("summon_vegetable", Some(1))],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// Famine Reaping — heavy AoE damage + frozen debuff.
///
/// DDGC reference: dmg 25–36, atk 85%, crit 5%,
/// launch 1234, target #1234 (AoE all ranks, marked),
/// effect "Debuff Frozen-30_2" (30% frozen resist reduction for 2 rounds).
/// Game-gap: #1234 targeting approximated as AllEnemies.
/// Game-gap: "Debuff Frozen-30_2" modeled as apply_status("frozen", Some(2)) —
/// the 30% resist reduction detail is a game-gap.
pub fn famine_reaping() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("famine_reaping"),
        vec![
            EffectNode::damage(30.5),
            EffectNode::apply_status("frozen", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Scarecrow Shriek — AoE stress.
///
/// DDGC reference: dmg 0–0, atk 85%, crit 0%,
/// launch 1234, target ~1234 (all enemies, inverse targeting),
/// effect "Stress Range 7-15".
/// Game-gap: ~1234 targeting approximated as AllEnemies.
/// Game-gap: "Stress Range 7-15" averaged to 11.
pub fn scarecrow_shriek() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("scarecrow_shriek"),
        vec![EffectNode::apply_status("stress", Some(11))],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Grave Tug — pull + damage (3-round cooldown).
///
/// DDGC reference: dmg 9–12, atk 85%, crit 0%,
/// launch 1234, target 34 (back ranks),
/// effect "Pull 2A".
/// Cooldown: 3 rounds.
/// AI behavior: fires with near-guaranteed priority (1000 base_chance)
/// when off cooldown.
/// Game-gap: position-based targeting (ranks 3-4) approximated as AllEnemies.
pub fn grave_tug() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("grave_tug"),
        vec![
            EffectNode::damage(10.5),
            EffectNode::pull(2),
        ],
        TargetSelector::AllEnemies,
        1,
        Some(3),
    )
}

/// Crop Rot Claw — AoE damage + frozen.
///
/// DDGC reference: dmg 12–24, atk 85%, crit 5%,
/// launch 12 (front ranks), target #1234 (AoE all ranks, marked),
/// effect "Frozen 2".
/// Note: this skill is not in the AI brain's active rotation — it
/// serves as a backup option.
/// Game-gap: #1234 targeting approximated as AllEnemies.
/// Game-gap: position-based launch restriction (ranks 1-2) not modeled.
pub fn crop_rot_claw() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("tiller_crop_rot_claw"),
        vec![
            EffectNode::damage(18.0),
            EffectNode::apply_status("frozen", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 5 Skeletal Tiller skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        bone_reforge(),
        famine_reaping(),
        scarecrow_shriek(),
        grave_tug(),
        crop_rot_claw(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skeletal_tiller_archetype_is_enemy_eldritch_summoner() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Skeletal Tiller");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 150.0);
        assert_eq!(arch.max_health, 150.0);
        assert_eq!(arch.speed, 6.0, "skeletal_tiller has SPD 6");
        assert_eq!(arch.defense, 0.10, "skeletal_tiller has 10% defense");
        assert_eq!(arch.attack, 30.5, "attack from famine_reaping avg 25-36");
        assert_eq!(arch.crit_chance, 0.05, "crit 5% from famine_reaping/crop_rot_claw");
    }

    #[test]
    fn skeletal_tiller_bone_reforge_is_summon() {
        let skill = bone_reforge();
        assert_eq!(skill.id.0, "bone_reforge");
        let has_summon = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("summon_vegetable")
        });
        assert!(has_summon, "bone_reforge must apply summon_vegetable status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "bone_reforge targets self (summon for own team)"
        );
    }

    #[test]
    fn skeletal_tiller_famine_reaping_applies_frozen() {
        let skill = famine_reaping();
        assert_eq!(skill.id.0, "famine_reaping");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "famine_reaping must deal damage");
        let has_frozen = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("frozen")
        });
        assert!(has_frozen, "famine_reaping must apply frozen status");
    }

    #[test]
    fn skeletal_tiller_scarecrow_shriek_applies_stress() {
        let skill = scarecrow_shriek();
        assert_eq!(skill.id.0, "scarecrow_shriek");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "scarecrow_shriek must apply stress status");
    }

    #[test]
    fn skeletal_tiller_grave_tug_has_cooldown_and_pull() {
        let skill = grave_tug();
        assert_eq!(skill.id.0, "grave_tug");
        assert_eq!(skill.cooldown, Some(3), "grave_tug has 3-round cooldown");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "grave_tug must deal damage");
        let has_pull = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Pull)
        });
        assert!(has_pull, "grave_tug must pull");
    }

    #[test]
    fn skeletal_tiller_crop_rot_claw_applies_frozen() {
        let skill = crop_rot_claw();
        assert_eq!(skill.id.0, "tiller_crop_rot_claw");
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
    fn skeletal_tiller_skill_pack_has_five_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 5);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"bone_reforge"), "missing bone_reforge");
        assert!(ids.contains(&"famine_reaping"), "missing famine_reaping");
        assert!(ids.contains(&"scarecrow_shriek"), "missing scarecrow_shriek");
        assert!(ids.contains(&"grave_tug"), "missing grave_tug");
        assert!(ids.contains(&"tiller_crop_rot_claw"), "missing tiller_crop_rot_claw");
    }

    #[test]
    fn skeletal_tiller_summon_plus_frozen_plus_stress_identity() {
        // The core identity of skeletal_tiller is a summon-control boss that
        // continuously re-summons vegetable minions while pressuring heroes
        // with frozen debuffs and stress.
        let pack = skill_pack();

        let has_summon = pack.iter().any(|s| {
            s.id.0 == "bone_reforge"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("summon_vegetable")
                })
        });

        let has_frozen = pack.iter().any(|s| {
            s.effects.iter().any(|e| {
                matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                    && e.status_kind.as_deref() == Some("frozen")
            })
        });

        let has_stress = pack.iter().any(|s| {
            s.id.0 == "scarecrow_shriek"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stress")
                })
        });

        assert!(has_summon, "skeletal_tiller must have summon skill");
        assert!(has_frozen, "skeletal_tiller must have frozen skill");
        assert!(has_stress, "skeletal_tiller must have stress skill");
    }
}
