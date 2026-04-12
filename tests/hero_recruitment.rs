//! Recruitment and class requirement semantic tests.
//!
//! Verifies that the headless project preserves DDGC's recruitment and
//! class requirement semantics: only base class IDs appear in the recruit
//! pool, class requirement matching checks all three class representations,
//! and variant IDs normalize back to base IDs.

use game_ddgc_headless::heroes::{
    hero_class_identity, hero_matches_class_requirement, is_base_recruit_class,
    normalize_recruit_class_id, ChaosMode, HeroFamilyRegistry, RecruitPool,
};

/// Verifies that recruitment uses only base class IDs.
///
/// Mirrors DDGC's `StageCoach.GetRecruitableHeroClasses` which filters
/// all registered hero classes through `CharacterHelper.IsBaseRecruitHeroClass`.
/// Variant IDs like `"alchemist1"` or `"alchemist2"` must never appear in
/// the recruit pool.
#[test]
fn recruitment_uses_base_class_ids_only() {
    let registry = HeroFamilyRegistry::new();
    let pool = RecruitPool::new(&registry);

    // All 5 base class IDs are in the recruit pool
    for family in registry.all_families() {
        assert!(
            pool.contains(family.base_id),
            "Base class '{}' should be in recruit pool",
            family.base_id
        );
    }

    // Variant IDs are NOT in the recruit pool
    for family in registry.all_families() {
        assert!(
            !pool.contains(family.white_id),
            "White variant '{}' should NOT be in recruit pool",
            family.white_id
        );
        assert!(
            !pool.contains(family.black_id),
            "Black variant '{}' should NOT be in recruit pool",
            family.black_id
        );
    }

    // is_base_recruit_class mirrors DDGC's IsBaseRecruitHeroClass
    for family in registry.all_families() {
        assert!(
            is_base_recruit_class(family.base_id, &registry),
            "is_base_recruit_class('{}') should be true",
            family.base_id
        );
        assert!(
            !is_base_recruit_class(family.white_id, &registry),
            "is_base_recruit_class('{}') should be false",
            family.white_id
        );
        assert!(
            !is_base_recruit_class(family.black_id, &registry),
            "is_base_recruit_class('{}') should be false",
            family.black_id
        );
    }

    // Unknown IDs are not recruitable
    assert!(!is_base_recruit_class("nonexistent", &registry));
}

/// Verifies that class requirement matching succeeds for current class ID,
/// base class ID, and active chaos variant ID.
///
/// Mirrors DDGC's `CharacterHelper.HeroMatchesClassRequirement` which
/// checks all three class representations. A trinket requiring `"alchemist"`
/// must match a hero currently in white chaos mode whose `ChaosClassId`
/// is `"alchemist1"`.
#[test]
fn hero_matches_base_and_current_variant_requirements() {
    let registry = HeroFamilyRegistry::new();

    // Test each family in each chaos mode
    for family in registry.all_families() {
        // Normal mode: class_id == base_id, chaos_class_id == base_id
        let normal_identity =
            hero_class_identity(family.base_id, ChaosMode::Normal, &registry).unwrap();

        // A requirement for the base ID matches
        assert!(
            hero_matches_class_requirement(&normal_identity, family.base_id, &registry),
            "Normal {} should match base requirement '{}'",
            family.base_id,
            family.base_id
        );

        // White chaos: class_id == base_id, chaos_class_id == white_id
        let white_identity =
            hero_class_identity(family.base_id, ChaosMode::White, &registry).unwrap();

        // Base class requirement still matches (critical DDGC contract)
        assert!(
            hero_matches_class_requirement(&white_identity, family.base_id, &registry),
            "White {} should match base requirement '{}'",
            family.base_id,
            family.base_id
        );
        // White variant ID also matches
        assert!(
            hero_matches_class_requirement(&white_identity, family.white_id, &registry),
            "White {} should match chaos variant requirement '{}'",
            family.base_id,
            family.white_id
        );
        // Black variant ID should NOT match (wrong chaos mode)
        assert!(
            !hero_matches_class_requirement(&white_identity, family.black_id, &registry),
            "White {} should NOT match black variant requirement '{}'",
            family.base_id,
            family.black_id
        );

        // Black chaos: class_id == base_id, chaos_class_id == black_id
        let black_identity =
            hero_class_identity(family.base_id, ChaosMode::Black, &registry).unwrap();

        // Base class requirement still matches (critical DDGC contract)
        assert!(
            hero_matches_class_requirement(&black_identity, family.base_id, &registry),
            "Black {} should match base requirement '{}'",
            family.base_id,
            family.base_id
        );
        // Black variant ID also matches
        assert!(
            hero_matches_class_requirement(&black_identity, family.black_id, &registry),
            "Black {} should match chaos variant requirement '{}'",
            family.base_id,
            family.black_id
        );
        // White variant ID should NOT match (wrong chaos mode)
        assert!(
            !hero_matches_class_requirement(&black_identity, family.white_id, &registry),
            "Black {} should NOT match white variant requirement '{}'",
            family.base_id,
            family.white_id
        );
    }

    // Cross-family mismatch: alchemist hero should not match tank requirement
    let alchemist =
        hero_class_identity("alchemist", ChaosMode::Normal, &registry).unwrap();
    assert!(
        !hero_matches_class_requirement(&alchemist, "tank", &registry),
        "Alchemist should not match tank requirement"
    );
}

/// Verifies that variant IDs normalize back to base IDs.
///
/// Mirrors DDGC's `CharacterHelper.NormalizeRecruitHeroClassId` and
/// `CharacterHelper.GetBaseHeroClassId` contract: stripping variant
/// suffixes yields the base class ID used for recruitment.
#[test]
fn variant_ids_normalize_back_to_base() {
    let registry = HeroFamilyRegistry::new();

    // Base IDs normalize to themselves
    for family in registry.all_families() {
        let normalized = normalize_recruit_class_id(family.base_id, &registry);
        assert_eq!(
            normalized,
            Some(family.base_id),
            "Base ID '{}' should normalize to itself",
            family.base_id
        );
    }

    // White variant IDs normalize to their base
    for family in registry.all_families() {
        let normalized = normalize_recruit_class_id(family.white_id, &registry);
        assert_eq!(
            normalized,
            Some(family.base_id),
            "White ID '{}' should normalize to base '{}'",
            family.white_id,
            family.base_id
        );
    }

    // Black variant IDs normalize to their base
    for family in registry.all_families() {
        let normalized = normalize_recruit_class_id(family.black_id, &registry);
        assert_eq!(
            normalized,
            Some(family.base_id),
            "Black ID '{}' should normalize to base '{}'",
            family.black_id,
            family.base_id
        );
    }

    // Unknown IDs return None
    assert_eq!(normalize_recruit_class_id("nonexistent", &registry), None);
    assert_eq!(normalize_recruit_class_id("alchemist3", &registry), None);
    assert_eq!(normalize_recruit_class_id("", &registry), None);
}
