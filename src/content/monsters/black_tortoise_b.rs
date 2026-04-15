//! Black Tortoise B (Snake Body) — XuanWu boss part (controller + blight + debuff).
//!
//! DDGC reference: Beast-type boss part from the XuanWu dungeon.
//! Tier 1 base stats: HP 115, DEF 30%, PROT 0.4, SPD 7, 2 turns/round.
//! Skills: call_roll, rain_spray, freezing_cold, benefits_stress,
//! fangs_sprayed, armor, fangs_sprayed_1, snake_bites.
//!
//! This body is the fast, debuff-oriented Snake half of the Black Tortoise
//! composite boss. It specializes in AoE frozen (freezing_cold), blight
//! (fangs_sprayed), and debuff/stress (benefits_stress). Its `armor` skill
//! links 80% incoming damage to the Tortoise body (mirroring near_mountain_river).
//! When balanced, it marks heroes with SnakeMark and applies debuffs.
//! When unbalanced (rage == 0), it switches to heavy damage (snake_bites)
//! and targeted blight (fangs_sprayed_1).
//!
//! Game-gaps:
//! - Dual-body coordination (BlackTortoiseField rage/unbalance) not modeled
//! - Call Roll hero-marking mechanic modeled as status markers only
//! - Share Damage 80% mechanic modeled as status marker only
//! - Rage/unbalance phase transitions not modeled in Archetype
//! - $1234 conditional targeting (marked heroes) approximated as AllEnemies
//! - Two turns per round not modeled in Archetype
//! - PROT (0.4), MAGIC_PROT (0.4) not modeled in Archetype
//! - Position-based targeting (launch any, target ~1234/@1234) not modeled
//! - Disease Random chance mechanic simplified to status marker
//! - Revival mechanic (dead body returns at 20 HP after 2 turns) not modeled
//! - Pair bounce damage on marked heroes not modeled

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Black Tortoise B (Snake Body) base archetype — tier 1 boss stats from DDGC data.
///
/// HP 115, weapon damage derived from snake_bites skill (4-8 avg 6),
/// speed 7, defense 0.30 (30% dodge).
/// Controller role: fast debuffer with blight, stress, and share-damage.
/// Crit 6% from snake_bites skill.
/// PROT 0.4, MAGIC_PROT 0.4, Stun Resist 25%, Poison Resist 50%,
/// Bleed Resist 50%, Debuff Resist 40%, Move Resist 100% (immune),
/// Burn Resist 100% (immune), Frozen Resist 25% — all not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Black Tortoise B"),
        side: CombatSide::Enemy,
        health: 115.0,
        max_health: 115.0,
        attack: 6.0,
        defense: 0.30,
        speed: 7.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.06,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Black Tortoise B Skills ────────────────────────────────────────────────

/// Call Roll — mark 2 random heroes with SnakeMark and initialize BlackTortoise field.
///
/// DDGC reference: dmg 0-0, atk 0%, crit 0%,
/// launch any, target ~1234 (AoE all ranks),
/// effect "Snake Call The Roll Effect" (marks 2 heroes with SnakeMark;
/// initializes BlackTortoise field).
/// Always used first at round start (base_chance 1000.0).
/// Game-gap: hero-marking mechanic modeled as status marker only.
/// Game-gap: BlackTortoiseField initialization not modeled.
pub fn call_roll() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("snake_call_roll"),
        vec![EffectNode::apply_status("snake_mark", Some(1))],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Rain Spray — AoE magic attack with damp and accuracy debuff.
