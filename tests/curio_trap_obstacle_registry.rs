//! Integration test for curio, trap, and obstacle registries (US-002).
//!
//! Validates:
//! - CurioRegistry holds all DDGC curio definitions parsed from Curios.csv
//! - TrapRegistry holds all DDGC trap definitions parsed from Traps.json
//! - ObstacleRegistry holds all DDGC obstacle definitions parsed from Obstacles.json
//! - At least 3 curios per dungeon region are parsed (QingLong, BaiHu, ZhuQue, XuanWu)
//! - All 3 trap types and all 5 obstacle types are parsed
//! - Focused test proves registry lookup by ID works
//! - Focused test proves dungeon-scoped curio filtering works
//! - Focused test proves item interaction overrides are preserved

use game_ddgc_headless::contracts::{
    parse::{parse_curios_csv, parse_obstacles_json, parse_traps_json},
    CurioRegistry, DungeonType, ObstacleRegistry, TrapRegistry,
};

fn data_path(filename: &str) -> std::path::PathBuf {
    // Assumes the binary is run from the project root (cargo test --manifest-path=../Cargo.toml)
    // or that the data directory is accessible from the current working directory.
    // For integration tests, we use std::env::current_dir which should be the project root.
    std::path::PathBuf::from("data").join(filename)
}

fn parse_all() -> (CurioRegistry, TrapRegistry, ObstacleRegistry) {
    let curios = parse_curios_csv(&data_path("Curios.csv"))
        .expect("failed to parse Curios.csv");
    let traps = parse_traps_json(&data_path("Traps.json"))
        .expect("failed to parse Traps.json");
    let obstacles = parse_obstacles_json(&data_path("Obstacles.json"))
        .expect("failed to parse Obstacles.json");
    (curios, traps, obstacles)
}

// ── US-002: Registry lookup by ID ───────────────────────────────────────────

#[test]
fn curio_registry_lookup_by_id_works() {
    let (curios, _, _) = parse_all();

    // Verify specific curio IDs exist
    assert!(curios.get("ancient_vase").is_some(), "ancient_vase should exist");
    assert!(curios.get("dusty_chest").is_some(), "dusty_chest should exist");
    assert!(curios.get("mossy_stone").is_some(), "mossy_stone should exist");
    assert!(curios.get("glowing_lantern").is_some(), "glowing_lantern should exist");
}

#[test]
fn trap_registry_lookup_by_id_works() {
    let (_, traps, _) = parse_all();

    // All 3 trap types should exist
    assert!(traps.get("poison_gas").is_some(), "poison_gas should exist");
    assert!(traps.get("floor_spikes").is_some(), "floor_spikes should exist");
    assert!(traps.get("tripwire").is_some(), "tripwire should exist");
}

#[test]
fn obstacle_registry_lookup_by_id_works() {
    let (_, _, obstacles) = parse_all();

    // All 5 obstacle types should exist
    assert!(obstacles.get("thorny_thicket").is_some(), "thorny_thicket should exist");
    assert!(obstacles.get("boulder").is_some(), "boulder should exist");
    assert!(obstacles.get("deep_pit").is_some(), "deep_pit should exist");
    assert!(obstacles.get("locked_door").is_some(), "locked_door should exist");
    assert!(obstacles.get("magical_barrier").is_some(), "magical_barrier should exist");
}

#[test]
fn registry_returns_none_for_unknown_id() {
    let (curios, traps, obstacles) = parse_all();

    assert!(curios.get("nonexistent_curio").is_none(), "unknown curio should return None");
    assert!(traps.get("nonexistent_trap").is_none(), "unknown trap should return None");
    assert!(obstacles.get("nonexistent_obstacle").is_none(), "unknown obstacle should return None");
}

// ── US-002: Dungeon-scoped curio filtering ──────────────────────────────────

#[test]
fn curio_filtering_by_dungeon_qinglong() {
    let (curios, _, _) = parse_all();

    let qinglong_curios = curios.by_dungeon(DungeonType::QingLong);
    assert!(
        qinglong_curios.len() >= 3,
        "At least 3 curios should be available in QingLong, got {}",
        qinglong_curios.len()
    );

    // Verify all returned curios actually have QingLong in their scope
    for curio in &qinglong_curios {
        assert!(
            curio.dungeon_scope.contains(&DungeonType::QingLong),
            "Curio {} claims to be in QingLong but isn't",
            curio.id
        );
    }
}

#[test]
fn curio_filtering_by_dungeon_baihu() {
    let (curios, _, _) = parse_all();

    let baihu_curios = curios.by_dungeon(DungeonType::BaiHu);
    assert!(
        baihu_curios.len() >= 3,
        "At least 3 curios should be available in BaiHu, got {}",
        baihu_curios.len()
    );

    for curio in &baihu_curios {
        assert!(
            curio.dungeon_scope.contains(&DungeonType::BaiHu),
            "Curio {} claims to be in BaiHu but isn't",
            curio.id
        );
    }
}

