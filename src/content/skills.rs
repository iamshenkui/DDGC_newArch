//! DDGC combat skills — migrated from DDGC skill definitions.
//!
//! DDGC skills use damage ranges (min/max) which are averaged for this
//! migration slice (see MIGRATION_BLOCKERS.md B-006). Skill usage limits
//! (LimitPerTurn/LimitPerBattle) are tracked in game-layer state (B-005).
//! Multi-hit is achieved by repeating `EffectNode::damage()` entries (B-009).

use framework_combat::effects::{EffectCondition, EffectNode};
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::{SlotRange, TargetSelector};

// ── Crusader Skills ───────────────────────────────────────────────────────

/// Crusading Strike — basic melee attack against front-rank enemies.
///
/// DDGC reference: hits ranks 1–2, moderate damage, no cooldown.
/// Damage averaged from DDGC range (8–15) → 12.
pub fn crusading_strike() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("crusading_strike"),
        vec![EffectNode::damage(12.0)],
        TargetSelector::ByPosition(SlotRange::new(0, 1)), // front 2 ranks
        1,
        None,
    )
}

/// Holy Lance — ranged holy strike that can hit any rank.
///
/// DDGC reference: hits ranks 1–4, damage + self-heal, 2-turn cooldown.
/// Damage averaged from DDGC range (6–12) → 9.
pub fn holy_lance() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("holy_lance"),
        vec![
            EffectNode::damage(9.0),
            EffectNode::heal(3.0),
        ],
        TargetSelector::AllEnemies,
        1,
        Some(2),
    )
}

// ── Vestal Skills ─────────────────────────────────────────────────────────

/// Divine Grace — single-target heal for an ally.
///
/// DDGC reference: heals 8–12 HP, averaged to 10.
pub fn divine_grace() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("divine_grace"),
        vec![EffectNode::heal(10.0)],
        TargetSelector::AllAllies,
        1,
        None,
    )
}

// ── Bone Soldier Skills ───────────────────────────────────────────────────

/// Rend — slash attack that inflicts bleed.
///
/// DDGC reference: hits rank 1–2, low damage + bleed status.
/// Damage averaged from DDGC range (4–8) → 6.
pub fn rend() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("rend"),
        vec![
            EffectNode::damage(6.0),
            EffectNode::apply_status("bleed", Some(3)),
        ],
        TargetSelector::ByPosition(SlotRange::new(0, 1)),
        1,
        None,
    )
}

// ── Necromancer Skills ────────────────────────────────────────────────────

/// Skull Bash — heavy single-target attack with stun chance.
///
/// DDGC reference: hits rank 1, high damage, 60% stun chance.
/// Damage averaged from DDGC range (10–18) → 14.
/// Stun approximated via EffectCondition::Probability.
pub fn skull_bash() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("skull_bash"),
        vec![
            EffectNode::damage(14.0),
            EffectNode::apply_status("stun", Some(1))
                .with_condition(EffectCondition::Probability(0.60)),
        ],
        TargetSelector::ByPosition(SlotRange::new(0, 0)), // rank 1 only
        1,
        Some(3),
    )
}

/// Grave Bash — multi-hit attack that strikes twice.
///
/// DDGC reference: 2 hits, moderate damage each, hits any rank.
/// Damage per hit averaged from DDGC range (3–7) → 5.
/// Multi-hit implemented as two separate EffectNode::damage entries (B-009).
pub fn grave_bash() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("grave_bash"),
        vec![
            EffectNode::damage(5.0),
            EffectNode::damage(5.0),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrated_skill_pack_validates() {
        let skills = [
            crusading_strike(),
            holy_lance(),
            divine_grace(),
            rend(),
            skull_bash(),
            grave_bash(),
        ];

        for skill in &skills {
            assert!(
                skill.validate().is_ok(),
                "Skill '{}' failed validation",
                skill.id.0
            );
        }
    }

    #[test]
    fn crusading_strike_targets_front_ranks() {
        let skill = crusading_strike();
        assert!(matches!(skill.target_selector, TargetSelector::ByPosition(_)));
        assert_eq!(skill.action_cost, 1);
        assert!(skill.cooldown.is_none());
    }

    #[test]
    fn holy_lance_has_cooldown() {
        let skill = holy_lance();
        assert_eq!(skill.cooldown, Some(2));
        assert_eq!(skill.effects.len(), 2); // damage + heal
    }

    #[test]
    fn rend_applies_bleed() {
        let skill = rend();
        assert_eq!(skill.effects.len(), 2); // damage + apply_status
    }

    #[test]
    fn skull_bash_has_stun_with_probability() {
        let skill = skull_bash();
        assert_eq!(skill.effects.len(), 2); // damage + conditional stun
        assert_eq!(skill.cooldown, Some(3));
    }

    #[test]
    fn grave_bash_is_multi_hit() {
        let skill = grave_bash();
        assert_eq!(skill.effects.len(), 2); // 2 damage nodes = multi-hit
    }

    #[test]
    fn divine_grace_heals_allies() {
        let skill = divine_grace();
        assert!(matches!(skill.target_selector, TargetSelector::AllAllies));
    }
}
