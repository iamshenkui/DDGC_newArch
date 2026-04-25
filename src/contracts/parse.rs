//! Parsing utilities for DDGC data files (Curios.csv, Traps.json, Obstacles.json, Buildings.json).

use std::path::Path;

use serde::Deserialize;
use crate::contracts::{
    ActOutAction, ActOutEntry, BuildingRegistry, BuildingType, CampEffect, CampEffectType,
    CampTargetSelection, CampingSkill, CampingSkillRegistry, CurioDefinition,
    CurioRegistry, CurioResult, CurioResultType, DungeonType, ItemInteraction,
    ObstacleDefinition, ObstacleRegistry, OverstressType, QuirkClassification, QuirkDefinition,
    QuirkRegistry, ReactionEffect, ReactionEntry, ReactionTrigger, TownBuilding, TrapDefinition,
    TrapRegistry, TrapVariation, TraitDefinition, TraitRegistry, TrinketDefinition,
    TrinketRarity, TrinketRegistry, UnlockCondition, UpgradeEffect, UpgradeLevel, UpgradeTree,
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

        // Check if a building with this ID already exists; if so, merge upgrade trees
        if let Some(existing) = registry.buildings.get_mut(&raw.id) {
            // Merge upgrade trees from the new building into the existing one
            for new_tree in upgrade_trees {
                if !existing.upgrade_trees.iter().any(|t| t.tree_id == new_tree.tree_id) {
                    existing.upgrade_trees.push(new_tree);
                }
            }
        } else {
            let building = TownBuilding::new(&raw.id, building_type, unlock_conditions, upgrade_trees);
            registry.register(building);
        }
    }

    Ok(registry)
}

/// Parse JsonTrinkets.json into a TrinketRegistry.
pub fn parse_trinkets_json(path: &Path) -> Result<TrinketRegistry, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read JsonTrinkets.json: {}", e))?;

    #[derive(Deserialize)]
    struct RawTrinket {
        id: String,
        buffs: Vec<String>,
        hero_class_requirements: Vec<String>,
        rarity: String,
        price: u32,
        limit: u32,
        origin_dungeon: String,
    }

    #[derive(Deserialize)]
    struct RawTrinketsRoot {
        trinkets: Vec<RawTrinket>,
    }

    let root: RawTrinketsRoot = serde_json::from_str(&content)
        .map_err(|e| format!("failed to parse JsonTrinkets.json: {}", e))?;

    let mut registry = TrinketRegistry::new();

    for raw in root.trinkets {
        let rarity = match raw.rarity.as_str() {
            "common" => TrinketRarity::Common,
            "uncommon" => TrinketRarity::Uncommon,
            "rare" => TrinketRarity::Rare,
            "epic" => TrinketRarity::Epic,
            "legendary" => TrinketRarity::Legendary,
            _ => return Err(format!("unknown TrinketRarity: {}", raw.rarity)),
        };

        let origin_dungeon = match raw.origin_dungeon.as_str() {
            "QingLong" => DungeonType::QingLong,
            "BaiHu" => DungeonType::BaiHu,
            "ZhuQue" => DungeonType::ZhuQue,
            "XuanWu" => DungeonType::XuanWu,
            _ => return Err(format!("unknown DungeonType: {}", raw.origin_dungeon)),
        };

        let trinket = TrinketDefinition::new(
            &raw.id,
            raw.buffs,
            raw.hero_class_requirements,
            rarity,
            raw.price,
            raw.limit,
            origin_dungeon,
        );
        registry.register(trinket);
    }

    Ok(registry)
}

/// Parse JsonQuirks.json into a QuirkRegistry.
pub fn parse_quirks_json(path: &Path) -> Result<QuirkRegistry, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read JsonQuirks.json: {}", e))?;

    #[derive(Deserialize)]
    struct RawQuirk {
        id: String,
        is_positive: bool,
        is_disease: bool,
        classification: String,
        buffs: Vec<String>,
        incompatible_quirks: Vec<String>,
        curio_tag: String,
    }

    #[derive(Deserialize)]
    struct RawQuirksRoot {
        quirks: Vec<RawQuirk>,
    }

    let root: RawQuirksRoot = serde_json::from_str(&content)
        .map_err(|e| format!("failed to parse JsonQuirks.json: {}", e))?;

    let mut registry = QuirkRegistry::new();

    for raw in root.quirks {
        let classification = match raw.classification.as_str() {
            "personality" => QuirkClassification::Personality,
            "physical" => QuirkClassification::Physical,
            "disease" => QuirkClassification::Disease,
            "preference" => QuirkClassification::Preference,
            "belief" => QuirkClassification::Belief,
            "talent" => QuirkClassification::Talent,
            "habit" => QuirkClassification::Habit,
            "social" => QuirkClassification::Social,
            _ => return Err(format!("unknown QuirkClassification: {}", raw.classification)),
        };

        let quirk = QuirkDefinition::new(
            &raw.id,
            raw.is_positive,
            raw.is_disease,
            classification,
            raw.buffs,
            raw.incompatible_quirks,
            &raw.curio_tag,
        );
        registry.register(quirk);
    }

    Ok(registry)
}

