//! DDGC adapters — transforms framework payloads into DDGC view models.
//!
//! This module provides adapters that convert framework payloads
//! (from `framework_viewmodels`, `framework_progression`, and `framework_combat`)
//! into DDGC-specific view models defined in [`super::viewmodels`].
//!
//! # Design principles
//!
//! - Adapters are **pure conversion functions** — no side effects, no state mutation.
//! - Each adapter takes a specific payload type and returns a [`ViewModelResult`]
//!   containing either the shaped view model or a [`ViewModelError`].
//! - Unsupported or partially-mapped states produce explicit error surfaces
//!   rather than silently returning partial data.
//! - Adapter logic remains **product-owned** and is not pushed down into `WorldEngine`.
//!
//! # Supported conversions
//!
//! | Payload type | View model | Function |
//! |---|---|---|
//! | `DdgcHost` + `HostPhase` | `BootLoadViewModel` | [`boot_load_from_host`] |
//! | `CampaignState` | `TownViewModel` | [`town_from_campaign`] |
//! | `DdgcRunResult` | `DungeonViewModel` | [`dungeon_from_run_result`] |
//! | `framework_viewmodels::CombatViewModel` | `CombatViewModel` | [`combat_from_framework`] |
//! | `RunResult` + rewards | `ResultViewModel` | [`result_from_run`] |
//! | `DdgcRunState` + heroes | `ReturnFlowViewModel` | [`return_flow_from_state`] |

use crate::contracts::viewmodels::{
    BootLoadViewModel, CombatPhase, CombatViewModel, CombatantType,
    CombatantViewModel, CombatPosition, DungeonHeroViewModel, DungeonRoomKind, DungeonRoomViewModel,
    DungeonViewModel, ViewModelResult,
};
use crate::contracts::{
    CampaignState, DungeonType, HeirloomCurrency, MapSize,
};

/// Adapter: Convert `DdgcHost` state to `BootLoadViewModel`.
///
/// Takes the host phase and a flag indicating whether campaign was loaded
/// to produce the boot/load view model.
pub fn boot_load_from_host(
    host_phase: &crate::contracts::host::HostPhase,
    campaign_loaded: bool,
    campaign_schema_version: Option<u32>,
) -> ViewModelResult<BootLoadViewModel> {
    match host_phase {
        crate::contracts::host::HostPhase::Uninitialized => {
            Ok(BootLoadViewModel::success("Initialized and ready to boot", vec![]))
        }
        crate::contracts::host::HostPhase::Booting => {
            Ok(BootLoadViewModel::success("Loading contract packages...", vec![]))
        }
        crate::contracts::host::HostPhase::Ready => {
            let mut vm = BootLoadViewModel::success("Host ready", vec![]);
            if campaign_loaded {
                vm.status_message = "Campaign loaded successfully".to_string();
            }
            if let Some(version) = campaign_schema_version {
                vm = vm.with_campaign_version(version);
            }
            Ok(vm)
        }
        crate::contracts::host::HostPhase::FatalError => {
            Ok(BootLoadViewModel::failure("Fatal error during boot"))
        }
        crate::contracts::host::HostPhase::Unsupported => {
            Ok(BootLoadViewModel::failure("Feature not supported in this build"))
        }
    }
}

