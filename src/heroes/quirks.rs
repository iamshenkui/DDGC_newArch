//! Quirk application logic for heroes.
//!
//! Handles applying quirks to hero quirk state, enforcing incompatible quirk
//! replacement and maximum quirk slot limits per category.

use crate::contracts::{QuirkDefinition, QuirkRegistry};
use crate::run::flow::{HeroQuirkState, MAX_NEGATIVE_QUIRKS, MAX_POSITIVE_QUIRKS};

/// Apply a quirk to a hero's quirk state.
///
/// This function:
/// - Looks up the quirk in the registry
/// - Removes any incompatible quirks already present in the hero's state
/// - Enforces maximum slot limits per category (5 positive, 5 negative including diseases)
/// - When a category is full, replaces the oldest (first) quirk
/// - Adds the new quirk to the appropriate category (positive/negative/disease)
///
/// Returns the updated HeroQuirkState.
pub fn apply_quirk(
    mut quirk_state: HeroQuirkState,
    quirk_id: &str,
    quirk_registry: &QuirkRegistry,
) -> HeroQuirkState {
    let quirk = match quirk_registry.get(quirk_id) {
        Some(q) => q,
        None => return quirk_state, // Unknown quirk, no-op
    };

    // Remove any incompatible quirks already present
    remove_incompatible_quirks(&quirk, &mut quirk_state);

    // Add to appropriate category based on quirk type
    if quirk.is_disease {
        add_to_category(&mut quirk_state.diseases, quirk_id, MAX_NEGATIVE_QUIRKS);
    } else if quirk.is_positive {
        add_to_category(&mut quirk_state.positive, quirk_id, MAX_POSITIVE_QUIRKS);
    } else {
        add_to_category(&mut quirk_state.negative, quirk_id, MAX_NEGATIVE_QUIRKS);
    }

    quirk_state
}

/// Remove any quirks from the hero's state that are incompatible with the given quirk.
fn remove_incompatible_quirks(quirk: &QuirkDefinition, quirk_state: &mut HeroQuirkState) {
    for incompatible_id in &quirk.incompatible_quirks {
        // Remove from positive
        quirk_state.positive.retain(|id| id != incompatible_id);
        // Remove from negative
        quirk_state.negative.retain(|id| id != incompatible_id);
        // Remove from diseases
        quirk_state.diseases.retain(|id| id != incompatible_id);
    }
}

/// Add a quirk ID to a category vector, enforcing the max slot limit.
///
/// If the category is full, replaces the oldest (first) quirk.
fn add_to_category(category: &mut Vec<String>, quirk_id: &str, max_slots: usize) {
    // If already present, no-op
    if category.contains(&quirk_id.to_string()) {
        return;
    }

    // If there's room, just add it
    if category.len() < max_slots {
        category.push(quirk_id.to_string());
        return;
    }

    // Category is full - replace the oldest (first) quirk
    category.remove(0);
    category.push(quirk_id.to_string());
}

