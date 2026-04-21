//! Integration test for building registry (US-007).
//!
//! Validates:
//! - BuildingRegistry holds all 10 DDGC building definitions parsed from Buildings.json
//! - Each building has correct unlock conditions, upgrade trees, service parameters
//! - At least 3 buildings (StageCoach, Abbey, Guild) are fully parsed with all upgrade levels
//! - Focused test proves building lookup by ID works
//! - Focused test proves upgrade tree traversal produces correct cost/effect at each level
//! - Focused test proves all 10 buildings are loadable

use game_ddgc_headless::contracts::{
    parse::parse_buildings_json,
    BuildingRegistry, BuildingType,
};

fn data_path(filename: &str) -> std::path::PathBuf {
    std::path::PathBuf::from("data").join(filename)
}

fn parse_all() -> BuildingRegistry {
    parse_buildings_json(&data_path("Buildings.json"))
        .expect("failed to parse Buildings.json")
}

// ── US-007: All 10 buildings are loadable ─────────────────────────────────────

#[test]
fn all_10_buildings_are_loadable() {
    let registry = parse_all();

    assert_eq!(
        registry.len(),
        10,
        "All 10 buildings should be parsed, got {}",
        registry.len()
    );
}

#[test]
fn all_building_ids_are_unique() {
    let registry = parse_all();
    let ids = registry.all_ids();

    // Check no duplicates
    let mut unique_ids = ids.clone();
    unique_ids.sort();
    unique_ids.dedup();

    assert_eq!(
        ids.len(),
        unique_ids.len(),
        "All building IDs should be unique"
    );
}

// ── US-007: Building lookup by ID ─────────────────────────────────────────────

#[test]
fn stagecoach_lookup_by_id_works() {
    let registry = parse_all();

    let stagecoach = registry.get("stagecoach");
    assert!(
        stagecoach.is_some(),
        "stagecoach should exist in registry"
    );

    let stagecoach = stagecoach.unwrap();
    assert_eq!(stagecoach.id, "stagecoach");
    assert_eq!(stagecoach.building_type, BuildingType::Barracks);
}

#[test]
fn abbey_lookup_by_id_works() {
    let registry = parse_all();

    let abbey = registry.get("abbey");
    assert!(
        abbey.is_some(),
        "abbey should exist in registry"
    );

    let abbey = abbey.unwrap();
    assert_eq!(abbey.id, "abbey");
    assert_eq!(abbey.building_type, BuildingType::Cathedral);
}

#[test]
fn guild_lookup_by_id_works() {
    let registry = parse_all();

    let guild = registry.get("guild");
    assert!(
        guild.is_some(),
        "guild should exist in registry"
    );

    let guild = guild.unwrap();
    assert_eq!(guild.id, "guild");
    assert_eq!(guild.building_type, BuildingType::Tower);
}

#[test]
fn all_10_buildings_lookup_by_id_works() {
    let registry = parse_all();

    let expected_ids = [
        "stagecoach", "abbey", "guild", "blacksmith", "inn",
        "tavern", "graveyard", "museum", "provisioner", "sanctuary",
    ];

    for id in &expected_ids {
        assert!(
            registry.get(*id).is_some(),
            "Building {} should exist in registry",
            id
        );
    }
}

#[test]
fn registry_returns_none_for_unknown_id() {
    let registry = parse_all();

    assert!(
        registry.get("nonexistent_building").is_none(),
        "Unknown building should return None"
    );
}

// ── US-007: Upgrade tree traversal ────────────────────────────────────────────

#[test]
fn stagecoach_has_two_upgrade_trees() {
    let registry = parse_all();
    let stagecoach = registry.get("stagecoach").expect("stagecoach should exist");

    assert_eq!(
        stagecoach.upgrade_trees.len(),
        2,
        "stagecoach should have 2 upgrade trees"
    );
}

