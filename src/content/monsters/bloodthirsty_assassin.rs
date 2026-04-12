//! Bloodthirsty Assassin — Cross-dungeon paired boss (bruiser + HP-averaging).
//!
//! DDGC reference: Eldritch-type boss from Cross dungeon (嗜血刺客).
//! Tier 1 base stats: HP 150, DEF 15%, PROT 0.5, SPD 6, 2 turns/round.
//! Skills: bloodstrike_ambush, phantom_lunge, crimson_duet, scarlet_guillotine.
//!
//! The Bloodthirsty Assassin is a paired boss that fights alongside
//! bloodthirsty_shadow. Its signature mechanic is crimson_duet, which
//! averages HP between itself and its shadow partner. The assassin
//! serves as the primary damage dealer with heavy melee strikes and
//! an ignore-defense finisher (scarlet_guillotine).
//!
//! AI brain: always uses crimson_duet on the last turn of each round;
//! otherwise uses bloodstrike_ambush (40%), phantom_lunge (30%), or
//! scarlet_guillotine (30%).
//!
//! Game-gaps:
//! - crimson_duet "Average Shadow Hp" mechanic modeled as
//!   apply_status("average_hp", None) — the actual HP-averaging is not modeled
//! - scarlet_guillotine is_ignore_def True modeled by skill name only —
//!   framework has no "bypass protection" mechanic
//! - phantom_lunge "ba Bleed Debuff" modeled as apply_status("bleed_debuff", Some(1))
//! - phantom_lunge "Blight 1" modeled as apply_status("blight", Some(1))
//! - Position-based targeting (launch 12, target 4/~34/1234/@1234) approximated
//!   as AllEnemies or SelfOnly
//! - ~34 AoE targeting approximated as AllEnemies
//! - @1234 ally targeting approximated as AllAllies (includes self)
//! - PROT (0.5), MAGIC_PROT (0.5), Stun Resist 70%, Poison Resist 70%,
//!   Bleed Resist 30%, Debuff Resist 40%, Move Resist 200% (immune),
//!   Burn Resist 25%, Frozen Resist 25% not modeled
//! - 2 turns per round not modeled in Archetype
//! - Size 2 (occupies 2 slots) not modeled in Archetype

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Bloodthirsty Assassin base archetype — tier 1 boss stats from DDGC data.
///
/// HP 150, weapon damage derived from bloodstrike_ambush skill
/// (dmg 15–24 avg 19.5), speed 6, defense 0.15 (15% dodge).
/// Bruiser role: heavy melee damage with HP-averaging partner mechanic.
/// Crit 10% from bloodstrike_ambush.
/// PROT 0.5, MAGIC_PROT 0.5, Stun Resist 70%, Poison Resist 70%,
/// Bleed Resist 30%, Debuff Resist 40%, Move Resist 200% (immune),
/// Burn Resist 25%, Frozen Resist 25% — all not modeled.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Bloodthirsty Assassin"),
        side: CombatSide::Enemy,
        health: 150.0,
        max_health: 150.0,
        attack: 19.5,
        defense: 0.15,
        speed: 6.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.10,
        dodge: 0.0,
    }
}

// ── Bloodthirsty Assassin Skills ─────────────────────────────────────────────

