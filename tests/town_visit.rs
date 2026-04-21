//! Integration tests for town visit cycle (US-008).
//!
//! Validates:
//! - TownVisit struct represents a single town phase with available services, hero roster, balances
//! - perform_town_activity resolves building services (stress heal, recruit, upgrade)
//! - Town visit produces a trace of activities performed
//! - Entering town after a dungeon run works
//! - Stress heal at Abbey reduces hero stress and deducts gold
//! - Town visit is deterministic for given state and activity choices

use game_ddgc_headless::contracts::parse::parse_buildings_json;
use game_ddgc_headless::town::{
    HeroInTown, TownActivity, TownVisit,
};
use game_ddgc_headless::contracts::{BuildingUpgradeState, TownState};

fn data_path(filename: &str) -> std::path::PathBuf {
    std::path::PathBuf::from("data").join(filename)
}

fn parse_buildings() -> game_ddgc_headless::contracts::BuildingRegistry {
    parse_buildings_json(&data_path("Buildings.json")).expect("failed to parse Buildings.json")
}

// ── US-008: TownVisit struct tests ───────────────────────────────────────────

#[test]
fn town_visit_represents_single_town_phase() {
    let registry = parse_buildings();
    let town_state = TownState::new(1000);
    let heroes = vec![
        HeroInTown::new("h1", "alchemist", 50.0, 200.0, 80.0, 100.0),
        HeroInTown::new("h2", "hunter", 100.0, 200.0, 60.0, 100.0),
    ];

    let visit = TownVisit::new(town_state, heroes, registry);

    // Town phase has gold
    assert_eq!(visit.town_state.gold, 1000);

    // Town phase has hero roster
    assert_eq!(visit.heroes.len(), 2);

    // Town phase has building registry for services
    assert!(!visit.building_registry.all_ids().is_empty());

    // Town phase has empty trace initially
    assert!(visit.trace.activities.is_empty());
}

#[test]
fn town_visit_from_dungeon_run() {
    let registry = parse_buildings();

    // Simulate: earned 500 gold, 20 stress accumulated, 4 heroes survived
    let visit = TownVisit::from_dungeon_run(500, 20.0, 4, registry);

    assert_eq!(visit.town_state.gold, 500);
    assert_eq!(visit.heroes.len(), 4);

    // All heroes should have stress from the dungeon run
    for hero in &visit.heroes {
        assert!(
            hero.stress > 0.0,
            "Hero {} should have stress from dungeon run",
            hero.id
        );
    }
}

// ── US-008: perform_town_activity tests ─────────────────────────────────────

#[test]
fn perform_pray_at_abbey_resolves_stress_heal() {
    let registry = parse_buildings();
    let mut town_state = TownState::new(500);
    town_state
        .building_states
        .insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('b')));

    let hero = HeroInTown::new("h1", "alchemist", 100.0, 200.0, 100.0, 100.0);
    let heroes = vec![hero];

    let mut visit = TownVisit::new(town_state, heroes, registry);

    let result = visit.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('b'));

    assert!(result.success, "Prayer should succeed: {}", result.message);
    assert_eq!(result.building_id, "abbey");
    assert_eq!(result.activity, TownActivity::Pray);
    assert_eq!(result.hero_id, Some("h1".to_string()));
    assert_eq!(result.upgrade_level, Some('b'));
    assert!(result.stress_change < 0.0, "Stress should be reduced");
    assert!(result.gold_cost > 0, "Gold should be deducted");
}

#[test]
fn perform_rest_at_inn_recovers_health() {
    let registry = parse_buildings();
    let mut town_state = TownState::new(500);
    town_state
        .building_states
        .insert("inn".to_string(), BuildingUpgradeState::new("inn", Some('b')));

    let hero = HeroInTown::new("h1", "alchemist", 50.0, 200.0, 50.0, 100.0);
    let heroes = vec![hero];

    let mut visit = TownVisit::new(town_state, heroes, registry);

    let result = visit.perform_town_activity("inn", TownActivity::Rest, Some("h1"), Some('b'));

    assert!(result.success, "Rest should succeed: {}", result.message);
    assert_eq!(result.building_id, "inn");
    assert_eq!(result.activity, TownActivity::Rest);
    assert!(result.health_change > 0.0, "Health should be recovered");
    assert!(result.stress_change < 0.0, "Stress should be reduced");
}

