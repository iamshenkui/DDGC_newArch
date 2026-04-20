//! DDGC migrated content — actors, skills, statuses, and hero content.
//!
//! All DDGC-specific content lives here. No DDGC constants go into
//! framework crates. This module provides factory functions and a
//! `ContentPack` that bundles everything the game layer needs.

pub mod actors;
pub mod heroes;
pub mod monsters;
pub mod skills;
pub mod statuses;

use framework_combat::skills::{SkillDefinition, SkillId};
use std::collections::HashMap;

use crate::content::actors::Archetype;
use crate::run::damage_policy::DamageRange;

/// Bundles all migrated DDGC content into a single lookup structure.
///
/// The `ContentPack::default()` constructor registers every archetype,
/// skill, and status defined in this migration slice.
pub struct ContentPack {
    pub archetypes: HashMap<String, Archetype>,
    pub skills: HashMap<String, SkillDefinition>,
    /// Damage ranges for skills that deal damage.
    /// Maps skill ID to DamageRange with DDGC min/max values.
    pub damage_ranges: HashMap<String, DamageRange>,
}

impl ContentPack {
    pub fn new() -> Self {
        ContentPack {
            archetypes: HashMap::new(),
            skills: HashMap::new(),
            damage_ranges: HashMap::new(),
        }
    }

    pub fn register_archetype(&mut self, archetype: Archetype) {
        self.archetypes.insert(archetype.name.0.clone(), archetype);
    }

    pub fn register_skill(&mut self, skill: SkillDefinition) {
        self.skills.insert(skill.id.0.clone(), skill);
    }

    pub fn get_skill(&self, id: &SkillId) -> Option<&SkillDefinition> {
        self.skills.get(&id.0)
    }

    pub fn get_archetype(&self, name: &str) -> Option<&Archetype> {
        self.archetypes.get(name)
    }

    /// Register a damage range for a skill.
    pub fn register_damage_range(&mut self, skill_id: &str, range: DamageRange) {
        self.damage_ranges.insert(skill_id.to_string(), range);
    }

    /// Get the damage range for a skill, if one exists.
    pub fn get_damage_range(&self, skill_id: &str) -> Option<DamageRange> {
        self.damage_ranges.get(skill_id).copied()
    }
}

impl Default for ContentPack {
    fn default() -> Self {
        let mut pack = ContentPack::new();

        // Ally archetypes — legacy tutorial heroes
        pack.register_archetype(actors::crusader());
        pack.register_archetype(actors::vestal());

        // Ally archetypes — recruitable hero class families (base variants)
        pack.register_archetype(heroes::alchemist::archetype());
        pack.register_archetype(heroes::diviner::archetype());
        pack.register_archetype(heroes::hunter::archetype());
        pack.register_archetype(heroes::shaman::archetype());
        pack.register_archetype(heroes::tank::archetype());

        // Ally archetypes — recruitable hero class families (white variants)
        pack.register_archetype(heroes::white::alchemist_archetype());
        pack.register_archetype(heroes::white::diviner_archetype());
        pack.register_archetype(heroes::white::hunter_archetype());
        pack.register_archetype(heroes::white::shaman_archetype());
        pack.register_archetype(heroes::white::tank_archetype());

        // Ally archetypes — recruitable hero class families (black variants)
        pack.register_archetype(heroes::black::alchemist_archetype());
        pack.register_archetype(heroes::black::diviner_archetype());
        pack.register_archetype(heroes::black::hunter_archetype());
        pack.register_archetype(heroes::black::shaman_archetype());
        pack.register_archetype(heroes::black::tank_archetype());

        // Enemy archetypes
        pack.register_archetype(actors::bone_soldier());
        pack.register_archetype(actors::necromancer());

        // Monster family content — each migration slice adds a registration here
        monsters::register_content(&mut pack);

        // Skills — legacy tutorial skills
        pack.register_skill(skills::crusading_strike());
        pack.register_skill(skills::holy_lance());
        pack.register_skill(skills::divine_grace());
        pack.register_skill(skills::rend());
        pack.register_skill(skills::skull_bash());
        pack.register_skill(skills::grave_bash());

        // Skills — Alchemist base skill pack
        for skill in heroes::alchemist::skill_pack() {
            pack.register_skill(skill);
        }

        // Skills — Diviner base skill pack
        for skill in heroes::diviner::skill_pack() {
            pack.register_skill(skill);
        }

        // Skills — Hunter base skill pack
        for skill in heroes::hunter::skill_pack() {
            pack.register_skill(skill);
        }

        // Skills — Shaman base skill pack
        for skill in heroes::shaman::skill_pack() {
            pack.register_skill(skill);
        }

        // Skills — Tank base skill pack
        for skill in heroes::tank::skill_pack() {
            pack.register_skill(skill);
        }

        // Skills — Alchemist white skill pack
        for skill in heroes::white::alchemist_skill_pack() {
            pack.register_skill(skill);
        }

        // Skills — Diviner white skill pack
        for skill in heroes::white::diviner_skill_pack() {
            pack.register_skill(skill);
        }

        // Skills — Hunter white skill pack
        for skill in heroes::white::hunter_skill_pack() {
            pack.register_skill(skill);
        }

        // Skills — Shaman white skill pack
        for skill in heroes::white::shaman_skill_pack() {
            pack.register_skill(skill);
        }

        // Skills — Tank white skill pack
        for skill in heroes::white::tank_skill_pack() {
            pack.register_skill(skill);
        }

        // Skills — Alchemist black skill pack
        for skill in heroes::black::alchemist_skill_pack() {
            pack.register_skill(skill);
        }

        // Skills — Diviner black skill pack
        for skill in heroes::black::diviner_skill_pack() {
            pack.register_skill(skill);
        }

        // Skills — Hunter black skill pack
        for skill in heroes::black::hunter_skill_pack() {
            pack.register_skill(skill);
        }

        // Skills — Shaman black skill pack
        for skill in heroes::black::shaman_skill_pack() {
            pack.register_skill(skill);
        }

        // Skills — Tank black skill pack
        for skill in heroes::black::tank_skill_pack() {
            pack.register_skill(skill);
        }

        // Damage ranges — DDGC min/max values for migrated skills that deal damage.
        // These ranges are verified against DDGC source data.
        // See MIGRATION_BLOCKERS.md B-006 for context on why ranges are stored separately.
        register_skill_damage_ranges(&mut pack);

        pack
    }
}