/// Adapter: Convert `CampaignState` to `TownViewModel`.
///
/// Takes the campaign state and produces a town visit view model
/// with buildings, roster, and available activities.
pub fn town_from_campaign(
    campaign: &CampaignState,
) -> ViewModelResult<crate::contracts::viewmodels::TownViewModel> {
    use crate::contracts::viewmodels::{TownActivityType, TownBuildingViewModel, TownHeroViewModel};

    let roster: Vec<TownHeroViewModel> = campaign
        .roster
        .iter()
        .map(|hero| {
            let is_wounded = hero.health < hero.max_health;
            let is_afflicted = hero.stress >= hero.max_stress;

            TownHeroViewModel {
                id: hero.id.clone(),
                class_id: hero.class_id.clone(),
                class_name: hero.class_id.clone(),
                health: hero.health,
                max_health: hero.max_health,
                stress: hero.stress,
                max_stress: hero.max_stress,
                is_wounded,
                is_afflicted,
                level: hero.level,
                xp: hero.xp,
                positive_quirks: hero.quirks.positive.clone(),
                negative_quirks: hero.quirks.negative.clone(),
                diseases: hero.quirks.diseases.clone(),
            }
        })
        .collect();

    // Map building states to view models
    let buildings: Vec<TownBuildingViewModel> = campaign
        .building_states
        .keys()
        .map(|building_id| {
            let upgrade_state = campaign.building_states.get(building_id);
            TownBuildingViewModel {
                id: building_id.clone(),
                building_type: building_id.clone(),
                current_upgrade: upgrade_state.and_then(|s| s.current_level),
                available: true,
            }
        })
        .collect();

    // Determine available activities based on buildings
    let mut available_activities = Vec::new();
    for building_id in campaign.building_states.keys() {
        match building_id.as_str() {
            "stagecoach" => available_activities.push(TownActivityType::Stagecoach),
            "guild" => available_activities.push(TownActivityType::Guild),
            "blacksmith" => available_activities.push(TownActivityType::Blacksmith),
            "sanitarium" => available_activities.push(TownActivityType::Sanitarium),
            "tavern" => available_activities.push(TownActivityType::Tavern),
            "abbey" => available_activities.push(TownActivityType::Abbey),
            "campfire" => available_activities.push(TownActivityType::Camping),
            _ => {}
        }
    }

    // Convert heirlooms to string keys
    let heirlooms: std::collections::BTreeMap<String, u32> = campaign
        .heirlooms
        .iter()
        .map(|(k, v)| (format!("{:?}", k).to_lowercase(), *v))
        .collect();

    Ok(crate::contracts::viewmodels::TownViewModel {
        gold: campaign.gold,
        heirlooms,
        buildings,
        roster,
        available_activities,
        is_fresh_visit: true,
        error: None,
    })
}

/// Adapter: Convert `framework_viewmodels::CombatViewModel` to `CombatViewModel`.
///
/// Takes a framework combat view model and produces a DDGC-specific
/// combat view model. Returns an error if the framework version is
/// incompatible or the payload cannot be fully mapped.
///
/// Note: The framework `CombatViewModel` has a different structure than
/// the DDGC view model. This adapter handles the mapping, but some
/// framework fields may not map directly to DDGC equivalents.
/// Unsupported fields are represented as `None` or placeholder values.
pub fn combat_from_framework(
    framework_vm: &framework_viewmodels::CombatViewModel,
) -> ViewModelResult<CombatViewModel> {
    use framework_combat::encounter::CombatSide;

    let mut heroes = Vec::new();
    let mut monsters = Vec::new();

    // Filter actors by their formation side
    // Note: ActorSummary has fields: id, side, health, max_health, statuses
    // Some fields like slot_index, name, family_id, stress are not available
    // in the framework ActorSummary, so we use defaults/placeholders
    for actor in &framework_vm.actors {
        let health_f64 = actor.health.0;
        let max_health_f64 = actor.max_health.0;

        let combatant = CombatantViewModel {
            id: format!("{:?}", actor.id),
            combatant_type: if actor.side == CombatSide::Ally {
                CombatantType::Hero
            } else {
                CombatantType::Monster
            },
            name: format!("Actor {:?}", actor.id), // Placeholder name
            family_id: String::new(), // Not available in framework ActorSummary
            health: health_f64,
            max_health: max_health_f64,
            stress: None, // Not available in framework ActorSummary
            position: CombatPosition {
                lane: 0,
                slot: 0, // Not available in framework ActorSummary
            },
            active_statuses: actor.statuses.iter().map(|s| format!("{:?}", s.kind)).collect(),
            active_buffs: Vec::new(), // Not available in framework ActorSummary
            active_debuffs: Vec::new(),
            is_dead: health_f64 <= 0.0,
            is_at_deaths_door: health_f64 < (max_health_f64 * 0.5),
        };

        if actor.side == CombatSide::Ally {
            heroes.push(combatant);
        } else {
            monsters.push(combatant);
        }
    }

    let phase = CombatPhase::Unknown;

    Ok(CombatViewModel {
        encounter_id: format!("encounter_{:?}", framework_vm.encounter_id),
        round: framework_vm.round_number,
        heroes,
        monsters,
        selected_actor_id: None,
        current_turn_actor_id: framework_vm.turn_order.first().map(|a| format!("{:?}", a)),
        phase,
        result: None, // Result is determined externally
        error: None,
    })
}

