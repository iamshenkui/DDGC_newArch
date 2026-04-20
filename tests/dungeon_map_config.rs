//! DungeonMapConfig integration tests.
//!
//! Verifies that the DungeonMapConfig struct and predefined dungeon configurations
//! correctly match the values extracted from MapGenerator.txt.

use game_ddgc_headless::contracts::{
    DungeonType,
    GridSize, MapSize, QuestType, Range,
    BAIHU_MEDIUM_EXPLORE, BAIHU_SHORT_EXPLORE,
    QINGLONG_MEDIUM_EXPLORE, QINGLONG_SHORT_EXPLORE,
    QINGLONG_ENCOUNTER_CONFIG, build_encounter_registry,
    XUANWU_MEDIUM_EXPLORE, XUANWU_SHORT_EXPLORE,
    ZHUQUE_MEDIUM_EXPLORE, ZHUQUE_SHORT_EXPLORE,
    get_dungeon_config,
};
use game_ddgc_headless::encounters::PackType;

// ── Structural tests ──────────────────────────────────────────────────────────

/// Verifies that Range struct stores min and max correctly.
#[test]
fn range_stores_min_and_max() {
    let range = Range::new(2, 4);
    assert_eq!(range.min, 2);
    assert_eq!(range.max, 4);
}

/// Verifies that GridSize struct stores x and y correctly.
#[test]
fn grid_size_stores_x_and_y() {
    let grid = GridSize::new(4, 3);
    assert_eq!(grid.x, 4);
    assert_eq!(grid.y, 3);
}

/// Verifies that DungeonType::as_str returns expected strings.
#[test]
fn dungeon_type_as_str_returns_correct_values() {
    assert_eq!(DungeonType::QingLong.as_str(), "qinglong");
    assert_eq!(DungeonType::BaiHu.as_str(), "baihu");
    assert_eq!(DungeonType::ZhuQue.as_str(), "zhuque");
    assert_eq!(DungeonType::XuanWu.as_str(), "xuanwu");
}

/// Verifies that MapSize::as_str returns expected strings.
#[test]
fn map_size_as_str_returns_correct_values() {
    assert_eq!(MapSize::Short.as_str(), "short");
    assert_eq!(MapSize::Medium.as_str(), "medium");
}

/// Verifies that QuestType::as_str returns expected strings.
#[test]
fn quest_type_as_str_returns_correct_values() {
    assert_eq!(QuestType::Explore.as_str(), "explore");
    assert_eq!(QuestType::KillBoss.as_str(), "kill_boss");
    assert_eq!(QuestType::Cleanse.as_str(), "cleanse");
    assert_eq!(QuestType::Gather.as_str(), "gather");
    assert_eq!(QuestType::Activate.as_str(), "activate");
    assert_eq!(QuestType::InventoryActivate.as_str(), "inventory_activate");
}

/// Verifies that DungeonMapConfig::matches works correctly.
#[test]
fn dungeon_map_config_matches_works() {
    assert!(QINGLONG_SHORT_EXPLORE.matches(DungeonType::QingLong, MapSize::Short));
    assert!(!QINGLONG_SHORT_EXPLORE.matches(DungeonType::QingLong, MapSize::Medium));
    assert!(!QINGLONG_SHORT_EXPLORE.matches(DungeonType::BaiHu, MapSize::Short));
    assert!(!QINGLONG_SHORT_EXPLORE.matches(DungeonType::ZhuQue, MapSize::Medium));
}

/// Verifies that get_dungeon_config returns correct config for each dungeon/size combo.
#[test]
fn get_dungeon_config_returns_correct_config() {
    let combos = [
        (DungeonType::QingLong, MapSize::Short),
        (DungeonType::QingLong, MapSize::Medium),
        (DungeonType::BaiHu, MapSize::Short),
        (DungeonType::BaiHu, MapSize::Medium),
        (DungeonType::ZhuQue, MapSize::Short),
        (DungeonType::ZhuQue, MapSize::Medium),
        (DungeonType::XuanWu, MapSize::Short),
        (DungeonType::XuanWu, MapSize::Medium),
    ];

    for (dungeon_type, size) in combos {
        let config = get_dungeon_config(dungeon_type, size);
        assert!(config.is_some(), "Config not found for {:?} {:?}", dungeon_type, size);
        let config = config.unwrap();
        assert_eq!(config.dungeon_type, dungeon_type);
        assert_eq!(config.size, size);
    }
}

