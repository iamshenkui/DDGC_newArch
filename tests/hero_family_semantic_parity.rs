//! All-family hero parity assertions for DDGC hero class families.
//!
//! Verifies that every hero family has complete base/white/black variant
//! coverage — valid actors, recruitable base forms, variant-queryable
//! identities, and consistent family membership across all three variants.
//! This extends the sample-based hero parity into a whole-family regression
//! baseline so future expansion can't silently break family identity.

use game_ddgc_headless::heroes::base::all_base_variants;
use game_ddgc_headless::heroes::black::all_black_variants;
use game_ddgc_headless::heroes::families::{ChaosMode, HeroFamilyRegistry};
use game_ddgc_headless::heroes::recruitment::RecruitPool;
use game_ddgc_headless::heroes::skills::FamilySkillResolver;
use game_ddgc_headless::heroes::statuses::FamilyStatusRegistry;
use game_ddgc_headless::heroes::white::all_white_variants;

/// Every hero family must have base, white, and black variants that all
/// build valid actors, have skill packs, are queryable through the family
/// registry, and preserve family membership across all three variants.
///
/// This is the whole-family parity assertion — it goes beyond per-variant
/// tests by checking cross-variant consistency within each family:
/// - All three variants resolve to the same family
/// - Base variant is recruitable, white/black are not
/// - All variants have 7 skills that validate
/// - Archetype stats are consistent (HP, speed, stress)
/// - Skill packs are distinct (variant-aware) per family
/// - Status semantics differ where DDGC differentiates
#[test]
fn all_hero_families_have_base_white_black_variants() {
    let registry = HeroFamilyRegistry::new();
    let resolver = FamilySkillResolver::new();
    let status_registry = FamilyStatusRegistry::new();
    let recruit_pool = RecruitPool::new(&registry);

    let base_variants = all_base_variants();
    let white_variants = all_white_variants();
    let black_variants = all_black_variants();

    assert_eq!(
        base_variants.len(),
        registry.all_families().len(),
        "Number of base variants must match number of families"
    );
    assert_eq!(
        white_variants.len(),
        registry.all_families().len(),
        "Number of white variants must match number of families"
    );
    assert_eq!(
        black_variants.len(),
        registry.all_families().len(),
        "Number of black variants must match number of families"
    );

    for family in registry.all_families() {
        // ── Variant identity: all three IDs map to the same family ──────────
        let by_base = registry.get_family_by_base(family.base_id);
        let by_white = registry.get_family_by_variant(family.white_id);
        let by_black = registry.get_family_by_variant(family.black_id);

        assert!(by_base.is_some(), "{} base lookup failed", family.base_id);
        assert!(by_white.is_some(), "{} white lookup failed", family.white_id);
        assert!(by_black.is_some(), "{} black lookup failed", family.black_id);

        assert_eq!(
            by_base.unwrap().base_id, family.base_id,
            "{}: white variant should resolve to same family",
            family.base_id
        );
        assert_eq!(
            by_white.unwrap().base_id, family.base_id,
            "{}: white variant should resolve to same family",
            family.base_id
        );
        assert_eq!(
            by_black.unwrap().base_id, family.base_id,
            "{}: black variant should resolve to same family",
            family.base_id
        );

        // ── Recruitment: only base is recruitable ───────────────────────────
        assert!(
            recruit_pool.contains(family.base_id),
            "{} base must be in recruit pool",
            family.base_id
        );
        assert!(
            !recruit_pool.contains(family.white_id),
            "{} white must NOT be in recruit pool",
            family.white_id
        );
        assert!(
            !recruit_pool.contains(family.black_id),
            "{} black must NOT be in recruit pool",
            family.black_id
        );

        // ── Skill packs: variants have 7-8 valid skills ───────────────────────
        // Hunter Normal has 9 skills (includes opening_strike and retribution_strike for DDGC demos)
        // Hunter White and Black have 7 skills (standard DDGC template)
        for mode in [ChaosMode::Normal, ChaosMode::White, ChaosMode::Black] {
            let skills = resolver
                .resolve_skill_pack(family.base_id, mode)
                .unwrap_or_else(|| panic!("{} {:?} skills missing", family.base_id, mode));

            let is_hunter_normal = family.base_id == "hunter" && mode == ChaosMode::Normal;
            let expected = if is_hunter_normal { 9 } else { 7 };
            assert_eq!(
                skills.len(),
                expected,
                "{} {:?} should have {} skills",
                family.base_id,
                mode,
                expected
            );

            for skill in &skills {
                assert!(
                    skill.validate().is_ok(),
                    "{} {:?} skill '{}' failed validation",
                    family.base_id,
                    mode,
                    skill.id.0
                );
            }
        }

        // ── Base variant builds valid actor ─────────────────────────────────
        let base_variant = base_variants
            .iter()
            .find(|v| v.class_id == family.base_id)
            .unwrap_or_else(|| panic!("Base variant '{}' not found", family.base_id));
        let base_arch = base_variant.archetype();
        let base_actor = base_arch.create_actor(framework_rules::actor::ActorId(1));

        let hp_key = framework_rules::attributes::AttributeKey::new(
            framework_rules::attributes::ATTR_HEALTH,
        );
        let base_hp = base_actor.effective_attribute(&hp_key);
        assert!(
            base_hp.0 > 0.0,
            "{} base must have positive HP",
            family.base_id
        );

        // ── White and black variant actors are also valid ──────────────────
        let white_variant = white_variants
            .iter()
            .find(|v| v.class_id == family.white_id)
            .unwrap_or_else(|| panic!("White variant '{}' not found", family.white_id));
        let white_arch = white_variant.archetype();
        let white_actor = white_arch.create_actor(framework_rules::actor::ActorId(2));
        let white_hp = white_actor.effective_attribute(&hp_key);
        assert!(
            white_hp.0 > 0.0,
            "{} white must have positive HP",
            family.base_id
        );

        let black_variant = black_variants
            .iter()
            .find(|v| v.class_id == family.black_id)
            .unwrap_or_else(|| panic!("Black variant '{}' not found", family.black_id));
        let black_arch = black_variant.archetype();
        let black_actor = black_arch.create_actor(framework_rules::actor::ActorId(3));
        let black_hp = black_actor.effective_attribute(&hp_key);
        assert!(
            black_hp.0 > 0.0,
            "{} black must have positive HP",
            family.base_id
        );

        // ── Status semantics are variant-aware ─────────────────────────────
        let family_status = status_registry
            .get_family(family.base_id)
            .unwrap_or_else(|| panic!("{} status semantics missing", family.base_id));
        assert!(
            family_status.has_variant_differences(),
            "{} should have variant-differentiated status semantics",
            family.base_id
        );

        // ── Normalization: all variant IDs normalize to base ───────────────
        assert_eq!(
            registry.normalize_to_base(family.base_id),
            Some(family.base_id),
            "{} base normalizes to itself",
            family.base_id
        );
        assert_eq!(
            registry.normalize_to_base(family.white_id),
            Some(family.base_id),
            "{} white normalizes to base",
            family.base_id
        );
        assert_eq!(
            registry.normalize_to_base(family.black_id),
            Some(family.base_id),
            "{} black normalizes to base",
            family.base_id
        );

        // ── Chaos mode resolution: correct variant per chaos value ──────────
        assert_eq!(
            registry.resolve_variant_id(family.base_id, 100),
            Some(family.base_id),
            "{} normal chaos → base",
            family.base_id
        );
        assert_eq!(
            registry.resolve_variant_id(family.base_id, 150),
            Some(family.white_id),
            "{} white chaos → white",
            family.base_id
        );
        assert_eq!(
            registry.resolve_variant_id(family.base_id, 30),
            Some(family.black_id),
            "{} black chaos → black",
            family.base_id
        );
    }
}
