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
    /// Hero display name.
    pub name: String,
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
    /// View model kind identifier.
    pub kind: String,
    /// Human-readable title for the town surface.
    pub title: String,
    /// Campaign name.
    pub campaign_name: String,
    /// Campaign summary/description.
    pub campaign_summary: String,
    /// Current gold.
    pub gold: u32,
    /// Heirloom currency balances.
    pub heirlooms: std::collections::BTreeMap<String, u32>,
    /// Available buildings in town.
    pub buildings: Vec<TownBuildingViewModel>,
    /// Heroes currently on the roster (primary field for frontend).
    pub heroes: Vec<TownHeroViewModel>,
    /// Heroes currently on the roster (alias for heroes).
    pub roster: Vec<TownHeroViewModel>,
    /// Available activities for this visit.
    pub available_activities: Vec<TownActivityType>,
    /// Label for the next action (e.g., "Provision Expedition").
    pub next_action_label: String,
    /// Whether this is a fresh visit (new week) or continuing.
    pub is_fresh_visit: bool,
    /// Error details if town state has issues.
    pub error: Option<ViewModelError>,
}

impl TownViewModel {
    /// Create an empty town view model with default values.
    pub fn empty() -> Self {
        TownViewModel {
            kind: "town".to_string(),
            title: String::new(),
            campaign_name: String::new(),
            campaign_summary: String::new(),
            gold: 0,
            heirlooms: std::collections::BTreeMap::new(),
            buildings: Vec::new(),
            heroes: Vec::new(),
            roster: Vec::new(),
            available_activities: Vec::new(),
            next_action_label: String::new(),
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
// Building Detail View Model
// ─────────────────────────────────────────────────────────────────────────────

/// Building action available in a building detail view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildingAction {
    /// Action identifier.
    pub id: String,
    /// Human-readable action label.
    pub label: String,
    /// Detailed description of what the action does.
    pub description: String,
    /// Cost to perform the action (e.g., "500 Gold").
    pub cost: String,
    /// Whether the action is currently available.
    pub is_available: bool,
    /// Whether this action is unsupported in the current build.
    pub is_unsupported: bool,
}

/// Status of a building in town.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuildingStatus {
    /// Building is available and fully operational.
    Ready,
    /// Building is partially available (some features limited).
    Partial,
    /// Building is locked and not accessible.
    Locked,
}

impl BuildingStatus {
    /// Parse from a string (e.g., "ready", "partial", "locked").
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "ready" => BuildingStatus::Ready,
            "partial" => BuildingStatus::Partial,
            "locked" => BuildingStatus::Locked,
            _ => BuildingStatus::Locked,
        }
    }
}

/// Building detail view model — full building inspection for town interactions.
///
/// This view model represents the detailed state of a single town building,
/// used when the player opens a building to see its actions, costs, and status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildingDetailViewModel {
    /// View model kind identifier.
    pub kind: String,
    /// Building identifier.
    pub building_id: String,
    /// Human-readable building label.
    pub label: String,
    /// Current building status.
    pub status: BuildingStatus,
    /// Detailed description of the building.
    pub description: String,
    /// Available actions in this building.
    pub actions: Vec<BuildingAction>,
    /// Requirement for upgrading this building (if upgradeable).
    pub upgrade_requirement: Option<String>,
}

