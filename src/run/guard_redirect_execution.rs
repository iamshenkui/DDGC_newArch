//! Guard damage redirection execution — redirects damage from protected targets to guards.
//!
//! This module is the execution slice for B-008: when a guarded target is hit,
//! the guarding ally absorbs the damage instead through the reactive queue harness.
//!
//! ## Design
//!
//! When a reactive guard redirect event is drained from the queue:
//! 1. The damage applied to the protected target is reversed (undone)
//! 2. The same damage is applied to the guarding actor instead
//! 3. The guard status is consumed after redirection (one-shot per hit)
//!
//! ## Damage Flow
//!
//! The original damage to the protected target has already been applied by the
//! time the reactive queue is processed. So guard redirect works by:
//! - Reversing: subtract damage from protected target's HP
//! - Redirecting: add same damage to guard's HP
//!
//! Net effect: protected target takes no net damage, guard absorbs the full hit.
//!
//! ## Determinism
//!
//! Guard redirect execution is deterministic: the same reactive event always
//! produces the same damage redistribution in the same trace order.

use framework_rules::actor::{ActorAggregate, ActorId};
use framework_rules::attributes::{AttributeKey, AttributeValue, ATTR_HEALTH};
use std::collections::HashMap;

use crate::run::reactive_events::ReactiveEvent;

/// The status kind key for guard marker statuses.
pub const GUARD_KIND: &str = "guard";

/// Execute a single guard redirect from a reactive event.
///
/// The guard (who is protecting the target) absorbs the damage that was
/// originally targeted at the protected actor.
///
/// Returns the damage amount redirected, if successful.
#[must_use]
pub fn execute_guard_redirect(
    event: &ReactiveEvent,
    actors: &mut HashMap<ActorId, ActorAggregate>,
) -> Option<f64> {
    if !event.is_guard_redirect() {
        return None;
    }

    let damage_amount = event.damage_amount?;

    let protected = event.triggered_on;
    let guard = event.reactor;

    // Step 1: Reverse the damage applied to the protected target
    // (subtract what was already added by resolve_skill)
    if let Some(protected_actor) = actors.get_mut(&protected) {
        let current_hp = protected_actor
            .effective_attribute(&AttributeKey::new(ATTR_HEALTH));
        protected_actor.set_base(
            AttributeKey::new(ATTR_HEALTH),
            AttributeValue(current_hp.0 + damage_amount),
        );
    }

    // Step 2: Apply the same damage to the guard
    if let Some(guard_actor) = actors.get_mut(&guard) {
        let current_hp = guard_actor.effective_attribute(&AttributeKey::new(ATTR_HEALTH));
        guard_actor.set_base(
            AttributeKey::new(ATTR_HEALTH),
            AttributeValue(current_hp.0 - damage_amount),
        );

        // Consume the guard status after redirect (one-shot per hit)
        consume_guard_status(actors, guard);
    }

    Some(damage_amount)
}

/// Consume the guard status from an actor after redirect fires.
///
/// Iterates through active statuses and removes the guard status.
/// This is the "one-shot per hit" behavior: the redirect
/// consumes the guard marker.
fn consume_guard_status(
    actors: &mut HashMap<ActorId, ActorAggregate>,
    guard: ActorId,
) {
    if let Some(actor) = actors.get_mut(&guard) {
        let guard_id = actor
            .statuses
            .active()
            .iter()
            .find(|(_, status)| status.kind.0 == GUARD_KIND)
            .map(|(id, _)| *id);

        if let Some(id) = guard_id {
            actor.statuses.remove(id);
        }
    }
}

