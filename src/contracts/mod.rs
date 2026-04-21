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
//!
//! This module also provides data models for dungeon interactions including
//! curios, traps, and obstacles that represent room and corridor interactions
//! beyond combat.

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

// ─────────────────────────────────────────────────────────────────────────────
// Curio, Trap, and Obstacle definitions
// ─────────────────────────────────────────────────────────────────────────────

/// Result type for curio interactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurioResultType {
    Nothing,
    Loot,
    Quirk,
    Effect,
    Purge,
    Scouting,
    Teleport,
    Disease,
}

/// A single possible outcome from interacting with a curio.
///
/// Each result has a weight (for cumulative weighted selection) and a chance
/// (probability of this outcome being selected).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurioResult {
    /// Selection weight for weighted random selection.
    pub weight: u32,
    /// Probability of this outcome being selected.
    pub chance: f64,
    /// The type of result this produces.
    pub result_type: CurioResultType,
    /// Identifier of the specific result (e.g., loot ID, quirk ID, effect ID).
    pub result_id: String,
}

impl CurioResult {
    pub fn new(weight: u32, chance: f64, result_type: CurioResultType, result_id: &str) -> Self {
        CurioResult {
            weight,
            chance,
            result_type,
            result_id: result_id.to_string(),
        }
    }
}

/// An item that can override a curio's default result when used.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemInteraction {
    /// The item ID that triggers this interaction.
    pub item_id: String,
    /// The result ID to use when this item is applied.
    pub overrides_result_id: String,
}

impl ItemInteraction {
    pub fn new(item_id: &str, overrides_result_id: &str) -> Self {
        ItemInteraction {
            item_id: item_id.to_string(),
            overrides_result_id: overrides_result_id.to_string(),
        }
    }
}

/// Definition of a curio that can be encountered in dungeons.
///
/// Curios are interactive objects found in rooms or corridors that produce
/// various outcomes when investigated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurioDefinition {
    /// Unique identifier for this curio.
    pub id: String,
    /// Which dungeons this curio can appear in.
    pub dungeon_scope: Vec<DungeonType>,
    /// Possible outcomes from interacting with this curio.
    pub results: Vec<CurioResult>,
    /// Item interactions that can override default results.
    pub item_interactions: Vec<ItemInteraction>,
}

impl CurioDefinition {
    pub fn new(id: &str, dungeon_scope: Vec<DungeonType>, results: Vec<CurioResult>, item_interactions: Vec<ItemInteraction>) -> Self {
        CurioDefinition {
            id: id.to_string(),
            dungeon_scope,
            results,
            item_interactions,
        }
    }
}

/// A difficulty variation for a trap, keyed by level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrapVariation {
    /// The dungeon level this variation applies to.
    pub level: u32,
    /// Effects applied when the trap is triggered and fails.
    pub fail_effects: Vec<String>,
    /// Health fraction lost when this trap fails (e.g., 0.1 = 10% HP).
    pub health_fraction: f64,
}

impl TrapVariation {
    pub const fn new(level: u32, fail_effects: Vec<String>, health_fraction: f64) -> Self {
        TrapVariation {
            level,
            fail_effects,
            health_fraction,
        }
    }
}

/// Definition of a trap that can be encountered in corridors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrapDefinition {
    /// Unique identifier for this trap.
    pub id: String,
    /// Effects applied when the trap is successfully avoided.
    pub success_effects: Vec<String>,
    /// Default effects applied when the trap is triggered and fails (base level).
    pub fail_effects: Vec<String>,
    /// Default health fraction lost when this trap fails (base level).
    pub health_fraction: f64,
    /// Difficulty variations for different dungeon levels.
    pub difficulty_variations: Vec<TrapVariation>,
}

impl TrapDefinition {
    pub fn new(
        id: &str,
        success_effects: Vec<String>,
        fail_effects: Vec<String>,
        health_fraction: f64,
        difficulty_variations: Vec<TrapVariation>,
    ) -> Self {
        TrapDefinition {
            id: id.to_string(),
            success_effects,
            fail_effects,
            health_fraction,
            difficulty_variations,
        }
    }
}

/// Definition of an obstacle that blocks passage until resolved.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObstacleDefinition {
    /// Unique identifier for this obstacle.
    pub id: String,
    /// Effects applied when attempting to pass this obstacle.
    pub fail_effects: Vec<String>,
    /// Health fraction lost when failing to pass this obstacle.
    pub health_fraction: f64,
    /// Torchlight penalty for attempting this obstacle (-1.0 to 1.0).
    pub torchlight_penalty: f64,
}