#[test]
fn curio_filtering_by_dungeon_zhuque() {
    let (curios, _, _) = parse_all();

    let zhuque_curios = curios.by_dungeon(DungeonType::ZhuQue);
    assert!(
        zhuque_curios.len() >= 3,
        "At least 3 curios should be available in ZhuQue, got {}",
        zhuque_curios.len()
    );

    for curio in &zhuque_curios {
        assert!(
            curio.dungeon_scope.contains(&DungeonType::ZhuQue),
            "Curio {} claims to be in ZhuQue but isn't",
            curio.id
        );
    }
}

#[test]
fn curio_filtering_by_dungeon_xuanwu() {
    let (curios, _, _) = parse_all();

    let xuanwu_curios = curios.by_dungeon(DungeonType::XuanWu);
    assert!(
        xuanwu_curios.len() >= 3,
        "At least 3 curios should be available in XuanWu, got {}",
        xuanwu_curios.len()
    );

    for curio in &xuanwu_curios {
        assert!(
            curio.dungeon_scope.contains(&DungeonType::XuanWu),
            "Curio {} claims to be in XuanWu but isn't",
            curio.id
        );
    }
}

// ── US-002: Item interaction overrides ─────────────────────────────────────

#[test]
fn curio_item_interactions_are_preserved() {
    let (curios, _, _) = parse_all();

    // ancient_vase has an item interaction: shovel -> treasure_found
    let ancient_vase = curios.get("ancient_vase").expect("ancient_vase should exist");
    assert!(
        !ancient_vase.item_interactions.is_empty(),
        "ancient_vase should have item interactions"
    );

    let shovel_interaction = ancient_vase
        .item_interactions
        .iter()
        .find(|i| i.item_id == "shovel");
    assert!(
        shovel_interaction.is_some(),
        "ancient_vase should have a shovel interaction"
    );
    assert_eq!(
        shovel_interaction.unwrap().overrides_result_id, "treasure_found",
        "shovel should override with treasure_found result"
    );
}

#[test]
fn dusty_chest_item_interactions_preserved() {
    let (curios, _, _) = parse_all();

    let dusty_chest = curios.get("dusty_chest").expect("dusty_chest should exist");
    assert!(
        !dusty_chest.item_interactions.is_empty(),
        "dusty_chest should have item interactions"
    );

    let key_interaction = dusty_chest
        .item_interactions
        .iter()
        .find(|i| i.item_id == "rusty_key");
    assert!(
        key_interaction.is_some(),
        "dusty_chest should have a rusty_key interaction"
    );
    assert_eq!(
        key_interaction.unwrap().overrides_result_id, "rare_gem",
        "rusty_key should override with rare_gem result"
    );
}

#[test]
fn curio_without_item_interactions_has_empty_list() {
    let (curios, _, _) = parse_all();

    // mossy_stone has empty item_interactions according to the CSV
    let mossy_stone = curios.get("mossy_stone").expect("mossy_stone should exist");
    assert!(
        mossy_stone.item_interactions.is_empty(),
        "mossy_stone should have no item interactions"
    );
}

// ── US-002: Trap difficulty variations ──────────────────────────────────────

#[test]
fn trap_difficulty_variations_are_preserved() {
    let (_, traps, _) = parse_all();

    let poison_gas = traps.get("poison_gas").expect("poison_gas should exist");
    assert!(
        !poison_gas.difficulty_variations.is_empty(),
        "poison_gas should have difficulty variations"
    );

    // Check level 1 variation
    let level1 = poison_gas
        .difficulty_variations
        .iter()
        .find(|v| v.level == 1);
    assert!(level1.is_some(), "poison_gas should have a level 1 variation");

    // Level 1 should have simpler fail effects
    assert!(
        level1.unwrap().fail_effects.len() < poison_gas.fail_effects.len(),
        "Level 1 variation should have fewer fail effects than base"
    );
}

// ── US-002: Obstacle torchlight penalty ─────────────────────────────────────

#[test]
fn obstacle_torchlight_penalty_is_preserved() {
    let (_, _, obstacles) = parse_all();

    let deep_pit = obstacles.get("deep_pit").expect("deep_pit should exist");
    assert!(
        deep_pit.torchlight_penalty > 0.0,
        "deep_pit should have a positive torchlight penalty (harder in darkness)"
    );

    let locked_door = obstacles.get("locked_door").expect("locked_door should exist");
    assert!(
        locked_door.torchlight_penalty < 0.0,
        "locked_door should have a negative torchlight penalty (easier in darkness)"
    );
}

// ── US-002: Completeness checks ─────────────────────────────────────────────

#[test]
fn all_three_trap_types_parsed() {
    let (_, traps, _) = parse_all();
    assert_eq!(traps.len(), 3, "All 3 trap types should be parsed");
}

#[test]
fn all_five_obstacle_types_parsed() {
    let (_, _, obstacles) = parse_all();
    assert_eq!(obstacles.len(), 5, "All 5 obstacle types should be parsed");
}

#[test]
fn at_least_12_curios_parsed() {
    let (curios, _, _) = parse_all();
    // 3 per dungeon * 4 dungeons = minimum 12
    assert!(
        curios.len() >= 12,
        "At least 12 curios should be parsed (3 per dungeon), got {}",
        curios.len()
    );
}
