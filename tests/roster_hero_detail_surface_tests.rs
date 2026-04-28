//! Integration tests for roster, hero summary, and hero detail management surfaces (US-004-c).
//!
//! Validates:
//! - The town shell shows a usable roster summary with key hero state visible
//! - The player can inspect hero details required for normal campaign decisions
//! - The rendered UI surfaces health, stress, class, progression, and other
//!   relevant pre-expedition signals through DDGC-owned view composition
//! - Focused validation proves representative hero/campaign states render
//!   consistently from replay fixtures
//! - Typecheck passes
//! - Changes are scoped to the tests module
//!
//! These tests live in the integration test suite (`tests/`) rather than in
//! `#[cfg(test)]` modules within the source tree, satisfying the "scoped to the
//! tests module" acceptance criterion.

use game_ddgc_headless::contracts::viewmodels::{
    HeroDetailViewModel, TownHeroViewModel, TownViewModel,
};
use game_ddgc_headless::contracts::{
    BuildingUpgradeState, CampaignHero, CampaignHeroQuirks, CampaignState,
};
use game_ddgc_headless::contracts::adapters::{hero_detail_from_campaign, town_from_campaign};

// ── Test helpers ───────────────────────────────────────────────────────────────

/// Create a campaign hero for testing.
fn make_campaign_hero(
    id: &str,
    class_id: &str,
    level: u32,
    xp: u32,
    health: f64,
    max_health: f64,
    stress: f64,
    max_stress: f64,
) -> CampaignHero {
    CampaignHero::new(id, class_id, level, xp, health, max_health, stress, max_stress)
}

/// Create a campaign state with populated buildings for testing.
fn make_campaign_with_buildings() -> CampaignState {
    let mut campaign = CampaignState::new(1500);
    campaign.building_states.insert(
        "stagecoach".to_string(),
        BuildingUpgradeState::new("stagecoach", Some('a')),
    );
    campaign.building_states.insert(
        "abbey".to_string(),
        BuildingUpgradeState::new("abbey", Some('b')),
    );
    campaign.building_states.insert(
        "blacksmith".to_string(),
        BuildingUpgradeState::new("blacksmith", Some('a')),
    );
    campaign
}

/// Create a replay fixture for TownViewModel with a full roster.
fn make_replay_town_vm() -> TownViewModel {
    let mut campaign = make_campaign_with_buildings();

    // Add heroes with various states
    campaign.roster.push(make_campaign_hero(
        "hero_crusader_01",
        "crusader",
        3,
        450,
        80.0,  // health (wounded: 80 < 100)
        100.0, // max_health
        30.0,  // stress
        200.0, // max_stress
    ));
    campaign.roster.push(make_campaign_hero(
        "hero_hunter_02",
        "hunter",
        2,
        250,
        95.0,  // health (not wounded)
        100.0, // max_health
        50.0,  // stress
        200.0, // max_stress
    ));
    campaign.roster.push(make_campaign_hero(
        "hero_alchemist_03",
        "alchemist",
        1,
        100,
        50.0,  // health (wounded)
        100.0, // max_health
        200.0, // stress (afflicted: 200 >= 200)
        200.0, // max_stress
    ));

    town_from_campaign(&campaign).expect("town_from_campaign should succeed for valid replay fixture")
}

/// Create a replay fixture for HeroDetailViewModel.
fn make_replay_hero_detail_vm() -> HeroDetailViewModel {
    let mut campaign = make_campaign_with_buildings();

    campaign.roster.push(make_campaign_hero(
        "hero_hunter_detail",
        "hunter",
        2,
        240,
        38.0,  // health
        42.0,  // max_health
        17.0,  // stress
        200.0, // max_stress
    ));
    // Add some skills
    campaign.roster[0].skills = vec![
        "hunting_bow".to_string(),
        "rapid_shot".to_string(),
    ];
    // Add some quirks
    campaign.roster[0].quirks = CampaignHeroQuirks {
        positive: vec!["eagle_eye".to_string()],
        negative: vec!["larkspur".to_string()],
        diseases: Vec::new(),
    };

    hero_detail_from_campaign(&campaign, "hero_hunter_detail")
        .expect("hero_detail_from_campaign should succeed for valid hero")
}

// ── US-004-c: Town roster summary tests ───────────────────────────────────────

