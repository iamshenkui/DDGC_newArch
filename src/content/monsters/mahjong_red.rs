//! Mahjong Red — ZhuQue boss minion (mark + bonus damage to marked).
//!
//! DDGC reference: Eldritch-type minion from the ZhuQue dungeon.
//! Stats: HP 20, DEF 10%, PROT 0, SPD 5, 1 turn/round.
//! Skills: lucky_charity, fortune_ante.
//!
//! The Mahjong Red is a summoned minion that marks targets with
//! `lucky_charity` (magic damage + mark) and deals bonus damage to marked
//! targets with `fortune_ante`. Its AI brain equally weights both skills
//! (0.5 each). It is immune to bleed and poison.
//!
//! Game-gaps:
//! - Mark Target effect (lucky_charity) modeled as "mark" status marker only
//! - SmallDmg Marked Target effect (fortune_ante) modeled as "mark_damage"
//!   status marker only — bonus damage against marked targets not modeled
//! - Magic damage type (lucky_charity) not modeled — treated as normal damage
//! - PROT 0, MAGIC_PROT 0.5 not modeled in Archetype
//! - Stun Resist 50%, Poison Resist 100% (immune), Bleed Resist 100% (immune),
//!   Debuff Resist 40%, Move Resist 70%, Burn Resist 35%, Frozen Resist 35%
//!   not modeled

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Mahjong Red base archetype — minion stats from DDGC data.
///
/// HP 20, weapon damage derived from lucky_charity skill (magic dmg 4–6 avg 5),
/// speed 5, defense 0.10 (10% dodge).
/// Skirmisher role: marks targets and exploits marks for bonus damage.
/// Crit 5% from both skills.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Mahjong Red"),
        side: CombatSide::Enemy,
        health: 20.0,
        max_health: 20.0,
        attack: 5.0,
        defense: 0.10,
        speed: 5.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.05,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Mahjong Red Skills ───────────────────────────────────────────────────────

/// Lucky Charity — mark target + magic damage.
///
/// DDGC reference: magic dmg 4–6, atk 85%, crit 5%,
/// launch ranks 1,2,3,4, target 1234 (any rank),
/// effect "Mark Target".
/// Game-gap: magic damage type not modeled — treated as normal damage.
/// Game-gap: Mark Target modeled as "mark" status marker only.
pub fn lucky_charity() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("lucky_charity"),
        vec![
            EffectNode::damage(5.0),
            EffectNode::apply_status("mark", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Fortune Ante — damage + bonus damage against marked target.
///
/// DDGC reference: dmg 4–6, atk 85%, crit 5%,
/// launch ranks 1,2,3,4, target 1234 (any rank),
/// effect "SmallDmg Marked Target".
/// Game-gap: SmallDmg Marked Target (bonus damage against marked targets)
/// modeled as "mark_damage" status marker only.
pub fn fortune_ante() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("fortune_ante"),
        vec![
            EffectNode::damage(5.0),
            EffectNode::apply_status("mark_damage", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 2 Mahjong Red skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![lucky_charity(), fortune_ante()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mahjong_red_archetype_is_enemy_eldritch_skirmisher() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Mahjong Red");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 20.0);
        assert_eq!(arch.max_health, 20.0);
        assert_eq!(arch.speed, 5.0, "mahjong_red has SPD 5");
        assert_eq!(arch.defense, 0.10, "mahjong_red has 10% defense");
        assert_eq!(arch.attack, 5.0, "attack from lucky_charity avg 4-6");
        assert_eq!(arch.crit_chance, 0.05, "crit 5% from all skills");
    }

    #[test]
    fn mahjong_red_lucky_charity_applies_mark() {
        let skill = lucky_charity();
        assert_eq!(skill.id.0, "lucky_charity");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "lucky_charity must deal damage");
        let has_mark = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("mark")
        });
        assert!(has_mark, "lucky_charity must apply mark status");
    }

    #[test]
    fn mahjong_red_fortune_ante_applies_mark_damage() {
        let skill = fortune_ante();
        assert_eq!(skill.id.0, "fortune_ante");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "fortune_ante must deal damage");
        let has_mark_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("mark_damage")
        });
        assert!(has_mark_damage, "fortune_ante must apply mark_damage status");
    }

    #[test]
    fn mahjong_red_skill_pack_has_two_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 2);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"lucky_charity"), "missing lucky_charity");
        assert!(ids.contains(&"fortune_ante"), "missing fortune_ante");
    }

    #[test]
    fn mahjong_red_mark_plus_mark_damage_identity() {
        // The core identity of mahjong_red is a mark-exploiting skirmisher
        // that marks targets and deals bonus damage to marked targets.
        let pack = skill_pack();

        let has_mark = pack.iter().any(|s| {
            s.id.0 == "lucky_charity"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("mark")
                })
        });

        let has_mark_damage = pack.iter().any(|s| {
            s.id.0 == "fortune_ante"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("mark_damage")
                })
        });

        assert!(has_mark, "mahjong_red must have mark skill");
        assert!(has_mark_damage, "mahjong_red must have mark_damage skill");
    }
}
