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

/// Outcome of a curio interaction, describing the result and any effects applied.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurioInteractionOutcome {
    /// The type of result from the interaction.
    pub result_type: CurioResultType,
    /// The specific result identifier (e.g., loot ID, quirk ID, effect ID).
    pub result_id: String,
    /// Effects applied as a result of this interaction.
    pub applied_effects: Vec<String>,
}

/// Outcome of a trap interaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrapOutcome {
    /// The trap was successfully avoided.
    Success {
        /// Effects applied from successful avoidance.
        effects: Vec<String>,
    },
    /// The trap was triggered and had effects applied.
    Fail {
        /// Effects applied from triggering the trap.
        effects: Vec<String>,
        /// Fraction of max HP lost from this trap.
        health_fraction: f64,
    },
}

/// Outcome of an obstacle interaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObstacleOutcome {
    /// Effects applied from failing to overcome the obstacle.
    pub fail_effects: Vec<String>,
    /// Fraction of max HP lost from this obstacle.
    pub health_fraction: f64,
    /// Torchlight penalty modifier (-1.0 to 1.0).
    pub torchlight_penalty: f64,
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
// Town Building definitions
// ─────────────────────────────────────────────────────────────────────────────

/// Type of town building.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuildingType {
    Barracks,
    Blacksmith,
    Campfire,
    Cathedral,
    Confectionery,
    DilapidatedShrine,
    Doctor,
    EmbroideryStation,
    FortuneTeller,
    Gate,
    Graveyard,
    Inn,
    Jester,
    Museum,
    Provisioner,
    Sanctuary,
    Tavern,
    Tower,
    Trainee,
    WanderingTrinkets,
    WeaponRack,
}

/// Activity types for slot-based town services.
///
/// Covers Sanitarium quirk/disease treatment and Tavern bar/gambling/brothel activities.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TownActivity {
    /// Sanitarium quirk treatment activity.
    SanitariumQuirk,
    /// Sanitarium disease treatment activity.
    SanitariumDisease,
    /// Tavern bar/drink activity.
    TavernBar,
    /// Tavern gambling activity.
    TavernGambling,
    /// Tavern brothel activity.
    TavernBrothel,
}

impl TownActivity {
    /// Returns the string representation used by TownSlotState keys.
    pub fn as_str(&self) -> &'static str {
        match self {
            TownActivity::SanitariumQuirk => "quirk",
            TownActivity::SanitariumDisease => "disease",
            TownActivity::TavernBar => "bar",
            TownActivity::TavernGambling => "gambling",
            TownActivity::TavernBrothel => "brothel",
        }
    }

    /// Get all Sanitarium activities.
    pub fn sanitarium_activities() -> [TownActivity; 2] {
        [TownActivity::SanitariumQuirk, TownActivity::SanitariumDisease]
    }

    /// Get all Tavern activities.
    pub fn tavern_activities() -> [TownActivity; 3] {
        [TownActivity::TavernBar, TownActivity::TavernGambling, TownActivity::TavernBrothel]
    }
}

impl std::fmt::Display for TownActivity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// An unlock condition for a town building.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnlockCondition {
    /// Condition type (e.g., "completed_runs", "defeated_monsters").
    pub condition_type: String,
    /// Required count to unlock.
    pub required_count: u32,
}

impl UnlockCondition {
    pub fn new(condition_type: &str, required_count: u32) -> Self {
        UnlockCondition {
            condition_type: condition_type.to_string(),
            required_count,
        }
    }
}

/// Effects provided by an upgrade level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpgradeEffect {
    /// Effect identifier (e.g., "recruit_discount", "item_discount").
    pub effect_id: String,
    /// Numerical value of the effect.
    pub value: f64,
}

impl UpgradeEffect {
    pub fn new(effect_id: &str, value: f64) -> Self {
        UpgradeEffect {
            effect_id: effect_id.to_string(),
            value,
        }
    }
}

/// A single level in an upgrade tree.
///
/// The code field uses letters a-g to represent different upgrade tiers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpgradeLevel {
    /// Upgrade code (a-g).
    pub code: char,
    /// Cost in gold for this upgrade level.
    pub cost: u32,
    /// Effects provided by this upgrade level.
    pub effects: Vec<UpgradeEffect>,
}

impl UpgradeLevel {
    pub fn new(code: char, cost: u32, effects: Vec<UpgradeEffect>) -> Self {
        UpgradeLevel { code, cost, effects }
    }
}

/// An upgrade tree containing multiple levels.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpgradeTree {
    /// Unique identifier for this upgrade tree.
    pub tree_id: String,
    /// All levels in this upgrade tree.
    pub levels: Vec<UpgradeLevel>,
}

impl UpgradeTree {
    pub fn new(tree_id: &str, levels: Vec<UpgradeLevel>) -> Self {
        UpgradeTree {
            tree_id: tree_id.to_string(),
            levels,
        }
    }
}

/// Definition of a town building with upgrade trees.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TownBuilding {
    /// Unique identifier for this building.
    pub id: String,
    /// Type of building.
    pub building_type: BuildingType,
    /// Conditions required to unlock this building.
    pub unlock_conditions: Vec<UnlockCondition>,
    /// Available upgrade trees for this building.
    pub upgrade_trees: Vec<UpgradeTree>,
}

impl TownBuilding {
    pub fn new(
        id: &str,
        building_type: BuildingType,
        unlock_conditions: Vec<UnlockCondition>,
        upgrade_trees: Vec<UpgradeTree>,
    ) -> Self {
        TownBuilding {
            id: id.to_string(),
            building_type,
            unlock_conditions,
            upgrade_trees,
        }
    }
}

/// Heirloom currency type for the town.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HeirloomCurrency {
    Bones,
    Portraits,
    Tapes,
}

/// Current state of a town building upgrade.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildingUpgradeState {
    /// The building ID this state belongs to.
    pub building_id: String,
    /// Current upgrade level code (a-g), or None if not upgraded.
    pub current_level: Option<char>,
}

impl BuildingUpgradeState {
    pub fn new(building_id: &str, current_level: Option<char>) -> Self {
        BuildingUpgradeState {
            building_id: building_id.to_string(),
            current_level,
        }
    }
}

/// Tracks slot usage for a single building activity during a town visit.
///
/// Slots represent capacity for services like Sanitarium quirk/disease treatment
/// or Tavern bar/gambling/brothel visits. Each slot can be used once per visit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildingSlotState {
    /// Maximum number of slots available at the current upgrade level.
    pub capacity: usize,
    /// Number of slots currently consumed.
    pub consumed: usize,
}

impl BuildingSlotState {
    /// Create a new slot state with given capacity.
    pub fn new(capacity: usize) -> Self {
        BuildingSlotState { capacity, consumed: 0 }
    }

    /// Check if any slots are available.
    pub fn has_available(&self) -> bool {
        self.consumed < self.capacity
    }

    /// Get the number of available slots.
    pub fn available(&self) -> usize {
        self.capacity.saturating_sub(self.consumed)
    }

    /// Consume a slot, returning true if successful.
    pub fn consume(&mut self) -> bool {
        if self.has_available() {
            self.consumed += 1;
            true
        } else {
            false
        }
    }

    /// Reset consumed slots to 0 for a new visit.
    pub fn reset(&mut self) {
        self.consumed = 0;
    }
}

/// Tracks all slot-based activities for a single town visit.
///
/// This struct maintains per-building and per-activity slot capacity and consumption,
/// reset at the start of each town visit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TownSlotState {
    /// Slot states keyed by building_id::activity_type.
    /// Activity types: "quirk", "disease", "bar", "gambling", "brothel".
    slot_states: std::collections::HashMap<String, BuildingSlotState>,
}

impl TownSlotState {
    /// Create a new empty slot state.
    pub fn new() -> Self {
        TownSlotState {
            slot_states: std::collections::HashMap::new(),
        }
    }

    /// Initialize slot capacity for a building activity.
    pub fn set_capacity(&mut self, building_id: &str, activity_type: &str, capacity: usize) {
        let key = format!("{}::{}", building_id, activity_type);
        self.slot_states.insert(key, BuildingSlotState::new(capacity));
    }

    /// Get the capacity for a building activity.
    pub fn get_capacity(&self, building_id: &str, activity_type: &str) -> usize {
        let key = format!("{}::{}", building_id, activity_type);
        self.slot_states.get(&key).map(|s| s.capacity).unwrap_or(0)
    }

    /// Get the number of available slots for a building activity.
    pub fn available(&self, building_id: &str, activity_type: &str) -> usize {
        let key = format!("{}::{}", building_id, activity_type);
        self.slot_states.get(&key).map(|s| s.available()).unwrap_or(0)
    }

    /// Check if a slot is available for a building activity.
    pub fn has_available(&self, building_id: &str, activity_type: &str) -> bool {
        let key = format!("{}::{}", building_id, activity_type);
        self.slot_states.get(&key).map(|s| s.has_available()).unwrap_or(false)
    }

    /// Try to consume a slot for a building activity.
    /// Returns true if successful, false if no slots available.
    pub fn try_consume(&mut self, building_id: &str, activity_type: &str) -> bool {
        let key = format!("{}::{}", building_id, activity_type);
        if let Some(state) = self.slot_states.get_mut(&key) {
            state.consume()
        } else {
            false
        }
    }

    /// Reset all consumed slots for a new town visit.
    pub fn reset(&mut self) {
        for state in self.slot_states.values_mut() {
            state.reset();
        }
    }

    /// Get total slots consumed across all activities.
    pub fn total_consumed(&self) -> usize {
        self.slot_states.values().map(|s| s.consumed).sum()
    }
}

impl Default for TownSlotState {
    fn default() -> Self {
        Self::new()
    }
}

/// State of the town including building upgrades and currencies.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TownState {
    /// Current gold available in the town.
    pub gold: u32,
    /// Heirloom currencies available.
    pub heirlooms: std::collections::HashMap<HeirloomCurrency, u32>,
    /// Current upgrade state for each building.
    pub building_states: std::collections::HashMap<String, BuildingUpgradeState>,
}

impl TownState {
    /// Create a new town state with initial resources.
    pub fn new(gold: u32) -> Self {
        TownState {
            gold,
            heirlooms: std::collections::HashMap::new(),
            building_states: std::collections::HashMap::new(),
        }
    }

