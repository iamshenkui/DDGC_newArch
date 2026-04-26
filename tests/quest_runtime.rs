//! Quest runtime and reward handoff integration tests (US-003-c).
//!
//! Validates that the quest runtime correctly:
//! - Accepts and tracks quests with full identity, difficulty, dungeon, objective,
//!   progress counters, completion/failure status, and rewards
//! - Updates quest progress as dungeon/run events occur
//! - Resolves deterministic reward or penalty handoff into campaign state
//! - Proves one representative quest (KillBoss) can be accepted, progressed,
//!   completed, and rewarded through the full run loop
//! - Explicitly classifies and traces unsupported quest types (not silently ignored)
//!
//! These tests live in the integration test suite (`tests/`) and exercise the
//! public API of the `game_ddgc_headless` crate, satisfying the "scoped to the
//! tests module" acceptance criterion for US-003-c.

use game_ddgc_headless::contracts::{
    CampaignState, DungeonType, HeirloomCurrency, MapSize, QuestType,
};
use game_ddgc_headless::run::quest_runtime::QuestRuntime;
use game_ddgc_headless::state::{QuestDifficulty, QuestObjective, QuestPenalties, QuestRewards, QuestState};

// ───────────────────────────────────────────────────────────────────
// Acceptance tests — quest state structure
// ───────────────────────────────────────────────────────────────────

/// Acceptance criterion: Quest state includes quest identity, difficulty, dungeon,
/// objective, progress counters, completion/failure status, and rewards.
#[test]
fn quest_state_contains_all_required_fields() {
    let quest = QuestState::new(
        "test_quest_001",
        QuestDifficulty::Standard,
        DungeonType::QingLong,
        MapSize::Short,
        QuestObjective::KillBoss,
        2,
        QuestRewards::standard(),
        QuestPenalties::standard(),
    );

    // Identity
    assert_eq!(quest.quest_id, "test_quest_001");

    // Difficulty
    assert_eq!(quest.difficulty, QuestDifficulty::Standard);

    // Dungeon and map size
    assert_eq!(quest.dungeon, DungeonType::QingLong);
    assert_eq!(quest.map_size, MapSize::Short);

    // Objective
    assert_eq!(quest.objective, QuestObjective::KillBoss);

    // Progress counters
    assert_eq!(quest.current_step, 0);
    assert_eq!(quest.max_steps, 2);

    // Completion/failure status
    assert!(!quest.completed);
    assert!(!quest.failed);

    // Rewards and penalties
    assert_eq!(quest.rewards.gold, 500);
    assert_eq!(quest.rewards.xp, 200);
    assert_eq!(quest.penalties.gold, -100);
}

/// Verify hard difficulty quest has different reward/penalty values.
#[test]
fn quest_state_hard_difficulty_has_scaled_rewards() {
    let quest = QuestState::new(
        "hard_quest",
        QuestDifficulty::Hard,
        DungeonType::BaiHu,
        MapSize::Medium,
        QuestObjective::KillBoss,
        3,
        QuestRewards::hard(),
        QuestPenalties::hard(),
    );

    assert_eq!(quest.difficulty, QuestDifficulty::Hard);
    assert_eq!(quest.rewards.gold, 1000);   // Hard = 1000 vs Standard = 500
    assert_eq!(quest.rewards.xp, 400);       // Hard = 400 vs Standard = 200
    assert_eq!(quest.penalties.gold, -250);   // Hard = -250 vs Standard = -100
}

// ───────────────────────────────────────────────────────────────────
// Acceptance tests — quest acceptance
// ───────────────────────────────────────────────────────────────────

/// Acceptance criterion: KillBoss quest (the representative test quest) can be
/// accepted via `accept_quest_from_type`.
#[test]
fn accept_kill_boss_quest_via_type_succeeds() {
    let mut runtime = QuestRuntime::new();

    let result = runtime.accept_quest_from_type(
        "q1_kill_boss",
        QuestType::KillBoss,
        DungeonType::QingLong,
        MapSize::Short,
    );

    assert!(result.is_ok(), "KillBoss quest should be accepted");
    assert_eq!(result.unwrap(), "q1_kill_boss");
    assert_eq!(runtime.active_count(), 1);
}

