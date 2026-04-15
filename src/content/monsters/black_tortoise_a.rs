//! Black Tortoise A (Tortoise Body) — XuanWu boss part (tank + share-damage + frozen control).
//!
//! DDGC reference: Beast-type boss part from the XuanWu dungeon.
//! Tier 1 base stats: HP 115, DEF 30%, PROT 0.8, SPD 4, 2 turns/round.
//! Skills: call_roll, rain_spray, ice_spike, hunger_cold,
//! inner_battle, near_mountain_river, hunger_cold_1, unexpectedly.
//!
//! This body is the slow, heavily armored Tortoise half of the Black Tortoise
//! composite boss. It specializes in control (damp/frozen debuffs, stress),
//! and the `near_mountain_river` skill links 80% incoming damage to the Snake
//! body. When balanced, it uses call_roll to mark heroes, then debuffs them.
//! When unbalanced (rage == 0), it switches to heavy damage (unexpectedly)
//! and targeted stress (hunger_cold_1).
//!
//! Game-gaps:
//! - Dual-body coordination (BlackTortoiseField rage/unbalance) not modeled
//! - Call Roll hero-marking mechanic modeled as status markers only
//! - Share Damage 80% mechanic modeled as status marker only
//! - Rage/unbalance phase transitions not modeled in Archetype
//! - $1234 conditional targeting (marked heroes) approximated as AllEnemies
//! - Two turns per round not modeled in Archetype
//! - PROT (0.8), MAGIC_PROT (0.2) not modeled in Archetype
//! - Position-based targeting (launch any, target ~1234/@1234) not modeled
//! - Random movement (inner_battle's "Random Move Range 1 2") simplified
//! - Disease Random chance mechanic simplified to status marker
//! - Revival mechanic (dead body returns at 20 HP after 2 turns) not modeled
//! - Pair bounce damage on marked heroes not modeled

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Black Tortoise A (Tortoise Body) base archetype — tier 1 boss stats from DDGC data.
///
/// HP 115, weapon damage derived from unexpectedly skill (4-8 avg 6),
/// speed 4, defense 0.30 (30% dodge).
/// Tank role: slow armored body with control and share-damage mechanics.
/// Crit 6% from unexpectedly skill.
/// PROT 0.8, MAGIC_PROT 0.2, Stun Resist 50%, Poison Resist 50%,
/// Bleed Resist 25%, Debuff Resist 40%, Move Resist 100% (immune),
/// Burn Resist 50%, Frozen Resist 100% (immune) — all not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Black Tortoise A"),
        side: CombatSide::Enemy,
        health: 115.0,
        max_health: 115.0,
        attack: 6.0,
        defense: 0.30,
        speed: 4.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.06,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Black Tortoise A Skills ────────────────────────────────────────────────

/// Call Roll — mark 2 random heroes with TortoiseMark and initialize BlackTortoise field.
///
/// DDGC reference: dmg 0-0, atk 0%, crit 0%,
/// launch any, target ~1234 (AoE all ranks),
/// effect "Tortoise Call The Roll Effect" (marks 2 heroes with TortoiseMark;
/// initializes BlackTortoise field; checks unbalance).
/// Always used first at round start (base_chance 1000.0).
/// Game-gap: hero-marking mechanic modeled as status marker only.
/// Game-gap: BlackTortoiseField initialization not modeled.
pub fn call_roll() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("tortoise_call_roll"),
        vec![EffectNode::apply_status("tortoise_mark", Some(1))],
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
        SkillId::new("tortoise_rain_spray"),
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

