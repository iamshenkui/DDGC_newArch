//! Vermilion Bird — ZhuQue boss family (burn-heal cycle + shared-health tails).
//!
//! DDGC reference: Beast-type boss from the ZhuQue dungeon.
//! Tier 1 base stats: HP 160, DEF 30%, PROT 0.5, SPD 7, 2 turns/round.
//! Skills: singing_loudly, ruin, ruin1, precise_pecking, iron_feather,
//! bide, calm_nerves, explosion.
//!
//! This family's defining identity is a two-phase boss that applies burn
//! pressure (ruin/ruin1) and sustains itself through a heal cycle
//! (calm_nerves absorbs PB charge → explosion converts it to massive heal).
//! Tail_A provides stress/disorder control and protection buffs; Tail_B
//! marks targets, debuffs heroes, and charges PB for the explosion cycle.
//!
//! Game-gaps:
//! - Shared health pool between main body and tails not modeled
//! - Two-phase AI (Phase 1: singing+ruin/pecking; Phase 2: calm/explosion cycle) not modeled
//! - Multiple turns per round not modeled in Archetype
//! - PROT (0.5) and MAGIC_PROT (0.8) not modeled in Archetype
//! - Burn/Stun/Move/etc. resistances not modeled in Archetype
//! - Position-based targeting (launch 12/1234, target $1234/~1234/@12) not modeled
//! - PB charge/absorb mechanic (absorb_vb_pb) not modeled
//! - Self-buff side effects on ruin/ruin1 (Vermilion Bird Buff 1) dropped
//! - Self-buff/damage on calm_nerves (enemy damage) dropped in favor of heal identity
//! - Vermilion Bird Field arena mechanic not modeled
//! - Random single-target ($ prefix) not modeled — uses AllEnemies
//! - suck_hp (life steal) on precise_pecking not modeled
//! - .move on singing_loudly, bide, iron_feather dropped as game-gaps

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Vermilion Bird base archetype — tier 1 boss stats from DDGC data.
///
/// HP 160, weapon damage derived from singing_loudly/ruin1 (2-5 avg 3.5),
/// speed 7, defense 0.30 (30% dodge).
/// Summoner role: boss with tail parts and two-phase heal cycle.
/// Crit 6% from singing_loudly skill.
/// PROT 0.5, MAGIC_PROT 0.8, Stun Resist 40%, Poison Resist 60%,
/// Bleed Resist 20%, Debuff Resist 40%, Move Resist 50%, Burn Resist 100%,
/// Frozen Resist 25% — all not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Vermilion Bird"),
        side: CombatSide::Enemy,
        health: 160.0,
        max_health: 160.0,
        attack: 3.5,
        defense: 0.30,
        speed: 7.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.06,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Vermilion Bird Skills ────────────────────────────────────────────────

/// Singing Loudly — ranged attack with stun, used on cooldown.
///
/// DDGC reference: dmg 2-5 (avg 3.5), atk 90%, crit 6%,
/// effect "Stun 1", launch ranks 1-2, target ranks 1-4, .move 0 1.
/// Phase 1+2 skill: used on 2-round cooldown in both phases.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
/// Game-gap: .move 0 1 (self-reposition) dropped.
pub fn singing_loudly() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("singing_loudly"),
        vec![
            EffectNode::damage(3.5),
            EffectNode::apply_status("stun", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        Some(2),
    )
}