/// Parse JsonTraits.json into a TraitRegistry.
pub fn parse_traits_json(path: &Path) -> Result<TraitRegistry, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read JsonTraits.json: {}", e))?;

    #[derive(Deserialize)]
    struct RawActOutEntry {
        action: String,
        weight: u32,
    }

    #[derive(Deserialize)]
    struct RawReactionEntry {
        trigger: String,
        probability: f64,
        effect: String,
    }

    #[derive(Deserialize)]
    struct RawTrait {
        id: String,
        overstress_type: String,
        buff_ids: Vec<String>,
        combat_start_turn_act_outs: Vec<RawActOutEntry>,
        reaction_act_outs: Vec<RawReactionEntry>,
    }

    #[derive(Deserialize)]
    struct RawTraitsRoot {
        traits: Vec<RawTrait>,
    }

    let root: RawTraitsRoot = serde_json::from_str(&content)
        .map_err(|e| format!("failed to parse JsonTraits.json: {}", e))?;

    let mut registry = TraitRegistry::new();

    for raw in root.traits {
        let overstress_type = match raw.overstress_type.as_str() {
            "affliction" => OverstressType::Affliction,
            "virtue" => OverstressType::Virtue,
            _ => return Err(format!("unknown OverstressType: {}", raw.overstress_type)),
        };

        let combat_start_turn_act_outs: Vec<ActOutEntry> = raw
            .combat_start_turn_act_outs
            .into_iter()
            .map(|entry| {
                let action = ActOutAction::from_str(&entry.action)
                    .ok_or_else(|| format!("unknown ActOutAction: {}", entry.action))?;
                Ok::<ActOutEntry, String>(ActOutEntry::new(action, entry.weight))
            })
            .collect::<Result<Vec<ActOutEntry>, String>>()?;

        let reaction_act_outs: Vec<ReactionEntry> = raw
            .reaction_act_outs
            .into_iter()
            .map(|entry| {
                let trigger = ReactionTrigger::from_str(&entry.trigger)
                    .ok_or_else(|| format!("unknown ReactionTrigger: {}", entry.trigger))?;
                let effect = ReactionEffect::from_str(&entry.effect)
                    .ok_or_else(|| format!("unknown ReactionEffect: {}", entry.effect))?;
                Ok::<ReactionEntry, String>(ReactionEntry::new(trigger, entry.probability, effect))
            })
            .collect::<Result<Vec<ReactionEntry>, String>>()?;

        let trait_def = TraitDefinition::new(
            &raw.id,
            overstress_type,
            raw.buff_ids,
            combat_start_turn_act_outs,
            reaction_act_outs,
        );
        registry.register(trait_def);
    }

    Ok(registry)
}