/// Ice Spike — single-target magic attack with frozen effect.
///
/// DDGC reference: dmg 1-2 (avg 1.5), atk 72%, crit 0%,
/// launch any, target 1234 (single target),
/// effect "Frozen 1" (dotFrozen 2 for 3 turns).
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn ice_spike() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("ice_spike"),
        vec![
            EffectNode::damage(1.5),
            EffectNode::apply_status("frozen", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Hunger Cold — AoE magic attack with stress.
///
/// DDGC reference: dmg 1-2 (avg 1.5), atk 72%, crit 0%,
/// launch any, target ~1234 (AoE all ranks),
/// effect "Stress 1" (+10 stress).
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn hunger_cold() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("hunger_cold"),
        vec![
            EffectNode::damage(1.5),
            EffectNode::apply_status("stress", Some(10)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Inner Battle — random movement and stress attack.
///
/// DDGC reference: dmg 0-0, atk 72%, crit 0%,
/// launch any, target 1234 (single target),
/// effects "Random Move Range 1 2" (random move 1-2 ranks) +
/// "Stress Range 5-8" (5-8 stress, avg ~6.5).
/// Game-gap: random movement simplified to status marker.
/// Game-gap: stress range 5-8 averaged to 6.
pub fn inner_battle() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("inner_battle"),
        vec![
            EffectNode::apply_status("random_move", Some(1)),
            EffectNode::apply_status("stress", Some(6)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Near Mountain River — share 80% damage with Snake body for 3 turns.
///
/// DDGC reference: dmg 0-0, atk 0%, crit 0%,
/// launch any, target @1234 (ally, not self — targets the Snake body),
/// effect "Share Damage 80" (links 80% incoming damage to Snake body for 3 turns).
/// Game-gap: cross-body damage sharing modeled as status marker only.
/// Game-gap: ally-targeting simplified to AllAllies.
pub fn near_mountain_river() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("near_mountain_river"),
        vec![EffectNode::apply_status("share_damage", Some(3))],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Hunger Cold 1 — targeted stress attack on marked heroes.
///
/// DDGC reference: dmg 3-6 (avg 4.5), atk 72%, crit 0%, magic damage,
/// launch any, target $1234 (heroes with TortoiseMark),
/// effect "Stress 2" (+15 stress).
/// Unbalance-phase skill: targets marked heroes specifically.
/// Game-gap: $1234 conditional targeting (marked heroes) approximated as AllEnemies.
pub fn hunger_cold_1() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("hunger_cold_1"),
        vec![
            EffectNode::damage(4.5),
            EffectNode::apply_status("stress", Some(15)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Unexpectedly — heavy pure damage attack (unbalance phase).
///
/// DDGC reference: dmg 4-8 (avg 6), atk 72%, crit 6%,
/// launch any, target 1234 (single target), no named effect.
/// Unbalance-phase skill: highest single-hit damage in Tortoise kit.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn unexpectedly() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("unexpectedly"),
        vec![EffectNode::damage(6.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 8 Black Tortoise A skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        call_roll(),
        rain_spray(),
        ice_spike(),
        hunger_cold(),
        inner_battle(),
        near_mountain_river(),
        hunger_cold_1(),
        unexpectedly(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_tortoise_a_archetype_is_enemy_beast_tank_boss() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Black Tortoise A");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 115.0);
        assert_eq!(arch.max_health, 115.0);
        assert_eq!(arch.speed, 4.0);
        assert_eq!(arch.defense, 0.30, "black_tortoise_A has 30% defense");
        assert_eq!(arch.crit_chance, 0.06, "black_tortoise_A has 6% crit");
        assert_eq!(arch.attack, 6.0, "black_tortoise_A attack from unexpectedly avg");
    }

    #[test]
    fn black_tortoise_a_call_roll_applies_tortoise_mark() {
        let skill = call_roll();
        assert_eq!(skill.id.0, "tortoise_call_roll");
        let has_mark = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("tortoise_mark")
        });
        assert!(has_mark, "call_roll must apply tortoise_mark status");
    }

    #[test]
    fn black_tortoise_a_rain_spray_deals_damage_damp_and_acc_down() {
        let skill = rain_spray();
        assert_eq!(skill.id.0, "tortoise_rain_spray");
        assert!(
            skill.effects.len() >= 3,
            "rain_spray should have damage + damp + acc_down"
        );
    }

    #[test]
    fn black_tortoise_a_ice_spike_deals_damage_and_frozen() {
        let skill = ice_spike();
        assert_eq!(skill.id.0, "ice_spike");
        assert!(
            skill.effects.len() >= 2,
            "ice_spike should have damage + frozen status"
        );
    }

    #[test]
    fn black_tortoise_a_hunger_cold_deals_damage_and_stress() {
        let skill = hunger_cold();
        assert_eq!(skill.id.0, "hunger_cold");
        assert!(
            skill.effects.len() >= 2,
            "hunger_cold should have damage + stress status"
        );
    }

    #[test]
    fn black_tortoise_a_inner_battle_applies_random_move_and_stress() {
        let skill = inner_battle();
        assert_eq!(skill.id.0, "inner_battle");
        let has_move = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("random_move")
        });
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_move, "inner_battle must apply random_move status");
        assert!(has_stress, "inner_battle must apply stress status");
    }

    #[test]
    fn black_tortoise_a_near_mountain_river_applies_share_damage() {
        let skill = near_mountain_river();
        assert_eq!(skill.id.0, "near_mountain_river");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllAllies",
            "near_mountain_river targets allies (Snake body)"
        );
        let has_share = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("share_damage")
        });
        assert!(has_share, "near_mountain_river must apply share_damage status");
    }

    #[test]
    fn black_tortoise_a_hunger_cold_1_deals_damage_and_stress() {
        let skill = hunger_cold_1();
        assert_eq!(skill.id.0, "hunger_cold_1");
        assert!(
            skill.effects.len() >= 2,
            "hunger_cold_1 should have damage + stress"
        );
    }

    #[test]
    fn black_tortoise_a_unexpectedly_deals_heavy_damage() {
        let skill = unexpectedly();
        assert_eq!(skill.id.0, "unexpectedly");
        assert!(
            !skill.effects.is_empty(),
            "unexpectedly should have at least damage"
        );
    }

    #[test]
    fn black_tortoise_a_skill_pack_has_eight_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 8);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"tortoise_call_roll"), "missing tortoise_call_roll");
        assert!(ids.contains(&"tortoise_rain_spray"), "missing tortoise_rain_spray");
        assert!(ids.contains(&"ice_spike"), "missing ice_spike");
        assert!(ids.contains(&"hunger_cold"), "missing hunger_cold");
        assert!(ids.contains(&"inner_battle"), "missing inner_battle");
        assert!(ids.contains(&"near_mountain_river"), "missing near_mountain_river");
        assert!(ids.contains(&"hunger_cold_1"), "missing hunger_cold_1");
        assert!(ids.contains(&"unexpectedly"), "missing unexpectedly");
    }

    #[test]
    fn black_tortoise_a_dual_body_plus_share_damage_identity() {
        // The core identity of black_tortoise_A is a tank body that:
        // 1. Marks heroes with TortoiseMark (call_roll)
        // 2. Shares damage with the Snake body (near_mountain_river)
        // 3. Controls heroes with damp/frozen debuffs (rain_spray, ice_spike)
        // 4. Has unbalance-phase heavy damage (unexpectedly)
        let pack = skill_pack();

        // Must have TortoiseMark mechanic
        let has_mark = pack.iter().any(|s| {
            s.id.0 == "tortoise_call_roll"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("tortoise_mark")
                })
        });

        // Must have share damage mechanic
        let has_share = pack.iter().any(|s| {
            s.id.0 == "near_mountain_river"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("share_damage")
                })
        });

        // Must have damp/frozen control
        let has_frozen = pack.iter().any(|s| {
            s.id.0 == "ice_spike"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("frozen")
                })
        });

        // Must have unbalance-phase heavy damage
        let has_heavy = pack.iter().any(|s| {
            s.id.0 == "unexpectedly"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::Damage)
                })
        });

        assert!(has_mark, "black_tortoise_A must have TortoiseMark mechanic");
        assert!(has_share, "black_tortoise_A must have share_damage mechanic");
        assert!(has_frozen, "black_tortoise_A must have frozen control");
        assert!(has_heavy, "black_tortoise_A must have unexpectedly heavy damage");
    }
}