/// Acceptance criterion: Quests can be accepted directly with QuestState.
#[test]
fn accept_quest_directly_with_quest_state() {
    let mut runtime = QuestRuntime::new();

    let quest = QuestState::new(
        "direct_quest",
        QuestDifficulty::Standard,
        DungeonType::QingLong,
        MapSize::Short,
        QuestObjective::KillBoss,
        2,
        QuestRewards::standard(),
        QuestPenalties::standard(),
    );

    let result = runtime.accept_quest(quest);
    assert!(result.is_ok());
    assert_eq!(runtime.active_count(), 1);
}

/// Acceptance criterion: Multiple quests can be tracked simultaneously.
#[test]
fn accept_multiple_quests_tracks_all() {
    let mut runtime = QuestRuntime::new();

    runtime
        .accept_quest_from_type(
            "quest_1",
            QuestType::KillBoss,
            DungeonType::QingLong,
            MapSize::Short,
        )
        .unwrap();

    runtime
        .accept_quest_from_type(
            "quest_2",
            QuestType::KillBoss,
            DungeonType::BaiHu,
            MapSize::Medium,
        )
        .unwrap();

    assert_eq!(runtime.active_count(), 2);
}

// ───────────────────────────────────────────────────────────────────
// Acceptance tests — unsupported quest classification
// ───────────────────────────────────────────────────────────────────

/// Acceptance criterion: Unsupported quest types are explicitly classified and
/// traced, not silently ignored.
#[test]
fn unsupported_gather_quest_returns_trace() {
    let mut runtime = QuestRuntime::new();

    let result = runtime.accept_quest_from_type(
        "gather_quest",
        QuestType::Gather,
        DungeonType::QingLong,
        MapSize::Short,
    );

    assert!(result.is_err(), "Gather quest should be rejected");
    let trace = result.unwrap_err();
    assert_eq!(trace.quest_type, QuestType::Gather);
    assert!(!trace.is_supported());
    assert!(runtime.has_unsupported_quests());
}

/// Verify unsupported quest type is recorded in traces list.
#[test]
fn unsupported_quest_trace_recorded_in_runtime() {
    let mut runtime = QuestRuntime::new();

    // Attempt several unsupported quest types
    let _ = runtime.accept_quest_from_type("q_explore", QuestType::Explore, DungeonType::QingLong, MapSize::Short);
    let _ = runtime.accept_quest_from_type("q_cleanse", QuestType::Cleanse, DungeonType::QingLong, MapSize::Short);
    let _ = runtime.accept_quest_from_type("q_gather", QuestType::Gather, DungeonType::QingLong, MapSize::Short);
    let _ = runtime.accept_quest_from_type("q_activate", QuestType::Activate, DungeonType::QingLong, MapSize::Short);
    let _ = runtime.accept_quest_from_type("q_inventory", QuestType::InventoryActivate, DungeonType::QingLong, MapSize::Short);

    // Only KillBoss is supported
    assert!(runtime.has_unsupported_quests());
    let traces = runtime.unsupported_traces();
    assert_eq!(traces.len(), 5);

    for trace in traces {
        assert!(!trace.is_supported(), "Only KillBoss should be supported");
    }
}

// ───────────────────────────────────────────────────────────────────
// Acceptance tests — progress updates
// ───────────────────────────────────────────────────────────────────

/// Acceptance criterion: Quest progress updates as dungeon/run events occur.
#[test]
fn quest_progress_updates_on_run_completion() {
    let mut runtime = QuestRuntime::new();

    runtime
        .accept_quest_from_type(
            "test_quest",
            QuestType::KillBoss,
            DungeonType::QingLong,
            MapSize::Short,
        )
        .unwrap();

    // Simulate first run: rooms_cleared=9, battles_won=3, completed=false
    let updated = runtime.update_from_run(
        DungeonType::QingLong,
        MapSize::Short,
        9,
        3,
        false,
    );

    assert_eq!(updated, 1, "One quest should be updated");
    assert_eq!(runtime.active_quests()[0].current_step, 1);
    assert!(!runtime.active_quests()[0].completed);
}

