//! Frostvein Clam — XuanWu boss (riposte + pearlkin-summon + frozen-pressure).
//!
//! DDGC reference: Eldritch-type boss from the XuanWu dungeon (霜纹巨蚌).
//! Tier 1 base stats: HP 150, DEF 15%, PROT 0.7, SPD 5, 1 turn/round.
//! Skills: glacial_torrent, abyssal_glare, nacreous_homunculus, prismatic_clench,
//! riposte1 (counter-attack triggered by prismatic_clench).
//!
//! The Frostvein Clam is a summon-riposte boss that summons pearlkin minions
//! (opalescent + flawed) and pressures heroes with frozen DoT and stress.
//! Its AI brain prioritizes nacreous_homunculus (weight 1000 when space available)
//! over prismatic_clench (1.35) and glacial_torrent/abyssal_glare (0.3 each).
//!
//! Game-gaps:
//! - Nacreous Homunculus summon modeled as status marker only — actual spawn
//!   of pearlkin_opalescent + pearlkin_flawed not modeled
//! - Prismatic Clench riposte modeled as status marker only — auto-counter
//!   attack mechanic not modeled
//! - Prismatic Clench +50% Protection buff modeled as "prot_buff" status marker
//! - Glacial Torrent "Frozen 1" (Frozen DoT 2/turn for 3 turns) modeled as
//!   apply_status("frozen", Some(3)) — the DoT damage is a game-gap
//! - Abyssal Glare Stress Range 7-10 averaged to 8
//! - Position-based targeting (launch 34, target 1234/~1234) approximated
//!   as AllEnemies or SelfOnly
//! - ~1234 AoE targeting approximated as AllEnemies
//! - PROT (0.7), MAGIC_PROT (0.5), Stun Resist 25%, Poison Resist 50%,
//!   Bleed Resist 75%, Debuff Resist 40%, Move Resist 100% (immune),
//!   Burn Resist 25%, Frozen Resist 100% (immune) not modeled
//! - 1 turn per round — standard, no special handling needed
//! - Size 2 (occupies 2 slots) not modeled in Archetype

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Frostvein Clam base archetype — tier 1 boss stats from DDGC data.
///
/// HP 150, weapon damage derived from glacial_torrent/abyssal_glare skill
/// (magic_dmg 4–6 avg 5.0), speed 5, defense 0.15 (15% dodge).
/// Summoner role: summons pearlkin minions and pressures with frozen DoT,
/// stress, and riposte shell defense.
/// Crit 5% from glacial_torrent/abyssal_glare.
/// PROT 0.7, MAGIC_PROT 0.5, Stun Resist 25%, Poison Resist 50%,
/// Bleed Resist 75%, Debuff Resist 40%, Move Resist 100% (immune),
/// Burn Resist 25%, Frozen Resist 100% (immune) — all not modeled.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Frostvein Clam"),
        side: CombatSide::Enemy,
        health: 150.0,
        max_health: 150.0,
        attack: 5.0,
        defense: 0.15,
        speed: 5.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.05,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Frostvein Clam Skills ────────────────────────────────────────────────────

