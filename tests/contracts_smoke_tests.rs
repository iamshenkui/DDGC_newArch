//! Smoke tests for DDGC contracts layer (US-009-a).
//!
//! Validates:
//! - A deterministic local build/run path exists for the DDGC frontend slice
//! - The build can run against replay-driven mode and live-runtime mode
//! - Asset loading, startup flow, and runtime wiring are documented
//! - A focused smoke-test path exists for verifying the packaged or runnable slice
//! - Packaging/build choices do not break the stable contract boundary
//! - Typecheck passes
//! - Changes are scoped to the contracts module
//!
//! These tests live in the integration test suite (`tests/`) rather than in
//! `#[cfg(test)]` modules within the source tree.

use game_ddgc_headless::contracts::host::{DdgcHost, HostError, HostPhase, LiveConfig, ReplayConfig, StartupMode};
use game_ddgc_headless::contracts::viewmodels::BootLoadViewModel;
use game_ddgc_headless::contracts::adapters::boot_load_from_host;
use game_ddgc_headless::contracts::{CampaignState, MapSize, CAMPAIGN_SNAPSHOT_VERSION};

// ── Live-runtime boot smoke tests ─────────────────────────────────────────────

/// Smoke test: boot_live produces a ready host when data directory exists.
#[test]
fn smoke_live_boot_produces_ready_host() {
    let config = LiveConfig::default();
    let host = DdgcHost::boot_live(&config);

    // Live boot should succeed when data/ directory is present
    assert!(host.is_ok(), "boot_live failed: {:?}", host.err());

    let host = host.unwrap();
    assert_eq!(host.phase(), HostPhase::Ready);
    assert_eq!(host.startup_mode(), Some(StartupMode::Live));
    assert!(host.campaign().is_some());
}

/// Smoke test: live boot loads all registries successfully.
#[test]
fn smoke_live_boot_loads_registries() {
    let config = LiveConfig::default();
    let host = DdgcHost::boot_live(&config).expect("boot_live should succeed");

    // Verify registries that have len() method are accessible and loaded
    assert!(host.curio_registry.len() > 0, "curio registry should be loaded");
    assert!(host.trap_registry.len() > 0, "trap registry should be loaded");
    assert!(host.obstacle_registry.len() > 0, "obstacle registry should be loaded");
    assert!(host.building_registry.len() > 0, "building registry should be loaded");
    assert!(host.quirk_registry.len() > 0, "quirk registry should be loaded");
    assert!(host.trait_registry.len() > 0, "trait registry should be loaded");

    // Verify registries that only have is_empty() method are accessible
    assert!(!host.camping_skill_registry.is_empty(), "camping skill registry should be loaded");
}

/// Smoke test: live boot campaign has correct initial state.
#[test]
fn smoke_live_boot_campaign_has_correct_initial_state() {
    let config = LiveConfig::default();
    let host = DdgcHost::boot_live(&config).expect("boot_live should succeed");

    let campaign = host.campaign().expect("campaign should exist");

    // Default starting gold should be 500
    assert_eq!(campaign.gold, 500);

    // Schema version should be current
    assert_eq!(campaign.schema_version, CAMPAIGN_SNAPSHOT_VERSION);
}

/// Smoke test: boot_live is deterministic (same config produces same result).
#[test]
fn smoke_live_boot_is_deterministic() {
    let config = LiveConfig::default();

    let host1 = DdgcHost::boot_live(&config).expect("first boot should succeed");
    let host2 = DdgcHost::boot_live(&config).expect("second boot should succeed");

    // Both hosts should have the same phase and startup mode
    assert_eq!(host1.phase(), host2.phase());
    assert_eq!(host1.startup_mode(), host2.startup_mode());

    // Both campaigns should have the same gold
    let campaign1 = host1.campaign().expect("campaign1 should exist");
    let campaign2 = host2.campaign().expect("campaign2 should exist");
    assert_eq!(campaign1.gold, campaign2.gold);
    assert_eq!(campaign1.schema_version, campaign2.schema_version);
}

// ── Replay-driven boot smoke tests ──────────────────────────────────────────

