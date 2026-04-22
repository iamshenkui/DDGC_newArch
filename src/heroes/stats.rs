//! Hero stats computation from base stats + equipment + trinkets.
//!
//! Replaces hardcoded archetype values with dynamic computation based on
//! equipment upgrade levels and equipped trinkets.

use std::collections::HashMap;

use framework_combat::encounter::CombatSide;

use crate::content::actors::{Archetype, ArchetypeName};
use crate::contracts::{
    BuffRegistry, EquipmentSlot, QuirkRegistry, TrinketDefinition,
};
use crate::run::flow::HeroQuirkState;

/// DDGC buff attribute key to Archetype field mapping.
const ATTR_MAPPING: &[(&str, &str)] = &[
    ("ATK", "attack"),
    ("DEF", "defense"),
    ("MAXHP", "max_health"),
    ("SPD", "speed"),
    ("SPEED", "speed"),
    ("CRIT", "crit_chance"),
    ("ACC", "accuracy"),
    ("ACCURACY", "accuracy"),
    ("DODGE", "dodge"),
    ("DOD", "dodge"),
];

/// Base stats for each hero class at level 0 with no equipment.
///
/// These match the original hardcoded values in the archetype() factory functions.
#[derive(Debug, Clone)]
pub struct BaseStats {
    pub max_health: f64,
    pub attack: f64,
    pub defense: f64,
    pub speed: f64,
    pub crit_chance: f64,
    pub accuracy: f64,
    pub dodge: f64,
}

/// Base stats per hero class.
pub const BASE_STATS: &[(&str, BaseStats)] = &[
    (
        "alchemist",
        BaseStats {
            max_health: 139.0,
            attack: 26.0,
            defense: 0.0,
            speed: 5.0,
            crit_chance: 0.02,
            accuracy: 0.95,
            dodge: 0.00,
        },
    ),
    (
        "diviner",
        BaseStats {
            max_health: 160.0,
            attack: 36.0,
            defense: 0.0,
            speed: 5.0,
            crit_chance: 0.02,
            accuracy: 0.95,
            dodge: 0.05,
        },
    ),
    (
        "hunter",
        BaseStats {
            max_health: 152.0,
            attack: 40.0,
            defense: 0.0,
            speed: 6.0,
            crit_chance: 0.02,
            accuracy: 0.95,
            dodge: 0.05,
        },
    ),
    (
        "shaman",
        BaseStats {
            max_health: 135.0,
            attack: 39.0,
            defense: 0.0,
            speed: 5.0,
            crit_chance: 0.03,
            accuracy: 0.95,
            dodge: 0.00,
        },
    ),
    (
        "tank",
        BaseStats {
            max_health: 192.0,
            attack: 31.0,
            defense: 0.0,
            speed: 7.0,
            crit_chance: 0.03,
            accuracy: 0.95,
            dodge: 0.00,
        },
    ),
];

/// Compute hero stats from base stats + equipment + trinkets.
///
/// # Parameters
/// - `hero_class`: Hero class ID (e.g., "alchemist", "tank")
/// - `weapon_level`: Weapon upgrade level (0 = base)
/// - `armor_level`: Armor upgrade level (0 = base)
/// - `trinkets`: Slice of equipped trinket definitions
///
/// # Returns
/// An `Archetype` with all stats computed from base + equipment + trinkets.
///
/// # Backward Compatibility
/// Level-0 equipment + empty trinkets produces identical stats to the
/// original hardcoded archetype() factory functions.
pub fn compute_hero_stats(
    hero_class: &str,
    weapon_level: u32,
    armor_level: u32,
    trinkets: &[&TrinketDefinition],
) -> Archetype {
    _compute_hero_stats_impl(hero_class, weapon_level, armor_level, trinkets, None, None)
}

