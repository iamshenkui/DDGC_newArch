//! Glutton Pawnshop — Cross-dungeon controller boss (tag-based debuff mechanic).
//!
//! DDGC reference: Eldritch-type boss from Cross dungeon (饕餮当铺).
//! Tier 1 base stats: HP 150, DEF 15%, PROT 0.5, SPD 5, 2 turns/round.
//! Skills: flesh_usury_contract, compound_agony, invitation, foreclosed_wail.
//!
//! The Glutton Pawnshop is the only DDGC boss with a `controller:` block,
//! using a tag-based debuff system. Its signature mechanic is a two-turn
//! combo: flesh_usury_contract applies a "gp_control" tag to a hero, then
//! compound_agony triggers tag-based bleed/blight/debuff on marked targets.
//! Invitation marks a target for increased damage, and foreclosed_wail
//! applies AoE stress pressure.
//!
//! AI brain: always uses flesh_usury_contract first (base_chance 1000),
//! then randomly selects compound_agony (35%), invitation (35%),
//! or foreclosed_wail (30%).
//!
//! Game-gaps:
//! - The `controller:` block mechanic is not modeled — the framework has no
//!   concept of a "controller" entity that directs tag application
//! - "gp_control" tag from flesh_usury_contract modeled as
//!   apply_status("gp_control", Some(1)) — actual tag-driven behavior not modeled
//! - "gp Tag Bleed/Blight/Debuff" from compound_agony modeled as separate
//!   apply_status("gp_tag_bleed"/"gp_tag_blight"/"gp_tag_debuff", Some(1)) —
//!   the conditional tag-triggered behavior (only affects gp_control-marked targets)
//!   is not modeled
//! - $1234 conditional targeting (targets heroes with specific marks) approximated
//!   as AllEnemies — same game-gap as other conditional targeting
//! - invitation "Mark Target" modeled as apply_status("mark", Some(1))
//! - invitation is_ignore_def True modeled by skill name only — framework has no
//!   "bypass protection" mechanic
//! - ~1234 AoE targeting on foreclosed_wail approximated as AllEnemies
//! - Position-based targeting (launch 1/234, target 1234/$1234/~1234) not modeled
//! - PROT (0.5), MAGIC_PROT (0.5), Stun Resist 100% (immune), Poison Resist 100%
//!   (immune), Bleed Resist 100% (immune), Debuff Resist 40%, Move Resist 100%
//!   (immune), Burn Resist 25%, Frozen Resist 25% not modeled
//! - 2 turns per round not modeled in Archetype
//! - Size 3 (largest boss, occupies 3 slots) not modeled in Archetype

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Glutton Pawnshop base archetype — tier 1 boss stats from DDGC data.
///
/// HP 150, weapon damage derived from compound_agony/invitation skills
/// (dmg 10–12 avg 11.0), speed 5, defense 0.15 (15% dodge).
/// Controller role: tag-based debuff mechanic with control + conditional effects.
/// Crit 5% from skills.
/// PROT 0.5, MAGIC_PROT 0.5, Stun Resist 100%, Poison Resist 100%,
/// Bleed Resist 100%, Debuff Resist 40%, Move Resist 100%,
/// Burn Resist 25%, Frozen Resist 25% — all not modeled.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Glutton Pawnshop"),
        side: CombatSide::Enemy,
        health: 150.0,
        max_health: 150.0,
        attack: 11.0,
        defense: 0.15,
        speed: 5.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.05,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Glutton Pawnshop Skills ─────────────────────────────────────────────

/// Flesh Usury Contract — applies gp_control tag to a single target.
///
/// DDGC reference: dmg 0–0, atk 85%, crit 5%,
/// launch rank 1, target 1234 (any single target),
/// effect "gp_control 1".
/// Game-gap: "gp_control" tag modeled as apply_status("gp_control", Some(1)) —
///   the tag-driven conditional mechanic is not modeled.
/// Game-gap: 1234 targeting approximated as AllEnemies.
pub fn flesh_usury_contract() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("flesh_usury_contract"),
        vec![EffectNode::apply_status("gp_control", Some(1))],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Compound Agony — tag-based bleed/blight/debuff trigger on marked targets.
