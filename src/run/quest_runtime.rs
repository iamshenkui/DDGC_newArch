//! Quest runtime — tracks active quests during dungeon runs and resolves reward handoff.
//!
//! This module implements the runner-layer logic for quest management:
//! - Quest acceptance and tracking during dungeon runs
//! - Progress updates as dungeon/run events occur
//! - Deterministic reward/penalty resolution into campaign state
//! - Explicit classification of unsupported quest types (not silently ignored)
//!
//! The representative test quest is a standard-difficulty KillBoss quest.

use crate::contracts::{CampaignState, DungeonType, MapSize};
use crate::state::{
    QuestDifficulty, QuestObjective, QuestPenalties, QuestRewards, QuestState,
    UnsupportedQuestTrace,
};

/// Tracks all active quests during a dungeon run.
///
/// This struct is created when a run starts and maintains quest state
/// until the run completes, at which point rewards/penalties are resolved.
#[derive(Debug, Clone)]
pub struct QuestRuntime {
    /// Active quests being tracked during this run.
    active_quests: Vec<QuestState>,
    /// Traces for any unsupported quest types encountered.
    unsupported_traces: Vec<UnsupportedQuestTrace>,
}

impl QuestRuntime {
    /// Create a new quest runtime with no active quests.
    pub fn new() -> Self {
        QuestRuntime {
            active_quests: Vec::new(),
            unsupported_traces: Vec::new(),
        }
    }

    /// Accept a quest for tracking during the run.
    ///
    /// Returns the accepted quest ID if successful, or an unsupported trace
    /// if the quest type is not supported.
    pub fn accept_quest(&mut self, quest: QuestState) -> Result<String, UnsupportedQuestTrace> {
        let quest_type = self.infer_quest_type(&quest);
        let quest_id = quest.quest_id.clone();

        // Check if the objective type is supported using explicit classification
        // (QuestObjective::is_supported() returns true for all, so we check explicitly)
        if !Self::is_objective_type_supported(&quest.objective) {
            let trace = UnsupportedQuestTrace::new(quest_type);
            self.unsupported_traces.push(trace.clone());
            return Err(trace);
        }

        self.active_quests.push(quest);
        Ok(quest_id)
    }

    /// Check if an objective type is explicitly supported.
    ///
    /// Only KillBoss is currently supported. All other objectives require
    /// additional game systems that are not yet implemented.
    fn is_objective_type_supported(objective: &QuestObjective) -> bool {
        matches!(objective, QuestObjective::KillBoss)
    }

    /// Accept a quest from a contracts QuestType with standard parameters.
    ///
    /// This is the main entry point for accepting quests. Returns the accepted
    /// quest ID if the quest type is supported, or an unsupported trace.
    pub fn accept_quest_from_type(
        &mut self,
        quest_id: &str,
        quest_type: crate::contracts::QuestType,
        dungeon: DungeonType,
        map_size: MapSize,
    ) -> Result<String, UnsupportedQuestTrace> {
        // Map QuestType to QuestObjective
        let objective = match QuestObjective::from_quest_type(quest_type) {
            Some(obj) => obj,
            None => {
                let trace = UnsupportedQuestTrace::new(quest_type);
                self.unsupported_traces.push(trace.clone());
                return Err(trace);
            }
        };

        // Check if objective is supported using explicit classification
        if !Self::is_objective_type_supported(&objective) {
            let trace = UnsupportedQuestTrace::new(quest_type);
            self.unsupported_traces.push(trace.clone());
            return Err(trace);
        }

        // Create quest state with standard difficulty
        let quest = QuestState::new(
            quest_id,
            QuestDifficulty::Standard,
            dungeon,
            map_size,
            objective,
            2, // max_steps for KillBoss is 2 (representative test quest)
            QuestRewards::standard(),
            QuestPenalties::standard(),
        );

        self.active_quests.push(quest);
        Ok(quest_id.to_string())
    }

