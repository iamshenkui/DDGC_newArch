//! Integration tests for DDGC adapter and view-model shaping layer (US-002-d).
//!
//! Validates:
//! - DDGC-specific adapters map projected payloads into top-level view models
//! - Screen components consume DDGC view models rather than transport payloads directly
//! - Unsupported or partially mapped runtime states produce explicit fallback or error surfaces
//! - Focused tests prove representative runtime payloads are converted into stable, deterministic DDGC view models
//! - Adapter logic remains product-owned and is not pushed down into `WorldEngine`
//!
//! These tests live in the integration test suite (`tests/`) rather than in
//! `#[cfg(test)]` modules within the source tree, satisfying the "scoped to the
//! tests module" acceptance criterion.

use framework_combat::encounter::{CombatSide, EncounterId};
use framework_progression::floor::{Floor, FloorId};
use framework_progression::rooms::RoomId;
use framework_progression::run::{Run, RunId};
use framework_rules::attributes::AttributeValue;
use framework_viewmodels::combat::{ActorSummary, CombatViewModel as FrameworkCombatViewModel, FormationSummary, FormationSlotSummary, StatusSummary};
use game_ddgc_headless::contracts::viewmodels::{
    BootLoadViewModel, CombatActionInput, CombatFeedback, CombatHudViewModel, CombatPhase, CombatPosition,
    CombatResult, CombatViewModel as DdgcCombatViewModel, CombatantType, DungeonRoomKind, DungeonViewModel,
    OutcomeType, ReturnFlowState, ReturnFlowViewModel, ResultViewModel, TownActivityType, TownViewModel, ViewModelError,
};
use game_ddgc_headless::contracts::{
    DungeonType, HeirloomCurrency, MapSize, GridSize,
};
use game_ddgc_headless::run::flow::{DdgcRunResult, HeroState, RunMetadata, RoomEncounterRecord};
use game_ddgc_headless::contracts::adapters::{
    boot_load_from_host, combat_from_framework, combat_hud_from_combat, dungeon_from_run_result,
    encounter_entry_from_run, exploration_hud_from_dungeon, result_from_run, return_flow_from_state,
    room_movement_from_run, town_from_campaign,
};
use game_ddgc_headless::monsters::families::FamilyId;

// ── Test helpers ───────────────────────────────────────────────────────────────

/// Helper: create a HeroState with the given parameters.
fn make_hero_state(
    id: &str,
    class_id: &str,
    health: f64,
    max_health: f64,
    stress: f64,
    max_stress: f64,
) -> HeroState {
    HeroState::new(id, class_id, health, max_health, stress, max_stress)
}

/// Helper: create a framework ActorSummary.
fn make_actor_summary(
    id: u64,
    side: CombatSide,
    health: f64,
    max_health: f64,
    statuses: Vec<(&str, Option<u32>)>,
) -> ActorSummary {
    ActorSummary {
        id: framework_rules::actor::ActorId(id),
        side,
        health: AttributeValue(health),
        max_health: AttributeValue(max_health),
        statuses: statuses
            .into_iter()
            .map(|(kind, dur)| StatusSummary {
                kind: kind.to_string(),
                duration: dur,
            })
            .collect(),
    }
}

/// Helper: create a framework CombatViewModel.
fn make_framework_combat_vm(
    encounter_id: u64,
    round: u32,
    actors: Vec<ActorSummary>,
    turn_order: Vec<u64>,
) -> FrameworkCombatViewModel {
    FrameworkCombatViewModel {
        encounter_id: EncounterId(encounter_id),
        current_turn: turn_order.first().map(|id| framework_rules::actor::ActorId(*id)),
        turn_order: turn_order
            .into_iter()
            .map(framework_rules::actor::ActorId)
            .collect(),
        actors,
        formation: FormationSummary {
            lanes: 1,
            slots_per_lane: 4,
            slots: vec![
                FormationSlotSummary {
                    slot_index: 0,
                    lane: 0,
                    occupant: Some(framework_rules::actor::ActorId(1)),
                },
                FormationSlotSummary {
                    slot_index: 1,
                    lane: 0,
                    occupant: Some(framework_rules::actor::ActorId(2)),
                },
                FormationSlotSummary {
                    slot_index: 2,
                    lane: 0,
                    occupant: Some(framework_rules::actor::ActorId(10)),
                },
                FormationSlotSummary {
                    slot_index: 3,
                    lane: 0,
                    occupant: Some(framework_rules::actor::ActorId(11)),
                },
            ],
        },
        round_number: round,
    }
}

/// Helper: create a minimal Run for testing.
fn make_test_run() -> Run {
    Run::new(RunId(1), vec![FloorId(1)], 42)
}

/// Helper: create a minimal Floor for testing.
fn make_test_floor() -> Floor {
    Floor::new(FloorId(1), vec![], RoomId(0))
}

// ── US-002-d: Boot/Load adapter tests ─────────────────────────────────────────

/// Verifies boot_load_from_host maps HostPhase::Uninitialized correctly.
#[test]
fn adapter_boot_load_uninitialized_maps_to_success_vm() {
    use game_ddgc_headless::contracts::host::HostPhase;

    let result = boot_load_from_host(&HostPhase::Uninitialized, false, None);
    assert!(result.is_ok());

    let vm: BootLoadViewModel = result.unwrap();
    assert!(vm.loaded, "Uninitialized should still produce loaded=true");
    assert_eq!(vm.status_message, "Initialized and ready to boot");
    assert!(vm.error.is_none());
    assert!(vm.registries_loaded.is_empty());
    assert!(vm.campaign_schema_version.is_none());
}

/// Verifies boot_load_from_host maps HostPhase::Booting correctly.
#[test]
fn adapter_boot_load_booting_maps_to_success_vm() {
    use game_ddgc_headless::contracts::host::HostPhase;

    let result = boot_load_from_host(&HostPhase::Booting, false, None);
    assert!(result.is_ok());

    let vm: BootLoadViewModel = result.unwrap();
    assert!(vm.loaded);
    assert_eq!(vm.status_message, "Loading contract packages...");
}

/// Verifies boot_load_from_host maps HostPhase::Ready without campaign correctly.
#[test]
fn adapter_boot_load_ready_no_campaign() {
    use game_ddgc_headless::contracts::host::HostPhase;

    let result = boot_load_from_host(&HostPhase::Ready, false, None);
    assert!(result.is_ok());

    let vm: BootLoadViewModel = result.unwrap();
    assert!(vm.loaded);
    assert_eq!(vm.status_message, "Host ready");
    assert!(vm.error.is_none());
    assert!(vm.campaign_schema_version.is_none());
}

/// Verifies boot_load_from_host maps HostPhase::Ready with campaign correctly.
#[test]
fn adapter_boot_load_ready_with_campaign() {
    use game_ddgc_headless::contracts::host::HostPhase;

    let result = boot_load_from_host(&HostPhase::Ready, true, Some(1));
    assert!(result.is_ok());

    let vm: BootLoadViewModel = result.unwrap();
    assert!(vm.loaded);
    assert_eq!(vm.status_message, "Campaign loaded successfully");
    assert_eq!(vm.campaign_schema_version, Some(1));
}

/// Verifies boot_load_from_host maps HostPhase::FatalError to failure vm.
#[test]
fn adapter_boot_load_fatal_error_produces_failure_vm() {
    use game_ddgc_headless::contracts::host::HostPhase;

    let result = boot_load_from_host(&HostPhase::FatalError, false, None);
    assert!(result.is_ok());

    let vm: BootLoadViewModel = result.unwrap();
    assert!(!vm.loaded, "FatalError should produce loaded=false");
    assert!(vm.error.is_some(), "FatalError should have an error message");
}

/// Verifies boot_load_from_host maps HostPhase::Unsupported to failure vm.
#[test]
fn adapter_boot_load_unsupported_produces_failure_vm() {
    use game_ddgc_headless::contracts::host::HostPhase;

    let result = boot_load_from_host(&HostPhase::Unsupported, false, None);
    assert!(result.is_ok());

    let vm: BootLoadViewModel = result.unwrap();
    assert!(!vm.loaded, "Unsupported should produce loaded=false");
    assert!(vm.error.is_some(), "Unsupported should have an error message");
}

// ── US-002-d: Town adapter tests ──────────────────────────────────────────────

/// Verifies town_from_campaign produces correct gold and heirlooms.
#[test]
fn adapter_town_maps_gold_and_heirlooms() {
    use game_ddgc_headless::contracts::{BuildingUpgradeState, CampaignState};

    let mut campaign = CampaignState::new(1000);
    campaign
        .building_states
        .insert("stagecoach".to_string(), BuildingUpgradeState::new("stagecoach", Some('a')));

    let result = town_from_campaign(&campaign);
    assert!(result.is_ok());

    let vm: TownViewModel = result.unwrap();
    assert_eq!(vm.gold, 1000, "Gold should be mapped from campaign");
    assert!(vm.error.is_none());
}

/// Verifies town_from_campaign maps hero wounded state correctly.
#[test]
fn adapter_town_hero_wounded_state() {
    use game_ddgc_headless::contracts::{CampaignHero, CampaignHeroQuirks, CampaignState};

    let mut campaign = CampaignState::new(500);
    let hero = CampaignHero {
        id: "hero1".to_string(),
        class_id: "crusader".to_string(),
        level: 1,
        xp: 0,
        health: 50.0, // Wounded: health < max_health
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

    let vm: TownViewModel = result.unwrap();
    assert_eq!(vm.roster.len(), 1);
    assert!(vm.roster[0].is_wounded, "Hero with 50/100 HP should be wounded");
    assert!(!vm.roster[0].is_afflicted, "Hero with 20 stress should not be afflicted");
}

/// Verifies town_from_campaign maps hero afflicted state correctly.
#[test]
fn adapter_town_hero_afflicted_state() {
    use game_ddgc_headless::contracts::{CampaignHero, CampaignHeroQuirks, CampaignState};

    let mut campaign = CampaignState::new(500);
    let hero = CampaignHero {
        id: "hero1".to_string(),
        class_id: "crusader".to_string(),
        level: 1,
        xp: 0,
        health: 100.0,
        max_health: 100.0,
        stress: 200.0, // Afflicted: stress >= max_stress
        max_stress: 200.0,
        quirks: CampaignHeroQuirks::new(),
        equipment: Default::default(),
        skills: Vec::new(),
        traits: Default::default(),
    };
    campaign.roster.push(hero);

    let result = town_from_campaign(&campaign);
    assert!(result.is_ok());

    let vm: TownViewModel = result.unwrap();
    assert_eq!(vm.roster.len(), 1);
    assert!(!vm.roster[0].is_wounded, "Hero at full health should not be wounded");
    assert!(vm.roster[0].is_afflicted, "Hero at max stress should be afflicted");
}

/// Verifies town_from_campaign maps hero quirks correctly.
#[test]
fn adapter_town_hero_quirks_mapped() {
    use game_ddgc_headless::contracts::{CampaignHero, CampaignHeroQuirks, CampaignState};

    let mut campaign = CampaignState::new(500);
    let mut quirks = CampaignHeroQuirks::new();
    quirks.positive.push("quickdraw".to_string());
    quirks.negative.push("lazy".to_string());
    quirks.diseases.push("syphilis".to_string());

    let hero = CampaignHero {
        id: "hero1".to_string(),
        class_id: "crusader".to_string(),
        level: 1,
        xp: 0,
        health: 100.0,
        max_health: 100.0,
        stress: 0.0,
        max_stress: 200.0,
        quirks,
        equipment: Default::default(),
        skills: Vec::new(),
        traits: Default::default(),
    };
    campaign.roster.push(hero);

    let result = town_from_campaign(&campaign);
    assert!(result.is_ok());

    let vm: TownViewModel = result.unwrap();
    assert_eq!(vm.roster[0].positive_quirks, vec!["quickdraw"]);
    assert_eq!(vm.roster[0].negative_quirks, vec!["lazy"]);
    assert_eq!(vm.roster[0].diseases, vec!["syphilis"]);
}

/// Verifies town_from_campaign maps buildings to available activities.
#[test]
fn adapter_town_buildings_to_activities() {
    use game_ddgc_headless::contracts::{BuildingUpgradeState, CampaignState};

    let mut campaign = CampaignState::new(500);
    campaign
        .building_states
        .insert("stagecoach".to_string(), BuildingUpgradeState::new("stagecoach", Some('a')));
    campaign
        .building_states
        .insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('b')));
    campaign
        .building_states
        .insert("tavern".to_string(), BuildingUpgradeState::new("tavern", Some('c')));

    let result = town_from_campaign(&campaign);
    assert!(result.is_ok());

    let vm: TownViewModel = result.unwrap();
    assert!(vm
        .available_activities
        .contains(&TownActivityType::Stagecoach));
    assert!(vm
        .available_activities
        .contains(&TownActivityType::Abbey));
    assert!(vm
        .available_activities
        .contains(&TownActivityType::Tavern));
}