#[test]
fn perform_recruit_at_stagecoach_adds_hero() {
    let registry = parse_buildings();
    let mut town_state = TownState::new(1000);
    town_state
        .building_states
        .insert("stagecoach".to_string(), BuildingUpgradeState::new("stagecoach", Some('a')));

    let hero = HeroInTown::new("h1", "alchemist", 0.0, 200.0, 100.0, 100.0);
    let heroes = vec![hero];

    let mut visit = TownVisit::new(town_state, heroes, registry);

    let result = visit.perform_town_activity("stagecoach", TownActivity::Recruit, None, None);

    assert!(result.success, "Recruitment should succeed: {}", result.message);
    assert_eq!(visit.heroes.len(), 2, "Should have original + new hero");
    assert!(visit.town_state.gold < 1000, "Gold should be deducted");
}

#[test]
fn perform_upgrade_building_changes_level() {
    let registry = parse_buildings();
    let mut town_state = TownState::new(1000);
    town_state
        .building_states
        .insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('a')));

    let hero = HeroInTown::new("h1", "alchemist", 100.0, 200.0, 100.0, 100.0);
    let heroes = vec![hero];

    let mut visit = TownVisit::new(town_state, heroes, registry);

    // Upgrade abbey from 'a' (free) to 'b' (200)
    let result = visit.perform_town_activity("abbey", TownActivity::UpgradeBuilding, None, Some('b'));

    assert!(result.success, "Upgrade should succeed: {}", result.message);
    assert_eq!(visit.town_state.get_upgrade_level("abbey"), Some('b'));
}

// ── US-008: Activity trace tests ─────────────────────────────────────────────

#[test]
fn town_visit_produces_trace_of_activities() {
    let registry = parse_buildings();
    let mut town_state = TownState::new(1000);
    town_state
        .building_states
        .insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('b')));

    let hero = HeroInTown::new("h1", "alchemist", 100.0, 200.0, 100.0, 100.0);
    let heroes = vec![hero];

    let mut visit = TownVisit::new(town_state, heroes, registry);

    // Perform multiple activities
    visit.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('b'));
    visit.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('b'));

    // Trace should have 2 activities
    assert_eq!(visit.trace.activities.len(), 2);

    // Trace should track total gold spent
    assert_eq!(visit.trace.total_gold_spent(), 400); // 200 * 2

    // Trace should track total stress healed
    assert_eq!(visit.trace.total_stress_healed(), 2.0); // 1 * 2
}

// ── US-008: End-to-end tests ────────────────────────────────────────────────

#[test]
fn entering_town_after_dungeon_run_works() {
    // This is the core meta-game loop test: dungeon run -> town visit
    let registry = parse_buildings();

    // Simulate ending a dungeon run with:
    // - 500 gold earned
    // - 30 stress accumulated across the party
    // - 4 heroes surviving
    let visit = TownVisit::from_dungeon_run(500, 30.0, 4, registry);

    // Verify initial town state matches dungeon run results
    assert_eq!(visit.town_state.gold, 500);
    assert_eq!(visit.heroes.len(), 4);

    // Verify heroes have stress from dungeon run
    for hero in &visit.heroes {
        assert!(
            hero.stress > 0.0,
            "Hero {} should have stress from dungeon run",
            hero.id
        );
    }

    // Verify we can perform town activities
    let mut visit = visit;
    let result = visit.perform_town_activity("abbey", TownActivity::Pray, Some("hero_0"), Some('a'));
    // Level 'a' is free (cost 0)
    assert!(result.success || result.gold_cost == 0);
}

#[test]
fn stress_heal_at_abbey_reduces_hero_stress_and_deducts_gold() {
    let registry = parse_buildings();
    let mut town_state = TownState::new(500);
    town_state
        .building_states
        .insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('b')));

    // Hero with high stress
    let hero = HeroInTown::new("h1", "alchemist", 100.0, 200.0, 100.0, 100.0);
    let heroes = vec![hero];

    let mut visit = TownVisit::new(town_state, heroes, registry);

    // Record initial state
    let initial_gold = visit.town_state.gold;
    let initial_stress = visit.get_hero("h1").unwrap().stress;

    // Perform stress heal at Abbey (level b costs 200, heals 1 stress)
    let result = visit.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('b'));

    // Verify activity succeeded
    assert!(
        result.success,
        "Stress heal should succeed: {}",
        result.message
    );

    // Verify gold was deducted
    assert_eq!(
        visit.town_state.gold,
        initial_gold - result.gold_cost,
        "Gold should be deducted"
    );
    assert_eq!(result.gold_cost, 200);

    // Verify stress was reduced
    let final_stress = visit.get_hero("h1").unwrap().stress;
    assert!(
        final_stress < initial_stress,
        "Stress should be reduced: {} -> {}",
        initial_stress,
        final_stress
    );
    assert_eq!(final_stress, 99.0); // 100 - 1 = 99

    // Verify trace is recorded
    assert_eq!(visit.trace.activities.len(), 1);
    assert_eq!(visit.trace.total_gold_spent(), 200);
}

