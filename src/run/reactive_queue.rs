//! Deterministic reactive queue harness for post-damage follow-up actions.
//!
//! This module provides the execution backbone for reactive combat mechanics
//! (riposte counter-attacks, guard damage redirection). It is the plumbing
//! layer that sits between the damage resolution step (which produces
//! `ReactiveEvent`s) and the actual reaction execution.
//!
//! ## Design Principles
//!
//! 1. **Deterministic ordering**: Events are stored in a `BTreeMap` keyed by
//!    `reactor ActorId`, guaranteeing the same input always produces the same
//!    evaluation sequence regardless of insertion order.
//!
//! 2. **No re-application of the original hit**: The queue only holds *reactive*
//!    follow-up events. The original damage is already applied; reactions are
//!    separate counter-attacks or redirects that happen after.
//!
//! 3. **No recursion / no infinite loops**: Each `(triggered_on, reactor)` pair
//!    can only be enqueued once per trigger cycle. The `already_queued` set
//!    prevents the same actor from reacting to the same hit multiple times.
//!
//! 4. **One-shot per trigger**: When an actor reacts, they are marked as
//!    "reacted" and will not react again to the same triggering event, even
//!    if the reaction itself would otherwise qualify.
//!
//! ## Usage
//!
//! ```ignore
//! let mut queue = ReactiveQueue::new();
//! queue.enqueue(reactive_event_1);
//! queue.enqueue(reactive_event_2);
//!
//! // Drain and process each reaction in deterministic order
//! while let Some(event) = queue.drain_next() {
//!     // execute reaction...
//! }
//! ```

use std::collections::BTreeMap;

use framework_rules::actor::ActorId;

use crate::run::reactive_events::ReactiveEvent;

/// A deterministic queue for scheduling reactive combat follow-up actions.
///
/// Reactive events (riposte counter-attacks, guard redirects) are enqueued
/// after a damage step and processed in a stable, deterministic order
/// (by reactor ActorId ascending). The queue prevents:
///
/// - **Duplicate reactions**: The same `(triggered_on, reactor)` pair can only
///   be enqueued once per trigger cycle via `already_queued`
/// - **Infinite recursion**: Reacted actors are tracked in `already_reacted`
///   and cannot trigger further reactive events within the same chain
///
/// # Example
///
/// ```ignore
/// let mut queue = ReactiveQueue::new();
/// queue.enqueue(riposte_event.clone());
/// queue.enqueue(guard_event.clone());
///
/// // Reactions come out in ActorId order: ActorId(1) before ActorId(5)
/// while let Some(event) = queue.drain_next() {
///     process_reaction(event);
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct ReactiveQueue {
    /// Events queued for processing, keyed by (triggered_on.0, reactor.0).
    /// BTreeMap gives us deterministic ordering (sorted by triggered_on first, then reactor).
    /// We use u64 tuple since ActorId doesn't implement Ord but ActorId.0 is u64.
    events: BTreeMap<(u64, u64), ReactiveEvent>,
    /// Set of (triggered_on, reactor) pairs already enqueued in this cycle.
    /// Used to prevent duplicate enqueues for the same trigger.
    already_queued: std::collections::HashSet<(ActorId, ActorId)>,
    /// Set of actors who have already reacted in this chain.
    /// Once an actor reacts, they cannot trigger further reactive events
    /// in the same chain (prevents infinite recursion).
    already_reacted: std::collections::HashSet<ActorId>,
}

impl ReactiveQueue {
    /// Create a new empty reactive queue.
    pub fn new() -> Self {
        ReactiveQueue {
            events: BTreeMap::new(),
            already_queued: std::collections::HashSet::new(),
            already_reacted: std::collections::HashSet::new(),
        }
    }

    /// Enqueue a reactive event for later processing.
    ///
    /// Returns `true` if the event was enqueued, `false` if it was skipped
    /// because:
    /// - The `(triggered_on, reactor)` pair is already in `already_queued`
    ///   (same actor already queued for this same trigger)
    /// - The `reactor` is in `already_reacted` (actor already reacted and
    ///   cannot trigger further reactions in this chain)
    ///
    /// Events are keyed by `reactor ActorId` in the internal `BTreeMap`,
    /// guaranteeing deterministic evaluation order regardless of enqueue order.
    pub fn enqueue(&mut self, event: ReactiveEvent) -> bool {
        // Prevent duplicate enqueues for the same (triggered_on, reactor) pair
        let key = (event.triggered_on, event.reactor);
        if self.already_queued.contains(&key) {
            return false;
        }

        // Prevent recursion: an actor that has already reacted in this chain
        // cannot trigger further reactive events
        if self.already_reacted.contains(&event.reactor) {
            return false;
        }

        self.already_queued.insert(key);
        // BTreeMap::insert replaces existing value for the same key.
        // Since we deduplicate by (triggered_on, reactor) above, we never
        // have key collisions at this point.
        // Key is (triggered_on.0, reactor.0) for deterministic ordering.
        self.events.insert((event.triggered_on.0, event.reactor.0), event);
        true
    }

