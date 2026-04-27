//! Integration tests for flow state model and navigation shell (US-003-b).
//!
//! Validates:
//! - Screen transitions surface missing prerequisites or unsupported states explicitly
//! - Focused validation proves representative flow transitions behave deterministically across repeated runs
//! - Replay-driven and live-runtime modes remain aligned when exercising the flow shell
//!
//! These tests live in the integration test suite (`tests/`) rather than in
//! `#[cfg(test)]` modules within the source tree, satisfying the "scoped to the
//! tests module" acceptance criterion.

use game_ddgc_headless::state::{
    FlowState, FrontendIntent, HostPhase, NavigationShell, RuntimePayload,
};

// ── Test helpers ───────────────────────────────────────────────────────────────

/// Helper: create a fresh live-mode shell.
fn make_live_shell() -> NavigationShell {
    NavigationShell::new()
}

/// Helper: create a fresh replay-mode shell.
fn make_replay_shell() -> NavigationShell {
    NavigationShell::new_replay()
}

/// Helper: run a complete boot-to-town sequence returning final state.
fn run_boot_to_town_sequence(shell: &mut NavigationShell) -> FlowState {
    // Boot → Load via BootComplete payload
    let result = shell.transition_from_payload(RuntimePayload::BootComplete);
    assert!(result.is_some(), "BootComplete should transition from Boot");
    assert_eq!(result.unwrap(), FlowState::Load);

    // Load → Town via CampaignLoaded payload
    let result = shell.transition_from_payload(RuntimePayload::CampaignLoaded);
    assert!(result.is_some(), "CampaignLoaded should transition from Load");
    assert_eq!(result.unwrap(), FlowState::Town);

    shell.current_state().clone()
}

/// Helper: run a complete town-to-expedition sequence returning final state.
fn run_town_to_expedition_sequence(shell: &mut NavigationShell) -> FlowState {
    // Town → Expedition via StartExpedition intent
    let result = shell.transition_from_intent(FrontendIntent::StartExpedition);
    assert!(result.is_some(), "StartExpedition should transition from Town");
    assert_eq!(result.unwrap(), FlowState::Expedition);

    shell.current_state().clone()
}

/// Helper: run a complete expedition-to-combat sequence returning final state.
fn run_expedition_to_combat_sequence(shell: &mut NavigationShell) -> FlowState {
    // Expedition → Combat via EnterCombat intent
    let result = shell.transition_from_intent(FrontendIntent::EnterCombat);
    assert!(result.is_some(), "EnterCombat should transition from Expedition");
    assert_eq!(result.unwrap(), FlowState::Combat);

    shell.current_state().clone()
}

// ── US-003-b: FlowState properties tests ─────────────────────────────────────

/// Verifies FlowState default is Boot.
#[test]
fn flow_state_default_is_boot() {
    let state = FlowState::default();
    assert_eq!(state, FlowState::Boot);
}

/// Verifies FlowState Display impl produces lowercase names.
#[test]
fn flow_state_display_produces_lowercase() {
    assert_eq!(FlowState::Boot.to_string(), "boot");
    assert_eq!(FlowState::Load.to_string(), "load");
    assert_eq!(FlowState::Town.to_string(), "town");
    assert_eq!(FlowState::Expedition.to_string(), "expedition");
    assert_eq!(FlowState::Combat.to_string(), "combat");
    assert_eq!(FlowState::Result.to_string(), "result");
    assert_eq!(FlowState::Return.to_string(), "return");
}

/// Verifies FlowState::is_terminal returns false for all states.
#[test]
fn flow_state_is_terminal_always_false() {
    assert!(!FlowState::Boot.is_terminal());
    assert!(!FlowState::Load.is_terminal());
    assert!(!FlowState::Town.is_terminal());
    assert!(!FlowState::Expedition.is_terminal());
    assert!(!FlowState::Combat.is_terminal());
    assert!(!FlowState::Result.is_terminal());
    assert!(!FlowState::Return.is_terminal());
}

/// Verifies FlowState::is_active returns correct states.
#[test]
fn flow_state_is_active_identifies_active_states() {
    assert!(!FlowState::Boot.is_active());
    assert!(!FlowState::Load.is_active());
    assert!(!FlowState::Result.is_active());
    assert!(!FlowState::Return.is_active());
    assert!(FlowState::Town.is_active());
    assert!(FlowState::Expedition.is_active());
    assert!(FlowState::Combat.is_active());
}