#[test]
fn stagecoach_recruit_tree_has_correct_levels() {
    let registry = parse_all();
    let stagecoach = registry.get("stagecoach").expect("stagecoach should exist");

    // Find the recruit tree
    let recruit_tree = stagecoach
        .upgrade_trees
        .iter()
        .find(|t| t.tree_id == "stagecoach_recruit")
        .expect("stagecoach_recruit tree should exist");

    assert_eq!(recruit_tree.levels.len(), 4, "recruit tree should have 4 levels");

    // Level a should be free
    let level_a = recruit_tree
        .levels
        .iter()
        .find(|l| l.code == 'a')
        .expect("level a should exist");
    assert_eq!(level_a.cost, 0, "level a should be free");

    // Level d should have recruit_discount 0.3
    let level_d = recruit_tree
        .levels
        .iter()
        .find(|l| l.code == 'd')
        .expect("level d should exist");
    assert_eq!(level_d.cost, 1000, "level d should cost 1000");

    let discount_effect = level_d
        .effects
        .iter()
        .find(|e| e.effect_id == "recruit_discount")
        .expect("recruit_discount effect should exist at level d");
    assert_eq!(discount_effect.value, 0.3, "recruit_discount at level d should be 0.3");
}

#[test]
fn abbey_has_correct_unlock_conditions() {
    let registry = parse_all();
    let abbey = registry.get("abbey").expect("abbey should exist");

    assert_eq!(
        abbey.unlock_conditions.len(),
        1,
        "abbey should have 1 unlock condition"
    );

    let condition = &abbey.unlock_conditions[0];
    assert_eq!(condition.condition_type, "completed_runs");
    assert_eq!(condition.required_count, 1);
}

#[test]
fn abbey_prayer_tree_has_correct_stress_heal_values() {
    let registry = parse_all();
    let abbey = registry.get("abbey").expect("abbey should exist");

    let prayer_tree = abbey
        .upgrade_trees
        .iter()
        .find(|t| t.tree_id == "abbey_prayer")
        .expect("abbey_prayer tree should exist");

    // Level b: stress_heal = 1
    let level_b = prayer_tree
        .levels
        .iter()
        .find(|l| l.code == 'b')
        .expect("level b should exist");
    let stress_heal_b = level_b
        .effects
        .iter()
        .find(|e| e.effect_id == "stress_heal")
        .expect("stress_heal effect should exist at level b");
    assert_eq!(stress_heal_b.value, 1.0);

    // Level d: stress_heal = 3
    let level_d = prayer_tree
        .levels
        .iter()
        .find(|l| l.code == 'd')
        .expect("level d should exist");
    let stress_heal_d = level_d
        .effects
        .iter()
        .find(|e| e.effect_id == "stress_heal")
        .expect("stress_heal effect should exist at level d");
    assert_eq!(stress_heal_d.value, 3.0);
}

#[test]
fn guild_has_correct_upgrade_costs() {
    let registry = parse_all();

    // Level costs should increase with each level
    let training_tree = registry
        .get("guild")
        .expect("guild should exist")
        .upgrade_trees
        .iter()
        .find(|t| t.tree_id == "guild_training")
        .expect("guild_training tree should exist");

    let costs: Vec<u32> = training_tree.levels.iter().map(|l| l.cost).collect();
    assert!(costs.windows(2).all(|w| w[0] <= w[1]), "costs should be non-decreasing");
}

#[test]
fn guild_experience_boost_increases_with_level() {
    let registry = parse_all();

    let training_tree = registry
        .get("guild")
        .expect("guild should exist")
        .upgrade_trees
        .iter()
        .find(|t| t.tree_id == "guild_training")
        .expect("guild_training tree should exist");

    let experience_values: Vec<f64> = training_tree
        .levels
        .iter()
        .filter_map(|l| l.effects.iter().find(|e| e.effect_id == "experience_boost").map(|e| e.value))
        .collect();

    // Values should be increasing (0.1, 0.2, 0.3)
    assert!(experience_values.windows(2).all(|w| w[0] < w[1]), "experience_boost should increase with level");
}

// ── US-007: BuildingRegistry helper methods ────────────────────────────────────

#[test]
fn get_effect_at_level_works_for_stagecoach() {
    let registry = parse_all();

    // Level b should have recruit_discount = 0.1
    let effect = registry.get_effect_at_level("stagecoach", 'b', "recruit_discount");
    assert_eq!(effect, Some(0.1), "stagecoach level b should have recruit_discount 0.1");

    // Level d should have recruit_discount = 0.3
    let effect = registry.get_effect_at_level("stagecoach", 'd', "recruit_discount");
    assert_eq!(effect, Some(0.3), "stagecoach level d should have recruit_discount 0.3");

    // Unknown effect should return None
    let effect = registry.get_effect_at_level("stagecoach", 'b', "unknown_effect");
    assert_eq!(effect, None, "unknown effect should return None");
}

