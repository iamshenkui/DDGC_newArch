//! Mantis Spiny Flower — QingLong controller family (armor-piercing blight + AoE bleed).
//!
//! DDGC reference: Beast-type controller from the QingLong dungeon.
//! Tier 1 base stats: HP 88, DEF 7.5%, SPD 7, crit 12% on normal_attack.
//! Skills: ignore_armor (blight), crowd_bleed (AoE bleed), normal_attack, move.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Mantis Spiny Flower base archetype — tier 1 stats from DDGC data.
///
/// HP 88, weapon damage 30–42 (avg 36), speed 7, dodge 0%, crit 12%.
/// Defense 7.5% mapped to `defense` field as 0.075.
/// Controller role: applies armor-piercing blight and AoE bleed rather than raw damage.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Mantis Spiny Flower"),
        side: CombatSide::Enemy,
        health: 88.0,
        max_health: 88.0,
        attack: 36.0,
        defense: 0.075,
        speed: 7.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.12,
        dodge: 0.0,
    }
}

// ── Mantis Spiny Flower Skills ────────────────────────────────────────────

/// Ignore Armor — melee blight attack targeting all enemy ranks.
///
/// DDGC reference: dmg 10–14 (avg 12), applies "New Blight 1",
/// atk 82.5%, launch ranks 1–2, target ranks 1–4, move 0 1.
/// Game-gap: position-based targeting and self-movement not modeled —
/// targets AllEnemies; armor-piercing semantic is captured by skill name only.
pub fn ignore_armor() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("ignore_armor"),
        vec![
            EffectNode::damage(12.0),
            EffectNode::apply_status("blight", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Crowd Bleed — melee AoE bleed targeting front 2 enemies.
///
/// DDGC reference: dmg 10–14 (avg 12), applies "New Bleed 1 2" (bleed for 2 rounds),
/// atk 82.5%, launch ranks 1–2, target ~12 (front 2).
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn crowd_bleed() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("crowd_bleed"),
        vec![
            EffectNode::damage(12.0),
            EffectNode::apply_status("bleed", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Normal Attack — basic melee strike targeting front ranks.
///
/// DDGC reference: dmg 30–42 (avg 36), atk 72%, crit 12%,
/// launch ranks 1–2, target ranks 1–2.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn normal_attack() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("normal_attack"),
        vec![EffectNode::damage(36.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Move — self-repositioning skill (move forward 1 rank).
///
/// DDGC reference: 0 dmg, atk 0%, launch ranks 3–4, moves self forward 1.
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

/// All 4 Mantis Spiny Flower skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![ignore_armor(), crowd_bleed(), normal_attack(), move_skill()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mantis_spiny_flower_archetype_is_enemy_beast() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Mantis Spiny Flower");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 88.0);
        assert_eq!(arch.max_health, 88.0);
        assert_eq!(arch.speed, 7.0);
        assert!(arch.defense > 0.0, "should have nonzero defense");
    }

    #[test]
    fn mantis_spiny_flower_ignore_armor_applies_blight() {
        let skill = ignore_armor();
        assert_eq!(skill.id.0, "ignore_armor");
        // Must have damage + blight status
        assert!(skill.effects.len() >= 2, "ignore_armor should have damage + blight");
    }

    #[test]
    fn mantis_spiny_flower_crowd_bleed_applies_bleed() {
        let skill = crowd_bleed();
        assert_eq!(skill.id.0, "crowd_bleed");
        // Must have damage + bleed status
        assert!(skill.effects.len() >= 2, "crowd_bleed should have damage + bleed");
    }

    #[test]
    fn mantis_spiny_flower_skill_pack_has_four_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 4);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"ignore_armor"), "missing ignore_armor skill");
        assert!(ids.contains(&"crowd_bleed"), "missing crowd_bleed skill");
        assert!(ids.contains(&"normal_attack"), "missing normal_attack skill");
        assert!(ids.contains(&"move"), "missing move skill");
    }

    #[test]
    fn mantis_spiny_flower_ignore_armor_plus_crowd_bleed_identity() {
        // The core identity of mantis_spiny_flower is ignore_armor (armor-piercing blight)
        // combined with crowd_bleed (AoE bleed). This test preserves that.
        let pack = skill_pack();
        let has_blight = pack.iter().any(|s| {
            s.effects.iter().any(|e| {
                e.status_kind.as_deref() == Some("blight")
            })
        });
        let has_bleed = pack.iter().any(|s| {
            s.effects.iter().any(|e| {
                e.status_kind.as_deref() == Some("bleed")
            })
        });
        assert!(has_blight, "mantis_spiny_flower must apply blight (ignore_armor)");
        assert!(has_bleed, "mantis_spiny_flower must apply bleed (crowd_bleed)");
    }
}