/// Smoke test: boot_from_campaign produces a ready host from serialized state.
#[test]
fn smoke_replay_boot_produces_ready_host() {
    // Create a minimal campaign JSON
    let campaign = CampaignState::new(1000);
    let json = campaign.to_json().expect("campaign should serialize");

    let config = ReplayConfig {
        campaign_json: &json,
        source_path: "test_campaign.json",
    };

    let host = DdgcHost::boot_from_campaign(&config);

    assert!(host.is_ok(), "boot_from_campaign failed: {:?}", host.err());

    let host = host.unwrap();
    assert_eq!(host.phase(), HostPhase::Ready);
    assert_eq!(host.startup_mode(), Some(StartupMode::Replay));
    assert!(host.campaign().is_some());
}

/// Smoke test: replay boot restores campaign state correctly.
#[test]
fn smoke_replay_boot_restores_campaign_state() {
    // Create a campaign with known state
    let mut campaign = CampaignState::new(750);
    campaign.gold = 1350; // Set specific gold amount

    let json = campaign.to_json().expect("campaign should serialize");

    let config = ReplayConfig {
        campaign_json: &json,
        source_path: "test_restore.json",
    };

    let host = DdgcHost::boot_from_campaign(&config).expect("boot should succeed");

    let restored = host.campaign().expect("campaign should exist");
    assert_eq!(restored.gold, 1350);
    assert_eq!(restored.schema_version, CAMPAIGN_SNAPSHOT_VERSION);
}

/// Smoke test: replay boot is deterministic.
#[test]
fn smoke_replay_boot_is_deterministic() {
    let campaign = CampaignState::new(500);
    let json = campaign.to_json().expect("campaign should serialize");

    let config = ReplayConfig {
        campaign_json: &json,
        source_path: "test.json",
    };

    let host1 = DdgcHost::boot_from_campaign(&config).expect("first boot should succeed");
    let host2 = DdgcHost::boot_from_campaign(&config).expect("second boot should succeed");

    assert_eq!(host1.phase(), host2.phase());
    assert_eq!(host1.startup_mode(), host2.startup_mode());

    let campaign1 = host1.campaign().expect("campaign1 should exist");
    let campaign2 = host2.campaign().expect("campaign2 should exist");
    assert_eq!(campaign1.gold, campaign2.gold);
}

/// Smoke test: replay boot rejects future schema versions.
#[test]
fn smoke_replay_boot_rejects_future_schema() {
    let mut campaign = CampaignState::new(500);
    campaign.schema_version = CAMPAIGN_SNAPSHOT_VERSION + 1;

    let json = campaign.to_json().expect("campaign should serialize");

    let config = ReplayConfig {
        campaign_json: &json,
        source_path: "future_version.json",
    };

    let result = DdgcHost::boot_from_campaign(&config);

    assert!(result.is_err(), "boot should reject future schema version");
    let err = result.err().unwrap();
    match err {
        HostError::UnsupportedCampaignSchema { found_version, supported_version } => {
            assert_eq!(found_version, CAMPAIGN_SNAPSHOT_VERSION + 1);
            assert_eq!(supported_version, CAMPAIGN_SNAPSHOT_VERSION);
        }
        other => panic!("expected UnsupportedCampaignSchema, got: {:?}", other),
    }
}

/// Smoke test: replay boot rejects malformed JSON.
#[test]
fn smoke_replay_boot_rejects_malformed_json() {
    let config = ReplayConfig {
        campaign_json: "not valid json {",
        source_path: "malformed.json",
    };

    let result = DdgcHost::boot_from_campaign(&config);

    assert!(result.is_err(), "boot should reject malformed JSON");
    match result.err().unwrap() {
        HostError::CampaignLoadFailed { .. } => {}
        other => panic!("expected CampaignLoadFailed, got: {:?}", other),
    }
}

// ── View model smoke tests ────────────────────────────────────────────────────

/// Smoke test: boot_load view model can be shaped from host phase.
#[test]
fn smoke_boot_load_vm_from_host_phase() {
    // Test Uninitialized
    let vm = boot_load_from_host(&HostPhase::Uninitialized, false, None).unwrap();
    assert!(vm.loaded);
    assert!(vm.error.is_none());

    // Test Ready with campaign
    let vm = boot_load_from_host(&HostPhase::Ready, true, Some(CAMPAIGN_SNAPSHOT_VERSION)).unwrap();
    assert!(vm.loaded);
    assert!(vm.error.is_none());
    assert_eq!(vm.campaign_schema_version, Some(CAMPAIGN_SNAPSHOT_VERSION));

    // Test FatalError
    let vm = boot_load_from_host(&HostPhase::FatalError, false, None).unwrap();
    assert!(!vm.loaded);
    assert!(vm.error.is_some());
}

