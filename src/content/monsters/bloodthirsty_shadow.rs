//! Bloodthirsty Shadow — Cross-dungeon paired boss support unit.
//!
//! DDGC reference: Eldritch-type boss part from Cross dungeon (嗜血暗影).
//! Tier 1 base stats: HP 150, DEF 15%, PROT 0.5, SPD 6, 1 turn/round.
//! Skills: haemogorging_aura, phantom_resonance, umbral_cyclone.
//!
//! The Bloodthirsty Shadow is the paired support unit that fights alongside
//! bloodthirsty_assassin. It provides AoE stress pressure, buffs the assassin
//! via phantom_resonance, and deals magic bleed damage. Its AI brain
//! reacts to the assassin's phantom_lunge by using umbral_cyclone, otherwise
//! alternating between haemogorging_aura (50%) and phantom_resonance (50%).
//!
//! Game-gaps:
//! - phantom_resonance "bs Buff" modeled as apply_status("assassin_buff", None)
//!   — the actual buff to the assassin is not modeled
//! - haemogorging_aura Stress 2 modeled as apply_status("stress", Some(2))
//! - umbral_cyclone "Strong 100 Bleed 1" modeled as apply_status("bleed", Some(1))
//! - AI brain reactive skill (umbral_cyclone after phantom_lunge) not modeled
//! - Position-based targeting (launch 34, target ~1234/1234/@1234) approximated
//!   as AllEnemies or AllAllies
//! - ~1234 AoE targeting approximated as AllEnemies
//! - @1234 ally targeting approximated as AllAllies
//! - PROT (0.5), MAGIC_PROT (0.5), Stun Resist 100% (immune),
//!   Poison Resist 100% (immune), Bleed Resist 100% (immune),
//!   Debuff Resist 40%, Move Resist 200% (immune),
//!   Burn Resist 25%, Frozen Resist 25% not modeled
//! - 1 turn per round — standard, no special handling needed
//! - Size 2 (occupies 2 slots) not modeled in Archetype

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Bloodthirsty Shadow base archetype — tier 1 boss stats from DDGC data.
///
/// HP 150, weapon damage derived from haemogorging_aura/umbral_cyclone
/// (magic_dmg 5–6 avg 5.5), speed 6, defense 0.15 (15% dodge).
/// Support role: buffs the assassin and provides AoE stress + bleed pressure.
/// Crit 5% from umbral_cyclone.
/// PROT 0.5, MAGIC_PROT 0.5, Stun Resist 100% (immune),
/// Poison Resist 100% (immune), Bleed Resist 100% (immune),
/// Debuff Resist 40%, Move Resist 200% (immune),
/// Burn Resist 25%, Frozen Resist 25% — all not modeled.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Bloodthirsty Shadow"),
        side: CombatSide::Enemy,
        health: 150.0,
        max_health: 150.0,
        attack: 5.5,
        defense: 0.15,
        speed: 6.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.05,
        dodge: 0.0,
    }
}

// ── Bloodthirsty Shadow Skills ───────────────────────────────────────────────

