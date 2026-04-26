//! Campaign snapshot and persistence schema integration tests (US-001-d).
//!
//! Validates that the `CampaignState` persistence schema round-trips correctly
//! through the full save/load boundary exposed by `GameState`. Covers:
//!
//! - Full campaign save/load round-trip with all gameplay-significant fields
//! - Per-domain field preservation (gold, heirlooms, roster, inventory, town,
//!   run history, quest progress)
//! - Hero substructure preservation (quirks, traits, skills, equipment)
//! - Deterministic serialization (identical state → identical file bytes)
//! - Schema version validation and rejection of unsupported versions
//! - Error handling for malformed JSON, missing files, and invalid paths
//! - Multiple save/load cycle integrity
//! - Empty/fresh campaign initialization
//! - Canonical JSON structure verification
//!
//! These tests live in the integration test suite (`tests/`) and exercise the
//! public API of the `game_ddgc_headless` crate, satisfying the "scoped to the
//! tests module" acceptance criterion for US-001-d.

use game_ddgc_headless::contracts::{
    BuildingUpgradeState, CampaignHero, CampaignInventoryItem, CampaignQuestProgress,
    CampaignRunRecord, CampaignState, DungeonType, HeirloomCurrency, MapSize,
    CAMPAIGN_SNAPSHOT_VERSION,
};
use game_ddgc_headless::state::GameState;

use std::path::{Path, PathBuf};

// ───────────────────────────────────────────────────────────────────
// Test helpers
// ───────────────────────────────────────────────────────────────────

/// Build a fully-populated `CampaignState` exercising every field in the schema.
///
/// This is the canonical test fixture for the save/load boundary integration
/// tests. It populates all seven top-level domains and every hero subdomain so
/// that a single round-trip can prove no gameplay-significant field is lost.
fn build_full_campaign() -> CampaignState {
    let mut state = CampaignState::new(1500);

    // Heirlooms: all three currency types
    state.heirlooms.insert(HeirloomCurrency::Bones, 42);
    state.heirlooms.insert(HeirloomCurrency::Portraits, 15);
    state.heirlooms.insert(HeirloomCurrency::Tapes, 7);

    // Town: three buildings at different upgrade levels
    state.building_states.insert(
        "inn".to_string(),
        BuildingUpgradeState::new("inn", Some('b')),
    );
    state.building_states.insert(
        "blacksmith".to_string(),
        BuildingUpgradeState::new("blacksmith", Some('a')),
    );
    state.building_states.insert(
        "abbey".to_string(),
        BuildingUpgradeState::new("abbey", None),
    );

    // Roster: two heroes with full sub-state
    let mut hero1 = CampaignHero::new("hero_1", "alchemist", 3, 450, 85.0, 100.0, 25.0, 200.0);
    hero1.quirks.positive = vec!["eagle_eye".to_string(), "tough".to_string()];
    hero1.quirks.negative = vec!["kleptomaniac".to_string()];
    hero1.quirks.diseases = vec!["consumption".to_string()];
    hero1.traits.virtues = vec!["courageous".to_string()];
    hero1.traits.afflictions = vec!["paranoid".to_string()];
    hero1.skills = vec![
        "skill_fire_bomb".to_string(),
        "skill_acid_spray".to_string(),
        "skill_healing_vapor".to_string(),
        "skill_toxin_grenade".to_string(),
    ];
    hero1.equipment.weapon_level = 2;
    hero1.equipment.armor_level = 1;
    hero1.equipment.trinkets = vec!["sage_stone".to_string(), "lucky_charm".to_string()];
    state.roster.push(hero1);

    let hero2 = CampaignHero::new("hero_2", "hunter", 2, 200, 100.0, 100.0, 10.0, 200.0);
    state.roster.push(hero2);

    // Inventory: three item stacks
    state.inventory.push(CampaignInventoryItem::new("torch", 4));
    state.inventory.push(CampaignInventoryItem::new("shovel", 1));
    state.inventory.push(CampaignInventoryItem::new("bandage", 3));

    // Run history: two runs (one completed, one abandoned)
    state.run_history.push(CampaignRunRecord::new(
        DungeonType::QingLong,
        MapSize::Short,
        9, 3, true, 350,
    ));
    state.run_history.push(CampaignRunRecord::new(
        DungeonType::BaiHu,
        MapSize::Medium,
        10, 2, false, 125,
    ));

    // Quests: one in-progress
    let mut q = CampaignQuestProgress::new("kill_boss_qinglong", 2);
    q.current_step = 1;
    state.quest_progress.push(q);

    state
}

/// Create a temporary file path for campaign save testing.
fn temp_save_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("ddgc_test_campaign_{}.json", name))
}

/// Load GameState from the real data directory using CARGO_MANIFEST_DIR.
fn load_state() -> GameState {
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set during test");
    let data_dir = PathBuf::from(manifest_dir).join("data");
    GameState::load_from(&data_dir).expect("failed to load game state from data dir")
}

// ───────────────────────────────────────────────────────────────────
// Full round-trip: all domains via GameState save/load
// ───────────────────────────────────────────────────────────────────