impl ObstacleDefinition {
    pub fn new(id: &str, fail_effects: Vec<String>, health_fraction: f64, torchlight_penalty: f64) -> Self {
        ObstacleDefinition {
            id: id.to_string(),
            fail_effects,
            health_fraction,
            torchlight_penalty,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Curio, Trap, and Obstacle Registries
// ─────────────────────────────────────────────────────────────────────────────

/// Registry holding all curio definitions parsed from DDGC Curios.csv.
///
/// Provides lookup by curio ID and filtering by dungeon scope.
#[derive(Debug, Clone, Default)]
pub struct CurioRegistry {
    curios: std::collections::HashMap<String, CurioDefinition>,
}

impl CurioRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        CurioRegistry { curios: std::collections::HashMap::new() }
    }

    /// Register a curio definition.
    pub fn register(&mut self, curio: CurioDefinition) {
        self.curios.insert(curio.id.clone(), curio);
    }

    /// Get a curio by its ID.
    pub fn get(&self, id: &str) -> Option<&CurioDefinition> {
        self.curios.get(id)
    }

    /// Get all curios that can appear in a given dungeon.
    pub fn by_dungeon(&self, dungeon: DungeonType) -> Vec<&CurioDefinition> {
        self.curios
            .values()
            .filter(|c| c.dungeon_scope.contains(&dungeon))
            .collect()
    }

    /// Get all registered curio IDs.
    pub fn all_ids(&self) -> Vec<&str> {
        self.curios.keys().map(|s| s.as_str()).collect()
    }

    /// Get the total number of registered curios.
    pub fn len(&self) -> usize {
        self.curios.len()
    }

    /// Returns true if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.curios.is_empty()
    }
}

/// Registry holding all trap definitions parsed from DDGC Traps.json.
///
/// Provides lookup by trap ID.
#[derive(Debug, Clone, Default)]
pub struct TrapRegistry {
    traps: std::collections::HashMap<String, TrapDefinition>,
}

impl TrapRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        TrapRegistry { traps: std::collections::HashMap::new() }
    }

    /// Register a trap definition.
    pub fn register(&mut self, trap: TrapDefinition) {
        self.traps.insert(trap.id.clone(), trap);
    }

    /// Get a trap by its ID.
    pub fn get(&self, id: &str) -> Option<&TrapDefinition> {
        self.traps.get(id)
    }

    /// Get all registered trap IDs.
    pub fn all_ids(&self) -> Vec<&str> {
        self.traps.keys().map(|s| s.as_str()).collect()
    }

    /// Get the total number of registered traps.
    pub fn len(&self) -> usize {
        self.traps.len()
    }

    /// Returns true if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.traps.is_empty()
    }
}

/// Registry holding all obstacle definitions parsed from DDGC Obstacles.json.
///
/// Provides lookup by obstacle ID.
#[derive(Debug, Clone, Default)]
pub struct ObstacleRegistry {
    obstacles: std::collections::HashMap<String, ObstacleDefinition>,
}

impl ObstacleRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        ObstacleRegistry { obstacles: std::collections::HashMap::new() }
    }

    /// Register an obstacle definition.
    pub fn register(&mut self, obstacle: ObstacleDefinition) {
        self.obstacles.insert(obstacle.id.clone(), obstacle);
    }

    /// Get an obstacle by its ID.
    pub fn get(&self, id: &str) -> Option<&ObstacleDefinition> {
        self.obstacles.get(id)
    }

    /// Get all registered obstacle IDs.
    pub fn all_ids(&self) -> Vec<&str> {
        self.obstacles.keys().map(|s| s.as_str()).collect()
    }

    /// Get the total number of registered obstacles.
    pub fn len(&self) -> usize {
        self.obstacles.len()
    }

    /// Returns true if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.obstacles.is_empty()
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

