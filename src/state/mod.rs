//! DDGC game state — loaded content datasets and campaign persistence.
//!
//! This module holds in-memory game state loaded from DDGC data files,
//! plus the active campaign snapshot. It provides a bridge between the
//! contracts-layer parsing and the run-layer execution.
//!
//! # Architecture
//!
//! - **Contracts** (`crate::contracts`): data model and JSON parsing.
//! - **State** (`crate::state`): loaded content datasets + campaign persistence (this module).
//! - **Planner** (`crate::planner`): selection/recommendation logic.
//! - **Run** (`crate::run`): runtime phase resolution and effect application.
//!
//! # Save/Load boundary
//!
//! [`GameState::campaign`] ([`CampaignState`]) is the canonical save/load
//! boundary for the headless migration. It captures every gameplay-significant
//! field needed to faithfully save and restore a DDGC campaign across sessions:
//! gold, heirloom currencies, hero roster (health, stress, level, xp, quirks,
//! traits, skills, equipment), inventory, town building states, run history,
//! and quest progress.
//!
//! Persistence is JSON-based via serde. The schema is explicitly versioned
//! ([`CAMPAIGN_SNAPSHOT_VERSION`]) so consumers can reject unsupported formats.
//! All keyed collections use `BTreeMap` so serialization is deterministic —
//! identical state always produces identical save bytes.

use std::path::{Path, PathBuf};

use crate::contracts::{
    BuffRegistry, BuildingRegistry, CampaignState, CampingSkillRegistry, CurioRegistry,
    DungeonEncounterRegistry, EquipmentRegistry, ObstacleRegistry, QuestRegistry,
    QuirkRegistry, TraitRegistry, TrapRegistry, TrinketRegistry,
    parse::{parse_buildings_json, parse_camping_json, parse_curios_csv, parse_obstacles_json,
            parse_quirks_json, parse_traits_json, parse_traps_json},
};

/// The runtime phase of the application host.
///
/// This enum mirrors [`crate::contracts::host::HostPhase`] to allow
/// the state layer to track host-level state transitions without
/// depending on the contracts::host module directly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostPhase {
    /// Host is initialized but not yet booted.
    Uninitialized,
    /// Host is booting (contract packages loading).
    Booting,
    /// Host is ready to run.
    Ready,
    /// Host encountered a fatal error and cannot proceed.
    FatalError,
    /// Host is unsupported (feature not available in this build).
    Unsupported,
}

impl Default for HostPhase {
    fn default() -> Self {
        HostPhase::Uninitialized
    }
}

impl std::fmt::Display for HostPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HostPhase::Uninitialized => write!(f, "uninitialized"),
            HostPhase::Booting => write!(f, "booting"),
            HostPhase::Ready => write!(f, "ready"),
            HostPhase::FatalError => write!(f, "fatal_error"),
            HostPhase::Unsupported => write!(f, "unsupported"),
        }
    }
}

/// Full game state loaded from DDGC data files.
///
/// Holds all parsed content datasets plus the active campaign snapshot.
/// Construct via [`GameState::load`] or [`GameState::load_from`].
///
/// # Campaign lifecycle
///
/// After loading content, start a fresh campaign with [`GameState::new_campaign`]
/// or restore a saved campaign with [`GameState::load_campaign`]. Persist progress
/// at any time with [`GameState::save_campaign`].
///
/// # Host phase
///
/// The [`host_phase`] field tracks the runtime phase of the application host,
/// mirroring the `HostPhase` enum from `DdgcHost` in the contracts module.
/// This allows the state layer to drive host-level state transitions.
#[derive(Debug, Clone)]
pub struct GameState {
    /// All camping skill definitions from JsonCamping.json.
    pub camping_skills: CampingSkillRegistry,
    /// The active campaign snapshot — canonical save/load boundary.
    pub campaign: CampaignState,
    /// Path to the data directory used for loading.
    pub data_dir: PathBuf,

    // ─── Contract registries ────────────────────────────────────────────

    /// Curio registry loaded from Curios.csv.
    pub curio_registry: CurioRegistry,
    /// Trap registry loaded from Traps.json.
    pub trap_registry: TrapRegistry,
    /// Obstacle registry loaded from Obstacles.json.
    pub obstacle_registry: ObstacleRegistry,
    /// Building registry loaded from Buildings.json.
    pub building_registry: BuildingRegistry,
    /// Quest registry loaded from Quests.json.
    pub quest_registry: QuestRegistry,
    /// Trinket registry loaded from Trinkets.json.
    pub trinket_registry: TrinketRegistry,
    /// Quirk registry loaded from JsonQuirks.json.
    pub quirk_registry: QuirkRegistry,
    /// Trait registry loaded from JsonTraits.json.
    pub trait_registry: TraitRegistry,
    /// Dungeon encounter registry loaded from encounter definitions.
    pub dungeon_encounter_registry: DungeonEncounterRegistry,
    /// Equipment registry loaded from equipment definitions.
    pub equipment_registry: EquipmentRegistry,
    /// Buff registry loaded from buff definitions.
    pub buff_registry: BuffRegistry,

    // ─── Host phase ─────────────────────────────────────────────────────

    /// Current runtime phase of the application host.
    pub host_phase: HostPhase,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            camping_skills: CampingSkillRegistry::new(),
            campaign: CampaignState::new(0),
            data_dir: PathBuf::new(),
            curio_registry: CurioRegistry::new(),
            trap_registry: TrapRegistry::new(),
            obstacle_registry: ObstacleRegistry::new(),
            building_registry: BuildingRegistry::new(),
            quest_registry: QuestRegistry::new(),
            trinket_registry: TrinketRegistry::new(),
            quirk_registry: QuirkRegistry::new(),
            trait_registry: TraitRegistry::new(),
            dungeon_encounter_registry: DungeonEncounterRegistry::new(),
            equipment_registry: EquipmentRegistry::new(),
            buff_registry: BuffRegistry::new(),
            host_phase: HostPhase::Uninitialized,
        }
    }
}

impl GameState {
    /// Load game state from the default data directory.
    ///
    /// The default data directory is `<project_root>/data/`.
    /// Project root is determined by the `CARGO_MANIFEST_DIR` environment
    /// variable at compile time, or the current working directory at runtime
    /// as a fallback.
    ///
    /// The campaign is initialized as an empty campaign with 0 gold.
    /// Call [`GameState::new_campaign`] or [`GameState::load_campaign`]
    /// to set up the campaign state.
    pub fn load() -> Result<Self, String> {
        let data_dir = Self::default_data_dir()?;
        Self::load_from(&data_dir)
    }

    /// Load game state from a specific data directory.
    ///
    /// Loads all contract packages (Curios.csv, Traps.json, Obstacles.json,
    /// Buildings.json, Quests.json, Trinkets.json, JsonQuirks.json,
    /// JsonTraits.json, JsonCamping.json) and initializes the campaign as
    /// an empty campaign with 0 gold.
    ///
    /// The host_phase is set to `HostPhase::Uninitialized` after loading.
    /// Use [`GameState::new_campaign`] or [`GameState::load_campaign`]
    /// to set up the campaign state.
    pub fn load_from(data_dir: &Path) -> Result<Self, String> {
        if !data_dir.exists() {
            return Err(format!("data directory not found: {}", data_dir.display()));
        }

        // Load Curios.csv
        let curios_path = data_dir.join("Curios.csv");
        let curio_registry = if curios_path.exists() {
            parse_curios_csv(&curios_path).map_err(|e| format!("Curios.csv: {}", e))?
        } else {
            CurioRegistry::new()
        };

        // Load Traps.json
        let traps_path = data_dir.join("Traps.json");
        let trap_registry = if traps_path.exists() {
            parse_traps_json(&traps_path).map_err(|e| format!("Traps.json: {}", e))?
        } else {
            TrapRegistry::new()
        };

        // Load Obstacles.json
        let obstacles_path = data_dir.join("Obstacles.json");
        let obstacle_registry = if obstacles_path.exists() {
            parse_obstacles_json(&obstacles_path).map_err(|e| format!("Obstacles.json: {}", e))?
        } else {
            ObstacleRegistry::new()
        };

        // Load Buildings.json
        let buildings_path = data_dir.join("Buildings.json");
        let building_registry = if buildings_path.exists() {
            parse_buildings_json(&buildings_path).map_err(|e| format!("Buildings.json: {}", e))?
        } else {
            BuildingRegistry::new()
        };

        // Load JsonCamping.json (required)
        let camping_path = data_dir.join("JsonCamping.json");
        let camping_skills = if camping_path.exists() {
            parse_camping_json(&camping_path).map_err(|e| format!("JsonCamping.json: {}", e))?
        } else {
            return Err(format!("JsonCamping.json not found at {}", camping_path.display()));
        };

        // Load JsonQuirks.json
        let quirks_path = data_dir.join("JsonQuirks.json");
        let quirk_registry = if quirks_path.exists() {
            parse_quirks_json(&quirks_path).map_err(|e| format!("JsonQuirks.json: {}", e))?
        } else {
            QuirkRegistry::new()
        };

        // Load JsonTraits.json
        let traits_path = data_dir.join("JsonTraits.json");
        let trait_registry = if traits_path.exists() {
            parse_traits_json(&traits_path).map_err(|e| format!("JsonTraits.json: {}", e))?
        } else {
            TraitRegistry::new()
        };

        // Initialize empty registries for types not loaded from files
        let quest_registry = QuestRegistry::new();
        let trinket_registry = TrinketRegistry::new();
        let dungeon_encounter_registry = DungeonEncounterRegistry::new();
        let equipment_registry = EquipmentRegistry::new();
        let buff_registry = BuffRegistry::new();

        Ok(GameState {
            camping_skills,
            campaign: CampaignState::new(0),
            data_dir: data_dir.to_path_buf(),
            curio_registry,
            trap_registry,
            obstacle_registry,
            building_registry,
            quest_registry,
            trinket_registry,
            quirk_registry,
            trait_registry,
            dungeon_encounter_registry,
            equipment_registry,
            buff_registry,
            host_phase: HostPhase::Uninitialized,
        })
    }