#[test]
fn full_campaign_save_load_roundtrip_via_gamestate() {
    let mut state = load_state();
    state.new_campaign(1500);

    // Populate campaign with full state
    let mut hero = CampaignHero::new("hero_1", "alchemist", 3, 450, 85.0, 100.0, 25.0, 200.0);
    hero.quirks.positive = vec!["eagle_eye".to_string()];
    hero.quirks.negative = vec!["kleptomaniac".to_string()];
    hero.equipment.weapon_level = 2;
    hero.equipment.armor_level = 1;
    hero.equipment.trinkets = vec!["sage_stone".to_string()];
    state.campaign.roster.push(hero);
    state.campaign.heirlooms.insert(HeirloomCurrency::Bones, 42);
    state.campaign.heirlooms.insert(HeirloomCurrency::Portraits, 15);
    state.campaign.building_states.insert(
        "inn".to_string(),
        BuildingUpgradeState::new("inn", Some('b')),
    );
    state.campaign.inventory.push(CampaignInventoryItem::new("torch", 4));
    state.campaign.inventory.push(CampaignInventoryItem::new("shovel", 1));
    state.campaign.run_history.push(CampaignRunRecord::new(
        DungeonType::QingLong, MapSize::Short,
        9, 3, true, 350,
    ));
    state.campaign.quest_progress.push({
        let mut q = CampaignQuestProgress::new("kill_boss_qinglong", 2);
        q.current_step = 1;
        q
    });

    let save_path = temp_save_path("gamestate_full_roundtrip");
    state.save_campaign(&save_path).expect("save_campaign should succeed");

    // Load into a fresh GameState
    let mut state2 = load_state();
    state2.load_campaign(&save_path).expect("load_campaign should succeed");
    std::fs::remove_file(&save_path).ok();

    // Verify all top-level domains
    assert_eq!(state2.campaign.schema_version, CAMPAIGN_SNAPSHOT_VERSION);
    assert_eq!(state2.campaign.gold, 1500);

    // Heirlooms
    assert_eq!(state2.campaign.heirlooms.len(), 2);
    assert_eq!(state2.campaign.heirlooms[&HeirloomCurrency::Bones], 42);
    assert_eq!(state2.campaign.heirlooms[&HeirloomCurrency::Portraits], 15);

    // Town
    assert_eq!(state2.campaign.building_states.len(), 1);
    assert_eq!(state2.campaign.building_states["inn"].current_level, Some('b'));

    // Roster
    assert_eq!(state2.campaign.roster.len(), 1);
    let h = &state2.campaign.roster[0];
    assert_eq!(h.id, "hero_1");
    assert_eq!(h.class_id, "alchemist");
    assert_eq!(h.level, 3);
    assert_eq!(h.xp, 450);
    assert_eq!(h.health, 85.0);
    assert_eq!(h.max_health, 100.0);
    assert_eq!(h.stress, 25.0);
    assert_eq!(h.max_stress, 200.0);
    assert_eq!(h.quirks.positive, vec!["eagle_eye"]);
    assert_eq!(h.quirks.negative, vec!["kleptomaniac"]);
    assert_eq!(h.equipment.weapon_level, 2);
    assert_eq!(h.equipment.armor_level, 1);
    assert_eq!(h.equipment.trinkets, vec!["sage_stone"]);

    // Inventory
    assert_eq!(state2.campaign.inventory.len(), 2);

    // Run history
    assert_eq!(state2.campaign.run_history.len(), 1);
    assert_eq!(state2.campaign.run_history[0].dungeon, DungeonType::QingLong);

    // Quests
    assert_eq!(state2.campaign.quest_progress.len(), 1);
    assert_eq!(state2.campaign.quest_progress[0].quest_id, "kill_boss_qinglong");
}

#[test]
fn full_campaign_identity_roundtrip() {
    let original = build_full_campaign();
    let json = original.to_json().expect("serialization must succeed");
    let restored = CampaignState::from_json(&json).expect("deserialization must succeed");
    assert_eq!(original, restored, "round-trip must produce identical CampaignState");
}

// ───────────────────────────────────────────────────────────────────
// Per-domain field preservation tests
// ───────────────────────────────────────────────────────────────────

#[test]
fn gold_roundtrip_preserves_value() {
    for gold in [0, 100, 1500, 9999u32] {
        let state = CampaignState::new(gold);
        let json = state.to_json().unwrap();
        let restored = CampaignState::from_json(&json).unwrap();
        assert_eq!(restored.gold, gold, "gold should be {} after round-trip", gold);
    }
}

#[test]
fn heirlooms_roundtrip_preserves_all_currencies() {
    let mut state = CampaignState::new(100);
    state.heirlooms.insert(HeirloomCurrency::Bones, 99);
    state.heirlooms.insert(HeirloomCurrency::Tapes, 33);

    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();

    assert_eq!(restored.heirlooms.get(&HeirloomCurrency::Bones), Some(&99));
    assert_eq!(restored.heirlooms.get(&HeirloomCurrency::Tapes), Some(&33));
    assert_eq!(restored.heirlooms.get(&HeirloomCurrency::Portraits), None);
}

#[test]
fn building_states_roundtrip_preserves_upgrade_levels() {
    let mut state = CampaignState::new(100);
    state.building_states.insert(
        "inn".to_string(),
        BuildingUpgradeState::new("inn", Some('b')),
    );
    state.building_states.insert(
        "blacksmith".to_string(),
        BuildingUpgradeState::new("blacksmith", Some('a')),
    );
    state.building_states.insert(
        "abbey".to_string(),
        BuildingUpgradeState::new("abbey", None),
    );

    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();

    assert_eq!(restored.building_states.len(), 3);
    assert_eq!(restored.building_states["inn"].building_id, "inn");
    assert_eq!(restored.building_states["inn"].current_level, Some('b'));
    assert_eq!(restored.building_states["blacksmith"].building_id, "blacksmith");
    assert_eq!(restored.building_states["blacksmith"].current_level, Some('a'));
    assert_eq!(restored.building_states["abbey"].building_id, "abbey");
    assert_eq!(restored.building_states["abbey"].current_level, None);
}