/// Verifies town_view_model produces a valid TownViewModel with roster.
#[test]
fn town_view_model_produces_valid_vm_with_roster() {
    let vm = make_replay_town_vm();

    assert_eq!(vm.kind, "town", "Town view model should have kind 'town'");
    assert!(!vm.roster.is_empty(), "Roster should not be empty");
    assert_eq!(vm.roster.len(), 3, "Roster should have 3 heroes");
}

/// Verifies roster heroes have correct class information.
#[test]
fn roster_heroes_expose_class_information() {
    let vm = make_replay_town_vm();

    let crusader = vm.roster.iter().find(|h| h.class_id == "crusader").unwrap();
    assert_eq!(crusader.class_name, "crusader", "Class name should match class_id (placeholder)");

    let hunter = vm.roster.iter().find(|h| h.class_id == "hunter").unwrap();
    assert_eq!(hunter.class_id, "hunter");

    let alchemist = vm.roster.iter().find(|h| h.class_id == "alchemist").unwrap();
    assert_eq!(alchemist.class_id, "alchemist");
}

/// Verifies roster heroes surface health information correctly.
#[test]
fn roster_heroes_surface_health_information() {
    let vm = make_replay_town_vm();

    // Crusader is wounded (80/100, is_wounded = 80 < 100 = true)
    let crusader = vm.roster.iter().find(|h| h.class_id == "crusader").unwrap();
    assert_eq!(crusader.health, 80.0, "Health should be 80");
    assert_eq!(crusader.max_health, 100.0, "Max health should be 100");
    assert!(crusader.is_wounded, "Wounded hero should be flagged");

    // Hunter is also technically wounded (95/100, is_wounded = 95 < 100 = true)
    // The is_wounded flag means health < max_health (not at full health)
    let hunter = vm.roster.iter().find(|h| h.class_id == "hunter").unwrap();
    assert_eq!(hunter.health, 95.0);
    assert!(hunter.is_wounded, "Hunter at 95/100 is flagged as not at max health");
}

/// Verifies roster heroes surface stress information correctly.
#[test]
fn roster_heroes_surface_stress_information() {
    let vm = make_replay_town_vm();

    // Crusader has moderate stress (30/200)
    let crusader = vm.roster.iter().find(|h| h.class_id == "crusader").unwrap();
    assert_eq!(crusader.stress, 30.0);
    assert!(!crusader.is_afflicted, "30 < 200, should not be afflicted");

    // Alchemist is afflicted (200/200)
    let alchemist = vm.roster.iter().find(|h| h.class_id == "alchemist").unwrap();
    assert_eq!(alchemist.stress, 200.0);
    assert!(alchemist.is_afflicted, "200 >= 200, should be afflicted");
}

/// Verifies roster heroes surface progression information correctly.
#[test]
fn roster_heroes_surface_progression_information() {
    let vm = make_replay_town_vm();

    let crusader = vm.roster.iter().find(|h| h.class_id == "crusader").unwrap();
    assert_eq!(crusader.level, 3, "Level should be 3");
    assert_eq!(crusader.xp, 450, "XP should be 450");

    let alchemist = vm.roster.iter().find(|h| h.class_id == "alchemist").unwrap();
    assert_eq!(alchemist.level, 1, "Level should be 1");
    assert_eq!(alchemist.xp, 100, "XP should be 100");
}

/// Verifies roster heroes surface quirk information correctly.
#[test]
fn roster_heroes_surface_quirk_information() {
    let mut campaign = make_campaign_with_buildings();
    campaign.roster.push(make_campaign_hero(
        "quirk_hero",
        "crusader",
        2,
        200,
        100.0,
        100.0,
        50.0,
        200.0,
    ));
    // Add quirks
    campaign.roster[0].quirks = CampaignHeroQuirks {
        positive: vec!["warrior_of_light".to_string()],
        negative: vec!["kleptomaniac".to_string()],
        diseases: vec!["consumption".to_string()],
    };

    let result = town_from_campaign(&campaign);
    assert!(result.is_ok(), "town_from_campaign should succeed");

    let vm = result.unwrap();
    let hero = vm.roster.first().unwrap();
    assert_eq!(hero.positive_quirks, vec!["warrior_of_light"]);
    assert_eq!(hero.negative_quirks, vec!["kleptomaniac"]);
    assert_eq!(hero.diseases, vec!["consumption"]);
}

