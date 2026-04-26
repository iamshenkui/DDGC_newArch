//! Campaign state transition integration tests (US-004-b).
//!
//! Validates that the explicit campaign state transition methods correctly mutate
//! the persistent `CampaignState` rather than only transient run state. Covers:
//!
//! - Dungeon reward application (gold, heirlooms, XP) via explicit transitions
//! - Inventory changes (consumption and looting) via explicit transitions
//! - Hero vitals sync from dungeon runs to persistent campaign state
//! - Roster management (add/remove heroes) via explicit transitions
//! - Equipment upgrades (weapon, armor, trinkets) via explicit transitions
//! - Town activity application (stress heal, health heal, gold spent)
//! - Dungeon run recording for run history tracking
//! - Three-loop continuity test proving hero/estate/inventory persistence
//! - Edge cases: overflow saturation, inventory depletion, vitals clamping
//!
//! These tests live in the integration test suite (`tests/`) and exercise the
//! public API of the `game_ddgc_headless` crate, satisfying the "scoped to the
//! tests module" acceptance criterion for US-004-b.

use game_ddgc_headless::contracts::{
    CampaignHero, CampaignInventoryItem, CampaignState, DungeonType,
    HeirloomCurrency, MapSize,
};
use game_ddgc_headless::state::GameState;

use std::collections::BTreeMap;
use std::path::PathBuf;

// ───────────────────────────────────────────────────────────────────
// Test helpers
// ───────────────────────────────────────────────────────────────────

/// Load GameState from the real data directory using CARGO_MANIFEST_DIR.
fn load_state() -> GameState {
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set during test");
    let data_dir = PathBuf::from(manifest_dir).join("data");
    GameState::load_from(&data_dir).expect("failed to load game state from data dir")
}

/// Create a temporary file path for campaign save testing.
fn temp_save_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("ddgc_test_transitions_{}.json", name))
}

// ───────────────────────────────────────────────────────────────────
// Dungeon reward transitions: gold, heirlooms, XP
// ───────────────────────────────────────────────────────────────────

#[test]
fn apply_dungeon_gold_increases_campaign_gold() {
    let mut campaign = CampaignState::new(500);
    campaign.apply_dungeon_gold(350);
    assert_eq!(campaign.gold, 850);
}

#[test]
fn apply_dungeon_gold_saturates_on_overflow() {
    let mut campaign = CampaignState::new(u32::MAX);
    campaign.apply_dungeon_gold(100);
    assert_eq!(campaign.gold, u32::MAX);
}

#[test]
fn apply_dungeon_gold_does_nothing_for_zero() {
    let mut campaign = CampaignState::new(500);
    campaign.apply_dungeon_gold(0);
    assert_eq!(campaign.gold, 500);
}

#[test]
fn apply_dungeon_heirlooms_adds_currencies() {
    let mut campaign = CampaignState::new(0);
    let mut heirlooms = BTreeMap::new();
    heirlooms.insert(HeirloomCurrency::Bones, 15);
    heirlooms.insert(HeirloomCurrency::Portraits, 5);
    campaign.apply_dungeon_heirlooms(&heirlooms);

    assert_eq!(campaign.heirlooms[&HeirloomCurrency::Bones], 15);
    assert_eq!(campaign.heirlooms[&HeirloomCurrency::Portraits], 5);
}

#[test]
fn apply_dungeon_heirlooms_accumulates_existing() {
    let mut campaign = CampaignState::new(0);
    campaign.heirlooms.insert(HeirloomCurrency::Bones, 10);

    let mut heirlooms = BTreeMap::new();
    heirlooms.insert(HeirloomCurrency::Bones, 15);
    campaign.apply_dungeon_heirlooms(&heirlooms);

    assert_eq!(campaign.heirlooms[&HeirloomCurrency::Bones], 25);
}

