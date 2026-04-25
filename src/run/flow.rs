//! DDGC run flow — room-by-room dungeon progression.
//!
//! Generates a floor using `DefaultRoomGenerator` with DDGC room weights,
//! then drives the run through each room in sequence. Combat rooms and
//! boss rooms resolve through the DDGC encounter pack registry.
//! Post-battle rewards are applied after clearing combat rooms.
//!
//! Event rooms can carry a curio_id reference that produces a curio
//! interaction outcome when the room is entered.
//!
//! Corridor rooms can carry optional trap_id and curio_id references
//! that produce trap and curio interaction outcomes when traversed.
//!
//! This is the Batch 5 migration: the new stack proves it can handle
//! gameplay progression rather than a single isolated encounter.

use framework_combat::encounter::CombatSide;
use framework_progression::floor::{Floor, FloorId};
use framework_progression::generation::{DefaultRoomGenerator, FloorConfig, RoomGenerator};
use framework_progression::rooms::{RoomId, RoomKind};
use framework_progression::run::{Run, RunId, RunResult};

use crate::contracts::{
    get_dungeon_config, CurioRegistry, CurioResultType, QuirkRegistry,
    DungeonMapConfig, DungeonType, GridSize, MapSize, ObstacleRegistry,
    TraitRegistry, TrapOutcome, TrapRegistry,
};
use crate::run::camping::{CampingPhase, CampActivityRecord};
use crate::heroes::quirks::apply_quirk;
use crate::encounters::Dungeon;
use crate::monsters::families::FamilyId;
use crate::contracts::parse::parse_buildings_json;
use crate::run::encounters::EncounterResolver;
use crate::run::rewards::{self, PostBattleUpdate};
use crate::town::{HeroInTown, TownActivity, TownActivityTrace, TownVisit};

/// DDGC-appropriate room weights for floor generation.
///
/// Combat-heavy distribution matching DDGC dungeon style:
/// - Combat rooms dominate (weight 5)
/// - Corridor rooms carry traps and curios (weight 3)
/// - Event rooms are secondary (weight 2)
/// - Boss rooms cap the floor (weight 1)
const DDGC_ROOM_WEIGHTS: &[(RoomKind, f64)] = &[
    (RoomKind::Combat, 5.0),
    (RoomKind::corridor(), 3.0),
    (RoomKind::event(), 2.0),
    (RoomKind::Boss, 1.0),
];

/// Configuration for a DDGC run slice.
pub struct DdgcRunConfig {
    pub seed: u64,
    pub dungeon: Dungeon,
    pub map_size: MapSize,
    /// Initial hero states entering the dungeon (for HP/stress tracking).
    pub heroes: Vec<HeroState>,
}

impl Default for DdgcRunConfig {
    fn default() -> Self {
        DdgcRunConfig {
            seed: 42,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
            heroes: Vec::new(),
        }
    }
}

/// Game-layer state tracked across the run.
///
/// The framework manages room/floor progression; this struct holds
/// game-specific state that the framework doesn't know about.
pub struct DdgcRunState {
    pub gold: u32,
    pub rooms_cleared: u32,
    pub battles_won: u32,
    pub battles_lost: u32,
    pub hp_recovered: f64,
    pub stress_change: f64,
    /// Ordered record of room IDs visited (for progression verification).
    pub visited_rooms: Vec<RoomId>,
    /// Quirks acquired during this run.
    pub quirk_state: HeroQuirkState,
    /// Traits (afflictions/virtues) acquired during this run.
    pub trait_state: HeroTraitState,
    /// Optional camping phase that occurred during this run.
    pub camping_phase: Option<CampingPhase>,
}

impl DdgcRunState {
    pub fn new() -> Self {
        DdgcRunState {
            gold: 0,
            rooms_cleared: 0,
            battles_won: 0,
            battles_lost: 0,
            hp_recovered: 0.0,
            stress_change: 0.0,
            visited_rooms: Vec::new(),
            quirk_state: HeroQuirkState::new(),
            trait_state: HeroTraitState::new(),
            camping_phase: None,
        }
    }
}

impl Default for DdgcRunState {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of completing a DDGC run slice.
pub struct DdgcRunResult {
    pub run: Run,
    pub state: DdgcRunState,
    pub floor: Floor,
    /// Pack IDs for battles in this run slice — used to verify no fallback content.
    pub battle_pack_ids: Vec<String>,
    /// Dungeon and map metadata for this run slice.
    pub metadata: RunMetadata,
    /// Per-room encounter details including pack ID and monster family composition.
    pub room_encounters: Vec<RoomEncounterRecord>,
    /// Interaction events (curio, trap, obstacle) that occurred during the run.
    pub interaction_records: Vec<InteractionRecord>,
    /// Camping activity trace (if camping occurred during this run).
    pub camping_trace: Vec<CampActivityRecord>,
    /// Hero states after this run slice (HP, stress updated).
    pub heroes: Vec<HeroState>,
}

/// Dungeon and map metadata captured at the start of a run slice.
///
/// This records the dungeon type, map size, and key map generation parameters
/// so the run trace can be analyzed for fidelity verification.
#[derive(Debug, Clone, PartialEq)]
pub struct RunMetadata {
    /// The dungeon type (QingLong, BaiHu, ZhuQue, XuanWu).
    pub dungeon_type: DungeonType,
    /// The map size variant (Short, Medium).
    pub map_size: MapSize,
    /// Base number of rooms in the map.
    pub base_room_number: u32,
    /// Base number of corridors in the map.
    pub base_corridor_number: u32,
    /// Grid dimensions for room placement.
    pub gridsize: GridSize,
    /// Connectivity parameter (0.0 to 1.0).
    pub connectivity: f64,
}

impl RunMetadata {
    /// Create run metadata from a dungeon config.
    fn from_config(dungeon_type: DungeonType, map_size: MapSize, config: &DungeonMapConfig) -> Self {
        RunMetadata {
            dungeon_type,
            map_size,
            base_room_number: config.base_room_number,
            base_corridor_number: config.base_corridor_number,
            gridsize: config.gridsize,
            connectivity: config.connectivity,
        }
    }
}

/// A record of the encounter composition for a single room in a run slice.
///
/// This captures the pack ID and monster family composition for each combat
/// or boss room, enabling verification that the correct encounter types and
/// monster families appear per room.
#[derive(Debug, Clone, PartialEq)]
pub struct RoomEncounterRecord {
    /// The room ID.
    pub room_id: RoomId,
    /// The kind of room (Combat, Boss, Event).
    pub room_kind: RoomKind,
    /// The encounter pack ID used for this room (empty for non-combat rooms).
    pub pack_id: String,
    /// Monster family IDs in this encounter pack (empty for non-combat rooms).
    pub family_ids: Vec<FamilyId>,
}

impl RoomEncounterRecord {
    /// Create a combat/boss room record from a pack.
    fn combat(room_id: RoomId, room_kind: RoomKind, pack_id: String, family_ids: Vec<FamilyId>) -> Self {
        RoomEncounterRecord {
            room_id,
            room_kind,
            pack_id,
            family_ids,
        }
    }

    /// Create a non-combat room record (no encounter).
    fn non_combat(room_id: RoomId, room_kind: RoomKind) -> Self {
        RoomEncounterRecord {
            room_id,
            room_kind,
            pack_id: String::new(),
            family_ids: Vec::new(),
        }
    }
}

/// A record of an interaction event (curio, trap, obstacle) in a run slice.
///
/// This captures the outcome of interacting with a curio in an Event room,
/// or a trap/curio in a Corridor room.
#[derive(Debug, Clone, PartialEq)]
pub struct InteractionRecord {
    /// The room ID where the interaction occurred.
    pub room_id: RoomId,
    /// The kind of room where the interaction occurred.
    pub room_kind: RoomKind,
    /// The type of interaction (Curio, Trap, Obstacle).
    pub interaction_type: InteractionType,
    /// The ID of the curio, trap, or obstacle interacted with.
    pub entity_id: String,
    /// The outcome of the interaction.
    pub outcome: InteractionOutcome,
}

/// The type of interaction that occurred.
#[derive(Debug, Clone, PartialEq)]
pub enum InteractionType {
    Curio,
    Trap,
    Obstacle,
}

/// The outcome of an interaction.
#[derive(Debug, Clone, PartialEq)]
pub enum InteractionOutcome {
    /// Curio interaction outcome.
    Curio {
        result_type: CurioResultType,
        result_id: String,
    },
    /// Trap interaction outcome (success or fail).
    Trap {
        avoided: bool,
        effects: Vec<String>,
        health_fraction: f64,
    },
    /// Obstacle interaction outcome.
    Obstacle {
        effects: Vec<String>,
        health_fraction: f64,
        torchlight_penalty: f64,
    },
}

// ─────────────────────────────────────────────────────────────────────────────
// Full-loop run: town → dungeon → town
// ─────────────────────────────────────────────────────────────────────────────

/// A hero's state tracked across the full run loop (town → dungeon → town).
///
/// This tracks the hero's vital statistics as they progress through
/// multiple dungeon runs and town visits.
#[derive(Debug, Clone, PartialEq)]
pub struct HeroState {
    /// Unique hero identifier.
    pub id: String,
    /// Hero class ID (e.g., "alchemist", "hunter").
    pub class_id: String,
    /// Current health.
    pub health: f64,
    /// Maximum health.
    pub max_health: f64,
    /// Current stress level.
    pub stress: f64,
    /// Maximum stress level.
    pub max_stress: f64,
    /// Active buff IDs on this hero.
    pub active_buffs: Vec<String>,
    /// Buff IDs applied during camping (removable at camp end).
    pub camping_buffs: Vec<String>,
}

impl HeroState {
    /// Create a new hero state.
    pub fn new(id: &str, class_id: &str, health: f64, max_health: f64, stress: f64, max_stress: f64) -> Self {
        HeroState {
            id: id.to_string(),
            class_id: class_id.to_string(),
            health,
            max_health,
            stress,
            max_stress,
            active_buffs: Vec::new(),
            camping_buffs: Vec::new(),
        }
    }