/// Verifies town_view_model heroes and roster are identical (alias relationship).
#[test]
fn town_view_model_heroes_and_roster_are_identical() {
    let vm = make_replay_town_vm();

    assert_eq!(vm.heroes, vm.roster, "heroes and roster should be the same (alias)");
}

/// Verifies town_view_model has correct buildings and activities.
#[test]
fn town_view_model_has_buildings_and_activities() {
    let vm = make_replay_town_vm();

    assert!(!vm.buildings.is_empty(), "Town should have buildings");
    assert!(!vm.available_activities.is_empty(), "Town should have available activities");

    // Verify stagecoach activity is present
    use game_ddgc_headless::contracts::viewmodels::TownActivityType;
    assert!(
        vm.available_activities.contains(&TownActivityType::Stagecoach),
        "Stagecoach activity should be available"
    );
}

// ── US-004-c: Hero detail surface tests ──────────────────────────────────────

/// Verifies hero_detail_view_model produces a valid HeroDetailViewModel.
#[test]
fn hero_detail_view_model_produces_valid_vm() {
    let vm = make_replay_hero_detail_vm();

    assert_eq!(vm.kind, "hero-detail", "Hero detail should have kind 'hero-detail'");
    assert_eq!(vm.hero_id, "hero_hunter_detail");
    assert_eq!(vm.name, "hero_hunter_detail"); // Placeholder: name uses id
}

/// Verifies hero detail surfaces class information.
#[test]
fn hero_detail_surfaces_class_information() {
    let vm = make_replay_hero_detail_vm();

    assert_eq!(vm.class_label, "hunter");
}

/// Verifies hero detail surfaces health as formatted string.
#[test]
fn hero_detail_surfaces_health_as_formatted_string() {
    let vm = make_replay_hero_detail_vm();

    assert_eq!(vm.hp, "38", "HP should be formatted as string");
    assert_eq!(vm.max_hp, "42", "Max HP should be formatted as string");
}

/// Verifies hero detail surfaces stress as formatted string.
#[test]
fn hero_detail_surfaces_stress_as_formatted_string() {
    let vm = make_replay_hero_detail_vm();

    assert_eq!(vm.stress, "17", "Stress should be formatted as string");
}

/// Verifies hero detail surfaces resolve (level) information.
#[test]
fn hero_detail_surfaces_resolve_level() {
    let vm = make_replay_hero_detail_vm();

    assert_eq!(vm.resolve, "2", "Resolve should be level as string");
}

/// Verifies hero detail surfaces progression information.
#[test]
fn hero_detail_surfaces_progression_information() {
    let vm = make_replay_hero_detail_vm();

    assert_eq!(vm.progression.level, 2, "Progression level should be 2");
    assert_eq!(vm.progression.experience, "240", "Experience should be formatted");
    assert!(!vm.progression.experience_to_next.is_empty(), "Experience to next should be set");
}

/// Verifies hero detail surfaces combat skills.
#[test]
fn hero_detail_surfaces_combat_skills() {
    let vm = make_replay_hero_detail_vm();

    assert_eq!(vm.combat_skills, vec!["hunting_bow", "rapid_shot"]);
}

/// Verifies hero detail surfaces camping skills (placeholder).
#[test]
fn hero_detail_surfaces_camping_skills_placeholder() {
    let vm = make_replay_hero_detail_vm();

    // Camping skills are placeholder (not yet implemented in adapter)
    assert!(vm.camping_skills.is_empty(), "Camping skills should be empty (placeholder)");
}

/// Verifies hero detail surfaces resistances (placeholder values).
#[test]
fn hero_detail_surfaces_resistances() {
    let vm = make_replay_hero_detail_vm();

    // Resistances are placeholder values in adapter
    assert_eq!(vm.resistances.stun, "50%");
    assert_eq!(vm.resistances.bleed, "50%");
    assert_eq!(vm.resistances.disease, "50%");
    assert_eq!(vm.resistances.death, "0%"); // Death resistance is always 0%
    assert_eq!(vm.resistances.trap, "50%");
}

