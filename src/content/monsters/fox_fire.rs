//! Fox Fire — ZhuQue defensive bruiser family (bleed + burn debuff + ally guard).
//!
//! DDGC reference: Beast-type defensive bruiser from the ZhuQue dungeon.
//! Tier 1 base stats: HP 65, DEF 7.5%, PROT 0.4, SPD 6, crit 0%.
//! Skills: bite (bleed), vomit (burn resist down + disease), protect (ally guard), move.
//!
//! This family's defining identity is a bruiser that combines bleed and
//! burn-debuff pressure with an ally-guard mechanic.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Fox Fire base archetype — tier 1 stats from DDGC data.
///
/// HP 65, weapon damage derived from bite/vomit skill (8–12 avg 10),
/// speed 6, defense 0.075 (7.5% dodge).
/// Bruiser role: bleed + burn debuff + ally guard with PROT 0.4 (not modeled in Archetype).
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Fox Fire"),
        side: CombatSide::Enemy,
        health: 65.0,
        max_health: 65.0,
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

// ── Fox Fire Skills ────────────────────────────────────────────────────────

/// Bite — melee attack that deals damage and applies bleed.
///
/// DDGC reference: dmg 8–12 (avg 10), atk 82.5%, crit 0%,
/// effect "Strong Bleed 1", launch 12, target 12.
/// Game-gap: position-based targeting (.launch 12, .target 12) is
/// approximated as AllEnemies.
pub fn bite() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("bite"),
        vec![
            EffectNode::damage(10.0),
            EffectNode::apply_status("bleed", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Vomit — melee attack that deals damage, reduces burn resist, and applies disease.
///
/// DDGC reference: dmg 8–12 (avg 10), atk 82.5%, crit 0%,
/// effects "Burn Resist -5" and "Disease Random 4",
/// launch 12, target 1234.
/// Game-gap: position-based targeting (.launch 12, .target 1234) is
/// approximated as AllEnemies.
/// Game-gap: "Disease Random 4" random disease table selection is not modeled.
pub fn vomit() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("vomit"),
        vec![
            EffectNode::damage(10.0),
            EffectNode::apply_status("burn_resist_down", Some(5)),
            EffectNode::apply_status("disease", Some(4)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Protect — zero-damage ally buff that applies guard and defend.
///
/// DDGC reference: dmg 0–0, atk 100%, crit 0%,
/// effects "Fox Fire Guard 1" and "Fox Fire Defend 1",
/// launch 12, target @1234, self_target_valid False, is_crit_valid False.
/// Game-gap: @1234 (any ally rank, cannot target self) is approximated as
/// AllAllies (which includes self).
pub fn protect() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("protect"),
        vec![
            EffectNode::apply_status("guard", Some(1)),
            EffectNode::apply_status("defend", Some(1)),
        ],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Move — reposition self forward 1 rank.
///
/// DDGC reference: dmg 0–0, atk 0%, crit 0%,
/// launch 34, target @23, .move 0 1.
/// Game-gap: movement direction is approximated as push(1).
pub fn move_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("move"),
        vec![EffectNode::push(1)],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// All 4 Fox Fire skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![bite(), vomit(), protect(), move_skill()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fox_fire_archetype_is_enemy_beast_bruiser() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Fox Fire");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 65.0);
        assert_eq!(arch.max_health, 65.0);
        assert_eq!(arch.speed, 6.0);
        assert_eq!(arch.defense, 0.075, "fox_fire has 7.5% defense");
        assert_eq!(arch.crit_chance, 0.0, "fox_fire has 0% crit");
        assert_eq!(arch.attack, 10.0, "fox_fire attack from bite/vomit avg");
    }

    #[test]
    fn fox_fire_bite_deals_damage_and_applies_bleed() {
        let skill = bite();
        assert_eq!(skill.id.0, "bite");
        assert!(
            skill.effects.len() >= 2,
            "bite should have damage + bleed effects"
        );
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        assert!(has_damage, "bite must have damage effect");
        let has_bleed = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("bleed")
        });
        assert!(has_bleed, "bite must apply bleed status");
    }

    #[test]
    fn fox_fire_vomit_deals_damage_and_applies_debuffs() {
        let skill = vomit();
        assert_eq!(skill.id.0, "vomit");
        assert!(
            skill.effects.len() >= 3,
            "vomit should have damage + burn resist down + disease effects"
        );
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        assert!(has_damage, "vomit must have damage effect");
        let has_burn_resist_down = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("burn_resist_down")
        });
        assert!(has_burn_resist_down, "vomit must apply burn resist down");
        let has_disease = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("disease")
        });
        assert!(has_disease, "vomit must apply disease status");
    }

    #[test]
    fn fox_fire_protect_applies_guard_and_defend() {
        let skill = protect();
        assert_eq!(skill.id.0, "protect");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllAllies",
            "protect targets allies"
        );
        let has_guard = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("guard")
        });
        assert!(has_guard, "protect must apply guard status");
        let has_defend = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("defend")
        });
        assert!(has_defend, "protect must apply defend status");
        let has_damage = skill
            .effects
            .iter()
            .any(|e| matches!(e.kind, framework_combat::effects::EffectKind::Damage));
        assert!(!has_damage, "protect should not deal damage");
    }

    #[test]
    fn fox_fire_skill_pack_has_four_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 4);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"bite"), "missing bite skill");
        assert!(ids.contains(&"vomit"), "missing vomit skill");
        assert!(ids.contains(&"protect"), "missing protect skill");
        assert!(ids.contains(&"move"), "missing move skill");
    }

    #[test]
    fn fox_fire_bleed_plus_burn_debuff_plus_guard_identity() {
        // The core identity of fox_fire is bleed + burn debuff + guard.
        // This test preserves that identity.
        let pack = skill_pack();
        let has_bleed = pack.iter().any(|s| {
            s.id.0 == "bite"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("bleed")
                })
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Damage)
                })
        });
        let has_burn_debuff = pack.iter().any(|s| {
            s.id.0 == "vomit"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("burn_resist_down")
                })
        });
        let has_guard = pack.iter().any(|s| {
            s.id.0 == "protect"
                && format!("{:?}", s.target_selector) == "AllAllies"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("guard")
                })
        });
        assert!(
            has_bleed,
            "fox_fire must have bite with damage and bleed status"
        );
        assert!(
            has_burn_debuff,
            "fox_fire must have vomit with burn resist down"
        );
        assert!(
            has_guard,
            "fox_fire must have protect with ally guard status"
        );
    }
}