#[test]
fn roster_roundtrip_preserves_multiple_heroes() {
    let mut state = CampaignState::new(100);
    state.roster.push(CampaignHero::new("h1", "alchemist", 3, 450, 85.0, 100.0, 25.0, 200.0));
    state.roster.push(CampaignHero::new("h2", "crusader", 1, 0, 100.0, 100.0, 0.0, 200.0));
    state.roster.push(CampaignHero::new("h3", "hunter", 5, 2000, 90.0, 110.0, 10.0, 220.0));

    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();

    assert_eq!(restored.roster.len(), 3);
    assert_eq!(restored.roster[0].id, "h1");
    assert_eq!(restored.roster[0].level, 3);
    assert_eq!(restored.roster[1].id, "h2");
    assert_eq!(restored.roster[1].level, 1);
    assert_eq!(restored.roster[2].id, "h3");
    assert_eq!(restored.roster[2].level, 5);
}

#[test]
fn inventory_roundtrip_preserves_id_quantity_and_order() {
    let mut state = CampaignState::new(100);
    state.inventory.push(CampaignInventoryItem::new("torch", 8));
    state.inventory.push(CampaignInventoryItem::new("bandage", 4));
    state.inventory.push(CampaignInventoryItem::new("shovel", 1));
    state.inventory.push(CampaignInventoryItem::new("key", 2));

    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();

    assert_eq!(restored.inventory.len(), 4);
    assert_eq!(restored.inventory[0].id, "torch");
    assert_eq!(restored.inventory[0].quantity, 8);
    assert_eq!(restored.inventory[1].id, "bandage");
    assert_eq!(restored.inventory[1].quantity, 4);
    assert_eq!(restored.inventory[3].id, "key");
    assert_eq!(restored.inventory[3].quantity, 2);
}

#[test]
fn run_history_roundtrip_preserves_all_dungeon_types() {
    let mut state = CampaignState::new(100);

    // Each dungeon type with both map sizes, both completed and abandoned
    state.run_history.push(CampaignRunRecord::new(
        DungeonType::QingLong, MapSize::Short, 8, 4, true, 300,
    ));
    state.run_history.push(CampaignRunRecord::new(
        DungeonType::BaiHu, MapSize::Medium, 12, 3, true, 500,
    ));
    state.run_history.push(CampaignRunRecord::new(
        DungeonType::ZhuQue, MapSize::Short, 3, 1, false, 50,
    ));
    state.run_history.push(CampaignRunRecord::new(
        DungeonType::XuanWu, MapSize::Medium, 0, 0, false, 0,
    ));

    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();

    assert_eq!(restored.run_history.len(), 4);

    // QingLong Short, completed
    let r0 = &restored.run_history[0];
    assert_eq!(r0.dungeon, DungeonType::QingLong);
    assert_eq!(r0.map_size, MapSize::Short);
    assert_eq!(r0.rooms_cleared, 8);
    assert_eq!(r0.battles_won, 4);
    assert!(r0.completed);
    assert_eq!(r0.gold_earned, 300);

    // XuanWu Medium, 0 rooms cleared, abandoned
    let r3 = &restored.run_history[3];
    assert_eq!(r3.dungeon, DungeonType::XuanWu);
    assert_eq!(r3.map_size, MapSize::Medium);
    assert_eq!(r3.rooms_cleared, 0);
    assert_eq!(r3.battles_won, 0);
    assert!(!r3.completed);
    assert_eq!(r3.gold_earned, 0);
}

#[test]
fn quest_progress_roundtrip_preserves_all_states() {
    let mut state = CampaignState::new(100);

    // In-progress quest
    let mut q0 = CampaignQuestProgress::new("cleanse_all_dungeons", 4);
    q0.current_step = 2;
    state.quest_progress.push(q0);

    // Completed quest
    let mut q1 = CampaignQuestProgress::new("collect_heirlooms", 3);
    q1.current_step = 3;
    q1.completed = true;
    state.quest_progress.push(q1);

    // Not-yet-started quest
    state.quest_progress.push(CampaignQuestProgress::new("kill_boss_qinglong", 2));

    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();

    assert_eq!(restored.quest_progress.len(), 3);

    let qp0 = &restored.quest_progress[0];
    assert_eq!(qp0.quest_id, "cleanse_all_dungeons");
    assert_eq!(qp0.current_step, 2);
    assert_eq!(qp0.max_steps, 4);
    assert!(!qp0.completed);

    let qp1 = &restored.quest_progress[1];
    assert_eq!(qp1.quest_id, "collect_heirlooms");
    assert_eq!(qp1.current_step, 3);
    assert_eq!(qp1.max_steps, 3);
    assert!(qp1.completed);

    let qp2 = &restored.quest_progress[2];
    assert_eq!(qp2.quest_id, "kill_boss_qinglong");
    assert_eq!(qp2.current_step, 0);
    assert_eq!(qp2.max_steps, 2);
    assert!(!qp2.completed);
}

// ───────────────────────────────────────────────────────────────────
// Hero substructure round-trip tests
// ───────────────────────────────────────────────────────────────────

#[test]
fn hero_quirks_roundtrip_preserves_categories_and_negative_count() {
    let mut hero = CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0);
    hero.quirks.positive = vec!["eagle_eye".to_string(), "tough".to_string()];
    hero.quirks.negative = vec!["fearful".to_string(), "kleptomaniac".to_string()];
    hero.quirks.diseases = vec!["rabies".to_string(), "consumption".to_string()];

    let mut state = CampaignState::new(500);
    state.roster.push(hero);

    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    let h = &restored.roster[0];

    assert_eq!(h.quirks.positive, vec!["eagle_eye", "tough"]);
    assert_eq!(h.quirks.negative, vec!["fearful", "kleptomaniac"]);
    assert_eq!(h.quirks.diseases, vec!["rabies", "consumption"]);
    assert_eq!(h.quirks.negative_count(), 4); // 2 negative + 2 diseases
}

