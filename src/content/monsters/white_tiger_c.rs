//! White Tiger (Final Form) — BaiHu boss family (multi-phase summoner + storm control).
//!
//! DDGC reference: Beast-type boss from the BaiHu dungeon.
//! Tier 1 base stats: HP 115, DEF 30%, PROT 0.6, SPD 7, 3 turns/round.
//! Skills: thunder_lightning, paw, raging_fire, true_strike, jump,
//! deter_stress, deter_def.
//!
//! This family's defining identity is a multi-phase boss that starts as
//! two clones (white_tiger_A and white_tiger_B) and transitions to the
//! final form (white_tiger_C) after press-attack accumulation. The C form
//! is a size-2 boss with 3 turns per round, featuring stun, burn, stress,
//! and positional disruption skills.
//!
//! Game-gaps:
//! - Multi-phase transition (A/B clones → C final form) not modeled
//! - Press-attack-count mechanic that triggers phase transition not modeled
//! - WhiteTigerField rank-press damage modifier not modeled
//! - Torch manipulation during phase transition not modeled
//! - Multiple turns per round not modeled in Archetype
//! - PROT (0.6), MAGIC_PROT (0.2) not modeled in Archetype
//! - Position-based targeting (launch 12/34, target ~12/~34) not modeled
//! - `.is_ignore_def True` on paw not modeled in framework
//! - "White Tiger Debuff 2" / "White Tiger Buff 1" custom status effects
//!   simplified to generic status markers
//! - "White Tiger Speed 1" on jump modeled as reposition status marker
//! - "White Tiger Shuffle Target" and "Disorder" on tiger_swing (B form)
//!   simplified to generic status markers

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// White Tiger C (final form) base archetype — tier 1 boss stats from DDGC data.
///
/// HP 115, weapon damage derived from highest-damage skill (paw/true_strike:
/// 6-8 avg 7), speed 7, defense 0.30 (30% dodge).
/// Summoner role: multi-phase boss that transitions from A/B clones.
/// Crit 6% from multiple skills.
/// PROT 0.6, MAGIC_PROT 0.2, Stun Resist 40%, Poison Resist 60%,
/// Bleed Resist 20%, Debuff Resist 40%, Move Resist 50%,
/// Burn Resist 20%, Frozen Resist 25% — all not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("White Tiger C"),
        side: CombatSide::Enemy,
        health: 115.0,
        max_health: 115.0,
        attack: 7.0,
        defense: 0.30,
        speed: 7.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.06,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── White Tiger C Skills ─────────────────────────────────────────────────

