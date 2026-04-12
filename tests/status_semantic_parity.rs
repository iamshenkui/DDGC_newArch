//! Status semantic parity integration tests.
//!
//! Verifies that migrated statuses preserve their stacking rules, tick timing,
//! reactive semantics, and resource interactions — not just their modifier values.

use game_ddgc_headless::content::statuses;
use game_ddgc_headless::parity::StatusParityFixture;

use framework_rules::actor::{ActorAggregate, ActorId};
use framework_rules::attributes::{AttributeKey, AttributeValue};
use framework_rules::statuses::StackRule;

/// Verifies status stack rules are preserved:
/// - bleed uses Stack{max:3} and 3 stacks reduce effective HP by 3x tick damage
/// - stun uses Replace and re-apply replaces
/// - riposte uses Replace
/// - horror uses Stack{max:3} and 3 stacks increase effective stress by 3x tick value
#[test]
fn status_stack_rules_are_preserved() {
    let fixture = StatusParityFixture::new();

    // bleed: Stack{max:3}
    let bleed_exp = &fixture.bleed;
    assert!(matches!(bleed_exp.stack_rule, StackRule::Stack { max: 3 }),
        "bleed stack rule should be Stack {{ max: 3 }}");

    // 3 bleed stacks reduce effective HP by 3x tick damage
    let mut actor = ActorAggregate::new(ActorId(1));
    actor.set_base(AttributeKey::new("health"), AttributeValue(100.0));
    actor.statuses.attach(statuses::bleed(5.0, 3));
    actor.statuses.attach(statuses::bleed(5.0, 3));
    actor.statuses.attach(statuses::bleed(5.0, 3));
    let health = actor.effective_attribute(&AttributeKey::new("health"));
    assert_eq!(health.0, 85.0, "3 bleed stacks should reduce effective HP by 15 (3 x 5)");

    // stun: Replace — re-apply replaces
    let stun_exp = &fixture.stun;
    assert!(matches!(stun_exp.stack_rule, StackRule::Replace),
        "stun stack rule should be Replace");
    let mut actor2 = ActorAggregate::new(ActorId(2));
    actor2.statuses.attach(statuses::stun(2));
    actor2.statuses.attach(statuses::stun(1)); // should replace
    assert_eq!(actor2.statuses.active().len(), 1, "stun Replace should keep only 1 instance");

    // riposte: Replace
    let riposte_exp = &fixture.riposte;
    assert!(matches!(riposte_exp.stack_rule, StackRule::Replace),
        "riposte stack rule should be Replace");

    // horror: Stack{max:3} and 3 stacks increase effective stress by 3x tick value
    let horror_exp = &fixture.horror;
    assert!(matches!(horror_exp.stack_rule, StackRule::Stack { max: 3 }),
        "horror stack rule should be Stack {{ max: 3 }}");
    let mut actor3 = ActorAggregate::new(ActorId(3));
    actor3.set_base(AttributeKey::new("stress"), AttributeValue(0.0));
    actor3.statuses.attach(statuses::horror(10.0, 3));
    actor3.statuses.attach(statuses::horror(10.0, 3));
    actor3.statuses.attach(statuses::horror(10.0, 3));
    let stress = actor3.effective_attribute(&AttributeKey::new("stress"));
    assert_eq!(stress.0, 30.0, "3 horror stacks should increase effective stress by 30 (3 x 10)");
}

/// Verifies status tick timing is preserved:
/// - bleed(5,3) on actor with 100HP — after tick() status is still active, not in expired list;
///   after 3 ticks, bleed expires and is returned by tick()
/// - stun(1) expires after exactly 1 tick
#[test]
fn status_tick_timing_is_preserved() {
    // bleed(5,3): still active after 1 tick, expires after 3 ticks
    let mut actor = ActorAggregate::new(ActorId(10));
    actor.set_base(AttributeKey::new("health"), AttributeValue(100.0));
    actor.statuses.attach(statuses::bleed(5.0, 3));

    // Tick 1: still active
    let expired = actor.statuses.tick();
    assert!(expired.is_empty(), "bleed should not expire after 1 tick");
    assert!(actor.statuses.active().values().any(|s| s.kind.0 == "bleed"),
        "bleed should still be active after 1 tick");

    // Tick 2: still active
    let expired = actor.statuses.tick();
    assert!(expired.is_empty(), "bleed should not expire after 2 ticks");

    // Tick 3: expires
    let expired = actor.statuses.tick();
    assert_eq!(expired.len(), 1, "bleed should expire after 3 ticks");
    assert_eq!(expired[0].kind.0, "bleed");

    // stun(1): expires after exactly 1 tick
    let mut actor2 = ActorAggregate::new(ActorId(11));
    actor2.statuses.attach(statuses::stun(1));

    let expired = actor2.statuses.tick();
    assert_eq!(expired.len(), 1, "stun should expire after 1 tick");
    assert_eq!(expired[0].kind.0, "stun");
    assert!(actor2.statuses.active().is_empty(), "no statuses should remain after stun expires");
}

/// Verifies reactive status semantics are preserved:
/// - riposte has kind=="riposte", modifiers.is_empty(), StackRule::Replace
/// - test verifies marker is detectable via statuses.active() iteration
#[test]
fn reactive_status_semantics_are_preserved() {
    let fixture = StatusParityFixture::new();
    let riposte_exp = &fixture.riposte;

    // Structural expectations
    assert_eq!(riposte_exp.kind, "riposte");
    assert!(!riposte_exp.has_modifiers, "riposte should have no modifiers");
    assert!(matches!(riposte_exp.stack_rule, StackRule::Replace));

    // Verify marker is detectable via statuses.active() iteration
    let mut actor = ActorAggregate::new(ActorId(20));
    actor.statuses.attach(statuses::riposte(2));

    let found = actor.statuses.active().values().any(|s| {
        s.kind.0 == "riposte" && s.modifiers.is_empty()
    });
    assert!(found, "riposte marker should be detectable via active() iteration");
}

/// Verifies status interacts with resources as expected:
/// - bleed(5,3) reduces effective_attribute(ATTR_HEALTH) by 5.0 per stack
/// - horror(10,3) increases effective_attribute(ATTR_STRESS) by 10.0 per stack
/// - Both active simultaneously on same actor modifies both attributes
#[test]
fn status_interacts_with_resources_as_expected() {
    let health_key = AttributeKey::new("health");
    let stress_key = AttributeKey::new("stress");

    // bleed(5,3) reduces effective_attribute(ATTR_HEALTH) by 5.0 per stack
    let mut actor = ActorAggregate::new(ActorId(30));
    actor.set_base(health_key.clone(), AttributeValue(100.0));
    actor.set_base(stress_key.clone(), AttributeValue(0.0));
    actor.statuses.attach(statuses::bleed(5.0, 3));
    let health = actor.effective_attribute(&health_key);
    assert_eq!(health.0, 95.0, "1 bleed stack should reduce effective health by 5.0");

    // horror(10,3) increases effective_attribute(ATTR_STRESS) by 10.0 per stack
    actor.statuses.attach(statuses::horror(10.0, 3));
    let stress = actor.effective_attribute(&stress_key);
    assert_eq!(stress.0, 10.0, "1 horror stack should increase effective stress by 10.0");

    // Both active simultaneously modifies both attributes
    let health = actor.effective_attribute(&health_key);
    let stress = actor.effective_attribute(&stress_key);
    assert_eq!(health.0, 95.0, "bleed should still reduce health when horror is also active");
    assert_eq!(stress.0, 10.0, "horror should still increase stress when bleed is also active");
}