/// Register DDGC-verified damage ranges for migrated skills.
///
/// These ranges are extracted from DDGC source data comments in skill definitions.
/// All values are verified against DDGC original data.
fn register_skill_damage_ranges(pack: &mut ContentPack) {
    // Legacy tutorial skills (from skills.rs)
    // DDGC references are in the skill function comments.
    pack.register_damage_range("crusading_strike", DamageRange::new(8.0, 15.0));
    pack.register_damage_range("holy_lance", DamageRange::new(6.0, 12.0));
    pack.register_damage_range("rend", DamageRange::new(4.0, 8.0));
    pack.register_damage_range("skull_bash", DamageRange::new(10.0, 18.0));
    pack.register_damage_range("grave_bash", DamageRange::new(3.0, 7.0));

    // Mantis Walking Flower skills (from monsters/mantis_walking_flower.rs)
    // DDGC reference: dmg 10–14 (avg 12)
    pack.register_damage_range("weak", DamageRange::new(10.0, 14.0));
    pack.register_damage_range("crowd_bleed", DamageRange::new(10.0, 14.0));
    // DDGC reference: dmg 30–42 (avg 36)
    pack.register_damage_range("normal_attack", DamageRange::new(30.0, 42.0));

    // White Tiger A skills (from monsters/white_tiger_a.rs)
    // DDGC reference: dmg 6-8 (avg 7)
    pack.register_damage_range("drag", DamageRange::new(6.0, 8.0));
    pack.register_damage_range("pounce", DamageRange::new(6.0, 8.0));
    // DDGC reference: dmg 3-4 (avg 3.5)
    pack.register_damage_range("pounce_bite", DamageRange::new(3.0, 4.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_pack_contains_all_content() {
        let pack = ContentPack::default();

        // Archetypes
        assert!(pack.get_archetype("Crusader").is_some(), "Crusader missing");
        assert!(pack.get_archetype("Vestal").is_some(), "Vestal missing");
        assert!(pack.get_archetype("Bone Soldier").is_some(), "Bone Soldier missing");
        assert!(pack.get_archetype("Necromancer").is_some(), "Necromancer missing");

        // Skills
        assert!(pack.get_skill(&SkillId::new("crusading_strike")).is_some());
        assert!(pack.get_skill(&SkillId::new("holy_lance")).is_some());
        assert!(pack.get_skill(&SkillId::new("divine_grace")).is_some());
        assert!(pack.get_skill(&SkillId::new("rend")).is_some());
        assert!(pack.get_skill(&SkillId::new("skull_bash")).is_some());
        assert!(pack.get_skill(&SkillId::new("grave_bash")).is_some());
    }

    #[test]
    fn all_skills_validate() {
        let pack = ContentPack::default();
        for (name, skill) in &pack.skills {
            assert!(skill.validate().is_ok(), "Skill '{}' failed validation", name);
        }
    }

    #[test]
    fn migrated_skills_have_damage_ranges_from_ddgc_source() {
        // US-806-a acceptance: At least 5 migrated skills have verified
        // min/max values matching DDGC source data.
        let pack = ContentPack::default();

        // Verify crusading_strike: DDGC range (8–15)
        let range = pack.get_damage_range("crusading_strike");
        assert!(range.is_some(), "crusading_strike should have damage range");
        let r = range.unwrap();
        assert_eq!(r.min, 8.0, "crusading_strike min");
        assert_eq!(r.max, 15.0, "crusading_strike max");
        // Note: DamageRange average is (8+15)/2 = 11.5, which may differ from
        // the pre-averaged EffectNode::damage(12.0) value due to rounding in the original.

        // Verify rend: DDGC range (4–8)
        let range = pack.get_damage_range("rend");
        assert!(range.is_some(), "rend should have damage range");
        let r = range.unwrap();
        assert_eq!(r.min, 4.0, "rend min");
        assert_eq!(r.max, 8.0, "rend max");
        assert_eq!(r.average, 6.0, "rend average");

        // Verify skull_bash: DDGC range (10–18)
        let range = pack.get_damage_range("skull_bash");
        assert!(range.is_some(), "skull_bash should have damage range");
        let r = range.unwrap();
        assert_eq!(r.min, 10.0, "skull_bash min");
        assert_eq!(r.max, 18.0, "skull_bash max");
        assert_eq!(r.average, 14.0, "skull_bash average");

        // Verify grave_bash: DDGC range (3–7)
        let range = pack.get_damage_range("grave_bash");
        assert!(range.is_some(), "grave_bash should have damage range");
        let r = range.unwrap();
        assert_eq!(r.min, 3.0, "grave_bash min");
        assert_eq!(r.max, 7.0, "grave_bash max");
        assert_eq!(r.average, 5.0, "grave_bash average");

        // Verify weak (mantis_walking_flower): DDGC range (10–14)
        let range = pack.get_damage_range("weak");
        assert!(range.is_some(), "weak should have damage range");
        let r = range.unwrap();
        assert_eq!(r.min, 10.0, "weak min");
        assert_eq!(r.max, 14.0, "weak max");
        assert_eq!(r.average, 12.0, "weak average");

        // Verify normal_attack (mantis_walking_flower): DDGC range (30–42)
        let range = pack.get_damage_range("normal_attack");
        assert!(range.is_some(), "normal_attack should have damage range");
        let r = range.unwrap();
        assert_eq!(r.min, 30.0, "normal_attack min");
        assert_eq!(r.max, 42.0, "normal_attack max");
        assert_eq!(r.average, 36.0, "normal_attack average");
    }

    #[test]
    fn damage_ranges_are_lookupable_by_skill_id() {
        // Verify the lookup/mapping exists from skill ID to DamageRange
        let pack = ContentPack::default();

        // Should return Some for skills with damage ranges
        assert!(pack.get_damage_range("crusading_strike").is_some());
        assert!(pack.get_damage_range("rend").is_some());
        assert!(pack.get_damage_range("skull_bash").is_some());

        // Should return None for skills without registered damage ranges
        // (skills that only heal, apply status, or don't have explicit DDGC ranges)
        assert!(pack.get_damage_range("divine_grace").is_none(), "heal-only skill should not have damage range");
        assert!(pack.get_damage_range("move").is_none(), "movement-only skill should not have damage range");
    }

    // ── US-808-c: SG-001 variance-restored acceptance tests ──────────────

    #[test]
    fn all_damage_ranges_resolve_with_fixed_average_policy() {
        // SG-001 acceptance: FixedAverage policy returns the pre-computed average
        // for every registered damage range, proving deterministic resolution.
        use crate::run::damage_policy::DamagePolicy;
        let pack = ContentPack::default();
        for (skill_id, range) in &pack.damage_ranges {
            let resolved = DamagePolicy::FixedAverage.resolve(*range, 1, skill_id);
            assert_eq!(
                resolved, range.average,
                "FixedAverage for '{}' should return {}, got {}",
                skill_id, range.average, resolved
            );
        }
    }

    #[test]
    fn all_damage_ranges_resolve_with_rolled_policy_in_range() {
        // SG-001 acceptance: Rolled policy returns a value within [min, max]
        // for every registered damage range, proving variance is available.
        use crate::run::damage_policy::DamagePolicy;
        let pack = ContentPack::default();
        for (skill_id, range) in &pack.damage_ranges {
            for actor_id in 0..50u64 {
                let resolved = DamagePolicy::Rolled.resolve(*range, actor_id, skill_id);
                assert!(
                    (range.min..=range.max).contains(&resolved),
                    "Rolled for '{}' actor={} returned {}, outside [{}, {}]",
                    skill_id, actor_id, resolved, range.min, range.max
                );
            }
        }
    }

    #[test]
    fn rolled_policy_produces_variance_across_registered_ranges() {
        // SG-001 acceptance: For non-fixed damage ranges, the Rolled policy
        // produces different values for different actor IDs, proving variance
        // is actually restored (not just in-range but varying).
        use crate::run::damage_policy::DamagePolicy;
        let pack = ContentPack::default();
        for (skill_id, range) in &pack.damage_ranges {
            if range.is_fixed() {
                continue;
            }
            // Sample multiple actors and assert at least two values differ.
            let values: Vec<f64> = (0..20u64)
                .map(|actor| DamagePolicy::Rolled.resolve(*range, actor, skill_id))
                .collect();
            let has_variance = values.iter().any(|v| (*v - values[0]).abs() > f64::EPSILON);
            assert!(
                has_variance,
                "Rolled policy for '{}' produced no variance across 20 actors",
                skill_id
            );
        }
    }
}