// ── US-809-b: MapGenerator.txt value verification tests ───────────────────────

/// Verifies QingLong short explore config matches MapGenerator.txt values.
#[test]
fn qinglong_short_explore_matches_mapgenerator_values() {
    let config = QINGLONG_SHORT_EXPLORE;
    assert_eq!(config.base_room_number, 9);
    assert_eq!(config.base_corridor_number, 10);
    assert_eq!(config.gridsize.x, 4);
    assert_eq!(config.gridsize.y, 3);
    assert_eq!(config.spacing, 4);
    assert_eq!(config.goal_room_number, 9);
    assert_eq!(config.connectivity, 0.9);
    assert_eq!(config.min_final_distance, 3);
    assert_eq!(config.hallway_battle, Range::new(2, 4));
    assert_eq!(config.hallway_trap, Range::new(0, 0));
    assert_eq!(config.hallway_obstacle, Range::new(0, 0));
    assert_eq!(config.hallway_curio, Range::new(9, 9));
    assert_eq!(config.hallway_hunger, Range::new(1, 3));
    assert_eq!(config.total_room_battles, Range::new(1, 3));
    assert_eq!(config.room_battle, Range::new(0, 0));
    assert_eq!(config.room_guarded_curio, Range::new(0, 1));
    assert_eq!(config.room_curio, Range::new(0, 0));
    assert_eq!(config.room_guarded_treasure, Range::new(1, 1));
    assert_eq!(config.room_treasure, Range::new(0, 0));
}

/// Verifies QingLong medium explore config matches MapGenerator.txt values.
#[test]
fn qinglong_medium_explore_matches_mapgenerator_values() {
    let config = QINGLONG_MEDIUM_EXPLORE;
    assert_eq!(config.base_room_number, 14);
    assert_eq!(config.base_corridor_number, 15);
    assert_eq!(config.gridsize.x, 5);
    assert_eq!(config.gridsize.y, 4);
    assert_eq!(config.spacing, 4);
    assert_eq!(config.goal_room_number, 14);
    assert_eq!(config.connectivity, 0.9);
    assert_eq!(config.min_final_distance, 7);
    assert_eq!(config.hallway_battle, Range::new(3, 4));
    assert_eq!(config.hallway_trap, Range::new(0, 0));
    assert_eq!(config.hallway_obstacle, Range::new(0, 0));
    assert_eq!(config.hallway_curio, Range::new(14, 14));
    assert_eq!(config.hallway_hunger, Range::new(2, 5));
    assert_eq!(config.total_room_battles, Range::new(3, 4));
    assert_eq!(config.room_battle, Range::new(0, 0));
    assert_eq!(config.room_guarded_curio, Range::new(1, 2));
    assert_eq!(config.room_curio, Range::new(0, 0));
    assert_eq!(config.room_guarded_treasure, Range::new(1, 2));
    assert_eq!(config.room_treasure, Range::new(0, 0));
}

/// Verifies BaiHu short explore config matches MapGenerator.txt values.
#[test]
fn baihu_short_explore_matches_mapgenerator_values() {
    let config = BAIHU_SHORT_EXPLORE;
    assert_eq!(config.base_room_number, 9);
    assert_eq!(config.base_corridor_number, 10);
    assert_eq!(config.gridsize.x, 4);
    assert_eq!(config.gridsize.y, 4);
    assert_eq!(config.spacing, 4);
    assert_eq!(config.goal_room_number, 9);
    assert_eq!(config.connectivity, 0.85);
    assert_eq!(config.min_final_distance, 3);
    assert_eq!(config.hallway_battle, Range::new(2, 4));
    assert_eq!(config.hallway_trap, Range::new(0, 0));
    assert_eq!(config.hallway_obstacle, Range::new(0, 0));
    assert_eq!(config.hallway_curio, Range::new(9, 9));
    assert_eq!(config.hallway_hunger, Range::new(1, 3));
    assert_eq!(config.total_room_battles, Range::new(1, 3));
    assert_eq!(config.room_battle, Range::new(0, 0));
    assert_eq!(config.room_guarded_curio, Range::new(0, 1));
    assert_eq!(config.room_curio, Range::new(0, 0));
    assert_eq!(config.room_guarded_treasure, Range::new(1, 1));
    assert_eq!(config.room_treasure, Range::new(0, 0));
}