    /// Determine the default data directory.
    ///
    /// Uses `CARGO_MANIFEST_DIR` (set by cargo during build) to locate the
    /// project root. Falls back to the current working directory if the env
    /// var is not set.
    fn default_data_dir() -> Result<PathBuf, String> {
        if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
            Ok(PathBuf::from(manifest_dir).join("data"))
        } else {
            // Fallback: try current working directory
            let cwd = std::env::current_dir()
                .map_err(|e| format!("cannot determine current directory: {}", e))?;
            let data_dir = cwd.join("data");
            if data_dir.exists() {
                Ok(data_dir)
            } else {
                Err("CARGO_MANIFEST_DIR not set and data/ not found in CWD".to_string())
            }
        }
    }

    // ───────────────────────────────────────────────────────────────
    // Camping skill accessors
    // ───────────────────────────────────────────────────────────────

    /// Get the total number of camping skills loaded.
    pub fn camping_skill_count(&self) -> usize {
        self.camping_skills.len()
    }

    /// Get a camping skill by ID.
    pub fn camping_skill(&self, id: &str) -> Option<&crate::contracts::CampingSkill> {
        self.camping_skills.get(id)
    }

    /// Validate all camping skills in the loaded state.
    ///
    /// Returns Ok if all 87 skills pass the runtime schema validation.
    pub fn validate_camping_skills(&self) -> Result<(), Vec<String>> {
        self.camping_skills.validate()
    }

    /// Get all camping skills available to a specific hero class.
    pub fn camping_skills_for_class(&self, class_id: &str) -> Vec<&crate::contracts::CampingSkill> {
        self.camping_skills.for_class(class_id)
    }

    /// Get all generic camping skills (available to all classes).
    pub fn generic_camping_skills(&self) -> Vec<&crate::contracts::CampingSkill> {
        self.camping_skills.generic_skills()
    }

    /// Get all class-specific camping skills.
    pub fn class_specific_camping_skills(&self) -> Vec<&crate::contracts::CampingSkill> {
        self.camping_skills.class_specific_skills()
    }

    // ───────────────────────────────────────────────────────────────
    // Registry accessors
    // ───────────────────────────────────────────────────────────────

    /// Get the total number of curios loaded.
    pub fn curio_count(&self) -> usize {
        self.curio_registry.len()
    }

    /// Get a curio by ID.
    pub fn curio(&self, id: &str) -> Option<&crate::contracts::CurioDefinition> {
        self.curio_registry.get(id)
    }

    /// Get the total number of traps loaded.
    pub fn trap_count(&self) -> usize {
        self.trap_registry.len()
    }

    /// Get a trap by ID.
    pub fn trap(&self, id: &str) -> Option<&crate::contracts::TrapDefinition> {
        self.trap_registry.get(id)
    }

    /// Get the total number of obstacles loaded.
    pub fn obstacle_count(&self) -> usize {
        self.obstacle_registry.len()
    }

    /// Get an obstacle by ID.
    pub fn obstacle(&self, id: &str) -> Option<&crate::contracts::ObstacleDefinition> {
        self.obstacle_registry.get(id)
    }

    /// Get the total number of buildings loaded.
    pub fn building_count(&self) -> usize {
        self.building_registry.len()
    }

    /// Get a building by ID.
    pub fn building(&self, id: &str) -> Option<&crate::contracts::TownBuilding> {
        self.building_registry.get(id)
    }

    /// Get the total number of quirks loaded.
    pub fn quirk_count(&self) -> usize {
        self.quirk_registry.len()
    }

    /// Get a quirk by ID.
    pub fn quirk(&self, id: &str) -> Option<&crate::contracts::QuirkDefinition> {
        self.quirk_registry.get(id)
    }

    /// Get the total number of traits loaded.
    pub fn trait_count(&self) -> usize {
        self.trait_registry.len()
    }

    /// Get a trait by ID.
    pub fn trait_def(&self, id: &str) -> Option<&crate::contracts::TraitDefinition> {
        self.trait_registry.get(id)
    }

    /// Get the total number of trinkets loaded.
    pub fn trinket_count(&self) -> usize {
        self.trinket_registry.len()
    }

    /// Get a trinket by ID.
    pub fn trinket(&self, id: &str) -> Option<&crate::contracts::TrinketDefinition> {
        self.trinket_registry.get(id)
    }

    /// Get the total number of quests loaded.
    pub fn quest_count(&self) -> usize {
        self.quest_registry.len()
    }

    /// Get a quest by ID.
    pub fn quest(&self, id: &str) -> Option<&crate::contracts::QuestDefinition> {
        self.quest_registry.get(id)
    }

    /// Get the total number of dungeon encounters loaded.
    pub fn dungeon_encounter_count(&self) -> usize {
        self.dungeon_encounter_registry.configs().len()
    }

    /// Get the total number of equipment items loaded.
    pub fn equipment_count(&self) -> usize {
        self.equipment_registry.len()
    }

    /// Get the number of buff overrides registered.
    pub fn buff_override_count(&self) -> usize {
        // BuffRegistry stores overrides in a private HashMap;
        // we can only expose the count via resolve_buff usage
        // For actual buff resolution, use resolve_buff or resolve_buffs
        0
    }

    /// Resolve a buff ID to attribute modifiers.
    pub fn resolve_buff(&self, buff_id: &str) -> Vec<crate::contracts::AttributeModifier> {
        self.buff_registry.resolve_buff(buff_id)
    }

    /// Check if the host is in the Ready phase.
    pub fn is_ready(&self) -> bool {
        self.host_phase == HostPhase::Ready
    }

    /// Get the current host phase.
    pub fn host_phase(&self) -> &HostPhase {
        &self.host_phase
    }

    /// Transition to a new host phase.
    pub fn set_host_phase(&mut self, phase: HostPhase) {
        self.host_phase = phase;
    }

    // ───────────────────────────────────────────────────────────────
    // Campaign persistence — canonical save/load boundary
    // ───────────────────────────────────────────────────────────────

    /// Start a fresh campaign with the given starting gold.
    ///
    /// Replaces the current campaign with a new empty campaign.
    /// The schema version is set to [`CAMPAIGN_SNAPSHOT_VERSION`].
    /// Content datasets (camping skills, etc.) are preserved.
    pub fn new_campaign(&mut self, starting_gold: u32) {
        self.campaign = CampaignState::new(starting_gold);
    }

    /// Save the current campaign state to a JSON file.
    ///
    /// Delegates to [`CampaignState::save_to_file`] which serializes the full
    /// campaign snapshot using the Phase 7 schema. Serialization is deterministic:
    /// the same campaign state always produces identical JSON bytes.
    ///
    /// The in-memory campaign state is never modified by this operation.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization or file I/O fails. Errors are surfaced
    /// explicitly and do not modify the in-memory state.
    pub fn save_campaign(&self, path: &Path) -> Result<(), String> {
        self.campaign.save_to_file(path)
    }

    /// Load a campaign state from a JSON save file.
    ///
    /// Delegates to [`CampaignState::load_from_file`] which reads the file,
    /// deserializes the full campaign snapshot, and validates the schema version.
    /// On success, the current campaign is replaced with the loaded state.
    /// Content datasets (camping skills, etc.) are preserved.
    ///
    /// If loading fails for any reason, the in-memory campaign state is left
    /// unchanged — errors do not silently reset progress.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read, the JSON is malformed,
    /// or the schema version is unsupported. The existing campaign state
    /// is preserved on error.
    pub fn load_campaign(&mut self, path: &Path) -> Result<(), String> {
        self.campaign = CampaignState::load_from_file(path)?;
        Ok(())
    }

    /// Validate the current campaign's schema version.
    ///
    /// Returns `Ok(())` if the version matches [`CAMPAIGN_SNAPSHOT_VERSION`].
    pub fn validate_campaign(&self) -> Result<(), String> {
        self.campaign.validate_version()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Quest Runtime State
// ─────────────────────────────────────────────────────────────────────────────

/// Quest difficulty level, corresponding to dungeon level constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuestDifficulty {
    /// Standard difficulty (dungeon level 3).
    Standard,
    /// Hard difficulty (dungeon level 5).
    Hard,
}

impl QuestDifficulty {
    /// Returns the dungeon level number for this difficulty.
    pub fn dungeon_level(&self) -> u32 {
        match self {
            QuestDifficulty::Standard => 3,
            QuestDifficulty::Hard => 5,
        }
    }
}

/// Objective type for a quest.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QuestObjective {
    /// Clear all rooms in the dungeon.
    ClearDungeon,
    /// Kill a specific boss.
    KillBoss,
    /// Cleanse all corruption in the dungeon.
    CleanseCorruption,
    /// Gather items from the dungeon.
    GatherItems,
    /// Activate a specific mechanism.
    ActivateMechanism,
    /// Use an inventory item to complete an objective.
    UseInventoryItem,
}

impl QuestObjective {
    /// Classify a QuestType into supported or unsupported.
    ///
    /// Returns `None` for unsupported quest types.
    pub fn from_quest_type(quest_type: crate::contracts::QuestType) -> Option<QuestObjective> {
        match quest_type {
            crate::contracts::QuestType::Explore => Some(QuestObjective::ClearDungeon),
            crate::contracts::QuestType::KillBoss => Some(QuestObjective::KillBoss),
            crate::contracts::QuestType::Cleanse => Some(QuestObjective::CleanseCorruption),
            crate::contracts::QuestType::Gather => Some(QuestObjective::GatherItems),
            crate::contracts::QuestType::Activate => Some(QuestObjective::ActivateMechanism),
            crate::contracts::QuestType::InventoryActivate => Some(QuestObjective::UseInventoryItem),
        }
    }

    /// Returns true if this objective type is supported.
    pub fn is_supported(&self) -> bool {
        true
    }
}

/// Rewards granted upon quest completion.
#[derive(Debug, Clone, PartialEq)]
pub struct QuestRewards {
    /// Gold awarded on completion.
    pub gold: u32,
    /// Heirloom currencies awarded on completion.
    pub heirlooms: std::collections::BTreeMap<crate::contracts::HeirloomCurrency, u32>,
    /// XP granted to heroes on completion.
    pub xp: u32,
}

impl QuestRewards {
    /// Create rewards for a standard difficulty quest.
    pub fn standard() -> Self {
        use crate::contracts::HeirloomCurrency;
        let mut heirlooms = std::collections::BTreeMap::new();
        heirlooms.insert(HeirloomCurrency::Bones, 10);
        heirlooms.insert(HeirloomCurrency::Portraits, 5);
        QuestRewards {
            gold: 500,
            heirlooms,
            xp: 200,
        }
    }

    /// Create rewards for a hard difficulty quest.
    pub fn hard() -> Self {
        use crate::contracts::HeirloomCurrency;
        let mut heirlooms = std::collections::BTreeMap::new();
        heirlooms.insert(HeirloomCurrency::Bones, 25);
        heirlooms.insert(HeirloomCurrency::Portraits, 15);
        heirlooms.insert(HeirloomCurrency::Tapes, 5);
        QuestRewards {
            gold: 1000,
            heirlooms,
            xp: 400,
        }
    }
}

/// Penalties applied upon quest failure.
#[derive(Debug, Clone, PartialEq)]
pub struct QuestPenalties {
    /// Gold lost on failure.
    pub gold: i32,
    /// Heirloom currencies lost on failure.
    pub heirlooms: std::collections::BTreeMap<crate::contracts::HeirloomCurrency, i32>,
}

impl QuestPenalties {
    /// Create penalties for a standard difficulty quest.
    pub fn standard() -> Self {
        use crate::contracts::HeirloomCurrency;
        let mut heirlooms = std::collections::BTreeMap::new();
        heirlooms.insert(HeirloomCurrency::Bones, -5);
        heirlooms.insert(HeirloomCurrency::Portraits, -2);
        QuestPenalties {
            gold: -100,
            heirlooms,
        }
    }

    /// Create penalties for a hard difficulty quest.
    pub fn hard() -> Self {
        use crate::contracts::HeirloomCurrency;
        let mut heirlooms = std::collections::BTreeMap::new();
        heirlooms.insert(HeirloomCurrency::Bones, -15);
        heirlooms.insert(HeirloomCurrency::Portraits, -10);
        heirlooms.insert(HeirloomCurrency::Tapes, -3);
        QuestPenalties {
            gold: -250,
            heirlooms,
        }
    }
}

/// Quest runtime state — tracks all runtime information for an active quest.
///
/// This struct holds the complete runtime state for a quest that is distinct
/// from the persisted `CampaignQuestProgress` in the save/load boundary.
/// It captures: identity, difficulty, dungeon, objective, progress counters,
/// completion/failure status, and reward definitions.
#[derive(Debug, Clone, PartialEq)]
pub struct QuestState {
    /// Unique quest identifier.
    pub quest_id: String,
    /// Difficulty level of the quest.
    pub difficulty: QuestDifficulty,
    /// Dungeon type for this quest.
    pub dungeon: crate::contracts::DungeonType,
    /// Map size for this quest.
    pub map_size: crate::contracts::MapSize,
    /// Objective type and description.
    pub objective: QuestObjective,
    /// Current progress step.
    pub current_step: u32,
    /// Total number of steps required.
    pub max_steps: u32,
    /// Whether the quest has been completed.
    pub completed: bool,
    /// Whether the quest has failed.
    pub failed: bool,
    /// Rewards granted on completion.
    pub rewards: QuestRewards,
    /// Penalties applied on failure.
    pub penalties: QuestPenalties,
}

impl QuestState {
    /// Create a new quest state for a KillBoss quest (the representative test quest).
    ///
    /// This creates a standard difficulty KillBoss quest for QingLong dungeon.
    pub fn new_kill_boss_quest(quest_id: &str) -> Self {
        QuestState {
            quest_id: quest_id.to_string(),
            difficulty: QuestDifficulty::Standard,
            dungeon: crate::contracts::DungeonType::QingLong,
            map_size: crate::contracts::MapSize::Short,
            objective: QuestObjective::KillBoss,
            current_step: 0,
            max_steps: 2,
            completed: false,
            failed: false,
            rewards: QuestRewards::standard(),
            penalties: QuestPenalties::standard(),
        }
    }

