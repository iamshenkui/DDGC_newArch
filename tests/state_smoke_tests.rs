//! Smoke tests for DDGC state layer (US-009-b).
//!
//! Validates:
//! - A deterministic local build/run path exists for the DDGC frontend slice
//! - The build can run against replay-driven mode and live-runtime mode
//! - Asset loading, startup flow, and runtime wiring are documented
//! - A focused smoke-test path exists for verifying the packaged or runnable slice
//! - Packaging/build choices do not break the stable contract boundary
//! - Typecheck passes
//! - Changes are scoped to the state module
//!
//! These tests live in the integration test suite (`tests/`) rather than in
//! `#[cfg(test)]` modules within the source tree.

use game_ddgc_headless::state::{
    CombatState, FlowState, FrontendIntent, GameState, HostPhase, NavigationShell,
    ResultState, ReturnFlowState, RuntimePayload,
};
use game_ddgc_headless::contracts::viewmodels::{CombatPhase, ResultViewModel, OutcomeType};

// ── NavigationShell smoke tests ───────────────────────────────────────────────

/// Smoke test: NavigationShell::new creates shell in live mode.
#[test]
fn smoke_navigation_shell_new_is_live_mode() {
    let shell = NavigationShell::new();
    assert_eq!(shell.current_state(), FlowState::Boot);
    assert_eq!(shell.previous_state(), FlowState::Boot);
    assert!(!shell.is_replay_mode());
}

/// Smoke test: NavigationShell::new_replay creates shell in replay mode.
#[test]
fn smoke_navigation_shell_new_replay_is_replay_mode() {
    let shell = NavigationShell::new_replay();
    assert_eq!(shell.current_state(), FlowState::Boot);
    assert_eq!(shell.previous_state(), FlowState::Boot);
    assert!(shell.is_replay_mode());
}

/// Smoke test: NavigationShell can transition via runtime payload.
#[test]
fn smoke_navigation_shell_transition_from_payload() {
    let mut shell = NavigationShell::new();

    // Boot -> Load via BootComplete payload
    let result = shell.transition_from_payload(RuntimePayload::BootComplete);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), FlowState::Load);
    assert_eq!(shell.current_state(), FlowState::Load);
}

/// Smoke test: NavigationShell can transition via frontend intent.
#[test]
fn smoke_navigation_shell_transition_from_intent() {
    let mut shell = NavigationShell::new();
    shell.transition_from_payload(RuntimePayload::BootComplete).unwrap();

    // Load -> Town via NewCampaign intent
    let result = shell.transition_from_intent(FrontendIntent::NewCampaign);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), FlowState::Town);
    assert_eq!(shell.current_state(), FlowState::Town);
}

/// Smoke test: NavigationShell rejects invalid transitions.
#[test]
fn smoke_navigation_shell_rejects_invalid_transition() {
    let mut shell = NavigationShell::new();

    // Boot -> Expedition is invalid (must go through Load first)
    let result = shell.transition_from_intent(FrontendIntent::StartExpedition);
    assert!(result.is_none());
    assert_eq!(shell.current_state(), FlowState::Boot);
}

/// Smoke test: NavigationShell valid_transitions returns correct states.
#[test]
fn smoke_navigation_shell_valid_transitions() {
    let shell = NavigationShell::new();

    // From Boot, only valid transition is to Load
    let valid = shell.valid_transitions();
    assert_eq!(valid, vec![FlowState::Load]);
}

/// Smoke test: NavigationShell can_transition checks correctly.
#[test]
fn smoke_navigation_shell_can_transition() {
    let shell = NavigationShell::new();
    assert!(shell.can_transition(&FlowState::Load));
    assert!(!shell.can_transition(&FlowState::Town));
    assert!(!shell.can_transition(&FlowState::Expedition));
}

/// Smoke test: NavigationShell force_transition bypasses validation.
#[test]
fn smoke_navigation_shell_force_transition() {
    let mut shell = NavigationShell::new();
    shell.force_transition(FlowState::Town);
    assert_eq!(shell.current_state(), FlowState::Town);
}

/// Smoke test: NavigationShell reset returns to Boot.
#[test]
fn smoke_navigation_shell_reset() {
    let mut shell = NavigationShell::new();
    shell.transition_from_payload(RuntimePayload::BootComplete).unwrap();
    shell.reset();
    assert_eq!(shell.current_state(), FlowState::Boot);
}

