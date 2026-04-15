//! Reactive combat events — first-class representation of post-damage follow-ups.
//!
//! This module provides explicit event types for reactive follow-up work
//! (riposte counter-attacks, guard damage redirection) so they are represented
//! as first-class game-layer events rather than implicit side effects hidden
//! inside damage resolution.
//!
//! ## Data Model
//!
//! [`ReactiveEvent`] is the core type — it captures:
//! - `triggered_on`: who was hit (target of the original attack)
//! - `reactor`: who may react (has riposte/guard marker status)
//! - `kind`: what kind of reactive follow-up this is
//! - `source_skill`: the skill that caused the triggering hit
//! - `attacker`: the actor who initiated the original attack
//! - `damage_amount`: the damage dealt (used for guard redirect calculations)
//!
//! ## Determinism
//!
//! Reactive events are created deterministically from a resolved damage step.
//! The same battle state + same damage step always produces the same set of
//! reactive events in the same order.

use serde::{Deserialize, Serialize};

use framework_combat::skills::SkillId;
use framework_rules::actor::ActorId;

/// Kind of reactive follow-up event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReactiveEventKind {
    /// Counter-attack triggered by riposte status.
    Riposte,
    /// Damage redirection from guard protection.
    GuardRedirect,
}

/// A reactive event triggered by a damage action.
///
/// This is the data-model slice only — no reaction execution happens here.
/// The event is created deterministically from a resolved damage step and
/// stored for later processing by the reactive queue harness (P1-2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReactiveEvent {
    /// The actor who was hit (target of the original attack).
    pub triggered_on: ActorId,
    /// The actor who may react (has riposte/guard marker status).
    pub reactor: ActorId,
    /// What kind of reactive follow-up this is.
    pub kind: ReactiveEventKind,
    /// The skill that caused the triggering hit.
    pub source_skill: SkillId,
    /// The actor who initiated the original attack.
    pub attacker: ActorId,
    /// The damage amount that triggered this event (if applicable).
    pub damage_amount: Option<f64>,
}

impl ReactiveEvent {
    /// Create a new riposte reactive event.
    ///
    /// Called when a riposte-marked actor is hit and may counter-attack.
    pub fn riposte(
        triggered_on: ActorId,
        reactor: ActorId,
        source_skill: SkillId,
        attacker: ActorId,
    ) -> Self {
        ReactiveEvent {
            triggered_on,
            reactor,
            kind: ReactiveEventKind::Riposte,
            source_skill,
            attacker,
            damage_amount: None,
        }
    }

    /// Create a new guard redirect reactive event.
    ///
    /// Called when a guarded target is hit and damage may be redirected.
    pub fn guard_redirect(
        triggered_on: ActorId,
        reactor: ActorId,
        source_skill: SkillId,
        attacker: ActorId,
        damage_amount: f64,
    ) -> Self {
        ReactiveEvent {
            triggered_on,
            reactor,
            kind: ReactiveEventKind::GuardRedirect,
            source_skill,
            attacker,
            damage_amount: Some(damage_amount),
        }
    }

    /// Returns true if this is a riposte event.
    pub fn is_riposte(&self) -> bool {
        self.kind == ReactiveEventKind::Riposte
    }

    /// Returns true if this is a guard redirect event.
    pub fn is_guard_redirect(&self) -> bool {
        self.kind == ReactiveEventKind::GuardRedirect
    }
}

/// Input data extracted from a resolved damage step.
///
/// Used to construct reactive events deterministically.
#[derive(Debug, Clone, PartialEq)]
pub struct DamageStepContext {
    /// The actor who initiated the attack.
    pub attacker: ActorId,
    /// The skill that was used.
    pub skill: SkillId,
    /// The actor who was hit.
    pub target: ActorId,
    /// The damage amount dealt (None if damage was 0 or resisted).
    pub damage_amount: Option<f64>,
}

impl DamageStepContext {
    /// Create a new damage step context.
    pub fn new(
        attacker: ActorId,
        skill: SkillId,
        target: ActorId,
        damage_amount: Option<f64>,
    ) -> Self {
        DamageStepContext {
            attacker,
            skill,
            target,
            damage_amount,
        }
    }
}

