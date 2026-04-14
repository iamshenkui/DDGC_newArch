//! Guard detection — identifies actors with guard marker status who can protect allies.
//!
//! This module provides game-layer detection of guard-eligible actors
//! from the active combat state. It is the relationship-resolution slice (P1-6):
//! detection only — no redirect execution happens here.
//!
//! ## Design
//!
//! Guard is a marker status (`StatusKind::new("guard")`) with no modifiers.
//! When an actor with this status is on the same team as a hit target,
//! they become eligible to redirect damage for that target.
//!
//! ## Relationship Model
//!
//! The guard relationship is implicit in the status markers:
//! - Actor B has "guard" status → Actor B is a potential guard
//! - When Actor A is hit, any guard on Actor A's team (except Actor A) can redirect
//!
//! This models the DDGC protect skill semantics where a Tank applies guard
//! to allies, and those guards redirect damage when allies are attacked.
//!
//! ## Determinism
//!
//! When multiple guards are eligible for the same target, the one with the
//! lowest ActorId is selected for deterministic ordering. The same combat
//! state always produces the same guard relationship.

use framework_combat::encounter::CombatSide;
use framework_rules::actor::{ActorAggregate, ActorId};
use std::collections::HashMap;

/// The status kind key for guard marker statuses.
const GUARD_KIND: &str = "guard";

/// A detected guard relationship.
///
/// Represents a potential guard redirect: the `guard` actor can redirect
/// damage for the `protected` actor when they are hit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardRelation {
    /// The actor who is being protected (was hit).
    pub protected: ActorId,
    /// The actor who can redirect damage (has guard status).
    pub guard: ActorId,
}

impl GuardRelation {
    /// Create a new guard relation.
    pub fn new(protected: ActorId, guard: ActorId) -> Self {
        GuardRelation { protected, guard }
    }
}

/// Detect all guard relationships for a given hit target.
///
/// When `target` is hit, this finds all actors on the same side who have
/// "guard" status and can redirect damage for the target (excluding the
/// target themselves).
///
/// Returns guard relations sorted by guard ActorId ascending for deterministic
/// ordering.
///
/// # Arguments
///
/// * `target` — the actor who was hit
/// * `actors` — all actors in the current combat state
/// * `side_lookup` — map from ActorId to CombatSide
///
/// # Example
///
/// ```ignore
/// let relations = detect_guard_relations_for_target(target, &actors, &side_lookup);
/// for relation in relations {
///     // guard may redirect damage for protected target
/// }
/// ```
pub fn detect_guard_relations_for_target(
    target: ActorId,
    actors: &HashMap<ActorId, ActorAggregate>,
    side_lookup: &HashMap<ActorId, CombatSide>,
) -> Vec<GuardRelation> {
    let target_side = match side_lookup.get(&target) {
        Some(side) => *side,
        None => return Vec::new(),
    };

    // Find actors on the same side with guard status, excluding the target
    let mut guards: Vec<ActorId> = actors
        .values()
        .filter(|actor| {
            actor.id != target
                && side_lookup.get(&actor.id) == Some(&target_side)
                && has_guard_status(actor)
        })
        .map(|actor| actor.id)
        .collect();

    // Sort by ActorId for deterministic ordering
    guards.sort_by_key(|id| id.0);

    guards
        .into_iter()
        .map(|guard| GuardRelation::new(target, guard))
        .collect()
}

/// Detect all guard relationships in the current combat state.
///
/// This finds all potential guard redirects: for every actor that has
/// "guard" status, they can protect any other ally.
///
/// Returns all guard relations sorted by (protected, guard) ActorId pairs
/// for deterministic ordering.
///
/// # Arguments
///
/// * `actors` — all actors in the current combat state
/// * `side_lookup` — map from ActorId to CombatSide
///
/// # Example
///
/// ```ignore
/// let relations = detect_all_guard_relations(&actors, &side_lookup);
/// for relation in relations {
///     // guard may redirect damage for protected
/// }
/// ```
pub fn detect_all_guard_relations(
    actors: &HashMap<ActorId, ActorAggregate>,
    side_lookup: &HashMap<ActorId, CombatSide>,
) -> Vec<GuardRelation> {
    // Group actors by side
    let mut ally_actors: Vec<ActorId> = Vec::new();
    let mut enemy_actors: Vec<ActorId> = Vec::new();

    for (actor_id, _actor) in actors.iter() {
        match side_lookup.get(actor_id) {
            Some(CombatSide::Ally) => ally_actors.push(*actor_id),
            Some(CombatSide::Enemy) => enemy_actors.push(*actor_id),
            Some(CombatSide::Neutral) | None => {} // Neutral actors don't participate in guard relationships
        }
    }

    // Sort for deterministic processing
    ally_actors.sort_by_key(|id| id.0);
    enemy_actors.sort_by_key(|id| id.0);

    let mut all_relations = Vec::new();

    // For each ally, find guards among other allies
    for &ally in &ally_actors {
        let relations = detect_guard_relations_for_target(ally, actors, side_lookup);
        all_relations.extend(relations);
    }

    // For each enemy, find guards among other enemies
    for &enemy in &enemy_actors {
        let relations = detect_guard_relations_for_target(enemy, actors, side_lookup);
        all_relations.extend(relations);
    }

    // Sort by (protected, guard) for deterministic output
    all_relations.sort_by_key(|r| (r.protected.0, r.guard.0));
    all_relations
}

