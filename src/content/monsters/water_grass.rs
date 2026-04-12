//! Water Grass — XuanWu control family (stun + bleed + pull).
//!
//! DDGC reference: Eldritch-type controller monster from the XuanWu dungeon.
//! Tier 1 base stats: HP 62, DEF 7.5%, PROT 0.3, SPD 3, crit 6%.
//! Skills: stun (dmg + stun), puncture (dmg + bleed + debuff + disease),
//!         attack_crowd (AoE dmg), convolve (dmg + pull), move (reposition).
//!
//! This family's defining identity is stun-plus-bleed-plus-pull control:
//! the water grass stuns front-rank targets, bleeds back-rank targets through
//! puncture, and pulls distant enemies forward with convolve, while pressuring
//! all ranks with AoE attack_crowd.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Water Grass base archetype — tier 1 stats from DDGC data.
///
/// HP 62, weapon damage derived from puncture skill (12–23 avg 17.5),
/// speed 3, defense 0.075 (7.5% dodge), crit 6% (from puncture).
/// Controller role: stun + bleed + pull control with PROT 0.3 and MAGIC_PROT 0.6 (not modeled in Archetype).
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Water Grass"),
        side: CombatSide::Enemy,
        health: 62.0,
        max_health: 62.0,
        attack: 17.5,
        defense: 0.075,
        speed: 3.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.06,
        dodge: 0.0,
    }
}

// ── Water Grass Skills ─────────────────────────────────────────────────────

