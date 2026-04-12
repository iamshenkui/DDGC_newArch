//! Robber Ranged — QingLong skirmisher family (stun + multi-shot).
//!
//! DDGC reference: Man-type ranged monster from the QingLong dungeon.
//! Tier 1 base stats: HP 10, DEF 15%, PROT 0.2, SPD 3.
//! Skills: normal_attack (ranged), multiple_shot (AoE ranged),
//! throw_stone (ranged stun + debuff), move.
//!
//! This family is distinct from robber_melee: it is a low-HP back-rank
//! skirmisher that uses multiple_shot for AoE pressure and throw_stone
//! for stun utility, rather than front-line debuff/bleed.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Robber Ranged base archetype — tier 1 stats from DDGC data.
///
/// HP 10, weapon damage derived from normal_attack skill (2–4 avg 3.0),
/// speed 3, dodge 15% mapped to defense field as 0.15.
/// Skirmisher role: low-HP back-rank ranged that stuns and multi-targets.
/// Crit 12% from normal_attack skill.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Robber Ranged"),
        side: CombatSide::Enemy,
        health: 10.0,
        max_health: 10.0,
        attack: 3.0,
        defense: 0.15,
        speed: 3.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.12,
        dodge: 0.0,
    }
}

// ── Robber Ranged Skills ────────────────────────────────────────────────────

/// Normal Attack — basic ranged strike from back ranks.
///
/// DDGC reference: dmg 2–4 (avg 3.0), atk 64%, crit 12%,
/// launch ranks 3–4, target ranks 2–3–4.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn normal_attack() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("normal_attack"),
        vec![EffectNode::damage(3.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Multiple Shot — AoE ranged attack hitting multiple targets.
///
/// DDGC reference: dmg 2–4 (avg 3.0), atk 48%, crit 10.5%,
/// launch ranks 3–4, target ~34 (AoE ranks 3–4).
/// Game-gap: position-based targeting and AoE vs single-target distinction
/// not modeled — targets AllEnemies.
pub fn multiple_shot() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("multiple_shot"),
        vec![EffectNode::damage(3.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Throw Stone — ranged stun utility that applies stun and debuffs stun resist.
///
/// DDGC reference: dmg 0–0, atk 64%, crit 0%,
/// effects "Stun 1" "Stun Resist 50",
/// launch ranks 3–4, target ranks 1–2–3.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn throw_stone() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("throw_stone"),
        vec![
            EffectNode::apply_status("stun", Some(1)),
            EffectNode::apply_status("stun_resist_down", Some(50)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Move Skill — reposition backward.
///
/// DDGC reference: atk 0%, dmg 0–0, .move 1 0,
/// launch ranks 1–2, target @23 (self/allies).
/// Approximated as push(1) with SelfOnly targeting.
pub fn move_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("move"),
        vec![EffectNode::push(1)],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// All 4 Robber Ranged skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![normal_attack(), multiple_shot(), throw_stone(), move_skill()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn robber_ranged_archetype_is_enemy_man_skirmisher() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Robber Ranged");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 10.0);
        assert_eq!(arch.max_health, 10.0);
        assert_eq!(arch.speed, 3.0, "robber_ranged has speed 3");
        assert_eq!(arch.defense, 0.15, "robber_ranged has 15% defense");
        assert_eq!(arch.crit_chance, 0.12, "robber_ranged has 12% crit");
        assert_eq!(arch.attack, 3.0, "robber_ranged attack from normal_attack avg");
    }

    #[test]
    fn robber_ranged_normal_attack_deals_damage() {
        let skill = normal_attack();
        assert_eq!(skill.id.0, "normal_attack");
        assert!(
            skill.effects.len() >= 1,
            "normal_attack should have damage effect"
        );
    }

    #[test]
    fn robber_ranged_multiple_shot_deals_damage() {
        let skill = multiple_shot();
        assert_eq!(skill.id.0, "multiple_shot");
        assert!(
            skill.effects.len() >= 1,
            "multiple_shot should have damage effect"
        );
    }

    #[test]
    fn robber_ranged_throw_stone_applies_stun() {
        let skill = throw_stone();
        assert_eq!(skill.id.0, "throw_stone");
        let has_stun = skill
            .effects
            .iter()
            .any(|e| e.status_kind.as_deref() == Some("stun"));
        let has_stun_resist_down = skill
            .effects
            .iter()
            .any(|e| e.status_kind.as_deref() == Some("stun_resist_down"));
        assert!(has_stun, "throw_stone must apply stun status");
        assert!(
            has_stun_resist_down,
            "throw_stone must apply stun_resist_down status"
        );
    }

    #[test]
    fn robber_ranged_skill_pack_has_four_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 4);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"normal_attack"), "missing normal_attack skill");
        assert!(ids.contains(&"multiple_shot"), "missing multiple_shot skill");
        assert!(ids.contains(&"throw_stone"), "missing throw_stone skill");
        assert!(ids.contains(&"move"), "missing move skill");
    }

    #[test]
    fn robber_ranged_throw_stone_plus_multiple_shot_identity() {
        // The core identity of robber_ranged is throw_stone (stun utility)
        // plus multiple_shot (AoE pressure). This test preserves that identity.
        let pack = skill_pack();
        let has_throw_stone_stun = pack.iter().any(|s| {
            s.id.0 == "throw_stone"
                && s.effects
                    .iter()
                    .any(|e| e.status_kind.as_deref() == Some("stun"))
        });
        let has_multi_shot_damage = pack.iter().any(|s| {
            s.id.0 == "multiple_shot"
                && s.effects
                    .iter()
                    .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage))
        });
        assert!(
            has_throw_stone_stun,
            "robber_ranged must have throw_stone with stun effect"
        );
        assert!(
            has_multi_shot_damage,
            "robber_ranged must have multiple_shot with damage effect"
        );
    }
}