/// Compute hero stats from base stats + equipment + trinkets + quirks.
///
/// # Parameters
/// - `hero_class`: Hero class ID (e.g., "alchemist", "tank")
/// - `weapon_level`: Weapon upgrade level (0 = base)
/// - `armor_level`: Armor upgrade level (0 = base)
/// - `trinkets`: Slice of equipped trinket definitions
/// - `quirk_state`: Hero's active quirk state
/// - `quirk_registry`: Registry containing quirk definitions
///
/// # Returns
/// An `Archetype` with all stats computed from base + equipment + trinkets + quirks.
pub fn compute_hero_stats_with_quirks(
    hero_class: &str,
    weapon_level: u32,
    armor_level: u32,
    trinkets: &[&TrinketDefinition],
    quirk_state: &HeroQuirkState,
    quirk_registry: &QuirkRegistry,
) -> Archetype {
    _compute_hero_stats_impl(hero_class, weapon_level, armor_level, trinkets, Some(quirk_state), Some(quirk_registry))
}
///
/// This is the internal implementation that accepts optional quirk state and registry.
/// Use `compute_hero_stats` for the backward-compatible version without quirk support.
fn _compute_hero_stats_impl(
    hero_class: &str,
    weapon_level: u32,
    armor_level: u32,
    trinkets: &[&TrinketDefinition],
    quirk_state: Option<&HeroQuirkState>,
    quirk_registry: Option<&QuirkRegistry>,
) -> Archetype {
    // Start with base stats for the class
    let base = BASE_STATS
        .iter()
        .find(|(id, _)| *id == hero_class)
        .map(|(_, stats)| stats.clone())
        .unwrap_or_else(|| {
            // Fallback for unknown classes - use alchemist stats as default
            BaseStats {
                max_health: 139.0,
                attack: 26.0,
                defense: 0.0,
                speed: 5.0,
                crit_chance: 0.02,
                accuracy: 0.95,
                dodge: 0.00,
            }
        });

    // Collect all stat modifiers
    let mut modifiers: HashMap<String, f64> = HashMap::new();

    // Apply equipment modifiers
    apply_equipment_modifiers(hero_class, weapon_level, armor_level, &mut modifiers);

    // Apply trinket modifiers
    apply_trinket_modifiers(trinkets, &mut modifiers);

    // Apply quirk modifiers if both quirk state and registry are provided
    if let (Some(quirks), Some(registry)) = (quirk_state, quirk_registry) {
        apply_quirk_modifiers(quirks, registry, &mut modifiers);
    }

    // Compute final stats
    let max_health = apply_modifier("max_health", base.max_health, &modifiers);
    let attack = apply_modifier("attack", base.attack, &modifiers);
    let defense = apply_modifier("defense", base.defense, &modifiers);
    let speed = apply_modifier("speed", base.speed, &modifiers);
    let crit_chance = apply_modifier("crit_chance", base.crit_chance, &modifiers);
    let accuracy = apply_modifier("accuracy", base.accuracy, &modifiers);
    let dodge = apply_modifier("dodge", base.dodge, &modifiers);

    Archetype {
        name: ArchetypeName::new(hero_class),
        side: CombatSide::Ally,
        health: max_health,
        max_health,
        attack,
        defense,
        speed,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance,
        accuracy,
        dodge,
    }
}

