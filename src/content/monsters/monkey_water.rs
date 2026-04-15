//! Monkey Water — XuanWu bruiser family (stress-tag melee + charge).
//!
//! DDGC reference: Unholy-type bruiser monster from the XuanWu dungeon.
//! Tier 1 base stats: HP 98, DEF 7.5%, PROT 0.3, SPD 6, crit 6%.
//! Skills: base_melee (dmg + stress tag), rush (dmg + stress tag + move),
//!         stress (dmg + stress), move (reposition).
//!
//! This family's defining identity is rush-plus-stress: the monkey charges
//! back-rank targets with a stress-tag melee, applies stress at range, and
//! repositions with a forward rush that also tags targets.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Monkey Water base archetype — tier 1 stats from DDGC data.
///
/// HP 98, weapon damage derived from base_melee skill (23–36 avg 29.5),
/// speed 6, defense 0.075 (7.5% dodge), crit 6% (from rush).
/// Bruiser role: heavy stress-tag melee with PROT 0.3 and MAGIC_PROT 0.6 (not modeled in Archetype).
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Monkey Water"),
        side: CombatSide::Enemy,
        health: 98.0,
        max_health: 98.0,
        attack: 29.5,
        defense: 0.075,
        speed: 6.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.06,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Monkey Water Skills ─────────────────────────────────────────────────────

/// Base Melee — melee attack that deals high damage and applies stress tag.
///
/// DDGC reference: dmg 23–36 (avg 29.5), atk 81%, crit 3%,
/// effect "Stress 100 Tag", launch 12, target 12.
/// Game-gap: "Stress 100 Tag" (guaranteed stress + tag debuff) is modeled as
/// two separate status effects — the tagging mechanic is a game-gap.
/// Game-gap: launch/target rank positioning is not modeled — approximated as AllEnemies.
pub fn base_melee() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("base_melee"),
        vec![
            EffectNode::damage(29.5),
            EffectNode::apply_status("stress", Some(2)),
            EffectNode::apply_status("tag", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Rush — melee charge that deals damage and applies stress tag while moving forward.
///
/// DDGC reference: dmg 15–24 (avg 19.5), atk 82.5%, crit 6%,
/// effect "Stress 100 Tag", launch 12, target 34, move 0 1.
/// Game-gap: "Stress 100 Tag" is modeled as two status effects.
/// Game-gap: position-based targeting (launch 12, target 34) is not modeled —
/// approximated as AllEnemies. The self-movement (.move 0 1) is dropped as a game-gap.
pub fn rush() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("rush"),
        vec![
            EffectNode::damage(19.5),
            EffectNode::apply_status("stress", Some(2)),
            EffectNode::apply_status("tag", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Stress — ranged attack that deals damage and applies stress.
///
/// DDGC reference: dmg 15–24 (avg 19.5), atk 82.5%, crit 0%,
/// effect "Stress 2", launch 12, target 1234.
/// Game-gap: launch/target rank positioning is not modeled — approximated as AllEnemies.
pub fn stress() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("stress"),
        vec![
            EffectNode::damage(19.5),
            EffectNode::apply_status("stress", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Move — repositioning skill (self forward 1).
///
/// DDGC reference: atk 0%, dmg 0–0, launch 34, target @23, move 0 1.
/// Game-gap: position-based targeting and move direction are not modeled —
/// approximated as push(1) with SelfOnly.
pub fn move_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("move"),
        vec![EffectNode::push(1)],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// All 4 Monkey Water skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![base_melee(), rush(), stress(), move_skill()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monkey_water_archetype_is_enemy_unholy_bruiser() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Monkey Water");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 98.0);
        assert_eq!(arch.max_health, 98.0);
        assert_eq!(arch.speed, 6.0, "monkey_water has SPD 6");
        assert_eq!(arch.defense, 0.075, "monkey_water has 7.5% defense");
        assert_eq!(arch.crit_chance, 0.06, "monkey_water has 6% crit from rush");
        assert_eq!(arch.attack, 29.5, "monkey_water attack from base_melee avg");
    }

    #[test]
    fn monkey_water_base_melee_deals_damage_and_applies_stress_tag() {
        let skill = base_melee();
        assert_eq!(skill.id.0, "base_melee");
        assert!(
            skill.effects.len() >= 3,
            "base_melee should have damage + stress + tag effects"
        );
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        assert!(has_damage, "base_melee must have damage effect");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "base_melee must apply stress status");
        let has_tag = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("tag")
        });
        assert!(has_tag, "base_melee must apply tag status");
    }

    #[test]
    fn monkey_water_rush_deals_damage_and_applies_stress_tag() {
        let skill = rush();
        assert_eq!(skill.id.0, "rush");
        assert!(
            skill.effects.len() >= 3,
            "rush should have damage + stress + tag effects"
        );
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        assert!(has_damage, "rush must have damage effect");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "rush must apply stress status");
        let has_tag = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("tag")
        });
        assert!(has_tag, "rush must apply tag status");
    }

    #[test]
    fn monkey_water_stress_deals_damage_and_applies_stress() {
        let skill = stress();
        assert_eq!(skill.id.0, "stress");
        assert!(
            skill.effects.len() >= 2,
            "stress should have damage + stress effects"
        );
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        assert!(has_damage, "stress must have damage effect");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "stress must apply stress status");
    }

    #[test]
    fn monkey_water_move_skill_is_self_only_push() {
        let skill = move_skill();
        assert_eq!(skill.id.0, "move");
        assert!(
            skill.effects.len() >= 1,
            "move should have at least one effect"
        );
        let has_push = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Push));
        assert!(has_push, "move must have push effect");
    }

    #[test]
    fn monkey_water_skill_pack_has_four_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 4);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"base_melee"), "missing base_melee skill");
        assert!(ids.contains(&"rush"), "missing rush skill");
        assert!(ids.contains(&"stress"), "missing stress skill");
        assert!(ids.contains(&"move"), "missing move skill");
    }

    #[test]
    fn monkey_water_rush_plus_stress_identity() {
        // The core identity of monkey_water is rush + stress-tag melee.
        // This test preserves that identity.
        let pack = skill_pack();
        let has_rush_with_stress_tag = pack.iter().any(|s| {
            s.id.0 == "rush"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Damage)
                })
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stress")
                })
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("tag")
                })
        });
        let has_stress = pack.iter().any(|s| {
            s.id.0 == "stress"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Damage)
                })
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stress")
                })
        });
        assert!(
            has_rush_with_stress_tag,
            "monkey_water must have rush skill with damage, stress, and tag"
        );
        assert!(
            has_stress,
            "monkey_water must have stress skill with damage and stress"
        );
    }
}