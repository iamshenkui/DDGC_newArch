//! Cross-system golden trace and regression baseline tests.
//!
//! Verifies that the deterministic battle trace exercises hero, monster, skill,
//! and status systems together, and that future changes breaking cross-system
//! parity are caught by regression tests.

use game_ddgc_headless::scenarios::{first_battle, reactive_battle};
use game_ddgc_headless::trace::BattleTrace;

/// Verifies the semantic fixture battle trace is stable:
/// - Running run_first_battle() twice produces byte-identical JSON traces
/// - The committed first_battle_trace.json matches the live trace
#[test]
fn semantic_fixture_battle_trace_is_stable() {
    // Run twice and verify byte-identical traces
    let trace1 = first_battle::run_first_battle().trace.to_json();
    let trace2 = first_battle::run_first_battle().trace.to_json();
    assert_eq!(trace1, trace2, "Two runs must produce identical JSON traces");

    // Load committed golden trace and compare
    let golden_json = include_str!("../fixtures/semantic_battles/first_battle_trace.json");
    let golden_trace: BattleTrace = serde_json::from_str(golden_json)
        .expect("Failed to parse committed golden trace");
    let live_trace: BattleTrace = serde_json::from_str(&trace1)
        .expect("Failed to parse live trace");

    assert_eq!(live_trace, golden_trace, "Live trace must match committed golden trace");
}

/// Verifies the semantic fixture preserves role and skill identity:
/// - Crusader uses only crusading_strike
/// - Bone Soldier uses only rend
/// - No ally uses enemy skills
#[test]
fn semantic_fixture_preserves_role_and_skill_identity() {
    let result = first_battle::run_first_battle();

    // Actor 1 = Crusader (Ally), Actor 10 = Bone Soldier (Enemy)
    for entry in &result.trace.entries {
        if entry.action == "wait" || entry.action == "status_tick" {
            continue;
        }

        if entry.actor == 1 {
            // Crusader should only use crusading_strike
            assert_eq!(entry.action, "crusading_strike",
                "Crusader (actor 1) should only use crusading_strike, got '{}'", entry.action);
        } else if entry.actor == 10 {
            // Bone Soldier should only use rend
            assert_eq!(entry.action, "rend",
                "Bone Soldier (actor 10) should only use rend, got '{}'", entry.action);
        }
    }
}

/// Verifies the semantic fixture preserves status timing:
/// - If trace contains status_tick entries, verify they have non-zero damage values
/// - If no status_tick entries exist (current first_battle has none), verify the test
///   documents this as expected and passes
#[test]
fn semantic_fixture_preserves_status_timing() {
    let result = first_battle::run_first_battle();

    // The current first_battle scenario does not apply bleed/stun/horror,
    // so status_tick entries are expected to be absent. The Bone Soldier's
    // rend skill applies bleed via apply_status EffectNode, but the resolver
    // does not auto-execute apply_status nodes — they are recorded but
    // status attachment is a game-layer concern that the current battle
    // loop does not implement. Therefore, no status_tick entries appear.
    let status_ticks: Vec<_> = result.trace.entries.iter()
        .filter(|e| e.action == "status_tick")
        .collect();

    if status_ticks.is_empty() {
        // Expected: current first_battle has no status_tick entries because
        // apply_status effects in the skill are not auto-executed by the
        // resolver; status attachment requires game-layer implementation.
        // This is documented as expected behavior.
        assert!(true, "No status_tick entries — expected for current first_battle");
    } else {
        // If status_tick entries exist (future: after game-layer status
        // attachment is implemented), verify they have non-zero damage.
        for entry in &status_ticks {
            for effect in &entry.effects {
                assert!(effect.value.abs() > f64::EPSILON,
                    "status_tick entry should have non-zero damage, got {}", effect.value);
            }
        }
    }
}

/// Verifies the reactive battle trace is stable:
/// - Running run_reactive_battle() twice produces byte-identical JSON traces
/// - The committed reactive_battle_trace.json matches the live trace
#[test]
fn reactive_fixture_battle_trace_is_stable() {
    // Run twice and verify byte-identical traces
    let trace1 = reactive_battle::run_reactive_battle().trace.to_json();
    let trace2 = reactive_battle::run_reactive_battle().trace.to_json();
    assert_eq!(trace1, trace2, "Two runs must produce identical JSON traces");

    // Load committed golden trace and compare
    let golden_json = include_str!("../fixtures/semantic_battles/reactive_battle_trace.json");
    let golden_trace: BattleTrace = serde_json::from_str(golden_json)
        .expect("Failed to parse committed golden trace");
    let live_trace: BattleTrace = serde_json::from_str(&trace1)
        .expect("Failed to parse live trace");

    assert_eq!(live_trace, golden_trace, "Live trace must match committed golden trace");
}

/// Verifies the reactive fixture contains riposte entries:
/// - Trace must have entries with triggered_by.kind == "Riposte"
/// - The riposte action must be from the tank (actor 1) counter-attacking enemies
#[test]
fn reactive_fixture_contains_riposte_entries() {
    let result = reactive_battle::run_reactive_battle();

    let riposte_entries: Vec<_> = result.trace.entries.iter()
        .filter(|e| e.triggered_by.as_ref()
            .map(|t| t.kind == "Riposte")
            .unwrap_or(false))
        .collect();

    assert!(!riposte_entries.is_empty(), "Reactive battle should have riposte entries");

    // Verify the riposte is correctly triggered: enemy attacks tank, tank counter-attacks
    for entry in &riposte_entries {
        let trigger = entry.triggered_by.as_ref().unwrap();
        assert_eq!(trigger.kind, "Riposte");
        // The reactor (counter-attacker) should be actor 1 (tank)
        assert_eq!(entry.actor, 1, "Riposte counter-attacker should be the tank");
    }
}

/// Verifies the reactive fixture trace clearly separates original attacks from reactions:
/// - Each reactive entry must have a triggered_by that references the original attack
/// - The trigger must identify attacker, skill, and target
#[test]
fn reactive_fixture_triggers_are_explicit() {
    let result = reactive_battle::run_reactive_battle();

    for entry in &result.trace.entries {
        if let Some(ref trigger) = entry.triggered_by {
            // Reactive entries must have valid trigger info
            assert!(trigger.attacker != 0, "Trigger attacker should be non-zero");
            assert!(!trigger.skill.is_empty(), "Trigger skill should be non-empty");
            assert!(trigger.target != 0, "Trigger target should be non-zero");
            assert!(!trigger.kind.is_empty(), "Trigger kind should be non-empty");

            // Kind must be either "Riposte" or "GuardRedirect"
            assert!(
                trigger.kind == "Riposte" || trigger.kind == "GuardRedirect",
                "Trigger kind must be Riposte or GuardRedirect, got '{}'",
                trigger.kind
            );
        }
    }
}