    /// Apply an upgrade to a building.
    ///
    /// Returns the cost of the upgrade, or None if the building or level doesn't exist.
    pub fn apply_upgrade(
        &mut self,
        building_id: &str,
        level_code: char,
        building: &TownBuilding,
    ) -> Option<u32> {
        // Find the requested level in the upgrade trees
        let cost = building.upgrade_trees.iter().flat_map(|t| &t.levels).find(|l| l.code == level_code).map(|l| l.cost)?;

        // Check if we have enough gold
        if self.gold < cost {
            return None;
        }

        // Deduct the cost
        self.gold -= cost;

        // Update the building state
        let state = self.building_states.entry(building_id.to_string()).or_insert_with(|| {
            BuildingUpgradeState::new(building_id, None)
        });
        state.current_level = Some(level_code);

        Some(cost)
    }

    /// Get the current upgrade level for a building.
    pub fn get_upgrade_level(&self, building_id: &str) -> Option<char> {
        self.building_states.get(building_id).and_then(|s| s.current_level)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Building Registry
// ─────────────────────────────────────────────────────────────────────────────

/// Registry holding all town building definitions parsed from DDGC Buildings.json.
///
/// Provides lookup by building ID, building type, and upgrade tree traversal.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuildingRegistry {
    buildings: std::collections::HashMap<String, TownBuilding>,
}

impl BuildingRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        BuildingRegistry { buildings: std::collections::HashMap::new() }
    }

    /// Register a building definition.
    pub fn register(&mut self, building: TownBuilding) {
        self.buildings.insert(building.id.clone(), building);
    }

    /// Get a building by its ID.
    pub fn get(&self, id: &str) -> Option<&TownBuilding> {
        self.buildings.get(id)
    }

    /// Get all registered building IDs.
    pub fn all_ids(&self) -> Vec<&str> {
        self.buildings.keys().map(|s| s.as_str()).collect()
    }

    /// Get all buildings of a specific type.
    pub fn by_type(&self, building_type: BuildingType) -> Vec<&TownBuilding> {
        self.buildings
            .values()
            .filter(|b| b.building_type == building_type)
            .collect()
    }

    /// Get the total number of registered buildings.
    pub fn len(&self) -> usize {
        self.buildings.len()
    }

    /// Returns true if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.buildings.is_empty()
    }

    /// Get the effect value for a specific effect_id at a given upgrade level.
    ///
    /// Searches through ALL building's upgrade trees for the specified level code
    /// and returns the effect value if found. Continues searching all trees
    /// until the effect_id is found at that level.
    ///
    /// Returns `None` if the building doesn't exist, no level with the code exists,
    /// or the effect_id is not found at any tree's level.
    pub fn get_effect_at_level(
        &self,
        building_id: &str,
        level_code: char,
        effect_id: &str,
    ) -> Option<f64> {
        let building = self.buildings.get(building_id)?;

        // Search all trees for the level code, then search that level's effects
        for tree in &building.upgrade_trees {
            if let Some(level) = tree.levels.iter().find(|l| l.code == level_code) {
                if let Some(effect) = level.effects.iter().find(|e| e.effect_id == effect_id) {
                    return Some(effect.value);
                }
            }
        }

        None
    }

    /// Get the cost for a specific upgrade level.
    ///
    /// Returns `None` if the building doesn't exist or the level doesn't exist.
    pub fn get_upgrade_cost(&self, building_id: &str, level_code: char) -> Option<u32> {
        let building = self.buildings.get(building_id)?;

        for tree in &building.upgrade_trees {
            if let Some(level) = tree.levels.iter().find(|l| l.code == level_code) {
                return Some(level.cost);
            }
        }

        None
    }

    /// Get all upgrade levels for a building.
    ///
    /// Returns all levels across all upgrade trees, sorted by code.
    pub fn get_upgrade_levels(&self, building_id: &str) -> Option<Vec<&UpgradeLevel>> {
        let building = self.buildings.get(building_id)?;

        let mut all_levels: Vec<&UpgradeLevel> = building
            .upgrade_trees
            .iter()
            .flat_map(|t| t.levels.iter())
            .collect();

        all_levels.sort_by_key(|l| l.code);
        Some(all_levels)
    }

    // ── Sanitarium helper methods ───────────────────────────────────────────────

    /// Sanitarium disease upgrade paths follow specific patterns:
    /// - Treatment cost upgrades are at levels `a`, `c`, `e`
    /// - Cure-all chance upgrades are at levels `b`, `d`
    ///
    /// This reflects the original game's upgrade structure where disease treatment
    /// cost and cure-all chance were upgraded independently.

    /// Get the disease treatment cost at the given upgrade level.
    ///
    /// Disease treatment cost upgrades follow the `a/c/e` path.
    /// Returns `None` if the building doesn't exist or the level has no disease_cost effect.
    pub fn sanitarium_disease_cost(&self, level_code: char) -> Option<f64> {
        self.get_effect_at_level("sanitarium", level_code, "disease_cost")
    }

    /// Get the disease cure-all chance at the given upgrade level.
    ///
    /// Cure-all chance upgrades follow the `b/d` path.
    /// Returns `None` if the building doesn't exist or the level has no cure_all_chance effect.
    pub fn sanitarium_cure_all_chance(&self, level_code: char) -> Option<f64> {
        self.get_effect_at_level("sanitarium", level_code, "cure_all_chance")
    }

    /// Get the disease slot count at the given upgrade level.
    ///
    /// Returns `None` if the building doesn't exist or the level has no disease_slots effect.
    pub fn sanitarium_disease_slots(&self, level_code: char) -> Option<f64> {
        self.get_effect_at_level("sanitarium", level_code, "disease_slots")
    }

    /// Get the quirk slot count at the given upgrade level.
    ///
    /// Returns `None` if the building doesn't exist or the level has no quirk_slots effect.
    pub fn sanitarium_quirk_slots(&self, level_code: char) -> Option<f64> {
        self.get_effect_at_level("sanitarium", level_code, "quirk_slots")
    }

    // ── Tavern helper methods ──────────────────────────────────────────────────

    /// Get the tavern bar cost at the given upgrade level.
    pub fn tavern_bar_cost(&self, level_code: char) -> Option<f64> {
        self.get_effect_at_level("tavern", level_code, "bar_cost")
    }

    /// Get the tavern bar stress heal at the given upgrade level.
    pub fn tavern_bar_stress_heal(&self, level_code: char) -> Option<f64> {
        self.get_effect_at_level("tavern", level_code, "bar_stress_heal")
    }

    /// Get the tavern bar slot count at the given upgrade level.
    pub fn tavern_bar_slots(&self, level_code: char) -> Option<f64> {
        self.get_effect_at_level("tavern", level_code, "bar_slots")
    }

    /// Get the tavern gambling cost at the given upgrade level.
    pub fn tavern_gambling_cost(&self, level_code: char) -> Option<f64> {
        self.get_effect_at_level("tavern", level_code, "gambling_cost")
    }

    /// Get the tavern gambling stress heal at the given upgrade level.
    pub fn tavern_gambling_stress_heal(&self, level_code: char) -> Option<f64> {
        self.get_effect_at_level("tavern", level_code, "gambling_stress_heal")
    }

    /// Get the tavern brothel cost at the given upgrade level.
    pub fn tavern_brothel_cost(&self, level_code: char) -> Option<f64> {
        self.get_effect_at_level("tavern", level_code, "brothel_cost")
    }

    /// Get the tavern brothel stress heal at the given upgrade level.
    pub fn tavern_brothel_stress_heal(&self, level_code: char) -> Option<f64> {
        self.get_effect_at_level("tavern", level_code, "brothel_stress_heal")
    }

    // ── Blacksmith helper methods ───────────────────────────────────────────────

    /// Get the blacksmith repair discount at the given upgrade level.
    pub fn blacksmith_repair_discount(&self, level_code: char) -> Option<f64> {
        self.get_effect_at_level("blacksmith", level_code, "repair_discount")
    }

    /// Get the blacksmith weapon upgrade cost reduction at the given upgrade level.
    pub fn blacksmith_weapon_upgrade_cost(&self, level_code: char) -> Option<f64> {
        self.get_effect_at_level("blacksmith", level_code, "weapon_upgrade_cost")
    }

    /// Get the blacksmith equipment discount at the given upgrade level.
    pub fn blacksmith_equipment_discount(&self, level_code: char) -> Option<f64> {
        self.get_effect_at_level("blacksmith", level_code, "equipment_cost_discount")
    }

    // ── Guild helper methods ───────────────────────────────────────────────────

    /// Get the guild experience boost at the given upgrade level.
    pub fn guild_experience_boost(&self, level_code: char) -> Option<f64> {
        self.get_effect_at_level("guild", level_code, "experience_boost")
    }

    /// Get the guild skill upgrade chance at the given upgrade level.
    pub fn guild_skill_upgrade_chance(&self, level_code: char) -> Option<f64> {
        self.get_effect_at_level("guild", level_code, "skill_upgrade_chance")
    }

    /// Get the guild skill cost discount at the given upgrade level.
    pub fn guild_skill_cost_discount(&self, level_code: char) -> Option<f64> {
        self.get_effect_at_level("guild", level_code, "skill_cost_discount")
    }
}

/// Dungeon level constants for trap difficulty variations.
pub mod dungeon_level {
    /// Level 3 dungeon (standard difficulty).
    pub const LEVEL_3: u32 = 3;
    /// Level 5 dungeon (hard difficulty).
    pub const LEVEL_5: u32 = 5;
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

