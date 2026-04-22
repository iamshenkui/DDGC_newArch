//! Disease runtime event seam — explicit disease application events for combat and curio mechanics.
//!
//! This module provides a first-class `DiseaseEvent` type for detecting when diseases are
//! applied during combat through status effects. When a monster skill applies a disease
//! status to a hero, this module detects that pattern and emits a structured `DiseaseEvent`
//! that carries the disease ID and target.
//!
//! ## Design
//!
//! Disease statuses are tracked as quirks with `is_disease=true` in the quirk registry.
//! When a skill applies a disease status (e.g., "consumptive", "bloated"), this module
//! detects that pattern from resolved effect results and emits a `DiseaseEvent` that
//! can be processed to call `apply_quirk` on the hero's quirk state.
//!
//! The event is created deterministically from resolved skill effects without immediately
//! mutating the hero's quirk state. Actual quirk state changes happen when the event
//! is processed after battle.

use serde::{Deserialize, Serialize};

use framework_combat::results::EffectResult;
use framework_rules::actor::ActorId;

/// A disease runtime event — the explicit seam for disease application in combat.
///
/// This event is created deterministically from a resolved skill effect that applied
/// a disease status. It is NOT the status application itself — it is the game-layer
/// interpretation of that status as a disease quirk application request.
///
/// The event carries enough information to apply the disease as a quirk:
/// - `disease_id`: the disease quirk ID (e.g., "consumptive", "bloated")
/// - `target`: the hero actor who received the disease
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiseaseEvent {
    /// The disease quirk ID (e.g., "consumptive", "bloated").
    pub disease_id: String,
    /// The hero actor who received the disease.
    pub target: ActorId,
}

impl DiseaseEvent {
    /// Create a new disease event.
    pub fn new(disease_id: &str, target: ActorId) -> Self {
        DiseaseEvent {
            disease_id: disease_id.to_string(),
            target,
        }
    }
}

/// Extract disease events from resolved skill effect results.
///
/// This is a pure (non-mutating) function that reads EffectResult data
/// to detect disease status applications. It does not modify hero quirk state.
///
/// When a generic `"disease"` status is applied (used by monster skills that
/// reference a random disease table), it is mapped deterministically to a
/// specific disease from `disease_pool` using the target actor ID as selector.
///
/// Returns a list of `DiseaseEvent` for each detected disease effect.
pub fn extract_disease_events(
    effects: &[EffectResult],
    known_diseases: &[&str],
    disease_pool: &[&str],
) -> Vec<DiseaseEvent> {
    let mut events = Vec::new();

    for result in effects {
        // Check for ApplyStatus effect
        if let framework_combat::results::EffectResultKind::ApplyStatus = result.kind {
            // Check each applied status for disease kinds
            for status in &result.applied_statuses {
                let status_kind = status.kind.0.as_str();
                // Check if this status is a known specific disease
                if known_diseases.contains(&status_kind) {
                    events.push(DiseaseEvent::new(status_kind, result.actor));
                } else if status_kind == "disease" && !disease_pool.is_empty() {
                    // Map generic "disease" status to a specific disease from the pool
                    // Use actor ID deterministically to select which disease
                    let idx = (result.actor.0 as usize) % disease_pool.len();
                    events.push(DiseaseEvent::new(disease_pool[idx], result.actor));
                }
            }
        }
    }

    events
}

#[cfg(test)]
mod tests {
    use super::*;
    use framework_combat::results::EffectResultKind;
    use framework_rules::statuses::{StackRule, StatusEffect, StatusKind};