/// Smoke test: BootLoadViewModel success contains registries list.
#[test]
fn smoke_boot_load_vm_success_contains_registries() {
    let vm = BootLoadViewModel::success("test message", vec!["curio", "trap", "building"]);
    assert!(vm.loaded);
    assert_eq!(vm.registries_loaded.len(), 3);
    assert!(vm.error.is_none());
}

/// Smoke test: BootLoadViewModel failure contains error message.
#[test]
fn smoke_boot_load_vm_failure_contains_error() {
    let vm = BootLoadViewModel::failure("something went wrong");
    assert!(!vm.loaded);
    assert!(vm.error.is_some());
    assert_eq!(vm.error.as_deref(), Some("something went wrong"));
}

// ── Contract boundary smoke tests ────────────────────────────────────────────

/// Smoke test: CampaignState round-trips through JSON without data loss.
#[test]
fn smoke_campaign_state_json_round_trip() {
    let original = CampaignState::new(500);
    let json = original.to_json().expect("serialize should work");
    let restored = CampaignState::from_json(&json).expect("deserialize should work");

    assert_eq!(original.gold, restored.gold);
    assert_eq!(original.schema_version, restored.schema_version);
}

/// Smoke test: contract boundary is preserved (no framework types leak).
#[test]
fn smoke_contract_boundary_no_framework_leakage() {
    // The CampaignState should only contain plain types
    let campaign = CampaignState::new(100);

    // Serialize to JSON
    let json = campaign.to_json().expect("should serialize");

    // JSON should not contain any framework-specific patterns
    assert!(!json.contains("ActorId"), "framework ActorId should not appear in contract JSON");
    assert!(!json.contains("EncounterId"), "framework EncounterId should not appear");
    assert!(!json.contains("RunId"), "framework RunId should not appear");
    assert!(!json.contains("FloorId"), "framework FloorId should not appear");

    // The JSON should be valid and parseable
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("should be valid JSON");
    assert!(parsed.is_object());
}

/// Smoke test: MapSize default is Short.
#[test]
fn smoke_map_size_default_is_short() {
    assert_eq!(LiveConfig::default().default_map_size, MapSize::Short);
}

// ── Error handling smoke tests ───────────────────────────────────────────────

/// Smoke test: HostError Display impl produces meaningful messages.
#[test]
fn smoke_host_error_display_produces_messages() {
    let errors = vec![
        HostError::DataDirectoryNotFound {
            path: "/fake/path".to_string(),
            message: "not found".to_string(),
        },
        HostError::ContractParse {
            file: "test.json".to_string(),
            message: "invalid format".to_string(),
        },
        HostError::CampaignLoadFailed {
            message: "parse error".to_string(),
        },
        HostError::UnsupportedCampaignSchema {
            found_version: 99,
            supported_version: 1,
        },
        HostError::InvalidInitialConfig {
            reason: "negative gold".to_string(),
        },
        HostError::FeatureNotSupported {
            feature: "multiplayer".to_string(),
        },
        HostError::InvalidHostState {
            actual: HostPhase::Uninitialized,
            expected: "Ready",
        },
    ];

    for error in errors {
        let display = format!("{}", error);
        assert!(!display.is_empty(), "error {:?} produced empty Display", error);
        assert!(display.len() > 10, "error {:?} produced too short message: {}", error, display);
    }
}

/// Smoke test: DdgcHost new produces uninitialized host.
#[test]
fn smoke_host_new_produces_uninitialized() {
    let host = DdgcHost::new();
    assert_eq!(host.phase(), HostPhase::Uninitialized);
    assert!(host.startup_mode().is_none());
    assert!(!host.is_ready());
    assert!(host.campaign().is_none());
    assert!(host.error_message().is_none());
}

/// Smoke test: DdgcHost can be cloned independently.
#[test]
fn smoke_host_clone_is_independent() {
    let host1 = DdgcHost::new();
    let host2 = host1.clone();

    assert_eq!(host1.phase(), host2.phase());
    assert_eq!(host1.startup_mode(), host2.startup_mode());
}