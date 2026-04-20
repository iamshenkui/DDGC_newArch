//! DDGC dungeon map generation parameters — contracts module.
//!
//! This module provides the [`DungeonMapConfig`] struct and predefined dungeon
//! configurations extracted from MapGenerator.txt. These parameters control room
//! counts, corridor counts, grid size, connectivity, and density values for
//! each dungeon type and size variant.
//!
//! # Dungeon Types
//!
//! - QingLong (Azure Dragon) — lowest connectivity (0.9)
//! - BaiHu (White Tiger) — lower connectivity (0.85)
//! - ZhuQue (Vermilion Bird) — highest connectivity (0.95)
//! - XuanWu (Black Tortoise) — medium connectivity (0.9)
//!
//! # Size Variants
//!
//! Each dungeon supports multiple size variants. This module provides `short`
//! and `medium` variants as specified in the acceptance criteria.

use serde::{Deserialize, Serialize};

/// Represents a min/max range pair for density and count parameters.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Range {
    pub min: u32,
    pub max: u32,
}

impl Range {
    pub const fn new(min: u32, max: u32) -> Self {
        Range { min, max }
    }
}

/// Represents a 2D grid size.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GridSize {
    pub x: u32,
    pub y: u32,
}

impl GridSize {
    pub const fn new(x: u32, y: u32) -> Self {
        GridSize { x, y }
    }
}

/// Dungeon type identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DungeonType {
    QingLong,
    BaiHu,
    ZhuQue,
    XuanWu,
}

impl DungeonType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DungeonType::QingLong => "qinglong",
            DungeonType::BaiHu => "baihu",
            DungeonType::ZhuQue => "zhuque",
            DungeonType::XuanWu => "xuanwu",
        }
    }

    /// Convert from the game-layer `Dungeon` enum to `DungeonType`.
    ///
    /// Returns `None` for `Dungeon::Cross`, which has no associated map config.
    pub fn from_dungeon(dungeon: crate::monsters::families::Dungeon) -> Option<DungeonType> {
        use crate::monsters::families::Dungeon as D;
        match dungeon {
            D::QingLong => Some(DungeonType::QingLong),
            D::BaiHu => Some(DungeonType::BaiHu),
            D::ZhuQue => Some(DungeonType::ZhuQue),
            D::XuanWu => Some(DungeonType::XuanWu),
            D::Cross => None,
        }
    }
}

/// Size variant for dungeon maps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MapSize {
    Short,
    Medium,
}

impl MapSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            MapSize::Short => "short",
            MapSize::Medium => "medium",
        }
    }
}

/// Quest type that determines map generation behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuestType {
    Explore,
    KillBoss,
    Cleanse,
    Gather,
    Activate,
    InventoryActivate,
}

impl QuestType {
    pub fn as_str(&self) -> &'static str {
        match self {
            QuestType::Explore => "explore",
            QuestType::KillBoss => "kill_boss",
            QuestType::Cleanse => "cleanse",
            QuestType::Gather => "gather",
            QuestType::Activate => "activate",
            QuestType::InventoryActivate => "inventory_activate",
        }
    }
}

