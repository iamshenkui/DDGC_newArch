//! DDGC status effects — migrated from DDGC buff/debuff definitions.
//!
//! DDGC statuses use the framework's `StatusEffect` + `Modifier` system.
//! Game-specific mechanics (riposte triggers, death's door, stress thresholds)
//! are implemented as marker statuses + game-layer logic (B-008, B-003).

use framework_rules::attributes::{AttributeKey, AttributeValue, ATTR_HEALTH};
use framework_rules::modifiers::{Modifier, ModifierSource};
use framework_rules::statuses::{StackRule, StatusEffect, StatusKind};

use crate::content::actors::ATTR_STRESS;

// ── Status Factory Functions ──────────────────────────────────────────────

/// Bleed — damage-over-time that stacks up to 3 times.
///
/// DDGC reference: deals N damage per tick, duration 3 turns.
/// Each stack adds an independent modifier. `CombatResolver::end_turn()`
/// auto-ticks statuses, so bleed damage applies between turns.
pub fn bleed(damage_per_tick: f64, duration: u32) -> StatusEffect {
    StatusEffect::new(
        StatusKind::new("bleed"),
        Some(duration),
        vec![Modifier::new(
            ModifierSource::Status,
            AttributeKey::new(ATTR_HEALTH),
            AttributeValue(-damage_per_tick),
            0,
            Some(duration),
        )],
        StackRule::Stack { max: 3 },
    )
}

/// Stun — prevents the actor from acting for one turn.
///
/// DDGC reference: the actor skips their next turn.
/// No modifier is needed — game-layer code checks for the "stun" status
/// kind and skips the actor's turn. The framework's `StatusEffect::tick()`
/// decrements the duration automatically.
pub fn stun(duration: u32) -> StatusEffect {
    StatusEffect::new(
        StatusKind::new("stun"),
        Some(duration),
        vec![],
        StackRule::Replace,
    )
}

/// Riposte — marker status that triggers a counter-attack when hit.
///
/// DDGC reference: the next time this actor is hit, they counter-attack.
/// No framework modifier — game-layer code checks for "riposte" status
/// after each damage event and applies the counter (B-008).
pub fn riposte(duration: u32) -> StatusEffect {
    StatusEffect::new(
        StatusKind::new("riposte"),
        Some(duration),
        vec![],
        StackRule::Replace,
    )
}

// ── Additional DDGC Statuses (for migration completeness) ────────────────

/// Horror — stress-over-time effect.
///
/// DDGC reference: increases stress each turn. Uses the DDGC-specific
/// `ATTR_STRESS` key with a positive modifier (stress goes up).
pub fn horror(stress_per_tick: f64, duration: u32) -> StatusEffect {
    StatusEffect::new(
        StatusKind::new("horror"),
        Some(duration),
        vec![Modifier::new(
            ModifierSource::Status,
            AttributeKey::new(ATTR_STRESS),
            AttributeValue(stress_per_tick),
            0,
            Some(duration),
        )],
        StackRule::Stack { max: 3 },
    )
}

/// Burn — fire damage-over-time that stacks up to 3 times.
///
/// DDGC reference: deals N fire damage per tick, duration 3 turns.
/// Functionally similar to bleed but represents fire damage.
pub fn burn(damage_per_tick: f64, duration: u32) -> StatusEffect {
    StatusEffect::new(
        StatusKind::new("burn"),
        Some(duration),
        vec![Modifier::new(
            ModifierSource::Status,
            AttributeKey::new(ATTR_HEALTH),
            AttributeValue(-damage_per_tick),
            0,
            Some(duration),
        )],
        StackRule::Stack { max: 3 },
    )
}