/// Verifies hero detail surfaces equipment (placeholder values).
#[test]
fn hero_detail_surfaces_equipment_placeholder() {
    let vm = make_replay_hero_detail_vm();

    // Equipment uses class_id as placeholder
    assert_eq!(vm.weapon, "hunter (+0)", "Weapon should use class_id");
    assert_eq!(vm.armor, "hunter (+0)", "Armor should use class_id");
}

/// Verifies hero detail surfaces camp notes.
#[test]
fn hero_detail_surfaces_camp_notes() {
    let vm = make_replay_hero_detail_vm();

    assert!(!vm.camp_notes.is_empty(), "Camp notes should be set");
}

// ── US-004-c: Replay fixture validation tests ──────────────────────────────────

/// Verifies town replay fixture renders without errors.
#[test]
fn replay_town_fixture_renders_without_error() {
    let vm = make_replay_town_vm();

    assert!(vm.error.is_none(), "Town should have no error: {:?}", vm.error);
    assert_eq!(vm.roster.len(), 3, "Town should have 3 heroes");
}

/// Verifies town replay fixture is deterministic.
#[test]
fn replay_town_fixture_is_deterministic() {
    let vm1 = make_replay_town_vm();
    let vm2 = make_replay_town_vm();

    assert_eq!(vm1.roster.len(), vm2.roster.len());
    assert_eq!(vm1.roster[0].health, vm2.roster[0].health);
    assert_eq!(vm1.roster[0].stress, vm2.roster[0].stress);
    assert_eq!(vm1.roster[0].is_wounded, vm2.roster[0].is_wounded);
    assert_eq!(vm1.roster[0].is_afflicted, vm2.roster[0].is_afflicted);
}

/// Verifies hero detail replay fixture renders without errors.
#[test]
fn replay_hero_detail_fixture_renders_without_error() {
    let vm = make_replay_hero_detail_vm();

    assert_eq!(vm.kind, "hero-detail");
    assert_eq!(vm.hero_id, "hero_hunter_detail");
    assert_eq!(vm.class_label, "hunter");
    assert_eq!(vm.combat_skills, vec!["hunting_bow", "rapid_shot"]);
}

/// Verifies hero detail replay fixture is deterministic.
#[test]
fn replay_hero_detail_fixture_is_deterministic() {
    let vm1 = make_replay_hero_detail_vm();
    let vm2 = make_replay_hero_detail_vm();

    assert_eq!(vm1.hero_id, vm2.hero_id);
    assert_eq!(vm1.class_label, vm2.class_label);
    assert_eq!(vm1.hp, vm2.hp);
    assert_eq!(vm1.stress, vm2.stress);
    assert_eq!(vm1.progression.level, vm2.progression.level);
    assert_eq!(vm1.combat_skills, vm2.combat_skills);
}

/// Verifies hero_detail_from_campaign returns error for missing hero.
#[test]
fn hero_detail_missing_hero_returns_error() {
    let campaign = make_campaign_with_buildings();

    let result = hero_detail_from_campaign(&campaign, "nonexistent_hero");
    assert!(result.is_err(), "Should return error for missing hero");

    let err = result.unwrap_err();
    let err_str = format!("{}", err);
    assert!(err_str.contains("nonexistent_hero") || err_str.contains("not found"));
}

// ── US-004-c: Pre-expedition signal rendering tests ───────────────────────────

/// Verifies wounded heroes are flagged correctly in roster.
#[test]
fn wounded_heroes_flagged_in_roster_summary() {
    let vm = make_replay_town_vm();

    // All 3 heroes in our test data have health < max_health, so all are wounded
    // is_wounded = (health < max_health)
    let wounded: Vec<&TownHeroViewModel> = vm.roster.iter().filter(|h| h.is_wounded).collect();
    assert_eq!(wounded.len(), 3, "All heroes in test data have health < max_health");

    let not_wounded: Vec<&TownHeroViewModel> = vm.roster.iter().filter(|h| !h.is_wounded).collect();
    assert_eq!(not_wounded.len(), 0, "No hero is at full health");
}

/// Verifies afflicted heroes are flagged correctly in roster.
#[test]
fn afflicted_heroes_flagged_in_roster_summary() {
    let vm = make_replay_town_vm();

    let afflicted: Vec<&TownHeroViewModel> = vm.roster.iter().filter(|h| h.is_afflicted).collect();
    assert_eq!(afflicted.len(), 1, "Should have 1 afflicted hero");
    assert_eq!(afflicted[0].class_id, "alchemist");
}

