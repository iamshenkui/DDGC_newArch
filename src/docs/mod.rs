//! Canonical save/load boundary for the DDGC headless migration.
//!
//! # The `CampaignState` schema
//!
//! [`CampaignState`] (defined in [`crate::contracts`]) is the **single source of
//! truth** for campaign persistence. Every gameplay-significant piece of state
//! that must survive a save/load cycle lives in this struct. The schema captures:
//!
//! | Domain | Field | Type | Description |
//! |--------|-------|------|-------------|
//! | **Version** | `schema_version` | `u32` | Schema format version for forward/backward compatibility |
//! | **Gold** | `gold` | `u32` | Current gold balance |
//! | **Heirlooms** | `heirlooms` | `BTreeMap<HeirloomCurrency, u32>` | Heirloom currency balances (Bones, Portraits, Tapes) |
//! | **Town** | `building_states` | `BTreeMap<String, BuildingUpgradeState>` | Building upgrade levels keyed by building ID |
//! | **Roster** | `roster` | `Vec<CampaignHero>` | Hero roster with full state per hero |
//! | **Inventory** | `inventory` | `Vec<CampaignInventoryItem>` | Estate inventory items with quantities |
//! | **Run history** | `run_history` | `Vec<CampaignRunRecord>` | Completed/abandoned dungeon runs |
//! | **Quests** | `quest_progress` | `Vec<CampaignQuestProgress>` | Active quest step tracking |
//!
//! ## Hero substructure
//!
//! Each [`CampaignHero`] captures the full persisted hero state:
//!
//! | Field | Type | Description |
//! |-------|------|-------------|
//! | `id` | `String` | Unique hero identifier |
//! | `class_id` | `String` | Hero class (e.g. `"alchemist"`, `"crusader"`) |
//! | `level` | `u32` | Resolve level |
//! | `xp` | `u32` | Experience toward next level |
//! | `health` | `f64` | Current health |
//! | `max_health` | `f64` | Maximum health |
//! | `stress` | `f64` | Current stress |
//! | `max_stress` | `f64` | Maximum stress |
//! | `quirks` | `CampaignHeroQuirks` | Positive, negative, and disease quirks |
//! | `traits` | `CampaignHeroTraits` | Afflictions and virtues |
//! | `skills` | `Vec<String>` | Equipped skill IDs (order preserved) |
//! | `equipment` | `CampaignHeroEquipment` | Weapon/armor levels and trinket slots |
//!
//! # Schema versioning
//!
//! The schema is explicitly versioned via [`CAMPAIGN_SNAPSHOT_VERSION`]. Every
//! save file begins with a `schema_version` field. Consumers **MUST** reject
//! snapshots whose version differs from the expected value.
//!
//! When the schema format changes in a backward-incompatible way:
//! 1. Increment `CAMPAIGN_SNAPSHOT_VERSION`.
//! 2. Update this documentation to reflect the new fields.
//! 3. Add a migration path if backward compatibility is required.
//!
//! # Deterministic serialization
//!
//! All keyed collections (`heirlooms`, `building_states`) use [`BTreeMap`]
//! rather than [`HashMap`]. This guarantees that `serde_json::to_string` produces
//! **byte-identical output** for identical logical state. The guarantees are:
//!
//! - Same state → same JSON bytes, every time.
//! - JSON keys appear in sorted order.
//! - Save-file diffing is reliable.
//! - Integrity hashes (e.g. SHA-256 of the save file) are stable.
//!
//! # Boundary contract
//!
//! `CampaignState` is the **canonical save/load boundary**. No
//! framework-specific types (`ActorId`, `EncounterId`, `SkillDefinition`, etc.)
//! appear in this schema. All identifiers are plain [`String`] values so the
//! persisted state is fully decoupled from the framework type graph.
//!
//! This means:
//! - The save file is a standalone JSON document with no binary blobs.
//! - The save file can be inspected, diffed, and edited with standard tools.
//! - Framework crate upgrades cannot break save compatibility unless the
//!   schema version is explicitly bumped.
//!
//! # Architecture
//!
//! ```text
//! contracts/  (data model + JSON parsing)
//!     │
//!     ▼
//! state/      (GameState: content loading + campaign CRUD)
//!     │
//!     ▼
//! docs/       (this module: canonical documentation + schema tests)
//! ```
//!
//! The `docs` module sits at the verification layer. It imports types from
//! `contracts` and `state` to prove, via focused tests, that the schema
//! faithfully round-trips every gameplay-significant field.
//!
//! # Test coverage
//!
//! The tests in this module are the **canonical acceptance tests** for the
//! save/load boundary. They verify:
//!
//! - Every top-level domain (gold, heirlooms, roster, inventory, town, run
//!   history, quests) round-trips without data loss.
//! - Every hero subdomain (health, stress, quirks, traits, skills, equipment)
//!   round-trips without data loss.
//! - Serialization is deterministic (identical state → identical bytes).
//! - Schema version validation rejects unsupported versions.
//! - The canonical JSON structure is a valid JSON object with the expected keys.
//!
//! If a new gameplay-significant field is added to `CampaignState`, a
//! corresponding test **must** be added here before the field is considered
//! safe for production save/load.