#[test]
fn hero_quirks_empty_categories_roundtrip() {
    let hero = CampaignHero::new("h1", "hunter", 1, 0, 100.0, 100.0, 0.0, 200.0);
    let mut state = CampaignState::new(500);
    state.roster.push(hero);

    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    let h = &restored.roster[0];

    assert!(h.quirks.positive.is_empty());
    assert!(h.quirks.negative.is_empty());
    assert!(h.quirks.diseases.is_empty());
    assert_eq!(h.quirks.negative_count(), 0);
}

#[test]
fn hero_traits_roundtrip_preserves_afflictions_and_virtues() {
    let mut hero = CampaignHero::new("h1", "crusader", 1, 0, 100.0, 100.0, 0.0, 200.0);
    hero.traits.afflictions = vec!["paranoid".to_string(), "fearful".to_string()];
    hero.traits.virtues = vec!["courageous".to_string(), "stalwart".to_string()];

    let mut state = CampaignState::new(500);
    state.roster.push(hero);

    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    let h = &restored.roster[0];

    assert_eq!(h.traits.afflictions, vec!["paranoid", "fearful"]);
    assert_eq!(h.traits.virtues, vec!["courageous", "stalwart"]);
}

#[test]
fn hero_skills_roundtrip_preserves_full_four_skill_order() {
    let mut hero = CampaignHero::new("h1", "shaman", 1, 0, 100.0, 100.0, 0.0, 200.0);
    hero.skills = vec![
        "skill_lightning".to_string(),
        "skill_hex".to_string(),
        "skill_totem".to_string(),
        "skill_heal".to_string(),
    ];

    let mut state = CampaignState::new(500);
    state.roster.push(hero);

    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();

    let h = &restored.roster[0];
    assert_eq!(h.skills.len(), 4);
    assert_eq!(h.skills[0], "skill_lightning");
    assert_eq!(h.skills[1], "skill_hex");
    assert_eq!(h.skills[2], "skill_totem");
    assert_eq!(h.skills[3], "skill_heal");
}

#[test]
fn hero_skills_empty_roster_roundtrip() {
    let hero = CampaignHero::new("h1", "crusader", 1, 0, 100.0, 100.0, 0.0, 200.0);
    let mut state = CampaignState::new(500);
    state.roster.push(hero);

    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();

    assert!(restored.roster[0].skills.is_empty());
}

#[test]
fn hero_equipment_roundtrip_preserves_all_levels_and_trinkets() {
    let mut hero = CampaignHero::new("h1", "tank", 1, 0, 100.0, 100.0, 0.0, 200.0);
    hero.equipment.weapon_level = 4;
    hero.equipment.armor_level = 3;
    hero.equipment.trinkets = vec![
        "shield_medallion".to_string(),
        "sun_ring".to_string(),
    ];

    let mut state = CampaignState::new(500);
    state.roster.push(hero);

    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    let h = &restored.roster[0];

    assert_eq!(h.equipment.weapon_level, 4);
    assert_eq!(h.equipment.armor_level, 3);
    assert_eq!(h.equipment.trinkets.len(), 2);
    assert_eq!(h.equipment.trinkets[0], "shield_medallion");
    assert_eq!(h.equipment.trinkets[1], "sun_ring");
}

#[test]
fn hero_equipment_default_levels_roundtrip() {
    let hero = CampaignHero::new("h1", "jester", 1, 0, 100.0, 100.0, 0.0, 200.0);
    let mut state = CampaignState::new(500);
    state.roster.push(hero);

    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    let h = &restored.roster[0];

    assert_eq!(h.equipment.weapon_level, 0);
    assert_eq!(h.equipment.armor_level, 0);
    assert!(h.equipment.trinkets.is_empty());
}

// ───────────────────────────────────────────────────────────────────
// Deterministic serialization tests
// ───────────────────────────────────────────────────────────────────

#[test]
fn identical_state_produces_identical_json_bytes() {
    let campaign = build_full_campaign();
    let json_a = campaign.to_json().unwrap();
    let json_b = campaign.to_json().unwrap();
    assert_eq!(
        json_a, json_b,
        "identical CampaignState must produce identical JSON bytes"
    );
}

#[test]
fn identical_state_produces_identical_save_files() {
    let campaign = build_full_campaign();
    let path_a = temp_save_path("det_a");
    let path_b = temp_save_path("det_b");

    // Use GameState to write through the same pipeline
    let mut state = load_state();
    state.campaign = campaign.clone();
    state.save_campaign(&path_a).unwrap();
    state.campaign = campaign.clone();
    state.save_campaign(&path_b).unwrap();

    let bytes_a = std::fs::read(&path_a).unwrap();
    let bytes_b = std::fs::read(&path_b).unwrap();
    std::fs::remove_file(&path_a).ok();
    std::fs::remove_file(&path_b).ok();

    assert_eq!(
        bytes_a, bytes_b,
        "identical state must produce identical save file bytes"
    );
}

#[test]
fn btree_map_heirlooms_produce_sorted_json_keys() {
    let mut state = CampaignState::new(100);
    state.heirlooms.insert(HeirloomCurrency::Bones, 10);
    state.heirlooms.insert(HeirloomCurrency::Portraits, 20);
    state.heirlooms.insert(HeirloomCurrency::Tapes, 30);

    let json = state.to_json().unwrap();
    let bones_pos = json.find("Bones").unwrap();
    let portraits_pos = json.find("Portraits").unwrap();
    let tapes_pos = json.find("Tapes").unwrap();

    assert!(
        bones_pos < portraits_pos,
        "Bones must appear before Portraits in deterministic JSON"
    );
    assert!(
        portraits_pos < tapes_pos,
        "Portraits must appear before Tapes in deterministic JSON"
    );
}

