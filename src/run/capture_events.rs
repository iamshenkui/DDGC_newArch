//! Capture runtime event seam — explicit capture request events for boss captor mechanics.
//!
//! This module provides a first-class `CaptureEvent` type distinct from generic status
//! application, so boss skills that emit `apply_status("capture", N)` markers can be
//! detected and routed to real captive state through a stable game-layer pathway.
//!
//! ## Design
//!
//! `EffectNode::apply_status("capture", Some(1))` is the marker pattern
//! used by necrodrake_embryosac's `untimely_progeny` skill. This module detects
//! that pattern from resolved effect results and emits a structured `CaptureEvent`
//! that carries:
//! - The captor (`ActorId`) — the egg_membrane_empty actor
//! - The captured hero (`ActorId`) — the target of untimely_progeny
//! - The turn number when capture occurred
//!
//! The event is created deterministically from in-progress encounter state without
//! immediately mutating that state. Actual captive state changes happen when the
//! event is processed in the battle loop.

use serde::{Deserialize, Serialize};

use framework_combat::results::EffectResult;
use framework_rules::actor::ActorId;

/// A capture runtime event — the explicit seam for boss captor mechanics.
///
/// This event is created deterministically from a resolved skill effect that
/// applied a `capture` status. It is NOT the status application itself —
/// it is the game-layer interpretation of that status as a captive state request.
///
/// The event carries enough information to place the hero into captive state:
/// - `captor`: the egg_membrane_empty actor performing the capture
/// - `captured`: the hero actor being captured
/// - `capture_turn`: the turn number when capture occurred
///
/// The event does NOT mutate encounter state. That happens when the
/// event is processed by the battle loop's captor processing step.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureEvent {
    /// The actor (egg_membrane_empty) performing the capture.
    pub captor: ActorId,
    /// The hero actor being captured.
    pub captured: ActorId,
    /// The turn number when capture occurred.
    pub capture_turn: u32,
}

impl CaptureEvent {
    /// Create a new capture event.
    pub fn new(captor: ActorId, captured: ActorId, capture_turn: u32) -> Self {
        CaptureEvent {
            captor,
            captured,
            capture_turn,
        }
    }
}

/// Extract capture events from resolved skill effect results.
///
/// This is a pure (non-mutating) function that reads EffectResult data
/// to detect capture status applications. It does not modify actors,
/// formation, or encounter state.
///
/// Returns a list of `CaptureEvent` for each detected capture effect.
pub fn extract_capture_events(
    captor: ActorId,
    results: &[EffectResult],
    capture_turn: u32,
) -> Vec<CaptureEvent> {
    let mut events = Vec::new();

    for result in results {
        // Check for ApplyStatus effect with "capture" kind
        if let framework_combat::results::EffectResultKind::ApplyStatus = result.kind {
            // The status kind is not directly in EffectResult — we need to check
            // applied_statuses for the capture status
            for status in &result.applied_statuses {
                if status.kind.0 == "capture" {
                    events.push(CaptureEvent::new(captor, result.actor, capture_turn));
                    break;
                }
            }
        }
    }

    events
}

/// Check if a skill effect result contains a capture effect.
pub fn has_capture_effect(results: &[EffectResult]) -> bool {
    results.iter().any(|r| {
        if let framework_combat::results::EffectResultKind::ApplyStatus = r.kind {
            r.applied_statuses
                .iter()
                .any(|s| s.kind.0 == "capture")
        } else {
            false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_event_creation() {
        let event = CaptureEvent::new(ActorId(10), ActorId(1), 5);
        assert_eq!(event.captor, ActorId(10));
        assert_eq!(event.captured, ActorId(1));
        assert_eq!(event.capture_turn, 5);
    }

    #[test]
    fn capture_event_serialization() {
        let event = CaptureEvent::new(ActorId(10), ActorId(1), 5);
        let json = serde_json::to_string(&event).unwrap();
        let parsed: CaptureEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, event);
    }

    #[test]
    fn has_capture_effect_false_for_empty_results() {
        let results: Vec<framework_combat::results::EffectResult> = vec![];
        assert!(!has_capture_effect(&results));
    }
}