/// Frozen — ice damage-over-time that stacks up to 3 times.
///
/// DDGC reference: deals N ice damage per tick, duration 3 turns.
/// Functionally similar to bleed but represents ice/frozen damage.
pub fn frozen(damage_per_tick: f64, duration: u32) -> StatusEffect {
    StatusEffect::new(
        StatusKind::new("frozen"),
        Some(duration),
        vec![Modifier::new(
            ModifierSource::Status,
            AttributeKey::new(ATTR_HEALTH),
            AttributeValue(-damage_per_tick),
            0,
            Some(duration),
        )],
        StackRule::Stack { max: 3 },
    )
}

/// Tagged — marker status for mark/tag mechanics.
///
/// DDGC reference: tagged status enables conditional effects (Hunter's mark
/// system, Tank's self-mark). Duration typically 2–3 rounds.
/// No modifier — game-layer code checks for "tagged" status kind.
pub fn tagged(duration: u32) -> StatusEffect {
    StatusEffect::new(
        StatusKind::new("tagged"),
        Some(duration),
        vec![],
        StackRule::Replace,
    )
}

/// Guard — marker status for protection/guard mechanics.
///
/// DDGC reference: Tank's protect skill guards an ally for 3 rounds,
/// reducing their incoming damage by 20%.
/// No modifier — game-layer code applies damage reduction when guard is active.
pub fn guard(duration: u32) -> StatusEffect {
    StatusEffect::new(
        StatusKind::new("guard"),
        Some(duration),
        vec![],
        StackRule::Replace,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use framework_rules::actor::{ActorAggregate, ActorId};

    #[test]
    fn migrated_status_pack_applies_expected_effects() {
        // Test bleed: reduces health via modifier
        let mut actor = ActorAggregate::new(ActorId(1));
        actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(50.0));

        actor.statuses.attach(bleed(5.0, 3));
        let health = actor.effective_attribute(&AttributeKey::new(ATTR_HEALTH));
        assert_eq!(health.0, 45.0, "Bleed should reduce effective health by 5");

        // Tick: duration should decrease
        let expired = actor.statuses.tick();
        assert!(expired.is_empty(), "Bleed should not expire after 1 tick");

        // Test stun: no modifier, but status is present
        let mut actor2 = ActorAggregate::new(ActorId(2));
        actor2.statuses.attach(stun(1));
        assert!(actor2.statuses.active().len() == 1);
        // Tick once: stun should expire
        let expired = actor2.statuses.tick();
        assert_eq!(expired.len(), 1, "Stun should expire after 1 tick");

        // Test riposte: marker status, no modifier
        let mut actor3 = ActorAggregate::new(ActorId(3));
        actor3.statuses.attach(riposte(1));
        assert!(actor3.statuses.active().len() == 1);
        let health3 = actor3.effective_attribute(&AttributeKey::new(ATTR_HEALTH));
        assert_eq!(health3.0, 0.0, "Riposte has no health modifier");
    }

    #[test]
    fn bleed_stacks_up_to_max() {
        let mut actor = ActorAggregate::new(ActorId(10));
        actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));

        // Attach 3 bleeds
        actor.statuses.attach(bleed(5.0, 3));
        actor.statuses.attach(bleed(5.0, 3));
        actor.statuses.attach(bleed(5.0, 3));

        let health = actor.effective_attribute(&AttributeKey::new(ATTR_HEALTH));
        assert_eq!(health.0, 85.0, "3 bleed stacks should reduce health by 15");
    }

    #[test]
    fn horror_increases_stress() {
        let mut actor = ActorAggregate::new(ActorId(20));
        actor.set_base(AttributeKey::new(ATTR_STRESS), AttributeValue(0.0));

        actor.statuses.attach(horror(10.0, 3));
        let stress = actor.effective_attribute(&AttributeKey::new(ATTR_STRESS));
        assert_eq!(stress.0, 10.0, "Horror should increase effective stress by 10");
    }

    #[test]
    fn stun_is_replace_stacked() {
        let mut actor = ActorAggregate::new(ActorId(30));
        actor.statuses.attach(stun(2));
        actor.statuses.attach(stun(1)); // should replace

        // With Replace, there should only be 1 stun active
        assert_eq!(actor.statuses.active().len(), 1);
    }
}