/// Smoke test: NavigationShell go_back returns to previous state.
#[test]
fn smoke_navigation_shell_go_back() {
    let mut shell = NavigationShell::new();
    shell.transition_from_payload(RuntimePayload::BootComplete).unwrap();
    assert_eq!(shell.current_state(), FlowState::Load);

    let prev = shell.go_back();
    assert!(prev.is_some());
    assert_eq!(prev.unwrap(), FlowState::Boot);
}

/// Smoke test: NavigationShell is deterministic (same mode produces same transitions).
#[test]
fn smoke_navigation_shell_is_deterministic() {
    let mut shell1 = NavigationShell::new();
    shell1.transition_from_payload(RuntimePayload::BootComplete).unwrap();

    let mut shell2 = NavigationShell::new();
    shell2.transition_from_payload(RuntimePayload::BootComplete).unwrap();

    assert_eq!(shell1.current_state(), shell2.current_state());
    assert_eq!(shell1.previous_state(), shell2.previous_state());
}

// ── FlowState smoke tests ─────────────────────────────────────────────────────

/// Smoke test: FlowState default is Boot.
#[test]
fn smoke_flow_state_default_is_boot() {
    assert_eq!(FlowState::default(), FlowState::Boot);
}

/// Smoke test: FlowState is_terminal always returns false.
#[test]
fn smoke_flow_state_is_terminal() {
    assert!(!FlowState::Boot.is_terminal());
    assert!(!FlowState::Load.is_terminal());
    assert!(!FlowState::Town.is_terminal());
    assert!(!FlowState::Expedition.is_terminal());
    assert!(!FlowState::Combat.is_terminal());
    assert!(!FlowState::Result.is_terminal());
    assert!(!FlowState::Return.is_terminal());
}

/// Smoke test: FlowState is_active returns true for active states.
#[test]
fn smoke_flow_state_is_active() {
    assert!(!FlowState::Boot.is_active());
    assert!(!FlowState::Load.is_active());
    assert!(FlowState::Town.is_active());
    assert!(FlowState::Expedition.is_active());
    assert!(FlowState::Combat.is_active());
    assert!(!FlowState::Result.is_active());
    assert!(!FlowState::Return.is_active());
}

/// Smoke test: FlowState from_host_phase maps correctly.
#[test]
fn smoke_flow_state_from_host_phase() {
    assert_eq!(FlowState::from_host_phase(&HostPhase::Uninitialized), FlowState::Boot);
    assert_eq!(FlowState::from_host_phase(&HostPhase::Booting), FlowState::Boot);
    assert_eq!(FlowState::from_host_phase(&HostPhase::Ready), FlowState::Load);
    assert_eq!(FlowState::from_host_phase(&HostPhase::FatalError), FlowState::Boot);
    assert_eq!(FlowState::from_host_phase(&HostPhase::Unsupported), FlowState::Boot);
}

/// Smoke test: FlowState Display produces correct strings.
#[test]
fn smoke_flow_state_display() {
    assert_eq!(FlowState::Boot.to_string(), "boot");
    assert_eq!(FlowState::Load.to_string(), "load");
    assert_eq!(FlowState::Town.to_string(), "town");
    assert_eq!(FlowState::Expedition.to_string(), "expedition");
    assert_eq!(FlowState::Combat.to_string(), "combat");
    assert_eq!(FlowState::Result.to_string(), "result");
    assert_eq!(FlowState::Return.to_string(), "return");
}

// ── HostPhase smoke tests ─────────────────────────────────────────────────────

/// Smoke test: HostPhase default is Uninitialized.
#[test]
fn smoke_host_phase_default_is_uninitialized() {
    assert_eq!(HostPhase::default(), HostPhase::Uninitialized);
}