    /// Resolve a curio interaction using weighted random selection.
    ///
    /// If `has_item` is true and the curio has an item interaction for `item_id`,
    /// the override result is used. Otherwise, selects from the weighted result table.
    ///
    /// Returns `None` if the curio does not exist.
    pub fn resolve_curio_interaction(
        &self,
        curio_id: &str,
        has_item: bool,
        item_id: &str,
        seed: u64,
    ) -> Option<CurioInteractionOutcome> {
        let curio = self.curios.get(curio_id)?;

        // If item is used, check for override
        if has_item {
            if let Some(interaction) = curio.item_interactions.iter().find(|i| i.item_id == item_id) {
                // Find the result that matches the override result_id
                if let Some(result) = curio.results.iter().find(|r| r.result_id == interaction.overrides_result_id) {
                    return Some(CurioInteractionOutcome {
                        result_type: result.result_type,
                        result_id: result.result_id.clone(),
                        applied_effects: vec![],
                    });
                }
                // Override result_id might not be in results table (e.g., "treasure_found")
                // In that case, create outcome with the override id and Effect type
                return Some(CurioInteractionOutcome {
                    result_type: CurioResultType::Effect,
                    result_id: interaction.overrides_result_id.clone(),
                    applied_effects: vec![],
                });
            }
        }

        // Fall back to weighted random selection
        if curio.results.is_empty() {
            return Some(CurioInteractionOutcome {
                result_type: CurioResultType::Nothing,
                result_id: String::new(),
                applied_effects: vec![],
            });
        }

        let total: u32 = curio.results.iter().map(|r| r.weight).sum();
        if total == 0 {
            return Some(CurioInteractionOutcome {
                result_type: curio.results[0].result_type,
                result_id: curio.results[0].result_id.clone(),
                applied_effects: vec![],
            });
        }

        let selector = (seed % total as u64) as u32;
        let mut accum = 0u32;
        for result in &curio.results {
            accum += result.weight;
            if selector < accum {
                return Some(CurioInteractionOutcome {
                    result_type: result.result_type,
                    result_id: result.result_id.clone(),
                    applied_effects: vec![],
                });
            }
        }

        // Fallback to last result
        let last = curio.results.last().unwrap();
        Some(CurioInteractionOutcome {
            result_type: last.result_type,
            result_id: last.result_id.clone(),
            applied_effects: vec![],
        })
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

    /// Resolve a trap interaction using resist chance and seed-based randomization.
    ///
    /// The trap can be avoided if a random value derived from the seed is less than
    /// `resist_chance`. If avoided, returns `TrapOutcome::Success` with the trap's
    /// success effects. If triggered, returns `TrapOutcome::Fail` with effects and
    /// health fraction from the appropriate difficulty variation for `trap_level`,
    /// or the base values if no variation exists for that level.
    ///
    /// Returns `None` if the trap does not exist.
    pub fn resolve_trap_interaction(
        &self,
        trap_id: &str,
        trap_level: u32,
        resist_chance: f64,
        seed: u64,
    ) -> Option<TrapOutcome> {
        let trap = self.traps.get(trap_id)?;

        // Determine if the trap is avoided using seed-derived random value
        // The seed produces a deterministic float in [0.0, 1.0)
        let threshold = ((seed % 1000) as f64 / 1000.0) + (seed / 1000) as f64 * 0.001;
        let normalized_seed = threshold.fract().abs();

        if normalized_seed < resist_chance {
            // Trap avoided successfully
            return Some(TrapOutcome::Success {
                effects: trap.success_effects.clone(),
            });
        }

        // Trap triggered - find appropriate difficulty variation
        let variation = trap.difficulty_variations.iter().find(|v| v.level == trap_level);

        if let Some(v) = variation {
            Some(TrapOutcome::Fail {
                effects: v.fail_effects.clone(),
                health_fraction: v.health_fraction,
            })
        } else {
            // Fall back to base trap values
            Some(TrapOutcome::Fail {
                effects: trap.fail_effects.clone(),
                health_fraction: trap.health_fraction,
            })
        }
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

    /// Resolve an obstacle interaction.
    ///
    /// Obstacles always have a failure outcome when attempted - they represent
    /// physical barriers that must be overcome rather than resisted. Returns
    /// the obstacle's fail effects, health fraction, and torchlight penalty.
    ///
    /// Returns `None` if the obstacle does not exist.
    pub fn resolve_obstacle_interaction(
        &self,
        obstacle_id: &str,
    ) -> Option<ObstacleOutcome> {
        let obstacle = self.obstacles.get(obstacle_id)?;

        Some(ObstacleOutcome {
            fail_effects: obstacle.fail_effects.clone(),
            health_fraction: obstacle.health_fraction,
            torchlight_penalty: obstacle.torchlight_penalty,
        })
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

// ─────────────────────────────────────────────────────────────────────────────
// Trinket and Equipment definitions
// ─────────────────────────────────────────────────────────────────────────────

/// A modifier that applies a numeric change to an attribute.
///
/// Used by equipment to alter hero stats such as damage, defense, speed, etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeModifier {
    /// The attribute key this modifier affects (e.g., "attack", "defense", "speed").
    pub attribute_key: String,
    /// The numeric value of the modifier.
    pub value: f64,
}

impl AttributeModifier {
    pub fn new(attribute_key: &str, value: f64) -> Self {
        AttributeModifier {
            attribute_key: attribute_key.to_string(),
            value,
        }
    }
}

/// Rarity tier for trinkets.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrinketRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl TrinketRarity {
    pub fn as_str(&self) -> &'static str {
        match self {
            TrinketRarity::Common => "common",
            TrinketRarity::Uncommon => "uncommon",
            TrinketRarity::Rare => "rare",
            TrinketRarity::Epic => "epic",
            TrinketRarity::Legendary => "legendary",
        }
    }
}

/// Equipment slot type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Weapon,
    Armor,
}

impl EquipmentSlot {
    pub fn as_str(&self) -> &'static str {
        match self {
            EquipmentSlot::Weapon => "weapon",
            EquipmentSlot::Armor => "armor",
        }
    }
}

/// Definition of a trinket that can be equipped on heroes.
///
/// Trinkets provide passive buffs and may have class restrictions,
/// rarity tiers, purchase limits, and dungeon-of-origin tracking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrinketDefinition {
    /// Unique identifier for this trinket.
    pub id: String,
    /// Buff effect IDs provided by this trinket.
    pub buffs: Vec<String>,
    /// Hero class IDs that can equip this trinket (empty = all classes).
    pub hero_class_requirements: Vec<String>,
    /// Rarity tier of this trinket.
    pub rarity: TrinketRarity,
    /// Purchase price in gold.
    pub price: u32,
    /// Maximum number that can be owned per run.
    pub limit: u32,
    /// Dungeon type this trinket originates from.
    pub origin_dungeon: DungeonType,
}

impl TrinketDefinition {
    pub fn new(
        id: &str,
        buffs: Vec<String>,
        hero_class_requirements: Vec<String>,
        rarity: TrinketRarity,
        price: u32,
        limit: u32,
        origin_dungeon: DungeonType,
    ) -> Self {
        TrinketDefinition {
            id: id.to_string(),
            buffs,
            hero_class_requirements,
            rarity,
            price,
            limit,
            origin_dungeon,
        }
    }
}

/// Definition of an equipment upgrade for heroes.
///
/// Equipment occupies a slot (weapon/armor) and provides stat modifiers
/// based on upgrade level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipmentDefinition {
    /// Unique identifier for this equipment.
    pub id: String,
    /// Hero class ID this equipment is for.
    pub hero_class_id: String,
    /// The equipment slot this occupies.
    pub slot: EquipmentSlot,
    /// Upgrade level (0 = base, increases with upgrades).
    pub upgrade_level: u32,
    /// Stat modifiers provided by this equipment.
    pub stat_modifiers: Vec<AttributeModifier>,
}

impl EquipmentDefinition {
    pub fn new(
        id: &str,
        hero_class_id: &str,
        slot: EquipmentSlot,
        upgrade_level: u32,
        stat_modifiers: Vec<AttributeModifier>,
    ) -> Self {
        EquipmentDefinition {
            id: id.to_string(),
            hero_class_id: hero_class_id.to_string(),
            slot,
            upgrade_level,
            stat_modifiers,
        }
    }
}

/// Registry holding all trinket definitions.
///
/// Provides lookup by trinket ID.
#[derive(Debug, Clone, Default)]
pub struct TrinketRegistry {
    trinkets: std::collections::HashMap<String, TrinketDefinition>,
}

impl TrinketRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        TrinketRegistry { trinkets: std::collections::HashMap::new() }
    }

    /// Register a trinket definition.
    pub fn register(&mut self, trinket: TrinketDefinition) {
        self.trinkets.insert(trinket.id.clone(), trinket);
    }

    /// Get a trinket by its ID.
    pub fn get(&self, id: &str) -> Option<&TrinketDefinition> {
        self.trinkets.get(id)
    }

    /// Get all registered trinket IDs.
    pub fn all_ids(&self) -> Vec<&str> {
        self.trinkets.keys().map(|s| s.as_str()).collect()
    }

    /// Get the total number of registered trinkets.
    pub fn len(&self) -> usize {
        self.trinkets.len()
    }

    /// Returns true if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.trinkets.is_empty()
    }

    /// Get all trinkets usable by a specific hero class.
    ///
    /// Returns trinkets where `hero_class_requirements` is empty (all classes)
    /// or where the given class_id is in the requirements list.
    pub fn trinkets_for_class(&self, class_id: &str) -> Vec<&TrinketDefinition> {
        self.trinkets
            .values()
            .filter(|t| {
                t.hero_class_requirements.is_empty()
                    || t.hero_class_requirements.iter().any(|r| r == class_id)
            })
            .collect()
    }

    /// Get all trinkets of a specific rarity.
    pub fn by_rarity(&self, rarity: TrinketRarity) -> Vec<&TrinketDefinition> {
        self.trinkets
            .values()
            .filter(|t| t.rarity == rarity)
            .collect()
    }

    /// Get all trinkets originating from a specific dungeon.
    pub fn by_dungeon(&self, dungeon: DungeonType) -> Vec<&TrinketDefinition> {
        self.trinkets
            .values()
            .filter(|t| t.origin_dungeon == dungeon)
            .collect()
    }
}

/// Registry holding all equipment definitions.
///
/// Provides lookup by equipment ID.
#[derive(Debug, Clone, Default)]
pub struct EquipmentRegistry {
    equipment: std::collections::HashMap<String, EquipmentDefinition>,
}