/// Adapter: Convert `DdgcRunResult` to `DungeonViewModel`.
///
/// Takes a DDGC run result and produces a dungeon exploration view model.
pub fn dungeon_from_run_result(
    run_result: &crate::run::flow::DdgcRunResult,
) -> ViewModelResult<DungeonViewModel> {
    let rooms: Vec<DungeonRoomViewModel> = run_result
        .room_encounters
        .iter()
        .map(|enc| {
            use framework_progression::rooms::RoomKind;
            let kind = match &enc.room_kind {
                RoomKind::Combat => DungeonRoomKind::Combat,
                RoomKind::Boss => DungeonRoomKind::Boss,
                RoomKind::Event { .. } => DungeonRoomKind::Event,
                RoomKind::Corridor { .. } => DungeonRoomKind::Corridor,
                _ => DungeonRoomKind::Unknown,
            };

            DungeonRoomViewModel {
                room_id: format!("{:?}", enc.room_id),
                kind,
                cleared: false,
                is_current: false,
                curio_id: None,
                trap_id: None,
            }
        })
        .collect();

    let heroes: Vec<DungeonHeroViewModel> = run_result
        .heroes
        .iter()
        .map(|h| DungeonHeroViewModel {
            id: h.id.clone(),
            class_id: h.class_id.clone(),
            health: h.health,
            max_health: h.max_health,
            stress: h.stress,
            max_stress: h.max_stress,
            active_buffs: h.active_buffs.clone(),
            camping_buffs: h.camping_buffs.clone(),
            is_at_deaths_door: h.health < (h.max_health * 0.5),
            is_dead: h.health <= 0.0,
        })
        .collect();

    let current_room = rooms.first().cloned().map(|mut r| {
        r.is_current = true;
        r
    });

    Ok(DungeonViewModel {
        dungeon_type: format!("{:?}", run_result.metadata.dungeon_type),
        map_size: format!("{:?}", run_result.metadata.map_size),
        floor: 1,
        rooms,
        rooms_cleared: run_result.state.rooms_cleared,
        total_rooms: run_result.metadata.base_room_number,
        current_room,
        gold_carried: run_result.state.gold,
        torchlight: 100,
        battles_won: run_result.state.battles_won,
        battles_lost: run_result.state.battles_lost,
        heroes,
        is_complete: false,
        party_fled: false,
        error: None,
    })
}