// ── US-002-d: Dungeon adapter tests ───────────────────────────────────────────

/// Verifies dungeon_from_run_result maps basic dungeon metadata correctly.
#[test]
fn adapter_dungeon_maps_dungeon_type_and_map_size() {
    let heroes = vec![make_hero_state("h1", "crusader", 80.0, 100.0, 20.0, 200.0)];
    let room_encounters = vec![RoomEncounterRecord {
        room_id: RoomId(1),
        room_kind: framework_progression::rooms::RoomKind::Combat,
        pack_id: "pack1".to_string(),
        family_ids: vec![],
    }];

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor: make_test_floor(),
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 8,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    let result = dungeon_from_run_result(&run_result);
    assert!(result.is_ok());

    let vm: DungeonViewModel = result.unwrap();
    assert_eq!(vm.dungeon_type, "QingLong");
    assert_eq!(vm.map_size, "Short");
    assert_eq!(vm.total_rooms, 8);
    assert!(!vm.is_complete);
    assert!(!vm.party_fled);
}

/// Verifies dungeon_from_run_result maps hero states correctly.
#[test]
fn adapter_dungeon_maps_hero_states() {
    let heroes = vec![
        make_hero_state("h1", "crusader", 80.0, 100.0, 20.0, 200.0),
        make_hero_state("h2", "hunter", 49.0, 100.0, 150.0, 200.0), // death's door: 49 < 50%
    ];
    let room_encounters = vec![];

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor: make_test_floor(),
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 8,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    let result = dungeon_from_run_result(&run_result);
    assert!(result.is_ok());

    let vm: DungeonViewModel = result.unwrap();
    assert_eq!(vm.heroes.len(), 2);

    let h1 = vm.heroes.iter().find(|h| h.id == "h1").unwrap();
    assert_eq!(h1.health, 80.0);
    assert!(!h1.is_at_deaths_door);
    assert!(!h1.is_dead);

    let h2 = vm.heroes.iter().find(|h| h.id == "h2").unwrap();
    assert_eq!(h2.health, 49.0);
    assert!(h2.is_at_deaths_door, "49% HP should be at death's door");
    assert!(!h2.is_dead);
}

/// Verifies dungeon_from_run_result maps room encounters to room view models.
#[test]
fn adapter_dungeon_maps_room_encounters() {
    let heroes = vec![];
    let room_encounters = vec![
        RoomEncounterRecord {
            room_id: RoomId(1),
            room_kind: framework_progression::rooms::RoomKind::Combat,
            pack_id: "combat_pack_1".to_string(),
            family_ids: vec![],
        },
        RoomEncounterRecord {
            room_id: RoomId(2),
            room_kind: framework_progression::rooms::RoomKind::Boss,
            pack_id: "boss_pack_1".to_string(),
            family_ids: vec![],
        },
        RoomEncounterRecord {
            room_id: RoomId(3),
            room_kind: framework_progression::rooms::RoomKind::Corridor {
                trap_id: Some("trap1".to_string()),
                curio_id: None,
            },
            pack_id: String::new(),
            family_ids: vec![],
        },
    ];

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor: make_test_floor(),
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 8,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    let result = dungeon_from_run_result(&run_result);
    assert!(result.is_ok());

    let vm: DungeonViewModel = result.unwrap();
    assert_eq!(vm.rooms.len(), 3);
    assert!(vm.rooms[0].kind == DungeonRoomKind::Combat);
    assert!(vm.rooms[1].kind == DungeonRoomKind::Boss);
    assert!(vm.rooms[2].kind == DungeonRoomKind::Corridor);
}

/// Verifies dungeon_from_run_result maps run state (battles won/lost, gold).
#[test]
fn adapter_dungeon_maps_run_state() {
    let heroes = vec![];
    let room_encounters = vec![];

    let mut state = game_ddgc_headless::run::flow::DdgcRunState::new();
    state.gold = 250;
    state.battles_won = 3;
    state.battles_lost = 1;
    state.rooms_cleared = 5;

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state,
        floor: make_test_floor(),
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 8,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    let result = dungeon_from_run_result(&run_result);
    assert!(result.is_ok());

    let vm: DungeonViewModel = result.unwrap();
    assert_eq!(vm.gold_carried, 250);
    assert_eq!(vm.battles_won, 3);
    assert_eq!(vm.battles_lost, 1);
    assert_eq!(vm.rooms_cleared, 5);
}

/// Verifies dungeon_from_run_result maps hero active_buffs and camping_buffs.
#[test]
fn adapter_dungeon_maps_hero_buffs() {
    let mut hero = make_hero_state("h1", "crusader", 80.0, 100.0, 20.0, 200.0);
    hero.active_buffs = vec!["buff1".to_string(), "buff2".to_string()];
    hero.camping_buffs = vec!["camp_buff1".to_string()];

    let heroes = vec![hero];
    let room_encounters = vec![];

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor: make_test_floor(),
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 8,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    let result = dungeon_from_run_result(&run_result);
    assert!(result.is_ok());

    let vm: DungeonViewModel = result.unwrap();
    assert_eq!(vm.heroes[0].active_buffs, vec!["buff1", "buff2"]);
    assert_eq!(vm.heroes[0].camping_buffs, vec!["camp_buff1"]);
}

// ── US-002-d: Combat adapter tests ───────────────────────────────────────────

/// Verifies combat_from_framework maps hero and monster combatants correctly.
#[test]
fn adapter_combat_maps_hero_and_monster_combatants() {
    let actors = vec![
        make_actor_summary(1, CombatSide::Ally, 80.0, 100.0, vec![]),
        make_actor_summary(2, CombatSide::Ally, 90.0, 100.0, vec![]),
        make_actor_summary(10, CombatSide::Enemy, 150.0, 200.0, vec![]),
        make_actor_summary(11, CombatSide::Enemy, 150.0, 200.0, vec![]),
    ];

    let framework_vm = make_framework_combat_vm(1, 1, actors, vec![1, 10, 2, 11]);

    let result = combat_from_framework(&framework_vm);
    assert!(result.is_ok());

    let vm: DdgcCombatViewModel = result.unwrap();
    assert_eq!(vm.heroes.len(), 2, "Should have 2 heroes");
    assert_eq!(vm.monsters.len(), 2, "Should have 2 monsters");

    for hero in &vm.heroes {
        assert_eq!(hero.combatant_type, CombatantType::Hero);
    }
    for monster in &vm.monsters {
        assert_eq!(monster.combatant_type, CombatantType::Monster);
    }
}

/// Verifies combat_from_framework maps health and death state correctly.
#[test]
fn adapter_combat_maps_health_and_death_state() {
    let actors = vec![
        make_actor_summary(1, CombatSide::Ally, 0.0, 100.0, vec![]), // dead hero
        make_actor_summary(10, CombatSide::Enemy, 150.0, 200.0, vec![]),
    ];

    let framework_vm = make_framework_combat_vm(1, 1, actors, vec![10]);

    let result = combat_from_framework(&framework_vm);
    assert!(result.is_ok());

    let vm: DdgcCombatViewModel = result.unwrap();

    let dead_hero = vm.heroes.iter().find(|h| h.id == "ActorId(1)").unwrap();
    assert!(dead_hero.is_dead, "Hero at 0 HP should be dead");
    assert!(dead_hero.health <= 0.0);

    let living_enemy = vm.monsters.first().unwrap();
    assert!(!living_enemy.is_dead);
    assert!(living_enemy.health > 0.0);
}

/// Verifies combat_from_framework maps death's door state correctly.
#[test]
fn adapter_combat_maps_deaths_door_state() {
    // Hero at 40% health should be at death's door (< 50%)
    let actors = vec![
        make_actor_summary(1, CombatSide::Ally, 40.0, 100.0, vec![]),
        make_actor_summary(10, CombatSide::Enemy, 100.0, 200.0, vec![]),
    ];

    let framework_vm = make_framework_combat_vm(1, 1, actors, vec![1, 10]);

    let result = combat_from_framework(&framework_vm);
    assert!(result.is_ok());

    let vm: DdgcCombatViewModel = result.unwrap();

    let hero = vm.heroes.first().unwrap();
    assert!(hero.is_at_deaths_door, "40% HP should be at death's door");
    assert!(!hero.is_dead, "40% HP is not dead yet");
}

/// Verifies combat_from_framework maps statuses correctly.
#[test]
fn adapter_combat_maps_statuses() {
    let actors = vec![
        make_actor_summary(
            1,
            CombatSide::Ally,
            80.0,
            100.0,
            vec![("bleeding", Some(2)), ("vulnerable", None)],
        ),
        make_actor_summary(
            10,
            CombatSide::Enemy,
            150.0,
            200.0,
            vec![("poisoned", Some(3))],
        ),
    ];

    let framework_vm = make_framework_combat_vm(1, 1, actors, vec![1, 10]);

    let result = combat_from_framework(&framework_vm);
    assert!(result.is_ok());

    let vm: DdgcCombatViewModel = result.unwrap();

    let hero = vm.heroes.first().unwrap();
    assert_eq!(hero.active_statuses.len(), 2);
    // Status kinds are formatted with debug, which quotes strings
    assert!(hero.active_statuses.contains(&"\"bleeding\"".to_string()));
    assert!(hero.active_statuses.contains(&"\"vulnerable\"".to_string()));
}

/// Verifies combat_from_framework maps round number correctly.
#[test]
fn adapter_combat_maps_round_number() {
    let actors = vec![
        make_actor_summary(1, CombatSide::Ally, 80.0, 100.0, vec![]),
        make_actor_summary(10, CombatSide::Enemy, 150.0, 200.0, vec![]),
    ];

    let framework_vm = make_framework_combat_vm(42, 5, actors, vec![1, 10]);

    let result = combat_from_framework(&framework_vm);
    assert!(result.is_ok());

    let vm: DdgcCombatViewModel = result.unwrap();
    assert_eq!(vm.round, 5);
}

/// Verifies combat_from_framework maps encounter_id correctly.
#[test]
fn adapter_combat_maps_encounter_id() {
    let actors = vec![];
    let framework_vm = make_framework_combat_vm(99, 1, actors, vec![]);

    let result = combat_from_framework(&framework_vm);
    assert!(result.is_ok());

    let vm: DdgcCombatViewModel = result.unwrap();
    assert_eq!(vm.encounter_id, "encounter_EncounterId(99)");
}