/// Quest does not update when dungeon type does not match.
#[test]
fn quest_does_not_update_for_wrong_dungeon() {
    let mut runtime = QuestRuntime::new();

    runtime
        .accept_quest_from_type(
            "test_quest",
            QuestType::KillBoss,
            DungeonType::QingLong,
            MapSize::Short,
        )
        .unwrap();

    // Try to update for a different dungeon
    let updated = runtime.update_from_run(
        DungeonType::BaiHu, // Different dungeon
        MapSize::Short,
        9,
        3,
        true,
    );

    assert_eq!(updated, 0, "No quest should be updated for wrong dungeon");
    assert_eq!(runtime.active_quests()[0].current_step, 0);
}

/// Quest does not update when map size does not match.
#[test]
fn quest_does_not_update_for_wrong_map_size() {
    let mut runtime = QuestRuntime::new();

    runtime
        .accept_quest_from_type(
            "test_quest",
            QuestType::KillBoss,
            DungeonType::QingLong,
            MapSize::Short,
        )
        .unwrap();

    // Try to update for a different map size
    let updated = runtime.update_from_run(
        DungeonType::QingLong,
        MapSize::Medium, // Different map size
        9,
        3,
        true,
    );

    assert_eq!(updated, 0);
    assert_eq!(runtime.active_quests()[0].current_step, 0);
}

/// Multiple runs accumulate progress correctly.
#[test]
fn quest_progress_accumulates_across_multiple_runs() {
    let mut runtime = QuestRuntime::new();

    // Create quest with max_steps = 2 for testing multiple runs
    let quest = QuestState::new(
        "multi_run_quest",
        QuestDifficulty::Standard,
        DungeonType::QingLong,
        MapSize::Short,
        QuestObjective::KillBoss,
        2,
        QuestRewards::standard(),
        QuestPenalties::standard(),
    );
    runtime.accept_quest(quest).unwrap();

    // First run: battles_won > 0 advances progress
    runtime.update_from_run(
        DungeonType::QingLong,
        MapSize::Short,
        9,
        3,
        false, // not completed yet
    );

    assert_eq!(runtime.active_quests()[0].current_step, 1);
    assert!(!runtime.active_quests()[0].completed);

    // Second run: completes the quest
    runtime.update_from_run(
        DungeonType::QingLong,
        MapSize::Short,
        9,
        3,
        true, // completed
    );

    // KillBoss advances on battles_won > 0, so after 2 runs should be at max_steps
    assert_eq!(runtime.active_quests()[0].current_step, 2);
    assert!(runtime.active_quests()[0].completed || runtime.active_quests()[0].can_complete());
}

// ───────────────────────────────────────────────────────────────────
// Acceptance tests — reward/penalty resolution
// ───────────────────────────────────────────────────────────────────

/// Acceptance criterion: Quest completion resolves deterministic reward handoff
/// into campaign state.
#[test]
fn quest_completion_applies_rewards_to_campaign() {
    let mut runtime = QuestRuntime::new();

    // Create quest with max_steps = 1 so single run completes it
    let quest = QuestState::new(
        "reward_test_quest",
        QuestDifficulty::Standard,
        DungeonType::QingLong,
        MapSize::Short,
        QuestObjective::KillBoss,
        1, // max_steps = 1
        QuestRewards::standard(),
        QuestPenalties::standard(),
    );
    runtime.accept_quest(quest).unwrap();

    // Complete the quest
    runtime.update_from_run(
        DungeonType::QingLong,
        MapSize::Short,
        9,
        3,
        true,
    );

    // Resolve rewards into campaign
    let mut campaign = CampaignState::new(500);
    let summary = runtime.resolve_quest_outcomes(&mut campaign);

    assert_eq!(summary.completed, 1);
    assert_eq!(summary.failed, 0);
    assert!(!summary.rewards_applied.is_empty());

    // Check rewards were applied: standard rewards = 500 gold, 10 bones, 5 portraits
    assert_eq!(campaign.gold, 1000); // 500 initial + 500 reward
    assert_eq!(
        *campaign.heirlooms.get(&HeirloomCurrency::Bones).unwrap(),
        10
    );
    assert_eq!(
        *campaign.heirlooms.get(&HeirloomCurrency::Portraits).unwrap(),
        5
    );
}