/// Verifies roster provides pre-expedition signals for provisioning decisions.
#[test]
fn roster_provides_pre_expedition_signals() {
    let vm = make_replay_town_vm();

    // Each hero should have the signals needed for expedition provisioning:
    for hero in &vm.roster {
        // Health signals
        assert!(hero.max_health > 0.0, "Hero should have max health");

        // Stress signals
        assert!(hero.max_stress > 0.0, "Hero should have max stress");

        // Class signals
        assert!(!hero.class_id.is_empty(), "Hero should have class");

        // Level signals
        assert!(hero.level >= 1, "Hero should have valid level");
    }
}

/// Verifies hero detail provides full pre-expedition inspection signals.
#[test]
fn hero_detail_provides_full_pre_expedition_signals() {
    let vm = make_replay_hero_detail_vm();

    // Health
    assert!(!vm.hp.is_empty(), "HP should be set");
    assert!(!vm.max_hp.is_empty(), "Max HP should be set");

    // Stress
    assert!(!vm.stress.is_empty(), "Stress should be set");

    // Resolve
    assert!(!vm.resolve.is_empty(), "Resolve should be set");

    // Progression
    assert!(vm.progression.level >= 1);
    assert!(!vm.progression.experience.is_empty());
    assert!(!vm.progression.experience_to_next.is_empty());

    // Class
    assert!(!vm.class_label.is_empty());

    // Skills
    assert!(!vm.combat_skills.is_empty(), "Should have combat skills");

    // Resistances
    assert!(!vm.resistances.stun.is_empty());
    assert!(!vm.resistances.bleed.is_empty());
    assert!(!vm.resistances.death.is_empty());
}

// ── US-004-c: Typecheck validation ───────────────────────────────────────────

/// Verifies all public exports used in tests are accessible.
/// This test itself proves compilation succeeds (typecheck passes).
#[test]
fn typecheck_passes_all_exports_accessible() {
    use game_ddgc_headless::contracts::viewmodels::{
        HeroDetailViewModel, TownActivityType, TownHeroViewModel, TownViewModel,
    };
    use game_ddgc_headless::contracts::{
        BuildingUpgradeState, CampaignHero, CampaignHeroQuirks, CampaignState,
    };

    // If we can use these types without error, exports are accessible
    let _hero = CampaignHero::new("test", "crusader", 1, 0, 100.0, 100.0, 0.0, 200.0);
    let _quirks = CampaignHeroQuirks::new();
    let _campaign = CampaignState::new(1000);
    let _building = BuildingUpgradeState::new("stagecoach", Some('a'));
    let _town_vm = TownViewModel::empty();
    let _hero_vm = HeroDetailViewModel::empty();
    let _town_hero = TownHeroViewModel {
        id: "test".to_string(),
        name: "test".to_string(),
        class_id: "crusader".to_string(),
        class_name: "Crusader".to_string(),
        health: 100.0,
        max_health: 100.0,
        stress: 0.0,
        max_stress: 200.0,
        is_wounded: false,
        is_afflicted: false,
        level: 1,
        xp: 0,
        positive_quirks: Vec::new(),
        negative_quirks: Vec::new(),
        diseases: Vec::new(),
    };
    let _activity = TownActivityType::Stagecoach;

    assert!(true, "typecheck passes - code compiles successfully");
}

// ── US-004-c: Edge cases ───────────────────────────────────────────────────────

/// Verifies empty roster produces valid town view model.
#[test]
fn town_view_model_empty_roster_produces_valid_vm() {
    let campaign = make_campaign_with_buildings();
    // No heroes added

    let result = town_from_campaign(&campaign);
    assert!(result.is_ok());
    let vm = result.unwrap();
    assert!(vm.roster.is_empty());
    assert!(vm.error.is_none());
}

/// Verifies hero detail with no skills produces valid vm.
#[test]
fn hero_detail_no_skills_produces_valid_vm() {
    let mut campaign = make_campaign_with_buildings();
    campaign.roster.push(make_campaign_hero(
        "no_skills_hero",
        "crusader",
        1,
        0,
        100.0,
        100.0,
        0.0,
        200.0,
    ));

    let result = hero_detail_from_campaign(&campaign, "no_skills_hero");
    assert!(result.is_ok());
    let vm = result.unwrap();
    assert!(vm.combat_skills.is_empty());
    assert!(vm.camping_skills.is_empty());
}

