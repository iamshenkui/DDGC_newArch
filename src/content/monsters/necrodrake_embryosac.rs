//! Necrodrake Embryosac — XuanWu boss (capture-and-release + blight-pressure).
//!
//! DDGC reference: Man-type boss from the XuanWu dungeon.
//! Tier 1 base stats: HP 150, DEF 10%, PROT 0.5, SPD 5, 2 turns/round.
//! Skills: requiem_stillbirth, placental_tap, untimely_progeny,
//! doom_symbiosis, ecdysis_metamorphosis.
//!
//! The Necrodrake Embryosac is a capture-control boss that pressures heroes
//! with stress and accuracy debuffs, marks targets for increased damage,
//! and captures heroes into egg_membrane cauldrons. When a hero is captured,
//! the egg_membrane_empty transforms into egg_membrane_full, dealing passive
//! damage to the captured hero each round. The embryosac can self-cleanse
//! all debuffs and deal % HP damage via ecdysis_metamorphosis.
//!
//! Game-gaps:
//! - CaptureNe effect (untimely_progeny) modeled as status marker only
//! - egg_membrane_empty ↔ egg_membrane_full state transition not modeled
//! - life_link between necrodrake and egg_membrane variants not modeled
//! - per_turn_damage_range 5–6 (egg_membrane_full passive DoT) not modeled
//! - Position-based targeting (launch 34/43, target ~1234/1234)
//!   approximated as AllEnemies/AllAllies
//! - ~1234 AoE inverse targeting approximated as AllEnemies
//! - Two turns per round not modeled in Archetype
//! - PROT (0.5), MAGIC_PROT (0.7) not modeled in Archetype
//! - Stun Resist 50%, Poison Resist 50%, Bleed Resist 50%, Debuff Resist 40%,
//!   Move Resist 200% (immune), Burn Resist 25%, Frozen Resist 25% not modeled
//! - Magic damage type (placental_tap, doom_symbiosis) not modeled
//! - Heal Self Range 5–6 (placental_tap) modeled as heal_self status marker
//! - Mark Target 2 (placental_tap) modeled as status marker only
//! - Remove All Debuff (ecdysis_metamorphosis) modeled as cleanse_self status marker
//! - Damage Percent Target 2 (ecdysis_metamorphosis) modeled as status marker only
//! - ACC Debuff 2 + Target Dodge -2 (requiem_stillbirth) modeled as status markers
//! - ecdysis_metamorphosis targets self (.launch 43 .target empty) — modeled as SelfOnly
//! - is_crit_valid False on ecdysis_metamorphosis not modeled

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Necrodrake Embryosac base archetype — tier 1 boss stats from DDGC data.
///
/// HP 150, weapon damage derived from placental_tap skill (magic 10–12 avg 11),
/// speed 5, defense 0.10 (10% dodge).
/// Summoner role: captures heroes into egg_membrane cauldrons and pressures
/// with blight, stress, and mark.
/// Crit 12% from requiem_stillbirth/placental_tap.
/// PROT 0.5, MAGIC_PROT 0.7, Stun Resist 50%, Poison Resist 50%,
/// Bleed Resist 50%, Debuff Resist 40%, Move Resist 200% (immune),
/// Burn Resist 25%, Frozen Resist 25% — all not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Necrodrake Embryosac"),
        side: CombatSide::Enemy,
        health: 150.0,
        max_health: 150.0,
        attack: 11.0,
        defense: 0.10,
        speed: 5.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.12,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Necrodrake Embryosac Skills ──────────────────────────────────────────────

