//! Alligator Yangtze — BaiHu bruiser family (melee + bleed + riposte).
//!
//! DDGC reference: Beast-type frontline monster from the BaiHu dungeon.
//! Tier 1 base stats: HP 94, DEF 7.5%, PROT 0.4, SPD 6.
//! Skills: normal_attack (melee), bleed (melee + bleed), mark_riposte (self-buff),
//! riposte1 (reactive counter-attack).
//!
//! This family's defining identity is a high-HP bruiser that applies bleed
//! and punishes attackers with a riposte counter-attack buff.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Alligator Yangtze base archetype — tier 1 stats from DDGC data.
///
/// HP 94, weapon damage derived from normal_attack skill (27–36 avg 31.5),
/// speed 6, defense 0.075 (7.5% dodge).
/// Bruiser role: high-HP frontline with PROT 0.4 (not modeled in Archetype).
/// Crit 12% from normal_attack skill.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Alligator Yangtze"),
        side: CombatSide::Enemy,
        health: 94.0,
        max_health: 94.0,
        attack: 31.5,
        defense: 0.075,
        speed: 6.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.12,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Alligator Yangtze Skills ─────────────────────────────────────────────

/// Normal Attack — melee strike from back ranks.
///
/// DDGC reference: dmg 27–36 (avg 31.5), atk 82.5%, crit 12%,
/// launch ranks 3–4, target ranks 1–2, .move 0 1 (self-reposition).
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
/// Game-gap: self-movement (.move 0 1) embedded in offensive skill dropped.
pub fn normal_attack() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("normal_attack"),
        vec![EffectNode::damage(31.5)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Bleed — melee attack that applies a bleed DoT.
///
/// DDGC reference: dmg 14–18 (avg 16), atk 82.5%, crit 0%,
/// effect "New Bleed 1" (dotBleed 12/round, duration 3),
/// launch ranks 1–2, target ranks 1–2.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn bleed() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("bleed"),
        vec![
            EffectNode::damage(16.0),
            EffectNode::apply_status("bleed", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Mark Riposte — self-buff that enables counter-attacks when hit.
///
/// DDGC reference: dmg 0–0, atk 100%, effect "AY Riposte 1",
/// self-targeted, duration 3 rounds, riposte_chance_add 100%.
/// Modeled as a self-applied "riposte" status with duration 3.
/// Game-gap: reactive counter-attack mechanic not natively modeled in
/// the framework — the riposte status exists as a marker, but the
/// actual counter-attack trigger is game-specific logic.
pub fn mark_riposte() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("mark_riposte"),
        vec![EffectNode::apply_status("riposte", Some(3))],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// Riposte 1 — reactive counter-attack triggered by the riposte buff.
///
/// DDGC reference: dmg 12–19 (avg 15.5), atk 82.5%, crit 12%,
/// launch ranks 1–4, target ranks 1–4 (any position).
/// This skill is not player-selectable — it fires automatically when
/// the alligator is attacked while the riposte buff is active.
/// Included in the skill pack for registration, but the reactive
/// trigger mechanism is a game-gap.
pub fn riposte1() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("riposte1"),
        vec![EffectNode::damage(15.5)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 4 Alligator Yangtze skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![normal_attack(), bleed(), mark_riposte(), riposte1()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alligator_yangtze_archetype_is_enemy_beast_bruiser() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Alligator Yangtze");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 94.0);
        assert_eq!(arch.max_health, 94.0);
        assert_eq!(arch.speed, 6.0);
        assert_eq!(arch.defense, 0.075, "alligator_yangtze has 7.5% defense");
        assert_eq!(arch.crit_chance, 0.12, "alligator_yangtze has 12% crit");
        assert_eq!(arch.attack, 31.5, "alligator_yangtze attack from normal_attack avg");
    }

    #[test]
    fn alligator_yangtze_normal_attack_deals_damage() {
        let skill = normal_attack();
        assert_eq!(skill.id.0, "normal_attack");
        assert!(
            !skill.effects.is_empty(),
            "normal_attack should have damage effect"
        );
    }

    #[test]
    fn alligator_yangtze_bleed_applies_damage_and_bleed() {
        let skill = bleed();
        assert_eq!(skill.id.0, "bleed");
        assert!(
            skill.effects.len() >= 2,
            "bleed should have damage + apply_status effect"
        );
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        let has_bleed = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus));
        assert!(has_damage, "bleed skill must have damage effect");
        assert!(has_bleed, "bleed skill must have bleed status effect");
    }

    #[test]
    fn alligator_yangtze_mark_riposte_applies_riposte_status() {
        let skill = mark_riposte();
        assert_eq!(skill.id.0, "mark_riposte");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "mark_riposte targets self"
        );
        let has_riposte = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("riposte")
        });
        assert!(
            has_riposte,
            "mark_riposte must apply riposte status"
        );
    }

    #[test]
    fn alligator_yangtze_skill_pack_has_four_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 4);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"normal_attack"), "missing normal_attack skill");
        assert!(ids.contains(&"bleed"), "missing bleed skill");
        assert!(ids.contains(&"mark_riposte"), "missing mark_riposte skill");
        assert!(ids.contains(&"riposte1"), "missing riposte1 skill");
    }

    #[test]
    fn alligator_yangtze_bruiser_plus_riposte_identity() {
        // The core identity of alligator_yangtze is bruiser plus riposte.
        // This test preserves that identity.
        let pack = skill_pack();
        let has_bruiser_hit = pack.iter().any(|s| {
            s.id.0 == "normal_attack"
                && s.effects
                    .iter()
                    .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage))
        });
        let has_bleed = pack.iter().any(|s| {
            s.id.0 == "bleed"
                && s.effects
                    .iter()
                    .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus))
        });
        let has_riposte = pack.iter().any(|s| {
            s.id.0 == "mark_riposte"
                && format!("{:?}", s.target_selector) == "SelfOnly"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("riposte")
                })
        });
        assert!(
            has_bruiser_hit,
            "alligator_yangtze must have normal_attack with damage"
        );
        assert!(
            has_bleed,
            "alligator_yangtze must have bleed skill with status"
        );
        assert!(
            has_riposte,
            "alligator_yangtze must have mark_riposte self-buff"
        );
    }
}