#[test]
fn apply_dungeon_heirlooms_saturates_on_overflow() {
    let mut campaign = CampaignState::new(0);
    campaign.heirlooms.insert(HeirloomCurrency::Bones, u32::MAX - 5);
    let mut heirlooms = BTreeMap::new();
    heirlooms.insert(HeirloomCurrency::Bones, 10);
    campaign.apply_dungeon_heirlooms(&heirlooms);
    assert_eq!(campaign.heirlooms[&HeirloomCurrency::Bones], u32::MAX);
}

#[test]
fn apply_dungeon_xp_distributes_evenly_to_roster() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));
    campaign.roster.push(CampaignHero::new("h2", "hunter", 1, 0, 100.0, 100.0, 0.0, 200.0));
    campaign.roster.push(CampaignHero::new("h3", "crusader", 1, 0, 100.0, 100.0, 0.0, 200.0));

    campaign.apply_dungeon_xp(300);

    // 300 XP / 3 heroes = 100 XP each
    assert_eq!(campaign.roster[0].xp, 100);
    assert_eq!(campaign.roster[1].xp, 100);
    assert_eq!(campaign.roster[2].xp, 100);
}

#[test]
fn apply_dungeon_xp_ignores_empty_roster() {
    let mut campaign = CampaignState::new(0);
    campaign.apply_dungeon_xp(100);
    // Should not panic, just do nothing
    assert_eq!(campaign.roster.len(), 0);
}

#[test]
fn apply_dungeon_xp_rounds_down_odd_xp() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));
    campaign.roster.push(CampaignHero::new("h2", "hunter", 1, 0, 100.0, 100.0, 0.0, 200.0));

    campaign.apply_dungeon_xp(101);

    // 101 / 2 = 50 (integer division truncates)
    assert_eq!(campaign.roster[0].xp, 50);
    assert_eq!(campaign.roster[1].xp, 50);
}

// ───────────────────────────────────────────────────────────────────
// Inventory transitions
// ───────────────────────────────────────────────────────────────────

#[test]
fn apply_inventory_change_adds_new_item() {
    let mut campaign = CampaignState::new(0);
    campaign.apply_inventory_change("torch", 5);
    assert_eq!(campaign.inventory.len(), 1);
    assert_eq!(campaign.inventory[0].id, "torch");
    assert_eq!(campaign.inventory[0].quantity, 5);
}

#[test]
fn apply_inventory_change_accumulates_existing() {
    let mut campaign = CampaignState::new(0);
    campaign.inventory.push(CampaignInventoryItem::new("torch", 3));
    campaign.apply_inventory_change("torch", 5);
    assert_eq!(campaign.inventory.len(), 1);
    assert_eq!(campaign.inventory[0].quantity, 8);
}

#[test]
fn apply_inventory_change_consumes_item() {
    let mut campaign = CampaignState::new(0);
    campaign.inventory.push(CampaignInventoryItem::new("torch", 5));
    campaign.apply_inventory_change("torch", -3);
    assert_eq!(campaign.inventory.len(), 1);
    assert_eq!(campaign.inventory[0].quantity, 2);
}

#[test]
fn apply_inventory_change_removes_item_when_depleted() {
    let mut campaign = CampaignState::new(0);
    campaign.inventory.push(CampaignInventoryItem::new("torch", 3));
    campaign.apply_inventory_change("torch", -5);
    assert!(!campaign.inventory.iter().any(|i| i.id == "torch"));
}

#[test]
fn apply_inventory_change_ignores_zero() {
    let mut campaign = CampaignState::new(0);
    campaign.inventory.push(CampaignInventoryItem::new("torch", 5));
    campaign.apply_inventory_change("torch", 0);
    assert_eq!(campaign.inventory[0].quantity, 5);
}

#[test]
fn apply_inventory_change_does_not_add_negative_quantity() {
    let mut campaign = CampaignState::new(0);
    campaign.apply_inventory_change("torch", -5);
    assert!(campaign.inventory.is_empty());
}

// ───────────────────────────────────────────────────────────────────
// Hero vitals transitions
// ───────────────────────────────────────────────────────────────────