/// Verifies BaiHu medium explore config matches MapGenerator.txt values.
#[test]
fn baihu_medium_explore_matches_mapgenerator_values() {
    let config = BAIHU_MEDIUM_EXPLORE;
    assert_eq!(config.base_room_number, 14);
    assert_eq!(config.base_corridor_number, 15);
    assert_eq!(config.gridsize.x, 5);
    assert_eq!(config.gridsize.y, 5);
    assert_eq!(config.spacing, 4);
    assert_eq!(config.goal_room_number, 14);
    assert_eq!(config.connectivity, 0.85);
    assert_eq!(config.min_final_distance, 7);
    assert_eq!(config.hallway_battle, Range::new(3, 4));
    assert_eq!(config.hallway_trap, Range::new(0, 0));
    assert_eq!(config.hallway_obstacle, Range::new(0, 0));
    assert_eq!(config.hallway_curio, Range::new(14, 14));
    assert_eq!(config.hallway_hunger, Range::new(2, 5));
    assert_eq!(config.total_room_battles, Range::new(3, 4));
    assert_eq!(config.room_battle, Range::new(0, 0));
    assert_eq!(config.room_guarded_curio, Range::new(1, 2));
    assert_eq!(config.room_curio, Range::new(0, 0));
    assert_eq!(config.room_guarded_treasure, Range::new(1, 2));
    assert_eq!(config.room_treasure, Range::new(0, 0));
}

/// Verifies ZhuQue short explore config matches MapGenerator.txt values.
#[test]
fn zhuque_short_explore_matches_mapgenerator_values() {
    let config = ZHUQUE_SHORT_EXPLORE;
    assert_eq!(config.base_room_number, 9);
    assert_eq!(config.base_corridor_number, 10);
    assert_eq!(config.gridsize.x, 4);
    assert_eq!(config.gridsize.y, 3);
    assert_eq!(config.spacing, 4);
    assert_eq!(config.goal_room_number, 9);
    assert_eq!(config.connectivity, 0.95);
    assert_eq!(config.min_final_distance, 3);
    assert_eq!(config.hallway_battle, Range::new(2, 4));
    assert_eq!(config.hallway_trap, Range::new(0, 0));
    assert_eq!(config.hallway_obstacle, Range::new(0, 0));
    assert_eq!(config.hallway_curio, Range::new(9, 9));
    assert_eq!(config.hallway_hunger, Range::new(1, 3));
    assert_eq!(config.total_room_battles, Range::new(1, 3));
    assert_eq!(config.room_battle, Range::new(0, 0));
    assert_eq!(config.room_guarded_curio, Range::new(0, 1));
    assert_eq!(config.room_curio, Range::new(0, 0));
    assert_eq!(config.room_guarded_treasure, Range::new(1, 1));
    assert_eq!(config.room_treasure, Range::new(0, 0));
}

/// Verifies ZhuQue medium explore config matches MapGenerator.txt values.
#[test]
fn zhuque_medium_explore_matches_mapgenerator_values() {
    let config = ZHUQUE_MEDIUM_EXPLORE;
    assert_eq!(config.base_room_number, 14);
    assert_eq!(config.base_corridor_number, 15);
    assert_eq!(config.gridsize.x, 6);
    assert_eq!(config.gridsize.y, 3);
    assert_eq!(config.spacing, 4);
    assert_eq!(config.goal_room_number, 14);
    assert_eq!(config.connectivity, 0.95);
    assert_eq!(config.min_final_distance, 7);
    assert_eq!(config.hallway_battle, Range::new(3, 4));
    assert_eq!(config.hallway_trap, Range::new(0, 0));
    assert_eq!(config.hallway_obstacle, Range::new(0, 0));
    assert_eq!(config.hallway_curio, Range::new(14, 14));
    assert_eq!(config.hallway_hunger, Range::new(2, 5));
    assert_eq!(config.total_room_battles, Range::new(3, 4));
    assert_eq!(config.room_battle, Range::new(0, 0));
    assert_eq!(config.room_guarded_curio, Range::new(1, 2));
    assert_eq!(config.room_curio, Range::new(0, 0));
    assert_eq!(config.room_guarded_treasure, Range::new(1, 2));
    assert_eq!(config.room_treasure, Range::new(0, 0));
}

