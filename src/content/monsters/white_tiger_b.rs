//! White Tiger B (Clone 2) — BaiHu boss part (magic debuff + burn + disorder controller).
//!
//! DDGC reference: Beast-type boss part from the BaiHu dungeon.
//! Tier 1 base stats: HP 90, DEF 30%, PROT 0.6, SPD 7, 2 turns/round.
//! Skills: allow_return, fire_soul_shadow, tiger_swing, thunder_shadow, jump.
//!
//! This unit is a clone-phase unit that appears alongside white_tiger_A.
//! Clone B focuses on magic damage with burn, disorder, and debuff/buff effects.
//! After press_attack_count reaches 2, both clones are destroyed and
//! white_tiger_C (final form) is summoned.
//!
//! Game-gaps:
//! - Multi-phase transition (A/B → C) not modeled
//! - Press-attack-count mechanic not modeled
//! - WhiteTigerField rank-press damage modifier not modeled
//! - PROT (0.6), MAGIC_PROT (0.2) not modeled in Archetype
//! - Position-based targeting not modeled
//! - "White Tiger Shuffle Target" on tiger_swing simplified to status marker
//! - "Disorder" on tiger_swing simplified to status marker
//! - "White Tiger Debuff 1" + "White Tiger Buff 1" on thunder_shadow
//!   simplified to generic debuff/buff markers
//! - `.move 3 0` on allow_return dropped (self-movement)
//! - 2 turns/round not modeled in Archetype

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// White Tiger B (Clone 2) base archetype — tier 1 boss part stats from DDGC data.
///
/// HP 90, weapon damage derived from highest-damage skill (fire_soul_shadow/tiger_sing/
/// thunder_shadow: magic_dmg 2-5 avg 3.5), speed 7, defense 0.30 (30% dodge).
/// Controller role: magic clone with burn, disorder, and debuff/buff effects.
/// Crit 6% from fire_soul_shadow/tiger_swing/thunder_shadow.
/// PROT 0.6, MAGIC_PROT 0.2, Stun Resist 40%, Poison Resist 60%,
/// Bleed Resist 20%, Debuff Resist 40%, Move Resist 50%,
/// Burn Resist 20%, Frozen Resist 25% — all not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("White Tiger B"),
        side: CombatSide::Enemy,
        health: 90.0,
        max_health: 90.0,
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

// ── White Tiger B Skills ──────────────────────────────────────────────────

/// Allow Return — AoE stress attack (no damage).
///
/// DDGC reference: dmg 0-0, atk 72%, crit 0%,
/// launch rank 1, target ~1234 (AoE all ranks),
/// effect "Stress 1", move 3 0 (self forward 3).
/// Game-gap: AoE vs single-target distinction not modeled — targets AllEnemies.
/// Game-gap: self-movement (move 3 0) dropped.
pub fn allow_return() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("allow_return"),
        vec![EffectNode::apply_status("stress", Some(1))],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Fire Soul Shadow — magic attack with burn.
