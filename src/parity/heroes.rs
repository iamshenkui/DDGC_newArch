//! Hero semantic parity fixtures — expectations for Crusader and Vestal archetypes.
//!
//! These fixtures capture the original DDGC hero role identity, position
//! preferences, resource semantics, and skill access constraints. Parity
//! tests use these fixtures to verify the migration preserves hero identity
//! rather than flattening heroes into interchangeable actor templates.

use framework_combat::encounter::CombatSide;

/// Parity expectations for a single hero archetype.
#[derive(Debug, Clone)]
pub struct HeroExpectation {
    pub name: &'static str,
    pub side: CombatSide,
    pub min_health: f64,
    pub max_health: f64,
    pub min_attack: f64,
    pub max_attack: f64,
    pub min_defense: f64,
    pub max_defense: f64,
    pub min_speed: f64,
    pub max_speed: f64,
    pub initial_stress: f64,
    pub max_stress: f64,
    pub accessible_skills: &'static [&'static str],
}

/// Fixture bundle containing parity expectations for all hero archetypes.
pub struct HeroParityFixture {
    pub crusader: HeroExpectation,
    pub vestal: HeroExpectation,
}

impl HeroParityFixture {
    pub fn new() -> Self {
        HeroParityFixture {
            crusader: HeroExpectation {
                name: "Crusader",
                side: CombatSide::Ally,
                min_health: 25.0,
                max_health: f64::MAX,
                min_attack: 0.0,
                max_attack: f64::MAX,
                min_defense: 3.0,
                max_defense: f64::MAX,
                min_speed: 0.0,
                max_speed: 6.0,
                initial_stress: 0.0,
                max_stress: 200.0,
                accessible_skills: &["crusading_strike", "holy_lance"],
            },
            vestal: HeroExpectation {
                name: "Vestal",
                side: CombatSide::Ally,
                min_health: 0.0,
                max_health: 30.0,
                min_attack: 0.0,
                max_attack: 10.0,
                min_defense: 0.0,
                max_defense: 0.0,
                min_speed: 6.0,
                max_speed: f64::MAX,
                initial_stress: 0.0,
                max_stress: 200.0,
                accessible_skills: &["divine_grace"],
            },
        }
    }
}

impl Default for HeroParityFixture {
    fn default() -> Self {
        Self::new()
    }
}