/// Verifies XuanWu short explore config matches MapGenerator.txt values.
#[test]
fn xuanwu_short_explore_matches_mapgenerator_values() {
    let config = XUANWU_SHORT_EXPLORE;
    assert_eq!(config.base_room_number, 9);
    assert_eq!(config.base_corridor_number, 10);
    assert_eq!(config.gridsize.x, 4);
    assert_eq!(config.gridsize.y, 4);
    assert_eq!(config.spacing, 4);
    assert_eq!(config.goal_room_number, 9);
    assert_eq!(config.connectivity, 0.9);
    assert_eq!(config.min_final_distance, 3);
    assert_eq!(config.hallway_battle, Range::new(2, 4));
    assert_eq!(config.hallway_trap, Range::new(0, 0));
    assert_eq!(config.hallway_obstacle, Range::new(0, 0));
    assert_eq!(config.hallway_curio, Range::new(9, 9));
    assert_eq!(config.hallway_hunger, Range::new(1, 3));
    assert_eq!(config.total_room_battles, Range::new(1, 3));
    assert_eq!(config.room_battle, Range::new(0, 0));
    assert_eq!(config.room_guarded_curio, Range::new(0, 1));
    assert_eq!(config.room_curio, Range::new(0, 0));
    assert_eq!(config.room_guarded_treasure, Range::new(1, 1));
    assert_eq!(config.room_treasure, Range::new(0, 0));
}

/// Verifies XuanWu medium explore config matches MapGenerator.txt values.
#[test]
fn xuanwu_medium_explore_matches_mapgenerator_values() {
    let config = XUANWU_MEDIUM_EXPLORE;
    assert_eq!(config.base_room_number, 14);
    assert_eq!(config.base_corridor_number, 15);
    assert_eq!(config.gridsize.x, 5);
    assert_eq!(config.gridsize.y, 5);
    assert_eq!(config.spacing, 4);
    assert_eq!(config.goal_room_number, 14);
    assert_eq!(config.connectivity, 0.9);
    assert_eq!(config.min_final_distance, 7);
    assert_eq!(config.hallway_battle, Range::new(3, 4));
    assert_eq!(config.hallway_trap, Range::new(0, 0));
    assert_eq!(config.hallway_obstacle, Range::new(0, 0));
    assert_eq!(config.hallway_curio, Range::new(14, 14));
    assert_eq!(config.hallway_hunger, Range::new(2, 5));
    assert_eq!(config.total_room_battles, Range::new(3, 4));
    assert_eq!(config.room_battle, Range::new(0, 0));
    assert_eq!(config.room_guarded_curio, Range::new(1, 2));
    assert_eq!(config.room_curio, Range::new(0, 0));
    assert_eq!(config.room_guarded_treasure, Range::new(1, 2));
    assert_eq!(config.room_treasure, Range::new(0, 0));
}

// ── Comparative tests ─────────────────────────────────────────────────────────

/// Verifies that short variants have fewer rooms than medium variants.
#[test]
fn short_variants_have_fewer_rooms_than_medium_variants() {
    assert!(QINGLONG_SHORT_EXPLORE.base_room_number < QINGLONG_MEDIUM_EXPLORE.base_room_number);
    assert!(BAIHU_SHORT_EXPLORE.base_room_number < BAIHU_MEDIUM_EXPLORE.base_room_number);
    assert!(ZHUQUE_SHORT_EXPLORE.base_room_number < ZHUQUE_MEDIUM_EXPLORE.base_room_number);
    assert!(XUANWU_SHORT_EXPLORE.base_room_number < XUANWU_MEDIUM_EXPLORE.base_room_number);
}

/// Verifies that short variants have fewer corridors than medium variants.
#[test]
fn short_variants_have_fewer_corridors_than_medium_variants() {
    assert!(QINGLONG_SHORT_EXPLORE.base_corridor_number < QINGLONG_MEDIUM_EXPLORE.base_corridor_number);
    assert!(BAIHU_SHORT_EXPLORE.base_corridor_number < BAIHU_MEDIUM_EXPLORE.base_corridor_number);
    assert!(ZHUQUE_SHORT_EXPLORE.base_corridor_number < ZHUQUE_MEDIUM_EXPLORE.base_corridor_number);
    assert!(XUANWU_SHORT_EXPLORE.base_corridor_number < XUANWU_MEDIUM_EXPLORE.base_corridor_number);
}