/// Acceptance criterion: Quest failure applies penalties to campaign state.
#[test]
fn quest_failure_applies_penalties_to_campaign() {
    let mut runtime = QuestRuntime::new();

    runtime
        .accept_quest_from_type(
            "penalty_test_quest",
            QuestType::KillBoss,
            DungeonType::QingLong,
            MapSize::Short,
        )
        .unwrap();

    // Mark quest as failed (simulating a run failure event)
    runtime.fail_quest("penalty_test_quest");

    let mut campaign = CampaignState::new(500);
    let summary = runtime.resolve_quest_outcomes(&mut campaign);

    assert_eq!(summary.completed, 0);
    assert_eq!(summary.failed, 1);
    assert!(!summary.penalties_applied.is_empty());

    // Standard penalties: -100 gold, -5 bones, -2 portraits
    assert_eq!(campaign.gold, 400); // 500 - 100
}

/// Incomplete quest does not apply any rewards or penalties.
#[test]
fn incomplete_quest_resolves_nothing() {
    let mut runtime = QuestRuntime::new();

    runtime
        .accept_quest_from_type(
            "incomplete_quest",
            QuestType::KillBoss,
            DungeonType::QingLong,
            MapSize::Short,
        )
        .unwrap();

    // Update but don't complete
    runtime.update_from_run(
        DungeonType::QingLong,
        MapSize::Short,
        9,
        3,
        false, // not completed
    );

    let mut campaign = CampaignState::new(500);
    let summary = runtime.resolve_quest_outcomes(&mut campaign);

    assert_eq!(summary.completed, 0);
    assert_eq!(summary.failed, 0);
    assert!(summary.rewards_applied.is_empty());
    assert!(summary.penalties_applied.is_empty());
    assert_eq!(campaign.gold, 500); // Unchanged
}

/// Quest outcome summary captures all completed and failed quests.
#[test]
fn quest_outcome_summary_records_all_quests() {
    let mut runtime = QuestRuntime::new();

    // Add two quests
    let quest1 = QuestState::new(
        "complete_me",
        QuestDifficulty::Standard,
        DungeonType::QingLong,
        MapSize::Short,
        QuestObjective::KillBoss,
        1,
        QuestRewards::standard(),
        QuestPenalties::standard(),
    );
    let quest2 = QuestState::new(
        "fail_me",
        QuestDifficulty::Standard,
        DungeonType::BaiHu,
        MapSize::Medium,
        QuestObjective::KillBoss,
        1,
        QuestRewards::standard(),
        QuestPenalties::standard(),
    );

    runtime.accept_quest(quest1).unwrap();
    runtime.accept_quest(quest2).unwrap();

    // Complete first quest
    runtime.update_from_run(DungeonType::QingLong, MapSize::Short, 9, 3, true);

    // Fail second quest
    runtime.fail_quest("fail_me");

    let mut campaign = CampaignState::new(500);
    let summary = runtime.resolve_quest_outcomes(&mut campaign);

    assert_eq!(summary.completed, 1);
    assert_eq!(summary.failed, 1);
    assert_eq!(summary.rewards_applied.len(), 1);
    assert_eq!(summary.penalties_applied.len(), 1);
}

// ───────────────────────────────────────────────────────────────────
// Acceptance tests — full representative quest loop
// ───────────────────────────────────────────────────────────────────