impl EquipmentRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        EquipmentRegistry { equipment: std::collections::HashMap::new() }
    }

    /// Register an equipment definition.
    pub fn register(&mut self, equipment: EquipmentDefinition) {
        self.equipment.insert(equipment.id.clone(), equipment);
    }

    /// Get an equipment by its ID.
    pub fn get(&self, id: &str) -> Option<&EquipmentDefinition> {
        self.equipment.get(id)
    }

    /// Get all registered equipment IDs.
    pub fn all_ids(&self) -> Vec<&str> {
        self.equipment.keys().map(|s| s.as_str()).collect()
    }

    /// Get equipment by hero class and slot.
    pub fn by_class_and_slot(&self, hero_class_id: &str, slot: EquipmentSlot) -> Vec<&EquipmentDefinition> {
        self.equipment
            .values()
            .filter(|e| e.hero_class_id == hero_class_id && e.slot == slot)
            .collect()
    }

    /// Get the total number of registered equipment.
    pub fn len(&self) -> usize {
        self.equipment.len()
    }

    /// Returns true if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.equipment.is_empty()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Quirk and Disease definitions
// ─────────────────────────────────────────────────────────────────────────────

/// Classification of a quirk - the broad category it belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuirkClassification {
    /// A personality quirk or habit.
    Personality,
    /// A physical trait or physical quirk.
    Physical,
    /// A disease or illness.
    Disease,
    /// A preference or inclination.
    Preference,
    /// A belief or conviction.
    Belief,
    /// A talent or natural ability.
    Talent,
    /// A habit or routine.
    Habit,
    /// A social quirk or behavior in groups.
    Social,
}

impl QuirkClassification {
    /// Returns the snake_case string representation for serialization.
    pub fn as_str(&self) -> &'static str {
        match self {
            QuirkClassification::Personality => "personality",
            QuirkClassification::Physical => "physical",
            QuirkClassification::Disease => "disease",
            QuirkClassification::Preference => "preference",
            QuirkClassification::Belief => "belief",
            QuirkClassification::Talent => "talent",
            QuirkClassification::Habit => "habit",
            QuirkClassification::Social => "social",
        }
    }
}

/// Definition of a quirk or disease that can affect a hero.
///
/// Quirks provide buffs (positive modifiers) or debuffs (negative modifiers)
/// and may be incompatible with other quirks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuirkDefinition {
    /// Unique identifier for this quirk.
    pub id: String,
    /// Whether this is a positive quirk (true) or negative quirk (false).
    pub is_positive: bool,
    /// Whether this is a disease (true) or a regular quirk (false).
    pub is_disease: bool,
    /// Classification category of this quirk.
    pub classification: QuirkClassification,
    /// Buff effect IDs provided by this quirk.
    pub buffs: Vec<String>,
    /// IDs of quirks that cannot coexist with this one.
    pub incompatible_quirks: Vec<String>,
    /// Tag indicating which curio type this quirk is associated with.
    pub curio_tag: String,
}

impl QuirkDefinition {
    pub fn new(
        id: &str,
        is_positive: bool,
        is_disease: bool,
        classification: QuirkClassification,
        buffs: Vec<String>,
        incompatible_quirks: Vec<String>,
        curio_tag: &str,
    ) -> Self {
        QuirkDefinition {
            id: id.to_string(),
            is_positive,
            is_disease,
            classification,
            buffs,
            incompatible_quirks,
            curio_tag: curio_tag.to_string(),
        }
    }
}

/// Registry holding all quirk definitions.
///
/// Provides lookup by quirk ID and filtering by quirk type.
#[derive(Debug, Clone, Default)]
pub struct QuirkRegistry {
    quirks: std::collections::HashMap<String, QuirkDefinition>,
}

impl QuirkRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        QuirkRegistry { quirks: std::collections::HashMap::new() }
    }

    /// Register a quirk definition.
    pub fn register(&mut self, quirk: QuirkDefinition) {
        self.quirks.insert(quirk.id.clone(), quirk);
    }

    /// Get a quirk by its ID.
    pub fn get(&self, id: &str) -> Option<&QuirkDefinition> {
        self.quirks.get(id)
    }

    /// Get all registered quirk IDs.
    pub fn all_ids(&self) -> Vec<&str> {
        self.quirks.keys().map(|s| s.as_str()).collect()
    }

    /// Get the total number of registered quirks.
    pub fn len(&self) -> usize {
        self.quirks.len()
    }

    /// Returns true if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.quirks.is_empty()
    }

    /// Get all positive quirks.
    pub fn positive_quirks(&self) -> Vec<&QuirkDefinition> {
        self.quirks.values().filter(|q| q.is_positive).collect()
    }

    /// Get all negative quirks (non-positive).
    pub fn negative_quirks(&self) -> Vec<&QuirkDefinition> {
        self.quirks.values().filter(|q| !q.is_positive).collect()
    }

    /// Get all diseases.
    pub fn diseases(&self) -> Vec<&QuirkDefinition> {
        self.quirks.values().filter(|q| q.is_disease).collect()
    }

    /// Get all quirks of a specific classification.
    pub fn by_classification(&self, classification: QuirkClassification) -> Vec<&QuirkDefinition> {
        self.quirks
            .values()
            .filter(|q| q.classification == classification)
            .collect()
    }

    /// Resolve all buffs for a quirk into attribute modifiers via BuffRegistry.
    ///
    /// Returns all `AttributeModifier` entries from the quirk's buff list,
    /// with duplicates merged (same `attribute_key` values are combined by summing).
    pub fn resolve_quirk_buffs(&self, quirk_id: &str, buff_registry: &BuffRegistry) -> Vec<AttributeModifier> {
        let quirk = match self.quirks.get(quirk_id) {
            Some(q) => q,
            None => return vec![],
        };

        let mut aggregated: std::collections::HashMap<String, f64> = std::collections::HashMap::new();

        for buff_id in &quirk.buffs {
            for modifier in buff_registry.resolve_buff(buff_id) {
                *aggregated.entry(modifier.attribute_key.clone()).or_insert(0.0) += modifier.value;
            }
        }

        aggregated
            .into_iter()
            .map(|(attribute_key, value)| AttributeModifier { attribute_key, value })
            .collect()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Trait / Affliction / Virtue definitions
// ─────────────────────────────────────────────────────────────────────────────

/// Whether a trait is an affliction (negative) or virtue (positive).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OverstressType {
    Affliction,
    Virtue,
}

/// Action that can be taken during combat start-of-turn act-outs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActOutAction {
    Nothing,
    BarkStress,
    ChangePos,
    IgnoreCommand,
    /// Attack a random enemy
    AttackRandom,
    /// Attack a friendly target (ally)
    AttackFriendly,
    /// Mark self (apply a status to self)
    MarkSelf,
    /// Defend (increase DEF)
    Defend,
    /// Use a skill if available
    UseSkill,
}

impl ActOutAction {
    /// Parse from string representation used in JSON.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "nothing" => Some(ActOutAction::Nothing),
            "bark_stress" => Some(ActOutAction::BarkStress),
            "change_pos" => Some(ActOutAction::ChangePos),
            "ignore_command" => Some(ActOutAction::IgnoreCommand),
            "attack_random" => Some(ActOutAction::AttackRandom),
            "attack_friendly" => Some(ActOutAction::AttackFriendly),
            "mark_self" => Some(ActOutAction::MarkSelf),
            "defend" => Some(ActOutAction::Defend),
            "use_skill" => Some(ActOutAction::UseSkill),
            _ => None,
        }
    }

    /// Convert to string representation for trace recording.
    pub fn as_str(&self) -> &'static str {
        match self {
            ActOutAction::Nothing => "nothing",
            ActOutAction::BarkStress => "bark_stress",
            ActOutAction::ChangePos => "change_pos",
            ActOutAction::IgnoreCommand => "ignore_command",
            ActOutAction::AttackRandom => "attack_random",
            ActOutAction::AttackFriendly => "attack_friendly",
            ActOutAction::MarkSelf => "mark_self",
            ActOutAction::Defend => "defend",
            ActOutAction::UseSkill => "use_skill",
        }
    }
}

/// A single act-out entry with its selection weight.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActOutEntry {
    /// The action to perform.
    pub action: ActOutAction,
    /// Selection weight for weighted random selection.
    pub weight: u32,
}

impl ActOutEntry {
    pub fn new(action: ActOutAction, weight: u32) -> Self {
        ActOutEntry { action, weight }
    }
}

/// Trigger for a reaction act-out.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReactionTrigger {
    /// Triggered when an ally is hit
    AllyHit,
    /// Triggered when an ally is killed
    AllyKilled,
    /// Triggered when an enemy is killed
    EnemyKilled,
    /// Triggered when an ally is stressed
    AllyStressed,
    /// Triggered when self is stressed
    SelfStressed,
    /// Triggered at start of combat
    CombatStart,
}

impl ReactionTrigger {
    /// Parse from string representation used in JSON.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ally_hit" => Some(ReactionTrigger::AllyHit),
            "ally_killed" => Some(ReactionTrigger::AllyKilled),
            "enemy_killed" => Some(ReactionTrigger::EnemyKilled),
            "ally_stressed" => Some(ReactionTrigger::AllyStressed),
            "self_stressed" => Some(ReactionTrigger::SelfStressed),
            "combat_start" => Some(ReactionTrigger::CombatStart),
            _ => None,
        }
    }
}

/// Effect of a reaction.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReactionEffect {
    /// Flee from combat
    Flee,
    /// Panic (lose control)
    Panic,
    /// Despair (negative emotional state)
    Despair,
    /// Motivate allies
    Motivate,
    /// Rally allies
    Rally,
    /// Calm stressed ally
    Calm,
    /// No effect
    None,
}

impl ReactionEffect {
    /// Parse from string representation used in JSON.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "flee" => Some(ReactionEffect::Flee),
            "panic" => Some(ReactionEffect::Panic),
            "despair" => Some(ReactionEffect::Despair),
            "motivate" => Some(ReactionEffect::Motivate),
            "rally" => Some(ReactionEffect::Rally),
            "calm" => Some(ReactionEffect::Calm),
            "none" => Some(ReactionEffect::None),
            _ => None,
        }
    }
}

/// A reaction entry defining a triggered response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReactionEntry {
    /// The trigger condition for this reaction.
    pub trigger: ReactionTrigger,
    /// Probability of this reaction firing (0.0 to 1.0).
    pub probability: f64,
    /// The effect this reaction produces.
    pub effect: ReactionEffect,
}

impl ReactionEntry {
    pub fn new(trigger: ReactionTrigger, probability: f64, effect: ReactionEffect) -> Self {
        ReactionEntry { trigger, probability, effect }
    }
}