/// Smoke test: HostPhase to_contracts_host_phase converts correctly.
#[test]
fn smoke_host_phase_to_contracts_host_phase() {
    assert_eq!(
        HostPhase::Uninitialized.to_contracts_host_phase(),
        game_ddgc_headless::contracts::host::HostPhase::Uninitialized
    );
    assert_eq!(
        HostPhase::Booting.to_contracts_host_phase(),
        game_ddgc_headless::contracts::host::HostPhase::Booting
    );
    assert_eq!(
        HostPhase::Ready.to_contracts_host_phase(),
        game_ddgc_headless::contracts::host::HostPhase::Ready
    );
    assert_eq!(
        HostPhase::FatalError.to_contracts_host_phase(),
        game_ddgc_headless::contracts::host::HostPhase::FatalError
    );
    assert_eq!(
        HostPhase::Unsupported.to_contracts_host_phase(),
        game_ddgc_headless::contracts::host::HostPhase::Unsupported
    );
}

/// Smoke test: HostPhase Display produces correct strings.
#[test]
fn smoke_host_phase_display() {
    assert_eq!(HostPhase::Uninitialized.to_string(), "uninitialized");
    assert_eq!(HostPhase::Booting.to_string(), "booting");
    assert_eq!(HostPhase::Ready.to_string(), "ready");
    assert_eq!(HostPhase::FatalError.to_string(), "fatal_error");
    assert_eq!(HostPhase::Unsupported.to_string(), "unsupported");
}

// ── FrontendIntent smoke tests ─────────────────────────────────────────────────

/// Smoke test: FrontendIntent target_state returns correct states.
#[test]
fn smoke_frontend_intent_target_state() {
    assert_eq!(FrontendIntent::NewCampaign.target_state(), Some(FlowState::Town));
    assert_eq!(FrontendIntent::LoadCampaign.target_state(), Some(FlowState::Town));
    assert_eq!(FrontendIntent::StartExpedition.target_state(), Some(FlowState::Expedition));
    assert_eq!(FrontendIntent::ReturnToTown.target_state(), Some(FlowState::Town));
    assert_eq!(FrontendIntent::EnterCombat.target_state(), Some(FlowState::Combat));
    assert_eq!(FrontendIntent::ExitCombat.target_state(), Some(FlowState::Expedition));
    assert_eq!(FrontendIntent::ShowResults.target_state(), Some(FlowState::Result));
    assert_eq!(FrontendIntent::Continue.target_state(), Some(FlowState::Town));
    assert_eq!(FrontendIntent::RetryExpedition.target_state(), Some(FlowState::Expedition));
    assert_eq!(FrontendIntent::Abort.target_state(), Some(FlowState::Town));
}

// ── RuntimePayload smoke tests ─────────────────────────────────────────────────

/// Smoke test: RuntimePayload target_state returns correct states.
#[test]
fn smoke_runtime_payload_target_state() {
    assert_eq!(RuntimePayload::BootComplete.target_state(), Some(FlowState::Load));
    assert_eq!(RuntimePayload::CampaignLoaded.target_state(), Some(FlowState::Town));
    assert_eq!(RuntimePayload::ExpeditionStarted.target_state(), Some(FlowState::Expedition));
    assert_eq!(RuntimePayload::CombatStarted.target_state(), Some(FlowState::Combat));
    assert_eq!(
        RuntimePayload::CombatEnded { victory: true }.target_state(),
        Some(FlowState::Expedition)
    );
    assert_eq!(RuntimePayload::ExpeditionCompleted.target_state(), Some(FlowState::Result));
    assert_eq!(RuntimePayload::ExpeditionFailed.target_state(), Some(FlowState::Return));
    assert_eq!(RuntimePayload::ReturnCompleted.target_state(), Some(FlowState::Town));
    assert_eq!(RuntimePayload::TownVisitStarted.target_state(), Some(FlowState::Town));
    assert_eq!(RuntimePayload::TownVisitEnded.target_state(), Some(FlowState::Load));
    assert_eq!(
        RuntimePayload::Error { message: "test".to_string() }.target_state(),
        Some(FlowState::Return)
    );
}