#[test]
fn sync_hero_vitals_updates_health_and_stress() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 20.0, 200.0));
    campaign.sync_hero_vitals("h1", 75.0, 45.0);

    assert_eq!(campaign.roster[0].health, 75.0);
    assert_eq!(campaign.roster[0].stress, 45.0);
}

#[test]
fn sync_hero_vitals_clamps_health_to_max() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));
    campaign.sync_hero_vitals("h1", 150.0, 0.0);

    assert_eq!(campaign.roster[0].health, 100.0); // clamped to max
}

#[test]
fn sync_hero_vitals_clamps_stress_to_max() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));
    campaign.sync_hero_vitals("h1", 50.0, 250.0);

    assert_eq!(campaign.roster[0].stress, 200.0); // clamped to max
}

#[test]
fn sync_hero_vitals_clamps_to_zero() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 50.0, 200.0));
    campaign.sync_hero_vitals("h1", -10.0, -5.0);

    assert_eq!(campaign.roster[0].health, 0.0);
    assert_eq!(campaign.roster[0].stress, 0.0);
}

#[test]
fn sync_hero_vitals_ignores_nonexistent_hero() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));
    // Should not panic
    campaign.sync_hero_vitals("nonexistent", 50.0, 50.0);
    assert_eq!(campaign.roster[0].health, 100.0); // unchanged
}

// ───────────────────────────────────────────────────────────────────
// Roster management transitions
// ───────────────────────────────────────────────────────────────────

#[test]
fn add_hero_append_to_roster() {
    let mut campaign = CampaignState::new(0);
    let hero = CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0);
    campaign.add_hero(hero);

    assert_eq!(campaign.roster.len(), 1);
    assert_eq!(campaign.roster[0].id, "h1");
}

#[test]
fn add_hero_assigns_id_if_empty() {
    let mut campaign = CampaignState::new(0);
    let hero = CampaignHero::new("", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0);
    campaign.add_hero(hero);

    assert!(!campaign.roster[0].id.is_empty());
    assert!(campaign.roster[0].id.starts_with("hero_"));
}

#[test]
fn add_hero_multiple_assigns_unique_ids() {
    let mut campaign = CampaignState::new(0);
    let hero1 = CampaignHero::new("", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0);
    let hero2 = CampaignHero::new("", "hunter", 1, 0, 100.0, 100.0, 0.0, 200.0);
    let hero3 = CampaignHero::new("", "crusader", 1, 0, 100.0, 100.0, 0.0, 200.0);
    campaign.add_hero(hero1);
    campaign.add_hero(hero2);
    campaign.add_hero(hero3);

    assert_eq!(campaign.roster.len(), 3);
    assert_ne!(campaign.roster[0].id, campaign.roster[1].id);
    assert_ne!(campaign.roster[1].id, campaign.roster[2].id);
}

#[test]
fn remove_hero_returns_and_removes_hero() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));
    campaign.roster.push(CampaignHero::new("h2", "hunter", 1, 0, 100.0, 100.0, 0.0, 200.0));

    let removed = campaign.remove_hero("h1");

    assert!(removed.is_some());
    assert_eq!(removed.unwrap().id, "h1");
    assert_eq!(campaign.roster.len(), 1);
    assert_eq!(campaign.roster[0].id, "h2");
}

#[test]
fn remove_hero_returns_none_for_nonexistent() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));

    let removed = campaign.remove_hero("nonexistent");

    assert!(removed.is_none());
    assert_eq!(campaign.roster.len(), 1);
}

// ───────────────────────────────────────────────────────────────────
// Equipment upgrade transitions
// ───────────────────────────────────────────────────────────────────

#[test]
fn upgrade_hero_weapon_increments_weapon_level() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));
    assert_eq!(campaign.roster[0].equipment.weapon_level, 0);

    campaign.upgrade_hero_weapon("h1");

    assert_eq!(campaign.roster[0].equipment.weapon_level, 1);
}

#[test]
fn upgrade_hero_weapon_ignores_nonexistent_hero() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));
    campaign.upgrade_hero_weapon("nonexistent");
    assert_eq!(campaign.roster[0].equipment.weapon_level, 0);
}

