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

    // K9: Moth Mimicry B (US-410)
    registry.register(MonsterFamily {
        id: FamilyId::new("moth_mimicry_B"),
        dungeon: Dungeon::QingLong,
        tier: MonsterTier::Common,
        role: FamilyRole::Ranged,
        monster_type: MonsterType::Eldritch,
        skill_ids: vec![
            SkillId::new("poison"),
            SkillId::new("stress"),
            SkillId::new("stress_crowd"),
        ],
        archetype_name: "Moth Mimicry B".to_string(),
    });

    // K10: Robber Melee (US-411)
    registry.register(MonsterFamily {
        id: FamilyId::new("robber_melee"),
        dungeon: Dungeon::QingLong,
        tier: MonsterTier::Common,
        role: FamilyRole::Skirmisher,
        monster_type: MonsterType::Man,
        skill_ids: vec![
            SkillId::new("normal_attack"),
            SkillId::new("bleed"),
            SkillId::new("smoke_bomb"),
            SkillId::new("move"),
        ],
        archetype_name: "Robber Melee".to_string(),
    });

    // K11: Robber Ranged (US-412)
    registry.register(MonsterFamily {
        id: FamilyId::new("robber_ranged"),
        dungeon: Dungeon::QingLong,
        tier: MonsterTier::Common,
        role: FamilyRole::Skirmisher,
        monster_type: MonsterType::Man,
        skill_ids: vec![
            SkillId::new("normal_attack"),
            SkillId::new("multiple_shot"),
            SkillId::new("throw_stone"),
            SkillId::new("move"),
        ],
        archetype_name: "Robber Ranged".to_string(),
    });

    // K12: Metal Armor (US-413)
    registry.register(MonsterFamily {
        id: FamilyId::new("metal_armor"),
        dungeon: Dungeon::BaiHu,
        tier: MonsterTier::Common,
        role: FamilyRole::Tank,
        monster_type: MonsterType::Unholy,
        skill_ids: vec![
            SkillId::new("stun"),
            SkillId::new("bleed"),
            SkillId::new("normal_attack"),
            SkillId::new("move"),
        ],
        archetype_name: "Metal Armor".to_string(),
    });

    // K13: Tiger Sword (US-414)
    registry.register(MonsterFamily {
        id: FamilyId::new("tiger_sword"),
        dungeon: Dungeon::BaiHu,
        tier: MonsterTier::Common,
        role: FamilyRole::Bruiser,
        monster_type: MonsterType::Unholy,
        skill_ids: vec![
            SkillId::new("normal_attack"),
            SkillId::new("pull"),
            SkillId::new("move"),
        ],
        archetype_name: "Tiger Sword".to_string(),
    });

    // K14: Lizard (US-415)
    registry.register(MonsterFamily {
        id: FamilyId::new("lizard"),
        dungeon: Dungeon::BaiHu,
        tier: MonsterTier::Common,
        role: FamilyRole::Controller,
        monster_type: MonsterType::Eldritch,
        skill_ids: vec![
            SkillId::new("stun"),
            SkillId::new("intimidate"),
            SkillId::new("stress"),
            SkillId::new("move"),
        ],
        archetype_name: "Lizard".to_string(),
    });

    // K15: Unicorn Beetle A (US-416)
    registry.register(MonsterFamily {
        id: FamilyId::new("unicorn_beetle_A"),
        dungeon: Dungeon::BaiHu,
        tier: MonsterTier::Common,
        role: FamilyRole::Ranged,
        monster_type: MonsterType::Eldritch,
        skill_ids: vec![
            SkillId::new("normal_attack"),
            SkillId::new("bleed"),
            SkillId::new("bleed_crowd"),
            SkillId::new("move"),
        ],
        archetype_name: "Unicorn Beetle A".to_string(),
    });

    // K16: Unicorn Beetle B (US-417)
    registry.register(MonsterFamily {
        id: FamilyId::new("unicorn_beetle_B"),
        dungeon: Dungeon::BaiHu,
        tier: MonsterTier::Common,
        role: FamilyRole::Ranged,
        monster_type: MonsterType::Eldritch,
        skill_ids: vec![
            SkillId::new("normal_attack"),
            SkillId::new("bleed"),
            SkillId::new("stress"),
            SkillId::new("move"),
        ],
        archetype_name: "Unicorn Beetle B".to_string(),
    });

    // K17: Alligator Yangtze (US-418)
    registry.register(MonsterFamily {
        id: FamilyId::new("alligator_yangtze"),
        dungeon: Dungeon::BaiHu,
        tier: MonsterTier::Common,
        role: FamilyRole::Bruiser,
        monster_type: MonsterType::Beast,
        skill_ids: vec![
            SkillId::new("normal_attack"),
            SkillId::new("bleed"),
            SkillId::new("mark_riposte"),
            SkillId::new("riposte1"),
        ],
        archetype_name: "Alligator Yangtze".to_string(),
    });

    // K18: Ghost Fire Assist (US-419)
    registry.register(MonsterFamily {
        id: FamilyId::new("ghost_fire_assist"),
        dungeon: Dungeon::ZhuQue,
        tier: MonsterTier::Common,
        role: FamilyRole::Support,
        monster_type: MonsterType::Eldritch,
        skill_ids: vec![
            SkillId::new("assist"),
            SkillId::new("buff_self"),
            SkillId::new("ghost_fire_split"),
        ],
        archetype_name: "Ghost Fire Assist".to_string(),
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

    #[test]
    fn registry_resolves_moth_mimicry_b() {
        let registry = build_registry();

        let family = registry
            .get("moth_mimicry_B")
            .expect("moth_mimicry_B should be registered");

        assert_eq!(family.id.0, "moth_mimicry_B");
        assert_eq!(family.dungeon, Dungeon::QingLong);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Ranged);
        assert_eq!(family.monster_type, MonsterType::Eldritch);
        assert_eq!(family.archetype_name, "Moth Mimicry B");
        assert_eq!(family.skill_ids.len(), 3);
    }

    #[test]
    fn registry_moth_mimicry_b_has_poison_and_stress_crowd() {
        let registry = build_registry();

        let family = registry.get("moth_mimicry_B").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"poison"),
            "moth_mimicry_B must have poison skill"
        );
        assert!(
            skill_ids.contains(&"stress_crowd"),
            "moth_mimicry_B must have stress_crowd skill"
        );
    }

    #[test]
    fn registry_resolves_robber_melee() {
        let registry = build_registry();

        let family = registry
            .get("robber_melee")
            .expect("robber_melee should be registered");

        assert_eq!(family.id.0, "robber_melee");
        assert_eq!(family.dungeon, Dungeon::QingLong);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Skirmisher);
        assert_eq!(family.monster_type, MonsterType::Man);
        assert_eq!(family.archetype_name, "Robber Melee");
        assert_eq!(family.skill_ids.len(), 4);
    }

    #[test]
    fn registry_robber_melee_has_smoke_bomb_and_bleed() {
        let registry = build_registry();

        let family = registry.get("robber_melee").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"smoke_bomb"),
            "robber_melee must have smoke_bomb skill"
        );
        assert!(
            skill_ids.contains(&"bleed"),
            "robber_melee must have bleed skill"
        );
    }

    #[test]
    fn registry_resolves_robber_ranged() {
        let registry = build_registry();

        let family = registry
            .get("robber_ranged")
            .expect("robber_ranged should be registered");

        assert_eq!(family.id.0, "robber_ranged");
        assert_eq!(family.dungeon, Dungeon::QingLong);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Skirmisher);
        assert_eq!(family.monster_type, MonsterType::Man);
        assert_eq!(family.archetype_name, "Robber Ranged");
        assert_eq!(family.skill_ids.len(), 4);
    }

    #[test]
    fn registry_robber_ranged_has_throw_stone_and_multiple_shot() {
        let registry = build_registry();

        let family = registry.get("robber_ranged").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"throw_stone"),
            "robber_ranged must have throw_stone skill"
        );
        assert!(
            skill_ids.contains(&"multiple_shot"),
            "robber_ranged must have multiple_shot skill"
        );
    }

    #[test]
    fn registry_resolves_metal_armor() {
        let registry = build_registry();

        let family = registry
            .get("metal_armor")
            .expect("metal_armor should be registered");

        assert_eq!(family.id.0, "metal_armor");
        assert_eq!(family.dungeon, Dungeon::BaiHu);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Tank);
        assert_eq!(family.monster_type, MonsterType::Unholy);
        assert_eq!(family.archetype_name, "Metal Armor");
        assert_eq!(family.skill_ids.len(), 4);
    }

    #[test]
    fn registry_metal_armor_has_stun_and_bleed() {
        let registry = build_registry();

        let family = registry.get("metal_armor").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"stun"),
            "metal_armor must have stun skill"
        );
        assert!(
            skill_ids.contains(&"bleed"),
            "metal_armor must have bleed skill"
        );
    }

    #[test]
    fn registry_resolves_tiger_sword() {
        let registry = build_registry();

        let family = registry
            .get("tiger_sword")
            .expect("tiger_sword should be registered");

        assert_eq!(family.id.0, "tiger_sword");
        assert_eq!(family.dungeon, Dungeon::BaiHu);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Bruiser);
        assert_eq!(family.monster_type, MonsterType::Unholy);
        assert_eq!(family.archetype_name, "Tiger Sword");
        assert_eq!(family.skill_ids.len(), 3);
    }

    #[test]
    fn registry_tiger_sword_has_normal_attack_and_pull() {
        let registry = build_registry();

        let family = registry.get("tiger_sword").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"normal_attack"),
            "tiger_sword must have normal_attack skill"
        );
        assert!(
            skill_ids.contains(&"pull"),
            "tiger_sword must have pull skill"
        );
    }

    #[test]
    fn registry_resolves_lizard() {
        let registry = build_registry();

        let family = registry
            .get("lizard")
            .expect("lizard should be registered");

        assert_eq!(family.id.0, "lizard");
        assert_eq!(family.dungeon, Dungeon::BaiHu);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Controller);
        assert_eq!(family.monster_type, MonsterType::Eldritch);
        assert_eq!(family.archetype_name, "Lizard");
        assert_eq!(family.skill_ids.len(), 4);
    }

    #[test]
    fn registry_lizard_has_stun_and_intimidate_and_stress() {
        let registry = build_registry();

        let family = registry.get("lizard").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"stun"),
            "lizard must have stun skill"
        );
        assert!(
            skill_ids.contains(&"intimidate"),
            "lizard must have intimidate skill"
        );
        assert!(
            skill_ids.contains(&"stress"),
            "lizard must have stress skill"
        );
    }

    #[test]
    fn registry_resolves_unicorn_beetle_a() {
        let registry = build_registry();

        let family = registry
            .get("unicorn_beetle_A")
            .expect("unicorn_beetle_A should be registered");

        assert_eq!(family.id.0, "unicorn_beetle_A");
        assert_eq!(family.dungeon, Dungeon::BaiHu);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Ranged);
        assert_eq!(family.monster_type, MonsterType::Eldritch);
        assert_eq!(family.archetype_name, "Unicorn Beetle A");
        assert_eq!(family.skill_ids.len(), 4);
    }

    #[test]
    fn registry_unicorn_beetle_a_has_bleed_and_bleed_crowd() {
        let registry = build_registry();

        let family = registry.get("unicorn_beetle_A").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"bleed"),
            "unicorn_beetle_A must have bleed skill"
        );
        assert!(
            skill_ids.contains(&"bleed_crowd"),
            "unicorn_beetle_A must have bleed_crowd skill"
        );
    }

    #[test]
    fn registry_resolves_unicorn_beetle_b() {
        let registry = build_registry();

        let family = registry
            .get("unicorn_beetle_B")
            .expect("unicorn_beetle_B should be registered");

        assert_eq!(family.id.0, "unicorn_beetle_B");
        assert_eq!(family.dungeon, Dungeon::BaiHu);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Ranged);
        assert_eq!(family.monster_type, MonsterType::Eldritch);
        assert_eq!(family.archetype_name, "Unicorn Beetle B");
        assert_eq!(family.skill_ids.len(), 4);
    }

    #[test]
    fn registry_unicorn_beetle_b_has_bleed_and_stress() {
        let registry = build_registry();

        let family = registry.get("unicorn_beetle_B").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"bleed"),
            "unicorn_beetle_B must have bleed skill"
        );
        assert!(
            skill_ids.contains(&"stress"),
            "unicorn_beetle_B must have stress skill"
        );
    }

    #[test]
    fn registry_resolves_alligator_yangtze() {
        let registry = build_registry();

        let family = registry
            .get("alligator_yangtze")
            .expect("alligator_yangtze should be registered");

        assert_eq!(family.id.0, "alligator_yangtze");
        assert_eq!(family.dungeon, Dungeon::BaiHu);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Bruiser);
        assert_eq!(family.monster_type, MonsterType::Beast);
        assert_eq!(family.archetype_name, "Alligator Yangtze");
        assert_eq!(family.skill_ids.len(), 4);
    }

    #[test]
    fn registry_alligator_yangtze_has_bleed_and_riposte() {
        let registry = build_registry();

        let family = registry.get("alligator_yangtze").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"bleed"),
            "alligator_yangtze must have bleed skill"
        );
        assert!(
            skill_ids.contains(&"mark_riposte"),
            "alligator_yangtze must have mark_riposte skill"
        );
    }

    #[test]
    fn registry_resolves_ghost_fire_assist() {
        let registry = build_registry();

        let family = registry
            .get("ghost_fire_assist")
            .expect("ghost_fire_assist should be registered");

        assert_eq!(family.id.0, "ghost_fire_assist");
        assert_eq!(family.dungeon, Dungeon::ZhuQue);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Support);
        assert_eq!(family.monster_type, MonsterType::Eldritch);
        assert_eq!(family.archetype_name, "Ghost Fire Assist");
        assert_eq!(family.skill_ids.len(), 3);
    }

    #[test]
    fn registry_ghost_fire_assist_has_assist_and_ghost_fire_split() {
        let registry = build_registry();

        let family = registry.get("ghost_fire_assist").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"assist"),
            "ghost_fire_assist must have assist skill"
        );
        assert!(
            skill_ids.contains(&"ghost_fire_split"),
            "ghost_fire_assist must have ghost_fire_split skill"
        );
    }
}
