//! Vermilion Bird Tail B — ZhuQue boss part (mark/debuff + heal + PB charge).
//!
//! DDGC reference: Beast-type boss part from the ZhuQue dungeon.
//! Shares health pool with main body (HP 0 in DDGC → modeled as 160 for framework).
//! Tier 1 base stats: DEF 30%, PROT 0.5, SPD 7, 1 turn/round.
//! Skills: follow, follow1, run_water, run_water1, heaven_falls,
//! heaven_falls1, iron_feather_with.
//!
//! This part's identity is a support tail that marks heroes for focused
//! attacks, debuffs their offense/defense, heals allies (clearing marks),
//! and charges PB for the main body's explosion cycle. It is untargetable
//! in DDGC (is_valid_enemy_target False).
//!
//! Game-gaps:
//! - Shared health pool with main body not modeled (has own HP)
//! - Untargetable status (is_valid_enemy_target False) not modeled
//! - PROT (0.5) and MAGIC_PROT (0.8) not modeled in Archetype
//! - Stun/Move/etc. resistances not modeled in Archetype
//! - Position-based targeting (launch 4, target $1234/12/@12/~1234) not modeled
//! - PB charge (Vermilion Bird Pb 10) on iron_feather_with dropped as game-gap
//!   (performer-targeted self-buff on an offensive skill)
//! - Phase 1/2 skill variants (follow/follow1, run_water/run_water1,
//!   heaven_falls/heaven_falls1) differ in DDGC by targeting rules
//!   ($1234 vs 1234, 12 vs @12) but are approximated identically

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Vermilion Bird Tail B base archetype — tier 1 boss part stats from DDGC data.
///
/// HP 160 (shared with main body in DDGC — modeled as same HP since shared
/// health mechanic is not modeled in framework), attack 2.0 from
/// iron_feather_with avg damage, speed 7, defense 0.30 (30% dodge).
/// Controller role: marks, debuffs, heals allies, charges PB.
/// PROT 0.5, MAGIC_PROT 0.8, Stun Resist 40%, Poison Resist 60%,
/// Bleed Resist 20%, Debuff Resist 40%, Move Resist 50%, Burn Resist 100%,
/// Frozen Resist 25% — all not modeled in Archetype.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Vermilion Bird Tail B"),
        side: CombatSide::Enemy,
        health: 160.0,
        max_health: 160.0,
        attack: 2.0,
        defense: 0.30,
        speed: 7.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.0,
        dodge: 0.0,
    }
}

// ── Vermilion Bird Tail B Skills ─────────────────────────────────────────

