//! Monster semantic parity fixtures — expectations for Bone Soldier and Necromancer archetypes.
//!
//! These fixtures capture the original DDGC monster threat models, position
//! logic, and behavioral differentiation. Parity tests use these fixtures
//! to verify the migration preserves monster identity rather than flattening
//! them into interchangeable enemy templates.

use framework_combat::encounter::CombatSide;

/// Parity expectations for a single monster archetype.
#[derive(Debug, Clone)]
pub struct MonsterExpectation {
    pub name: &'static str,
    pub side: CombatSide,
    pub min_health: f64,
    pub max_health: f64,
    pub min_attack: f64,
    pub max_attack: f64,
    pub min_speed: f64,
    pub max_speed: f64,
    pub dodge: f64,
}

/// Fixture bundle containing parity expectations for all monster archetypes.
pub struct MonsterParityFixture {
    pub bone_soldier: MonsterExpectation,
    pub necromancer: MonsterExpectation,
}

impl MonsterParityFixture {
    pub fn new() -> Self {
        MonsterParityFixture {
            bone_soldier: MonsterExpectation {
                name: "Bone Soldier",
                side: CombatSide::Enemy,
                min_health: 0.0,
                max_health: 25.0,
                min_attack: 6.0,
                max_attack: 10.0,
                min_speed: 5.0,
                max_speed: f64::MAX,
                dodge: 0.10,
            },
            necromancer: MonsterExpectation {
                name: "Necromancer",
                side: CombatSide::Enemy,
                min_health: 40.0,
                max_health: f64::MAX,
                min_attack: 10.0,
                max_attack: f64::MAX,
                min_speed: 0.0,
                max_speed: 5.0,
                dodge: 0.0,
            },
        }
    }
}

impl Default for MonsterParityFixture {
    fn default() -> Self {
        Self::new()
    }
}
