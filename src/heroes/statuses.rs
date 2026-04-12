//! Variant-aware status semantics for DDGC hero class families.
//!
//! DDGC hero variants differ primarily in their skill effect chains — the same
//! status kinds (bleed, burn, frozen, stun, tagged, guard) may be used across
//! all variants, but the way they're applied (additional effects, different
//! damage values, extra status kinds) varies. This module tracks those
//! differences so the game layer can query variant-specific status semantics
//! without flattening base/white/black into one identical payload.

use std::collections::HashSet;

use crate::heroes::families::{ChaosMode, HeroFamilyRegistry};
use crate::heroes::skills::FamilySkillResolver;

// ── Variant Status Profile ───────────────────────────────────────────────

/// The set of status kinds that a hero variant's skills can apply.
#[derive(Debug, Clone)]
pub struct VariantStatusProfile {
    /// Which status kinds this variant can apply (e.g., "bleed", "burn", "stun", "tagged", "guard").
    pub status_kinds: HashSet<String>,
    /// Total number of apply_status effect nodes across all skills.
    pub apply_status_count: usize,
    /// Total number of effect nodes across all skills.
    pub total_effect_count: usize,
}

// ── Family Status Semantics ───────────────────────────────────────────────

/// Variant-aware status semantics for a hero class family.
///
/// Tracks how status usage differs across base/white/black variants.
/// DDGC differentiates behavior primarily through:
/// - Additional status applications in variant effect chains
/// - Different damage values for the same status kind
/// - Variant-specific marker statuses (tagged, guard)
#[derive(Debug)]
pub struct FamilyStatusSemantics {
    /// The base class ID for this family.
    pub base_id: String,
    /// Status profile for the base (normal) variant.
    pub base_profile: VariantStatusProfile,
    /// Status profile for the white (+1) variant.
    pub white_profile: VariantStatusProfile,
    /// Status profile for the black (+2) variant.
    pub black_profile: VariantStatusProfile,
}

impl FamilyStatusSemantics {
    /// Check if a given status kind is used differently across variants.
    ///
    /// A status is variant-differentiated if it appears in some variants
    /// but not all — indicating that the variant adds or removes that
    /// status from the family's effect vocabulary.
    pub fn is_variant_differentiated(&self, status_kind: &str) -> bool {
        let in_base = self.base_profile.status_kinds.contains(status_kind);
        let in_white = self.white_profile.status_kinds.contains(status_kind);
        let in_black = self.black_profile.status_kinds.contains(status_kind);

        // Variant-differentiated if not uniformly present/absent across all three
        !(in_base && in_white && in_black) && (in_base || in_white || in_black)
    }

    /// Get the status profile for a specific chaos mode.
    pub fn profile_for_mode(&self, mode: ChaosMode) -> &VariantStatusProfile {
        match mode {
            ChaosMode::Normal => &self.base_profile,
            ChaosMode::White => &self.white_profile,
            ChaosMode::Black => &self.black_profile,
        }
    }

    /// Check if any variant in this family has different status profiles.
    ///
    /// A family has variant differences when at least one variant applies
    /// a different set of status kinds or a different number of status
    /// applications compared to the base.
    pub fn has_variant_differences(&self) -> bool {
        self.base_profile.apply_status_count != self.white_profile.apply_status_count
            || self.base_profile.apply_status_count != self.black_profile.apply_status_count
            || self.base_profile.status_kinds != self.white_profile.status_kinds
            || self.base_profile.status_kinds != self.black_profile.status_kinds
    }
}

// ── Family Status Registry ────────────────────────────────────────────────

/// Registry of variant-aware status semantics for all hero families.
///
/// Constructed by inspecting all hero variant skill packs and extracting
/// status kinds from their effect chains. This provides a runtime queryable
/// view of how status semantics vary across variants.
pub struct FamilyStatusRegistry {
    families: Vec<FamilyStatusSemantics>,
}

impl FamilyStatusRegistry {
    /// Create the registry by inspecting all hero variant skill packs.
    pub fn new() -> Self {
        let resolver = FamilySkillResolver::new();
        let hero_registry = HeroFamilyRegistry::new();

        let mut families = Vec::new();

        for family in hero_registry.all_families() {
            let base_skills = resolver
                .resolve_skill_pack(family.base_id, ChaosMode::Normal)
                .expect("base skills must exist");
            let white_skills = resolver
                .resolve_skill_pack(family.base_id, ChaosMode::White)
                .expect("white skills must exist");
            let black_skills = resolver
                .resolve_skill_pack(family.base_id, ChaosMode::Black)
                .expect("black skills must exist");

            let base_status_kinds = crate::heroes::skills::extract_status_kinds(&base_skills);
            let white_status_kinds = crate::heroes::skills::extract_status_kinds(&white_skills);
            let black_status_kinds = crate::heroes::skills::extract_status_kinds(&black_skills);

            let base_apply_count = crate::heroes::skills::count_apply_status(&base_skills);
            let white_apply_count = crate::heroes::skills::count_apply_status(&white_skills);
            let black_apply_count = crate::heroes::skills::count_apply_status(&black_skills);

            let base_total = crate::heroes::skills::total_effect_count(&base_skills);
            let white_total = crate::heroes::skills::total_effect_count(&white_skills);
            let black_total = crate::heroes::skills::total_effect_count(&black_skills);

            families.push(FamilyStatusSemantics {
                base_id: family.base_id.to_string(),
                base_profile: VariantStatusProfile {
                    status_kinds: base_status_kinds.into_iter().collect(),
                    apply_status_count: base_apply_count,
                    total_effect_count: base_total,
                },
                white_profile: VariantStatusProfile {
                    status_kinds: white_status_kinds.into_iter().collect(),
                    apply_status_count: white_apply_count,
                    total_effect_count: white_total,
                },
                black_profile: VariantStatusProfile {
                    status_kinds: black_status_kinds.into_iter().collect(),
                    apply_status_count: black_apply_count,
                    total_effect_count: black_total,
                },
            });
        }

        FamilyStatusRegistry { families }
    }

    /// Look up status semantics by base ID.
    pub fn get_family(&self, base_id: &str) -> Option<&FamilyStatusSemantics> {
        self.families.iter().find(|f| f.base_id == base_id)
    }

    /// Get all family status semantics.
    pub fn all_families(&self) -> &[FamilyStatusSemantics] {
        &self.families
    }
}

impl Default for FamilyStatusRegistry {
    fn default() -> Self {
        Self::new()
    }
}