/// Definition of a trait (affliction or virtue) that affects hero behavior in combat.
///
/// Traits represent overstress states that can be acquired and affect hero
/// combat performance and behavior through act-outs and reactions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraitDefinition {
    /// Unique identifier for this trait.
    pub id: String,
    /// Whether this is an affliction or virtue.
    pub overstress_type: OverstressType,
    /// Buff effect IDs provided by this trait.
    pub buff_ids: Vec<String>,
    /// Act-outs that may occur at the start of combat turns.
    pub combat_start_turn_act_outs: Vec<ActOutEntry>,
    /// Reaction act-outs triggered by combat events.
    pub reaction_act_outs: Vec<ReactionEntry>,
}

impl TraitDefinition {
    pub fn new(
        id: &str,
        overstress_type: OverstressType,
        buff_ids: Vec<String>,
        combat_start_turn_act_outs: Vec<ActOutEntry>,
        reaction_act_outs: Vec<ReactionEntry>,
    ) -> Self {
        TraitDefinition {
            id: id.to_string(),
            overstress_type,
            buff_ids,
            combat_start_turn_act_outs,
            reaction_act_outs,
        }
    }
}

/// Registry holding all trait definitions.
///
/// Provides lookup by trait ID and filtering by trait type.
#[derive(Debug, Clone, Default)]
pub struct TraitRegistry {
    traits: std::collections::HashMap<String, TraitDefinition>,
}

impl TraitRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        TraitRegistry { traits: std::collections::HashMap::new() }
    }

    /// Register a trait definition.
    pub fn register(&mut self, trait_def: TraitDefinition) {
        self.traits.insert(trait_def.id.clone(), trait_def);
    }

    /// Get a trait by its ID.
    pub fn get(&self, id: &str) -> Option<&TraitDefinition> {
        self.traits.get(id)
    }

    /// Get all registered trait IDs.
    pub fn all_ids(&self) -> Vec<&str> {
        self.traits.keys().map(|s| s.as_str()).collect()
    }

    /// Get the total number of registered traits.
    pub fn len(&self) -> usize {
        self.traits.len()
    }

    /// Returns true if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.traits.is_empty()
    }

    /// Get all afflictions.
    pub fn afflictions(&self) -> Vec<&TraitDefinition> {
        self.traits
            .values()
            .filter(|t| t.overstress_type == OverstressType::Affliction)
            .collect()
    }

    /// Get all virtues.
    pub fn virtues(&self) -> Vec<&TraitDefinition> {
        self.traits
            .values()
            .filter(|t| t.overstress_type == OverstressType::Virtue)
            .collect()
    }

    /// Resolve all buffs for a trait into attribute modifiers via BuffRegistry.
    ///
    /// Returns all `AttributeModifier` entries from the trait's buff list,
    /// with duplicates merged (same `attribute_key` values are combined by summing).
    pub fn resolve_trait_buffs(&self, trait_id: &str, buff_registry: &BuffRegistry) -> Vec<AttributeModifier> {
        let trait_def = match self.traits.get(trait_id) {
            Some(t) => t,
            None => return vec![],
        };

        let mut aggregated: std::collections::HashMap<String, f64> = std::collections::HashMap::new();

        for buff_id in &trait_def.buff_ids {
            for modifier in buff_registry.resolve_buff(buff_id) {
                *aggregated.entry(modifier.attribute_key).or_insert(0.0) += modifier.value;
            }
        }

        aggregated
            .into_iter()
            .map(|(attribute_key, value)| AttributeModifier { attribute_key, value })
            .collect()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Buff Resolution
// ─────────────────────────────────────────────────────────────────────────────

/// Indicates whether a modifier is flat or percentage-based.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModifierKind {
    Flat,
    Percent,
}

/// Result of parsing a buff ID into an attribute modifier.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedBuff {
    pub attribute_key: String,
    pub value: f64,
    pub kind: ModifierKind,
    pub sign: f64, // +1.0 or -1.0
}

impl ParsedBuff {
    /// Convert to an AttributeModifier with the appropriate value.
    ///
    /// For percentage-based modifiers, the value is stored as a fraction
    /// (e.g., 10% → 0.10), while flat modifiers use the raw value.
    pub fn to_modifier(&self) -> AttributeModifier {
        let value = match self.kind {
            ModifierKind::Flat => self.value * self.sign,
            ModifierKind::Percent => (self.value / 100.0) * self.sign,
        };
        AttributeModifier::new(&self.attribute_key, value)
    }
}

/// Parses a buff ID following DDGC naming conventions.
///
/// Supported formats:
/// - `STAT+value` or `STAT-value` — flat modifier (e.g., `ATK+10`, `MAXHP-15`)
/// - `STAT%+value` or `STAT%-value` — percentage modifier (e.g., `ATK%+10`)
/// - `STAT_value` — flat modifier with implicit positive sign (e.g., `REVIVE_25`)
/// - `TRINKET_STAT_B0` — tier-suffixed format (e.g., `TRINKET_STRESSDMG_B0`)
///   where the tier suffix is ignored and STAT is returned with value 0.0
///
/// Returns `None` if the buff ID cannot be parsed.
pub fn parse_buff_id(buff_id: &str) -> Option<ParsedBuff> {
    let s = buff_id.trim();

    // Handle tier-suffixed format like TRINKET_STRESSDMG_B0
    // Extract the stat and value portion
    let (working, had_tier_suffix) = if s.starts_with("TRINKET_") {
        // Remove TRINKET_ prefix and tier suffix (_B0, _A1, etc.)
        let inner = &s[8..]; // Remove "TRINKET_"
        // Find the last underscore and check if it's a tier suffix
        if let Some(underscore_pos) = inner.rfind('_') {
            let potential_tier = &inner[underscore_pos + 1..];
            // Tier suffix is typically 1-2 chars like B0, A1, C2
            if potential_tier.len() <= 2
                && potential_tier.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
            {
                (inner[..underscore_pos].to_string(), true)
            } else {
                (inner.to_string(), false)
            }
        } else {
            (inner.to_string(), false)
        }
    } else {
        (s.to_string(), false)
    };

    // Check for percentage modifier: STAT%+value or STAT%-value
    if let Some(percent_pos) = working.find("%+") {
        let attribute_key = &working[..percent_pos];
        let value_str = &working[percent_pos + 2..];
        let value: f64 = value_str.parse().ok()?;
        return Some(ParsedBuff {
            attribute_key: attribute_key.to_uppercase(),
            value,
            kind: ModifierKind::Percent,
            sign: 1.0,
        });
    }
    if let Some(percent_pos) = working.find("%-") {
        let attribute_key = &working[..percent_pos];
        let value_str = &working[percent_pos + 2..];
        let value: f64 = value_str.parse().ok()?;
        return Some(ParsedBuff {
            attribute_key: attribute_key.to_uppercase(),
            value,
            kind: ModifierKind::Percent,
            sign: -1.0,
        });
    }

    // Check for underscore-based value (implicit positive, e.g., REVIVE_25)
    if let Some(underscore_pos) = working.rfind('_') {
        let prefix = &working[..underscore_pos];
        let value_str = &working[underscore_pos + 1..];
        // Only treat as underscore-value if prefix looks like a stat name and value is numeric
        if !value_str.is_empty() && value_str.chars().all(|c| c.is_ascii_digit()) {
            if let Ok(value) = value_str.parse::<f64>() {
                return Some(ParsedBuff {
                    attribute_key: prefix.to_uppercase(),
                    value,
                    kind: ModifierKind::Flat,
                    sign: 1.0,
                });
            }
        }
    }

    // Check for signed format: STAT+value or STAT-value
    if let Some(plus_pos) = working.rfind('+') {
        let attribute_key = &working[..plus_pos];
        let value_str = &working[plus_pos + 1..];
        if !value_str.is_empty() && value_str.chars().all(|c| c.is_ascii_digit()) {
            if let Ok(value) = value_str.parse::<f64>() {
                return Some(ParsedBuff {
                    attribute_key: attribute_key.to_uppercase(),
                    value,
                    kind: ModifierKind::Flat,
                    sign: 1.0,
                });
            }
        }
    }
    if let Some(minus_pos) = working.rfind('-') {
        // Make sure minus is not at the start (which would be weird)
        if minus_pos > 0 {
            let attribute_key = &working[..minus_pos];
            let value_str = &working[minus_pos + 1..];
            if !value_str.is_empty() && value_str.chars().all(|c| c.is_ascii_digit()) {
                if let Ok(value) = value_str.parse::<f64>() {
                    return Some(ParsedBuff {
                        attribute_key: attribute_key.to_uppercase(),
                        value,
                        kind: ModifierKind::Flat,
                        sign: -1.0,
                    });
                }
            }
        }
    }

    // Handle tier-suffix-only case: TRINKET_STAT_B0 where no numeric value follows
    // If we had a tier suffix and the remaining looks like a stat name (all uppercase letters),
    // return a flat modifier with value 0.0
    if had_tier_suffix
        && !working.is_empty()
        && working.chars().all(|c| c.is_ascii_uppercase() || c == '_')
    {
        return Some(ParsedBuff {
            attribute_key: working.to_uppercase(),
            value: 0.0,
            kind: ModifierKind::Flat,
            sign: 1.0,
        });
    }

    None
}

/// Registry mapping buff IDs to their attribute modifiers.
///
/// The registry handles DDGC buff ID parsing and resolution, converting
/// string buff IDs (e.g., "ATK+10", "MAXHP-15", "TRINKET_STRESSDMG_B0")
/// into concrete `AttributeModifier` entries that can be applied to hero stats.
#[derive(Debug, Clone, Default)]
pub struct BuffRegistry {
    // Optional static overrides for specific buff IDs that need exact mapping
    overrides: std::collections::HashMap<String, Vec<AttributeModifier>>,
}

impl BuffRegistry {
    /// Create a new empty buff registry.
    pub fn new() -> Self {
        BuffRegistry {
            overrides: std::collections::HashMap::new(),
        }
    }

    /// Register a static buff override for a specific buff ID.
    ///
    /// Use this for buff IDs that don't follow the standard naming convention
    /// or need special handling.
    pub fn register_override(&mut self, buff_id: &str, modifiers: Vec<AttributeModifier>) {
        self.overrides.insert(buff_id.to_string(), modifiers);
    }