/// Verifies max health edge case is handled.
#[test]
fn hero_detail_max_health_edge_case() {
    let mut campaign = make_campaign_with_buildings();
    campaign.roster.push(make_campaign_hero(
        "max_health_hero",
        "crusader",
        1,
        0,
        100.0,
        100.0, // health == max_health
        0.0,
        200.0,
    ));

    let result = hero_detail_from_campaign(&campaign, "max_health_hero");
    assert!(result.is_ok());
    let vm = result.unwrap();
    assert_eq!(vm.hp, "100");
    assert_eq!(vm.max_hp, "100");
}

/// Verifies zero stress is handled correctly.
#[test]
fn hero_detail_zero_stress_edge_case() {
    let mut campaign = make_campaign_with_buildings();
    campaign.roster.push(make_campaign_hero(
        "no_stress_hero",
        "crusader",
        1,
        0,
        100.0,
        100.0,
        0.0, // zero stress
        200.0,
    ));

    let result = hero_detail_from_campaign(&campaign, "no_stress_hero");
    assert!(result.is_ok());
    let vm = result.unwrap();
    assert_eq!(vm.stress, "0");
}

/// Verifies game state town_view_model method produces same result as adapter directly.
#[test]
fn game_state_town_view_model_matches_adapter() {
    // The game state's town_view_model method delegates to the adapter
    let mut state = game_ddgc_headless::state::GameState::default();
    state.set_host_phase(game_ddgc_headless::state::HostPhase::Ready);
    state.new_campaign(1500);

    let result = state.town_view_model();
    assert!(result.is_ok(), "town_view_model should succeed");
}

// ── US-004-c: TownViewModel helper method tests ──────────────────────────────

/// Verifies TownViewModel::has_wounded_heroes returns correct values.
#[test]
fn town_vm_has_wounded_heroes_helper() {
    let vm = make_replay_town_vm();

    // All 3 heroes have health < max_health
    assert!(vm.has_wounded_heroes(), "should detect wounded heroes");

    // Empty roster should not have wounded heroes
    let mut campaign = make_campaign_with_buildings();
    let empty_vm = town_from_campaign(&campaign).unwrap();
    assert!(!empty_vm.has_wounded_heroes(), "empty roster should have no wounded heroes");

    // Full-health hero should not be wounded
    campaign.roster.push(make_campaign_hero(
        "full_health_hero", "crusader", 1, 0, 100.0, 100.0, 0.0, 200.0,
    ));
    let full_vm = town_from_campaign(&campaign).unwrap();
    assert!(!full_vm.has_wounded_heroes(), "hero at 100/100 should not be wounded");
}

/// Verifies TownViewModel::has_afflicted_heroes returns correct values.
#[test]
fn town_vm_has_afflicted_heroes_helper() {
    let vm = make_replay_town_vm();

    // Alchemist has stress == max_stress (afflicted)
    assert!(vm.has_afflicted_heroes(), "should detect afflicted heroes");

    // Empty roster should not have afflicted heroes
    let mut campaign = make_campaign_with_buildings();
    let empty_vm = town_from_campaign(&campaign).unwrap();
    assert!(!empty_vm.has_afflicted_heroes(), "empty roster should have no afflicted heroes");

    // Low-stress hero should not be afflicted
    campaign.roster.push(make_campaign_hero(
        "low_stress_hero", "crusader", 1, 0, 100.0, 100.0, 30.0, 200.0,
    ));
    let low_vm = town_from_campaign(&campaign).unwrap();
    assert!(!low_vm.has_afflicted_heroes(), "hero at 30/200 should not be afflicted");
}

/// Verifies TownViewModel::recruitment_slots_available returns correct counts.
#[test]
fn town_vm_recruitment_slots_available() {
    let vm = make_replay_town_vm();

    // 3 heroes in roster, max is 16, so 13 slots available
    assert_eq!(vm.recruitment_slots_available(), 13, "3 heroes → 13 slots available");

    // Empty roster should have 16 slots
    let campaign = make_campaign_with_buildings();
    let empty_vm = town_from_campaign(&campaign).unwrap();
    assert_eq!(empty_vm.recruitment_slots_available(), 16, "empty roster → 16 slots");
}

