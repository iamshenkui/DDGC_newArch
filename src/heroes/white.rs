//! White (+1) hero variant definitions for all DDGC recruitable hero class families.
//!
//! Each recruitable DDGC hero profession has a white variant that activates when
//! the hero's chaos stored value >= 150 (positive chaos). The white variant uses
//! the family's white class ID (suffix +1) and provides variant-specific archetype
//! and skill pack.
//!
//! White variants are NOT recruitable — only base variants appear in the
//! StageCoach. White variants are activated through chaos mode switching.

use crate::content::actors::Archetype;
use crate::content::heroes;
use crate::heroes::base::BaseHeroVariant;
use crate::heroes::families::HeroFamilyRegistry;
use framework_combat::skills::SkillDefinition;

/// A white hero variant: the positive-chaos form of a hero class family.
///
/// Each recruitable DDGC hero profession has a white variant that activates
/// when the hero's chaos stored value >= 150. The white variant uses the
/// family's white class ID (suffix +1, e.g., "alchemist1") and provides
/// variant-specific archetype and skill pack with modified effect chains.
pub struct WhiteHeroVariant {
    /// The white variant class ID (matches HeroClassFamily::white_id).
    pub class_id: &'static str,
    /// The base class ID for this family (e.g., "alchemist").
    pub base_class_id: &'static str,
    /// The display name for this variant.
    pub display_name: &'static str,
}

impl WhiteHeroVariant {
    /// Get the archetype for this white hero variant.
    ///
    /// White variants share the same base stats as their base counterparts
    /// but have a distinct archetype name for registry lookup.
    pub fn archetype(&self) -> Archetype {
        match self.class_id {
            "alchemist1" => heroes::white::alchemist_archetype(),
            "diviner1" => heroes::white::diviner_archetype(),
            "hunter1" => heroes::white::hunter_archetype(),
            "shaman1" => heroes::white::shaman_archetype(),
            "tank1" => heroes::white::tank_archetype(),
            _ => unreachable!("Unknown white hero class: {}", self.class_id),
        }
    }

    /// Get the skill pack for this white hero variant.
    ///
    /// Each white variant has exactly 7 skills matching the DDGC hero skill
    /// template, but with variant-specific effect chains that differ from base.
    pub fn skill_pack(&self) -> Vec<SkillDefinition> {
        match self.class_id {
            "alchemist1" => heroes::white::alchemist_skill_pack(),
            "diviner1" => heroes::white::diviner_skill_pack(),
            "hunter1" => heroes::white::hunter_skill_pack(),
            "shaman1" => heroes::white::shaman_skill_pack(),
            "tank1" => heroes::white::tank_skill_pack(),
            _ => unreachable!("Unknown white hero class: {}", self.class_id),
        }
    }

    /// Check if this white variant's class ID is a valid white variant in the registry.
    ///
    /// Returns true if the class_id maps to a known family via the white_id field.
    pub fn is_white_variant(&self, registry: &HeroFamilyRegistry) -> bool {
        registry
            .get_family_by_variant(self.class_id)
            .map(|f| f.white_id == self.class_id)
            .unwrap_or(false)
    }

    /// Get the corresponding base variant for this white variant.
    pub fn base_variant(&self) -> BaseHeroVariant {
        BaseHeroVariant {
            class_id: self.base_class_id,
            display_name: self.display_name.trim_end_matches(" (White)"),
        }
    }
}

/// All white hero variants for the 5 recruitable DDGC hero class families.
///
/// Each entry corresponds to a family in HERO_CLASS_FAMILIES.md.
pub fn all_white_variants() -> [WhiteHeroVariant; 5] {
    [
        WhiteHeroVariant {
            class_id: "alchemist1",
            base_class_id: "alchemist",
            display_name: "Alchemist (White)",
        },
        WhiteHeroVariant {
            class_id: "diviner1",
            base_class_id: "diviner",
            display_name: "Diviner (White)",
        },
        WhiteHeroVariant {
            class_id: "hunter1",
            base_class_id: "hunter",
            display_name: "Hunter (White)",
        },
        WhiteHeroVariant {
            class_id: "shaman1",
            base_class_id: "shaman",
            display_name: "Shaman (White)",
        },
        WhiteHeroVariant {
            class_id: "tank1",
            base_class_id: "tank",
            display_name: "Tank (White)",
        },
    ]
}