/// Check if an actor has an active guard status.
///
/// Iterates over the actor's active statuses and returns true if any
/// have `StatusKind::new("guard")`. The status is a marker with no
/// modifiers — the presence check is purely kind-based.
pub fn has_guard_status(actor: &ActorAggregate) -> bool {
    actor.statuses.active().values().any(|status| {
        status.kind.0 == GUARD_KIND
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use framework_combat::encounter::CombatSide;
    use framework_rules::attributes::{AttributeKey, AttributeValue, ATTR_HEALTH};

    fn make_actor(id: u64, _side: CombatSide, has_guard: bool) -> (ActorId, ActorAggregate) {
        let actor_id = ActorId(id);
        let mut actor = ActorAggregate::new(actor_id);
        actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));

        if has_guard {
            actor.statuses.attach(crate::content::statuses::guard(3));
        }

        (actor_id, actor)
    }

    fn make_side_lookup(actors: &[(ActorId, ActorAggregate)], side: CombatSide) -> HashMap<ActorId, CombatSide> {
        actors
            .iter()
            .map(|(id, _)| (*id, side))
            .collect()
    }

    #[test]
    fn detect_guard_relations_finds_guards_on_same_side() {
        // Setup: 3 allies, ActorId(1) has guard
        let (id1, actor1) = make_actor(1, CombatSide::Ally, true);   // has guard
        let (id2, actor2) = make_actor(2, CombatSide::Ally, false);  // no guard
        let (id3, actor3) = make_actor(3, CombatSide::Ally, false);  // no guard

        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        actors.insert(id1, actor1.clone());
        actors.insert(id2, actor2.clone());
        actors.insert(id3, actor3.clone());

        let side_lookup = make_side_lookup(&[(id1, actor1), (id2, actor2), (id3, actor3)], CombatSide::Ally);

        // When actor 2 is hit, actor 1 (guard) can protect
        let relations = detect_guard_relations_for_target(id2, &actors, &side_lookup);

        assert_eq!(relations.len(), 1, "Should find exactly 1 guard for actor 2");
        assert_eq!(relations[0].protected, id2);
        assert_eq!(relations[0].guard, id1);
    }

    #[test]
    fn detect_guard_relations_excludes_target_from_guards() {
        // Setup: actor 1 has guard, actor 2 does not
        let (id1, actor1) = make_actor(1, CombatSide::Ally, true);   // has guard
        let (id2, actor2) = make_actor(2, CombatSide::Ally, false);  // no guard, is target

        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        actors.insert(id1, actor1.clone());
        actors.insert(id2, actor2.clone());

        let side_lookup = make_side_lookup(&[(id1, actor1), (id2, actor2)], CombatSide::Ally);

        // When actor 1 (guard) is hit, they should not be their own guard
        let relations = detect_guard_relations_for_target(id1, &actors, &side_lookup);

        assert!(relations.is_empty(), "Target with guard status should not guard themselves");
    }

    #[test]
    fn detect_guard_relations_ignores_enemies() {
        // Setup: 1 ally with guard, 1 enemy
        let (id1, actor1) = make_actor(1, CombatSide::Ally, true);   // ally has guard
        let (id2, actor2) = make_actor(2, CombatSide::Enemy, false); // enemy

        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        actors.insert(id1, actor1.clone());
        actors.insert(id2, actor2.clone());

        let mut side_lookup = HashMap::new();
        side_lookup.insert(id1, CombatSide::Ally);
        side_lookup.insert(id2, CombatSide::Enemy);

        // When enemy is hit, ally guard should not protect
        let relations = detect_guard_relations_for_target(id2, &actors, &side_lookup);

        assert!(relations.is_empty(), "Ally guard should not protect enemy");
    }

    #[test]
    fn detect_guard_relations_multiple_guards_sorted_by_id() {
        // Setup: 3 allies, actors 1 and 3 have guard
        let (id1, actor1) = make_actor(1, CombatSide::Ally, true);   // has guard
        let (id2, actor2) = make_actor(2, CombatSide::Ally, false);  // no guard, is target
        let (id3, actor3) = make_actor(3, CombatSide::Ally, true);   // has guard

        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        actors.insert(id1, actor1.clone());
        actors.insert(id2, actor2.clone());
        actors.insert(id3, actor3.clone());

        let side_lookup = make_side_lookup(&[(id1, actor1), (id2, actor2), (id3, actor3)], CombatSide::Ally);

        // When actor 2 is hit, both actors 1 and 3 can guard (sorted by id)
        let relations = detect_guard_relations_for_target(id2, &actors, &side_lookup);

        assert_eq!(relations.len(), 2, "Should find 2 guards for actor 2");
        assert_eq!(relations[0].guard, id1, "First guard should be actor 1 (lower id)");
        assert_eq!(relations[1].guard, id3, "Second guard should be actor 3 (higher id)");
    }

    #[test]
    fn detect_all_guard_relations_finds_all_protections() {
        // Setup: 2 allies with guard status
        let (id1, actor1) = make_actor(1, CombatSide::Ally, true);
        let (id2, actor2) = make_actor(2, CombatSide::Ally, true);

        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        actors.insert(id1, actor1.clone());
        actors.insert(id2, actor2.clone());

        let side_lookup = make_side_lookup(&[(id1, actor1), (id2, actor2)], CombatSide::Ally);

        let relations = detect_all_guard_relations(&actors, &side_lookup);

        // Actor 1 can guard for actor 2
        // Actor 2 can guard for actor 1
        // Each pair should appear once
        assert_eq!(relations.len(), 2, "Should find 2 guard relations");

        // Verify the pairs
        let has_1_guards_2 = relations.iter().any(|r| r.protected == id2 && r.guard == id1);
        let has_2_guards_1 = relations.iter().any(|r| r.protected == id1 && r.guard == id2);
        assert!(has_1_guards_2, "Actor 1 should guard actor 2");
        assert!(has_2_guards_1, "Actor 2 should guard actor 1");
    }

    #[test]
    fn detect_guard_relations_is_deterministic() {
        let (id1, actor1) = make_actor(1, CombatSide::Ally, true);
        let (id2, actor2) = make_actor(2, CombatSide::Ally, false);
        let (id3, actor3) = make_actor(3, CombatSide::Ally, true);

        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        // Insert in non-sorted order
        actors.insert(id3, actor3.clone());
        actors.insert(id1, actor1.clone());
        actors.insert(id2, actor2.clone());

        let side_lookup = make_side_lookup(&[(id1, actor1), (id2, actor2), (id3, actor3)], CombatSide::Ally);

        let relations1 = detect_guard_relations_for_target(id2, &actors, &side_lookup);
        let relations2 = detect_guard_relations_for_target(id2, &actors, &side_lookup);

        assert_eq!(relations1, relations2, "Guard detection must be deterministic");
        assert_eq!(relations1.len(), 2);
        assert_eq!(relations1[0].guard, id1);
        assert_eq!(relations1[1].guard, id3);
    }

    #[test]
    fn detect_guard_relations_returns_empty_when_no_guards() {
        let (id1, actor1) = make_actor(1, CombatSide::Ally, false);
        let (id2, actor2) = make_actor(2, CombatSide::Ally, false);

        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        actors.insert(id1, actor1.clone());
        actors.insert(id2, actor2.clone());

        let side_lookup = make_side_lookup(&[(id1, actor1), (id2, actor2)], CombatSide::Ally);

        let relations = detect_guard_relations_for_target(id2, &actors, &side_lookup);

        assert!(relations.is_empty(), "Should return empty when no guards");
    }

    #[test]
    fn has_guard_status_detects_guard() {
        let (_id1, actor1) = make_actor(1, CombatSide::Ally, true);
        let (_id2, actor2) = make_actor(2, CombatSide::Ally, false);

        assert!(has_guard_status(&actor1), "Actor with guard status should return true");
        assert!(!has_guard_status(&actor2), "Actor without guard status should return false");
    }

    #[test]
    fn has_guard_status_ignores_expired_statuses() {
        let actor_id = ActorId(1);
        let mut actor = ActorAggregate::new(actor_id);
        actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));

        // Attach guard with duration 1
        actor.statuses.attach(crate::content::statuses::guard(1));
        assert!(has_guard_status(&actor), "Should detect guard before tick");

        // Tick once — duration expires
        let expired = actor.statuses.tick();
        assert_eq!(expired.len(), 1, "Guard should expire after one tick");

        assert!(!has_guard_status(&actor), "Should not detect expired guard");
    }

    #[test]
    fn guard_relation_equality() {
        let rel1 = GuardRelation::new(ActorId(1), ActorId(2));
        let rel2 = GuardRelation::new(ActorId(1), ActorId(2));
        let rel3 = GuardRelation::new(ActorId(1), ActorId(3));

        assert_eq!(rel1, rel2, "Same protected and guard should be equal");
        assert_ne!(rel1, rel3, "Different guard should not be equal");
    }

    #[test]
    fn detect_guard_relations_unknown_target_side() {
        let (id1, actor1) = make_actor(1, CombatSide::Ally, true);

        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        actors.insert(id1, actor1.clone());

        // Empty side_lookup — target side unknown
        let side_lookup = HashMap::new();

        let relations = detect_guard_relations_for_target(id1, &actors, &side_lookup);
        assert!(relations.is_empty(), "Should return empty when target side is unknown");
    }
}