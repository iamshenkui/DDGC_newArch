//! DungeonMapConfig integration tests.
//!
//! Verifies that the DungeonMapConfig struct and predefined dungeon configurations
//! correctly match the values extracted from MapGenerator.txt.

use game_ddgc_headless::contracts::{
    DungeonType, GridSize, MapSize, QuestType, Range,
    BAIHU_MEDIUM_EXPLORE, BAIHU_SHORT_EXPLORE,
    QINGLONG_MEDIUM_EXPLORE, QINGLONG_SHORT_EXPLORE,
    XUANWU_MEDIUM_EXPLORE, XUANWU_SHORT_EXPLORE,
    ZHUQUE_MEDIUM_EXPLORE, ZHUQUE_SHORT_EXPLORE,
    get_dungeon_config,
};

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