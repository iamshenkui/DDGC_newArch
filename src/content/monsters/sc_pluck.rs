//! SC Pluck — XuanWu boss minion (plucked string, magic damage + burn).
//!
//! DDGC reference: Eldritch-type minion from the XuanWu dungeon.
//! Tier 1 base stats: HP 20, DEF 10%, PROT 0.5, SPD 5, 1 turn/round.
//! Skills: ossein_arsonist_lyre, move.
//!
//! SC Pluck is a plucked-string minion summoned by the Scorchthroat Chanteuse.
//! It deals ranged magic damage with burn from back positions.
//!
//! Game-gaps:
//! - Position-based targeting (launch 3,4, target 1234) approximated as AllEnemies
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

/// SC Pluck base archetype — tier 1 boss minion stats from DDGC data.
///
/// HP 20, weapon damage derived from ossein_arsonist_lyre (4–6 avg 5),
/// speed 5, defense 0.10 (10% dodge).
/// Controller role: ranged magic damage + burn from back positions.
/// Crit 7% from ossein_arsonist_lyre.
/// PROT 0.5, MAGIC_PROT 0.5 — not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("SC Pluck"),
        side: CombatSide::Enemy,
        health: 20.0,
        max_health: 20.0,
        attack: 5.0,
        defense: 0.10,
        speed: 5.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.07,
        dodge: 0.0,
    }
}

// ── SC Pluck Skills ──────────────────────────────────────────────────────────

/// Ossein Arsonist Lyre — ranged magic damage + burn.
///
/// DDGC reference: dmg 4–6 (magic), atk 85%, crit 7%,
/// launch ranks 3,4, target 1234 (any enemy),
/// effects "Burn 1".
/// Game-gap: magic damage type not modeled — treated as normal damage.
pub fn ossein_arsonist_lyre() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("ossein_arsonist_lyre"),
        vec![
            EffectNode::damage(5.0),
            EffectNode::apply_status("burn", Some(1)),
        ],
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

/// All 2 SC Pluck skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![ossein_arsonist_lyre(), move_skill()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sc_pluck_archetype_is_enemy_eldritch_controller() {
        let arch = archetype();
        assert_eq!(arch.name.0, "SC Pluck");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 20.0);
        assert_eq!(arch.max_health, 20.0);
        assert_eq!(arch.speed, 5.0, "sc_pluck has SPD 5");
        assert_eq!(arch.defense, 0.10, "sc_pluck has 10% defense");
        assert_eq!(arch.attack, 5.0, "attack from ossein_arsonist_lyre avg 4-6");
        assert_eq!(arch.crit_chance, 0.07, "crit 7% from ossein_arsonist_lyre");
    }

    #[test]
    fn sc_pluck_ossein_arsonist_lyre_applies_burn() {
        let skill = ossein_arsonist_lyre();
        assert_eq!(skill.id.0, "ossein_arsonist_lyre");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "ossein_arsonist_lyre must deal damage");
        let has_burn = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("burn")
        });
        assert!(has_burn, "ossein_arsonist_lyre must apply burn status");
    }

    #[test]
    fn sc_pluck_skill_pack_has_two_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 2);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"ossein_arsonist_lyre"), "missing ossein_arsonist_lyre");
        assert!(ids.contains(&"move"), "missing move");
    }

    #[test]
    fn sc_pluck_burn_identity() {
        let pack = skill_pack();
        let has_burn = pack.iter().any(|s| {
            s.id.0 == "ossein_arsonist_lyre"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("burn")
                })
        });
        assert!(has_burn, "sc_pluck must have burn skill");
    }
}
