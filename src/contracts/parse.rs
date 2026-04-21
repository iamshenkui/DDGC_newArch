//! Parsing utilities for DDGC data files (Curios.csv, Traps.json, Obstacles.json, Buildings.json).

use std::path::Path;

use serde::Deserialize;
use crate::contracts::{
    BuildingRegistry, BuildingType, CurioDefinition, CurioRegistry, CurioResult,
    CurioResultType, DungeonType, ItemInteraction, ObstacleDefinition, ObstacleRegistry,
    TownBuilding, TrapDefinition, TrapRegistry, TrapVariation, UnlockCondition,
    UpgradeEffect, UpgradeLevel, UpgradeTree,
};

/// Parse a JSON-encoded dungeon scope string into a Vec<DungeonType>.
fn parse_dungeon_scope(s: &str) -> Result<Vec<DungeonType>, String> {
    // The scope is stored as JSON array string like "[\"QingLong\",\"BaiHu\"]"
    let s = s.trim();
    if s.is_empty() {
        return Ok(vec![]);
    }

    // Parse the JSON array
    let parsed: Vec<String> = serde_json::from_str(s)
        .map_err(|e| format!("failed to parse dungeon_scope JSON: {}", e))?;

    parsed
        .iter()
        .map(|name| match name.as_str() {
            "QingLong" => Ok(DungeonType::QingLong),
            "BaiHu" => Ok(DungeonType::BaiHu),
            "ZhuQue" => Ok(DungeonType::ZhuQue),
            "XuanWu" => Ok(DungeonType::XuanWu),
            _ => Err(format!("unknown dungeon type: {}", name)),
        })
        .collect()
}

/// Parse a JSON-encoded results string into a Vec<CurioResult>.
fn parse_curio_results(s: &str) -> Result<Vec<CurioResult>, String> {
    if s.is_empty() {
        return Ok(vec![]);
    }

    #[derive(Deserialize)]
    struct RawCurioResult {
        weight: u32,
        chance: f64,
        result_type: String,
        result_id: String,
    }

    let raw: Vec<RawCurioResult> = serde_json::from_str(s)
        .map_err(|e| format!("failed to parse results JSON: {}", e))?;

    raw.iter()
        .map(|r| {
            let result_type = match r.result_type.as_str() {
                "Nothing" => CurioResultType::Nothing,
                "Loot" => CurioResultType::Loot,
                "Quirk" => CurioResultType::Quirk,
                "Effect" => CurioResultType::Effect,
                "Purge" => CurioResultType::Purge,
                "Scouting" => CurioResultType::Scouting,
                "Teleport" => CurioResultType::Teleport,
                "Disease" => CurioResultType::Disease,
                _ => return Err(format!("unknown CurioResultType: {}", r.result_type)),
            };
            Ok(CurioResult::new(r.weight, r.chance, result_type, &r.result_id))
        })
        .collect()
}

/// Parse a JSON-encoded item interactions string into a Vec<ItemInteraction>.
fn parse_item_interactions(s: &str) -> Result<Vec<ItemInteraction>, String> {
    if s.is_empty() {
        return Ok(vec![]);
    }

    #[derive(Deserialize)]
    struct RawItemInteraction {
        item_id: String,
        overrides_result_id: String,
    }

    let raw: Vec<RawItemInteraction> = serde_json::from_str(s)
        .map_err(|e| format!("failed to parse item_interactions JSON: {}", e))?;

    Ok(raw
        .into_iter()
        .map(|r| ItemInteraction::new(&r.item_id, &r.overrides_result_id))
        .collect())
}

/// Parse Curios.csv into a CurioRegistry.
pub fn parse_curios_csv(path: &Path) -> Result<CurioRegistry, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read Curios.csv: {}", e))?;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(content.as_bytes());

    let mut registry = CurioRegistry::new();

    for result in reader.records() {
        let record = result.map_err(|e| format!("CSV parse error: {}", e))?;

        if record.len() < 4 {
            return Err(format!("CSV record has too few fields: {:?}", record));
        }

        let id = record.get(0).ok_or("missing id field")?;
        let dungeon_scope_str = record.get(1).ok_or("missing dungeon_scope field")?;
        let results_str = record.get(2).ok_or("missing results field")?;
        let item_interactions_str = record.get(3).ok_or("missing item_interactions field")?;

        let dungeon_scope = parse_dungeon_scope(dungeon_scope_str)?;
        let results = parse_curio_results(results_str)?;
        let item_interactions = parse_item_interactions(item_interactions_str)?;

        let curio = CurioDefinition::new(id, dungeon_scope, results, item_interactions);
        registry.register(curio);
    }

    Ok(registry)
}