#[test]
fn get_effect_at_level_works_for_abbey() {
    let registry = parse_all();

    // Level c in abbey_prayer tree has stress_heal = 2
    // Note: get_effect_at_level returns the first matching level across all trees,
    // so it finds level 'c' in abbey_prayer first (stress_heal = 2)
    let effect = registry.get_effect_at_level("abbey", 'c', "stress_heal");
    assert_eq!(effect, Some(2.0), "abbey level c (prayer tree) should have stress_heal 2.0");
}

#[test]
fn get_upgrade_cost_works() {
    let registry = parse_all();

    // Stagecoach level b costs 250
    let cost = registry.get_upgrade_cost("stagecoach", 'b');
    assert_eq!(cost, Some(250), "stagecoach level b should cost 250");

    // Stagecoach level d costs 1000
    let cost = registry.get_upgrade_cost("stagecoach", 'd');
    assert_eq!(cost, Some(1000), "stagecoach level d should cost 1000");

    // Unknown level should return None
    let cost = registry.get_upgrade_cost("stagecoach", 'z');
    assert_eq!(cost, None, "unknown level should return None");
}

#[test]
fn get_upgrade_levels_returns_sorted_levels() {
    let registry = parse_all();

    let levels = registry.get_upgrade_levels("stagecoach");
    assert!(levels.is_some(), "stagecoach should have upgrade levels");

    let levels = levels.unwrap();
    // stagecoach_recruit has 4 levels (a,b,c,d) and stagecoach_heroes has 3 levels (a,b,c) = 7 total
    assert_eq!(levels.len(), 7, "stagecoach has 7 total levels across both trees");

    // Levels should be sorted by code
    let codes: Vec<char> = levels.iter().map(|l| l.code).collect();
    let mut sorted_codes = codes.clone();
    sorted_codes.sort();
    assert_eq!(codes, sorted_codes, "levels should be sorted by code");
}

// ── US-007: BuildingType filtering ───────────────────────────────────────────

#[test]
fn by_type_returns_correct_buildings() {
    let registry = parse_all();

    // There should be exactly one building of type Barracks (stagecoach)
    let barracks = registry.by_type(BuildingType::Barracks);
    assert_eq!(barracks.len(), 1, "should have 1 Barracks building");
    assert_eq!(barracks[0].id, "stagecoach");

    // There should be exactly one building of type Cathedral (abbey)
    let cathedrals = registry.by_type(BuildingType::Cathedral);
    assert_eq!(cathedrals.len(), 1, "should have 1 Cathedral building");
    assert_eq!(cathedrals[0].id, "abbey");

    // There should be exactly one building of type Tower (guild)
    let towers = registry.by_type(BuildingType::Tower);
    assert_eq!(towers.len(), 1, "should have 1 Tower building");
    assert_eq!(towers[0].id, "guild");
}

// ── US-007: Unlock conditions ─────────────────────────────────────────────────

#[test]
fn unlocked_buildings_have_no_conditions() {
    let registry = parse_all();

    // stagecoach should have no unlock conditions
    let stagecoach = registry.get("stagecoach").expect("stagecoach should exist");
    assert!(
        stagecoach.unlock_conditions.is_empty(),
        "stagecoach should have no unlock conditions"
    );

    // blacksmith should have no unlock conditions
    let blacksmith = registry.get("blacksmith").expect("blacksmith should exist");
    assert!(
        blacksmith.unlock_conditions.is_empty(),
        "blacksmith should have no unlock conditions"
    );
}

#[test]
fn locked_buildings_have_correct_conditions() {
    let registry = parse_all();

    // abbey requires completed_runs >= 1
    let abbey = registry.get("abbey").expect("abbey should exist");
    assert_eq!(abbey.unlock_conditions.len(), 1);
    assert_eq!(abbey.unlock_conditions[0].condition_type, "completed_runs");
    assert_eq!(abbey.unlock_conditions[0].required_count, 1);

    // graveyard requires defeated_monsters >= 50
    let graveyard = registry.get("graveyard").expect("graveyard should exist");
    assert_eq!(graveyard.unlock_conditions.len(), 1);
    assert_eq!(graveyard.unlock_conditions[0].condition_type, "defeated_monsters");
    assert_eq!(graveyard.unlock_conditions[0].required_count, 50);

    // museum requires completed_runs >= 3
    let museum = registry.get("museum").expect("museum should exist");
    assert_eq!(museum.unlock_conditions.len(), 1);
    assert_eq!(museum.unlock_conditions[0].condition_type, "completed_runs");
    assert_eq!(museum.unlock_conditions[0].required_count, 3);
}