    #[test]
    fn extract_disease_events_finds_disease_status() {
        // Create an effect result with consumptive (disease) status
        let status = StatusEffect::new(
            StatusKind::new("consumptive"),
            Some(3),
            vec![],
            StackRule::Refresh,
        );

        let effect = EffectResult::new(
            EffectResultKind::ApplyStatus,
            ActorId(1),
            vec![ActorId(1)],
        )
        .with_status(status);

        let effects = vec![effect];
        let known_diseases = vec!["consumptive", "bloated"];
        let disease_pool = vec!["consumptive", "bloated"];

        let events = extract_disease_events(&effects, &known_diseases, &disease_pool);

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].disease_id, "consumptive");
        assert_eq!(events[0].target, ActorId(1));
    }

    #[test]
    fn extract_disease_events_ignores_non_disease_status() {
        // Create an effect result with non-disease status (stun)
        let status = StatusEffect::new(
            StatusKind::new("stun"),
            Some(1),
            vec![],
            StackRule::Refresh,
        );

        let effect = EffectResult::new(
            EffectResultKind::ApplyStatus,
            ActorId(1),
            vec![ActorId(1)],
        )
        .with_status(status);

        let effects = vec![effect];
        let known_diseases = vec!["consumptive", "bloated"];
        let disease_pool = vec!["consumptive", "bloated"];

        let events = extract_disease_events(&effects, &known_diseases, &disease_pool);

        assert!(events.is_empty());
    }

    #[test]
    fn extract_disease_events_handles_multiple_effects() {
        // Create multiple effect results
        let consumptive_status = StatusEffect::new(
            StatusKind::new("consumptive"),
            Some(3),
            vec![],
            StackRule::Refresh,
        );

        let bloated_status = StatusEffect::new(
            StatusKind::new("bloated"),
            Some(2),
            vec![],
            StackRule::Refresh,
        );

        let effect1 = EffectResult::new(
            EffectResultKind::ApplyStatus,
            ActorId(1),
            vec![ActorId(1)],
        )
        .with_status(consumptive_status);

        let effect2 = EffectResult::new(
            EffectResultKind::ApplyStatus,
            ActorId(2),
            vec![ActorId(2)],
        )
        .with_status(bloated_status);

        let effects = vec![effect1, effect2];
        let known_diseases = vec!["consumptive", "bloated"];
        let disease_pool = vec!["consumptive", "bloated"];

        let events = extract_disease_events(&effects, &known_diseases, &disease_pool);

        assert_eq!(events.len(), 2);
        assert_eq!(events[0].disease_id, "consumptive");
        assert_eq!(events[1].disease_id, "bloated");
    }

    #[test]
    fn extract_disease_events_maps_generic_disease_status() {
        // Monster skills apply a generic "disease" status — map it to a specific disease
        let status = StatusEffect::new(
            StatusKind::new("disease"),
            Some(6),
            vec![],
            StackRule::Refresh,
        );

        let effect = EffectResult::new(
            EffectResultKind::ApplyStatus,
            ActorId(1),
            vec![ActorId(1)],
        )
        .with_status(status);

        let effects = vec![effect];
        let known_diseases = vec!["consumptive", "bloated"];
        let disease_pool = vec!["consumptive", "bloated"];

        let events = extract_disease_events(&effects, &known_diseases, &disease_pool);

        assert_eq!(events.len(), 1);
        // ActorId(1) % 2 = 1 → "bloated"
        assert_eq!(events[0].disease_id, "bloated");
        assert_eq!(events[0].target, ActorId(1));
    }

    #[test]
    fn extract_disease_events_generic_disease_with_single_pool_member() {
        let status = StatusEffect::new(
            StatusKind::new("disease"),
            Some(4),
            vec![],
            StackRule::Refresh,
        );

        let effect = EffectResult::new(
            EffectResultKind::ApplyStatus,
            ActorId(10),
            vec![ActorId(10)],
        )
        .with_status(status);

        let effects = vec![effect];
        let known_diseases = vec!["consumptive"];
        let disease_pool = vec!["consumptive"];

        let events = extract_disease_events(&effects, &known_diseases, &disease_pool);

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].disease_id, "consumptive");
    }

    #[test]
    fn disease_event_creation() {
        let event = DiseaseEvent::new("consumptive", ActorId(5));

        assert_eq!(event.disease_id, "consumptive");
        assert_eq!(event.target, ActorId(5));
    }
}