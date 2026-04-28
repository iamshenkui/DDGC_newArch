//! Integration tests for provisioning, expedition setup, and launch flow (US-006-c).
//!
//! Validates:
//! - The provisioning view model contracts are complete and round-trip through JSON
//! - The expedition setup view model contracts are complete and round-trip through JSON
//! - Expedition launch request/response contracts work correctly
//! - The adapter functions for provisioning and expedition produce correct results
//! - The town -> provision -> launch path is reproducible through the frontend shell
//! - The expedition launch path does not depend on hidden debug controls
//! - Typecheck passes
//! - Changes are scoped to the tests module
//!
//! These tests live in the integration test suite (`tests/`) rather than in
//! `#[cfg(test)]` modules within the source tree, satisfying the "scoped to the
//! tests module" acceptance criterion.

use game_ddgc_headless::contracts::viewmodels::{
    ExpeditionHeroSummary, ExpeditionLaunchRequest, ExpeditionLaunchResult,
    ExpeditionSetupViewModel, ProvisioningHeroSummary, ProvisioningViewModel, ViewModelError,
};
use game_ddgc_headless::contracts::{CampaignHero, CampaignState};
use game_ddgc_headless::state::{FlowState, FrontendIntent, GameState, NavigationShell, RuntimePayload};

// ── Test helpers ───────────────────────────────────────────────────────────────

/// Create a minimal campaign with sample heroes for provisioning tests.
fn make_provisioning_campaign(gold: u32) -> CampaignState {
    let mut campaign = CampaignState::new(gold);
    campaign.roster.push(CampaignHero::new(
        "h1", "crusader", 3, 500, 80.0, 100.0, 30.0, 200.0,
    ));
    campaign.roster.push(CampaignHero::new(
        "h2", "hunter", 2, 300, 100.0, 100.0, 200.0, 200.0,
    ));
    campaign.roster.push(CampaignHero::new(
        "h3", "alchemist", 2, 350, 90.0, 100.0, 15.0, 200.0,
    ));
    campaign.roster.push(CampaignHero::new(
        "h4", "shaman", 1, 120, 70.0, 100.0, 45.0, 200.0,
    ));
    campaign
}

/// Load a real GameState for adapter integration tests.
fn load_real_game_state() -> GameState {
    let data_dir = std::path::PathBuf::from("data");
    GameState::load_from(&data_dir).expect("failed to load state")
}

/// Helper: run the player-facing boot-to-town launch path.
fn run_player_facing_launch_path(shell: &mut NavigationShell) -> FlowState {
    let result = shell.transition_from_payload(RuntimePayload::BootComplete);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), FlowState::Load);

    let result = shell.transition_from_payload(RuntimePayload::CampaignLoaded);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), FlowState::Town);

    shell.current_state().clone()
}

// ── Part 1: ProvisioningViewModel contract tests ──────────────────────────────

/// Verifies ProvisioningViewModel can be constructed with all fields.
#[test]
fn provisioning_vm_construction() {
    let party = vec![
        ProvisioningHeroSummary::new("h1", "Shen", "Hunter", "38/42", "42", 38.0, 42.0, "17", "200", 2, 240, true, false, true),
        ProvisioningHeroSummary::new("h2", "Bai Xiu", "White", "41/41", "41", 41.0, 41.0, "8", "200", 2, 180, false, false, true),
    ];

    let vm = ProvisioningViewModel::new(
        "Provision Expedition",
        "The Azure Lantern",
        "The Depths Await",
        "Assemble your party and provision wisely.",
        party,
        4,
        true,
        "Adequate",
        "150 Gold",
    );

    assert_eq!(vm.kind, "provisioning");
    assert_eq!(vm.title, "Provision Expedition");
    assert_eq!(vm.campaign_name, "The Azure Lantern");
    assert_eq!(vm.expedition_label, "The Depths Await");
    assert_eq!(vm.expedition_summary, "Assemble your party and provision wisely.");
    assert_eq!(vm.party.len(), 2);
    assert_eq!(vm.max_party_size, 4);
    assert!(vm.is_ready_to_launch);
    assert_eq!(vm.supply_level, "Adequate");
    assert_eq!(vm.provision_cost, "150 Gold");
}

/// Verifies ProvisioningViewModel with empty party is not ready to launch.
#[test]
fn provisioning_vm_empty_party_not_ready() {
    let vm = ProvisioningViewModel::new(
        "Provision Expedition",
        "Test Campaign",
        "Test Expedition",
        "Test summary",
        vec![],
        4,
        false,
        "None",
        "0 Gold",
    );

    assert_eq!(vm.party.len(), 0);
    assert!(!vm.is_ready_to_launch);
}

/// Verifies ProvisioningViewModel round-trips through JSON.
#[test]
fn provisioning_vm_json_roundtrip() {
    let party = vec![
        ProvisioningHeroSummary::new("h1", "Shen", "Hunter", "38/42", "42", 38.0, 42.0, "17", "200", 2, 240, true, false, true),
    ];

    let original = ProvisioningViewModel::new(
        "Provision Expedition",
        "The Azure Lantern",
        "The Depths Await",
        "Assemble your party.",
        party,
        4,
        true,
        "Adequate",
        "150 Gold",
    );

    let json = serde_json::to_string(&original).expect("serialize should succeed");
    let restored: ProvisioningViewModel =
        serde_json::from_str(&json).expect("deserialize should succeed");

    assert_eq!(original.kind, restored.kind);
    assert_eq!(original.title, restored.title);
    assert_eq!(original.campaign_name, restored.campaign_name);
    assert_eq!(original.expedition_label, restored.expedition_label);
    assert_eq!(original.party.len(), restored.party.len());
    assert_eq!(original.max_party_size, restored.max_party_size);
    assert_eq!(original.is_ready_to_launch, restored.is_ready_to_launch);
    assert_eq!(original.supply_level, restored.supply_level);
    assert_eq!(original.provision_cost, restored.provision_cost);
}

