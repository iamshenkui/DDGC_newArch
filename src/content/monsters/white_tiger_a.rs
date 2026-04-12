//! White Tiger A (Clone 1) — BaiHu boss part (pull + stun + bleed skirmisher).
//!
//! DDGC reference: Beast-type boss part from the BaiHu dungeon.
//! Tier 1 base stats: HP 90, DEF 30%, PROT 0.6, SPD 7, 2 turns/round.
//! Skills: drag, angry_eyes, pounce, pounce_bite, jump.
//!
//! This unit is a clone-phase unit that appears alongside white_tiger_B.
//! After press_attack_count reaches 2, both clones are destroyed and
//! white_tiger_C (final form) is summoned. Clone A focuses on physical
//! damage with pull, stun, and bleed effects.
//!
//! Game-gaps:
//! - Multi-phase transition (A/B → C) not modeled
//! - Press-attack-count mechanic not modeled
//! - WhiteTigerField rank-press damage modifier not modeled
//! - PROT (0.6), MAGIC_PROT (0.2) not modeled in Archetype
//! - Position-based targeting (launch 1-4, target 1234/~1234) not modeled
//! - `.move N M` self-movement on drag/pounce/pounce_bite dropped
//! - 2 turns/round not modeled in Archetype

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// White Tiger A (Clone 1) base archetype — tier 1 boss part stats from DDGC data.
///
/// HP 90, weapon damage derived from drag/pounce skill avg (6-8 avg 7),
/// speed 7, defense 0.30 (30% dodge).
/// Skirmisher role: physical clone with pull, stun, and bleed.
/// Crit 6% from drag/pounce/pounce_bite.
/// PROT 0.6, MAGIC_PROT 0.2, Stun Resist 40%, Poison Resist 60%,
/// Bleed Resist 20%, Debuff Resist 40%, Move Resist 50%,
/// Burn Resist 20%, Frozen Resist 25% — all not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("White Tiger A"),
        side: CombatSide::Enemy,
        health: 90.0,
        max_health: 90.0,
        attack: 7.0,
        defense: 0.30,
        speed: 7.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.06,
        dodge: 0.0,
    }
}

// ── White Tiger A Skills ──────────────────────────────────────────────────

/// Drag — ranged attack with pull.
///
/// DDGC reference: dmg 6-8 (avg 7), atk 72%, crit 6%,
/// launch rank 1, target ranks 1,3,4, effect "Pull 2A",
/// move 1 0 (self forward 1).
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
/// Game-gap: self-movement (move 1 0) dropped.
pub fn drag() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("drag"),
        vec![
            EffectNode::damage(7.0),
            EffectNode::pull(2),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Angry Eyes — stress-only attack (no damage).
///
/// DDGC reference: dmg 0-0, atk 72%, crit 0%,
/// launch rank 2, target ranks 1,2,4, effect "Stress 2".
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn angry_eyes() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("angry_eyes"),
        vec![EffectNode::apply_status("stress", Some(2))],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Pounce — ranged attack with stun.
///
/// DDGC reference: dmg 6-8 (avg 7), atk 72%, crit 6%,
/// launch rank 3, target ranks 1,2,3, effect "Stun 1",
/// move 0 1 (self back 1).
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
/// Game-gap: self-movement (move 0 1) dropped.
pub fn pounce() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("pounce"),
        vec![
            EffectNode::damage(7.0),
            EffectNode::apply_status("stun", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Pounce Bite — ranged attack with push and bleed.
///
/// DDGC reference: dmg 3-4 (avg 3.5), atk 72%, crit 6%,
/// launch rank 4, target ranks 2,3,4,
/// effects "Push 1A" + "Bleed 1", move 0 2 (self forward 2).
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
/// Game-gap: self-movement (move 0 2) dropped.
pub fn pounce_bite() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("pounce_bite"),
        vec![
            EffectNode::damage(3.5),
            EffectNode::push(1),
            EffectNode::apply_status("bleed", Some(1)),
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

/// All 5 White Tiger A skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        drag(),
        angry_eyes(),
        pounce(),
        pounce_bite(),
        jump(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_tiger_a_archetype_is_enemy_beast_support() {
        let arch = archetype();
        assert_eq!(arch.name.0, "White Tiger A");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 90.0);
        assert_eq!(arch.max_health, 90.0);
        assert_eq!(arch.speed, 7.0);
        assert_eq!(arch.defense, 0.30, "white_tiger_A has 30% defense");
        assert_eq!(arch.crit_chance, 0.06, "white_tiger_A has 6% crit");
        assert_eq!(arch.attack, 7.0, "white_tiger_A attack from drag/pounce avg");
    }

    #[test]
    fn white_tiger_a_drag_deals_damage_and_pull() {
        let skill = drag();
        assert_eq!(skill.id.0, "drag");
        assert!(
            skill.effects.len() >= 2,
            "drag should have damage + pull effect"
        );
    }

    #[test]
    fn white_tiger_a_angry_eyes_applies_stress() {
        let skill = angry_eyes();
        assert_eq!(skill.id.0, "angry_eyes");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "angry_eyes must apply stress status");
    }

    #[test]
    fn white_tiger_a_pounce_deals_damage_and_stun() {
        let skill = pounce();
        assert_eq!(skill.id.0, "pounce");
        assert!(
            skill.effects.len() >= 2,
            "pounce should have damage + stun status"
        );
    }

    #[test]
    fn white_tiger_a_pounce_bite_deals_damage_push_and_bleed() {
        let skill = pounce_bite();
        assert_eq!(skill.id.0, "pounce_bite");
        assert!(
            skill.effects.len() >= 3,
            "pounce_bite should have damage + push + bleed"
        );
    }

    #[test]
    fn white_tiger_a_jump_applies_white_tiger_speed() {
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
    fn white_tiger_a_skill_pack_has_five_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 5);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"drag"), "missing drag");
        assert!(ids.contains(&"angry_eyes"), "missing angry_eyes");
        assert!(ids.contains(&"pounce"), "missing pounce");
        assert!(ids.contains(&"pounce_bite"), "missing pounce_bite");
        assert!(ids.contains(&"jump"), "missing jump");
    }

    #[test]
    fn white_tiger_a_pull_plus_stun_plus_bleed_identity() {
        // The core identity of white_tiger_A is a physical clone with
        // pull, stun, and bleed mechanics for positional disruption.
        let pack = skill_pack();

        let has_pull = pack.iter().any(|s| {
            s.id.0 == "drag"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Pull)
                })
        });

        let has_stun = pack.iter().any(|s| {
            s.id.0 == "pounce"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stun")
                })
        });

        let has_bleed = pack.iter().any(|s| {
            s.id.0 == "pounce_bite"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("bleed")
                })
        });

        let has_stress = pack.iter().any(|s| {
            s.id.0 == "angry_eyes"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stress")
                })
        });

        let has_jump = pack.iter().any(|s| {
            s.id.0 == "jump"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("white_tiger_speed")
                })
        });

        assert!(has_pull, "white_tiger_A must have pull mechanic");
        assert!(has_stun, "white_tiger_A must have stun mechanic");
        assert!(has_bleed, "white_tiger_A must have bleed mechanic");
        assert!(has_stress, "white_tiger_A must have stress mechanic");
        assert!(has_jump, "white_tiger_A must have jump/reposition mechanic");
    }
}