/// Dungeon map generation configuration extracted from MapGenerator.txt.
///
/// This struct contains all parameters that control how a dungeon map is generated,
/// including room counts, corridor counts, grid dimensions, connectivity, and
/// density values for various room and hallway features.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DungeonMapConfig {
    /// Size variant (short, medium, long).
    pub size: MapSize,
    /// Quest type that determines map generation behavior.
    pub quest_type: QuestType,
    /// Dungeon type (QingLong, BaiHu, ZhuQue, XuanWu).
    pub dungeon_type: DungeonType,
    /// Base number of rooms in the map.
    pub base_room_number: u32,
    /// Base number of corridors in the map.
    pub base_corridor_number: u32,
    /// Grid dimensions for room placement.
    pub gridsize: GridSize,
    /// Spacing between grid cells.
    pub spacing: u32,
    /// Number of goal rooms in the map.
    pub goal_room_number: u32,
    /// Connectivity parameter (0.0 to 1.0) — higher means more interconnected.
    pub connectivity: f64,
    /// Minimum distance from start to goal room.
    pub min_final_distance: u32,
    /// Hallway battle density range.
    pub hallway_battle: Range,
    /// Hallway trap density range.
    pub hallway_trap: Range,
    /// Hallway obstacle density range.
    pub hallway_obstacle: Range,
    /// Hallway curio density range.
    pub hallway_curio: Range,
    /// Hallway hunger density range.
    pub hallway_hunger: Range,
    /// Total room battles density range.
    pub total_room_battles: Range,
    /// Room battle density range.
    pub room_battle: Range,
    /// Room guarded curio density range.
    pub room_guarded_curio: Range,
    /// Room curio density range.
    pub room_curio: Range,
    /// Room guarded treasure density range.
    pub room_guarded_treasure: Range,
    /// Room treasure density range.
    pub room_treasure: Range,
}

