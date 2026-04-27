//! DDGC view models — top-level screen-state shapes for the frontend host.
//!
//! This module provides DDGC-specific view model types that map from
//! framework payloads (framework_viewmodels, framework_progression, framework_combat)
//! into product-specific shapes consumable by screen components.
//!
//! # Design principles
//!
//! - View models are **read-only data shapes** — no behavior, only data.
//! - They are **product-owned** — defined in `game_ddgc_headless`, not in framework crates.
//! - Unsupported or partially-mapped runtime states produce explicit [`ViewModelError`]
//!   surfaces rather than silently returning partial data.
//! - Adapters (in [`super::adapters`]) handle the conversion from framework payloads
//!   to these view models.
//!
//! # View model states
//!
//! - [`BootLoadViewModel`] — initial game boot and loading state
//! - [`TownViewModel`] — town visit with activities, buildings, and hero roster
//! - [`DungeonViewModel`] — active dungeon run with room progression
//! - [`CombatViewModel`] — battle state with encounter resolution
//! - [`ResultViewModel`] — outcome of combat or dungeon completion
//! - [`ReturnFlowViewModel`] — returning from dungeon back to town

use serde::{Deserialize, Serialize};

/// Errors that can occur when shaping a view model from a payload.
///
/// These errors indicate that the runtime state could not be fully mapped
/// to a DDGC view model, allowing screen components to render explicit
/// fallback or error surfaces rather than silent partial data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ViewModelError {
    /// The runtime state is not supported by DDGC view model mapping.
    UnsupportedState {
        state_type: String,
        detail: String,
    },
    /// The runtime state is only partially mapped — some data may be incomplete.
    PartialMapping {
        state_type: String,
        missing_fields: Vec<String>,
    },
    /// Required data was missing from the payload.
    MissingRequiredField {
        field: String,
        context: String,
    },
    /// The payload schema version is incompatible with the view model.
    IncompatibleSchema {
        expected: String,
        found: String,
    },
}

impl ViewModelError {
    /// Returns a human-readable description of this error.
    pub fn description(&self) -> String {
        match self {
            ViewModelError::UnsupportedState { state_type, detail } => {
                format!("unsupported {} state: {}", state_type, detail)
            }
            ViewModelError::PartialMapping { state_type, missing_fields } => {
                format!(
                    "partial {} mapping, missing fields: {}",
                    state_type,
                    missing_fields.join(", ")
                )
            }
            ViewModelError::MissingRequiredField { field, context } => {
                format!("missing required field '{}' in {}", field, context)
            }
            ViewModelError::IncompatibleSchema { expected, found } => {
                format!("incompatible schema: expected {}, found {}", expected, found)
            }
        }
    }
}

impl std::fmt::Display for ViewModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl std::error::Error for ViewModelError {}

/// Result type for view model shaping operations.
pub type ViewModelResult<T> = Result<T, ViewModelError>;

// ─────────────────────────────────────────────────────────────────────────────
// Boot/Load View Model
// ─────────────────────────────────────────────────────────────────────────────

/// Boot/load phase — initial game startup and loading state.
///
/// This view model represents the initial application state before
/// the player enters the town or begins a dungeon run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BootLoadViewModel {
    /// Whether the host finished loading successfully.
    pub loaded: bool,
    /// Human-readable status message for the loading process.
    pub status_message: String,
    /// Contract registries that were loaded (by name).
    pub registries_loaded: Vec<String>,
    /// Error details if loading failed.
    pub error: Option<String>,
    /// Schema version of the loaded campaign (if applicable).
    pub campaign_schema_version: Option<u32>,
}

impl BootLoadViewModel {
    /// Create a successful boot view model.
    pub fn success(status_message: &str, registries: Vec<&str>) -> Self {
        BootLoadViewModel {
            loaded: true,
            status_message: status_message.to_string(),
            registries_loaded: registries.iter().map(|s| s.to_string()).collect(),
            error: None,
            campaign_schema_version: None,
        }
    }

    /// Create a failed boot view model.
    pub fn failure(error_message: &str) -> Self {
        BootLoadViewModel {
            loaded: false,
            status_message: String::new(),
            registries_loaded: Vec::new(),
            error: Some(error_message.to_string()),
            campaign_schema_version: None,
        }
    }