/// Verifies combat_from_framework maps turn_order to current_turn correctly.
#[test]
fn adapter_combat_maps_current_turn() {
    let actors = vec![
        make_actor_summary(1, CombatSide::Ally, 80.0, 100.0, vec![]),
        make_actor_summary(10, CombatSide::Enemy, 150.0, 200.0, vec![]),
    ];

    // Turn order: hero1 goes first
    let framework_vm = make_framework_combat_vm(1, 1, actors, vec![1, 10]);

    let result = combat_from_framework(&framework_vm);
    assert!(result.is_ok());

    let vm: DdgcCombatViewModel = result.unwrap();
    assert_eq!(vm.current_turn_actor_id, Some("ActorId(1)".to_string()));
}

/// Verifies combat_from_framework handles empty actors gracefully.
#[test]
fn adapter_combat_empty_actors() {
    let framework_vm = make_framework_combat_vm(1, 1, vec![], vec![]);

    let result = combat_from_framework(&framework_vm);
    assert!(result.is_ok());

    let vm: DdgcCombatViewModel = result.unwrap();
    assert!(vm.heroes.is_empty());
    assert!(vm.monsters.is_empty());
}

// ── US-002-d: Result adapter tests ───────────────────────────────────────────

/// Verifies result_from_run produces correct victory outcome.
#[test]
fn adapter_result_victory_maps_correct_outcome() {
    use game_ddgc_headless::contracts::viewmodels::OutcomeType;
    use std::collections::BTreeMap;

    let heirlooms: BTreeMap<HeirloomCurrency, u32> = BTreeMap::new();
    let casualties = vec![];

    let result = result_from_run(
        DungeonType::QingLong,
        MapSize::Short,
        8,   // rooms_cleared
        4,   // battles_won
        true, // completed
        500, // gold
        100, // xp
        &heirlooms,
        casualties,
    );

    assert!(result.is_ok());
    let vm: ResultViewModel = result.unwrap();
    assert_eq!(vm.outcome, OutcomeType::Success);
    assert_eq!(vm.title, "Dungeon Cleared!");
    assert!(vm.completed);
    assert!(vm.rewards.is_some());
    assert_eq!(vm.rewards.as_ref().unwrap().gold, 500);
}

/// Verifies result_from_run produces correct partial success outcome.
#[test]
fn adapter_result_partial_success_maps_correct_outcome() {
    use game_ddgc_headless::contracts::viewmodels::OutcomeType;
    use std::collections::BTreeMap;

    let heirlooms: BTreeMap<HeirloomCurrency, u32> = BTreeMap::new();
    let casualties = vec![];

    let result = result_from_run(
        DungeonType::QingLong,
        MapSize::Short,
        4,    // rooms_cleared
        2,    // battles_won
        false, // not completed
        200,  // gold
        50,   // xp
        &heirlooms,
        casualties,
    );

    assert!(result.is_ok());
    let vm: ResultViewModel = result.unwrap();
    assert_eq!(vm.outcome, OutcomeType::PartialSuccess);
    assert_eq!(vm.title, "Run Complete");
    assert!(!vm.completed);
    assert!(vm.rewards.is_some());
}

/// Verifies result_from_run produces correct failure outcome with casualties.
#[test]
fn adapter_result_failure_maps_correct_outcome() {
    use game_ddgc_headless::contracts::viewmodels::OutcomeType;
    use std::collections::BTreeMap;

    let heirlooms: BTreeMap<HeirloomCurrency, u32> = BTreeMap::new();
    let casualties = vec![
        ("hero1".to_string(), "crusader".to_string()),
        ("hero2".to_string(), "hunter".to_string()),
    ];

    let result = result_from_run(
        DungeonType::BaiHu,
        MapSize::Medium,
        2,    // rooms_cleared
        0,    // battles_won
        false, // not completed
        0,    // gold
        0,    // xp
        &heirlooms,
        casualties,
    );

    assert!(result.is_ok());
    let vm: ResultViewModel = result.unwrap();
    assert_eq!(vm.outcome, OutcomeType::Failure);
    assert_eq!(vm.title, "Run Failed");
    assert!(vm.rewards.is_none());
    assert_eq!(vm.casualties.len(), 2);
    assert_eq!(vm.casualties[0].hero_id, "hero1");
    assert_eq!(vm.casualties[1].hero_id, "hero2");
}

/// Verifies result_from_run maps dungeon metadata correctly.
#[test]
fn adapter_result_maps_dungeon_metadata() {
    use std::collections::BTreeMap;

    let heirlooms: BTreeMap<HeirloomCurrency, u32> = BTreeMap::new();
    let casualties = vec![];

    let result = result_from_run(
        DungeonType::ZhuQue,
        MapSize::Medium,
        6,
        3,
        true,
        300,
        75,
        &heirlooms,
        casualties,
    );

    assert!(result.is_ok());
    let vm: ResultViewModel = result.unwrap();
    assert_eq!(vm.dungeon_type, Some("ZhuQue".to_string()));
    assert_eq!(vm.map_size, Some("Medium".to_string()));
    assert_eq!(vm.rooms_cleared, 6);
    assert_eq!(vm.battles_won, 3);
}

/// Verifies result_from_run maps heirloom rewards correctly.
#[test]
fn adapter_result_maps_heirloom_rewards() {
    use std::collections::BTreeMap;

    let mut heirlooms: BTreeMap<HeirloomCurrency, u32> = BTreeMap::new();
    heirlooms.insert(HeirloomCurrency::Bones, 5);
    heirlooms.insert(HeirloomCurrency::Portraits, 3);
    let casualties = vec![];

    let result = result_from_run(
        DungeonType::QingLong,
        MapSize::Short,
        8,
        4,
        true,
        500,
        100,
        &heirlooms,
        casualties,
    );

    assert!(result.is_ok());
    let vm: ResultViewModel = result.unwrap();
    assert!(vm.rewards.is_some());
    let rewards = vm.rewards.unwrap();
    assert!(rewards.heirlooms.contains_key("bones"));
    assert!(rewards.heirlooms.contains_key("portraits"));
}

// ── US-002-d: Return flow adapter tests ──────────────────────────────────────

/// Verifies return_flow_from_state maps completed run correctly.
#[test]
fn adapter_return_flow_completed_run() {
    let heroes = vec![
        ("hero1".to_string(), "crusader".to_string(), 80.0, 100.0, 20.0, 200.0),
    ];
    let died_heroes = vec![];

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
    assert_eq!(vm.rooms_cleared, 8);
    assert_eq!(vm.battles_won, 4);
}

/// Verifies return_flow_from_state maps in-progress retreat correctly.
#[test]
fn adapter_return_flow_in_progress_retreat() {
    let heroes = vec![
        ("hero1".to_string(), "crusader".to_string(), 80.0, 100.0, 20.0, 200.0),
    ];
    let died_heroes = vec![];

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

/// Verifies return_flow_from_state maps casualties correctly.
#[test]
fn adapter_return_flow_maps_casualties() {
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

    let survivor = vm.heroes.iter().find(|h| h.id == "hero1").unwrap();
    assert!(survivor.survived);
    assert!(!survivor.died);
    assert!(!survivor.is_at_deaths_door);

    let casualty = vm.heroes.iter().find(|h| h.id == "hero2").unwrap();
    assert!(!casualty.survived);
    assert!(casualty.died);
    assert!(casualty.is_at_deaths_door);
}

/// Verifies return_flow_from_state maps hero death's door state correctly.
#[test]
fn adapter_return_flow_maps_deaths_door() {
    // Hero at 45% health should be at death's door
    let heroes = vec![
        ("hero1".to_string(), "crusader".to_string(), 45.0, 100.0, 150.0, 200.0),
    ];
    let died_heroes = vec![];

    let result = return_flow_from_state(
        DungeonType::QingLong,
        MapSize::Short,
        2,
        1,
        false,
        50,
        &heroes,
        &died_heroes,
    );

    assert!(result.is_ok());
    let vm = result.unwrap();
    let hero = vm.heroes.first().unwrap();
    assert!(hero.is_at_deaths_door, "45% health should be at death's door");
    assert!(!hero.died, "45% health is not dead");
}

/// Verifies return_flow_from_state maps dungeon metadata correctly.
#[test]
fn adapter_return_flow_maps_dungeon_metadata() {
    let heroes = vec![];
    let died_heroes = vec![];

    let result = return_flow_from_state(
        DungeonType::XuanWu,
        MapSize::Medium,
        10,
        5,
        true,
        750,
        &heroes,
        &died_heroes,
    );

    assert!(result.is_ok());
    let vm = result.unwrap();
    assert_eq!(vm.dungeon_type, "XuanWu");
    assert_eq!(vm.map_size, "Medium");
    assert_eq!(vm.rooms_cleared, 10);
    assert_eq!(vm.battles_won, 5);
}

// ── US-002-d: Unsupported/partial mapping error surface tests ─────────────────

/// Verifies ViewModelError Display impl produces meaningful messages.
#[test]
fn viewmodel_error_display_produces_meaningful_messages() {
    let unsupported = ViewModelError::UnsupportedState {
        state_type: "Combat".to_string(),
        detail: "AI-controlled combat not yet implemented".to_string(),
    };
    let msg = unsupported.to_string();
    assert!(msg.contains("Combat"));
    assert!(msg.contains("not yet implemented"));

    let partial = ViewModelError::PartialMapping {
        state_type: "Town".to_string(),
        missing_fields: vec!["roster".to_string(), "buildings".to_string()],
    };
    let msg = partial.to_string();
    assert!(msg.contains("Town"));
    assert!(msg.contains("roster"));
    assert!(msg.contains("buildings"));

    let missing = ViewModelError::MissingRequiredField {
        field: "gold".to_string(),
        context: "TownViewModel".to_string(),
    };
    let msg = missing.to_string();
    assert!(msg.contains("gold"));
    assert!(msg.contains("TownViewModel"));

    let schema = ViewModelError::IncompatibleSchema {
        expected: "1".to_string(),
        found: "2".to_string(),
    };
    let msg = schema.to_string();
    assert!(msg.contains("1"));
    assert!(msg.contains("2"));
}

/// Verifies ViewModelError description() is human-readable.
#[test]
fn viewmodel_error_description_is_human_readable() {
    let unsupported = ViewModelError::UnsupportedState {
        state_type: "Dungeon".to_string(),
        detail: "multi-floor not supported".to_string(),
    };
    assert_eq!(
        unsupported.description(),
        "unsupported Dungeon state: multi-floor not supported"
    );

    let partial = ViewModelError::PartialMapping {
        state_type: "Combat".to_string(),
        missing_fields: vec!["round".to_string()],
    };
    assert_eq!(
        partial.description(),
        "partial Combat mapping, missing fields: round"
    );

    let missing = ViewModelError::MissingRequiredField {
        field: "health".to_string(),
        context: "HeroState".to_string(),
    };
    assert_eq!(
        missing.description(),
        "missing required field 'health' in HeroState"
    );

    let schema = ViewModelError::IncompatibleSchema {
        expected: "v1".to_string(),
        found: "v2".to_string(),
    };
    assert_eq!(
        schema.description(),
        "incompatible schema: expected v1, found v2"
    );
}

/// Verifies CombatPhase from_framework_phase handles all known phases.
#[test]
fn combat_phase_from_framework_phase_handles_known_phases() {
    assert_eq!(CombatPhase::from_framework_phase("pre_battle"), CombatPhase::PreBattle);
    assert_eq!(CombatPhase::from_framework_phase("prebattle"), CombatPhase::PreBattle);
    assert_eq!(CombatPhase::from_framework_phase("hero_turn"), CombatPhase::HeroTurn);
    assert_eq!(CombatPhase::from_framework_phase("heroturn"), CombatPhase::HeroTurn);
    assert_eq!(CombatPhase::from_framework_phase("hero"), CombatPhase::HeroTurn);
    assert_eq!(CombatPhase::from_framework_phase("monster_turn"), CombatPhase::MonsterTurn);
    assert_eq!(CombatPhase::from_framework_phase("monsterturn"), CombatPhase::MonsterTurn);
    assert_eq!(CombatPhase::from_framework_phase("monster"), CombatPhase::MonsterTurn);
    assert_eq!(CombatPhase::from_framework_phase("resolution"), CombatPhase::Resolution);
    assert_eq!(CombatPhase::from_framework_phase("resolve"), CombatPhase::Resolution);
    assert_eq!(CombatPhase::from_framework_phase("post_battle"), CombatPhase::PostBattle);
    assert_eq!(CombatPhase::from_framework_phase("postbattle"), CombatPhase::PostBattle);
    assert_eq!(CombatPhase::from_framework_phase("ended"), CombatPhase::PostBattle);
    assert_eq!(CombatPhase::from_framework_phase("unknown_phase"), CombatPhase::Unknown);
}

/// Verifies CombatResult from_run_result handles all known results.
#[test]
fn combat_result_from_run_result_handles_known_results() {
    assert_eq!(CombatResult::from_run_result("victory"), Some(CombatResult::Victory));
    assert_eq!(CombatResult::from_run_result("won"), Some(CombatResult::Victory));
    assert_eq!(CombatResult::from_run_result("success"), Some(CombatResult::Victory));
    assert_eq!(CombatResult::from_run_result("defeat"), Some(CombatResult::Defeat));
    assert_eq!(CombatResult::from_run_result("lost"), Some(CombatResult::Defeat));
    assert_eq!(CombatResult::from_run_result("failed"), Some(CombatResult::Defeat));
    assert_eq!(CombatResult::from_run_result("fled"), Some(CombatResult::Fled));
    assert_eq!(CombatResult::from_run_result("run"), Some(CombatResult::Fled));
    assert_eq!(CombatResult::from_run_result("escaped"), Some(CombatResult::Fled));
    assert_eq!(CombatResult::from_run_result("draw"), Some(CombatResult::Draw));
    assert_eq!(CombatResult::from_run_result("tie"), Some(CombatResult::Draw));
    assert_eq!(CombatResult::from_run_result("unknown"), None);
}

/// Verifies TownActivityType from_building_type handles all known types.
#[test]
fn town_activity_type_from_building_type_handles_known_types() {
    assert_eq!(TownActivityType::from_building_type("stagecoach"), TownActivityType::Stagecoach);
    assert_eq!(TownActivityType::from_building_type("guild"), TownActivityType::Guild);
    assert_eq!(TownActivityType::from_building_type("blacksmith"), TownActivityType::Blacksmith);
    assert_eq!(TownActivityType::from_building_type("sanitarium"), TownActivityType::Sanitarium);
    assert_eq!(TownActivityType::from_building_type("tavern"), TownActivityType::Tavern);
    assert_eq!(TownActivityType::from_building_type("abbey"), TownActivityType::Abbey);
    assert_eq!(TownActivityType::from_building_type("campfire"), TownActivityType::Camping);
    assert_eq!(TownActivityType::from_building_type("unknown_building"), TownActivityType::Other("unknown_building".to_string()));
}

// ── US-002-d: Determinism tests ──────────────────────────────────────────────

/// Verifies dungeon_from_run_result is deterministic for same input.
#[test]
fn adapter_dungeon_deterministic_for_same_input() {
    let heroes = vec![
        make_hero_state("h1", "crusader", 80.0, 100.0, 20.0, 200.0),
        make_hero_state("h2", "hunter", 90.0, 100.0, 30.0, 200.0),
    ];
    let room_encounters = vec![
        RoomEncounterRecord {
            room_id: RoomId(1),
            room_kind: framework_progression::rooms::RoomKind::Combat,
            pack_id: "pack1".to_string(),
            family_ids: vec![],
        },
        RoomEncounterRecord {
            room_id: RoomId(2),
            room_kind: framework_progression::rooms::RoomKind::Boss,
            pack_id: "boss_pack".to_string(),
            family_ids: vec![],
        },
    ];

    let make_run_result = || DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor: make_test_floor(),
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 8,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters: room_encounters.clone(),
        interaction_records: vec![],
        camping_trace: vec![],
        heroes: heroes.clone(),
    };

    let result1 = dungeon_from_run_result(&make_run_result());
    let result2 = dungeon_from_run_result(&make_run_result());

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert_eq!(result1.unwrap(), result2.unwrap());
}

