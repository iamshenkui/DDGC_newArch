//! Integration tests for family-aware skill and status migration (US-306).
//!
//! Tests verify that:
//! - Skill packs are variant-aware (different skill packs per chaos mode)
//! - Status semantics are variant-aware (status profiles differ across variants)
//! - Variant skills preserve original intent (don't degenerate to identical payloads)

use game_ddgc_headless::heroes::families::{ChaosMode, HeroFamilyRegistry};
use game_ddgc_headless::heroes::skills::FamilySkillResolver;
use game_ddgc_headless::heroes::statuses::FamilyStatusRegistry;

/// Each family must resolve distinct skill packs for each chaos mode.
///
/// This test verifies that base, white, and black variants have different
/// skill definitions — different skill IDs, different effect chains, or
/// both. If all three modes produced the same skill pack, that would mean
/// variant differences were lost.
#[test]
fn family_skill_packs_are_variant_aware() {
    let resolver = FamilySkillResolver::new();
    let registry = HeroFamilyRegistry::new();

    for family in registry.all_families() {
        let base_skills = resolver
            .resolve_skill_pack(family.base_id, ChaosMode::Normal)
            .unwrap_or_else(|| panic!("{} base skills missing", family.base_id));
        let white_skills = resolver
            .resolve_skill_pack(family.base_id, ChaosMode::White)
            .unwrap_or_else(|| panic!("{} white skills missing", family.base_id));
        let black_skills = resolver
            .resolve_skill_pack(family.base_id, ChaosMode::Black)
            .unwrap_or_else(|| panic!("{} black skills missing", family.base_id));

        // All variants have 7-9 skills (Hunter base has 9 skills with opening_strike/retribution_strike)
        let base_expected = if family.base_id == "hunter" { 9 } else { 7 };
        assert_eq!(base_skills.len(), base_expected, "{} base should have {} skills", family.base_id, base_expected);
        assert_eq!(white_skills.len(), 7, "{} white should have 7 skills", family.base_id);
        assert_eq!(black_skills.len(), 7, "{} black should have 7 skills", family.base_id);

        // Skill IDs must be distinct across variants
        let base_ids: Vec<String> = base_skills.iter().map(|s| s.id.0.clone()).collect();
        let white_ids: Vec<String> = white_skills.iter().map(|s| s.id.0.clone()).collect();
        let black_ids: Vec<String> = black_skills.iter().map(|s| s.id.0.clone()).collect();

        for base_id in &base_ids {
            assert!(
                !white_ids.contains(base_id),
                "{}: base skill ID '{}' should not appear in white variant",
                family.base_id,
                base_id
            );
            assert!(
                !black_ids.contains(base_id),
                "{}: base skill ID '{}' should not appear in black variant",
                family.base_id,
                base_id
            );
        }

        for white_id in &white_ids {
            assert!(
                !black_ids.contains(white_id),
                "{}: white skill ID '{}' should not appear in black variant",
                family.base_id,
                white_id
            );
        }

        // All skills must validate
        for skill in &base_skills {
            assert!(skill.validate().is_ok(), "{} base skill '{}' invalid", family.base_id, skill.id.0);
        }
        for skill in &white_skills {
            assert!(skill.validate().is_ok(), "{} white skill '{}' invalid", family.base_id, skill.id.0);
        }
        for skill in &black_skills {
            assert!(skill.validate().is_ok(), "{} black skill '{}' invalid", family.base_id, skill.id.0);
        }
    }
}

