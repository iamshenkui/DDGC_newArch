//! Skill semantic parity integration tests.
//!
//! Verifies that migrated skills preserve their original targeting semantics,
//! position constraints, effect chains, and usage restrictions.

use game_ddgc_headless::content::ContentPack;
use game_ddgc_headless::parity::skills::{EffectExpectation, SkillParityFixture, TargetSelectorPattern};

use framework_combat::effects::EffectKind;
use framework_combat::skills::SkillId;
use framework_combat::targeting::TargetSelector;

/// Verifies skill targeting semantics are preserved:
/// - crusading_strike==AllEnemies, divine_grace==AllAllies,
///   holy_lance==AllEnemies, rend==AllEnemies,
///   skull_bash==AllEnemies, grave_bash==AllEnemies
#[test]
fn skill_targeting_semantics_are_preserved() {
    let fixture = SkillParityFixture::new();
    let pack = ContentPack::default();

    for exp in fixture.all() {
        let skill = pack.get_skill(&SkillId::new(exp.name))
            .unwrap_or_else(|| panic!("Skill '{}' missing from ContentPack", exp.name));

        let matches = matches!(
            (&skill.target_selector, &exp.target_selector),
            (TargetSelector::AllEnemies, TargetSelectorPattern::AllEnemies)
                | (TargetSelector::AllAllies, TargetSelectorPattern::AllAllies)
        );
        assert!(matches, "Skill '{}' targeting mismatch: expected {:?}, got {:?}",
            exp.name, exp.target_selector, skill.target_selector);
    }
}

/// Verifies skill position constraints are preserved:
/// - crusading_strike action_cost==1 cooldown==None
/// - holy_lance cooldown==Some(2)
/// - divine_grace cooldown==None
/// - skull_bash cooldown==Some(3)
#[test]
fn skill_position_constraints_are_preserved() {
    let fixture = SkillParityFixture::new();
    let pack = ContentPack::default();

    for exp in fixture.all() {
        let skill = pack.get_skill(&SkillId::new(exp.name))
            .unwrap_or_else(|| panic!("Skill '{}' missing from ContentPack", exp.name));

        assert_eq!(skill.action_cost, exp.action_cost,
            "Skill '{}' action_cost mismatch", exp.name);
        assert_eq!(skill.cooldown, exp.cooldown,
            "Skill '{}' cooldown mismatch", exp.name);
    }
}

/// Verifies skill effect chain semantics are preserved:
/// - crusading_strike 1 effect
/// - holy_lance 2 effects
/// - rend 2 effects (damage+status)
/// - skull_bash 2 effects (damage+conditional status)
/// - grave_bash 2 effects (damage+damage)
#[test]
fn skill_effect_chain_semantics_are_preserved() {
    let fixture = SkillParityFixture::new();
    let pack = ContentPack::default();

    for exp in fixture.all() {
        let skill = pack.get_skill(&SkillId::new(exp.name))
            .unwrap_or_else(|| panic!("Skill '{}' missing from ContentPack", exp.name));

        assert_eq!(skill.effects.len(), exp.effect_count,
            "Skill '{}' effect count mismatch: expected {}, got {}",
            exp.name, exp.effect_count, skill.effects.len());

        // Verify effect chain pattern matches
        for (i, effect_exp) in exp.effect_chain.iter().enumerate() {
            let node = &skill.effects[i];
            let matches = match effect_exp {
                EffectExpectation::Damage => matches!(node.kind, EffectKind::Damage),
                EffectExpectation::Heal => matches!(node.kind, EffectKind::Heal),
                EffectExpectation::ApplyStatus => {
                    matches!(node.kind, EffectKind::ApplyStatus) && node.conditions.is_empty()
                }
                EffectExpectation::ConditionalApplyStatus => {
                    matches!(node.kind, EffectKind::ApplyStatus) && !node.conditions.is_empty()
                }
            };
            assert!(matches,
                "Skill '{}' effect[{}] mismatch: expected {:?}, got {:?}",
                exp.name, i, effect_exp, node.kind);
        }
    }
}

/// Verifies skill usage restrictions match original intent:
/// - All 6 skills pass validate()
/// - holy_lance and skull_bash have cooldown.is_some()
/// - Rest have cooldown.is_none()
/// - All have action_cost >= 1
#[test]
fn skill_usage_restrictions_match_original_intent() {
    let fixture = SkillParityFixture::new();
    let pack = ContentPack::default();

    for exp in fixture.all() {
        let skill = pack.get_skill(&SkillId::new(exp.name))
            .unwrap_or_else(|| panic!("Skill '{}' missing from ContentPack", exp.name));

        // All skills validate
        assert!(skill.validate().is_ok(),
            "Skill '{}' should pass validation", exp.name);

        // All skills have action_cost >= 1
        assert!(skill.action_cost >= 1,
            "Skill '{}' action_cost {} should be >= 1", exp.name, skill.action_cost);

        // Cooldown presence matches expectation
        let has_cooldown = skill.cooldown.is_some();
        let expects_cooldown = exp.cooldown.is_some();
        assert_eq!(has_cooldown, expects_cooldown,
            "Skill '{}' cooldown presence mismatch: expected {:?}, got {:?}",
            exp.name, exp.cooldown, skill.cooldown);
    }
}