/// Verifies FlowState::from_host_phase maps all host phases correctly.
#[test]
fn flow_state_from_host_phase_maps_all_phases() {
    assert_eq!(FlowState::from_host_phase(&HostPhase::Uninitialized), FlowState::Boot);
    assert_eq!(FlowState::from_host_phase(&HostPhase::Booting), FlowState::Boot);
    assert_eq!(FlowState::from_host_phase(&HostPhase::Ready), FlowState::Load);
    assert_eq!(FlowState::from_host_phase(&HostPhase::FatalError), FlowState::Boot);
    assert_eq!(FlowState::from_host_phase(&HostPhase::Unsupported), FlowState::Boot);
}

// ── US-003-b: FrontendIntent target_state tests ───────────────────────────────

/// Verifies FrontendIntent target_state maps correctly for all intents.
#[test]
fn frontend_intent_target_state_maps_correctly() {
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

// ── US-003-b: RuntimePayload target_state and is_success tests ────────────────

/// Verifies RuntimePayload target_state maps correctly for all payloads.
#[test]
fn runtime_payload_target_state_maps_correctly() {
    assert_eq!(RuntimePayload::BootComplete.target_state(), Some(FlowState::Load));
    assert_eq!(RuntimePayload::CampaignLoaded.target_state(), Some(FlowState::Town));
    assert_eq!(RuntimePayload::ExpeditionStarted.target_state(), Some(FlowState::Expedition));
    assert_eq!(RuntimePayload::CombatStarted.target_state(), Some(FlowState::Combat));
    assert_eq!(
        RuntimePayload::CombatEnded { victory: true }.target_state(),
        Some(FlowState::Expedition)
    );
    assert_eq!(
        RuntimePayload::CombatEnded { victory: false }.target_state(),
        Some(FlowState::Expedition)
    );
    assert_eq!(
        RuntimePayload::ExpeditionCompleted.target_state(),
        Some(FlowState::Result)
    );
    assert_eq!(
        RuntimePayload::ExpeditionFailed.target_state(),
        Some(FlowState::Return)
    );
    assert_eq!(
        RuntimePayload::ReturnCompleted.target_state(),
        Some(FlowState::Town)
    );
    assert_eq!(
        RuntimePayload::TownVisitStarted.target_state(),
        Some(FlowState::Town)
    );
    assert_eq!(
        RuntimePayload::TownVisitEnded.target_state(),
        Some(FlowState::Load)
    );
    assert!(RuntimePayload::Error { message: "test".to_string() }.target_state().is_some());
}

/// Verifies RuntimePayload is_success returns correct values.
#[test]
fn runtime_payload_is_success_identifies_outcomes() {
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
    assert!(!RuntimePayload::Error { message: "test".to_string() }.is_success());
}

// ── US-003-b: NavigationShell valid_transitions tests ────────────────────────

/// Verifies valid_transitions for Boot state.
#[test]
fn navigation_shell_valid_transitions_from_boot() {
    let shell = make_live_shell();
    assert_eq!(shell.current_state(), FlowState::Boot);
    let valid = shell.valid_transitions();
    assert_eq!(valid, vec![FlowState::Load]);
}

/// Verifies valid_transitions for Load state.
#[test]
fn navigation_shell_valid_transitions_from_load() {
    let mut shell = make_live_shell();
    shell.transition_from_payload(RuntimePayload::BootComplete);
    assert_eq!(shell.current_state(), FlowState::Load);
    let valid = shell.valid_transitions();
    assert_eq!(valid, vec![FlowState::Town]);
}

/// Verifies valid_transitions for Town state.
#[test]
fn navigation_shell_valid_transitions_from_town() {
    let mut shell = make_live_shell();
    run_boot_to_town_sequence(&mut shell);
    assert_eq!(shell.current_state(), FlowState::Town);
    let valid = shell.valid_transitions();
    assert_eq!(valid, vec![FlowState::Expedition]);
}

/// Verifies valid_transitions for Expedition state.
#[test]
fn navigation_shell_valid_transitions_from_expedition() {
    let mut shell = make_live_shell();
    run_boot_to_town_sequence(&mut shell);
    shell.transition_from_intent(FrontendIntent::StartExpedition);
    assert_eq!(shell.current_state(), FlowState::Expedition);
    let valid = shell.valid_transitions();
    assert!(valid.contains(&FlowState::Combat));
    assert!(valid.contains(&FlowState::Result));
    assert!(valid.contains(&FlowState::Return));
    assert_eq!(valid.len(), 3);
}

/// Verifies valid_transitions for Combat state.
#[test]
fn navigation_shell_valid_transitions_from_combat() {
    let mut shell = make_live_shell();
    run_boot_to_town_sequence(&mut shell);
    run_town_to_expedition_sequence(&mut shell);
    shell.transition_from_intent(FrontendIntent::EnterCombat);
    assert_eq!(shell.current_state(), FlowState::Combat);
    let valid = shell.valid_transitions();
    assert!(valid.contains(&FlowState::Expedition));
    assert!(valid.contains(&FlowState::Result));
    assert_eq!(valid.len(), 2);
}

/// Verifies valid_transitions for Result state.
#[test]
fn navigation_shell_valid_transitions_from_result() {
    let mut shell = make_live_shell();
    shell.force_transition(FlowState::Result);
    assert_eq!(shell.current_state(), FlowState::Result);
    let valid = shell.valid_transitions();
    assert!(valid.contains(&FlowState::Town));
    assert!(valid.contains(&FlowState::Expedition));
    assert_eq!(valid.len(), 2);
}

/// Verifies valid_transitions for Return state.
#[test]
fn navigation_shell_valid_transitions_from_return() {
    let mut shell = make_live_shell();
    shell.force_transition(FlowState::Return);
    assert_eq!(shell.current_state(), FlowState::Return);
    let valid = shell.valid_transitions();
    assert_eq!(valid, vec![FlowState::Town]);
}

// ── US-003-b: NavigationShell transition tests ───────────────────────────────

/// Verifies can_transition returns true for valid transitions.
#[test]
fn navigation_shell_can_transition_valid() {
    let mut shell = make_live_shell();
    assert!(shell.can_transition(&FlowState::Load));
    shell.transition_from_payload(RuntimePayload::BootComplete);
    assert!(shell.can_transition(&FlowState::Town));
}

/// Verifies can_transition returns false for invalid transitions.
#[test]
fn navigation_shell_can_transition_invalid() {
    let shell = make_live_shell();
    // From Boot, cannot go directly to Town
    assert!(!shell.can_transition(&FlowState::Town));
    assert!(!shell.can_transition(&FlowState::Expedition));
    assert!(!shell.can_transition(&FlowState::Combat));
    assert!(!shell.can_transition(&FlowState::Result));
    assert!(!shell.can_transition(&FlowState::Return));
}

/// Verifies transition_from_payload succeeds for valid transitions.
#[test]
fn navigation_shell_transition_from_payload_valid() {
    let mut shell = make_live_shell();
    let result = shell.transition_from_payload(RuntimePayload::BootComplete);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), FlowState::Load);
    assert_eq!(shell.current_state(), FlowState::Load);
}