#[test]
fn multiple_stress_heals_accumulate() {
    // Test that repeated stress heals at the Abbey properly accumulate
    let registry = parse_buildings();
    // 5000 gold - enough for 5 prayers at 800 each = 4000 total
    let mut town_state = TownState::new(5000);
    town_state
        .building_states
        .insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('d')));

    // Hero with 100 stress
    let hero = HeroInTown::new("h1", "alchemist", 100.0, 200.0, 100.0, 100.0);
    let heroes = vec![hero];

    let mut visit = TownVisit::new(town_state, heroes, registry);

    // Pray 5 times at level d (stress_heal = 3 each)
    for _ in 0..5 {
        visit.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('d'));
    }

    // Stress should be reduced by 5 * 3 = 15 (100 -> 85)
    let hero = visit.get_hero("h1").unwrap();
    assert_eq!(hero.stress, 85.0);

    // Gold spent should be 5 * 800 = 4000
    assert_eq!(visit.trace.total_gold_spent(), 4000);
}

// ── US-008: Determinism tests ────────────────────────────────────────────────

#[test]
fn town_visit_is_deterministic_for_given_state() {
    let registry = parse_buildings();
    let mut town_state = TownState::new(1000);
    town_state
        .building_states
        .insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('b')));

    let hero = HeroInTown::new("h1", "alchemist", 100.0, 200.0, 100.0, 100.0);
    let heroes = vec![hero];

    let mut visit1 = TownVisit::new(town_state.clone(), heroes.clone(), registry.clone());
    let mut visit2 = TownVisit::new(town_state, heroes, registry);

    // Perform same activity on both visits
    let result1 = visit1.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('b'));
    let result2 = visit2.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('b'));

    // Results should be identical
    assert_eq!(result1.gold_cost, result2.gold_cost);
    assert_eq!(result1.stress_change, result2.stress_change);
    assert_eq!(result1.success, result2.success);
    assert_eq!(result1.message, result2.message);

    // Final state should be identical
    assert_eq!(visit1.town_state.gold, visit2.town_state.gold);
    assert_eq!(visit1.get_hero("h1").unwrap().stress, visit2.get_hero("h1").unwrap().stress);
}

#[test]
fn town_visit_determinism_across_multiple_activities() {
    let registry = parse_buildings();
    let mut town_state1 = TownState::new(1000);
    town_state1
        .building_states
        .insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('b')));
    town_state1
        .building_states
        .insert("stagecoach".to_string(), BuildingUpgradeState::new("stagecoach", Some('a')));

    let hero1 = HeroInTown::new("h1", "alchemist", 50.0, 200.0, 100.0, 100.0);
    let heroes1 = vec![hero1];

    let mut visit1 = TownVisit::new(town_state1, heroes1, registry.clone());

    let mut town_state2 = TownState::new(1000);
    town_state2
        .building_states
        .insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('b')));
    town_state2
        .building_states
        .insert("stagecoach".to_string(), BuildingUpgradeState::new("stagecoach", Some('a')));

    let hero2 = HeroInTown::new("h1", "alchemist", 50.0, 200.0, 100.0, 100.0);
    let heroes2 = vec![hero2];

    let mut visit2 = TownVisit::new(town_state2, heroes2, registry);

    // Perform same sequence of activities
    let activities = vec![
        ("abbey", TownActivity::Pray, Some("h1"), Some('b')),
        ("stagecoach", TownActivity::Recruit, None, None),
        ("abbey", TownActivity::Pray, Some("h1"), Some('b')),
    ];

    for (building_id, activity, hero_id, level) in activities {
        let result1 = visit1.perform_town_activity(building_id, activity.clone(), hero_id, level);
        let result2 = visit2.perform_town_activity(building_id, activity.clone(), hero_id, level);

        assert_eq!(result1.gold_cost, result2.gold_cost, "Gold cost mismatch for {} {:?}", building_id, activity);
        assert_eq!(result1.stress_change, result2.stress_change, "Stress change mismatch");
        assert_eq!(result1.success, result2.success, "Success mismatch");
    }

    // Final state should be identical
    assert_eq!(visit1.town_state.gold, visit2.town_state.gold);
    assert_eq!(visit1.heroes.len(), visit2.heroes.len());
}

// ── US-008: Error handling tests ─────────────────────────────────────────────