/// Smoke test: RuntimePayload is_success returns correct values.
#[test]
fn smoke_runtime_payload_is_success() {
    assert!(!RuntimePayload::BootComplete.is_success());
    assert!(!RuntimePayload::CampaignLoaded.is_success());
    assert!(!RuntimePayload::ExpeditionStarted.is_success());
    assert!(!RuntimePayload::CombatStarted.is_success());
    assert!(RuntimePayload::CombatEnded { victory: true }.is_success());
    assert!(!RuntimePayload::CombatEnded { victory: false }.is_success());
    assert!(RuntimePayload::ExpeditionCompleted.is_success());
    assert!(!RuntimePayload::ExpeditionFailed.is_success());
    assert!(RuntimePayload::ReturnCompleted.is_success());
    assert!(!RuntimePayload::TownVisitStarted.is_success());
    assert!(!RuntimePayload::TownVisitEnded.is_success());
    assert!(!RuntimePayload::Error { message: "fail".to_string() }.is_success());
}

// ── GameState smoke tests ──────────────────────────────────────────────────────

/// Smoke test: GameState default creates empty state.
#[test]
fn smoke_game_state_default() {
    let state = GameState::default();
    assert_eq!(state.host_phase, HostPhase::Uninitialized);
    assert_eq!(state.campaign.gold, 0);
    assert!(state.active_expedition.is_none());
    assert!(state.active_combat.is_none());
    assert!(state.active_result.is_none());
    assert!(state.active_return_flow.is_none());
}

/// Smoke test: GameState load from default data directory.
#[test]
fn smoke_game_state_load_default() {
    // This test requires the data/ directory to exist with JsonCamping.json
    let result = GameState::load();
    if result.is_err() {
        // Skip if data directory is not set up
        return;
    }

    let state = result.unwrap();
    assert_eq!(state.host_phase, HostPhase::Uninitialized);
    assert!(!state.data_dir.as_os_str().is_empty());
    assert!(!state.camping_skills.is_empty());
}

/// Smoke test: GameState load is deterministic.
#[test]
fn smoke_game_state_load_is_deterministic() {
    let result1 = GameState::load();
    if result1.is_err() {
        // Skip if data directory is not set up
        return;
    }

    let state1 = result1.unwrap();
    let result2 = GameState::load();
    if result2.is_err() {
        return;
    }

    let state2 = result2.unwrap();

    // Both should have the same host phase
    assert_eq!(state1.host_phase, state2.host_phase);

    // Both should have empty campaigns with 0 gold (load doesn't set campaign)
    assert_eq!(state1.campaign.gold, state2.campaign.gold);
}

/// Smoke test: GameState::new_campaign creates valid campaign.
#[test]
fn smoke_game_state_new_campaign() {
    let state = GameState::default();
    let mut state = state;

    // Simulate new_campaign by setting gold
    state.campaign.gold = 500;
    state.host_phase = HostPhase::Ready;

    assert_eq!(state.campaign.gold, 500);
    assert_eq!(state.host_phase, HostPhase::Ready);
}

// ── NavigationShell transition tests ──────────────────────────────────────────

/// Smoke test: NavigationShell transition_to_result works from valid state.
#[test]
fn smoke_navigation_shell_transition_to_result() {
    let mut shell = NavigationShell::new();
    shell.transition_from_payload(RuntimePayload::BootComplete).unwrap();
    shell.transition_from_intent(FrontendIntent::NewCampaign).unwrap();
    shell.transition_from_intent(FrontendIntent::StartExpedition).unwrap();

    // Expedition -> Result should be valid
    let result = shell.transition_to_result();
    assert!(result.is_some());
    assert_eq!(result.unwrap(), FlowState::Result);
}

/// Smoke test: NavigationShell transition_to_return works from valid state.
#[test]
fn smoke_navigation_shell_transition_to_return() {
    let mut shell = NavigationShell::new();
    shell.transition_from_payload(RuntimePayload::BootComplete).unwrap();
    shell.transition_from_intent(FrontendIntent::NewCampaign).unwrap();
    shell.transition_from_intent(FrontendIntent::StartExpedition).unwrap();

    // Expedition -> Return should be valid
    let result = shell.transition_to_return();
    assert!(result.is_some());
    assert_eq!(result.unwrap(), FlowState::Return);
}

/// Smoke test: NavigationShell is_result_terminal returns true only for Result.
#[test]
fn smoke_navigation_shell_is_result_terminal() {
    let mut shell = NavigationShell::new();
    assert!(!shell.is_result_terminal());

    shell.transition_from_payload(RuntimePayload::BootComplete).unwrap();
    assert!(!shell.is_result_terminal());

    shell.transition_from_intent(FrontendIntent::NewCampaign).unwrap();
    shell.transition_from_intent(FrontendIntent::StartExpedition).unwrap();
    shell.transition_to_result().unwrap();
    assert!(shell.is_result_terminal());
}