    /// Infer the contracts QuestType from a QuestState.
    fn infer_quest_type(&self, quest: &QuestState) -> crate::contracts::QuestType {
        match quest.objective {
            QuestObjective::ClearDungeon => crate::contracts::QuestType::Explore,
            QuestObjective::KillBoss => crate::contracts::QuestType::KillBoss,
            QuestObjective::CleanseCorruption => crate::contracts::QuestType::Cleanse,
            QuestObjective::GatherItems => crate::contracts::QuestType::Gather,
            QuestObjective::ActivateMechanism => crate::contracts::QuestType::Activate,
            QuestObjective::UseInventoryItem => crate::contracts::QuestType::InventoryActivate,
        }
    }

    /// Update all active quests based on a dungeon run event.
    ///
    /// Called after a dungeon run completes to update quest progress.
    /// Returns the number of quests that were updated.
    pub fn update_from_run(
        &mut self,
        dungeon: DungeonType,
        map_size: MapSize,
        rooms_cleared: u32,
        battles_won: u32,
        completed: bool,
    ) -> usize {
        let mut updated = 0;
        for quest in &mut self.active_quests {
            if quest.is_active()
                && quest
                    .update_from_run(dungeon, map_size, rooms_cleared, battles_won, completed)
                    .is_some()
            {
                updated += 1;
                // After update, if the quest can complete (all steps done),
                // mark it as completed. This ensures the completed flag is set
                // when the quest reaches max_steps from run completion.
                if completed && quest.can_complete() {
                    quest.completed = true;
                }
            }
        }
        updated
    }

    /// Resolve all quest completions and failures, applying rewards/penalties to campaign.
    ///
    /// Returns a summary of what was resolved.
    pub fn resolve_quest_outcomes(&mut self, campaign: &mut CampaignState) -> QuestOutcomeSummary {
        let mut summary = QuestOutcomeSummary::default();

        for quest in &mut self.active_quests {
            // Check for quest completion - either explicitly marked or can_complete
            // (can_complete means current_step >= max_steps and not failed)
            if quest.completed || quest.can_complete() {
                // Apply rewards directly - don't call complete() since it checks
                // is_active() which would fail for already-completed quests
                let rewards = quest.rewards.clone();
                quest.apply_rewards_to_campaign(&rewards, campaign);
                summary.completed += 1;
                summary.rewards_applied.push(QuestRewardRecord {
                    quest_id: quest.quest_id.clone(),
                    rewards,
                });
            } else if quest.failed {
                // Quest was marked as failed - apply penalties directly
                // (don't call fail() since it would return None for already-failed quests)
                quest.apply_penalties_to_campaign(&quest.penalties, campaign);
                summary.failed += 1;
                summary.penalties_applied.push(QuestPenaltyRecord {
                    quest_id: quest.quest_id.clone(),
                    penalties: quest.penalties.clone(),
                });
            }
        }

        summary
    }

    /// Get all active quests (not completed or failed).
    pub fn active_quests(&self) -> &[QuestState] {
        &self.active_quests
    }

    /// Get all unsupported quest traces encountered.
    pub fn unsupported_traces(&self) -> &[UnsupportedQuestTrace] {
        &self.unsupported_traces
    }

    /// Check if any unsupported quest types were encountered.
    pub fn has_unsupported_quests(&self) -> bool {
        !self.unsupported_traces.is_empty()
    }

    /// Get the count of active (not completed/failed) quests.
    pub fn active_count(&self) -> usize {
        self.active_quests.iter().filter(|q| q.is_active()).count()
    }
}

impl Default for QuestRuntime {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary of quest outcomes resolved after a run.
#[derive(Debug, Clone, Default)]
pub struct QuestOutcomeSummary {
    /// Number of quests completed.
    pub completed: usize,
    /// Number of quests failed.
    pub failed: usize,
    /// Rewards applied for completed quests.
    pub rewards_applied: Vec<QuestRewardRecord>,
    /// Penalties applied for failed quests.
    pub penalties_applied: Vec<QuestPenaltyRecord>,
}

/// Record of rewards applied for a completed quest.
#[derive(Debug, Clone)]
pub struct QuestRewardRecord {
    pub quest_id: String,
    pub rewards: QuestRewards,
}

/// Record of penalties applied for a failed quest.
#[derive(Debug, Clone)]
pub struct QuestPenaltyRecord {
    pub quest_id: String,
    pub penalties: QuestPenalties,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{DungeonType, MapSize};