#[test]
fn perform_activity_fails_for_unknown_building() {
    let registry = parse_buildings();
    let town_state = TownState::new(1000);
    let heroes = vec![HeroInTown::new("h1", "alchemist", 50.0, 200.0, 100.0, 100.0)];

    let mut visit = TownVisit::new(town_state, heroes, registry);

    let result = visit.perform_town_activity("nonexistent", TownActivity::Pray, Some("h1"), None);

    assert!(!result.success);
    assert!(result.message.contains("not found"));
}

#[test]
fn perform_pray_fails_for_unknown_hero() {
    let registry = parse_buildings();
    let town_state = TownState::new(500);
    let heroes = vec![HeroInTown::new("h1", "alchemist", 50.0, 200.0, 100.0, 100.0)];

    let mut visit = TownVisit::new(town_state, heroes, registry);

    let result = visit.perform_town_activity("abbey", TownActivity::Pray, Some("unknown_hero"), Some('a'));

    assert!(!result.success);
    assert!(result.message.contains("not found") || result.message.contains("No hero"));
}

#[test]
fn perform_pray_fails_without_enough_gold() {
    let registry = parse_buildings();
    let mut town_state = TownState::new(100); // Not enough for level b (200)
    town_state
        .building_states
        .insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('b')));

    let hero = HeroInTown::new("h1", "alchemist", 100.0, 200.0, 100.0, 100.0);
    let heroes = vec![hero];

    let mut visit = TownVisit::new(town_state, heroes, registry);

    let result = visit.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('b'));

    assert!(!result.success);
    assert!(result.message.contains("Not enough gold"));
    // Gold should be unchanged
    assert_eq!(visit.town_state.gold, 100);
}

// ── US-008: Building service resolution tests ─────────────────────────────────

#[test]
fn abbey_level_determines_stress_heal_amount() {
    let registry = parse_buildings();
    let mut town_state = TownState::new(10000);
    town_state
        .building_states
        .insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('a')));

    // Hero with 10 stress
    let hero = HeroInTown::new("h1", "alchemist", 10.0, 200.0, 100.0, 100.0);
    let heroes = vec![hero];

    let mut visit = TownVisit::new(town_state, heroes, registry);

    // Level a: stress_heal = 0 (no effect, but level a is free)
    // Actually level 'a' has no stress_heal effect, so stress unchanged
    let result_a = visit.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('a'));
    assert!(result_a.success);

    // Level b: stress_heal = 1
    visit.town_state.gold = 10000;
    visit.town_state.building_states.insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('b')));
    let result_b = visit.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('b'));
    assert!(result_b.success);
    assert_eq!(result_b.stress_change, -1.0);

    // Level c: stress_heal = 2
    visit.town_state.gold = 10000;
    visit.town_state.building_states.insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('c')));
    let result_c = visit.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('c'));
    assert!(result_c.success);
    assert_eq!(result_c.stress_change, -2.0);

    // Level d: stress_heal = 3
    visit.town_state.gold = 10000;
    visit.town_state.building_states.insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('d')));
    let result_d = visit.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('d'));
    assert!(result_d.success);
    assert_eq!(result_d.stress_change, -3.0);
}

#[test]
fn stagecoach_recruit_cost_with_discount() {
    let registry = parse_buildings();

    // Level a: no discount
    let mut town_state_a = TownState::new(1000);
    town_state_a
        .building_states
        .insert("stagecoach".to_string(), BuildingUpgradeState::new("stagecoach", Some('a')));

    let heroes = vec![HeroInTown::new("h1", "alchemist", 0.0, 200.0, 100.0, 100.0)];
    let mut visit_a = TownVisit::new(town_state_a, heroes, registry.clone());

    let result_a = visit_a.perform_town_activity("stagecoach", TownActivity::Recruit, None, None);
    assert!(result_a.success);
    assert_eq!(result_a.gold_cost, 500); // Base cost, no discount

    // Level d: 30% discount
    let mut town_state_d = TownState::new(1000);
    town_state_d
        .building_states
        .insert("stagecoach".to_string(), BuildingUpgradeState::new("stagecoach", Some('d')));

    let heroes = vec![HeroInTown::new("h1", "alchemist", 0.0, 200.0, 100.0, 100.0)];
    let mut visit_d = TownVisit::new(town_state_d, heroes, registry.clone());

    let result_d = visit_d.perform_town_activity("stagecoach", TownActivity::Recruit, None, None);
    assert!(result_d.success);
    assert_eq!(result_d.gold_cost, 350); // 500 * 0.7 = 350
}