/// Check if an actor has an active guard status.
pub fn has_guard_status(
    actor: &ActorAggregate,
) -> bool {
    actor
        .statuses
        .active()
        .values()
        .any(|status| status.kind.0 == GUARD_KIND)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guard_kind_constant_is_correct() {
        assert_eq!(GUARD_KIND, "guard");
    }

    #[test]
    fn execute_guard_redirect_reverses_damage_to_protected() {
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();

        // Protected target with 50 HP
        let protected_id = ActorId(1);
        let mut protected = ActorAggregate::new(protected_id);
        protected.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(50.0));
        actors.insert(protected_id, protected);

        // Guard with 100 HP
        let guard_id = ActorId(2);
        let mut guard = ActorAggregate::new(guard_id);
        guard.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
        guard.statuses.attach(crate::content::statuses::guard(3));
        actors.insert(guard_id, guard);

        // Create a guard redirect event: 30 damage from attacker(3) to protected(1), guard(2)
        let event = ReactiveEvent::guard_redirect(
            ActorId(1),                    // triggered_on (protected)
            ActorId(2),                    // reactor (guard)
            framework_combat::skills::SkillId::new("heavy_strike"),
            ActorId(3),                    // attacker
            30.0,
        );

        let result = execute_guard_redirect(&event, &mut actors);

        assert!(result.is_some(), "Guard redirect should succeed");
        assert_eq!(result.unwrap(), 30.0);

        // Protected target should be restored to 50 (50 + 30 reversed)
        let protected_hp = actors[&protected_id]
            .effective_attribute(&AttributeKey::new(ATTR_HEALTH));
        assert_eq!(protected_hp.0, 80.0, "Protected should have damage reversed");

        // Guard should take the damage: 100 - 30 = 70
        let guard_hp = actors[&guard_id]
            .effective_attribute(&AttributeKey::new(ATTR_HEALTH));
        assert_eq!(guard_hp.0, 70.0, "Guard should absorb the damage");

        // Guard status should be consumed
        let guard_actor = &actors[&guard_id];
        assert!(!has_guard_status(guard_actor), "Guard status should be consumed");
    }

    #[test]
    fn execute_guard_redirect_returns_none_for_non_guard_event() {
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();

        let event = ReactiveEvent::riposte(
            ActorId(1),
            ActorId(2),
            framework_combat::skills::SkillId::new("normal_attack"),
            ActorId(3),
        );

        let result = execute_guard_redirect(&event, &mut actors);

        assert!(result.is_none(), "Non-guard event should return None");
    }

    #[test]
    fn execute_guard_redirect_with_no_damage_amount() {
        use crate::run::reactive_events::ReactiveEventKind;

        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();

        let protected_id = ActorId(1);
        let mut protected = ActorAggregate::new(protected_id);
        protected.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(50.0));
        actors.insert(protected_id, protected);

        let guard_id = ActorId(2);
        let mut guard = ActorAggregate::new(guard_id);
        guard.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
        guard.statuses.attach(crate::content::statuses::guard(3));
        actors.insert(guard_id, guard);

        // Event with no damage amount (e.g., missed attack)
        let event = ReactiveEvent {
            triggered_on: ActorId(1),
            reactor: ActorId(2),
            kind: ReactiveEventKind::GuardRedirect,
            source_skill: framework_combat::skills::SkillId::new("miss_skill"),
            attacker: ActorId(3),
            damage_amount: None,
        };

        let result = execute_guard_redirect(&event, &mut actors);

        assert!(result.is_none(), "Guard redirect with no damage should return None");
    }

    #[test]
    fn guard_redirect_consumes_guard_status() {
        use crate::content::statuses::guard as guard_status;

        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();

        let guard_id = ActorId(2);
        let mut guard_actor = ActorAggregate::new(guard_id);
        guard_actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
        guard_actor.statuses.attach(guard_status(3));
        actors.insert(guard_id, guard_actor);

        let protected_id = ActorId(1);
        let mut protected = ActorAggregate::new(protected_id);
        protected.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(50.0));
        actors.insert(protected_id, protected);

        let event = ReactiveEvent::guard_redirect(
            protected_id,
            guard_id,
            framework_combat::skills::SkillId::new("attack"),
            ActorId(3),
            25.0,
        );

        let _ = execute_guard_redirect(&event, &mut actors);

        let guard_actor = &actors[&guard_id];
        assert!(!has_guard_status(guard_actor), "Guard status should be consumed after redirect");
    }
}