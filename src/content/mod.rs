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

/// Bundles all migrated DDGC content into a single lookup structure.
///
/// The `ContentPack::default()` constructor registers every archetype,
/// skill, and status defined in this migration slice.
pub struct ContentPack {
    pub archetypes: HashMap<String, Archetype>,
    pub skills: HashMap<String, SkillDefinition>,
}

impl ContentPack {
    pub fn new() -> Self {
        ContentPack {
            archetypes: HashMap::new(),
            skills: HashMap::new(),
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

        pack
    }
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
}