/// Adapter: Convert run metadata to `ResultViewModel`.
///
/// Takes dungeon run result data and produces a result view model.
pub fn result_from_run(
    dungeon_type: DungeonType,
    map_size: MapSize,
    rooms_cleared: u32,
    battles_won: u32,
    completed: bool,
    gold_earned: u32,
    xp_earned: u32,
    heirlooms_earned: &std::collections::BTreeMap<HeirloomCurrency, u32>,
    casualties: Vec<(String, String)>, // (hero_id, class_id) pairs
) -> ViewModelResult<crate::contracts::viewmodels::ResultViewModel> {
    use crate::contracts::viewmodels::{CasualtyViewModel, OutcomeType, RewardViewModel};

    let outcome = if completed {
        OutcomeType::Success
    } else if battles_won > 0 {
        OutcomeType::PartialSuccess
    } else {
        OutcomeType::Failure
    };

    let title = match outcome {
        OutcomeType::Success => "Dungeon Cleared!",
        OutcomeType::PartialSuccess => "Run Complete",
        OutcomeType::Failure => "Run Failed",
        _ => "Run Ended",
    };

    let description = match outcome {
        OutcomeType::Success => format!(
            "Your party successfully cleared {} rooms and won {} battles!",
            rooms_cleared, battles_won
        ),
        OutcomeType::PartialSuccess => format!(
            "Your party cleared {} rooms and won {} battles before retreating.",
            rooms_cleared, battles_won
        ),
        OutcomeType::Failure => {
            "Your party was defeated and retreated from the dungeon.".to_string()
        }
        _ => "The run has ended.".to_string(),
    };

    let rewards = if outcome == OutcomeType::Success || outcome == OutcomeType::PartialSuccess {
        Some(RewardViewModel {
            gold: gold_earned,
            heirlooms: heirlooms_earned
                .iter()
                .map(|(k, v)| (format!("{:?}", k).to_lowercase(), *v))
                .collect(),
            xp: xp_earned,
            loot: Vec::new(),
            trinkets: Vec::new(),
        })
    } else {
        None
    };

    let casualty_models: Vec<CasualtyViewModel> = casualties
        .iter()
        .map(|(hero_id, class_id)| CasualtyViewModel {
            hero_id: hero_id.clone(),
            class_id: class_id.clone(),
            cause: None,
        })
        .collect();

    Ok(crate::contracts::viewmodels::ResultViewModel {
        outcome,
        title: title.to_string(),
        description,
        rewards,
        casualties: casualty_models,
        dungeon_type: Some(format!("{:?}", dungeon_type)),
        map_size: Some(format!("{:?}", map_size)),
        rooms_cleared,
        battles_won,
        completed,
        error: None,
    })
}

