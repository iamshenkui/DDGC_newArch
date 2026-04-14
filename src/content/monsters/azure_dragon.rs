//! Azure Dragon — QingLong boss family (summoner + ball-swap + storm control).
//!
//! DDGC reference: Beast-type boss from the QingLong dungeon.
//! Tier 1 base stats: HP 150, DEF 30%, PROT 0.8, SPD 7, 2 turns/round.
//! Skills: bloodscale_reaping, dragonfear_crash, summit_relocation,
//! soulfog_enthrall, dragoncry_storm, volt_tyranny, voltic_baptism,
//! capricious_skies, swap_dragon_ball, swap_dragon_ball_other,
//! swap_dragon_ball_summon.
//!
//! This family's defining identity is a summoner boss that swaps between
//! wind and thunder dragon balls, using the active ball type to determine
//! its attack skill set. Phase 1 manages ball summon/swap; Phase 2
//! (scales depleted) uses random buffs/debuffs.
//!
//! Game-gaps:
//! - Ball swap/summon mechanics modeled as status markers only
//! - Two-phase AI (scales intact vs depleted) not modeled
//! - Ball-type-dependent skill selection not modeled
//! - Multiple turns per round not modeled in Archetype
//! - PROT (0.8) and MAGIC_PROT (0.6) not modeled in Archetype
//! - Position-based targeting (launch 23, target 1234/234/~1234/@23/@14) not modeled
//! - Random buff/debuff tables (voltic_baptism, capricious_skies) not modeled
//! - Azure Dragon Disorder effect (80% chance condition) simplified to status marker
//! - Shared health pool between ball parts not modeled

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Azure Dragon base archetype — tier 1 boss stats from DDGC data.
///
/// HP 150, weapon damage derived from volt_tyranny skill (3-6 avg 4.5),
/// speed 7, defense 0.30 (30% dodge).
/// Summoner role: boss with ball-summon mechanics and two-phase AI.
/// Crit 12% from volt_tyranny skill.
/// PROT 0.8, MAGIC_PROT 0.6, Stun Resist 40%, Poison Resist 60%,
/// Bleed Resist 20%, Debuff Resist 40%, Move Resist 200% (immune),
/// Burn Resist 20%, Frozen Resist 25% — all not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Azure Dragon"),
        side: CombatSide::Enemy,
        health: 150.0,
        max_health: 150.0,
        attack: 4.5,
        defense: 0.30,
        speed: 7.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.12,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Azure Dragon Skills ──────────────────────────────────────────────────