/// Ruin — AoE magic attack with burn (Phase 1 version).
///
/// DDGC reference: magic_dmg 1-3 (avg 2), atk 90%, crit 0%,
/// effects "Burn 1" + "Vermilion Bird Buff 1" (+10% dmg for 3 rounds, performer self-buff),
/// launch ranks 1-2, target ~1234 (AoE all ranks).
/// Phase 1 skill: 50/50 with precise_pecking.
/// Game-gap: magic damage not modeled — treated as normal damage.
/// Game-gap: AoE vs single-target distinction not modeled — targets AllEnemies.
/// Game-gap: performer self-buff (Vermilion Bird Buff 1) dropped.
pub fn ruin() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("ruin"),
        vec![
            EffectNode::damage(2.0),
            EffectNode::apply_status("burn", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Ruin1 — AoE magic attack with burn (Phase 2 version, higher damage).
///
/// DDGC reference: magic_dmg 2-5 (avg 3.5), atk 90%, crit 0%,
/// effects "Burn 1" + "Vermilion Bird Buff 1",
/// launch ranks 1-2, target ~1234 (AoE all ranks).
/// Phase 2 skill: used as primary attack when not in calm/explosion cycle.
/// Game-gap: same as ruin.
pub fn ruin1() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("ruin1"),
        vec![
            EffectNode::damage(3.5),
            EffectNode::apply_status("burn", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Precise Pecking — melee attack with bleed + slow + life steal.
///
/// DDGC reference: dmg 1-3 (avg 2), atk 90%, crit 0%,
/// effects "Phoenix Bleed 1" + "Speed -1",
/// launch ranks 1-2, target $1234 (random single), suck_hp 0.5.
/// Phase 1 skill: 50/50 with ruin.
/// Game-gap: random single-target ($ prefix) not modeled — targets AllEnemies.
/// Game-gap: suck_hp (50% life steal) not modeled.
pub fn precise_pecking() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("precise_pecking"),
        vec![
            EffectNode::damage(2.0),
            EffectNode::apply_status("bleed", Some(1)),
            EffectNode::apply_status("slow", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Iron Feather — self physical damage buff.
///
/// DDGC reference: dmg 0-0, atk 0%, crit 0%,
/// effect "Vermilion Bird Damage +10" (+10% physical damage for 1 round),
/// launch ranks 1-2, target @12 (ally ranks 1-2), .move 1 0.
/// Phase 2 skill: 30% chance when at rank 1.
/// Game-gap: ally-targeting (@12) simplified to SelfOnly.
/// Game-gap: .move 1 0 (forward movement) dropped.
pub fn iron_feather() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("iron_feather"),
        vec![EffectNode::apply_status("dmg_up_phys", Some(1))],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// Bide — ranged attack with stress.
///
/// DDGC reference: dmg 1-3 (avg 2), atk 90%, crit 0%,
/// effect "Stress Range 7-8" (avg 8),
/// launch ranks 1-4, target $1234 (random single), .move 0 1.
/// Phase 2 skill: used at rank 2.
/// Game-gap: random single-target ($ prefix) not modeled — targets AllEnemies.
/// Game-gap: .move 0 1 (self-reposition) dropped.
pub fn bide() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("bide"),
        vec![
            EffectNode::damage(2.0),
            EffectNode::apply_status("stress", Some(8)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Calm Nerves — self magic damage buff + heal (absorbs 25% PB charge).
///
/// DDGC reference: dmg 1-3 (avg 2) to random enemy, atk 90%, crit 0%,
/// effects "Vermilion Bird Magic Damage +10" (+10% magic dmg, permanent stacking)
/// + "Heal Self Range 5-10" (avg 8),
/// launch ranks 1-4, target $1234, .absorb_vb_pb 25 0.
/// Phase 2 skill: core of the calm_nerves/explosion cycle.
///
/// The heal + magic buff are performer-targeted (self), while the damage
/// goes to a random enemy. Since the heal is the CORE identity of this
/// skill (the "calm" that sustains the boss), we model it as SelfOnly
/// and drop the enemy damage as a game-gap.
///
/// Game-gap: enemy damage (1-3 avg 2) dropped in favor of preserving heal identity.
/// Game-gap: .absorb_vb_pb 25 0 (absorb 25% of PB charge) not modeled.
pub fn calm_nerves() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("calm_nerves"),
        vec![
            EffectNode::apply_status("magic_dmg_up", Some(1)),
            EffectNode::apply_status("heal_self", Some(8)),
        ],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// Explosion — self magic damage buff + massive heal (absorbs 100% PB charge).
///
/// DDGC reference: dmg 0-0, atk 90%, crit 0%,
/// effects "Vermilion Bird Magic Damage +10" + "Heal Self Range 5-10",
/// launch ranks 1-4, target $1234, .absorb_vb_pb 0 100.
/// Phase 2 skill: triggered on every 4th turn in the calm/explosion cycle.
/// Converts all accumulated PB charge into a massive heal.
///
/// Game-gap: .absorb_vb_pb 0 100 (convert 100% PB charge to heal) not modeled
/// — uses pb_absorb status marker to preserve identity.
pub fn explosion() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("explosion"),
        vec![
            EffectNode::apply_status("magic_dmg_up", Some(1)),
            EffectNode::apply_status("heal_self", Some(8)),
            EffectNode::apply_status("pb_absorb", Some(1)),
        ],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// All 8 Vermilion Bird skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        singing_loudly(),
        ruin(),
        ruin1(),
        precise_pecking(),
        iron_feather(),
        bide(),
        calm_nerves(),
        explosion(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vermilion_bird_archetype_is_enemy_beast_summoner_boss() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Vermilion Bird");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 160.0);
        assert_eq!(arch.max_health, 160.0);
        assert_eq!(arch.speed, 7.0);
        assert_eq!(arch.defense, 0.30, "vermilion_bird has 30% defense");
        assert_eq!(arch.crit_chance, 0.06, "vermilion_bird has 6% crit");
        assert_eq!(arch.attack, 3.5, "vermilion_bird attack from singing_loudly/ruin1 avg");
    }

    #[test]
    fn vermilion_bird_singing_loudly_deals_damage_and_stun() {
        let skill = singing_loudly();
        assert_eq!(skill.id.0, "singing_loudly");
        assert_eq!(skill.cooldown, Some(2), "singing_loudly has 2-round cooldown");
        assert!(
            skill.effects.len() >= 2,
            "singing_loudly should have damage + stun status"
        );
    }

    #[test]
    fn vermilion_bird_ruin_deals_damage_and_burn() {
        let skill = ruin();
        assert_eq!(skill.id.0, "ruin");
        let has_burn = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("burn")
        });
        assert!(has_burn, "ruin must apply burn status");
    }

    #[test]
    fn vermilion_bird_ruin1_deals_higher_damage_and_burn() {
        let skill = ruin1();
        assert_eq!(skill.id.0, "ruin1");
        let has_burn = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("burn")
        });
        assert!(has_burn, "ruin1 must apply burn status");
    }

    #[test]
    fn vermilion_bird_precise_pecking_deals_damage_bleed_and_slow() {
        let skill = precise_pecking();
        assert_eq!(skill.id.0, "precise_pecking");
        assert!(
            skill.effects.len() >= 3,
            "precise_pecking should have damage + bleed + slow"
        );
    }

    #[test]
    fn vermilion_bird_iron_feather_applies_phys_dmg_buff() {
        let skill = iron_feather();
        assert_eq!(skill.id.0, "iron_feather");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "iron_feather targets self"
        );
        let has_buff = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("dmg_up_phys")
        });
        assert!(has_buff, "iron_feather must apply dmg_up_phys status");
    }

    #[test]
    fn vermilion_bird_bide_deals_damage_and_stress() {
        let skill = bide();
        assert_eq!(skill.id.0, "bide");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "bide must apply stress status");
    }

    #[test]
    fn vermilion_bird_calm_nerves_heals_and_buffs_magic() {
        let skill = calm_nerves();
        assert_eq!(skill.id.0, "calm_nerves");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "calm_nerves targets self for heal"
        );
        let has_heal = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("heal_self")
        });
        let has_magic_buff = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("magic_dmg_up")
        });
        assert!(has_heal, "calm_nerves must apply heal_self status");
        assert!(has_magic_buff, "calm_nerves must apply magic_dmg_up status");
    }

    #[test]
    fn vermilion_bird_explosion_heals_buffs_and_absorbs_pb() {
        let skill = explosion();
        assert_eq!(skill.id.0, "explosion");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "explosion targets self for heal"
        );
        let has_heal = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("heal_self")
        });
        let has_pb_absorb = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("pb_absorb")
        });
        assert!(has_heal, "explosion must apply heal_self status");
        assert!(has_pb_absorb, "explosion must apply pb_absorb status");
    }

    #[test]
    fn vermilion_bird_skill_pack_has_eight_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 8);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"singing_loudly"), "missing singing_loudly");
        assert!(ids.contains(&"ruin"), "missing ruin");
        assert!(ids.contains(&"ruin1"), "missing ruin1");
        assert!(ids.contains(&"precise_pecking"), "missing precise_pecking");
        assert!(ids.contains(&"iron_feather"), "missing iron_feather");
        assert!(ids.contains(&"bide"), "missing bide");
        assert!(ids.contains(&"calm_nerves"), "missing calm_nerves");
        assert!(ids.contains(&"explosion"), "missing explosion");
    }

    #[test]
    fn vermilion_bird_shared_health_plus_burn_heal_cycle_identity() {
        // The core identity of vermilion_bird is a two-phase boss that:
        // 1. Applies burn pressure through ruin/ruin1
        // 2. Heals itself through calm_nerves/explosion cycle
        // 3. Has shared-health tail parts that provide support
        let pack = skill_pack();

        // Must have burn-inflicting skills (ruin + ruin1)
        let has_burn = pack.iter().any(|s| {
            (s.id.0 == "ruin" || s.id.0 == "ruin1")
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("burn")
                })
        });

        // Must have self-heal skill (calm_nerves)
        let has_calm = pack.iter().any(|s| {
            s.id.0 == "calm_nerves"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("heal_self")
                })
        });

        // Must have PB explosion heal (explosion)
        let has_explosion = pack.iter().any(|s| {
            s.id.0 == "explosion"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("pb_absorb")
                })
        });

        // Must have stun skill (singing_loudly)
        let has_stun = pack.iter().any(|s| {
            s.id.0 == "singing_loudly"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stun")
                })
        });

        assert!(has_burn, "vermilion_bird must have burn-inflicting skills");
        assert!(has_calm, "vermilion_bird must have calm_nerves self-heal");
        assert!(has_explosion, "vermilion_bird must have explosion PB heal");
        assert!(has_stun, "vermilion_bird must have singing_loudly stun");
    }
}