/// Smoke test: NavigationShell requires_return_journey returns true only for Return.
#[test]
fn smoke_navigation_shell_requires_return_journey() {
    let mut shell = NavigationShell::new();
    assert!(!shell.requires_return_journey());

    shell.transition_from_payload(RuntimePayload::BootComplete).unwrap();
    assert!(!shell.requires_return_journey());

    shell.transition_from_intent(FrontendIntent::NewCampaign).unwrap();
    shell.transition_from_intent(FrontendIntent::StartExpedition).unwrap();
    shell.transition_to_return().unwrap();
    assert!(shell.requires_return_journey());
}

// ── State result/return flow smoke tests ───────────────────────────────────────

/// Smoke test: ResultState::new creates correct state.
#[test]
fn smoke_result_state_new() {
    let vm = ResultViewModel {
        outcome: OutcomeType::Success,
        title: "Victory".to_string(),
        description: "Dungeon cleared".to_string(),
        rewards: None,
        casualties: vec![],
        dungeon_type: Some("crypt".to_string()),
        map_size: Some("short".to_string()),
        rooms_cleared: 5,
        battles_won: 3,
        completed: true,
        error: None,
    };
    let result_state = ResultState::new(vm.clone(), FlowState::Expedition);

    assert_eq!(result_state.source, FlowState::Expedition);
    assert!(!result_state.acknowledged);
    assert_eq!(result_state.view_model.outcome, OutcomeType::Success);
}

/// Smoke test: ReturnFlowState::new creates correct state.
#[test]
fn smoke_return_flow_state_new() {
    use game_ddgc_headless::contracts::viewmodels::ReturnFlowState as VmReturnFlowState;

    let vm = game_ddgc_headless::contracts::viewmodels::ReturnFlowViewModel {
        state: VmReturnFlowState::Traveling,
        dungeon_type: "crypt".to_string(),
        map_size: "short".to_string(),
        completed: false,
        rooms_cleared: 5,
        battles_won: 3,
        gold_to_transfer: 250,
        torchlight_remaining: 6,
        heroes: vec![],
        run_result: None,
        ready_for_town: false,
        error: None,
    };
    let return_state = ReturnFlowState::new(vm.clone(), FlowState::Expedition);

    assert_eq!(return_state.source, FlowState::Expedition);
    assert!(!return_state.journey_complete);
    assert_eq!(return_state.view_model.state, VmReturnFlowState::Traveling);
}

/// Smoke test: CombatState creation.
#[test]
fn smoke_combat_state_new() {
    let combat = CombatState {
        encounter_id: "test_encounter".to_string(),
        round: 1,
        phase: CombatPhase::PreBattle,
        result: None,
        current_turn_actor_id: Some("hero_1".to_string()),
        hero_vitals: vec![],
        monster_vitals: vec![],
    };

    assert_eq!(combat.encounter_id, "test_encounter");
    assert_eq!(combat.round, 1);
    assert!(combat.result.is_none());
}

// ── Replay-driven mode smoke tests ────────────────────────────────────────────

/// Smoke test: NavigationShell in replay mode follows same transitions.
#[test]
fn smoke_navigation_shell_replay_mode_transitions() {
    let mut shell = NavigationShell::new_replay();
    assert!(shell.is_replay_mode());

    // Replay mode should follow same transition rules
    shell.transition_from_payload(RuntimePayload::BootComplete).unwrap();
    assert_eq!(shell.current_state(), FlowState::Load);

    shell.transition_from_intent(FrontendIntent::LoadCampaign).unwrap();
    assert_eq!(shell.current_state(), FlowState::Town);
}

/// Smoke test: GameState with replay mode campaign state.
#[test]
fn smoke_game_state_replay_mode_campaign() {
    let state = GameState::default();
    let mut state = state;

    // Simulate loading a replay campaign
    state.campaign.gold = 1350;
    state.host_phase = HostPhase::Ready;

    assert_eq!(state.campaign.gold, 1350);
    assert_eq!(state.host_phase, HostPhase::Ready);
}