/// Parse Traps.json into a TrapRegistry.
pub fn parse_traps_json(path: &Path) -> Result<TrapRegistry, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read Traps.json: {}", e))?;

    #[derive(Deserialize)]
    struct RawTrapVariation {
        level: u32,
        fail_effects: Vec<String>,
        health_fraction: f64,
    }

    #[derive(Deserialize)]
    struct RawTrap {
        id: String,
        success_effects: Vec<String>,
        fail_effects: Vec<String>,
        health_fraction: f64,
        difficulty_variations: Vec<RawTrapVariation>,
    }

    #[derive(Deserialize)]
    struct RawTrapsRoot {
        traps: Vec<RawTrap>,
    }

    let root: RawTrapsRoot = serde_json::from_str(&content)
        .map_err(|e| format!("failed to parse Traps.json: {}", e))?;

    let mut registry = TrapRegistry::new();

    for raw in root.traps {
        let variations: Vec<TrapVariation> = raw
            .difficulty_variations
            .into_iter()
            .map(|v| TrapVariation::new(v.level, v.fail_effects, v.health_fraction))
            .collect();

        let trap = TrapDefinition::new(
            &raw.id,
            raw.success_effects,
            raw.fail_effects,
            raw.health_fraction,
            variations,
        );
        registry.register(trap);
    }

    Ok(registry)
}

/// Parse Obstacles.json into an ObstacleRegistry.
pub fn parse_obstacles_json(path: &Path) -> Result<ObstacleRegistry, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read Obstacles.json: {}", e))?;

    #[derive(Deserialize)]
    struct RawObstacle {
        id: String,
        fail_effects: Vec<String>,
        health_fraction: f64,
        torchlight_penalty: f64,
    }

    #[derive(Deserialize)]
    struct RawObstaclesRoot {
        obstacles: Vec<RawObstacle>,
    }

    let root: RawObstaclesRoot = serde_json::from_str(&content)
        .map_err(|e| format!("failed to parse Obstacles.json: {}", e))?;

    let mut registry = ObstacleRegistry::new();

    for raw in root.obstacles {
        let obstacle = ObstacleDefinition::new(
            &raw.id,
            raw.fail_effects,
            raw.health_fraction,
            raw.torchlight_penalty,
        );
        registry.register(obstacle);
    }

    Ok(registry)
}