/// Thunder Lightning — AoE magic attack with stun.
///
/// DDGC reference: magic_dmg 2-5 (avg 3.5), atk 72%, crit 6%,
/// launch ranks 3-4, target ~34 (AoE enemy ranks 3-4),
/// effect "Weak Stun 1".
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn thunder_lightning() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("thunder_lightning"),
        vec![
            EffectNode::damage(3.5),
            EffectNode::apply_status("stun", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Paw — heavy melee attack with push and ignore defense.
///
/// DDGC reference: dmg 6-8 (avg 7), atk 72%, crit 6%,
/// launch ranks 3-4, target ranks 1-2, effect "Push 1A",
/// move 0 2 (forward 2), .is_ignore_def True.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
/// Game-gap: ignore_def not modeled in framework.
/// Game-gap: push(1) approximated from "Push 1A".
pub fn paw() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("paw"),
        vec![
            EffectNode::damage(7.0),
            EffectNode::push(1),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Raging Fire — melee magic attack with debuff and burn.
///
/// DDGC reference: magic_dmg 1-3 (avg 2), atk 72%, crit 6%,
/// launch ranks 1-2, target ~12 (AoE enemy ranks 1-2),
/// effects "White Tiger Debuff 2" + "Burn 1".
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
/// Game-gap: "White Tiger Debuff 2" simplified to generic debuff marker.
pub fn raging_fire() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("raging_fire"),
        vec![
            EffectNode::damage(2.0),
            EffectNode::apply_status("debuff", Some(2)),
            EffectNode::apply_status("burn", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// True Strike — ranged attack with stun and pull.
///
/// DDGC reference: dmg 6-8 (avg 7), atk 72%, crit 6%,
/// launch ranks 1-2, target ranks 3-4,
/// effect "Weak Stun 1", move 2 0 (pull target forward 2).
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn true_strike() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("true_strike"),
        vec![
            EffectNode::damage(7.0),
            EffectNode::apply_status("stun", Some(1)),
            EffectNode::pull(2),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Jump — reposition to terrain tile (always used on first initiative).
///
/// DDGC reference: dmg 0-0, atk 0%, crit 0%,
/// effect "White Tiger Speed 1",
/// launch any, target @1234 (ally positions including terrain),
/// move 0 0.
/// Game-gap: actual terrain repositioning modeled as status marker only.
/// Game-gap: ally-targeting simplified to SelfOnly.
pub fn jump() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("jump"),
        vec![EffectNode::apply_status("white_tiger_speed", Some(1))],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// Deter Stress — melee magic attack with AoE stress group.
///
/// DDGC reference: magic_dmg 2-5 (avg 3.5), atk 72%, crit 6%,
/// launch ranks 1-2, target ~12 (AoE enemy ranks 1-2),
/// effect "Stress Target Group 1", move 2 0 (forward 2).
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
/// Game-gap: "Stress Target Group 1" simplified to generic stress.
pub fn deter_stress() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("deter_stress"),
        vec![
            EffectNode::damage(3.5),
            EffectNode::apply_status("stress", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Deter Defense — AoE stress attack (no damage).
///
/// DDGC reference: magic_dmg 0-0, atk 72%, crit 0%,
/// launch ranks 3-4, target ~1234 (AoE all ranks),
/// effect "Stress 1".
/// Game-gap: AoE vs single-target distinction not modeled — targets AllEnemies.
pub fn deter_def() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("deter_def"),
        vec![EffectNode::apply_status("stress", Some(1))],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 7 White Tiger C skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        thunder_lightning(),
        paw(),
        raging_fire(),
        true_strike(),
        jump(),
        deter_stress(),
        deter_def(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_tiger_c_archetype_is_enemy_beast_summoner_boss() {
        let arch = archetype();
        assert_eq!(arch.name.0, "White Tiger C");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 115.0);
        assert_eq!(arch.max_health, 115.0);
        assert_eq!(arch.speed, 7.0);
        assert_eq!(arch.defense, 0.30, "white_tiger_C has 30% defense");
        assert_eq!(arch.crit_chance, 0.06, "white_tiger_C has 6% crit");
        assert_eq!(arch.attack, 7.0, "white_tiger_C attack from paw/true_strike avg");
    }

    #[test]
    fn white_tiger_c_thunder_lightning_deals_damage_and_stun() {
        let skill = thunder_lightning();
        assert_eq!(skill.id.0, "thunder_lightning");
        assert!(
            skill.effects.len() >= 2,
            "thunder_lightning should have damage + stun status"
        );
    }

    #[test]
    fn white_tiger_c_paw_deals_damage_and_push() {
        let skill = paw();
        assert_eq!(skill.id.0, "paw");
        assert!(
            skill.effects.len() >= 2,
            "paw should have damage + push effect"
        );
    }

    #[test]
    fn white_tiger_c_raging_fire_deals_damage_debuff_and_burn() {
        let skill = raging_fire();
        assert_eq!(skill.id.0, "raging_fire");
        assert!(
            skill.effects.len() >= 3,
            "raging_fire should have damage + debuff + burn"
        );
    }

    #[test]
    fn white_tiger_c_true_strike_deals_damage_stun_and_pull() {
        let skill = true_strike();
        assert_eq!(skill.id.0, "true_strike");
        assert!(
            skill.effects.len() >= 3,
            "true_strike should have damage + stun + pull"
        );
    }

    #[test]
    fn white_tiger_c_jump_applies_white_tiger_speed() {
        let skill = jump();
        assert_eq!(skill.id.0, "jump");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "jump targets self"
        );
        let has_speed = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("white_tiger_speed")
        });
        assert!(has_speed, "jump must apply white_tiger_speed status");
    }

    #[test]
    fn white_tiger_c_deter_stress_deals_damage_and_stress() {
        let skill = deter_stress();
        assert_eq!(skill.id.0, "deter_stress");
        assert!(
            skill.effects.len() >= 2,
            "deter_stress should have damage + stress"
        );
    }

    #[test]
    fn white_tiger_c_deter_def_applies_stress() {
        let skill = deter_def();
        assert_eq!(skill.id.0, "deter_def");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "deter_def must apply stress status");
    }

    #[test]
    fn white_tiger_c_skill_pack_has_seven_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 7);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"thunder_lightning"), "missing thunder_lightning");
        assert!(ids.contains(&"paw"), "missing paw");
        assert!(ids.contains(&"raging_fire"), "missing raging_fire");
        assert!(ids.contains(&"true_strike"), "missing true_strike");
        assert!(ids.contains(&"jump"), "missing jump");
        assert!(ids.contains(&"deter_stress"), "missing deter_stress");
        assert!(ids.contains(&"deter_def"), "missing deter_def");
    }

    #[test]
    fn white_tiger_c_multi_phase_plus_storm_control_identity() {
        // The core identity of white_tiger_C is a final-form boss with
        // stun control, burn/debuff pressure, and positional disruption.
        // This test preserves that identity.
        let pack = skill_pack();

        // Must have stun mechanics (thunder_lightning, true_strike)
        let has_stun = pack.iter().any(|s| {
            (s.id.0 == "thunder_lightning" || s.id.0 == "true_strike")
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stun")
                })
        });

        // Must have burn/debuff mechanic (raging_fire)
        let has_debuff_burn = pack.iter().any(|s| {
            s.id.0 == "raging_fire"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("burn")
                })
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("debuff")
                })
        });

        // Must have positional disruption (paw push, true_strike pull)
        let has_push = pack.iter().any(|s| {
            s.id.0 == "paw"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Push)
                })
        });

        let has_pull = pack.iter().any(|s| {
            s.id.0 == "true_strike"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Pull)
                })
        });

        // Must have stress mechanic (deter_stress, deter_def)
        let has_stress = pack.iter().any(|s| {
            (s.id.0 == "deter_stress" || s.id.0 == "deter_def")
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stress")
                })
        });

        // Must have jump/reposition mechanic
        let has_jump = pack.iter().any(|s| {
            s.id.0 == "jump"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("white_tiger_speed")
                })
        });

        assert!(has_stun, "white_tiger_C must have stun mechanics");
        assert!(has_debuff_burn, "white_tiger_C must have burn/debuff mechanic");
        assert!(has_push, "white_tiger_C must have push mechanic");
        assert!(has_pull, "white_tiger_C must have pull mechanic");
        assert!(has_stress, "white_tiger_C must have stress mechanic");
        assert!(has_jump, "white_tiger_C must have jump/reposition mechanic");
    }
}