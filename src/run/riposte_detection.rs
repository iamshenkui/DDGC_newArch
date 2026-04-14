//! Riposte detection — identifies actors with riposte marker status.
//!
//! This module provides game-layer detection of riposte-eligible actors
//! from the active combat state. It is the eligibility slice (P1-4):
//! detection only — no counter-attack execution happens here.
//!
//! ## Design
//!
//! Riposte is a marker status (`StatusKind::new("riposte")`) with no modifiers.
//! When an actor with this status is hit, they become eligible to counter-attack
//! the attacker. The detection function iterates over all actors in combat
//! state and returns those with an active riposte status.
//!
//! ## Determinism
//!
//! Detection order is deterministic: actors are iterated and results are
//! sorted by ActorId ascending. The same combat state always produces the
//! same riposte candidate list.

use framework_rules::actor::{ActorAggregate, ActorId};
use std::collections::HashMap;

/// The status kind key for riposte marker statuses.
const RIPOSTE_KIND: &str = "riposte";

/// Detect all riposte-eligible actors in the given combat state.
///
/// An actor is riposte-eligible if they have an active "riposte" status.
/// The status is a marker with no modifiers — its presence alone grants
/// counter-attack eligibility when the actor is hit.
///
/// Returns actors sorted by ActorId ascending for deterministic ordering.
///
/// # Arguments
///
/// * `actors` — all actors in the current combat state (HashMap from ActorId to ActorAggregate)
///
/// # Example
///
/// ```
/// let candidates = detect_riposte_candidates(&actors);
/// for candidate in candidates {
///     // reactor may counter-attack when hit
/// }
/// ```
pub fn detect_riposte_candidates(
    actors: &HashMap<ActorId, ActorAggregate>,
) -> Vec<ActorId> {
    let mut candidates: Vec<ActorId> = actors
        .values()
        .filter(|actor| has_riposte_status(actor))
        .map(|actor| actor.id())
        .collect();

    // Sort by ActorId for deterministic ordering
    candidates.sort_by_key(|id| id.0);
    candidates
}

/// Check if an actor has an active riposte status.
///
/// Iterates over the actor's active statuses and returns true if any
/// have `StatusKind::new("riposte")`. The status is a marker with no
/// modifiers — the presence check is purely kind-based.
fn has_riposte_status(actor: &ActorAggregate) -> bool {
    actor.statuses.active().values().any(|status| {
        status.kind.0 == RIPOSTE_KIND
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use framework_rules::attributes::{AttributeKey, AttributeValue, ATTR_HEALTH};

    fn make_actor(id: u64, has_riposte: bool) -> (ActorId, ActorAggregate) {
        let actor_id = ActorId(id);
        let mut actor = ActorAggregate::new(actor_id);
        actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));

        if has_riposte {
            // Use the content statuses module to attach riposte
            actor.statuses.attach(crate::content::statuses::riposte(2));
        }

        (actor_id, actor)
    }

    #[test]
    fn detect_riposte_candidates_finds_marked_actors() {
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();

        let (id1, actor1) = make_actor(1, true);  // has riposte
        let (id2, actor2) = make_actor(2, false); // no riposte
        let (id3, actor3) = make_actor(3, true);  // has riposte

        actors.insert(id1, actor1);
        actors.insert(id2, actor2);
        actors.insert(id3, actor3);

        let candidates = detect_riposte_candidates(&actors);

        assert_eq!(candidates.len(), 2, "Should find exactly 2 riposte candidates");
        assert_eq!(candidates[0], ActorId(1), "First candidate should be actor 1");
        assert_eq!(candidates[1], ActorId(3), "Second candidate should be actor 3");
    }

    #[test]
    fn detect_riposte_candidates_returns_empty_when_none() {
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();

        let (id1, actor1) = make_actor(1, false);
        let (id2, actor2) = make_actor(2, false);

        actors.insert(id1, actor1);
        actors.insert(id2, actor2);

        let candidates = detect_riposte_candidates(&actors);

        assert!(candidates.is_empty(), "Should return empty when no riposte actors");
    }

    #[test]
    fn detect_riposte_candidates_is_deterministic() {
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();

        // Insert in non-sorted order
        let (id5, actor5) = make_actor(5, true);
        let (id2, actor2) = make_actor(2, true);
        let (id8, actor8) = make_actor(8, true);

        actors.insert(id5, actor5);
        actors.insert(id2, actor2);
        actors.insert(id8, actor8);

        let candidates1 = detect_riposte_candidates(&actors);
        let candidates2 = detect_riposte_candidates(&actors);

        assert_eq!(candidates1, candidates2, "Detection must be deterministic");
        assert_eq!(candidates1[0], ActorId(2), "First should be actor 2");
        assert_eq!(candidates1[1], ActorId(5), "Second should be actor 5");
        assert_eq!(candidates1[2], ActorId(8), "Third should be actor 8");
    }

    #[test]
    fn detect_riposte_candidates_ignores_expired_statuses() {
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();

        let actor_id = ActorId(1);
        let mut actor = ActorAggregate::new(actor_id);
        actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));

        // Attach riposte with duration 1
        actor.statuses.attach(crate::content::statuses::riposte(1));
        actors.insert(actor_id, actor);

        // Initially has riposte
        let candidates_before = detect_riposte_candidates(&actors);
        assert_eq!(candidates_before.len(), 1, "Should detect riposte before tick");

        // Tick once — duration expires
        let expired = actors.get_mut(&actor_id).unwrap().statuses.tick();
        assert_eq!(expired.len(), 1, "Riposte should expire after one tick");

        // After tick, riposte should be gone
        let candidates_after = detect_riposte_candidates(&actors);
        assert!(candidates_after.is_empty(), "Should not detect expired riposte");
    }

    #[test]
    fn non_riposte_actors_are_ignored() {
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();

        // Actor with stun, poison, guard — but NOT riposte
        let actor_id = ActorId(99);
        let mut actor = ActorAggregate::new(actor_id);
        actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
        actor.statuses.attach(crate::content::statuses::stun(2));
        actor.statuses.attach(crate::content::statuses::bleed(5.0, 3));
        actor.statuses.attach(crate::content::statuses::guard(3));

        actors.insert(actor_id, actor);

        let candidates = detect_riposte_candidates(&actors);

        assert!(candidates.is_empty(), "Actors with other statuses but no riposte should be ignored");
    }

    #[test]
    fn actors_with_multiple_statuses_including_riposte_are_detected() {
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();

        let actor_id = ActorId(7);
        let mut actor = ActorAggregate::new(actor_id);
        actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
        // Actor has multiple statuses including riposte
        actor.statuses.attach(crate::content::statuses::bleed(5.0, 3));
        actor.statuses.attach(crate::content::statuses::riposte(2));
        actor.statuses.attach(crate::content::statuses::stun(1));

        actors.insert(actor_id, actor);

        let candidates = detect_riposte_candidates(&actors);

        assert_eq!(candidates.len(), 1, "Should detect actor with riposte among other statuses");
        assert_eq!(candidates[0], ActorId(7));
    }
}