/// Acceptance criterion: Focused test proves one representative quest can be
/// accepted, progressed, completed, and rewarded through the run loop.
///
/// This is the primary end-to-end acceptance test for US-003-c.
#[test]
fn representative_kill_boss_quest_full_run_loop() {
    // 1. SETUP: Create quest runtime and accept a KillBoss quest
    let mut runtime = QuestRuntime::new();

    let quest = QuestState::new(
        "kill_boss_qinglong",
        QuestDifficulty::Standard,
        DungeonType::QingLong,
        MapSize::Short,
        QuestObjective::KillBoss,
        1, // max_steps = 1 so one completed run completes it
        QuestRewards::standard(),
        QuestPenalties::standard(),
    );

    let quest_id = runtime
        .accept_quest(quest)
        .expect("KillBoss quest should be accepted");

    assert_eq!(quest_id, "kill_boss_qinglong");
    assert_eq!(runtime.active_count(), 1);

    // Verify initial quest state
    let quest_state = &runtime.active_quests()[0];
    assert_eq!(quest_state.quest_id, "kill_boss_qinglong");
    assert_eq!(quest_state.difficulty, QuestDifficulty::Standard);
    assert_eq!(quest_state.dungeon, DungeonType::QingLong);
    assert_eq!(quest_state.objective, QuestObjective::KillBoss);
    assert_eq!(quest_state.current_step, 0);
    assert!(!quest_state.completed);
    assert!(!quest_state.failed);

    // 2. PROGRESS: Single dungeon run that completes the quest
    let updated = runtime.update_from_run(
        DungeonType::QingLong,
        MapSize::Short,
        9,  // rooms_cleared
        3,  // battles_won
        true, // completed
    );

    assert_eq!(updated, 1, "Quest should be updated");

    let quest_state = &runtime.active_quests()[0];
    assert!(quest_state.completed || quest_state.can_complete());

    // 3. RESOLVE: Apply rewards to campaign state
    let mut campaign = CampaignState::new(500); // Start with 500 gold
    let summary = runtime.resolve_quest_outcomes(&mut campaign);

    // Verify resolution
    assert_eq!(summary.completed, 1);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.rewards_applied.len(), 1);

    // Verify campaign was updated correctly
    // Standard rewards: +500 gold, +10 bones, +5 portraits, +200 XP distributed
    assert_eq!(campaign.gold, 1000); // 500 + 500
    assert_eq!(
        *campaign.heirlooms.get(&HeirloomCurrency::Bones).unwrap(),
        10
    );
    assert_eq!(
        *campaign.heirlooms.get(&HeirloomCurrency::Portraits).unwrap(),
        5
    );
}

/// Verify the full loop works with hard difficulty and larger map.
#[test]
fn representative_kill_boss_quest_hard_difficulty_full_loop() {
    let mut runtime = QuestRuntime::new();

    // Hard difficulty KillBoss quest for BaiHu dungeon
    let quest = QuestState::new(
        "hard_boss_baihu",
        QuestDifficulty::Hard,
        DungeonType::BaiHu,
        MapSize::Medium,
        QuestObjective::KillBoss,
        2, // max_steps = 2
        QuestRewards::hard(),
        QuestPenalties::hard(),
    );

    runtime.accept_quest(quest).unwrap();

    // First run: partial progress
    runtime.update_from_run(DungeonType::BaiHu, MapSize::Medium, 12, 4, false);
    assert_eq!(runtime.active_quests()[0].current_step, 1);
    assert!(!runtime.active_quests()[0].completed);

    // Second run: complete
    runtime.update_from_run(DungeonType::BaiHu, MapSize::Medium, 12, 4, true);
    assert!(runtime.active_quests()[0].completed || runtime.active_quests()[0].can_complete());

    // Resolve
    let mut campaign = CampaignState::new(1000);
    let summary = runtime.resolve_quest_outcomes(&mut campaign);

    assert_eq!(summary.completed, 1);
    // Hard rewards: +1000 gold, +25 bones, +15 portraits, +5 tapes
    assert_eq!(campaign.gold, 2000); // 1000 + 1000
    assert_eq!(
        *campaign.heirlooms.get(&HeirloomCurrency::Bones).unwrap(),
        25
    );
}

// ───────────────────────────────────────────────────────────────────
// Edge case tests
// ───────────────────────────────────────────────────────────────────

/// Empty runtime resolves to empty summary.
#[test]
fn empty_runtime_resolves_to_empty_summary() {
    let mut runtime = QuestRuntime::new();
    let mut campaign = CampaignState::new(500);

    let summary = runtime.resolve_quest_outcomes(&mut campaign);

    assert_eq!(summary.completed, 0);
    assert_eq!(summary.failed, 0);
    assert!(summary.rewards_applied.is_empty());
    assert!(summary.penalties_applied.is_empty());
}