/// Verifies ProvisioningViewModel kind field is always "provisioning".
#[test]
fn provisioning_vm_kind_is_provisioning() {
    let vm = ProvisioningViewModel::new("Title", "Campaign", "Expedition", "Summary", vec![], 4, false, "None", "0 Gold");
    assert_eq!(vm.kind, "provisioning");
}

/// Verifies ProvisioningViewModel party field accesses correctly.
#[test]
fn provisioning_vm_party_selectable_heroes() {
    let party = vec![
        ProvisioningHeroSummary::new("h1", "Shen", "Hunter", "38/42", "42", 38.0, 42.0, "17", "200", 2, 240, true, false, true),
        ProvisioningHeroSummary::new("h2", "Bai Xiu", "White", "41/41", "41", 41.0, 41.0, "8", "200", 2, 180, false, false, false),
        ProvisioningHeroSummary::new("h3", "Hei Zhen", "Black", "34/40", "40", 34.0, 40.0, "24", "200", 1, 60, true, false, true),
    ];

    let vm = ProvisioningViewModel::new("Title", "Campaign", "Expedition", "Summary", party, 4, true, "Adequate", "100 Gold");

    assert_eq!(vm.party.len(), 3);
    assert!(vm.party[0].is_selected);
    assert!(!vm.party[1].is_selected);
    assert!(vm.party[2].is_selected);
    assert!(vm.party[0].is_wounded);
    assert!(!vm.party[1].is_wounded);
    assert_eq!(vm.party[0].level, 2);
    assert_eq!(vm.party[1].class_label, "White");
    assert_eq!(vm.party[2].hp, "34/40");
}

/// Verifies ProvisioningViewModel max_party_size is correctly enforced by the frontend.
#[test]
fn provisioning_vm_max_party_size_sufficient() {
    let party = vec![
        ProvisioningHeroSummary::new("h1", "A", "Crusader", "40/40", "40", 40.0, 40.0, "0", "200", 1, 0, false, false, true),
        ProvisioningHeroSummary::new("h2", "B", "Hunter", "40/40", "40", 40.0, 40.0, "0", "200", 1, 0, false, false, true),
        ProvisioningHeroSummary::new("h3", "C", "Alchemist", "40/40", "40", 40.0, 40.0, "0", "200", 1, 0, false, false, true),
        ProvisioningHeroSummary::new("h4", "D", "Shaman", "40/40", "40", 40.0, 40.0, "0", "200", 1, 0, false, false, true),
    ];

    // Max party size of 4 with 4 selected = ready
    let vm = ProvisioningViewModel::new("Title", "Campaign", "Expedition", "Summary", party, 4, true, "Adequate", "100 Gold");
    assert!(vm.is_ready_to_launch);
    assert_eq!(vm.party.len(), 4);
    assert_eq!(vm.max_party_size, 4);
}

// ── Part 2: ProvisioningHeroSummary contract tests ────────────────────────────

/// Verifies ProvisioningHeroSummary can be constructed correctly.
#[test]
fn provisioning_hero_summary_construction() {
    let hero = ProvisioningHeroSummary::new(
        "hero-hunter-01", "Shen", "Hunter",
        "38 / 42", "42", 38.0, 42.0,
        "17", "200", 2, 240,
        true, false, true,
    );

    assert_eq!(hero.id, "hero-hunter-01");
    assert_eq!(hero.name, "Shen");
    assert_eq!(hero.class_label, "Hunter");
    assert_eq!(hero.hp, "38 / 42");
    assert_eq!(hero.max_hp, "42");
    assert!((hero.health - 38.0).abs() < f64::EPSILON);
    assert!((hero.max_health - 42.0).abs() < f64::EPSILON);
    assert_eq!(hero.stress, "17");
    assert_eq!(hero.max_stress, "200");
    assert_eq!(hero.level, 2);
    assert_eq!(hero.xp, 240);
    assert!(hero.is_wounded);
    assert!(!hero.is_afflicted);
    assert!(hero.is_selected);
}

/// Verifies ProvisioningHeroSummary round-trips through JSON.
#[test]
fn provisioning_hero_summary_json_roundtrip() {
    let original = ProvisioningHeroSummary::new(
        "hero-hunter-01", "Shen", "Hunter",
        "38 / 42", "42", 38.0, 42.0,
        "17", "200", 2, 240,
        true, false, true,
    );

    let json = serde_json::to_string(&original).expect("serialize should succeed");
    let restored: ProvisioningHeroSummary =
        serde_json::from_str(&json).expect("deserialize should succeed");

    assert_eq!(original.id, restored.id);
    assert_eq!(original.name, restored.name);
    assert_eq!(original.class_label, restored.class_label);
    assert_eq!(original.hp, restored.hp);
    assert_eq!(original.level, restored.level);
    assert_eq!(original.xp, restored.xp);
    assert_eq!(original.is_selected, restored.is_selected);
    assert_eq!(original.is_wounded, restored.is_wounded);
    assert_eq!(original.is_afflicted, restored.is_afflicted);
}

/// Verifies ProvisioningHeroSummary can represent wounded and afflicted states.
#[test]
fn provisioning_hero_summary_wounded_afflicted() {
    let healthy = ProvisioningHeroSummary::new(
        "h1", "A", "Crusader", "100/100", "100", 100.0, 100.0, "0", "200", 1, 0,
        false, false, true,
    );
    assert!(!healthy.is_wounded);
    assert!(!healthy.is_afflicted);

    let wounded = ProvisioningHeroSummary::new(
        "h2", "B", "Hunter", "50/100", "100", 50.0, 100.0, "0", "200", 1, 0,
        true, false, true,
    );
    assert!(wounded.is_wounded);
    assert!(!wounded.is_afflicted);

    let afflicted = ProvisioningHeroSummary::new(
        "h3", "C", "Alchemist", "100/100", "100", 100.0, 100.0, "200", "200", 1, 0,
        false, true, true,
    );
    assert!(!afflicted.is_wounded);
    assert!(afflicted.is_afflicted);

    let both = ProvisioningHeroSummary::new(
        "h4", "D", "Shaman", "30/100", "100", 30.0, 100.0, "180", "200", 1, 0,
        true, true, true,
    );
    assert!(both.is_wounded);
    assert!(both.is_afflicted);
}