impl BuildingDetailViewModel {
    /// Create an empty building detail view model.
    pub fn empty() -> Self {
        BuildingDetailViewModel {
            kind: "building-detail".to_string(),
            building_id: String::new(),
            label: String::new(),
            status: BuildingStatus::Locked,
            description: String::new(),
            actions: Vec::new(),
            upgrade_requirement: None,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Hero Detail View Model
// ─────────────────────────────────────────────────────────────────────────────

/// Hero progression information for detail view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeroProgression {
    /// Current level.
    pub level: u32,
    /// Current experience points.
    pub experience: String,
    /// Experience needed for next level.
    pub experience_to_next: String,
}

/// Hero resistances for detail view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeroResistances {
    /// Stun resistance.
    pub stun: String,
    /// Bleed resistance.
    pub bleed: String,
    /// Disease resistance.
    pub disease: String,
    /// Move resistance.
    pub mov: String,
    /// Death resistance.
    pub death: String,
    /// Trap resistance.
    pub trap: String,
    /// Hazard resistance.
    pub hazard: String,
}

/// Hero detail view model — full hero inspection for campaign decisions.
///
/// This view model represents the detailed state of a single hero,
/// used when the player inspects a hero from the roster to make
/// expedition provisioning decisions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeroDetailViewModel {
    /// View model kind identifier.
    pub kind: String,
    /// Unique hero identifier.
    pub hero_id: String,
    /// Hero display name.
    pub name: String,
    /// Hero class display label.
    pub class_label: String,
    /// Current health (formatted string).
    pub hp: String,
    /// Maximum health (formatted string).
    pub max_hp: String,
    /// Current stress level (formatted string).
    pub stress: String,
    /// Resolve level.
    pub resolve: String,
    /// Hero progression information.
    pub progression: HeroProgression,
    /// Hero resistances.
    pub resistances: HeroResistances,
    /// Combat skill IDs.
    pub combat_skills: Vec<String>,
    /// Camping skill IDs.
    pub camping_skills: Vec<String>,
    /// Weapon description.
    pub weapon: String,
    /// Armor description.
    pub armor: String,
    /// Camp notes.
    pub camp_notes: String,
}

impl HeroDetailViewModel {
    /// Create an empty hero detail view model.
    pub fn empty() -> Self {
        HeroDetailViewModel {
            kind: "hero-detail".to_string(),
            hero_id: String::new(),
            name: String::new(),
            class_label: String::new(),
            hp: String::new(),
            max_hp: String::new(),
            stress: String::new(),
            resolve: String::new(),
            progression: HeroProgression {
                level: 0,
                experience: String::new(),
                experience_to_next: String::new(),
            },
            resistances: HeroResistances {
                stun: String::new(),
                bleed: String::new(),
                disease: String::new(),
                mov: String::new(),
                death: String::new(),
                trap: String::new(),
                hazard: String::new(),
            },
            combat_skills: Vec::new(),
            camping_skills: Vec::new(),
            weapon: String::new(),
            armor: String::new(),
            camp_notes: String::new(),
        }
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
// Exploration HUD View Model
// ─────────────────────────────────────────────────────────────────────────────

/// Minimal HUD view model for the exploration shell.
///
/// This view model presents only the essential expedition context needed
/// for the player to understand their current state in the dungeon.
/// It is a lightweight subset of DungeonViewModel optimized for HUD rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExplorationHudViewModel {
    /// Dungeon type identifier.
    pub dungeon_type: String,
    /// Map size variant.
    pub map_size: String,
    /// Current floor number.
    pub floor: u32,
    /// Number of rooms cleared.
    pub rooms_cleared: u32,
    /// Total rooms in the dungeon.
    pub total_rooms: u32,
    /// Gold carried by the party.
    pub gold_carried: u32,
    /// Torchlight level (0-100).
    pub torchlight: u32,
    /// Number of battles won.
    pub battles_won: u32,
    /// Number of battles lost.
    pub battles_lost: u32,
    /// Hero vitals for the HUD (minimal hero state).
    pub hero_vitals: Vec<HeroVitalViewModel>,
    /// Current room kind (if in a room).
    pub current_room_kind: Option<DungeonRoomKind>,
    /// Whether the dungeon is complete.
    pub is_complete: bool,
    /// Error details if any.
    pub error: Option<ViewModelError>,
}

/// Minimal hero vital statistics for HUD display.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeroVitalViewModel {
    /// Unique hero identifier.
    pub id: String,
    /// Hero class identifier.
    pub class_id: String,
    /// Health fraction (0.0 to 1.0).
    pub health_fraction: f64,
    /// Stress fraction (0.0 to 1.0).
    pub stress_fraction: f64,
    /// Whether this hero is at death's door.
    pub is_at_deaths_door: bool,
    /// Whether this hero is dead.
    pub is_dead: bool,
}

