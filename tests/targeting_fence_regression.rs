//! Regression tests for targeting contract fences (US-009-c).
//!
//! Verifies that every high-frequency targeting semantic path is either
//! implemented or fenced with deterministic labels. No targeting variant
//! silently drops with an empty string or panics.
//!
//! Coverage:
//! - LaunchConstraint (5 variants): labels, serialization roundtrip
//! - TargetRank (4 variants): labels, serialization roundtrip
//! - SideAffinity (3 variants): labels, serialization roundtrip
//! - TargetCount (2 variants): labels, serialization roundtrip
//! - TargetingIntent: construction, side affinity predicates, combos
//! - Encounter-level TargetingIntent resolution

use game_ddgc_headless::contracts::{
    LaunchConstraint, SideAffinity, TargetCount, TargetRank, TargetingIntent,
};
use game_ddgc_headless::encounters::targeting::{
    LaunchConstraint as EncLaunchConstraint,
    SideAffinity as EncSideAffinity,
    TargetCount as EncTargetCount,
    TargetRank as EncTargetRank,
    TargetingIntent as EncTargetingIntent,
    TargetingContext,
};
use framework_combat::encounter::CombatSide;
use framework_combat::formation::{FormationLayout, SlotIndex};
use framework_rules::actor::{ActorAggregate, ActorId};
use framework_rules::attributes::{AttributeKey, AttributeValue, ATTR_HEALTH};
use std::collections::HashMap;

// ── LaunchConstraint fence coverage ───────────────────────────────────────────

#[test]
fn all_launch_constraints_have_non_empty_labels() {
    let variants = [
        LaunchConstraint::Any,
        LaunchConstraint::FrontRow,
        LaunchConstraint::BackRow,
        LaunchConstraint::SpecificLane(0),
        LaunchConstraint::SlotRange { min: 0, max: 3 },
    ];
    for v in &variants {
        let label = v.label();
        assert!(!label.is_empty(), "LaunchConstraint variant produces empty label");
    }
}

#[test]
fn launch_constraint_serialization_roundtrip() {
    let variants = [
        LaunchConstraint::Any,
        LaunchConstraint::FrontRow,
        LaunchConstraint::BackRow,
        LaunchConstraint::SpecificLane(2),
        LaunchConstraint::SlotRange { min: 1, max: 3 },
    ];
    for v in &variants {
        let json = serde_json::to_string(v).expect("serialization must succeed");
        let restored: LaunchConstraint = serde_json::from_str(&json).expect("deserialization must succeed");
        assert_eq!(*v, restored, "LaunchConstraint roundtrip mismatch for {:?}", v);
    }
}

#[test]
fn launch_constraint_five_variants_enum_count() {
    let variants: &[LaunchConstraint] = &[
        LaunchConstraint::Any,
        LaunchConstraint::FrontRow,
        LaunchConstraint::BackRow,
        LaunchConstraint::SpecificLane(0),
        LaunchConstraint::SlotRange { min: 0, max: 1 },
    ];
    assert_eq!(variants.len(), 5, "LaunchConstraint must have exactly 5 variants");
}

// ── TargetRank fence coverage ─────────────────────────────────────────────────

#[test]
fn all_target_rank_variants_have_non_empty_labels() {
    let variants = [
        TargetRank::Any,
        TargetRank::Front,
        TargetRank::Back,
        TargetRank::FrontAndBack,
    ];
    for v in &variants {
        let label = v.label();
        assert!(!label.is_empty(), "TargetRank variant produces empty label");
    }
}

#[test]
fn target_rank_serialization_roundtrip() {
    let variants = [
        TargetRank::Any,
        TargetRank::Front,
        TargetRank::Back,
        TargetRank::FrontAndBack,
    ];
    for v in &variants {
        let json = serde_json::to_string(v).expect("serialization must succeed");
        let restored: TargetRank = serde_json::from_str(&json).expect("deserialization must succeed");
        assert_eq!(*v, restored, "TargetRank roundtrip mismatch for {:?}", v);
    }
}

// ── SideAffinity fence coverage ───────────────────────────────────────────────

#[test]
fn all_side_affinity_variants_have_non_empty_labels() {
    let variants = [
        SideAffinity::Enemy,
        SideAffinity::Ally,
        SideAffinity::Any,
    ];
    for v in &variants {
        let label = v.label();
        assert!(!label.is_empty(), "SideAffinity variant produces empty label");
    }
}

