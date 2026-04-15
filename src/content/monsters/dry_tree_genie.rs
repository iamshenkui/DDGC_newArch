//! Dry Tree Genie — QingLong ranged family (bleed + slow-crowd + stress).
//!
//! DDGC reference: Eldritch-type ranged monster from the QingLong dungeon.
//! Tier 1 base stats: HP 90, DEF 0%, PROT 20%, SPD 3.
//! Skills: bleed (ranged bleed), slow_crowd (AoE slow + stress), stress (stress), move.
//!
//! This family is distinct from the mantis families: it is Eldritch-type
//! (not Beast), uses a Ranged role (not Controller), and its identity
//! centers on ranged bleed + crowd slow + stress pressure rather than
//! poison/blight/weak + AoE bleed.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Dry Tree Genie base archetype — tier 1 stats from DDGC data.
///
/// HP 90, weapon damage derived from bleed skill (26–39 avg 32.5),
/// speed 3, dodge 0%, crit 0%.
/// Defense 0% mapped to `defense` field as 0.0.
/// Ranged role: applies bleed, slow, and stress from back ranks.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Dry Tree Genie"),
        side: CombatSide::Enemy,
        health: 90.0,
        max_health: 90.0,
        attack: 32.5,
        defense: 0.0,
        speed: 3.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.0,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Dry Tree Genie Skills ──────────────────────────────────────────────────

/// Bleed — ranged bleed attack targeting all enemy ranks.
///
/// DDGC reference: dmg 26–39 (avg 32.5), applies "New Bleed 1",
/// atk 82.5%, launch ranks 3–4, target ranks 1–4.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn bleed() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("bleed"),
        vec![
            EffectNode::damage(32.5),
            EffectNode::apply_status("bleed", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Slow Crowd — ranged AoE slow + stress targeting all enemy ranks.
///
/// DDGC reference: dmg 9–13 (avg 11), applies "Speed -1" and "Stress Range 2-5",
/// atk 82.5%, launch ranks 3–4, target ~1234 (AoE all 4 positions).
/// Game-gap: position-based targeting not modeled — targets AllEnemies;
/// stress range 2–5 approximated as stress(3).
pub fn slow_crowd() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("slow_crowd"),
        vec![
            EffectNode::damage(11.0),
            EffectNode::apply_status("slow", Some(1)),
            EffectNode::apply_status("stress", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Stress — ranged stress attack targeting all enemy ranks.
///
/// DDGC reference: dmg 3–4 (avg 3.5), applies "Stress 2",
/// atk 82.5%, launch ranks 3–4, target ranks 1–4.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn stress() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("stress"),
        vec![
            EffectNode::damage(3.5),
            EffectNode::apply_status("stress", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Move — self-repositioning skill (move forward 1 rank).
///
/// DDGC reference: 0 dmg, atk 0%, launch ranks 1–2, target @23 (self formation),
/// .move 1 0 (forward 1).
/// Game-gap: movement not fully modeled — uses push(1) on self as approximation.
pub fn move_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("move"),
        vec![EffectNode::push(1)],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// All 4 Dry Tree Genie skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![bleed(), slow_crowd(), stress(), move_skill()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dry_tree_genie_archetype_is_enemy_eldritch() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Dry Tree Genie");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 90.0);
        assert_eq!(arch.max_health, 90.0);
        assert_eq!(arch.speed, 3.0);
        assert_eq!(arch.defense, 0.0, "dry_tree_genie has 0% defense");
    }

    #[test]
    fn dry_tree_genie_bleed_applies_bleed() {
        let skill = bleed();
        assert_eq!(skill.id.0, "bleed");
        // Must have damage + bleed status
        assert!(skill.effects.len() >= 2, "bleed should have damage + bleed status");
    }

    #[test]
    fn dry_tree_genie_slow_crowd_applies_slow_and_stress() {
        let skill = slow_crowd();
        assert_eq!(skill.id.0, "slow_crowd");
        // Must have damage + slow + stress
        assert!(skill.effects.len() >= 3, "slow_crowd should have damage + slow + stress");
    }

    #[test]
    fn dry_tree_genie_stress_applies_stress() {
        let skill = stress();
        assert_eq!(skill.id.0, "stress");
        // Must have damage + stress
        assert!(skill.effects.len() >= 2, "stress should have damage + stress status");
    }

    #[test]
    fn dry_tree_genie_skill_pack_has_four_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 4);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"bleed"), "missing bleed skill");
        assert!(ids.contains(&"slow_crowd"), "missing slow_crowd skill");
        assert!(ids.contains(&"stress"), "missing stress skill");
        assert!(ids.contains(&"move"), "missing move skill");
    }

    #[test]
    fn dry_tree_genie_bleed_plus_slow_crowd_plus_stress_identity() {
        // The core identity of dry_tree_genie is bleed + slow-crowd + stress
        // pressure from back ranks. This test preserves that identity.
        let pack = skill_pack();
        let has_bleed = pack.iter().any(|s| {
            s.effects.iter().any(|e| {
                e.status_kind.as_deref() == Some("bleed")
            })
        });
        let has_slow = pack.iter().any(|s| {
            s.effects.iter().any(|e| {
                e.status_kind.as_deref() == Some("slow")
            })
        });
        let has_stress = pack.iter().any(|s| {
            s.effects.iter().any(|e| {
                e.status_kind.as_deref() == Some("stress")
            })
        });
        assert!(has_bleed, "dry_tree_genie must apply bleed");
        assert!(has_slow, "dry_tree_genie must apply slow (slow_crowd)");
        assert!(has_stress, "dry_tree_genie must apply stress");
    }
}
