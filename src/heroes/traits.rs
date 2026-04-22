//! Trait (affliction/virtue) application logic for heroes.
//!
//! Handles applying traits to hero trait state when overstress occurs,
//! and resolving attribute modifiers from active traits.

use crate::contracts::{ActOutEntry, AttributeModifier, BuffRegistry, OverstressType, TraitRegistry};
use crate::run::flow::HeroTraitState;

/// Virtue probability range (5-10%, we use 7.5% as default).
const VIRTUE_PROBABILITY: f64 = 0.075;

/// Apply a trait to a hero's trait state.
///
/// If the trait is already present, this is a no-op.
/// Otherwise, adds the trait to the appropriate category (afflictions or virtues).
///
/// Returns the updated HeroTraitState.
pub fn apply_trait(
    mut trait_state: HeroTraitState,
    trait_id: &str,
    trait_registry: &TraitRegistry,
) -> HeroTraitState {
    let trait_def = match trait_registry.get(trait_id) {
        Some(t) => t,
        None => return trait_state, // Unknown trait, no-op
    };

    let is_virtue = trait_def.overstress_type == OverstressType::Virtue;
    trait_state.add_trait(trait_id, is_virtue);
    trait_state
}

/// Resolve all attribute modifiers from a hero's active traits.
///
/// Aggregates buffs from all afflictions and virtues into a single vector
/// of attribute modifiers, with duplicate attribute keys summed together.
///
/// Similar to resolve_quirk_modifiers but for traits.
pub fn resolve_trait_modifiers(
    trait_state: &HeroTraitState,
    trait_registry: &TraitRegistry,
    buff_registry: &BuffRegistry,
) -> Vec<AttributeModifier> {
    let mut aggregated: std::collections::HashMap<String, f64> = std::collections::HashMap::new();

    // Collect modifiers from all afflictions
    for trait_id in &trait_state.afflictions {
        for modifier in trait_registry.resolve_trait_buffs(trait_id, buff_registry) {
            *aggregated.entry(modifier.attribute_key).or_insert(0.0) += modifier.value;
        }
    }

    // Collect modifiers from all virtues
    for trait_id in &trait_state.virtues {
        for modifier in trait_registry.resolve_trait_buffs(trait_id, buff_registry) {
            *aggregated.entry(modifier.attribute_key).or_insert(0.0) += modifier.value;
        }
    }

    aggregated
        .into_iter()
        .map(|(attribute_key, value)| AttributeModifier { attribute_key, value })
        .collect()
}

/// Resolve overstress for a hero, selecting a new trait based on deterministic seed.
///
/// This function:
/// - Uses the seed to make a deterministic roll for virtue vs affliction
/// - Virtue has approximately 7.5% chance (within 5-10% range specified)
/// - Affliction uses weighted selection based on act-out "nothing" weights
/// - Virtue uses uniform selection (only 1 virtue in current data)
///
/// Returns the selected trait ID, or None if no traits are available.
pub fn resolve_overstress(
    _trait_state: &HeroTraitState,
    seed: u64,
    trait_registry: &TraitRegistry,
) -> Option<String> {
    // Roll for virtue vs affliction using seed-derived probability
    let normalized = (seed % 1000) as f64 / 1000.0;

    if normalized < VIRTUE_PROBABILITY {
        // Virtue roll - select uniformly from available virtues
        select_random_virtue(seed, trait_registry)
    } else {
        // Affliction roll - use weighted selection based on act-out weights
        select_random_affliction(seed, trait_registry)
    }
}

/// Select a random affliction using weighted random selection.
///
/// Uses the "nothing" act-out weight as the selection weight for each affliction.
/// Higher "nothing" weight means the affliction is more likely to be selected.
fn select_random_affliction(seed: u64, trait_registry: &TraitRegistry) -> Option<String> {
    let afflictions = trait_registry.afflictions();
    if afflictions.is_empty() {
        return None;
    }

    // Calculate total weight based on "nothing" act-out weight
    let mut total_weight = 0u32;
    let mut affliction_weights: Vec<(String, u32)> = Vec::new();

    for trait_def in &afflictions {
        let nothing_weight = trait_def
            .combat_start_turn_act_outs
            .iter()
            .find(|a| a.action == crate::contracts::ActOutAction::Nothing)
            .map(|a| a.weight)
            .unwrap_or(1); // Default weight of 1 if nothing not found

        total_weight += nothing_weight;
        affliction_weights.push((trait_def.id.clone(), nothing_weight));
    }

    if total_weight == 0 {
        // Fallback to uniform selection
        return afflictions.first().map(|t| t.id.clone());
    }

    // Weighted random selection
    let selector = (seed % total_weight as u64) as u32;
    let mut accum = 0u32;

    for (trait_id, weight) in &affliction_weights {
        accum += weight;
        if selector < accum {
            return Some(trait_id.clone());
        }
    }

    // Fallback to last
    affliction_weights.last().map(|(id, _)| id.clone())
}

