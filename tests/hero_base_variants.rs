//! Integration tests for base hero variant migration (US-303).
//!
//! Validates that all recruitable DDGC hero class families have base variants
//! that build valid actors, have skill packs, and are recruitable.

use framework_rules::actor::ActorId;
use framework_rules::attributes::{AttributeKey, ATTR_HEALTH, ATTR_SPEED};

use game_ddgc_headless::content::actors::{ATTR_MAX_HEALTH, ATTR_STRESS};
use game_ddgc_headless::contracts::{DungeonType, TrinketDefinition, TrinketRarity};
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

        // Each base variant has 7-10 skills (DDGC hero template + optional DDGC condition skills)
        // Hunter has 12 skills (includes opening_strike, desperate_strike, retribution_strike,
        // xuanwu_strike, and executioner_strike for DDGC condition demos)
        let is_hunter = variant.class_id == "hunter";
        let expected = if is_hunter { 12 } else { 7 };
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

#[test]
fn level_1_equipped_hero_has_different_stats_than_level_0() {
    // Test that a hero with level-1 weapon has higher attack than level-0.
    // This validates the equipment-aware archetype creation.
    let variants = all_base_variants();

    for variant in &variants {
        let archetype_level_0 = variant.archetype_with_equipment(0, 0, &[]);
        let archetype_level_1_weapon = variant.archetype_with_equipment(1, 0, &[]);
        let archetype_level_1_armor = variant.archetype_with_equipment(0, 1, &[]);

        // Level-1 weapon should have strictly higher attack than level-0
        assert!(
            archetype_level_1_weapon.attack > archetype_level_0.attack,
            "{}: level-1 weapon attack ({}) should be > level-0 attack ({})",
            variant.display_name,
            archetype_level_1_weapon.attack,
            archetype_level_0.attack
        );

        // Level-1 armor should have strictly higher max_health than level-0
        assert!(
            archetype_level_1_armor.max_health > archetype_level_0.max_health,
            "{}: level-1 armor max_health ({}) should be > level-0 max_health ({})",
            variant.display_name,
            archetype_level_1_armor.max_health,
            archetype_level_0.max_health
        );

        // Level-1 armor should have higher or equal defense (some classes get DEF boost)
        assert!(
            archetype_level_1_armor.defense >= archetype_level_0.defense,
            "{}: level-1 armor defense ({}) should be >= level-0 defense ({})",
            variant.display_name,
            archetype_level_1_armor.defense,
            archetype_level_0.defense
        );
    }
}

#[test]
fn equipment_aware_archetype_with_trinkets_modifies_stats() {
    // Test that trinkets modify stats when using archetype_with_equipment
    let trinket = TrinketDefinition::new(
        "test_atk_trinket",
        vec!["ATK+10".to_string()],
        vec![],
        TrinketRarity::Common,
        100,
        1,
        DungeonType::QingLong,
    );

    let variant = &all_base_variants()[0]; // alchemist
    let archetype_no_trinket = variant.archetype_with_equipment(0, 0, &[]);
    let archetype_with_trinket = variant.archetype_with_equipment(0, 0, &[&trinket]);

    // With trinket: ATK should be base + 10
    assert_eq!(
        archetype_with_trinket.attack,
        archetype_no_trinket.attack + 10.0,
        "trinket should add exactly 10 ATK"
    );
}