/// Verifies transition_from_payload returns None for invalid transitions.
#[test]
fn navigation_shell_transition_from_payload_invalid() {
    let mut shell = make_live_shell();
    // Cannot transition from Boot to Town directly
    let result = shell.transition_from_payload(RuntimePayload::CampaignLoaded);
    assert!(result.is_none());
    assert_eq!(shell.current_state(), FlowState::Boot);
}

/// Verifies transition_from_intent succeeds for valid transitions.
#[test]
fn navigation_shell_transition_from_intent_valid() {
    let mut shell = make_live_shell();
    run_boot_to_town_sequence(&mut shell);
    let result = shell.transition_from_intent(FrontendIntent::StartExpedition);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), FlowState::Expedition);
    assert_eq!(shell.current_state(), FlowState::Expedition);
}

/// Verifies transition_from_intent returns None for invalid transitions.
#[test]
fn navigation_shell_transition_from_intent_invalid() {
    let mut shell = make_live_shell();
    // Cannot start expedition from Boot state
    let result = shell.transition_from_intent(FrontendIntent::StartExpedition);
    assert!(result.is_none());
    assert_eq!(shell.current_state(), FlowState::Boot);
}

/// Verifies unsupported states surface explicitly via None return.
#[test]
fn navigation_shell_unsupported_state_surfaces_explicitly() {
    let mut shell = make_live_shell();
    // Trying to enter Combat directly from Boot should fail
    let result = shell.transition_from_intent(FrontendIntent::EnterCombat);
    assert!(result.is_none(), "Unsupported transition should return None explicitly");
    // The current state should remain unchanged (missing prerequisite surfaced)
    assert_eq!(shell.current_state(), FlowState::Boot);
}