/// Haemogorging Aura — AoE magic damage + stress.
///
/// DDGC reference: magic_dmg 5–6, atk 85%, crit 0%,
/// launch ranks 3,4, target ~1234 (AoE all ranks),
/// effect "Stress 2".
/// Game-gap: ~1234 AoE targeting approximated as AllEnemies.
pub fn haemogorging_aura() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("haemogorging_aura"),
        vec![
            EffectNode::damage(5.5),
            EffectNode::apply_status("stress", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Phantom Resonance — buffs the assassin partner.
///
/// DDGC reference: dmg 0–0, atk 100%, crit 0%,
/// launch ranks 3,4, target @1234 (all allies, not self),
/// effect "bs Buff" (buffs the bloodthirsty_assassin).
/// Game-gap: "bs Buff" modeled as apply_status("assassin_buff", None) —
///   actual buff mechanic not modeled.
/// Game-gap: @1234 ally targeting approximated as AllAllies.
pub fn phantom_resonance() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("phantom_resonance"),
        vec![EffectNode::apply_status("assassin_buff", None)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Umbral Cyclone — magic damage + bleed.
///
/// DDGC reference: magic_dmg 5–6, atk 85%, crit 5%,
/// launch ranks 3,4, target 1234 (any rank),
/// effect "Strong 100 Bleed 1".
/// Game-gap: "Strong 100 Bleed 1" modeled as apply_status("bleed", Some(1)).
/// Game-gap: 1234 targeting approximated as AllEnemies.
pub fn umbral_cyclone() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("umbral_cyclone"),
        vec![
            EffectNode::damage(5.5),
            EffectNode::apply_status("bleed", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 3 Bloodthirsty Shadow skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        haemogorging_aura(),
        phantom_resonance(),
        umbral_cyclone(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bloodthirsty_shadow_archetype_is_enemy_eldritch_support() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Bloodthirsty Shadow");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 150.0);
        assert_eq!(arch.max_health, 150.0);
        assert_eq!(arch.speed, 6.0, "bloodthirsty_shadow has SPD 6");
        assert_eq!(arch.defense, 0.15, "bloodthirsty_shadow has 15% defense");
        assert_eq!(arch.attack, 5.5, "attack from magic_dmg 5-6 avg 5.5");
        assert_eq!(arch.crit_chance, 0.05, "crit 5% from umbral_cyclone");
    }

    #[test]
    fn bloodthirsty_shadow_haemogorging_aura_applies_damage_and_stress() {
        let skill = haemogorging_aura();
        assert_eq!(skill.id.0, "haemogorging_aura");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "haemogorging_aura must deal damage");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "haemogorging_aura must apply stress status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllEnemies",
            "haemogorging_aura targets all enemies (~1234 AoE)"
        );
    }

    #[test]
    fn bloodthirsty_shadow_phantom_resonance_applies_assassin_buff() {
        let skill = phantom_resonance();
        assert_eq!(skill.id.0, "phantom_resonance");
        let has_buff = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("assassin_buff")
        });
        assert!(has_buff, "phantom_resonance must apply assassin_buff status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllAllies",
            "phantom_resonance targets allies (@1234)"
        );
    }

    #[test]
    fn bloodthirsty_shadow_umbral_cyclone_applies_damage_and_bleed() {
        let skill = umbral_cyclone();
        assert_eq!(skill.id.0, "umbral_cyclone");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "umbral_cyclone must deal damage");
        let has_bleed = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("bleed")
        });
        assert!(has_bleed, "umbral_cyclone must apply bleed status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllEnemies",
            "umbral_cyclone targets all enemies (target 1234)"
        );
    }

    #[test]
    fn bloodthirsty_shadow_skill_pack_has_three_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 3);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"haemogorging_aura"), "missing haemogorging_aura");
        assert!(ids.contains(&"phantom_resonance"), "missing phantom_resonance");
        assert!(ids.contains(&"umbral_cyclone"), "missing umbral_cyclone");
    }

    #[test]
    fn bloodthirsty_shadow_stress_plus_buff_plus_bleed_identity() {
        // The core identity of bloodthirsty_shadow is the paired support unit
        // that buffs the assassin and provides AoE stress + bleed pressure.
        let pack = skill_pack();

        let has_stress = pack.iter().any(|s| {
            s.id.0 == "haemogorging_aura"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stress")
                })
        });

        let has_buff = pack.iter().any(|s| {
            s.id.0 == "phantom_resonance"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("assassin_buff")
                })
        });

        let has_bleed = pack.iter().any(|s| {
            s.id.0 == "umbral_cyclone"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("bleed")
                })
        });

        assert!(has_stress, "bloodthirsty_shadow must have stress skill");
        assert!(has_buff, "bloodthirsty_shadow must have assassin buff skill");
        assert!(has_bleed, "bloodthirsty_shadow must have bleed skill");
    }
}
