//! Hero variant switching and golden trace tests.
//!
//! Verifies that family identity survives chaos-mode variant switching and
//! that the golden trace fixture remains stable. These tests provide a
//! regression baseline so future expansion can't silently break variant
//! switching or family identity.

use game_ddgc_headless::heroes::families::{ChaosMode, HeroFamilyRegistry};
use game_ddgc_headless::heroes::recruitment::{
    hero_class_identity, hero_matches_class_requirement,
};
use game_ddgc_headless::heroes::skills::FamilySkillResolver;
use serde::{Deserialize, Serialize};

// ── Golden Trace Types ──────────────────────────────────────────────────────

/// Golden trace fixture for variant switching transitions.
#[derive(Debug, Deserialize, Serialize)]
struct VariantSwitchTrace {
    scenario: String,
    families: Vec<FamilyTransitions>,
}

#[derive(Debug, Deserialize, Serialize)]
struct FamilyTransitions {
    base_id: String,
    transitions: Vec<Transition>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Transition {
    from_chaos: u32,
    from_variant: String,
    to_chaos: u32,
    to_variant: String,
    family_preserved: bool,
}

// ── Tests ───────────────────────────────────────────────────────────────────

/// Verifies that hero variant switching preserves family identity across all
/// chaos mode transitions for every hero family.
///
/// For each family, this test simulates the full cycle of chaos transitions:
///   base (normal) → white (positive chaos) → black (negative chaos) → base
///
/// At each step, the test verifies:
/// - The variant ID resolved by the registry matches the expected variant
/// - The variant ID normalizes back to the same base family ID
/// - Class requirement matching succeeds for the base family ID regardless
///   of which variant is currently active
/// - Skill packs are available for every variant at every transition point
#[test]
fn hero_variant_switching_preserves_family_identity() {
    let registry = HeroFamilyRegistry::new();
    let resolver = FamilySkillResolver::new();

    for family in registry.all_families() {
        // Define the chaos transitions: normal → white → black → normal
        let transitions: [(u32, &str, ChaosMode); 3] = [
            (150, family.white_id, ChaosMode::White),
            (30, family.black_id, ChaosMode::Black),
            (100, family.base_id, ChaosMode::Normal),
        ];

        for (target_chaos, expected_variant, expected_mode) in &transitions {
            // Resolve variant ID for the target chaos value
            let resolved = registry.resolve_variant_id(family.base_id, *target_chaos);
            assert_eq!(
                resolved,
                Some(*expected_variant),
                "{}: chaos {} should resolve to variant '{}'",
                family.base_id,
                target_chaos,
                expected_variant
            );

            // The resolved variant must normalize back to the base family
            let normalized = registry
                .normalize_to_base(*expected_variant)
                .unwrap_or_else(|| {
                    panic!(
                        "{}: variant '{}' should normalize back to base",
                        family.base_id, expected_variant
                    )
                });
            assert_eq!(
                normalized, family.base_id,
                "{}: variant '{}' normalizes to '{}', expected '{}'",
                family.base_id, expected_variant, normalized, family.base_id
            );

            // Class requirement matching: hero in current variant should still
            // match a requirement for the base class ID
            let identity = hero_class_identity(family.base_id, *expected_mode, &registry)
                .unwrap_or_else(|| {
                    panic!(
                        "{}: failed to build class identity for {:?}",
                        family.base_id, expected_mode
                    )
                });
            assert!(
                hero_matches_class_requirement(&identity, family.base_id, &registry),
                "{}: hero in {:?} mode should match base class requirement '{}'",
                family.base_id,
                expected_mode,
                family.base_id
            );

            // Skill pack must be available for the variant
            let skills = resolver.resolve_skill_pack(family.base_id, *expected_mode);
            assert!(
                skills.is_some(),
                "{}: skill pack must exist for {:?} mode",
                family.base_id,
                expected_mode
            );
            let skills = skills.unwrap();
            // Hunter Normal has 8 skills (includes opening_strike and desperate_strike)
            let expected = if family.base_id == "hunter" && *expected_mode == ChaosMode::Normal {
                8
            } else {
                7
            };
            assert_eq!(
                skills.len(),
                expected,
                "{}: {:?} mode should have {} skills",
                family.base_id,
                expected_mode,
                expected
            );

            // After switching, the family lookup by the new variant ID still
            // points to the same family
            let family_after = registry
                .get_family_by_variant(*expected_variant)
                .unwrap_or_else(|| {
                    panic!(
                        "{}: variant '{}' should resolve to a family",
                        family.base_id, expected_variant
                    )
                });
            assert_eq!(
                family_after.base_id, family.base_id,
                "{}: family identity must survive switching to variant '{}'",
                family.base_id, expected_variant
            );
        }

        // After the full cycle (normal → white → black → normal), verify we
        // can return to the base variant and everything is consistent
        let back_to_base = registry.resolve_variant_id(family.base_id, 100);
        assert_eq!(
            back_to_base,
            Some(family.base_id),
            "{}: after full cycle, normal chaos should resolve back to base",
            family.base_id
        );
    }
}

/// Verifies that the hero variant trace fixture is stable — the committed
/// golden trace matches the live variant switching behavior.
///
/// This test loads the golden trace fixture and replays every transition,
/// verifying that the registry produces the same variant IDs and that
/// family identity is preserved at each step. If the registry changes
/// variant IDs or adds/removes families, this test will fail and the
/// golden trace must be updated.
#[test]
fn hero_variant_trace_is_stable() {
    let registry = HeroFamilyRegistry::new();

    // Load and parse the golden trace
    let golden_json =
        include_str!("../fixtures/hero_families/variant_switch_trace.json");
    let golden: VariantSwitchTrace = serde_json::from_str(golden_json)
        .expect("Failed to parse variant_switch_trace.json");

    // Verify the trace scenario name
    assert_eq!(
        golden.scenario, "hero_variant_switch",
        "Golden trace scenario should be 'hero_variant_switch'"
    );

    // Verify we have transitions for all 5 families
    assert_eq!(
        golden.families.len(),
        5,
        "Golden trace should have 5 families"
    );

    // Replay every transition in the golden trace and verify consistency
    for family_trace in &golden.families {
        let family = registry
            .get_family_by_base(&family_trace.base_id)
            .unwrap_or_else(|| {
                panic!(
                    "Family '{}' from golden trace not found in registry",
                    family_trace.base_id
                )
            });

        for transition in &family_trace.transitions {
            // Verify "from" variant: the from_chaos should resolve to from_variant
            let from_resolved = registry.resolve_variant_id(family.base_id, transition.from_chaos);
            assert_eq!(
                from_resolved,
                Some(transition.from_variant.as_str()),
                "{}: from_chaos {} should resolve to '{}', got {:?}",
                family.base_id,
                transition.from_chaos,
                transition.from_variant,
                from_resolved
            );

            // Verify "to" variant: the to_chaos should resolve to to_variant
            let to_resolved = registry.resolve_variant_id(family.base_id, transition.to_chaos);
            assert_eq!(
                to_resolved,
                Some(transition.to_variant.as_str()),
                "{}: to_chaos {} should resolve to '{}', got {:?}",
                family.base_id,
                transition.to_chaos,
                transition.to_variant,
                to_resolved
            );

            // Verify family identity is preserved
            if transition.family_preserved {
                let normalized = registry
                    .normalize_to_base(&transition.to_variant)
                    .unwrap_or_else(|| {
                        panic!(
                            "{}: variant '{}' should normalize to base",
                            family.base_id, transition.to_variant
                        )
                    });
                assert_eq!(
                    normalized, family.base_id,
                    "{}: family identity not preserved for variant '{}'",
                    family.base_id, transition.to_variant
                );
            }
        }
    }

    // Also verify that running the same transitions live produces the same
    // results as the golden trace (byte-identical if re-serialized)
    let live_trace = generate_live_trace(&registry);
    let live_json = serde_json::to_string_pretty(&live_trace).unwrap();

    // Re-serialize the golden to normalize formatting
    let golden_normalized = serde_json::to_string_pretty(&golden).unwrap();

    assert_eq!(
        live_json, golden_normalized,
        "Live variant switching trace must match committed golden trace"
    );
}

/// Generate a live variant switching trace from the current registry state.
fn generate_live_trace(registry: &HeroFamilyRegistry) -> VariantSwitchTrace {
    let families: Vec<FamilyTransitions> = registry
        .all_families()
        .iter()
        .map(|family| {
            let transitions = vec![
                Transition {
                    from_chaos: 100,
                    from_variant: family.base_id.to_string(),
                    to_chaos: 150,
                    to_variant: family.white_id.to_string(),
                    family_preserved: true,
                },
                Transition {
                    from_chaos: 150,
                    from_variant: family.white_id.to_string(),
                    to_chaos: 30,
                    to_variant: family.black_id.to_string(),
                    family_preserved: true,
                },
                Transition {
                    from_chaos: 30,
                    from_variant: family.black_id.to_string(),
                    to_chaos: 100,
                    to_variant: family.base_id.to_string(),
                    family_preserved: true,
                },
            ];
            FamilyTransitions {
                base_id: family.base_id.to_string(),
                transitions,
            }
        })
        .collect();

    VariantSwitchTrace {
        scenario: "hero_variant_switch".to_string(),
        families,
    }
}
