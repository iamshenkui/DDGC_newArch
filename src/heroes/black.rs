//! Black (+2) hero variant definitions for all DDGC recruitable hero class families.
//!
//! Each recruitable DDGC hero profession has a black variant that activates when
//! the hero's chaos stored value < 50 (negative chaos). The black variant uses
//! the family's black class ID (suffix +2) and provides variant-specific archetype
//! and skill pack.
//!
//! Black variants are NOT recruitable — only base variants appear in the
//! StageCoach. Black variants are activated through chaos mode switching.

use crate::content::actors::Archetype;
use crate::content::heroes;
use crate::heroes::base::BaseHeroVariant;
use crate::heroes::families::HeroFamilyRegistry;
use framework_combat::skills::SkillDefinition;

/// A black hero variant: the negative-chaos form of a hero class family.
///
/// Each recruitable DDGC hero profession has a black variant that activates
/// when the hero's chaos stored value < 50. The black variant uses the
/// family's black class ID (suffix +2, e.g., "alchemist2") and provides
/// variant-specific archetype and skill pack with modified effect chains.
pub struct BlackHeroVariant {
    /// The black variant class ID (matches HeroClassFamily::black_id).
    pub class_id: &'static str,
    /// The base class ID for this family (e.g., "alchemist").
    pub base_class_id: &'static str,
    /// The display name for this variant.
    pub display_name: &'static str,
}

impl BlackHeroVariant {
    /// Get the archetype for this black hero variant.
    ///
    /// Black variants share the same base stats as their base counterparts
    /// but have a distinct archetype name for registry lookup.
    pub fn archetype(&self) -> Archetype {
        match self.class_id {
            "alchemist2" => heroes::black::alchemist_archetype(),
            "diviner2" => heroes::black::diviner_archetype(),
            "hunter2" => heroes::black::hunter_archetype(),
            "shaman2" => heroes::black::shaman_archetype(),
            "tank2" => heroes::black::tank_archetype(),
            _ => unreachable!("Unknown black hero class: {}", self.class_id),
        }
    }

    /// Get the skill pack for this black hero variant.
    ///
    /// Each black variant has exactly 7 skills matching the DDGC hero skill
    /// template, but with variant-specific effect chains that differ from base.
    pub fn skill_pack(&self) -> Vec<SkillDefinition> {
        match self.class_id {
            "alchemist2" => heroes::black::alchemist_skill_pack(),
            "diviner2" => heroes::black::diviner_skill_pack(),
            "hunter2" => heroes::black::hunter_skill_pack(),
            "shaman2" => heroes::black::shaman_skill_pack(),
            "tank2" => heroes::black::tank_skill_pack(),
            _ => unreachable!("Unknown black hero class: {}", self.class_id),
        }
    }

    /// Check if this black variant's class ID is a valid black variant in the registry.
    ///
    /// Returns true if the class_id maps to a known family via the black_id field.
    pub fn is_black_variant(&self, registry: &HeroFamilyRegistry) -> bool {
        registry
            .get_family_by_variant(self.class_id)
            .map(|f| f.black_id == self.class_id)
            .unwrap_or(false)
    }

    /// Get the corresponding base variant for this black variant.
    pub fn base_variant(&self) -> BaseHeroVariant {
        BaseHeroVariant {
            class_id: self.base_class_id,
            display_name: self.display_name.trim_end_matches(" (Black)"),
        }
    }
}

/// All black hero variants for the 5 recruitable DDGC hero class families.
///
/// Each entry corresponds to a family in HERO_CLASS_FAMILIES.md.
pub fn all_black_variants() -> [BlackHeroVariant; 5] {
    [
        BlackHeroVariant {
            class_id: "alchemist2",
            base_class_id: "alchemist",
            display_name: "Alchemist (Black)",
        },
        BlackHeroVariant {
            class_id: "diviner2",
            base_class_id: "diviner",
            display_name: "Diviner (Black)",
        },
        BlackHeroVariant {
            class_id: "hunter2",
            base_class_id: "hunter",
            display_name: "Hunter (Black)",
        },
        BlackHeroVariant {
            class_id: "shaman2",
            base_class_id: "shaman",
            display_name: "Shaman (Black)",
        },
        BlackHeroVariant {
            class_id: "tank2",
            base_class_id: "tank",
            display_name: "Tank (Black)",
        },
    ]
}
