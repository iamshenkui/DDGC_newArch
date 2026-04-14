//! SC Bow — XuanWu boss minion (bowed string, magic damage).
//!
//! DDGC reference: Eldritch-type minion from the XuanWu dungeon.
//! Tier 1 base stats: HP 20, DEF 10%, PROT 0.5, SPD 5, 1 turn/round.
//! Skills: crematorium_bowstring, move.
//!
//! SC Bow is a bowed-string minion summoned by the Scorchthroat Chanteuse.
//! It deals heavy ranged magic damage with crematorium_bowstring.
//!
//! Game-gaps:
//! - Position-based targeting (launch 3,4, target $1234) approximated as AllEnemies
//! - Movement skill (move 1→0) approximated as push(1)
//! - PROT (0.5), MAGIC_PROT (0.5) not modeled in Archetype
//! - Stun Resist 50%, Poison Resist 25%, Bleed Resist 50%, Debuff Resist 40%,
//!   Move Resist 50%, Burn Resist 75%, Frozen Resist 25% not modeled
//! - magic_dmg not modeled (treated as normal damage)

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// SC Bow base archetype — tier 1 boss minion stats from DDGC data.
///
/// HP 20, weapon damage derived from crematorium_bowstring (8–12 avg 10),
/// speed 5, defense 0.10 (10% dodge).
/// Skirmisher role: heavy ranged magic damage from back positions.
/// Crit 5% from crematorium_bowstring.
/// PROT 0.5, MAGIC_PROT 0.5 — not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("SC Bow"),
        side: CombatSide::Enemy,
        health: 20.0,
        max_health: 20.0,
        attack: 10.0,
        defense: 0.10,
        speed: 5.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.05,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── SC Bow Skills ────────────────────────────────────────────────────────────

/// Crematorium Bowstring — ranged heavy magic damage.
///
/// DDGC reference: dmg 8–12 (magic), atk 85%, crit 5%,
/// launch ranks 3,4, target $1234 (conditional multi-target),
/// no additional effects.
/// Game-gap: $1234 conditional targeting approximated as AllEnemies.
/// Game-gap: magic damage type not modeled — treated as normal damage.
pub fn crematorium_bowstring() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("crematorium_bowstring"),
        vec![EffectNode::damage(10.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Move — self-reposition skill.
///
/// DDGC reference: dmg 0–0, atk 0%, crit 0%,
/// launch ranks 1,2, target @23 (ally ranks 2-3), .move 1 0.
/// Game-gap: position-based movement approximated as push(1).
pub fn move_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("move"),
        vec![EffectNode::push(1)],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// All 2 SC Bow skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![crematorium_bowstring(), move_skill()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sc_bow_archetype_is_enemy_eldritch_skirmisher() {
        let arch = archetype();
        assert_eq!(arch.name.0, "SC Bow");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 20.0);
        assert_eq!(arch.max_health, 20.0);
        assert_eq!(arch.speed, 5.0, "sc_bow has SPD 5");
        assert_eq!(arch.defense, 0.10, "sc_bow has 10% defense");
        assert_eq!(arch.attack, 10.0, "attack from crematorium_bowstring avg 8-12");
        assert_eq!(arch.crit_chance, 0.05, "crit 5% from crematorium_bowstring");
    }

    #[test]
    fn sc_bow_crematorium_bowstring_deals_damage() {
        let skill = crematorium_bowstring();
        assert_eq!(skill.id.0, "crematorium_bowstring");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "crematorium_bowstring must deal damage");
    }

    #[test]
    fn sc_bow_skill_pack_has_two_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 2);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"crematorium_bowstring"), "missing crematorium_bowstring");
        assert!(ids.contains(&"move"), "missing move");
    }

    #[test]
    fn sc_bow_magic_damage_identity() {
        let pack = skill_pack();
        let has_damage = pack.iter().any(|s| {
            s.id.0 == "crematorium_bowstring"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Damage)
                })
        });
        assert!(has_damage, "sc_bow must have magic damage skill");
    }
}