// ── US-007: Full parsing verification ─────────────────────────────────────────

#[test]
fn all_buildings_have_at_least_one_upgrade_tree() {
    let registry = parse_all();

    for id in registry.all_ids() {
        let building = registry.get(id).expect("building should exist");
        assert!(
            !building.upgrade_trees.is_empty(),
            "Building {} should have at least one upgrade tree",
            id
        );
    }
}

#[test]
fn all_upgrade_levels_have_valid_codes() {
    let registry = parse_all();

    for id in registry.all_ids() {
        let building = registry.get(id).expect("building should exist");
        for tree in &building.upgrade_trees {
            for level in &tree.levels {
                assert!(
                    level.code.is_ascii_lowercase(),
                    "Level code {} should be lowercase ASCII",
                    level.code
                );
            }
        }
    }
}

#[test]
fn all_upgrade_costs_are_non_negative() {
    let registry = parse_all();

    for id in registry.all_ids() {
        let building = registry.get(id).expect("building should exist");
        for tree in &building.upgrade_trees {
            for level in &tree.levels {
                // cost is u32, so always >= 0 - this is guaranteed by the type system
                // but we iterate to ensure all levels are accessible
                let _ = level.cost;
            }
        }
    }
}

#[test]
fn upgrade_trees_have_starting_level() {
    let registry = parse_all();

    for id in registry.all_ids() {
        let building = registry.get(id).expect("building should exist");
        for tree in &building.upgrade_trees {
            let has_starting_level = tree.levels.iter().any(|l| l.code == 'a' && l.cost == 0);
            assert!(
                has_starting_level,
                "Building {} tree {} should have a free starting level 'a'",
                id,
                tree.tree_id
            );
        }
    }
}

// ── US-007: Specific building verification ────────────────────────────────────

#[test]
fn blacksmith_has_repair_and_upgrade_trees() {
    let registry = parse_all();

    let blacksmith = registry.get("blacksmith").expect("blacksmith should exist");
    assert_eq!(blacksmith.building_type, BuildingType::Blacksmith);
    assert_eq!(blacksmith.upgrade_trees.len(), 2);

    let has_repair = blacksmith
        .upgrade_trees
        .iter()
        .any(|t| t.tree_id == "blacksmith_repair");
    let has_upgrade = blacksmith
        .upgrade_trees
        .iter()
        .any(|t| t.tree_id == "blacksmith_upgrade");

    assert!(has_repair, "blacksmith should have repair tree");
    assert!(has_upgrade, "blacksmith should have upgrade tree");
}

#[test]
fn inn_has_rest_and_food_trees() {
    let registry = parse_all();

    let inn = registry.get("inn").expect("inn should exist");
    assert_eq!(inn.building_type, BuildingType::Inn);
    assert_eq!(inn.upgrade_trees.len(), 2);

    let has_rest = inn.upgrade_trees.iter().any(|t| t.tree_id == "inn_rest");
    let has_food = inn.upgrade_trees.iter().any(|t| t.tree_id == "inn_food");

    assert!(has_rest, "inn should have rest tree");
    assert!(has_food, "inn should have food tree");
}

#[test]
fn provisioner_has_supplies_and_bonus_trees() {
    let registry = parse_all();

    let provisioner = registry.get("provisioner").expect("provisioner should exist");
    assert_eq!(provisioner.building_type, BuildingType::Provisioner);
    assert_eq!(provisioner.upgrade_trees.len(), 2);

    let has_supplies = provisioner
        .upgrade_trees
        .iter()
        .any(|t| t.tree_id == "provisioner_supplies");
    let has_bonus = provisioner
        .upgrade_trees
        .iter()
        .any(|t| t.tree_id == "provisioner_bonus");

    assert!(has_supplies, "provisioner should have supplies tree");
    assert!(has_bonus, "provisioner should have bonus tree");
}