/// Verifies combat_from_framework is deterministic for same input.
#[test]
fn adapter_combat_deterministic_for_same_input() {
    let actors = vec![
        make_actor_summary(1, CombatSide::Ally, 80.0, 100.0, vec![("bleeding", Some(2))]),
        make_actor_summary(10, CombatSide::Enemy, 150.0, 200.0, vec![]),
    ];

    let framework_vm1 = make_framework_combat_vm(1, 3, actors.clone(), vec![1, 10]);
    let framework_vm2 = make_framework_combat_vm(1, 3, actors, vec![1, 10]);

    let result1 = combat_from_framework(&framework_vm1);
    let result2 = combat_from_framework(&framework_vm2);

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert_eq!(result1.unwrap(), result2.unwrap());
}

/// Verifies result_from_run is deterministic for same input.
#[test]
fn adapter_result_deterministic_for_same_input() {
    use std::collections::BTreeMap;

    let heirlooms1: BTreeMap<HeirloomCurrency, u32> = BTreeMap::new();
    let heirlooms2: BTreeMap<HeirloomCurrency, u32> = BTreeMap::new();
    let casualties1 = vec![("hero1".to_string(), "crusader".to_string())];
    let casualties2 = vec![("hero1".to_string(), "crusader".to_string())];

    let result1 = result_from_run(
        DungeonType::QingLong,
        MapSize::Short,
        8,
        4,
        true,
        500,
        100,
        &heirlooms1,
        casualties1,
    );

    let result2 = result_from_run(
        DungeonType::QingLong,
        MapSize::Short,
        8,
        4,
        true,
        500,
        100,
        &heirlooms2,
        casualties2,
    );

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert_eq!(result1.unwrap(), result2.unwrap());
}

/// Verifies return_flow_from_state is deterministic for same input.
#[test]
fn adapter_return_flow_deterministic_for_same_input() {
    let heroes1 = vec![
        ("hero1".to_string(), "crusader".to_string(), 80.0, 100.0, 20.0, 200.0),
    ];
    let heroes2 = vec![
        ("hero1".to_string(), "crusader".to_string(), 80.0, 100.0, 20.0, 200.0),
    ];
    let died1 = vec![];
    let died2 = vec![];

    let result1 = return_flow_from_state(
        DungeonType::QingLong,
        MapSize::Short,
        8,
        4,
        true,
        500,
        &heroes1,
        &died1,
    );

    let result2 = return_flow_from_state(
        DungeonType::QingLong,
        MapSize::Short,
        8,
        4,
        true,
        500,
        &heroes2,
        &died2,
    );

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert_eq!(result1.unwrap(), result2.unwrap());
}

// ── US-005-a: Exploration scene slice tests ─────────────────────────────────────

/// Verifies exploration payloads render consistently across all dungeon types.
#[test]
fn adapter_exploration_payloads_render_consistently_for_all_dungeon_types() {
    // Test all four core DDGC dungeons to ensure exploration payloads render consistently
    for dungeon in [DungeonType::QingLong, DungeonType::BaiHu, DungeonType::ZhuQue, DungeonType::XuanWu] {
        for map_size in [MapSize::Short, MapSize::Medium] {
            let heroes = vec![
                make_hero_state("h1", "crusader", 80.0, 100.0, 20.0, 200.0),
                make_hero_state("h2", "hunter", 90.0, 100.0, 30.0, 200.0),
            ];
            let room_encounters = vec![
                RoomEncounterRecord {
                    room_id: RoomId(1),
                    room_kind: framework_progression::rooms::RoomKind::Combat,
                    pack_id: "pack1".to_string(),
                    family_ids: vec![],
                },
                RoomEncounterRecord {
                    room_id: RoomId(2),
                    room_kind: framework_progression::rooms::RoomKind::Boss,
                    pack_id: "boss_pack".to_string(),
                    family_ids: vec![],
                },
            ];

            let base_room_number = match map_size {
                MapSize::Short => 9,
                MapSize::Medium => 14,
            };

            let make_run_result = || DdgcRunResult {
                run: make_test_run(),
                state: game_ddgc_headless::run::flow::DdgcRunState::new(),
                floor: make_test_floor(),
                battle_pack_ids: vec![],
                metadata: RunMetadata {
                    dungeon_type: dungeon,
                    map_size,
                    base_room_number,
                    base_corridor_number: 4,
                    gridsize: GridSize::new(5, 5),
                    connectivity: 0.5,
                },
                room_encounters: room_encounters.clone(),
                interaction_records: vec![],
                camping_trace: vec![],
                heroes: heroes.clone(),
            };

            let result1 = dungeon_from_run_result(&make_run_result());
            let result2 = dungeon_from_run_result(&make_run_result());

            assert!(result1.is_ok(), "dungeon_from_run_result failed for {:?} {:?}", dungeon, map_size);
            assert!(result2.is_ok(), "dungeon_from_run_result failed for {:?} {:?}", dungeon, map_size);
            assert_eq!(result1.unwrap(), result2.unwrap(),
                "Exploration payload should render consistently for {:?} {:?}", dungeon, map_size);
        }
    }
}

/// Verifies exploration view model captures dungeon metadata correctly for all types.
#[test]
fn adapter_exploration_captures_dungeon_metadata() {
    let room_encounters = vec![
        RoomEncounterRecord {
            room_id: RoomId(1),
            room_kind: framework_progression::rooms::RoomKind::Combat,
            pack_id: "pack1".to_string(),
            family_ids: vec![],
        },
    ];

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor: make_test_floor(),
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::ZhuQue,
            map_size: MapSize::Medium,
            base_room_number: 14,
            base_corridor_number: 5,
            gridsize: GridSize::new(6, 6),
            connectivity: 0.95,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes: vec![],
    };

    let result = dungeon_from_run_result(&run_result);
    assert!(result.is_ok());

    let vm = result.unwrap();
    assert_eq!(vm.dungeon_type, "ZhuQue");
    assert_eq!(vm.map_size, "Medium");
    assert_eq!(vm.total_rooms, 14);
    assert_eq!(vm.floor, 1);
    assert!(!vm.is_complete);
    assert!(!vm.party_fled);
}

/// Verifies exploration view model hero states at death's door threshold.
#[test]
fn adapter_exploration_hero_at_deaths_door_threshold() {
    // Hero at exactly 50% health should NOT be at death's door
    let heroes = vec![
        make_hero_state("h1", "crusader", 50.0, 100.0, 20.0, 200.0), // 50% - not at DoD
        make_hero_state("h2", "hunter", 49.0, 100.0, 30.0, 200.0),    // 49% - at DoD
        make_hero_state("h3", "alchemist", 100.0, 100.0, 0.0, 200.0), // 100% - not at DoD
    ];
    let room_encounters = vec![];

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor: make_test_floor(),
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 9,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    let result = dungeon_from_run_result(&run_result);
    assert!(result.is_ok());

    let vm = result.unwrap();

    // Find heroes by id
    let h1 = vm.heroes.iter().find(|h| h.id == "h1").unwrap();
    let h2 = vm.heroes.iter().find(|h| h.id == "h2").unwrap();
    let h3 = vm.heroes.iter().find(|h| h.id == "h3").unwrap();

    assert!(!h1.is_at_deaths_door, "50% HP should NOT be at death's door (threshold is < 50%)");
    assert!(h2.is_at_deaths_door, "49% HP should be at death's door");
    assert!(!h3.is_at_deaths_door, "100% HP should NOT be at death's door");
}