// ── Part 3: ExpeditionSetupViewModel contract tests ───────────────────────────

/// Verifies ExpeditionSetupViewModel can be constructed with all fields.
#[test]
fn expedition_setup_vm_construction() {
    let party = vec![
        ExpeditionHeroSummary::new("h1", "Shen", "Hunter", "38/42", "42", "17", "200"),
        ExpeditionHeroSummary::new("h2", "Bai Xiu", "White", "41/41", "41", "8", "200"),
    ];

    let vm = ExpeditionSetupViewModel::new(
        "Expedition Launch",
        "The Depths Await",
        2,
        party,
        "Challenging",
        "Medium",
        vec!["Explore the dungeon".to_string(), "Collect resources".to_string()],
        vec!["Elevated enemy presence".to_string()],
        "Adequate",
        "150 Gold",
        true,
    );

    assert_eq!(vm.kind, "expedition");
    assert_eq!(vm.title, "Expedition Launch");
    assert_eq!(vm.expedition_name, "The Depths Await");
    assert_eq!(vm.party_size, 2);
    assert_eq!(vm.party.len(), 2);
    assert_eq!(vm.difficulty, "Challenging");
    assert_eq!(vm.estimated_duration, "Medium");
    assert_eq!(vm.objectives.len(), 2);
    assert_eq!(vm.warnings.len(), 1);
    assert_eq!(vm.supply_level, "Adequate");
    assert_eq!(vm.provision_cost, "150 Gold");
    assert!(vm.is_launchable);
}

/// Verifies ExpeditionSetupViewModel with empty party is not launchable.
#[test]
fn expedition_setup_vm_empty_party_not_launchable() {
    let vm = ExpeditionSetupViewModel::new(
        "Expedition Launch",
        "Test Expedition",
        0,
        vec![],
        "Normal",
        "Short",
        vec![],
        vec![],
        "None",
        "0 Gold",
        false,
    );

    assert_eq!(vm.party_size, 0);
    assert!(vm.party.is_empty());
    assert!(!vm.is_launchable);
}

/// Verifies ExpeditionSetupViewModel round-trips through JSON.
#[test]
fn expedition_setup_vm_json_roundtrip() {
    let party = vec![
        ExpeditionHeroSummary::new("h1", "Shen", "Hunter", "38/42", "42", "17", "200"),
    ];

    let original = ExpeditionSetupViewModel::new(
        "Expedition Launch",
        "The Depths Await",
        1,
        party,
        "Challenging",
        "Medium",
        vec!["Explore the dungeon".to_string()],
        vec![],
        "Adequate",
        "150 Gold",
        true,
    );

    let json = serde_json::to_string(&original).expect("serialize should succeed");
    let restored: ExpeditionSetupViewModel =
        serde_json::from_str(&json).expect("deserialize should succeed");

    assert_eq!(original.kind, restored.kind);
    assert_eq!(original.title, restored.title);
    assert_eq!(original.expedition_name, restored.expedition_name);
    assert_eq!(original.party_size, restored.party_size);
    assert_eq!(original.party.len(), restored.party.len());
    assert_eq!(original.difficulty, restored.difficulty);
    assert_eq!(original.estimated_duration, restored.estimated_duration);
    assert_eq!(original.supply_level, restored.supply_level);
    assert_eq!(original.provision_cost, restored.provision_cost);
    assert_eq!(original.is_launchable, restored.is_launchable);
}

/// Verifies ExpeditionSetupViewModel kind field is always "expedition".
#[test]
fn expedition_setup_vm_kind_is_expedition() {
    let vm = ExpeditionSetupViewModel::new("Title", "Name", 0, vec![], "Easy", "Short", vec![], vec![], "None", "0 Gold", false);
    assert_eq!(vm.kind, "expedition");
}

/// Verifies ExpeditionSetupViewModel carries multiple objectives and warnings.
#[test]
fn expedition_setup_vm_objectives_and_warnings() {
    let objectives = vec![
        "Explore the dungeon level".to_string(),
        "Collect resources".to_string(),
        "Return with treasures".to_string(),
        "Defeat the boss".to_string(),
    ];
    let warnings = vec![
        "Elevated enemy presence detected".to_string(),
        "Limited camping opportunities".to_string(),
        "Party has wounded heroes".to_string(),
    ];

    let vm = ExpeditionSetupViewModel::new(
        "Expedition Launch", "The Depths Await", 2, vec![], "Hard", "Long",
        objectives.clone(), warnings.clone(),
        "Adequate", "200 Gold", true,
    );

    assert_eq!(vm.objectives.len(), 4);
    assert_eq!(vm.warnings.len(), 3);
    for (i, obj) in objectives.iter().enumerate() {
        assert_eq!(&vm.objectives[i], obj);
    }
    for (i, warn) in warnings.iter().enumerate() {
        assert_eq!(&vm.warnings[i], warn);
    }
}

// ── Part 4: ExpeditionHeroSummary contract tests ──────────────────────────────

/// Verifies ExpeditionHeroSummary can be constructed correctly.
#[test]
fn expedition_hero_summary_construction() {
    let hero = ExpeditionHeroSummary::new(
        "hero-hunter-01", "Shen", "Hunter",
        "38 / 42", "42", "17", "200",
    );

    assert_eq!(hero.id, "hero-hunter-01");
    assert_eq!(hero.name, "Shen");
    assert_eq!(hero.class_label, "Hunter");
    assert_eq!(hero.hp, "38 / 42");
    assert_eq!(hero.max_hp, "42");
    assert_eq!(hero.stress, "17");
    assert_eq!(hero.max_stress, "200");
}

