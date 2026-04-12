//! Recruitment and class requirement semantics for DDGC hero class families.
//!
//! DDGC's StageCoach only recruits heroes using base class IDs — variant
//! IDs (white +1, black +2) are never directly recruited. This module
//! preserves that contract by providing:
//!
//! - `is_base_recruit_class` — mirrors `CharacterHelper.IsBaseRecruitHeroClass`
//! - `normalize_recruit_class_id` — mirrors `CharacterHelper.NormalizeRecruitHeroClassId`
//! - `HeroClassMatcher` — mirrors `CharacterHelper.HeroMatchesClassRequirement`
//! - `RecruitPool` — filters the available hero classes to only base IDs,
//!   mirroring the StageCoach `GetRecruitableHeroClasses` behavior.

use crate::heroes::families::{ChaosMode, HeroFamilyRegistry};

// ── Base Recruitment Predicate ─────────────────────────────────────────────

/// Check if a class ID is a recruitable base class.
///
/// Mirrors DDGC's `CharacterHelper.IsBaseRecruitHeroClass`: a class ID
/// is recruitable if it is a base ID (no variant suffix). Variant IDs
/// like `"alchemist1"` or `"alchemist2"` return `false`.
///
/// Unknown class IDs also return `false` — only registered base IDs
/// in the hero family registry are recruitable.
pub fn is_base_recruit_class(class_id: &str, registry: &HeroFamilyRegistry) -> bool {
    match registry.normalize_to_base(class_id) {
        Some(base_id) => base_id == class_id,
        None => false,
    }
}

/// Normalize a class ID to its base recruitment ID.
///
/// Mirrors DDGC's `CharacterHelper.NormalizeRecruitHeroClassId`: strips
/// the variant suffix (if any) and returns the base class ID. Used by
/// the StageCoach `RestockBonus` path to normalize incoming event class
/// IDs (which may be chaos variants) down to their recruitable base form.
///
/// Returns `None` if the class ID is not recognized in the registry.
pub fn normalize_recruit_class_id(class_id: &str, registry: &HeroFamilyRegistry) -> Option<&'static str> {
    registry.normalize_to_base(class_id)
}

// ── Class Requirement Matcher ──────────────────────────────────────────────

/// A hero class identity for class requirement matching.
///
/// Mirrors the three class ID views that DDGC exposes on the `Hero` class:
/// - `class_id`: the raw stored class ID (e.g. `"alchemist"`)
/// - `base_class_id`: the normalized base class ID (e.g. `"alchemist"`)
/// - `chaos_class_id`: the current chaos variant ID (e.g. `"alchemist1"`)
///
/// In DDGC, `Hero.MatchesClassRequirement` checks all three representations,
/// so a trinket requiring `"alchemist"` matches a hero in white chaos mode
/// whose `ChaosClassId` is `"alchemist1"`.
pub struct HeroClassIdentity {
    /// The raw stored class ID (mirrors `Hero.ClassStringId`).
    pub class_id: &'static str,
    /// The current chaos variant ID (mirrors `Hero.ChaosClassId`).
    pub chaos_class_id: &'static str,
}

/// Check if a hero matches a class requirement.
///
/// Mirrors DDGC's `CharacterHelper.HeroMatchesClassRequirement`: a hero
/// matches a required class ID if it matches **any** of the hero's three
/// class representations — the raw stored ID, the base class ID, or the
/// current chaos variant ID.
///
/// This ensures that trinkets and building requirements that reference
/// the base class ID (e.g. `"alchemist"`) still match heroes that are
/// currently in a chaos variant (e.g. white `"alchemist1"` or black
/// `"alchemist2"`).
pub fn hero_matches_class_requirement(
    hero: &HeroClassIdentity,
    required_class_id: &str,
    registry: &HeroFamilyRegistry,
) -> bool {
    let base_class_id = registry.normalize_to_base(hero.class_id);

    // Match against raw class ID
    if hero.class_id == required_class_id {
        return true;
    }

    // Match against base class ID
    if let Some(base_id) = base_class_id {
        if base_id == required_class_id {
            return true;
        }
    }

    // Match against current chaos variant ID
    if hero.chaos_class_id == required_class_id {
        return true;
    }

    false
}

/// Check if a hero matches any class requirement in a list.
///
/// Mirrors DDGC's `CharacterHelper.HeroMatchesAnyClassRequirement`:
/// an empty requirements list means no restriction (returns `true`).
pub fn hero_matches_any_class_requirement(
    hero: &HeroClassIdentity,
    required_class_ids: &[&str],
    registry: &HeroFamilyRegistry,
) -> bool {
    if required_class_ids.is_empty() {
        return true;
    }
    required_class_ids
        .iter()
        .any(|req| hero_matches_class_requirement(hero, req, registry))
}

// ── Recruit Pool ────────────────────────────────────────────────────────────

/// The recruitable hero class pool — filtered to base class IDs only.
///
/// Mirrors the DDGC StageCoach `GetRecruitableHeroClasses` behavior:
/// only base class IDs appear in the recruit pool. Variant IDs like
/// `"alchemist1"` or `"alchemist2"` are excluded because they represent
/// chaos-mode variants that are activated through gameplay, not through
/// recruitment.
pub struct RecruitPool {
    /// The base class IDs available for recruitment, in registry order.
    base_class_ids: Vec<&'static str>,
}

impl RecruitPool {
    /// Create a recruit pool from the hero family registry.
    ///
    /// Filters all registered families to only their base class IDs,
    /// mirroring `StageCoach.GetRecruitableHeroClasses`.
    pub fn new(registry: &HeroFamilyRegistry) -> Self {
        let base_class_ids = registry
            .all_families()
            .iter()
            .filter(|family| is_base_recruit_class(family.base_id, registry))
            .map(|family| family.base_id)
            .collect();

        RecruitPool { base_class_ids }
    }

    /// Get all base class IDs in the recruit pool.
    pub fn base_class_ids(&self) -> &[&'static str] {
        &self.base_class_ids
    }

    /// Check if a class ID is in the recruit pool.
    ///
    /// Only base class IDs are in the pool — variant IDs are never
    /// recruitable.
    pub fn contains(&self, class_id: &str) -> bool {
        self.base_class_ids.contains(&class_id)
    }
}

// ── Recruitment Helpers ─────────────────────────────────────────────────────

/// Build a `HeroClassIdentity` from a base class ID and chaos mode.
///
/// Resolves the chaos variant ID using the registry, mirroring how
/// DDGC's `Hero.ChaosClassId` is computed from `GetChaosHeroClassId`.
pub fn hero_class_identity(
    base_id: &'static str,
    chaos_mode: ChaosMode,
    registry: &HeroFamilyRegistry,
) -> Option<HeroClassIdentity> {
    let family = registry.get_family_by_base(base_id)?;
    let chaos_class_id = family.variant_id(chaos_mode);
    Some(HeroClassIdentity {
        class_id: base_id,
        chaos_class_id,
    })
}