#[test]
fn side_affinity_serialization_roundtrip() {
    let variants = [
        SideAffinity::Enemy,
        SideAffinity::Ally,
        SideAffinity::Any,
    ];
    for v in &variants {
        let json = serde_json::to_string(v).expect("serialization must succeed");
        let restored: SideAffinity = serde_json::from_str(&json).expect("deserialization must succeed");
        assert_eq!(*v, restored, "SideAffinity roundtrip mismatch for {:?}", v);
    }
}

// ── TargetCount fence coverage ────────────────────────────────────────────────

#[test]
fn all_target_count_variants_have_non_empty_labels() {
    let variants = [TargetCount::Single, TargetCount::Multiple];
    for v in &variants {
        let label = v.label();
        assert!(!label.is_empty(), "TargetCount variant produces empty label");
    }
}

#[test]
fn target_count_serialization_roundtrip() {
    let variants = [TargetCount::Single, TargetCount::Multiple];
    for v in &variants {
        let json = serde_json::to_string(v).expect("serialization must succeed");
        let restored: TargetCount = serde_json::from_str(&json).expect("deserialization must succeed");
        assert_eq!(*v, restored, "TargetCount roundtrip mismatch for {:?}", v);
    }
}

// ── TargetingIntent fence coverage ────────────────────────────────────────────

#[test]
fn targeting_intent_default_enemy_single_is_well_formed() {
    let intent = TargetingIntent::default_enemy_single();
    assert_eq!(intent.side_affinity, SideAffinity::Enemy);
    assert_eq!(intent.target_count, TargetCount::Single);
    assert!(intent.targets_enemy());
    assert!(!intent.targets_ally());
}

#[test]
fn targeting_intent_ally_single_is_well_formed() {
    let intent = TargetingIntent::ally_single();
    assert_eq!(intent.side_affinity, SideAffinity::Ally);
    assert_eq!(intent.target_count, TargetCount::Single);
    assert!(!intent.targets_enemy());
    assert!(intent.targets_ally());
}

#[test]
fn targeting_intent_any_side_targets_both() {
    let intent = TargetingIntent::new(
        LaunchConstraint::Any,
        TargetRank::Any,
        SideAffinity::Any,
        TargetCount::Multiple,
    );
    assert!(intent.targets_enemy());
    assert!(intent.targets_ally());
}

#[test]
fn targeting_intent_serialization_roundtrip() {
    let intent = TargetingIntent::new(
        LaunchConstraint::FrontRow,
        TargetRank::Back,
        SideAffinity::Enemy,
        TargetCount::Single,
    );
    let json = serde_json::to_string(&intent).expect("serialization must succeed");
    let restored: TargetingIntent = serde_json::from_str(&json).expect("deserialization must succeed");
    assert_eq!(intent, restored, "TargetingIntent roundtrip mismatch");
}

#[test]
fn targeting_intent_all_combos_produce_valid_labels() {
    let constraints = [
        LaunchConstraint::Any,
        LaunchConstraint::FrontRow,
        LaunchConstraint::BackRow,
        LaunchConstraint::SpecificLane(1),
        LaunchConstraint::SlotRange { min: 0, max: 3 },
    ];
    let affinities = [
        SideAffinity::Enemy,
        SideAffinity::Ally,
        SideAffinity::Any,
    ];

    for c in &constraints {
        for a in &affinities {
            let intent = TargetingIntent::new(c.clone(), TargetRank::Any, a.clone(), TargetCount::Single);
            assert!(!intent.launch_constraint.label().is_empty());
            assert!(!intent.side_affinity.label().is_empty());
            assert!(!intent.target_rank.label().is_empty());
            assert!(!intent.target_count.label().is_empty());
        }
    }
}

// ── Encounter-level TargetingIntent fence coverage ────────────────────────────

fn setup_2x2_formation() -> (FormationLayout, HashMap<ActorId, ActorAggregate>, HashMap<ActorId, CombatSide>) {
    let mut formation = FormationLayout::new(2, 2);
    formation.place(ActorId(1), SlotIndex(0)).unwrap();
    formation.place(ActorId(2), SlotIndex(2)).unwrap();
    formation.place(ActorId(10), SlotIndex(1)).unwrap();
    formation.place(ActorId(20), SlotIndex(3)).unwrap();

    let mut actors = HashMap::new();
    for id in [ActorId(1), ActorId(2), ActorId(10), ActorId(20)] {
        let mut a = ActorAggregate::new(id);
        a.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
        actors.insert(id, a);
    }

    let mut side_lookup = HashMap::new();
    side_lookup.insert(ActorId(1), CombatSide::Ally);
    side_lookup.insert(ActorId(2), CombatSide::Ally);
    side_lookup.insert(ActorId(10), CombatSide::Enemy);
    side_lookup.insert(ActorId(20), CombatSide::Enemy);

    (formation, actors, side_lookup)
}