/// Apply equipment stat modifiers to the accumulated modifiers map.
fn apply_equipment_modifiers(
    hero_class: &str,
    weapon_level: u32,
    armor_level: u32,
    modifiers: &mut HashMap<String, f64>,
) {
    // Define equipment stat modifiers per class per level.
    // Format: (class_id, slot, level) -> Vec of (attribute_key, value)
    //
    // Weapon upgrades primarily boost ATK.
    // Armor upgrades primarily boost DEF and MAXHP.
    //
    // Level 0 equipment provides no modifiers (backward compatible base).
    let equipment_table: &[(&str, EquipmentSlot, u32, &[(&str, f64)])] = &[
        // Alchemist Weapon
        ("alchemist", EquipmentSlot::Weapon, 0, &[]),
        ("alchemist", EquipmentSlot::Weapon, 1, &[("ATK", 5.0)]),
        ("alchemist", EquipmentSlot::Weapon, 2, &[("ATK", 12.0)]),
        ("alchemist", EquipmentSlot::Weapon, 3, &[("ATK", 20.0)]),
        // Alchemist Armor
        ("alchemist", EquipmentSlot::Armor, 0, &[]),
        ("alchemist", EquipmentSlot::Armor, 1, &[("DEF", 3.0), ("MAXHP", 15.0)]),
        ("alchemist", EquipmentSlot::Armor, 2, &[("DEF", 7.0), ("MAXHP", 35.0)]),
        ("alchemist", EquipmentSlot::Armor, 3, &[("DEF", 12.0), ("MAXHP", 60.0)]),
        // Diviner Weapon
        ("diviner", EquipmentSlot::Weapon, 0, &[]),
        ("diviner", EquipmentSlot::Weapon, 1, &[("ATK", 6.0)]),
        ("diviner", EquipmentSlot::Weapon, 2, &[("ATK", 14.0)]),
        ("diviner", EquipmentSlot::Weapon, 3, &[("ATK", 24.0)]),
        // Diviner Armor
        ("diviner", EquipmentSlot::Armor, 0, &[]),
        ("diviner", EquipmentSlot::Armor, 1, &[("DEF", 3.0), ("MAXHP", 18.0)]),
        ("diviner", EquipmentSlot::Armor, 2, &[("DEF", 7.0), ("MAXHP", 40.0)]),
        ("diviner", EquipmentSlot::Armor, 3, &[("DEF", 12.0), ("MAXHP", 68.0)]),
        // Hunter Weapon
        ("hunter", EquipmentSlot::Weapon, 0, &[]),
        ("hunter", EquipmentSlot::Weapon, 1, &[("ATK", 7.0)]),
        ("hunter", EquipmentSlot::Weapon, 2, &[("ATK", 16.0)]),
        ("hunter", EquipmentSlot::Weapon, 3, &[("ATK", 27.0)]),
        // Hunter Armor
        ("hunter", EquipmentSlot::Armor, 0, &[]),
        ("hunter", EquipmentSlot::Armor, 1, &[("DEF", 3.0), ("MAXHP", 16.0)]),
        ("hunter", EquipmentSlot::Armor, 2, &[("DEF", 7.0), ("MAXHP", 38.0)]),
        ("hunter", EquipmentSlot::Armor, 3, &[("DEF", 12.0), ("MAXHP", 65.0)]),
        // Shaman Weapon
        ("shaman", EquipmentSlot::Weapon, 0, &[]),
        ("shaman", EquipmentSlot::Weapon, 1, &[("ATK", 6.0)]),
        ("shaman", EquipmentSlot::Weapon, 2, &[("ATK", 15.0)]),
        ("shaman", EquipmentSlot::Weapon, 3, &[("ATK", 25.0)]),
        // Shaman Armor
        ("shaman", EquipmentSlot::Armor, 0, &[]),
        ("shaman", EquipmentSlot::Armor, 1, &[("DEF", 3.0), ("MAXHP", 14.0)]),
        ("shaman", EquipmentSlot::Armor, 2, &[("DEF", 7.0), ("MAXHP", 32.0)]),
        ("shaman", EquipmentSlot::Armor, 3, &[("DEF", 12.0), ("MAXHP", 55.0)]),
        // Tank Weapon
        ("tank", EquipmentSlot::Weapon, 0, &[]),
        ("tank", EquipmentSlot::Weapon, 1, &[("ATK", 5.0)]),
        ("tank", EquipmentSlot::Weapon, 2, &[("ATK", 12.0)]),
        ("tank", EquipmentSlot::Weapon, 3, &[("ATK", 20.0)]),
        // Tank Armor
        ("tank", EquipmentSlot::Armor, 0, &[]),
        ("tank", EquipmentSlot::Armor, 1, &[("DEF", 5.0), ("MAXHP", 25.0)]),
        ("tank", EquipmentSlot::Armor, 2, &[("DEF", 11.0), ("MAXHP", 55.0)]),
        ("tank", EquipmentSlot::Armor, 3, &[("DEF", 18.0), ("MAXHP", 90.0)]),
    ];

    // Find weapon modifiers
    for (class, slot, level, stat_mods) in equipment_table {
        if *class == hero_class && *slot == EquipmentSlot::Weapon && *level == weapon_level {
            for (attr, val) in *stat_mods {
                let mapped = map_attribute(attr);
                *modifiers.entry(mapped.to_string()).or_insert(0.0) += val;
            }
        }
    }

    // Find armor modifiers
    for (class, slot, level, stat_mods) in equipment_table {
        if *class == hero_class && *slot == EquipmentSlot::Armor && *level == armor_level {
            for (attr, val) in *stat_mods {
                let mapped = map_attribute(attr);
                *modifiers.entry(mapped.to_string()).or_insert(0.0) += val;
            }
        }
    }
}