    /// Set the campaign schema version (used when booting from a saved campaign).
    pub fn with_campaign_version(mut self, version: u32) -> Self {
        self.campaign_schema_version = Some(version);
        self
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Town View Model
// ─────────────────────────────────────────────────────────────────────────────

/// Activity type available in town.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TownActivityType {
    /// Stagecoach — recruit new heroes.
    Stagecoach,
    /// Guild — train hero skills.
    Guild,
    /// Blacksmith — repair and upgrade equipment.
    Blacksmith,
    /// Sanitarium — treat quirks and diseases.
    Sanitarium,
    /// Tavern — drink, gamble, or brothel.
    Tavern,
    /// Abbey — reduce hero stress.
    Abbey,
    /// Camping — rest at the campfire.
    Camping,
    /// Other activity.
    Other(String),
}

impl TownActivityType {
    /// Parse from a building type string.
    pub fn from_building_type(building_type: &str) -> Self {
        match building_type.to_lowercase().as_str() {
            "stagecoach" => TownActivityType::Stagecoach,
            "guild" => TownActivityType::Guild,
            "blacksmith" => TownActivityType::Blacksmith,
            "sanitarium" => TownActivityType::Sanitarium,
            "tavern" => TownActivityType::Tavern,
            "abbey" => TownActivityType::Abbey,
            "campfire" => TownActivityType::Camping,
            other => TownActivityType::Other(other.to_string()),
        }
    }
}

/// A building in town with its current upgrade state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TownBuildingViewModel {
    /// Building identifier.
    pub id: String,
    /// Building type.
    pub building_type: String,
    /// Current upgrade level code (a-g), or None if not upgraded.
    pub current_upgrade: Option<char>,
    /// Whether this building is currently available.
    pub available: bool,
}

/// A hero in town with their current vitals.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TownHeroViewModel {
    /// Unique hero identifier.
    pub id: String,
    /// Hero class identifier.
    pub class_id: String,
    /// Hero class display name.
    pub class_name: String,
    /// Current health.
    pub health: f64,
    /// Maximum health.
    pub max_health: f64,
    /// Current stress level.
    pub stress: f64,
    /// Maximum stress level.
    pub max_stress: f64,
    /// Whether the hero is wounded (health < max).
    pub is_wounded: bool,
    /// Whether the hero is afflicted (stress >= max).
    pub is_afflicted: bool,
    /// Level (resolve level).
    pub level: u32,
    /// Experience points.
    pub xp: u32,
    /// Positive quirk IDs.
    pub positive_quirks: Vec<String>,
    /// Negative quirk IDs.
    pub negative_quirks: Vec<String>,
    /// Disease quirk IDs.
    pub diseases: Vec<String>,
}

/// Town visit phase view model.
///
/// This view model represents the state when the player is in town
/// between dungeon runs, with access to various services and activities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TownViewModel {
    /// Current gold.
    pub gold: u32,
    /// Heirloom currency balances.
    pub heirlooms: std::collections::BTreeMap<String, u32>,
    /// Available buildings in town.
    pub buildings: Vec<TownBuildingViewModel>,
    /// Heroes currently on the roster.
    pub roster: Vec<TownHeroViewModel>,
    /// Available activities for this visit.
    pub available_activities: Vec<TownActivityType>,
    /// Whether this is a fresh visit (new week) or continuing.
    pub is_fresh_visit: bool,
    /// Error details if town state has issues.
    pub error: Option<ViewModelError>,
}

impl TownViewModel {
    /// Create an empty town view model with default values.
    pub fn empty() -> Self {
        TownViewModel {
            gold: 0,
            heirlooms: std::collections::BTreeMap::new(),
            buildings: Vec::new(),
            roster: Vec::new(),
            available_activities: Vec::new(),
            is_fresh_visit: true,
            error: None,
        }
    }

    /// Check if there are any wounded heroes.
    pub fn has_wounded_heroes(&self) -> bool {
        self.roster.iter().any(|h| h.is_wounded)
    }

    /// Check if there are any afflicted heroes.
    pub fn has_afflicted_heroes(&self) -> bool {
        self.roster.iter().any(|h| h.is_afflicted)
    }

