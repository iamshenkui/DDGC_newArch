//! Lizard — BaiHu controller family (fast stun + stress pressure).
//!
//! DDGC reference: Eldritch-type monster from the BaiHu dungeon.
//! Tier 1 base stats: HP 62, DEF 7.5%, PROT 0.6, SPD 6.
//! Skills: stun (melee + Weak Stun 3), intimidate (AoE Stress 0),
//!         stress (low dmg + Stress 2), move (reposition forward).
//!
//! This family's defining identity is a deterministic stun → intimidate → stress
//! combo cycle that applies sustained control and stress pressure on the party.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Lizard base archetype — tier 1 stats from DDGC data.
///
/// HP 62, weapon damage derived from stun skill (8–12 avg 10.0),
/// speed 6, defense 0.075 (7.5% dodge).
/// Controller role: fast frontline with stun + stress combo.
/// PROT 0.6 (not modeled in Archetype).
/// Crit 0% from stun skill.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Lizard"),
        side: CombatSide::Enemy,
        health: 62.0,
        max_health: 62.0,
        attack: 10.0,
        defense: 0.075,
        speed: 6.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.0,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Lizard Skills ─────────────────────────────────────────────────────────

/// Stun — melee strike that stuns the target.
///
/// DDGC reference: dmg 8–12 (avg 10.0), atk 82.5%, crit 0%,
/// effect "Weak Stun 3", launch ranks 1–2, target ranks 1–2-3-4.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
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

/// Intimidate — AoE stress attack with no damage.
///
/// DDGC reference: dmg 0–0, atk 82.5%, crit 0%,
/// effect "Stress 0", launch ranks 1–2, target ~1234 (AoE excluding self).
/// Game-gap: position-based targeting and AoE-vs-single not modeled — targets AllEnemies.
pub fn intimidate() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("intimidate"),
        vec![EffectNode::apply_status("stress", Some(0))],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Stress — low-damage attack that applies stress.
///
/// DDGC reference: dmg 2–3 (avg 2.5), atk 72%, crit 0%,
/// effect "Stress 2", launch ranks 1–2, target ranks 1–2-3-4.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn stress() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("stress"),
        vec![
            EffectNode::damage(2.5),
            EffectNode::apply_status("stress", Some(2)),
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

/// All 4 Lizard skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![stun(), intimidate(), stress(), move_skill()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lizard_archetype_is_enemy_eldritch_controller() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Lizard");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 62.0);
        assert_eq!(arch.max_health, 62.0);
        assert_eq!(arch.speed, 6.0);
        assert_eq!(arch.defense, 0.075, "lizard has 7.5% defense");
        assert_eq!(arch.crit_chance, 0.0, "lizard has 0% crit");
        assert_eq!(arch.attack, 10.0, "lizard attack from stun avg");
    }

    #[test]
    fn lizard_stun_deals_damage_and_applies_stun() {
        let skill = stun();
        assert_eq!(skill.id.0, "stun");
        assert!(
            skill.effects.len() >= 2,
            "stun should have damage + stun status effect"
        );
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        let has_stun = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
        });
        assert!(has_damage, "stun skill must have damage effect");
        assert!(has_stun, "stun skill must have stun status effect");
    }

    #[test]
    fn lizard_intimidate_applies_stress_only() {
        let skill = intimidate();
        assert_eq!(skill.id.0, "intimidate");
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        assert!(!has_damage, "intimidate should have no damage");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
        });
        assert!(has_stress, "intimidate must have stress status effect");
    }

    #[test]
    fn lizard_stress_deals_damage_and_applies_stress() {
        let skill = stress();
        assert_eq!(skill.id.0, "stress");
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
        });
        assert!(has_damage, "stress skill must have damage effect");
        assert!(has_stress, "stress skill must have stress status effect");
    }

    #[test]
    fn lizard_skill_pack_has_four_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 4);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"stun"), "missing stun skill");
        assert!(ids.contains(&"intimidate"), "missing intimidate skill");
        assert!(ids.contains(&"stress"), "missing stress skill");
        assert!(ids.contains(&"move"), "missing move skill");
    }

    #[test]
    fn lizard_stun_plus_intimidate_plus_stress_identity() {
        // The core identity of lizard is fast control + stress pressure
        // through a stun → intimidate → stress combo cycle.
        let pack = skill_pack();
        let has_stun = pack.iter().any(|s| {
            s.id.0 == "stun"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                })
        });
        let has_intimidate = pack.iter().any(|s| {
            s.id.0 == "intimidate"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                })
        });
        let has_stress = pack.iter().any(|s| {
            s.id.0 == "stress"
                && s.effects
                    .iter()
                    .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage))
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                })
        });
        assert!(has_stun, "lizard must have stun with status effect");
        assert!(has_intimidate, "lizard must have intimidate with status effect");
        assert!(has_stress, "lizard must have stress with damage + status");
    }
}