/// Apply trinket stat modifiers to the accumulated modifiers map.
fn apply_trinket_modifiers(
    trinkets: &[&TrinketDefinition],
    modifiers: &mut HashMap<String, f64>,
) {
    let buff_registry = BuffRegistry::new();

    for trinket in trinkets {
        let resolved = buff_registry.resolve_buffs(trinket);
        for modifier in resolved {
            let mapped = map_attribute(&modifier.attribute_key);
            *modifiers.entry(mapped.to_string()).or_insert(0.0) += modifier.value;
        }
    }
}

/// Apply quirk stat modifiers to the accumulated modifiers map.
///
/// Uses the provided QuirkRegistry to resolve quirk buffs.
fn apply_quirk_modifiers(
    quirk_state: &HeroQuirkState,
    quirk_registry: &QuirkRegistry,
    modifiers: &mut HashMap<String, f64>,
) {
    let buff_registry = BuffRegistry::new();

    // Collect modifiers from all quirk categories
    for quirk_id in quirk_state.positive.iter() {
        for modifier in quirk_registry.resolve_quirk_buffs(quirk_id, &buff_registry) {
            let mapped = map_attribute(&modifier.attribute_key);
            *modifiers.entry(mapped.to_string()).or_insert(0.0) += modifier.value;
        }
    }
    for quirk_id in quirk_state.negative.iter() {
        for modifier in quirk_registry.resolve_quirk_buffs(quirk_id, &buff_registry) {
            let mapped = map_attribute(&modifier.attribute_key);
            *modifiers.entry(mapped.to_string()).or_insert(0.0) += modifier.value;
        }
    }
    for quirk_id in quirk_state.diseases.iter() {
        for modifier in quirk_registry.resolve_quirk_buffs(quirk_id, &buff_registry) {
            let mapped = map_attribute(&modifier.attribute_key);
            *modifiers.entry(mapped.to_string()).or_insert(0.0) += modifier.value;
        }
    }
}

/// Map a DDGC buff attribute key to the internal archetype field name.
fn map_attribute(attr: &str) -> &str {
    for (ddgc_key, field_name) in ATTR_MAPPING {
        if ddgc_key.eq_ignore_ascii_case(attr) {
            return field_name;
        }
    }
    // Unknown attributes pass through as-is (for future extensibility)
    attr
}

