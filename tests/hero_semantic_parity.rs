//! Hero semantic parity integration tests.
//!
//! Verifies that hero archetypes preserve their original role identity,
//! position preference, resource semantics, and skill access constraints
//! rather than being flattened into interchangeable actor templates.

use game_ddgc_headless::content::actors;
use game_ddgc_headless::content::ContentPack;
use game_ddgc_headless::parity::HeroParityFixture;

use framework_combat::encounter::CombatSide;
use framework_combat::skills::SkillId;
use framework_rules::actor::ActorId;
use framework_rules::attributes::{AttributeKey, AttributeValue};

/// Verifies hero archetypes preserve their role identity:
/// - Crusader: side==Ally, HP>25, DEF>3, SPD<6 (tanky frontline)
/// - Vestal: side==Ally, HP<30, ATK<10, SPD>6 (fragile backline)
#[test]
fn hero_role_identity_is_preserved() {
    let fixture = HeroParityFixture::new();

    // Crusader role identity
    let crusader_arch = actors::crusader();
    let crusader_exp = &fixture.crusader;
    assert_eq!(crusader_arch.side, crusader_exp.side, "Crusader side mismatch");
    assert!(
        crusader_arch.health > crusader_exp.min_health,
        "Crusader HP {} should be > {}",
        crusader_arch.health,
        crusader_exp.min_health
    );
    assert!(
        crusader_arch.defense > crusader_exp.min_defense,
        "Crusader DEF {} should be > {}",
        crusader_arch.defense,
        crusader_exp.min_defense
    );
    assert!(
        crusader_arch.speed < crusader_exp.max_speed,
        "Crusader SPD {} should be < {}",
        crusader_arch.speed,
        crusader_exp.max_speed
    );

    // Vestal role identity
    let vestal_arch = actors::vestal();
    let vestal_exp = &fixture.vestal;
    assert_eq!(vestal_arch.side, vestal_exp.side, "Vestal side mismatch");
    assert!(
        vestal_arch.health < vestal_exp.max_health,
        "Vestal HP {} should be < {}",
        vestal_arch.health,
        vestal_exp.max_health
    );
    assert!(
        vestal_arch.attack < vestal_exp.max_attack,
        "Vestal ATK {} should be < {}",
        vestal_arch.attack,
        vestal_exp.max_attack
    );
    assert!(
        vestal_arch.speed > vestal_exp.min_speed,
        "Vestal SPD {} should be > {}",
        vestal_arch.speed,
        vestal_exp.min_speed
    );
}

/// Verifies hero archetypes preserve their position preference:
/// - Crusader: defense>=5, speed<=5 (frontline)
/// - Vestal: defense==0, speed>=8 (backline)
#[test]
fn hero_position_preference_is_preserved() {
    // Crusader: frontline profile
    let crusader_arch = actors::crusader();
    assert!(
        crusader_arch.defense >= 5.0,
        "Crusader defense {} should be >= 5 (frontline)",
        crusader_arch.defense
    );
    assert!(
        crusader_arch.speed <= 5.0,
        "Crusader speed {} should be <= 5 (frontline)",
        crusader_arch.speed
    );

    // Vestal: backline profile
    let vestal_arch = actors::vestal();
    assert_eq!(
        vestal_arch.defense, 0.0,
        "Vestal defense should be 0 (backline)"
    );
    assert!(
        vestal_arch.speed >= 8.0,
        "Vestal speed {} should be >= 8 (backline)",
        vestal_arch.speed
    );
}

/// Verifies hero resource semantics are preserved:
/// - Both heroes have stress==0, max_stress==200 after create_actor()
/// - effective_attribute(ATTR_STRESS) returns 0.0
#[test]
fn hero_resource_semantics_are_preserved() {
    let crusader_actor = actors::crusader().create_actor(ActorId(1));
    let vestal_actor = actors::vestal().create_actor(ActorId(2));

    let stress_key = AttributeKey::new(actors::ATTR_STRESS);
    let max_stress_key = AttributeKey::new(actors::ATTR_MAX_STRESS);

    // Crusader stress
    assert_eq!(
        crusader_actor.effective_attribute(&stress_key),
        AttributeValue(0.0),
        "Crusader initial stress should be 0"
    );
    assert_eq!(
        crusader_actor.effective_attribute(&max_stress_key),
        AttributeValue(200.0),
        "Crusader max_stress should be 200"
    );

    // Vestal stress
    assert_eq!(
        vestal_actor.effective_attribute(&stress_key),
        AttributeValue(0.0),
        "Vestal initial stress should be 0"
    );
    assert_eq!(
        vestal_actor.effective_attribute(&max_stress_key),
        AttributeValue(200.0),
        "Vestal max_stress should be 200"
    );
}

/// Verifies hero skill access constraints match original intent:
/// - Crusader can access crusading_strike and holy_lance from ContentPack
/// - Vestal can access divine_grace from ContentPack
/// - Neither hero archetype has side==Enemy
#[test]
fn hero_skill_access_constraints_match_original_intent() {
    let fixture = HeroParityFixture::new();
    let pack = ContentPack::default();

    // Crusader skill access
    for skill_name in fixture.crusader.accessible_skills {
        assert!(
            pack.get_skill(&SkillId::new(*skill_name)).is_some(),
            "Crusader should access skill '{}' from ContentPack",
            skill_name
        );
    }

    // Vestal skill access
    for skill_name in fixture.vestal.accessible_skills {
        assert!(
            pack.get_skill(&SkillId::new(*skill_name)).is_some(),
            "Vestal should access skill '{}' from ContentPack",
            skill_name
        );
    }

    // Neither hero archetype is an enemy
    assert_ne!(
        actors::crusader().side,
        CombatSide::Enemy,
        "Crusader must not be Enemy side"
    );
    assert_ne!(
        actors::vestal().side,
        CombatSide::Enemy,
        "Vestal must not be Enemy side"
    );
}