///
/// DDGC reference: dmg 1-2 (avg 1.5), atk 72%, crit 0%,
/// launch any, target ~1234 (AoE all ranks),
/// effect "Damp Acc Debuff 10" (applies Damp + -10% ACC for 3 turns).
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn rain_spray() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("snake_rain_spray"),
        vec![
            EffectNode::damage(1.5),
            EffectNode::apply_status("damp", Some(3)),
            EffectNode::apply_status("acc_down", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Freezing Cold — AoE magic attack with frozen effect.
///
/// DDGC reference: dmg 1-2 (avg 1.5), atk 72%, crit 0%,
/// launch any, target ~1234 (AoE all ranks),
/// effect "Frozen 1" (dotFrozen 2 for 3 turns).
/// Unlike the Tortoise's ice_spike (single-target), this is AoE.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn freezing_cold() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("freezing_cold"),
        vec![
            EffectNode::damage(1.5),
            EffectNode::apply_status("frozen", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Benefits Stress — debuff and stress attack.
///
/// DDGC reference: dmg 0-0, atk 72%, crit 0%,
/// launch any, target 1234 (single target),
/// effects "Snake Debuff 1" (-10% ACC, -20% DEF, 3 turns) + "Stress 2" (+15 stress).
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn benefits_stress() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("benefits_stress"),
        vec![
            EffectNode::apply_status("acc_down", Some(3)),
            EffectNode::apply_status("def_down", Some(3)),
            EffectNode::apply_status("stress", Some(15)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Fangs Sprayed — blight attack with disease chance.
///
/// DDGC reference: dmg 1-2 (avg 1.5), atk 72%, crit 0%,
/// launch any, target 1234 (single target),
/// effects "Blight 1" (dotPoison 2 for 3 turns) +
/// "Disease Random 7" (7% chance to apply a random disease).
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
/// Game-gap: Disease Random chance mechanic simplified to status marker.
pub fn fangs_sprayed() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("fangs_sprayed"),
        vec![
            EffectNode::damage(1.5),
            EffectNode::apply_status("blight", Some(3)),
            EffectNode::apply_status("disease", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Armor — share 80% damage with Tortoise body for 3 turns.
///
/// DDGC reference: dmg 0-0, atk 0%, crit 0%,
/// launch any, target @1234 (ally, not self — targets the Tortoise body),
/// effect "Share Damage 80" (links 80% incoming damage to Tortoise body for 3 turns).
/// Game-gap: cross-body damage sharing modeled as status marker only.
/// Game-gap: ally-targeting simplified to AllAllies.
pub fn armor() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("armor"),
        vec![EffectNode::apply_status("share_damage", Some(3))],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Fangs Sprayed 1 — targeted blight attack on marked heroes.
///
/// DDGC reference: dmg 3-6 (avg 4.5), atk 72%, crit 0%,
/// launch any, target $1234 (heroes with SnakeMark),
/// effects "Blight 1" (dotPoison 2 for 3 turns) +
/// "Disease Random 7" (7% chance random disease).
/// Unbalance-phase skill: targets marked heroes specifically.
/// Game-gap: $1234 conditional targeting (marked heroes) approximated as AllEnemies.
pub fn fangs_sprayed_1() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("fangs_sprayed_1"),
        vec![
            EffectNode::damage(4.5),
            EffectNode::apply_status("blight", Some(3)),
            EffectNode::apply_status("disease", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Snake Bites — heavy pure damage attack (unbalance phase).
///
/// DDGC reference: dmg 4-8 (avg 6), atk 72%, crit 6%,
/// launch any, target 1234 (single target), no named effect.
/// Unbalance-phase skill: highest single-hit damage in Snake kit.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn snake_bites() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("snake_bites"),
        vec![EffectNode::damage(6.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 8 Black Tortoise B skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        call_roll(),
        rain_spray(),
        freezing_cold(),
        benefits_stress(),
        fangs_sprayed(),
        armor(),
        fangs_sprayed_1(),
        snake_bites(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_tortoise_b_archetype_is_enemy_beast_controller_boss() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Black Tortoise B");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 115.0);
        assert_eq!(arch.max_health, 115.0);
        assert_eq!(arch.speed, 7.0);
        assert_eq!(arch.defense, 0.30, "black_tortoise_B has 30% defense");
        assert_eq!(arch.crit_chance, 0.06, "black_tortoise_B has 6% crit");
        assert_eq!(arch.attack, 6.0, "black_tortoise_B attack from snake_bites avg");
    }

    #[test]
    fn black_tortoise_b_call_roll_applies_snake_mark() {
        let skill = call_roll();
        assert_eq!(skill.id.0, "snake_call_roll");
        let has_mark = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("snake_mark")
        });
        assert!(has_mark, "call_roll must apply snake_mark status");
    }

    #[test]
    fn black_tortoise_b_rain_spray_deals_damage_damp_and_acc_down() {
        let skill = rain_spray();
        assert_eq!(skill.id.0, "snake_rain_spray");
        assert!(
            skill.effects.len() >= 3,
            "rain_spray should have damage + damp + acc_down"
        );
    }

    #[test]
    fn black_tortoise_b_freezing_cold_deals_damage_and_frozen() {
        let skill = freezing_cold();
        assert_eq!(skill.id.0, "freezing_cold");
        assert!(
            skill.effects.len() >= 2,
            "freezing_cold should have damage + frozen status"
        );
    }

    #[test]
    fn black_tortoise_b_benefits_stress_applies_debuffs_and_stress() {
        let skill = benefits_stress();
        assert_eq!(skill.id.0, "benefits_stress");
        let has_acc_down = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("acc_down")
        });
        let has_def_down = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("def_down")
        });
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_acc_down, "benefits_stress must apply acc_down");
        assert!(has_def_down, "benefits_stress must apply def_down");
        assert!(has_stress, "benefits_stress must apply stress");
    }

    #[test]
    fn black_tortoise_b_fangs_sprayed_deals_damage_blight_and_disease() {
        let skill = fangs_sprayed();
        assert_eq!(skill.id.0, "fangs_sprayed");
        assert!(
            skill.effects.len() >= 3,
            "fangs_sprayed should have damage + blight + disease"
        );
    }

    #[test]
    fn black_tortoise_b_armor_applies_share_damage() {
        let skill = armor();
        assert_eq!(skill.id.0, "armor");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllAllies",
            "armor targets allies (Tortoise body)"
        );
        let has_share = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("share_damage")
        });
        assert!(has_share, "armor must apply share_damage status");
    }

    #[test]
    fn black_tortoise_b_fangs_sprayed_1_deals_damage_blight_and_disease() {
        let skill = fangs_sprayed_1();
        assert_eq!(skill.id.0, "fangs_sprayed_1");
        assert!(
            skill.effects.len() >= 3,
            "fangs_sprayed_1 should have damage + blight + disease"
        );
    }

    #[test]
    fn black_tortoise_b_snake_bites_deals_heavy_damage() {
        let skill = snake_bites();
        assert_eq!(skill.id.0, "snake_bites");
        assert!(
            !skill.effects.is_empty(),
            "snake_bites should have at least damage"
        );
    }

    #[test]
    fn black_tortoise_b_skill_pack_has_eight_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 8);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"snake_call_roll"), "missing snake_call_roll");
        assert!(ids.contains(&"snake_rain_spray"), "missing snake_rain_spray");
        assert!(ids.contains(&"freezing_cold"), "missing freezing_cold");
        assert!(ids.contains(&"benefits_stress"), "missing benefits_stress");
        assert!(ids.contains(&"fangs_sprayed"), "missing fangs_sprayed");
        assert!(ids.contains(&"armor"), "missing armor");
        assert!(ids.contains(&"fangs_sprayed_1"), "missing fangs_sprayed_1");
        assert!(ids.contains(&"snake_bites"), "missing snake_bites");
    }

    #[test]
    fn black_tortoise_b_dual_body_plus_blight_debuff_identity() {
        // The core identity of black_tortoise_B is a controller body that:
        // 1. Marks heroes with SnakeMark (call_roll)
        // 2. Shares damage with the Tortoise body (armor)
        // 3. Controls heroes with frozen, debuff, and blight (freezing_cold, benefits_stress, fangs_sprayed)
        // 4. Has unbalance-phase heavy damage (snake_bites)
        let pack = skill_pack();

        // Must have SnakeMark mechanic
        let has_mark = pack.iter().any(|s| {
            s.id.0 == "snake_call_roll"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("snake_mark")
                })
        });

        // Must have share damage mechanic
        let has_share = pack.iter().any(|s| {
            s.id.0 == "armor"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("share_damage")
                })
        });

        // Must have frozen control
        let has_frozen = pack.iter().any(|s| {
            s.id.0 == "freezing_cold"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("frozen")
                })
        });

        // Must have blight mechanic
        let has_blight = pack.iter().any(|s| {
            s.id.0 == "fangs_sprayed"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("blight")
                })
        });

        // Must have unbalance-phase heavy damage
        let has_heavy = pack.iter().any(|s| {
            s.id.0 == "snake_bites"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Damage)
                })
        });

        assert!(has_mark, "black_tortoise_B must have SnakeMark mechanic");
        assert!(has_share, "black_tortoise_B must have share_damage mechanic");
        assert!(has_frozen, "black_tortoise_B must have frozen control");
        assert!(has_blight, "black_tortoise_B must have blight mechanic");
        assert!(has_heavy, "black_tortoise_B must have snake_bites heavy damage");
    }
}