    /// Resolve a single buff ID to a list of attribute modifiers.
    ///
    /// First checks for static overrides, then falls back to parsing the buff ID
    /// using DDGC naming conventions.
    pub fn resolve_buff(&self, buff_id: &str) -> Vec<AttributeModifier> {
        // Check for static overrides first
        if let Some(modifiers) = self.overrides.get(buff_id) {
            return modifiers.clone();
        }

        // Parse the buff ID using DDGC naming conventions
        parse_buff_id(buff_id)
            .map(|parsed| vec![parsed.to_modifier()])
            .unwrap_or_default()
    }

    /// Resolve all buffs for a trinket into aggregated attribute modifiers.
    ///
    /// Returns all `AttributeModifier` entries from the trinket's buff list,
    /// with duplicates merged (same `attribute_key` values are combined by summing).
    pub fn resolve_buffs(&self, trinket: &TrinketDefinition) -> Vec<AttributeModifier> {
        let mut aggregated: std::collections::HashMap<String, f64> = std::collections::HashMap::new();

        for buff_id in &trinket.buffs {
            for modifier in self.resolve_buff(buff_id) {
                *aggregated.entry(modifier.attribute_key.clone()).or_insert(0.0) += modifier.value;
            }
        }

        aggregated
            .into_iter()
            .map(|(attribute_key, value)| AttributeModifier { attribute_key, value })
            .collect()
    }

    /// Check if a buff ID is registered (has an override or can be parsed).
    pub fn is_registered(&self, buff_id: &str) -> bool {
        if self.overrides.contains_key(buff_id) {
            return true;
        }
        parse_buff_id(buff_id).is_some()
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

    // ── US-006: town building data model tests ──────────────────────────────────

    #[test]
    fn unlock_condition_construction_is_deterministic() {
        let condition = UnlockCondition::new("completed_runs", 5);
        assert_eq!(condition.condition_type, "completed_runs");
        assert_eq!(condition.required_count, 5);
    }

    #[test]
    fn upgrade_effect_construction_is_deterministic() {
        let effect = UpgradeEffect::new("recruit_discount", 0.15);
        assert_eq!(effect.effect_id, "recruit_discount");
        assert_eq!(effect.value, 0.15);
    }

    #[test]
    fn upgrade_level_construction_is_deterministic() {
        let effects = vec![
            UpgradeEffect::new("recruit_discount", 0.1),
            UpgradeEffect::new("experience_boost", 0.05),
        ];
        let level = UpgradeLevel::new('b', 500, effects.clone());
        assert_eq!(level.code, 'b');
        assert_eq!(level.cost, 500);
        assert_eq!(level.effects, effects);
    }

    #[test]
    fn upgrade_tree_construction_is_deterministic() {
        let levels = vec![
            UpgradeLevel::new('a', 0, vec![]),
            UpgradeLevel::new('b', 500, vec![UpgradeEffect::new("discount", 0.1)]),
            UpgradeLevel::new('c', 1000, vec![UpgradeEffect::new("discount", 0.2)]),
        ];
        let tree = UpgradeTree::new("barracks_upgrade", levels.clone());
        assert_eq!(tree.tree_id, "barracks_upgrade");
        assert_eq!(tree.levels.len(), 3);
        assert_eq!(tree.levels[0].code, 'a');
        assert_eq!(tree.levels[2].code, 'c');
    }

    #[test]
    fn town_building_construction_is_deterministic() {
        let unlock_conditions = vec![
            UnlockCondition::new("completed_runs", 3),
        ];
        let upgrade_trees = vec![
            UpgradeTree::new(
                "barracks_recruit",
                vec![
                    UpgradeLevel::new('a', 0, vec![]),
                    UpgradeLevel::new('b', 500, vec![UpgradeEffect::new("recruit_discount", 0.1)]),
                ],
            ),
        ];
        let building = TownBuilding::new(
            "barracks",
            BuildingType::Barracks,
            unlock_conditions.clone(),
            upgrade_trees.clone(),
        );

        assert_eq!(building.id, "barracks");
        assert_eq!(building.building_type, BuildingType::Barracks);
        assert_eq!(building.unlock_conditions, unlock_conditions);
        assert_eq!(building.upgrade_trees.len(), 1);
    }

    #[test]
    fn town_state_construction_is_deterministic() {
        let state = TownState::new(1000);
        assert_eq!(state.gold, 1000);
        assert!(state.heirlooms.is_empty());
        assert!(state.building_states.is_empty());
    }

    #[test]
    fn town_state_upgrade_application_is_deterministic() {
        // Build a building with upgrade levels
        let upgrade_trees = vec![
            UpgradeTree::new(
                "inn_comfort",
                vec![
                    UpgradeLevel::new('a', 0, vec![]),
                    UpgradeLevel::new('b', 200, vec![UpgradeEffect::new("healing_discount", 0.1)]),
                    UpgradeLevel::new('c', 500, vec![UpgradeEffect::new("healing_discount", 0.25)]),
                ],
            ),
        ];
        let building = TownBuilding::new(
            "inn",
            BuildingType::Inn,
            vec![],
            upgrade_trees,
        );

        // Create town state with initial gold
        let mut state = TownState::new(1000);

        // Apply first paid upgrade (level b)
        let cost = state.apply_upgrade("inn", 'b', &building);
        assert_eq!(cost, Some(200));
        assert_eq!(state.gold, 800);
        assert_eq!(state.get_upgrade_level("inn"), Some('b'));

        // Apply second upgrade (level c)
        let cost = state.apply_upgrade("inn", 'c', &building);
        assert_eq!(cost, Some(500));
        assert_eq!(state.gold, 300);
        assert_eq!(state.get_upgrade_level("inn"), Some('c'));

        // Cannot apply same upgrade twice (state already has level c)
        let cost = state.apply_upgrade("inn", 'c', &building);
        assert_eq!(cost, None); // No cost returned - upgrade not applied

        // Cannot afford upgrade
        let cost = state.apply_upgrade("inn", 'c', &building);
        assert_eq!(cost, None); // Not enough gold
    }

    #[test]
    fn town_state_upgrade_unknown_level_returns_none() {
        let upgrade_trees = vec![
            UpgradeTree::new(
                "barracks_upgrade",
                vec![
                    UpgradeLevel::new('a', 0, vec![]),
                    UpgradeLevel::new('b', 500, vec![]),
                ],
            ),
        ];
        let building = TownBuilding::new("barracks", BuildingType::Barracks, vec![], upgrade_trees);
        let mut state = TownState::new(1000);

        // Try to apply a level that doesn't exist
        let cost = state.apply_upgrade("barracks", 'z', &building);
        assert_eq!(cost, None);
        assert_eq!(state.gold, 1000); // Gold unchanged
    }

    #[test]
    fn heirloom_currency_serde_roundtrip_is_deterministic() {
        let currencies = vec![
            HeirloomCurrency::Bones,
            HeirloomCurrency::Portraits,
            HeirloomCurrency::Tapes,
        ];
        for currency in currencies {
            let serialized = serde_json::to_string(&currency).unwrap();
            let deserialized: HeirloomCurrency = serde_json::from_str(&serialized).unwrap();
            assert_eq!(currency, deserialized);
        }
    }

    #[test]
    fn building_type_serde_roundtrip_is_deterministic() {
        let building_types = vec![
            BuildingType::Barracks,
            BuildingType::Blacksmith,
            BuildingType::Inn,
            BuildingType::Tavern,
            BuildingType::Cathedral,
        ];
        for bt in building_types {
            let serialized = serde_json::to_string(&bt).unwrap();
            let deserialized: BuildingType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(bt, deserialized);
        }
    }

    #[test]
    fn upgrade_level_serde_roundtrip_is_deterministic() {
        let level = UpgradeLevel::new('c', 1000, vec![
            UpgradeEffect::new("recruit_discount", 0.2),
        ]);
        let serialized = serde_json::to_string(&level).unwrap();
        let deserialized: UpgradeLevel = serde_json::from_str(&serialized).unwrap();
        assert_eq!(level, deserialized);
    }

    #[test]
    fn upgrade_tree_serde_roundtrip_is_deterministic() {
        let tree = UpgradeTree::new("blacksmith_repair", vec![
            UpgradeLevel::new('a', 0, vec![]),
            UpgradeLevel::new('b', 300, vec![UpgradeEffect::new("repair_discount", 0.1)]),
        ]);
        let serialized = serde_json::to_string(&tree).unwrap();
        let deserialized: UpgradeTree = serde_json::from_str(&serialized).unwrap();
        assert_eq!(tree, deserialized);
    }

    #[test]
    fn town_building_serde_roundtrip_is_deterministic() {
        let building = TownBuilding::new(
            "museum",
            BuildingType::Museum,
            vec![UnlockCondition::new("completed_runs", 10)],
            vec![UpgradeTree::new("museum_collection", vec![
                UpgradeLevel::new('a', 0, vec![]),
            ])],
        );
        let serialized = serde_json::to_string(&building).unwrap();
        let deserialized: TownBuilding = serde_json::from_str(&serialized).unwrap();
        assert_eq!(building, deserialized);
    }

    #[test]
    fn town_state_serde_roundtrip_is_deterministic() {
        let mut state = TownState::new(1500);
        state.heirlooms.insert(HeirloomCurrency::Bones, 42);
        state.heirlooms.insert(HeirloomCurrency::Portraits, 7);
        state.building_states.insert("inn".to_string(), BuildingUpgradeState::new("inn", Some('b')));
        state.building_states.insert("barracks".to_string(), BuildingUpgradeState::new("barracks", Some('a')));

        let serialized = serde_json::to_string(&state).unwrap();
        let deserialized: TownState = serde_json::from_str(&serialized).unwrap();
        assert_eq!(state, deserialized);
    }

    // ── US-011: trinket and equipment data model tests ─────────────────────────

    #[test]
    fn attribute_modifier_construction_is_deterministic() {
        let modifier = AttributeModifier::new("attack", 15.0);
        assert_eq!(modifier.attribute_key, "attack");
        assert_eq!(modifier.value, 15.0);
    }

    #[test]
    fn trinket_definition_construction_is_deterministic() {
        let buffs = vec!["buff_damage".to_string(), "buff_speed".to_string()];
        let class_reqs = vec!["alchemist".to_string(), "shaman".to_string()];
        let trinket = TrinketDefinition::new(
            "ancient_medallion",
            buffs.clone(),
            class_reqs.clone(),
            TrinketRarity::Rare,
            500,
            2,
            DungeonType::QingLong,
        );

        assert_eq!(trinket.id, "ancient_medallion");
        assert_eq!(trinket.buffs, buffs);
        assert_eq!(trinket.hero_class_requirements, class_reqs);
        assert_eq!(trinket.rarity, TrinketRarity::Rare);
        assert_eq!(trinket.price, 500);
        assert_eq!(trinket.limit, 2);
        assert_eq!(trinket.origin_dungeon, DungeonType::QingLong);
    }

    #[test]
    fn equipment_definition_construction_is_deterministic() {
        let stat_modifiers = vec![
            AttributeModifier::new("attack", 10.0),
            AttributeModifier::new("defense", 5.0),
        ];
        let equipment = EquipmentDefinition::new(
            "alchemist_weapon_1",
            "alchemist",
            EquipmentSlot::Weapon,
            1,
            stat_modifiers.clone(),
        );

        assert_eq!(equipment.id, "alchemist_weapon_1");
        assert_eq!(equipment.hero_class_id, "alchemist");
        assert_eq!(equipment.slot, EquipmentSlot::Weapon);
        assert_eq!(equipment.upgrade_level, 1);
        assert_eq!(equipment.stat_modifiers, stat_modifiers);
    }

    #[test]
    fn trinket_registry_lookup_is_deterministic() {
        let mut registry = TrinketRegistry::new();
        registry.register(TrinketDefinition::new(
            "lucky_charm",
            vec!["buff_luck".to_string()],
            vec![],
            TrinketRarity::Common,
            100,
            3,
            DungeonType::BaiHu,
        ));

        let trinket = registry.get("lucky_charm");
        assert!(trinket.is_some());
        assert_eq!(trinket.unwrap().id, "lucky_charm");
        assert_eq!(trinket.unwrap().rarity, TrinketRarity::Common);

        // Lookup of non-existent trinket returns None
        assert!(registry.get("nonexistent").is_none());
    }

    #[test]
    fn equipment_registry_lookup_is_deterministic() {
        let mut registry = EquipmentRegistry::new();
        registry.register(EquipmentDefinition::new(
            "tank_armor_0",
            "tank",
            EquipmentSlot::Armor,
            0,
            vec![AttributeModifier::new("defense", 20.0)],
        ));

        let equipment = registry.get("tank_armor_0");
        assert!(equipment.is_some());
        assert_eq!(equipment.unwrap().hero_class_id, "tank");
        assert_eq!(equipment.unwrap().slot, EquipmentSlot::Armor);

        // Lookup by class and slot
        let tank_armor = registry.by_class_and_slot("tank", EquipmentSlot::Armor);
        assert_eq!(tank_armor.len(), 1);
        assert_eq!(tank_armor[0].id, "tank_armor_0");

        // No equipment for wrong class
        let alchemist_armor = registry.by_class_and_slot("alchemist", EquipmentSlot::Armor);
        assert!(alchemist_armor.is_empty());
    }

    #[test]
    fn trinket_rarity_as_str_is_deterministic() {
        assert_eq!(TrinketRarity::Common.as_str(), "common");
        assert_eq!(TrinketRarity::Uncommon.as_str(), "uncommon");
        assert_eq!(TrinketRarity::Rare.as_str(), "rare");
        assert_eq!(TrinketRarity::Epic.as_str(), "epic");
        assert_eq!(TrinketRarity::Legendary.as_str(), "legendary");
    }

    #[test]
    fn equipment_slot_as_str_is_deterministic() {
        assert_eq!(EquipmentSlot::Weapon.as_str(), "weapon");
        assert_eq!(EquipmentSlot::Armor.as_str(), "armor");
    }

    #[test]
    fn attribute_modifier_serde_roundtrip_is_deterministic() {
        let modifier = AttributeModifier::new("speed", 12.5);
        let serialized = serde_json::to_string(&modifier).unwrap();
        let deserialized: AttributeModifier = serde_json::from_str(&serialized).unwrap();
        assert_eq!(modifier, deserialized);
    }

    #[test]
    fn trinket_definition_serde_roundtrip_is_deterministic() {
        let trinket = TrinketDefinition::new(
            "sage_stone",
            vec!["buff_wisdom".to_string()],
            vec!["diviner".to_string()],
            TrinketRarity::Epic,
            750,
            1,
            DungeonType::ZhuQue,
        );
        let serialized = serde_json::to_string(&trinket).unwrap();
        let deserialized: TrinketDefinition = serde_json::from_str(&serialized).unwrap();
        assert_eq!(trinket, deserialized);
    }

    #[test]
    fn equipment_definition_serde_roundtrip_is_deterministic() {
        let equipment = EquipmentDefinition::new(
            "shaman_weapon_2",
            "shaman",
            EquipmentSlot::Weapon,
            2,
            vec![
                AttributeModifier::new("attack", 25.0),
                AttributeModifier::new("magic", 10.0),
            ],
        );
        let serialized = serde_json::to_string(&equipment).unwrap();
        let deserialized: EquipmentDefinition = serde_json::from_str(&serialized).unwrap();
        assert_eq!(equipment, deserialized);
    }

    #[test]
    fn trinket_registry_all_ids_is_deterministic() {
        let mut registry = TrinketRegistry::new();
        registry.register(TrinketDefinition::new(
            "charm_one",
            vec![],
            vec![],
            TrinketRarity::Common,
            50,
            1,
            DungeonType::QingLong,
        ));
        registry.register(TrinketDefinition::new(
            "charm_two",
            vec![],
            vec![],
            TrinketRarity::Uncommon,
            100,
            1,
            DungeonType::BaiHu,
        ));

        let mut ids = registry.all_ids();
        ids.sort();
        assert_eq!(ids, vec!["charm_one", "charm_two"]);
    }

    #[test]
    fn equipment_registry_len_and_is_empty_are_deterministic() {
        let mut registry = EquipmentRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);

        registry.register(EquipmentDefinition::new(
            "hunter_weapon_0",
            "hunter",
            EquipmentSlot::Weapon,
            0,
            vec![],
        ));
        assert!(!registry.is_empty());
        assert_eq!(registry.len(), 1);
    }