/// Parse Buildings.json into a BuildingRegistry.
pub fn parse_buildings_json(path: &Path) -> Result<BuildingRegistry, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read Buildings.json: {}", e))?;

    #[derive(Deserialize)]
    struct RawUnlockCondition {
        condition_type: String,
        required_count: u32,
    }

    #[derive(Deserialize)]
    struct RawUpgradeEffect {
        effect_id: String,
        value: f64,
    }

    #[derive(Deserialize)]
    struct RawUpgradeLevel {
        code: char,
        cost: u32,
        effects: Vec<RawUpgradeEffect>,
    }

    #[derive(Deserialize)]
    struct RawUpgradeTree {
        tree_id: String,
        levels: Vec<RawUpgradeLevel>,
    }

    #[derive(Deserialize)]
    struct RawTownBuilding {
        id: String,
        building_type: String,
        unlock_conditions: Vec<RawUnlockCondition>,
        upgrade_trees: Vec<RawUpgradeTree>,
    }

    #[derive(Deserialize)]
    struct RawBuildingsRoot {
        buildings: Vec<RawTownBuilding>,
    }

    let root: RawBuildingsRoot = serde_json::from_str(&content)
        .map_err(|e| format!("failed to parse Buildings.json: {}", e))?;

    let mut registry = BuildingRegistry::new();

    for raw in root.buildings {
        let building_type = match raw.building_type.as_str() {
            "Barracks" => BuildingType::Barracks,
            "Blacksmith" => BuildingType::Blacksmith,
            "Campfire" => BuildingType::Campfire,
            "Cathedral" => BuildingType::Cathedral,
            "Confectionery" => BuildingType::Confectionery,
            "DilapidatedShrine" => BuildingType::DilapidatedShrine,
            "Doctor" => BuildingType::Doctor,
            "EmbroideryStation" => BuildingType::EmbroideryStation,
            "FortuneTeller" => BuildingType::FortuneTeller,
            "Gate" => BuildingType::Gate,
            "Graveyard" => BuildingType::Graveyard,
            "Inn" => BuildingType::Inn,
            "Jester" => BuildingType::Jester,
            "Museum" => BuildingType::Museum,
            "Provisioner" => BuildingType::Provisioner,
            "Sanctuary" => BuildingType::Sanctuary,
            "Tavern" => BuildingType::Tavern,
            "Tower" => BuildingType::Tower,
            "Trainee" => BuildingType::Trainee,
            "WanderingTrinkets" => BuildingType::WanderingTrinkets,
            "WeaponRack" => BuildingType::WeaponRack,
            _ => return Err(format!("unknown BuildingType: {}", raw.building_type)),
        };

        let unlock_conditions: Vec<UnlockCondition> = raw
            .unlock_conditions
            .into_iter()
            .map(|uc| UnlockCondition::new(&uc.condition_type, uc.required_count))
            .collect();

        let upgrade_trees: Vec<UpgradeTree> = raw
            .upgrade_trees
            .into_iter()
            .map(|tree| {
                let levels: Vec<UpgradeLevel> = tree
                    .levels
                    .into_iter()
                    .map(|l| {
                        let effects: Vec<UpgradeEffect> = l
                            .effects
                            .into_iter()
                            .map(|e| UpgradeEffect::new(&e.effect_id, e.value))
                            .collect();
                        UpgradeLevel::new(l.code, l.cost, effects)
                    })
                    .collect();
                UpgradeTree::new(&tree.tree_id, levels)
            })
            .collect();

        let building = TownBuilding::new(&raw.id, building_type, unlock_conditions, upgrade_trees);
        registry.register(building);
    }

    Ok(registry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dungeon_scope_qinglong() {
        let result = parse_dungeon_scope("[\"QingLong\"]").unwrap();
        assert_eq!(result, vec![DungeonType::QingLong]);
    }

    #[test]
    fn parse_dungeon_scope_multiple() {
        let result = parse_dungeon_scope("[\"QingLong\",\"BaiHu\",\"ZhuQue\"]").unwrap();
        assert_eq!(result, vec![DungeonType::QingLong, DungeonType::BaiHu, DungeonType::ZhuQue]);
    }

    #[test]
    fn parse_curio_results_helper() {
        let json = r#"[{"weight":10,"chance":0.5,"result_type":"Loot","result_id":"gold"}]"#;
        let results = parse_curio_results(json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].weight, 10);
        assert_eq!(results[0].chance, 0.5);
        assert_eq!(results[0].result_type, CurioResultType::Loot);
        assert_eq!(results[0].result_id, "gold");
    }

    #[test]
    fn parse_item_interactions_helper() {
        let json = r#"[{"item_id":"shovel","overrides_result_id":"treasure"}]"#;
        let interactions = parse_item_interactions(json).unwrap();
        assert_eq!(interactions.len(), 1);
        assert_eq!(interactions[0].item_id, "shovel");
        assert_eq!(interactions[0].overrides_result_id, "treasure");
    }

    #[test]
    fn full_curio_registry_from_test_data() {
        // Create a temp file for testing
        let temp_dir = std::env::temp_dir();
        let csv_path = temp_dir.join("test_curios.csv");

        std::fs::write(
            &csv_path,
            r#"id,dungeon_scope,results,item_interactions
test_curio,"[""QingLong""]","[{""weight"":5,""chance"":0.5,""result_type"":""Nothing"",""result_id"":""""}]","[]"
"#,
        )
        .unwrap();

        let registry = parse_curios_csv(&csv_path).unwrap();
        assert_eq!(registry.len(), 1);
        assert!(registry.get("test_curio").is_some());

        std::fs::remove_file(csv_path).ok();
    }

    #[test]
    fn full_trap_registry_from_test_data() {
        let temp_dir = std::env::temp_dir();
        let json_path = temp_dir.join("test_traps.json");

        std::fs::write(
            &json_path,
            r#"{"traps":[{"id":"test_trap","success_effects":["avoid"],"fail_effects":["damage"],"health_fraction":0.1,"difficulty_variations":[]}]}"#,
        )
        .unwrap();

        let registry = parse_traps_json(&json_path).unwrap();
        assert_eq!(registry.len(), 1);
        assert!(registry.get("test_trap").is_some());

        std::fs::remove_file(json_path).ok();
    }

    #[test]
    fn full_obstacle_registry_from_test_data() {
        let temp_dir = std::env::temp_dir();
        let json_path = temp_dir.join("test_obstacles.json");

        std::fs::write(
            &json_path,
            r#"{"obstacles":[{"id":"test_obstacle","fail_effects":["damage"],"health_fraction":0.2,"torchlight_penalty":0.1}]}"#,
        )
        .unwrap();

        let registry = parse_obstacles_json(&json_path).unwrap();
        assert_eq!(registry.len(), 1);
        assert!(registry.get("test_obstacle").is_some());

        std::fs::remove_file(json_path).ok();
    }
}
