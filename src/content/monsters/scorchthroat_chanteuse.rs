//! Scorchthroat Chanteuse — XuanWu boss (summon-instrument + burn-magic pressure).
//!
//! DDGC reference: Eldritch-type boss from the XuanWu dungeon.
//! Tier 1 base stats: HP 150, DEF 15%, PROT 0.5, SPD 6, 2 turns/round.
//! Skills: cremona_last_chord, pyre_resonance, ashen_communion,
//! encore_embers, move.
//!
//! The Scorchthroat Chanteuse is a summon-burn boss that pressures heroes with
//! magic damage, burn DoT, and stress while continuously summoning instrument
//! minions (sc_blow/bow/pluck) that add stress and magic pressure. Its AI
//! prioritizes re-summoning instruments when alone (encore_embers with 10K
//! base_chance when only 1 monster remains).
//!
//! Game-gaps:
//! - Summon mechanic (encore_embers) modeled as status marker only
//! - AI priority system (10M base_chance when monsters_min: 1) not modeled
//! - Position-based targeting (launch 12, target ~1234/1234/$1234/@23)
//!   approximated as AllEnemies/AllAllies
//! - ~1234 inverted targeting on cremona_last_chord approximated as AllEnemies
//! - $1234 conditional targeting on ashen_communion approximated as AllEnemies
//! - cremona_last_chord ignore-DEF property modeled as status marker only
//! - Two turns per round not modeled in Archetype
//! - PROT (0.5), MAGIC_PROT (0.5) not modeled in Archetype
//! - Stun Resist 50%, Poison Resist 25%, Bleed Resist 200% (immune),
//!   Debuff Resist 40%, Move Resist 50%, Burn Resist 200% (immune),
//!   Frozen Resist 25% not modeled
//! - magic_dmg on pyre_resonance/ashen_communion not modeled (treated as normal damage)
//! - Self-movement on move skill dropped as game-gap

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Scorchthroat Chanteuse base archetype — tier 1 boss stats from DDGC data.
///
/// HP 150, weapon damage derived from pyre_resonance skill (12–24 avg 18),
/// speed 6, defense 0.15 (15% dodge).
/// Summoner role: summons instrument minions and pressures with burn/stress.
/// Crit 5% from cremona_last_chord/pyre_resonance/ashen_communion.
/// PROT 0.5, MAGIC_PROT 0.5, Stun Resist 50%, Poison Resist 25%,
/// Bleed Resist 200% (immune), Debuff Resist 40%, Move Resist 50%,
/// Burn Resist 200% (immune), Frozen Resist 25% — all not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Scorchthroat Chanteuse"),
        side: CombatSide::Enemy,
        health: 150.0,
        max_health: 150.0,
        attack: 18.0,
        defense: 0.15,
        speed: 6.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.05,
        dodge: 0.0,
    }
}

// ── Scorchthroat Chanteuse Skills ────────────────────────────────────────────