#[cfg(test)]
use crate::contracts::{
    BuildingUpgradeState, CampaignHero, CampaignInventoryItem, CampaignQuestProgress,
    CampaignRunRecord, CampaignState, DungeonType, HeirloomCurrency, MapSize,
    CAMPAIGN_SNAPSHOT_VERSION,
};

// ───────────────────────────────────────────────────────────────────
// Test helpers
// ───────────────────────────────────────────────────────────────────

/// Build a fully-populated `CampaignState` exercising every field in the schema.
///
/// This is the canonical test fixture for the save/load boundary. It populates
/// all seven top-level domains and every hero subdomain so that a single
/// round-trip can prove no gameplay-significant field is lost.
#[cfg(test)]
fn build_full_campaign() -> CampaignState {
    let mut state = CampaignState::new(1500);

    // Heirlooms: all three currencies
    state.heirlooms.insert(HeirloomCurrency::Bones, 42);
    state.heirlooms.insert(HeirloomCurrency::Portraits, 15);
    state.heirlooms.insert(HeirloomCurrency::Tapes, 7);

    // Town: two buildings at different upgrade levels
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

// ───────────────────────────────────────────────────────────────────
// Full round-trip: all domains
// ───────────────────────────────────────────────────────────────────

#[test]
fn full_campaign_roundtrip_serializes_and_deserializes() {
    let original = build_full_campaign();
    let json = original.to_json().expect("serialization must succeed");
    let restored = CampaignState::from_json(&json).expect("deserialization must succeed");
    assert_eq!(original, restored, "round-trip must produce identical state");
}

#[test]
fn full_campaign_roundtrip_preserves_all_top_level_fields() {
    let original = build_full_campaign();
    let json = original.to_json().expect("serialization must succeed");
    let restored = CampaignState::from_json(&json).expect("deserialization must succeed");

    // Schema version
    assert_eq!(restored.schema_version, CAMPAIGN_SNAPSHOT_VERSION);

    // Gold
    assert_eq!(restored.gold, 1500);

    // Heirlooms
    assert_eq!(restored.heirlooms.len(), 3);
    assert_eq!(restored.heirlooms[&HeirloomCurrency::Bones], 42);
    assert_eq!(restored.heirlooms[&HeirloomCurrency::Portraits], 15);
    assert_eq!(restored.heirlooms[&HeirloomCurrency::Tapes], 7);

    // Town
    assert_eq!(restored.building_states.len(), 3);
    assert_eq!(restored.building_states["inn"].current_level, Some('b'));
    assert_eq!(restored.building_states["blacksmith"].current_level, Some('a'));
    assert_eq!(restored.building_states["abbey"].current_level, None);

    // Roster
    assert_eq!(restored.roster.len(), 2);

    // Inventory
    assert_eq!(restored.inventory.len(), 3);

    // Run history
    assert_eq!(restored.run_history.len(), 2);

    // Quests
    assert_eq!(restored.quest_progress.len(), 1);
}

// ───────────────────────────────────────────────────────────────────
// Per-hero-subdomain round-trip tests
// ───────────────────────────────────────────────────────────────────

#[test]
fn hero_identity_roundtrip_preserves_id_class_level_xp() {
    let hero = CampaignHero::new("hero_1", "plague_doctor", 5, 1200, 50.0, 100.0, 0.0, 200.0);
    let mut state = CampaignState::new(0);
    state.roster.push(hero);
    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    let h = &restored.roster[0];
    assert_eq!(h.id, "hero_1");
    assert_eq!(h.class_id, "plague_doctor");
    assert_eq!(h.level, 5);
    assert_eq!(h.xp, 1200);
}

#[test]
fn hero_vitals_roundtrip_preserves_health_and_stress() {
    let hero = CampaignHero::new("hero_1", "crusader", 1, 0, 72.5, 100.0, 45.0, 200.0);
    let mut state = CampaignState::new(0);
    state.roster.push(hero);
    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    let h = &restored.roster[0];
    assert!((h.health - 72.5).abs() < f64::EPSILON);
    assert!((h.max_health - 100.0).abs() < f64::EPSILON);
    assert!((h.stress - 45.0).abs() < f64::EPSILON);
    assert!((h.max_stress - 200.0).abs() < f64::EPSILON);
}

#[test]
fn hero_quirks_roundtrip_preserves_all_three_categories() {
    let mut hero = CampaignHero::new("h1", "jester", 1, 0, 100.0, 100.0, 0.0, 200.0);
    hero.quirks.positive = vec!["eagle_eye".to_string(), "tough".to_string()];
    hero.quirks.negative = vec!["fearful".to_string(), "kleptomaniac".to_string()];
    hero.quirks.diseases = vec!["rabies".to_string(), "consumption".to_string()];
    let mut state = CampaignState::new(0);
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
fn hero_traits_roundtrip_preserves_afflictions_and_virtues() {
    let mut hero = CampaignHero::new("h1", "leper", 1, 0, 100.0, 100.0, 0.0, 200.0);
    hero.traits.afflictions = vec!["paranoid".to_string(), "fearful".to_string()];
    hero.traits.virtues = vec!["courageous".to_string()];
    let mut state = CampaignState::new(0);
    state.roster.push(hero);
    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    let h = &restored.roster[0];
    assert_eq!(h.traits.afflictions, vec!["paranoid", "fearful"]);
    assert_eq!(h.traits.virtues, vec!["courageous"]);
}

#[test]
fn hero_skills_roundtrip_preserves_order() {
    let mut hero = CampaignHero::new("h1", "shaman", 1, 0, 100.0, 100.0, 0.0, 200.0);
    hero.skills = vec![
        "skill_lightning".to_string(),
        "skill_hex".to_string(),
        "skill_totem".to_string(),
    ];
    let mut state = CampaignState::new(0);
    state.roster.push(hero);
    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    let h = &restored.roster[0];
    assert_eq!(h.skills.len(), 3);
    assert_eq!(h.skills[0], "skill_lightning");
    assert_eq!(h.skills[1], "skill_hex");
    assert_eq!(h.skills[2], "skill_totem");
}

#[test]
fn hero_equipment_roundtrip_preserves_levels_and_trinkets() {
    let mut hero = CampaignHero::new("h1", "tank", 1, 0, 100.0, 100.0, 0.0, 200.0);
    hero.equipment.weapon_level = 4;
    hero.equipment.armor_level = 3;
    hero.equipment.trinkets = vec![
        "shield_medallion".to_string(),
        "sun_ring".to_string(),
    ];
    let mut state = CampaignState::new(0);
    state.roster.push(hero);
    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    let h = &restored.roster[0];
    assert_eq!(h.equipment.weapon_level, 4);
    assert_eq!(h.equipment.armor_level, 3);
    assert_eq!(h.equipment.trinkets, vec!["shield_medallion", "sun_ring"]);
}

// ───────────────────────────────────────────────────────────────────
// Per-domain round-trip: inventory, run history, quests
// ───────────────────────────────────────────────────────────────────

#[test]
fn inventory_roundtrip_preserves_id_and_quantity() {
    let mut state = CampaignState::new(0);
    state.inventory.push(CampaignInventoryItem::new("torch", 8));
    state.inventory.push(CampaignInventoryItem::new("bandage", 4));
    state.inventory.push(CampaignInventoryItem::new("shovel", 1));
    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    assert_eq!(restored.inventory.len(), 3);
    assert_eq!(restored.inventory[0].id, "torch");
    assert_eq!(restored.inventory[0].quantity, 8);
    assert_eq!(restored.inventory[1].id, "bandage");
    assert_eq!(restored.inventory[1].quantity, 4);
    assert_eq!(restored.inventory[2].id, "shovel");
    assert_eq!(restored.inventory[2].quantity, 1);
}

#[test]
fn run_history_roundtrip_preserves_all_fields() {
    let mut state = CampaignState::new(0);
    state.run_history.push(CampaignRunRecord::new(
        DungeonType::ZhuQue, MapSize::Short,
        9, 3, true, 500,
    ));
    state.run_history.push(CampaignRunRecord::new(
        DungeonType::XuanWu, MapSize::Medium,
        2, 0, false, 25,
    ));
    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();

    assert_eq!(restored.run_history.len(), 2);

    let r0 = &restored.run_history[0];
    assert_eq!(r0.dungeon, DungeonType::ZhuQue);
    assert_eq!(r0.map_size, MapSize::Short);
    assert_eq!(r0.rooms_cleared, 9);
    assert_eq!(r0.battles_won, 3);
    assert!(r0.completed);
    assert_eq!(r0.gold_earned, 500);

    let r1 = &restored.run_history[1];
    assert_eq!(r1.dungeon, DungeonType::XuanWu);
    assert_eq!(r1.map_size, MapSize::Medium);
    assert_eq!(r1.rooms_cleared, 2);
    assert_eq!(r1.battles_won, 0);
    assert!(!r1.completed);
    assert_eq!(r1.gold_earned, 25);
}

#[test]
fn quest_progress_roundtrip_preserves_step_tracking() {
    let mut state = CampaignState::new(0);
    let mut q0 = CampaignQuestProgress::new("cleanse_all_dungeons", 4);
    q0.current_step = 2;
    state.quest_progress.push(q0);
    let mut q1 = CampaignQuestProgress::new("collect_heirlooms", 3);
    q1.current_step = 3;
    q1.completed = true;
    state.quest_progress.push(q1);

    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();

    assert_eq!(restored.quest_progress.len(), 2);

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
}

// ───────────────────────────────────────────────────────────────────
// Empty / fresh campaign
// ───────────────────────────────────────────────────────────────────

#[test]
fn empty_campaign_roundtrip_preserves_defaults() {
    let campaign = CampaignState::new(0);
    let json = campaign.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    assert_eq!(restored.schema_version, CAMPAIGN_SNAPSHOT_VERSION);
    assert_eq!(restored.gold, 0);
    assert!(restored.roster.is_empty());
    assert!(restored.heirlooms.is_empty());
    assert!(restored.building_states.is_empty());
    assert!(restored.inventory.is_empty());
    assert!(restored.run_history.is_empty());
    assert!(restored.quest_progress.is_empty());
}

#[test]
fn fresh_campaign_initializes_all_collections() {
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

// ───────────────────────────────────────────────────────────────────
// Deterministic serialization
// ───────────────────────────────────────────────────────────────────

#[test]
fn serialization_is_deterministic_same_state_same_bytes() {
    let campaign = build_full_campaign();
    let json_a = campaign.to_json().unwrap();
    let json_b = campaign.to_json().unwrap();
    assert_eq!(json_a, json_b,
        "identical CampaignState must produce identical JSON bytes");
}

#[test]
fn btree_map_heirlooms_produce_sorted_keys() {
    let mut state = CampaignState::new(100);
    state.heirlooms.insert(HeirloomCurrency::Bones, 10);
    state.heirlooms.insert(HeirloomCurrency::Portraits, 20);
    state.heirlooms.insert(HeirloomCurrency::Tapes, 30);
    let json = state.to_json().unwrap();
    // BTreeMap guarantees: Bones < Portraits < Tapes
    let bones_pos = json.find("Bones").unwrap();
    let portraits_pos = json.find("Portraits").unwrap();
    let tapes_pos = json.find("Tapes").unwrap();
    assert!(bones_pos < portraits_pos, "Bones must appear before Portraits in JSON");
    assert!(portraits_pos < tapes_pos, "Portraits must appear before Tapes in JSON");
}

#[test]
fn btree_map_building_states_produce_sorted_keys() {
    let mut state = CampaignState::new(100);
    state.building_states.insert(
        "tavern".to_string(),
        BuildingUpgradeState::new("tavern", Some('c')),
    );
    state.building_states.insert(
        "abbey".to_string(),
        BuildingUpgradeState::new("abbey", Some('a')),
    );
    let json = state.to_json().unwrap();
    let abbey_pos = json.find("abbey").unwrap();
    let tavern_pos = json.find("tavern").unwrap();
    assert!(abbey_pos < tavern_pos, "abbey must appear before tavern in JSON");
}

// ───────────────────────────────────────────────────────────────────
// Schema versioning
// ───────────────────────────────────────────────────────────────────

#[test]
fn new_campaign_uses_current_schema_version() {
    let campaign = CampaignState::new(500);
    assert_eq!(campaign.schema_version, CAMPAIGN_SNAPSHOT_VERSION);
    assert!(campaign.validate_version().is_ok());
}

#[test]
fn validate_version_rejects_unsupported() {
    let mut campaign = CampaignState::new(500);
    campaign.schema_version = 99;
    let result = campaign.validate_version();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("unsupported campaign schema version"));
}

#[test]
fn validate_version_accepts_current() {
    let campaign = CampaignState::new(500);
    assert_eq!(campaign.schema_version, CAMPAIGN_SNAPSHOT_VERSION);
    assert!(campaign.validate_version().is_ok());
}

// ───────────────────────────────────────────────────────────────────
// Canonical JSON structure
// ───────────────────────────────────────────────────────────────────

#[test]
fn campaign_json_is_valid_and_has_expected_top_level_keys() {
    let state = build_full_campaign();
    let json = state.to_json().unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json)
        .expect("campaign JSON must be valid JSON");

    assert!(parsed.is_object());
    assert_eq!(parsed["schema_version"], CAMPAIGN_SNAPSHOT_VERSION);
    assert_eq!(parsed["gold"], 1500);
    assert!(parsed["heirlooms"].is_object());
    assert!(parsed["building_states"].is_object());
    assert!(parsed["roster"].is_array());
    assert!(parsed["inventory"].is_array());
    assert!(parsed["run_history"].is_array());
    assert!(parsed["quest_progress"].is_array());
}