///
/// DDGC reference: dmg 10–12, atk 85%, crit 5%,
/// launch ranks 2,3,4, target $1234 (conditional — targets heroes with
/// specific marks), effects "gp Tag Bleed" "gp Tag Blight" "gp Tag Debuff".
/// Game-gap: "gp Tag Bleed" modeled as apply_status("gp_tag_bleed", Some(1)).
/// Game-gap: "gp Tag Blight" modeled as apply_status("gp_tag_blight", Some(1)).
/// Game-gap: "gp Tag Debuff" modeled as apply_status("gp_tag_debuff", Some(1)).
/// Game-gap: $1234 conditional targeting (only affects gp_control-marked targets)
///   approximated as AllEnemies — the conditional mechanic is not modeled.
pub fn compound_agony() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("compound_agony"),
        vec![
            EffectNode::damage(11.0),
            EffectNode::apply_status("gp_tag_bleed", Some(1)),
            EffectNode::apply_status("gp_tag_blight", Some(1)),
            EffectNode::apply_status("gp_tag_debuff", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Invitation — marks target with ignore-defense strike.
///
/// DDGC reference: dmg 10–12, atk 85%, crit 5%,
/// launch ranks 2,3,4, target 1234 (any single target),
/// effect "Mark Target", is_ignore_def True.
/// Game-gap: "Mark Target" modeled as apply_status("mark", Some(1)).
/// Game-gap: is_ignore_def modeled by skill name only — framework has no
///   "bypass protection" mechanic.
/// Game-gap: 1234 targeting approximated as AllEnemies.
pub fn invitation() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("invitation"),
        vec![
            EffectNode::damage(11.0),
            EffectNode::apply_status("mark", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Foreclosed Wail — AoE stress attack.
///
/// DDGC reference: dmg 0–0, atk 85%, crit 0%,
/// launch ranks 2,3,4, target ~1234 (AoE all ranks except rank 1),
/// effect "Stress 2".
/// Game-gap: ~1234 AoE targeting approximated as AllEnemies.
pub fn foreclosed_wail() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("foreclosed_wail"),
        vec![EffectNode::apply_status("stress", Some(2))],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 4 Glutton Pawnshop skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        flesh_usury_contract(),
        compound_agony(),
        invitation(),
        foreclosed_wail(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glutton_pawnshop_archetype_is_enemy_eldritch_controller() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Glutton Pawnshop");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 150.0);
        assert_eq!(arch.max_health, 150.0);
        assert_eq!(arch.speed, 5.0, "glutton_pawnshop has SPD 5");
        assert_eq!(arch.defense, 0.15, "glutton_pawnshop has 15% defense");
        assert_eq!(arch.attack, 11.0, "attack from compound_agony/invitation avg 10-12");
        assert_eq!(arch.crit_chance, 0.05, "crit 5% from skills");
    }

    #[test]
    fn glutton_pawnshop_flesh_usury_contract_applies_gp_control() {
        let skill = flesh_usury_contract();
        assert_eq!(skill.id.0, "flesh_usury_contract");
        let has_gp_control = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("gp_control")
        });
        assert!(has_gp_control, "flesh_usury_contract must apply gp_control status");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(!has_damage, "flesh_usury_contract deals no damage");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllEnemies",
            "flesh_usury_contract targets enemies (target 1234)"
        );
    }

    #[test]
    fn glutton_pawnshop_compound_agony_applies_tag_based_debuffs() {
        let skill = compound_agony();
        assert_eq!(skill.id.0, "compound_agony");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "compound_agony must deal damage");
        let has_tag_bleed = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("gp_tag_bleed")
        });
        assert!(has_tag_bleed, "compound_agony must apply gp_tag_bleed status");
        let has_tag_blight = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("gp_tag_blight")
        });
        assert!(has_tag_blight, "compound_agony must apply gp_tag_blight status");
        let has_tag_debuff = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("gp_tag_debuff")
        });
        assert!(has_tag_debuff, "compound_agony must apply gp_tag_debuff status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllEnemies",
            "compound_agony targets all enemies ($1234 conditional)"
        );
    }

    #[test]
    fn glutton_pawnshop_invitation_applies_mark_and_damage() {
        let skill = invitation();
        assert_eq!(skill.id.0, "invitation");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "invitation must deal damage");
        let has_mark = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("mark")
        });
        assert!(has_mark, "invitation must apply mark status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllEnemies",
            "invitation targets enemies (target 1234)"
        );
    }

    #[test]
    fn glutton_pawnshop_foreclosed_wail_applies_stress() {
        let skill = foreclosed_wail();
        assert_eq!(skill.id.0, "foreclosed_wail");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "foreclosed_wail must apply stress status");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(!has_damage, "foreclosed_wail deals no damage");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllEnemies",
            "foreclosed_wail targets all enemies (~1234 AoE)"
        );
    }

    #[test]
    fn glutton_pawnshop_skill_pack_has_four_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 4);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"flesh_usury_contract"), "missing flesh_usury_contract");
        assert!(ids.contains(&"compound_agony"), "missing compound_agony");
        assert!(ids.contains(&"invitation"), "missing invitation");
        assert!(ids.contains(&"foreclosed_wail"), "missing foreclosed_wail");
    }

    #[test]
    fn glutton_pawnshop_controller_plus_tag_identity() {
        // The core identity of glutton_pawnshop is a controller boss that
        // applies a control tag (gp_control) and then triggers tag-based
        // bleed/blight/debuff effects on marked targets.
        let pack = skill_pack();

        let has_control = pack.iter().any(|s| {
            s.id.0 == "flesh_usury_contract"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("gp_control")
                })
        });

        let has_tag_bleed = pack.iter().any(|s| {
            s.id.0 == "compound_agony"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("gp_tag_bleed")
                })
        });

        let has_tag_blight = pack.iter().any(|s| {
            s.id.0 == "compound_agony"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("gp_tag_blight")
                })
        });

        let has_tag_debuff = pack.iter().any(|s| {
            s.id.0 == "compound_agony"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("gp_tag_debuff")
                })
        });

        assert!(has_control, "glutton_pawnshop must have gp_control skill");
        assert!(has_tag_bleed, "glutton_pawnshop must have gp_tag_bleed in compound_agony");
        assert!(has_tag_blight, "glutton_pawnshop must have gp_tag_blight in compound_agony");
        assert!(has_tag_debuff, "glutton_pawnshop must have gp_tag_debuff in compound_agony");
    }
}