/// Build reactive events from a damage step context and reactor eligibility.
///
/// This is the deterministic constructor — the same inputs always produce
/// the same reactive events in the same order.
pub fn build_reactive_events(
    ctx: &DamageStepContext,
    reactor: ActorId,
    kind: ReactiveEventKind,
) -> Vec<ReactiveEvent> {
    let event = match kind {
        ReactiveEventKind::Riposte => ReactiveEvent::riposte(
            ctx.target,
            reactor,
            ctx.skill.clone(),
            ctx.attacker,
        ),
        ReactiveEventKind::GuardRedirect => {
            ReactiveEvent::guard_redirect(
                ctx.target,
                reactor,
                ctx.skill.clone(),
                ctx.attacker,
                ctx.damage_amount.unwrap_or(0.0),
            )
        }
    };
    vec![event]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reactive_event_ripost_created_deterministically() {
        // Same inputs always produce the same event
        let ctx = DamageStepContext::new(
            ActorId(10),                    // attacker
            SkillId::new("normal_attack"),  // skill
            ActorId(1),                     // target (has riposte)
            Some(24.0),                     // damage amount
        );

        let events1 = build_reactive_events(&ctx, ActorId(1), ReactiveEventKind::Riposte);
        let events2 = build_reactive_events(&ctx, ActorId(1), ReactiveEventKind::Riposte);

        assert_eq!(events1.len(), 1, "Should produce exactly one reactive event");
        assert_eq!(events1, events2, "Same inputs must produce identical events");

        let event = &events1[0];
        assert_eq!(event.triggered_on, ActorId(1));
        assert_eq!(event.reactor, ActorId(1));
        assert!(event.is_riposte());
        assert!(!event.is_guard_redirect());
        assert_eq!(event.attacker, ActorId(10));
        assert_eq!(event.source_skill, SkillId::new("normal_attack"));
    }

    #[test]
    fn reactive_event_guard_redirect_created_deterministically() {
        let ctx = DamageStepContext::new(
            ActorId(10),
            SkillId::new("heavy_strike"),
            ActorId(2),    // guarded target
            Some(40.0),
        );

        let events1 = build_reactive_events(&ctx, ActorId(3), ReactiveEventKind::GuardRedirect);
        let events2 = build_reactive_events(&ctx, ActorId(3), ReactiveEventKind::GuardRedirect);

        assert_eq!(events1.len(), 1);
        assert_eq!(events1, events2);

        let event = &events1[0];
        assert!(event.is_guard_redirect());
        assert!(!event.is_riposte());
        assert_eq!(event.triggered_on, ActorId(2));
        assert_eq!(event.reactor, ActorId(3)); // guard
        assert_eq!(event.damage_amount, Some(40.0));
    }

    #[test]
    fn reactive_event_equality_by_value() {
        let ctx1 = DamageStepContext::new(
            ActorId(5),
            SkillId::new("fireball"),
            ActorId(7),
            Some(30.0),
        );
        let ctx2 = DamageStepContext::new(
            ActorId(5),
            SkillId::new("fireball"),
            ActorId(7),
            Some(30.0),
        );

        let events1 = build_reactive_events(&ctx1, ActorId(7), ReactiveEventKind::Riposte);
        let events2 = build_reactive_events(&ctx2, ActorId(7), ReactiveEventKind::Riposte);

        assert_eq!(events1, events2);
    }

    #[test]
    fn reactive_event_different_inputs_produce_different_events() {
        let ctx1 = DamageStepContext::new(
            ActorId(10),
            SkillId::new("normal_attack"),
            ActorId(1),
            Some(24.0),
        );
        let ctx2 = DamageStepContext::new(
            ActorId(20), // different attacker
            SkillId::new("normal_attack"),
            ActorId(1),
            Some(24.0),
        );

        let events1 = build_reactive_events(&ctx1, ActorId(1), ReactiveEventKind::Riposte);
        let events2 = build_reactive_events(&ctx2, ActorId(1), ReactiveEventKind::Riposte);

        assert_ne!(events1, events2, "Different inputs should produce different events");
        assert_eq!(events1[0].attacker, ActorId(10));
        assert_eq!(events2[0].attacker, ActorId(20));
    }

    #[test]
    fn damage_step_context_captures_all_fields() {
        let ctx = DamageStepContext::new(
            ActorId(99),
            SkillId::new("test_skill"),
            ActorId(42),
            Some(15.5),
        );

        assert_eq!(ctx.attacker, ActorId(99));
        assert_eq!(ctx.target, ActorId(42));
        assert_eq!(ctx.skill, SkillId::new("test_skill"));
        assert_eq!(ctx.damage_amount, Some(15.5));
    }

    #[test]
    fn damage_step_context_with_no_damage() {
        // Miss or resist: damage_amount is None
        let ctx = DamageStepContext::new(
            ActorId(10),
            SkillId::new("miss_skill"),
            ActorId(1),
            None,
        );

        let events = build_reactive_events(&ctx, ActorId(1), ReactiveEventKind::Riposte);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].damage_amount, None);
    }
}