    /// Get the number of available hero slots for recruitment.
    pub fn recruitment_slots_available(&self) -> usize {
        // Default max roster is 16, configurable via Stagecoach upgrades
        16usize.saturating_sub(self.roster.len())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Dungeon View Model
// ─────────────────────────────────────────────────────────────────────────────

/// Room type in a dungeon.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DungeonRoomKind {
    /// Combat encounter room.
    Combat,
    /// Boss encounter room.
    Boss,
    /// Event room with curio interaction.
    Event,
    /// Corridor with potential trap/curio.
    Corridor,
    /// Unknown room type.
    Unknown,
}

/// A room in the dungeon map.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DungeonRoomViewModel {
    /// Room identifier.
    pub room_id: String,
    /// Room type.
    pub kind: DungeonRoomKind,
    /// Whether this room has been cleared.
    pub cleared: bool,
    /// Whether this room is the current room.
    pub is_current: bool,
    /// Curio ID present in this room (if any).
    pub curio_id: Option<String>,
    /// Trap ID present in this room (if any).
    pub trap_id: Option<String>,
}

/// Dungeon run phase view model.
///
/// This view model represents the state when the player is actively
/// exploring a dungeon, progressing through rooms.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DungeonViewModel {
    /// Dungeon type identifier.
    pub dungeon_type: String,
    /// Map size variant.
    pub map_size: String,
    /// Current floor number.
    pub floor: u32,
    /// Rooms in the dungeon.
    pub rooms: Vec<DungeonRoomViewModel>,
    /// Number of rooms cleared.
    pub rooms_cleared: u32,
    /// Total rooms in the dungeon.
    pub total_rooms: u32,
    /// Current room being explored.
    pub current_room: Option<DungeonRoomViewModel>,
    /// Gold carried into the dungeon (for torchlight, etc.).
    pub gold_carried: u32,
    /// Torchlight level (0-100).
    pub torchlight: u32,
    /// Number of battles won.
    pub battles_won: u32,
    /// Number of battles lost.
    pub battles_lost: u32,
    /// Hero states in the dungeon.
    pub heroes: Vec<DungeonHeroViewModel>,
    /// Whether the dungeon is complete (all rooms cleared or boss defeated).
    pub is_complete: bool,
    /// Whether the party fled the dungeon.
    pub party_fled: bool,
    /// Error details if dungeon state has issues.
    pub error: Option<ViewModelError>,
}

/// A hero's state in the dungeon.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DungeonHeroViewModel {
    /// Unique hero identifier.
    pub id: String,
    /// Hero class identifier.
    pub class_id: String,
    /// Current health.
    pub health: f64,
    /// Maximum health.
    pub max_health: f64,
    /// Current stress level.
    pub stress: f64,
    /// Maximum stress level.
    pub max_stress: f64,
    /// Active buff IDs.
    pub active_buffs: Vec<String>,
    /// Buff IDs applied during camping (temporary).
    pub camping_buffs: Vec<String>,
    /// Whether this hero is at death's door (HP < 50% of max).
    pub is_at_deaths_door: bool,
    /// Whether this hero is dead.
    pub is_dead: bool,
}

impl DungeonHeroViewModel {
    /// Health fraction (0.0 to 1.0).
    pub fn health_fraction(&self) -> f64 {
        if self.max_health > 0.0 {
            self.health / self.max_health
        } else {
            0.0
        }
    }

    /// Stress fraction (0.0 to 1.0).
    pub fn stress_fraction(&self) -> f64 {
        if self.max_stress > 0.0 {
            self.stress / self.max_stress
        } else {
            0.0
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Combat View Model
// ─────────────────────────────────────────────────────────────────────────────

/// Position in the combat formation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CombatPosition {
    /// Lane index (0-based).
    pub lane: u32,
    /// Slot index within the lane (0-based).
    pub slot: u32,
}

/// Combat participant (hero or monster).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CombatantViewModel {
    /// Unique combatant identifier.
    pub id: String,
    /// Combatant type.
    pub combatant_type: CombatantType,
    /// Display name.
    pub name: String,
    /// Family or class identifier.
    pub family_id: String,
    /// Current health.
    pub health: f64,
    /// Maximum health.
    pub max_health: f64,
    /// Current stress (heroes only).
    pub stress: Option<f64>,
    /// Position in formation.
    pub position: CombatPosition,
    /// Active status IDs.
    pub active_statuses: Vec<String>,
    /// Active buff IDs.
    pub active_buffs: Vec<String>,
    /// Active debuff IDs.
    pub active_debuffs: Vec<String>,
    /// Whether this combatant is dead.
    pub is_dead: bool,
    /// Whether this combatant is at death's door.
    pub is_at_deaths_door: bool,
}

/// Type of combat participant.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CombatantType {
    /// Hero party member.
    Hero,
    /// Monster enemy.
    Monster,
}