/// Verifies ExpeditionHeroSummary round-trips through JSON.
#[test]
fn expedition_hero_summary_json_roundtrip() {
    let original = ExpeditionHeroSummary::new(
        "hero-hunter-01", "Shen", "Hunter",
        "38 / 42", "42", "17", "200",
    );

    let json = serde_json::to_string(&original).expect("serialize should succeed");
    let restored: ExpeditionHeroSummary =
        serde_json::from_str(&json).expect("deserialize should succeed");

    assert_eq!(original.id, restored.id);
    assert_eq!(original.name, restored.name);
    assert_eq!(original.class_label, restored.class_label);
    assert_eq!(original.hp, restored.hp);
    assert_eq!(original.max_hp, restored.max_hp);
    assert_eq!(original.stress, restored.stress);
    assert_eq!(original.max_stress, restored.max_stress);
}

/// Verifies ExpeditionHeroSummary can represent multiple heroes in a party.
#[test]
fn expedition_hero_summary_party_list() {
    let party = vec![
        ExpeditionHeroSummary::new("h1", "Shen", "Hunter", "38/42", "42", "17", "200"),
        ExpeditionHeroSummary::new("h2", "Bai Xiu", "White", "41/41", "41", "8", "200"),
        ExpeditionHeroSummary::new("h3", "Hei Zhen", "Black", "34/40", "40", "24", "200"),
    ];

    assert_eq!(party.len(), 3);
    assert_eq!(party[0].class_label, "Hunter");
    assert_eq!(party[1].class_label, "White");
    assert_eq!(party[2].class_label, "Black");
}

// ── Part 5: ExpeditionLaunchRequest contract tests ────────────────────────────

/// Verifies ExpeditionLaunchRequest can be constructed correctly.
#[test]
fn expedition_launch_request_construction() {
    let request = ExpeditionLaunchRequest::new(vec!["h1".to_string(), "h2".to_string()]);

    assert_eq!(request.selected_hero_ids.len(), 2);
    assert!(request.selected_hero_ids.contains(&"h1".to_string()));
    assert!(request.selected_hero_ids.contains(&"h2".to_string()));
    assert!(request.quest_id.is_none());
    assert!(request.supply_level.is_none());
}

/// Verifies ExpeditionLaunchRequest builder method with_quest sets quest_id.
#[test]
fn expedition_launch_request_with_quest() {
    let request = ExpeditionLaunchRequest::new(vec!["h1".to_string()])
        .with_quest("kill_boss_qinglong_short");

    assert_eq!(request.quest_id, Some("kill_boss_qinglong_short".to_string()));
}

/// Verifies ExpeditionLaunchRequest builder method with_supply sets supply_level.
#[test]
fn expedition_launch_request_with_supply() {
    let request = ExpeditionLaunchRequest::new(vec!["h1".to_string()])
        .with_supply("Adequate");

    assert_eq!(request.supply_level, Some("Adequate".to_string()));
}

/// Verifies ExpeditionLaunchRequest builder methods can be chained.
#[test]
fn expedition_launch_request_chained_builders() {
    let request = ExpeditionLaunchRequest::new(vec!["h1".to_string(), "h2".to_string()])
        .with_quest("kill_boss_qinglong_short")
        .with_supply("Generous");

    assert_eq!(request.selected_hero_ids.len(), 2);
    assert_eq!(request.quest_id, Some("kill_boss_qinglong_short".to_string()));
    assert_eq!(request.supply_level, Some("Generous".to_string()));
}

/// Verifies ExpeditionLaunchRequest round-trips through JSON.
#[test]
fn expedition_launch_request_json_roundtrip() {
    let original = ExpeditionLaunchRequest::new(vec!["h1".to_string(), "h2".to_string()])
        .with_quest("test_quest")
        .with_supply("Adequate");

    let json = serde_json::to_string(&original).expect("serialize should succeed");
    let restored: ExpeditionLaunchRequest =
        serde_json::from_str(&json).expect("deserialize should succeed");

    assert_eq!(original.selected_hero_ids.len(), restored.selected_hero_ids.len());
    assert_eq!(original.quest_id, restored.quest_id);
    assert_eq!(original.supply_level, restored.supply_level);
}

/// Verifies ExpeditionLaunchRequest can be created with empty hero list.
#[test]
fn expedition_launch_request_empty_heroes() {
    let request = ExpeditionLaunchRequest::new(vec![]);
    assert!(request.selected_hero_ids.is_empty());
}

// ── Part 6: ExpeditionLaunchResult contract tests ─────────────────────────────

/// Verifies ExpeditionLaunchResult success can be constructed correctly.
#[test]
fn expedition_launch_result_success_construction() {
    let result = ExpeditionLaunchResult::success(
        "Expedition launched successfully",
        "The Depths Await",
        vec!["h1".to_string(), "h2".to_string()],
        Some("quest_01".to_string()),
        150,
        Some("QingLong".to_string()),
        Some("Short".to_string()),
    );

    assert!(result.success);
    assert_eq!(result.message, "Expedition launched successfully");
    assert_eq!(result.expedition_name, "The Depths Await");
    assert_eq!(result.selected_heroes.len(), 2);
    assert_eq!(result.quest_id, Some("quest_01".to_string()));
    assert_eq!(result.gold_cost, 150);
    assert_eq!(result.dungeon_type, Some("QingLong".to_string()));
    assert_eq!(result.map_size, Some("Short".to_string()));
    assert_eq!(result.next_state, "dungeon");
    assert!(result.error.is_none());
}

/// Verifies ExpeditionLaunchResult success without optional fields.
#[test]
fn expedition_launch_result_success_without_optionals() {
    let result = ExpeditionLaunchResult::success(
        "Expedition launched",
        "Test Expedition",
        vec!["h1".to_string()],
        None,
        50,
        None,
        None,
    );

    assert!(result.success);
    assert_eq!(result.selected_heroes.len(), 1);
    assert!(result.quest_id.is_none());
    assert!(result.dungeon_type.is_none());
    assert!(result.map_size.is_none());
    assert!(result.error.is_none());
}