/// Bloodstrike Ambush — heavy single-target melee strike.
///
/// DDGC reference: dmg 15–24, atk 70%, crit 10%,
/// launch ranks 1,2, target rank 4 (back-rank single target),
/// no additional effects.
/// Game-gap: single-rank targeting (target 4) approximated as AllEnemies.
pub fn bloodstrike_ambush() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("bloodstrike_ambush"),
        vec![EffectNode::damage(19.5)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Phantom Lunge — melee bleed + blight attack.
///
/// DDGC reference: dmg 10–12, atk 85%, crit 5%,
/// launch ranks 1,2, target ~34 (AoE ranks 3,4),
/// effects "ba Bleed Debuff" + "Blight 1".
/// Game-gap: ~34 AoE targeting approximated as AllEnemies.
/// Game-gap: "ba Bleed Debuff" modeled as apply_status("bleed_debuff", Some(1)).
/// Game-gap: "Blight 1" modeled as apply_status("blight", Some(1)).
pub fn phantom_lunge() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("phantom_lunge"),
        vec![
            EffectNode::damage(11.0),
            EffectNode::apply_status("bleed_debuff", Some(1)),
            EffectNode::apply_status("blight", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Crimson Duet — HP-averaging mechanic with shadow partner.
///
/// DDGC reference: dmg 0–0, atk 100%, crit 0%,
/// launch ranks 1,2, target @1234 (all allies, not self),
/// effect "Average Shadow Hp" (averages HP between assassin and shadow).
/// Game-gap: "Average Shadow Hp" mechanic modeled as
///   apply_status("average_hp", None) — actual HP redistribution not modeled.
/// Game-gap: @1234 (any ally rank, not self) approximated as AllAllies.
pub fn crimson_duet() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("crimson_duet"),
        vec![EffectNode::apply_status("average_hp", None)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Scarlet Guillotine — ignore-defense finishing strike.
///
/// DDGC reference: dmg 10–12, atk 85%, crit 5%,
/// launch ranks 1,2, target 1234 (any rank),
/// is_ignore_def True.
/// Game-gap: is_ignore_def modeled by skill name only — framework has
///   no "bypass protection" mechanic.
/// Game-gap: 1234 targeting approximated as AllEnemies.
pub fn scarlet_guillotine() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("scarlet_guillotine"),
        vec![EffectNode::damage(11.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 4 Bloodthirsty Assassin skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        bloodstrike_ambush(),
        phantom_lunge(),
        crimson_duet(),
        scarlet_guillotine(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bloodthirsty_assassin_archetype_is_enemy_eldritch_bruiser() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Bloodthirsty Assassin");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 150.0);
        assert_eq!(arch.max_health, 150.0);
        assert_eq!(arch.speed, 6.0, "bloodthirsty_assassin has SPD 6");
        assert_eq!(arch.defense, 0.15, "bloodthirsty_assassin has 15% defense");
        assert_eq!(arch.attack, 19.5, "attack from bloodstrike_ambush avg 15-24");
        assert_eq!(arch.crit_chance, 0.10, "crit 10% from bloodstrike_ambush");
    }

    #[test]
    fn bloodthirsty_assassin_bloodstrike_ambush_deals_heavy_damage() {
        let skill = bloodstrike_ambush();
        assert_eq!(skill.id.0, "bloodstrike_ambush");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "bloodstrike_ambush must deal damage");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllEnemies",
            "bloodstrike_ambush targets enemies (target 4)"
        );
    }

    #[test]
    fn bloodthirsty_assassin_phantom_lunge_applies_bleed_debuff_and_blight() {
        let skill = phantom_lunge();
        assert_eq!(skill.id.0, "phantom_lunge");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "phantom_lunge must deal damage");
        let has_bleed_debuff = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("bleed_debuff")
        });
        assert!(has_bleed_debuff, "phantom_lunge must apply bleed_debuff status");
        let has_blight = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("blight")
        });
        assert!(has_blight, "phantom_lunge must apply blight status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllEnemies",
            "phantom_lunge targets all enemies (~34 AoE)"
        );
    }

    #[test]
    fn bloodthirsty_assassin_crimson_duet_applies_average_hp() {
        let skill = crimson_duet();
        assert_eq!(skill.id.0, "crimson_duet");
        let has_average_hp = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("average_hp")
        });
        assert!(has_average_hp, "crimson_duet must apply average_hp status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllAllies",
            "crimson_duet targets allies (@1234)"
        );
    }

    #[test]
    fn bloodthirsty_assassin_scarlet_guillotine_deals_damage() {
        let skill = scarlet_guillotine();
        assert_eq!(skill.id.0, "scarlet_guillotine");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "scarlet_guillotine must deal damage");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllEnemies",
            "scarlet_guillotine targets all enemies (target 1234)"
        );
    }

    #[test]
    fn bloodthirsty_assassin_skill_pack_has_four_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 4);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"bloodstrike_ambush"), "missing bloodstrike_ambush");
        assert!(ids.contains(&"phantom_lunge"), "missing phantom_lunge");
        assert!(ids.contains(&"crimson_duet"), "missing crimson_duet");
        assert!(ids.contains(&"scarlet_guillotine"), "missing scarlet_guillotine");
    }

    #[test]
    fn bloodthirsty_assassin_paired_boss_hp_averaging_identity() {
        // The core identity of bloodthirsty_assassin is a paired boss that
        // averages HP with its shadow partner, while serving as the primary
        // damage dealer with heavy melee and ignore-defense finisher.
        let pack = skill_pack();

        let has_average_hp = pack.iter().any(|s| {
            s.id.0 == "crimson_duet"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("average_hp")
                })
        });

        let has_heavy_damage = pack.iter().any(|s| {
            s.id.0 == "bloodstrike_ambush"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Damage)
                })
        });

        let has_ignore_def = pack.iter().any(|s| {
            s.id.0 == "scarlet_guillotine"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Damage)
                })
        });

        assert!(has_average_hp, "bloodthirsty_assassin must have HP-averaging skill");
        assert!(has_heavy_damage, "bloodthirsty_assassin must have heavy damage skill");
        assert!(has_ignore_def, "bloodthirsty_assassin must have ignore-def finisher");
    }
}