#[test]
fn btree_map_building_states_produce_sorted_json_keys() {
    let mut state = CampaignState::new(100);
    state.building_states.insert(
        "tavern".to_string(),
        BuildingUpgradeState::new("tavern", Some('c')),
    );
    state.building_states.insert(
        "abbey".to_string(),
        BuildingUpgradeState::new("abbey", Some('a')),
    );
    state.building_states.insert(
        "guild".to_string(),
        BuildingUpgradeState::new("guild", Some('b')),
    );

    let json = state.to_json().unwrap();
    let abbey_pos = json.find("abbey").unwrap();
    let guild_pos = json.find("guild").unwrap();
    let tavern_pos = json.find("tavern").unwrap();

    assert!(abbey_pos < guild_pos, "abbey must appear before guild in sorted JSON");
    assert!(guild_pos < tavern_pos, "guild must appear before tavern in sorted JSON");
}

#[test]
fn deterministic_serialization_survives_roundtrip_cycles() {
    let campaign = build_full_campaign();
    let json_original = campaign.to_json().unwrap();

    // Serialize → deserialize → serialize → deserialize three times
    let mut current = campaign;
    for cycle in 0..3 {
        let json = current.to_json().unwrap();
        current = CampaignState::from_json(&json).unwrap();
        // After each full cycle, serialization should match the original
        let re_json = current.to_json().unwrap();
        assert_eq!(
            json_original, re_json,
            "cycle {}: serialization must match original after deserialize+serialize",
            cycle + 1
        );
    }
}

// ───────────────────────────────────────────────────────────────────
// Schema versioning tests
// ───────────────────────────────────────────────────────────────────

#[test]
fn new_campaign_uses_current_schema_version() {
    let campaign = CampaignState::new(500);
    assert_eq!(campaign.schema_version, CAMPAIGN_SNAPSHOT_VERSION);
    assert!(campaign.validate_version().is_ok());
}

#[test]
fn validate_version_rejects_unsupported_version() {
    let mut campaign = CampaignState::new(500);
    campaign.schema_version = 99;
    let result = campaign.validate_version();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("unsupported campaign schema version"));
}

#[test]
fn validate_version_rejects_zero_version() {
    let mut campaign = CampaignState::new(500);
    campaign.schema_version = 0;
    assert!(campaign.validate_version().is_err());
}

#[test]
fn validate_version_rejects_future_version() {
    let mut campaign = CampaignState::new(500);
    campaign.schema_version = CAMPAIGN_SNAPSHOT_VERSION + 10;
    assert!(campaign.validate_version().is_err());
}

#[test]
fn game_state_validates_schema_on_load() {
    let mut campaign = CampaignState::new(500);
    campaign.schema_version = 99;
    let save_path = temp_save_path("bad_version");
    std::fs::write(&save_path, campaign.to_json().unwrap()).unwrap();

    let mut state = load_state();
    let result = state.load_campaign(&save_path);
    std::fs::remove_file(&save_path).ok();

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("unsupported"));
}

#[test]
fn game_state_validate_campaign_delegates_to_schema_version() {
    let mut state = load_state();

    state.campaign = CampaignState::new(100);
    assert!(state.validate_campaign().is_ok());

    state.campaign.schema_version = 42;
    assert!(state.validate_campaign().is_err());
}

// ───────────────────────────────────────────────────────────────────
// Error handling tests
// ───────────────────────────────────────────────────────────────────

#[test]
fn save_campaign_fails_on_invalid_path() {
    let state = load_state();
    let result = state.save_campaign(Path::new("/nonexistent/directory/campaign.json"));
    assert!(result.is_err());
}

#[test]
fn load_campaign_fails_on_missing_file() {
    let mut state = load_state();
    let result = state.load_campaign(Path::new("/nonexistent/campaign.json"));
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        err.contains("cannot read")
            || err.contains("not found")
            || err.contains("No such file"),
        "unexpected error message: {}",
        err
    );
}

#[test]
fn load_campaign_fails_on_malformed_json() {
    let save_path = temp_save_path("bad_json");
    std::fs::write(&save_path, "this is not valid json {{{{{").unwrap();

    let mut state = load_state();
    let result = state.load_campaign(&save_path);
    std::fs::remove_file(&save_path).ok();

    assert!(result.is_err());
}

#[test]
fn load_campaign_fails_on_json_with_wrong_schema() {
    // Valid JSON but wrong shape (array instead of object)
    let save_path = temp_save_path("wrong_schema");
    std::fs::write(&save_path, "[1, 2, 3]").unwrap();

    let mut state = load_state();
    let result = state.load_campaign(&save_path);
    std::fs::remove_file(&save_path).ok();

    assert!(result.is_err());
}