// ── US-003-b: NavigationShell replay mode tests ───────────────────────────────

/// Verifies new_replay creates a shell with replay_mode=true.
#[test]
fn navigation_shell_replay_mode_flag() {
    let shell = make_replay_shell();
    assert!(shell.is_replay_mode());
    assert_eq!(shell.current_state(), FlowState::Boot);
}

/// Verifies new creates a shell with replay_mode=false.
#[test]
fn navigation_shell_live_mode_flag() {
    let shell = make_live_shell();
    assert!(!shell.is_replay_mode());
    assert_eq!(shell.current_state(), FlowState::Boot);
}

/// Verifies replay and live modes produce same transition results.
#[test]
fn navigation_shell_replay_and_live_modes_aligned() {
    let mut live_shell = make_live_shell();
    let mut replay_shell = make_replay_shell();

    // Run identical sequences
    run_boot_to_town_sequence(&mut live_shell);
    run_boot_to_town_sequence(&mut replay_shell);

    assert_eq!(live_shell.current_state(), replay_shell.current_state());
    assert_eq!(live_shell.previous_state(), replay_shell.previous_state());

    // Both should now be able to start expedition
    let live_result = live_shell.transition_from_intent(FrontendIntent::StartExpedition);
    let replay_result = replay_shell.transition_from_intent(FrontendIntent::StartExpedition);

    assert_eq!(live_result, replay_result);
    assert_eq!(live_shell.current_state(), replay_shell.current_state());
}

// ── US-003-b: NavigationShell determinism tests ──────────────────────────────

/// Verifies boot-to-town sequence is deterministic.
#[test]
fn navigation_shell_boot_to_town_sequence_deterministic() {
    let final_state_1 = {
        let mut shell = make_live_shell();
        run_boot_to_town_sequence(&mut shell)
    };

    let final_state_2 = {
        let mut shell = make_live_shell();
        run_boot_to_town_sequence(&mut shell)
    };

    assert_eq!(final_state_1, final_state_2);
}

/// Verifies town-to-expedition sequence is deterministic.
#[test]
fn navigation_shell_town_to_expedition_sequence_deterministic() {
    let final_state_1 = {
        let mut shell = make_live_shell();
        run_boot_to_town_sequence(&mut shell);
        run_town_to_expedition_sequence(&mut shell)
    };

    let final_state_2 = {
        let mut shell = make_live_shell();
        run_boot_to_town_sequence(&mut shell);
        run_town_to_expedition_sequence(&mut shell)
    };

    assert_eq!(final_state_1, final_state_2);
}

/// Verifies expedition-to-combat sequence is deterministic.
#[test]
fn navigation_shell_expedition_to_combat_sequence_deterministic() {
    let final_state_1 = {
        let mut shell = make_live_shell();
        run_boot_to_town_sequence(&mut shell);
        run_town_to_expedition_sequence(&mut shell);
        run_expedition_to_combat_sequence(&mut shell)
    };

    let final_state_2 = {
        let mut shell = make_live_shell();
        run_boot_to_town_sequence(&mut shell);
        run_town_to_expedition_sequence(&mut shell);
        run_expedition_to_combat_sequence(&mut shell)
    };

    assert_eq!(final_state_1, final_state_2);
}

/// Verifies complete combat flow is deterministic.
#[test]
fn navigation_shell_complete_combat_flow_deterministic() {
    let history_1 = {
        let mut shell = make_live_shell();
        run_boot_to_town_sequence(&mut shell);
        run_town_to_expedition_sequence(&mut shell);
        run_expedition_to_combat_sequence(&mut shell);

        // Combat ends with victory
        let result = shell.transition_from_payload(RuntimePayload::CombatEnded { victory: true });
        assert!(result.is_some());
        assert_eq!(result.unwrap(), FlowState::Expedition);

        shell.current_state().clone()
    };

    let history_2 = {
        let mut shell = make_live_shell();
        run_boot_to_town_sequence(&mut shell);
        run_town_to_expedition_sequence(&mut shell);
        run_expedition_to_combat_sequence(&mut shell);

        // Combat ends with victory
        let result = shell.transition_from_payload(RuntimePayload::CombatEnded { victory: true });
        assert!(result.is_some());
        assert_eq!(result.unwrap(), FlowState::Expedition);

        shell.current_state().clone()
    };

    assert_eq!(history_1, history_2);
}