/// Stun — ranged attack that deals damage and applies stun.
///
/// DDGC reference: dmg 8–15 (avg 11.5), atk 81%, crit 0%,
/// effect "Weak Stun 3", launch 34, target 12.
/// Game-gap: launch/target rank positioning is not modeled — approximated as AllEnemies.
pub fn stun() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("stun"),
        vec![
            EffectNode::damage(11.5),
            EffectNode::apply_status("stun", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Puncture — ranged attack that deals damage and applies bleed plus debuffs.
///
/// DDGC reference: dmg 12–23 (avg 17.5), atk 82.5%, crit 6%,
/// effects "New Bleed 1" "Bleed Resist -20" "Disease Random 6",
/// launch 34, target 1234.
/// Game-gap: launch/target rank positioning is not modeled — approximated as AllEnemies.
/// Game-gap: "Disease Random 6" random disease table selection is not modeled.
pub fn puncture() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("puncture"),
        vec![
            EffectNode::damage(17.5),
            EffectNode::apply_status("bleed", Some(1)),
            EffectNode::apply_status("bleed_resist_down", Some(20)),
            EffectNode::apply_status("disease", Some(6)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Attack Crowd — AoE ranged attack that deals damage to all enemy ranks.
///
/// DDGC reference: dmg 8–15 (avg 11.5), atk 82.5%, crit 3%,
/// launch 34, target ~1234.
/// Game-gap: launch/target rank positioning is not modeled — approximated as AllEnemies.
pub fn attack_crowd() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("attack_crowd"),
        vec![EffectNode::damage(11.5)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Convolve — ranged attack that deals damage and pulls the target forward.
///
/// DDGC reference: dmg 8–15 (avg 11.5), atk 82.5%, crit 0%,
/// effect "Pull 2A", launch 34, target 34.
/// Game-gap: launch/target rank positioning is not modeled — approximated as AllEnemies.
pub fn convolve() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("convolve"),
        vec![
            EffectNode::damage(11.5),
            EffectNode::pull(2),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Move — repositioning skill (self forward 1).
///
/// DDGC reference: atk 0%, dmg 0–0, launch 12, target @23, move 1 0.
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

/// All 5 Water Grass skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![stun(), puncture(), attack_crowd(), convolve(), move_skill()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn water_grass_archetype_is_enemy_eldritch_controller() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Water Grass");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 62.0);
        assert_eq!(arch.max_health, 62.0);
        assert_eq!(arch.speed, 3.0, "water_grass has SPD 3");
        assert_eq!(arch.defense, 0.075, "water_grass has 7.5% defense");
        assert_eq!(arch.crit_chance, 0.06, "water_grass has 6% crit from puncture");
        assert_eq!(arch.attack, 17.5, "water_grass attack from puncture avg");
    }

    #[test]
    fn water_grass_stun_deals_damage_and_applies_stun() {
        let skill = stun();
        assert_eq!(skill.id.0, "stun");
        assert!(
            skill.effects.len() >= 2,
            "stun should have damage + stun effects"
        );
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        assert!(has_damage, "stun must have damage effect");
        let has_stun = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stun")
        });
        assert!(has_stun, "stun must apply stun status");
    }

    #[test]
    fn water_grass_puncture_deals_damage_and_applies_bleed_plus_debuffs() {
        let skill = puncture();
        assert_eq!(skill.id.0, "puncture");
        assert!(
            skill.effects.len() >= 4,
            "puncture should have damage + bleed + bleed_resist_down + disease effects"
        );
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        assert!(has_damage, "puncture must have damage effect");
        let has_bleed = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("bleed")
        });
        assert!(has_bleed, "puncture must apply bleed status");
        let has_bleed_resist_down = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("bleed_resist_down")
        });
        assert!(has_bleed_resist_down, "puncture must apply bleed_resist_down status");
        let has_disease = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("disease")
        });
        assert!(has_disease, "puncture must apply disease status");
    }

    #[test]
    fn water_grass_attack_crowd_deals_aoe_damage() {
        let skill = attack_crowd();
        assert_eq!(skill.id.0, "attack_crowd");
        assert!(
            skill.effects.len() >= 1,
            "attack_crowd should have at least one effect"
        );
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        assert!(has_damage, "attack_crowd must have damage effect");
    }

    #[test]
    fn water_grass_convolve_deals_damage_and_pulls() {
        let skill = convolve();
        assert_eq!(skill.id.0, "convolve");
        assert!(
            skill.effects.len() >= 2,
            "convolve should have damage + pull effects"
        );
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        assert!(has_damage, "convolve must have damage effect");
        let has_pull = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Pull));
        assert!(has_pull, "convolve must have pull effect");
    }

    #[test]
    fn water_grass_move_skill_is_self_only_push() {
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
    fn water_grass_skill_pack_has_five_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 5);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"stun"), "missing stun skill");
        assert!(ids.contains(&"puncture"), "missing puncture skill");
        assert!(ids.contains(&"attack_crowd"), "missing attack_crowd skill");
        assert!(ids.contains(&"convolve"), "missing convolve skill");
        assert!(ids.contains(&"move"), "missing move skill");
    }

    #[test]
    fn water_grass_stun_bleed_crowd_pull_identity() {
        // The core identity of water_grass is stun + bleed + crowd attack + pull control.
        // This test preserves that identity.
        let pack = skill_pack();
        let has_stun = pack.iter().any(|s| {
            s.id.0 == "stun"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stun")
                })
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Damage)
                })
        });
        let has_bleed = pack.iter().any(|s| {
            s.id.0 == "puncture"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Damage)
                })
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("bleed")
                })
        });
        let has_crowd = pack.iter().any(|s| {
            s.id.0 == "attack_crowd"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Damage)
                })
        });
        let has_pull = pack.iter().any(|s| {
            s.id.0 == "convolve"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Damage)
                })
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Pull)
                })
        });
        assert!(
            has_stun,
            "water_grass must have stun skill with damage and stun status"
        );
        assert!(
            has_bleed,
            "water_grass must have puncture skill with damage and bleed status"
        );
        assert!(
            has_crowd,
            "water_grass must have attack_crowd skill with damage"
        );
        assert!(
            has_pull,
            "water_grass must have convolve skill with damage and pull"
        );
    }
}