#[test]
fn upgrade_hero_armor_increments_armor_level() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));
    assert_eq!(campaign.roster[0].equipment.armor_level, 0);

    campaign.upgrade_hero_armor("h1");

    assert_eq!(campaign.roster[0].equipment.armor_level, 1);
}

#[test]
fn upgrade_hero_armor_ignores_nonexistent_hero() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));
    campaign.upgrade_hero_armor("nonexistent");
    assert_eq!(campaign.roster[0].equipment.armor_level, 0);
}

#[test]
fn equip_trinket_adds_trinket_to_hero() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));
    assert!(campaign.roster[0].equipment.trinkets.is_empty());

    campaign.equip_trinket("h1", "sage_stone");

    assert_eq!(campaign.roster[0].equipment.trinkets.len(), 1);
    assert_eq!(campaign.roster[0].equipment.trinkets[0], "sage_stone");
}

#[test]
fn equip_trinket_does_not_duplicate() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));
    campaign.equip_trinket("h1", "sage_stone");
    campaign.equip_trinket("h1", "sage_stone");

    assert_eq!(campaign.roster[0].equipment.trinkets.len(), 1);
}

#[test]
fn equip_trinket_ignores_nonexistent_hero() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));
    campaign.equip_trinket("nonexistent", "sage_stone");
    assert!(campaign.roster[0].equipment.trinkets.is_empty());
}

// ───────────────────────────────────────────────────────────────────
// Town activity transitions
// ───────────────────────────────────────────────────────────────────

#[test]
fn apply_town_stress_heal_reduces_stress() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 50.0, 200.0));
    campaign.apply_town_stress_heal("h1", 20.0);
    assert_eq!(campaign.roster[0].stress, 30.0);
}

#[test]
fn apply_town_stress_heal_clamps_to_zero() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 15.0, 200.0));
    campaign.apply_town_stress_heal("h1", 20.0);
    assert_eq!(campaign.roster[0].stress, 0.0);
}

#[test]
fn apply_town_health_heal_increases_health() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 80.0, 100.0, 0.0, 200.0));
    campaign.apply_town_health_heal("h1", 15.0);
    assert_eq!(campaign.roster[0].health, 95.0);
}

#[test]
fn apply_town_health_heal_clamps_to_max() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 95.0, 100.0, 0.0, 200.0));
    campaign.apply_town_health_heal("h1", 10.0);
    assert_eq!(campaign.roster[0].health, 100.0);
}

#[test]
fn apply_town_gold_spent_deducts_gold() {
    let mut campaign = CampaignState::new(500);
    campaign.apply_town_gold_spent(100);
    assert_eq!(campaign.gold, 400);
}

#[test]
fn apply_town_gold_spent_saturates_at_zero() {
    let mut campaign = CampaignState::new(50);
    campaign.apply_town_gold_spent(100);
    assert_eq!(campaign.gold, 0);
}

// ───────────────────────────────────────────────────────────────────
// Dungeon run recording
// ───────────────────────────────────────────────────────────────────

#[test]
fn record_dungeon_run_append_to_history() {
    let mut campaign = CampaignState::new(0);
    campaign.record_dungeon_run(DungeonType::QingLong, MapSize::Short, 9, 3, true, 350);
    assert_eq!(campaign.run_history.len(), 1);
    assert_eq!(campaign.run_history[0].dungeon, DungeonType::QingLong);
    assert_eq!(campaign.run_history[0].map_size, MapSize::Short);
    assert_eq!(campaign.run_history[0].rooms_cleared, 9);
    assert_eq!(campaign.run_history[0].battles_won, 3);
    assert!(campaign.run_history[0].completed);
    assert_eq!(campaign.run_history[0].gold_earned, 350);
}