/// Verifies ExpeditionLaunchResult failure can be constructed correctly.
#[test]
fn expedition_launch_result_failure_construction() {
    let error = ViewModelError::MissingRequiredField {
        field: "hero_id".to_string(),
        context: "No heroes selected for expedition".to_string(),
    };
    let result = ExpeditionLaunchResult::failure("No heroes selected", error);

    assert!(!result.success);
    assert_eq!(result.message, "No heroes selected");
    assert!(result.selected_heroes.is_empty());
    assert!(result.quest_id.is_none());
    assert_eq!(result.gold_cost, 0);
    assert!(result.dungeon_type.is_none());
    assert!(result.map_size.is_none());
    assert_eq!(result.next_state, "town");
    assert!(result.error.is_some());
}

/// Verifies ExpeditionLaunchResult failure with ViewModelError::UnsupportedState.
#[test]
fn expedition_launch_result_failure_unsupported_state() {
    let error = ViewModelError::UnsupportedState {
        state_type: "expedition".to_string(),
        detail: "Campaign has no available quests".to_string(),
    };
    let result = ExpeditionLaunchResult::failure("No quests available", error);

    assert!(!result.success);
    assert!(result.error.is_some());
    let err = result.error.unwrap();
    match err {
        ViewModelError::UnsupportedState { state_type, .. } => {
            assert_eq!(state_type, "expedition");
        }
        other => panic!("expected UnsupportedState, got: {:?}", other),
    }
}

/// Verifies ExpeditionLaunchResult round-trips through JSON (success).
#[test]
fn expedition_launch_result_json_roundtrip_success() {
    let original = ExpeditionLaunchResult::success(
        "Expedition launched successfully",
        "The Depths Await",
        vec!["h1".to_string()],
        None,
        100,
        Some("QingLong".to_string()),
        Some("Short".to_string()),
    );

    let json = serde_json::to_string(&original).expect("serialize should succeed");
    let restored: ExpeditionLaunchResult =
        serde_json::from_str(&json).expect("deserialize should succeed");

    assert_eq!(original.success, restored.success);
    assert_eq!(original.message, restored.message);
    assert_eq!(original.gold_cost, restored.gold_cost);
    assert_eq!(original.selected_heroes.len(), restored.selected_heroes.len());
    assert_eq!(original.dungeon_type, restored.dungeon_type);
    assert_eq!(original.map_size, restored.map_size);
    assert_eq!(original.next_state, restored.next_state);
}

/// Verifies ExpeditionLaunchResult round-trips through JSON (failure).
#[test]
fn expedition_launch_result_json_roundtrip_failure() {
    let error = ViewModelError::MissingRequiredField {
        field: "heroes".to_string(),
        context: "No heroes provided".to_string(),
    };
    let original = ExpeditionLaunchResult::failure("Failed to launch", error);

    let json = serde_json::to_string(&original).expect("serialize should succeed");
    let restored: ExpeditionLaunchResult =
        serde_json::from_str(&json).expect("deserialize should succeed");

    assert_eq!(original.success, restored.success);
    assert_eq!(original.message, restored.message);
    assert!(restored.error.is_some());
}

// ── Part 7: Adapter integration tests ──────────────────────────────────────────

/// Verifies provisioning_from_campaign adapter works with a GameState campaign.
#[test]
fn provisioning_adapter_from_game_state() {
    let mut state = load_real_game_state();
    state.new_campaign(1000);

    // Add heroes to the campaign roster
    let campaign = &mut state.campaign;
    campaign.roster.push(CampaignHero::new(
        "h1", "crusader", 3, 500, 80.0, 100.0, 30.0, 200.0,
    ));
    campaign.roster.push(CampaignHero::new(
        "h2", "hunter", 2, 300, 100.0, 100.0, 200.0, 200.0,
    ));

    use game_ddgc_headless::contracts::adapters::provisioning_from_campaign;
    let result = provisioning_from_campaign(
        &state.campaign,
        &["h1".to_string()],
        "The Depths Await",
        "Explore the ancient ruins.",
    );

    assert!(result.is_ok(), "provisioning_from_campaign should succeed");
    let vm = result.unwrap();
    assert_eq!(vm.kind, "provisioning");
    assert_eq!(vm.party.len(), 2);
    // h1 is selected, h2 is not
    assert!(vm.party[0].is_selected);
    assert!(!vm.party[1].is_selected);
    // One hero selected = ready (minimum 1 for this campaign)
    assert!(vm.is_ready_to_launch);
}

/// Verifies provisioning_from_campaign with empty selection is not ready.
#[test]
fn provisioning_adapter_empty_selection_not_ready() {
    let campaign = make_provisioning_campaign(1000);

    use game_ddgc_headless::contracts::adapters::provisioning_from_campaign;
    let result = provisioning_from_campaign(&campaign, &[], "Test", "Test summary");

    assert!(result.is_ok());
    let vm = result.unwrap();
    assert!(!vm.is_ready_to_launch);
    assert_eq!(vm.supply_level, "None");
}

/// Verifies provisioning_hero_selection toggles a hero on.
#[test]
fn provisioning_adapter_hero_selection_toggle_on() {
    let campaign = make_provisioning_campaign(1000);

    use game_ddgc_headless::contracts::adapters::provisioning_hero_selection;
    let result = provisioning_hero_selection(&campaign, &["h1".to_string()], "h2");

    assert!(result.is_ok());
    let updated = result.unwrap();
    assert_eq!(updated.len(), 2);
    assert!(updated.contains(&"h1".to_string()));
    assert!(updated.contains(&"h2".to_string()));
}

/// Verifies provisioning_hero_selection toggles a hero off.
#[test]
fn provisioning_adapter_hero_selection_toggle_off() {
    let campaign = make_provisioning_campaign(1000);

    use game_ddgc_headless::contracts::adapters::provisioning_hero_selection;
    let result = provisioning_hero_selection(&campaign, &["h1".to_string(), "h2".to_string()], "h1");

    assert!(result.is_ok());
    let updated = result.unwrap();
    assert_eq!(updated.len(), 1);
    assert!(updated.contains(&"h2".to_string()));
}