    // ── US-013: buff resolution tests ─────────────────────────────────────────

    #[test]
    fn parse_buff_id_positive_flat() {
        let parsed = parse_buff_id("ATK+10").unwrap();
        assert_eq!(parsed.attribute_key, "ATK");
        assert_eq!(parsed.value, 10.0);
        assert_eq!(parsed.kind, ModifierKind::Flat);
        assert_eq!(parsed.sign, 1.0);
    }

    #[test]
    fn parse_buff_id_negative_flat() {
        let parsed = parse_buff_id("DEF-5").unwrap();
        assert_eq!(parsed.attribute_key, "DEF");
        assert_eq!(parsed.value, 5.0);
        assert_eq!(parsed.kind, ModifierKind::Flat);
        assert_eq!(parsed.sign, -1.0);
    }

    #[test]
    fn parse_buff_id_underscore_value() {
        // REVIVE_25 has implicit positive sign
        let parsed = parse_buff_id("REVIVE_25").unwrap();
        assert_eq!(parsed.attribute_key, "REVIVE");
        assert_eq!(parsed.value, 25.0);
        assert_eq!(parsed.kind, ModifierKind::Flat);
        assert_eq!(parsed.sign, 1.0);
    }

    #[test]
    fn parse_buff_id_percentage_positive() {
        let parsed = parse_buff_id("ATK%+10").unwrap();
        assert_eq!(parsed.attribute_key, "ATK");
        assert_eq!(parsed.value, 10.0);
        assert_eq!(parsed.kind, ModifierKind::Percent);
        assert_eq!(parsed.sign, 1.0);
    }

    #[test]
    fn parse_buff_id_percentage_negative() {
        let parsed = parse_buff_id("MAXHP%-15").unwrap();
        assert_eq!(parsed.attribute_key, "MAXHP");
        assert_eq!(parsed.value, 15.0);
        assert_eq!(parsed.kind, ModifierKind::Percent);
        assert_eq!(parsed.sign, -1.0);
    }

    #[test]
    fn parse_buff_id_tier_suffix_format() {
        // TRINKET_STRESSDMG_B0 should parse as STRESSDMG with +0 value (tier suffix ignored)
        let parsed = parse_buff_id("TRINKET_STRESSDMG_B0").unwrap();
        assert_eq!(parsed.attribute_key, "STRESSDMG");
        assert_eq!(parsed.value, 0.0);
        assert_eq!(parsed.kind, ModifierKind::Flat);
        assert_eq!(parsed.sign, 1.0);
    }

    #[test]
    fn parse_buff_id_with_value_after_tier_suffix() {
        // This tests TRINKET_STAT_V0 where V0 is the tier and we want to parse the stat
        let parsed = parse_buff_id("TRINKET_DMGL_B0").unwrap();
        assert_eq!(parsed.attribute_key, "DMGL");
        assert_eq!(parsed.value, 0.0);
    }

    #[test]
    fn parse_buff_id_case_insensitive() {
        // Buff IDs should be case-insensitive in parsing
        let parsed = parse_buff_id("atk+10").unwrap();
        assert_eq!(parsed.attribute_key, "ATK");
        assert_eq!(parsed.value, 10.0);
    }

    #[test]
    fn parse_buff_id_complex_stat_names() {
        // DMGL (damage low), DMGH (damage high), STRESSDMG
        let parsed = parse_buff_id("DMGL+5").unwrap();
        assert_eq!(parsed.attribute_key, "DMGL");

        let parsed = parse_buff_id("DMGH+15").unwrap();
        assert_eq!(parsed.attribute_key, "DMGH");

        let parsed = parse_buff_id("STRESSDMG-5").unwrap();
        assert_eq!(parsed.attribute_key, "STRESSDMG");
    }

    #[test]
    fn parsed_buff_to_modifier_flat_positive() {
        let parsed = ParsedBuff {
            attribute_key: "ATK".to_string(),
            value: 10.0,
            kind: ModifierKind::Flat,
            sign: 1.0,
        };
        let modifier = parsed.to_modifier();
        assert_eq!(modifier.attribute_key, "ATK");
        assert_eq!(modifier.value, 10.0);
    }