/// Bloodscale Reaping — wind-phase melee attack with bleed.
///
/// DDGC reference: dmg 2-4 (avg 3), atk 90%, crit 6%,
/// launch ranks 2-3, target ranks 1-4, effect "Bleed 1".
/// Wind-phase skill: used when wind ball is active.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn bloodscale_reaping() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("bloodscale_reaping"),
        vec![
            EffectNode::damage(3.0),
            EffectNode::apply_status("bleed", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Dragonfear Crash — wind-phase melee attack with stun.
///
/// DDGC reference: dmg 1-2 (avg 1.5), atk 90%, crit 6%,
/// launch ranks 2-3, target ranks 1-4, effect "Stun 1".
/// Wind-phase skill: used when wind ball is active.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn dragonfear_crash() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("dragonfear_crash"),
        vec![
            EffectNode::damage(1.5),
            EffectNode::apply_status("stun", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Summit Relocation — wind-phase melee attack with pull.
///
/// DDGC reference: dmg 1-2 (avg 1.5), atk 90%, crit 6%,
/// launch ranks 2-3, target ranks 2-4, effect "Pull 3".
/// Wind-phase skill: used when wind ball is active.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn summit_relocation() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("summit_relocation"),
        vec![
            EffectNode::damage(1.5),
            EffectNode::pull(3),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Soulfog Enthrall — thunder-phase magic attack with disorder + debuff.
///
/// DDGC reference: dmg 2-4 (avg 3), atk 90%, crit 6%,
/// launch ranks 2-3, target ranks 1-4,
/// effects "Azure Dragon Disorder" (80% chance) + "Azure Dragon Debuff 1"
/// (-5% defense for 3 rounds).
/// Thunder-phase skill: used when thunder ball is active.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
/// Game-gap: 80% chance on Disorder simplified to guaranteed status marker.
pub fn soulfog_enthrall() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("soulfog_enthrall"),
        vec![
            EffectNode::damage(3.0),
            EffectNode::apply_status("disorder", Some(1)),
            EffectNode::apply_status("def_down", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Dragoncry Storm — thunder-phase AoE magic attack with stress.
///
/// DDGC reference: dmg 1-2 (avg 1.5), atk 90%, crit 6%,
/// launch ranks 2-3, target ~1234 (AoE all ranks),
/// effect "Azure Dragon Stress 1" (+7 stress).
/// Thunder-phase skill: used when thunder ball is active.
/// Game-gap: AoE vs single-target distinction not modeled — targets AllEnemies.
pub fn dragoncry_storm() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("dragoncry_storm"),
        vec![
            EffectNode::damage(1.5),
            EffectNode::apply_status("stress", Some(7)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Volt Tyranny — thunder-phase high-crit magic attack with stress damage.
///
/// DDGC reference: dmg 3-6 (avg 4.5), atk 90%, crit 12%,
/// launch ranks 2-3, target ranks 1-4,
/// effect "Azure Dragon STRESSDMG20" (stress damage buff).
/// Thunder-phase skill: used when thunder ball is active.
/// This is the highest single-hit damage skill in the dragon's kit.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn volt_tyranny() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("volt_tyranny"),
        vec![
            EffectNode::damage(4.5),
            EffectNode::apply_status("stress_dmg", Some(20)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Voltic Baptism — self-buff with random stat enhancement.
///
/// DDGC reference: dmg 0-0, atk 100%, crit 0%,
/// launch ranks 2-3, target @23 (self/ally ranks 2-3),
/// effect "Azure Dragon Random Buff" (randomly one of: +20% magic dmg,
/// +20% physical dmg, +8% crit, +20% DEF, +15% SPD, duration 2).
/// Phase 2 skill: used when scales are depleted.
/// Game-gap: random buff table selection not modeled — uses status marker.
/// Game-gap: ally-targeting simplified to SelfOnly.
pub fn voltic_baptism() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("voltic_baptism"),
        vec![EffectNode::apply_status("random_buff", Some(2))],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// Capricious Skies — AoE random debuff on all heroes.
///
/// DDGC reference: dmg 0-0, atk 100%, crit 0%,
/// launch ranks 2-3, target ~1234 (AoE all ranks),
/// effect "Azure Dragon Random Debuff" (randomly one of: +20% magic dmg recv,
/// +20% physical dmg recv, STRESSDMG20, -10% DEF, -5% SPD, duration 2).
/// Phase 2 skill: used when scales are depleted and energy bar is full.
/// Game-gap: random debuff table selection not modeled — uses status marker.
/// Game-gap: AoE vs single-target distinction not modeled — targets AllEnemies.
pub fn capricious_skies() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("capricious_skies"),
        vec![EffectNode::apply_status("random_debuff", Some(2))],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Swap Dragon Ball — swap ball positions (wind/thunder).
///
/// DDGC reference: dmg 0-0, atk 100%, crit 0%,
/// target @14 (ally ranks 1,4 — ball positions),
/// effect "Azure Dragon Ball Swap" (custom swap effect).
/// Phase 1 skill: used to swap active ball position.
/// Game-gap: ball position swap mechanic modeled as status marker only.
/// Game-gap: ally-targeting simplified to SelfOnly.
pub fn swap_dragon_ball() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("swap_dragon_ball"),
        vec![EffectNode::apply_status("ball_swap", Some(1))],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// Swap Dragon Ball Other — apply ball active buff after swap.
///
/// DDGC reference: dmg 0-0, atk 100%, crit 0%,
/// target ranks 1-4,
/// effects "Azure Dragon Ball Swap" + "Azure Dragon Ball Active Buff"
/// (applies STRESSDMG20 + Bleed 1 to the swapped ball).
/// Phase 1 skill: used after initial swap to activate ball effects.
/// Game-gap: ball swap + active buff modeled as status markers only.
pub fn swap_dragon_ball_other() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("swap_dragon_ball_other"),
        vec![
            EffectNode::apply_status("ball_swap", Some(1)),
            EffectNode::apply_status("ball_active_buff", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Swap Dragon Ball Summon — summon both dragon balls.
///
/// DDGC reference: dmg 0-0, atk 100%, crit 0%, no target,
/// effect "Azure Dragon Ball Summon" (summons azure_dragon_ball_thunder
/// at rank 1 and azure_dragon_ball_wind at rank 4, limit 1 each).
/// Phase 1 skill: used on first turn to spawn both ball units.
/// Game-gap: actual summon behavior modeled as status marker only.
pub fn swap_dragon_ball_summon() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("swap_dragon_ball_summon"),
        vec![EffectNode::apply_status("summon_ball", Some(1))],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// All 11 Azure Dragon skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        bloodscale_reaping(),
        dragonfear_crash(),
        summit_relocation(),
        soulfog_enthrall(),
        dragoncry_storm(),
        volt_tyranny(),
        voltic_baptism(),
        capricious_skies(),
        swap_dragon_ball(),
        swap_dragon_ball_other(),
        swap_dragon_ball_summon(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn azure_dragon_archetype_is_enemy_beast_summoner_boss() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Azure Dragon");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 150.0);
        assert_eq!(arch.max_health, 150.0);
        assert_eq!(arch.speed, 7.0);
        assert_eq!(arch.defense, 0.30, "azure_dragon has 30% defense");
        assert_eq!(arch.crit_chance, 0.12, "azure_dragon has 12% crit");
        assert_eq!(arch.attack, 4.5, "azure_dragon attack from volt_tyranny avg");
    }

    #[test]
    fn azure_dragon_bloodscale_reaping_deals_damage_and_bleed() {
        let skill = bloodscale_reaping();
        assert_eq!(skill.id.0, "bloodscale_reaping");
        assert!(
            skill.effects.len() >= 2,
            "bloodscale_reaping should have damage + bleed status"
        );
    }

    #[test]
    fn azure_dragon_dragonfear_crash_deals_damage_and_stun() {
        let skill = dragonfear_crash();
        assert_eq!(skill.id.0, "dragonfear_crash");
        assert!(
            skill.effects.len() >= 2,
            "dragonfear_crash should have damage + stun status"
        );
    }

    #[test]
    fn azure_dragon_summit_relocation_deals_damage_and_pull() {
        let skill = summit_relocation();
        assert_eq!(skill.id.0, "summit_relocation");
        assert!(
            skill.effects.len() >= 2,
            "summit_relocation should have damage + pull effect"
        );
    }

    #[test]
    fn azure_dragon_soulfog_enthrall_deals_damage_disorder_and_debuff() {
        let skill = soulfog_enthrall();
        assert_eq!(skill.id.0, "soulfog_enthrall");
        assert!(
            skill.effects.len() >= 3,
            "soulfog_enthrall should have damage + disorder + def_down"
        );
    }

    #[test]
    fn azure_dragon_dragoncry_storm_deals_damage_and_stress() {
        let skill = dragoncry_storm();
        assert_eq!(skill.id.0, "dragoncry_storm");
        assert!(
            skill.effects.len() >= 2,
            "dragoncry_storm should have damage + stress status"
        );
    }

    #[test]
    fn azure_dragon_volt_tyranny_deals_damage_and_stress_dmg() {
        let skill = volt_tyranny();
        assert_eq!(skill.id.0, "volt_tyranny");
        assert!(
            skill.effects.len() >= 2,
            "volt_tyranny should have damage + stress_dmg status"
        );
    }

    #[test]
    fn azure_dragon_voltic_baptism_applies_random_buff() {
        let skill = voltic_baptism();
        assert_eq!(skill.id.0, "voltic_baptism");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "voltic_baptism targets self"
        );
        let has_buff = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("random_buff")
        });
        assert!(has_buff, "voltic_baptism must apply random_buff status");
    }

    #[test]
    fn azure_dragon_capricious_skies_applies_random_debuff() {
        let skill = capricious_skies();
        assert_eq!(skill.id.0, "capricious_skies");
        let has_debuff = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("random_debuff")
        });
        assert!(has_debuff, "capricious_skies must apply random_debuff status");
    }

    #[test]
    fn azure_dragon_swap_dragon_ball_summon_applies_summon_status() {
        let skill = swap_dragon_ball_summon();
        assert_eq!(skill.id.0, "swap_dragon_ball_summon");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "summon targets self"
        );
        let has_summon = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("summon_ball")
        });
        assert!(has_summon, "summon must apply summon_ball status");
    }

    #[test]
    fn azure_dragon_skill_pack_has_eleven_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 11);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"bloodscale_reaping"), "missing bloodscale_reaping");
        assert!(ids.contains(&"dragonfear_crash"), "missing dragonfear_crash");
        assert!(ids.contains(&"summit_relocation"), "missing summit_relocation");
        assert!(ids.contains(&"soulfog_enthrall"), "missing soulfog_enthrall");
        assert!(ids.contains(&"dragoncry_storm"), "missing dragoncry_storm");
        assert!(ids.contains(&"volt_tyranny"), "missing volt_tyranny");
        assert!(ids.contains(&"voltic_baptism"), "missing voltic_baptism");
        assert!(ids.contains(&"capricious_skies"), "missing capricious_skies");
        assert!(ids.contains(&"swap_dragon_ball"), "missing swap_dragon_ball");
        assert!(ids.contains(&"swap_dragon_ball_other"), "missing swap_dragon_ball_other");
        assert!(ids.contains(&"swap_dragon_ball_summon"), "missing swap_dragon_ball_summon");
    }

    #[test]
    fn azure_dragon_shared_health_plus_summon_ball_identity() {
        // The core identity of azure_dragon is a summoner boss that
        // summons dragon balls and swaps between wind/thunder phases.
        // This test preserves that identity.
        let pack = skill_pack();

        // Must have summon mechanic
        let has_summon = pack.iter().any(|s| {
            s.id.0 == "swap_dragon_ball_summon"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("summon_ball")
                })
        });

        // Must have ball swap mechanic
        let has_swap = pack.iter().any(|s| {
            s.id.0 == "swap_dragon_ball"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("ball_swap")
                })
        });

        // Must have wind-phase physical skills (bleed, stun, pull)
        let has_wind_phase = pack.iter().any(|s| {
            s.id.0 == "bloodscale_reaping"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("bleed")
                })
        });

        // Must have thunder-phase magic skills (disorder, stress, stress_dmg)
        let has_thunder_phase = pack.iter().any(|s| {
            s.id.0 == "volt_tyranny"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stress_dmg")
                })
        });

        // Must have phase-2 skills (random buff/debuff)
        let has_phase2 = pack.iter().any(|s| {
            s.id.0 == "capricious_skies"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("random_debuff")
                })
        });

        assert!(has_summon, "azure_dragon must have ball summon mechanic");
        assert!(has_swap, "azure_dragon must have ball swap mechanic");
        assert!(has_wind_phase, "azure_dragon must have wind-phase physical skills");
        assert!(has_thunder_phase, "azure_dragon must have thunder-phase magic skills");
        assert!(has_phase2, "azure_dragon must have phase-2 buff/debuff skills");
    }
}