    /// Drain and return the next reactive event in deterministic order.
    ///
    /// Returns `None` when the queue is empty.
    ///
    /// When an event is drained, the `reactor` is added to `already_reacted`
    /// to prevent the same actor from reacting multiple times in a chain.
    pub fn drain_next(&mut self) -> Option<ReactiveEvent> {
        // BTreeMap iteration yields events sorted by (triggered_on.0, reactor.0) —
        // this is our deterministic ordering guarantee
        let event = self.events.pop_first();
        if let Some(ref e) = event {
            self.already_reacted.insert(e.1.reactor);
        }
        event.map(|(_, v)| v)
    }

    /// Returns the number of events currently in the queue.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Returns `true` if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Mark an actor as having reacted, preventing them from triggering
    /// further reactive events in the current chain.
    ///
    /// This is called by the reaction execution layer after a reaction
    /// successfully fires.
    pub fn mark_reacted(&mut self, actor: ActorId) {
        self.already_reacted.insert(actor);
    }

    /// Returns `true` if the given actor has already reacted in this chain.
    pub fn has_reacted(&self, actor: ActorId) -> bool {
        self.already_reacted.contains(&actor)
    }

    /// Returns `true` if the given (triggered_on, reactor) pair is already
    /// enqueued in this cycle.
    pub fn is_enqueued(&self, triggered_on: ActorId, reactor: ActorId) -> bool {
        self.already_queued.contains(&(triggered_on, reactor))
    }

    /// Drain all remaining events from the queue.
    ///
    /// As each event is drained, the reactor is marked as reacted.
    /// Returns events in deterministic ActorId order.
    pub fn drain_all(&mut self) -> Vec<ReactiveEvent> {
        let mut out = Vec::new();
        while let Some(event) = self.drain_next() {
            out.push(event);
        }
        out
    }

