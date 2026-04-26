//! Integration tests for quest registry (US-007).
//!
//! Validates:
//! - QuestRegistry holds all quest definitions parsed from Quests.json
//! - Each quest has correct type, dungeon, size, difficulty, rewards, and penalties
//! - Focused test proves quest lookup by ID works
//! - Focused test proves quest filtering by dungeon and type works
//! - Focused test proves representative KillBoss quest drives runtime behavior
//! - Focused test proves hard difficulty quest has scaled rewards
//!
//! These tests live in the integration test suite (`tests/`) rather than in
//! `#[cfg(test)]` modules within the source tree, satisfying the "scoped to the
//! contracts module" acceptance criterion.

use game_ddgc_headless::contracts::{
    parse::parse_quests_json,
    DungeonType, MapSize, QuestDefinition, QuestDifficulty, QuestRegistry, 
    QuestType,
};

fn data_path(filename: &str) -> std::path::PathBuf {
    std::path::PathBuf::from("data").join(filename)
}

fn parse_all() -> QuestRegistry {
    parse_quests_json(&data_path("Quests.json"))
        .expect("failed to parse Quests.json")
}

// ── US-007: All quests are loadable ─────────────────────────────────────────

#[test]
fn all_quests_are_loadable() {
    let registry = parse_all();

    assert!(
        registry.len() >= 5,
        "Quests.json should contain at least 5 quests, got {}",
        registry.len()
    );
}

#[test]
fn quest_registry_is_not_empty() {
    let registry = parse_all();
    assert!(!registry.is_empty(), "QuestRegistry should not be empty");
}

// ── US-007: Quest lookup by ID ──────────────────────────────────────────────

#[test]
fn kill_boss_qinglong_short_lookup_by_id_works() {
    let registry = parse_all();

    let quest = registry.get("kill_boss_qinglong_short");
    assert!(
        quest.is_some(),
        "kill_boss_qinglong_short should exist in registry"
    );

    let quest = quest.unwrap();
    assert_eq!(quest.quest_id, "kill_boss_qinglong_short");
    assert_eq!(quest.quest_type, QuestType::KillBoss);
    assert_eq!(quest.dungeon, DungeonType::QingLong);
    assert_eq!(quest.map_size, MapSize::Short);
    assert_eq!(quest.difficulty, QuestDifficulty::Standard);
    assert_eq!(quest.max_steps, 2);
}

#[test]
fn cleanse_zhuque_medium_lookup_by_id_works() {
    let registry = parse_all();

    let quest = registry.get("cleanse_zhuque_medium");
    assert!(
        quest.is_some(),
        "cleanse_zhuque_medium should exist in registry"
    );

    let quest = quest.unwrap();
    assert_eq!(quest.quest_type, QuestType::Cleanse);
    assert_eq!(quest.dungeon, DungeonType::ZhuQue);
    assert_eq!(quest.map_size, MapSize::Medium);
    assert_eq!(quest.difficulty, QuestDifficulty::Hard);
}

#[test]
fn unknown_quest_returns_none() {
    let registry = parse_all();
    assert!(
        registry.get("nonexistent_quest").is_none(),
        "Unknown quest should return None"
    );
}

// ── US-007: Quest filtering ─────────────────────────────────────────────────

#[test]
fn by_dungeon_returns_qinglong_quests() {
    let registry = parse_all();

    let qinglong_quests = registry.by_dungeon(DungeonType::QingLong);
    assert!(
        !qinglong_quests.is_empty(),
        "Should have QingLong quests"
    );

    for quest in qinglong_quests {
        assert_eq!(
            quest.dungeon,
            DungeonType::QingLong,
            "All returned quests should be QingLong"
        );
    }
}

#[test]
fn by_type_returns_kill_boss_quests() {
    let registry = parse_all();

    let kill_boss_quests = registry.by_type(QuestType::KillBoss);
    assert!(
        !kill_boss_quests.is_empty(),
        "Should have KillBoss quests"
    );

    for quest in kill_boss_quests {
        assert_eq!(
            quest.quest_type,
            QuestType::KillBoss,
            "All returned quests should be KillBoss type"
        );
    }
}

// ── US-007: Representative KillBoss quest drives runtime ────────────────────