/// Verifies provisioning_hero_selection returns error for nonexistent hero.
#[test]
fn provisioning_adapter_hero_selection_nonexistent_hero() {
    let campaign = make_provisioning_campaign(1000);

    use game_ddgc_headless::contracts::adapters::provisioning_hero_selection;
    let result = provisioning_hero_selection(&campaign, &[], "nonexistent");

    assert!(result.is_err());
    match result.err().unwrap() {
        ViewModelError::MissingRequiredField { field, .. } => {
            assert_eq!(field, "hero_id");
        }
        other => panic!("expected MissingRequiredField, got: {:?}", other),
    }
}

/// Verifies expedition_setup_from_data adapter works with selected heroes.
#[test]
fn expedition_setup_adapter_from_game_state() {
    let campaign = make_provisioning_campaign(1000);
    let selected = vec!["h1".to_string(), "h2".to_string()];

    use game_ddgc_headless::contracts::adapters::expedition_setup_from_data;
    let result = expedition_setup_from_data(
        &campaign,
        &selected,
        None,
        "Adequate",
        "100 Gold",
    );

    assert!(result.is_ok(), "expedition_setup_from_data should succeed");
    let vm = result.unwrap();
    assert_eq!(vm.kind, "expedition");
    assert_eq!(vm.party_size, 2);
    assert!(vm.is_launchable);
    assert_eq!(vm.party.len(), 2);
}

/// Verifies expedition_setup_from_data works with a quest definition.
#[test]
fn expedition_setup_adapter_with_quest() {
    use game_ddgc_headless::contracts::{QuestDefinition, QuestDifficulty, QuestPenalties, QuestRewards, QuestType, DungeonType, MapSize};

    let campaign = make_provisioning_campaign(1000);
    let selected = vec!["h1".to_string()];

    let quest = QuestDefinition::new(
        "kill_boss_expedition",
        QuestType::KillBoss,
        DungeonType::QingLong,
        MapSize::Medium,
        QuestDifficulty::Standard,
        2,
        QuestRewards::standard(),
        QuestPenalties::standard(),
    );

    use game_ddgc_headless::contracts::adapters::expedition_setup_from_data;
    let result = expedition_setup_from_data(
        &campaign,
        &selected,
        Some(&quest),
        "Adequate",
        "50 Gold",
    );

    assert!(result.is_ok(), "expedition_setup_from_data with quest should succeed");
    let vm = result.unwrap();
    assert!(vm.is_launchable);
    assert_eq!(vm.party_size, 1);
}

/// Verifies expedition_launch adapter successfully launches an expedition.
#[test]
fn expedition_launch_adapter_success() {
    let campaign = make_provisioning_campaign(1500);
    let request = ExpeditionLaunchRequest::new(vec!["h1".to_string(), "h2".to_string()]);

    use game_ddgc_headless::contracts::adapters::expedition_launch;
    let result = expedition_launch(&campaign, &request);

    assert!(result.success);
    assert_eq!(result.selected_heroes.len(), 2);
    assert_eq!(result.gold_cost, 100); // 2 * 50
    assert_eq!(result.next_state, "dungeon");
    assert!(result.error.is_none());
}

/// Verifies expedition_launch fails with empty party.
#[test]
fn expedition_launch_adapter_empty_party_fails() {
    let campaign = make_provisioning_campaign(1000);
    let request = ExpeditionLaunchRequest::new(vec![]);

    use game_ddgc_headless::contracts::adapters::expedition_launch;
    let result = expedition_launch(&campaign, &request);

    assert!(!result.success);
    assert!(result.error.is_some());
}

/// Verifies expedition_launch fails with too many heroes.
#[test]
fn expedition_launch_adapter_too_many_heroes_fails() {
    let campaign = make_provisioning_campaign(1000);
    let request = ExpeditionLaunchRequest::new(vec![
        "h1".to_string(), "h2".to_string(),
        "h3".to_string(), "h4".to_string(), "h5".to_string(),
    ]);

    use game_ddgc_headless::contracts::adapters::expedition_launch;
    let result = expedition_launch(&campaign, &request);

    assert!(!result.success);
    assert!(result.error.is_some());
}

/// Verifies expedition_launch fails with nonexistent hero.
#[test]
fn expedition_launch_adapter_nonexistent_hero_fails() {
    let campaign = make_provisioning_campaign(1000);
    let request = ExpeditionLaunchRequest::new(vec!["nonexistent".to_string()]);

    use game_ddgc_headless::contracts::adapters::expedition_launch;
    let result = expedition_launch(&campaign, &request);

    assert!(!result.success);
    assert!(result.error.is_some());
}

/// Verifies expedition_launch deducts correct gold per hero.
#[test]
fn expedition_launch_adapter_deducts_gold() {
    let campaign = make_provisioning_campaign(1000);
    let request = ExpeditionLaunchRequest::new(vec!["h1".to_string()]);

    use game_ddgc_headless::contracts::adapters::expedition_launch;
    let result = expedition_launch(&campaign, &request);

    assert!(result.success);
    assert_eq!(result.gold_cost, 50); // 1 * 50
}

// ── Part 8: Flow integration tests ─────────────────────────────────────────────

/// Verifies the complete town -> provisioning -> expedition -> launch flow
/// using the frontend navigation shell and contract adapters together.
#[test]
fn town_to_provisioning_to_launch_full_flow() {
    // Step 1: Boot into Town via the standard player path
    let mut shell = NavigationShell::new();
    run_player_facing_launch_path(&mut shell);
    assert_eq!(shell.current_state(), FlowState::Town);

    // Step 2: Create campaign with heroes for provisioning
    let campaign = make_provisioning_campaign(2000);
    let selected = vec!["h1".to_string(), "h2".to_string(), "h3".to_string()];

    // Step 3: Provision the expedition
    use game_ddgc_headless::contracts::adapters::provisioning_from_campaign;
    let prov_result = provisioning_from_campaign(
        &campaign,
        &selected,
        "The Depths Await",
        "Assemble your party and venture into the depths.",
    );
    assert!(prov_result.is_ok());
    let prov_vm = prov_result.unwrap();
    assert_eq!(prov_vm.kind, "provisioning");
    assert_eq!(prov_vm.party.len(), 4);
    assert!(prov_vm.is_ready_to_launch);

    // Step 4: Set up the expedition review
    use game_ddgc_headless::contracts::adapters::expedition_setup_from_data;
    let setup_result = expedition_setup_from_data(
        &campaign,
        &selected,
        None,
        "Adequate",
        "150 Gold",
    );
    assert!(setup_result.is_ok());
    let setup_vm = setup_result.unwrap();
    assert_eq!(setup_vm.kind, "expedition");
    assert_eq!(setup_vm.party_size, 3);
    assert!(setup_vm.is_launchable);

    // Step 5: Launch the expedition
    use game_ddgc_headless::contracts::adapters::expedition_launch;
    let launch_result = expedition_launch(
        &campaign,
        &ExpeditionLaunchRequest::new(selected.clone()),
    );
    assert!(launch_result.success);
    assert_eq!(launch_result.selected_heroes.len(), 3);
    assert_eq!(launch_result.gold_cost, 150); // 3 * 50
    assert_eq!(launch_result.next_state, "dungeon");

    // Step 6: Verify the frontend shell can transition to Expedition
    let transition_result = shell.transition_from_intent(FrontendIntent::StartExpedition);
    assert!(transition_result.is_some());
    assert_eq!(shell.current_state(), FlowState::Expedition);
    assert_eq!(shell.previous_state(), FlowState::Town);
}