/// Status semantics must be variant-aware — at least one family has different
/// status profiles across variants.
///
/// DDGC variants differ primarily in effect chains. This test verifies that
/// the status profiles captured by FamilyStatusRegistry reflect those
/// differences: at least one family applies a different set of status kinds
/// or a different number of status applications across variants.
#[test]
fn family_status_semantics_are_variant_aware() {
    let status_registry = FamilyStatusRegistry::new();

    // At least one family must have variant differences
    let any_different = status_registry
        .all_families()
        .iter()
        .any(|f| f.has_variant_differences());
    assert!(
        any_different,
        "At least one hero family must have variant-differentiated status semantics"
    );

    for family in status_registry.all_families() {
        // White and black variants must apply at least one status kind —
        // all DDGC variants add tagged, stun, or additional DoT effects.
        assert!(
            !family.white_profile.status_kinds.is_empty(),
            "{} white variant should apply at least one status kind",
            family.base_id
        );
        assert!(
            !family.black_profile.status_kinds.is_empty(),
            "{} black variant should apply at least one status kind",
            family.base_id
        );

        // Verify that the status profile correctly records apply_status counts
        // and total effect counts (both must be > 0 for white/black)
        assert!(
            family.white_profile.apply_status_count > 0,
            "{} white variant should have at least one apply_status effect",
            family.base_id
        );
        assert!(
            family.black_profile.apply_status_count > 0,
            "{} black variant should have at least one apply_status effect",
            family.base_id
        );
    }

    // Verify specific known variant-differentiated status patterns:
    // - Shaman white adds cross-DoT (burn+frozen, bleed+frozen) not in base
    // - Diviner base has zero status kinds, but white/black add tagged/stun
    let shaman = status_registry.get_family("shaman").unwrap();
    assert!(
        shaman.has_variant_differences(),
        "Shaman family should have variant-differentiated status semantics"
    );

    let diviner = status_registry.get_family("diviner").unwrap();
    assert!(
        diviner.base_profile.status_kinds.is_empty(),
        "Diviner base variant should have zero status kinds (all damage/heal)"
    );
    assert!(
        !diviner.white_profile.status_kinds.is_empty(),
        "Diviner white variant should add status kinds (tagged)"
    );
    assert!(
        diviner.is_variant_differentiated("tagged"),
        "Diviner 'tagged' status should be variant-differentiated"
    );
}

/// Variant skills must preserve original intent — variant skill packs must not
/// degenerate into identical effect payloads.
///
/// This test verifies that for each family, at least one skill has a different
/// effect count between base and white, or base and black. If all skills had
/// the same number of effects in every variant, that would mean variant
/// differences were lost (same payload, different IDs).
#[test]
fn family_variant_skills_preserve_original_intent() {
    let resolver = FamilySkillResolver::new();
    let registry = HeroFamilyRegistry::new();

    for family in registry.all_families() {
        let base_skills = resolver
            .resolve_skill_pack(family.base_id, ChaosMode::Normal)
            .unwrap();
        let white_skills = resolver
            .resolve_skill_pack(family.base_id, ChaosMode::White)
            .unwrap();
        let black_skills = resolver
            .resolve_skill_pack(family.base_id, ChaosMode::Black)
            .unwrap();

        // Count effects per skill for each variant
        let base_effect_counts: Vec<usize> = base_skills.iter().map(|s| s.effects.len()).collect();
        let white_effect_counts: Vec<usize> = white_skills.iter().map(|s| s.effects.len()).collect();
        let black_effect_counts: Vec<usize> = black_skills.iter().map(|s| s.effects.len()).collect();

        // At least one skill in white must differ from base
        let white_differs = base_effect_counts
            .iter()
            .zip(white_effect_counts.iter())
            .any(|(b, w)| b != w);
        assert!(
            white_differs,
            "{}: at least one white skill must have different effect count from base",
            family.base_id
        );

        // At least one skill in black must differ from base
        let black_differs = base_effect_counts
            .iter()
            .zip(black_effect_counts.iter())
            .any(|(b, bk)| b != bk);
        assert!(
            black_differs,
            "{}: at least one black skill must have different effect count from base",
            family.base_id
        );

        // Total effect counts should also differ across variants (when skill counts match)
        // or trivially differ when skill counts differ
        let base_total: usize = base_effect_counts.iter().sum();
        let white_total: usize = white_effect_counts.iter().sum();
        let black_total: usize = black_effect_counts.iter().sum();

        if base_skills.len() == white_skills.len() {
            assert_ne!(
                base_total, white_total,
                "{}: base and white total effect counts should differ",
                family.base_id
            );
        }
        if base_skills.len() == black_skills.len() {
            assert_ne!(
                base_total, black_total,
                "{}: base and black total effect counts should differ",
                family.base_id
            );
        }
    }
}
