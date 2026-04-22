//! Base hero variant definitions for all DDGC recruitable hero class families.
//!
//! Each recruitable DDGC hero profession has a base variant that is available
//! through the StageCoach. The base variant uses the family's base class ID
//! (no suffix) and provides the normal-mode archetype and skill pack.
//!
//! Base variants are the recruitable form — DDGC's StageCoach only recruits
//! heroes using base class IDs. White (+1) and black (+2) variants are only
//! activated through chaos mode switching.

use crate::content::actors::Archetype;
use crate::content::heroes;
use crate::contracts::TrinketDefinition;
use crate::heroes::families::HeroFamilyRegistry;
use crate::heroes::stats::compute_hero_stats;
use framework_combat::skills::SkillDefinition;

/// A base hero variant: the recruitable form of a hero class family.
///
/// Each recruitable DDGC hero profession has a base variant that is
/// available through the StageCoach. The base variant uses the family's
/// base class ID (no suffix) and provides the normal-mode archetype and skill pack.
pub struct BaseHeroVariant {
    /// The base class ID (matches HeroClassFamily::base_id).
    pub class_id: &'static str,
    /// The display name for this hero.
    pub display_name: &'static str,
}

impl BaseHeroVariant {
    /// Get the archetype for this base hero variant (level-0, no equipment).
    ///
    /// This is the original backward-compatible method that returns hardcoded
    /// level-0 stats with no equipment bonuses.
    pub fn archetype(&self) -> Archetype {
        match self.class_id {
            "alchemist" => heroes::alchemist::archetype(),
            "diviner" => heroes::diviner::archetype(),
            "hunter" => heroes::hunter::archetype(),
            "shaman" => heroes::shaman::archetype(),
            "tank" => heroes::tank::archetype(),
            _ => unreachable!("Unknown base hero class: {}", self.class_id),
        }
    }

    /// Get the archetype for this base hero variant with equipment bonuses.
    ///
    /// # Parameters
    /// - `weapon_level`: Weapon upgrade level (0 = base, no bonus)
    /// - `armor_level`: Armor upgrade level (0 = base, no bonus)
    /// - `trinkets`: Slice of equipped trinket definitions (empty if none)
    ///
    /// When all parameters are 0/empty, this produces identical results to `archetype()`.
    pub fn archetype_with_equipment(
        &self,
        weapon_level: u32,
        armor_level: u32,
        trinkets: &[&TrinketDefinition],
    ) -> Archetype {
        // If no equipment upgrades, use the original hardcoded archetype
        if weapon_level == 0 && armor_level == 0 && trinkets.is_empty() {
            return self.archetype();
        }

        // Otherwise, compute stats from equipment + trinkets
        compute_hero_stats(self.class_id, weapon_level, armor_level, trinkets)
    }

    /// Get the skill pack for this base hero variant.
    ///
    /// Each base variant has exactly 7 skills matching the DDGC hero skill template.
    pub fn skill_pack(&self) -> Vec<SkillDefinition> {
        match self.class_id {
            "alchemist" => heroes::alchemist::skill_pack(),
            "diviner" => heroes::diviner::skill_pack(),
            "hunter" => heroes::hunter::skill_pack(),
            "shaman" => heroes::shaman::skill_pack(),
            "tank" => heroes::tank::skill_pack(),
            _ => unreachable!("Unknown base hero class: {}", self.class_id),
        }
    }

    /// Check if this base hero variant is recruitable.
    ///
    /// A hero is recruitable if their class ID is a base ID (no suffix)
    /// and exists in the hero family registry. This mirrors DDGC's
    /// `CharacterHelper.IsBaseRecruitHeroClass` check.
    pub fn is_recruitable(&self, registry: &HeroFamilyRegistry) -> bool {
        registry.get_family_by_base(self.class_id).is_some()
    }
}

/// All base hero variants for the 5 recruitable DDGC hero class families.
///
/// Each entry corresponds to a family in HERO_CLASS_FAMILIES.md.
pub fn all_base_variants() -> [BaseHeroVariant; 5] {
    [
        BaseHeroVariant {
            class_id: "alchemist",
            display_name: "Alchemist",
        },
        BaseHeroVariant {
            class_id: "diviner",
            display_name: "Diviner",
        },
        BaseHeroVariant {
            class_id: "hunter",
            display_name: "Hunter",
        },
        BaseHeroVariant {
            class_id: "shaman",
            display_name: "Shaman",
        },
        BaseHeroVariant {
            class_id: "tank",
            display_name: "Tank",
        },
    ]
}