impl ExplorationHudViewModel {
    /// Create an empty exploration HUD.
    pub fn empty() -> Self {
        ExplorationHudViewModel {
            dungeon_type: String::new(),
            map_size: String::new(),
            floor: 1,
            rooms_cleared: 0,
            total_rooms: 0,
            gold_carried: 0,
            torchlight: 100,
            battles_won: 0,
            battles_lost: 0,
            hero_vitals: Vec::new(),
            current_room_kind: None,
            is_complete: false,
            error: None,
        }
    }

    /// Check if any hero is at death's door.
    pub fn any_hero_at_deaths_door(&self) -> bool {
        self.hero_vitals.iter().any(|h| h.is_at_deaths_door)
    }

    /// Check if any hero is dead.
    pub fn any_hero_dead(&self) -> bool {
        self.hero_vitals.iter().any(|h| h.is_dead)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Room Movement Transition View Model
// ─────────────────────────────────────────────────────────────────────────────

/// Represents a room movement transition in the dungeon.
///
/// This view model captures the transition from one room to another,
/// surfacing movement and room entry events clearly.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomMovementViewModel {
    /// Previous room ID (if any).
    pub from_room_id: Option<String>,
    /// Previous room kind (if any).
    pub from_room_kind: Option<DungeonRoomKind>,
    /// Current room ID.
    pub to_room_id: String,
    /// Current room kind.
    pub to_room_kind: DungeonRoomKind,
    /// Whether the destination room has been cleared.
    pub is_cleared: bool,
    /// Interaction ID present in the room (curio, trap, etc.).
    pub interaction_id: Option<String>,
    /// Type of interaction in the room.
    pub interaction_type: InteractionType,
}

/// Type of interaction in a room.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InteractionType {
    /// No interaction.
    None,
    /// Curio interaction.
    Curio,
    /// Trap interaction.
    Trap,
    /// Combat encounter.
    Combat,
    /// Boss encounter.
    Boss,
}

impl InteractionType {
    /// Convert from DungeonRoomKind to InteractionType.
    pub fn from_room_kind(kind: &DungeonRoomKind) -> Self {
        match kind {
            DungeonRoomKind::Combat => InteractionType::Combat,
            DungeonRoomKind::Boss => InteractionType::Boss,
            DungeonRoomKind::Event => InteractionType::Curio,
            DungeonRoomKind::Corridor => InteractionType::Trap,
            DungeonRoomKind::Unknown => InteractionType::None,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Encounter Entry View Model
// ─────────────────────────────────────────────────────────────────────────────

/// Represents entering a combat encounter from exploration.
///
/// This view model captures the transition from exploration state
/// into a combat encounter, providing context needed for the frontend
/// to render the encounter entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncounterEntryViewModel {
    /// Encounter identifier.
    pub encounter_id: String,
    /// Room ID where the encounter occurs.
    pub room_id: String,
    /// Encounter type.
    pub encounter_type: EncounterType,
    /// Pack ID for this encounter.
    pub pack_id: String,
    /// Monster family IDs in this encounter.
    pub family_ids: Vec<String>,
    /// Party composition at encounter start.
    pub heroes: Vec<EncounterHeroViewModel>,
    /// Whether this is a boss encounter.
    pub is_boss: bool,
}

/// Type of encounter.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EncounterType {
    /// Regular combat encounter.
    Combat,
    /// Boss encounter.
    Boss,
}

/// Hero state at encounter entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncounterHeroViewModel {
    /// Unique hero identifier.
    pub id: String,
    /// Hero class identifier.
    pub class_id: String,
    /// Health at encounter start.
    pub health: f64,
    /// Maximum health.
    pub max_health: f64,
    /// Stress at encounter start.
    pub stress: f64,
    /// Maximum stress.
    pub max_stress: f64,
    /// Active buff IDs.
    pub active_buffs: Vec<String>,
    /// Whether hero is at death's door.
    pub is_at_deaths_door: bool,
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
// Combat HUD View Model
// ─────────────────────────────────────────────────────────────────────────────

/// Minimal HUD view model for the combat shell.
///
/// This view model presents only the essential combat context needed
/// for the player to understand their current battle state.
/// It is a lightweight subset of CombatViewModel optimized for HUD rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CombatHudViewModel {
    /// Encounter identifier.
    pub encounter_id: String,
    /// Current round number.
    pub round: u32,
    /// Combat phase.
    pub phase: CombatPhase,
    /// Combat result (if combat has ended).
    pub result: Option<CombatResult>,
    /// Current turn actor ID.
    pub current_turn_actor_id: Option<String>,
    /// Hero vitals for the HUD (minimal combatant state).
    pub hero_vitals: Vec<CombatantVitalViewModel>,
    /// Monster vitals for the HUD (minimal combatant state).
    pub monster_vitals: Vec<CombatantVitalViewModel>,
    /// Number of heroes alive.
    pub heroes_alive: u32,
    /// Number of monsters alive.
    pub monsters_alive: u32,
    /// Whether combat is in resolution phase.
    pub is_resolving: bool,
    /// Error details if any.
    pub error: Option<ViewModelError>,
}