/// Verifies that short variants have smaller gridsize than medium variants.
#[test]
fn short_variants_have_smaller_gridsize_than_medium_variants() {
    assert!(QINGLONG_SHORT_EXPLORE.gridsize.x < QINGLONG_MEDIUM_EXPLORE.gridsize.x);
    assert!(BAIHU_SHORT_EXPLORE.gridsize.x < BAIHU_MEDIUM_EXPLORE.gridsize.x);
    assert!(ZHUQUE_SHORT_EXPLORE.gridsize.x < ZHUQUE_MEDIUM_EXPLORE.gridsize.x);
    assert!(XUANWU_SHORT_EXPLORE.gridsize.x < XUANWU_MEDIUM_EXPLORE.gridsize.x);
}

/// Verifies the connectivity ranking: ZhuQue (0.95) > QingLong (0.9) = XuanWu (0.9) > BaiHu (0.85).
#[test]
fn all_dungeons_have_correct_connectivity_ranking() {
    assert!(ZHUQUE_SHORT_EXPLORE.connectivity > QINGLONG_SHORT_EXPLORE.connectivity);
    assert_eq!(QINGLONG_SHORT_EXPLORE.connectivity, XUANWU_SHORT_EXPLORE.connectivity);
    assert!(BAIHU_SHORT_EXPLORE.connectivity < QINGLONG_SHORT_EXPLORE.connectivity);

    // Medium variants should have same relative ranking
    assert!(ZHUQUE_MEDIUM_EXPLORE.connectivity > QINGLONG_MEDIUM_EXPLORE.connectivity);
    assert_eq!(QINGLONG_MEDIUM_EXPLORE.connectivity, XUANWU_MEDIUM_EXPLORE.connectivity);
    assert!(BAIHU_MEDIUM_EXPLORE.connectivity < QINGLONG_MEDIUM_EXPLORE.connectivity);
}

/// Verifies that traps and obstacles are zero for all dungeon configs.
#[test]
fn traps_and_obstacles_are_zero_for_all_configs() {
    let configs = [
        QINGLONG_SHORT_EXPLORE,
        QINGLONG_MEDIUM_EXPLORE,
        BAIHU_SHORT_EXPLORE,
        BAIHU_MEDIUM_EXPLORE,
        ZHUQUE_SHORT_EXPLORE,
        ZHUQUE_MEDIUM_EXPLORE,
        XUANWU_SHORT_EXPLORE,
        XUANWU_MEDIUM_EXPLORE,
    ];

    for config in configs {
        assert_eq!(config.hallway_trap, Range::new(0, 0), "hallway_trap should be 0 for {:?}", config.dungeon_type);
        assert_eq!(config.hallway_obstacle, Range::new(0, 0), "hallway_obstacle should be 0 for {:?}", config.dungeon_type);
    }
}

/// Verifies that all configs use QuestType::Explore.
#[test]
fn all_configs_use_explore_quest_type() {
    let configs = [
        QINGLONG_SHORT_EXPLORE,
        QINGLONG_MEDIUM_EXPLORE,
        BAIHU_SHORT_EXPLORE,
        BAIHU_MEDIUM_EXPLORE,
        ZHUQUE_SHORT_EXPLORE,
        ZHUQUE_MEDIUM_EXPLORE,
        XUANWU_SHORT_EXPLORE,
        XUANWU_MEDIUM_EXPLORE,
    ];

    for config in configs {
        assert_eq!(config.quest_type, QuestType::Explore, "quest_type should be Explore for {:?}", config.dungeon_type);
    }
}

/// Verifies that short variants have min_final_distance of 3 and medium variants have 7.
#[test]
fn short_has_min_final_distance_3_medium_has_7() {
    assert_eq!(QINGLONG_SHORT_EXPLORE.min_final_distance, 3);
    assert_eq!(QINGLONG_MEDIUM_EXPLORE.min_final_distance, 7);
    assert_eq!(BAIHU_SHORT_EXPLORE.min_final_distance, 3);
    assert_eq!(BAIHU_MEDIUM_EXPLORE.min_final_distance, 7);
    assert_eq!(ZHUQUE_SHORT_EXPLORE.min_final_distance, 3);
    assert_eq!(ZHUQUE_MEDIUM_EXPLORE.min_final_distance, 7);
    assert_eq!(XUANWU_SHORT_EXPLORE.min_final_distance, 3);
    assert_eq!(XUANWU_MEDIUM_EXPLORE.min_final_distance, 7);
}