    // ───────────────────────────────────────────────────────────────
    // Acceptance tests
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn accept_kill_boss_quest_succeeds() {
        let mut runtime = QuestRuntime::new();
        let result = runtime.accept_quest_from_type(
            "test_quest",
            crate::contracts::QuestType::KillBoss,
            DungeonType::QingLong,
            MapSize::Short,
        );

        assert!(result.is_ok(), "KillBoss quest should be accepted");
        assert_eq!(result.unwrap(), "test_quest");
        assert_eq!(runtime.active_count(), 1);
    }

    #[test]
    fn accept_unsupported_quest_returns_trace() {
        let mut runtime = QuestRuntime::new();

        // Gather is not yet supported
        let result = runtime.accept_quest_from_type(
            "gather_quest",
            crate::contracts::QuestType::Gather,
            DungeonType::QingLong,
            MapSize::Short,
        );

        assert!(result.is_err(), "Gather quest should be rejected");
        let trace = result.unwrap_err();
        assert_eq!(trace.quest_type, crate::contracts::QuestType::Gather);
        assert!(!trace.is_supported());
        assert!(runtime.has_unsupported_quests());
    }

    #[test]
    fn accept_multiple_quests_tracks_all() {
        let mut runtime = QuestRuntime::new();

        runtime
            .accept_quest_from_type(
                "quest_1",
                crate::contracts::QuestType::KillBoss,
                DungeonType::QingLong,
                MapSize::Short,
            )
            .unwrap();

        runtime
            .accept_quest_from_type(
                "quest_2",
                crate::contracts::QuestType::KillBoss,
                DungeonType::BaiHu,
                MapSize::Medium,
            )
            .unwrap();

        assert_eq!(runtime.active_count(), 2);
    }