/// Combat phase view model.
///
/// This view model represents the state during an active battle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CombatViewModel {
    /// Encounter identifier.
    pub encounter_id: String,
    /// Current round number.
    pub round: u32,
    /// Hero combatants.
    pub heroes: Vec<CombatantViewModel>,
    /// Monster combatants.
    pub monsters: Vec<CombatantViewModel>,
    /// Currently selected actor ID (if any).
    pub selected_actor_id: Option<String>,
    /// Current turn actor ID.
    pub current_turn_actor_id: Option<String>,
    /// Combat phase.
    pub phase: CombatPhase,
    /// Combat result (if combat has ended).
    pub result: Option<CombatResult>,
    /// Error details if combat state has issues.
    pub error: Option<ViewModelError>,
}

/// Combat phase within a battle.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CombatPhase {
    /// Not yet started.
    PreBattle,
    /// Heroes' turn to act.
    HeroTurn,
    /// Monsters' turn to act.
    MonsterTurn,
    /// Resolution phase (applying damage, effects).
    Resolution,
    /// Combat has ended.
    PostBattle,
    /// Unknown phase.
    Unknown,
}

impl CombatPhase {
    /// Parse from a framework combat phase string.
    pub fn from_framework_phase(phase: &str) -> Self {
        match phase.to_lowercase().as_str() {
            "pre_battle" | "prebattle" => CombatPhase::PreBattle,
            "hero_turn" | "heroturn" | "hero" => CombatPhase::HeroTurn,
            "monster_turn" | "monsterturn" | "monster" => CombatPhase::MonsterTurn,
            "resolution" | "resolve" => CombatPhase::Resolution,
            "post_battle" | "postbattle" | "ended" => CombatPhase::PostBattle,
            _ => CombatPhase::Unknown,
        }
    }
}

/// Result of a combat encounter.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CombatResult {
    /// Heroes won the encounter.
    Victory,
    /// Heroes lost the encounter.
    Defeat,
    /// Heroes fled the encounter.
    Fled,
    /// Encounter ended in a draw.
    Draw,
}

impl CombatResult {
    /// Parse from a framework run result string.
    pub fn from_run_result(result: &str) -> Option<Self> {
        match result.to_lowercase().as_str() {
            "victory" | "won" | "success" => Some(CombatResult::Victory),
            "defeat" | "lost" | "failed" => Some(CombatResult::Defeat),
            "fled" | "run" | "escaped" => Some(CombatResult::Fled),
            "draw" | "tie" => Some(CombatResult::Draw),
            _ => None,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Result View Model
// ─────────────────────────────────────────────────────────────────────────────

/// Outcome type for a completed dungeon run or combat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OutcomeType {
    /// Complete success — dungeon cleared or combat won.
    Success,
    /// Partial success — some progress made.
    PartialSuccess,
    /// Failure — party wiped or combat lost.
    Failure,
    /// Party fled.
    Fled,
    /// Aborted (game exit, etc.).
    Aborted,
}

/// Rewards granted from a dungeon run or combat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RewardViewModel {
    /// Gold earned.
    pub gold: u32,
    /// Heirloom currencies earned.
    pub heirlooms: std::collections::BTreeMap<String, u32>,
    /// Experience points earned.
    pub xp: u32,
    /// Loot items acquired.
    pub loot: Vec<String>,
    /// Trinkets acquired.
    pub trinkets: Vec<String>,
}

/// Casualties from a failed dungeon run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CasualtyViewModel {
    /// Hero identifier.
    pub hero_id: String,
    /// Hero class.
    pub class_id: String,
    /// Cause of death (if known).
    pub cause: Option<String>,
}

/// Result phase view model.
///
/// This view model represents the outcome after combat or dungeon completion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResultViewModel {
    /// The type of result.
    pub outcome: OutcomeType,
    /// Human-readable result title.
    pub title: String,
    /// Human-readable result description.
    pub description: String,
    /// Rewards granted (if successful or partial).
    pub rewards: Option<RewardViewModel>,
    /// Casualties (if failure).
    pub casualties: Vec<CasualtyViewModel>,
    /// Dungeon type (if dungeon result).
    pub dungeon_type: Option<String>,
    /// Map size (if dungeon result).
    pub map_size: Option<String>,
    /// Rooms cleared.
    pub rooms_cleared: u32,
    /// Battles won.
    pub battles_won: u32,
    /// Whether this was a completed run (all rooms).
    pub completed: bool,
    /// Error details if result state has issues.
    pub error: Option<ViewModelError>,
}