/// Runtime with no unsupported quests reports correctly.
#[test]
fn runtime_with_only_supported_quests_has_no_traces() {
    let mut runtime = QuestRuntime::new();

    runtime
        .accept_quest_from_type(
            "supported_1",
            QuestType::KillBoss,
            DungeonType::QingLong,
            MapSize::Short,
        )
        .unwrap();

    runtime
        .accept_quest_from_type(
            "supported_2",
            QuestType::KillBoss,
            DungeonType::BaiHu,
            MapSize::Medium,
        )
        .unwrap();

    assert!(!runtime.has_unsupported_quests());
    assert!(runtime.unsupported_traces().is_empty());
}

/// KillBoss is the only supported quest type — all others are traced.
#[test]
fn all_quest_types_except_kill_boss_are_unsupported() {
    let all_quest_types = [
        QuestType::Explore,
        QuestType::KillBoss,
        QuestType::Cleanse,
        QuestType::Gather,
        QuestType::Activate,
        QuestType::InventoryActivate,
    ];

    let supported_count = all_quest_types
        .iter()
        .filter(|qt| {
            let mut runtime = QuestRuntime::new();
            runtime
                .accept_quest_from_type("test", **qt, DungeonType::QingLong, MapSize::Short)
                .is_ok()
        })
        .count();

    // Only KillBoss should be supported
    assert_eq!(supported_count, 1);
}

/// Verify can_complete returns true only when step count is met and not failed.
#[test]
fn quest_can_complete_only_when_criteria_met() {
    let quest = QuestState::new(
        "test",
        QuestDifficulty::Standard,
        DungeonType::QingLong,
        MapSize::Short,
        QuestObjective::KillBoss,
        2,
        QuestRewards::standard(),
        QuestPenalties::standard(),
    );

    // Not completeable yet
    assert!(!quest.can_complete());

    // Even if completed flag is true but not at max_steps, can_complete is false
    // (because can_complete checks current_step >= max_steps && !failed)
    let mut quest_at_max = QuestState::new(
        "test2",
        QuestDifficulty::Standard,
        DungeonType::QingLong,
        MapSize::Short,
        QuestObjective::KillBoss,
        2,
        QuestRewards::standard(),
        QuestPenalties::standard(),
    );
    // Simulate reaching max steps
    quest_at_max.current_step = 2;
    assert!(quest_at_max.can_complete());

    // Failed quest cannot complete
    let mut failed_quest = QuestState::new(
        "test3",
        QuestDifficulty::Standard,
        DungeonType::QingLong,
        MapSize::Short,
        QuestObjective::KillBoss,
        2,
        QuestRewards::standard(),
        QuestPenalties::standard(),
    );
    failed_quest.current_step = 2;
    failed_quest.failed = true;
    assert!(!failed_quest.can_complete());
}

/// Gold and heirloom penalties cannot reduce campaign below zero.
#[test]
fn penalties_cannot_reduce_campaign_below_zero() {
    let mut runtime = QuestRuntime::new();

    let quest = QuestState::new(
        "penalty_quest",
        QuestDifficulty::Standard,
        DungeonType::QingLong,
        MapSize::Short,
        QuestObjective::KillBoss,
        1,
        QuestRewards::standard(),
        QuestPenalties::standard(),
    );
    runtime.accept_quest(quest).unwrap();

    // Mark as failed
    runtime.fail_quest("penalty_quest");

    // Campaign with 50 gold (less than -100 penalty)
    let mut campaign = CampaignState::new(50);
    campaign.heirlooms.insert(HeirloomCurrency::Bones, 2);
    campaign.heirlooms.insert(HeirloomCurrency::Portraits, 1);

    runtime.resolve_quest_outcomes(&mut campaign);

    // Gold should be clamped to 0
    assert_eq!(campaign.gold, 0);
    // Heirlooms should be clamped to 0
    assert_eq!(*campaign.heirlooms.get(&HeirloomCurrency::Bones).unwrap(), 0);
    assert_eq!(*campaign.heirlooms.get(&HeirloomCurrency::Portraits).unwrap(), 0);
}