    // ───────────────────────────────────────────────────────────────
    // Progress update tests
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn quest_progress_updates_on_run_completion() {
        let mut runtime = QuestRuntime::new();
        runtime
            .accept_quest_from_type(
                "test_quest",
                crate::contracts::QuestType::KillBoss,
                DungeonType::QingLong,
                MapSize::Short,
            )
            .unwrap();

        // First run: rooms_cleared=9, battles_won=3, completed=false
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

    #[test]
    fn quest_progress_updates_on_multiple_runs() {
        let mut runtime = QuestRuntime::new();
        runtime
            .accept_quest_from_type(
                "test_quest",
                crate::contracts::QuestType::KillBoss,
                DungeonType::QingLong,
                MapSize::Short,
            )
            .unwrap();

        // First run
        runtime.update_from_run(
            DungeonType::QingLong,
            MapSize::Short,
            9,
            3,
            false,
        );

        // Second run (quest completes on second run)
        let updated = runtime.update_from_run(
            DungeonType::QingLong,
            MapSize::Short,
            9,
            3,
            true, // completed
        );

        assert_eq!(updated, 1);
        // KillBoss advances on battles_won, so after 2 runs with battles_won > 0
        // the quest should be at max_steps
        assert_eq!(runtime.active_quests()[0].current_step, 2);
        assert!(runtime.active_quests()[0].completed);
    }

    #[test]
    fn quest_does_not_update_for_wrong_dungeon() {
        let mut runtime = QuestRuntime::new();
        runtime
            .accept_quest_from_type(
                "test_quest",
                crate::contracts::QuestType::KillBoss,
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

    // ───────────────────────────────────────────────────────────────
    // Reward/penalty resolution tests
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn quest_completion_applies_rewards() {
        let mut runtime = QuestRuntime::new();

        // Create quest directly with max_steps = 1 for simpler test
        let quest = QuestState::new(
            "test_quest",
            QuestDifficulty::Standard,
            DungeonType::QingLong,
            MapSize::Short,
            QuestObjective::KillBoss,
            1, // max_steps = 1 so one battle-won run completes it
            QuestRewards::standard(),
            QuestPenalties::standard(),
        );
        runtime.accept_quest(quest).unwrap();

        // Advance quest to completion (single run with battles_won > 0)
        runtime.update_from_run(
            DungeonType::QingLong,
            MapSize::Short,
            9,
            3,
            true, // completed
        );

        // Create campaign and resolve
        let mut campaign = CampaignState::new(500);
        let summary = runtime.resolve_quest_outcomes(&mut campaign);

        assert_eq!(summary.completed, 1);
        assert_eq!(summary.failed, 0);
        assert!(!summary.rewards_applied.is_empty());

        // Check rewards were applied (standard rewards: 500 gold, 10 bones, 5 portraits)
        assert_eq!(campaign.gold, 1000); // 500 + 500
        assert_eq!(*campaign.heirlooms.get(&crate::contracts::HeirloomCurrency::Bones).unwrap(), 10);
        assert_eq!(*campaign.heirlooms.get(&crate::contracts::HeirloomCurrency::Portraits).unwrap(), 5);
    }

    #[test]
    fn quest_failure_applies_penalties() {
        // Test that a quest that was tracked but failed applies penalties correctly.
        // Quest failure is tracked separately - here we test that a failed quest
        // that gets resolved applies the correct penalties.
        let mut runtime = QuestRuntime::new();

        // Accept a quest
        runtime
            .accept_quest_from_type(
                "test_quest",
                crate::contracts::QuestType::KillBoss,
                DungeonType::QingLong,
                MapSize::Short,
            )
            .unwrap();

        // Simulate the quest being marked as failed (e.g., from a run failure event)
        // by directly modifying the quest state - this represents the failure event
        // having been recorded somewhere in the run loop
        if let Some(quest) = runtime.active_quests.first_mut() {
            quest.failed = true;
        }

        let mut campaign = CampaignState::new(500);
        let summary = runtime.resolve_quest_outcomes(&mut campaign);

        assert_eq!(summary.completed, 0);
        assert_eq!(summary.failed, 1);
        assert!(!summary.penalties_applied.is_empty());

        // Check penalties were applied (standard penalties: -100 gold, -5 bones, -2 portraits)
        assert_eq!(campaign.gold, 400); // 500 - 100
    }

    #[test]
    fn incomplete_quest_does_not_apply_rewards() {
        let mut runtime = QuestRuntime::new();
        runtime
            .accept_quest_from_type(
                "test_quest",
                crate::contracts::QuestType::KillBoss,
                DungeonType::QingLong,
                MapSize::Short,
            )
            .unwrap();

        // Don't complete the quest - just update partially
        runtime.update_from_run(
            DungeonType::QingLong,
            MapSize::Short,
            9,
            3,
            false, // not completed
        );

        let mut campaign = CampaignState::new(500);
        let summary = runtime.resolve_quest_outcomes(&mut campaign);

        // No rewards or penalties should be applied
        assert_eq!(summary.completed, 0);
        assert_eq!(summary.failed, 0);
        assert!(summary.rewards_applied.is_empty());
        assert!(summary.penalties_applied.is_empty());

        // Campaign gold unchanged
        assert_eq!(campaign.gold, 500);
    }

    // ───────────────────────────────────────────────────────────────
    // Full representative quest test
    // ───────────────────────────────────────────────────────────────

    #[test]
    fn representative_kill_boss_quest_full_loop() {
        // This test proves the representative quest can be:
        // accepted, progressed, completed, and rewarded through the run loop

        let mut runtime = QuestRuntime::new();

        // 1. Accept the quest with max_steps = 1 for simpler test
        let quest = QuestState::new(
            "kill_boss_qinglong",
            QuestDifficulty::Standard,
            DungeonType::QingLong,
            MapSize::Short,
            QuestObjective::KillBoss,
            1, // max_steps = 1 so one completed run completes the quest
            QuestRewards::standard(),
            QuestPenalties::standard(),
        );
        let quest_id = runtime.accept_quest(quest).expect("KillBoss quest should be accepted");

        assert_eq!(quest_id, "kill_boss_qinglong");

        // 2. Single run that completes the quest
        runtime.update_from_run(
            DungeonType::QingLong,
            MapSize::Short,
            9,  // rooms_cleared
            3,  // battles_won
            true, // completed
        );

        let quest = &runtime.active_quests()[0];
        assert!(quest.can_complete());
        assert!(quest.completed);

        // 3. Resolve rewards into campaign
        let mut campaign = CampaignState::new(500);
        let summary = runtime.resolve_quest_outcomes(&mut campaign);

        assert_eq!(summary.completed, 1);
        assert_eq!(summary.failed, 0);
        assert_eq!(campaign.gold, 1000); // 500 initial + 500 reward
    }
}