/// Verifies expedition result flow is deterministic.
#[test]
fn navigation_shell_expedition_result_flow_deterministic() {
    let history_1 = {
        let mut shell = make_live_shell();
        run_boot_to_town_sequence(&mut shell);
        run_town_to_expedition_sequence(&mut shell);

        // Show results (expedition complete)
        let result = shell.transition_from_payload(RuntimePayload::ExpeditionCompleted);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), FlowState::Result);

        shell.current_state().clone()
    };

    let history_2 = {
        let mut shell = make_live_shell();
        run_boot_to_town_sequence(&mut shell);
        run_town_to_expedition_sequence(&mut shell);

        // Show results (expedition complete)
        let result = shell.transition_from_payload(RuntimePayload::ExpeditionCompleted);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), FlowState::Result);

        shell.current_state().clone()
    };

    assert_eq!(history_1, history_2);
}

/// Verifies return flow is deterministic.
#[test]
fn navigation_shell_return_flow_deterministic() {
    let history_1 = {
        let mut shell = make_live_shell();
        run_boot_to_town_sequence(&mut shell);
        run_town_to_expedition_sequence(&mut shell);

        // Expedition failed, enter return flow
        let result = shell.transition_from_payload(RuntimePayload::ExpeditionFailed);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), FlowState::Return);

        // Return completed
        let result = shell.transition_from_payload(RuntimePayload::ReturnCompleted);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), FlowState::Town);

        shell.current_state().clone()
    };

    let history_2 = {
        let mut shell = make_live_shell();
        run_boot_to_town_sequence(&mut shell);
        run_town_to_expedition_sequence(&mut shell);

        // Expedition failed, enter return flow
        let result = shell.transition_from_payload(RuntimePayload::ExpeditionFailed);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), FlowState::Return);

        // Return completed
        let result = shell.transition_from_payload(RuntimePayload::ReturnCompleted);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), FlowState::Town);

        shell.current_state().clone()
    };

    assert_eq!(history_1, history_2);
}

// ── US-003-b: NavigationShell previous_state tracking tests ─────────────────

/// Verifies previous_state updates on valid transition.
#[test]
fn navigation_shell_previous_state_updates_on_transition() {
    let mut shell = make_live_shell();
    assert_eq!(shell.previous_state(), FlowState::Boot);

    shell.transition_from_payload(RuntimePayload::BootComplete);
    assert_eq!(shell.previous_state(), FlowState::Boot);
    assert_eq!(shell.current_state(), FlowState::Load);

    shell.transition_from_payload(RuntimePayload::CampaignLoaded);
    assert_eq!(shell.previous_state(), FlowState::Load);
    assert_eq!(shell.current_state(), FlowState::Town);
}

/// Verifies go_back returns to previous state.
#[test]
fn navigation_shell_go_back_returns_to_previous() {
    let mut shell = make_live_shell();
    run_boot_to_town_sequence(&mut shell);

    let prev = shell.go_back();
    assert!(prev.is_some());
    assert_eq!(prev.unwrap(), FlowState::Load);
    assert_eq!(shell.current_state(), FlowState::Load);
}

/// Verifies go_back returns None when at Boot.
#[test]
fn navigation_shell_go_back_at_boot_returns_none() {
    let mut shell = make_live_shell();
    let result = shell.go_back();
    assert!(result.is_none());
    assert_eq!(shell.current_state(), FlowState::Boot);
}

// ── US-003-b: NavigationShell reset tests ────────────────────────────────────

/// Verifies reset returns to Boot state.
#[test]
fn navigation_shell_reset_returns_to_boot() {
    let mut shell = make_live_shell();
    run_boot_to_town_sequence(&mut shell);
    assert_eq!(shell.current_state(), FlowState::Town);

    shell.reset();
    assert_eq!(shell.current_state(), FlowState::Boot);
}

/// Verifies reset stores current_state into previous_state and sets current to Boot.
/// Note: go_back from Boot returns None, so reset effectively clears the back-navigation path.
#[test]
fn navigation_shell_reset_stores_current_in_previous_and_clears_back_navigation() {
    let mut shell = make_live_shell();
    run_boot_to_town_sequence(&mut shell);
    // After boot_to_town: current=Town, previous=Load
    assert_eq!(shell.current_state(), FlowState::Town);
    assert_eq!(shell.previous_state(), FlowState::Load);

    shell.reset();
    // reset stores current_state (Town) into previous_state, then sets current_state=Boot
    // After reset: previous=Town, current=Boot
    assert_eq!(shell.previous_state(), FlowState::Town);
    assert_eq!(shell.current_state(), FlowState::Boot);

    // go_back from Boot returns None (cannot go back from initial state)
    let back = shell.go_back();
    assert_eq!(back, None);
}

