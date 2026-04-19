//! Integration tests for black (+2) hero variant migration (US-305).
//!
//! Validates that all recruitable DDGC hero class families have black variants
//! that build valid actors, differ from base where expected, and that chaos-mode
//! black mapping resolves to suffix +2 IDs.

use framework_rules::actor::ActorId;
use framework_rules::attributes::{AttributeKey, ATTR_HEALTH, ATTR_SPEED};

use game_ddgc_headless::content::actors::{ATTR_MAX_HEALTH, ATTR_STRESS};
use game_ddgc_headless::heroes::base::all_base_variants;
use game_ddgc_headless::heroes::black::all_black_variants;
use game_ddgc_headless::heroes::families::{ChaosMode, HeroFamilyRegistry};

#[test]
fn all_black_hero_variants_build_valid_actors() {
    let variants = all_black_variants();

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
fn black_variants_differ_from_base_where_expected() {
    let base_variants = all_base_variants();
    let black_variants = all_black_variants();

    for (base, black) in base_variants.iter().zip(black_variants.iter()) {
        // Each black variant must have 7 skills (same as base)
        let base_skills = base.skill_pack();
        let black_skills = black.skill_pack();
        assert_eq!(
            black_skills.len(),
            7,
            "{} black variant should have 7 skills, has {}",
            black.display_name,
            black_skills.len()
        );

        // Black variant skill packs must differ from base in at least one skill.
        // Compare skill-by-skill: at least one pair must have different effect
        // chain lengths (black variants add extra effects like self-bleed, tagged
        // status, stun, or different damage values that base variants don't have).
        let mut found_difference = false;
        for (b, bl) in base_skills.iter().zip(black_skills.iter()) {
            if b.effects.len() != bl.effects.len() {
                found_difference = true;
                break;
            }
        }

        assert!(
            found_difference,
            "{} black variant skill pack is identical to base — expected variant-specific differences",
            black.display_name
        );
    }
}

#[test]
fn black_mode_maps_to_suffix_two_variant() {
    let registry = HeroFamilyRegistry::new();

    for family in registry.all_families() {
        // Chaos value at black threshold (< 50) must resolve to +2 variant
        let black_id = registry.resolve_variant_id(family.base_id, 49);
        assert_eq!(
            black_id,
            Some(family.black_id),
            "Chaos 49 should resolve to black variant for {}",
            family.base_id
        );

        // Black ID must end with '2' (suffix +2)
        assert!(
            family.black_id.ends_with('2'),
            "Black ID '{}' should end with '2'",
            family.black_id
        );

        // Black ID must be base_id + "2"
        let expected_black = format!("{}2", family.base_id);
        assert_eq!(
            family.black_id, expected_black,
            "Black ID should be base_id + '2' for {}",
            family.base_id
        );

        // Low chaos values (0-49) all resolve to black
        for chaos in [0, 25, 49] {
            let resolved = registry.resolve_variant_id(family.base_id, chaos);
            assert_eq!(
                resolved,
                Some(family.black_id),
                "Chaos {} should resolve to black variant for {}",
                chaos,
                family.base_id
            );
        }

        // Normal chaos values (50-149) should NOT resolve to black
        let normal = registry.resolve_variant_id(family.base_id, 100);
        assert_ne!(
            normal,
            Some(family.black_id),
            "Normal chaos should NOT resolve to black variant for {}",
            family.base_id
        );

        // White chaos values (>= 150) should NOT resolve to black
        let white = registry.resolve_variant_id(family.base_id, 150);
        assert_ne!(
            white,
            Some(family.black_id),
            "White chaos should NOT resolve to black variant for {}",
            family.base_id
        );
    }

    // Verify ChaosMode::Black is returned for chaos values < 50
    assert_eq!(ChaosMode::from_chaos_value(0), ChaosMode::Black);
    assert_eq!(ChaosMode::from_chaos_value(49), ChaosMode::Black);
}