    /// Reset the queue for a new trigger cycle.
    ///
    /// Clears `already_queued` and `already_reacted` but leaves `events`
    /// empty. Call this at the start of processing a new damage trigger.
    pub fn reset_for_next_trigger(&mut self) {
        self.already_queued.clear();
        self.already_reacted.clear();
        self.events.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run::reactive_events::ReactiveEventKind;

    fn make_riposte(triggered_on: ActorId, reactor: ActorId) -> ReactiveEvent {
        ReactiveEvent::riposte(
            triggered_on,
            reactor,
            framework_combat::skills::SkillId::new("normal_attack"),
            ActorId(99), // attacker
        )
    }

    fn make_guard_redirect(triggered_on: ActorId, reactor: ActorId, damage: f64) -> ReactiveEvent {
        ReactiveEvent::guard_redirect(
            triggered_on,
            reactor,
            framework_combat::skills::SkillId::new("heavy_strike"),
            ActorId(99),
            damage,
        )
    }

    // ── Determinism tests ──────────────────────────────────────────────────────

    #[test]
    fn drain_order_is_deterministic_by_actor_id() {
        let mut queue = ReactiveQueue::new();

        // Enqueue in "wrong" order (high IDs first)
        queue.enqueue(make_riposte(ActorId(1), ActorId(30)));
        queue.enqueue(make_riposte(ActorId(1), ActorId(10)));
        queue.enqueue(make_riposte(ActorId(1), ActorId(20)));

        // Drain should always come out in ActorId ascending order
        let events = queue.drain_all();
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].reactor, ActorId(10), "first should be actor 10");
        assert_eq!(events[1].reactor, ActorId(20), "second should be actor 20");
        assert_eq!(events[2].reactor, ActorId(30), "third should be actor 30");
    }

    #[test]
    fn same_inputs_produce_same_queue_state() {
        fn build_queue() -> ReactiveQueue {
            let mut q = ReactiveQueue::new();
            q.enqueue(make_riposte(ActorId(5), ActorId(2)));
            q.enqueue(make_guard_redirect(ActorId(5), ActorId(3), 40.0));
            q.enqueue(make_riposte(ActorId(5), ActorId(1)));
            q
        }

        let mut q1 = build_queue();
        let mut q2 = build_queue();

        let events1 = q1.drain_all();
        let events2 = q2.drain_all();

        assert_eq!(events1.len(), events2.len());
        for (e1, e2) in events1.iter().zip(events2.iter()) {
            assert_eq!(e1, e2, "Same inputs must produce identical events");
        }
    }

    #[test]
    fn deterministic_order_with_mixed_event_types() {
        let mut queue = ReactiveQueue::new();

        queue.enqueue(make_guard_redirect(ActorId(2), ActorId(5), 30.0));
        queue.enqueue(make_riposte(ActorId(2), ActorId(3)));
        queue.enqueue(make_guard_redirect(ActorId(2), ActorId(7), 20.0));
        queue.enqueue(make_riposte(ActorId(2), ActorId(1)));

        let events = queue.drain_all();
        assert_eq!(events.len(), 4);

        // Must be sorted by reactor ActorId regardless of enqueue order or type
        assert_eq!(events[0].reactor, ActorId(1));
        assert_eq!(events[0].kind, ReactiveEventKind::Riposte);

        assert_eq!(events[1].reactor, ActorId(3));
        assert_eq!(events[1].kind, ReactiveEventKind::Riposte);

        assert_eq!(events[2].reactor, ActorId(5));
        assert_eq!(events[2].kind, ReactiveEventKind::GuardRedirect);

        assert_eq!(events[3].reactor, ActorId(7));
        assert_eq!(events[3].kind, ReactiveEventKind::GuardRedirect);
    }

    // ── No-duplicate tests ─────────────────────────────────────────────────────

    #[test]
    fn duplicate_enqueued_pair_is_skipped() {
        let mut queue = ReactiveQueue::new();

        let event1 = make_riposte(ActorId(1), ActorId(5));
        let event2 = make_riposte(ActorId(1), ActorId(5)); // Same (triggered_on, reactor)

        assert!(queue.enqueue(event1));
        assert!(!queue.enqueue(event2), "duplicate pair should be skipped");

        let events = queue.drain_all();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].reactor, ActorId(5));
    }

    #[test]
    fn same_reactor_different_trigger_is_enqueued() {
        let mut queue = ReactiveQueue::new();

        // Actor 5 reacts to being hit by attacker 1
        queue.enqueue(make_riposte(ActorId(1), ActorId(5)));
        // Actor 5 reacts to being hit by attacker 2 (different trigger)
        queue.enqueue(make_riposte(ActorId(2), ActorId(5)));

        let events = queue.drain_all();
        assert_eq!(events.len(), 2, "same reactor, different triggers = both enqueued");
    }

    #[test]
    fn same_trigger_different_reactor_both_enqueued() {
        let mut queue = ReactiveQueue::new();

        // Both actor 3 and actor 5 react to actor 1 being hit
        queue.enqueue(make_riposte(ActorId(1), ActorId(3)));
        queue.enqueue(make_riposte(ActorId(1), ActorId(5)));

        let events = queue.drain_all();
        assert_eq!(events.len(), 2);
    }

    // ── No-recursion tests ─────────────────────────────────────────────────────

    #[test]
    fn reacted_actor_cannot_enqueue_further_events() {
        let mut queue = ReactiveQueue::new();

        queue.enqueue(make_riposte(ActorId(1), ActorId(5)));
        let event = queue.drain_next().unwrap();

        // Actor 5 has now reacted
        assert!(queue.has_reacted(ActorId(5)));
        assert_eq!(event.reactor, ActorId(5));

        // Try to enqueue another event where actor 5 is the reactor
        let blocked = queue.enqueue(make_riposte(ActorId(2), ActorId(5)));
        assert!(!blocked, "already-reacted actor should be blocked from enqueueing");
    }

    #[test]
    fn drain_all_marks_all_reactors_as_reacted() {
        let mut queue = ReactiveQueue::new();

        queue.enqueue(make_riposte(ActorId(1), ActorId(2)));
        queue.enqueue(make_riposte(ActorId(1), ActorId(3)));
        queue.enqueue(make_riposte(ActorId(1), ActorId(4)));

        queue.drain_all();

        assert!(queue.has_reacted(ActorId(2)));
        assert!(queue.has_reacted(ActorId(3)));
        assert!(queue.has_reacted(ActorId(4)));

        // All should now be blocked from enqueueing
        assert!(!queue.enqueue(make_riposte(ActorId(5), ActorId(2))));
        assert!(!queue.enqueue(make_riposte(ActorId(5), ActorId(3))));
        assert!(!queue.enqueue(make_riposte(ActorId(5), ActorId(4))));
    }

    #[test]
    fn no_recursion_riposte_chain_terminates() {
        // Simulate: Actor 1 hits Actor 2 (riposte), Actor 2 hits back (riposte),
        // Actor 1 hits back again — but Actor 2 should NOT riposte again
        // because they already reacted in this chain.
        let mut queue = ReactiveQueue::new();

        // Initial hit: 1 → 2, 2 ripostes
        queue.enqueue(make_riposte(ActorId(1), ActorId(2)));
        let first_reaction = queue.drain_next().unwrap();
        assert_eq!(first_reaction.reactor, ActorId(2));

        // Actor 2's riposte hits Actor 1. Now Actor 2 is already reacted —
        // so if Actor 1 hits back and Actor 2 would riposte again, it should be blocked.
        // This is the recursion termination guarantee.

        // Actor 2 is in already_reacted
        assert!(queue.has_reacted(ActorId(2)));

        // Actor 2 tries to riposte again (triggered_on=1, reactor=2) - should be blocked
        // because Actor 2 already reacted in this chain.
        let blocked = queue.enqueue(make_riposte(ActorId(1), ActorId(2)));
        assert!(!blocked, "Actor 2 already reacted and should not riposte again");
    }

    #[test]
    fn reset_for_next_trigger_clears_reacted_tracking() {
        let mut queue = ReactiveQueue::new();

        queue.enqueue(make_riposte(ActorId(1), ActorId(5)));
        queue.drain_next().unwrap();

        assert!(queue.has_reacted(ActorId(5)));

        // Reset for next trigger (new damage event)
        queue.reset_for_next_trigger();

        // Now actor 5 can react again
        assert!(!queue.has_reacted(ActorId(5)));
        assert!(queue.enqueue(make_riposte(ActorId(2), ActorId(5))));
    }

    // ── Queue state tests ──────────────────────────────────────────────────────

    #[test]
    fn is_empty_after_creation() {
        let queue = ReactiveQueue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn len_reflects_queue_size() {
        let mut queue = ReactiveQueue::new();
        assert_eq!(queue.len(), 0);

        queue.enqueue(make_riposte(ActorId(1), ActorId(2)));
        assert_eq!(queue.len(), 1);

        queue.enqueue(make_riposte(ActorId(1), ActorId(3)));
        assert_eq!(queue.len(), 2);

        queue.enqueue(make_riposte(ActorId(1), ActorId(4)));
        assert_eq!(queue.len(), 3);

        queue.drain_next();
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn is_enqueued_checks_correctly() {
        let mut queue = ReactiveQueue::new();

        queue.enqueue(make_riposte(ActorId(1), ActorId(5)));

        assert!(queue.is_enqueued(ActorId(1), ActorId(5)));
        assert!(!queue.is_enqueued(ActorId(1), ActorId(3)));
        assert!(!queue.is_enqueued(ActorId(2), ActorId(5)));
    }

    #[test]
    fn mark_reacted_prevents_future_enqueues() {
        let mut queue = ReactiveQueue::new();

        // Manually mark actor 7 as having reacted
        queue.mark_reacted(ActorId(7));

        // Actor 7 cannot enqueue any new events
        assert!(!queue.enqueue(make_riposte(ActorId(1), ActorId(7))));
        assert!(!queue.enqueue(make_guard_redirect(ActorId(1), ActorId(7), 20.0)));
    }

    // ── Original hit non-reapplication tests ─────────────────────────────────

    #[test]
    fn queue_contains_only_reactive_events_not_original_damage() {
        // The queue only holds ReactiveEvents produced by build_reactive_events().
        // The original damage that triggered the reactive event is already applied
        // before the queue is involved — the queue just schedules the follow-up.
        let mut queue = ReactiveQueue::new();

        let reactive = make_riposte(ActorId(1), ActorId(5));
        queue.enqueue(reactive);

        let events = queue.drain_all();
        assert_eq!(events.len(), 1);

        // The drained event is a riposte, not a damage application.
        // The original damage (e.g., 24 hp from normal_attack) was already
        // applied during the damage resolution step that preceded queue entry.
        assert!(events[0].is_riposte());
        assert!(!events[0].is_guard_redirect());
    }

    #[test]
    fn drain_next_returns_none_when_empty() {
        let mut queue = ReactiveQueue::new();
        assert!(queue.drain_next().is_none());
    }
}