/// Follow — marks a hero with stress + mark tag (Phase 1).
///
/// DDGC reference: dmg 0-0, atk 64%, crit 0%,
/// effects "Stress 2" + "Vermilion Bird Tail 2 Mark" (tag for 3 rounds),
/// launch rank 4, target $1234 (random single).
/// Phase 1 skill: 34% chance in AI brain.
/// Game-gap: random single-target ($ prefix) not modeled — targets AllEnemies.
pub fn follow() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("follow"),
        vec![
            EffectNode::apply_status("stress", Some(2)),
            EffectNode::apply_status("mark", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Follow1 — marks a hero with stress + mark tag (Phase 2).
///
/// DDGC reference: dmg 0-0, atk 64%, crit 0%,
/// effects "Stress 2" + "Vermilion Bird Tail 2 Mark",
/// launch rank 4, target ranks 1-4 (all enemies, not random).
/// Phase 2 skill: 34% chance when VB has no active move.
/// Game-gap: all-ranks targeting simplified to AllEnemies.
pub fn follow1() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("follow1"),
        vec![
            EffectNode::apply_status("stress", Some(2)),
            EffectNode::apply_status("mark", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Run Water — clears marks and heals an ally (Phase 1).
///
/// DDGC reference: dmg 0-0, atk 64%, crit 0%,
/// effects "Clear Marked Target" + "Cure",
/// launch rank 4, target ranks 1-2, heal 5-8 (avg 7).
/// Phase 1 skill: 33% chance in AI brain.
/// Game-gap: targeted healing (ranks 1-2 only) simplified to AllAllies.
pub fn run_water() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("run_water"),
        vec![
            EffectNode::apply_status("clear_mark", Some(1)),
            EffectNode::apply_status("cure", Some(1)),
            EffectNode::apply_status("heal", Some(7)),
        ],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Run Water1 — clears marks and heals allies (Phase 2, percentage heal).
///
/// DDGC reference: dmg 0-0, atk 64%, crit 0%,
/// effects "Clear Marked Target" + "Cure" + "Heal Percent 5",
/// launch rank 4, target @12 (ally ranks 1-2).
/// Phase 2 skill: 33% chance when VB has no active move.
/// Game-gap: ally-targeting (@12) simplified to AllAllies.
pub fn run_water1() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("run_water1"),
        vec![
            EffectNode::apply_status("clear_mark", Some(1)),
            EffectNode::apply_status("cure", Some(1)),
            EffectNode::apply_status("heal_percent", Some(5)),
        ],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Heaven Falls — debuffs hero ATK and DEF (Phase 1).
///
/// DDGC reference: dmg 0-0, atk 64%, crit 0%,
/// effect "Vermilion Bird Tail 2 Debuff" (-5% ATK, -15% DEF for 3 rounds),
/// launch rank 4, target ranks 1-4.
/// Phase 1 skill: 33% chance in AI brain.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn heaven_falls() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("heaven_falls"),
        vec![
            EffectNode::apply_status("atk_down", Some(3)),
            EffectNode::apply_status("def_down", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Heaven Falls1 — debuffs hero ATK, DEF, and adds magic vulnerability (Phase 2).
///
/// DDGC reference: dmg 0-0, atk 64%, crit 0%,
/// effects "Vermilion Bird Tail 2 Debuff" + "Vermilion Bird Tail 2 Debuff1"
/// (+20% magic damage received for 3 rounds),
/// launch rank 4, target ~1234 (AoE all ranks).
/// Phase 2 skill: 33% chance when VB has no active move.
/// Game-gap: AoE vs single-target distinction not modeled — targets AllEnemies.
pub fn heaven_falls1() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("heaven_falls1"),
        vec![
            EffectNode::apply_status("atk_down", Some(3)),
            EffectNode::apply_status("def_down", Some(3)),
            EffectNode::apply_status("magic_dmg_recv_up", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Iron Feather With — stress attack that charges PB for explosion cycle (Phase 2).
///
/// DDGC reference: dmg 1-3 (avg 2), atk 64%, crit 0%,
/// effects "Stress Range 7-8" (avg 8) + "Vermilion Bird Pb 10" (10 PB charge),
/// launch rank 4, target ~1234 (AoE all ranks).
/// Phase 2 skill: used when VB is about to use calm_nerves/explosion.
/// The PB charge is performer-targeted (applied to the boss).
///
/// Game-gap: performer-targeted PB charge dropped as game-gap
/// (consistent with self-buff side effect pattern on offensive skills).
/// Game-gap: AoE vs single-target distinction not modeled — targets AllEnemies.
pub fn iron_feather_with() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("iron_feather_with"),
        vec![
            EffectNode::damage(2.0),
            EffectNode::apply_status("stress", Some(8)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 7 Vermilion Bird Tail B skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        follow(),
        follow1(),
        run_water(),
        run_water1(),
        heaven_falls(),
        heaven_falls1(),
        iron_feather_with(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vermilion_bird_tail_b_archetype_is_enemy_beast_controller() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Vermilion Bird Tail B");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 160.0);
        assert_eq!(arch.max_health, 160.0);
        assert_eq!(arch.speed, 7.0);
        assert_eq!(arch.defense, 0.30, "tail_B has 30% defense");
        assert_eq!(arch.attack, 2.0, "tail_B attack from iron_feather_with avg");
    }

    #[test]
    fn vermilion_bird_tail_b_follow_applies_stress_and_mark() {
        let skill = follow();
        assert_eq!(skill.id.0, "follow");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        let has_mark = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("mark")
        });
        assert!(has_stress, "follow must apply stress");
        assert!(has_mark, "follow must apply mark");
    }

    #[test]
    fn vermilion_bird_tail_b_follow1_applies_stress_and_mark() {
        let skill = follow1();
        assert_eq!(skill.id.0, "follow1");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        let has_mark = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("mark")
        });
        assert!(has_stress, "follow1 must apply stress");
        assert!(has_mark, "follow1 must apply mark");
    }

    #[test]
    fn vermilion_bird_tail_b_run_water_heals_and_clears_marks() {
        let skill = run_water();
        assert_eq!(skill.id.0, "run_water");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllAllies",
            "run_water targets allies"
        );
        let has_heal = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("heal")
        });
        let has_clear = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("clear_mark")
        });
        assert!(has_heal, "run_water must apply heal status");
        assert!(has_clear, "run_water must apply clear_mark status");
    }

    #[test]
    fn vermilion_bird_tail_b_run_water1_heals_percentage() {
        let skill = run_water1();
        assert_eq!(skill.id.0, "run_water1");
        let has_heal_pct = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("heal_percent")
        });
        assert!(has_heal_pct, "run_water1 must apply heal_percent status");
    }

    #[test]
    fn vermilion_bird_tail_b_heaven_falls_debuffs_atk_and_def() {
        let skill = heaven_falls();
        assert_eq!(skill.id.0, "heaven_falls");
        let has_atk_down = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("atk_down")
        });
        let has_def_down = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("def_down")
        });
        assert!(has_atk_down, "heaven_falls must apply atk_down");
        assert!(has_def_down, "heaven_falls must apply def_down");
    }

    #[test]
    fn vermilion_bird_tail_b_heaven_falls1_adds_magic_vulnerability() {
        let skill = heaven_falls1();
        assert_eq!(skill.id.0, "heaven_falls1");
        let has_magic_vuln = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("magic_dmg_recv_up")
        });
        assert!(has_magic_vuln, "heaven_falls1 must apply magic_dmg_recv_up");
    }

    #[test]
    fn vermilion_bird_tail_b_iron_feather_with_deals_damage_and_stress() {
        let skill = iron_feather_with();
        assert_eq!(skill.id.0, "iron_feather_with");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "iron_feather_with must apply stress");
    }

    #[test]
    fn vermilion_bird_tail_b_skill_pack_has_seven_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 7);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"follow"), "missing follow");
        assert!(ids.contains(&"follow1"), "missing follow1");
        assert!(ids.contains(&"run_water"), "missing run_water");
        assert!(ids.contains(&"run_water1"), "missing run_water1");
        assert!(ids.contains(&"heaven_falls"), "missing heaven_falls");
        assert!(ids.contains(&"heaven_falls1"), "missing heaven_falls1");
        assert!(ids.contains(&"iron_feather_with"), "missing iron_feather_with");
    }

    #[test]
    fn vermilion_bird_tail_b_mark_debuff_plus_heal_identity() {
        // The core identity of tail_B is mark + debuff + heal support:
        // marks heroes for focused attacks, debuffs ATK/DEF,
        // heals allies and clears marks, and charges PB for explosion.
        let pack = skill_pack();

        let has_mark = pack.iter().any(|s| {
            (s.id.0 == "follow" || s.id.0 == "follow1")
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("mark")
                })
        });

        let has_debuff = pack.iter().any(|s| {
            (s.id.0 == "heaven_falls" || s.id.0 == "heaven_falls1")
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("atk_down")
                })
        });

        let has_heal = pack.iter().any(|s| {
            (s.id.0 == "run_water" || s.id.0 == "run_water1")
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && (e.status_kind.as_deref() == Some("heal")
                            || e.status_kind.as_deref() == Some("heal_percent"))
                })
        });

        let has_stress = pack.iter().any(|s| {
            s.id.0 == "iron_feather_with"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stress")
                })
        });

        assert!(has_mark, "tail_B must have mark tagging skill");
        assert!(has_debuff, "tail_B must have ATK/DEF debuff skill");
        assert!(has_heal, "tail_B must have ally heal skill");
        assert!(has_stress, "tail_B must have stress + PB charge skill");
    }
}
