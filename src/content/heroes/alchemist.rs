//! Alchemist hero — support/healer class family base variant.
//!
//! DDGC reference: back-rank healer with burn DoT and party-wide sustain.
//! Most skills deal 0% or negative damage (healing, stress heal, buffs).
//! Base weapon damage at level 0: 17–34, averaged to 26.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Alchemist base archetype — level 0 stats from DDGC data.
///
/// HP 139, weapon damage 17–34 (avg 26), speed 5, dodge 0%, crit 2%.
/// Lowest weapon damage of the 5 classes — primarily a support role.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Alchemist"),
        side: CombatSide::Ally,
        health: 139.0,
        max_health: 139.0,
        attack: 26.0,
        defense: 0.0,
        speed: 5.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.02,
        dodge: 0.00,
    }
}

// ── Alchemist Skills ───────────────────────────────────────────────────────

/// Heal Multi — AoE party heal.
///
/// DDGC reference: heals 9–11 HP to all allies, averaged to 10.
/// Launch ranks 3–4, targets all allies.
pub fn heal_multi() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("heal_multi"),
        vec![EffectNode::heal(10.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Heal Single — large single-target heal.
///
/// DDGC reference: heals 24–32 HP to a single ally, averaged to 28.
/// Game-gap: single-target selection not supported — targets all allies.
pub fn heal_single() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("heal_single"),
        vec![EffectNode::heal(28.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Miss Single — ally dodge + speed buff.
///
/// DDGC reference: grants +10% dodge and +10% speed to a single ally for 1 round.
/// Game-gap: buff effects not modeled — approximated as minor heal placeholder.
pub fn miss_single() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("miss_single"),
        vec![EffectNode::heal(1.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Stress Multi — party-wide stress heal.
///
/// DDGC reference: heals 5 stress to all allies.
/// Game-gap: stress heal approximated as minor HP heal.
pub fn stress_multi() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("stress_multi"),
        vec![EffectNode::heal(5.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// Burn Skill — damage + burn DoT.
///
/// DDGC reference: full weapon damage (0% mod) + burn DoT 12/round for 3 rounds.
/// Damage averaged from 17–34 → 26.
pub fn burn_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("burn_skill"),
        vec![
            EffectNode::damage(26.0),
            EffectNode::apply_status("burn", Some(3)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Push Skill — reduced damage + push.
///
/// DDGC reference: -67% damage (avg 9) + push enemy back 2 positions.
/// Game-gap: push/movement effects not modeled — damage only.
pub fn push_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("push_skill"),
        vec![EffectNode::damage(9.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Push Self — self-repositioning forward.
///
/// DDGC reference: moves Alchemist 2 ranks forward, no damage.
/// Game-gap: movement not modeled — approximated as minimal self-heal placeholder.
pub fn push_self() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("push_self"),
        vec![EffectNode::heal(1.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

/// All 7 Alchemist base skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![
        heal_multi(),
        heal_single(),
        miss_single(),
        stress_multi(),
        burn_skill(),
        push_skill(),
        push_self(),
    ]
}