    /// Create a new quest state with explicit parameters.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        quest_id: &str,
        difficulty: QuestDifficulty,
        dungeon: crate::contracts::DungeonType,
        map_size: crate::contracts::MapSize,
        objective: QuestObjective,
        max_steps: u32,
        rewards: QuestRewards,
        penalties: QuestPenalties,
    ) -> Self {
        QuestState {
            quest_id: quest_id.to_string(),
            difficulty,
            dungeon,
            map_size,
            objective,
            current_step: 0,
            max_steps,
            completed: false,
            failed: false,
            rewards,
            penalties,
        }
    }

    /// Check if the quest is still active (not completed or failed).
    pub fn is_active(&self) -> bool {
        !self.completed && !self.failed
    }

    /// Advance the quest by one step.
    ///
    /// Returns the new current step value.
    pub fn advance_progress(&mut self) -> u32 {
        if self.current_step < self.max_steps {
            self.current_step += 1;
        }
        self.current_step
    }

    /// Check if the quest can be completed (all steps done).
    pub fn can_complete(&self) -> bool {
        self.current_step >= self.max_steps && !self.failed
    }

    /// Complete the quest.
    ///
    /// Marks the quest as completed and returns the rewards.
    /// Returns None if the quest is already completed or failed.
    pub fn complete(&mut self) -> Option<QuestRewards> {
        if !self.is_active() {
            return None;
        }
        self.completed = true;
        Some(self.rewards.clone())
    }

    /// Fail the quest.
    ///
    /// Marks the quest as failed and returns the penalties.
    /// Returns None if the quest is already completed or failed.
    pub fn fail(&mut self) -> Option<QuestPenalties> {
        if !self.is_active() {
            return None;
        }
        self.failed = true;
        Some(self.penalties.clone())
    }

    /// Apply rewards to the campaign state.
    ///
    /// Adds gold and heirlooms to the campaign, and XP to roster heroes.
    pub fn apply_rewards_to_campaign(
        &self,
        rewards: &QuestRewards,
        campaign: &mut crate::contracts::CampaignState,
    ) {
        // Apply gold
        campaign.gold = campaign.gold.saturating_add(rewards.gold);

        // Apply heirlooms
        for (currency, amount) in &rewards.heirlooms {
            *campaign.heirlooms.entry(currency.clone()).or_insert(0) += *amount;
        }

        // Apply XP to roster (distribute evenly among heroes)
        if !campaign.roster.is_empty() && rewards.xp > 0 {
            let xp_per_hero = rewards.xp / campaign.roster.len() as u32;
            for hero in &mut campaign.roster {
                hero.xp = hero.xp.saturating_add(xp_per_hero);
            }
        }
    }

    /// Apply penalties to the campaign state.
    ///
    /// Subtracts gold and heirlooms from the campaign.
    pub fn apply_penalties_to_campaign(
        &self,
        penalties: &QuestPenalties,
        campaign: &mut crate::contracts::CampaignState,
    ) {
        // Apply gold penalty (can't go below 0)
        campaign.gold = campaign.gold.saturating_sub(penalties.gold.unsigned_abs());

        // Apply heirloom penalties (can't go below 0)
        for (currency, amount) in &penalties.heirlooms {
            let current = campaign.heirlooms.entry(currency.clone()).or_insert(0);
            *current = current.saturating_sub(amount.unsigned_abs());
        }
    }

    /// Update quest progress based on a dungeon run event.
    ///
    /// This is called after a dungeon run completes to update quest progress.
    /// Returns the updated step count if the quest was updated, or None if not applicable.
    pub fn update_from_run(
        &mut self,
        dungeon: crate::contracts::DungeonType,
        map_size: crate::contracts::MapSize,
        rooms_cleared: u32,
        battles_won: u32,
        completed: bool,
    ) -> Option<u32> {
        // Only update if this quest is for the given dungeon/map_size
        if self.dungeon != dungeon || self.map_size != map_size {
            return None;
        }

        if !self.is_active() {
            return None;
        }

        // Update progress based on objective type
        match self.objective {
            QuestObjective::ClearDungeon => {
                // Progress for clearing rooms
                if rooms_cleared > 0 {
                    self.advance_progress();
                }
            }
            QuestObjective::KillBoss => {
                // Progress for winning battles
                if battles_won > 0 {
                    self.advance_progress();
                }
            }
            QuestObjective::CleanseCorruption | QuestObjective::GatherItems
            | QuestObjective::ActivateMechanism | QuestObjective::UseInventoryItem => {
                // These objectives require specific items/events
                // For now, advance on room clears
                if rooms_cleared > 0 {
                    self.advance_progress();
                }
            }
        }

        // If the run completed and this is a completion-type objective, mark step done
        if completed && self.current_step < self.max_steps {
            self.current_step = self.max_steps;
        }

        Some(self.current_step)
    }

    /// Convert to a CampaignQuestProgress for persistence.
    pub fn to_campaign_quest_progress(&self) -> crate::contracts::CampaignQuestProgress {
        crate::contracts::CampaignQuestProgress {
            quest_id: self.quest_id.clone(),
            current_step: self.current_step,
            max_steps: self.max_steps,
            completed: self.completed,
        }
    }
}

/// Classification result for an unsupported quest type.
#[derive(Debug, Clone, PartialEq)]
pub struct UnsupportedQuestTrace {
    /// The quest type that is not supported.
    pub quest_type: crate::contracts::QuestType,
    /// Human-readable reason why the quest type is not supported.
    pub reason: &'static str,
}

impl UnsupportedQuestTrace {
    /// Create a new unsupported quest trace.
    pub fn new(quest_type: crate::contracts::QuestType) -> Self {
        let reason = match quest_type {
            crate::contracts::QuestType::Explore => {
                "Explore quests require full dungeon clearing which is not yet implemented"
            }
            crate::contracts::QuestType::KillBoss => {
                "KillBoss quests are fully supported"
            }
            crate::contracts::QuestType::Cleanse => {
                "Cleanse quests require corruption tracking which is not yet implemented"
            }
            crate::contracts::QuestType::Gather => {
                "Gather quests require item collection tracking which is not yet implemented"
            }
            crate::contracts::QuestType::Activate => {
                "Activate quests require mechanism activation which is not yet implemented"
            }
            crate::contracts::QuestType::InventoryActivate => {
                "InventoryActivate quests require inventory item usage which is not yet implemented"
            }
        };

        UnsupportedQuestTrace {
            quest_type,
            reason,
        }
    }

    /// Check if this trace indicates a supported quest type.
    pub fn is_supported(&self) -> bool {
        matches!(self.quest_type, crate::contracts::QuestType::KillBoss)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{
        BuildingUpgradeState, CampaignHero, CampaignInventoryItem, CampaignQuestProgress,
        CampaignRunRecord, CampEffectType, DungeonType, HeirloomCurrency, MapSize,
        CAMPAIGN_SNAPSHOT_VERSION,
    };

    /// Helper: resolve the data directory for tests.
    ///
    /// Uses `CARGO_MANIFEST_DIR` at test time (set by cargo test).
    fn test_data_dir() -> PathBuf {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR must be set during cargo test");
        PathBuf::from(manifest_dir).join("data")
    }

    /// Helper: load the real camping skill registry for tests.
    fn load_real_state() -> GameState {
        let data_dir = test_data_dir();
        GameState::load_from(&data_dir).expect("failed to load game state from data dir")
    }

    // ───────────────────────────────────────────────────────────────
    // State loading tests
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn state_loads_all_87_camping_skills() {
        let state = load_real_state();
        assert_eq!(state.camping_skill_count(), 87);
    }

    #[test]
    fn state_loads_from_default_data_dir() {
        let state = GameState::load();
        assert!(
            state.is_ok(),
            "GameState::load() should succeed from default data dir"
        );
        let state = state.unwrap();
        assert_eq!(state.camping_skill_count(), 87);
    }

    #[test]
    fn state_fails_when_json_camping_missing() {
        let result = GameState::load_from(Path::new("/nonexistent/path"));
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("not found"));
    }

    #[test]
    fn state_preserves_data_dir() {
        let state = load_real_state();
        assert!(state.data_dir.exists());
        assert!(state.data_dir.join("JsonCamping.json").exists());
    }

    // ───────────────────────────────────────────────────────────────
    // Full registry validation tests
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn full_camping_registry_validates_against_runtime_schema() {
        let state = load_real_state();
        let result = state.validate_camping_skills();
        assert!(
            result.is_ok(),
            "camping skill validation failed: {:?}",
            result.err()
        );
    }