/// Verifies exploration view model tracks battle progress correctly.
#[test]
fn adapter_exploration_tracks_battle_progress() {
    let mut state = game_ddgc_headless::run::flow::DdgcRunState::new();
    state.gold = 500;
    state.battles_won = 3;
    state.battles_lost = 1;
    state.rooms_cleared = 6;

    let heroes = vec![];
    let room_encounters = vec![
        RoomEncounterRecord {
            room_id: RoomId(1),
            room_kind: framework_progression::rooms::RoomKind::Combat,
            pack_id: "pack1".to_string(),
            family_ids: vec![],
        },
    ];

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state,
        floor: make_test_floor(),
        battle_pack_ids: vec!["pack1".to_string()],
        metadata: RunMetadata {
            dungeon_type: DungeonType::BaiHu,
            map_size: MapSize::Short,
            base_room_number: 9,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    let result = dungeon_from_run_result(&run_result);
    assert!(result.is_ok());

    let vm = result.unwrap();
    assert_eq!(vm.gold_carried, 500);
    assert_eq!(vm.battles_won, 3);
    assert_eq!(vm.battles_lost, 1);
    assert_eq!(vm.rooms_cleared, 6);
}

/// Verifies exploration view model maps room encounter kinds correctly.
#[test]
fn adapter_exploration_maps_room_kinds_correctly() {
    let heroes = vec![];
    let room_encounters = vec![
        RoomEncounterRecord {
            room_id: RoomId(1),
            room_kind: framework_progression::rooms::RoomKind::Combat,
            pack_id: "combat_pack".to_string(),
            family_ids: vec![],
        },
        RoomEncounterRecord {
            room_id: RoomId(2),
            room_kind: framework_progression::rooms::RoomKind::Boss,
            pack_id: "boss_pack".to_string(),
            family_ids: vec![],
        },
        RoomEncounterRecord {
            room_id: RoomId(3),
            room_kind: framework_progression::rooms::RoomKind::Event {
                curio_id: Some("ancient_vase".to_string()),
            },
            pack_id: String::new(),
            family_ids: vec![],
        },
        RoomEncounterRecord {
            room_id: RoomId(4),
            room_kind: framework_progression::rooms::RoomKind::Corridor {
                trap_id: Some("trap1".to_string()),
                curio_id: Some("crate".to_string()),
            },
            pack_id: String::new(),
            family_ids: vec![],
        },
    ];

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor: make_test_floor(),
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 9,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    let result = dungeon_from_run_result(&run_result);
    assert!(result.is_ok());

    let vm = result.unwrap();
    assert_eq!(vm.rooms.len(), 4);
    assert_eq!(vm.rooms[0].kind, DungeonRoomKind::Combat);
    assert_eq!(vm.rooms[1].kind, DungeonRoomKind::Boss);
    assert_eq!(vm.rooms[2].kind, DungeonRoomKind::Event);
    assert_eq!(vm.rooms[3].kind, DungeonRoomKind::Corridor);
}

/// Verifies exploration-to-combat transition is supported through view models.
/// The exploration state (DungeonViewModel) can coexist with combat state (CombatViewModel)
/// as they represent different phases of the same dungeon run.
#[test]
fn adapter_exploration_to_combat_transition_supported() {
    // Create exploration state
    let heroes = vec![make_hero_state("h1", "crusader", 80.0, 100.0, 20.0, 200.0)];
    let room_encounters = vec![
        RoomEncounterRecord {
            room_id: RoomId(1),
            room_kind: framework_progression::rooms::RoomKind::Combat,
            pack_id: "pack1".to_string(),
            family_ids: vec![],
        },
    ];

    let exploration_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor: make_test_floor(),
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 9,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    let exploration_vm = dungeon_from_run_result(&exploration_result);
    assert!(exploration_vm.is_ok());

    // Create corresponding combat state
    let actors = vec![
        make_actor_summary(1, CombatSide::Ally, 80.0, 100.0, vec![]),
        make_actor_summary(10, CombatSide::Enemy, 150.0, 200.0, vec![]),
    ];
    let framework_combat_vm = make_framework_combat_vm(1, 1, actors, vec![1, 10]);
    let combat_vm = combat_from_framework(&framework_combat_vm);
    assert!(combat_vm.is_ok());

    // Verify both view models can coexist - exploration has heroes, combat has combatants
    let exp_vm = exploration_vm.unwrap();
    let cbt_vm = combat_vm.unwrap();

    assert!(!exp_vm.heroes.is_empty(), "Exploration should have heroes");
    assert!(!cbt_vm.heroes.is_empty(), "Combat should have heroes");
    assert!(!cbt_vm.monsters.is_empty(), "Combat should have monsters");

    // Verify the hero in exploration matches the combat participant
    assert_eq!(exp_vm.heroes[0].id, "h1");
}

/// Verifies exploration payloads render deterministically across multiple dungeon configurations.
#[test]
fn adapter_exploration_multi_seed_determinism() {
    // Test that different seeds produce different but deterministic results
    let base_room_number = 9;
    let heroes = vec![make_hero_state("h1", "crusader", 80.0, 100.0, 20.0, 200.0)];

    // Run 1 with seed 1
    let room_encounters_1 = vec![
        RoomEncounterRecord {
            room_id: RoomId(1),
            room_kind: framework_progression::rooms::RoomKind::Combat,
            pack_id: "pack1".to_string(),
            family_ids: vec![],
        },
    ];
    let run_result_1 = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor: make_test_floor(),
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters: room_encounters_1,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes: heroes.clone(),
    };

    // Run 2 with different dungeon type
    let room_encounters_2 = vec![
        RoomEncounterRecord {
            room_id: RoomId(1),
            room_kind: framework_progression::rooms::RoomKind::Boss,
            pack_id: "boss_pack".to_string(),
            family_ids: vec![],
        },
    ];
    let run_result_2 = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor: make_test_floor(),
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::XuanWu,
            map_size: MapSize::Short,
            base_room_number,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters: room_encounters_2,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes: heroes.clone(),
    };

    let result_1a = dungeon_from_run_result(&run_result_1);
    let result_1b = dungeon_from_run_result(&run_result_1);
    let result_2 = dungeon_from_run_result(&run_result_2);

    assert!(result_1a.is_ok());
    assert!(result_1b.is_ok());
    assert!(result_2.is_ok());

    // Same input produces same output
    assert_eq!(result_1a.as_ref().unwrap(), result_1b.as_ref().unwrap());

    // Different input produces different output
    let vm_1 = result_1a.as_ref().unwrap();
    let vm_2 = result_2.as_ref().unwrap();
    assert_ne!(vm_1.dungeon_type, vm_2.dungeon_type);
}

// ── US-005-c: Exploration HUD adapter tests ─────────────────────────────────────

/// Verifies exploration_hud_from_dungeon produces correct HUD view model.
#[test]
fn adapter_exploration_hud_maps_basic_context() {
    let heroes = vec![make_hero_state("h1", "crusader", 80.0, 100.0, 20.0, 200.0)];
    let room_encounters = vec![RoomEncounterRecord {
        room_id: RoomId(1),
        room_kind: framework_progression::rooms::RoomKind::Combat,
        pack_id: "pack1".to_string(),
        family_ids: vec![],
    }];

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor: make_test_floor(),
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 8,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    let dungeon_vm = dungeon_from_run_result(&run_result).unwrap();
    let hud_vm = exploration_hud_from_dungeon(&dungeon_vm);

    assert!(hud_vm.is_ok());
    let hud = hud_vm.unwrap();
    assert_eq!(hud.dungeon_type, "QingLong");
    assert_eq!(hud.map_size, "Short");
    assert_eq!(hud.floor, 1);
    assert_eq!(hud.rooms_cleared, 0);
    assert_eq!(hud.total_rooms, 8);
}

/// Verifies exploration_hud_from_dungeon maps hero vitals correctly.
#[test]
fn adapter_exploration_hud_maps_hero_vitals() {
    let heroes = vec![
        make_hero_state("h1", "crusader", 80.0, 100.0, 20.0, 200.0),
        make_hero_state("h2", "hunter", 49.0, 100.0, 150.0, 200.0), // death's door: 49 < 50%
    ];
    let room_encounters = vec![];

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor: make_test_floor(),
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 8,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    let dungeon_vm = dungeon_from_run_result(&run_result).unwrap();
    let hud_vm = exploration_hud_from_dungeon(&dungeon_vm);

    assert!(hud_vm.is_ok());
    let hud = hud_vm.unwrap();
    assert_eq!(hud.hero_vitals.len(), 2);

    let h1 = hud.hero_vitals.iter().find(|h| h.id == "h1").unwrap();
    assert!((h1.health_fraction - 0.8).abs() < 0.001);
    assert!((h1.stress_fraction - 0.1).abs() < 0.001);
    assert!(!h1.is_at_deaths_door);
    assert!(!h1.is_dead);

    let h2 = hud.hero_vitals.iter().find(|h| h.id == "h2").unwrap();
    assert!((h2.health_fraction - 0.49).abs() < 0.001);
    assert!(h2.is_at_deaths_door, "49% HP should be at death's door");
    assert!(!h2.is_dead);
}

/// Verifies exploration_hud_from_dungeon detects death's door heroes.
#[test]
fn adapter_exploration_hud_detects_heros_at_deaths_door() {
    let heroes = vec![
        make_hero_state("h1", "crusader", 50.0, 100.0, 20.0, 200.0), // 50% - not at DoD
        make_hero_state("h2", "hunter", 49.0, 100.0, 30.0, 200.0),    // 49% - at DoD
    ];
    let room_encounters = vec![];

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor: make_test_floor(),
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 9,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    let dungeon_vm = dungeon_from_run_result(&run_result).unwrap();
    let hud_vm = exploration_hud_from_dungeon(&dungeon_vm);

    assert!(hud_vm.is_ok());
    let hud = hud_vm.unwrap();
    assert!(hud.any_hero_at_deaths_door());
}

/// Verifies exploration_hud_from_dungeon detects dead heroes.
#[test]
fn adapter_exploration_hud_detects_dead_heros() {
    let heroes = vec![
        make_hero_state("h1", "crusader", 0.0, 100.0, 0.0, 200.0), // dead
        make_hero_state("h2", "hunter", 80.0, 100.0, 30.0, 200.0),  // alive
    ];
    let room_encounters = vec![];

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor: make_test_floor(),
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 9,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    let dungeon_vm = dungeon_from_run_result(&run_result).unwrap();
    let hud_vm = exploration_hud_from_dungeon(&dungeon_vm);

    assert!(hud_vm.is_ok());
    let hud = hud_vm.unwrap();
    assert!(hud.any_hero_dead());
}

// ── US-005-c: Room movement adapter tests ──────────────────────────────────────