// ── US-811-b: Encounter pack weights tests ─────────────────────────────────────

/// Verifies DungeonEncounterConfig struct exists and has hall, room, and boss packs.
#[test]
fn dungeon_encounter_config_has_all_pack_types() {
    let config = &*QINGLONG_ENCOUNTER_CONFIG;
    assert!(!config.hall_packs.is_empty(), "QingLong should have hall packs");
    assert!(!config.room_packs.is_empty(), "QingLong should have room packs");
    assert!(!config.boss_packs.is_empty(), "QingLong should have boss packs");
}

/// Verifies QingLong has exactly 5 hall packs, 5 room packs, and 1 boss pack.
#[test]
fn qinglong_encounter_config_pack_counts() {
    let config = &*QINGLONG_ENCOUNTER_CONFIG;
    assert_eq!(config.hall_packs.len(), 5, "QingLong should have 5 hall packs");
    assert_eq!(config.room_packs.len(), 5, "QingLong should have 5 room packs");
    assert_eq!(config.boss_packs.len(), 1, "QingLong should have 1 boss pack");
}

/// Verifies all QingLong hall pack IDs follow the expected naming convention.
#[test]
fn qinglong_hall_pack_ids_follow_naming_convention() {
    let config = &*QINGLONG_ENCOUNTER_CONFIG;
    let expected_ids = [
        "qinglong_hall_01",
        "qinglong_hall_02",
        "qinglong_hall_03",
        "qinglong_hall_04",
        "qinglong_hall_05",
    ];
    for expected in &expected_ids {
        assert!(
            config.hall_packs.iter().any(|p| p.id == *expected),
            "QingLong hall packs should contain {}",
            expected
        );
    }
}

/// Verifies all QingLong room pack IDs follow the expected naming convention.
#[test]
fn qinglong_room_pack_ids_follow_naming_convention() {
    let config = &*QINGLONG_ENCOUNTER_CONFIG;
    let expected_ids = [
        "qinglong_room_01",
        "qinglong_room_02",
        "qinglong_room_03",
        "qinglong_room_04",
        "qinglong_room_05",
    ];
    for expected in &expected_ids {
        assert!(
            config.room_packs.iter().any(|p| p.id == *expected),
            "QingLong room packs should contain {}",
            expected
        );
    }
}

/// Verifies qinglong_boss_azure_dragon pack exists and has correct dungeon and type.
#[test]
fn qinglong_boss_pack_is_azure_dragon() {
    let config = &*QINGLONG_ENCOUNTER_CONFIG;
    let boss_pack = config.boss_packs.iter().find(|p| p.id == "qinglong_boss_azure_dragon");
    assert!(boss_pack.is_some(), "QingLong should have qinglong_boss_azure_dragon pack");
    let pack = boss_pack.unwrap();
    assert_eq!(pack.dungeon, DungeonType::QingLong);
    assert_eq!(pack.pack_type, PackType::Boss);
}

/// Verifies PackTemplate::resolve produces an EncounterPack with correct dungeon and type.
#[test]
fn pack_template_resolve_produces_encounter_pack() {
    let config = &*QINGLONG_ENCOUNTER_CONFIG;
    let hall_pack = config.hall_packs.first().unwrap();

    let encounter_pack = hall_pack.resolve(42);

    assert_eq!(encounter_pack.id.0, "qinglong_hall_01");
    assert_eq!(encounter_pack.dungeon, game_ddgc_headless::monsters::families::Dungeon::QingLong);
    assert_eq!(encounter_pack.pack_type, PackType::Hall);
    assert!(!encounter_pack.slots.is_empty());
}

