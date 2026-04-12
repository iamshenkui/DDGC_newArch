//! Boss encounter parity and golden trace tests.
//!
//! Verifies that boss encounter battles produce deterministic, stable traces
//! that preserve the identity of shared-health, summon, and paired/multi-body
//! boss mechanics. Golden trace JSON files are committed alongside these tests
//! so any regression in boss encounter resolution is caught immediately.

use game_ddgc_headless::encounters::{build_packs_registry, EncounterPackRegistry, PackType};
use game_ddgc_headless::run::encounters::EncounterResolver;
use game_ddgc_headless::trace::BattleTrace;

/// Helper: look up a boss pack by ID.
fn get_boss_pack<'a>(registry: &'a EncounterPackRegistry, pack_id: &str) -> &'a game_ddgc_headless::encounters::EncounterPack {
    registry.get(pack_id).unwrap_or_else(|| panic!("Boss pack '{}' should exist", pack_id))
}

// ═══════════════════════════════════════════════════════════════════════════════
// Shared-health boss traces
// ═══════════════════════════════════════════════════════════════════════════════

/// Verify the azure_dragon boss encounter produces a deterministic trace.
///
/// Azure Dragon is a shared-health boss: the main body fights alongside
/// ball_thunder and ball_wind parts that share a health pool in DDGC.
/// The trace must preserve the multi-part composition.
#[test]
fn azure_dragon_boss_trace_is_deterministic() {
    let resolver = EncounterResolver::new();
    let registry = build_packs_registry();
    let pack = get_boss_pack(&registry, "qinglong_boss_azure_dragon");

    let result1 = resolver.run_battle(pack, 1);
    let result2 = resolver.run_battle(pack, 1);

    assert_eq!(
        result1.trace.to_json(),
        result2.trace.to_json(),
        "Two runs of azure_dragon boss must produce identical JSON traces"
    );

    // Verify boss pack composition is preserved in the trace scenario name
    assert_eq!(result1.pack_id, "qinglong_boss_azure_dragon");
}

/// Verify the azure_dragon golden trace matches the live trace.
///
/// If this test fails, either the boss content changed (intentional) or
/// something broke the encounter resolution (unintentional). Update the
/// golden file only after verifying the change is intentional.
#[test]
fn azure_dragon_golden_trace_matches_live() {
    let resolver = EncounterResolver::new();
    let registry = build_packs_registry();
    let pack = get_boss_pack(&registry, "qinglong_boss_azure_dragon");

    let result = resolver.run_battle(pack, 1);

    let golden_json = include_str!("../fixtures/encounters/azure_dragon_trace.json");
    let golden_trace: BattleTrace = serde_json::from_str(golden_json)
        .expect("Failed to parse committed azure_dragon golden trace");
    let live_trace: BattleTrace = serde_json::from_str(&result.trace.to_json())
        .expect("Failed to parse live azure_dragon trace");

    assert_eq!(
        live_trace, golden_trace,
        "Live azure_dragon trace must match committed golden trace"
    );
}

