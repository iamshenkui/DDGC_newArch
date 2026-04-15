//! Azure Dragon Ball Thunder — QingLong boss part (thunder ball buff + stress).
//!
//! DDGC reference: Eldritch-type boss part from the QingLong dungeon.
//! Tier 1 base stats: HP 55, DEF 30%, PROT 0.2, SPD 10, 1 turn/round.
//! Skills: thunder_buff_magic (magic dmg buff), thunder_buff_stress (stress dmg buff),
//! thunder_stress_attack (AoE stress).
//!
//! This part's identity is a thunder-type support ball that buffs the main
//! boss's magic damage and stress output. It shares a health pool with
//! azure_dragon_ball_wind (the wind ball).
//!
//! Game-gaps:
//! - Shared health pool with wind ball not modeled
//! - Ball-type activation effects not modeled
//! - PROT (0.2), MAGIC_PROT (0.8) not modeled in Archetype
//! - Stun/Move immune (200% resist) not modeled
//! - Position-based targeting (launch 14, target ~@23) not modeled

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Azure Dragon Ball Thunder base archetype — tier 1 boss part stats from DDGC data.
///
/// HP 55, no damage skills (attack 0), speed 10 (faster than main body),
/// defense 0.30 (30% dodge).
/// Support role: buffs main boss magic damage and stress output.
/// PROT 0.2, MAGIC_PROT 0.8, Stun Resist 200% (immune),
/// Move Resist 200% (immune) — all not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Azure Dragon Ball Thunder"),
        side: CombatSide::Enemy,
        health: 55.0,
        max_health: 55.0,
        attack: 0.0,
        defense: 0.30,
        speed: 10.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.0,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Azure Dragon Ball Thunder Skills ─────────────────────────────────────

/// Thunder Buff Magic — buffs ally magic damage.
///
/// DDGC reference: dmg 0-0, atk 100%, crit 0%,
/// launch ranks 1,4, target ~@23 (AoE ally ranks 2-3),
/// effect "Azure Dragon Ball Buff 3" (+10% magic damage, 2 rounds).
/// Game-gap: ally-targeting simplified to AllAllies.
/// Game-gap: actual +10% magic damage modifier modeled as status marker.
pub fn thunder_buff_magic() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("thunder_buff_magic"),
        vec![EffectNode::apply_status("magic_dmg_up", Some(2))],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Thunder Buff Stress — buffs ally stress damage output.
///
/// DDGC reference: dmg 0-0, atk 100%, crit 0%,
/// launch ranks 1,4, target ~@23 (AoE ally ranks 2-3),
/// effect "Azure Dragon Ball Buff 4" (GIVE_STRESSDMG50, 3 rounds).
/// Game-gap: ally-targeting simplified to AllAllies.
/// Game-gap: actual stress damage modifier modeled as status marker.
pub fn thunder_buff_stress() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("thunder_buff_stress"),
        vec![EffectNode::apply_status("stress_dmg_up", Some(3))],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Thunder Stress Attack — applies stress to all enemies.
///
/// DDGC reference: dmg 0-0 (stress-only), atk 90%, crit 0%,
/// launch ranks 1,4, target ~1234 (AoE enemy ranks 1-4),
/// effect "Stress 7-10" (averaged to 9).
/// Game-gap: AoE vs single-target distinction not modeled — targets AllEnemies.
pub fn thunder_stress_attack() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("thunder_stress_attack"),
        vec![EffectNode::apply_status("stress", Some(9))],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 3 Azure Dragon Ball Thunder skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        thunder_buff_magic(),
        thunder_buff_stress(),
        thunder_stress_attack(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn azure_dragon_ball_thunder_archetype_is_enemy_eldritch_support() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Azure Dragon Ball Thunder");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 55.0);
        assert_eq!(arch.max_health, 55.0);
        assert_eq!(arch.speed, 10.0, "ball_thunder is faster than main body (SPD 10)");
        assert_eq!(arch.defense, 0.30, "ball_thunder has 30% defense");
        assert_eq!(arch.attack, 0.0, "ball_thunder has no damage skills");
    }

    #[test]
    fn azure_dragon_ball_thunder_buff_magic_applies_magic_dmg_up() {
        let skill = thunder_buff_magic();
        assert_eq!(skill.id.0, "thunder_buff_magic");
        let has_buff = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("magic_dmg_up")
        });
        assert!(has_buff, "thunder_buff_magic must apply magic_dmg_up status");
    }

    #[test]
    fn azure_dragon_ball_thunder_buff_stress_applies_stress_dmg_up() {
        let skill = thunder_buff_stress();
        assert_eq!(skill.id.0, "thunder_buff_stress");
        let has_buff = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress_dmg_up")
        });
        assert!(has_buff, "thunder_buff_stress must apply stress_dmg_up status");
    }

    #[test]
    fn azure_dragon_ball_thunder_stress_attack_applies_stress() {
        let skill = thunder_stress_attack();
        assert_eq!(skill.id.0, "thunder_stress_attack");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "thunder_stress_attack must apply stress status");
    }

    #[test]
    fn azure_dragon_ball_thunder_skill_pack_has_three_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 3);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"thunder_buff_magic"), "missing thunder_buff_magic");
        assert!(ids.contains(&"thunder_buff_stress"), "missing thunder_buff_stress");
        assert!(ids.contains(&"thunder_stress_attack"), "missing thunder_stress_attack");
    }

    #[test]
    fn azure_dragon_ball_thunder_buff_plus_stress_identity() {
        // The core identity of ball_thunder is a support that buffs
        // the main boss's magic/stress damage and applies AoE stress.
        let pack = skill_pack();

        let has_magic_buff = pack.iter().any(|s| {
            s.id.0 == "thunder_buff_magic"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("magic_dmg_up")
                })
        });

        let has_stress_buff = pack.iter().any(|s| {
            s.id.0 == "thunder_buff_stress"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stress_dmg_up")
                })
        });

        let has_stress_attack = pack.iter().any(|s| {
            s.id.0 == "thunder_stress_attack"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stress")
                })
        });

        assert!(has_magic_buff, "ball_thunder must have magic dmg buff");
        assert!(has_stress_buff, "ball_thunder must have stress dmg buff");
        assert!(has_stress_attack, "ball_thunder must have AoE stress attack");
    }
}