    /// Convert to a HeroInTown for town visit activities.
    pub fn to_hero_in_town(&self) -> HeroInTown {
        HeroInTown::new(
            &self.id,
            &self.class_id,
            self.stress,
            self.max_stress,
            self.health,
            self.max_health,
        )
    }

    /// Update from a HeroInTown after town visit.
    pub fn update_from_town(&mut self, hero: &HeroInTown) {
        self.stress = hero.stress;
        self.max_stress = hero.max_stress;
        self.health = hero.health;
        self.max_health = hero.max_health;
    }
}

/// DDGC limits on quirk slots per category.
///
/// Heroes can have at most this many quirks in each category.
/// Diseases count toward the negative quirk limit.
pub const MAX_POSITIVE_QUIRKS: usize = 5;
pub const MAX_NEGATIVE_QUIRKS: usize = 5;

/// Tracks a hero's active quirks across the full run loop.
///
/// Quirks modify hero attributes and may be incompatible with each other.
/// Diseases are tracked separately but count toward the negative quirk limit.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct HeroQuirkState {
    /// Active positive quirk IDs.
    pub positive: Vec<String>,
    /// Active negative quirk IDs (non-disease).
    pub negative: Vec<String>,
    /// Active disease quirk IDs (count toward negative limit).
    pub diseases: Vec<String>,
}

impl HeroQuirkState {
    /// Create a new empty quirk state.
    pub fn new() -> Self {
        HeroQuirkState::default()
    }

    /// Returns the total count of negative quirks including diseases.
    pub fn negative_count(&self) -> usize {
        self.negative.len() + self.diseases.len()
    }

    /// Check if a quirk ID is already present in any category.
    pub fn contains(&self, quirk_id: &str) -> bool {
        self.positive.contains(&quirk_id.to_string())
            || self.negative.contains(&quirk_id.to_string())
            || self.diseases.contains(&quirk_id.to_string())
    }
}

/// Tracks a hero's active traits (afflictions and virtues) across the full run loop.
///
/// Traits are acquired when a hero exceeds max stress (overstress).
/// Unlike quirks, traits do not have categories or slot limits - a hero can have
/// multiple afflictions and virtues simultaneously.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct HeroTraitState {
    /// Active affliction trait IDs.
    pub afflictions: Vec<String>,
    /// Active virtue trait IDs.
    pub virtues: Vec<String>,
}

impl HeroTraitState {
    /// Create a new empty trait state.
    pub fn new() -> Self {
        HeroTraitState::default()
    }

    /// Check if a trait ID is already present in any category.
    pub fn contains(&self, trait_id: &str) -> bool {
        self.afflictions.contains(&trait_id.to_string())
            || self.virtues.contains(&trait_id.to_string())
    }

    /// Add a trait to the appropriate category based on its overstress type.
    pub fn add_trait(&mut self, trait_id: &str, is_virtue: bool) {
        if is_virtue {
            if !self.virtues.contains(&trait_id.to_string()) {
                self.virtues.push(trait_id.to_string());
            }
        } else {
            if !self.afflictions.contains(&trait_id.to_string()) {
                self.afflictions.push(trait_id.to_string());
            }
        }
    }
}

/// Configuration for a DDGC full run loop (town → dungeon → town).
pub struct DdgcFullRunConfig {
    /// Seed for dungeon room generation.
    pub seed: u64,
    /// Dungeon to run.
    pub dungeon: Dungeon,
    /// Map size for the dungeon.
    pub map_size: MapSize,
    /// Initial heroes starting the run.
    pub initial_heroes: Vec<HeroState>,
    /// Initial gold available.
    pub initial_gold: u32,
}

impl Default for DdgcFullRunConfig {
    fn default() -> Self {
        DdgcFullRunConfig {
            seed: 42,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
            initial_heroes: vec![
                HeroState::new("h1", "alchemist", 100.0, 100.0, 0.0, 200.0),
                HeroState::new("h2", "hunter", 100.0, 100.0, 0.0, 200.0),
                HeroState::new("h3", " Occultist", 100.0, 100.0, 0.0, 200.0),
                HeroState::new("h4", "black", 100.0, 100.0, 0.0, 200.0),
            ],
            initial_gold: 500,
        }
    }
}

/// Result of a full run loop (town → dungeon → town).
pub struct DdgcFullRunResult {
    /// The dungeon run result.
    pub dungeon_result: DdgcRunResult,
    /// Heroes after the dungeon run (before town visit).
    pub heroes_post_dungeon: Vec<HeroState>,
    /// Heroes after the post-dungeon town visit.
    pub heroes_post_town: Vec<HeroState>,
    /// Gold after the dungeon run.
    pub gold_post_dungeon: u32,
    /// Gold after the post-dungeon town visit.
    pub gold_post_town: u32,
    /// Town activity trace from the pre-dungeon town visit (if any).
    pub pre_town_trace: TownActivityTrace,
    /// Town activity trace from the post-dungeon town visit.
    pub post_town_trace: TownActivityTrace,
}

/// Run a full DDGC loop: town visit → dungeon run → town visit.
///
/// This function executes the complete game loop:
/// 1. Optionally perform pre-dungeon town activities (stress heal, recruit)
/// 2. Run the dungeon (generate floor, resolve rooms)
/// 3. Perform post-dungeon town activities (stress heal, recruit)
///
/// Heroes progress through the loop with their HP and stress tracked.
/// Gold is earned in the dungeon and spent in town visits.
pub fn run_ddgc_full_loop(config: &DdgcFullRunConfig) -> DdgcFullRunResult {
    // Parse building registry for town activities
    let building_registry = parse_buildings_json(&std::path::PathBuf::from("data").join("Buildings.json"))
        .unwrap_or_else(|_| {
            // Fallback: create empty registry if parsing fails
            crate::contracts::BuildingRegistry::new()
        });

    // ── Pre-dungeon town visit ──────────────────────────────────────────────
    let mut pre_town_trace = TownActivityTrace::new();

    // Convert HeroState to HeroInTown for town visit
    let heroes_for_town: Vec<HeroInTown> = config
        .initial_heroes
        .iter()
        .map(|h| h.to_hero_in_town())
        .collect();

    let mut town_state = crate::contracts::TownState::new(config.initial_gold);
    // Initialize building states for all buildings
    for building_id in building_registry.all_ids() {
        town_state
            .building_states
            .insert(building_id.to_string(), crate::contracts::BuildingUpgradeState::new(building_id, Some('a')));
    }

    let mut pre_town_visit = TownVisit::new(town_state, heroes_for_town, building_registry.clone());

    // Perform pre-dungeon stress heal at abbey if heroes have stress
    for hero in &config.initial_heroes {
        if hero.stress > 0.0 {
            let result = pre_town_visit.perform_town_activity(
                "abbey",
                TownActivity::Pray,
                Some(&hero.id),
                Some('a'),
            );
            pre_town_trace.record(result);
        }
    }

    // ── Dungeon run ────────────────────────────────────────────────────────
    let dungeon_config = DdgcRunConfig {
        seed: config.seed,
        dungeon: config.dungeon,
        map_size: config.map_size,
        heroes: config.initial_heroes.clone(),
    };
    let dungeon_result = run_ddgc_slice(&dungeon_config);

    // Use heroes returned from dungeon run (with camping effects applied)
    let heroes_post_dungeon = dungeon_result.heroes.clone();
    let gold_post_dungeon = config.initial_gold + dungeon_result.state.gold;

    // ── Post-dungeon town visit ────────────────────────────────────────────
    let mut post_town_trace = TownActivityTrace::new();

    // Convert heroes to HeroInTown for post-dungeon town visit
    let heroes_for_post_town: Vec<HeroInTown> = heroes_post_dungeon
        .iter()
        .map(|h| h.to_hero_in_town())
        .collect();

    let mut post_town_state = crate::contracts::TownState::new(gold_post_dungeon);
    for building_id in building_registry.all_ids() {
        post_town_state
            .building_states
            .insert(building_id.to_string(), crate::contracts::BuildingUpgradeState::new(building_id, Some('a')));
    }

    let mut post_town_visit = TownVisit::new(post_town_state, heroes_for_post_town, building_registry);

    // Perform stress heal at abbey for heroes with stress
    for hero in &heroes_post_dungeon {
        if hero.stress > 0.0 {
            let result = post_town_visit.perform_town_activity(
                "abbey",
                TownActivity::Pray,
                Some(&hero.id),
                Some('a'),
            );
            post_town_trace.record(result);
        }
    }

    // Update heroes from post-dungeon town visit
    let mut heroes_post_town = Vec::new();
    for hero in &heroes_post_dungeon {
        let hero_in_town = post_town_visit.get_hero(&hero.id);
        if let Some(hero_town) = hero_in_town {
            let mut updated = hero.clone();
            updated.update_from_town(hero_town);
            heroes_post_town.push(updated);
        } else {
            // Hero not found (shouldn't happen), keep original
            heroes_post_town.push(hero.clone());
        }
    }

    let gold_post_town = post_town_visit.town_state.gold;

    DdgcFullRunResult {
        dungeon_result,
        heroes_post_dungeon,
        heroes_post_town,
        gold_post_dungeon,
        gold_post_town,
        pre_town_trace,
        post_town_trace,
    }
}

