//! DDGC actor archetypes — migrated from DDGC hero/monster definitions.
//!
//! DDGC uses paired attributes (current/max) and split defense types.
//! All DDGC-specific attribute keys are defined here as game-layer constants.
//! The framework's generic `AttributeKey` system handles them without modification.

use framework_combat::encounter::CombatSide;
use framework_rules::actor::{ActorAggregate, ActorId};
use framework_rules::attributes::{AttributeKey, AttributeValue, ATTR_ATTACK, ATTR_HEALTH, ATTR_SPEED};

// ── DDGC-Specific Attribute Keys ──────────────────────────────────────────

pub const ATTR_MAX_HEALTH: &str = "max_health";
pub const ATTR_DEFENSE: &str = "defense";
pub const ATTR_STRESS: &str = "stress";
pub const ATTR_MAX_STRESS: &str = "max_stress";
pub const ATTR_CHAOS: &str = "chaos";
pub const ATTR_MAX_CHAOS: &str = "max_chaos";
pub const ATTR_CRIT_CHANCE: &str = "crit_chance";
pub const ATTR_DODGE: &str = "dodge";

// ── Archetype Definition ──────────────────────────────────────────────────

/// A DDGC archetype: a named template for creating combat actors.
///
/// Unlike the consumer example's simple archetype, DDGC archetypes carry
/// paired attributes (HP current + max), stress/chaos gauges, and
/// crit/dodge ratings. The `create_actor` factory sets all of these
/// as base attributes on the produced `ActorAggregate`.
#[derive(Debug, Clone)]
pub struct Archetype {
    pub name: ArchetypeName,
    pub side: CombatSide,
    pub health: f64,
    pub max_health: f64,
    pub attack: f64,
    pub defense: f64,
    pub speed: f64,
    pub stress: f64,
    pub max_stress: f64,
    pub crit_chance: f64,
    pub dodge: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchetypeName(pub String);

impl ArchetypeName {
    pub fn new(name: impl Into<String>) -> Self {
        ArchetypeName(name.into())
    }
}

impl Archetype {
    /// Build a fully-initialized `ActorAggregate` from this archetype.
    ///
    /// Sets all DDGC attribute keys as base values using only public
    /// framework APIs. No framework crates contain DDGC constants.
    pub fn create_actor(&self, id: ActorId) -> ActorAggregate {
        let mut actor = ActorAggregate::new(id);

        // Standard framework attributes
        actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(self.health));
        actor.set_base(AttributeKey::new(ATTR_ATTACK), AttributeValue(self.attack));
        actor.set_base(AttributeKey::new(ATTR_DEFENSE), AttributeValue(self.defense));
        actor.set_base(AttributeKey::new(ATTR_SPEED), AttributeValue(self.speed));

        // DDGC paired attributes
        actor.set_base(AttributeKey::new(ATTR_MAX_HEALTH), AttributeValue(self.max_health));
        actor.set_base(AttributeKey::new(ATTR_STRESS), AttributeValue(self.stress));
        actor.set_base(AttributeKey::new(ATTR_MAX_STRESS), AttributeValue(self.max_stress));
        actor.set_base(AttributeKey::new(ATTR_CRIT_CHANCE), AttributeValue(self.crit_chance));
        actor.set_base(AttributeKey::new(ATTR_DODGE), AttributeValue(self.dodge));

        actor
    }
}

// ── Ally Archetypes (Player Squad Slice) ──────────────────────────────────

/// Crusader — frontline holy warrior.
///
/// DDGC reference: high HP, moderate attack, low speed, rank 1–2 melee.
pub fn crusader() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Crusader"),
        side: CombatSide::Ally,
        health: 33.0,
        max_health: 33.0,
        attack: 10.0,
        defense: 5.0,
        speed: 4.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.05,
        dodge: 0.05,
    }
}

/// Vestal — backline healer and support.
///
/// DDGC reference: moderate HP, low attack, high speed, rank 3–4 support.
pub fn vestal() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Vestal"),
        side: CombatSide::Ally,
        health: 24.0,
        max_health: 24.0,
        attack: 6.0,
        defense: 0.0,
        speed: 8.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.03,
        dodge: 0.10,
    }
}

// ── Enemy Archetypes ──────────────────────────────────────────────────────

/// Bone Soldier — basic undead foot soldier.
///
/// DDGC reference: low HP, moderate attack, fast but fragile.
pub fn bone_soldier() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Bone Soldier"),
        side: CombatSide::Enemy,
        health: 18.0,
        max_health: 18.0,
        attack: 8.0,
        defense: 2.0,
        speed: 7.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.0,
        dodge: 0.10,
    }
}

/// Necromancer — backline boss that summons undead.
///
/// DDGC reference: moderate HP, high attack from spells, slow.
pub fn necromancer() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Necromancer"),
        side: CombatSide::Enemy,
        health: 45.0,
        max_health: 45.0,
        attack: 12.0,
        defense: 1.0,
        speed: 3.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.0,
        dodge: 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrated_actor_pack_builds_valid_actors() {
        let pack = [
            crusader(),
            vestal(),
            bone_soldier(),
            necromancer(),
        ];

        for (i, archetype) in pack.iter().enumerate() {
            let actor = archetype.create_actor(ActorId(i as u64));

            // Every actor must have core DDGC attributes
            let health = actor.effective_attribute(&AttributeKey::new(ATTR_HEALTH));
            assert!(health.0 > 0.0, "{} has non-positive health", archetype.name.0);

            let max_health = actor.effective_attribute(&AttributeKey::new(ATTR_MAX_HEALTH));
            assert!(max_health.0 > 0.0, "{} has non-positive max_health", archetype.name.0);

            let attack = actor.effective_attribute(&AttributeKey::new(ATTR_ATTACK));
            assert!(attack.0 >= 0.0, "{} has negative attack", archetype.name.0);

            let speed = actor.effective_attribute(&AttributeKey::new(ATTR_SPEED));
            assert!(speed.0 > 0.0, "{} has non-positive speed", archetype.name.0);

            // Paired attributes: current <= max
            assert!(
                health.0 <= max_health.0,
                "{} current health {} exceeds max {}",
                archetype.name.0, health.0, max_health.0
            );

            // Stress starts at 0
            let stress = actor.effective_attribute(&AttributeKey::new(ATTR_STRESS));
            assert_eq!(stress.0, 0.0, "{} starts with non-zero stress", archetype.name.0);
        }
    }

    #[test]
    fn crusader_is_frontline_ally() {
        let arch = crusader();
        assert_eq!(arch.side, CombatSide::Ally);
        assert!(arch.health > 25.0, "Crusader should be tanky");
        assert!(arch.speed < 6.0, "Crusader should be slow");
    }

    #[test]
    fn vestal_is_backline_support() {
        let arch = vestal();
        assert_eq!(arch.side, CombatSide::Ally);
        assert!(arch.speed > 6.0, "Vestal should be fast");
        assert!(arch.dodge > 0.05, "Vestal should have decent dodge");
    }

    #[test]
    fn bone_soldier_is_fragile_enemy() {
        let arch = bone_soldier();
        assert_eq!(arch.side, CombatSide::Enemy);
        assert!(arch.health < 25.0, "Bone Soldier should be fragile");
    }

    #[test]
    fn necromancer_is_boss_tier() {
        let arch = necromancer();
        assert_eq!(arch.side, CombatSide::Enemy);
        assert!(arch.attack > 10.0, "Necromancer should hit hard");
    }
}
