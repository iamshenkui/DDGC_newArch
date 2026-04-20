//! Integration tests for base hero variant migration (US-303).
//!
//! Validates that all recruitable DDGC hero class families have base variants
//! that build valid actors, have skill packs, and are recruitable.

use framework_rules::actor::ActorId;
use framework_rules::attributes::{AttributeKey, ATTR_HEALTH, ATTR_SPEED};

use game_ddgc_headless::content::actors::{ATTR_MAX_HEALTH, ATTR_STRESS};
use game_ddgc_headless::heroes::base::all_base_variants;
use game_ddgc_headless::heroes::families::HeroFamilyRegistry;

#[test]
fn all_base_hero_variants_build_valid_actors() {
    let variants = all_base_variants();

    for (i, variant) in variants.iter().enumerate() {
        let archetype = variant.archetype();
        let actor = archetype.create_actor(ActorId(i as u64));

        // Every actor must have positive health
        let health = actor.effective_attribute(&AttributeKey::new(ATTR_HEALTH));
        assert!(
            health.0 > 0.0,
            "{} has non-positive health ({})",
            variant.display_name,
            health.0
        );

        // Every actor must have positive max health
        let max_health = actor.effective_attribute(&AttributeKey::new(ATTR_MAX_HEALTH));
        assert!(
            max_health.0 > 0.0,
            "{} has non-positive max_health ({})",
            variant.display_name,
            max_health.0
        );

        // Current health <= max health
        assert!(
            health.0 <= max_health.0,
            "{} current health {} exceeds max {}",
            variant.display_name,
            health.0,
            max_health.0
        );

        // Every actor must have positive speed
        let speed = actor.effective_attribute(&AttributeKey::new(ATTR_SPEED));
        assert!(
            speed.0 > 0.0,
            "{} has non-positive speed ({})",
            variant.display_name,
            speed.0
        );

        // Stress starts at 0
        let stress = actor.effective_attribute(&AttributeKey::new(ATTR_STRESS));
        assert_eq!(
            stress.0, 0.0,
            "{} starts with non-zero stress",
            variant.display_name
        );
    }
}

#[test]
fn all_base_hero_variants_have_skill_packs() {
    let variants = all_base_variants();

    for variant in &variants {
        let skills = variant.skill_pack();

        // Each base variant has 7-9 skills (DDGC hero template + optional DDGC condition skills)
        // Hunter has 9 skills (includes opening_strike and retribution_strike for DDGC condition demos)
        let is_hunter = variant.class_id == "hunter";
        let expected = if is_hunter { 9 } else { 7 };
        assert_eq!(
            skills.len(),
            expected,
            "{} should have {} skills, has {}",
            variant.display_name,
            expected,
            skills.len()
        );

        // Every skill must validate
        for skill in &skills {
            assert!(
                skill.validate().is_ok(),
                "{} skill '{}' failed validation",
                variant.display_name,
                skill.id.0
            );
        }
    }
}

#[test]
fn all_base_hero_variants_are_recruitable() {
    let registry = HeroFamilyRegistry::new();
    let variants = all_base_variants();

    for variant in &variants {
        assert!(
            variant.is_recruitable(&registry),
            "{} is not recruitable (class_id: {})",
            variant.display_name,
            variant.class_id
        );

        // Base class ID must exist in the family registry
        assert!(
            registry.get_family_by_base(variant.class_id).is_some(),
            "No family found for base class_id {}",
            variant.class_id
        );
    }
}