///
/// DDGC reference: magic_dmg 1-2 (avg 1.5), atk 72%, crit 6%,
/// launch rank 2, target ~14 (enemy ranks 1,4 excluding 2-3),
/// effect "Burn 1".
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn fire_soul_shadow() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("fire_soul_shadow"),
        vec![
            EffectNode::damage(1.5),
            EffectNode::apply_status("burn", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Tiger Swing — magic attack with shuffle and disorder.
///
/// DDGC reference: magic_dmg 2-5 (avg 3.5), atk 72%, crit 6%,
/// launch rank 3, target ~23 (enemy ranks 2-3),
/// effects "White Tiger Shuffle Target" + "Disorder".
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
/// Game-gap: "White Tiger Shuffle Target" simplified to status marker.
/// Game-gap: "Disorder" simplified to status marker.
pub fn tiger_swing() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("tiger_swing"),
        vec![
            EffectNode::damage(3.5),
            EffectNode::apply_status("shuffle", Some(1)),
            EffectNode::apply_status("disorder", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Thunder Shadow — magic attack with debuff and buff.
///
/// DDGC reference: magic_dmg 2-5 (avg 3.5), atk 72%, crit 6%,
/// launch rank 4, target ~234 (enemy ranks 2-4),
/// effects "White Tiger Debuff 1" + "White Tiger Buff 1" (self-buff).
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
/// Game-gap: "White Tiger Debuff 1" simplified to generic debuff marker.
/// Game-gap: "White Tiger Buff 1" (self-buff) simplified to status marker.
/// Game-gap: mixed targeting (enemy debuff + self buff) approximated;
///   debuff targets AllEnemies, buff targets SelfOnly — combined into AllEnemies.
pub fn thunder_shadow() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("thunder_shadow"),
        vec![
            EffectNode::damage(3.5),
            EffectNode::apply_status("debuff", Some(1)),
            EffectNode::apply_status("buff", Some(1)),
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

/// All 5 White Tiger B skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        allow_return(),
        fire_soul_shadow(),
        tiger_swing(),
        thunder_shadow(),
        jump(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_tiger_b_archetype_is_enemy_beast_controller() {
        let arch = archetype();
        assert_eq!(arch.name.0, "White Tiger B");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 90.0);
        assert_eq!(arch.max_health, 90.0);
        assert_eq!(arch.speed, 7.0);
        assert_eq!(arch.defense, 0.30, "white_tiger_B has 30% defense");
        assert_eq!(arch.crit_chance, 0.06, "white_tiger_B has 6% crit");
        assert_eq!(arch.attack, 3.5, "white_tiger_B attack from tiger_swing/thunder_shadow avg");
    }

    #[test]
    fn white_tiger_b_allow_return_applies_stress() {
        let skill = allow_return();
        assert_eq!(skill.id.0, "allow_return");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "allow_return must apply stress status");
    }

    #[test]
    fn white_tiger_b_fire_soul_shadow_deals_damage_and_burn() {
        let skill = fire_soul_shadow();
        assert_eq!(skill.id.0, "fire_soul_shadow");
        assert!(
            skill.effects.len() >= 2,
            "fire_soul_shadow should have damage + burn status"
        );
    }

    #[test]
    fn white_tiger_b_tiger_swing_deals_damage_shuffle_and_disorder() {
        let skill = tiger_swing();
        assert_eq!(skill.id.0, "tiger_swing");
        assert!(
            skill.effects.len() >= 3,
            "tiger_swing should have damage + shuffle + disorder"
        );
    }

    #[test]
    fn white_tiger_b_thunder_shadow_deals_damage_debuff_and_buff() {
        let skill = thunder_shadow();
        assert_eq!(skill.id.0, "thunder_shadow");
        assert!(
            skill.effects.len() >= 3,
            "thunder_shadow should have damage + debuff + buff"
        );
    }

    #[test]
    fn white_tiger_b_jump_applies_white_tiger_speed() {
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
    fn white_tiger_b_skill_pack_has_five_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 5);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"allow_return"), "missing allow_return");
        assert!(ids.contains(&"fire_soul_shadow"), "missing fire_soul_shadow");
        assert!(ids.contains(&"tiger_swing"), "missing tiger_swing");
        assert!(ids.contains(&"thunder_shadow"), "missing thunder_shadow");
        assert!(ids.contains(&"jump"), "missing jump");
    }

    #[test]
    fn white_tiger_b_burn_plus_disorder_plus_debuff_identity() {
        // The core identity of white_tiger_B is a magic clone with
        // burn, disorder, and debuff/buff control effects.
        let pack = skill_pack();

        let has_stress = pack.iter().any(|s| {
            s.id.0 == "allow_return"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stress")
                })
        });

        let has_burn = pack.iter().any(|s| {
            s.id.0 == "fire_soul_shadow"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("burn")
                })
        });

        let has_disorder = pack.iter().any(|s| {
            s.id.0 == "tiger_swing"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("disorder")
                })
        });

        let has_debuff = pack.iter().any(|s| {
            s.id.0 == "thunder_shadow"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("debuff")
                })
        });

        let has_jump = pack.iter().any(|s| {
            s.id.0 == "jump"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("white_tiger_speed")
                })
        });

        assert!(has_stress, "white_tiger_B must have stress mechanic");
        assert!(has_burn, "white_tiger_B must have burn mechanic");
        assert!(has_disorder, "white_tiger_B must have disorder mechanic");
        assert!(has_debuff, "white_tiger_B must have debuff mechanic");
        assert!(has_jump, "white_tiger_B must have jump/reposition mechanic");
    }
}