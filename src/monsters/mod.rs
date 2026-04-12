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

    // K19: Ghost Fire Damage (US-420)
    registry.register(MonsterFamily {
        id: FamilyId::new("ghost_fire_damage"),
        dungeon: Dungeon::ZhuQue,
        tier: MonsterTier::Common,
        role: FamilyRole::Ranged,
        monster_type: MonsterType::Eldritch,
        skill_ids: vec![
            SkillId::new("stress"),
            SkillId::new("burn_attack"),
            SkillId::new("ghost_fire_split"),
        ],
        archetype_name: "Ghost Fire Damage".to_string(),
    });

    // K20: Fox Fire (US-421)
    registry.register(MonsterFamily {
        id: FamilyId::new("fox_fire"),
        dungeon: Dungeon::ZhuQue,
        tier: MonsterTier::Common,
        role: FamilyRole::Bruiser,
        monster_type: MonsterType::Beast,
        skill_ids: vec![
            SkillId::new("bite"),
            SkillId::new("vomit"),
            SkillId::new("protect"),
            SkillId::new("move"),
        ],
        archetype_name: "Fox Fire".to_string(),
    });

    // K21: Moth Fire (US-422)
    registry.register(MonsterFamily {
        id: FamilyId::new("moth_fire"),
        dungeon: Dungeon::ZhuQue,
        tier: MonsterTier::Common,
        role: FamilyRole::Ranged,
        monster_type: MonsterType::Eldritch,
        skill_ids: vec![
            SkillId::new("stress_attack"),
            SkillId::new("cocoon"),
            SkillId::new("fly_into_fire"),
        ],
        archetype_name: "Moth Fire".to_string(),
    });

    // K22: Lantern (US-423)
    registry.register(MonsterFamily {
        id: FamilyId::new("lantern"),
        dungeon: Dungeon::ZhuQue,
        tier: MonsterTier::Common,
        role: FamilyRole::Ranged,
        monster_type: MonsterType::Eldritch,
        skill_ids: vec![
            SkillId::new("stress"),
            SkillId::new("burn_attack"),
        ],
        archetype_name: "Lantern".to_string(),
    });

    // K23: Snake Water (US-424)
    registry.register(MonsterFamily {
        id: FamilyId::new("snake_water"),
        dungeon: Dungeon::XuanWu,
        tier: MonsterTier::Common,
        role: FamilyRole::Controller,
        monster_type: MonsterType::Eldritch,
        skill_ids: vec![
            SkillId::new("stun"),
            SkillId::new("poison_fang"),
            SkillId::new("move"),
        ],
        archetype_name: "Snake Water".to_string(),
    });

    // K24: Water Grass (US-425)
    registry.register(MonsterFamily {
        id: FamilyId::new("water_grass"),
        dungeon: Dungeon::XuanWu,
        tier: MonsterTier::Common,
        role: FamilyRole::Controller,
        monster_type: MonsterType::Eldritch,
        skill_ids: vec![
            SkillId::new("stun"),
            SkillId::new("puncture"),
            SkillId::new("attack_crowd"),
            SkillId::new("convolve"),
            SkillId::new("move"),
        ],
        archetype_name: "Water Grass".to_string(),
    });

    // K25: Monkey Water (US-426)
    registry.register(MonsterFamily {
        id: FamilyId::new("monkey_water"),
        dungeon: Dungeon::XuanWu,
        tier: MonsterTier::Common,
        role: FamilyRole::Bruiser,
        monster_type: MonsterType::Unholy,
        skill_ids: vec![
            SkillId::new("base_melee"),
            SkillId::new("rush"),
            SkillId::new("stress"),
            SkillId::new("move"),
        ],
        archetype_name: "Monkey Water".to_string(),
    });

    // K29: Azure Dragon (US-430)
    registry.register(MonsterFamily {
        id: FamilyId::new("azure_dragon"),
        dungeon: Dungeon::QingLong,
        tier: MonsterTier::Boss,
        role: FamilyRole::Summoner,
        monster_type: MonsterType::Beast,
        skill_ids: vec![
            SkillId::new("bloodscale_reaping"),
            SkillId::new("dragonfear_crash"),
            SkillId::new("summit_relocation"),
            SkillId::new("soulfog_enthrall"),
            SkillId::new("dragoncry_storm"),
            SkillId::new("volt_tyranny"),
            SkillId::new("voltic_baptism"),
            SkillId::new("capricious_skies"),
            SkillId::new("swap_dragon_ball"),
            SkillId::new("swap_dragon_ball_other"),
            SkillId::new("swap_dragon_ball_summon"),
        ],
        archetype_name: "Azure Dragon".to_string(),
    });

    // K29: Azure Dragon Ball Thunder (US-430)
    registry.register(MonsterFamily {
        id: FamilyId::new("azure_dragon_ball_thunder"),
        dungeon: Dungeon::QingLong,
        tier: MonsterTier::Boss,
        role: FamilyRole::Support,
        monster_type: MonsterType::Eldritch,
        skill_ids: vec![
            SkillId::new("thunder_buff_magic"),
            SkillId::new("thunder_buff_stress"),
            SkillId::new("thunder_stress_attack"),
        ],
        archetype_name: "Azure Dragon Ball Thunder".to_string(),
    });

    // K29: Azure Dragon Ball Wind (US-430)
    registry.register(MonsterFamily {
        id: FamilyId::new("azure_dragon_ball_wind"),
        dungeon: Dungeon::QingLong,
        tier: MonsterTier::Boss,
        role: FamilyRole::Support,
        monster_type: MonsterType::Eldritch,
        skill_ids: vec![
            SkillId::new("wind_buff_acc"),
            SkillId::new("wind_shuffle"),
            SkillId::new("wind_buff_physic"),
        ],
        archetype_name: "Azure Dragon Ball Wind".to_string(),
    });

    // K30: Vermilion Bird (US-431)
    registry.register(MonsterFamily {
        id: FamilyId::new("vermilion_bird"),
        dungeon: Dungeon::ZhuQue,
        tier: MonsterTier::Boss,
        role: FamilyRole::Summoner,
        monster_type: MonsterType::Beast,
        skill_ids: vec![
            SkillId::new("singing_loudly"),
            SkillId::new("ruin"),
            SkillId::new("ruin1"),
            SkillId::new("precise_pecking"),
            SkillId::new("iron_feather"),
            SkillId::new("bide"),
            SkillId::new("calm_nerves"),
            SkillId::new("explosion"),
        ],
        archetype_name: "Vermilion Bird".to_string(),
    });

    // K30: Vermilion Bird Tail A (US-431)
    registry.register(MonsterFamily {
        id: FamilyId::new("vermilion_bird_tail_A"),
        dungeon: Dungeon::ZhuQue,
        tier: MonsterTier::Boss,
        role: FamilyRole::Support,
        monster_type: MonsterType::Beast,
        skill_ids: vec![
            SkillId::new("deterrence"),
            SkillId::new("confused"),
            SkillId::new("ignore_def"),
            SkillId::new("protect"),
        ],
        archetype_name: "Vermilion Bird Tail A".to_string(),
    });

    // K30: Vermilion Bird Tail B (US-431)
    registry.register(MonsterFamily {
        id: FamilyId::new("vermilion_bird_tail_B"),
        dungeon: Dungeon::ZhuQue,
        tier: MonsterTier::Boss,
        role: FamilyRole::Controller,
        monster_type: MonsterType::Beast,
        skill_ids: vec![
            SkillId::new("follow"),
            SkillId::new("follow1"),
            SkillId::new("run_water"),
            SkillId::new("run_water1"),
            SkillId::new("heaven_falls"),
            SkillId::new("heaven_falls1"),
            SkillId::new("iron_feather_with"),
        ],
        archetype_name: "Vermilion Bird Tail B".to_string(),
    });

    // K31: White Tiger C (US-432)
    registry.register(MonsterFamily {
        id: FamilyId::new("white_tiger_C"),
        dungeon: Dungeon::BaiHu,
        tier: MonsterTier::Boss,
        role: FamilyRole::Summoner,
        monster_type: MonsterType::Beast,
        skill_ids: vec![
            SkillId::new("thunder_lightning"),
            SkillId::new("paw"),
            SkillId::new("raging_fire"),
            SkillId::new("true_strike"),
            SkillId::new("jump"),
            SkillId::new("deter_stress"),
            SkillId::new("deter_def"),
        ],
        archetype_name: "White Tiger C".to_string(),
    });

    // K31: White Tiger A (US-432)
    registry.register(MonsterFamily {
        id: FamilyId::new("white_tiger_A"),
        dungeon: Dungeon::BaiHu,
        tier: MonsterTier::Boss,
        role: FamilyRole::Support,
        monster_type: MonsterType::Beast,
        skill_ids: vec![
            SkillId::new("drag"),
            SkillId::new("angry_eyes"),
            SkillId::new("pounce"),
            SkillId::new("pounce_bite"),
            SkillId::new("jump"),
        ],
        archetype_name: "White Tiger A".to_string(),
    });

    // K31: White Tiger B (US-432)
    registry.register(MonsterFamily {
        id: FamilyId::new("white_tiger_B"),
        dungeon: Dungeon::BaiHu,
        tier: MonsterTier::Boss,
        role: FamilyRole::Controller,
        monster_type: MonsterType::Beast,
        skill_ids: vec![
            SkillId::new("allow_return"),
            SkillId::new("fire_soul_shadow"),
            SkillId::new("tiger_swing"),
            SkillId::new("thunder_shadow"),
            SkillId::new("jump"),
        ],
        archetype_name: "White Tiger B".to_string(),
    });

    // K31: White Tiger Terrain (US-432)
    registry.register(MonsterFamily {
        id: FamilyId::new("white_tiger_terrain"),
        dungeon: Dungeon::BaiHu,
        tier: MonsterTier::Boss,
        role: FamilyRole::Support,
        monster_type: MonsterType::Corpse,
        skill_ids: vec![
            SkillId::new("occupy"),
        ],
        archetype_name: "White Tiger Terrain".to_string(),
    });

    // K32: Black Tortoise A (US-433)
    registry.register(MonsterFamily {
        id: FamilyId::new("black_tortoise_A"),
        dungeon: Dungeon::XuanWu,
        tier: MonsterTier::Boss,
        role: FamilyRole::Tank,
        monster_type: MonsterType::Beast,
        skill_ids: vec![
            SkillId::new("tortoise_call_roll"),
            SkillId::new("tortoise_rain_spray"),
            SkillId::new("ice_spike"),
            SkillId::new("hunger_cold"),
            SkillId::new("inner_battle"),
            SkillId::new("near_mountain_river"),
            SkillId::new("hunger_cold_1"),
            SkillId::new("unexpectedly"),
        ],
        archetype_name: "Black Tortoise A".to_string(),
    });

    // K32: Black Tortoise B (US-433)
    registry.register(MonsterFamily {
        id: FamilyId::new("black_tortoise_B"),
        dungeon: Dungeon::XuanWu,
        tier: MonsterTier::Boss,
        role: FamilyRole::Controller,
        monster_type: MonsterType::Beast,
        skill_ids: vec![
            SkillId::new("snake_call_roll"),
            SkillId::new("snake_rain_spray"),
            SkillId::new("freezing_cold"),
            SkillId::new("benefits_stress"),
            SkillId::new("fangs_sprayed"),
            SkillId::new("armor"),
            SkillId::new("fangs_sprayed_1"),
            SkillId::new("snake_bites"),
        ],
        archetype_name: "Black Tortoise B".to_string(),
    });

    // K33: Rotvine Wraith (US-434)
    registry.register(MonsterFamily {
        id: FamilyId::new("rotvine_wraith"),
        dungeon: Dungeon::XuanWu,
        tier: MonsterTier::Boss,
        role: FamilyRole::Summoner,
        monster_type: MonsterType::Eldritch,
        skill_ids: vec![
            SkillId::new("cadaver_bloom"),
            SkillId::new("rotvine_snare"),
            SkillId::new("sepsis_strangulate"),
            SkillId::new("telluric_resurrect"),
            SkillId::new("carrion_sowing"),
            SkillId::new("move"),
        ],
        archetype_name: "Rotvine Wraith".to_string(),
    });

    // K33: Rotten Fruit A (US-434)
    registry.register(MonsterFamily {
        id: FamilyId::new("rotten_fruit_A"),
        dungeon: Dungeon::XuanWu,
        tier: MonsterTier::Boss,
        role: FamilyRole::Support,
        monster_type: MonsterType::Eldritch,
        skill_ids: vec![
            SkillId::new("absorbed"),
        ],
        archetype_name: "Rotten Fruit A".to_string(),
    });

    // K33: Rotten Fruit B (US-434)
    registry.register(MonsterFamily {
        id: FamilyId::new("rotten_fruit_B"),
        dungeon: Dungeon::XuanWu,
        tier: MonsterTier::Boss,
        role: FamilyRole::Skirmisher,
        monster_type: MonsterType::Eldritch,
        skill_ids: vec![
            SkillId::new("fruit_explosion"),
        ],
        archetype_name: "Rotten Fruit B".to_string(),
    });

    // K33: Rotten Fruit C (US-434)
    registry.register(MonsterFamily {
        id: FamilyId::new("rotten_fruit_C"),
        dungeon: Dungeon::XuanWu,
        tier: MonsterTier::Boss,
        role: FamilyRole::Controller,
        monster_type: MonsterType::Eldritch,
        skill_ids: vec![
            SkillId::new("fruit_stress_explosion"),
        ],
        archetype_name: "Rotten Fruit C".to_string(),
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

    #[test]
    fn registry_resolves_ghost_fire_damage() {
        let registry = build_registry();

        let family = registry
            .get("ghost_fire_damage")
            .expect("ghost_fire_damage should be registered");

        assert_eq!(family.id.0, "ghost_fire_damage");
        assert_eq!(family.dungeon, Dungeon::ZhuQue);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Ranged);
        assert_eq!(family.monster_type, MonsterType::Eldritch);
        assert_eq!(family.archetype_name, "Ghost Fire Damage");
        assert_eq!(family.skill_ids.len(), 3);
    }

    #[test]
    fn registry_ghost_fire_damage_has_burn_attack_and_ghost_fire_split() {
        let registry = build_registry();

        let family = registry.get("ghost_fire_damage").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"burn_attack"),
            "ghost_fire_damage must have burn_attack skill"
        );
        assert!(
            skill_ids.contains(&"ghost_fire_split"),
            "ghost_fire_damage must have ghost_fire_split skill"
        );
    }

    #[test]
    fn registry_resolves_fox_fire() {
        let registry = build_registry();

        let family = registry
            .get("fox_fire")
            .expect("fox_fire should be registered");

        assert_eq!(family.id.0, "fox_fire");
        assert_eq!(family.dungeon, Dungeon::ZhuQue);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Bruiser);
        assert_eq!(family.monster_type, MonsterType::Beast);
        assert_eq!(family.archetype_name, "Fox Fire");
        assert_eq!(family.skill_ids.len(), 4);
    }

    #[test]
    fn registry_fox_fire_has_bite_and_protect() {
        let registry = build_registry();

        let family = registry.get("fox_fire").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"bite"),
            "fox_fire must have bite skill"
        );
        assert!(
            skill_ids.contains(&"protect"),
            "fox_fire must have protect skill"
        );
    }

    #[test]
    fn registry_resolves_moth_fire() {
        let registry = build_registry();

        let family = registry
            .get("moth_fire")
            .expect("moth_fire should be registered");

        assert_eq!(family.id.0, "moth_fire");
        assert_eq!(family.dungeon, Dungeon::ZhuQue);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Ranged);
        assert_eq!(family.monster_type, MonsterType::Eldritch);
        assert_eq!(family.archetype_name, "Moth Fire");
        assert_eq!(family.skill_ids.len(), 3);
    }

    #[test]
    fn registry_moth_fire_has_cocoon_and_fly_into_fire() {
        let registry = build_registry();

        let family = registry.get("moth_fire").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"cocoon"),
            "moth_fire must have cocoon skill"
        );
        assert!(
            skill_ids.contains(&"fly_into_fire"),
            "moth_fire must have fly_into_fire skill"
        );
    }

    #[test]
    fn registry_resolves_lantern() {
        let registry = build_registry();

        let family = registry
            .get("lantern")
            .expect("lantern should be registered");

        assert_eq!(family.id.0, "lantern");
        assert_eq!(family.dungeon, Dungeon::ZhuQue);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Ranged);
        assert_eq!(family.monster_type, MonsterType::Eldritch);
        assert_eq!(family.archetype_name, "Lantern");
        assert_eq!(family.skill_ids.len(), 2);
    }

    #[test]
    fn registry_lantern_has_stress_and_burn_attack() {
        let registry = build_registry();

        let family = registry.get("lantern").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"stress"),
            "lantern must have stress skill"
        );
        assert!(
            skill_ids.contains(&"burn_attack"),
            "lantern must have burn_attack skill"
        );
    }

    #[test]
    fn registry_resolves_snake_water() {
        let registry = build_registry();

        let family = registry
            .get("snake_water")
            .expect("snake_water should be registered");

        assert_eq!(family.id.0, "snake_water");
        assert_eq!(family.dungeon, Dungeon::XuanWu);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Controller);
        assert_eq!(family.monster_type, MonsterType::Eldritch);
        assert_eq!(family.archetype_name, "Snake Water");
        assert_eq!(family.skill_ids.len(), 3);
    }

    #[test]
    fn registry_snake_water_has_stun_and_poison_fang() {
        let registry = build_registry();

        let family = registry.get("snake_water").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"stun"),
            "snake_water must have stun skill"
        );
        assert!(
            skill_ids.contains(&"poison_fang"),
            "snake_water must have poison_fang skill"
        );
    }

    #[test]
    fn registry_resolves_water_grass() {
        let registry = build_registry();

        let family = registry
            .get("water_grass")
            .expect("water_grass should be registered");

        assert_eq!(family.id.0, "water_grass");
        assert_eq!(family.dungeon, Dungeon::XuanWu);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Controller);
        assert_eq!(family.monster_type, MonsterType::Eldritch);
        assert_eq!(family.archetype_name, "Water Grass");
        assert_eq!(family.skill_ids.len(), 5);
    }

    #[test]
    fn registry_water_grass_has_stun_and_puncture_and_convolve() {
        let registry = build_registry();

        let family = registry.get("water_grass").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"stun"),
            "water_grass must have stun skill"
        );
        assert!(
            skill_ids.contains(&"puncture"),
            "water_grass must have puncture skill"
        );
        assert!(
            skill_ids.contains(&"convolve"),
            "water_grass must have convolve skill"
        );
    }

    #[test]
    fn registry_resolves_monkey_water() {
        let registry = build_registry();

        let family = registry
            .get("monkey_water")
            .expect("monkey_water should be registered");

        assert_eq!(family.id.0, "monkey_water");
        assert_eq!(family.dungeon, Dungeon::XuanWu);
        assert_eq!(family.tier, MonsterTier::Common);
        assert_eq!(family.role, FamilyRole::Bruiser);
        assert_eq!(family.monster_type, MonsterType::Unholy);
        assert_eq!(family.archetype_name, "Monkey Water");
        assert_eq!(family.skill_ids.len(), 4);
    }

    #[test]
    fn registry_monkey_water_has_rush_and_stress() {
        let registry = build_registry();

        let family = registry.get("monkey_water").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"rush"),
            "monkey_water must have rush skill"
        );
        assert!(
            skill_ids.contains(&"stress"),
            "monkey_water must have stress skill"
        );
    }

    #[test]
    fn registry_resolves_azure_dragon() {
        let registry = build_registry();

        let family = registry
            .get("azure_dragon")
            .expect("azure_dragon should be registered");

        assert_eq!(family.id.0, "azure_dragon");
        assert_eq!(family.dungeon, Dungeon::QingLong);
        assert_eq!(family.tier, MonsterTier::Boss);
        assert_eq!(family.role, FamilyRole::Summoner);
        assert_eq!(family.monster_type, MonsterType::Beast);
        assert_eq!(family.archetype_name, "Azure Dragon");
        assert_eq!(family.skill_ids.len(), 11);
    }

    #[test]
    fn registry_azure_dragon_has_summon_ball_and_volt_tyranny() {
        let registry = build_registry();

        let family = registry.get("azure_dragon").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"swap_dragon_ball_summon"),
            "azure_dragon must have swap_dragon_ball_summon skill"
        );
        assert!(
            skill_ids.contains(&"volt_tyranny"),
            "azure_dragon must have volt_tyranny skill"
        );
    }

    #[test]
    fn registry_resolves_azure_dragon_ball_thunder() {
        let registry = build_registry();

        let family = registry
            .get("azure_dragon_ball_thunder")
            .expect("azure_dragon_ball_thunder should be registered");

        assert_eq!(family.id.0, "azure_dragon_ball_thunder");
        assert_eq!(family.dungeon, Dungeon::QingLong);
        assert_eq!(family.tier, MonsterTier::Boss);
        assert_eq!(family.role, FamilyRole::Support);
        assert_eq!(family.monster_type, MonsterType::Eldritch);
        assert_eq!(family.archetype_name, "Azure Dragon Ball Thunder");
        assert_eq!(family.skill_ids.len(), 3);
    }

    #[test]
    fn registry_azure_dragon_ball_thunder_has_buff_and_stress() {
        let registry = build_registry();

        let family = registry.get("azure_dragon_ball_thunder").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"thunder_buff_magic"),
            "azure_dragon_ball_thunder must have thunder_buff_magic skill"
        );
        assert!(
            skill_ids.contains(&"thunder_stress_attack"),
            "azure_dragon_ball_thunder must have thunder_stress_attack skill"
        );
    }

    #[test]
    fn registry_resolves_azure_dragon_ball_wind() {
        let registry = build_registry();

        let family = registry
            .get("azure_dragon_ball_wind")
            .expect("azure_dragon_ball_wind should be registered");

        assert_eq!(family.id.0, "azure_dragon_ball_wind");
        assert_eq!(family.dungeon, Dungeon::QingLong);
        assert_eq!(family.tier, MonsterTier::Boss);
        assert_eq!(family.role, FamilyRole::Support);
        assert_eq!(family.monster_type, MonsterType::Eldritch);
        assert_eq!(family.archetype_name, "Azure Dragon Ball Wind");
        assert_eq!(family.skill_ids.len(), 3);
    }

    #[test]
    fn registry_azure_dragon_ball_wind_has_buff_and_shuffle() {
        let registry = build_registry();

        let family = registry.get("azure_dragon_ball_wind").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"wind_buff_acc"),
            "azure_dragon_ball_wind must have wind_buff_acc skill"
        );
        assert!(
            skill_ids.contains(&"wind_shuffle"),
            "azure_dragon_ball_wind must have wind_shuffle skill"
        );
    }

    #[test]
    fn registry_resolves_vermilion_bird() {
        let registry = build_registry();

        let family = registry
            .get("vermilion_bird")
            .expect("vermilion_bird should be registered");

        assert_eq!(family.id.0, "vermilion_bird");
        assert_eq!(family.dungeon, Dungeon::ZhuQue);
        assert_eq!(family.tier, MonsterTier::Boss);
        assert_eq!(family.role, FamilyRole::Summoner);
        assert_eq!(family.monster_type, MonsterType::Beast);
        assert_eq!(family.archetype_name, "Vermilion Bird");
        assert_eq!(family.skill_ids.len(), 8);
    }

    #[test]
    fn registry_vermilion_bird_has_ruin_and_explosion() {
        let registry = build_registry();

        let family = registry.get("vermilion_bird").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"ruin"),
            "vermilion_bird must have ruin skill"
        );
        assert!(
            skill_ids.contains(&"explosion"),
            "vermilion_bird must have explosion skill"
        );
    }

    #[test]
    fn registry_resolves_vermilion_bird_tail_a() {
        let registry = build_registry();

        let family = registry
            .get("vermilion_bird_tail_A")
            .expect("vermilion_bird_tail_A should be registered");

        assert_eq!(family.id.0, "vermilion_bird_tail_A");
        assert_eq!(family.dungeon, Dungeon::ZhuQue);
        assert_eq!(family.tier, MonsterTier::Boss);
        assert_eq!(family.role, FamilyRole::Support);
        assert_eq!(family.monster_type, MonsterType::Beast);
        assert_eq!(family.archetype_name, "Vermilion Bird Tail A");
        assert_eq!(family.skill_ids.len(), 4);
    }

    #[test]
    fn registry_vermilion_bird_tail_a_has_deterrence_and_protect() {
        let registry = build_registry();

        let family = registry.get("vermilion_bird_tail_A").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"deterrence"),
            "vermilion_bird_tail_A must have deterrence skill"
        );
        assert!(
            skill_ids.contains(&"protect"),
            "vermilion_bird_tail_A must have protect skill"
        );
    }

    #[test]
    fn registry_resolves_vermilion_bird_tail_b() {
        let registry = build_registry();

        let family = registry
            .get("vermilion_bird_tail_B")
            .expect("vermilion_bird_tail_B should be registered");

        assert_eq!(family.id.0, "vermilion_bird_tail_B");
        assert_eq!(family.dungeon, Dungeon::ZhuQue);
        assert_eq!(family.tier, MonsterTier::Boss);
        assert_eq!(family.role, FamilyRole::Controller);
        assert_eq!(family.monster_type, MonsterType::Beast);
        assert_eq!(family.archetype_name, "Vermilion Bird Tail B");
        assert_eq!(family.skill_ids.len(), 7);
    }

    #[test]
    fn registry_vermilion_bird_tail_b_has_follow_and_heaven_falls() {
        let registry = build_registry();

        let family = registry.get("vermilion_bird_tail_B").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"follow"),
            "vermilion_bird_tail_B must have follow skill"
        );
        assert!(
            skill_ids.contains(&"heaven_falls"),
            "vermilion_bird_tail_B must have heaven_falls skill"
        );
    }

    #[test]
    fn registry_resolves_white_tiger_c() {
        let registry = build_registry();

        let family = registry
            .get("white_tiger_C")
            .expect("white_tiger_C should be registered");

        assert_eq!(family.id.0, "white_tiger_C");
        assert_eq!(family.dungeon, Dungeon::BaiHu);
        assert_eq!(family.tier, MonsterTier::Boss);
        assert_eq!(family.role, FamilyRole::Summoner);
        assert_eq!(family.monster_type, MonsterType::Beast);
        assert_eq!(family.archetype_name, "White Tiger C");
        assert_eq!(family.skill_ids.len(), 7);
    }

    #[test]
    fn registry_white_tiger_c_has_paw_and_storm_control() {
        let registry = build_registry();

        let family = registry.get("white_tiger_C").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"paw"),
            "white_tiger_C must have paw skill"
        );
        assert!(
            skill_ids.contains(&"thunder_lightning"),
            "white_tiger_C must have thunder_lightning skill"
        );
        assert!(
            skill_ids.contains(&"jump"),
            "white_tiger_C must have jump skill"
        );
    }

    #[test]
    fn registry_resolves_white_tiger_a() {
        let registry = build_registry();

        let family = registry
            .get("white_tiger_A")
            .expect("white_tiger_A should be registered");

        assert_eq!(family.id.0, "white_tiger_A");
        assert_eq!(family.dungeon, Dungeon::BaiHu);
        assert_eq!(family.tier, MonsterTier::Boss);
        assert_eq!(family.role, FamilyRole::Support);
        assert_eq!(family.monster_type, MonsterType::Beast);
        assert_eq!(family.archetype_name, "White Tiger A");
        assert_eq!(family.skill_ids.len(), 5);
    }

    #[test]
    fn registry_white_tiger_a_has_drag_and_pounce() {
        let registry = build_registry();

        let family = registry.get("white_tiger_A").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"drag"),
            "white_tiger_A must have drag skill"
        );
        assert!(
            skill_ids.contains(&"pounce"),
            "white_tiger_A must have pounce skill"
        );
    }

    #[test]
    fn registry_resolves_white_tiger_b() {
        let registry = build_registry();

        let family = registry
            .get("white_tiger_B")
            .expect("white_tiger_B should be registered");

        assert_eq!(family.id.0, "white_tiger_B");
        assert_eq!(family.dungeon, Dungeon::BaiHu);
        assert_eq!(family.tier, MonsterTier::Boss);
        assert_eq!(family.role, FamilyRole::Controller);
        assert_eq!(family.monster_type, MonsterType::Beast);
        assert_eq!(family.archetype_name, "White Tiger B");
        assert_eq!(family.skill_ids.len(), 5);
    }

    #[test]
    fn registry_white_tiger_b_has_fire_soul_shadow_and_tiger_swing() {
        let registry = build_registry();

        let family = registry.get("white_tiger_B").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"fire_soul_shadow"),
            "white_tiger_B must have fire_soul_shadow skill"
        );
        assert!(
            skill_ids.contains(&"tiger_swing"),
            "white_tiger_B must have tiger_swing skill"
        );
    }

    #[test]
    fn registry_resolves_white_tiger_terrain() {
        let registry = build_registry();

        let family = registry
            .get("white_tiger_terrain")
            .expect("white_tiger_terrain should be registered");

        assert_eq!(family.id.0, "white_tiger_terrain");
        assert_eq!(family.dungeon, Dungeon::BaiHu);
        assert_eq!(family.tier, MonsterTier::Boss);
        assert_eq!(family.role, FamilyRole::Support);
        assert_eq!(family.monster_type, MonsterType::Corpse);
        assert_eq!(family.archetype_name, "White Tiger Terrain");
        assert_eq!(family.skill_ids.len(), 1);
    }

    #[test]
    fn registry_white_tiger_terrain_has_occupy_skill() {
        let registry = build_registry();

        let family = registry.get("white_tiger_terrain").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"occupy"),
            "white_tiger_terrain must have occupy skill"
        );
    }

    #[test]
    fn registry_resolves_black_tortoise_a() {
        let registry = build_registry();

        let family = registry
            .get("black_tortoise_A")
            .expect("black_tortoise_A should be registered");

        assert_eq!(family.id.0, "black_tortoise_A");
        assert_eq!(family.dungeon, Dungeon::XuanWu);
        assert_eq!(family.tier, MonsterTier::Boss);
        assert_eq!(family.role, FamilyRole::Tank);
        assert_eq!(family.monster_type, MonsterType::Beast);
        assert_eq!(family.archetype_name, "Black Tortoise A");
        assert_eq!(family.skill_ids.len(), 8);
    }

    #[test]
    fn registry_black_tortoise_a_has_tortoise_mark_and_share_damage() {
        let registry = build_registry();

        let family = registry.get("black_tortoise_A").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"tortoise_call_roll"),
            "black_tortoise_A must have tortoise_call_roll skill"
        );
        assert!(
            skill_ids.contains(&"near_mountain_river"),
            "black_tortoise_A must have near_mountain_river skill"
        );
    }

    #[test]
    fn registry_resolves_black_tortoise_b() {
        let registry = build_registry();

        let family = registry
            .get("black_tortoise_B")
            .expect("black_tortoise_B should be registered");

        assert_eq!(family.id.0, "black_tortoise_B");
        assert_eq!(family.dungeon, Dungeon::XuanWu);
        assert_eq!(family.tier, MonsterTier::Boss);
        assert_eq!(family.role, FamilyRole::Controller);
        assert_eq!(family.monster_type, MonsterType::Beast);
        assert_eq!(family.archetype_name, "Black Tortoise B");
        assert_eq!(family.skill_ids.len(), 8);
    }

    #[test]
    fn registry_black_tortoise_b_has_snake_mark_and_share_damage() {
        let registry = build_registry();

        let family = registry.get("black_tortoise_B").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"snake_call_roll"),
            "black_tortoise_B must have snake_call_roll skill"
        );
        assert!(
            skill_ids.contains(&"armor"),
            "black_tortoise_B must have armor skill"
        );
    }

    #[test]
    fn registry_resolves_rotvine_wraith() {
        let registry = build_registry();

        let family = registry
            .get("rotvine_wraith")
            .expect("rotvine_wraith should be registered");

        assert_eq!(family.id.0, "rotvine_wraith");
        assert_eq!(family.dungeon, Dungeon::XuanWu);
        assert_eq!(family.tier, MonsterTier::Boss);
        assert_eq!(family.role, FamilyRole::Summoner);
        assert_eq!(family.monster_type, MonsterType::Eldritch);
        assert_eq!(family.archetype_name, "Rotvine Wraith");
        assert_eq!(family.skill_ids.len(), 6);
    }

    #[test]
    fn registry_rotvine_wraith_has_carrion_sowing_and_sepsis_strangulate() {
        let registry = build_registry();

        let family = registry.get("rotvine_wraith").unwrap();
        let skill_ids: Vec<&str> = family.skill_ids.iter().map(|s| s.0.as_str()).collect();
        assert!(
            skill_ids.contains(&"carrion_sowing"),
            "rotvine_wraith must have carrion_sowing skill"
        );
        assert!(
            skill_ids.contains(&"sepsis_strangulate"),
            "rotvine_wraith must have sepsis_strangulate skill"
        );
    }

    #[test]
    fn registry_resolves_rotten_fruit_a() {
        let registry = build_registry();

        let family = registry
            .get("rotten_fruit_A")
            .expect("rotten_fruit_A should be registered");

        assert_eq!(family.id.0, "rotten_fruit_A");
        assert_eq!(family.dungeon, Dungeon::XuanWu);
        assert_eq!(family.tier, MonsterTier::Boss);
        assert_eq!(family.role, FamilyRole::Support);
        assert_eq!(family.monster_type, MonsterType::Eldritch);
        assert_eq!(family.archetype_name, "Rotten Fruit A");
        assert_eq!(family.skill_ids.len(), 1);
    }

    #[test]
    fn registry_resolves_rotten_fruit_b() {
        let registry = build_registry();

        let family = registry
            .get("rotten_fruit_B")
            .expect("rotten_fruit_B should be registered");

        assert_eq!(family.id.0, "rotten_fruit_B");
        assert_eq!(family.dungeon, Dungeon::XuanWu);
        assert_eq!(family.tier, MonsterTier::Boss);
        assert_eq!(family.role, FamilyRole::Skirmisher);
        assert_eq!(family.monster_type, MonsterType::Eldritch);
        assert_eq!(family.archetype_name, "Rotten Fruit B");
        assert_eq!(family.skill_ids.len(), 1);
    }

    #[test]
    fn registry_resolves_rotten_fruit_c() {
        let registry = build_registry();

        let family = registry
            .get("rotten_fruit_C")
            .expect("rotten_fruit_C should be registered");

        assert_eq!(family.id.0, "rotten_fruit_C");
        assert_eq!(family.dungeon, Dungeon::XuanWu);
        assert_eq!(family.tier, MonsterTier::Boss);
        assert_eq!(family.role, FamilyRole::Controller);
        assert_eq!(family.monster_type, MonsterType::Eldritch);
        assert_eq!(family.archetype_name, "Rotten Fruit C");
        assert_eq!(family.skill_ids.len(), 1);
    }
}
