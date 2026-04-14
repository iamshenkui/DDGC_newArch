//! Metal Armor — BaiHu tank family (stun + bleed + disease).
//!
//! DDGC reference: Unholy-type frontline monster from the BaiHu dungeon.
//! Tier 1 base stats: HP 90, DEF 7.5%, PROT 0.8, SPD 4.
//! Skills: stun (melee + stun), bleed (melee + bleed + disease + shock),
//! normal_attack (melee), move (reposition forward).
//!
//! This family is the first BaiHu dungeon monster and the first Tank role
//! family. Its defining identity is high durability plus stun-bleed control.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Metal Armor base archetype — tier 1 stats from DDGC data.
///
/// HP 90, weapon damage derived from normal_attack skill (20–32 avg 26.0),
/// speed 4, dodge 7.5% mapped to defense field as 0.075.
/// Tank role: high-HP frontline with PROT 0.8 (not modeled in Archetype).
/// Crit 12% from normal_attack skill.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Metal Armor"),
        side: CombatSide::Enemy,
        health: 90.0,
        max_health: 90.0,
        attack: 26.0,
        defense: 0.075,
        speed: 4.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.12,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Metal Armor Skills ─────────────────────────────────────────────────────

/// Stun — melee attack that applies a stun effect.
///
/// DDGC reference: dmg 10–16 (avg 13.0), atk 82.5%, crit 0%,
/// effect "Weak Stun 3", launch ranks 1–2, target ranks 1–2.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn stun() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("stun"),
        vec![
            EffectNode::damage(13.0),
            EffectNode::apply_status("stun", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Bleed — melee attack that applies bleed, disease, and shock.
///
/// DDGC reference: dmg 20–32 (avg 26.0), atk 82.5%, crit 0%,
/// effects "New Bleed 1 2" + "Disease Random 6" + "Jellyfish Shock Debuff 1",
/// launch ranks 1–2, target ranks 1–2.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
/// Game-gap: "Disease Random" selects from a disease table at runtime —
/// modeled as a generic "disease" status.
pub fn bleed() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("bleed"),
        vec![
            EffectNode::damage(26.0),
            EffectNode::apply_status("bleed", Some(1)),
            EffectNode::apply_status("disease", Some(6)),
            EffectNode::apply_status("shock", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Normal Attack — basic melee strike from front ranks.
///
/// DDGC reference: dmg 20–32 (avg 26.0), atk 72%, crit 12%,
/// launch ranks 1–2, target ranks 1–2.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn normal_attack() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("normal_attack"),
        vec![EffectNode::damage(26.0)],
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

/// All 4 Metal Armor skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![stun(), bleed(), normal_attack(), move_skill()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metal_armor_archetype_is_enemy_unholy_tank() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Metal Armor");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 90.0);
        assert_eq!(arch.max_health, 90.0);
        assert_eq!(arch.speed, 4.0);
        assert_eq!(arch.defense, 0.075, "metal_armor has 7.5% defense");
        assert_eq!(arch.crit_chance, 0.12, "metal_armor has 12% crit");
        assert_eq!(arch.attack, 26.0, "metal_armor attack from normal_attack avg");
    }

    #[test]
    fn metal_armor_stun_applies_stun_status() {
        let skill = stun();
        assert_eq!(skill.id.0, "stun");
        assert!(
            skill.effects.len() >= 2,
            "stun should have damage + stun status"
        );
        let has_stun = skill
            .effects
            .iter()
            .any(|e| e.status_kind.as_deref() == Some("stun"));
        assert!(has_stun, "stun skill must apply stun status");
    }

    #[test]
    fn metal_armor_bleed_applies_bleed_disease_shock() {
        let skill = bleed();
        assert_eq!(skill.id.0, "bleed");
        assert!(
            skill.effects.len() >= 4,
            "bleed should have damage + bleed + disease + shock"
        );
        let has_bleed = skill
            .effects
            .iter()
            .any(|e| e.status_kind.as_deref() == Some("bleed"));
        let has_disease = skill
            .effects
            .iter()
            .any(|e| e.status_kind.as_deref() == Some("disease"));
        let has_shock = skill
            .effects
            .iter()
            .any(|e| e.status_kind.as_deref() == Some("shock"));
        assert!(has_bleed, "bleed skill must apply bleed status");
        assert!(has_disease, "bleed skill must apply disease status");
        assert!(has_shock, "bleed skill must apply shock status");
    }

    #[test]
    fn metal_armor_normal_attack_deals_damage() {
        let skill = normal_attack();
        assert_eq!(skill.id.0, "normal_attack");
        assert!(
            skill.effects.len() >= 1,
            "normal_attack should have damage effect"
        );
    }

    #[test]
    fn metal_armor_skill_pack_has_four_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 4);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"stun"), "missing stun skill");
        assert!(ids.contains(&"bleed"), "missing bleed skill");
        assert!(ids.contains(&"normal_attack"), "missing normal_attack skill");
        assert!(ids.contains(&"move"), "missing move skill");
    }

    #[test]
    fn metal_armor_stun_plus_bleed_identity() {
        // The core identity of metal_armor is stun (control) plus bleed (DOT + disease).
        // This test preserves that identity.
        let pack = skill_pack();
        let has_stun_control = pack.iter().any(|s| {
            s.id.0 == "stun"
                && s.effects
                    .iter()
                    .any(|e| e.status_kind.as_deref() == Some("stun"))
        });
        let has_bleed_dot = pack.iter().any(|s| {
            s.id.0 == "bleed"
                && s.effects
                    .iter()
                    .any(|e| e.status_kind.as_deref() == Some("bleed"))
        });
        assert!(
            has_stun_control,
            "metal_armor must have stun skill with stun status"
        );
        assert!(
            has_bleed_dot,
            "metal_armor must have bleed skill with bleed status"
        );
    }
}