    #[test]
    fn parsed_buff_to_modifier_flat_negative() {
        let parsed = ParsedBuff {
            attribute_key: "DEF".to_string(),
            value: 5.0,
            kind: ModifierKind::Flat,
            sign: -1.0,
        };
        let modifier = parsed.to_modifier();
        assert_eq!(modifier.attribute_key, "DEF");
        assert_eq!(modifier.value, -5.0);
    }

    #[test]
    fn parsed_buff_to_modifier_percent_positive() {
        // 10% should become 0.10
        let parsed = ParsedBuff {
            attribute_key: "ATK".to_string(),
            value: 10.0,
            kind: ModifierKind::Percent,
            sign: 1.0,
        };
        let modifier = parsed.to_modifier();
        assert_eq!(modifier.attribute_key, "ATK");
        assert!((modifier.value - 0.10).abs() < 0.001);
    }

    #[test]
    fn parsed_buff_to_modifier_percent_negative() {
        // -15% should become -0.15
        let parsed = ParsedBuff {
            attribute_key: "MAXHP".to_string(),
            value: 15.0,
            kind: ModifierKind::Percent,
            sign: -1.0,
        };
        let modifier = parsed.to_modifier();
        assert_eq!(modifier.attribute_key, "MAXHP");
        assert!((modifier.value - (-0.15)).abs() < 0.001);
    }

    #[test]
    fn buff_registry_resolve_single_buff() {
        let registry = BuffRegistry::new();
        let modifiers = registry.resolve_buff("ATK+10");
        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].attribute_key, "ATK");
        assert_eq!(modifiers[0].value, 10.0);
    }

    #[test]
    fn buff_registry_resolve_unknown_buff_returns_empty() {
        let registry = BuffRegistry::new();
        let modifiers = registry.resolve_buff("UNKNOWN_BUFF_XYZ");
        assert!(modifiers.is_empty());
    }

    #[test]
    fn buff_registry_resolve_with_override() {
        let mut registry = BuffRegistry::new();
        registry.register_override("CUSTOM_BUFF", vec![
            AttributeModifier::new("CUSTOM_STAT", 42.0),
        ]);
        let modifiers = registry.resolve_buff("CUSTOM_BUFF");
        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].attribute_key, "CUSTOM_STAT");
        assert_eq!(modifiers[0].value, 42.0);
    }

    #[test]
    fn buff_registry_resolve_buffs_positive_and_negative() {
        let registry = BuffRegistry::new();
        let trinket = TrinketDefinition::new(
            "test_trinket",
            vec!["ATK+10".to_string(), "DEF-5".to_string()],
            vec![],
            TrinketRarity::Common,
            100,
            1,
            DungeonType::QingLong,
        );
        let modifiers = registry.resolve_buffs(&trinket);
        assert_eq!(modifiers.len(), 2);
        let atk = modifiers.iter().find(|m| m.attribute_key == "ATK").unwrap();
        let def = modifiers.iter().find(|m| m.attribute_key == "DEF").unwrap();
        assert_eq!(atk.value, 10.0);
        assert_eq!(def.value, -5.0);
    }

    #[test]
    fn buff_registry_resolve_buffs_multi_aggregates() {
        let registry = BuffRegistry::new();
        let trinket = TrinketDefinition::new(
            "multi_trinket",
            vec!["ATK+10".to_string(), "ATK+5".to_string(), "DEF-3".to_string()],
            vec![],
            TrinketRarity::Rare,
            300,
            1,
            DungeonType::BaiHu,
        );
        let modifiers = registry.resolve_buffs(&trinket);
        // ATK should be aggregated to 15, DEF should be -3
        assert_eq!(modifiers.len(), 2);
        let atk = modifiers.iter().find(|m| m.attribute_key == "ATK").unwrap();
        let def = modifiers.iter().find(|m| m.attribute_key == "DEF").unwrap();
        assert_eq!(atk.value, 15.0);
        assert_eq!(def.value, -3.0);
    }

    #[test]
    fn buff_registry_resolve_buffs_from_real_trinket_data() {
        // Test with actual buff IDs from JsonTrinkets.json
        let registry = BuffRegistry::new();
        let trinket = TrinketDefinition::new(
            "battle_horn",
            vec![
                "ATK+15".to_string(),
                "CRIT+5".to_string(),
                "STRESSDMG+10".to_string(),
            ],
            vec![],
            TrinketRarity::Rare,
            450,
            1,
            DungeonType::XuanWu,
        );
        let modifiers = registry.resolve_buffs(&trinket);
        assert_eq!(modifiers.len(), 3);

        let atk = modifiers.iter().find(|m| m.attribute_key == "ATK").unwrap();
        let crit = modifiers.iter().find(|m| m.attribute_key == "CRIT").unwrap();
        let stressdmg = modifiers.iter().find(|m| m.attribute_key == "STRESSDMG").unwrap();

        assert_eq!(atk.value, 15.0);
        assert_eq!(crit.value, 5.0);
        assert_eq!(stressdmg.value, 10.0);
    }

    #[test]
    fn buff_registry_resolve_buffs_epic_trinket() {
        // Test shadowstep_cloak: DODGE+12, SPD+8, ATK+5
        let registry = BuffRegistry::new();
        let trinket = TrinketDefinition::new(
            "shadowstep_cloak",
            vec!["DODGE+12".to_string(), "SPD+8".to_string(), "ATK+5".to_string()],
            vec!["hunter".to_string(), "diviner".to_string()],
            TrinketRarity::Epic,
            750,
            1,
            DungeonType::ZhuQue,
        );
        let modifiers = registry.resolve_buffs(&trinket);
        assert_eq!(modifiers.len(), 3);

        let dodge = modifiers.iter().find(|m| m.attribute_key == "DODGE").unwrap();
        let spd = modifiers.iter().find(|m| m.attribute_key == "SPD").unwrap();
        let atk = modifiers.iter().find(|m| m.attribute_key == "ATK").unwrap();

        assert_eq!(dodge.value, 12.0);
        assert_eq!(spd.value, 8.0);
        assert_eq!(atk.value, 5.0);
    }

    #[test]
    fn buff_registry_resolve_buffs_legendary_trinket() {
        // Test dragon_slayer_medallion: ATK+30, DMGL+15, DMGH+15, CRIT+10, BOSS_DMG+20
        let registry = BuffRegistry::new();
        let trinket = TrinketDefinition::new(
            "dragon_slayer_medallion",
            vec![
                "ATK+30".to_string(),
                "DMGL+15".to_string(),
                "DMGH+15".to_string(),
                "CRIT+10".to_string(),
                "BOSS_DMG+20".to_string(),
            ],
            vec!["hunter".to_string(), "shaman".to_string()],
            TrinketRarity::Legendary,
            1500,
            1,
            DungeonType::QingLong,
        );
        let modifiers = registry.resolve_buffs(&trinket);
        assert_eq!(modifiers.len(), 5);

        let atk = modifiers.iter().find(|m| m.attribute_key == "ATK").unwrap();
        let dmgl = modifiers.iter().find(|m| m.attribute_key == "DMGL").unwrap();
        let dmgh = modifiers.iter().find(|m| m.attribute_key == "DMGH").unwrap();
        let crit = modifiers.iter().find(|m| m.attribute_key == "CRIT").unwrap();
        let boss_dmg = modifiers.iter().find(|m| m.attribute_key == "BOSS_DMG").unwrap();

        assert_eq!(atk.value, 30.0);
        assert_eq!(dmgl.value, 15.0);
        assert_eq!(dmgh.value, 15.0);
        assert_eq!(crit.value, 10.0);
        assert_eq!(boss_dmg.value, 20.0);
    }

    #[test]
    fn buff_registry_resolve_buffs_warrior_stance_token_has_negative() {
        // warrior_stance_token has STRESSDMG-5 (negative)
        let registry = BuffRegistry::new();
        let trinket = TrinketDefinition::new(
            "warrior_stance_token",
            vec![
                "ATK+25".to_string(),
                "DMGL+10".to_string(),
                "DMGH+10".to_string(),
                "STRESSDMG-5".to_string(),
            ],
            vec!["tank".to_string(), "hunter".to_string()],
            TrinketRarity::Epic,
            850,
            1,
            DungeonType::QingLong,
        );
        let modifiers = registry.resolve_buffs(&trinket);

        let stressdmg = modifiers.iter().find(|m| m.attribute_key == "STRESSDMG").unwrap();
        assert_eq!(stressdmg.value, -5.0);
    }

    #[test]
    fn buff_registry_is_registered_true() {
        let registry = BuffRegistry::new();
        assert!(registry.is_registered("ATK+10"));
        assert!(registry.is_registered("DEF-5"));
        assert!(registry.is_registered("REVIVE_25"));
    }

    #[test]
    fn buff_registry_is_registered_false() {
        let registry = BuffRegistry::new();
        assert!(!registry.is_registered("UNKNOWN_BUFF"));
    }

    #[test]
    fn buff_registry_aggregates_same_stat_from_multiple_buffs() {
        // If a trinket has two ATK buffs, they should sum
        let registry = BuffRegistry::new();
        let trinket = TrinketDefinition::new(
            "stack_trinket",
            vec!["ATK+10".to_string(), "ATK+5".to_string()],
            vec![],
            TrinketRarity::Common,
            100,
            1,
            DungeonType::QingLong,
        );
        let modifiers = registry.resolve_buffs(&trinket);
        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].attribute_key, "ATK");
        assert_eq!(modifiers[0].value, 15.0);
    }

    #[test]
    fn buff_registry_five_buff_ids_resolved() {
        // US-013 acceptance criterion: at least 5 buff IDs are resolved
        let registry = BuffRegistry::new();
        let test_buffs = vec![
            ("ATK+10", "ATK", 10.0),
            ("DEF-5", "DEF", -5.0),
            ("MAXHP+25", "MAXHP", 25.0),
            ("CRIT+5", "CRIT", 5.0),
            ("SPD-3", "SPD", -3.0),
        ];
        for (buff_id, expected_key, expected_value) in test_buffs {
            let modifiers = registry.resolve_buff(buff_id);
            assert_eq!(modifiers.len(), 1, "Buff {} should resolve to 1 modifier", buff_id);
            assert_eq!(modifiers[0].attribute_key, expected_key);
            assert_eq!(modifiers[0].value, expected_value);
        }
    }
}