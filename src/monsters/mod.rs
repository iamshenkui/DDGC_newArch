//! Monster family module — registry and definitions for DDGC monster families.
//!
//! This module provides the family-aware view of DDGC monsters, where each
//! enemy is represented as a `MonsterFamily` with role, type, dungeon,
//! behavior profile, and associated skill IDs.
//!
//! The `families` submodule contains the registry with lookup by family ID
//! for archetype, role, skill-pack, and behavior-profile.
//!
//! The `build_registry()` function populates the registry with all migrated
//! families. Each migration slice adds a registration call there.

pub mod families;

pub use families::*;

/// Build the monster family registry with all migrated families.
///
/// Each migration slice (US-405 onwards) adds a `registry.register()` call
/// here. The registry grows incrementally as families are migrated.
pub fn build_registry() -> MonsterFamilyRegistry {
    let mut registry = MonsterFamilyRegistry::new();

    // K4: Mantis Magic Flower (US-405)
    registry.register(MonsterFamily {
        id: FamilyId::new("mantis_magic_flower"),
        dungeon: Dungeon::QingLong,
        tier: MonsterTier::Common,
        role: FamilyRole::Controller,
        monster_type: MonsterType::Beast,
        skill_ids: vec![
            SkillId::new("poison"),
            SkillId::new("crowd_bleed"),
            SkillId::new("normal_attack"),
            SkillId::new("move"),
        ],
        archetype_name: "Mantis Magic Flower".to_string(),
    });

    // K5: Mantis Spiny Flower (US-406)
    registry.register(MonsterFamily {
        id: FamilyId::new("mantis_spiny_flower"),
        dungeon: Dungeon::QingLong,
        tier: MonsterTier::Common,
        role: FamilyRole::Controller,
        monster_type: MonsterType::Beast,
        skill_ids: vec![
            SkillId::new("ignore_armor"),
            SkillId::new("crowd_bleed"),
            SkillId::new("normal_attack"),
            SkillId::new("move"),
        ],
        archetype_name: "Mantis Spiny Flower".to_string(),
    });

    // K6: Mantis Walking Flower (US-407)
    registry.register(MonsterFamily {
        id: FamilyId::new("mantis_walking_flower"),
        dungeon: Dungeon::QingLong,
        tier: MonsterTier::Common,
        role: FamilyRole::Controller,
        monster_type: MonsterType::Beast,
        skill_ids: vec![
            SkillId::new("weak"),
            SkillId::new("crowd_bleed"),
            SkillId::new("normal_attack"),
            SkillId::new("move"),
        ],
        archetype_name: "Mantis Walking Flower".to_string(),
    });

    // K7: Dry Tree Genie (US-408)
    registry.register(MonsterFamily {
        id: FamilyId::new("dry_tree_genie"),
        dungeon: Dungeon::QingLong,
        tier: MonsterTier::Common,
        role: FamilyRole::Ranged,
        monster_type: MonsterType::Eldritch,
        skill_ids: vec![
            SkillId::new("bleed"),
            SkillId::new("slow_crowd"),
            SkillId::new("stress"),
            SkillId::new("move"),
        ],
        archetype_name: "Dry Tree Genie".to_string(),
    });

    // K8: Moth Mimicry A (US-409)
    registry.register(MonsterFamily {
        id: FamilyId::new("moth_mimicry_A"),
        dungeon: Dungeon::QingLong,
        tier: MonsterTier::Common,
        role: FamilyRole::Ranged,
        monster_type: MonsterType::Eldritch,
        skill_ids: vec![
            SkillId::new("normal_attack"),
            SkillId::new("poison"),
            SkillId::new("stress_poison"),
        ],
        archetype_name: "Moth Mimicry A".to_string(),
    });

    registry
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_resolves_mantis_magic_flower() {
        let registry = build_registry();

        let family = registry
            .get("mantis_magic_flower")
            .expect("mantis_magic_flower should be registered");

        assert_eq!(family.id.0, "mantis_magic_flower");
        assert_eq!(family.dungeon, Dungeon::QingLong);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Controller);
        assert_eq!(family.monster_type, MonsterType::Beast);
        assert_eq!(family.archetype_name, "Mantis Magic Flower");
        assert_eq!(family.skill_ids.len(), 4);
    }

    #[test]
    fn registry_mantis_magic_flower_has_poison_and_crowd_bleed() {
        let registry = build_registry();

        let family = registry.get("mantis_magic_flower").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"poison"),
            "mantis_magic_flower must have poison skill"
        );
        assert!(
            skill_ids.contains(&"crowd_bleed"),
            "mantis_magic_flower must have crowd_bleed skill"
        );
    }

    #[test]
    fn registry_resolves_mantis_spiny_flower() {
        let registry = build_registry();

        let family = registry
            .get("mantis_spiny_flower")
            .expect("mantis_spiny_flower should be registered");

        assert_eq!(family.id.0, "mantis_spiny_flower");
        assert_eq!(family.dungeon, Dungeon::QingLong);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Controller);
        assert_eq!(family.monster_type, MonsterType::Beast);
        assert_eq!(family.archetype_name, "Mantis Spiny Flower");
        assert_eq!(family.skill_ids.len(), 4);
    }

    #[test]
    fn registry_mantis_spiny_flower_has_ignore_armor_and_crowd_bleed() {
        let registry = build_registry();

        let family = registry.get("mantis_spiny_flower").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"ignore_armor"),
            "mantis_spiny_flower must have ignore_armor skill"
        );
        assert!(
            skill_ids.contains(&"crowd_bleed"),
            "mantis_spiny_flower must have crowd_bleed skill"
        );
    }

    #[test]
    fn registry_resolves_mantis_walking_flower() {
        let registry = build_registry();

        let family = registry
            .get("mantis_walking_flower")
            .expect("mantis_walking_flower should be registered");

        assert_eq!(family.id.0, "mantis_walking_flower");
        assert_eq!(family.dungeon, Dungeon::QingLong);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Controller);
        assert_eq!(family.monster_type, MonsterType::Beast);
        assert_eq!(family.archetype_name, "Mantis Walking Flower");
        assert_eq!(family.skill_ids.len(), 4);
    }

    #[test]
    fn registry_mantis_walking_flower_has_weak_and_crowd_bleed() {
        let registry = build_registry();

        let family = registry.get("mantis_walking_flower").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"weak"),
            "mantis_walking_flower must have weak skill"
        );
        assert!(
            skill_ids.contains(&"crowd_bleed"),
            "mantis_walking_flower must have crowd_bleed skill"
        );
    }

    #[test]
    fn registry_resolves_dry_tree_genie() {
        let registry = build_registry();

        let family = registry
            .get("dry_tree_genie")
            .expect("dry_tree_genie should be registered");

        assert_eq!(family.id.0, "dry_tree_genie");
        assert_eq!(family.dungeon, Dungeon::QingLong);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Ranged);
        assert_eq!(family.monster_type, MonsterType::Eldritch);
        assert_eq!(family.archetype_name, "Dry Tree Genie");
        assert_eq!(family.skill_ids.len(), 4);
    }

    #[test]
    fn registry_dry_tree_genie_has_bleed_and_slow_crowd_and_stress() {
        let registry = build_registry();

        let family = registry.get("dry_tree_genie").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"bleed"),
            "dry_tree_genie must have bleed skill"
        );
        assert!(
            skill_ids.contains(&"slow_crowd"),
            "dry_tree_genie must have slow_crowd skill"
        );
        assert!(
            skill_ids.contains(&"stress"),
            "dry_tree_genie must have stress skill"
        );
    }

    #[test]
    fn registry_resolves_moth_mimicry_a() {
        let registry = build_registry();

        let family = registry
            .get("moth_mimicry_A")
            .expect("moth_mimicry_A should be registered");

        assert_eq!(family.id.0, "moth_mimicry_A");
        assert_eq!(family.dungeon, Dungeon::QingLong);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Ranged);
        assert_eq!(family.monster_type, MonsterType::Eldritch);
        assert_eq!(family.archetype_name, "Moth Mimicry A");
        assert_eq!(family.skill_ids.len(), 3);
    }

    #[test]
    fn registry_moth_mimicry_a_has_poison_and_stress_poison() {
        let registry = build_registry();

        let family = registry.get("moth_mimicry_A").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"poison"),
            "moth_mimicry_A must have poison skill"
        );
        assert!(
            skill_ids.contains(&"stress_poison"),
            "moth_mimicry_A must have stress_poison skill"
        );
    }
}