/// Parse JsonCamping.json into a CampingSkillRegistry.
pub fn parse_camping_json(path: &Path) -> Result<CampingSkillRegistry, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read JsonCamping.json: {}", e))?;

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct RawChance {
        code: String,
        amount: f64,
    }

    #[derive(Deserialize)]
    struct RawCampEffect {
        selection: String,
        requirements: Vec<String>,
        chance: RawChance,
        #[serde(rename = "type")]
        effect_type: String,
        sub_type: String,
        amount: f64,
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct RawUpgradeRequirement {
        code: String,
        currency_cost: Vec<RawCurrencyCost>,
        prerequisite_requirements: Vec<String>,
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct RawCurrencyCost {
        #[serde(rename = "type")]
        currency_type: String,
        amount: f64,
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct RawCampingSkill {
        id: String,
        level: u32,
        cost: u32,
        use_limit: u32,
        effects: Vec<RawCampEffect>,
        hero_classes: Vec<String>,
        upgrade_requirements: Vec<RawUpgradeRequirement>,
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct RawConfig {
        class_specific_number_of_classes_threshold: u32,
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct RawCampingRoot {
        configuration: RawConfig,
        skills: Vec<RawCampingSkill>,
    }

    let root: RawCampingRoot = serde_json::from_str(&content)
        .map_err(|e| format!("failed to parse JsonCamping.json: {}", e))?;

    let mut registry = CampingSkillRegistry::new();

    for raw in root.skills {
        // Parse effects
        let effects: Vec<CampEffect> = raw
            .effects
            .into_iter()
            .map(|e| {
                let selection = CampTargetSelection::from_str(&e.selection)
                    .unwrap_or(CampTargetSelection::None);
                let effect_type = CampEffectType::from_str(&e.effect_type)
                    .unwrap_or(CampEffectType::None);
                CampEffect::new(
                    selection,
                    e.requirements,
                    e.chance.amount,
                    effect_type,
                    &e.sub_type,
                    e.amount,
                )
            })
            .collect();

        // Compute has_individual_target from effects
        let has_individual_target = effects.iter().any(|e| e.selection == CampTargetSelection::Individual);

        // Get upgrade cost from first upgrade requirement (level 0)
        let upgrade_cost = raw
            .upgrade_requirements
            .first()
            .and_then(|ur| ur.currency_cost.first())
            .map(|c| c.amount as u32)
            .unwrap_or(0);

        let skill = CampingSkill::new(
            &raw.id,
            raw.cost,
            raw.use_limit,
            has_individual_target,
            raw.hero_classes,
            effects,
            upgrade_cost,
        );

        registry.register(skill);
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

    #[test]
    fn full_trinket_registry_from_test_data() {
        let temp_dir = std::env::temp_dir();
        let json_path = temp_dir.join("test_trinkets.json");

        std::fs::write(
            &json_path,
            r#"{"trinkets":[
                {"id":"test_trinket_1","buffs":["ATK+10"],"hero_class_requirements":[],"rarity":"common","price":100,"limit":3,"origin_dungeon":"QingLong"},
                {"id":"test_trinket_2","buffs":["DEF+5"],"hero_class_requirements":["hunter"],"rarity":"rare","price":500,"limit":1,"origin_dungeon":"BaiHu"}
            ]}"#,
        )
        .unwrap();

        let registry = parse_trinkets_json(&json_path).unwrap();
        assert_eq!(registry.len(), 2);

        let t1 = registry.get("test_trinket_1").unwrap();
        assert_eq!(t1.rarity, crate::contracts::TrinketRarity::Common);
        assert!(t1.hero_class_requirements.is_empty());

        let t2 = registry.get("test_trinket_2").unwrap();
        assert_eq!(t2.rarity, crate::contracts::TrinketRarity::Rare);
        assert_eq!(t2.hero_class_requirements, vec!["hunter"]);

        std::fs::remove_file(json_path).ok();
    }

    #[test]
    fn trinket_registry_class_filtering() {
        use crate::contracts::{DungeonType, TrinketRarity};

        let mut registry = crate::contracts::TrinketRegistry::new();
        registry.register(crate::contracts::TrinketDefinition::new(
            "all_class_trinket",
            vec!["ATK+10".to_string()],
            vec![],
            TrinketRarity::Common,
            100,
            3,
            DungeonType::QingLong,
        ));
        registry.register(crate::contracts::TrinketDefinition::new(
            "hunter_only",
            vec!["ATK+15".to_string()],
            vec!["hunter".to_string()],
            TrinketRarity::Rare,
            300,
            1,
            DungeonType::BaiHu,
        ));
        registry.register(crate::contracts::TrinketDefinition::new(
            "shaman_or_hunter",
            vec!["MAGIC+20".to_string()],
            vec!["shaman".to_string(), "hunter".to_string()],
            TrinketRarity::Epic,
            600,
            1,
            DungeonType::ZhuQue,
        ));

        // Hunter should see all 3 trinkets
        let hunter_trinkets = registry.trinkets_for_class("hunter");
        assert_eq!(hunter_trinkets.len(), 3);

        // Shaman should see 2 trinkets (all_class + shaman_or_hunter)
        let shaman_trinkets = registry.trinkets_for_class("shaman");
        assert_eq!(shaman_trinkets.len(), 2);

        // Tank should only see 1 trinket (all_class_trinket)
        let tank_trinkets = registry.trinkets_for_class("tank");
        assert_eq!(tank_trinkets.len(), 1);
    }

    #[test]
    fn trinket_registry_rarity_filtering() {
        use crate::contracts::{DungeonType, TrinketRarity};

        let mut registry = crate::contracts::TrinketRegistry::new();
        registry.register(crate::contracts::TrinketDefinition::new(
            "common_1",
            vec![],
            vec![],
            TrinketRarity::Common,
            100,
            3,
            DungeonType::QingLong,
        ));
        registry.register(crate::contracts::TrinketDefinition::new(
            "rare_1",
            vec![],
            vec![],
            TrinketRarity::Rare,
            400,
            1,
            DungeonType::BaiHu,
        ));
        registry.register(crate::contracts::TrinketDefinition::new(
            "epic_1",
            vec![],
            vec![],
            TrinketRarity::Epic,
            800,
            1,
            DungeonType::ZhuQue,
        ));

        let common = registry.by_rarity(TrinketRarity::Common);
        assert_eq!(common.len(), 1);
        assert_eq!(common[0].id, "common_1");

        let rare = registry.by_rarity(TrinketRarity::Rare);
        assert_eq!(rare.len(), 1);
        assert_eq!(rare[0].id, "rare_1");

        let epic = registry.by_rarity(TrinketRarity::Epic);
        assert_eq!(epic.len(), 1);
        assert_eq!(epic[0].id, "epic_1");

        let legendary = registry.by_rarity(TrinketRarity::Legendary);
        assert!(legendary.is_empty());
    }

    #[test]
    fn trinket_registry_dungeon_filtering() {
        use crate::contracts::{DungeonType, TrinketRarity};

        let mut registry = crate::contracts::TrinketRegistry::new();
        registry.register(crate::contracts::TrinketDefinition::new(
            "qinglong_trinket",
            vec![],
            vec![],
            TrinketRarity::Common,
            100,
            3,
            DungeonType::QingLong,
        ));
        registry.register(crate::contracts::TrinketDefinition::new(
            "baihu_trinket",
            vec![],
            vec![],
            TrinketRarity::Rare,
            400,
            1,
            DungeonType::BaiHu,
        ));
        registry.register(crate::contracts::TrinketDefinition::new(
            "zhuque_trinket",
            vec![],
            vec![],
            TrinketRarity::Epic,
            800,
            1,
            DungeonType::ZhuQue,
        ));

        let qinglong = registry.by_dungeon(DungeonType::QingLong);
        assert_eq!(qinglong.len(), 1);
        assert_eq!(qinglong[0].id, "qinglong_trinket");

        let baihu = registry.by_dungeon(DungeonType::BaiHu);
        assert_eq!(baihu.len(), 1);
        assert_eq!(baihu[0].id, "baihu_trinket");

        let all_dungeons = registry.by_dungeon(DungeonType::XuanWu);
        assert!(all_dungeons.is_empty());
    }

    #[test]
    fn camping_skill_registry_parses_87_skills() {
        let temp_dir = std::env::temp_dir();
        let json_path = temp_dir.join("test_camping.json");

        // Read the real JsonCamping.json
        let real_path = std::path::Path::new("/mnt/d/GameDesign/游戏迁移/DDGC_newArch/data/JsonCamping.json");
        if real_path.exists() {
            std::fs::copy(real_path, &json_path).ok();
        } else {
            return;
        }

        let registry = parse_camping_json(&json_path).unwrap();
        assert_eq!(registry.len(), 87, "JsonCamping.json should contain 87 skills");

        std::fs::remove_file(json_path).ok();
    }

    #[test]
    fn camping_skill_shared_skill_hobby() {
        let temp_dir = std::env::temp_dir();
        let json_path = temp_dir.join("test_camping.json");

        // Read the real JsonCamping.json
        let real_path = std::path::Path::new("/mnt/d/GameDesign/游戏迁移/DDGC_newArch/data/JsonCamping.json");
        if real_path.exists() {
            std::fs::copy(real_path, &json_path).ok();
        } else {
            return;
        }

        let registry = parse_camping_json(&json_path).unwrap();

        // Test shared skill: "hobby" - truly generic (hero_classes is empty list)
        let skill = registry.get("hobby").expect("hobby skill should exist");
        assert_eq!(skill.time_cost, 2, "hobby should have time cost 2");
        assert_eq!(skill.use_limit, 1, "hobby should have use limit 1");
        assert!(!skill.has_individual_target, "hobby should be self-target only");
        assert!(skill.is_generic(), "hobby should be a generic skill");
        assert!(skill.classes.is_empty(), "hobby should have empty classes list");
        assert_eq!(skill.upgrade_cost, 1750, "hobby should have upgrade cost 1750");

        // Check effects - hobby heals stress to self
        assert_eq!(skill.effects.len(), 1, "hobby should have 1 effect");
        let effect = &skill.effects[0];
        assert_eq!(effect.effect_type, crate::contracts::CampEffectType::StressHealAmount);
        assert_eq!(effect.selection, crate::contracts::CampTargetSelection::SelfTarget);
        assert_eq!(effect.amount, 12.0, "hobby should heal 12 stress");
        assert_eq!(effect.chance, 1.0, "hobby effect should be guaranteed");

        std::fs::remove_file(json_path).ok();
    }

    #[test]
    fn camping_skill_class_specific_field_dressing() {
        let temp_dir = std::env::temp_dir();
        let json_path = temp_dir.join("test_camping.json");

        // Read the real JsonCamping.json
        let real_path = std::path::Path::new("/mnt/d/GameDesign/游戏迁移/DDGC_newArch/data/JsonCamping.json");
        if real_path.exists() {
            std::fs::copy(real_path, &json_path).ok();
        } else {
            return;
        }

        let registry = parse_camping_json(&json_path).unwrap();

        // Test class-specific skill: "field_dressing" - arbalest/musketeer only
        let skill = registry.get("field_dressing").expect("field_dressing skill should exist");
        assert_eq!(skill.time_cost, 3, "field_dressing should have time cost 3");
        assert_eq!(skill.use_limit, 1, "field_dressing should have use limit 1");
        assert!(skill.has_individual_target, "field_dressing should require individual target");
        assert!(!skill.is_generic(), "field_dressing should be class-specific");
        assert_eq!(skill.classes, vec!["arbalest", "musketeer"], "field_dressing should be arbalest/musketeer only");
        assert_eq!(skill.upgrade_cost, 1750, "field_dressing should have upgrade cost 1750");

        // Check effects - should have 3 effects (two health heals + remove bleed)
        assert_eq!(skill.effects.len(), 3, "field_dressing should have 3 effects");

        // First effect: health heal max health percent (75% chance)
        let effect0 = &skill.effects[0];
        assert_eq!(effect0.effect_type, crate::contracts::CampEffectType::HealthHealMaxHealthPercent);
        assert_eq!(effect0.selection, crate::contracts::CampTargetSelection::Individual);
        assert!((effect0.amount - 0.35).abs() < f64::EPSILON, "first heal should be 35%");
        assert!((effect0.chance - 0.75).abs() < f64::EPSILON, "first heal chance should be 75%");

        // Second effect: health heal max health percent (25% chance)
        let effect1 = &skill.effects[1];
        assert_eq!(effect1.effect_type, crate::contracts::CampEffectType::HealthHealMaxHealthPercent);
        assert!((effect1.amount - 0.50).abs() < f64::EPSILON, "second heal should be 50%");
        assert!((effect1.chance - 0.25).abs() < f64::EPSILON, "second heal chance should be 25%");

        // Third effect: remove bleeding (100% chance)
        let effect2 = &skill.effects[2];
        assert_eq!(effect2.effect_type, crate::contracts::CampEffectType::RemoveBleed);
        assert!((effect2.chance - 1.0).abs() < f64::EPSILON, "remove bleed should be guaranteed");

        std::fs::remove_file(json_path).ok();
    }

    #[test]
    fn camping_skill_class_filtering() {
        let temp_dir = std::env::temp_dir();
        let json_path = temp_dir.join("test_camping.json");

        // Read the real JsonCamping.json
        let real_path = std::path::Path::new("/mnt/d/GameDesign/游戏迁移/DDGC_newArch/data/JsonCamping.json");
        if real_path.exists() {
            std::fs::copy(real_path, &json_path).ok();
        } else {
            return;
        }

        let registry = parse_camping_json(&json_path).unwrap();

        // Encourage is generic, so should appear for all classes
        let arbalest_encourage = registry.for_class("arbalest");
        assert!(arbalest_encourage.iter().any(|s| s.id == "encourage"), "arbalest should have encourage");

        let crusader_encourage = registry.for_class("crusader");
        assert!(crusader_encourage.iter().any(|s| s.id == "encourage"), "crusader should have encourage");

        // Field dressing is arbalest/musketeer only
        let arbalest_fd = registry.for_class("arbalest");
        assert!(arbalest_fd.iter().any(|s| s.id == "field_dressing"), "arbalest should have field_dressing");

        let crusader_fd = registry.for_class("crusader");
        assert!(!crusader_fd.iter().any(|s| s.id == "field_dressing"), "crusader should NOT have field_dressing");

        std::fs::remove_file(json_path).ok();
    }
}
