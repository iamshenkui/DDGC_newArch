//! Vermilion Bird Tail A — ZhuQue boss part (stress/disorder control + protect buff).
//!
//! DDGC reference: Beast-type boss part from the ZhuQue dungeon.
//! Shares health pool with main body (HP 0 in DDGC → modeled as 160 for framework).
//! Tier 1 base stats: DEF 30%, PROT 0.5, SPD 7, 1 turn/round.
//! Skills: deterrence, confused, ignore_def, protect.
//!
//! This part's identity is a control-oriented tail that harasses heroes
//! with stress and disorder, and can protect the main body with a
//! damage-reduction buff. It is untargetable in DDGC
//! (is_valid_enemy_target False).
//!
//! Game-gaps:
//! - Shared health pool with main body not modeled (has own HP)
//! - Untargetable status (is_valid_enemy_target False) not modeled
//! - PROT (0.5) and MAGIC_PROT (0.8) not modeled in Archetype
//! - Stun/Move/etc. resistances not modeled in Archetype
//! - Position-based targeting (launch 3, target 1234/23) not modeled
//! - is_ignore_def on ignore_def captured by name only

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Vermilion Bird Tail A base archetype — tier 1 boss part stats from DDGC data.
///
/// HP 160 (shared with main body in DDGC — modeled as same HP since shared
/// health mechanic is not modeled in framework), no significant damage
/// (attack 2.0 from ignore_def avg), speed 7, defense 0.30 (30% dodge).
/// Support role: stress/disorder control + protection buff for main body.
/// PROT 0.5, MAGIC_PROT 0.8, Stun Resist 40%, Poison Resist 60%,
/// Bleed Resist 20%, Debuff Resist 40%, Move Resist 50%, Burn Resist 100%,
/// Frozen Resist 25% — all not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Vermilion Bird Tail A"),
        side: CombatSide::Enemy,
        health: 160.0,
        max_health: 160.0,
        attack: 2.0,
        defense: 0.30,
        speed: 7.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.0,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Vermilion Bird Tail A Skills ─────────────────────────────────────────

/// Deterrence — applies stress + disorient to a hero.
///
/// DDGC reference: dmg 0-0, atk 64%, crit 0%,
/// effects "Stress 2" + "Disorient 1",
/// launch rank 3, target ranks 1-4.
/// Phase 1 skill: 40% chance in AI brain.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn deterrence() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("deterrence"),
        vec![
            EffectNode::apply_status("stress", Some(2)),
            EffectNode::apply_status("disorient", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Confused — applies disorder to a hero.
///
/// DDGC reference: dmg 0-0, atk 64%, crit 0%,
/// effect "Disorder 1",
/// launch rank 3, target ranks 1-4.
/// Phase 1 skill: 40% chance in AI brain.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn confused() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("confused"),
        vec![EffectNode::apply_status("disorder", Some(1))],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Ignore Def — armor-piercing attack.
///
/// DDGC reference: dmg 1-3 (avg 2), atk 64%, crit 0%,
/// launch rank 3, target ranks 1-4, .is_ignore_def True.
/// Phase 1 skill: 30% chance in AI brain.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
/// Game-gap: is_ignore_def (bypass protection) captured by name only.
pub fn ignore_def() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("ignore_def"),
        vec![EffectNode::damage(2.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Protect — applies damage-reduction buff to main body.
///
/// DDGC reference: dmg 0-0, atk 0%, crit 0%,
/// effect "Vermilion Bird Tail 1 Buff" (-30% damage received for 3 rounds),
/// launch rank 3, target ranks 2-3 (ally = main body position).
/// Phase 2 skill: used when VB is about to use calm_nerves/explosion.
/// Game-gap: ally-targeting (ranks 2-3) simplified to AllAllies.
pub fn protect() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("protect"),
        vec![EffectNode::apply_status("dmg_reduction", Some(3))],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// All 4 Vermilion Bird Tail A skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        deterrence(),
        confused(),
        ignore_def(),
        protect(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vermilion_bird_tail_a_archetype_is_enemy_beast_support() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Vermilion Bird Tail A");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 160.0);
        assert_eq!(arch.max_health, 160.0);
        assert_eq!(arch.speed, 7.0);
        assert_eq!(arch.defense, 0.30, "tail_A has 30% defense");
        assert_eq!(arch.attack, 2.0, "tail_A attack from ignore_def avg");
    }

    #[test]
    fn vermilion_bird_tail_a_deterrence_applies_stress_and_disorient() {
        let skill = deterrence();
        assert_eq!(skill.id.0, "deterrence");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        let has_disorient = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("disorient")
        });
        assert!(has_stress, "deterrence must apply stress");
        assert!(has_disorient, "deterrence must apply disorient");
    }

    #[test]
    fn vermilion_bird_tail_a_confused_applies_disorder() {
        let skill = confused();
        assert_eq!(skill.id.0, "confused");
        let has_disorder = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("disorder")
        });
        assert!(has_disorder, "confused must apply disorder");
    }

    #[test]
    fn vermilion_bird_tail_a_ignore_def_deals_damage() {
        let skill = ignore_def();
        assert_eq!(skill.id.0, "ignore_def");
        assert!(
            skill.effects.len() >= 1,
            "ignore_def should have damage effect"
        );
    }

    #[test]
    fn vermilion_bird_tail_a_protect_applies_damage_reduction() {
        let skill = protect();
        assert_eq!(skill.id.0, "protect");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllAllies",
            "protect targets allies"
        );
        let has_reduction = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("dmg_reduction")
        });
        assert!(has_reduction, "protect must apply dmg_reduction status");
    }

    #[test]
    fn vermilion_bird_tail_a_skill_pack_has_four_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 4);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"deterrence"), "missing deterrence");
        assert!(ids.contains(&"confused"), "missing confused");
        assert!(ids.contains(&"ignore_def"), "missing ignore_def");
        assert!(ids.contains(&"protect"), "missing protect");
    }

    #[test]
    fn vermilion_bird_tail_a_stress_disorder_plus_protect_identity() {
        // The core identity of tail_A is control + protection:
        // stress/disorder harassment and damage-reduction buff for main body.
        let pack = skill_pack();

        let has_stress = pack.iter().any(|s| {
            s.id.0 == "deterrence"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stress")
                })
        });

        let has_disorder = pack.iter().any(|s| {
            s.id.0 == "confused"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("disorder")
                })
        });

        let has_protect = pack.iter().any(|s| {
            s.id.0 == "protect"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("dmg_reduction")
                })
        });

        assert!(has_stress, "tail_A must have stress harassment");
        assert!(has_disorder, "tail_A must have disorder control");
        assert!(has_protect, "tail_A must have protection buff");
    }
}