/// Verifies room_movement_from_run produces correct movement for first room.
#[test]
fn adapter_room_movement_first_room() {
    use framework_progression::rooms::{Room, RoomKind, RoomState};

    let heroes = vec![];
    let room_encounters = vec![
        RoomEncounterRecord {
            room_id: RoomId(1),
            room_kind: RoomKind::Combat,
            pack_id: "pack1".to_string(),
            family_ids: vec![],
        },
        RoomEncounterRecord {
            room_id: RoomId(2),
            room_kind: RoomKind::Event { curio_id: Some("ancient_vase".to_string()) },
            pack_id: String::new(),
            family_ids: vec![],
        },
    ];

    let mut floor = make_test_floor();
    floor.rooms = vec![RoomId(1), RoomId(2)];
    floor.rooms_map.insert(RoomId(1), Room {
        id: RoomId(1),
        kind: RoomKind::Combat,
        state: RoomState::Cleared,
        connections: vec![],
    });
    floor.rooms_map.insert(RoomId(2), Room {
        id: RoomId(2),
        kind: RoomKind::Event { curio_id: Some("ancient_vase".to_string()) },
        state: RoomState::Unvisited,
        connections: vec![],
    });

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor,
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 8,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    // First room (index 0) has no previous room
    let result = room_movement_from_run(&run_result, 0);
    assert!(result.is_ok());
    let movement = result.unwrap();
    assert!(movement.from_room_id.is_none());
    assert!(movement.from_room_kind.is_none());
    assert_eq!(movement.to_room_id, "RoomId(1)");
    assert_eq!(movement.to_room_kind, DungeonRoomKind::Combat);
    assert!(movement.is_cleared);
}

/// Verifies room_movement_from_run produces correct movement for subsequent rooms.
#[test]
fn adapter_room_movement_subsequent_room() {
    use framework_progression::rooms::{Room, RoomKind, RoomState};

    let heroes = vec![];
    let room_encounters = vec![
        RoomEncounterRecord {
            room_id: RoomId(1),
            room_kind: RoomKind::Combat,
            pack_id: "pack1".to_string(),
            family_ids: vec![],
        },
        RoomEncounterRecord {
            room_id: RoomId(2),
            room_kind: RoomKind::Event { curio_id: Some("ancient_vase".to_string()) },
            pack_id: String::new(),
            family_ids: vec![],
        },
    ];

    let mut floor = make_test_floor();
    floor.rooms = vec![RoomId(1), RoomId(2)];
    floor.rooms_map.insert(RoomId(1), Room {
        id: RoomId(1),
        kind: RoomKind::Combat,
        state: RoomState::Cleared,
        connections: vec![],
    });
    floor.rooms_map.insert(RoomId(2), Room {
        id: RoomId(2),
        kind: RoomKind::Event { curio_id: Some("ancient_vase".to_string()) },
        state: RoomState::Unvisited,
        connections: vec![],
    });

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor,
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 8,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    // Second room (index 1) has previous room
    let result = room_movement_from_run(&run_result, 1);
    assert!(result.is_ok());
    let movement = result.unwrap();
    assert!(movement.from_room_id.is_some());
    assert_eq!(movement.from_room_kind, Some(DungeonRoomKind::Combat));
    assert_eq!(movement.to_room_id, "RoomId(2)");
    assert_eq!(movement.to_room_kind, DungeonRoomKind::Event);
    assert!(!movement.is_cleared);
    assert_eq!(movement.interaction_id, Some("ancient_vase".to_string()));
}

/// Verifies room_movement_from_run returns error for invalid room index.
#[test]
fn adapter_room_movement_invalid_index() {
    let heroes = vec![];
    let room_encounters = vec![];

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor: make_test_floor(),
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 8,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    let result = room_movement_from_run(&run_result, 100);
    assert!(result.is_err());
}

// ── US-005-c: Encounter entry adapter tests ────────────────────────────────────

/// Verifies encounter_entry_from_run produces correct entry for combat room.
#[test]
fn adapter_encounter_entry_combat_room() {
    use framework_progression::rooms::{Room, RoomKind, RoomState};

    let heroes = vec![
        make_hero_state("h1", "crusader", 80.0, 100.0, 20.0, 200.0),
        make_hero_state("h2", "hunter", 90.0, 100.0, 30.0, 200.0),
    ];
    let room_encounters = vec![
        RoomEncounterRecord {
            room_id: RoomId(1),
            room_kind: RoomKind::Combat,
            pack_id: "combat_pack_1".to_string(),
            family_ids: vec![FamilyId::new("family_dragon")],
        },
    ];

    let mut floor = make_test_floor();
    floor.rooms = vec![RoomId(1)];
    floor.rooms_map.insert(RoomId(1), Room {
        id: RoomId(1),
        kind: RoomKind::Combat,
        state: RoomState::Unvisited,
        connections: vec![],
    });

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor,
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 8,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    let result = encounter_entry_from_run(&run_result, 0);
    assert!(result.is_ok());
    let entry = result.unwrap();
    assert_eq!(entry.room_id, "RoomId(1)");
    assert!(!entry.is_boss);
    assert_eq!(entry.pack_id, "combat_pack_1");
    assert!(entry.family_ids.contains(&"family_dragon".to_string()));
    assert_eq!(entry.heroes.len(), 2);
}

/// Verifies encounter_entry_from_run produces correct entry for boss room.
#[test]
fn adapter_encounter_entry_boss_room() {
    use framework_progression::rooms::{Room, RoomKind, RoomState};

    let heroes = vec![make_hero_state("h1", "crusader", 80.0, 100.0, 20.0, 200.0)];
    let room_encounters = vec![
        RoomEncounterRecord {
            room_id: RoomId(5),
            room_kind: RoomKind::Boss,
            pack_id: "boss_pack_1".to_string(),
            family_ids: vec![FamilyId::new("family_dragon")],
        },
    ];

    let mut floor = make_test_floor();
    floor.rooms = vec![RoomId(5)];
    floor.rooms_map.insert(RoomId(5), Room {
        id: RoomId(5),
        kind: RoomKind::Boss,
        state: RoomState::Unvisited,
        connections: vec![],
    });

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor,
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 8,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    let result = encounter_entry_from_run(&run_result, 0);
    assert!(result.is_ok());
    let entry = result.unwrap();
    assert_eq!(entry.room_id, "RoomId(5)");
    assert!(entry.is_boss);
    assert_eq!(entry.pack_id, "boss_pack_1");
}

/// Verifies encounter_entry_from_run returns error for non-combat room.
#[test]
fn adapter_encounter_entry_non_combat_room() {
    use framework_progression::rooms::{Room, RoomKind, RoomState};

    let heroes = vec![];
    let room_encounters = vec![
        RoomEncounterRecord {
            room_id: RoomId(2),
            room_kind: RoomKind::Event { curio_id: Some("ancient_vase".to_string()) },
            pack_id: String::new(),
            family_ids: vec![],
        },
    ];

    let mut floor = make_test_floor();
    floor.rooms = vec![RoomId(2)];
    floor.rooms_map.insert(RoomId(2), Room {
        id: RoomId(2),
        kind: RoomKind::Event { curio_id: Some("ancient_vase".to_string()) },
        state: RoomState::Unvisited,
        connections: vec![],
    });

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor,
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 8,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    let result = encounter_entry_from_run(&run_result, 0);
    assert!(result.is_err());
}

/// Verifies encounter_entry_from_run returns error for invalid room index.
#[test]
fn adapter_encounter_entry_invalid_index() {
    let heroes = vec![];
    let room_encounters = vec![];

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor: make_test_floor(),
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 8,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    let result = encounter_entry_from_run(&run_result, 100);
    assert!(result.is_err());
}

/// Verifies exploration to combat transition through adapters.
#[test]
fn adapter_exploration_to_combat_via_adapters() {
    use framework_progression::rooms::{Room, RoomKind, RoomState};

    let heroes = vec![make_hero_state("h1", "crusader", 80.0, 100.0, 20.0, 200.0)];
    let room_encounters = vec![
        RoomEncounterRecord {
            room_id: RoomId(1),
            room_kind: RoomKind::Combat,
            pack_id: "combat_pack_1".to_string(),
            family_ids: vec![FamilyId::new("family_dragon")],
        },
    ];

    let mut floor = make_test_floor();
    floor.rooms = vec![RoomId(1)];
    floor.rooms_map.insert(RoomId(1), Room {
        id: RoomId(1),
        kind: RoomKind::Combat,
        state: RoomState::Unvisited,
        connections: vec![],
    });

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor,
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 8,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.5,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    // Step 1: Create dungeon view model
    let dungeon_vm = dungeon_from_run_result(&run_result).unwrap();

    // Step 2: Create exploration HUD from dungeon
    let hud_vm = exploration_hud_from_dungeon(&dungeon_vm).unwrap();
    assert!(!hud_vm.hero_vitals.is_empty());

    // Step 3: Create room movement for the combat room
    let movement_vm = room_movement_from_run(&run_result, 0).unwrap();
    assert_eq!(movement_vm.to_room_kind, DungeonRoomKind::Combat);

    // Step 4: Create encounter entry for the combat room
    let encounter_vm = encounter_entry_from_run(&run_result, 0).unwrap();
    assert!(!encounter_vm.is_boss);
    assert_eq!(encounter_vm.heroes.len(), 1);
    assert_eq!(encounter_vm.heroes[0].id, "h1");
}

// ── US-006-a: Combat interaction and resolution slice tests ────────────────────

/// Verifies combat_hud_from_combat produces correct HUD view model.
#[test]
fn adapter_combat_hud_maps_basic_context() {
    let actors = vec![
        make_actor_summary(1, CombatSide::Ally, 80.0, 100.0, vec![]),
        make_actor_summary(10, CombatSide::Enemy, 150.0, 200.0, vec![]),
    ];
    let framework_vm = make_framework_combat_vm(1, 1, actors, vec![1, 10]);

    let combat_vm = combat_from_framework(&framework_vm).unwrap();
    let hud_vm = combat_hud_from_combat(&combat_vm);

    assert!(hud_vm.is_ok());
    let hud = hud_vm.unwrap();
    assert_eq!(hud.encounter_id, "encounter_EncounterId(1)");
    assert_eq!(hud.round, 1);
    assert_eq!(hud.phase, CombatPhase::Unknown);
    assert!(!hud.is_resolving);
}

/// Verifies combat_hud_from_combat maps hero and monster vitals correctly.
#[test]
fn adapter_combat_hud_maps_vitals() {
    let actors = vec![
        make_actor_summary(1, CombatSide::Ally, 80.0, 100.0, vec![]),
        make_actor_summary(2, CombatSide::Ally, 50.0, 100.0, vec![]), // at death's door: 50%
        make_actor_summary(10, CombatSide::Enemy, 150.0, 200.0, vec![]),
        make_actor_summary(11, CombatSide::Enemy, 0.0, 200.0, vec![]), // dead
    ];
    let framework_vm = make_framework_combat_vm(1, 1, actors, vec![1, 10]);

    let combat_vm = combat_from_framework(&framework_vm).unwrap();
    let hud_vm = combat_hud_from_combat(&combat_vm).unwrap();

    assert_eq!(hud_vm.hero_vitals.len(), 2);
    assert_eq!(hud_vm.monster_vitals.len(), 2);
    assert_eq!(hud_vm.heroes_alive, 2);
    assert_eq!(hud_vm.monsters_alive, 1);

    let h1 = hud_vm.hero_vitals.iter().find(|h| h.id == "ActorId(1)").unwrap();
    assert!((h1.health_fraction - 0.8).abs() < 0.001);
    assert!(!h1.is_at_deaths_door);
    assert!(!h1.is_dead);

    let h2 = hud_vm.hero_vitals.iter().find(|h| h.id == "ActorId(2)").unwrap();
    assert!((h2.health_fraction - 0.5).abs() < 0.001);
    assert!(!h2.is_at_deaths_door, "50% HP should NOT be at death's door (threshold is < 50%)");
    assert!(!h2.is_dead);

    let m11 = hud_vm.monster_vitals.iter().find(|m| m.id == "ActorId(11)").unwrap();
    assert!(m11.is_dead);
    assert!(m11.health_fraction < 0.001);
}