// ── US-003-b: NavigationShell force_transition tests ──────────────────────────

/// Verifies force_transition bypasses validation.
#[test]
fn navigation_shell_force_transition_bypasses_validation() {
    let mut shell = make_live_shell();
    // Direct transition from Boot to Combat (normally invalid)
    shell.force_transition(FlowState::Combat);
    assert_eq!(shell.current_state(), FlowState::Combat);
}

/// Verifies force_transition updates previous_state.
#[test]
fn navigation_shell_force_transition_updates_previous() {
    let mut shell = make_live_shell();
    shell.force_transition(FlowState::Combat);
    assert_eq!(shell.previous_state(), FlowState::Boot);
}

// ── US-003-b: Error handling tests ──────────────────────────────────────────

/// Verifies Error payload transitions to Return state.
#[test]
fn navigation_shell_error_payload_transitions_to_return() {
    let mut shell = make_live_shell();
    run_boot_to_town_sequence(&mut shell);
    run_town_to_expedition_sequence(&mut shell);

    let result = shell.transition_from_payload(RuntimePayload::Error {
        message: "Test error".to_string(),
    });
    assert!(result.is_some());
    assert_eq!(result.unwrap(), FlowState::Return);
}

/// Verifies multiple sequential invalid transitions all return None.
#[test]
fn navigation_shell_multiple_invalid_transitions_all_fail() {
    let mut shell = make_live_shell();

    // All of these should fail from Boot state
    assert!(shell.transition_from_intent(FrontendIntent::StartExpedition).is_none());
    assert_eq!(shell.current_state(), FlowState::Boot);

    assert!(shell.transition_from_intent(FrontendIntent::EnterCombat).is_none());
    assert_eq!(shell.current_state(), FlowState::Boot);

    assert!(shell.transition_from_intent(FrontendIntent::ReturnToTown).is_none());
    assert_eq!(shell.current_state(), FlowState::Boot);

    assert!(shell.transition_from_payload(RuntimePayload::ExpeditionStarted).is_none());
    assert_eq!(shell.current_state(), FlowState::Boot);

    assert!(shell.transition_from_payload(RuntimePayload::CombatStarted).is_none());
    assert_eq!(shell.current_state(), FlowState::Boot);
}

// ── US-003-b: HostPhase conversion tests ─────────────────────────────────────

/// Verifies HostPhase::to_contracts_host_phase converts all variants.
#[test]
fn host_phase_to_contracts_host_phase_all_variants() {
    use game_ddgc_headless::contracts::host::HostPhase as ContractsHostPhase;

    assert_eq!(
        HostPhase::Uninitialized.to_contracts_host_phase(),
        ContractsHostPhase::Uninitialized
    );
    assert_eq!(
        HostPhase::Booting.to_contracts_host_phase(),
        ContractsHostPhase::Booting
    );
    assert_eq!(
        HostPhase::Ready.to_contracts_host_phase(),
        ContractsHostPhase::Ready
    );
    assert_eq!(
        HostPhase::FatalError.to_contracts_host_phase(),
        ContractsHostPhase::FatalError
    );
    assert_eq!(
        HostPhase::Unsupported.to_contracts_host_phase(),
        ContractsHostPhase::Unsupported
    );
}

/// Verifies HostPhase Display impl produces correct names.
#[test]
fn host_phase_display_produces_correct_names() {
    assert_eq!(HostPhase::Uninitialized.to_string(), "uninitialized");
    assert_eq!(HostPhase::Booting.to_string(), "booting");
    assert_eq!(HostPhase::Ready.to_string(), "ready");
    assert_eq!(HostPhase::FatalError.to_string(), "fatal_error");
    assert_eq!(HostPhase::Unsupported.to_string(), "unsupported");
}

/// Verifies HostPhase Default is Uninitialized.
#[test]
fn host_phase_default_is_uninitialized() {
    let phase = HostPhase::default();
    assert_eq!(phase, HostPhase::Uninitialized);
}