/// Cremona Last Chord — ranged magic damage + stress, ignores defense.
///
/// DDGC reference: dmg 4–6 (magic), atk 85%, crit 5%,
/// launch ranks 1,2, target ~1234 (all, ignore def),
/// effects "Stress 1".
/// .is_ignore_def True — this attack bypasses protection.
/// Game-gap: ignore-DEF property modeled as status marker only.
/// Game-gap: magic damage type not modeled — treated as normal damage.
/// Game-gap: ~1234 inverted targeting approximated as AllEnemies.
pub fn cremona_last_chord() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("cremona_last_chord"),
        vec![
            EffectNode::damage(5.0),
            EffectNode::apply_status("stress", Some(1)),
            EffectNode::apply_status("ignore_def", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Pyre Resonance — ranged heavy magic damage + burn.
///
/// DDGC reference: dmg 12–24 (magic), atk 85%, crit 5%,
/// launch ranks 1,2, target 1234 (any single enemy),
/// effects "Burn 1".
/// Game-gap: magic damage type not modeled — treated as normal damage.
pub fn pyre_resonance() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("pyre_resonance"),
        vec![
            EffectNode::damage(18.0),
            EffectNode::apply_status("burn", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Ashen Communion — AoE magic damage + burn spread.
///
/// DDGC reference: dmg 2–4 (magic), atk 80%, crit 5%,
/// launch ranks 1,2, target $1234 (all/spread),
/// effects "Burn 1".
/// Game-gap: $1234 conditional targeting approximated as AllEnemies.
/// Game-gap: magic damage type not modeled — treated as normal damage.
pub fn ashen_communion() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("ashen_communion"),
        vec![
            EffectNode::damage(3.0),
            EffectNode::apply_status("burn", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Encore Embers — summon instrument minions (sc_blow/sc_bow/sc_pluck).
///
/// DDGC reference: dmg 0–0, atk 100%, crit 0%,
/// launch rank 1, target (open),
/// effect "sc Summon" — summons 2 of {sc_blow, sc_bow, sc_pluck},
/// each capped at 1 per combat, placed at rank 4.
/// AI behavior: fires with overwhelming priority (10K base_chance)
/// when only 1 monster remains (monsters_min: 1, monsters_max: 1).
/// Game-gap: summon mechanic modeled as status marker only.
/// Game-gap: AI conditional priority not modeled.
/// Game-gap: summon count and selection logic not modeled.
pub fn encore_embers() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("encore_embers"),
        vec![EffectNode::apply_status("summon_instrument", Some(1))],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// Move — self-reposition skill.
///
/// DDGC reference: dmg 0–0, atk 0%, crit 0%,
/// launch ranks 3,4, target @23 (ally ranks 2-3), .move 0 1.
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

/// All 5 Scorchthroat Chanteuse skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        cremona_last_chord(),
        pyre_resonance(),
        ashen_communion(),
        encore_embers(),
        move_skill(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scorchthroat_chanteuse_archetype_is_enemy_eldritch_summoner() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Scorchthroat Chanteuse");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 150.0);
        assert_eq!(arch.max_health, 150.0);
        assert_eq!(arch.speed, 6.0, "scorchthroat_chanteuse has SPD 6");
        assert_eq!(arch.defense, 0.15, "scorchthroat_chanteuse has 15% defense");
        assert_eq!(arch.attack, 18.0, "attack from pyre_resonance avg 12-24");
        assert_eq!(arch.crit_chance, 0.05, "crit 5% from main skills");
    }

    #[test]
    fn scorchthroat_chanteuse_cremona_last_chord_applies_stress_and_ignore_def() {
        let skill = cremona_last_chord();
        assert_eq!(skill.id.0, "cremona_last_chord");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "cremona_last_chord must deal damage");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "cremona_last_chord must apply stress status");
        let has_ignore_def = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("ignore_def")
        });
        assert!(has_ignore_def, "cremona_last_chord must apply ignore_def status");
    }

    #[test]
    fn scorchthroat_chanteuse_pyre_resonance_applies_burn() {
        let skill = pyre_resonance();
        assert_eq!(skill.id.0, "pyre_resonance");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "pyre_resonance must deal damage");
        let has_burn = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("burn")
        });
        assert!(has_burn, "pyre_resonance must apply burn status");
    }

    #[test]
    fn scorchthroat_chanteuse_ashen_communion_applies_burn() {
        let skill = ashen_communion();
        assert_eq!(skill.id.0, "ashen_communion");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "ashen_communion must deal damage");
        let has_burn = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("burn")
        });
        assert!(has_burn, "ashen_communion must apply burn status");
    }

    #[test]
    fn scorchthroat_chanteuse_encore_embers_is_summon() {
        let skill = encore_embers();
        assert_eq!(skill.id.0, "encore_embers");
        let has_summon = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("summon_instrument")
        });
        assert!(has_summon, "encore_embers must apply summon_instrument status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "encore_embers targets self (summon for own team)"
        );
    }

    #[test]
    fn scorchthroat_chanteuse_skill_pack_has_five_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 5);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"cremona_last_chord"), "missing cremona_last_chord");
        assert!(ids.contains(&"pyre_resonance"), "missing pyre_resonance");
        assert!(ids.contains(&"ashen_communion"), "missing ashen_communion");
        assert!(ids.contains(&"encore_embers"), "missing encore_embers");
        assert!(ids.contains(&"move"), "missing move");
    }

    #[test]
    fn scorchthroat_chanteuse_summon_plus_burn_plus_stress_identity() {
        // The core identity of scorchthroat_chanteuse is a summon-burn boss that
        // pressures with magic damage, burn DoT, and stress while continuously
        // summoning instrument minions.
        let pack = skill_pack();

        let has_summon = pack.iter().any(|s| {
            s.id.0 == "encore_embers"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("summon_instrument")
                })
        });

        let has_burn = pack.iter().any(|s| {
            s.effects.iter().any(|e| {
                matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                    && e.status_kind.as_deref() == Some("burn")
            })
        });

        let has_stress = pack.iter().any(|s| {
            s.effects.iter().any(|e| {
                matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                    && e.status_kind.as_deref() == Some("stress")
            })
        });

        assert!(has_summon, "scorchthroat_chanteuse must have summon skill");
        assert!(has_burn, "scorchthroat_chanteuse must have burn skill");
        assert!(has_stress, "scorchthroat_chanteuse must have stress skill");
    }
}