#[test]
fn record_dungeon_run_multiple_entries() {
    let mut campaign = CampaignState::new(0);
    campaign.record_dungeon_run(DungeonType::QingLong, MapSize::Short, 9, 3, true, 350);
    campaign.record_dungeon_run(DungeonType::BaiHu, MapSize::Medium, 12, 4, true, 500);
    assert_eq!(campaign.run_history.len(), 2);
    assert_eq!(campaign.run_history[0].dungeon, DungeonType::QingLong);
    assert_eq!(campaign.run_history[1].dungeon, DungeonType::BaiHu);
}

// ───────────────────────────────────────────────────────────────────
// State transitions persist across save/load
// ───────────────────────────────────────────────────────────────────

#[test]
fn dungeon_rewards_persist_across_save_load() {
    let save_path = temp_save_path("dungeon_rewards");

    let mut state = load_state();
    state.new_campaign(500);

    // Apply dungeon rewards via state transitions
    state.campaign.apply_dungeon_gold(350);
    let mut heirlooms = BTreeMap::new();
    heirlooms.insert(HeirloomCurrency::Bones, 15);
    heirlooms.insert(HeirloomCurrency::Portraits, 5);
    state.campaign.apply_dungeon_heirlooms(&heirlooms);
    state.campaign.apply_dungeon_xp(150);

    state.save_campaign(&save_path).unwrap();

    // Load and verify
    let mut state = load_state();
    state.load_campaign(&save_path).unwrap();
    std::fs::remove_file(&save_path).ok();

    assert_eq!(state.campaign.gold, 850);
    assert_eq!(state.campaign.heirlooms[&HeirloomCurrency::Bones], 15);
    assert_eq!(state.campaign.heirlooms[&HeirloomCurrency::Portraits], 5);
}

#[test]
fn hero_vitals_persist_across_save_load() {
    let save_path = temp_save_path("hero_vitals");

    let mut state = load_state();
    state.new_campaign(0);
    state.campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 50.0, 200.0));

    // Sync vitals after dungeon run
    state.campaign.sync_hero_vitals("h1", 75.0, 45.0);
    state.save_campaign(&save_path).unwrap();

    // Load and verify
    let mut state = load_state();
    state.load_campaign(&save_path).unwrap();
    std::fs::remove_file(&save_path).ok();

    assert_eq!(state.campaign.roster[0].health, 75.0);
    assert_eq!(state.campaign.roster[0].stress, 45.0);
}

#[test]
fn equipment_upgrades_persist_across_save_load() {
    let save_path = temp_save_path("equipment_upgrades");

    let mut state = load_state();
    state.new_campaign(0);
    state.campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));

    // Upgrade equipment
    state.campaign.upgrade_hero_weapon("h1");
    state.campaign.upgrade_hero_weapon("h1");
    state.campaign.upgrade_hero_armor("h1");
    state.campaign.equip_trinket("h1", "sage_stone");
    state.save_campaign(&save_path).unwrap();

    // Load and verify
    let mut state = load_state();
    state.load_campaign(&save_path).unwrap();
    std::fs::remove_file(&save_path).ok();

    assert_eq!(state.campaign.roster[0].equipment.weapon_level, 2);
    assert_eq!(state.campaign.roster[0].equipment.armor_level, 1);
    assert_eq!(state.campaign.roster[0].equipment.trinkets, vec!["sage_stone"]);
}

#[test]
fn roster_changes_persist_across_save_load() {
    let save_path = temp_save_path("roster_changes");

    let mut state = load_state();
    state.new_campaign(0);
    state.campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));

    // Remove hero
    let removed = state.campaign.remove_hero("h1");
    assert!(removed.is_some());
    state.save_campaign(&save_path).unwrap();

    // Load and verify
    let mut state = load_state();
    state.load_campaign(&save_path).unwrap();
    std::fs::remove_file(&save_path).ok();

    assert!(state.campaign.roster.is_empty());
}