impl ResultViewModel {
    /// Create a victory result.
    pub fn victory(title: &str, description: &str, rewards: RewardViewModel) -> Self {
        ResultViewModel {
            outcome: OutcomeType::Success,
            title: title.to_string(),
            description: description.to_string(),
            rewards: Some(rewards),
            casualties: Vec::new(),
            dungeon_type: None,
            map_size: None,
            rooms_cleared: 0,
            battles_won: 0,
            completed: false,
            error: None,
        }
    }

    /// Create a defeat result.
    pub fn defeat(title: &str, description: &str, casualties: Vec<CasualtyViewModel>) -> Self {
        ResultViewModel {
            outcome: OutcomeType::Failure,
            title: title.to_string(),
            description: description.to_string(),
            rewards: None,
            casualties,
            dungeon_type: None,
            map_size: None,
            rooms_cleared: 0,
            battles_won: 0,
            completed: false,
            error: None,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Return Flow View Model
// ─────────────────────────────────────────────────────────────────────────────

/// State of the return journey from dungeon to town.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReturnFlowState {
    /// Still in dungeon, not yet returning.
    InDungeon,
    /// Traveling back to town.
    Traveling,
    /// Arriving at town.
    Arriving,
    /// Arrived at town — showing results.
    ShowingResults,
    /// Return flow complete.
    Complete,
    /// Return flow failed (e.g., party wipe during retreat).
    Failed,
}

/// Hero state during return flow.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnFlowHeroViewModel {
    /// Unique hero identifier.
    pub id: String,
    /// Hero class identifier.
    pub class_id: String,
    /// Current health.
    pub health: f64,
    /// Maximum health.
    pub max_health: f64,
    /// Current stress level.
    pub stress: f64,
    /// Maximum stress level.
    pub max_stress: f64,
    /// Whether this hero survived the run.
    pub survived: bool,
    /// Whether this hero died during the run.
    pub died: bool,
    /// Whether this hero is at death's door.
    pub is_at_deaths_door: bool,
}

/// Return flow phase view model.
///
/// This view model represents the state when the party is returning
/// from a dungeon back to town after completing or abandoning a run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnFlowViewModel {
    /// Current return flow state.
    pub state: ReturnFlowState,
    /// Dungeon type being returned from.
    pub dungeon_type: String,
    /// Map size of the run.
    pub map_size: String,
    /// Whether the run was completed (all rooms cleared).
    pub completed: bool,
    /// Rooms cleared before returning.
    pub rooms_cleared: u32,
    /// Battles won before returning.
    pub battles_won: u32,
    /// Gold to be transferred to town.
    pub gold_to_transfer: u32,
    /// Torchlight remaining.
    pub torchlight_remaining: u32,
    /// Heroes during return.
    pub heroes: Vec<ReturnFlowHeroViewModel>,
    /// Result of the dungeon run.
    pub run_result: Option<ResultViewModel>,
    /// Whether the return flow is complete and ready to transition to town.
    pub ready_for_town: bool,
    /// Error details if return flow state has issues.
    pub error: Option<ViewModelError>,
}

impl ReturnFlowViewModel {
    /// Create an in-dungeon state (not yet returning).
    pub fn in_dungeon(dungeon_type: &str, map_size: &str) -> Self {
        ReturnFlowViewModel {
            state: ReturnFlowState::InDungeon,
            dungeon_type: dungeon_type.to_string(),
            map_size: map_size.to_string(),
            completed: false,
            rooms_cleared: 0,
            battles_won: 0,
            gold_to_transfer: 0,
            torchlight_remaining: 100,
            heroes: Vec::new(),
            run_result: None,
            ready_for_town: false,
            error: None,
        }
    }

    /// Check if any heroes died during the run.
    pub fn has_casualties(&self) -> bool {
        self.heroes.iter().any(|h| h.died)
    }

    /// Get the number of surviving heroes.
    pub fn survivors(&self) -> usize {
        self.heroes.iter().filter(|h| h.survived).count()
    }
}