#[test]
fn load_campaign_fails_on_json_missing_schema_version() {
    let save_path = temp_save_path("no_version");
    std::fs::write(&save_path, r#"{"gold": 100}"#).unwrap();

    let mut state = load_state();
    let result = state.load_campaign(&save_path);
    std::fs::remove_file(&save_path).ok();

    // Missing required field should fail deserialization
    assert!(result.is_err());
}

#[test]
fn from_json_fails_on_empty_string() {
    let result = CampaignState::from_json("");
    assert!(result.is_err());
}

#[test]
fn from_json_fails_on_null() {
    let result = CampaignState::from_json("null");
    assert!(result.is_err());
}

// ───────────────────────────────────────────────────────────────────
// Empty / fresh campaign tests
// ───────────────────────────────────────────────────────────────────

#[test]
fn empty_campaign_save_load_roundtrip() {
    let campaign = CampaignState::new(0);
    let json = campaign.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();

    assert_eq!(restored.gold, 0);
    assert_eq!(restored.schema_version, CAMPAIGN_SNAPSHOT_VERSION);
    assert!(restored.roster.is_empty());
    assert!(restored.heirlooms.is_empty());
    assert!(restored.building_states.is_empty());
    assert!(restored.inventory.is_empty());
    assert!(restored.run_history.is_empty());
    assert!(restored.quest_progress.is_empty());
}

#[test]
fn fresh_campaign_initializes_all_collections_empty() {
    let campaign = CampaignState::new(250);
    assert_eq!(campaign.gold, 250);
    assert_eq!(campaign.schema_version, CAMPAIGN_SNAPSHOT_VERSION);
    assert!(campaign.heirlooms.is_empty());
    assert!(campaign.building_states.is_empty());
    assert!(campaign.roster.is_empty());
    assert!(campaign.inventory.is_empty());
    assert!(campaign.run_history.is_empty());
    assert!(campaign.quest_progress.is_empty());
}

#[test]
fn game_state_new_campaign_replaces_existing() {
    let mut state = load_state();
    state.new_campaign(100);
    state.campaign.roster.push(
        CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0),
    );
    state.campaign.inventory.push(CampaignInventoryItem::new("torch", 5));

    // Replace campaign
    state.new_campaign(500);
    assert_eq!(state.campaign.gold, 500);
    assert_eq!(state.campaign.schema_version, CAMPAIGN_SNAPSHOT_VERSION);
    assert!(state.campaign.roster.is_empty());
    assert!(state.campaign.inventory.is_empty());
    // Content datasets must be preserved
    assert_eq!(state.camping_skill_count(), 87);
}

#[test]
fn game_state_new_campaign_preserves_content_datasets() {
    let mut state = load_state();
    assert_eq!(state.camping_skill_count(), 87);

    state.new_campaign(300);
    assert_eq!(state.campaign.gold, 300);
    assert_eq!(state.campaign.schema_version, CAMPAIGN_SNAPSHOT_VERSION);
    assert_eq!(state.camping_skill_count(), 87);
}

// ───────────────────────────────────────────────────────────────────
// Multiple save/load cycle tests
// ───────────────────────────────────────────────────────────────────

#[test]
fn multiple_save_load_cycles_preserve_state_integrity() {
    let mut state = load_state();
    state.new_campaign(1000);
    state.campaign.inventory.push(CampaignInventoryItem::new("torch", 5));
    state.campaign.heirlooms.insert(HeirloomCurrency::Bones, 20);

    let save_path = temp_save_path("multi_cycle");

    // Cycle 1: save → load
    state.save_campaign(&save_path).unwrap();
    state.load_campaign(&save_path).unwrap();
    assert_eq!(state.campaign.gold, 1000);
    assert_eq!(state.campaign.inventory[0].quantity, 5);
    assert_eq!(state.campaign.heirlooms[&HeirloomCurrency::Bones], 20);

    // Cycle 2: modify → save → load
    state.campaign.gold = 2000;
    state.campaign.inventory.push(CampaignInventoryItem::new("shovel", 2));
    state.campaign.heirlooms.insert(HeirloomCurrency::Portraits, 10);
    state.save_campaign(&save_path).unwrap();
    state.load_campaign(&save_path).unwrap();
    assert_eq!(state.campaign.gold, 2000);
    assert_eq!(state.campaign.inventory.len(), 2);
    assert_eq!(state.campaign.heirlooms.len(), 2);

    // Cycle 3: modify → save → load
    state.campaign.roster.push(
        CampaignHero::new("h1", "alchemist", 3, 450, 85.0, 100.0, 25.0, 200.0),
    );
    state.save_campaign(&save_path).unwrap();
    state.load_campaign(&save_path).unwrap();
    assert_eq!(state.campaign.gold, 2000);
    assert_eq!(state.campaign.roster.len(), 1);
    assert_eq!(state.campaign.roster[0].id, "h1");

    std::fs::remove_file(&save_path).ok();
}

#[test]
fn multiple_cycles_keep_json_structure_identical() {
    let campaign = build_full_campaign();
    let save_path = temp_save_path("multi_cycle_structure");

    let mut state = load_state();
    state.campaign = campaign.clone();
    state.save_campaign(&save_path).unwrap();
    let first_bytes = std::fs::read(&save_path).unwrap();

    // Load → save again without modification
    state.load_campaign(&save_path).unwrap();
    state.save_campaign(&save_path).unwrap();
    let second_bytes = std::fs::read(&save_path).unwrap();

    std::fs::remove_file(&save_path).ok();

    assert_eq!(
        first_bytes, second_bytes,
        "load-then-save without modification must produce identical file bytes"
    );
}

// ───────────────────────────────────────────────────────────────────
// Canonical JSON structure verification
// ───────────────────────────────────────────────────────────────────