/// Verifies the provisioning flow works without relying on force_transition.
#[test]
fn provisioning_flow_without_debug_shortcuts() {
    let mut shell = NavigationShell::new();
    run_player_facing_launch_path(&mut shell);

    // Navigation shell transitions should not use force_transition
    let result = shell.transition_from_intent(FrontendIntent::StartExpedition);
    assert!(result.is_some(), "StartExpedition should work without shortcuts");
    assert_eq!(shell.current_state(), FlowState::Expedition);

    // Verify the provisioning contracts are also shortcut-free
    let campaign = make_provisioning_campaign(1000);
    use game_ddgc_headless::contracts::adapters::{provisioning_from_campaign, expedition_launch};
    use game_ddgc_headless::contracts::viewmodels::ExpeditionLaunchRequest;

    // Provisioning adapter — no force, no debug
    let prov = provisioning_from_campaign(&campaign, &["h1".to_string()], "Test", "Test");
    assert!(prov.is_ok());
    let prov_vm = prov.unwrap();
    assert!(prov_vm.is_ready_to_launch);

    // Launch adapter — no force, no debug
    let launch = expedition_launch(&campaign, &ExpeditionLaunchRequest::new(vec!["h1".to_string()]));
    assert!(launch.success);
    assert_eq!(launch.selected_heroes.len(), 1);
}

/// Verifies the provisioning flow is reproducible (deterministic).
#[test]
fn provisioning_flow_is_deterministic() {
    let campaign = make_provisioning_campaign(1000);
    let selected = vec!["h1".to_string(), "h2".to_string()];

    // Run the full provisioning -> setup -> launch flow twice
    let run_flow = |campaign: &CampaignState| -> (String, u32, u32, bool) {
        use game_ddgc_headless::contracts::adapters::{provisioning_from_campaign, expedition_setup_from_data, expedition_launch};

        let prov = provisioning_from_campaign(campaign, &selected, "Test", "Test").unwrap();
        let setup = expedition_setup_from_data(campaign, &selected, None, "Adequate", "100 Gold").unwrap();
        let launch = expedition_launch(campaign, &ExpeditionLaunchRequest::new(selected.clone()));

        (prov.kind, prov.party.len() as u32, setup.party_size, launch.success)
    };

    let (kind1, party_len1, party_size1, success1) = run_flow(&campaign);
    let (kind2, party_len2, party_size2, success2) = run_flow(&campaign);

    assert_eq!(kind1, kind2);
    assert_eq!(party_len1, party_len2);
    assert_eq!(party_size1, party_size2);
    assert_eq!(success1, success2);
}

/// Verifies the complete expedition lifecycle: provision -> launch -> result -> return.
#[test]
fn provisioning_full_lifecycle_with_shell() {
    let mut shell = NavigationShell::new();
    run_player_facing_launch_path(&mut shell);

    // Town -> Expedition via intent
    let result = shell.transition_from_intent(FrontendIntent::StartExpedition);
    assert!(result.is_some());
    assert_eq!(shell.current_state(), FlowState::Expedition);

    // Expedition -> Combat via intent
    let result = shell.transition_from_intent(FrontendIntent::EnterCombat);
    assert!(result.is_some());
    assert_eq!(shell.current_state(), FlowState::Combat);

    // Combat -> Expedition via ExitCombat
    let result = shell.transition_from_intent(FrontendIntent::ExitCombat);
    assert!(result.is_some());
    assert_eq!(shell.current_state(), FlowState::Expedition);

    // Expedition -> Result via ExpeditionCompleted payload
    let result = shell.transition_from_payload(RuntimePayload::ExpeditionCompleted);
    assert!(result.is_some());
    assert_eq!(shell.current_state(), FlowState::Result);

    // Result -> Town via Continue
    let result = shell.transition_from_intent(FrontendIntent::Continue);
    assert!(result.is_some());
    assert_eq!(shell.current_state(), FlowState::Town);
}

/// Verifies the provisioning view model is usable via the frontend contract boundary.
#[test]
fn provisioning_vm_usable_via_contract_boundary() {
    // This test validates that the provisioning view model fields match
    // the frontend contract expectations for DdgcFrontendIntent types
    let campaign = make_provisioning_campaign(1000);

    use game_ddgc_headless::contracts::adapters::provisioning_from_campaign;
    let vm = provisioning_from_campaign(&campaign, &["h1".to_string()], "The Depths Await", "Explore")
        .expect("provisioning should succeed");

    // Contract boundary: frontend expects these fields
    assert_eq!(vm.kind, "provisioning", "frontend checks kind === 'provisioning'");
    assert!(!vm.title.is_empty(), "frontend renders vm.title");
    assert!(!vm.expedition_label.is_empty(), "frontend renders vm.expeditionLabel");
    assert!(vm.party.len() > 0, "frontend iterates vm.party");

    // Validate party structure for frontend contract
    for hero in &vm.party {
        assert!(!hero.id.is_empty(), "frontend uses hero.id as key");
        assert!(!hero.name.is_empty(), "frontend displays hero.name");
        assert!(!hero.class_label.is_empty(), "frontend displays hero.classLabel");
        assert!(!hero.hp.is_empty(), "frontend displays hero.hp");
    }
}