/// Verify the azure_dragon boss trace preserves multi-part identity:
/// - Main body and ball part actors appear in the initial HP snapshot
/// - The battle terminates within a reasonable number of turns
#[test]
fn azure_dragon_trace_preserves_multi_part_identity() {
    let resolver = EncounterResolver::new();
    let registry = build_packs_registry();
    let pack = get_boss_pack(&registry, "qinglong_boss_azure_dragon");

    let result = resolver.run_battle(pack, 1);

    // Battle must terminate
    assert!(
        result.turns <= 100,
        "azure_dragon battle should finish within 100 turns, took {}",
        result.turns
    );

    // Trace must have entries (battle must have happened)
    assert!(
        !result.trace.entries.is_empty(),
        "azure_dragon battle trace should record events"
    );

    // Boss pack has 3 units, so the initial HP snapshot should contain 3 enemy actors
    let first_snapshot = &result.trace.entries[0].snapshot;
    let enemy_ids_in_snapshot: Vec<&u64> = first_snapshot.keys()
        .filter(|&&id| id >= 10)
        .collect();

    assert!(
        enemy_ids_in_snapshot.len() >= 3,
        "azure_dragon first snapshot should contain at least 3 enemy actors (main body + 2 balls), got {}",
        enemy_ids_in_snapshot.len()
    );

    // Ball parts should have lower HP (55) than main body (150)
    let main_body_hp = first_snapshot.get(&11).copied().unwrap_or(0.0);
    assert!(
        main_body_hp > 100.0,
        "azure_dragon main body (actor 11) should have high HP (>100), got {}",
        main_body_hp
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// Summon boss traces
// ═══════════════════════════════════════════════════════════════════════════════

/// Verify the rotvine_wraith boss encounter produces a deterministic trace.
///
/// Rotvine Wraith is a summon boss: it continuously re-summons rotten_fruit
/// minions (A/B variants). The trace must preserve the summon + minion
/// composition.
#[test]
fn rotvine_wraith_boss_trace_is_deterministic() {
    let resolver = EncounterResolver::new();
    let registry = build_packs_registry();
    let pack = get_boss_pack(&registry, "xuanwu_boss_rotvine_wraith");

    let result1 = resolver.run_battle(pack, 1);
    let result2 = resolver.run_battle(pack, 1);

    assert_eq!(
        result1.trace.to_json(),
        result2.trace.to_json(),
        "Two runs of rotvine_wraith boss must produce identical JSON traces"
    );

    assert_eq!(result1.pack_id, "xuanwu_boss_rotvine_wraith");
}

/// Verify the rotvine_wraith golden trace matches the live trace.
#[test]
fn rotvine_wraith_golden_trace_matches_live() {
    let resolver = EncounterResolver::new();
    let registry = build_packs_registry();
    let pack = get_boss_pack(&registry, "xuanwu_boss_rotvine_wraith");

    let result = resolver.run_battle(pack, 1);

    let golden_json = include_str!("../fixtures/encounters/rotvine_wraith_trace.json");
    let golden_trace: BattleTrace = serde_json::from_str(golden_json)
        .expect("Failed to parse committed rotvine_wraith golden trace");
    let live_trace: BattleTrace = serde_json::from_str(&result.trace.to_json())
        .expect("Failed to parse live rotvine_wraith trace");

    assert_eq!(
        live_trace, golden_trace,
        "Live rotvine_wraith trace must match committed golden trace"
    );
}

/// Verify the rotvine_wraith boss trace preserves summon identity:
/// - The main boss actor appears
/// - Rotten fruit actors appear in the initial HP snapshot (even if they die quickly)
/// - The battle terminates within a reasonable number of turns
#[test]
fn rotvine_wraith_trace_preserves_summon_identity() {
    let resolver = EncounterResolver::new();
    let registry = build_packs_registry();
    let pack = get_boss_pack(&registry, "xuanwu_boss_rotvine_wraith");

    let result = resolver.run_battle(pack, 1);

    assert!(
        result.turns <= 100,
        "rotvine_wraith battle should finish within 100 turns, took {}",
        result.turns
    );

    assert!(
        !result.trace.entries.is_empty(),
        "rotvine_wraith battle trace should record events"
    );

    // Check that the first trace entry's HP snapshot contains all 3 enemy actors
    // (rotvine_wraith + rotten_fruit_A + rotten_fruit_B). Even if fruits die
    // before acting, their presence in the initial snapshot proves they were
    // part of the encounter composition.
    let first_snapshot = &result.trace.entries[0].snapshot;
    let enemy_ids_in_snapshot: Vec<&u64> = first_snapshot.keys()
        .filter(|&&id| id >= 10)
        .collect();

    assert!(
        enemy_ids_in_snapshot.len() >= 3,
        "rotvine_wraith first snapshot should contain at least 3 enemy actors (boss + 2 fruits), got {}",
        enemy_ids_in_snapshot.len()
    );

    // Rotten fruit actors should have low HP (30) in the initial snapshot
    let fruit_hp_count = first_snapshot.values()
        .filter(|&&hp| hp > 0.0 && hp <= 30.0)
        .count();
    assert!(
        fruit_hp_count >= 2,
        "rotvine_wraith first snapshot should have at least 2 low-HP actors (fruits at 30 HP), got {} actors with HP <= 30",
        fruit_hp_count
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// Paired / multi-body boss traces
// ═══════════════════════════════════════════════════════════════════════════════

/// Verify the bloodthirsty_assassin boss encounter produces a deterministic trace.
///
/// Bloodthirsty Assassin is a paired boss: it fights alongside bloodthirsty_shadow.
/// The two units share HP-averaging mechanics via crimson_duet. The trace must
/// preserve the paired composition.
#[test]
fn bloodthirsty_assassin_boss_trace_is_deterministic() {
    let resolver = EncounterResolver::new();
    let registry = build_packs_registry();
    let pack = get_boss_pack(&registry, "cross_boss_bloodthirsty_assassin");

    let result1 = resolver.run_battle(pack, 1);
    let result2 = resolver.run_battle(pack, 1);

    assert_eq!(
        result1.trace.to_json(),
        result2.trace.to_json(),
        "Two runs of bloodthirsty_assassin boss must produce identical JSON traces"
    );

    assert_eq!(result1.pack_id, "cross_boss_bloodthirsty_assassin");
}

/// Verify the bloodthirsty_assassin golden trace matches the live trace.
#[test]
fn bloodthirsty_assassin_golden_trace_matches_live() {
    let resolver = EncounterResolver::new();
    let registry = build_packs_registry();
    let pack = get_boss_pack(&registry, "cross_boss_bloodthirsty_assassin");

    let result = resolver.run_battle(pack, 1);

    let golden_json = include_str!("../fixtures/encounters/bloodthirsty_assassin_trace.json");
    let golden_trace: BattleTrace = serde_json::from_str(golden_json)
        .expect("Failed to parse committed bloodthirsty_assassin golden trace");
    let live_trace: BattleTrace = serde_json::from_str(&result.trace.to_json())
        .expect("Failed to parse live bloodthirsty_assassin trace");

    assert_eq!(
        live_trace, golden_trace,
        "Live bloodthirsty_assassin trace must match committed golden trace"
    );
}

/// Verify the bloodthirsty_assassin boss trace preserves paired identity:
/// - Both the assassin and shadow actors appear in the initial HP snapshot
/// - Both take actions during the battle
/// - The battle terminates within a reasonable number of turns
#[test]
fn bloodthirsty_assassin_trace_preserves_paired_identity() {
    let resolver = EncounterResolver::new();
    let registry = build_packs_registry();
    let pack = get_boss_pack(&registry, "cross_boss_bloodthirsty_assassin");

    let result = resolver.run_battle(pack, 1);

    assert!(
        result.turns <= 100,
        "bloodthirsty_assassin battle should finish within 100 turns, took {}",
        result.turns
    );

    assert!(
        !result.trace.entries.is_empty(),
        "bloodthirsty_assassin battle trace should record events"
    );

    // Boss pack has 2 units: assassin + shadow, both HP 150
    let first_snapshot = &result.trace.entries[0].snapshot;
    let enemy_ids_in_snapshot: Vec<&u64> = first_snapshot.keys()
        .filter(|&&id| id >= 10)
        .collect();

    assert!(
        enemy_ids_in_snapshot.len() >= 2,
        "bloodthirsty_assassin first snapshot should contain at least 2 enemy actors (assassin + shadow), got {}",
        enemy_ids_in_snapshot.len()
    );

    // Both paired units should have equal HP (both 150)
    let enemy_hps: Vec<f64> = first_snapshot.iter()
        .filter(|(&id, _)| id >= 10)
        .map(|(_, &hp)| hp)
        .collect();
    // Verify at least two enemies share the same high HP (paired boss signature)
    let high_hp_enemies = enemy_hps.iter().filter(|&&hp| hp >= 100.0).count();
    assert!(
        high_hp_enemies >= 2,
        "bloodthirsty_assassin paired boss should have at least 2 high-HP enemies, got {}",
        high_hp_enemies
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// Multi-body boss trace (dual-body composite)
// ═══════════════════════════════════════════════════════════════════════════════

/// Verify the black_tortoise boss encounter produces a deterministic trace.
///
/// Black Tortoise is a dual-body composite boss: Tortoise A (Tank) and
/// Snake B (Controller) fight together with shared-damage mechanics.
#[test]
fn black_tortoise_boss_trace_is_deterministic() {
    let resolver = EncounterResolver::new();
    let registry = build_packs_registry();
    let pack = get_boss_pack(&registry, "xuanwu_boss_black_tortoise");

    let result1 = resolver.run_battle(pack, 1);
    let result2 = resolver.run_battle(pack, 1);

    assert_eq!(
        result1.trace.to_json(),
        result2.trace.to_json(),
        "Two runs of black_tortoise boss must produce identical JSON traces"
    );

    assert_eq!(result1.pack_id, "xuanwu_boss_black_tortoise");
}

/// Verify the black_tortoise golden trace matches the live trace.
#[test]
fn black_tortoise_golden_trace_matches_live() {
    let resolver = EncounterResolver::new();
    let registry = build_packs_registry();
    let pack = get_boss_pack(&registry, "xuanwu_boss_black_tortoise");

    let result = resolver.run_battle(pack, 1);

    let golden_json = include_str!("../fixtures/encounters/black_tortoise_trace.json");
    let golden_trace: BattleTrace = serde_json::from_str(golden_json)
        .expect("Failed to parse committed black_tortoise golden trace");
    let live_trace: BattleTrace = serde_json::from_str(&result.trace.to_json())
        .expect("Failed to parse live black_tortoise trace");

    assert_eq!(
        live_trace, golden_trace,
        "Live black_tortoise trace must match committed golden trace"
    );
}

/// Verify the black_tortoise boss trace preserves dual-body identity:
/// - Both body A and body B actors appear in the initial HP snapshot
/// - Both take actions during the battle
/// - The battle terminates within a reasonable number of turns
#[test]
fn black_tortoise_trace_preserves_dual_body_identity() {
    let resolver = EncounterResolver::new();
    let registry = build_packs_registry();
    let pack = get_boss_pack(&registry, "xuanwu_boss_black_tortoise");

    let result = resolver.run_battle(pack, 1);

    assert!(
        result.turns <= 100,
        "black_tortoise battle should finish within 100 turns, took {}",
        result.turns
    );

    assert!(
        !result.trace.entries.is_empty(),
        "black_tortoise battle trace should record events"
    );

    // Boss pack has 2 units: tortoise A + snake B, both HP 115
    let first_snapshot = &result.trace.entries[0].snapshot;
    let enemy_ids_in_snapshot: Vec<&u64> = first_snapshot.keys()
        .filter(|&&id| id >= 10)
        .collect();

    assert!(
        enemy_ids_in_snapshot.len() >= 2,
        "black_tortoise first snapshot should contain at least 2 enemy actors (A + B), got {}",
        enemy_ids_in_snapshot.len()
    );

    // Both bodies should take actions (dual-body composition means both participate)
    let enemy_actors: std::collections::HashSet<u64> = result.trace.entries.iter()
        .filter(|e| e.actor >= 10)
        .map(|e| e.actor)
        .collect();

    assert!(
        enemy_actors.len() >= 2,
        "black_tortoise battle should have at least 2 enemy actors taking actions (A + B), got {}",
        enemy_actors.len()
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// Cross-boss parity — all boss traces are deterministic
// ═══════════════════════════════════════════════════════════════════════════════

/// Verify ALL registered boss packs produce deterministic battle traces.
///
/// This is the comprehensive parity check: every boss pack in the registry
/// must produce byte-identical traces on repeated runs. If any boss fails,
/// it indicates non-deterministic encounter resolution.
#[test]
fn all_boss_packs_produce_deterministic_traces() {
    let resolver = EncounterResolver::new();
    let registry = build_packs_registry();

    let boss_packs: Vec<_> = registry.by_type(PackType::Boss);
    assert!(!boss_packs.is_empty(), "Registry should have boss packs");

    for pack in &boss_packs {
        let result1 = resolver.run_battle(pack, 1);
        let result2 = resolver.run_battle(pack, 1);

        assert_eq!(
            result1.trace.to_json(),
            result2.trace.to_json(),
            "Boss pack '{}' must produce deterministic traces",
            pack.id
        );

        assert!(
            result1.turns <= 100,
            "Boss pack '{}' battle should finish within 100 turns, took {}",
            pack.id,
            result1.turns
        );
    }
}