/// Apply an accumulated modifier to a base value.
fn apply_modifier(field: &str, base: f64, modifiers: &HashMap<String, f64>) -> f64 {
    let delta = modifiers.get(field).copied().unwrap_or(0.0);
    (base + delta).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{DungeonType, TrinketRarity};
    use crate::contracts::parse::parse_quirks_json;
    use crate::run::flow::HeroQuirkState;
    use std::path::PathBuf;

    #[test]
    fn level_0_equipment_matches_hardcoded_stats() {
        // Test that level-0 equipment + no trinkets produces the same stats
        // as the original hardcoded archetype() factory functions.
        for (class_id, base_stats) in BASE_STATS {
            let archetype = compute_hero_stats(class_id, 0, 0, &[]);

            assert_eq!(
                archetype.max_health, base_stats.max_health,
                "max_health mismatch for {}",
                class_id
            );
            assert_eq!(
                archetype.attack, base_stats.attack,
                "attack mismatch for {}",
                class_id
            );
            assert_eq!(
                archetype.defense, base_stats.defense,
                "defense mismatch for {}",
                class_id
            );
            assert_eq!(
                archetype.speed, base_stats.speed,
                "speed mismatch for {}",
                class_id
            );
            assert_eq!(
                archetype.crit_chance, base_stats.crit_chance,
                "crit_chance mismatch for {}",
                class_id
            );
            assert_eq!(
                archetype.accuracy, base_stats.accuracy,
                "accuracy mismatch for {}",
                class_id
            );
            assert_eq!(
                archetype.dodge, base_stats.dodge,
                "dodge mismatch for {}",
                class_id
            );
        }
    }

    #[test]
    fn level_1_weapon_has_higher_atk_than_level_0() {
        // Test that upgrading weapon from level 0 to 1 increases ATK.
        for (class_id, _) in BASE_STATS {
            let stats_0 = compute_hero_stats(class_id, 0, 0, &[]);
            let stats_1 = compute_hero_stats(class_id, 1, 0, &[]);

            assert!(
                stats_1.attack > stats_0.attack,
                "{}: level-1 weapon ATK ({}) should be > level-0 ATK ({})",
                class_id,
                stats_1.attack,
                stats_0.attack
            );
        }
    }

    #[test]
    fn level_1_armor_increases_def_and_hp() {
        // Test that upgrading armor from level 0 to 1 increases DEF and MAXHP.
        for (class_id, _) in BASE_STATS {
            let stats_0 = compute_hero_stats(class_id, 0, 0, &[]);
            let stats_1 = compute_hero_stats(class_id, 0, 1, &[]);

            assert!(
                stats_1.defense >= stats_0.defense,
                "{}: level-1 armor DEF ({}) should be >= level-0 DEF ({})",
                class_id,
                stats_1.defense,
                stats_0.defense
            );
            assert!(
                stats_1.max_health > stats_0.max_health,
                "{}: level-1 armor MAXHP ({}) should be > level-0 MAXHP ({})",
                class_id,
                stats_1.max_health,
                stats_0.max_health
            );
        }
    }

    #[test]
    fn trinket_modifies_correct_attribute() {
        // Test that equipping a trinket correctly modifies the expected attribute.
        // Create a trinket that gives ATK+10.
        let trinket = TrinketDefinition::new(
            "test_atk_trinket",
            vec!["ATK+10".to_string()],
            vec![],
            TrinketRarity::Common,
            100,
            1,
            DungeonType::QingLong,
        );

        let stats_no_trinket = compute_hero_stats("alchemist", 0, 0, &[]);
        let stats_with_trinket = compute_hero_stats("alchemist", 0, 0, &[&trinket]);

        assert_eq!(
            stats_with_trinket.attack,
            stats_no_trinket.attack + 10.0,
            "trinket should add exactly 10 ATK"
        );
    }

    #[test]
    fn multiple_trinkets_stack_modifiers() {
        // Test that multiple trinkets' modifiers are properly summed.
        let trinket1 = TrinketDefinition::new(
            "test_atk_trinket",
            vec!["ATK+10".to_string()],
            vec![],
            TrinketRarity::Common,
            100,
            1,
            DungeonType::QingLong,
        );
        let trinket2 = TrinketDefinition::new(
            "test_atk_trinket2",
            vec!["ATK+15".to_string()],
            vec![],
            TrinketRarity::Common,
            100,
            1,
            DungeonType::QingLong,
        );

        let stats_no_trinket = compute_hero_stats("alchemist", 0, 0, &[]);
        let stats_with_both = compute_hero_stats("alchemist", 0, 0, &[&trinket1, &trinket2]);

        assert_eq!(
            stats_with_both.attack,
            stats_no_trinket.attack + 25.0,
            "two trinkets with ATK+10 and ATK+15 should add 25 total ATK"
        );
    }

    #[test]
    fn equipment_and_trinkets_combine_correctly() {
        // Test that equipment modifiers and trinket modifiers are both applied.
        let trinket = TrinketDefinition::new(
            "test_hp_trinket",
            vec!["MAXHP+20".to_string()],
            vec![],
            TrinketRarity::Common,
            100,
            1,
            DungeonType::QingLong,
        );

        // Alchemist with weapon level 2 (+12 ATK) and armor level 2 (+35 MAXHP, +7 DEF)
        // plus trinket (+20 MAXHP)
        let stats = compute_hero_stats("alchemist", 2, 2, &[&trinket]);

        assert_eq!(stats.attack, 26.0 + 12.0, "ATK should be base + weapon level 2 bonus");
        assert_eq!(stats.max_health, 139.0 + 35.0 + 20.0, "MAXHP should be base + armor + trinket");
        assert_eq!(stats.defense, 0.0 + 7.0, "DEF should be base + armor level 2 bonus");
    }

    #[test]
    fn unknown_class_uses_alchemist_fallback() {
        // Test that an unknown hero class falls back to alchemist stats.
        let stats = compute_hero_stats("unknown_class", 0, 0, &[]);
        let alchemist_stats = compute_hero_stats("alchemist", 0, 0, &[]);

        assert_eq!(stats.max_health, alchemist_stats.max_health);
        assert_eq!(stats.attack, alchemist_stats.attack);
    }

    #[test]
    fn negative_modifiers_cannot_reduce_below_zero() {
        // Test that accumulated negative modifiers don't reduce stats below 0.
        let trinket = TrinketDefinition::new(
            "negative_trinket",
            vec!["ATK-1000".to_string()],
            vec![],
            TrinketRarity::Common,
            100,
            1,
            DungeonType::QingLong,
        );

        let stats = compute_hero_stats("alchemist", 0, 0, &[&trinket]);

        assert!(
            stats.attack >= 0.0,
            "ATK should not go below 0, got {}",
            stats.attack
        );
    }

    #[test]
    fn quirk_modifies_hero_attributes() {
        // Test that quirks modify hero stats via compute_hero_stats_with_quirks.
        let quirks = parse_quirks_json(&PathBuf::from("data").join("JsonQuirks.json")
        ).expect("failed to parse JsonQuirks.json");
        let _buff_registry = BuffRegistry::new();

        // Base alchemist stats with no quirks
        let stats_no_quirks = compute_hero_stats("alchemist", 0, 0, &[]);

        // Apply quick_reflexes (SPD+5, DODGE+8)
        let quirk_state = HeroQuirkState::new();
        let quirk_state = crate::heroes::quirks::apply_quirk(quirk_state, "quick_reflexes", &quirks);
        let stats_with_quirk = compute_hero_stats_with_quirks(
            "alchemist", 0, 0, &[], &quirk_state, &quirks
        );

        // Speed should increase by 5
        assert_eq!(
            stats_with_quirk.speed,
            stats_no_quirks.speed + 5.0,
            "quick_reflexes should add 5 SPD"
        );
        // Dodge should increase by 8
        assert_eq!(
            stats_with_quirk.dodge,
            stats_no_quirks.dodge + 8.0,
            "quick_reflexes should add 8 DODGE"
        );
        // Health should stay the same
        assert_eq!(
            stats_with_quirk.max_health,
            stats_no_quirks.max_health,
            "quick_reflexes should not affect MAXHP"
        );
    }

    #[test]
    fn quirk_modifiers_combine_with_equipment_modifiers() {
        // Test that quirk modifiers stack with equipment modifiers.
        let quirks = parse_quirks_json(
            &PathBuf::from("data").join("JsonQuirks.json")
        ).expect("failed to parse JsonQuirks.json");

        // Alchemist with weapon level 2 (+12 ATK) and no quirks
        let stats_equip_only = compute_hero_stats_with_quirks(
            "alchemist", 2, 0, &[], &HeroQuirkState::new(), &quirks
        );

        // Apply natural_leader (ATK+5, DEF+5)
        let quirk_state = HeroQuirkState::new();
        let quirk_state = crate::heroes::quirks::apply_quirk(quirk_state, "natural_leader", &quirks);
        let stats_equip_and_quirk = compute_hero_stats_with_quirks(
            "alchemist", 2, 0, &[], &quirk_state, &quirks
        );

        // ATK should be base 26 + weapon 12 + quirk 5 = 43
        assert_eq!(
            stats_equip_and_quirk.attack,
            stats_equip_only.attack + 5.0,
            "natural_leader should add 5 ATK on top of equipment"
        );
        // DEF should be base 0 + quirk 5 = 5
        assert_eq!(
            stats_equip_and_quirk.defense,
            stats_equip_only.defense + 5.0,
            "natural_leader should add 5 DEF on top of equipment"
        );
    }

    #[test]
    fn negative_quirk_reduces_hero_attributes() {
        // Test that negative quirks reduce hero stats.
        let quirks = parse_quirks_json(
            &PathBuf::from("data").join("JsonQuirks.json")
        ).expect("failed to parse JsonQuirks.json");

        let stats_no_quirks = compute_hero_stats("alchemist", 0, 0, &[]);

        // Apply clumsy (SPD-3, DODGE-5)
        let quirk_state = HeroQuirkState::new();
        let quirk_state = crate::heroes::quirks::apply_quirk(quirk_state, "clumsy", &quirks);
        let stats_with_quirk = compute_hero_stats_with_quirks(
            "alchemist", 0, 0, &[], &quirk_state, &quirks
        );

        // Speed should decrease by 3 (base 5.0 -> 2.0)
        assert_eq!(
            stats_with_quirk.speed,
            stats_no_quirks.speed - 3.0,
            "clumsy should reduce SPD by 3"
        );
        // Dodge should decrease by 5, but base is 0.0 so it clamps to 0.0
        assert_eq!(
            stats_with_quirk.dodge,
            0.0,
            "clumsy should reduce DODGE, clamped at 0"
        );
    }

    #[test]
    fn disease_quirk_tracked_separately_and_modifies_stats() {
        // Test that disease quirks are tracked separately and modify stats.
        let quirks = parse_quirks_json(
            &PathBuf::from("data").join("JsonQuirks.json")
        ).expect("failed to parse JsonQuirks.json");

        let stats_no_quirks = compute_hero_stats("alchemist", 0, 0, &[]);

        // Apply consumptive (MAXHP-20, DEF-5, SPD-3)
        let quirk_state = HeroQuirkState::new();
        let quirk_state = crate::heroes::quirks::apply_quirk(quirk_state, "consumptive", &quirks);

        // Verify disease is tracked separately
        assert!(quirk_state.diseases.contains(&"consumptive".to_string()));
        assert!(quirk_state.negative.is_empty());

        let stats_with_quirk = compute_hero_stats_with_quirks(
            "alchemist", 0, 0, &[], &quirk_state, &quirks
        );

        // MAXHP should decrease by 20 (base 139.0 -> 119.0)
        assert_eq!(
            stats_with_quirk.max_health,
            stats_no_quirks.max_health - 20.0,
            "consumptive should reduce MAXHP by 20"
        );
        // DEF should decrease by 5, but base is 0.0 so it clamps to 0.0
        assert_eq!(
            stats_with_quirk.defense,
            0.0,
            "consumptive should reduce DEF, clamped at 0"
        );
        // SPD should decrease by 3 (base 5.0 -> 2.0)
        assert_eq!(
            stats_with_quirk.speed,
            stats_no_quirks.speed - 3.0,
            "consumptive should reduce SPD by 3"
        );
    }
}