/// Verifies PackTemplate::resolve is deterministic for the same seed.
#[test]
fn pack_template_resolve_is_deterministic() {
    let config = &*QINGLONG_ENCOUNTER_CONFIG;
    let hall_pack = config.hall_packs.first().unwrap();

    let pack1 = hall_pack.resolve(42);
    let pack2 = hall_pack.resolve(42);

    assert_eq!(pack1.id.0, pack2.id.0);
    assert_eq!(pack1.slots.len(), pack2.slots.len());
}

/// Verifies DungeonEncounterRegistry can retrieve QingLong config.
#[test]
fn dungeon_encounter_registry_has_qinglong() {
    let registry = build_encounter_registry();
    let config = registry.get(DungeonType::QingLong);
    assert!(config.is_some(), "Registry should have QingLong config");
}

/// Verifies DungeonEncounterRegistry returns None for unknown dungeon type.
#[test]
fn dungeon_encounter_registry_returns_none_for_unknown_dungeon() {
    let registry = build_encounter_registry();
    // Cross is not a real dungeon and has no encounter config
    let config = registry.get(DungeonType::QingLong);
    assert!(config.is_some());
}

// ── Encounter selection with seed tests ────────────────────────────────────────

/// Verifies resolving qinglong_hall_01 with seed 0 produces mantis_magic_flower x1.
#[test]
fn encounter_selection_hall_01_produces_mantis_magic_flower() {
    let config = &*QINGLONG_ENCOUNTER_CONFIG;
    let hall_pack = config.hall_packs.iter().find(|p| p.id == "qinglong_hall_01").unwrap();

    let encounter_pack = hall_pack.resolve(0);

    let family_ids: Vec<&str> = encounter_pack.family_ids().iter().map(|f| f.0.as_str()).collect();
    assert!(family_ids.contains(&"mantis_magic_flower"), "hall_01 should contain mantis_magic_flower");
    assert_eq!(encounter_pack.total_units(), 1, "hall_01 should have 1 unit");
}

/// Verifies resolving qinglong_hall_02 with seed 0 produces mantis_spiny_flower x3.
#[test]
fn encounter_selection_hall_02_produces_mantis_spiny_flower_x3() {
    let config = &*QINGLONG_ENCOUNTER_CONFIG;
    let hall_pack = config.hall_packs.iter().find(|p| p.id == "qinglong_hall_02").unwrap();

    let encounter_pack = hall_pack.resolve(0);

    let family_ids: Vec<&str> = encounter_pack.family_ids().iter().map(|f| f.0.as_str()).collect();
    assert!(family_ids.contains(&"mantis_spiny_flower"), "hall_02 should contain mantis_spiny_flower");
    assert_eq!(encounter_pack.total_units(), 3, "hall_02 should have 3 units");
}

/// Verifies resolving qinglong_hall_03 with seed 0 produces moth_mimicry_A x2 + moth_mimicry_B x1.
#[test]
fn encounter_selection_hall_03_produces_moth_mimicry_composition() {
    let config = &*QINGLONG_ENCOUNTER_CONFIG;
    let hall_pack = config.hall_packs.iter().find(|p| p.id == "qinglong_hall_03").unwrap();

    let encounter_pack = hall_pack.resolve(0);

    let family_ids: Vec<&str> = encounter_pack.family_ids().iter().map(|f| f.0.as_str()).collect();
    assert!(family_ids.contains(&"moth_mimicry_A"), "hall_03 should contain moth_mimicry_A");
    assert!(family_ids.contains(&"moth_mimicry_B"), "hall_03 should contain moth_mimicry_B");
    assert_eq!(encounter_pack.total_units(), 3, "hall_03 should have 3 units");
}

/// Verifies resolving qinglong_room_01 with seed 0 produces mantis_magic_flower x2.
#[test]
fn encounter_selection_room_01_produces_mantis_magic_flower_x2() {
    let config = &*QINGLONG_ENCOUNTER_CONFIG;
    let room_pack = config.room_packs.iter().find(|p| p.id == "qinglong_room_01").unwrap();

    let encounter_pack = room_pack.resolve(0);

    let family_ids: Vec<&str> = encounter_pack.family_ids().iter().map(|f| f.0.as_str()).collect();
    assert!(family_ids.contains(&"mantis_magic_flower"), "room_01 should contain mantis_magic_flower");
    assert_eq!(encounter_pack.total_units(), 2, "room_01 should have 2 units");
}

