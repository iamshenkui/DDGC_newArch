//! Pearlkin Opalescent — XuanWu boss part (debuff + damage).
//!
//! DDGC reference: Eldritch-type boss minion from the XuanWu dungeon (珍珠人•瑜).
//! Tier 1 base stats: HP 30, DEF 15%, PROT 0.5, SPD 5.
//! Skills: po_debuff (dodge debuff + damage), po_damage (magic damage).
//!
//! The Opalescent variant debuffs hero dodge while dealing moderate damage.
//! It is summoned by the Frostvein Clam via nacreous_homunculus and also
//! pre-placed in the boss encounter pack.
//!
//! Game-gaps:
//! - po_debuff "Target Dodge -1" (defense_rating_add -5%) modeled as
//!   apply_status("dodge_debuff", Some(5)) — specific dodge rating change not modeled
//! - po_debuff has no duration in DDGC — modeled as permanent (Some(0) duration)
//! - $1234 conditional targeting (targets heroes with specific marks) approximated
//!   as AllEnemies — conditional filter not modeled
//! - ~1234 AoE targeting approximated as AllEnemies
//! - PROT (0.5), MAGIC_PROT (0.7), Stun Resist 25%, Poison Resist 100% (immune),
//!   Bleed Resist 100% (immune), Debuff Resist 40%, Move Resist 50%,
//!   Burn Resist 25%, Frozen Resist 100% (immune) not modeled

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Pearlkin Opalescent base archetype — boss minion stats from DDGC data.
///
/// HP 30, attack derived from po_damage skill (magic_dmg 2–4 avg 3.0),
/// speed 5, defense 0.15 (15% dodge).
/// Support role: debuffs hero dodge while dealing moderate damage.
/// Crit 5% from skills.
/// PROT 0.5, MAGIC_PROT 0.7 — not modeled.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Pearlkin Opalescent"),
        side: CombatSide::Enemy,
        health: 30.0,
        max_health: 30.0,
        attack: 3.0,
        defense: 0.15,
        speed: 5.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.05,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Pearlkin Opalescent Skills ──────────────────────────────────────────────

/// PO Debuff — ranged dodge debuff + low physical damage.
///
/// DDGC reference: dmg 2–4, atk 85%, crit 5%,
/// launch ranks 1,2, target $1234 (conditional mark targeting),
/// effect "Target Dodge -1" (defense_rating_add -5%, no duration).
/// Game-gap: $1234 conditional targeting approximated as AllEnemies.
/// Game-gap: Dodge debuff modeled as "dodge_debuff" status marker —
/// specific dodge rating change not modeled.
/// Game-gap: No duration in DDGC modeled as Some(0) (permanent marker).
pub fn po_debuff() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("po_debuff"),
        vec![
            EffectNode::damage(3.0),
            EffectNode::apply_status("dodge_debuff", Some(5)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// PO Damage — ranged magic damage.
///
/// DDGC reference: magic_dmg 2–4, atk 85%, crit 5%,
/// launch ranks 1,2, target ~1234 (AoE all ranks),
/// no extra effect.
/// Game-gap: ~1234 AoE targeting approximated as AllEnemies.
pub fn po_damage() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("po_damage"),
        vec![EffectNode::damage(3.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// All 2 Pearlkin Opalescent skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![po_debuff(), po_damage()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pearlkin_opalescent_archetype_is_enemy_eldritch_support() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Pearlkin Opalescent");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 30.0);
        assert_eq!(arch.max_health, 30.0);
        assert_eq!(arch.speed, 5.0);
        assert_eq!(arch.defense, 0.15, "pearlkin_opalescent has 15% defense");
        assert_eq!(arch.attack, 3.0, "attack from po_damage avg 2-4");
        assert_eq!(arch.crit_chance, 0.05, "crit 5% from skills");
    }

    #[test]
    fn pearlkin_opalescent_po_debuff_applies_damage_and_dodge_debuff() {
        let skill = po_debuff();
        assert_eq!(skill.id.0, "po_debuff");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "po_debuff must deal damage");
        let has_debuff = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
                && e.status_kind.as_deref() == Some("dodge_debuff")
        });
        assert!(has_debuff, "po_debuff must apply dodge_debuff status");
    }

    #[test]
    fn pearlkin_opalescent_po_damage_applies_damage() {
        let skill = po_damage();
        assert_eq!(skill.id.0, "po_damage");
        let has_damage = skill.effects.iter().any(|e| {
            matches!(e.kind, framework_combat::effects::EffectKind::Damage)
        });
        assert!(has_damage, "po_damage must deal damage");
    }

    #[test]
    fn pearlkin_opalescent_skill_pack_has_two_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 2);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"po_debuff"), "missing po_debuff");
        assert!(ids.contains(&"po_damage"), "missing po_damage");
    }
}