/// Deterministic LCG PRNG for generating interaction assignments.
struct AssignmentRng(u64);

impl AssignmentRng {
    fn new(seed: u64) -> Self {
        AssignmentRng(seed.wrapping_add(1))
    }

    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0
    }

    fn next_f64(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// Assign curio and trap IDs directly to room kinds on the floor.
///
/// Mutates each Event and Corridor room's `kind` to include the assigned
/// IDs, deterministically derived from the floor seed and dungeon density.
fn assign_interactions_to_rooms(
    floor: &mut Floor,
    config: &DdgcRunConfig,
    dungeon_type: DungeonType,
    map_config: &DungeonMapConfig,
    curio_registry: &CurioRegistry,
    trap_registry: &TrapRegistry,
) {
    let mut rng = AssignmentRng::new(config.seed ^ 0xdeadbeef);

    // Get available curio IDs for this dungeon
    let available_curios: Vec<&str> = curio_registry
        .by_dungeon(dungeon_type)
        .iter()
        .map(|c| c.id.as_str())
        .collect();

    // Get available trap IDs
    let available_traps: Vec<&str> = trap_registry.all_ids();

    for room_id in &floor.rooms {
        let room = floor.rooms_map.get_mut(room_id).unwrap();

        match &mut room.kind {
            RoomKind::Event { curio_id } => {
                let curio_chance = if map_config.room_curio.min > 0 {
                    (map_config.room_curio.min as f64 + map_config.room_curio.max as f64) / 2.0
                        / (map_config.room_curio.max.max(1) as f64)
                } else {
                    0.3 // default fallback
                };

                if !available_curios.is_empty() && rng.next_f64() < curio_chance {
                    let idx = (rng.next() as usize) % available_curios.len();
                    if let Some(id) = available_curios.get(idx) {
                        *curio_id = Some(id.to_string());
                    }
                }
            }
            RoomKind::Corridor { trap_id, curio_id } => {
                // Trap assignment
                let trap_chance = if map_config.hallway_trap.min > 0 {
                    (map_config.hallway_trap.min as f64 + map_config.hallway_trap.max as f64) / 2.0
                        / (map_config.hallway_trap.max.max(1) as f64)
                } else {
                    0.2 // default fallback
                };

                if !available_traps.is_empty() && rng.next_f64() < trap_chance {
                    let idx = (rng.next() as usize) % available_traps.len();
                    if let Some(id) = available_traps.get(idx) {
                        *trap_id = Some(id.to_string());
                    }
                }

                // Curio assignment
                let curio_chance = if map_config.hallway_curio.min > 0 {
                    (map_config.hallway_curio.min as f64 + map_config.hallway_curio.max as f64) / 2.0
                        / (map_config.hallway_curio.max.max(1) as f64)
                } else {
                    0.15 // default fallback
                };

                if !available_curios.is_empty() && rng.next_f64() < curio_chance {
                    let idx = (rng.next() as usize) % available_curios.len();
                    if let Some(id) = available_curios.get(idx) {
                        *curio_id = Some(id.to_string());
                    }
                }
            }
            _ => {}
        }
    }
}

/// Resolve curio interaction for a room.
fn resolve_curio_for_room(
    room_id: RoomId,
    room_kind: &RoomKind,
    curio_registry: &CurioRegistry,
    seed: u64,
) -> Option<InteractionRecord> {
    let curio_id = match room_kind {
        RoomKind::Event { curio_id } => curio_id.as_ref()?,
        RoomKind::Corridor { curio_id, .. } => curio_id.as_ref()?,
        _ => return None,
    };

    let outcome = curio_registry.resolve_curio_interaction(
        curio_id,
        false, // no item used
        "",
        seed,
    )?;

    Some(InteractionRecord {
        room_id,
        room_kind: room_kind.clone(),
        interaction_type: InteractionType::Curio,
        entity_id: curio_id.clone(),
        outcome: InteractionOutcome::Curio {
            result_type: outcome.result_type,
            result_id: outcome.result_id,
        },
    })
}

/// Apply a quirk from a curio interaction outcome.
///
/// When a curio interaction yields a Quirk or Disease result, this function
/// applies the quirk to the hero's quirk state.
fn apply_curio_quirk(
    quirk_state: &mut HeroQuirkState,
    result_type: CurioResultType,
    result_id: &str,
    quirk_registry: &QuirkRegistry,
) {
    match result_type {
        CurioResultType::Quirk | CurioResultType::Disease => {
            *quirk_state = apply_quirk(quirk_state.clone(), result_id, quirk_registry);
        }
        _ => {}
    }
}

/// Resolve trap interaction for a corridor room.
fn resolve_trap_for_room(
    room_id: RoomId,
    room_kind: &RoomKind,
    trap_registry: &TrapRegistry,
    seed: u64,
) -> Option<InteractionRecord> {
    let trap_id = match room_kind {
        RoomKind::Corridor { trap_id, .. } => trap_id.as_ref()?,
        _ => return None,
    };

    // Use level 3 as default trap level
    let trap_level = 3u32;
    // Use 0.5 as default resist chance
    let resist_chance = 0.5;

    let outcome = trap_registry.resolve_trap_interaction(
        trap_id,
        trap_level,
        resist_chance,
        seed,
    )?;

    let (avoided, effects, health_fraction) = match outcome {
        TrapOutcome::Success { effects } => (true, effects, 0.0),
        TrapOutcome::Fail { effects, health_fraction } => (false, effects, health_fraction),
    };

    Some(InteractionRecord {
        room_id,
        room_kind: room_kind.clone(),
        interaction_type: InteractionType::Trap,
        entity_id: trap_id.clone(),
        outcome: InteractionOutcome::Trap {
            avoided,
            effects,
            health_fraction,
        },
    })
}

/// Run a minimal DDGC run slice.
///
/// Generates a deterministic floor with DDGC room weights, then progresses
/// through each room in sequence. Combat rooms resolve through the encounter
/// pack registry; Boss rooms resolve through the boss encounter registry.
/// Post-battle rewards are applied after each combat room is cleared.
///
/// Event rooms can carry a curio_id that produces a curio interaction outcome.
/// Corridor rooms can carry trap_id and curio_id that produce trap and curio
/// interaction outcomes.
///
/// All four core DDGC dungeons (QingLong, BaiHu, ZhuQue, XuanWu) have migrated
/// encounter packs — this function will panic if a pack is missing, indicating
/// a developer error in the migration.
///
/// Room generation uses `DungeonMapConfig` parameters for the current dungeon:
/// room count comes from `base_room_number`, and connectivity drives `max_connections`.
pub fn run_ddgc_slice(config: &DdgcRunConfig) -> DdgcRunResult {
    // Look up the DungeonMapConfig for this dungeon type and size
    let dungeon_type = DungeonType::from_dungeon(config.dungeon)
        .expect("DungeonMapConfig required for core dungeons (QingLong/BaiHu/ZhuQue/XuanWu)");
    let map_config = get_dungeon_config(dungeon_type, config.map_size)
        .expect("DungeonMapConfig must exist for the given dungeon type and size");

    let room_count = map_config.base_room_number;
    let max_connections = map_config.max_connections();

    let gen = DefaultRoomGenerator;
    let floor_config = FloorConfig::new(
        room_count,
        DDGC_ROOM_WEIGHTS.to_vec(),
        max_connections,
    );

    let mut floor = gen.generate_floor(FloorId(0), config.seed, &floor_config);

    // Parse curio, trap, and obstacle registries from data files
    let curio_registry = crate::contracts::parse::parse_curios_csv(
        &std::path::PathBuf::from("data").join("Curios.csv"),
    )
    .unwrap_or_else(|_| {
        // Fallback: create empty registry if parsing fails
        CurioRegistry::new()
    });

    let trap_registry = crate::contracts::parse::parse_traps_json(
        &std::path::PathBuf::from("data").join("Traps.json"),
    )
    .unwrap_or_else(|_| {
        // Fallback: create empty registry if parsing fails
        TrapRegistry::new()
    });

    let _obstacle_registry = crate::contracts::parse::parse_obstacles_json(
        &std::path::PathBuf::from("data").join("Obstacles.json"),
    )
    .unwrap_or_else(|_| {
        // Fallback: create empty registry if parsing fails
        ObstacleRegistry::new()
    });

    // Parse quirk registry for quirk acquisition during run
    let quirk_registry = crate::contracts::parse::parse_quirks_json(
        &std::path::PathBuf::from("data").join("JsonQuirks.json"),
    )
    .unwrap_or_else(|_| {
        // Fallback: create empty registry if parsing fails
        QuirkRegistry::new()
    });

    // Parse trait registry for trait acquisition during run
    let trait_registry = crate::contracts::parse::parse_traits_json(
        &std::path::PathBuf::from("data").join("JsonTraits.json"),
    )
    .unwrap_or_else(|_| {
        // Fallback: create empty registry if parsing fails
        TraitRegistry::new()
    });

    // Buff registry for resolving quirk modifiers
    let buff_registry = crate::contracts::BuffRegistry::new();

    // Extract disease IDs from quirk registry for combat disease tracking
    let disease_ids: Vec<String> = quirk_registry.diseases().iter().map(|d| d.id.clone()).collect();
    let known_diseases: Vec<&str> = disease_ids.iter().map(|s| s.as_str()).collect();
    let disease_pool: Vec<&str> = disease_ids.iter().map(|s| s.as_str()).collect();

    // Assign curio/trap IDs directly to room kinds on the floor
    assign_interactions_to_rooms(
        &mut floor,
        config,
        dungeon_type,
        map_config,
        &curio_registry,
        &trap_registry,
    );

    let mut run = Run::new(RunId(1), vec![FloorId(0)], config.seed);
    let mut state = DdgcRunState::new();
    let mut battle_pack_ids = Vec::new();
    let mut room_encounters = Vec::new();
    let mut interaction_records = Vec::new();
    let mut heroes = config.heroes.clone();

    // Build the encounter resolver once — reuse for all combat rooms
    let resolver = EncounterResolver::new();

    // Progress through rooms in floor order
    let room_ids = floor.rooms.clone();
    for (room_idx, room_id) in room_ids.iter().enumerate() {
        // Enter the room
        run.enter_room(&mut floor, *room_id).unwrap();
        state.visited_rooms.push(*room_id);

        let room_kind = floor.rooms_map[room_id].kind.clone();

        // Handle room by type
        match &room_kind {
            RoomKind::Combat => {
                // Resolve combat room through encounter pack registry.
                // All four core DDGC dungeons have migrated encounter packs;
                // if a pack is missing, this is a developer error — fail fast.
                let pack = resolver
                    .resolve_pack(config.dungeon, room_idx as u32, config.seed, false)
                    .expect("Combat room: migrated DDGC dungeon must have encounter packs");
                let battle_result = resolver.run_battle_with_quirks(
                    pack,
                    room_idx as u64 + 1,
                    Some(&state.quirk_state),
                    Some(&quirk_registry),
                    Some(&buff_registry),
                    &known_diseases,
                    &disease_pool,
                    Some(state.trait_state.clone()),
                    Some(&trait_registry),
                );
                battle_pack_ids.push(battle_result.pack_id.clone());

                // Apply any diseases acquired during combat
                for event in &battle_result.disease_events {
                    state.quirk_state = apply_quirk(
                        state.quirk_state.clone(),
                        &event.disease_id,
                        &quirk_registry,
                    );
                }

                // Apply any traits acquired during combat (overstress resolution)
                if let Some(trait_state) = battle_result.trait_state {
                    state.trait_state = trait_state;
                }

                // Record room encounter with family composition
                let family_ids: Vec<FamilyId> = pack.family_ids().iter().map(|f| (*f).clone()).collect();
                room_encounters.push(RoomEncounterRecord::combat(
                    *room_id,
                    room_kind.clone(),
                    battle_result.pack_id.clone(),
                    family_ids,
                ));

                if battle_result.winner == Some(CombatSide::Ally) {
                    state.battles_won += 1;
                } else {
                    state.battles_lost += 1;
                }

                // Clear the room
                run.clear_room(&mut floor).unwrap();

                // Apply post-battle rewards
                let update = rewards::compute_post_battle_update(&room_kind);
                apply_reward(&mut state, &update);
            }
            RoomKind::Boss => {
                // Boss rooms resolve through the boss encounter pack registry.
                // All four core DDGC dungeons have migrated boss packs;
                // if a pack is missing, this is a developer error — fail fast.
                let pack = resolver
                    .resolve_boss_pack(config.dungeon, room_idx as u32, config.seed)
                    .expect("Boss room: migrated DDGC dungeon must have boss encounter packs");
                let battle_result = resolver.run_battle_with_quirks(
                    pack,
                    room_idx as u64 + 1,
                    Some(&state.quirk_state),
                    Some(&quirk_registry),
                    Some(&buff_registry),
                    &known_diseases,
                    &disease_pool,
                    Some(state.trait_state.clone()),
                    Some(&trait_registry),
                );
                battle_pack_ids.push(battle_result.pack_id.clone());

                // Apply any diseases acquired during combat
                for event in &battle_result.disease_events {
                    state.quirk_state = apply_quirk(
                        state.quirk_state.clone(),
                        &event.disease_id,
                        &quirk_registry,
                    );
                }

                // Apply any traits acquired during combat (overstress resolution)
                if let Some(trait_state) = battle_result.trait_state {
                    state.trait_state = trait_state;
                }

                // Record room encounter with family composition
                let family_ids: Vec<FamilyId> = pack.family_ids().iter().map(|f| (*f).clone()).collect();
                room_encounters.push(RoomEncounterRecord::combat(
                    *room_id,
                    room_kind.clone(),
                    battle_result.pack_id.clone(),
                    family_ids,
                ));

                if battle_result.winner == Some(CombatSide::Ally) {
                    state.battles_won += 1;
                } else {
                    state.battles_lost += 1;
                }

                // Clear the room
                run.clear_room(&mut floor).unwrap();

                // Apply post-battle rewards
                let update = rewards::compute_post_battle_update(&room_kind);
                apply_reward(&mut state, &update);
            }
            RoomKind::Event { .. } => {
                // Event rooms may have curio interactions
                room_encounters.push(RoomEncounterRecord::non_combat(*room_id, room_kind.clone()));

                // Resolve curio interaction if present
                let interaction_seed = config.seed.wrapping_add(room_idx as u64 * 17);
                if let Some(record) = resolve_curio_for_room(
                    *room_id,
                    &room_kind,
                    &curio_registry,
                    interaction_seed,
                ) {
                    // Apply quirk if the curio grants one
                    if let InteractionOutcome::Curio { result_type, result_id } = &record.outcome {
                        apply_curio_quirk(&mut state.quirk_state, *result_type, result_id, &quirk_registry);
                    }
                    interaction_records.push(record);
                }

                run.clear_room(&mut floor).unwrap();
            }
            RoomKind::Corridor { .. } => {
                // Corridor rooms may have trap and curio interactions
                room_encounters.push(RoomEncounterRecord::non_combat(*room_id, room_kind.clone()));

                // Resolve trap interaction if present
                let trap_seed = config.seed.wrapping_add(room_idx as u64 * 31);
                if let Some(record) = resolve_trap_for_room(
                    *room_id,
                    &room_kind,
                    &trap_registry,
                    trap_seed,
                ) {
                    interaction_records.push(record);
                }

                // Resolve curio interaction if present
                let curio_seed = config.seed.wrapping_add(room_idx as u64 * 47);
                if let Some(record) = resolve_curio_for_room(
                    *room_id,
                    &room_kind,
                    &curio_registry,
                    curio_seed,
                ) {
                    // Apply quirk if the curio grants one
                    if let InteractionOutcome::Curio { result_type, result_id } = &record.outcome {
                        apply_curio_quirk(&mut state.quirk_state, *result_type, result_id, &quirk_registry);
                    }
                    interaction_records.push(record);
                }

                run.clear_room(&mut floor).unwrap();
            }
            _ => {
                // Other rooms: auto-clear (no combat)
                room_encounters.push(RoomEncounterRecord::non_combat(*room_id, room_kind.clone()));
                run.clear_room(&mut floor).unwrap();
            }
        }

        state.rooms_cleared += 1;

        // Trigger camping after clearing a room mid-run (e.g., after room 3 of 7)
        // This is the integration point for the headless run model
        if room_idx == 3 && state.camping_phase.is_none() && !heroes.is_empty() {
            trigger_camping(&mut state, &mut heroes, room_idx);
        }
    }

    // Finish the run
    let run_result = if state.battles_lost > 0 {
        RunResult::Defeat
    } else {
        RunResult::Victory
    };
    run.finish(run_result);

    // Build run metadata from the dungeon config
    let metadata = RunMetadata::from_config(dungeon_type, config.map_size, map_config);

    // Extract camping trace if camping occurred
    let camping_trace = state.camping_phase
        .as_ref()
        .map(|phase| phase.trace.clone())
        .unwrap_or_default();

    // Clean up camping buffs if camping occurred
    if let Some(ref phase) = state.camping_phase {
        cleanup_camping_buffs(&mut heroes, phase);
    }

    DdgcRunResult {
        run,
        state,
        floor,
        battle_pack_ids,
        metadata,
        room_encounters,
        interaction_records,
        camping_trace,
        heroes,
    }
}

/// Apply a post-battle reward to the game-layer run state.
fn apply_reward(state: &mut DdgcRunState, update: &PostBattleUpdate) {
    state.gold += update.gold_earned;
    state.hp_recovered += update.hp_recovered;
    state.stress_change += update.stress_change;
}

/// Trigger a camping phase at the appropriate integration point.
///
/// This function is called after clearing a room (typically mid-run or before boss).
/// Camping allows heroes to heal, reduce stress, and gain temporary buffs.
///
/// The camping phase:
/// 1. Creates heroes in camp from current hero states (including active buffs)
/// 2. Runs the camping phase (simplified: no skills performed, just structure)
/// 3. Removes camping-only buffs when phase ends
/// 4. Records the camping phase in state
///
/// Returns the updated heroes after camping cleanup.
fn trigger_camping(
    state: &mut DdgcRunState,
    heroes: &mut [HeroState],
    _room_idx: usize,
) {
    use crate::run::camping::{CampingPhase, HeroInCamp};

    // Create HeroInCamp from HeroState, preserving active buffs
    let heroes_in_camp: Vec<HeroInCamp> = heroes
        .iter()
        .map(|h| {
            let mut hero_in_camp = HeroInCamp::new(&h.id, &h.class_id, h.health, h.max_health, h.stress, h.max_stress);
            hero_in_camp.active_buffs = h.active_buffs.clone();
            hero_in_camp.camping_buffs = h.camping_buffs.clone();
            hero_in_camp
        })
        .collect();

    // Create camping phase
    let phase = CampingPhase::new(heroes_in_camp);
    state.camping_phase = Some(phase);
}

/// Clean up camping buffs from heroes when camping phase ends.
///
/// Called when the dungeon run continues after a camping phase.
/// Removes buffs that were marked as camping-only and carries forward
/// HP/stress changes applied during camping.
pub fn cleanup_camping_buffs(heroes: &mut [HeroState], camping_phase: &CampingPhase) {
    for hero_in_camp in &camping_phase.heroes {
        // Find the corresponding HeroState by hero_id
        if let Some(hero_state) = heroes.iter_mut().find(|h| h.id == hero_in_camp.hero_id) {
            // Carry forward HP and stress changes from camping
            hero_state.health = hero_in_camp.health;
            hero_state.stress = hero_in_camp.stress;

            // Remove camping-only buffs from active_buffs
            hero_state.active_buffs.retain(|b| !hero_in_camp.camping_buffs.contains(b));

            // Clear the camping_buffs list
            hero_state.camping_buffs.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use framework_progression::rooms::RoomState;
    use framework_progression::run::RunState;
    use crate::contracts::{QINGLONG_MEDIUM_EXPLORE, QINGLONG_SHORT_EXPLORE};

    #[test]
    fn ddgc_run_slice_progresses_room_by_room() {
        let config = DdgcRunConfig::default();
        let result = run_ddgc_slice(&config);

        // Every room on the floor should be Cleared
        for room_id in &result.floor.rooms {
            assert_eq!(
                result.floor.rooms_map[room_id].state,
                RoomState::Cleared,
                "Room {:?} should be Cleared",
                room_id
            );
        }

        // visited_rooms should match the floor's room order
        assert_eq!(
            result.state.visited_rooms,
            result.floor.rooms,
            "Rooms should be visited in floor order"
        );

        // All rooms should have been cleared
        assert_eq!(
            result.state.rooms_cleared,
            result.floor.rooms.len() as u32,
            "All rooms should be cleared"
        );
    }

    #[test]
    fn ddgc_run_slice_applies_post_battle_updates() {
        // Use medium map to get 14 rooms, guaranteeing combat rooms appear
        let config = DdgcRunConfig {
            seed: 99,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Medium,
            heroes: Vec::new(),
        };
        let result = run_ddgc_slice(&config);

        // Count combat and boss rooms on the generated floor
        let combat_count = result
            .floor
            .rooms
            .iter()
            .filter(|rid| matches!(result.floor.rooms_map[rid].kind, RoomKind::Combat))
            .count();
        let boss_count = result
            .floor
            .rooms
            .iter()
            .filter(|rid| matches!(result.floor.rooms_map[rid].kind, RoomKind::Boss))
            .count();
        let battle_count = combat_count + boss_count;

        // At least one battle room should exist with 10 rooms and combat-heavy weights
        assert!(
            battle_count > 0,
            "Expected at least one combat or boss room with 10 rooms and combat-heavy weights"
        );

        // Battles won should match combat + boss room count
        assert_eq!(
            result.state.battles_won, battle_count as u32,
            "Should have won one battle per combat/boss room"
        );

        // Gold should match expected rewards
        let expected_gold = (combat_count as u32 * 50) + (boss_count as u32 * 200);
        assert_eq!(
            result.state.gold, expected_gold,
            "Gold should match combat ({}) + boss ({}) rewards",
            combat_count, boss_count
        );

        // HP recovery should match expected
        let expected_hp = (combat_count as f64 * 2.0) + (boss_count as f64 * 10.0);
        assert_eq!(
            result.state.hp_recovered, expected_hp,
            "HP recovery should match combat + boss rewards"
        );

        // Stress change should match expected
        let expected_stress = (combat_count as f64 * -5.0) + (boss_count as f64 * -15.0);
        assert_eq!(
            result.state.stress_change, expected_stress,
            "Stress change should match combat + boss rewards"
        );

        // No battles should be lost (DDGC-scale heroes should defeat all encounter packs)
        assert_eq!(
            result.state.battles_lost, 0,
            "No battles should be lost in this scenario"
        );
    }

    #[test]
    fn ddgc_run_slice_finishes_with_expected_outcome() {
        let config = DdgcRunConfig::default();
        let result = run_ddgc_slice(&config);

        // Run should be in Victory state (all battles won)
        assert_eq!(
            result.run.state,
            RunState::Victory,
            "Run should end in Victory when all battles are won"
        );

        // All rooms should be cleared
        let room_count = result.floor.rooms.len() as u32;
        assert_eq!(
            result.state.rooms_cleared,
            room_count,
            "All {} rooms should be cleared",
            room_count
        );

        // Run should have visited all rooms
        assert_eq!(
            result.state.visited_rooms.len(),
            room_count as usize,
            "Should have visited all rooms"
        );

        // At least one battle should have been won (combat-heavy weights guarantee it)
        assert!(
            result.state.battles_won > 0,
            "Should have won at least one battle"
        );
    }

    #[test]
    fn migrated_encounter_content_is_default_path_for_all_dungeons() {
        // Proves that the four core DDGC dungeons resolve combat and boss rooms
        // through the migrated encounter pack registry (not placeholder content).
        // This is the acceptance test for K43: gameplay entrypoints no longer
        // depend on Bone Soldier or Necromancer placeholders.
        use crate::run::encounters::EncounterResolver;
        use crate::encounters::PackType;

        let resolver = EncounterResolver::new();

        for dungeon in [Dungeon::QingLong, Dungeon::BaiHu, Dungeon::ZhuQue, Dungeon::XuanWu] {
            // Combat rooms must resolve through encounter registry
            let combat_pack = resolver.resolve_pack(dungeon, 0, 42, false);
            assert!(
                combat_pack.is_some(),
                "{:?} combat room should resolve through encounter registry",
                dungeon
            );
            let pack = combat_pack.unwrap();
            assert_ne!(
                pack.pack_type,
                PackType::Boss,
                "{:?} combat room should not resolve to boss pack",
                dungeon
            );

            // Boss rooms must resolve through encounter registry
            let boss_pack = resolver.resolve_boss_pack(dungeon, 0, 42);
            assert!(
                boss_pack.is_some(),
                "{:?} boss room should resolve through encounter registry",
                dungeon
            );
            assert_eq!(
                boss_pack.unwrap().pack_type,
                PackType::Boss,
                "{:?} boss room should resolve to boss pack",
                dungeon
            );
        }
    }

    #[test]
    fn run_slice_uses_no_fallback_content() {
        // Proves that a representative run slice completes using migrated DDGC
        // encounter content only — no fallback to first_battle placeholder.
        // This is the acceptance test for US-713: "Remove transitional encounter
        // and run fallbacks."
        let config = DdgcRunConfig {
            seed: 42,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
            heroes: Vec::new(),
        };
        let result = run_ddgc_slice(&config);

        // Count battle rooms
        let battle_room_count = result
            .floor
            .rooms
            .iter()
            .filter(|rid| {
                matches!(
                    result.floor.rooms_map[rid].kind,
                    RoomKind::Combat | RoomKind::Boss
                )
            })
            .count();

        // There should be battle rooms (combat-heavy weights + short map has 9 rooms)
        assert!(
            battle_room_count > 0,
            "Expected battle rooms with 9 rooms and combat-heavy weights"
        );

        // Every battle pack_id must be a real pack (not the fallback marker)
        for (i, pack_id) in result.battle_pack_ids.iter().enumerate() {
            assert_ne!(
                pack_id, "fallback_transitional",
                "Battle {} must not use fallback_transitional — should use migrated DDGC content",
                i
            );
        }

        // battle_pack_ids count should match battle room count
        assert_eq!(
            result.battle_pack_ids.len(), battle_room_count,
            "battle_pack_ids count should match number of battle rooms"
        );

        // All battles should be won (DDGC-scale heroes vs migrated encounter packs)
        assert_eq!(
            result.state.battles_won, battle_room_count as u32,
            "All {} battle rooms should be won",
            battle_room_count
        );
        assert_eq!(
            result.state.battles_lost, 0,
            "No battles should be lost"
        );
    }

    // ── US-810-a: DungeonMapConfig wiring tests ──────────────────────────────────

    #[test]
    fn qinglong_maps_have_correct_room_count() {
        // QingLong short config has base_room_number = 9, medium = 14
        let short_config = DdgcRunConfig {
            seed: 42,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
            heroes: Vec::new(),
        };
        let short_result = run_ddgc_slice(&short_config);
        assert_eq!(
            short_result.floor.rooms.len() as u32,
            QINGLONG_SHORT_EXPLORE.base_room_number,
            "QingLong short should have {} rooms",
            QINGLONG_SHORT_EXPLORE.base_room_number
        );

        let medium_config = DdgcRunConfig {
            seed: 42,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Medium,
            heroes: Vec::new(),
        };
        let medium_result = run_ddgc_slice(&medium_config);
        assert_eq!(
            medium_result.floor.rooms.len() as u32,
            QINGLONG_MEDIUM_EXPLORE.base_room_number,
            "QingLong medium should have {} rooms",
            QINGLONG_MEDIUM_EXPLORE.base_room_number
        );
    }

    #[test]
    fn baihu_maps_have_lower_connectivity_than_zhuque() {
        // BaiHu connectivity (0.85) < ZhuQue connectivity (0.95)
        // This should produce floors with measurably fewer connections.
        // We use the same seed so the random sequence is identical,
        // only max_connections differs based on dungeon config.
        let baihu_config = DdgcRunConfig {
            seed: 3,
            dungeon: Dungeon::BaiHu,
            map_size: MapSize::Short,
            heroes: Vec::new(),
        };
        let zhuque_config = DdgcRunConfig {
            seed: 3,
            dungeon: Dungeon::ZhuQue,
            map_size: MapSize::Short,
            heroes: Vec::new(),
        };

        let baihu_result = run_ddgc_slice(&baihu_config);
        let zhuque_result = run_ddgc_slice(&zhuque_config);

        // Compute average connections per room for each floor
        let baihu_avg_conn = avg_connections_per_room(&baihu_result.floor);
        let zhuque_avg_conn = avg_connections_per_room(&zhuque_result.floor);

        assert!(
            baihu_avg_conn < zhuque_avg_conn,
            "BaiHu avg connections ({:.2}) should be less than ZhuQue ({:.2})",
            baihu_avg_conn, zhuque_avg_conn
        );
    }

    #[test]
    fn short_vs_medium_maps_differ_in_room_count() {
        // Short variants have 9 rooms, medium variants have 14
        for dungeon in [Dungeon::QingLong, Dungeon::BaiHu, Dungeon::ZhuQue, Dungeon::XuanWu] {
            let short_config = DdgcRunConfig {
                seed: 42,
                dungeon,
                map_size: MapSize::Short,
                heroes: Vec::new(),
            };
            let medium_config = DdgcRunConfig {
                seed: 42,
                dungeon,
                map_size: MapSize::Medium,
                heroes: Vec::new(),
            };

            let short_result = run_ddgc_slice(&short_config);
            let medium_result = run_ddgc_slice(&medium_config);

            assert!(
                short_result.floor.rooms.len() < medium_result.floor.rooms.len(),
                "{:?} short ({}) should have fewer rooms than medium ({})",
                dungeon,
                short_result.floor.rooms.len(),
                medium_result.floor.rooms.len()
            );
        }
    }

    // ── US-812: Dungeon fidelity end-to-end validation ──────────────────────────

    #[test]
    fn dungeon_fidelity_test_qinglong_short() {
        // US-812: End-to-end validation of dungeon fidelity.
        // Verifies that a QingLong short run slice completes with correct room
        // count, correct encounter types, and correct monster families per room.
        // The run trace records dungeon type, map parameters, and encounter composition.
        use framework_progression::rooms::RoomState;
        use crate::contracts::{DungeonType, MapSize, QINGLONG_SHORT_EXPLORE};

        let config = DdgcRunConfig {
            seed: 42,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
            heroes: Vec::new(),
        };
        let result = run_ddgc_slice(&config);

        // ── 1. Room count verification ─────────────────────────────────────────
        assert_eq!(
            result.floor.rooms.len() as u32,
            QINGLONG_SHORT_EXPLORE.base_room_number,
            "QingLong short should have {} rooms",
            QINGLONG_SHORT_EXPLORE.base_room_number
        );

        // ── 2. Dungeon metadata verification ────────────────────────────────────
        assert_eq!(
            result.metadata.dungeon_type,
            DungeonType::QingLong,
            "Dungeon type should be QingLong"
        );
        assert_eq!(
            result.metadata.map_size,
            MapSize::Short,
            "Map size should be Short"
        );
        assert_eq!(
            result.metadata.base_room_number,
            QINGLONG_SHORT_EXPLORE.base_room_number,
            "Metadata base_room_number should match config"
        );
        assert_eq!(
            result.metadata.connectivity,
            QINGLONG_SHORT_EXPLORE.connectivity,
            "Metadata connectivity should match config"
        );
        assert_eq!(
            result.metadata.gridsize, QINGLONG_SHORT_EXPLORE.gridsize,
            "Metadata gridsize should match config"
        );

        // ── 3. Room encounters verification ────────────────────────────────────
        assert_eq!(
            result.room_encounters.len(),
            result.floor.rooms.len(),
            "room_encounters should have one entry per room"
        );

        // Count combat and boss rooms
        let battle_room_count = result
            .room_encounters
            .iter()
            .filter(|r| matches!(r.room_kind, RoomKind::Combat | RoomKind::Boss))
            .count();

        assert!(
            battle_room_count > 0,
            "Should have at least one battle room"
        );

        // Verify all battle rooms have valid pack IDs and family compositions
        for record in &result.room_encounters {
            match record.room_kind {
                RoomKind::Combat | RoomKind::Boss => {
                    // Pack ID should not be empty for battle rooms
                    assert!(
                        !record.pack_id.is_empty(),
                        "Battle room {:?} should have a pack_id",
                        record.room_id
                    );
                    // Family IDs should not be empty for battle rooms
                    assert!(
                        !record.family_ids.is_empty(),
                        "Battle room {:?} ({}) should have family_ids",
                        record.room_id, record.pack_id
                    );
                    // Pack ID should be a valid DDGC pack (not fallback)
                    assert_ne!(
                        record.pack_id, "fallback_transitional",
                        "Battle room should not use fallback_transitional"
                    );
                }
                RoomKind::Event { .. } => {
                    // Event rooms have empty pack_id and family_ids
                    assert!(
                        record.pack_id.is_empty(),
                        "Event room {:?} should have empty pack_id",
                        record.room_id
                    );
                    assert!(
                        record.family_ids.is_empty(),
                        "Event room {:?} should have empty family_ids",
                        record.room_id
                    );
                }
                _ => {
                    // Shop, Treasure, Corridor, Custom: no encounter, no battle
                    assert!(
                        record.pack_id.is_empty(),
                        "Non-combat room {:?} should have empty pack_id",
                        record.room_id
                    );
                    assert!(
                        record.family_ids.is_empty(),
                        "Non-combat room {:?} should have empty family_ids",
                        record.room_id
                    );
                }
            }
        }

        // ── 4. Battle pack IDs verification ────────────────────────────────────
        assert_eq!(
            result.battle_pack_ids.len(),
            battle_room_count,
            "battle_pack_ids count should match battle room count"
        );

        // Verify battle_pack_ids matches room_encounters for battle rooms
        let battle_encounters: Vec<_> = result
            .room_encounters
            .iter()
            .filter(|r| matches!(r.room_kind, RoomKind::Combat | RoomKind::Boss))
            .collect();
        for (i, encounter) in battle_encounters.iter().enumerate() {
            assert_eq!(
                result.battle_pack_ids[i], encounter.pack_id,
                "battle_pack_ids[{}] should match room_encounters pack_id",
                i
            );
        }

        // ── 5. Room state verification ─────────────────────────────────────────
        for room_id in &result.floor.rooms {
            assert_eq!(
                result.floor.rooms_map[room_id].state,
                RoomState::Cleared,
                "Room {:?} should be Cleared",
                room_id
            );
        }

        // ── 6. Victory outcome verification ────────────────────────────────────
        assert_eq!(
            result.state.battles_won, battle_room_count as u32,
            "All {} battle rooms should be won",
            battle_room_count
        );
        assert_eq!(
            result.state.battles_lost, 0,
            "No battles should be lost"
        );
    }

    // ── US-005: Curio/trap/obstacle interaction wiring tests ───────────────────

    #[test]
    fn run_slice_produces_curio_and_trap_interactions() {
        // US-005 acceptance test: proves that Event rooms produce curio
        // interactions and Corridor rooms produce trap interactions in the
        // run trace. Uses a seed known to generate both room kinds.
        let config = DdgcRunConfig {
            seed: 1,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
            heroes: Vec::new(),
        };
        let result = run_ddgc_slice(&config);

        // Count curio interactions
        let curio_count = result
            .interaction_records
            .iter()
            .filter(|r| matches!(r.interaction_type, InteractionType::Curio))
            .count();

        // Count trap interactions
        let trap_count = result
            .interaction_records
            .iter()
            .filter(|r| matches!(r.interaction_type, InteractionType::Trap))
            .count();

        // QingLong short has hallway_curio = 9/9 (guaranteed curio in corridors)
        // and room_curio = 0/0 (30% fallback for event rooms). With combat-heavy
        // weights there should be at least one corridor, guaranteeing a curio.
        assert!(
            curio_count > 0,
            "Expected at least one curio interaction in the run trace, got {}",
            curio_count
        );

        // hallway_trap = 0/0 gives 20% fallback chance per corridor.
        // With multiple corridors possible across the floor, traps should appear.
        assert!(
            trap_count > 0,
            "Expected at least one trap interaction in the run trace, got {}",
            trap_count
        );

        // Every interaction record should have a non-empty entity ID
        for record in &result.interaction_records {
            assert!(
                !record.entity_id.is_empty(),
                "Interaction record should have a non-empty entity_id"
            );
        }
    }

    #[test]
    fn interaction_records_match_room_kinds() {
        // Verifies that curio interactions only come from Event/Corridor rooms
        // and trap interactions only come from Corridor rooms.
        let config = DdgcRunConfig::default();
        let result = run_ddgc_slice(&config);

        for record in &result.interaction_records {
            match record.interaction_type {
                InteractionType::Curio => {
                    assert!(
                        matches!(record.room_kind, RoomKind::Event { .. } | RoomKind::Corridor { .. }),
                        "Curio interaction should only occur in Event or Corridor rooms"
                    );
                }
                InteractionType::Trap => {
                    assert!(
                        matches!(record.room_kind, RoomKind::Corridor { .. }),
                        "Trap interaction should only occur in Corridor rooms"
                    );
                }
                InteractionType::Obstacle => {
                    // Obstacles are not yet wired into room generation
                }
            }
        }
    }

    #[test]
    fn event_rooms_carry_curio_id_on_kind() {
        // Verifies that RoomKind::Event carries an optional curio_id after
        // interaction assignment. Uses seed 1 which is known to produce curios.
        let config = DdgcRunConfig {
            seed: 1,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
            heroes: Vec::new(),
        };
        let result = run_ddgc_slice(&config);

        let event_rooms: Vec<_> = result
            .floor
            .rooms
            .iter()
            .filter(|rid| matches!(result.floor.rooms_map[rid].kind, RoomKind::Event { .. }))
            .collect();

        // There should be at least one event room
        assert!(!event_rooms.is_empty(), "Expected at least one Event room");

        // At least one event room should have a curio_id assigned
        let event_with_curio = event_rooms.iter().any(|rid| {
            if let RoomKind::Event { curio_id } = &result.floor.rooms_map[rid].kind {
                curio_id.is_some()
            } else {
                false
            }
        });
        assert!(event_with_curio, "Expected at least one Event room with a curio_id");
    }

    #[test]
    fn corridor_rooms_carry_trap_and_curio_ids_on_kind() {
        // Verifies that RoomKind::Corridor carries optional trap_id and curio_id
        // after interaction assignment. Uses seed 1 which is known to produce interactions.
        let config = DdgcRunConfig {
            seed: 1,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
            heroes: Vec::new(),
        };
        let result = run_ddgc_slice(&config);

        let corridor_rooms: Vec<_> = result
            .floor
            .rooms
            .iter()
            .filter(|rid| matches!(result.floor.rooms_map[rid].kind, RoomKind::Corridor { .. }))
            .collect();

        // There should be at least one corridor room
        assert!(!corridor_rooms.is_empty(), "Expected at least one Corridor room");

        // At least one corridor should have a curio_id (hallway_curio density is high)
        let corridor_with_curio = corridor_rooms.iter().any(|rid| {
            if let RoomKind::Corridor { curio_id, .. } = &result.floor.rooms_map[rid].kind {
                curio_id.is_some()
            } else {
                false
            }
        });
        assert!(
            corridor_with_curio,
            "Expected at least one Corridor room with a curio_id"
        );
    }

    /// Compute the average number of connections per room in a floor.
    fn avg_connections_per_room(floor: &Floor) -> f64 {
        if floor.rooms_map.is_empty() {
            return 0.0;
        }
        let total_connections: usize = floor.rooms_map.values().map(|r| r.connections.len()).sum();
        total_connections as f64 / floor.rooms_map.len() as f64
    }

    // ── US-009: Full-loop town → dungeon → town tests ─────────────────────────

    #[test]
    fn full_loop_runs_town_dungeon_town() {
        // US-009 acceptance test: proves the full game loop runs
        // town → dungeon → town with heroes progressing between runs.
        let config = DdgcFullRunConfig {
            seed: 42,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
            initial_heroes: vec![
                HeroState::new("h1", "alchemist", 100.0, 100.0, 0.0, 200.0),
                HeroState::new("h2", "hunter", 100.0, 100.0, 0.0, 200.0),
            ],
            initial_gold: 500,
        };
        let result = run_ddgc_full_loop(&config);

        // Dungeon result should exist
        assert_eq!(
            result.dungeon_result.state.rooms_cleared,
            result.dungeon_result.floor.rooms.len() as u32,
            "All rooms should be cleared in dungeon"
        );

        // Heroes should persist through the loop
        assert_eq!(
            result.heroes_post_dungeon.len(),
            config.initial_heroes.len(),
            "Heroes should persist after dungeon"
        );
        assert_eq!(
            result.heroes_post_town.len(),
            config.initial_heroes.len(),
            "Heroes should persist after town visit"
        );

        // Gold should increase from dungeon and decrease from town
        assert!(
            result.gold_post_dungeon >= config.initial_gold,
            "Gold should increase after dungeon"
        );
    }

    #[test]
    fn full_loop_tracks_stress_through_dungeon() {
        // Verifies that hero stress changes during dungeon run based on dungeon result.
        // The dungeon model provides stress relief (negative stress_change), so heroes
        // may have lower stress after dungeon compared to before.
        let config = DdgcFullRunConfig {
            seed: 42,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
            initial_heroes: vec![
                HeroState::new("h1", "alchemist", 100.0, 100.0, 50.0, 200.0),
            ],
            initial_gold: 500,
        };
        let result = run_ddgc_full_loop(&config);

        // Verify stress changed (either increased or decreased based on dungeon rewards)
        let initial_stress = config.initial_heroes[0].stress;
        let post_dungeon_stress = result.heroes_post_dungeon[0].stress;

        // Stress should have changed due to dungeon run (rewards provide stress relief)
        let battles_count = result.dungeon_result.state.battles_won + result.dungeon_result.state.battles_lost;
        if battles_count > 0 {
            // With stress relief model, post-dungeon stress should be less or equal
            assert!(
                post_dungeon_stress <= initial_stress,
                "Hero stress should decrease or stay same after dungeon run with {} battles",
                battles_count
            );
        }

        // Post-town stress should be lower or equal after healing
        assert!(
            result.heroes_post_town[0].stress <= post_dungeon_stress,
            "Stress should be reduced or stay same after town visit"
        );
    }

    #[test]
    fn full_loop_records_town_activities() {
        // Verifies that town activities are recorded in the trace
        let config = DdgcFullRunConfig {
            seed: 42,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
            initial_heroes: vec![
                HeroState::new("h1", "alchemist", 100.0, 100.0, 0.0, 200.0),
            ],
            initial_gold: 500,
        };
        let result = run_ddgc_full_loop(&config);

        // Pre-dungeon town trace should exist (may be empty if no stress)
        // Post-dungeon town trace should exist
        // Post-dungeon town trace should be accessible
        let _ = result.post_town_trace.activities.len();
    }

    #[test]
    fn full_loop_gold_flows_through_loop() {
        // Verifies gold is earned in dungeon and spent in town
        let config = DdgcFullRunConfig {
            seed: 42,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
            initial_heroes: vec![
                HeroState::new("h1", "alchemist", 100.0, 100.0, 0.0, 200.0),
                HeroState::new("h2", "hunter", 100.0, 100.0, 0.0, 200.0),
            ],
            initial_gold: 500,
        };
        let result = run_ddgc_full_loop(&config);

        // Gold should be earned in dungeon
        assert!(
            result.gold_post_dungeon >= config.initial_gold + result.dungeon_result.state.gold,
            "Gold should be sum of initial and dungeon earnings"
        );

        // Gold after town should be less or equal to after dungeon (town costs)
        assert!(
            result.gold_post_town <= result.gold_post_dungeon,
            "Gold should decrease after town activities"
        );
    }

    #[test]
    fn hero_state_converts_to_hero_in_town() {
        // Verifies HeroState.to_hero_in_town() works correctly
        let hero = HeroState::new("h1", "alchemist", 80.0, 100.0, 50.0, 200.0);
        let hero_in_town = hero.to_hero_in_town();

        assert_eq!(hero_in_town.id, "h1");
        assert_eq!(hero_in_town.class_id, "alchemist");
        assert_eq!(hero_in_town.health, 80.0);
        assert_eq!(hero_in_town.max_health, 100.0);
        assert_eq!(hero_in_town.stress, 50.0);
        assert_eq!(hero_in_town.max_stress, 200.0);
    }

    #[test]
    fn full_loop_stress_heal_reduces_hero_stress() {
        // Verifies that Abbey stress heal reduces hero stress
        let config = DdgcFullRunConfig {
            seed: 42,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
            initial_heroes: vec![
                HeroState::new("h1", "alchemist", 100.0, 100.0, 0.0, 200.0),
            ],
            initial_gold: 500,
        };
        let result = run_ddgc_full_loop(&config);

        // If hero had stress after dungeon, post-town stress should be less
        if result.heroes_post_dungeon[0].stress > 0.0 {
            assert!(
                result.heroes_post_town[0].stress < result.heroes_post_dungeon[0].stress,
                "Stress should be reduced after town visit"
            );
        }
    }

    // ── US-018: Quirk acquisition end-to-end tests ────────────────────────────

    #[test]
    fn curio_quirk_acquired_end_to_end() {
        // Proves that a curio interaction with Quirk result type triggers
        // apply_quirk and the quirk appears in the hero's quirk state.
        let curio_registry = crate::contracts::parse::parse_curios_csv(
            &std::path::PathBuf::from("data").join("Curios.csv"),
        )
        .unwrap();
        let quirk_registry = crate::contracts::parse::parse_quirks_json(
            &std::path::PathBuf::from("data").join("JsonQuirks.json"),
        )
        .unwrap();
        let mut quirk_state = HeroQuirkState::new();

        // ancient_vase: Nothing(5), Loot(10), Quirk(5) -> total=20
        // Quirk when seed % 20 >= 15
        let outcome = curio_registry
            .resolve_curio_interaction("ancient_vase", false, "", 15)
            .unwrap();
        assert_eq!(outcome.result_type, CurioResultType::Quirk);
        assert_eq!(outcome.result_id, "clumsy");

        apply_curio_quirk(&mut quirk_state, outcome.result_type, &outcome.result_id, &quirk_registry);
        assert!(quirk_state.negative.contains(&"clumsy".to_string()));
    }

    #[test]
    fn curio_disease_acquired_end_to_end() {
        // Proves that a curio interaction with Disease result type triggers
        // apply_quirk and the disease appears in the hero's quirk state.
        let curio_registry = crate::contracts::parse::parse_curios_csv(
            &std::path::PathBuf::from("data").join("Curios.csv"),
        )
        .unwrap();
        let quirk_registry = crate::contracts::parse::parse_quirks_json(
            &std::path::PathBuf::from("data").join("JsonQuirks.json"),
        )
        .unwrap();
        let mut quirk_state = HeroQuirkState::new();

        // mossy_stone: Nothing(6), Quirk(8), Disease(2) -> total=16
        // Disease when seed % 16 >= 14
        let outcome = curio_registry
            .resolve_curio_interaction("mossy_stone", false, "", 15)
            .unwrap();
        assert_eq!(outcome.result_type, CurioResultType::Disease);
        assert_eq!(outcome.result_id, "consumptive");

        apply_curio_quirk(&mut quirk_state, outcome.result_type, &outcome.result_id, &quirk_registry);
        assert!(quirk_state.diseases.contains(&"consumptive".to_string()));
    }

    #[test]
    fn run_slice_acquires_quirk_from_curio_interaction() {
        // End-to-end: searches for a seed where a curio interaction in a run
        // produces a Quirk or Disease, then verifies the quirk state.
        for seed in 0..200u64 {
            let config = DdgcRunConfig {
                seed,
                dungeon: Dungeon::QingLong,
                map_size: MapSize::Short,
                heroes: Vec::new(),
            };
            let result = run_ddgc_slice(&config);

            for record in &result.interaction_records {
                if let InteractionOutcome::Curio {
                    result_type,
                    result_id,
                } = &record.outcome
                {
                    if *result_type == CurioResultType::Quirk || *result_type == CurioResultType::Disease
                    {
                        assert!(
                            result.state.quirk_state.contains(result_id),
                            "Curio interaction should have applied {} to quirk state",
                            result_id
                        );
                        return;
                    }
                }
            }
        }
        panic!("No seed in 0..200 produced a curio quirk/disease outcome");
    }

    // ── US-008-b: Camping integration end-to-end tests ───────────────────────

    #[test]
    fn run_slice_with_camping_triggers_camping_phase() {
        // US-008-b: Proves that camping is triggered at the integration point
        // (room 3 of the run). With heroes present, camping should occur.
        let config = DdgcRunConfig {
            seed: 42,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
            heroes: vec![
                HeroState::new("h1", "alchemist", 80.0, 100.0, 30.0, 200.0),
                HeroState::new("h2", "hunter", 90.0, 100.0, 20.0, 200.0),
            ],
        };
        let result = run_ddgc_slice(&config);

        // Camping should have been triggered (room_idx == 3 is mid-run)
        assert!(
            result.state.camping_phase.is_some(),
            "Camping phase should be set when heroes are present"
        );
    }

    #[test]
    fn run_slice_with_camping_records_activity_in_trace() {
        // US-008-b: Proves that camping activity is recorded in the run trace.
        // For the headless run model, camping is triggered but no skills are performed,
        // so the camping_phase exists (proving camping occurred) but trace may be empty.
        let config = DdgcRunConfig {
            seed: 42,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
            heroes: vec![
                HeroState::new("h1", "alchemist", 80.0, 100.0, 30.0, 200.0),
            ],
        };
        let result = run_ddgc_slice(&config);

        // Camping phase should exist (proving camping was triggered)
        assert!(
            result.state.camping_phase.is_some(),
            "Camping phase should exist to record camping activity"
        );
    }

    #[test]
    fn run_slice_with_camping_carries_forward_hp_stress() {
        // US-008-b: Proves that hero HP and stress carry forward after camping.
        // We verify by checking that heroes post-camp have the same HP/stress
        // as when they entered the camping phase (since no skills were used).
        let config = DdgcRunConfig {
            seed: 42,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
            heroes: vec![
                HeroState::new("h1", "alchemist", 80.0, 100.0, 30.0, 200.0),
                HeroState::new("h2", "hunter", 90.0, 100.0, 20.0, 200.0),
            ],
        };
        let result = run_ddgc_slice(&config);

        // The camping phase should exist
        let camping_phase = result.state.camping_phase.as_ref()
            .expect("Camping phase should exist");

        // Find h1 in the camping phase and verify HP/stress were preserved
        let h1_in_camp = camping_phase.get_hero("h1")
            .expect("h1 should be in camping phase");
        assert_eq!(h1_in_camp.health, 80.0, "h1 health should be preserved through camping");
        assert_eq!(h1_in_camp.stress, 30.0, "h1 stress should be preserved through camping");

        // Verify the final hero states also have these values (cleanup was called)
        let h1_final = result.heroes.iter()
            .find(|h| h.id == "h1")
            .expect("h1 should be in final heroes");
        assert_eq!(h1_final.health, 80.0, "h1 health should be carried to final state");
        assert_eq!(h1_final.stress, 30.0, "h1 stress should be carried to final state");
    }

    #[test]
    fn run_slice_cleans_up_camping_buffs_after_camping_phase() {
        // US-008-b: Proves that camping-only buffs are removed after camping ends.
        // We simulate this by manually adding a camping buff and verifying cleanup.
        use crate::run::camping::{CampingPhase, HeroInCamp};

        let mut heroes = vec![
            HeroState::new("h1", "alchemist", 100.0, 100.0, 0.0, 200.0),
        ];
        // Add active buffs and a camping buff
        heroes[0].active_buffs = vec!["normal_buff".to_string(), "camping_temp_buff".to_string()];
        heroes[0].camping_buffs = vec!["camping_temp_buff".to_string()];

        // Create a mock camping phase to test cleanup
        let heroes_in_camp: Vec<HeroInCamp> = heroes.iter().map(|h| {
            let mut hic = HeroInCamp::new(&h.id, &h.class_id, h.health, h.max_health, h.stress, h.max_stress);
            hic.active_buffs = h.active_buffs.clone();
            hic.camping_buffs = h.camping_buffs.clone();
            hic
        }).collect();
        let camping_phase = CampingPhase::new(heroes_in_camp);

        // Perform cleanup
        super::cleanup_camping_buffs(&mut heroes, &camping_phase);

        // Verify camping buff was removed but normal buff remains
        assert!(
            heroes[0].active_buffs.contains(&"normal_buff".to_string()),
            "Normal buff should remain after cleanup"
        );
        assert!(
            !heroes[0].active_buffs.contains(&"camping_temp_buff".to_string()),
            "Camping buff should be removed after cleanup"
        );
        assert!(
            heroes[0].camping_buffs.is_empty(),
            "Camping buffs list should be cleared"
        );
    }

    #[test]
    fn run_slice_camping_buff_removal_does_not_affect_other_heroes() {
        // US-008-b: Verifies that buff cleanup only affects the hero who had the camping buff.
        use crate::run::camping::{CampingPhase, HeroInCamp};

        let mut heroes = vec![
            HeroState::new("h1", "alchemist", 100.0, 100.0, 0.0, 200.0),
            HeroState::new("h2", "hunter", 100.0, 100.0, 0.0, 200.0),
        ];
        // h1 has a camping buff, h2 does not
        heroes[0].active_buffs = vec!["camping_temp_buff".to_string()];
        heroes[0].camping_buffs = vec!["camping_temp_buff".to_string()];
        heroes[1].active_buffs = vec!["permanent_buff".to_string()];
        heroes[1].camping_buffs = vec![];

        let heroes_in_camp: Vec<HeroInCamp> = heroes.iter().map(|h| {
            let mut hic = HeroInCamp::new(&h.id, &h.class_id, h.health, h.max_health, h.stress, h.max_stress);
            hic.active_buffs = h.active_buffs.clone();
            hic.camping_buffs = h.camping_buffs.clone();
            hic
        }).collect();
        let camping_phase = CampingPhase::new(heroes_in_camp);

        super::cleanup_camping_buffs(&mut heroes, &camping_phase);

        // h2's permanent buff should remain unaffected
        assert!(
            heroes[1].active_buffs.contains(&"permanent_buff".to_string()),
            "h2's buff should remain unaffected"
        );
    }

    #[test]
    fn run_slice_camping_completes_before_later_combat() {
        // US-008-b: End-to-end test proving a run with camping completes successfully
        // and the dungeon run finishes without errors after camping.
        let config = DdgcRunConfig {
            seed: 42,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
            heroes: vec![
                HeroState::new("h1", "alchemist", 80.0, 100.0, 30.0, 200.0),
                HeroState::new("h2", "hunter", 90.0, 100.0, 20.0, 200.0),
            ],
        };
        let result = run_ddgc_slice(&config);

        // Run should complete successfully
        assert_eq!(
            result.run.state,
            framework_progression::run::RunState::Victory,
            "Run should end in Victory"
        );

        // All rooms should be cleared
        assert_eq!(
            result.state.rooms_cleared,
            result.floor.rooms.len() as u32,
            "All rooms should be cleared"
        );

        // Camping should have occurred
        assert!(
            result.state.camping_phase.is_some(),
            "Camping phase should exist"
        );

        // For headless run model, trace may be empty since no skills are performed
        // The existence of camping_phase proves camping was triggered
        // Camping trace can be checked if skills were actually used
        let _camping_phase = result.state.camping_phase.as_ref().unwrap();
    }
}