#[test]
fn town_activity_deducts_gold_and_heals_stress() {
    let save_path = temp_save_path("town_activity");

    let mut state = load_state();
    state.new_campaign(500);
    state.campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 80.0, 100.0, 60.0, 200.0));

    // Apply town stress heal at Abbey (gold spent)
    state.campaign.apply_town_gold_spent(100);
    state.campaign.apply_town_stress_heal("h1", 25.0);
    state.save_campaign(&save_path).unwrap();

    // Load and verify
    let mut state = load_state();
    state.load_campaign(&save_path).unwrap();
    std::fs::remove_file(&save_path).ok();

    assert_eq!(state.campaign.gold, 400);
    assert_eq!(state.campaign.roster[0].stress, 35.0);
}

// ───────────────────────────────────────────────────────────────────
// Three-loop continuity test
// ───────────────────────────────────────────────────────────────────

#[test]
fn three_loop_seeded_scenario_preserves_hero_estate_inventory_continuity() {
    // US-004 acceptance test: proves that a three-loop seeded scenario
    // preserves hero/estate/inventory continuity when dungeon rewards and
    // town activities are applied through explicit campaign state transitions.
    //
    // Loop 1: Fresh campaign, start with 2 heroes, 500 gold, basic inventory
    // Loop 2: After dungeon rewards (gold, XP, heirlooms) and town stress heal
    // Loop 3: After equipment upgrades, more dungeon rewards, roster growth

    let save_path = temp_save_path("three_loop_continuity");

    // ── Loop 1: Fresh campaign ────────────────────────────────────────────
    let mut state = load_state();
    state.new_campaign(500);

    // Add initial heroes
    let mut hero1 = CampaignHero::new("hero_1", "crusader", 1, 0, 100.0, 100.0, 20.0, 200.0);
    hero1.skills = vec!["skill_stab".to_string(), "skill_inspire".to_string()];
    hero1.equipment.weapon_level = 1;
    hero1.equipment.armor_level = 1;
    hero1.quirks.positive = vec!["eagle_eye".to_string()];
    state.campaign.roster.push(hero1);

    let mut hero2 = CampaignHero::new("hero_2", "alchemist", 1, 0, 100.0, 100.0, 15.0, 200.0);
    hero2.skills = vec!["skill_fire_bomb".to_string()];
    hero2.equipment.weapon_level = 0;
    hero2.equipment.armor_level = 1;
    state.campaign.roster.push(hero2);

    // Initial inventory: torches, bandages, shovel
    state.campaign.inventory.push(CampaignInventoryItem::new("torch", 8));
    state.campaign.inventory.push(CampaignInventoryItem::new("bandage", 4));
    state.campaign.inventory.push(CampaignInventoryItem::new("shovel", 1));

    // Initial heirlooms
    state.campaign.heirlooms.insert(HeirloomCurrency::Bones, 10);

    state.save_campaign(&save_path).unwrap();

    // ── Loop 2: Load, apply dungeon rewards, apply town activity ──────────
    let mut state = load_state();
    state.load_campaign(&save_path).unwrap();

    // Verify initial state
    assert_eq!(state.campaign.gold, 500);
    assert_eq!(state.campaign.roster.len(), 2);
    assert_eq!(state.campaign.roster[0].id, "hero_1");
    assert_eq!(state.campaign.roster[1].id, "hero_2");
    assert_eq!(state.campaign.inventory.len(), 3);
    assert_eq!(state.campaign.heirlooms[&HeirloomCurrency::Bones], 10);

    // Simulate dungeon run 1: QingLong short, won, earned rewards
    // Apply dungeon rewards via explicit state transitions
    state.campaign.apply_dungeon_gold(350);
    state.campaign.apply_dungeon_xp(150);

    // Apply some heirlooms from dungeon
    let mut heirlooms = BTreeMap::new();
    heirlooms.insert(HeirloomCurrency::Bones, 15);
    heirlooms.insert(HeirloomCurrency::Portraits, 5);
    state.campaign.apply_dungeon_heirlooms(&heirlooms);

    // Consume torches from dungeon
    state.campaign.apply_inventory_change("torch", -3);

    // Add loot found
    state.campaign.apply_inventory_change("antiquarian_teacup", 1);

    // Update hero vitals from dungeon run
    state.campaign.sync_hero_vitals("hero_1", 75.0, 45.0);
    state.campaign.sync_hero_vitals("hero_2", 80.0, 35.0);

    // Record the run
    state.campaign.record_dungeon_run(
        DungeonType::QingLong,
        MapSize::Short,
        9,
        3,
        true,
        350,
    );

    // Apply town activity: stress heal at Abbey (gold spent reduces campaign gold)
    state.campaign.apply_town_gold_spent(100);
    state.campaign.apply_town_stress_heal("hero_1", 20.0);
    state.campaign.apply_town_stress_heal("hero_2", 15.0);

    state.save_campaign(&save_path).unwrap();

    // ── Loop 3: Load, more dungeon rewards, equipment upgrade, recruit ─────
    let mut state = load_state();
    state.load_campaign(&save_path).unwrap();

    // Verify loop 2 state preserved
    assert_eq!(state.campaign.gold, 750); // 500 + 350 - 100
    assert_eq!(state.campaign.roster.len(), 2);
    assert_eq!(state.campaign.roster[0].xp, 75); // XP distributed: 150/2 = 75 each
    assert_eq!(state.campaign.roster[1].xp, 75);
    assert_eq!(state.campaign.heirlooms[&HeirloomCurrency::Bones], 25); // 10 + 15
    assert_eq!(state.campaign.heirlooms[&HeirloomCurrency::Portraits], 5);
    assert_eq!(state.campaign.inventory.len(), 4);
    assert_eq!(state.campaign.inventory[0].quantity, 5); // torches: 8-3=5
    assert_eq!(state.campaign.run_history.len(), 1);

    // Hero vitals after town stress heal
    assert_eq!(state.campaign.roster[0].stress, 25.0); // 45 - 20
    assert_eq!(state.campaign.roster[1].stress, 20.0); // 35 - 15

    // Simulate dungeon run 2: BaiHu medium, won, earned more rewards
    state.campaign.apply_dungeon_gold(500);
    state.campaign.apply_dungeon_xp(200);

    // More heirlooms
    let mut heirlooms2 = BTreeMap::new();
    heirlooms2.insert(HeirloomCurrency::Bones, 20);
    heirlooms2.insert(HeirloomCurrency::Tapes, 3);
    state.campaign.apply_dungeon_heirlooms(&heirlooms2);

    // Consume more torches
    state.campaign.apply_inventory_change("torch", -2);

    // Add more loot
    state.campaign.apply_inventory_change("sacred_chalice", 1);

    // Upgrade hero 1's weapon
    state.campaign.upgrade_hero_weapon("hero_1");
    // Upgrade hero 2's armor
    state.campaign.upgrade_hero_armor("hero_2");

    // Record the run
    state.campaign.record_dungeon_run(
        DungeonType::BaiHu,
        MapSize::Medium,
        12,
        4,
        true,
        500,
    );

    // Town activity: recruit a new hero
    state.campaign.apply_town_gold_spent(500);
    let new_hero = CampaignHero::new("hero_3", "hunter", 1, 0, 100.0, 100.0, 0.0, 200.0);
    state.campaign.add_hero(new_hero);

    state.save_campaign(&save_path).unwrap();

    // ── Loop 4: Final verification ────────────────────────────────────────
    let mut state = load_state();
    state.load_campaign(&save_path).unwrap();

    // Gold: 750 + 500 - 500 = 750
    assert_eq!(state.campaign.gold, 750, "Gold should reflect all earnings and spending");

    // Roster should have 3 heroes
    assert_eq!(state.campaign.roster.len(), 3, "Should have recruited hero_3");
    assert_eq!(state.campaign.roster[0].id, "hero_1");
    assert_eq!(state.campaign.roster[1].id, "hero_2");
    assert_eq!(state.campaign.roster[2].id, "hero_3");

    // hero_1: weapon upgraded, XP accumulated
    assert_eq!(state.campaign.roster[0].equipment.weapon_level, 2);
    assert_eq!(state.campaign.roster[0].xp, 175); // 75 + 100 from second dungeon run

    // hero_2: armor upgraded
    assert_eq!(state.campaign.roster[1].equipment.armor_level, 2);

    // Run history should have 2 entries
    assert_eq!(state.campaign.run_history.len(), 2);
    assert_eq!(state.campaign.run_history[0].dungeon, DungeonType::QingLong);
    assert_eq!(state.campaign.run_history[1].dungeon, DungeonType::BaiHu);

    // Heirlooms accumulated
    assert_eq!(state.campaign.heirlooms[&HeirloomCurrency::Bones], 45); // 25 + 20
    assert_eq!(state.campaign.heirlooms[&HeirloomCurrency::Portraits], 5);
    assert_eq!(state.campaign.heirlooms[&HeirloomCurrency::Tapes], 3);

    // Inventory: torches 8-3-2=3, plus loot
    let torch_qty = state
        .campaign
        .inventory
        .iter()
        .find(|i| i.id == "torch")
        .map(|i| i.quantity)
        .unwrap_or(0);
    assert_eq!(torch_qty, 3, "Torches should be consumed across runs");

    // Should have 5 inventory items: torch, bandage, shovel, antiquarian_teacup, sacred_chalice
    assert_eq!(
        state.campaign.inventory.len(),
        5,
        "Inventory should include dungeon loot"
    );

    std::fs::remove_file(&save_path).ok();
}