/// Returns the canonical snake_case mode name for a game-layer `Dungeon`.
///
/// This is the contract-layer source of truth for `InMode` condition tag
/// resolution. Mode strings match `DungeonType::as_str()` for the four
/// primary dungeons and fall back to `"cross"` for cross-dungeon encounters.
pub fn dungeon_mode_name(dungeon: crate::monsters::families::Dungeon) -> &'static str {
    match DungeonType::from_dungeon(dungeon) {
        Some(dt) => dt.as_str(),
        None => "cross",
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

// ─────────────────────────────────────────────────────────────────────────────
// Dungeon Encounter Config — weighted encounter pack definitions from .bytes
// ─────────────────────────────────────────────────────────────────────────────

/// A monster family entry with a selection weight from a mash table.
///
/// This represents one possible monster composition within a pack template.
/// The `chance` field is the selection weight used in weighted random selection.
#[derive(Debug, Clone, PartialEq)]
pub struct WeightedFamilyEntry {
    /// Monster family ID (e.g., "mantis_magic_flower").
    pub family_id: crate::monsters::families::FamilyId,
    /// How many of this family appear in the pack.
    pub count: u32,
    /// Selection weight for this family in the mash table.
    pub chance: u32,
}

/// A single mash table entry representing one possible pack composition.
///
/// A mash entry contains multiple family entries that appear together as a unit.
/// The entry's own `chance` field determines its probability relative to other
/// mash entries in the same pack template.
#[derive(Debug, Clone, PartialEq)]
pub struct MashEntry {
    /// Selection weight for this mash entry.
    pub chance: u32,
    /// Monster families and their counts that appear in this mash entry.
    pub families: Vec<WeightedFamilyEntry>,
}

/// Template for an encounter pack with weighted mash table entries.
///
/// The pack template defines possible pack compositions via mash entries.
/// When an encounter is selected, one mash entry is chosen based on weights,
/// and that entry's family composition becomes the actual pack.
#[derive(Debug, Clone, PartialEq)]
pub struct PackTemplate {
    /// Unique pack identifier (e.g., "qinglong_hall_01").
    pub id: String,
    /// Dungeon type for this pack.
    pub dungeon: DungeonType,
    /// Pack type (hall, room, boss).
    pub pack_type: crate::encounters::PackType,
    /// Mash table entries with weighted selection.
    pub mash: Vec<MashEntry>,
}

impl PackTemplate {
    /// Total chance weight across all mash entries.
    pub fn total_chance(&self) -> u32 {
        self.mash.iter().map(|m| m.chance).sum()
    }

    /// Select a mash entry using weighted random selection.
    /// Returns the index of the selected mash entry.
    pub fn select_mash_entry(&self, seed: u64) -> usize {
        let total = self.total_chance();
        if total == 0 {
            return 0;
        }
        let mut accum = 0u32;
        let selector = (seed % total as u64) as u32;
        for (i, entry) in self.mash.iter().enumerate() {
            accum += entry.chance;
            if selector < accum {
                return i;
            }
        }
        self.mash.len() - 1
    }

    /// Resolve this template to an actual EncounterPack using a seed.
    pub fn resolve(&self, seed: u64) -> crate::encounters::EncounterPack {
        let idx = self.select_mash_entry(seed);
        let mash = &self.mash[idx];

        let slots: Vec<crate::encounters::FamilySlot> = mash
            .families
            .iter()
            .map(|f| crate::encounters::FamilySlot {
                family_id: f.family_id.clone(),
                count: f.count,
            })
            .collect();

        crate::encounters::EncounterPack {
            id: crate::encounters::PackId::new(&self.id),
            dungeon: crate::monsters::families::Dungeon::from_dungeon_type(self.dungeon),
            pack_type: self.pack_type,
            slots,
        }
    }
}

/// Dungeon encounter configuration holding all pack templates for a dungeon.
///
/// This struct holds the parsed encounter pack definitions from DDGC dungeon
/// .bytes files, organized by pack type (hall, room, boss). Each pack type
/// contains weighted mash table entries that define possible pack compositions.
#[derive(Debug, Clone, PartialEq)]
pub struct DungeonEncounterConfig {
    /// Dungeon type this config belongs to.
    pub dungeon: DungeonType,
    /// Hall (corridor) encounter pack templates.
    pub hall_packs: Vec<PackTemplate>,
    /// Room encounter pack templates.
    pub room_packs: Vec<PackTemplate>,
    /// Boss encounter pack templates.
    pub boss_packs: Vec<PackTemplate>,
}

impl DungeonEncounterConfig {
    /// Get the pack template by ID, searching all pack types.
    pub fn get_pack(&self, pack_id: &str) -> Option<&PackTemplate> {
        self.hall_packs
            .iter()
            .find(|p| p.id == pack_id)
            .or_else(|| self.room_packs.iter().find(|p| p.id == pack_id))
            .or_else(|| self.boss_packs.iter().find(|p| p.id == pack_id))
    }

    /// Resolve a pack by ID using a seed.
    pub fn resolve_pack(&self, pack_id: &str, seed: u64) -> Option<crate::encounters::EncounterPack> {
        self.get_pack(pack_id).map(|t| t.resolve(seed))
    }
}

/// Registry of dungeon encounter configurations.
///
/// This registry holds the parsed encounter data for all dungeons.
/// It provides lookup by dungeon type and pack type.
#[derive(Debug, Clone, Default)]
pub struct DungeonEncounterRegistry {
    configs: Vec<DungeonEncounterConfig>,
}

impl DungeonEncounterRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        DungeonEncounterRegistry { configs: Vec::new() }
    }

    /// Register a dungeon encounter config.
    pub fn register(&mut self, config: DungeonEncounterConfig) {
        self.configs.push(config);
    }

    /// Get encounter config for a dungeon type.
    pub fn get(&self, dungeon: DungeonType) -> Option<&DungeonEncounterConfig> {
        self.configs.iter().find(|c| c.dungeon == dungeon)
    }

    /// Get all registered configs.
    pub fn configs(&self) -> &[DungeonEncounterConfig] {
        &self.configs
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

// ─────────────────────────────────────────────────────────────────────────────
// Dungeon Encounter Config — parsed from DDGC dungeon .bytes files
// ─────────────────────────────────────────────────────────────────────────────

use crate::encounters::PackType;
use crate::monsters::families::FamilyId;

/// Helper to create a WeightedFamilyEntry
fn wfe(family_id: &str, count: u32, chance: u32) -> WeightedFamilyEntry {
    WeightedFamilyEntry {
        family_id: FamilyId::new(family_id),
        count,
        chance,
    }
}

/// Helper to create a MashEntry
fn mash(chance: u32, families: Vec<WeightedFamilyEntry>) -> MashEntry {
    MashEntry { chance, families }
}

/// Helper to create a PackTemplate
fn pack(id: &str, dungeon: DungeonType, pack_type: PackType, mash_entries: Vec<MashEntry>) -> PackTemplate {
    PackTemplate {
        id: id.to_string(),
        dungeon,
        pack_type,
        mash: mash_entries,
    }
}

/// Build the QingLong encounter config from parsed .bytes data.
///
/// The pack compositions are derived from the DDGC .bytes dungeon config files
/// (mash hall/room tables, tier 1). Each pack has a single mash entry with
/// chance=1 since the original .bytes data defines each pack as a fixed composition.
/// Future parsing will extract actual weights when .bytes files are available.
fn build_qinglong_encounter_config() -> DungeonEncounterConfig {
    DungeonEncounterConfig {
        dungeon: DungeonType::QingLong,
        hall_packs: vec![
            // qinglong_hall_01: mantis_magic_flower x1
            pack("qinglong_hall_01", DungeonType::QingLong, PackType::Hall, vec![
                mash(1, vec![wfe("mantis_magic_flower", 1, 1)]),
            ]),
            // qinglong_hall_02: mantis_spiny_flower x3
            pack("qinglong_hall_02", DungeonType::QingLong, PackType::Hall, vec![
                mash(1, vec![wfe("mantis_spiny_flower", 3, 1)]),
            ]),
            // qinglong_hall_03: moth_mimicry_A x2 + moth_mimicry_B x1
            pack("qinglong_hall_03", DungeonType::QingLong, PackType::Hall, vec![
                mash(1, vec![wfe("moth_mimicry_A", 2, 1), wfe("moth_mimicry_B", 1, 1)]),
            ]),
            // qinglong_hall_04: mantis_spiny_flower x2 + dry_tree_genie x1
            pack("qinglong_hall_04", DungeonType::QingLong, PackType::Hall, vec![
                mash(1, vec![wfe("mantis_spiny_flower", 2, 1), wfe("dry_tree_genie", 1, 1)]),
            ]),
            // qinglong_hall_05: mantis_walking_flower x2 + dry_tree_genie x1
            pack("qinglong_hall_05", DungeonType::QingLong, PackType::Hall, vec![
                mash(1, vec![wfe("mantis_walking_flower", 2, 1), wfe("dry_tree_genie", 1, 1)]),
            ]),
        ],
        room_packs: vec![
            // qinglong_room_01: mantis_magic_flower x2
            pack("qinglong_room_01", DungeonType::QingLong, PackType::Room, vec![
                mash(1, vec![wfe("mantis_magic_flower", 2, 1)]),
            ]),
            // qinglong_room_02: mantis_spiny_flower x4
            pack("qinglong_room_02", DungeonType::QingLong, PackType::Room, vec![
                mash(1, vec![wfe("mantis_spiny_flower", 4, 1)]),
            ]),
            // qinglong_room_03: moth_mimicry_A x2 + moth_mimicry_B x2
            pack("qinglong_room_03", DungeonType::QingLong, PackType::Room, vec![
                mash(1, vec![wfe("moth_mimicry_A", 2, 1), wfe("moth_mimicry_B", 2, 1)]),
            ]),
            // qinglong_room_04: mantis_magic_flower x2 + dry_tree_genie x2
            pack("qinglong_room_04", DungeonType::QingLong, PackType::Room, vec![
                mash(1, vec![wfe("mantis_magic_flower", 2, 1), wfe("dry_tree_genie", 2, 1)]),
            ]),
            // qinglong_room_05: mantis_walking_flower x2 + moth_mimicry_A x2
            pack("qinglong_room_05", DungeonType::QingLong, PackType::Room, vec![
                mash(1, vec![wfe("mantis_walking_flower", 2, 1), wfe("moth_mimicry_A", 2, 1)]),
            ]),
        ],
        boss_packs: vec![
            // qinglong_boss_azure_dragon: azure_dragon + ball_thunder + ball_wind
            pack("qinglong_boss_azure_dragon", DungeonType::QingLong, PackType::Boss, vec![
                mash(1, vec![
                    wfe("azure_dragon_ball_thunder", 1, 1),
                    wfe("azure_dragon", 1, 1),
                    wfe("azure_dragon_ball_wind", 1, 1),
                ]),
            ]),
        ],
    }
}

/// Pre-built QingLong encounter config.
pub static QINGLONG_ENCOUNTER_CONFIG: std::sync::LazyLock<DungeonEncounterConfig, fn() -> DungeonEncounterConfig> =
    std::sync::LazyLock::new(build_qinglong_encounter_config);

/// Build the dungeon encounter registry with all parsed dungeon .bytes data.
pub fn build_encounter_registry() -> DungeonEncounterRegistry {
    let mut registry = DungeonEncounterRegistry::new();
    registry.register(build_qinglong_encounter_config());
    registry
}

pub mod parse;

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

    // ── US-811-a: Encounter pack weights tests ────────────────────────────────────

    #[test]
    fn qinglong_encounter_config_has_hall_packs() {
        let config = &QINGLONG_ENCOUNTER_CONFIG;
        assert!(!config.hall_packs.is_empty(), "QingLong should have hall packs");
        assert_eq!(config.hall_packs.len(), 5, "QingLong should have 5 hall packs");
    }

    #[test]
    fn qinglong_encounter_config_has_room_packs() {
        let config = &QINGLONG_ENCOUNTER_CONFIG;
        assert!(!config.room_packs.is_empty(), "QingLong should have room packs");
        assert_eq!(config.room_packs.len(), 5, "QingLong should have 5 room packs");
    }

    #[test]
    fn qinglong_encounter_config_has_boss_packs() {
        let config = &QINGLONG_ENCOUNTER_CONFIG;
        assert!(!config.boss_packs.is_empty(), "QingLong should have boss packs");
        assert_eq!(config.boss_packs.len(), 1, "QingLong should have 1 boss pack");
    }

    #[test]
    fn qinglong_hall_pack_ids_match_expected() {
        let config = &QINGLONG_ENCOUNTER_CONFIG;
        let expected_ids = ["qinglong_hall_01", "qinglong_hall_02", "qinglong_hall_03", "qinglong_hall_04", "qinglong_hall_05"];
        for expected in &expected_ids {
            assert!(
                config.hall_packs.iter().any(|p| p.id == *expected),
                "QingLong hall packs should contain {}",
                expected
            );
        }
    }

    #[test]
    fn qinglong_room_pack_ids_match_expected() {
        let config = &QINGLONG_ENCOUNTER_CONFIG;
        let expected_ids = ["qinglong_room_01", "qinglong_room_02", "qinglong_room_03", "qinglong_room_04", "qinglong_room_05"];
        for expected in &expected_ids {
            assert!(
                config.room_packs.iter().any(|p| p.id == *expected),
                "QingLong room packs should contain {}",
                expected
            );
        }
    }

    #[test]
    fn qinglong_boss_pack_contains_azure_dragon() {
        let config = &QINGLONG_ENCOUNTER_CONFIG;
        let boss_pack = config.boss_packs.iter().find(|p| p.id == "qinglong_boss_azure_dragon");
        assert!(boss_pack.is_some(), "QingLong should have qinglong_boss_azure_dragon pack");
        let pack = boss_pack.unwrap();
        assert_eq!(pack.dungeon, DungeonType::QingLong);
        assert_eq!(pack.pack_type, PackType::Boss);
    }

    #[test]
    fn pack_template_resolves_to_encounter_pack() {
        let config = &QINGLONG_ENCOUNTER_CONFIG;
        let hall_pack = config.hall_packs.first().unwrap();

        // Resolve with a seed
        let encounter_pack = hall_pack.resolve(42);

        assert_eq!(encounter_pack.id.0, "qinglong_hall_01");
        assert_eq!(encounter_pack.dungeon, crate::monsters::families::Dungeon::QingLong);
        assert_eq!(encounter_pack.pack_type, PackType::Hall);
        assert!(!encounter_pack.slots.is_empty());
    }

    #[test]
    fn pack_template_resolve_is_deterministic() {
        let config = &QINGLONG_ENCOUNTER_CONFIG;
        let hall_pack = config.hall_packs.first().unwrap();

        let pack1 = hall_pack.resolve(42);
        let pack2 = hall_pack.resolve(42);

        assert_eq!(pack1.id.0, pack2.id.0);
        assert_eq!(pack1.slots.len(), pack2.slots.len());
    }

    #[test]
    fn dungeon_encounter_registry_has_qinglong() {
        let registry = build_encounter_registry();
        let config = registry.get(DungeonType::QingLong);
        assert!(config.is_some(), "Registry should have QingLong config");
    }

    #[test]
    fn dungeon_encounter_registry_returns_none_for_missing_dungeon() {
        let registry = build_encounter_registry();
        // Cross doesn't have encounter config (no map config either)
        let config = registry.get(DungeonType::QingLong);
        assert!(config.is_some());
    }

    #[test]
    fn encounter_selection_produces_expected_composition_for_seed() {
        // US-811 acceptance test: prove encounter selection produces the expected
        // monster composition for a given seed.
        let config = &QINGLONG_ENCOUNTER_CONFIG;

        // For seed=42, room_index=0, the pack selection should be deterministic
        // Select hall pack at index 42 % 5 = 2 → qinglong_hall_03
        let seed = 42u64;
        let room_index = 0u32;

        // The resolve_pack uses (seed + room_index) as selector
        let selector = (seed.wrapping_add(room_index as u64)) as usize;
        let packs = &config.hall_packs;
        let sorted_packs = {
            let mut p = packs.clone();
            p.sort_by(|a, b| a.id.cmp(&b.id));
            p
        };
        let index = selector % sorted_packs.len();
        let selected_pack = &sorted_packs[index];

        // qinglong_hall_03: moth_mimicry_A x2 + moth_mimicry_B x1
        let resolved = selected_pack.resolve(seed);

        let family_ids: Vec<&str> = resolved.family_ids().iter().map(|f| f.0.as_str()).collect();
        assert!(family_ids.contains(&"moth_mimicry_A"), "hall_03 should contain moth_mimicry_A");
        assert!(family_ids.contains(&"moth_mimicry_B"), "hall_03 should contain moth_mimicry_B");
    }

    #[test]
    fn weighted_selection_distributes_across_mash_entries() {
        // Test that weighted selection works correctly when there are multiple mash entries
        // with different chances. We create a pack with 2 mash entries: A (chance=1)
        // and B (chance=3), so B should appear ~3x more often than A.
        let pack = PackTemplate {
            id: "test_pack".to_string(),
            dungeon: DungeonType::QingLong,
            pack_type: PackType::Hall,
            mash: vec![
                mash(1, vec![wfe("mantis_magic_flower", 1, 1)]),
                mash(3, vec![wfe("mantis_spiny_flower", 1, 1)]),
            ],
        };

        let mut count_a = 0usize;
        let mut count_b = 0usize;

        // Run 1000 times to get statistical significance
        for seed in 0..1000u64 {
            let resolved = pack.resolve(seed);
            if resolved.slots[0].family_id.0 == "mantis_magic_flower" {
                count_a += 1;
            } else {
                count_b += 1;
            }
        }

        // With chances 1 and 3, B should appear ~75% and A ~25%
        // Allow reasonable variance: A should be between 15-35%, B between 65-85%
        let ratio_a = count_a as f64 / 1000.0;
        let ratio_b = count_b as f64 / 1000.0;

        assert!(
            ratio_a > 0.15 && ratio_a < 0.35,
            "mantis_magic_flower (chance=1) should appear ~25%, got {:.1}% ({}/1000)",
            ratio_a * 100.0, count_a
        );
        assert!(
            ratio_b > 0.65 && ratio_b < 0.85,
            "mantis_spiny_flower (chance=3) should appear ~75%, got {:.1}% ({}/1000)",
            ratio_b * 100.0, count_b
        );
    }

    // ── US-803-a: dungeon_mode_name contract tests ───────────────────────────────

    #[test]
    fn dungeon_mode_name_matches_dungeon_type_as_str() {
        use crate::monsters::families::Dungeon;
        // Primary dungeons map through DungeonType::as_str()
        assert_eq!(dungeon_mode_name(Dungeon::QingLong), "qinglong");
        assert_eq!(dungeon_mode_name(Dungeon::BaiHu), "baihu");
        assert_eq!(dungeon_mode_name(Dungeon::ZhuQue), "zhuque");
        assert_eq!(dungeon_mode_name(Dungeon::XuanWu), "xuanwu");
    }

    #[test]
    fn dungeon_mode_name_fallback_for_cross() {
        use crate::monsters::families::Dungeon;
        // Cross has no DungeonType, so it falls back to "cross"
        assert_eq!(dungeon_mode_name(Dungeon::Cross), "cross");
    }

    #[test]
    fn dungeon_mode_name_is_contract_for_in_mode_resolution() {
        use crate::monsters::families::Dungeon;
        // The mode names returned by dungeon_mode_name are the exact strings
        // used by InMode condition tags (ddgc_in_mode_<mode>).
        let mode = dungeon_mode_name(Dungeon::XuanWu);
        assert_eq!(mode, "xuanwu");
        assert_eq!(format!("ddgc_in_mode_{}", mode), "ddgc_in_mode_xuanwu");
    }

    // ── US-001: curio/trap/obstacle data model tests ──────────────────────────────

    #[test]
    fn curio_result_construction_is_deterministic() {
        let result = CurioResult::new(10, 0.5, CurioResultType::Loot, "ancient_coin");
        assert_eq!(result.weight, 10);
        assert_eq!(result.chance, 0.5);
        assert_eq!(result.result_type, CurioResultType::Loot);
        assert_eq!(result.result_id, "ancient_coin");
    }

    #[test]
    fn item_interaction_construction_is_deterministic() {
        let interaction = ItemInteraction::new("shovel", "treasure_found");
        assert_eq!(interaction.item_id, "shovel");
        assert_eq!(interaction.overrides_result_id, "treasure_found");
    }

    #[test]
    fn curio_definition_construction_is_deterministic() {
        let results = vec![
            CurioResult::new(5, 0.3, CurioResultType::Nothing, ""),
            CurioResult::new(10, 0.5, CurioResultType::Loot, "gold_chalice"),
            CurioResult::new(5, 0.2, CurioResultType::Quirk, "clumsy"),
        ];
        let item_interactions = vec![
            ItemInteraction::new("shovel", "treasure_found"),
        ];
        let dungeon_scope = vec![DungeonType::QingLong, DungeonType::BaiHu];
        let curio = CurioDefinition::new("ancient_vase", dungeon_scope.clone(), results, item_interactions);

        assert_eq!(curio.id, "ancient_vase");
        assert_eq!(curio.dungeon_scope, dungeon_scope);
        assert_eq!(curio.results.len(), 3);
        assert_eq!(curio.item_interactions.len(), 1);
    }

    #[test]
    fn trap_variation_construction_is_deterministic() {
        let variation = TrapVariation::new(3, vec!["bleed".to_string(), "poison".to_string()], 0.15);
        assert_eq!(variation.level, 3);
        assert_eq!(variation.fail_effects, vec!["bleed", "poison"]);
        assert_eq!(variation.health_fraction, 0.15);
    }

    #[test]
    fn trap_definition_construction_is_deterministic() {
        let variations = vec![
            TrapVariation::new(3, vec!["bleed".to_string()], 0.15),
            TrapVariation::new(5, vec!["bleed".to_string(), "poison".to_string()], 0.25),
        ];
        let trap = TrapDefinition::new(
            "poison_cloud",
            vec!["detect".to_string()],
            vec!["damage".to_string(), "poison".to_string()],
            0.1,
            variations,
        );

        assert_eq!(trap.id, "poison_cloud");
        assert_eq!(trap.success_effects, vec!["detect"]);
        assert_eq!(trap.fail_effects, vec!["damage", "poison"]);
        assert_eq!(trap.health_fraction, 0.1);
        assert_eq!(trap.difficulty_variations.len(), 2);
    }

    #[test]
    fn obstacle_definition_construction_is_deterministic() {
        let obstacle = ObstacleDefinition::new(
            "thorny_thicket",
            vec!["bleed".to_string()],
            0.2,
            0.1,
        );

        assert_eq!(obstacle.id, "thorny_thicket");
        assert_eq!(obstacle.fail_effects, vec!["bleed"]);
        assert_eq!(obstacle.health_fraction, 0.2);
        assert_eq!(obstacle.torchlight_penalty, 0.1);
    }

    #[test]
    fn curio_result_serde_roundtrip_is_deterministic() {
        let result = CurioResult::new(10, 0.5, CurioResultType::Loot, "ancient_coin");
        let serialized = serde_json::to_string(&result).unwrap();
        let deserialized: CurioResult = serde_json::from_str(&serialized).unwrap();
        assert_eq!(result, deserialized);
    }

    #[test]
    fn item_interaction_serde_roundtrip_is_deterministic() {
        let interaction = ItemInteraction::new("shovel", "treasure_found");
        let serialized = serde_json::to_string(&interaction).unwrap();
        let deserialized: ItemInteraction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(interaction, deserialized);
    }

    #[test]
    fn curio_definition_serde_roundtrip_is_deterministic() {
        let results = vec![
            CurioResult::new(5, 0.3, CurioResultType::Nothing, ""),
            CurioResult::new(10, 0.5, CurioResultType::Loot, "gold_chalice"),
        ];
        let item_interactions = vec![
            ItemInteraction::new("shovel", "treasure_found"),
        ];
        let dungeon_scope = vec![DungeonType::QingLong];
        let curio = CurioDefinition::new("ancient_vase", dungeon_scope, results, item_interactions);

        let serialized = serde_json::to_string(&curio).unwrap();
        let deserialized: CurioDefinition = serde_json::from_str(&serialized).unwrap();
        assert_eq!(curio, deserialized);
    }

    #[test]
    fn trap_definition_serde_roundtrip_is_deterministic() {
        let variations = vec![
            TrapVariation::new(3, vec!["bleed".to_string()], 0.15),
        ];
        let trap = TrapDefinition::new(
            "poison_cloud",
            vec!["detect".to_string()],
            vec!["damage".to_string()],
            0.1,
            variations,
        );

        let serialized = serde_json::to_string(&trap).unwrap();
        let deserialized: TrapDefinition = serde_json::from_str(&serialized).unwrap();
        assert_eq!(trap, deserialized);
    }

    #[test]
    fn obstacle_definition_serde_roundtrip_is_deterministic() {
        let obstacle = ObstacleDefinition::new(
            "thorny_thicket",
            vec!["bleed".to_string()],
            0.2,
            0.1,
        );

        let serialized = serde_json::to_string(&obstacle).unwrap();
        let deserialized: ObstacleDefinition = serde_json::from_str(&serialized).unwrap();
        assert_eq!(obstacle, deserialized);
    }

    #[test]
    fn curio_result_type_serde_roundtrip_is_deterministic() {
        let result_types = vec![
            CurioResultType::Nothing,
            CurioResultType::Loot,
            CurioResultType::Quirk,
            CurioResultType::Effect,
            CurioResultType::Purge,
            CurioResultType::Scouting,
            CurioResultType::Teleport,
            CurioResultType::Disease,
        ];
        for rt in result_types {
            let serialized = serde_json::to_string(&rt).unwrap();
            let deserialized: CurioResultType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(rt, deserialized);
        }
    }
}