    #[test]
    fn every_individual_camping_skill_passes_validation() {
        let state = load_real_state();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).expect("skill should exist");
            let errors = skill.validate();
            assert!(
                errors.is_empty(),
                "skill '{}' failed validation: {:?}",
                skill_id,
                errors
            );
        }
    }

    // ───────────────────────────────────────────────────────────────
    // Content integrity tests
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn all_skills_have_positive_time_cost() {
        let state = load_real_state();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).unwrap();
            assert!(
                skill.time_cost > 0,
                "skill '{}' has zero time_cost",
                skill_id
            );
        }
    }

    #[test]
    fn all_skills_have_positive_use_limit() {
        let state = load_real_state();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).unwrap();
            assert!(
                skill.use_limit > 0,
                "skill '{}' has zero use_limit",
                skill_id
            );
        }
    }

    #[test]
    fn all_skills_have_at_least_one_effect() {
        let state = load_real_state();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).unwrap();
            assert!(
                !skill.effects.is_empty(),
                "skill '{}' has no effects",
                skill_id
            );
        }
    }

    #[test]
    fn all_effects_have_valid_type() {
        let state = load_real_state();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).unwrap();
            for (i, effect) in skill.effects.iter().enumerate() {
                assert_ne!(
                    effect.effect_type,
                    CampEffectType::None,
                    "skill '{}' effect {} has None type",
                    skill_id,
                    i
                );
            }
        }
    }

    #[test]
    fn all_effects_have_valid_chance() {
        let state = load_real_state();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).unwrap();
            for (i, effect) in skill.effects.iter().enumerate() {
                assert!(
                    effect.chance >= 0.0 && effect.chance <= 1.0,
                    "skill '{}' effect {} has invalid chance {}",
                    skill_id,
                    i,
                    effect.chance
                );
            }
        }
    }

    // ───────────────────────────────────────────────────────────────
    // Class filtering tests
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn exactly_one_generic_skill() {
        let state = load_real_state();
        let generic = state.generic_camping_skills();
        assert_eq!(generic.len(), 1);
        assert_eq!(generic[0].id, "hobby");
    }

    #[test]
    fn generic_skill_hobby_preserves_source_data() {
        let state = load_real_state();
        let skill = state.camping_skill("hobby").unwrap();
        assert_eq!(skill.time_cost, 2);
        assert_eq!(skill.use_limit, 1);
        assert!(skill.classes.is_empty());
        assert!(skill.is_generic());
        assert!(!skill.has_individual_target);
        assert_eq!(skill.effects.len(), 1);
        assert_eq!(skill.effects[0].effect_type, CampEffectType::StressHealAmount);
        assert!((skill.effects[0].amount - 12.0).abs() < f64::EPSILON);
        assert!((skill.effects[0].chance - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn class_specific_count_is_86() {
        let state = load_real_state();
        let specific = state.class_specific_camping_skills();
        assert_eq!(specific.len(), 86);
    }

    #[test]
    fn encourage_preserves_source_data() {
        let state = load_real_state();
        let skill = state.camping_skill("encourage").unwrap();
        assert_eq!(skill.time_cost, 2);
        assert_eq!(skill.use_limit, 1);
        assert!(skill.has_individual_target);
        assert_eq!(skill.effects.len(), 1);
        assert_eq!(skill.effects[0].effect_type, CampEffectType::StressHealAmount);
        assert!((skill.effects[0].amount - 15.0).abs() < f64::EPSILON);
        assert_eq!(skill.classes.len(), 16);
    }

    #[test]
    fn field_dressing_preserves_source_data() {
        let state = load_real_state();
        let skill = state.camping_skill("field_dressing").unwrap();
        assert_eq!(skill.time_cost, 3);
        assert_eq!(skill.use_limit, 1);
        assert!(skill.has_individual_target);
        assert_eq!(skill.classes, vec!["arbalest", "musketeer"]);
        assert_eq!(skill.effects.len(), 3);

        // Effect 0: 35% heal, 75% chance
        assert_eq!(skill.effects[0].effect_type, CampEffectType::HealthHealMaxHealthPercent);
        assert!((skill.effects[0].amount - 0.35).abs() < f64::EPSILON);
        assert!((skill.effects[0].chance - 0.75).abs() < f64::EPSILON);

        // Effect 1: 50% heal, 25% chance
        assert_eq!(skill.effects[1].effect_type, CampEffectType::HealthHealMaxHealthPercent);
        assert!((skill.effects[1].amount - 0.50).abs() < f64::EPSILON);
        assert!((skill.effects[1].chance - 0.25).abs() < f64::EPSILON);

        // Effect 2: remove bleed, 100% chance
        assert_eq!(skill.effects[2].effect_type, CampEffectType::RemoveBleed);
        assert!((skill.effects[2].chance - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn supply_has_use_limit_3() {
        let state = load_real_state();
        let skill = state.camping_skill("supply").unwrap();
        assert_eq!(skill.use_limit, 3);
        assert_eq!(skill.time_cost, 1);
        assert_eq!(skill.classes, vec!["antiquarian"]);
        assert_eq!(skill.effects.len(), 1);
        assert_eq!(skill.effects[0].effect_type, CampEffectType::Loot);
    }

    #[test]
    fn dark_ritual_preserves_reduce_torch_effect() {
        let state = load_real_state();
        let skill = state.camping_skill("dark_ritual").unwrap();
        assert_eq!(skill.time_cost, 4);
        assert_eq!(skill.use_limit, 1);
        assert_eq!(skill.classes, vec!["occultist"]);
        assert_eq!(skill.effects.len(), 4);
        let torch_effect = skill
            .effects
            .iter()
            .find(|e| e.effect_type == CampEffectType::ReduceTorch)
            .expect("dark_ritual should have a reduce_torch effect");
        assert!((torch_effect.amount - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn zealous_speech_has_highest_time_cost_5() {
        let state = load_real_state();
        let skill = state.camping_skill("zealous_speech").unwrap();
        assert_eq!(skill.time_cost, 5);
        assert_eq!(skill.use_limit, 1);
        assert_eq!(skill.classes, vec!["crusader"]);
    }

    #[test]
    fn self_medicate_has_five_effects() {
        let state = load_real_state();
        let skill = state.camping_skill("self_medicate").unwrap();
        assert_eq!(skill.time_cost, 3);
        assert_eq!(skill.classes, vec!["plague_doctor"]);
        assert_eq!(skill.effects.len(), 5);
        let types: Vec<_> = skill.effects.iter().map(|e| &e.effect_type).collect();
        assert!(types.contains(&&CampEffectType::StressHealAmount));
        assert!(types.contains(&&CampEffectType::HealthHealMaxHealthPercent));
        assert!(types.contains(&&CampEffectType::RemovePoison));
        assert!(types.contains(&&CampEffectType::RemoveBleed));
        assert!(types.contains(&&CampEffectType::Buff));
    }

    #[test]
    fn first_aid_heals_and_cleanses() {
        let state = load_real_state();
        let skill = state.camping_skill("first_aid").unwrap();
        assert_eq!(skill.time_cost, 2);
        assert_eq!(skill.effects.len(), 3);
        let types: Vec<_> = skill.effects.iter().map(|e| &e.effect_type).collect();
        assert!(types.contains(&&CampEffectType::HealthHealMaxHealthPercent));
        assert!(types.contains(&&CampEffectType::RemoveBleed));
        assert!(types.contains(&&CampEffectType::RemovePoison));
    }

    // ───────────────────────────────────────────────────────────────
    // Effect type coverage test
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn effect_type_coverage_matches_source() {
        let state = load_real_state();
        use std::collections::HashSet;
        let mut types = HashSet::new();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).unwrap();
            for effect in &skill.effects {
                types.insert(effect.effect_type.clone());
            }
        }
        // All 19 effect types used in JsonCamping.json should be present
        assert!(types.contains(&CampEffectType::StressHealAmount));
        assert!(types.contains(&CampEffectType::HealthHealMaxHealthPercent));
        assert!(types.contains(&CampEffectType::RemoveBleed));
        assert!(types.contains(&CampEffectType::RemovePoison));
        assert!(types.contains(&CampEffectType::Buff));
        assert!(types.contains(&CampEffectType::RemoveDeathRecovery));
        assert!(types.contains(&CampEffectType::ReduceAmbushChance));
        assert!(types.contains(&CampEffectType::RemoveDisease));
        assert!(types.contains(&CampEffectType::StressDamageAmount));
        assert!(types.contains(&CampEffectType::Loot));
        assert!(types.contains(&CampEffectType::ReduceTorch));
        assert!(types.contains(&CampEffectType::HealthDamageMaxHealthPercent));
        assert!(types.contains(&CampEffectType::StressHealPercent));
        assert!(types.contains(&CampEffectType::RemoveDebuff));
        assert!(types.contains(&CampEffectType::RemoveAllDebuff));
        assert!(types.contains(&CampEffectType::HealthHealRange));
        assert!(types.contains(&CampEffectType::HealthHealAmount));
        assert!(types.contains(&CampEffectType::ReduceTurbulenceChance));
        assert!(types.contains(&CampEffectType::ReduceRiptideChance));
        assert_eq!(types.len(), 19);
    }

    // ───────────────────────────────────────────────────────────────
    // Distribution tests
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn time_cost_distribution_matches_source() {
        let state = load_real_state();
        let mut counts = std::collections::HashMap::new();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).unwrap();
            *counts.entry(skill.time_cost).or_insert(0) += 1;
        }
        assert_eq!(counts.get(&1).copied().unwrap_or(0), 5);
        assert_eq!(counts.get(&2).copied().unwrap_or(0), 20);
        assert_eq!(counts.get(&3).copied().unwrap_or(0), 35);
        assert_eq!(counts.get(&4).copied().unwrap_or(0), 26);
        assert_eq!(counts.get(&5).copied().unwrap_or(0), 1);
    }

    #[test]
    fn use_limit_distribution_matches_source() {
        let state = load_real_state();
        let mut counts = std::collections::HashMap::new();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).unwrap();
            *counts.entry(skill.use_limit).or_insert(0) += 1;
        }
        assert_eq!(counts.get(&1).copied().unwrap_or(0), 86);
        assert_eq!(counts.get(&3).copied().unwrap_or(0), 1);
    }

    // ───────────────────────────────────────────────────────────────
    // Class coverage tests
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn all_31_hero_classes_have_skills() {
        let state = load_real_state();
        let all_classes = [
            "bounty_hunter", "crusader", "vestal", "occultist", "hellion",
            "grave_robber", "highwayman", "plague_doctor", "jester", "leper",
            "arbalest", "man_at_arms", "houndmaster", "abomination", "antiquarian",
            "musketeer", "alchemist", "alchemist1", "alchemist2",
            "diviner", "diviner1", "diviner2",
            "hunter", "hunter1", "hunter2",
            "shaman", "shaman1", "shaman2",
            "tank", "tank1", "tank2",
        ];
        for class in &all_classes {
            let class_skills = state.camping_skills_for_class(class);
            assert!(
                !class_skills.is_empty(),
                "class '{}' should have at least one skill",
                class
            );
        }
    }

    #[test]
    fn class_filtering_includes_generic_skills() {
        let state = load_real_state();
        for class in &["alchemist", "crusader", "arbalest"] {
            let skills = state.camping_skills_for_class(class);
            assert!(
                skills.iter().any(|s| s.id == "hobby"),
                "class '{}' should have generic skill 'hobby'",
                class
            );
        }
    }

    #[test]
    fn class_filtering_excludes_other_class_specific_skills() {
        let state = load_real_state();
        // field_dressing is arbalest/musketeer only
        let crusader_skills = state.camping_skills_for_class("crusader");
        assert!(
            !crusader_skills.iter().any(|s| s.id == "field_dressing"),
            "crusader should NOT have field_dressing"
        );
        let arbalest_skills = state.camping_skills_for_class("arbalest");
        assert!(
            arbalest_skills.iter().any(|s| s.id == "field_dressing"),
            "arbalest should have field_dressing"
        );
    }

    // ───────────────────────────────────────────────────────────────
    // Registry technical tests
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn camping_skill_lookup_by_id() {
        let state = load_real_state();
        assert!(state.camping_skill("encourage").is_some());
        assert!(state.camping_skill("hobby").is_some());
        assert!(state.camping_skill("field_dressing").is_some());
        assert!(state.camping_skill("dark_ritual").is_some());
        assert!(state.camping_skill("supply").is_some());
        assert!(state.camping_skill("nonexistent_skill").is_none());
    }

    #[test]
    fn all_skills_have_upgrade_cost() {
        let state = load_real_state();
        for skill_id in state.camping_skills.all_ids() {
            let skill = state.camping_skill(skill_id).unwrap();
            assert!(
                skill.upgrade_cost > 0,
                "skill '{}' has zero upgrade_cost",
                skill_id
            );
        }
    }

    #[test]
    fn skill_with_individual_target_is_flagged() {
        let state = load_real_state();
        let field_dressing = state.camping_skill("field_dressing").unwrap();
        assert!(field_dressing.has_individual_target);
        let encourage = state.camping_skill("encourage").unwrap();
        assert!(encourage.has_individual_target);
        let hobby = state.camping_skill("hobby").unwrap();
        assert!(!hobby.has_individual_target);
    }

    #[test]
    fn registry_is_not_empty() {
        let state = load_real_state();
        assert!(!state.camping_skills.is_empty());
        assert_eq!(state.camping_skill_count(), 87);
    }

    // ───────────────────────────────────────────────────────────────
    // Campaign persistence tests — save/load boundary
    // ───────────────────────────────────────────────────────────────

    /// Build a non-trivial CampaignState with all substates populated.
    fn build_test_campaign() -> CampaignState {
        let mut state = CampaignState::new(1500);
        state.heirlooms.insert(HeirloomCurrency::Bones, 42);
        state.heirlooms.insert(HeirloomCurrency::Portraits, 15);
        state.heirlooms.insert(HeirloomCurrency::Tapes, 7);
        state.building_states.insert(
            "inn".to_string(),
            BuildingUpgradeState::new("inn", Some('b')),
        );
        state.building_states.insert(
            "blacksmith".to_string(),
            BuildingUpgradeState::new("blacksmith", Some('a')),
        );

        let mut hero = CampaignHero::new("hero_1", "alchemist", 3, 450, 85.0, 100.0, 25.0, 200.0);
        hero.quirks.positive = vec!["eagle_eye".to_string(), "tough".to_string()];
        hero.quirks.negative = vec!["kleptomaniac".to_string()];
        hero.quirks.diseases = vec!["consumption".to_string()];
        hero.traits.virtues = vec!["courageous".to_string()];
        hero.skills = vec![
            "skill_fire_bomb".to_string(),
            "skill_acid_spray".to_string(),
            "skill_healing_vapor".to_string(),
            "skill_toxin_grenade".to_string(),
        ];
        hero.equipment.weapon_level = 2;
        hero.equipment.armor_level = 1;
        hero.equipment.trinkets = vec!["sage_stone".to_string(), "lucky_charm".to_string()];
        state.roster.push(hero);

        let hero2 = CampaignHero::new("hero_2", "hunter", 2, 200, 100.0, 100.0, 10.0, 200.0);
        state.roster.push(hero2);

        state.inventory.push(CampaignInventoryItem::new("torch", 4));
        state.inventory.push(CampaignInventoryItem::new("shovel", 1));
        state.inventory.push(CampaignInventoryItem::new("bandage", 3));

        state.run_history.push(CampaignRunRecord::new(
            DungeonType::QingLong, MapSize::Short,
            9, 3, true, 350,
        ));
        state.run_history.push(CampaignRunRecord::new(
            DungeonType::BaiHu, MapSize::Medium,
            10, 2, false, 125,
        ));

        state.quest_progress.push({
            let mut q = CampaignQuestProgress::new("kill_boss_qinglong", 2);
            q.current_step = 1;
            q
        });

        state
    }

    /// Helper: create a temporary file path for save testing.
    fn temp_save_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("ddgc_test_campaign_{}.json", name))
    }

    // ── Save/load round-trip tests ──────────────────────────────────

    #[test]
    fn save_and_load_campaign_full_roundtrip() {
        let _ = load_real_state(); // ensure content loads
        let campaign = build_test_campaign();
        let save_path = temp_save_path("full_roundtrip");

        // Write via state layer
        let json = campaign.to_json().unwrap();
        std::fs::write(&save_path, &json).unwrap();

        // Read back
        let loaded_json = std::fs::read_to_string(&save_path).unwrap();
        let restored = CampaignState::from_json(&loaded_json).unwrap();
        std::fs::remove_file(&save_path).ok();

        assert_eq!(campaign, restored);
    }

    #[test]
    fn campaign_save_load_preserves_all_gameplay_fields() {
        let original = build_test_campaign();
        let save_path = temp_save_path("all_fields");
        let json = original.to_json().unwrap();
        std::fs::write(&save_path, &json).unwrap();

        let loaded_json = std::fs::read_to_string(&save_path).unwrap();
        let restored = CampaignState::from_json(&loaded_json).unwrap();
        std::fs::remove_file(&save_path).ok();

        // Gold
        assert_eq!(restored.gold, 1500);
        assert_eq!(restored.schema_version, CAMPAIGN_SNAPSHOT_VERSION);

        // Heirlooms
        assert_eq!(restored.heirlooms[&HeirloomCurrency::Bones], 42);
        assert_eq!(restored.heirlooms[&HeirloomCurrency::Portraits], 15);
        assert_eq!(restored.heirlooms[&HeirloomCurrency::Tapes], 7);

        // Buildings
        assert_eq!(restored.building_states["inn"].current_level, Some('b'));
        assert_eq!(restored.building_states["blacksmith"].current_level, Some('a'));

        // Roster
        assert_eq!(restored.roster.len(), 2);
        let h1 = &restored.roster[0];
        assert_eq!(h1.id, "hero_1");
        assert_eq!(h1.class_id, "alchemist");
        assert_eq!(h1.level, 3);
        assert_eq!(h1.xp, 450);
        assert_eq!(h1.health, 85.0);
        assert_eq!(h1.max_health, 100.0);
        assert_eq!(h1.stress, 25.0);
        assert_eq!(h1.max_stress, 200.0);
        assert_eq!(h1.quirks.positive.len(), 2);
        assert_eq!(h1.quirks.negative.len(), 1);
        assert_eq!(h1.quirks.diseases.len(), 1);
        assert_eq!(h1.traits.virtues.len(), 1);
        assert_eq!(h1.skills.len(), 4);
        assert_eq!(h1.equipment.weapon_level, 2);
        assert_eq!(h1.equipment.armor_level, 1);
        assert_eq!(h1.equipment.trinkets.len(), 2);

        // Inventory
        assert_eq!(restored.inventory.len(), 3);

        // Run history
        assert_eq!(restored.run_history.len(), 2);
        assert_eq!(restored.run_history[0].dungeon, DungeonType::QingLong);
        assert!(restored.run_history[0].completed);
        assert_eq!(restored.run_history[1].dungeon, DungeonType::BaiHu);
        assert!(!restored.run_history[1].completed);

        // Quests
        assert_eq!(restored.quest_progress.len(), 1);
        assert_eq!(restored.quest_progress[0].quest_id, "kill_boss_qinglong");
        assert_eq!(restored.quest_progress[0].current_step, 1);
    }

    #[test]
    fn campaign_serialization_is_deterministic() {
        let campaign = build_test_campaign();
        let json_a = campaign.to_json().unwrap();
        let json_b = campaign.to_json().unwrap();
        assert_eq!(json_a, json_b, "identical campaign state must produce identical JSON bytes");
    }

    #[test]
    fn deterministic_campaign_produces_identical_save_file() {
        let campaign = build_test_campaign();
        let path_a = temp_save_path("det_a");
        let path_b = temp_save_path("det_b");

        std::fs::write(&path_a, campaign.to_json().unwrap()).unwrap();
        std::fs::write(&path_b, campaign.to_json().unwrap()).unwrap();

        let bytes_a = std::fs::read(&path_a).unwrap();
        let bytes_b = std::fs::read(&path_b).unwrap();
        std::fs::remove_file(&path_a).ok();
        std::fs::remove_file(&path_b).ok();

        assert_eq!(bytes_a, bytes_b, "identical state must produce identical save files");
    }

    // ── Schema versioning tests ─────────────────────────────────────

    #[test]
    fn new_campaign_has_current_schema_version() {
        let campaign = CampaignState::new(500);
        assert_eq!(campaign.schema_version, CAMPAIGN_SNAPSHOT_VERSION);
        assert!(campaign.validate_version().is_ok());
    }

    #[test]
    fn load_rejects_unsupported_schema_version() {
        let mut campaign = CampaignState::new(500);
        campaign.schema_version = 99;
        let save_path = temp_save_path("bad_version");
        std::fs::write(&save_path, campaign.to_json().unwrap()).unwrap();

        let loaded_json = std::fs::read_to_string(&save_path).unwrap();
        let loaded = CampaignState::from_json(&loaded_json).unwrap();
        std::fs::remove_file(&save_path).ok();

        let result = loaded.validate_version();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("unsupported campaign schema version"));
    }

    #[test]
    fn state_layer_validate_campaign_delegates_to_schema_version() {
        let mut state = load_real_state();
        state.campaign = CampaignState::new(100);
        assert!(state.validate_campaign().is_ok());

        state.campaign.schema_version = 42;
        assert!(state.validate_campaign().is_err());
    }

    // ── Empty/fresh campaign tests ──────────────────────────────────

    #[test]
    fn empty_campaign_save_load_roundtrip() {
        let campaign = CampaignState::new(0);
        let json = campaign.to_json().unwrap();
        let restored = CampaignState::from_json(&json).unwrap();
        assert_eq!(restored.gold, 0);
        assert_eq!(restored.schema_version, CAMPAIGN_SNAPSHOT_VERSION);
        assert!(restored.roster.is_empty());
        assert!(restored.heirlooms.is_empty());
        assert!(restored.building_states.is_empty());
        assert!(restored.inventory.is_empty());
        assert!(restored.run_history.is_empty());
        assert!(restored.quest_progress.is_empty());
    }

    #[test]
    fn fresh_campaign_initializes_all_collections_empty() {
        let campaign = CampaignState::new(250);
        assert_eq!(campaign.gold, 250);
        assert!(campaign.heirlooms.is_empty());
        assert!(campaign.building_states.is_empty());
        assert!(campaign.roster.is_empty());
        assert!(campaign.inventory.is_empty());
        assert!(campaign.run_history.is_empty());
        assert!(campaign.quest_progress.is_empty());
    }

    #[test]
    fn state_new_campaign_preserves_content_datasets() {
        let mut state = load_real_state();
        assert_eq!(state.camping_skill_count(), 87);

        state.new_campaign(300);
        assert_eq!(state.campaign.gold, 300);
        assert_eq!(state.campaign.schema_version, CAMPAIGN_SNAPSHOT_VERSION);
        assert_eq!(state.camping_skill_count(), 87); // content preserved
    }

    // ── Gameplay-significant field tests ────────────────────────────

    #[test]
    fn hero_quirks_roundtrip_preserves_categories() {
        let mut hero = CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0);
        hero.quirks.positive = vec!["eagle_eye".to_string()];
        hero.quirks.negative = vec!["fearful".to_string()];
        hero.quirks.diseases = vec!["rabies".to_string()];

        let mut state = CampaignState::new(500);
        state.roster.push(hero);

        let json = state.to_json().unwrap();
        let restored = CampaignState::from_json(&json).unwrap();

        let h = &restored.roster[0];
        assert_eq!(h.quirks.positive, vec!["eagle_eye"]);
        assert_eq!(h.quirks.negative, vec!["fearful"]);
        assert_eq!(h.quirks.diseases, vec!["rabies"]);
        assert_eq!(h.quirks.negative_count(), 2);
    }

    #[test]
    fn hero_traits_roundtrip_preserves_afflictions_and_virtues() {
        let mut hero = CampaignHero::new("h1", "crusader", 1, 0, 100.0, 100.0, 0.0, 200.0);
        hero.traits.virtues = vec!["courageous".to_string()];
        hero.traits.afflictions = vec!["paranoid".to_string()];

        let mut state = CampaignState::new(500);
        state.roster.push(hero);

        let json = state.to_json().unwrap();
        let restored = CampaignState::from_json(&json).unwrap();

        let h = &restored.roster[0];
        assert_eq!(h.traits.virtues, vec!["courageous"]);
        assert_eq!(h.traits.afflictions, vec!["paranoid"]);
    }

    #[test]
    fn hero_equipment_roundtrip_preserves_levels_and_trinkets() {
        let mut hero = CampaignHero::new("h1", "tank", 1, 0, 100.0, 100.0, 0.0, 200.0);
        hero.equipment.weapon_level = 3;
        hero.equipment.armor_level = 2;
        hero.equipment.trinkets = vec!["shield_medallion".to_string()];

        let mut state = CampaignState::new(500);
        state.roster.push(hero);

        let json = state.to_json().unwrap();
        let restored = CampaignState::from_json(&json).unwrap();

        let h = &restored.roster[0];
        assert_eq!(h.equipment.weapon_level, 3);
        assert_eq!(h.equipment.armor_level, 2);
        assert_eq!(h.equipment.trinkets, vec!["shield_medallion"]);
    }

    #[test]
    fn hero_skills_roundtrip_preserves_order_and_ids() {
        let mut hero = CampaignHero::new("h1", "shaman", 1, 0, 100.0, 100.0, 0.0, 200.0);
        hero.skills = vec![
            "skill_lightning".to_string(),
            "skill_hex".to_string(),
            "skill_totem".to_string(),
        ];

        let mut state = CampaignState::new(500);
        state.roster.push(hero);

        let json = state.to_json().unwrap();
        let restored = CampaignState::from_json(&json).unwrap();

        assert_eq!(restored.roster[0].skills.len(), 3);
        assert_eq!(restored.roster[0].skills[0], "skill_lightning");
        assert_eq!(restored.roster[0].skills[1], "skill_hex");
        assert_eq!(restored.roster[0].skills[2], "skill_totem");
    }

    #[test]
    fn inventory_items_roundtrip_preserves_id_and_quantity() {
        let mut state = CampaignState::new(500);
        state.inventory.push(CampaignInventoryItem::new("torch", 8));
        state.inventory.push(CampaignInventoryItem::new("bandage", 4));

        let json = state.to_json().unwrap();
        let restored = CampaignState::from_json(&json).unwrap();

        assert_eq!(restored.inventory.len(), 2);
        assert_eq!(restored.inventory[0].id, "torch");
        assert_eq!(restored.inventory[0].quantity, 8);
        assert_eq!(restored.inventory[1].id, "bandage");
        assert_eq!(restored.inventory[1].quantity, 4);
    }

    #[test]
    fn run_history_roundtrip_preserves_all_fields() {
        let mut state = CampaignState::new(500);
        state.run_history.push(CampaignRunRecord::new(
            DungeonType::ZhuQue, MapSize::Short,
            9, 3, true, 500,
        ));

        let json = state.to_json().unwrap();
        let restored = CampaignState::from_json(&json).unwrap();

        assert_eq!(restored.run_history.len(), 1);
        let run = &restored.run_history[0];
        assert_eq!(run.dungeon, DungeonType::ZhuQue);
        assert_eq!(run.map_size, MapSize::Short);
        assert_eq!(run.rooms_cleared, 9);
        assert_eq!(run.battles_won, 3);
        assert!(run.completed);
        assert_eq!(run.gold_earned, 500);
    }

    #[test]
    fn quest_progress_roundtrip_preserves_step_tracking() {
        let mut state = CampaignState::new(500);
        let mut q = CampaignQuestProgress::new("cleanse_all_dungeons", 4);
        q.current_step = 2;
        state.quest_progress.push(q);

        let json = state.to_json().unwrap();
        let restored = CampaignState::from_json(&json).unwrap();

        assert_eq!(restored.quest_progress.len(), 1);
        let qp = &restored.quest_progress[0];
        assert_eq!(qp.quest_id, "cleanse_all_dungeons");
        assert_eq!(qp.current_step, 2);
        assert_eq!(qp.max_steps, 4);
        assert!(!qp.completed);
    }

    #[test]
    fn btree_map_heirlooms_produce_deterministic_json_keys() {
        let mut state = CampaignState::new(100);
        state.heirlooms.insert(HeirloomCurrency::Bones, 10);
        state.heirlooms.insert(HeirloomCurrency::Portraits, 20);
        state.heirlooms.insert(HeirloomCurrency::Tapes, 30);

        let json_a = state.to_json().unwrap();
        let json_b = state.to_json().unwrap();
        assert_eq!(json_a, json_b);
        // BTreeMap guarantees sorted keys — Bones < Portraits < Tapes
        let bones_pos = json_a.find("Bones").unwrap();
        let portraits_pos = json_a.find("Portraits").unwrap();
        let tapes_pos = json_a.find("Tapes").unwrap();
        assert!(bones_pos < portraits_pos);
        assert!(portraits_pos < tapes_pos);
    }

    #[test]
    fn btree_map_building_states_produce_deterministic_json_keys() {
        let mut state = CampaignState::new(100);
        state.building_states.insert(
            "tavern".to_string(),
            BuildingUpgradeState::new("tavern", Some('c')),
        );
        state.building_states.insert(
            "abbey".to_string(),
            BuildingUpgradeState::new("abbey", Some('a')),
        );

        let json_a = state.to_json().unwrap();
        let json_b = state.to_json().unwrap();
        assert_eq!(json_a, json_b);
        // BTreeMap guarantees sorted keys — abbey < tavern
        let abbey_pos = json_a.find("abbey").unwrap();
        let tavern_pos = json_a.find("tavern").unwrap();
        assert!(abbey_pos < tavern_pos);
    }

    #[test]
    fn campaign_save_file_is_valid_json() {
        let state = CampaignState::new(100);
        let json = state.to_json().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_object());
        assert_eq!(parsed["schema_version"], CAMPAIGN_SNAPSHOT_VERSION);
        assert_eq!(parsed["gold"], 100);
    }

    #[test]
    fn game_state_save_and_load_campaign_roundtrip() {
        let mut state = load_real_state();
        state.new_campaign(1500);

        let mut hero = CampaignHero::new("hero_1", "alchemist", 3, 450, 85.0, 100.0, 25.0, 200.0);
        hero.quirks.positive = vec!["eagle_eye".to_string()];
        hero.equipment.weapon_level = 2;
        state.campaign.roster.push(hero);
        state.campaign.inventory.push(CampaignInventoryItem::new("torch", 4));

        let save_path = temp_save_path("state_roundtrip");
        state.save_campaign(&save_path).unwrap();

        let mut state2 = load_real_state();
        state2.load_campaign(&save_path).unwrap();
        std::fs::remove_file(&save_path).ok();

        assert_eq!(state2.campaign.gold, 1500);
        assert_eq!(state2.campaign.roster.len(), 1);
        assert_eq!(state2.campaign.roster[0].id, "hero_1");
        assert_eq!(state2.campaign.roster[0].equipment.weapon_level, 2);
        assert_eq!(state2.campaign.inventory.len(), 1);
        assert_eq!(state2.campaign.inventory[0].id, "torch");
    }

    #[test]
    fn game_state_save_campaign_errors_on_invalid_path() {
        let state = load_real_state();
        let result = state.save_campaign(Path::new("/nonexistent/dir/campaign.json"));
        assert!(result.is_err());
    }

    #[test]
    fn game_state_load_campaign_errors_on_missing_file() {
        let mut state = load_real_state();
        let result = state.load_campaign(Path::new("/nonexistent/campaign.json"));
        assert!(result.is_err());
    }

    #[test]
    fn game_state_load_campaign_errors_on_invalid_json() {
        let save_path = temp_save_path("bad_json");
        std::fs::write(&save_path, "not valid json {{{").unwrap();

        let mut state = load_real_state();
        let result = state.load_campaign(&save_path);
        std::fs::remove_file(&save_path).ok();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("deserialization error") || err.contains("expected value"),
            "unexpected error message: {}",
            err
        );
    }

    #[test]
    fn game_state_new_campaign_replaces_existing() {
        let mut state = load_real_state();
        state.new_campaign(100);
        state.campaign.roster.push(
            CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0),
        );

        state.new_campaign(500); // replaces campaign
        assert_eq!(state.campaign.gold, 500);
        assert!(state.campaign.roster.is_empty());
        assert_eq!(state.camping_skill_count(), 87); // content unchanged
    }

    #[test]
    fn multiple_save_load_cycles_preserve_state() {
        let mut state = load_real_state();
        state.new_campaign(1000);
        state.campaign.inventory.push(CampaignInventoryItem::new("torch", 5));

        let save_path = temp_save_path("multi_cycle");

        // Cycle 1: save → load
        state.save_campaign(&save_path).unwrap();
        state.load_campaign(&save_path).unwrap();
        assert_eq!(state.campaign.gold, 1000);
        assert_eq!(state.campaign.inventory[0].quantity, 5);

        // Cycle 2: modify → save → load
        state.campaign.gold = 2000;
        state.campaign.inventory.push(CampaignInventoryItem::new("shovel", 2));
        state.save_campaign(&save_path).unwrap();
        state.load_campaign(&save_path).unwrap();
        assert_eq!(state.campaign.gold, 2000);
        assert_eq!(state.campaign.inventory.len(), 2);

        std::fs::remove_file(&save_path).ok();
    }

    #[test]
    fn game_state_load_campaign_preserves_state_on_failure() {
        let mut state = load_real_state();
        state.new_campaign(500);
        state.campaign.roster.push(
            CampaignHero::new("h1", "alchemist", 1, 50, 90.0, 100.0, 15.0, 200.0),
        );
        state.campaign.inventory.push(CampaignInventoryItem::new("torch", 8));
        state.campaign.heirlooms.insert(HeirloomCurrency::Bones, 30);

        // Attempt to load from a nonexistent file — must fail
        let result = state.load_campaign(Path::new("/nonexistent/campaign_save.json"));
        assert!(result.is_err(), "loading a missing file must return an error");

        // Existing campaign state must be preserved
        assert_eq!(state.campaign.gold, 500);
        assert_eq!(state.campaign.roster.len(), 1);
        assert_eq!(state.campaign.roster[0].id, "h1");
        assert_eq!(state.campaign.roster[0].health, 90.0);
        assert_eq!(state.campaign.roster[0].stress, 15.0);
        assert_eq!(state.campaign.inventory[0].id, "torch");
        assert_eq!(state.campaign.inventory[0].quantity, 8);
        assert_eq!(state.campaign.heirlooms[&HeirloomCurrency::Bones], 30);
        assert_eq!(state.camping_skill_count(), 87);
    }

    #[test]
    fn campaign_continues_across_multiple_persisted_loops() {
        let save_path = temp_save_path("multi_loop_e2e");

        // ── Session 1: fresh campaign, first dungeon run ──────────────
        let mut state = load_real_state();
        state.new_campaign(500);
        state.campaign.roster.push(
            CampaignHero::new("hero_1", "crusader", 1, 50, 90.0, 100.0, 15.0, 200.0),
        );
        state.campaign.inventory.push(CampaignInventoryItem::new("torch", 8));
        state.campaign.inventory.push(CampaignInventoryItem::new("shovel", 2));
        state.save_campaign(&save_path).unwrap();

        // ── Session 2: load, simulate dungeon rewards, save ───────────
        let mut state = load_real_state();
        state.load_campaign(&save_path).unwrap();
        assert_eq!(state.campaign.gold, 500);
        assert_eq!(state.campaign.roster.len(), 1);
        assert_eq!(state.campaign.inventory.len(), 2);
        assert_eq!(state.campaign.roster[0].health, 90.0);

        // Simulate rewards from a completed dungeon run
        state.campaign.gold += 350;
        state.campaign.roster[0].xp += 200;
        state.campaign.roster[0].health = 75.0;
        state.campaign.roster[0].stress = 45.0;
        state.campaign.run_history.push(CampaignRunRecord::new(
            DungeonType::QingLong, MapSize::Short,
            9, 3, true, 350,
        ));
        state.campaign.heirlooms.insert(HeirloomCurrency::Bones, 15);
        state.campaign.inventory[0].quantity -= 3;
        state.campaign.inventory.push(CampaignInventoryItem::new("bandage", 2));
        state.save_campaign(&save_path).unwrap();

        // ── Session 3: load, verify accumulated state ─────────────────
        let mut state = load_real_state();
        state.load_campaign(&save_path).unwrap();
        assert_eq!(state.campaign.gold, 850, "gold should accumulate across sessions");
        assert_eq!(state.campaign.roster.len(), 1);
        assert_eq!(state.campaign.roster[0].health, 75.0);
        assert_eq!(state.campaign.roster[0].stress, 45.0);
        assert_eq!(state.campaign.roster[0].xp, 250);
        assert_eq!(state.campaign.run_history.len(), 1);
        assert_eq!(state.campaign.run_history[0].gold_earned, 350);
        assert_eq!(state.campaign.inventory[0].quantity, 5);
        assert_eq!(state.campaign.heirlooms[&HeirloomCurrency::Bones], 15);

        // Simulate recruiting a second hero and another dungeon
        state.campaign.roster.push(
            CampaignHero::new("hero_2", "hunter", 1, 0, 100.0, 100.0, 0.0, 200.0),
        );
        state.campaign.roster[1].skills = vec![
            "skill_aimed_shot".to_string(),
            "skill_quick_shot".to_string(),
        ];
        state.campaign.roster[1].equipment.weapon_level = 1;
        state.campaign.gold += 500;
        state.campaign.roster[0].xp += 300;
        state.campaign.roster[0].health = 60.0;
        state.campaign.roster[1].xp += 250;
        state.campaign.roster[1].health = 80.0;
        state.campaign.run_history.push(CampaignRunRecord::new(
            DungeonType::BaiHu, MapSize::Medium,
            12, 4, true, 500,
        ));
        state.campaign.inventory[0].quantity -= 2;
        state.save_campaign(&save_path).unwrap();

        // ── Session 4: load, verify all accumulated state ─────────────
        let mut state = load_real_state();
        state.load_campaign(&save_path).unwrap();
        assert_eq!(state.campaign.gold, 1350, "gold should accumulate across all sessions");
        assert_eq!(state.campaign.roster.len(), 2);
        assert_eq!(state.campaign.roster[0].id, "hero_1");
        assert_eq!(state.campaign.roster[0].health, 60.0);
        assert_eq!(state.campaign.roster[0].xp, 550);
        assert_eq!(state.campaign.roster[1].id, "hero_2");
        assert_eq!(state.campaign.roster[1].health, 80.0);
        assert_eq!(state.campaign.roster[1].xp, 250);
        assert_eq!(state.campaign.roster[1].equipment.weapon_level, 1);
        assert_eq!(state.campaign.roster[1].skills, vec!["skill_aimed_shot", "skill_quick_shot"]);
        assert_eq!(state.campaign.run_history.len(), 2);
        assert_eq!(state.campaign.run_history[0].dungeon, DungeonType::QingLong);
        assert_eq!(state.campaign.run_history[1].dungeon, DungeonType::BaiHu);
        assert_eq!(state.campaign.inventory.len(), 3);
        assert_eq!(state.campaign.inventory[0].quantity, 3); // torches: 8-3-2=3
        assert_eq!(state.campaign.heirlooms[&HeirloomCurrency::Bones], 15);
        assert_eq!(state.camping_skill_count(), 87); // content intact

        std::fs::remove_file(&save_path).ok();
    }

    // ── US-004: Three-loop continuity test ────────────────────────────────────

    #[test]
    fn three_loop_seeded_scenario_preserves_hero_estate_inventory_continuity() {
        // US-004 acceptance test: proves that a three-loop seeded scenario
        // preserves hero/estate/inventory continuity when dungeon rewards and
        // town activities are applied through explicit campaign state transitions.
        //
        // Loop 1: Fresh campaign, start with 2 heroes, 500 gold, basic inventory
        // Loop 2: After dungeon rewards (gold, XP, heirlooms) and town stress heal
        // Loop 3: After equipment upgrades, more dungeon rewards, roster growth

        let save_path = temp_save_path("three_loop_continuity");

        // ── Loop 1: Fresh campaign ────────────────────────────────────────────
        let mut state = load_real_state();
        state.new_campaign(500);

        // Add initial heroes
        let mut hero1 = CampaignHero::new("hero_1", "crusader", 1, 0, 100.0, 100.0, 20.0, 200.0);
        hero1.skills = vec!["skill_stab".to_string(), "skill_inspire".to_string()];
        hero1.equipment.weapon_level = 1;
        hero1.equipment.armor_level = 1;
        hero1.quirks.positive = vec!["eagle_eye".to_string()];
        state.campaign.roster.push(hero1);

        let mut hero2 = CampaignHero::new("hero_2", "alchemist", 1, 0, 100.0, 100.0, 15.0, 200.0);
        hero2.skills = vec!["skill_fire_bomb".to_string()];
        hero2.equipment.weapon_level = 0;
        hero2.equipment.armor_level = 1;
        state.campaign.roster.push(hero2);

        // Initial inventory: torches, bandages, shovel
        state.campaign.inventory.push(CampaignInventoryItem::new("torch", 8));
        state.campaign.inventory.push(CampaignInventoryItem::new("bandage", 4));
        state.campaign.inventory.push(CampaignInventoryItem::new("shovel", 1));

        // Initial heirlooms
        state.campaign.heirlooms.insert(HeirloomCurrency::Bones, 10);

        state.save_campaign(&save_path).unwrap();

        // ── Loop 2: Load, apply dungeon rewards, apply town activity ──────────
        let mut state = load_real_state();
        state.load_campaign(&save_path).unwrap();

        // Verify initial state
        assert_eq!(state.campaign.gold, 500);
        assert_eq!(state.campaign.roster.len(), 2);
        assert_eq!(state.campaign.roster[0].id, "hero_1");
        assert_eq!(state.campaign.roster[1].id, "hero_2");
        assert_eq!(state.campaign.inventory.len(), 3);
        assert_eq!(state.campaign.heirlooms[&HeirloomCurrency::Bones], 10);

        // Simulate dungeon run 1: QingLong short, won, earned rewards
        // Apply dungeon rewards via explicit state transitions
        state.campaign.apply_dungeon_gold(350);
        state.campaign.apply_dungeon_xp(150);

        // Apply some heirlooms from dungeon
        let mut heirlooms = std::collections::BTreeMap::new();
        heirlooms.insert(HeirloomCurrency::Bones, 15);
        heirlooms.insert(HeirloomCurrency::Portraits, 5);
        state.campaign.apply_dungeon_heirlooms(&heirlooms);

        // Consume torches from dungeon
        state.campaign.apply_inventory_change("torch", -3);

        // Add loot found
        state.campaign.apply_inventory_change("antiquarian_teacup", 1);

        // Update hero vitals from dungeon run
        state.campaign.sync_hero_vitals("hero_1", 75.0, 45.0);
        state.campaign.sync_hero_vitals("hero_2", 80.0, 35.0);

        // Record the run
        state.campaign.record_dungeon_run(
            DungeonType::QingLong,
            MapSize::Short,
            9,
            3,
            true,
            350,
        );

        // Apply town activity: stress heal at Abbey (gold spent reduces campaign gold)
        state.campaign.apply_town_gold_spent(100);
        state.campaign.apply_town_stress_heal("hero_1", 20.0);
        state.campaign.apply_town_stress_heal("hero_2", 15.0);

        state.save_campaign(&save_path).unwrap();

        // ── Loop 3: Load, more dungeon rewards, equipment upgrade, recruit ─────
        let mut state = load_real_state();
        state.load_campaign(&save_path).unwrap();

        // Verify loop 2 state preserved
        assert_eq!(state.campaign.gold, 750); // 500 + 350 - 100
        assert_eq!(state.campaign.roster.len(), 2);
        assert_eq!(state.campaign.roster[0].xp, 75); // XP distributed: 150/2 = 75 each
        assert_eq!(state.campaign.roster[1].xp, 75);
        assert_eq!(state.campaign.heirlooms[&HeirloomCurrency::Bones], 25); // 10 + 15
        assert_eq!(state.campaign.heirlooms[&HeirloomCurrency::Portraits], 5);
        assert_eq!(state.campaign.inventory.len(), 4);
        assert_eq!(state.campaign.inventory[0].quantity, 5); // torches: 8-3=5
        assert_eq!(state.campaign.run_history.len(), 1);

        // Hero vitals after town stress heal
        assert_eq!(state.campaign.roster[0].stress, 25.0); // 45 - 20
        assert_eq!(state.campaign.roster[1].stress, 20.0); // 35 - 15

        // Simulate dungeon run 2: BaiHu medium, won, earned more rewards
        state.campaign.apply_dungeon_gold(500);
        state.campaign.apply_dungeon_xp(200);

        // More heirlooms
        let mut heirlooms2 = std::collections::BTreeMap::new();
        heirlooms2.insert(HeirloomCurrency::Bones, 20);
        heirlooms2.insert(HeirloomCurrency::Tapes, 3);
        state.campaign.apply_dungeon_heirlooms(&heirlooms2);

        // Consume more torches
        state.campaign.apply_inventory_change("torch", -2);

        // Add more loot
        state.campaign.apply_inventory_change("sacred_chalice", 1);

        // Upgrade hero 1's weapon
        state.campaign.upgrade_hero_weapon("hero_1");
        // Upgrade hero 2's armor
        state.campaign.upgrade_hero_armor("hero_2");

        // Record the run
        state.campaign.record_dungeon_run(
            DungeonType::BaiHu,
            MapSize::Medium,
            12,
            4,
            true,
            500,
        );

        // Town activity: recruit a new hero
        state.campaign.apply_town_gold_spent(500);
        let new_hero = CampaignHero::new("hero_3", "hunter", 1, 0, 100.0, 100.0, 0.0, 200.0);
        state.campaign.add_hero(new_hero);

        state.save_campaign(&save_path).unwrap();

        // ── Loop 4: Final verification ────────────────────────────────────────
        let mut state = load_real_state();
        state.load_campaign(&save_path).unwrap();

        // Gold: 750 + 500 - 500 = 750
        assert_eq!(state.campaign.gold, 750, "Gold should reflect all earnings and spending");

        // Roster should have 3 heroes
        assert_eq!(state.campaign.roster.len(), 3, "Should have recruited hero_3");
        assert_eq!(state.campaign.roster[0].id, "hero_1");
        assert_eq!(state.campaign.roster[1].id, "hero_2");
        assert_eq!(state.campaign.roster[2].id, "hero_3");

        // hero_1: weapon upgraded, XP accumulated
        assert_eq!(state.campaign.roster[0].equipment.weapon_level, 2);
        assert_eq!(state.campaign.roster[0].xp, 175); // 75 + 100 from second dungeon run

        // hero_2: armor upgraded
        assert_eq!(state.campaign.roster[1].equipment.armor_level, 2);

        // Run history should have 2 entries
        assert_eq!(state.campaign.run_history.len(), 2);
        assert_eq!(state.campaign.run_history[0].dungeon, DungeonType::QingLong);
        assert_eq!(state.campaign.run_history[1].dungeon, DungeonType::BaiHu);

        // Heirlooms accumulated
        assert_eq!(state.campaign.heirlooms[&HeirloomCurrency::Bones], 45); // 25 + 20
        assert_eq!(state.campaign.heirlooms[&HeirloomCurrency::Portraits], 5);
        assert_eq!(state.campaign.heirlooms[&HeirloomCurrency::Tapes], 3);

        // Inventory: torches 8-3-2=3, plus loot
        let torch_qty = state.campaign.inventory.iter()
            .find(|i| i.id == "torch")
            .map(|i| i.quantity)
            .unwrap_or(0);
        assert_eq!(torch_qty, 3, "Torches should be consumed across runs");

        // Should have 5 inventory items: torch, bandage, shovel, antiquarian_teacup, sacred_chalice
        assert_eq!(state.campaign.inventory.len(), 5, "Inventory should include dungeon loot");

        std::fs::remove_file(&save_path).ok();
    }

    #[test]
    fn apply_dungeon_gold_saturates_on_overflow() {
        let mut campaign = CampaignState::new(u32::MAX);
        campaign.apply_dungeon_gold(100);
        assert_eq!(campaign.gold, u32::MAX);
    }

    #[test]
    fn apply_inventory_change_removes_item_when_depleted() {
        let mut campaign = CampaignState::new(0);
        campaign.inventory.push(CampaignInventoryItem::new("torch", 3));

        // Consume more than we have
        campaign.apply_inventory_change("torch", -5);

        // Item should be removed
        assert!(!campaign.inventory.iter().any(|i| i.id == "torch"));
    }

    #[test]
    fn sync_hero_vitals_clamps_to_valid_range() {
        let mut campaign = CampaignState::new(0);
        campaign.roster.push(CampaignHero::new("h1", "crusader", 1, 0, 100.0, 100.0, 0.0, 200.0));

        // Try to set health above max and stress above max
        campaign.sync_hero_vitals("h1", 150.0, 250.0);

        let hero = campaign.roster.iter().find(|h| h.id == "h1").unwrap();
        assert_eq!(hero.health, 100.0); // Clamped to max
        assert_eq!(hero.stress, 200.0); // Clamped to max
    }

    #[test]
    fn add_hero_assigns_id_if_empty() {
        let mut campaign = CampaignState::new(0);
        let hero = CampaignHero::new("", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0);
        campaign.add_hero(hero);

        assert!(!campaign.roster[0].id.is_empty());
        assert!(campaign.roster[0].id.starts_with("hero_"));
    }

    #[test]
    fn remove_hero_returns_and_removes_hero() {
        let mut campaign = CampaignState::new(0);
        campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));
        campaign.roster.push(CampaignHero::new("h2", "hunter", 1, 0, 100.0, 100.0, 0.0, 200.0));

        let removed = campaign.remove_hero("h1");

        assert!(removed.is_some());
        assert_eq!(removed.unwrap().id, "h1");
        assert_eq!(campaign.roster.len(), 1);
        assert_eq!(campaign.roster[0].id, "h2");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Quest Runtime State Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod quest_tests {
    use super::*;

    // ── QuestDifficulty tests ─────────────────────────────────────────────────

    #[test]
    fn quest_difficulty_dungeon_level_standard() {
        assert_eq!(QuestDifficulty::Standard.dungeon_level(), 3);
    }

    #[test]
    fn quest_difficulty_dungeon_level_hard() {
        assert_eq!(QuestDifficulty::Hard.dungeon_level(), 5);
    }

    // ── QuestObjective tests ─────────────────────────────────────────────────

    #[test]
    fn quest_objective_from_quest_type_explore() {
        let obj = QuestObjective::from_quest_type(crate::contracts::QuestType::Explore);
        assert_eq!(obj, Some(QuestObjective::ClearDungeon));
    }

    #[test]
    fn quest_objective_from_quest_type_kill_boss() {
        let obj = QuestObjective::from_quest_type(crate::contracts::QuestType::KillBoss);
        assert_eq!(obj, Some(QuestObjective::KillBoss));
    }

    #[test]
    fn quest_objective_from_quest_type_cleanse() {
        let obj = QuestObjective::from_quest_type(crate::contracts::QuestType::Cleanse);
        assert_eq!(obj, Some(QuestObjective::CleanseCorruption));
    }

    #[test]
    fn quest_objective_from_quest_type_gather() {
        let obj = QuestObjective::from_quest_type(crate::contracts::QuestType::Gather);
        assert_eq!(obj, Some(QuestObjective::GatherItems));
    }

    #[test]
    fn quest_objective_from_quest_type_activate() {
        let obj = QuestObjective::from_quest_type(crate::contracts::QuestType::Activate);
        assert_eq!(obj, Some(QuestObjective::ActivateMechanism));
    }

    #[test]
    fn quest_objective_from_quest_type_inventory_activate() {
        let obj = QuestObjective::from_quest_type(crate::contracts::QuestType::InventoryActivate);
        assert_eq!(obj, Some(QuestObjective::UseInventoryItem));
    }

    // ── QuestRewards tests ──────────────────────────────────────────────────

    #[test]
    fn quest_rewards_standard_has_gold_and_heirlooms() {
        let rewards = QuestRewards::standard();
        assert_eq!(rewards.gold, 500);
        assert!(rewards.heirlooms.contains_key(&crate::contracts::HeirloomCurrency::Bones));
        assert!(rewards.heirlooms.contains_key(&crate::contracts::HeirloomCurrency::Portraits));
        assert_eq!(rewards.xp, 200);
    }

    #[test]
    fn quest_rewards_hard_has_more_rewards() {
        let hard = QuestRewards::hard();
        let standard = QuestRewards::standard();
        assert!(hard.gold > standard.gold);
        assert!(hard.xp > standard.xp);
    }

    // ── QuestPenalties tests ─────────────────────────────────────────────────

    #[test]
    fn quest_penalties_standard_are_negative() {
        let penalties = QuestPenalties::standard();
        assert!(penalties.gold < 0);
        for (_, amount) in &penalties.heirlooms {
            assert!(*amount < 0);
        }
    }

    // ── QuestState creation tests ────────────────────────────────────────────

    #[test]
    fn quest_state_new_kill_boss_quest_has_correct_defaults() {
        let quest = QuestState::new_kill_boss_quest("test_quest");
        assert_eq!(quest.quest_id, "test_quest");
        assert_eq!(quest.difficulty, QuestDifficulty::Standard);
        assert_eq!(quest.dungeon, crate::contracts::DungeonType::QingLong);
        assert_eq!(quest.map_size, crate::contracts::MapSize::Short);
        assert_eq!(quest.objective, QuestObjective::KillBoss);
        assert_eq!(quest.current_step, 0);
        assert_eq!(quest.max_steps, 2);
        assert!(!quest.completed);
        assert!(!quest.failed);
    }

    #[test]
    fn quest_state_is_active_when_created() {
        let quest = QuestState::new_kill_boss_quest("test");
        assert!(quest.is_active());
    }

    #[test]
    fn quest_state_is_not_active_after_complete() {
        let mut quest = QuestState::new_kill_boss_quest("test");
        let rewards = quest.complete();
        assert!(rewards.is_some());
        assert!(!quest.is_active());
        assert!(quest.completed);
    }

    #[test]
    fn quest_state_is_not_active_after_fail() {
        let mut quest = QuestState::new_kill_boss_quest("test");
        let penalties = quest.fail();
        assert!(penalties.is_some());
        assert!(!quest.is_active());
        assert!(quest.failed);
    }

    #[test]
    fn quest_state_cannot_complete_twice() {
        let mut quest = QuestState::new_kill_boss_quest("test");
        quest.complete();
        let second = quest.complete();
        assert!(second.is_none());
    }

    #[test]
    fn quest_state_cannot_fail_after_complete() {
        let mut quest = QuestState::new_kill_boss_quest("test");
        quest.complete();
        let failed = quest.fail();
        assert!(failed.is_none());
    }

    // ── Quest progress tests ─────────────────────────────────────────────────

    #[test]
    fn quest_state_advance_progress_increments_step() {
        let mut quest = QuestState::new_kill_boss_quest("test");
        assert_eq!(quest.current_step, 0);
        quest.advance_progress();
        assert_eq!(quest.current_step, 1);
    }

    #[test]
    fn quest_state_advance_progress_maxes_out() {
        let mut quest = QuestState::new_kill_boss_quest("test");
        quest.advance_progress();
        quest.advance_progress();
        quest.advance_progress(); // should not exceed max_steps
        assert_eq!(quest.current_step, 2);
    }

    #[test]
    fn quest_state_can_complete_when_steps_done() {
        let mut quest = QuestState::new_kill_boss_quest("test");
        quest.advance_progress();
        quest.advance_progress();
        assert!(quest.can_complete());
    }

    // ── Reward/penalty handoff tests ────────────────────────────────────────

    #[test]
    fn quest_state_apply_rewards_increases_campaign_gold() {
        let quest = QuestState::new_kill_boss_quest("test");
        let mut campaign = crate::contracts::CampaignState::new(100);
        let rewards = quest.rewards.clone();
        quest.apply_rewards_to_campaign(&rewards, &mut campaign);
        assert_eq!(campaign.gold, 600); // 100 + 500
    }

    #[test]
    fn quest_state_apply_rewards_adds_heirlooms() {
        let quest = QuestState::new_kill_boss_quest("test");
        let mut campaign = crate::contracts::CampaignState::new(100);
        let rewards = quest.rewards.clone();
        quest.apply_rewards_to_campaign(&rewards, &mut campaign);
        assert_eq!(campaign.heirlooms[&crate::contracts::HeirloomCurrency::Bones], 10);
    }

    #[test]
    fn quest_state_apply_rewards_distributes_xp_to_roster() {
        let quest = QuestState::new_kill_boss_quest("test");
        let mut campaign = crate::contracts::CampaignState::new(100);
        // Add heroes to roster
        campaign.roster.push(crate::contracts::CampaignHero::new(
            "hero_1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0,
        ));
        campaign.roster.push(crate::contracts::CampaignHero::new(
            "hero_2", "hunter", 1, 0, 100.0, 100.0, 0.0, 200.0,
        ));
        let rewards = quest.rewards.clone();
        quest.apply_rewards_to_campaign(&rewards, &mut campaign);
        // 200 XP / 2 heroes = 100 XP each
        assert_eq!(campaign.roster[0].xp, 100);
        assert_eq!(campaign.roster[1].xp, 100);
    }

    #[test]
    fn quest_state_apply_penalties_reduces_campaign_gold() {
        let quest = QuestState::new_kill_boss_quest("test");
        let mut campaign = crate::contracts::CampaignState::new(500);
        let penalties = quest.penalties.clone();
        quest.apply_penalties_to_campaign(&penalties, &mut campaign);
        assert_eq!(campaign.gold, 400); // 500 - 100
    }

    #[test]
    fn quest_state_apply_penalties_cannot_go_negative() {
        let quest = QuestState::new_kill_boss_quest("test");
        let mut campaign = crate::contracts::CampaignState::new(50);
        let penalties = quest.penalties.clone();
        quest.apply_penalties_to_campaign(&penalties, &mut campaign);
        assert_eq!(campaign.gold, 0); // 50 - 100, but not below 0
    }

    // ── Update from run tests ────────────────────────────────────────────────

    #[test]
    fn quest_state_update_from_run_progresses_quest() {
        let mut quest = QuestState::new_kill_boss_quest("test");
        let updated = quest.update_from_run(
            crate::contracts::DungeonType::QingLong,
            crate::contracts::MapSize::Short,
            9,  // rooms_cleared
            3,  // battles_won
            true, // completed
        );
        assert!(updated.is_some());
        assert_eq!(quest.current_step, 2); // maxed out from battles_won
    }

    #[test]
    fn quest_state_update_from_run_ignores_wrong_dungeon() {
        let mut quest = QuestState::new_kill_boss_quest("test");
        let updated = quest.update_from_run(
            crate::contracts::DungeonType::BaiHu, // wrong dungeon
            crate::contracts::MapSize::Short,
            9, 3, true,
        );
        assert!(updated.is_none());
        assert_eq!(quest.current_step, 0);
    }

    #[test]
    fn quest_state_update_from_run_ignores_completed_quest() {
        let mut quest = QuestState::new_kill_boss_quest("test");
        quest.complete();
        let updated = quest.update_from_run(
            crate::contracts::DungeonType::QingLong,
            crate::contracts::MapSize::Short,
            9, 3, true,
        );
        assert!(updated.is_none());
    }

    // ── Unsupported quest type tracing tests ─────────────────────────────────

    #[test]
    fn unsupported_quest_trace_kill_boss_is_supported() {
        let trace = UnsupportedQuestTrace::new(crate::contracts::QuestType::KillBoss);
        assert!(trace.is_supported());
    }

    #[test]
    fn unsupported_quest_trace_explore_is_not_supported() {
        let trace = UnsupportedQuestTrace::new(crate::contracts::QuestType::Explore);
        assert!(!trace.is_supported());
        assert!(trace.reason.contains("Explore"));
    }

    #[test]
    fn unsupported_quest_trace_cleanse_is_not_supported() {
        let trace = UnsupportedQuestTrace::new(crate::contracts::QuestType::Cleanse);
        assert!(!trace.is_supported());
    }

    #[test]
    fn unsupported_quest_trace_gather_is_not_supported() {
        let trace = UnsupportedQuestTrace::new(crate::contracts::QuestType::Gather);
        assert!(!trace.is_supported());
    }

    #[test]
    fn unsupported_quest_trace_activate_is_not_supported() {
        let trace = UnsupportedQuestTrace::new(crate::contracts::QuestType::Activate);
        assert!(!trace.is_supported());
    }

    #[test]
    fn unsupported_quest_trace_inventory_activate_is_not_supported() {
        let trace = UnsupportedQuestTrace::new(crate::contracts::QuestType::InventoryActivate);
        assert!(!trace.is_supported());
    }

    // ── Conversion tests ────────────────────────────────────────────────────

    #[test]
    fn quest_state_to_campaign_quest_progress_preserves_fields() {
        let quest = QuestState::new_kill_boss_quest("my_quest");
        let progress = quest.to_campaign_quest_progress();
        assert_eq!(progress.quest_id, "my_quest");
        assert_eq!(progress.current_step, 0);
        assert_eq!(progress.max_steps, 2);
        assert!(!progress.completed);
    }

    // ── Full quest lifecycle test ────────────────────────────────────────────

    #[test]
    fn full_quest_lifecycle_accepted_progressed_completed_rewarded() {
        // This is the representative test proving a KillBoss quest can be:
        // 1. Accepted
        // 2. Progressed through dungeon runs
        // 3. Completed
        // 4. Rewarded through the run loop

        // ── 1. Accept: Create a new KillBoss quest ────────────────────────
        let mut quest = QuestState::new_kill_boss_quest("kill_boss_qinglong");
        assert!(quest.is_active());
        assert_eq!(quest.current_step, 0);

        // ── 2. Progress: Simulate dungeon run events ─────────────────────
        // First dungeon run: clear some rooms, win battles
        let step1 = quest.update_from_run(
            crate::contracts::DungeonType::QingLong,
            crate::contracts::MapSize::Short,
            5,  // rooms_cleared
            2,  // battles_won
            false, // not completed yet
        );
        assert!(step1.is_some());
        assert_eq!(quest.current_step, 1);

        // Second dungeon run: win remaining battles
        let step2 = quest.update_from_run(
            crate::contracts::DungeonType::QingLong,
            crate::contracts::MapSize::Short,
            9,  // rooms_cleared
            3,  // battles_won
            true, // completed
        );
        assert!(step2.is_some());
        assert!(quest.can_complete());

        // ── 3. Complete the quest ─────────────────────────────────────────
        let rewards = quest.complete();
        assert!(rewards.is_some());
        assert!(!quest.is_active());
        assert!(quest.completed);

        // ── 4. Apply rewards to campaign ───────────────────────────────────
        let mut campaign = crate::contracts::CampaignState::new(1000);
        campaign.roster.push(crate::contracts::CampaignHero::new(
            "hero_1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0,
        ));
        let rewards = rewards.unwrap();
        quest.apply_rewards_to_campaign(&rewards, &mut campaign);

        // Verify gold increased
        assert_eq!(campaign.gold, 1500); // 1000 + 500
        // Verify heirlooms were added
        assert_eq!(campaign.heirlooms[&crate::contracts::HeirloomCurrency::Bones], 10);
        // Verify XP was distributed
        assert_eq!(campaign.roster[0].xp, 200);
    }

    #[test]
    fn full_quest_lifecycle_with_failure() {
        // Test quest failure and penalty application

        let mut quest = QuestState::new_kill_boss_quest("kill_boss_qinglong");

        // Simulate a failed run (no battles won, so no progress)
        let updated = quest.update_from_run(
            crate::contracts::DungeonType::QingLong,
            crate::contracts::MapSize::Short,
            3,  // rooms_cleared
            0,  // battles_won - lost!
            false,
        );
        assert!(updated.is_some());
        // KillBoss objective requires battles_won > 0 to advance, so step stays at 0
        assert_eq!(quest.current_step, 0);

        // Fail the quest
        let penalties = quest.fail();
        assert!(penalties.is_some());
        assert!(quest.failed);
        assert!(!quest.is_active());

        // Apply penalties
        let mut campaign = crate::contracts::CampaignState::new(500);
        let penalties = penalties.unwrap();
        quest.apply_penalties_to_campaign(&penalties, &mut campaign);

        // Verify gold decreased
        assert_eq!(campaign.gold, 400); // 500 - 100
    }
}