#[test]
fn campaign_json_is_valid_and_has_expected_top_level_keys() {
    let state = build_full_campaign();
    let json = state.to_json().unwrap();
    let parsed: serde_json::Value =
        serde_json::from_str(&json).expect("campaign JSON must be valid JSON");

    assert!(parsed.is_object(), "top-level JSON must be an object");
    assert_eq!(parsed["schema_version"], CAMPAIGN_SNAPSHOT_VERSION);
    assert_eq!(parsed["gold"], 1500);
    assert!(parsed["heirlooms"].is_object(), "heirlooms must be a JSON object");
    assert!(parsed["building_states"].is_object(), "building_states must be a JSON object");
    assert!(parsed["roster"].is_array(), "roster must be a JSON array");
    assert!(parsed["inventory"].is_array(), "inventory must be a JSON array");
    assert!(parsed["run_history"].is_array(), "run_history must be a JSON array");
    assert!(parsed["quest_progress"].is_array(), "quest_progress must be a JSON array");
}

#[test]
fn campaign_json_hero_substructure_is_valid() {
    let mut hero = CampaignHero::new("hero_1", "alchemist", 3, 450, 85.0, 100.0, 25.0, 200.0);
    hero.quirks.positive = vec!["eagle_eye".to_string()];
    hero.skills = vec!["skill_fire_bomb".to_string()];
    hero.equipment.trinkets = vec!["sage_stone".to_string()];

    let mut state = CampaignState::new(500);
    state.roster.push(hero);

    let json = state.to_json().unwrap();
    let parsed: serde_json::Value =
        serde_json::from_str(&json).expect("campaign JSON must be valid JSON");

    let heroes = &parsed["roster"];
    assert!(heroes.is_array());
    let h = &heroes[0];
    assert_eq!(h["id"], "hero_1");
    assert_eq!(h["class_id"], "alchemist");
    assert_eq!(h["level"], 3);
    assert!(h["quirks"].is_object());
    assert!(h["quirks"]["positive"].is_array());
    assert!(h["quirks"]["negative"].is_array());
    assert!(h["quirks"]["diseases"].is_array());
    assert!(h["traits"].is_object());
    assert!(h["skills"].is_array());
    assert!(h["equipment"].is_object());
    assert_eq!(h["equipment"]["weapon_level"], 0);
    assert_eq!(h["equipment"]["armor_level"], 0);
    assert!(h["equipment"]["trinkets"].is_array());
}

#[test]
fn campaign_json_schema_version_always_first_key() {
    let state = CampaignState::new(100);
    let json = state.to_json().unwrap();

    // schema_version should appear early in the JSON output
    // (Rust struct serialization produces keys in field declaration order)
    let ver_pos = json.find("schema_version").unwrap();
    let gold_pos = json.find("gold").unwrap();
    assert!(
        ver_pos < gold_pos,
        "schema_version must appear before gold in struct-order JSON"
    );
}

#[test]
fn heirloom_currency_serializes_as_pascal_case() {
    let mut state = CampaignState::new(100);
    state.heirlooms.insert(HeirloomCurrency::Bones, 1);

    let json = state.to_json().unwrap();
    assert!(json.contains("Bones"), "HeirloomCurrency must serialize as PascalCase");
    assert!(
        !json.contains("bones"),
        "HeirloomCurrency must NOT serialize as lowercase"
    );
}

// ───────────────────────────────────────────────────────────────────
// Round-trip integrity: no field loss for full campaign
// ───────────────────────────────────────────────────────────────────

#[test]
fn full_campaign_roundtrip_preserves_all_gameplay_fields() {
    let original = build_full_campaign();
    let json = original.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();

    // Schema version
    assert_eq!(restored.schema_version, CAMPAIGN_SNAPSHOT_VERSION);

    // Gold
    assert_eq!(restored.gold, 1500);

    // Heirlooms: all three currencies
    assert_eq!(restored.heirlooms.len(), 3);
    assert_eq!(restored.heirlooms[&HeirloomCurrency::Bones], 42);
    assert_eq!(restored.heirlooms[&HeirloomCurrency::Portraits], 15);
    assert_eq!(restored.heirlooms[&HeirloomCurrency::Tapes], 7);

    // Town: three buildings
    assert_eq!(restored.building_states.len(), 3);
    assert_eq!(restored.building_states["inn"].current_level, Some('b'));
    assert_eq!(restored.building_states["blacksmith"].current_level, Some('a'));
    assert_eq!(restored.building_states["abbey"].current_level, None);

    // Roster: two heroes
    assert_eq!(restored.roster.len(), 2);

    let h1 = &restored.roster[0];
    assert_eq!(h1.id, "hero_1");
    assert_eq!(h1.class_id, "alchemist");
    assert_eq!(h1.level, 3);
    assert_eq!(h1.xp, 450);
    assert!((h1.health - 85.0).abs() < f64::EPSILON);
    assert!((h1.max_health - 100.0).abs() < f64::EPSILON);
    assert!((h1.stress - 25.0).abs() < f64::EPSILON);
    assert!((h1.max_stress - 200.0).abs() < f64::EPSILON);
    assert_eq!(h1.quirks.positive.len(), 2);
    assert_eq!(h1.quirks.negative.len(), 1);
    assert_eq!(h1.quirks.diseases.len(), 1);
    assert_eq!(h1.traits.virtues.len(), 1);
    assert_eq!(h1.traits.afflictions.len(), 1);
    assert_eq!(h1.skills.len(), 4);
    assert_eq!(h1.skills[0], "skill_fire_bomb");
    assert_eq!(h1.skills[3], "skill_toxin_grenade");
    assert_eq!(h1.equipment.weapon_level, 2);
    assert_eq!(h1.equipment.armor_level, 1);
    assert_eq!(h1.equipment.trinkets.len(), 2);
    assert_eq!(h1.equipment.trinkets[0], "sage_stone");

    let h2 = &restored.roster[1];
    assert_eq!(h2.id, "hero_2");
    assert_eq!(h2.class_id, "hunter");
    assert_eq!(h2.level, 2);

    // Inventory: three items
    assert_eq!(restored.inventory.len(), 3);
    assert_eq!(restored.inventory[0].id, "torch");
    assert_eq!(restored.inventory[0].quantity, 4);
    assert_eq!(restored.inventory[1].id, "shovel");
    assert_eq!(restored.inventory[1].quantity, 1);
    assert_eq!(restored.inventory[2].id, "bandage");
    assert_eq!(restored.inventory[2].quantity, 3);

    // Run history: two runs
    assert_eq!(restored.run_history.len(), 2);
    assert_eq!(restored.run_history[0].dungeon, DungeonType::QingLong);
    assert_eq!(restored.run_history[0].map_size, MapSize::Short);
    assert_eq!(restored.run_history[0].rooms_cleared, 9);
    assert_eq!(restored.run_history[0].battles_won, 3);
    assert!(restored.run_history[0].completed);
    assert_eq!(restored.run_history[0].gold_earned, 350);
    assert_eq!(restored.run_history[1].dungeon, DungeonType::BaiHu);
    assert!(!restored.run_history[1].completed);

    // Quests: one in-progress
    assert_eq!(restored.quest_progress.len(), 1);
    assert_eq!(restored.quest_progress[0].quest_id, "kill_boss_qinglong");
    assert_eq!(restored.quest_progress[0].current_step, 1);
    assert_eq!(restored.quest_progress[0].max_steps, 2);
    assert!(!restored.quest_progress[0].completed);
}

