//! Integration tests for white (+1) hero variant migration (US-304).
//!
//! Validates that all recruitable DDGC hero class families have white variants
//! that build valid actors, differ from base where expected, and that chaos-mode
//! white mapping resolves to suffix +1 IDs.

use framework_rules::actor::ActorId;
use framework_rules::attributes::{AttributeKey, ATTR_HEALTH, ATTR_SPEED};

use game_ddgc_headless::content::actors::ATTR_MAX_HEALTH;
use game_ddgc_headless::heroes::base::all_base_variants;
use game_ddgc_headless::heroes::families::{ChaosMode, HeroFamilyRegistry};
use game_ddgc_headless::heroes::white::all_white_variants;

#[test]
fn all_white_hero_variants_build_valid_actors() {
    let variants = all_white_variants();

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
        let stress = actor.effective_attribute(&AttributeKey::new("stress"));
        assert_eq!(
            stress.0, 0.0,
            "{} starts with non-zero stress",
            variant.display_name
        );
    }
}

#[test]
fn white_variants_differ_from_base_where_expected() {
    let base_variants = all_base_variants();
    let white_variants = all_white_variants();

    for (base, white) in base_variants.iter().zip(white_variants.iter()) {
        // Each white variant must have 7 skills (same as base)
        let base_skills = base.skill_pack();
        let white_skills = white.skill_pack();
        assert_eq!(
            white_skills.len(),
            7,
            "{} white variant should have 7 skills, has {}",
            white.display_name,
            white_skills.len()
        );

        // White variant skill packs must differ from base in at least one skill.
        // Compare skill-by-skill: at least one pair must have different effect
        // chain lengths (white variants add extra effects like self-heal, secondary
        // DoT, or tagged status that base variants don't have).
        let mut found_difference = false;
        for (b, w) in base_skills.iter().zip(white_skills.iter()) {
            if b.effects.len() != w.effects.len() {
                found_difference = true;
                break;
            }
        }

        assert!(
            found_difference,
            "{} white variant skill pack is identical to base — expected variant-specific differences",
            white.display_name
        );
    }
}

#[test]
fn white_mode_maps_to_suffix_one_variant() {
    let registry = HeroFamilyRegistry::new();

    for family in registry.all_families() {
        // Chaos value at white threshold (150) must resolve to +1 variant
        let white_id = registry.resolve_variant_id(family.base_id, 150);
        assert_eq!(
            white_id,
            Some(family.white_id),
            "Chaos 150 should resolve to white variant for {}",
            family.base_id
        );

        // White ID must end with '1' (suffix +1)
        assert!(
            family.white_id.ends_with('1'),
            "White ID '{}' should end with '1'",
            family.white_id
        );

        // White ID must be base_id + "1"
        let expected_white = format!("{}1", family.base_id);
        assert_eq!(
            family.white_id, expected_white,
            "White ID should be base_id + '1' for {}",
            family.base_id
        );

        // High chaos values (150-200) all resolve to white
        for chaos in [150, 175, 200] {
            let resolved = registry.resolve_variant_id(family.base_id, chaos);
            assert_eq!(
                resolved,
                Some(family.white_id),
                "Chaos {} should resolve to white variant for {}",
                chaos,
                family.base_id
            );
        }

        // Normal chaos values (50-149) should NOT resolve to white
        let normal = registry.resolve_variant_id(family.base_id, 100);
        assert_ne!(
            normal,
            Some(family.white_id),
            "Normal chaos should NOT resolve to white variant for {}",
            family.base_id
        );

        // Black chaos values (< 50) should NOT resolve to white
        let black = registry.resolve_variant_id(family.base_id, 30);
        assert_ne!(
            black,
            Some(family.white_id),
            "Black chaos should NOT resolve to white variant for {}",
            family.base_id
        );
    }

    // Verify ChaosMode::White is returned for chaos values >= 150
    assert_eq!(ChaosMode::from_chaos_value(150), ChaosMode::White);
    assert_eq!(ChaosMode::from_chaos_value(200), ChaosMode::White);
}