/// Select a random virtue using uniform random selection.
fn select_random_virtue(seed: u64, trait_registry: &TraitRegistry) -> Option<String> {
    let virtues = trait_registry.virtues();
    if virtues.is_empty() {
        return None;
    }

    let index = (seed % virtues.len() as u64) as usize;
    virtues.get(index).map(|t| t.id.clone())
}

/// Resolve an act-out for an afflicted hero at the start of their turn.
///
/// Uses weighted random selection based on the affliction's act-out table.
/// The seed ensures deterministic resolution for the same affliction_id + seed pair.
///
/// Returns the selected act-out entry (including the action and weight).
pub fn resolve_act_out(
    affliction_id: &str,
    seed: u64,
    trait_registry: &TraitRegistry,
) -> Option<ActOutEntry> {
    let trait_def = trait_registry.get(affliction_id)?;

    if trait_def.combat_start_turn_act_outs.is_empty() {
        return None;
    }

    // Calculate total weight
    let total_weight: u32 = trait_def
        .combat_start_turn_act_outs
        .iter()
        .map(|entry| entry.weight)
        .sum();

    if total_weight == 0 {
        // Fallback: return first entry
        return trait_def.combat_start_turn_act_outs.first().cloned();
    }

    // Weighted random selection using seed
    let selector = (seed % total_weight as u64) as u32;
    let mut accum = 0u32;

    for entry in &trait_def.combat_start_turn_act_outs {
        accum += entry.weight;
        if selector < accum {
            return Some(entry.clone());
        }
    }

    // Fallback to last entry
    trait_def.combat_start_turn_act_outs.last().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::parse::parse_traits_json;
    use std::path::PathBuf;

    fn data_path(filename: &str) -> PathBuf {
        PathBuf::from("data").join(filename)
    }

    fn parse_traits() -> TraitRegistry {
        parse_traits_json(&data_path("JsonTraits.json"))
            .expect("failed to parse JsonTraits.json")
    }

    // ── Trait application tests ───────────────────────────────────────────────

    #[test]
    fn apply_trait_adds_affliction() {
        let traits = parse_traits();
        let state = HeroTraitState::new();

        let state = apply_trait(state, "fearful", &traits);

        assert!(state.afflictions.contains(&"fearful".to_string()));
        assert!(state.virtues.is_empty());
    }

    #[test]
    fn apply_trait_adds_virtue() {
        let traits = parse_traits();
        let state = HeroTraitState::new();

        let state = apply_trait(state, "courageous", &traits);

        assert!(state.virtues.contains(&"courageous".to_string()));
        assert!(state.afflictions.is_empty());
    }

    #[test]
    fn apply_trait_duplicate_is_noop() {
        let traits = parse_traits();
        let state = HeroTraitState::new();

        let state = apply_trait(state, "fearful", &traits);
        let state = apply_trait(state, "fearful", &traits);

        assert_eq!(state.afflictions.len(), 1);
    }

    #[test]
    fn apply_trait_unknown_trait_is_noop() {
        let traits = parse_traits();
        let state = HeroTraitState::new();

        let state = apply_trait(state, "nonexistent_trait", &traits);

        assert!(state.afflictions.is_empty());
        assert!(state.virtues.is_empty());
    }

    #[test]
    fn hero_trait_state_contains() {
        let state = HeroTraitState::new();
        assert!(!state.contains("fearful"));

        let traits = parse_traits();
        let state = apply_trait(state, "fearful", &traits);
        assert!(state.contains("fearful"));
    }

    #[test]
    fn hero_trait_state_add_trait_to_afflictions() {
        let mut state = HeroTraitState::new();
        state.add_trait("fearful", false);
        assert!(state.afflictions.contains(&"fearful".to_string()));
        assert!(!state.virtues.contains(&"fearful".to_string()));
    }

    #[test]
    fn hero_trait_state_add_trait_to_virtues() {
        let mut state = HeroTraitState::new();
        state.add_trait("courageous", true);
        assert!(state.virtues.contains(&"courageous".to_string()));
        assert!(!state.afflictions.contains(&"courageous".to_string()));
    }

    // ── Trait buff resolution tests ──────────────────────────────────────────

    #[test]
    fn resolve_trait_modifiers_from_affliction() {
        let traits = parse_traits();
        let buff_registry = BuffRegistry::new();
        let state = HeroTraitState::new();

        let state = apply_trait(state, "fearful", &traits);
        let modifiers = resolve_trait_modifiers(&state, &traits, &buff_registry);

        // fearful: SPD-2, DODGE-3, ACC-5
        let spd = modifiers.iter().find(|m| m.attribute_key == "SPD");
        assert!(spd.is_some());
        assert_eq!(spd.unwrap().value, -2.0);

        let dodge = modifiers.iter().find(|m| m.attribute_key == "DODGE");
        assert!(dodge.is_some());
        assert_eq!(dodge.unwrap().value, -3.0);

        let acc = modifiers.iter().find(|m| m.attribute_key == "ACC");
        assert!(acc.is_some());
        assert_eq!(acc.unwrap().value, -5.0);
    }

    #[test]
    fn resolve_trait_modifiers_from_virtue() {
        let traits = parse_traits();
        let buff_registry = BuffRegistry::new();
        let state = HeroTraitState::new();

        let state = apply_trait(state, "courageous", &traits);
        let modifiers = resolve_trait_modifiers(&state, &traits, &buff_registry);

        // courageous: ATK+5, DEF+3, STRESSRES+15
        let atk = modifiers.iter().find(|m| m.attribute_key == "ATK");
        assert!(atk.is_some());
        assert_eq!(atk.unwrap().value, 5.0);

        let def = modifiers.iter().find(|m| m.attribute_key == "DEF");
        assert!(def.is_some());
        assert_eq!(def.unwrap().value, 3.0);

        let stressres = modifiers.iter().find(|m| m.attribute_key == "STRESSRES");
        assert!(stressres.is_some());
        assert_eq!(stressres.unwrap().value, 15.0);
    }

    #[test]
    fn resolve_trait_modifiers_aggregates_multiple_traits() {
        let traits = parse_traits();
        let buff_registry = BuffRegistry::new();
        let state = HeroTraitState::new();

        let state = apply_trait(state, "fearful", &traits);
        let state = apply_trait(state, "courageous", &traits);
        let modifiers = resolve_trait_modifiers(&state, &traits, &buff_registry);

        // Both traits present, modifiers should be combined
        let atk = modifiers.iter().find(|m| m.attribute_key == "ATK");
        assert!(atk.is_some());
        assert_eq!(atk.unwrap().value, 5.0); // From courageous

        let spd = modifiers.iter().find(|m| m.attribute_key == "SPD");
        assert!(spd.is_some());
        assert_eq!(spd.unwrap().value, -2.0); // From fearful
    }

    // ── Overstress resolution tests ───────────────────────────────────────────

    #[test]
    fn resolve_overstress_returns_affliction_with_high_probability() {
        let traits = parse_traits();
        let state = HeroTraitState::new();

        // Run many seeds and count results - should be mostly afflictions
        let mut affliction_count = 0;
        let mut virtue_count = 0;
        let iterations = 1000;

        for seed in 0..iterations {
            if let Some(trait_id) = resolve_overstress(&state, seed, &traits) {
                if traits.get(&trait_id).map(|t| t.overstress_type == OverstressType::Virtue).unwrap_or(false) {
                    virtue_count += 1;
                } else {
                    affliction_count += 1;
                }
            }
        }

        // Virtue should be rare (~7.5%), afflictions common
        let virtue_ratio = virtue_count as f64 / iterations as f64;
        assert!(
            virtue_ratio < 0.15,
            "Virtue ratio {} should be less than 15% (expected ~7.5%)",
            virtue_ratio
        );
        assert!(
            affliction_count > virtue_count,
            "Afflictions ({}) should outnumber virtues ({})",
            affliction_count,
            virtue_count
        );
    }

    #[test]
    fn resolve_overstress_virtue_is_possible() {
        let traits = parse_traits();
        let state = HeroTraitState::new();

        // Find a seed that produces a virtue
        let mut found_virtue = false;
        for seed in 0..10000u64 {
            if let Some(trait_id) = resolve_overstress(&state, seed, &traits) {
                if traits.get(&trait_id).map(|t| t.overstress_type == OverstressType::Virtue).unwrap_or(false) {
                    found_virtue = true;
                    break;
                }
            }
        }

        assert!(found_virtue, "Should be able to find a seed that produces a virtue");
    }

    #[test]
    fn resolve_overstress_deterministic_for_same_seed() {
        let traits = parse_traits();
        let state = HeroTraitState::new();

        let result1 = resolve_overstress(&state, 42, &traits);
        let result2 = resolve_overstress(&state, 42, &traits);

        assert_eq!(result1, result2, "Same seed should produce same result");
    }

    #[test]
    fn resolve_overstress_different_seeds_different_results() {
        let traits = parse_traits();
        let state = HeroTraitState::new();

        // With enough seeds, we should see different results
        let mut results: std::collections::HashSet<String> = std::collections::HashSet::new();
        for seed in 0..100u64 {
            if let Some(trait_id) = resolve_overstress(&state, seed, &traits) {
                results.insert(trait_id);
            }
        }

        // Should have multiple different results across 100 seeds
        assert!(
            results.len() > 1,
            "Different seeds should produce different results, got only {} unique results",
            results.len()
        );
    }

    #[test]
    fn resolve_overstress_empty_registry_returns_none() {
        let traits = TraitRegistry::new();
        let state = HeroTraitState::new();

        let result = resolve_overstress(&state, 42, &traits);
        assert!(result.is_none());
    }

    // ── Act-out resolution tests (US-021) ─────────────────────────────────────

    #[test]
    fn resolve_act_out_nothing_is_most_common() {
        // For 'fearful' affliction: nothing(40), bark_stress(30), change_pos(20), ignore_command(10)
        // Nothing should be the most common outcome across many seeds.
        let traits = parse_traits();
        let mut outcome_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        let iterations = 1000;

        for seed in 0..iterations {
            if let Some(entry) = resolve_act_out("fearful", seed, &traits) {
                *outcome_counts.entry(entry.action.as_str().to_string()).or_insert(0) += 1;
            }
        }

        // Nothing should appear most frequently (weight 40 vs next highest 30)
        let nothing_count = outcome_counts.get("nothing").copied().unwrap_or(0);
        let bark_count = outcome_counts.get("bark_stress").copied().unwrap_or(0);
        let change_count = outcome_counts.get("change_pos").copied().unwrap_or(0);
        let ignore_count = outcome_counts.get("ignore_command").copied().unwrap_or(0);

        assert!(
            nothing_count > bark_count,
            "Nothing ({}) should be more common than bark_stress ({})",
            nothing_count, bark_count
        );
        assert!(
            nothing_count > change_count,
            "Nothing ({}) should be more common than change_pos ({})",
            nothing_count, change_count
        );
        assert!(
            nothing_count > ignore_count,
            "Nothing ({}) should be more common than ignore_command ({})",
            nothing_count, ignore_count
        );
    }

    #[test]
    fn resolve_act_out_other_outcomes_are_possible() {
        // Prove that all act-out outcomes can occur given the right seed.
        let traits = parse_traits();
        let mut observed_outcomes: std::collections::HashSet<String> = std::collections::HashSet::new();

        // Search many seeds to find all possible outcomes
        for seed in 0..5000u64 {
            if let Some(entry) = resolve_act_out("fearful", seed, &traits) {
                observed_outcomes.insert(entry.action.as_str().to_string());
            }
        }

        // All four outcomes should be observed
        assert!(
            observed_outcomes.contains("nothing"),
            "nothing outcome should be possible"
        );
        assert!(
            observed_outcomes.contains("bark_stress"),
            "bark_stress outcome should be possible"
        );
        assert!(
            observed_outcomes.contains("change_pos"),
            "change_pos outcome should be possible"
        );
        assert!(
            observed_outcomes.contains("ignore_command"),
            "ignore_command outcome should be possible"
        );
    }

    #[test]
    fn resolve_act_out_deterministic_for_same_seed() {
        // Same affliction_id + same seed must always produce the same result.
        let traits = parse_traits();

        for seed in [0u64, 42, 100, 9999] {
            let result1 = resolve_act_out("fearful", seed, &traits);
            let result2 = resolve_act_out("fearful", seed, &traits);
            assert_eq!(
                result1, result2,
                "Same seed ({}) should produce same act-out result",
                seed
            );
        }
    }

    #[test]
    fn resolve_act_out_returns_none_for_unknown_affliction() {
        let traits = parse_traits();

        let result = resolve_act_out("nonexistent_affliction", 42, &traits);
        assert!(result.is_none());
    }

    #[test]
    fn resolve_act_out_hopeless_has_different_distribution() {
        // 'hopeless' has: nothing(50), bark_stress(35), change_pos(10), ignore_command(5)
        // nothing should be even more dominant here (50% vs 40% for fearful).
        let traits = parse_traits();
        let mut outcome_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        let iterations = 1000;

        for seed in 0..iterations {
            if let Some(entry) = resolve_act_out("hopeless", seed, &traits) {
                *outcome_counts.entry(entry.action.as_str().to_string()).or_insert(0) += 1;
            }
        }

        let nothing_count = outcome_counts.get("nothing").copied().unwrap_or(0);
        let nothing_ratio = nothing_count as f64 / iterations as f64;

        // Nothing should be > 40% (it's 50% for hopeless)
        assert!(
            nothing_ratio > 0.40,
            "hopeless nothing ratio ({:.1}%) should be > 40%",
            nothing_ratio * 100.0
        );
    }
}