/// Requiem Stillbirth — AoE stress + accuracy debuff + dodge debuff.
///
/// DDGC reference: dmg 0–0, atk 64%, crit 12%,
/// launch ranks 3,4, target ~1234 (AoE all ranks),
/// effects "Stress 1" + "ACC Debuff 2" + "Target Dodge -2".
/// Game-gap: ~1234 AoE targeting approximated as AllEnemies.
/// Game-gap: ACC Debuff 2 modeled as status marker only.
/// Game-gap: Target Dodge -2 modeled as status marker only.
pub fn requiem_stillbirth() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("requiem_stillbirth"),
        vec![
            EffectNode::apply_status("stress", Some(1)),
            EffectNode::apply_status("acc_debuff", Some(2)),
            EffectNode::apply_status("dodge_debuff", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Placental Tap — mark target + heal self.
///
/// DDGC reference: magic dmg 10–12, atk 64%, crit 12%,
/// launch ranks 4,3, target 1234 (any rank),
/// effects "Mark Target 2" + "Heal Self Range 5-6".
/// Game-gap: magic damage type not modeled — treated as normal damage.
/// Game-gap: Mark Target 2 modeled as status marker only.
/// Game-gap: Heal Self Range 5-6 modeled as heal_self status marker (avg 5.5 → 5).
pub fn placental_tap() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("placental_tap"),
        vec![
            EffectNode::damage(11.0),
            EffectNode::apply_status("mark", Some(2)),
            EffectNode::apply_status("heal_self", Some(5)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Untimely Progeny — capture a hero into egg_membrane.
///
/// DDGC reference: dmg 0–0, atk 100%, crit 6%,
/// launch ranks 3,4, target 1234 (any rank),
/// effect "CaptureNe".
/// Game-gap: CaptureNe effect modeled as status marker only.
/// Game-gap: egg_membrane_empty → egg_membrane_full transition not modeled.
/// Game-gap: Captured hero is placed inside egg_membrane_full not modeled.
pub fn untimely_progeny() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("untimely_progeny"),
        vec![EffectNode::apply_status("capture", Some(1))],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Doom Symbiosis — AoE blight.
///
/// DDGC reference: magic dmg 5–6, atk 85%, crit 6%,
/// launch ranks 4,3, target ~1234 (AoE all ranks),
/// effect "Blight 1".
/// Game-gap: ~1234 AoE targeting approximated as AllEnemies.
/// Game-gap: magic damage type not modeled — treated as normal damage.
pub fn doom_symbiosis() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("doom_symbiosis"),
        vec![
            EffectNode::damage(5.5),
            EffectNode::apply_status("blight", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Ecdysis Metamorphosis — self-cleanse + % HP damage.
///
/// DDGC reference: dmg 0–0, atk 100%, crit 0% (is_crit_valid False),
/// launch ranks 4,3, target self (empty .target field),
/// effects "Remove All Debuff" + "Damage Percent Target 2".
/// Game-gap: Remove All Debuff modeled as cleanse_self status marker only.
/// Game-gap: Damage Percent Target 2 (deals 2% of target's current HP)
/// modeled as damage_percent_target status marker only.
/// Game-gap: is_crit_valid False not modeled.
pub fn ecdysis_metamorphosis() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("ecdysis_metamorphosis"),
        vec![
            EffectNode::apply_status("cleanse_self", Some(1)),
            EffectNode::apply_status("damage_percent_target", Some(2)),
        ],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// All 5 Necrodrake Embryosac skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        requiem_stillbirth(),
        placental_tap(),
        untimely_progeny(),
        doom_symbiosis(),
        ecdysis_metamorphosis(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn necrodrake_embryosac_archetype_is_enemy_man_summoner() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Necrodrake Embryosac");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 150.0);
        assert_eq!(arch.max_health, 150.0);
        assert_eq!(arch.speed, 5.0, "necrodrake_embryosac has SPD 5");
        assert_eq!(arch.defense, 0.10, "necrodrake_embryosac has 10% defense");
        assert_eq!(arch.attack, 11.0, "attack from placental_tap avg 10-12");
        assert_eq!(arch.crit_chance, 0.12, "crit 12% from requiem_stillbirth/placental_tap");
    }

    #[test]
    fn necrodrake_embryosac_requiem_stillbirth_applies_stress_and_debuffs() {
        let skill = requiem_stillbirth();
        assert_eq!(skill.id.0, "requiem_stillbirth");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "requiem_stillbirth must apply stress status");
        let has_acc_debuff = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("acc_debuff")
        });
        assert!(has_acc_debuff, "requiem_stillbirth must apply acc_debuff status");
        let has_dodge_debuff = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("dodge_debuff")
        });
        assert!(has_dodge_debuff, "requiem_stillbirth must apply dodge_debuff status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllEnemies",
            "requiem_stillbirth targets all enemies (AoE ~1234)"
        );
    }

    #[test]
    fn necrodrake_embryosac_placental_tap_deals_damage_marks_and_heals() {
        let skill = placental_tap();
        assert_eq!(skill.id.0, "placental_tap");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "placental_tap must deal damage");
        let has_mark = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("mark")
        });
        assert!(has_mark, "placental_tap must apply mark status");
        let has_heal = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("heal_self")
        });
        assert!(has_heal, "placental_tap must apply heal_self status");
    }

    #[test]
    fn necrodrake_embryosac_untimely_progeny_applies_capture() {
        let skill = untimely_progeny();
        assert_eq!(skill.id.0, "untimely_progeny");
        let has_capture = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("capture")
        });
        assert!(has_capture, "untimely_progeny must apply capture status");
    }

    #[test]
    fn necrodrake_embryosac_doom_symbiosis_applies_blight() {
        let skill = doom_symbiosis();
        assert_eq!(skill.id.0, "doom_symbiosis");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "doom_symbiosis must deal damage");
        let has_blight = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("blight")
        });
        assert!(has_blight, "doom_symbiosis must apply blight status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllEnemies",
            "doom_symbiosis targets all enemies (AoE ~1234)"
        );
    }

    #[test]
    fn necrodrake_embryosac_ecdysis_metamorphosis_cleanses_and_pct_damages() {
        let skill = ecdysis_metamorphosis();
        assert_eq!(skill.id.0, "ecdysis_metamorphosis");
        let has_cleanse = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("cleanse_self")
        });
        assert!(has_cleanse, "ecdysis_metamorphosis must apply cleanse_self status");
        let has_pct_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("damage_percent_target")
        });
        assert!(has_pct_damage, "ecdysis_metamorphosis must apply damage_percent_target status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "ecdysis_metamorphosis targets self (empty .target in DDGC)"
        );
    }

    #[test]
    fn necrodrake_embryosac_skill_pack_has_five_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 5);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"requiem_stillbirth"), "missing requiem_stillbirth");
        assert!(ids.contains(&"placental_tap"), "missing placental_tap");
        assert!(ids.contains(&"untimely_progeny"), "missing untimely_progeny");
        assert!(ids.contains(&"doom_symbiosis"), "missing doom_symbiosis");
        assert!(ids.contains(&"ecdysis_metamorphosis"), "missing ecdysis_metamorphosis");
    }

    #[test]
    fn necrodrake_embryosac_capture_plus_blight_plus_cleanse_identity() {
        // The core identity of necrodrake_embryosac is a capture-control boss
        // that captures heroes into egg_membrane cauldrons while pressuring
        // with blight, stress, mark, and self-cleanse mechanics.
        let pack = skill_pack();

        let has_capture = pack.iter().any(|s| {
            s.id.0 == "untimely_progeny"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("capture")
                })
        });

        let has_blight = pack.iter().any(|s| {
            s.id.0 == "doom_symbiosis"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("blight")
                })
        });

        let has_cleanse = pack.iter().any(|s| {
            s.id.0 == "ecdysis_metamorphosis"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("cleanse_self")
                })
        });

        assert!(has_capture, "necrodrake_embryosac must have capture skill");
        assert!(has_blight, "necrodrake_embryosac must have blight skill");
        assert!(has_cleanse, "necrodrake_embryosac must have cleanse skill");
    }
}