/// Verifies combat_hud_from_combat detects death's door correctly.
#[test]
fn adapter_combat_hud_detects_deaths_door() {
    // Hero at 49% health should be at death's door
    let actors = vec![
        make_actor_summary(1, CombatSide::Ally, 49.0, 100.0, vec![]),
        make_actor_summary(10, CombatSide::Enemy, 100.0, 200.0, vec![]),
    ];
    let framework_vm = make_framework_combat_vm(1, 1, actors, vec![1, 10]);

    let combat_vm = combat_from_framework(&framework_vm).unwrap();
    let hud_vm = combat_hud_from_combat(&combat_vm).unwrap();

    let h1 = hud_vm.hero_vitals.iter().find(|h| h.id == "ActorId(1)").unwrap();
    assert!(h1.is_at_deaths_door, "49% HP should be at death's door");
}

/// Verifies combat_hud_from_combat marks resolution phase correctly.
#[test]
fn adapter_combat_hud_marks_resolution_phase() {
    let actors = vec![
        make_actor_summary(1, CombatSide::Ally, 80.0, 100.0, vec![]),
        make_actor_summary(10, CombatSide::Enemy, 150.0, 200.0, vec![]),
    ];
    let framework_vm = make_framework_combat_vm(1, 1, actors, vec![1, 10]);

    let combat_vm = combat_from_framework(&framework_vm).unwrap();
    let hud_vm = combat_hud_from_combat(&combat_vm).unwrap();

    assert!(!hud_vm.is_resolving);
    assert!(hud_vm.is_combat_active());
}

/// Verifies combat_hud_from_combat handles post-battle state.
#[test]
fn adapter_combat_hud_post_battle() {
    let actors = vec![
        make_actor_summary(1, CombatSide::Ally, 80.0, 100.0, vec![]),
        make_actor_summary(10, CombatSide::Enemy, 0.0, 200.0, vec![]),
    ];
    let framework_vm = make_framework_combat_vm(1, 1, actors, vec![1, 10]);

    let mut combat_vm = combat_from_framework(&framework_vm).unwrap();
    combat_vm.phase = CombatPhase::PostBattle;
    combat_vm.result = Some(CombatResult::Victory);

    let hud_vm = combat_hud_from_combat(&combat_vm).unwrap();

    assert!(!hud_vm.is_combat_active());
    assert_eq!(hud_vm.result, Some(CombatResult::Victory));
}

/// Verifies combat_hud_from_combat is deterministic for same input.
#[test]
fn adapter_combat_hud_deterministic() {
    let actors = vec![
        make_actor_summary(1, CombatSide::Ally, 80.0, 100.0, vec![("bleeding", Some(2))]),
        make_actor_summary(10, CombatSide::Enemy, 150.0, 200.0, vec![]),
    ];
    let framework_vm1 = make_framework_combat_vm(1, 3, actors.clone(), vec![1, 10]);
    let framework_vm2 = make_framework_combat_vm(1, 3, actors, vec![1, 10]);

    let combat_vm1 = combat_from_framework(&framework_vm1).unwrap();
    let combat_vm2 = combat_from_framework(&framework_vm2).unwrap();

    let hud_vm1 = combat_hud_from_combat(&combat_vm1).unwrap();
    let hud_vm2 = combat_hud_from_combat(&combat_vm2).unwrap();

    assert_eq!(hud_vm1, hud_vm2);
}

/// Verifies combat_hud_from_combat tracks alive counts correctly.
#[test]
fn adapter_combat_hud_tracks_alive_counts() {
    // 2 heroes alive, 1 monster alive
    let actors = vec![
        make_actor_summary(1, CombatSide::Ally, 80.0, 100.0, vec![]),
        make_actor_summary(2, CombatSide::Ally, 0.0, 100.0, vec![]), // dead
        make_actor_summary(10, CombatSide::Enemy, 150.0, 200.0, vec![]),
        make_actor_summary(11, CombatSide::Enemy, 0.0, 200.0, vec![]), // dead
    ];
    let framework_vm = make_framework_combat_vm(1, 1, actors, vec![1, 10]);

    let combat_vm = combat_from_framework(&framework_vm).unwrap();
    let hud_vm = combat_hud_from_combat(&combat_vm).unwrap();

    assert_eq!(hud_vm.heroes_alive, 1);
    assert_eq!(hud_vm.monsters_alive, 1);
    assert!(!hud_vm.all_heroes_dead());
    assert!(!hud_vm.all_monsters_dead());
}

/// Verifies combat_hud_from_combat detects all-dead states.
#[test]
fn adapter_combat_hud_detects_all_dead() {
    // All heroes dead
    let actors = vec![
        make_actor_summary(1, CombatSide::Ally, 0.0, 100.0, vec![]),
        make_actor_summary(10, CombatSide::Enemy, 150.0, 200.0, vec![]),
    ];
    let framework_vm = make_framework_combat_vm(1, 1, actors, vec![1, 10]);

    let combat_vm = combat_from_framework(&framework_vm).unwrap();
    let hud_vm = combat_hud_from_combat(&combat_vm).unwrap();

    assert!(hud_vm.all_heroes_dead());
    assert!(!hud_vm.all_monsters_dead());
}

/// Verifies CombatHudViewModel empty constructor works.
#[test]
fn combat_hud_view_model_empty() {
    let hud = CombatHudViewModel::empty();
    assert!(hud.encounter_id.is_empty());
    assert_eq!(hud.round, 0);
    assert_eq!(hud.phase, CombatPhase::Unknown);
    assert!(hud.result.is_none());
    assert!(hud.hero_vitals.is_empty());
    assert!(hud.monster_vitals.is_empty());
    assert_eq!(hud.heroes_alive, 0);
    assert_eq!(hud.monsters_alive, 0);
    assert!(!hud.is_resolving);
}

/// Verifies CombatActionInput actor_id extraction.
#[test]
fn combat_action_input_actor_id() {
    let attack = CombatActionInput::Attack {
        attacker_id: "hero1".to_string(),
        target_position: CombatPosition { lane: 0, slot: 1 },
    };
    assert_eq!(attack.actor_id(), Some("hero1"));

    let defend = CombatActionInput::Defend {
        defender_id: "hero2".to_string(),
    };
    assert_eq!(defend.actor_id(), Some("hero2"));

    let retreat = CombatActionInput::Retreat {
        party_member_id: "hero1".to_string(),
    };
    assert_eq!(retreat.actor_id(), Some("hero1"));
}

/// Verifies CombatFeedback description generation.
#[test]
fn combat_feedback_description() {
    let damage = CombatFeedback::DamageDealt {
        target_id: "monster1".to_string(),
        damage: 25.0,
        is_fatal: false,
        damage_type: "physical".to_string(),
    };
    assert!(damage.description().contains("25"));
    assert!(damage.description().contains("physical"));

    let fatal = CombatFeedback::DamageDealt {
        target_id: "hero1".to_string(),
        damage: 50.0,
        is_fatal: true,
        damage_type: "magic".to_string(),
    };
    assert!(fatal.description().contains("fatally wounded"));

    let status = CombatFeedback::StatusApplied {
        target_id: "hero1".to_string(),
        status_id: "bleeding".to_string(),
        duration: Some(3),
    };
    assert!(status.description().contains("bleeding"));
    assert!(status.description().contains("3"));

    let died = CombatFeedback::CombatantDied {
        combatant_id: "monster1".to_string(),
        combatant_type: CombatantType::Monster,
        cause: "overkill".to_string(),
    };
    assert!(died.description().contains("Monster"));
    assert!(died.description().contains("monster1"));
    assert!(died.description().contains("overkill"));
}

/// Verifies combat to combat HUD transition.
#[test]
fn adapter_combat_to_combat_hud_transition() {
    let actors = vec![
        make_actor_summary(1, CombatSide::Ally, 80.0, 100.0, vec![]),
        make_actor_summary(10, CombatSide::Enemy, 150.0, 200.0, vec![]),
    ];
    let framework_vm = make_framework_combat_vm(42, 3, actors, vec![1, 10]);

    let combat_vm = combat_from_framework(&framework_vm).unwrap();
    let hud_vm = combat_hud_from_combat(&combat_vm).unwrap();

    assert_eq!(hud_vm.encounter_id, "encounter_EncounterId(42)");
    assert_eq!(hud_vm.round, 3);
    assert_eq!(hud_vm.hero_vitals.len(), 1);
    assert_eq!(hud_vm.monster_vitals.len(), 1);
    assert!(hud_vm.is_combat_active());
}

/// Verifies combat HUD status count is mapped correctly.
#[test]
fn adapter_combat_hud_status_count() {
    let actors = vec![
        make_actor_summary(1, CombatSide::Ally, 80.0, 100.0, vec![("bleeding", Some(2)), ("vulnerable", None)]),
        make_actor_summary(10, CombatSide::Enemy, 150.0, 200.0, vec![]),
    ];
    let framework_vm = make_framework_combat_vm(1, 1, actors, vec![1, 10]);

    let combat_vm = combat_from_framework(&framework_vm).unwrap();
    let hud_vm = combat_hud_from_combat(&combat_vm).unwrap();

    let h1 = hud_vm.hero_vitals.iter().find(|h| h.id == "ActorId(1)").unwrap();
    assert_eq!(h1.status_count, 2);
}

// ── US-008-a: Replay-driven end-to-end validation for the vertical slice ────────

/// Replay fixture for BootLoadViewModel — represents initial game boot state.
fn make_replay_boot_load() -> BootLoadViewModel {
    BootLoadViewModel::success("Campaign loaded successfully", vec!["heroes", "monsters", "dungeons"])
        .with_campaign_version(1)
}

/// Replay fixture for TownViewModel — represents town visit with activities and roster.
fn make_replay_town_vm() -> TownViewModel {
    use game_ddgc_headless::contracts::{BuildingUpgradeState, CampaignHero, CampaignHeroQuirks, CampaignState};

    let mut campaign = CampaignState::new(1500);
    campaign.roster.push(CampaignHero {
        id: "h1".to_string(),
        class_id: "crusader".to_string(),
        level: 3,
        xp: 500,
        health: 80.0,
        max_health: 100.0,
        stress: 30.0,
        max_stress: 200.0,
        quirks: CampaignHeroQuirks::new(),
        equipment: Default::default(),
        skills: Vec::new(),
        traits: Default::default(),
    });
    campaign.roster.push(CampaignHero {
        id: "h2".to_string(),
        class_id: "hunter".to_string(),
        level: 2,
        xp: 300,
        health: 95.0,
        max_health: 100.0,
        stress: 50.0,
        max_stress: 200.0,
        quirks: CampaignHeroQuirks::new(),
        equipment: Default::default(),
        skills: Vec::new(),
        traits: Default::default(),
    });
    campaign.building_states.insert(
        "stagecoach".to_string(),
        BuildingUpgradeState::new("stagecoach", Some('a')),
    );

    town_from_campaign(&campaign).expect("town_from_campaign should succeed for valid replay fixture")
}

/// Replay fixture for DungeonViewModel — represents active dungeon run state.
fn make_replay_dungeon_vm() -> DungeonViewModel {
    let heroes = vec![
        make_hero_state("h1", "crusader", 80.0, 100.0, 30.0, 200.0),
        make_hero_state("h2", "hunter", 95.0, 100.0, 50.0, 200.0),
    ];
    let room_encounters = vec![
        RoomEncounterRecord {
            room_id: RoomId(1),
            room_kind: framework_progression::rooms::RoomKind::Combat,
            pack_id: "pack1".to_string(),
            family_ids: vec![],
        },
        RoomEncounterRecord {
            room_id: RoomId(2),
            room_kind: framework_progression::rooms::RoomKind::Boss,
            pack_id: "boss_pack".to_string(),
            family_ids: vec![],
        },
    ];

    let run_result = DdgcRunResult {
        run: make_test_run(),
        state: game_ddgc_headless::run::flow::DdgcRunState::new(),
        floor: make_test_floor(),
        battle_pack_ids: vec![],
        metadata: RunMetadata {
            dungeon_type: DungeonType::QingLong,
            map_size: MapSize::Short,
            base_room_number: 9,
            base_corridor_number: 4,
            gridsize: GridSize::new(5, 5),
            connectivity: 0.9,
        },
        room_encounters,
        interaction_records: vec![],
        camping_trace: vec![],
        heroes,
    };

    dungeon_from_run_result(&run_result).expect("dungeon_from_run_result should succeed for valid replay fixture")
}