/// Resolve all attribute modifiers from a hero's active quirks.
///
/// Aggregates buffs from all positive, negative, and disease quirks into
/// a single vector of attribute modifiers, with duplicate attribute keys
/// summed together.
pub fn resolve_quirk_modifiers(
    quirk_state: &HeroQuirkState,
    quirk_registry: &QuirkRegistry,
    buff_registry: &crate::contracts::BuffRegistry,
) -> Vec<crate::contracts::AttributeModifier> {
    let mut aggregated: std::collections::HashMap<String, f64> = std::collections::HashMap::new();

    // Collect modifiers from all quirk categories
    for quirk_id in quirk_state.positive.iter() {
        for modifier in quirk_registry.resolve_quirk_buffs(quirk_id, buff_registry) {
            *aggregated.entry(modifier.attribute_key).or_insert(0.0) += modifier.value;
        }
    }
    for quirk_id in quirk_state.negative.iter() {
        for modifier in quirk_registry.resolve_quirk_buffs(quirk_id, buff_registry) {
            *aggregated.entry(modifier.attribute_key).or_insert(0.0) += modifier.value;
        }
    }
    for quirk_id in quirk_state.diseases.iter() {
        for modifier in quirk_registry.resolve_quirk_buffs(quirk_id, buff_registry) {
            *aggregated.entry(modifier.attribute_key).or_insert(0.0) += modifier.value;
        }
    }

    aggregated
        .into_iter()
        .map(|(attribute_key, value)| crate::contracts::AttributeModifier { attribute_key, value })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::parse::parse_quirks_json;
    use std::path::PathBuf;

    fn data_path(filename: &str) -> PathBuf {
        PathBuf::from("data").join(filename)
    }

    fn parse_quirks() -> QuirkRegistry {
        parse_quirks_json(&data_path("JsonQuirks.json"))
            .expect("failed to parse JsonQuirks.json")
    }

    // ── Quirk application tests ───────────────────────────────────────────────

    #[test]
    fn apply_quirk_adds_positive_quirk() {
        let quirks = parse_quirks();
        let state = HeroQuirkState::new();

        let state = apply_quirk(state, "quick_reflexes", &quirks);

        assert!(state.positive.contains(&"quick_reflexes".to_string()));
    }

    #[test]
    fn apply_quirk_adds_negative_quirk() {
        let quirks = parse_quirks();
        let state = HeroQuirkState::new();

        let state = apply_quirk(state, "clumsy", &quirks);

        assert!(state.negative.contains(&"clumsy".to_string()));
    }

    #[test]
    fn apply_quirk_adds_disease_quirk() {
        let quirks = parse_quirks();
        let state = HeroQuirkState::new();

        let state = apply_quirk(state, "consumptive", &quirks);

        assert!(state.diseases.contains(&"consumptive".to_string()));
        assert!(state.negative.is_empty());
    }

    #[test]
    fn apply_quirk_replaces_incompatible_quirk() {
        let quirks = parse_quirks();
        let state = HeroQuirkState::new();

        // Apply quick_reflexes first
        let state = apply_quirk(state, "quick_reflexes", &quirks);
        assert!(state.positive.contains(&"quick_reflexes".to_string()));

        // Apply clumsy (incompatible with quick_reflexes)
        let state = apply_quirk(state, "clumsy", &quirks);

        // quick_reflexes should be removed, clumsy should be added
        assert!(!state.positive.contains(&"quick_reflexes".to_string()));
        assert!(state.negative.contains(&"clumsy".to_string()));
    }

    #[test]
    fn apply_quirk_disease_replaces_incompatible() {
        let quirks = parse_quirks();
        let state = HeroQuirkState::new();

        // Apply robust (positive, incompatible with consumptive)
        let state = apply_quirk(state, "robust", &quirks);
        assert!(state.positive.contains(&"robust".to_string()));

        // Apply consumptive (disease, incompatible with robust)
        let state = apply_quirk(state, "consumptive", &quirks);

        // robust should be removed, consumptive should be in diseases
        assert!(!state.positive.contains(&"robust".to_string()));
        assert!(state.diseases.contains(&"consumptive".to_string()));
    }

    #[test]
    fn apply_quirk_unknown_quirk_is_noop() {
        let quirks = parse_quirks();
        let state = HeroQuirkState::new();

        let state = apply_quirk(state, "nonexistent_quirk", &quirks);

        assert!(state.positive.is_empty());
        assert!(state.negative.is_empty());
        assert!(state.diseases.is_empty());
    }

    #[test]
    fn apply_quirk_duplicate_is_noop() {
        let quirks = parse_quirks();
        let state = HeroQuirkState::new();

        let state = apply_quirk(state, "quick_reflexes", &quirks);
        let state = apply_quirk(state, "quick_reflexes", &quirks);

        assert_eq!(state.positive.len(), 1);
    }

    #[test]
    fn resolve_quirk_modifiers_aggregates_from_all_categories() {
        let quirks = parse_quirks();
        let buff_registry = crate::contracts::BuffRegistry::new();
        let state = HeroQuirkState::new();

        // Apply quirks from different categories (all mutually compatible)
        let state = apply_quirk(state, "quick_reflexes", &quirks); // SPD+5, DODGE+8
        let state = apply_quirk(state, "weak_will", &quirks); // RESIST_STUN-10, RESIST_BLIND-10, STRESS+5
        let state = apply_quirk(state, "consumptive", &quirks); // MAXHP-20, DEF-5, SPD-3

        let modifiers = resolve_quirk_modifiers(&state, &quirks, &buff_registry);

        // SPD: quick_reflexes +5, consumptive -3 = +2
        let spd = modifiers.iter().find(|m| m.attribute_key == "SPD");
        assert!(spd.is_some());
        assert_eq!(spd.unwrap().value, 2.0); // 5 - 3 = 2

        // DODGE: quick_reflexes +8
        let dodge = modifiers.iter().find(|m| m.attribute_key == "DODGE");
        assert!(dodge.is_some());
        assert_eq!(dodge.unwrap().value, 8.0);

        // MAXHP: consumptive -20
        let maxhp = modifiers.iter().find(|m| m.attribute_key == "MAXHP");
        assert!(maxhp.is_some());
        assert_eq!(maxhp.unwrap().value, -20.0);

        // DEF: consumptive -5
        let def = modifiers.iter().find(|m| m.attribute_key == "DEF");
        assert!(def.is_some());
        assert_eq!(def.unwrap().value, -5.0);
    }

    #[test]
    fn hero_quirk_state_contains() {
        let state = HeroQuirkState::new();
        assert!(!state.contains("quick_reflexes"));

        let state = apply_quirk(state, "quick_reflexes", &parse_quirks());
        assert!(state.contains("quick_reflexes"));
    }

    #[test]
    fn hero_quirk_state_negative_count_includes_diseases() {
        let quirks = parse_quirks();
        let state = HeroQuirkState::new();

        // Add a negative quirk
        let state = apply_quirk(state, "clumsy", &quirks);
        assert_eq!(state.negative_count(), 1);

        // Add a disease
        let state = apply_quirk(state, "consumptive", &quirks);
        assert_eq!(state.negative_count(), 2);
    }

    #[test]
    fn apply_quirk_enforces_max_positive_slots() {
        let quirks = parse_quirks();
        let state = HeroQuirkState::new();

        // Apply 6 positive quirks (max is 5)
        let state = apply_quirk(state, "quick_reflexes", &quirks);
        let state = apply_quirk(state, "iron_will", &quirks);
        let state = apply_quirk(state, "natural_leader", &quirks);
        let state = apply_quirk(state, "light_sleeper", &quirks);
        let state = apply_quirk(state, "panacea", &quirks);
        let state = apply_quirk(state, "robust", &quirks);

        // Should only have 5 positive quirks (oldest replaced)
        assert_eq!(state.positive.len(), MAX_POSITIVE_QUIRKS);
        // quick_reflexes should have been replaced by robust
        assert!(!state.positive.contains(&"quick_reflexes".to_string()));
        assert!(state.positive.contains(&"robust".to_string()));
    }

    #[test]
    fn apply_quirk_enforces_max_negative_slots() {
        let quirks = parse_quirks();
        let state = HeroQuirkState::new();

        // Apply negative quirks until we exceed the limit
        let state = apply_quirk(state, "clumsy", &quirks);
        let state = apply_quirk(state, "weak_will", &quirks);
        let state = apply_quirk(state, "loner", &quirks);
        let state = apply_quirk(state, "night_owl", &quirks);
        let state = apply_quirk(state, "rampage", &quirks);
        let state = apply_quirk(state, "fearful", &quirks);

        // Should only have 5 negative quirks (oldest replaced)
        assert_eq!(state.negative.len(), MAX_NEGATIVE_QUIRKS);
        // clumsy should have been replaced by fearful
        assert!(!state.negative.contains(&"clumsy".to_string()));
        assert!(state.negative.contains(&"fearful".to_string()));
    }

    #[test]
    fn apply_quirk_disease_counts_toward_negative_limit() {
        let quirks = parse_quirks();
        let state = HeroQuirkState::new();

        // Fill negative slots
        let state = apply_quirk(state, "clumsy", &quirks);
        let state = apply_quirk(state, "weak_will", &quirks);
        let state = apply_quirk(state, "loner", &quirks);
        let state = apply_quirk(state, "night_owl", &quirks);
        // 4 negative quirks, add a disease
        let state = apply_quirk(state, "consumptive", &quirks);

        // negative_count includes diseases
        assert_eq!(state.negative_count(), 5);
        // diseases vector should contain the disease
        assert!(state.diseases.contains(&"consumptive".to_string()));
    }
}
