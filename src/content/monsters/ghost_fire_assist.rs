//! Ghost Fire Assist — ZhuQue support family (ally buff + self-split on death).
//!
//! DDGC reference: Eldritch-type support monster from the ZhuQue dungeon.
//! Tier 1 base stats: HP 72, DEF 7.5%, PROT 0.3, SPD 6.
//! Skills: assist (ally buff), buff_self (damage + self DEF buff), ghost_fire_split (summon).
//!
//! This family's defining identity is a pure support unit that buffs allies
//! with attack/crit/damage bonuses, defends itself with a DEF buff while
//! dealing minor damage, and can split into a new ghost fire on death.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Ghost Fire Assist base archetype — tier 1 stats from DDGC data.
///
/// HP 72, weapon damage derived from buff_self skill (9–15 avg 12),
/// speed 6, defense 0.075 (7.5% dodge).
/// Support role: ally-buffer with PROT 0.3 (not modeled in Archetype).
/// Crit 0% — buff_self has no crit at tier 1.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Ghost Fire Assist"),
        side: CombatSide::Enemy,
        health: 72.0,
        max_health: 72.0,
        attack: 12.0,
        defense: 0.075,
        speed: 6.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.0,
        dodge: 0.0,
    }
}

// ── Ghost Fire Assist Skills ─────────────────────────────────────────────

/// Assist — zero-damage ally buff.
///
/// DDGC reference: dmg 0–0, atk 100%, crit 0%,
/// target @1234 (any ally rank, cannot target self),
/// effect "Ghost Fire Assist Buff": attack_rating_add +7%,
/// crit_chance_add +5%, damage_percent_add +25%.
/// Game-gap: DDGC excludes self from assist targeting — AllAllies includes
/// the performer. The specific stat buffs (+ATK, +CRIT, +DMG) are not
/// individually modeled; captured as a single "assist_buff" status marker.
pub fn assist() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("assist"),
        vec![EffectNode::apply_status("assist_buff", Some(1))],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Buff Self — ranged attack with a self-DEF buff side effect.
///
/// DDGC reference: dmg 9–15 (avg 12), atk 82.5%, crit 0%,
/// target 1234 (any rank), effect "Ghost Fire Self Buff 1":
/// defense_rating_add +5% applied to performer (self).
/// Game-gap: the self-DEF buff targets the performer in DDGC regardless
/// of who the attack hits, but the framework applies all effects to the
/// skill's target_selector result. The DEF buff side effect is not modeled
/// here — only the damage portion is captured.
pub fn buff_self() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("buff_self"),
        vec![EffectNode::damage(12.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Ghost Fire Split — zero-damage summon skill.
///
/// DDGC reference: dmg 0–0, atk 100%, crit 0%,
/// effect "Ghost Fire Summon 1": 50/50 chance to summon either
/// ghost_fire_assist_1 or ghost_fire_damage_1, apply_once true.
/// 3-round cooldown in AI brain (space_skill type).
/// Modeled as a self-applied "split" status marker to preserve identity.
/// Game-gap: the actual summon mechanic (spawning a new ghost fire unit
/// of either type) is not modeled in the framework.
pub fn ghost_fire_split() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("ghost_fire_split"),
        vec![EffectNode::apply_status("split", Some(1))],
        TargetSelector::SelfOnly,
        1,
        Some(3),
    )
}

/// All 3 Ghost Fire Assist skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![assist(), buff_self(), ghost_fire_split()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ghost_fire_assist_archetype_is_enemy_eldritch_support() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Ghost Fire Assist");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 72.0);
        assert_eq!(arch.max_health, 72.0);
        assert_eq!(arch.speed, 6.0);
        assert_eq!(arch.defense, 0.075, "ghost_fire_assist has 7.5% defense");
        assert_eq!(arch.crit_chance, 0.0, "ghost_fire_assist has 0% crit");
        assert_eq!(arch.attack, 12.0, "ghost_fire_assist attack from buff_self avg");
    }

    #[test]
    fn ghost_fire_assist_assist_applies_ally_buff() {
        let skill = assist();
        assert_eq!(skill.id.0, "assist");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllAllies",
            "assist targets all allies"
        );
        let has_buff = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("assist_buff")
        });
        assert!(has_buff, "assist must apply assist_buff status");
    }

    #[test]
    fn ghost_fire_assist_buff_self_deals_damage() {
        let skill = buff_self();
        assert_eq!(skill.id.0, "buff_self");
        assert!(
            skill.effects.len() >= 1,
            "buff_self should have damage effect"
        );
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        assert!(has_damage, "buff_self must have damage effect");
    }

    #[test]
    fn ghost_fire_assist_ghost_fire_split_applies_split_status() {
        let skill = ghost_fire_split();
        assert_eq!(skill.id.0, "ghost_fire_split");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "ghost_fire_split targets self"
        );
        let has_split = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("split")
        });
        assert!(has_split, "ghost_fire_split must apply split status");
        assert_eq!(skill.cooldown, Some(3), "ghost_fire_split has 3-round cooldown");
    }

    #[test]
    fn ghost_fire_assist_skill_pack_has_three_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 3);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"assist"), "missing assist skill");
        assert!(ids.contains(&"buff_self"), "missing buff_self skill");
        assert!(ids.contains(&"ghost_fire_split"), "missing ghost_fire_split skill");
    }

    #[test]
    fn ghost_fire_assist_assist_plus_split_identity() {
        // The core identity of ghost_fire_assist is assist (ally buff) plus split-on-death.
        // This test preserves that identity.
        let pack = skill_pack();
        let has_assist = pack.iter().any(|s| {
            s.id.0 == "assist"
                && format!("{:?}", s.target_selector) == "AllAllies"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("assist_buff")
                })
        });
        let has_split = pack.iter().any(|s| {
            s.id.0 == "ghost_fire_split"
                && format!("{:?}", s.target_selector) == "SelfOnly"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("split")
                })
        });
        assert!(
            has_assist,
            "ghost_fire_assist must have assist skill targeting all allies with buff"
        );
        assert!(
            has_split,
            "ghost_fire_assist must have ghost_fire_split self-applied status"
        );
    }
}
