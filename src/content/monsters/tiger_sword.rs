//! Tiger Sword — BaiHu bruiser family (heavy melee + pull).
//!
//! DDGC reference: Unholy-type frontline monster from the BaiHu dungeon.
//! Tier 1 base stats: HP 90, DEF 0%, PROT 0.6, SPD 3.
//! Skills: normal_attack (melee), pull (ranged + pull 2), move (reposition forward).
//!
//! This family's defining identity is heavy melee damage combined with a pull
//! mechanic that drags back-rank enemies into the front line.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Tiger Sword base archetype — tier 1 stats from DDGC data.
///
/// HP 90, weapon damage derived from normal_attack skill (20–32 avg 26.0),
/// speed 3, defense 0.0 (0% dodge).
/// Bruiser role: high-HP frontline with PROT 0.6 (not modeled in Archetype).
/// Crit 3% from normal_attack skill.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Tiger Sword"),
        side: CombatSide::Enemy,
        health: 90.0,
        max_health: 90.0,
        attack: 26.0,
        defense: 0.0,
        speed: 3.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.03,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Tiger Sword Skills ─────────────────────────────────────────────────────

/// Normal Attack — melee strike from front ranks.
///
/// DDGC reference: dmg 20–32 (avg 26.0), atk 82.5%, crit 3%,
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

/// Pull — ranged attack that pulls the target forward 2 positions.
///
/// DDGC reference: dmg 20–32 (avg 26.0), atk 76.5%, crit 6%,
/// effect "Pull 2A", launch ranks 3–4, target ranks 3–4.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn pull() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("pull"),
        vec![
            EffectNode::damage(26.0),
            EffectNode::pull(2),
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

/// All 3 Tiger Sword skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![normal_attack(), pull(), move_skill()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiger_sword_archetype_is_enemy_unholy_bruiser() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Tiger Sword");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 90.0);
        assert_eq!(arch.max_health, 90.0);
        assert_eq!(arch.speed, 3.0);
        assert_eq!(arch.defense, 0.0, "tiger_sword has 0% defense");
        assert_eq!(arch.crit_chance, 0.03, "tiger_sword has 3% crit");
        assert_eq!(arch.attack, 26.0, "tiger_sword attack from normal_attack avg");
    }

    #[test]
    fn tiger_sword_normal_attack_deals_damage() {
        let skill = normal_attack();
        assert_eq!(skill.id.0, "normal_attack");
        assert!(
            !skill.effects.is_empty(),
            "normal_attack should have damage effect"
        );
    }

    #[test]
    fn tiger_sword_pull_applies_damage_and_pull() {
        let skill = pull();
        assert_eq!(skill.id.0, "pull");
        assert!(
            skill.effects.len() >= 2,
            "pull should have damage + pull effect"
        );
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        let has_pull = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Pull));
        assert!(has_damage, "pull skill must have damage effect");
        assert!(has_pull, "pull skill must have pull effect");
    }

    #[test]
    fn tiger_sword_skill_pack_has_three_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 3);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"normal_attack"), "missing normal_attack skill");
        assert!(ids.contains(&"pull"), "missing pull skill");
        assert!(ids.contains(&"move"), "missing move skill");
    }

    #[test]
    fn tiger_sword_heavy_hit_plus_pull_identity() {
        // The core identity of tiger_sword is heavy melee hit plus pull mechanic.
        // This test preserves that identity.
        let pack = skill_pack();
        let has_heavy_hit = pack.iter().any(|s| {
            s.id.0 == "normal_attack"
                && s.effects
                    .iter()
                    .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage))
        });
        let has_pull = pack.iter().any(|s| {
            s.id.0 == "pull"
                && s.effects
                    .iter()
                    .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Pull))
        });
        assert!(
            has_heavy_hit,
            "tiger_sword must have normal_attack with damage"
        );
        assert!(
            has_pull,
            "tiger_sword must have pull skill with pull effect"
        );
    }
}