/// Glacial Torrent — ranged frozen DoT + magic damage.
///
/// DDGC reference: magic_dmg 4–6, atk 85%, crit 5%,
/// launch ranks 3,4, target 1234 (any rank),
/// effect "Frozen 1" (Frozen DoT 2/turn for 3 turns, 100% chance).
/// Game-gap: Frozen DoT damage (2/turn for 3 turns) modeled as
/// apply_status("frozen", Some(3)) — duration captured, per-tick damage not modeled.
/// Game-gap: 1234 targeting approximated as AllEnemies.
pub fn glacial_torrent() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("glacial_torrent"),
        vec![
            EffectNode::damage(5.0),
            EffectNode::apply_status("frozen", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Abyssal Glare — ranged stress + magic damage.
///
/// DDGC reference: magic_dmg 4–6, atk 85%, crit 5%,
/// launch ranks 3,4, target ~1234 (AoE all ranks),
/// effect "Stress Range 7-10".
/// Game-gap: ~1234 AoE targeting approximated as AllEnemies.
/// Game-gap: Stress Range 7-10 averaged to 8.
pub fn abyssal_glare() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("abyssal_glare"),
        vec![
            EffectNode::damage(5.0),
            EffectNode::apply_status("stress", Some(8)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Nacreous Homunculus — summon pearlkin minions.
///
/// DDGC reference: magic_dmg 0–0, atk 100%, crit 0%,
/// launch ranks 1,2, target self (empty .target),
/// effect "fc Summon" (summons up to 1x pearlkin_opalescent AND 1x pearlkin_flawed,
/// summon_chances 1.0 each, summon_limits 1 each).
/// Game-gap: Summon mechanic modeled as "summon_pearlkin" status marker only.
/// Game-gap: Pearlkin selection (opalescent + flawed) and limits not modeled.
/// Game-gap: AI brain prioritizes this skill with weight 1000 when 2 slots
/// are empty — not modeled.
pub fn nacreous_homunculus() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("nacreous_homunculus"),
        vec![EffectNode::apply_status("summon_pearlkin", Some(2))],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// Prismatic Clench — self-riposte + protection buff.
///
/// DDGC reference: dmg 0–0, atk 100%, crit 0%,
/// launch ranks 3,4, target self (empty .target = performer),
/// effect "fc Riposte" (1-turn riposte + +50% Protection for 1 turn, 100% chance).
/// Game-gap: Riposte auto-counter mechanic modeled as "riposte" status marker only.
/// Game-gap: +50% Protection modeled as "prot_buff" status marker only.
pub fn prismatic_clench() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("prismatic_clench"),
        vec![
            EffectNode::apply_status("riposte", Some(1)),
            EffectNode::apply_status("prot_buff", Some(50)),
        ],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// Riposte1 — counter-attack triggered by prismatic_clench.
///
/// DDGC reference: magic_dmg 4–6, atk 100%, crit 5%,
/// launch ranks 1,2,3,4, target 1234.
/// This is the auto-counter skill that fires when an enemy attacks
/// the Frostvein Clam while it has the riposte status.
/// Game-gap: Auto-counter trigger mechanic not modeled — skill is registered
/// in the pack but the framework has no riposte resolution system.
pub fn riposte1() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("clam_riposte"),
        vec![EffectNode::damage(5.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 5 Frostvein Clam skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        glacial_torrent(),
        abyssal_glare(),
        nacreous_homunculus(),
        prismatic_clench(),
        riposte1(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frostvein_clam_archetype_is_enemy_eldritch_summoner() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Frostvein Clam");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 150.0);
        assert_eq!(arch.max_health, 150.0);
        assert_eq!(arch.speed, 5.0, "frostvein_clam has SPD 5");
        assert_eq!(arch.defense, 0.15, "frostvein_clam has 15% defense");
        assert_eq!(arch.attack, 5.0, "attack from glacial_torrent/abyssal_glare avg 4-6");
        assert_eq!(arch.crit_chance, 0.05, "crit 5% from skills");
    }

    #[test]
    fn frostvein_clam_glacial_torrent_applies_damage_and_frozen() {
        let skill = glacial_torrent();
        assert_eq!(skill.id.0, "glacial_torrent");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "glacial_torrent must deal damage");
        let has_frozen = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("frozen")
        });
        assert!(has_frozen, "glacial_torrent must apply frozen status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllEnemies",
            "glacial_torrent targets all enemies (target 1234)"
        );
    }

    #[test]
    fn frostvein_clam_abyssal_glare_applies_damage_and_stress() {
        let skill = abyssal_glare();
        assert_eq!(skill.id.0, "abyssal_glare");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "abyssal_glare must deal damage");
        let has_stress = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("stress")
        });
        assert!(has_stress, "abyssal_glare must apply stress status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "AllEnemies",
            "abyssal_glare targets all enemies (~1234 AoE)"
        );
    }

    #[test]
    fn frostvein_clam_nacreous_homunculus_applies_summon_marker() {
        let skill = nacreous_homunculus();
        assert_eq!(skill.id.0, "nacreous_homunculus");
        let has_summon = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("summon_pearlkin")
        });
        assert!(has_summon, "nacreous_homunculus must apply summon_pearlkin status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "nacreous_homunculus targets self (empty .target in DDGC)"
        );
    }

    #[test]
    fn frostvein_clam_prismatic_clench_applies_riposte_and_prot_buff() {
        let skill = prismatic_clench();
        assert_eq!(skill.id.0, "prismatic_clench");
        let has_riposte = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("riposte")
        });
        assert!(has_riposte, "prismatic_clench must apply riposte status");
        let has_prot_buff = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("prot_buff")
        });
        assert!(has_prot_buff, "prismatic_clench must apply prot_buff status");
        assert_eq!(
            format!("{:?}", skill.target_selector),
            "SelfOnly",
            "prismatic_clench targets self (performer in DDGC)"
        );
    }

    #[test]
    fn frostvein_clam_skill_pack_has_five_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 5);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"glacial_torrent"), "missing glacial_torrent");
        assert!(ids.contains(&"abyssal_glare"), "missing abyssal_glare");
        assert!(ids.contains(&"nacreous_homunculus"), "missing nacreous_homunculus");
        assert!(ids.contains(&"prismatic_clench"), "missing prismatic_clench");
        assert!(ids.contains(&"clam_riposte"), "missing clam_riposte");
    }

    #[test]
    fn frostvein_clam_riposte_plus_summon_identity() {
        // The core identity of frostvein_clam is a summon-riposte boss that
        // summons pearlkin minions while defending with riposte shell and
        // pressuring heroes with frozen DoT and stress.
        let pack = skill_pack();

        let has_summon = pack.iter().any(|s| {
            s.id.0 == "nacreous_homunculus"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("summon_pearlkin")
                })
        });

        let has_riposte = pack.iter().any(|s| {
            s.id.0 == "prismatic_clench"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("riposte")
                })
        });

        let has_frozen = pack.iter().any(|s| {
            s.id.0 == "glacial_torrent"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("frozen")
                })
        });

        let has_stress = pack.iter().any(|s| {
            s.id.0 == "abyssal_glare"
                && s.effects.iter().any(|e| {
                    matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                        && e.status_kind.as_deref() == Some("stress")
                })
        });

        assert!(has_summon, "frostvein_clam must have summon_pearlkin skill");
        assert!(has_riposte, "frostvein_clam must have riposte skill");
        assert!(has_frozen, "frostvein_clam must have frozen DoT skill");
        assert!(has_stress, "frostvein_clam must have stress skill");
    }
}