/// Verifies the expedition setup view model is usable via the frontend contract boundary.
#[test]
fn expedition_setup_vm_usable_via_contract_boundary() {
    let campaign = make_provisioning_campaign(1000);
    let selected = vec!["h1".to_string()];

    use game_ddgc_headless::contracts::adapters::expedition_setup_from_data;
    let vm = expedition_setup_from_data(&campaign, &selected, None, "Adequate", "100 Gold")
        .expect("expedition setup should succeed");

    // Contract boundary: frontend expects these fields
    assert_eq!(vm.kind, "expedition", "frontend checks kind === 'expedition'");
    assert!(!vm.title.is_empty(), "frontend renders vm.title");
    assert!(!vm.expedition_name.is_empty(), "frontend renders vm.expeditionName");
    assert!(!vm.difficulty.is_empty(), "frontend renders vm.difficulty");
    assert!(!vm.estimated_duration.is_empty(), "frontend renders vm.estimatedDuration");

    // Objectives and warnings should be presentable
    assert!(!vm.supply_level.is_empty(), "frontend renders vm.supplyLevel");
    assert!(!vm.provision_cost.is_empty(), "frontend renders vm.provisionCost");
    assert!(vm.is_launchable || vm.party_size == 0, "frontend checks vm.isLaunchable");
}

/// Verifies expedition launch result is usable via the frontend contract boundary.
#[test]
fn expedition_launch_result_usable_via_contract_boundary() {
    let campaign = make_provisioning_campaign(1000);

    use game_ddgc_headless::contracts::adapters::expedition_launch;
    let result = expedition_launch(&campaign, &ExpeditionLaunchRequest::new(vec!["h1".to_string()]));

    // Contract boundary: frontend expects these fields
    assert_eq!(result.success, true, "frontend checks result.success");
    assert!(result.selected_heroes.len() > 0, "frontend reads result.selectedHeroes");
    assert_eq!(result.next_state, "dungeon", "frontend transitions using result.nextState");
}

/// Verifies multiple provisioning cycles can occur without state corruption.
#[test]
fn multiple_provisioning_cycles_via_contracts() {
    let campaign = make_provisioning_campaign(2000);

    // Cycle 1: Provision with 2 heroes
    use game_ddgc_headless::contracts::adapters::{provisioning_from_campaign, expedition_launch};
    let selected1 = vec!["h1".to_string(), "h2".to_string()];
    let prov1 = provisioning_from_campaign(&campaign, &selected1, "Expedition 1", "First expedition").unwrap();
    assert!(prov1.is_ready_to_launch);

    let launch1 = expedition_launch(&campaign, &ExpeditionLaunchRequest::new(selected1));
    assert!(launch1.success);
    assert_eq!(launch1.gold_cost, 100);

    // Cycle 2: Provision with 3 different heroes
    let selected2 = vec!["h2".to_string(), "h3".to_string(), "h4".to_string()];
    let prov2 = provisioning_from_campaign(&campaign, &selected2, "Expedition 2", "Second expedition").unwrap();
    assert!(prov2.is_ready_to_launch);

    let launch2 = expedition_launch(&campaign, &ExpeditionLaunchRequest::new(selected2));
    assert!(launch2.success);
    assert_eq!(launch2.gold_cost, 150);

    // Cycle 3: Single hero
    let selected3 = vec!["h4".to_string()];
    let prov3 = provisioning_from_campaign(&campaign, &selected3, "Expedition 3", "Third expedition").unwrap();
    assert!(prov3.is_ready_to_launch);

    let launch3 = expedition_launch(&campaign, &ExpeditionLaunchRequest::new(selected3));
    assert!(launch3.success);
    assert_eq!(launch3.gold_cost, 50);
}

// ── Part 9: ViewModelError contract tests ─────────────────────────────────────

/// Verifies ViewModelError::description produces meaningful messages.
#[test]
fn vm_error_description_produces_messages() {
    let errors = vec![
        ViewModelError::UnsupportedState {
            state_type: "provisioning".to_string(),
            detail: "campaign has no roster".to_string(),
        },
        ViewModelError::PartialMapping {
            state_type: "expedition".to_string(),
            missing_fields: vec!["quest_id".to_string(), "difficulty".to_string()],
        },
        ViewModelError::MissingRequiredField {
            field: "hero_id".to_string(),
            context: "hero not found in roster".to_string(),
        },
        ViewModelError::IncompatibleSchema {
            expected: "1".to_string(),
            found: "2".to_string(),
        },
    ];

    for error in &errors {
        let desc = error.description();
        assert!(!desc.is_empty(), "error {:?} produced empty description", error);
        assert!(desc.len() > 10, "error {:?} produced too short message: {}", error, desc);
    }
}

/// Verifies ViewModelError Display impl is not empty.
#[test]
fn vm_error_display_is_not_empty() {
    let error = ViewModelError::MissingRequiredField {
        field: "test_field".to_string(),
        context: "test context".to_string(),
    };
    let display = format!("{}", error);
    assert!(!display.is_empty());
    assert_eq!(display, error.description());
}

/// Verifies ViewModelError round-trips through JSON.
#[test]
fn vm_error_json_roundtrip() {
    let original = ViewModelError::MissingRequiredField {
        field: "hero_id".to_string(),
        context: "hero not found in roster".to_string(),
    };

    let json = serde_json::to_string(&original).expect("serialize should succeed");
    let restored: ViewModelError =
        serde_json::from_str(&json).expect("deserialize should succeed");

    match restored {
        ViewModelError::MissingRequiredField { field, context } => {
            assert_eq!(field, "hero_id");
            assert_eq!(context, "hero not found in roster");
        }
        other => panic!("expected MissingRequiredField, got: {:?}", other),
    }
}