// ───────────────────────────────────────────────────────────────────
// Edge cases
// ───────────────────────────────────────────────────────────────────

#[test]
fn apply_dungeon_xp_works_on_single_hero_roster() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 50, 100.0, 100.0, 0.0, 200.0));
    campaign.apply_dungeon_xp(100);
    assert_eq!(campaign.roster[0].xp, 150);
}

#[test]
fn sync_hero_vitals_does_not_crash_on_empty_roster() {
    let mut campaign = CampaignState::new(0);
    // Should not panic
    campaign.sync_hero_vitals("h1", 50.0, 50.0);
}

#[test]
fn remove_hero_from_single_hero_roster_leaves_empty() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));
    let removed = campaign.remove_hero("h1");
    assert!(removed.is_some());
    assert!(campaign.roster.is_empty());
}

#[test]
fn town_health_heal_works_with_zero_roster() {
    let mut campaign = CampaignState::new(0);
    // Should not panic
    campaign.apply_town_health_heal("h1", 20.0);
}

#[test]
fn town_stress_heal_works_with_zero_roster() {
    let mut campaign = CampaignState::new(0);
    // Should not panic
    campaign.apply_town_stress_heal("h1", 20.0);
}

#[test]
fn record_dungeon_run_does_not_panic_on_empty_roster() {
    let mut campaign = CampaignState::new(0);
    // Should not panic - roster can be empty for a failed expedition
    campaign.record_dungeon_run(DungeonType::QingLong, MapSize::Short, 0, 0, false, 0);
    assert_eq!(campaign.run_history.len(), 1);
}

#[test]
fn inventory_change_handles_empty_campaign() {
    let mut campaign = CampaignState::new(0);
    // Should not panic
    campaign.apply_inventory_change("new_item", 1);
    assert_eq!(campaign.inventory.len(), 1);
}

#[test]
fn equipment_upgrades_work_multiple_times() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));
    campaign.upgrade_hero_weapon("h1");
    campaign.upgrade_hero_weapon("h1");
    campaign.upgrade_hero_weapon("h1");
    assert_eq!(campaign.roster[0].equipment.weapon_level, 3);
}

#[test]
fn trinket_equip_multiple_different_trinkets() {
    let mut campaign = CampaignState::new(0);
    campaign.roster.push(CampaignHero::new("h1", "alchemist", 1, 0, 100.0, 100.0, 0.0, 200.0));
    campaign.equip_trinket("h1", "trinket_a");
    campaign.equip_trinket("h1", "trinket_b");
    campaign.equip_trinket("h1", "trinket_c");
    assert_eq!(campaign.roster[0].equipment.trinkets.len(), 3);
}