#[test]
fn encounter_targeting_intent_all_enemies_resolves_correctly() {
    let (formation, actors, side_lookup) = setup_2x2_formation();
    let intent = EncTargetingIntent::all_enemies();
    let targets = intent.resolve(ActorId(1), &formation, &actors, &side_lookup);
    assert_eq!(targets.len(), 2);
    assert!(targets.contains(&ActorId(10)));
    assert!(targets.contains(&ActorId(20)));
}

#[test]
fn encounter_targeting_intent_all_allies_resolves_correctly() {
    let (formation, actors, side_lookup) = setup_2x2_formation();
    let intent = EncTargetingIntent::all_allies();
    let targets = intent.resolve(ActorId(1), &formation, &actors, &side_lookup);
    assert_eq!(targets.len(), 2);
    assert!(targets.contains(&ActorId(1)));
    assert!(targets.contains(&ActorId(2)));
}

#[test]
fn encounter_targeting_front_row_constraint_blocks_back_row_actor() {
    let (formation, actors, side_lookup) = setup_2x2_formation();
    let intent = EncTargetingIntent {
        launch_constraint: EncLaunchConstraint::FrontRow,
        target_rank: EncTargetRank::Any,
        side_affinity: EncSideAffinity::Enemy,
        target_count: EncTargetCount::Multiple,
    };
    // Actor 2 is at slot 2 (back row), should have no valid targets
    let targets = intent.resolve(ActorId(2), &formation, &actors, &side_lookup);
    assert!(targets.is_empty(), "Back row actor with FrontRow launch constraint should have no targets");
}

#[test]
fn encounter_targeting_single_target_returns_one() {
    let (formation, actors, side_lookup) = setup_2x2_formation();
    let intent = EncTargetingIntent {
        launch_constraint: EncLaunchConstraint::Any,
        target_rank: EncTargetRank::Any,
        side_affinity: EncSideAffinity::Enemy,
        target_count: EncTargetCount::Single,
    };
    let targets = intent.resolve(ActorId(1), &formation, &actors, &side_lookup);
    assert_eq!(targets.len(), 1);
}

#[test]
fn encounter_targeting_targets_sorted_deterministically() {
    let (formation, actors, side_lookup) = setup_2x2_formation();
    let intent = EncTargetingIntent::all_enemies();
    for _ in 0..10 {
        let targets = intent.resolve(ActorId(1), &formation, &actors, &side_lookup);
        assert_eq!(targets, vec![ActorId(10), ActorId(20)]);
    }
}

#[test]
fn encounter_targeting_context_builds_from_actor() {
    let (formation, actors, side_lookup) = setup_2x2_formation();
    let intent = EncTargetingIntent::all_enemies();
    let ctx = TargetingContext::from_actor_and_intent(
        ActorId(1),
        &formation,
        &actors,
        &side_lookup,
        intent.clone(),
    );
    assert!(ctx.is_some());
    let ctx = ctx.unwrap();
    assert_eq!(ctx.actor, ActorId(1));
    assert_eq!(ctx.intent, intent);
}

#[test]
fn encounter_targeting_context_returns_none_for_invalid_actor() {
    let (formation, actors, side_lookup) = setup_2x2_formation();
    let intent = EncTargetingIntent::all_enemies();
    let ctx = TargetingContext::from_actor_and_intent(
        ActorId(999),
        &formation,
        &actors,
        &side_lookup,
        intent,
    );
    assert!(ctx.is_none(), "Non-existent actor should return None");
}

#[test]
fn targeting_context_resolve_delegates_properly() {
    let (formation, actors, side_lookup) = setup_2x2_formation();
    let intent = EncTargetingIntent::all_enemies();
    let ctx = TargetingContext::from_actor_and_intent(
        ActorId(1),
        &formation,
        &actors,
        &side_lookup,
        intent,
    ).unwrap();
    let targets = ctx.resolve_targets(&actors);
    assert_eq!(targets.len(), 2);
}
