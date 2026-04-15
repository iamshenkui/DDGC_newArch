//! Robber Melee — QingLong skirmisher family (debuff + bleed).
//!
//! DDGC reference: Man-type melee monster from the QingLong dungeon.
//! Tier 1 base stats: HP 15, DEF 15%, PROT 0.5, SPD 5.
//! Skills: normal_attack (melee), bleed (melee + bleed DOT),
//! smoke_bomb (AoE accuracy + debuff resist debuff), move.
//!
//! This family is distinct from robber_ranged: it is a low-HP front-line
//! debuffer that uses smoke_bomb to disable hero accuracy and debuff resistance.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Robber Melee base archetype — tier 1 stats from DDGC data.
///
/// HP 15, weapon damage derived from normal_attack skill (2–4 avg 3.0),
/// speed 5, dodge 15% mapped to defense field as 0.15.
/// Skirmisher role: low-HP melee that debuffs heroes from the front line.
/// Crit 12% from normal_attack skill.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Robber Melee"),
        side: CombatSide::Enemy,
        health: 15.0,
        max_health: 15.0,
        attack: 3.0,
        defense: 0.15,
        speed: 5.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.12,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Robber Melee Skills ─────────────────────────────────────────────────────

/// Normal Attack — basic melee strike from front ranks.
///
/// DDGC reference: dmg 2–4 (avg 3.0), atk 64%, crit 12%,
/// launch ranks 1–2, target ranks 1–2.
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

/// Bleed — melee attack that applies a bleeding wound.
///
/// DDGC reference: dmg 1–2 (avg 1.5), atk 64%, crit 0%,
/// effect "Bleed 1", launch ranks 1–2, target ranks 1–2.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn bleed() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("bleed"),
        vec![
            EffectNode::damage(1.5),
            EffectNode::apply_status("bleed", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Smoke Bomb — AoE debuff that reduces accuracy and debuff resistance.
///
/// DDGC reference: dmg 0–0, atk 64%, crit 0%,
/// effects "Acc Debuff 10" "Debuff Resist 50",
/// launch rank 1, target ~12 (AoE ranks 1–2).
/// Game-gap: position-based targeting and AoE vs single-target distinction
/// not modeled — targets AllEnemies.
pub fn smoke_bomb() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("smoke_bomb"),
        vec![
            EffectNode::apply_status("accuracy_debuff", Some(10)),
            EffectNode::apply_status("debuff_resist_down", Some(50)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Move Skill — reposition forward.
///
/// DDGC reference: atk 0%, dmg 0–0, .move 0 1,
/// launch ranks 3–4, target @23 (self/allies).
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

/// All 4 Robber Melee skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![normal_attack(), bleed(), smoke_bomb(), move_skill()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn robber_melee_archetype_is_enemy_man_skirmisher() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Robber Melee");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 15.0);
        assert_eq!(arch.max_health, 15.0);
        assert_eq!(arch.speed, 5.0);
        assert_eq!(arch.defense, 0.15, "robber_melee has 15% defense");
        assert_eq!(arch.crit_chance, 0.12, "robber_melee has 12% crit");
        assert_eq!(arch.attack, 3.0, "robber_melee attack from normal_attack avg");
    }

    #[test]
    fn robber_melee_normal_attack_deals_damage() {
        let skill = normal_attack();
        assert_eq!(skill.id.0, "normal_attack");
        assert!(
            !skill.effects.is_empty(),
            "normal_attack should have damage effect"
        );
    }

    #[test]
    fn robber_melee_bleed_applies_bleed_status() {
        let skill = bleed();
        assert_eq!(skill.id.0, "bleed");
        assert!(
            skill.effects.len() >= 2,
            "bleed should have damage + bleed status"
        );
        let has_bleed = skill
            .effects
            .iter()
            .any(|e| e.status_kind.as_deref() == Some("bleed"));
        assert!(has_bleed, "bleed skill must apply bleed status");
    }

    #[test]
    fn robber_melee_smoke_bomb_applies_debuffs() {
        let skill = smoke_bomb();
        assert_eq!(skill.id.0, "smoke_bomb");
        let has_acc_debuff = skill
            .effects
            .iter()
            .any(|e| e.status_kind.as_deref() == Some("accuracy_debuff"));
        let has_debuff_resist = skill
            .effects
            .iter()
            .any(|e| e.status_kind.as_deref() == Some("debuff_resist_down"));
        assert!(
            has_acc_debuff,
            "smoke_bomb must apply accuracy_debuff status"
        );
        assert!(
            has_debuff_resist,
            "smoke_bomb must apply debuff_resist_down status"
        );
    }

    #[test]
    fn robber_melee_skill_pack_has_four_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 4);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"normal_attack"), "missing normal_attack skill");
        assert!(ids.contains(&"bleed"), "missing bleed skill");
        assert!(ids.contains(&"smoke_bomb"), "missing smoke_bomb skill");
        assert!(ids.contains(&"move"), "missing move skill");
    }

    #[test]
    fn robber_melee_smoke_bomb_plus_bleed_identity() {
        // The core identity of robber_melee is smoke_bomb (AoE debuff)
        // plus bleed (melee DOT). This test preserves that identity.
        let pack = skill_pack();
        let has_smoke_bomb_debuff = pack.iter().any(|s| {
            s.id.0 == "smoke_bomb"
                && s.effects.iter().any(|e| {
                    e.status_kind.as_deref() == Some("accuracy_debuff")
                        || e.status_kind.as_deref() == Some("debuff_resist_down")
                })
        });
        let has_bleed_dot = pack.iter().any(|s| {
            s.id.0 == "bleed"
                && s.effects
                    .iter()
                    .any(|e| e.status_kind.as_deref() == Some("bleed"))
        });
        assert!(
            has_smoke_bomb_debuff,
            "robber_melee must have smoke_bomb with debuff effects"
        );
        assert!(
            has_bleed_dot,
            "robber_melee must have bleed skill with bleed status"
        );
    }
}