// ───────────────────────────────────────────────────────────────────
// Save file integrity: round-trip through actual GameState pipeline
// ───────────────────────────────────────────────────────────────────

#[test]
fn game_state_save_and_load_campaign_preserves_all_fields() {
    let mut state = load_state();
    state.new_campaign(1500);

    let mut hero = CampaignHero::new("hero_1", "alchemist", 3, 450, 85.0, 100.0, 25.0, 200.0);
    hero.quirks.positive = vec!["eagle_eye".to_string(), "tough".to_string()];
    hero.quirks.negative = vec!["kleptomaniac".to_string()];
    hero.quirks.diseases = vec!["consumption".to_string()];
    hero.traits.virtues = vec!["courageous".to_string()];
    hero.skills = vec![
        "skill_fire_bomb".to_string(),
        "skill_acid_spray".to_string(),
    ];
    hero.equipment.weapon_level = 2;
    hero.equipment.armor_level = 1;
    hero.equipment.trinkets = vec!["sage_stone".to_string()];
    state.campaign.roster.push(hero);

    state.campaign.heirlooms.insert(HeirloomCurrency::Bones, 42);
    state.campaign.heirlooms.insert(HeirloomCurrency::Portraits, 15);
    state.campaign.building_states.insert(
        "inn".to_string(),
        BuildingUpgradeState::new("inn", Some('b')),
    );
    state.campaign.inventory.push(CampaignInventoryItem::new("torch", 4));
    state.campaign.inventory.push(CampaignInventoryItem::new("shovel", 1));
    state.campaign.run_history.push(CampaignRunRecord::new(
        DungeonType::QingLong, MapSize::Short,
        9, 3, true, 350,
    ));
    let mut q = CampaignQuestProgress::new("kill_boss_qinglong", 2);
    q.current_step = 1;
    state.campaign.quest_progress.push(q);

    let save_path = temp_save_path("gamestate_all_fields");
    state.save_campaign(&save_path).unwrap();

    let mut state2 = load_state();
    state2.load_campaign(&save_path).unwrap();
    std::fs::remove_file(&save_path).ok();

    let c = &state2.campaign;

    // Top-level
    assert_eq!(c.schema_version, CAMPAIGN_SNAPSHOT_VERSION);
    assert_eq!(c.gold, 1500);

    // Heirlooms
    assert_eq!(c.heirlooms.len(), 2);
    assert_eq!(c.heirlooms[&HeirloomCurrency::Bones], 42);
    assert_eq!(c.heirlooms[&HeirloomCurrency::Portraits], 15);

    // Buildings
    assert_eq!(c.building_states.len(), 1);
    assert_eq!(c.building_states["inn"].current_level, Some('b'));

    // Roster hero sub-state
    assert_eq!(c.roster.len(), 1);
    let h = &c.roster[0];
    assert_eq!(h.id, "hero_1");
    assert_eq!(h.class_id, "alchemist");
    assert_eq!(h.level, 3);
    assert_eq!(h.xp, 450);
    assert_eq!(h.health, 85.0);
    assert_eq!(h.stress, 25.0);
    assert_eq!(h.quirks.positive.len(), 2);
    assert_eq!(h.quirks.negative.len(), 1);
    assert_eq!(h.quirks.diseases.len(), 1);
    assert_eq!(h.quirks.negative_count(), 2);
    assert_eq!(h.traits.virtues, vec!["courageous"]);
    assert_eq!(h.skills.len(), 2);
    assert_eq!(h.skills[0], "skill_fire_bomb");
    assert_eq!(h.equipment.weapon_level, 2);
    assert_eq!(h.equipment.armor_level, 1);
    assert_eq!(h.equipment.trinkets, vec!["sage_stone"]);

    // Inventory
    assert_eq!(c.inventory.len(), 2);

    // Run history
    assert_eq!(c.run_history.len(), 1);
    assert_eq!(c.run_history[0].dungeon, DungeonType::QingLong);
    assert_eq!(c.run_history[0].rooms_cleared, 9);
    assert!(c.run_history[0].completed);

    // Quests
    assert_eq!(c.quest_progress.len(), 1);
    assert_eq!(c.quest_progress[0].quest_id, "kill_boss_qinglong");
    assert_eq!(c.quest_progress[0].current_step, 1);
}
