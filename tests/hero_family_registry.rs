//! Hero family registry integration tests.
//!
//! Verifies that the HeroFamilyRegistry correctly maps base/white/black variant
//! IDs, normalizes variant IDs back to base, and rejects unknown variant IDs.

use game_ddgc_headless::heroes::{ChaosMode, HeroFamilyRegistry};

/// Verifies that the family registry correctly maps base, white (+1), and
/// black (+2) variant IDs for every recruitable DDGC hero class family.
#[test]
fn family_registry_maps_base_white_black_variants() {
    let registry = HeroFamilyRegistry::new();

    for family in registry.all_families() {
        // Lookup by base ID must find the family
        let found = registry.get_family_by_base(family.base_id);
        assert!(found.is_some(), "Family not found by base_id '{}'", family.base_id);
        assert_eq!(found.unwrap().base_id, family.base_id);

        // Lookup by white ID must find the same family
        let found = registry.get_family_by_variant(family.white_id);
        assert!(found.is_some(), "Family not found by white_id '{}'", family.white_id);
        assert_eq!(found.unwrap().base_id, family.base_id);

        // Lookup by black ID must find the same family
        let found = registry.get_family_by_variant(family.black_id);
        assert!(found.is_some(), "Family not found by black_id '{}'", family.black_id);
        assert_eq!(found.unwrap().base_id, family.base_id);

        // Chaos mode variant resolution
        let base_via_normal = registry.resolve_variant_id(family.base_id, 100);
        assert_eq!(base_via_normal, Some(family.base_id),
            "Normal chaos should resolve to base for '{}'", family.base_id);

        let white_via_chaos = registry.resolve_variant_id(family.base_id, 150);
        assert_eq!(white_via_chaos, Some(family.white_id),
            "White chaos should resolve to +1 variant for '{}'", family.base_id);

        let black_via_chaos = registry.resolve_variant_id(family.base_id, 30);
        assert_eq!(black_via_chaos, Some(family.black_id),
            "Black chaos should resolve to +2 variant for '{}'", family.base_id);

        // Verify variant ID format: white has +1 suffix, black has +2
        assert!(family.white_id.ends_with('1'),
            "White ID '{}' should end with '1'", family.white_id);
        assert!(family.black_id.ends_with('2'),
            "Black ID '{}' should end with '2'", family.black_id);
        assert!(family.white_id.starts_with(family.base_id),
            "White ID '{}' should start with base_id '{}'", family.white_id, family.base_id);
        assert!(family.black_id.starts_with(family.base_id),
            "Black ID '{}' should start with base_id '{}'", family.black_id, family.base_id);
    }

    // Verify all 5 families are present
    assert_eq!(registry.all_families().len(), 5, "Should have 5 hero class families");
}

/// Verifies that the registry normalizes variant IDs back to their base IDs,
/// matching the DDGC `CharacterHelper::GetBaseHeroClassId` contract.
#[test]
fn family_registry_normalizes_variant_to_base() {
    let registry = HeroFamilyRegistry::new();

    // Base IDs normalize to themselves
    for family in registry.all_families() {
        assert_eq!(registry.normalize_to_base(family.base_id), Some(family.base_id),
            "Base ID '{}' should normalize to itself", family.base_id);
    }

    // White and black variant IDs normalize back to base
    for family in registry.all_families() {
        assert_eq!(registry.normalize_to_base(family.white_id), Some(family.base_id),
            "White ID '{}' should normalize to base '{}'", family.white_id, family.base_id);
        assert_eq!(registry.normalize_to_base(family.black_id), Some(family.base_id),
            "Black ID '{}' should normalize to base '{}'", family.black_id, family.base_id);
    }

    // ChaosMode::from_chaos_value matches DDGC thresholds
    assert_eq!(ChaosMode::from_chaos_value(0), ChaosMode::Black, "0 → Black");
    assert_eq!(ChaosMode::from_chaos_value(49), ChaosMode::Black, "49 → Black");
    assert_eq!(ChaosMode::from_chaos_value(50), ChaosMode::Normal, "50 → Normal");
    assert_eq!(ChaosMode::from_chaos_value(100), ChaosMode::Normal, "100 → Normal");
    assert_eq!(ChaosMode::from_chaos_value(149), ChaosMode::Normal, "149 → Normal");
    assert_eq!(ChaosMode::from_chaos_value(150), ChaosMode::White, "150 → White");
    assert_eq!(ChaosMode::from_chaos_value(200), ChaosMode::White, "200 → White");
}

/// Verifies that the registry rejects unknown variant IDs that do not
/// belong to any registered hero class family.
#[test]
fn family_registry_rejects_unknown_variant() {
    let registry = HeroFamilyRegistry::new();

    // Completely unknown IDs return None
    assert!(registry.get_family_by_variant("nonexistent").is_none());
    assert!(registry.get_family_by_variant("unknown_hero").is_none());
    assert!(registry.normalize_to_base("nonexistent").is_none());
    assert!(registry.get_family_by_base("nonexistent").is_none());

    // IDs with wrong suffixes return None
    assert!(registry.get_family_by_variant("alchemist3").is_none());
    assert!(registry.get_family_by_variant("tank0").is_none());

    // Partial matches don't accidentally succeed
    assert!(registry.get_family_by_variant("alchemis").is_none());
    assert!(registry.get_family_by_variant("hunt").is_none());

    // Empty string
    assert!(registry.get_family_by_variant("").is_none());

    // resolve_variant_id for unknown base returns None
    assert!(registry.resolve_variant_id("nonexistent", 100).is_none());
}