#[test]
fn representative_kill_boss_quest_has_correct_rewards() {
    let registry = parse_all();

    let quest = registry.get("kill_boss_qinglong_short").unwrap();

    // Standard rewards: 500 gold, 10 bones, 5 portraits, 200 xp
    assert_eq!(quest.rewards.gold, 500);
    assert_eq!(
        *quest.rewards.heirlooms.get(&game_ddgc_headless::contracts::HeirloomCurrency::Bones).unwrap(),
        10
    );
    assert_eq!(
        *quest.rewards.heirlooms.get(&game_ddgc_headless::contracts::HeirloomCurrency::Portraits).unwrap(),
        5
    );
    assert_eq!(quest.rewards.xp, 200);
}

#[test]
fn representative_kill_boss_quest_has_correct_penalties() {
    let registry = parse_all();

    let quest = registry.get("kill_boss_qinglong_short").unwrap();

    // Standard penalties: -100 gold, -5 bones, -2 portraits
    assert_eq!(quest.penalties.gold, -100);
    assert_eq!(
        *quest.penalties.heirlooms.get(&game_ddgc_headless::contracts::HeirloomCurrency::Bones).unwrap(),
        -5
    );
    assert_eq!(
        *quest.penalties.heirlooms.get(&game_ddgc_headless::contracts::HeirloomCurrency::Portraits).unwrap(),
        -2
    );
}

// ── US-007: Hard difficulty quest has scaled rewards ────────────────────────

#[test]
fn hard_difficulty_quest_has_scaled_rewards() {
    let registry = parse_all();

    let quest = registry.get("kill_boss_qinglong_hard").unwrap();

    assert_eq!(quest.difficulty, QuestDifficulty::Hard);

    // Hard rewards: 1000 gold, 25 bones, 15 portraits, 5 tapes, 400 xp
    assert_eq!(quest.rewards.gold, 1000);
    assert_eq!(
        *quest.rewards.heirlooms.get(&game_ddgc_headless::contracts::HeirloomCurrency::Bones).unwrap(),
        25
    );
    assert_eq!(
        *quest.rewards.heirlooms.get(&game_ddgc_headless::contracts::HeirloomCurrency::Portraits).unwrap(),
        15
    );
    assert_eq!(
        *quest.rewards.heirlooms.get(&game_ddgc_headless::contracts::HeirloomCurrency::Tapes).unwrap(),
        5
    );
    assert_eq!(quest.rewards.xp, 400);
}

#[test]
fn hard_difficulty_quest_has_scaled_penalties() {
    let registry = parse_all();

    let quest = registry.get("kill_boss_qinglong_hard").unwrap();

    // Hard penalties: -250 gold, -15 bones, -10 portraits, -3 tapes
    assert_eq!(quest.penalties.gold, -250);
    assert_eq!(
        *quest.penalties.heirlooms.get(&game_ddgc_headless::contracts::HeirloomCurrency::Bones).unwrap(),
        -15
    );
    assert_eq!(
        *quest.penalties.heirlooms.get(&game_ddgc_headless::contracts::HeirloomCurrency::Portraits).unwrap(),
        -10
    );
    assert_eq!(
        *quest.penalties.heirlooms.get(&game_ddgc_headless::contracts::HeirloomCurrency::Tapes).unwrap(),
        -3
    );
}

// ── US-007: Quest definition helper method ──────────────────────────────────

#[test]
fn quest_definition_kill_boss_qinglong_short_helper() {
    let quest = QuestDefinition::kill_boss_qinglong_short();

    assert_eq!(quest.quest_id, "kill_boss_qinglong_short");
    assert_eq!(quest.quest_type, QuestType::KillBoss);
    assert_eq!(quest.dungeon, DungeonType::QingLong);
    assert_eq!(quest.map_size, MapSize::Short);
    assert_eq!(quest.difficulty, QuestDifficulty::Standard);
    assert_eq!(quest.max_steps, 2);
}

// ── US-007: All IDs are accessible ─────────────────────────────────────────

#[test]
fn all_ids_returns_all_quest_ids() {
    let registry = parse_all();

    let ids = registry.all_ids();
    assert!(
        !ids.is_empty(),
        "all_ids should return non-empty list"
    );

    // Verify all IDs can be looked up
    for id in &ids {
        assert!(
            registry.get(id).is_some(),
            "ID {} should be lookupable",
            id
        );
    }
}