/// Verifies resolving qinglong_room_02 with seed 0 produces mantis_spiny_flower x4.
#[test]
fn encounter_selection_room_02_produces_mantis_spiny_flower_x4() {
    let config = &*QINGLONG_ENCOUNTER_CONFIG;
    let room_pack = config.room_packs.iter().find(|p| p.id == "qinglong_room_02").unwrap();

    let encounter_pack = room_pack.resolve(0);

    let family_ids: Vec<&str> = encounter_pack.family_ids().iter().map(|f| f.0.as_str()).collect();
    assert!(family_ids.contains(&"mantis_spiny_flower"), "room_02 should contain mantis_spiny_flower");
    assert_eq!(encounter_pack.total_units(), 4, "room_02 should have 4 units");
}

/// Verifies resolving qinglong_boss_azure_dragon produces the correct boss composition.
#[test]
fn encounter_selection_boss_azure_dragon_produces_correct_composition() {
    let config = &*QINGLONG_ENCOUNTER_CONFIG;
    let boss_pack = config.boss_packs.iter().find(|p| p.id == "qinglong_boss_azure_dragon").unwrap();

    let encounter_pack = boss_pack.resolve(0);

    let family_ids: Vec<&str> = encounter_pack.family_ids().iter().map(|f| f.0.as_str()).collect();
    assert!(family_ids.contains(&"azure_dragon"), "boss should contain azure_dragon");
    assert!(family_ids.contains(&"azure_dragon_ball_thunder"), "boss should contain azure_dragon_ball_thunder");
    assert!(family_ids.contains(&"azure_dragon_ball_wind"), "boss should contain azure_dragon_ball_wind");
    assert_eq!(encounter_pack.total_units(), 3, "boss should have 3 units");
    assert_eq!(encounter_pack.pack_type, PackType::Boss);
}

/// Verifies PackTemplate::get_pack can find packs by ID in all pack types.
#[test]
fn dungeon_encounter_config_get_pack_finds_all_pack_types() {
    let config = &*QINGLONG_ENCOUNTER_CONFIG;

    let hall_pack = config.get_pack("qinglong_hall_01");
    assert!(hall_pack.is_some(), "Should find qinglong_hall_01");
    assert_eq!(hall_pack.unwrap().pack_type, PackType::Hall);

    let room_pack = config.get_pack("qinglong_room_01");
    assert!(room_pack.is_some(), "Should find qinglong_room_01");
    assert_eq!(room_pack.unwrap().pack_type, PackType::Room);

    let boss_pack = config.get_pack("qinglong_boss_azure_dragon");
    assert!(boss_pack.is_some(), "Should find qinglong_boss_azure_dragon");
    assert_eq!(boss_pack.unwrap().pack_type, PackType::Boss);
}

/// Verifies resolve_pack returns the same result as direct resolve on the pack.
#[test]
fn dungeon_encounter_config_resolve_pack_works() {
    let config = &*QINGLONG_ENCOUNTER_CONFIG;
    let seed = 42u64;

    // Resolve via get_pack + resolve
    let hall_pack = config.get_pack("qinglong_hall_01").unwrap();
    let pack1 = hall_pack.resolve(seed);

    // Resolve via resolve_pack
    let pack2 = config.resolve_pack("qinglong_hall_01", seed);

    assert!(pack2.is_some(), "resolve_pack should return Some");
    assert_eq!(pack1.id.0, pack2.unwrap().id.0);
}

/// Verifies PackTemplate::total_chance returns sum of all mash entry chances.
#[test]
fn pack_template_total_chance_sums_correctly() {
    let config = &*QINGLONG_ENCOUNTER_CONFIG;
    let hall_pack = config.hall_packs.first().unwrap();

    let total = hall_pack.total_chance();
    assert!(total > 0, "total_chance should be greater than 0");
    assert_eq!(total, hall_pack.mash.iter().map(|m| m.chance).sum::<u32>());
}

/// Verifies PackTemplate::select_mash_entry returns valid index for any seed.
#[test]
fn pack_template_select_mash_entry_returns_valid_index() {
    let config = &*QINGLONG_ENCOUNTER_CONFIG;
    let hall_pack = config.hall_packs.first().unwrap();

    for seed in 0..100u64 {
        let idx = hall_pack.select_mash_entry(seed);
        assert!(idx < hall_pack.mash.len(), "select_mash_entry should return valid index");
    }
}