impl DungeonMapConfig {
    /// Create a new dungeon map config with all parameters.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        size: MapSize,
        quest_type: QuestType,
        dungeon_type: DungeonType,
        base_room_number: u32,
        base_corridor_number: u32,
        gridsize: GridSize,
        spacing: u32,
        goal_room_number: u32,
        connectivity: f64,
        min_final_distance: u32,
        hallway_battle: Range,
        hallway_trap: Range,
        hallway_obstacle: Range,
        hallway_curio: Range,
        hallway_hunger: Range,
        total_room_battles: Range,
        room_battle: Range,
        room_guarded_curio: Range,
        room_curio: Range,
        room_guarded_treasure: Range,
        room_treasure: Range,
    ) -> Self {
        DungeonMapConfig {
            size,
            quest_type,
            dungeon_type,
            base_room_number,
            base_corridor_number,
            gridsize,
            spacing,
            goal_room_number,
            connectivity,
            min_final_distance,
            hallway_battle,
            hallway_trap,
            hallway_obstacle,
            hallway_curio,
            hallway_hunger,
            total_room_battles,
            room_battle,
            room_guarded_curio,
            room_curio,
            room_guarded_treasure,
            room_treasure,
        }
    }

    /// Returns true if this config matches the given dungeon type and size.
    pub fn matches(&self, dungeon_type: DungeonType, size: MapSize) -> bool {
        self.dungeon_type == dungeon_type && self.size == size
    }

    /// Derives `max_connections` for floor generation from the `connectivity` parameter.
    ///
    /// Connectivity is a float in [0.0, 1.0] representing how interconnected the dungeon is.
    /// Higher connectivity → more connections between rooms.
    ///
    /// The formula maps the DDGC connectivity range (0.85–0.95) to max_connections (10–12),
    /// which controls how many extra random connections the room generator adds per room.
    pub fn max_connections(&self) -> u32 {
        // Map [0.85, 0.95] → [10, 12] using linear scaling:
        // max_connections = ((connectivity - 0.5) * 20.0).round() as u32 + 3
        // 0.85 → 10, 0.9 → 11, 0.95 → 12
        ((self.connectivity - 0.5) * 20.0).round() as u32 + 3
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Predefined dungeon configs extracted from MapGenerator.txt
// ─────────────────────────────────────────────────────────────────────────────

/// QingLong (Azure Dragon) dungeon configs — short variant.
pub const QINGLONG_SHORT_EXPLORE: DungeonMapConfig = DungeonMapConfig::new(
    MapSize::Short,
    QuestType::Explore,
    DungeonType::QingLong,
    9,  // base_room_number
    10, // base_corridor_number
    GridSize::new(4, 3), // gridsize
    4,  // spacing
    9,  // goal_room_number
    0.9, // connectivity
    3,  // min_final_distance
    Range::new(2, 4),  // hallway_battle
    Range::new(0, 0),  // hallway_trap
    Range::new(0, 0),  // hallway_obstacle
    Range::new(9, 9),  // hallway_curio
    Range::new(1, 3),  // hallway_hunger
    Range::new(1, 3),  // total_room_battles
    Range::new(0, 0),  // room_battle
    Range::new(0, 1),  // room_guarded_curio
    Range::new(0, 0),  // room_curio
    Range::new(1, 1),  // room_guarded_treasure
    Range::new(0, 0),  // room_treasure
);

/// QingLong (Azure Dragon) dungeon configs — medium variant.
pub const QINGLONG_MEDIUM_EXPLORE: DungeonMapConfig = DungeonMapConfig::new(
    MapSize::Medium,
    QuestType::Explore,
    DungeonType::QingLong,
    14, // base_room_number
    15, // base_corridor_number
    GridSize::new(5, 4), // gridsize
    4,  // spacing
    14, // goal_room_number
    0.9, // connectivity
    7,  // min_final_distance
    Range::new(3, 4),  // hallway_battle
    Range::new(0, 0),  // hallway_trap
    Range::new(0, 0),  // hallway_obstacle
    Range::new(14, 14), // hallway_curio
    Range::new(2, 5),  // hallway_hunger
    Range::new(3, 4),  // total_room_battles
    Range::new(0, 0),  // room_battle
    Range::new(1, 2),  // room_guarded_curio
    Range::new(0, 0),  // room_curio
    Range::new(1, 2),  // room_guarded_treasure
    Range::new(0, 0),  // room_treasure
);

/// BaiHu (White Tiger) dungeon configs — short variant.
pub const BAIHU_SHORT_EXPLORE: DungeonMapConfig = DungeonMapConfig::new(
    MapSize::Short,
    QuestType::Explore,
    DungeonType::BaiHu,
    9,  // base_room_number
    10, // base_corridor_number
    GridSize::new(4, 4), // gridsize
    4,  // spacing
    9,  // goal_room_number
    0.85, // connectivity
    3,  // min_final_distance
    Range::new(2, 4),  // hallway_battle
    Range::new(0, 0),  // hallway_trap
    Range::new(0, 0),  // hallway_obstacle
    Range::new(9, 9),  // hallway_curio
    Range::new(1, 3),  // hallway_hunger
    Range::new(1, 3),  // total_room_battles
    Range::new(0, 0),  // room_battle
    Range::new(0, 1),  // room_guarded_curio
    Range::new(0, 0),  // room_curio
    Range::new(1, 1),  // room_guarded_treasure
    Range::new(0, 0),  // room_treasure
);

/// BaiHu (White Tiger) dungeon configs — medium variant.
pub const BAIHU_MEDIUM_EXPLORE: DungeonMapConfig = DungeonMapConfig::new(
    MapSize::Medium,
    QuestType::Explore,
    DungeonType::BaiHu,
    14, // base_room_number
    15, // base_corridor_number
    GridSize::new(5, 5), // gridsize
    4,  // spacing
    14, // goal_room_number
    0.85, // connectivity
    7,  // min_final_distance
    Range::new(3, 4),  // hallway_battle
    Range::new(0, 0),  // hallway_trap
    Range::new(0, 0),  // hallway_obstacle
    Range::new(14, 14), // hallway_curio
    Range::new(2, 5),  // hallway_hunger
    Range::new(3, 4),  // total_room_battles
    Range::new(0, 0),  // room_battle
    Range::new(1, 2),  // room_guarded_curio
    Range::new(0, 0),  // room_curio
    Range::new(1, 2),  // room_guarded_treasure
    Range::new(0, 0),  // room_treasure
);

/// ZhuQue (Vermilion Bird) dungeon configs — short variant.
pub const ZHUQUE_SHORT_EXPLORE: DungeonMapConfig = DungeonMapConfig::new(
    MapSize::Short,
    QuestType::Explore,
    DungeonType::ZhuQue,
    9,  // base_room_number
    10, // base_corridor_number
    GridSize::new(4, 3), // gridsize
    4,  // spacing
    9,  // goal_room_number
    0.95, // connectivity
    3,  // min_final_distance
    Range::new(2, 4),  // hallway_battle
    Range::new(0, 0),  // hallway_trap
    Range::new(0, 0),  // hallway_obstacle
    Range::new(9, 9),  // hallway_curio
    Range::new(1, 3),  // hallway_hunger
    Range::new(1, 3),  // total_room_battles
    Range::new(0, 0),  // room_battle
    Range::new(0, 1),  // room_guarded_curio
    Range::new(0, 0),  // room_curio
    Range::new(1, 1),  // room_guarded_treasure
    Range::new(0, 0),  // room_treasure
);

/// ZhuQue (Vermilion Bird) dungeon configs — medium variant.
pub const ZHUQUE_MEDIUM_EXPLORE: DungeonMapConfig = DungeonMapConfig::new(
    MapSize::Medium,
    QuestType::Explore,
    DungeonType::ZhuQue,
    14, // base_room_number
    15, // base_corridor_number
    GridSize::new(6, 3), // gridsize
    4,  // spacing
    14, // goal_room_number
    0.95, // connectivity
    7,  // min_final_distance
    Range::new(3, 4),  // hallway_battle
    Range::new(0, 0),  // hallway_trap
    Range::new(0, 0),  // hallway_obstacle
    Range::new(14, 14), // hallway_curio
    Range::new(2, 5),  // hallway_hunger
    Range::new(3, 4),  // total_room_battles
    Range::new(0, 0),  // room_battle
    Range::new(1, 2),  // room_guarded_curio
    Range::new(0, 0),  // room_curio
    Range::new(1, 2),  // room_guarded_treasure
    Range::new(0, 0),  // room_treasure
);

/// XuanWu (Black Tortoise) dungeon configs — short variant.
pub const XUANWU_SHORT_EXPLORE: DungeonMapConfig = DungeonMapConfig::new(
    MapSize::Short,
    QuestType::Explore,
    DungeonType::XuanWu,
    9,  // base_room_number
    10, // base_corridor_number
    GridSize::new(4, 4), // gridsize
    4,  // spacing
    9,  // goal_room_number
    0.9, // connectivity
    3,  // min_final_distance
    Range::new(2, 4),  // hallway_battle
    Range::new(0, 0),  // hallway_trap
    Range::new(0, 0),  // hallway_obstacle
    Range::new(9, 9),  // hallway_curio
    Range::new(1, 3),  // hallway_hunger
    Range::new(1, 3),  // total_room_battles
    Range::new(0, 0),  // room_battle
    Range::new(0, 1),  // room_guarded_curio
    Range::new(0, 0),  // room_curio
    Range::new(1, 1),  // room_guarded_treasure
    Range::new(0, 0),  // room_treasure
);

/// XuanWu (Black Tortoise) dungeon configs — medium variant.
pub const XUANWU_MEDIUM_EXPLORE: DungeonMapConfig = DungeonMapConfig::new(
    MapSize::Medium,
    QuestType::Explore,
    DungeonType::XuanWu,
    14, // base_room_number
    15, // base_corridor_number
    GridSize::new(5, 5), // gridsize
    4,  // spacing
    14, // goal_room_number
    0.9, // connectivity
    7,  // min_final_distance
    Range::new(3, 4),  // hallway_battle
    Range::new(0, 0),  // hallway_trap
    Range::new(0, 0),  // hallway_obstacle
    Range::new(14, 14), // hallway_curio
    Range::new(2, 5),  // hallway_hunger
    Range::new(3, 4),  // total_room_battles
    Range::new(0, 0),  // room_battle
    Range::new(1, 2),  // room_guarded_curio
    Range::new(0, 0),  // room_curio
    Range::new(1, 2),  // room_guarded_treasure
    Range::new(0, 0),  // room_treasure
);

/// Returns the dungeon config for a given dungeon type and size.
pub fn get_dungeon_config(dungeon_type: DungeonType, size: MapSize) -> Option<&'static DungeonMapConfig> {
    match (dungeon_type, size) {
        (DungeonType::QingLong, MapSize::Short) => Some(&QINGLONG_SHORT_EXPLORE),
        (DungeonType::QingLong, MapSize::Medium) => Some(&QINGLONG_MEDIUM_EXPLORE),
        (DungeonType::BaiHu, MapSize::Short) => Some(&BAIHU_SHORT_EXPLORE),
        (DungeonType::BaiHu, MapSize::Medium) => Some(&BAIHU_MEDIUM_EXPLORE),
        (DungeonType::ZhuQue, MapSize::Short) => Some(&ZHUQUE_SHORT_EXPLORE),
        (DungeonType::ZhuQue, MapSize::Medium) => Some(&ZHUQUE_MEDIUM_EXPLORE),
        (DungeonType::XuanWu, MapSize::Short) => Some(&XUANWU_SHORT_EXPLORE),
        (DungeonType::XuanWu, MapSize::Medium) => Some(&XUANWU_MEDIUM_EXPLORE),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_new() {
        let range = Range::new(2, 4);
        assert_eq!(range.min, 2);
        assert_eq!(range.max, 4);
    }

    #[test]
    fn grid_size_new() {
        let grid = GridSize::new(4, 3);
        assert_eq!(grid.x, 4);
        assert_eq!(grid.y, 3);
    }

    #[test]
    fn dungeon_type_as_str() {
        assert_eq!(DungeonType::QingLong.as_str(), "qinglong");
        assert_eq!(DungeonType::BaiHu.as_str(), "baihu");
        assert_eq!(DungeonType::ZhuQue.as_str(), "zhuque");
        assert_eq!(DungeonType::XuanWu.as_str(), "xuanwu");
    }

    #[test]
    fn map_size_as_str() {
        assert_eq!(MapSize::Short.as_str(), "short");
        assert_eq!(MapSize::Medium.as_str(), "medium");
    }

    #[test]
    fn quest_type_as_str() {
        assert_eq!(QuestType::Explore.as_str(), "explore");
        assert_eq!(QuestType::KillBoss.as_str(), "kill_boss");
        assert_eq!(QuestType::Cleanse.as_str(), "cleanse");
        assert_eq!(QuestType::Gather.as_str(), "gather");
        assert_eq!(QuestType::Activate.as_str(), "activate");
        assert_eq!(QuestType::InventoryActivate.as_str(), "inventory_activate");
    }

    #[test]
    fn dungeon_map_config_matches() {
        assert!(QINGLONG_SHORT_EXPLORE.matches(DungeonType::QingLong, MapSize::Short));
        assert!(!QINGLONG_SHORT_EXPLORE.matches(DungeonType::QingLong, MapSize::Medium));
        assert!(!QINGLONG_SHORT_EXPLORE.matches(DungeonType::BaiHu, MapSize::Short));
    }

    #[test]
    fn get_dungeon_config_returns_correct_config() {
        let config = get_dungeon_config(DungeonType::QingLong, MapSize::Short);
        assert!(config.is_some());
        assert_eq!(config.unwrap().dungeon_type, DungeonType::QingLong);
        assert_eq!(config.unwrap().size, MapSize::Short);
    }

    // ── US-809-a: MapGenerator.txt value verification tests ─────────────────────

    #[test]
    fn qinglong_short_explore_matches_mapgenerator_values() {
        // Extracted from MapGenerator.txt: qinglong, short, explore
        let config = QINGLONG_SHORT_EXPLORE;
        assert_eq!(config.base_room_number, 9);
        assert_eq!(config.base_corridor_number, 10);
        assert_eq!(config.gridsize.x, 4);
        assert_eq!(config.gridsize.y, 3);
        assert_eq!(config.connectivity, 0.9);
        assert_eq!(config.min_final_distance, 3);
        assert_eq!(config.hallway_battle.min, 2);
        assert_eq!(config.hallway_battle.max, 4);
        assert_eq!(config.hallway_curio.min, 9);
        assert_eq!(config.hallway_curio.max, 9);
    }

    #[test]
    fn qinglong_medium_explore_matches_mapgenerator_values() {
        // Extracted from MapGenerator.txt: qinglong, medium, explore
        let config = QINGLONG_MEDIUM_EXPLORE;
        assert_eq!(config.base_room_number, 14);
        assert_eq!(config.base_corridor_number, 15);
        assert_eq!(config.gridsize.x, 5);
        assert_eq!(config.gridsize.y, 4);
        assert_eq!(config.connectivity, 0.9);
        assert_eq!(config.min_final_distance, 7);
        assert_eq!(config.hallway_battle.min, 3);
        assert_eq!(config.hallway_battle.max, 4);
    }

    #[test]
    fn baihu_short_explore_matches_mapgenerator_values() {
        // Extracted from MapGenerator.txt: baihu, short, explore
        let config = BAIHU_SHORT_EXPLORE;
        assert_eq!(config.base_room_number, 9);
        assert_eq!(config.base_corridor_number, 10);
        assert_eq!(config.gridsize.x, 4);
        assert_eq!(config.gridsize.y, 4);
        assert_eq!(config.connectivity, 0.85);
        assert_eq!(config.min_final_distance, 3);
        // BaiHu has lower connectivity than QingLong (0.85 vs 0.9)
        assert!(config.connectivity < QINGLONG_SHORT_EXPLORE.connectivity);
    }

    #[test]
    fn baihu_medium_explore_matches_mapgenerator_values() {
        // Extracted from MapGenerator.txt: baihu, medium, explore
        let config = BAIHU_MEDIUM_EXPLORE;
        assert_eq!(config.base_room_number, 14);
        assert_eq!(config.base_corridor_number, 15);
        assert_eq!(config.gridsize.x, 5);
        assert_eq!(config.gridsize.y, 5);
        assert_eq!(config.connectivity, 0.85);
        assert_eq!(config.min_final_distance, 7);
    }

    #[test]
    fn zhuque_short_explore_matches_mapgenerator_values() {
        // Extracted from MapGenerator.txt: zhuque, short, explore
        let config = ZHUQUE_SHORT_EXPLORE;
        assert_eq!(config.base_room_number, 9);
        assert_eq!(config.base_corridor_number, 10);
        assert_eq!(config.gridsize.x, 4);
        assert_eq!(config.gridsize.y, 3);
        assert_eq!(config.connectivity, 0.95);
        assert_eq!(config.min_final_distance, 3);
        // ZhuQue has highest connectivity (0.95)
        assert!(config.connectivity > QINGLONG_SHORT_EXPLORE.connectivity);
        assert!(config.connectivity > BAIHU_SHORT_EXPLORE.connectivity);
    }

    #[test]
    fn zhuque_medium_explore_matches_mapgenerator_values() {
        // Extracted from MapGenerator.txt: zhuque, medium, explore
        let config = ZHUQUE_MEDIUM_EXPLORE;
        assert_eq!(config.base_room_number, 14);
        assert_eq!(config.base_corridor_number, 15);
        assert_eq!(config.gridsize.x, 6);
        assert_eq!(config.gridsize.y, 3);
        assert_eq!(config.connectivity, 0.95);
        assert_eq!(config.min_final_distance, 7);
    }

    #[test]
    fn xuanwu_short_explore_matches_mapgenerator_values() {
        // Extracted from MapGenerator.txt: xuanwu, short, explore
        let config = XUANWU_SHORT_EXPLORE;
        assert_eq!(config.base_room_number, 9);
        assert_eq!(config.base_corridor_number, 10);
        assert_eq!(config.gridsize.x, 4);
        assert_eq!(config.gridsize.y, 4);
        assert_eq!(config.connectivity, 0.9);
        assert_eq!(config.min_final_distance, 3);
    }

    #[test]
    fn xuanwu_medium_explore_matches_mapgenerator_values() {
        // Extracted from MapGenerator.txt: xuanwu, medium, explore
        let config = XUANWU_MEDIUM_EXPLORE;
        assert_eq!(config.base_room_number, 14);
        assert_eq!(config.base_corridor_number, 15);
        assert_eq!(config.gridsize.x, 5);
        assert_eq!(config.gridsize.y, 5);
        assert_eq!(config.connectivity, 0.9);
        assert_eq!(config.min_final_distance, 7);
    }

    #[test]
    fn short_vs_medium_differs_in_room_count() {
        // Short variants have fewer rooms than medium variants
        assert!(QINGLONG_SHORT_EXPLORE.base_room_number < QINGLONG_MEDIUM_EXPLORE.base_room_number);
        assert!(BAIHU_SHORT_EXPLORE.base_room_number < BAIHU_MEDIUM_EXPLORE.base_room_number);
        assert!(ZHUQUE_SHORT_EXPLORE.base_room_number < ZHUQUE_MEDIUM_EXPLORE.base_room_number);
        assert!(XUANWU_SHORT_EXPLORE.base_room_number < XUANWU_MEDIUM_EXPLORE.base_room_number);
    }

    #[test]
    fn all_dungeons_have_correct_connectivity_ranking() {
        // ZhuQue (0.95) > QingLong (0.9) = XuanWu (0.9) > BaiHu (0.85)
        assert!(ZHUQUE_SHORT_EXPLORE.connectivity > QINGLONG_SHORT_EXPLORE.connectivity);
        assert_eq!(QINGLONG_SHORT_EXPLORE.connectivity, XUANWU_SHORT_EXPLORE.connectivity);
        assert!(BAIHU_SHORT_EXPLORE.connectivity < QINGLONG_SHORT_EXPLORE.connectivity);
    }

    #[test]
    fn traps_and_obstacles_are_zero_for_all_configs() {
        // MapGenerator.txt shows all hallway_trap and hallway_obstacle are 0 0
        for config in [
            QINGLONG_SHORT_EXPLORE,
            QINGLONG_MEDIUM_EXPLORE,
            BAIHU_SHORT_EXPLORE,
            BAIHU_MEDIUM_EXPLORE,
            ZHUQUE_SHORT_EXPLORE,
            ZHUQUE_MEDIUM_EXPLORE,
            XUANWU_SHORT_EXPLORE,
            XUANWU_MEDIUM_EXPLORE,
        ] {
            assert_eq!(config.hallway_trap.min, 0);
            assert_eq!(config.hallway_trap.max, 0);
            assert_eq!(config.hallway_obstacle.min, 0);
            assert_eq!(config.hallway_obstacle.max, 0);
        }
    }

    #[test]
    fn max_connections_derives_correctly_from_connectivity() {
        // Formula: ((connectivity - 0.5) * 20.0).round() as u32 + 3
        // BaiHu (0.85) → 10, QingLong (0.9) → 11, ZhuQue (0.95) → 12
        assert_eq!(BAIHU_SHORT_EXPLORE.max_connections(), 10);
        assert_eq!(QINGLONG_SHORT_EXPLORE.max_connections(), 11);
        assert_eq!(ZHUQUE_SHORT_EXPLORE.max_connections(), 12);
    }

    #[test]
    fn baihu_has_lower_max_connections_than_zhuque() {
        // Since BaiHu connectivity (0.85) < ZhuQue connectivity (0.95),
        // BaiHu max_connections (5) < ZhuQue max_connections (6)
        assert!(BAIHU_SHORT_EXPLORE.max_connections() < ZHUQUE_SHORT_EXPLORE.max_connections());
    }

    #[test]
    fn dungeon_type_from_dungeon_converts_correctly() {
        use crate::monsters::families::Dungeon;
        assert_eq!(DungeonType::from_dungeon(Dungeon::QingLong), Some(DungeonType::QingLong));
        assert_eq!(DungeonType::from_dungeon(Dungeon::BaiHu), Some(DungeonType::BaiHu));
        assert_eq!(DungeonType::from_dungeon(Dungeon::ZhuQue), Some(DungeonType::ZhuQue));
        assert_eq!(DungeonType::from_dungeon(Dungeon::XuanWu), Some(DungeonType::XuanWu));
        assert_eq!(DungeonType::from_dungeon(Dungeon::Cross), None);
    }
}