/// Minimal combatant vital statistics for HUD display.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CombatantVitalViewModel {
    /// Unique combatant identifier.
    pub id: String,
    /// Combatant type.
    pub combatant_type: CombatantType,
    /// Health fraction (0.0 to 1.0).
    pub health_fraction: f64,
    /// Whether this combatant is at death's door.
    pub is_at_deaths_door: bool,
    /// Whether this combatant is dead.
    pub is_dead: bool,
    /// Active status count.
    pub status_count: usize,
}

impl CombatHudViewModel {
    /// Create an empty combat HUD.
    pub fn empty() -> Self {
        CombatHudViewModel {
            encounter_id: String::new(),
            round: 0,
            phase: CombatPhase::Unknown,
            result: None,
            current_turn_actor_id: None,
            hero_vitals: Vec::new(),
            monster_vitals: Vec::new(),
            heroes_alive: 0,
            monsters_alive: 0,
            is_resolving: false,
            error: None,
        }
    }

    /// Check if combat is active (not ended).
    pub fn is_combat_active(&self) -> bool {
        self.result.is_none() && self.phase != CombatPhase::PostBattle
    }

    /// Check if all heroes are dead.
    pub fn all_heroes_dead(&self) -> bool {
        self.heroes_alive == 0
    }

    /// Check if all monsters are dead.
    pub fn all_monsters_dead(&self) -> bool {
        self.monsters_alive == 0
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Combat Action Input Contracts
// ─────────────────────────────────────────────────────────────────────────────

/// Player action input during combat.
///
/// These contracts represent the player's intent to perform an action
/// during combat, which gets translated into framework combat actions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CombatActionInput {
    /// Attack an enemy target.
    Attack {
        /// Actor ID of the attacker.
        attacker_id: String,
        /// Target position to attack.
        target_position: CombatPosition,
    },
    /// Defend (raise guard).
    Defend {
        /// Actor ID of the defender.
        defender_id: String,
    },
    /// Use a skill on a target.
    UseSkill {
        /// Actor ID of the skill user.
        user_id: String,
        /// Skill identifier.
        skill_id: String,
        /// Target position (if applicable).
        target_position: Option<CombatPosition>,
    },
    /// Use an item.
    UseItem {
        /// Actor ID of the item user.
        user_id: String,
        /// Item identifier.
        item_id: String,
        /// Target position (if applicable).
        target_position: Option<CombatPosition>,
    },
    /// Focus fire on a target.
    FocusFire {
        /// Actor IDs of the attackers.
        attacker_ids: Vec<String>,
        /// Target position to focus.
        target_position: CombatPosition,
    },
    /// Guard redirect - protect an ally.
    Guard {
        /// Actor ID of the guard.
        guard_id: String,
        /// Actor ID of the ally to protect.
        ally_id: String,
    },
    /// Retreat/flee from combat.
    Retreat {
        /// Actor ID of the party member initiating retreat.
        party_member_id: String,
    },
}

impl CombatActionInput {
    /// Returns the actor ID involved in this action (if any).
    pub fn actor_id(&self) -> Option<&str> {
        match self {
            CombatActionInput::Attack { attacker_id, .. } => Some(attacker_id),
            CombatActionInput::Defend { defender_id } => Some(defender_id),
            CombatActionInput::UseSkill { user_id, .. } => Some(user_id),
            CombatActionInput::UseItem { user_id, .. } => Some(user_id),
            CombatActionInput::FocusFire { attacker_ids, .. } => attacker_ids.first().map(|s| s.as_str()),
            CombatActionInput::Guard { guard_id, .. } => Some(guard_id),
            CombatActionInput::Retreat { party_member_id } => Some(party_member_id),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Combat Feedback Contracts
// ─────────────────────────────────────────────────────────────────────────────

/// Feedback event from combat resolution.
///
/// These contracts represent the outcomes of combat actions
/// that need to be displayed to the player.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CombatFeedback {
    /// Damage was dealt.
    DamageDealt {
        /// Target ID that took damage.
        target_id: String,
        /// Amount of damage dealt.
        damage: f64,
        /// Whether this was fatal.
        is_fatal: bool,
        /// Damage type (physical, magic, etc.).
        damage_type: String,
    },
    /// Healing was applied.
    HealingApplied {
        /// Target ID that was healed.
        target_id: String,
        /// Amount of healing applied.
        amount: f64,
    },
    /// Status was applied.
    StatusApplied {
        /// Target ID that received the status.
        target_id: String,
        /// Status identifier.
        status_id: String,
        /// Duration in turns (if applicable).
        duration: Option<u32>,
    },
    /// Status was removed.
    StatusRemoved {
        /// Target ID that lost the status.
        target_id: String,
        /// Status identifier.
        status_id: String,
    },
    /// Combatant died.
    CombatantDied {
        /// Combatant ID that died.
        combatant_id: String,
        /// Combatant type.
        combatant_type: CombatantType,
        /// Cause of death.
        cause: String,
    },
    /// Combatant was revived.
    CombatantRevived {
        /// Combatant ID that was revived.
        combatant_id: String,
        /// Amount of health restored.
        health_restored: f64,
    },
    /// Guard was established.
    GuardEstablished {
        /// Guard ID.
        guard_id: String,
        /// Protected ally ID.
        ally_id: String,
    },
    /// Guard was broken.
    GuardBroken {
        /// Guard ID.
        guard_id: String,
        /// Ally ID that was left unprotected.
        ally_id: String,
    },
    /// Combat round ended.
    RoundEnded {
        /// Round number.
        round: u32,
    },
    /// Combat ended.
    CombatEnded {
        /// Result of the combat.
        result: CombatResult,
    },
}

impl CombatFeedback {
    /// Human-readable description of this feedback event.
    pub fn description(&self) -> String {
        match self {
            CombatFeedback::DamageDealt { target_id, damage, is_fatal, damage_type } => {
                if *is_fatal {
                    format!("{} was fatally wounded by {} {} damage", target_id, damage, damage_type)
                } else {
                    format!("{} took {} {} damage", target_id, damage, damage_type)
                }
            }
            CombatFeedback::HealingApplied { target_id, amount } => {
                format!("{} recovered {} health", target_id, amount)
            }
            CombatFeedback::StatusApplied { target_id, status_id, duration } => {
                if let Some(d) = duration {
                    format!("{} gained {} for {} turns", target_id, status_id, d)
                } else {
                    format!("{} gained {}", target_id, status_id)
                }
            }
            CombatFeedback::StatusRemoved { target_id, status_id } => {
                format!("{} lost {}", target_id, status_id)
            }
            CombatFeedback::CombatantDied { combatant_id, combatant_type, cause } => {
                format!("{:?} {} died: {}", combatant_type, combatant_id, cause)
            }
            CombatFeedback::CombatantRevived { combatant_id, health_restored } => {
                format!("{} was revived with {} health", combatant_id, health_restored)
            }
            CombatFeedback::GuardEstablished { guard_id, ally_id } => {
                format!("{} is guarding {}", guard_id, ally_id)
            }
            CombatFeedback::GuardBroken { guard_id, ally_id } => {
                format!("{} is no longer guarding {}", guard_id, ally_id)
            }
            CombatFeedback::RoundEnded { round } => {
                format!("Round {} ended", round)
            }
            CombatFeedback::CombatEnded { result } => {
                format!("Combat ended: {:?}", result)
            }
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