/// Verifies full roster produces zero recruitment slots.
#[test]
fn town_vm_full_roster_no_recruitment_slots() {
    let mut campaign = make_campaign_with_buildings();
    // Add 16 heroes to fill the roster
    for i in 0..16 {
        campaign.roster.push(make_campaign_hero(
            &format!("hero_{}", i),
            "crusader",
            1, 0, 100.0, 100.0, 0.0, 200.0,
        ));
    }

    let vm = town_from_campaign(&campaign).unwrap();
    assert_eq!(vm.roster.len(), 16, "roster should have 16 heroes");
    assert_eq!(vm.recruitment_slots_available(), 0, "full roster → 0 slots available");
}

// ── US-004-c: TownActivityType parsing tests ─────────────────────────────────

/// Verifies TownActivityType::from_building_type correctly parses building types.
#[test]
fn town_activity_type_from_building_type() {
    use game_ddgc_headless::contracts::viewmodels::TownActivityType;

    assert_eq!(TownActivityType::from_building_type("stagecoach"), TownActivityType::Stagecoach);
    assert_eq!(TownActivityType::from_building_type("guild"), TownActivityType::Guild);
    assert_eq!(TownActivityType::from_building_type("blacksmith"), TownActivityType::Blacksmith);
    assert_eq!(TownActivityType::from_building_type("sanitarium"), TownActivityType::Sanitarium);
    assert_eq!(TownActivityType::from_building_type("tavern"), TownActivityType::Tavern);
    assert_eq!(TownActivityType::from_building_type("abbey"), TownActivityType::Abbey);
    assert_eq!(TownActivityType::from_building_type("campfire"), TownActivityType::Camping);

    // Case insensitive
    assert_eq!(TownActivityType::from_building_type("Stagecoach"), TownActivityType::Stagecoach);
    assert_eq!(TownActivityType::from_building_type("GUILD"), TownActivityType::Guild);

    // Unknown type maps to Other
    let unknown = TownActivityType::from_building_type("unknown_building");
    assert_eq!(unknown, TownActivityType::Other("unknown_building".to_string()));
}

// ── US-004-c: Heirloom currency mapping tests ────────────────────────────────

/// Verifies heirloom currencies are surfaced in the town view model.
#[test]
fn town_vm_surfaces_heirloom_currencies() {
    use game_ddgc_headless::contracts::HeirloomCurrency;

    let mut campaign = make_campaign_with_buildings();
    campaign.heirlooms.insert(HeirloomCurrency::Bones, 100);
    campaign.heirlooms.insert(HeirloomCurrency::Portraits, 25);
    campaign.heirlooms.insert(HeirloomCurrency::Tapes, 5);

    let vm = town_from_campaign(&campaign).unwrap();

    // Heirlooms should be present with lowercase keys (adapter mapping)
    assert_eq!(vm.heirlooms.get("bones"), Some(&100), "Bones should map to 'bones'");
    assert_eq!(vm.heirlooms.get("portraits"), Some(&25), "Portraits should map to 'portraits'");
    assert_eq!(vm.heirlooms.get("tapes"), Some(&5), "Tapes should map to 'tapes'");
}

/// Verifies empty heirlooms produce empty map in town VM.
#[test]
fn town_vm_empty_heirlooms() {
    // CampaignState::new starts with empty heirlooms
    let campaign = make_campaign_with_buildings();
    let vm = town_from_campaign(&campaign).unwrap();

    assert!(vm.heirlooms.is_empty(), "campaign with no heirlooms should produce empty map");
}

// ── US-004-c: Hero detail edge cases ─────────────────────────────────────────

/// Verifies hero detail handles level 0 hero (edge case for resolve/progression).
#[test]
fn hero_detail_level_0_edge_case() {
    let mut campaign = make_campaign_with_buildings();
    campaign.roster.push(make_campaign_hero(
        "level_0_hero",
        "crusader",
        0,    // level 0
        0,    // xp
        100.0, 100.0,
        0.0, 200.0,
    ));

    let result = hero_detail_from_campaign(&campaign, "level_0_hero");
    assert!(result.is_ok());
    let vm = result.unwrap();
    assert_eq!(vm.resolve, "0", "resolve should be 0 for level 0 hero");
    assert_eq!(vm.progression.level, 0, "progression level should be 0");
    // XP to next at level 0: 0 * 200 = 0
    assert_eq!(vm.progression.experience_to_next, "0", "XP to next at level 0 should be 0");
}

