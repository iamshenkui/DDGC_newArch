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
fn all_11_buildings_are_loadable() {
    let registry = parse_all();

    assert_eq!(
        registry.len(),
        11,
        "All 11 buildings should be parsed, got {}",
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
fn all_11_buildings_lookup_by_id_works() {
    let registry = parse_all();

    let expected_ids = [
        "stagecoach", "abbey", "guild", "blacksmith", "inn",
        "tavern", "graveyard", "museum", "provisioner", "sanctuary",
        "sanitarium",
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
fn blacksmith_has_repair_upgrade_and_discount_trees() {
    let registry = parse_all();

    let blacksmith = registry.get("blacksmith").expect("blacksmith should exist");
    assert_eq!(blacksmith.building_type, BuildingType::Blacksmith);
    assert_eq!(blacksmith.upgrade_trees.len(), 3);

    let has_repair = blacksmith
        .upgrade_trees
        .iter()
        .any(|t| t.tree_id == "blacksmith_repair");
    let has_upgrade = blacksmith
        .upgrade_trees
        .iter()
        .any(|t| t.tree_id == "blacksmith_upgrade");
    let has_discount = blacksmith
        .upgrade_trees
        .iter()
        .any(|t| t.tree_id == "blacksmith_equipment_discount");

    assert!(has_repair, "blacksmith should have repair tree");
    assert!(has_upgrade, "blacksmith should have upgrade tree");
    assert!(has_discount, "blacksmith should have equipment discount tree");
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

// ── US-001-a: Sanitarium extended registry tests ───────────────────────────────

#[test]
fn sanitarium_has_all_required_upgrade_trees() {
    let registry = parse_all();

    let sanitarium = registry.get("sanitarium").expect("sanitarium should exist");
    assert_eq!(sanitarium.building_type, BuildingType::Sanctuary);

    // Verify we have the expected upgrade trees for disease and quirk treatment
    let tree_ids: Vec<&str> = sanitarium.upgrade_trees.iter().map(|t| t.tree_id.as_str()).collect();

    // Disease-related trees
    assert!(tree_ids.contains(&"disease_cost"), "sanitarium should have disease_cost tree");
    assert!(tree_ids.contains(&"disease_cure_all_chance"), "sanitarium should have disease_cure_all_chance tree");
    assert!(tree_ids.contains(&"disease_slots"), "sanitarium should have disease_slots tree");

    // Quirk-related trees
    assert!(tree_ids.contains(&"quirk_positive_cost"), "sanitarium should have quirk_positive_cost tree");
    assert!(tree_ids.contains(&"quirk_negative_cost"), "sanitarium should have quirk_negative_cost tree");
    assert!(tree_ids.contains(&"quirk_permanent_negative_cost"), "sanitarium should have quirk_permanent_negative_cost tree");
    assert!(tree_ids.contains(&"quirk_treatment_chance"), "sanitarium should have quirk_treatment_chance tree");
    assert!(tree_ids.contains(&"quirk_slots"), "sanitarium should have quirk_slots tree");
}

#[test]
fn sanitarium_disease_upgrade_paths_follow_pattern() {
    let registry = parse_all();

    // Disease treatment cost upgrades follow a/c/e pattern
    // Level a: cost = 750 (base)
    // Level c: cost = 600
    // Level e: cost = 450
    assert_eq!(registry.sanitarium_disease_cost('a'), Some(750.0), "disease cost at level a should be 750");
    assert_eq!(registry.sanitarium_disease_cost('c'), Some(600.0), "disease cost at level c should be 600");
    assert_eq!(registry.sanitarium_disease_cost('e'), Some(450.0), "disease cost at level e should be 450");

    // Cure-all chance upgrades follow b/d pattern
    // Level b: chance = 0.5
    // Level d: chance = 0.75
    assert_eq!(registry.sanitarium_cure_all_chance('b'), Some(0.5), "cure-all chance at level b should be 0.5");
    assert_eq!(registry.sanitarium_cure_all_chance('d'), Some(0.75), "cure-all chance at level d should be 0.75");
}

#[test]
fn sanitarium_disease_slots_increase_with_level() {
    let registry = parse_all();

    // Disease slots: a=1, b=2, c=3, d=3, e=3
    assert_eq!(registry.sanitarium_disease_slots('a'), Some(1.0), "disease slots at level a should be 1");
    assert_eq!(registry.sanitarium_disease_slots('b'), Some(2.0), "disease slots at level b should be 2");
    assert_eq!(registry.sanitarium_disease_slots('c'), Some(3.0), "disease slots at level c should be 3");
    assert_eq!(registry.sanitarium_disease_slots('d'), Some(3.0), "disease slots at level d should be 3");
    assert_eq!(registry.sanitarium_disease_slots('e'), Some(3.0), "disease slots at level e should be 3");
}

#[test]
fn sanitarium_quirk_slots_increase_with_level() {
    let registry = parse_all();

    // Quirk slots: a=1, b=2, c=2, d=3, e=3
    assert_eq!(registry.sanitarium_quirk_slots('a'), Some(1.0), "quirk slots at level a should be 1");
    assert_eq!(registry.sanitarium_quirk_slots('b'), Some(2.0), "quirk slots at level b should be 2");
    assert_eq!(registry.sanitarium_quirk_slots('c'), Some(2.0), "quirk slots at level c should be 2");
    assert_eq!(registry.sanitarium_quirk_slots('d'), Some(3.0), "quirk slots at level d should be 3");
    assert_eq!(registry.sanitarium_quirk_slots('e'), Some(3.0), "quirk slots at level e should be 3");
}

#[test]
fn sanitarium_quirk_treatment_costs_decrease_with_level() {
    let registry = parse_all();

    // Positive quirk cost: a=7500, b=5000, c=3750, d=3125, e=2500
    let cost_a = registry.get_effect_at_level("sanitarium", 'a', "positive_quirk_cost");
    let cost_e = registry.get_effect_at_level("sanitarium", 'e', "positive_quirk_cost");
    assert_eq!(cost_a, Some(7500.0), "positive quirk cost at level a should be 7500");
    assert_eq!(cost_e, Some(2500.0), "positive quirk cost at level e should be 2500");

    // Negative quirk cost: a=1500, b=1125, c=937, d=843, e=750
    let neg_a = registry.get_effect_at_level("sanitarium", 'a', "negative_quirk_cost");
    let neg_e = registry.get_effect_at_level("sanitarium", 'e', "negative_quirk_cost");
    assert_eq!(neg_a, Some(1500.0), "negative quirk cost at level a should be 1500");
    assert_eq!(neg_e, Some(750.0), "negative quirk cost at level e should be 750");
}

// ── US-001-a: Tavern extended registry tests ───────────────────────────────────

#[test]
fn tavern_has_all_required_upgrade_trees() {
    let registry = parse_all();

    let tavern = registry.get("tavern").expect("tavern should exist");
    assert_eq!(tavern.building_type, BuildingType::Tavern);

    let tree_ids: Vec<&str> = tavern.upgrade_trees.iter().map(|t| t.tree_id.as_str()).collect();

    // Bar-related trees
    assert!(tree_ids.contains(&"tavern_bar_cost"), "tavern should have bar_cost tree");
    assert!(tree_ids.contains(&"tavern_bar_stress_heal"), "tavern should have bar_stress_heal tree");
    assert!(tree_ids.contains(&"tavern_bar_slots"), "tavern should have bar_slots tree");

    // Gambling-related trees
    assert!(tree_ids.contains(&"tavern_gambling_cost"), "tavern should have gambling_cost tree");
    assert!(tree_ids.contains(&"tavern_gambling_stress_heal"), "tavern should have gambling_stress_heal tree");

    // Brothel-related trees
    assert!(tree_ids.contains(&"tavern_brothel_cost"), "tavern should have brothel_cost tree");
    assert!(tree_ids.contains(&"tavern_brothel_stress_heal"), "tavern should have brothel_stress_heal tree");
}

#[test]
fn tavern_bar_activities_follow_pattern() {
    let registry = parse_all();

    // Bar cost: a=1000, b=900, c=900, d=900, e=700, f=700
    assert_eq!(registry.tavern_bar_cost('a'), Some(1000.0), "bar cost at level a should be 1000");
    assert_eq!(registry.tavern_bar_cost('b'), Some(900.0), "bar cost at level b should be 900");
    assert_eq!(registry.tavern_bar_cost('e'), Some(700.0), "bar cost at level e should be 700");
    assert_eq!(registry.tavern_bar_cost('f'), Some(700.0), "bar cost at level f should be 700");

    // Bar stress heal: a=45, d=100, f=100
    assert_eq!(registry.tavern_bar_stress_heal('a'), Some(45.0), "bar stress heal at level a should be 45");
    assert_eq!(registry.tavern_bar_stress_heal('d'), Some(100.0), "bar stress heal at level d should be 100");
    assert_eq!(registry.tavern_bar_stress_heal('f'), Some(100.0), "bar stress heal at level f should be 100");
}

#[test]
fn tavern_bar_slots_increase_with_level() {
    let registry = parse_all();

    // Bar slots: a=1, b=1, c=2, d=2, e=2, f=3
    assert_eq!(registry.tavern_bar_slots('a'), Some(1.0), "bar slots at level a should be 1");
    assert_eq!(registry.tavern_bar_slots('c'), Some(2.0), "bar slots at level c should be 2");
    assert_eq!(registry.tavern_bar_slots('f'), Some(3.0), "bar slots at level f should be 3");
}

#[test]
fn tavern_gambling_activities_follow_pattern() {
    let registry = parse_all();

    // Gambling cost: a=1250, b=1100, e=900, f=900
    assert_eq!(registry.tavern_gambling_cost('a'), Some(1250.0), "gambling cost at level a should be 1250");
    assert_eq!(registry.tavern_gambling_cost('b'), Some(1100.0), "gambling cost at level b should be 1100");
    assert_eq!(registry.tavern_gambling_cost('e'), Some(900.0), "gambling cost at level e should be 900");

    // Gambling stress heal: a=55, d=86, f=86
    assert_eq!(registry.tavern_gambling_stress_heal('a'), Some(55.0), "gambling stress heal at level a should be 55");
    assert_eq!(registry.tavern_gambling_stress_heal('d'), Some(86.0), "gambling stress heal at level d should be 86");
}

#[test]
fn tavern_brothel_activities_follow_pattern() {
    let registry = parse_all();

    // Brothel cost: a=1500, b=1350, e=1100, f=1100
    assert_eq!(registry.tavern_brothel_cost('a'), Some(1500.0), "brothel cost at level a should be 1500");
    assert_eq!(registry.tavern_brothel_cost('b'), Some(1350.0), "brothel cost at level b should be 1350");
    assert_eq!(registry.tavern_brothel_cost('e'), Some(1100.0), "brothel cost at level e should be 1100");

    // Brothel stress heal: a=65, d=100, f=100
    assert_eq!(registry.tavern_brothel_stress_heal('a'), Some(65.0), "brothel stress heal at level a should be 65");
    assert_eq!(registry.tavern_brothel_stress_heal('d'), Some(100.0), "brothel stress heal at level d should be 100");
}

// ── US-001-a: Blacksmith extended registry tests ────────────────────────────────

#[test]
fn blacksmith_has_all_required_upgrade_trees() {
    let registry = parse_all();

    let blacksmith = registry.get("blacksmith").expect("blacksmith should exist");
    assert_eq!(blacksmith.building_type, BuildingType::Blacksmith);

    let tree_ids: Vec<&str> = blacksmith.upgrade_trees.iter().map(|t| t.tree_id.as_str()).collect();

    assert!(tree_ids.contains(&"blacksmith_repair"), "blacksmith should have repair tree");
    assert!(tree_ids.contains(&"blacksmith_upgrade"), "blacksmith should have upgrade tree");
    assert!(tree_ids.contains(&"blacksmith_equipment_discount"), "blacksmith should have equipment_discount tree");
}

#[test]
fn blacksmith_repair_discount_increases_with_level() {
    let registry = parse_all();

    // Repair discount: level a has no effects, b=0.1, c=0.2, d=0.3
    assert_eq!(registry.blacksmith_repair_discount('a'), None, "repair discount at level a should be None (no effects)");
    assert_eq!(registry.blacksmith_repair_discount('b'), Some(0.1), "repair discount at level b should be 0.1");
    assert_eq!(registry.blacksmith_repair_discount('c'), Some(0.2), "repair discount at level c should be 0.2");
    assert_eq!(registry.blacksmith_repair_discount('d'), Some(0.3), "repair discount at level d should be 0.3");
}

#[test]
fn blacksmith_weapon_upgrade_cost_reduces_with_level() {
    let registry = parse_all();

    // Weapon upgrade cost: level a has no effects, b=0.15, c=0.25
    assert_eq!(registry.blacksmith_weapon_upgrade_cost('a'), None, "weapon upgrade cost at level a should be None");
    assert_eq!(registry.blacksmith_weapon_upgrade_cost('b'), Some(0.15), "weapon upgrade cost at level b should be 0.15");
    assert_eq!(registry.blacksmith_weapon_upgrade_cost('c'), Some(0.25), "weapon upgrade cost at level c should be 0.25");
}

#[test]
fn blacksmith_equipment_discount_increases_with_level() {
    let registry = parse_all();

    // Equipment discount: level a has effect 0.0 (base), b=0.1, c=0.2, d=0.3, e=0.4, f=0.5
    assert_eq!(registry.blacksmith_equipment_discount('a'), Some(0.0), "equipment discount at level a should be 0.0");
    assert_eq!(registry.blacksmith_equipment_discount('b'), Some(0.1), "equipment discount at level b should be 0.1");
    assert_eq!(registry.blacksmith_equipment_discount('d'), Some(0.3), "equipment discount at level d should be 0.3");
    assert_eq!(registry.blacksmith_equipment_discount('f'), Some(0.5), "equipment discount at level f should be 0.5");
}

// ── US-001-a: Guild extended registry tests ────────────────────────────────────

#[test]
fn guild_has_all_required_upgrade_trees() {
    let registry = parse_all();

    let guild = registry.get("guild").expect("guild should exist");
    assert_eq!(guild.building_type, BuildingType::Tower);

    let tree_ids: Vec<&str> = guild.upgrade_trees.iter().map(|t| t.tree_id.as_str()).collect();

    assert!(tree_ids.contains(&"guild_training"), "guild should have training tree");
    assert!(tree_ids.contains(&"guild_skills"), "guild should have skills tree");
    assert!(tree_ids.contains(&"guild_skill_discount"), "guild should have skill_discount tree");
}

#[test]
fn guild_experience_boost_via_helper_method() {
    let registry = parse_all();

    // Experience boost: level a has no effects, b=0.1, c=0.2, d=0.3
    assert_eq!(registry.guild_experience_boost('a'), None, "experience boost at level a should be None");
    assert_eq!(registry.guild_experience_boost('b'), Some(0.1), "experience boost at level b should be 0.1");
    assert_eq!(registry.guild_experience_boost('c'), Some(0.2), "experience boost at level c should be 0.2");
    assert_eq!(registry.guild_experience_boost('d'), Some(0.3), "experience boost at level d should be 0.3");
}

#[test]
fn guild_skill_upgrade_chance_via_helper_method() {
    let registry = parse_all();

    // Skill upgrade chance: level a has no effects, b=0.05, c=0.1
    assert_eq!(registry.guild_skill_upgrade_chance('a'), None, "skill upgrade chance at level a should be None");
    assert_eq!(registry.guild_skill_upgrade_chance('b'), Some(0.05), "skill upgrade chance at level b should be 0.05");
    assert_eq!(registry.guild_skill_upgrade_chance('c'), Some(0.1), "skill upgrade chance at level c should be 0.1");
}

#[test]
fn guild_skill_cost_discount_increases_with_level() {
    let registry = parse_all();

    // Skill cost discount: level a has effect 0.0 (base), b=0.1, c=0.2, d=0.3, e=0.4, f=0.5
    assert_eq!(registry.guild_skill_cost_discount('a'), Some(0.0), "skill cost discount at level a should be 0.0");
    assert_eq!(registry.guild_skill_cost_discount('b'), Some(0.1), "skill cost discount at level b should be 0.1");
    assert_eq!(registry.guild_skill_cost_discount('d'), Some(0.3), "skill cost discount at level d should be 0.3");
    assert_eq!(registry.guild_skill_cost_discount('f'), Some(0.5), "skill cost discount at level f should be 0.5");
}

// ── US-001-a: TownSlotState tests ──────────────────────────────────────────────

#[test]
fn town_slot_state_tracks_availability() {
    let mut slot_state = game_ddgc_headless::contracts::TownSlotState::new();

    // Initially no slots
    assert_eq!(slot_state.available("sanitarium", "disease"), 0);
    assert!(!slot_state.has_available("sanitarium", "disease"));

    // Set capacity
    slot_state.set_capacity("sanitarium", "disease", 2);
    assert_eq!(slot_state.available("sanitarium", "disease"), 2);
    assert!(slot_state.has_available("sanitarium", "disease"));

    // Consume one slot
    assert!(slot_state.try_consume("sanitarium", "disease"));
    assert_eq!(slot_state.available("sanitarium", "disease"), 1);

    // Consume another slot
    assert!(slot_state.try_consume("sanitarium", "disease"));
    assert_eq!(slot_state.available("sanitarium", "disease"), 0);
    assert!(!slot_state.has_available("sanitarium", "disease"));

    // Try to consume when full
    assert!(!slot_state.try_consume("sanitarium", "disease"));
}

#[test]
fn town_slot_state_reset_clears_consumed() {
    let mut slot_state = game_ddgc_headless::contracts::TownSlotState::new();

    // Set capacity and consume
    slot_state.set_capacity("sanitarium", "disease", 2);
    slot_state.try_consume("sanitarium", "disease");
    slot_state.try_consume("sanitarium", "disease");
    assert_eq!(slot_state.available("sanitarium", "disease"), 0);

    // Reset
    slot_state.reset();
    assert_eq!(slot_state.available("sanitarium", "disease"), 2);
}

#[test]
fn town_slot_state_multiple_activities() {
    let mut slot_state = game_ddgc_headless::contracts::TownSlotState::new();

    // Set up multiple building activities
    slot_state.set_capacity("sanitarium", "quirk", 1);
    slot_state.set_capacity("sanitarium", "disease", 2);
    slot_state.set_capacity("tavern", "bar", 3);

    // Sanitarium quirk: 1 slot
    assert!(slot_state.try_consume("sanitarium", "quirk"));
    assert!(!slot_state.has_available("sanitarium", "quirk"));

    // Sanitarium disease: 2 slots
    assert!(slot_state.try_consume("sanitarium", "disease"));
    assert!(slot_state.try_consume("sanitarium", "disease"));
    assert!(!slot_state.has_available("sanitarium", "disease"));

    // Tavern bar: 3 slots
    assert!(slot_state.try_consume("tavern", "bar"));
    assert!(slot_state.try_consume("tavern", "bar"));
    assert!(slot_state.try_consume("tavern", "bar"));
    assert!(!slot_state.has_available("tavern", "bar"));

    // Total consumed: 1 + 2 + 3 = 6
    assert_eq!(slot_state.total_consumed(), 6);
}

// ── US-001-a: Deterministic loading tests ─────────────────────────────────────

#[test]
fn registry_loading_is_deterministic() {
    // Parse the registry twice
    let registry1 = parse_all();
    let registry2 = parse_all();

    // They should be identical
    assert_eq!(registry1.len(), registry2.len());

    for id in registry1.all_ids() {
        let building1 = registry1.get(id).expect("building should exist");
        let building2 = registry2.get(id).expect("building should exist");
        assert_eq!(building1, building2, "Building {} should be identical across parses", id);
    }
}

#[test]
fn upgrade_data_preserved_at_all_levels() {
    let registry = parse_all();

    // For each building, verify that all upgrade levels have correct data
    for id in registry.all_ids() {
        let building = registry.get(id).expect("building should exist");

        for tree in &building.upgrade_trees {
            for level in &tree.levels {
                // Cost is u32, always non-negative by type system

                // Effects should have non-NaN values
                for effect in &level.effects {
                    assert!(!effect.value.is_nan(), "Building {} tree {} level {} effect {} has NaN value",
                        id, tree.tree_id, level.code, effect.effect_id);
                    assert!(effect.value.is_finite(), "Building {} tree {} level {} effect {} has infinite value",
                        id, tree.tree_id, level.code, effect.effect_id);
                }
            }
        }
    }
}
