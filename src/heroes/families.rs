//! Hero class family registry and normalization helpers.
//!
//! Each recruitable DDGC hero profession is represented as a `HeroClassFamily` with
//! base, white (+1), and black (+2) variant IDs. The `HeroFamilyRegistry` provides
//! lookup by base or variant ID, normalization back to base, and chaos-mode variant
//! resolution — all without using framework internals.

use std::collections::HashMap;

// ── Chaos Mode ────────────────────────────────────────────────────────────

/// Determines which hero variant is active based on chaos value.
///
/// DDGC chaos is stored 0–200 (displayed −100 to +100). The thresholds
/// mirror `Hero.cs` in the original source:
/// - stored < 50 → Black (negative chaos)
/// - 50–149 → Normal (base variant)
/// - >= 150 → White (positive chaos)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChaosMode {
    /// Normal mode — base variant (no suffix).
    Normal,
    /// White / positive chaos — +1 variant suffix.
    White,
    /// Black / negative chaos — +2 variant suffix.
    Black,
}

impl ChaosMode {
    /// Resolve chaos mode from a stored chaos value (0–200 range).
    pub fn from_chaos_value(stored_value: u32) -> Self {
        if stored_value < 50 {
            ChaosMode::Black
        } else if stored_value >= 150 {
            ChaosMode::White
        } else {
            ChaosMode::Normal
        }
    }
}

// ── Hero Class Family ─────────────────────────────────────────────────────

/// A hero class family: one logical profession with base/white/black variants.
///
/// Maps to the DDGC `CharacterHelper::GetBaseHeroClassId` /
/// `CharacterHelper::GetChaosHeroClassId` contract:
/// - `base_id` has no suffix (e.g. `"alchemist"`)
/// - `white_id` has suffix `"1"` (e.g. `"alchemist1"`)
/// - `black_id` has suffix `"2"` (e.g. `"alchemist2"`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeroClassFamily {
    /// Base class ID — used for recruitment and normalization.
    pub base_id: &'static str,
    /// White variant ID with +1 suffix.
    pub white_id: &'static str,
    /// Black variant ID with +2 suffix.
    pub black_id: &'static str,
    /// Human-readable family name.
    pub display_name: &'static str,
}

impl HeroClassFamily {
    /// Resolve the variant ID for the given chaos mode.
    pub fn variant_id(&self, mode: ChaosMode) -> &'static str {
        match mode {
            ChaosMode::Normal => self.base_id,
            ChaosMode::White => self.white_id,
            ChaosMode::Black => self.black_id,
        }
    }

    /// All variant IDs for this family (base, white, black).
    pub fn all_variant_ids(&self) -> [&'static str; 3] {
        [self.base_id, self.white_id, self.black_id]
    }
}

// ── Registry ──────────────────────────────────────────────────────────────

/// The hero class family registry — provides lookup by base ID or any variant ID.
///
/// Constructed via `HeroFamilyRegistry::new()` with all five recruitable DDGC
/// hero class families from `HERO_CLASS_FAMILIES.md`.
pub struct HeroFamilyRegistry {
    families: Vec<HeroClassFamily>,
    /// Map from any variant ID (base, white, or black) to the family index.
    id_to_family: HashMap<&'static str, usize>,
}

impl HeroFamilyRegistry {
    /// Create the registry with all recruitable DDGC hero class families.
    pub fn new() -> Self {
        let families = vec![
            HeroClassFamily {
                base_id: "alchemist",
                white_id: "alchemist1",
                black_id: "alchemist2",
                display_name: "Alchemist",
            },
            HeroClassFamily {
                base_id: "diviner",
                white_id: "diviner1",
                black_id: "diviner2",
                display_name: "Diviner",
            },
            HeroClassFamily {
                base_id: "hunter",
                white_id: "hunter1",
                black_id: "hunter2",
                display_name: "Hunter",
            },
            HeroClassFamily {
                base_id: "shaman",
                white_id: "shaman1",
                black_id: "shaman2",
                display_name: "Shaman",
            },
            HeroClassFamily {
                base_id: "tank",
                white_id: "tank1",
                black_id: "tank2",
                display_name: "Tank",
            },
        ];

        let mut id_to_family = HashMap::new();
        for (i, family) in families.iter().enumerate() {
            id_to_family.insert(family.base_id, i);
            id_to_family.insert(family.white_id, i);
            id_to_family.insert(family.black_id, i);
        }

        HeroFamilyRegistry {
            families,
            id_to_family,
        }
    }

    /// Look up a family by its base ID.
    pub fn get_family_by_base(&self, base_id: &str) -> Option<&HeroClassFamily> {
        self.id_to_family
            .get(base_id)
            .map(|&i| &self.families[i])
    }

    /// Look up a family by any variant ID (base, white, or black).
    pub fn get_family_by_variant(&self, variant_id: &str) -> Option<&HeroClassFamily> {
        self.id_to_family
            .get(variant_id)
            .map(|&i| &self.families[i])
    }

    /// Normalize a variant ID to its base ID.
    ///
    /// Returns the base ID if the variant belongs to a known family,
    /// or `None` if the variant ID is unknown.
    pub fn normalize_to_base(&self, variant_id: &str) -> Option<&'static str> {
        self.get_family_by_variant(variant_id)
            .map(|f| f.base_id)
    }

    /// Resolve the current variant ID for a family given a chaos stored value.
    ///
    /// This is the game-layer helper that maps chaos mode to variant IDs
    /// without using framework internals. The chaos thresholds match the
    /// DDGC original: < 50 → black, 50–149 → base, >= 150 → white.
    pub fn resolve_variant_id(
        &self,
        base_id: &str,
        chaos_value: u32,
    ) -> Option<&'static str> {
        let mode = ChaosMode::from_chaos_value(chaos_value);
        self.get_family_by_base(base_id)
            .map(|f| f.variant_id(mode))
    }

    /// Get all registered families.
    pub fn all_families(&self) -> &[HeroClassFamily] {
        &self.families
    }
}

impl Default for HeroFamilyRegistry {
    fn default() -> Self {
        Self::new()
    }
}
