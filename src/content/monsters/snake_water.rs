//! Snake Water — XuanWu control family (stun + blight).
//!
//! DDGC reference: Eldritch-type controller monster from the XuanWu dungeon.
//! Tier 1 base stats: HP 62, DEF 7.5%, PROT 0.4, SPD 6, crit 6%.
//! Skills: stun (low dmg + stun), poison_fang (moderate dmg + blight), move (reposition).
//!
//! This family's defining identity is stun-plus-blight control: the snake stuns
//! a front-rank target and applies blight through its fangs, with an even 50/50
//! AI weighting between the two offensive skills.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Snake Water base archetype — tier 1 stats from DDGC data.
///
/// HP 62, weapon damage derived from poison_fang skill (15–24 avg 19.5),
/// speed 6, defense 0.075 (7.5% dodge), crit 6% (from poison_fang).
/// Controller role: stun + blight control with PROT 0.4 and MAGIC_PROT 0.2 (not modeled in Archetype).
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Snake Water"),
        side: CombatSide::Enemy,
        health: 62.0,
        max_health: 62.0,
        attack: 19.5,
        defense: 0.075,
        speed: 6.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.06,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Snake Water Skills ─────────────────────────────────────────────────────

/// Stun — melee attack that deals low damage and applies stun.
///
/// DDGC reference: dmg 8–12 (avg 10), atk 81%, crit 0%,
/// effect "Weak Stun 3", launch 12, target 12.
/// Game-gap: launch/target rank positioning is not modeled — approximated as AllEnemies.
pub fn stun() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("stun"),
        vec![
            EffectNode::damage(10.0),
            EffectNode::apply_status("stun", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Poison Fang — melee attack that deals moderate damage and applies blight.
///
/// DDGC reference: dmg 15–24 (avg 19.5), atk 82.5%, crit 6%,
/// effect "New Blight 1", launch 12, target 1234.
/// Game-gap: launch/target rank positioning is not modeled — approximated as AllEnemies.
pub fn poison_fang() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("poison_fang"),
        vec![
            EffectNode::damage(19.5),
            EffectNode::apply_status("blight", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Move — repositioning skill (self forward 1).
///
/// DDGC reference: atk 0%, dmg 0–0, launch 34, target @23, move 0 forward 1.
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

/// All 3 Snake Water skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![stun(), poison_fang(), move_skill()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_water_archetype_is_enemy_eldritch_controller() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Snake Water");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 62.0);
        assert_eq!(arch.max_health, 62.0);
        assert_eq!(arch.speed, 6.0);
        assert_eq!(arch.defense, 0.075, "snake_water has 7.5% defense");
        assert_eq!(arch.crit_chance, 0.06, "snake_water has 6% crit from poison_fang");
        assert_eq!(arch.attack, 19.5, "snake_water attack from poison_fang avg");
    }

    #[test]
    fn snake_water_stun_deals_damage_and_applies_stun() {
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
    fn snake_water_poison_fang_deals_damage_and_applies_blight() {
        let skill = poison_fang();
        assert_eq!(skill.id.0, "poison_fang");
        assert!(
            skill.effects.len() >= 2,
            "poison_fang should have damage + blight effects"
        );
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        assert!(has_damage, "poison_fang must have damage effect");
        let has_blight = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("blight")
        });
        assert!(has_blight, "poison_fang must apply blight status");
    }

    #[test]
    fn snake_water_move_skill_is_self_only_push() {
        let skill = move_skill();
        assert_eq!(skill.id.0, "move");
        assert!(
            skill.effects.len() >= 1,
            "move should have at least one effect"
        );
        let has_push = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Push)
        });
        assert!(has_push, "move must have push effect");
    }

    #[test]
    fn snake_water_skill_pack_has_three_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 3);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"stun"), "missing stun skill");
        assert!(ids.contains(&"poison_fang"), "missing poison_fang skill");
        assert!(ids.contains(&"move"), "missing move skill");
    }

    #[test]
    fn snake_water_stun_plus_blight_identity() {
        // The core identity of snake_water is stun + blight control.
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
        let has_blight = pack.iter().any(|s| {
            s.id.0 == "poison_fang"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Damage)
                })
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("blight")
                })
        });
        assert!(
            has_stun,
            "snake_water must have stun skill with damage and stun status"
        );
        assert!(
            has_blight,
            "snake_water must have poison_fang skill with damage and blight status"
        );
    }
}