/// Replay fixture for CombatViewModel — represents active combat state.
fn make_replay_combat_vm() -> DdgcCombatViewModel {
    let actors = vec![
        make_actor_summary(1, CombatSide::Ally, 80.0, 100.0, vec![]),
        make_actor_summary(2, CombatSide::Ally, 95.0, 100.0, vec![]),
        make_actor_summary(10, CombatSide::Enemy, 150.0, 200.0, vec![]),
        make_actor_summary(11, CombatSide::Enemy, 200.0, 200.0, vec![]),
    ];
    let framework_vm = make_framework_combat_vm(1, 1, actors, vec![1, 2, 10, 11]);

    combat_from_framework(&framework_vm).expect("combat_from_framework should succeed for valid replay fixture")
}

/// Replay fixture for CombatHudViewModel — represents combat HUD state.
fn make_replay_combat_hud_vm() -> CombatHudViewModel {
    let combat_vm = make_replay_combat_vm();
    combat_hud_from_combat(&combat_vm).expect("combat_hud_from_combat should succeed for valid replay fixture")
}

/// Replay fixture for ResultViewModel — represents dungeon/combat result state.
fn make_replay_result_vm() -> ResultViewModel {
    let heirlooms: std::collections::BTreeMap<HeirloomCurrency, u32> = std::collections::BTreeMap::new();

    result_from_run(
        DungeonType::ZhuQue,
        MapSize::Medium,
        14,
        6,
        true,
        800,
        200,
        &heirlooms,
        vec![],
    ).expect("result_from_run should succeed for valid replay fixture")
}

/// Replay fixture for ReturnFlowViewModel — represents return-to-town flow state.
fn make_replay_return_flow_vm() -> ReturnFlowViewModel {
    let heroes: Vec<(String, String, f64, f64, f64, f64)> = vec![
        ("h1".to_string(), "crusader".to_string(), 40.0, 100.0, 150.0, 200.0), // wounded, stressed
        ("h2".to_string(), "hunter".to_string(), 95.0, 100.0, 50.0, 200.0),
    ];
    let died_heroes: Vec<(String, String)> = vec![];

    return_flow_from_state(
        DungeonType::BaiHu,
        MapSize::Short,
        9,
        4,
        true,
        500,
        &heroes,
        &died_heroes,
    ).expect("return_flow_from_state should succeed for valid replay fixture")
}

/// Verifies BootLoadViewModel replay fixture renders without errors.
#[test]
fn replay_boot_load_fixture_renders_without_error() {
    let vm = make_replay_boot_load();
    assert!(vm.loaded, "BootLoad should be loaded");
    assert!(vm.error.is_none(), "BootLoad should have no error: {:?}", vm.error);
}

/// Verifies BootLoadViewModel replay fixture is deterministic.
#[test]
fn replay_boot_load_fixture_deterministic() {
    let vm1 = make_replay_boot_load();
    let vm2 = make_replay_boot_load();
    assert_eq!(vm1, vm2, "BootLoad fixture should be deterministic");
}

/// Verifies TownViewModel replay fixture renders without errors.
#[test]
fn replay_town_fixture_renders_without_error() {
    let vm = make_replay_town_vm();
    assert!(vm.error.is_none(), "Town should have no error: {:?}", vm.error);
    assert_eq!(vm.roster.len(), 2, "Town should have 2 heroes");
}

/// Verifies TownViewModel replay fixture is deterministic.
#[test]
fn replay_town_fixture_deterministic() {
    let vm1 = make_replay_town_vm();
    let vm2 = make_replay_town_vm();
    assert_eq!(vm1, vm2, "Town fixture should be deterministic");
}

/// Verifies DungeonViewModel replay fixture renders without errors.
#[test]
fn replay_dungeon_fixture_renders_without_error() {
    let vm = make_replay_dungeon_vm();
    assert!(vm.error.is_none(), "Dungeon should have no error: {:?}", vm.error);
    assert_eq!(vm.dungeon_type, "QingLong", "Dungeon type should be QingLong");
    assert_eq!(vm.map_size, "Short", "Map size should be Short");
}

/// Verifies DungeonViewModel replay fixture is deterministic.
#[test]
fn replay_dungeon_fixture_deterministic() {
    let vm1 = make_replay_dungeon_vm();
    let vm2 = make_replay_dungeon_vm();
    assert_eq!(vm1, vm2, "Dungeon fixture should be deterministic");
}

/// Verifies CombatViewModel replay fixture renders without errors.
#[test]
fn replay_combat_fixture_renders_without_error() {
    let vm = make_replay_combat_vm();
    assert!(vm.error.is_none(), "Combat should have no error: {:?}", vm.error);
    assert_eq!(vm.heroes.len(), 2, "Combat should have 2 heroes");
    assert_eq!(vm.monsters.len(), 2, "Combat should have 2 monsters");
}

/// Verifies CombatViewModel replay fixture is deterministic.
#[test]
fn replay_combat_fixture_deterministic() {
    let vm1 = make_replay_combat_vm();
    let vm2 = make_replay_combat_vm();
    assert_eq!(vm1, vm2, "Combat fixture should be deterministic");
}

/// Verifies CombatHudViewModel replay fixture renders without errors.
#[test]
fn replay_combat_hud_fixture_renders_without_error() {
    let vm = make_replay_combat_hud_vm();
    assert!(vm.is_combat_active(), "Combat HUD should show active combat");
    assert_eq!(vm.heroes_alive, 2, "Should have 2 heroes alive");
    assert_eq!(vm.monsters_alive, 2, "Should have 2 monsters alive");
}

/// Verifies CombatHudViewModel replay fixture is deterministic.
#[test]
fn replay_combat_hud_fixture_deterministic() {
    let vm1 = make_replay_combat_hud_vm();
    let vm2 = make_replay_combat_hud_vm();
    assert_eq!(vm1, vm2, "CombatHUD fixture should be deterministic");
}

/// Verifies ResultViewModel replay fixture renders without errors.
#[test]
fn replay_result_fixture_renders_without_error() {
    let vm = make_replay_result_vm();
    assert_eq!(vm.outcome, OutcomeType::Success, "Result should be Success");
    assert!(vm.error.is_none(), "Result should have no error: {:?}", vm.error);
}

/// Verifies ResultViewModel replay fixture is deterministic.
#[test]
fn replay_result_fixture_deterministic() {
    let vm1 = make_replay_result_vm();
    let vm2 = make_replay_result_vm();
    assert_eq!(vm1, vm2, "Result fixture should be deterministic");
}

/// Verifies ReturnFlowViewModel replay fixture renders without errors.
#[test]
fn replay_return_flow_fixture_renders_without_error() {
    let vm = make_replay_return_flow_vm();
    assert!(vm.error.is_none(), "ReturnFlow should have no error: {:?}", vm.error);
    assert_eq!(vm.heroes.len(), 2, "ReturnFlow should have 2 heroes");
}

/// Verifies ReturnFlowViewModel replay fixture is deterministic.
#[test]
fn replay_return_flow_fixture_deterministic() {
    let vm1 = make_replay_return_flow_vm();
    let vm2 = make_replay_return_flow_vm();
    assert_eq!(vm1, vm2, "ReturnFlow fixture should be deterministic");
}

/// Verifies vertical slice can be rendered end-to-end from replay fixtures.
///
/// This validates that all view models in the representative slice can be
/// created and rendered without errors from replay fixtures, proving the
/// contracts are stable for frontend consumption.
#[test]
fn replay_vertical_slice_end_to_end_renders() {
    // Boot
    let boot_vm = make_replay_boot_load();
    assert!(boot_vm.loaded);

    // Town
    let town_vm = make_replay_town_vm();
    assert!(town_vm.error.is_none());

    // Dungeon
    let dungeon_vm = make_replay_dungeon_vm();
    assert!(dungeon_vm.error.is_none());

    // Combat
    let combat_vm = make_replay_combat_vm();
    assert!(combat_vm.error.is_none());

    // Combat HUD
    let combat_hud_vm = make_replay_combat_hud_vm();
    assert!(combat_hud_vm.is_combat_active());

    // Result
    let result_vm = make_replay_result_vm();
    assert!(result_vm.error.is_none());

    // Return flow
    let return_vm = make_replay_return_flow_vm();
    assert!(return_vm.error.is_none());
}

/// Verifies ViewModelError descriptions are actionable for debugging.
///
/// When adapter mapping fails, the error should provide enough context
/// to identify the source of the failure without requiring deep framework knowledge.
#[test]
fn viewmodel_error_descriptions_are_actionable() {
    let err = ViewModelError::UnsupportedState {
        state_type: "Combat".to_string(),
        detail: "monster turn not supported".to_string(),
    };
    let desc = err.description();
    assert!(desc.contains("Combat"), "Error should mention state type");
    assert!(desc.contains("monster turn not supported"), "Error should mention detail");

    let err2 = ViewModelError::PartialMapping {
        state_type: "Town".to_string(),
        missing_fields: vec!["building_states".to_string(), "roster".to_string()],
    };
    let desc2 = err2.description();
    assert!(desc2.contains("Town"), "Error should mention state type");
    assert!(desc2.contains("building_states"), "Error should list missing fields");

    let err3 = ViewModelError::MissingRequiredField {
        field: "health".to_string(),
        context: "CampaignHero".to_string(),
    };
    let desc3 = err3.description();
    assert!(desc3.contains("health"), "Error should mention field");
    assert!(desc3.contains("CampaignHero"), "Error should mention context");

    let err4 = ViewModelError::IncompatibleSchema {
        expected: "2.0".to_string(),
        found: "1.0".to_string(),
    };
    let desc4 = err4.description();
    assert!(desc4.contains("2.0"), "Error should mention expected version");
    assert!(desc4.contains("1.0"), "Error should mention found version");
}

/// Verifies replay-driven and live-runtime validation consume same contract boundary.
///
/// Both replay fixtures and live-constructed payloads should produce valid
/// view models when passed through the same adapters, proving the contract
/// boundary is stable for frontend consumption.
#[test]
fn replay_and_live_consume_same_contract_boundary() {
    // Create live-constructed combat VM
    let actors = vec![
        make_actor_summary(1, CombatSide::Ally, 80.0, 100.0, vec![]),
        make_actor_summary(10, CombatSide::Enemy, 150.0, 200.0, vec![]),
    ];
    let framework_vm = make_framework_combat_vm(1, 1, actors, vec![1, 10]);
    let live_combat_vm = combat_from_framework(&framework_vm).unwrap();

    // Create replay fixture combat VM
    let replay_combat_vm = make_replay_combat_vm();

    // Both should be valid CombatViewModels (same structure, potentially different IDs)
    assert!(live_combat_vm.error.is_none(), "Live combat VM should have no error");
    assert!(replay_combat_vm.error.is_none(), "Replay combat VM should have no error");

    // Both should have heroes and monsters
    assert!(!live_combat_vm.heroes.is_empty(), "Live combat VM should have heroes");
    assert!(!replay_combat_vm.heroes.is_empty(), "Replay combat VM should have heroes");
    assert!(!live_combat_vm.monsters.is_empty(), "Live combat VM should have monsters");
    assert!(!replay_combat_vm.monsters.is_empty(), "Replay combat VM should have monsters");
}