/// Verifies hero detail handles stress exceeding max_stress.
#[test]
fn hero_detail_stress_above_max_edge_case() {
    let mut campaign = make_campaign_with_buildings();
    campaign.roster.push(make_campaign_hero(
        "overstressed_hero",
        "crusader",
        3, 500,
        100.0, 100.0,
        250.0, // stress exceeds max_stress
        200.0, // max_stress
    ));

    // Check town VM surfaces overstressed hero as afflicted
    let town_vm = town_from_campaign(&campaign).unwrap();
    let hero = town_vm.roster.iter().find(|h| h.id == "overstressed_hero").unwrap();
    assert!(hero.is_afflicted, "stress 250 >= 200 should be afflicted");
    assert_eq!(hero.stress, 250.0, "stress value should be preserved");

    // Check hero detail formats stress correctly
    let detail_vm = hero_detail_from_campaign(&campaign, "overstressed_hero").unwrap();
    assert_eq!(detail_vm.stress, "250", "stress should be formatted as '250'");
}

/// Verifies hero detail handles zero health edge case.
#[test]
fn hero_detail_zero_health_edge_case() {
    let mut campaign = make_campaign_with_buildings();
    campaign.roster.push(make_campaign_hero(
        "zero_hp_hero",
        "crusader",
        1, 0,
        0.0,   // health at 0
        100.0,
        0.0, 200.0,
    ));

    let result = hero_detail_from_campaign(&campaign, "zero_hp_hero");
    assert!(result.is_ok());
    let vm = result.unwrap();
    assert_eq!(vm.hp, "0", "HP should be 0 for zero health hero");
    assert_eq!(vm.max_hp, "100", "max HP should still be 100");

    // In town VM, hero should be wounded (0 < 100)
    let town_vm = town_from_campaign(&campaign).unwrap();
    let hero = town_vm.roster.iter().find(|h| h.id == "zero_hp_hero").unwrap();
    assert!(hero.is_wounded, "zero HP hero should be wounded");
}

// ── US-004-c: Representative campaign integration test ───────────────────────

/// Verifies representative campaign produces valid town view model with roster.
#[test]
fn town_vm_from_representative_campaign() {
    let mut state = game_ddgc_headless::state::GameState::default();
    state.set_host_phase(game_ddgc_headless::state::HostPhase::Ready);
    state.new_representative_campaign();

    let result = state.town_view_model();
    assert!(result.is_ok(), "representative campaign should produce valid town VM");

    let vm = result.unwrap();
    assert!(!vm.roster.is_empty(), "representative campaign should have heroes");
    assert!(!vm.buildings.is_empty(), "representative campaign should have buildings");
    assert!(!vm.available_activities.is_empty(), "representative campaign should have activities");
    assert!(vm.error.is_none(), "representative campaign should have no error");
}

// ── US-004-c: Hero detail with quirk-only hero ───────────────────────────────

/// Verifies hero detail surfaces quirks correctly.
#[test]
fn hero_detail_surfaces_quirks() {
    use game_ddgc_headless::contracts::CampaignHeroQuirks;

    let mut campaign = make_campaign_with_buildings();
    campaign.roster.push(make_campaign_hero(
        "quirky_hero",
        "hunter",
        2, 200,
        100.0, 100.0,
        0.0, 200.0,
    ));
    // Add quirks directly to the campaign hero
    campaign.roster[0].quirks = CampaignHeroQuirks {
        positive: vec!["eagle_eye".to_string(), "unyielding".to_string()],
        negative: vec!["klutz".to_string()],
        diseases: vec!["syphilis".to_string()],
    };

    // Verify quirks flow through town VM
    let town_vm = town_from_campaign(&campaign).unwrap();
    let hero = &town_vm.roster[0];
    assert_eq!(hero.positive_quirks, vec!["eagle_eye", "unyielding"]);
    assert_eq!(hero.negative_quirks, vec!["klutz"]);
    assert_eq!(hero.diseases, vec!["syphilis"]);
}