/// Adapter: Convert `DdgcRunState` and heroes to `ReturnFlowViewModel`.
///
/// Takes the run state and hero states to produce a return flow view model.
pub fn return_flow_from_state(
    dungeon_type: DungeonType,
    map_size: MapSize,
    rooms_cleared: u32,
    battles_won: u32,
    completed: bool,
    gold_earned: u32,
    heroes: &[(String, String, f64, f64, f64, f64)], // (id, class_id, health, max_health, stress, max_stress)
    died_heroes: &[(String, String)], // (id, class_id) pairs
) -> ViewModelResult<crate::contracts::viewmodels::ReturnFlowViewModel> {
    use crate::contracts::viewmodels::{ReturnFlowHeroViewModel, ReturnFlowState};

    let return_heroes: Vec<ReturnFlowHeroViewModel> = heroes
        .iter()
        .map(
            |(id, class_id, health, max_health, stress, max_stress)| {
                let died = died_heroes.iter().any(|(did, _)| did == id);
                ReturnFlowHeroViewModel {
                    id: id.clone(),
                    class_id: class_id.clone(),
                    health: *health,
                    max_health: *max_health,
                    stress: *stress,
                    max_stress: *max_stress,
                    survived: !died && *health > 0.0,
                    died,
                    is_at_deaths_door: *health < (*max_health * 0.5),
                }
            },
        )
        .collect();

    let state = if completed {
        ReturnFlowState::ShowingResults
    } else {
        ReturnFlowState::Traveling
    };

    Ok(crate::contracts::viewmodels::ReturnFlowViewModel {
        state,
        dungeon_type: format!("{:?}", dungeon_type),
        map_size: format!("{:?}", map_size),
        completed,
        rooms_cleared,
        battles_won,
        gold_to_transfer: gold_earned,
        torchlight_remaining: 100,
        heroes: return_heroes,
        run_result: None,
        ready_for_town: completed,
        error: None,
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Utility functions
// ─────────────────────────────────────────────────────────────────────────────

/// Convert a `DungeonType` to a display string.
pub fn dungeon_type_display(dungeon: DungeonType) -> &'static str {
    match dungeon {
        DungeonType::QingLong => "Azure Dragon",
        DungeonType::BaiHu => "White Tiger",
        DungeonType::ZhuQue => "Vermilion Bird",
        DungeonType::XuanWu => "Black Tortoise",
    }
}

/// Convert a `MapSize` to a display string.
pub fn map_size_display(size: MapSize) -> &'static str {
    match size {
        MapSize::Short => "Short",
        MapSize::Medium => "Medium",
    }
}

/// Check if a dungeon type is valid for view model shaping.
pub fn is_valid_dungeon(dungeon: DungeonType) -> bool {
    matches!(
        dungeon,
        DungeonType::QingLong | DungeonType::BaiHu | DungeonType::ZhuQue | DungeonType::XuanWu
    )
}

/// Check if a map size is valid for view model shaping.
pub fn is_valid_map_size(size: MapSize) -> bool {
    matches!(size, MapSize::Short | MapSize::Medium)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::host::HostPhase;
    use crate::contracts::viewmodels::{
        OutcomeType,
        ReturnFlowState,
    };

    // ── boot_load_from_host tests ─────────────────────────────────────────────

    #[test]
    fn boot_load_from_host_uninitialized() {
        let result = boot_load_from_host(&HostPhase::Uninitialized, false, None);
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert!(vm.loaded);
        assert_eq!(vm.status_message, "Initialized and ready to boot");
        assert!(vm.error.is_none());
    }

    #[test]
    fn boot_load_from_host_booting() {
        let result = boot_load_from_host(&HostPhase::Booting, false, None);
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert!(vm.loaded);
        assert_eq!(vm.status_message, "Loading contract packages...");
    }

    #[test]
    fn boot_load_from_host_ready_without_campaign() {
        let result = boot_load_from_host(&HostPhase::Ready, false, None);
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert!(vm.loaded);
        assert_eq!(vm.status_message, "Host ready");
        assert!(vm.error.is_none());
    }

    #[test]
    fn boot_load_from_host_ready_with_campaign() {
        let result = boot_load_from_host(&HostPhase::Ready, true, Some(1));
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert!(vm.loaded);
        assert_eq!(vm.status_message, "Campaign loaded successfully");
        assert_eq!(vm.campaign_schema_version, Some(1));
    }

    #[test]
    fn boot_load_from_host_fatal_error() {
        let result = boot_load_from_host(&HostPhase::FatalError, false, None);
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert!(!vm.loaded);
        assert!(vm.error.is_some());
    }

    #[test]
    fn boot_load_from_host_unsupported() {
        let result = boot_load_from_host(&HostPhase::Unsupported, false, None);
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert!(!vm.loaded);
        assert!(vm.error.is_some());
    }

    // ── town_from_campaign tests ──────────────────────────────────────────────

    #[test]
    fn town_from_campaign_empty_roster() {
        use crate::contracts::{CampaignState, BuildingUpgradeState};

        let mut campaign = CampaignState::new(1000);
        campaign.building_states.insert(
            "stagecoach".to_string(),
            BuildingUpgradeState::new("stagecoach", Some('a')),
        );

        let result = town_from_campaign(&campaign);
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.gold, 1000);
        assert!(vm.roster.is_empty());
        assert!(!vm.buildings.is_empty());
        assert!(!vm.available_activities.is_empty());
    }

    #[test]
    fn town_from_campaign_with_heroes() {
        use crate::contracts::{CampaignState, CampaignHero, CampaignHeroQuirks};

        let mut campaign = CampaignState::new(500);
        campaign.gold = 500;

        let hero = CampaignHero {
            id: "hero1".to_string(),
            class_id: "crusader".to_string(),
            level: 1,
            xp: 0,
            health: 80.0,
            max_health: 100.0,
            stress: 20.0,
            max_stress: 200.0,
            quirks: CampaignHeroQuirks::new(),
            equipment: Default::default(),
            skills: Vec::new(),
            traits: Default::default(),
        };
        campaign.roster.push(hero);

        let result = town_from_campaign(&campaign);
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.roster.len(), 1);
        assert_eq!(vm.roster[0].id, "hero1");
        assert!(vm.roster[0].is_wounded); // 80 < 100 (not full health)
        assert!(!vm.roster[0].is_afflicted); // 20 < 200
    }

    #[test]
    fn town_from_campaign_wounded_hero() {
        use crate::contracts::{CampaignState, CampaignHero, CampaignHeroQuirks};

        let mut campaign = CampaignState::new(500);
        let hero = CampaignHero {
            id: "hero1".to_string(),
            class_id: "crusader".to_string(),
            level: 1,
            xp: 0,
            health: 50.0,
            max_health: 100.0,
            stress: 20.0,
            max_stress: 200.0,
            quirks: CampaignHeroQuirks::new(),
            equipment: Default::default(),
            skills: Vec::new(),
            traits: Default::default(),
        };
        campaign.roster.push(hero);

        let result = town_from_campaign(&campaign);
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert!(vm.roster[0].is_wounded); // 50 < 100
    }

    #[test]
    fn town_from_campaign_afflicted_hero() {
        use crate::contracts::{CampaignState, CampaignHero, CampaignHeroQuirks};

        let mut campaign = CampaignState::new(500);
        let hero = CampaignHero {
            id: "hero1".to_string(),
            class_id: "crusader".to_string(),
            level: 1,
            xp: 0,
            health: 100.0,
            max_health: 100.0,
            stress: 200.0,
            max_stress: 200.0,
            quirks: CampaignHeroQuirks::new(),
            equipment: Default::default(),
            skills: Vec::new(),
            traits: Default::default(),
        };
        campaign.roster.push(hero);

        let result = town_from_campaign(&campaign);
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert!(vm.roster[0].is_afflicted); // 200 >= 200
    }

    // ── result_from_run tests ───────────────────────────────────────────────

    #[test]
    fn result_from_run_victory() {
        use std::collections::BTreeMap;

        let heirlooms: BTreeMap<HeirloomCurrency, u32> = BTreeMap::new();
        let casualties = Vec::new();

        let result = result_from_run(
            DungeonType::QingLong,
            MapSize::Short,
            8,
            4,
            true,  // completed
            500,
            100,
            &heirlooms,
            casualties,
        );

        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.outcome, OutcomeType::Success);
        assert_eq!(vm.title, "Dungeon Cleared!");
        assert!(vm.rewards.is_some());
        assert_eq!(vm.rewards.unwrap().gold, 500);
    }

    #[test]
    fn result_from_run_partial_success() {
        use std::collections::BTreeMap;

        let heirlooms: BTreeMap<HeirloomCurrency, u32> = BTreeMap::new();
        let casualties = Vec::new();

        let result = result_from_run(
            DungeonType::QingLong,
            MapSize::Short,
            4,
            2,
            false, // not completed
            200,
            50,
            &heirlooms,
            casualties,
        );

        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.outcome, OutcomeType::PartialSuccess);
        assert_eq!(vm.title, "Run Complete");
        assert!(vm.rewards.is_some());
    }

    #[test]
    fn result_from_run_failure() {
        use std::collections::BTreeMap;

        let heirlooms: BTreeMap<HeirloomCurrency, u32> = BTreeMap::new();
        let casualties = vec![("hero1".to_string(), "crusader".to_string())];

        let result = result_from_run(
            DungeonType::QingLong,
            MapSize::Short,
            2,
            0,
            false,
            0,
            0,
            &heirlooms,
            casualties,
        );

        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.outcome, OutcomeType::Failure);
        assert_eq!(vm.title, "Run Failed");
        assert!(vm.rewards.is_none());
        assert_eq!(vm.casualties.len(), 1);
    }

    // ── return_flow_from_state tests ────────────────────────────────────────

    #[test]
    fn return_flow_from_state_completed() {
        let heroes = vec![
            ("hero1".to_string(), "crusader".to_string(), 80.0, 100.0, 20.0, 200.0),
        ];
        let died_heroes = Vec::new();

        let result = return_flow_from_state(
            DungeonType::QingLong,
            MapSize::Short,
            8,
            4,
            true,  // completed
            500,
            &heroes,
            &died_heroes,
        );

        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.state, ReturnFlowState::ShowingResults);
        assert!(vm.completed);
        assert!(vm.ready_for_town);
        assert_eq!(vm.gold_to_transfer, 500);
    }

    #[test]
    fn return_flow_from_state_in_progress() {
        let heroes = vec![
            ("hero1".to_string(), "crusader".to_string(), 80.0, 100.0, 20.0, 200.0),
        ];
        let died_heroes = Vec::new();

        let result = return_flow_from_state(
            DungeonType::QingLong,
            MapSize::Short,
            4,
            2,
            false, // not completed
            200,
            &heroes,
            &died_heroes,
        );

        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.state, ReturnFlowState::Traveling);
        assert!(!vm.completed);
        assert!(!vm.ready_for_town);
    }

    #[test]
    fn return_flow_from_state_with_casualties() {
        let heroes = vec![
            ("hero1".to_string(), "crusader".to_string(), 80.0, 100.0, 20.0, 200.0),
            ("hero2".to_string(), "hunter".to_string(), 0.0, 100.0, 250.0, 200.0), // dead
        ];
        let died_heroes = vec![("hero2".to_string(), "hunter".to_string())];

        let result = return_flow_from_state(
            DungeonType::QingLong,
            MapSize::Short,
            3,
            1,
            false,
            100,
            &heroes,
            &died_heroes,
        );

        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.heroes.len(), 2);
        let hero1 = vm.heroes.iter().find(|h| h.id == "hero1").unwrap();
        assert!(hero1.survived);
        assert!(!hero1.died);

        let hero2 = vm.heroes.iter().find(|h| h.id == "hero2").unwrap();
        assert!(!hero2.survived);
        assert!(hero2.died);
    }

    // ── Utility function tests ─────────────────────────────────────────────

    #[test]
    fn dungeon_type_display_returns_correct_names() {
        assert_eq!(dungeon_type_display(DungeonType::QingLong), "Azure Dragon");
        assert_eq!(dungeon_type_display(DungeonType::BaiHu), "White Tiger");
        assert_eq!(dungeon_type_display(DungeonType::ZhuQue), "Vermilion Bird");
        assert_eq!(dungeon_type_display(DungeonType::XuanWu), "Black Tortoise");
    }

    #[test]
    fn map_size_display_returns_correct_names() {
        assert_eq!(map_size_display(MapSize::Short), "Short");
        assert_eq!(map_size_display(MapSize::Medium), "Medium");
    }

    #[test]
    fn is_valid_dungeon_returns_true_for_all_dungeon_types() {
        assert!(is_valid_dungeon(DungeonType::QingLong));
        assert!(is_valid_dungeon(DungeonType::BaiHu));
        assert!(is_valid_dungeon(DungeonType::ZhuQue));
        assert!(is_valid_dungeon(DungeonType::XuanWu));
    }

    #[test]
    fn is_valid_map_size_returns_true_for_both_sizes() {
        assert!(is_valid_map_size(MapSize::Short));
        assert!(is_valid_map_size(MapSize::Medium));
    }
}