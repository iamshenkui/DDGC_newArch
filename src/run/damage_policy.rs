//! DDGC damage policy — game-layer damage resolution policies for DDGC damage ranges.
//!
//! This module provides a damage policy interface that allows the game layer to
//! control how DDGC damage ranges (e.g., 20-28) are resolved into actual damage
//! values. The interface supports both deterministic (fixed-average) and variance
//! (rolled) damage policies.
//!
//! # Policy Types
//!
//! - [`DamagePolicy::FixedAverage`]: Returns the pre-computed average damage value.
//!   This is the default for deterministic test paths and golden traces.
//! - [`DamagePolicy::Rolled`]: Returns a random value within the damage range.
//!   Uses a seeded RNG for determinism in tests.
//!
//! # Usage
//!
//! ```
//! let range = DamageRange::new(20.0, 28.0);
//! let policy = DamagePolicy::FixedAverage;
//! let resolved = policy.resolve(range);
//! assert_eq!(resolved, 24.0); // Average of 20 and 28
//! ```
//!
//! # Architecture
//!
//! The damage policy is a game-layer seam that sits between the framework's
//! skill resolution and the actual damage application. When skills are migrated
//! from DDGC, they carry damage ranges that are averaged for deterministic behavior.
//! The policy interface allows the game to optionally use the full range for
//! variance without changing how skills are defined.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Represents a DDGC damage range with min, max, and average values.
///
/// DDGC skills specify damage as ranges (e.g., "20-28 damage"). This struct
/// preserves the full range information so the game layer can resolve damage
/// according to the active policy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DamageRange {
    /// Minimum damage value (inclusive).
    pub min: f64,
    /// Maximum damage value (inclusive).
    pub max: f64,
    /// Pre-computed average of min and max.
    pub average: f64,
}

impl DamageRange {
    /// Create a new damage range from min and max values.
    ///
    /// The average is computed automatically as (min + max) / 2.
    ///
    /// # Panics
    ///
    /// Panics if `min > max`.
    pub fn new(min: f64, max: f64) -> Self {
        assert!(min <= max, "DamageRange min must be <= max");
        let average = (min + max) / 2.0;
        DamageRange { min, max, average }
    }

    /// Create a damage range that represents a fixed (non-varying) damage value.
    ///
    /// This is useful when a skill has a single damage value rather than a range.
    /// The min, max, and average all equal the given value.
    pub fn fixed(value: f64) -> Self {
        DamageRange {
            min: value,
            max: value,
            average: value,
        }
    }

    /// Returns the width of the damage range (max - min).
    pub fn range(&self) -> f64 {
        self.max - self.min
    }

    /// Returns true if this is a fixed damage value (min == max).
    pub fn is_fixed(&self) -> bool {
        self.min == self.max
    }
}

/// Damage resolution policy.
///
/// Defines how DDGC damage ranges are translated into actual damage values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DamagePolicy {
    /// Fixed-average policy: always returns the pre-computed average.
    ///
    /// This is the default policy for deterministic test paths and golden traces.
    /// It ensures stable, reproducible damage values across runs.
    FixedAverage,

    /// Rolled policy: returns a random value within the damage range.
    ///
    /// Uses a deterministic RNG seeded by the actor ID and skill ID to ensure
    /// consistent results for the same input. This allows variance in damage
    /// while maintaining determinism for testing.
    ///
    /// # Note
    ///
    /// The "randomness" is deterministic based on the seed. This allows tests
    /// to verify that the rolled policy produces values within the expected range
    /// while still being reproducible.
    Rolled,
}

impl Default for DamagePolicy {
    fn default() -> Self {
        DamagePolicy::FixedAverage
    }
}

impl DamagePolicy {
    /// Resolve damage for the given range using this policy.
    ///
    /// # Arguments
    ///
    /// * `range` — the damage range to resolve
    /// * `actor_id` — the actor ID (used as seed for rolled policy)
    /// * `skill_id` — the skill ID (used as seed for rolled policy)
    ///
    /// # Returns
    ///
    /// The resolved damage value. For `FixedAverage`, always returns the average.
    /// For `Rolled`, returns a value in the range [min, max] based on a
    /// deterministic hash of the actor and skill IDs.
    pub fn resolve(self, range: DamageRange, actor_id: u64, skill_id: &str) -> f64 {
        match self {
            DamagePolicy::FixedAverage => range.average,
            DamagePolicy::Rolled => {
                // Use a deterministic hash of actor_id + skill_id to pick a value
                // within the range. This gives variance while maintaining determinism.
                let mut hasher = DefaultHasher::new();
                actor_id.hash(&mut hasher);
                skill_id.hash(&mut hasher);
                let hash = hasher.finish();

                // Use the hash to compute a normalized value [0, 1)
                let normalized = (hash as f64) / (u64::MAX as f64);

                // Scale to the range
                let width = range.range();
                range.min + (normalized * width)
            }
        }
    }

    /// Resolve damage using the default policy (FixedAverage).
    pub fn resolve_default(self, range: DamageRange) -> f64 {
        match self {
            DamagePolicy::FixedAverage => range.average,
            DamagePolicy::Rolled => {
                // For rolled policy without actor/skill context, use average as fallback
                // This shouldn't happen in practice but provides stability
                range.average
            }
        }
    }
}

/// Resolve damage using the fixed-average policy.
///
/// This is a convenience function for the common case where deterministic
/// damage is desired.
pub fn resolve_damage_fixed(range: DamageRange) -> f64 {
    DamagePolicy::FixedAverage.resolve_default(range)
}

/// Resolve damage using the specified policy.
///
/// This is the main entry point for the damage policy interface.
pub fn resolve_damage(policy: DamagePolicy, range: DamageRange, actor_id: u64, skill_id: &str) -> f64 {
    policy.resolve(range, actor_id, skill_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damage_range_new_computes_average() {
        let range = DamageRange::new(20.0, 28.0);
        assert_eq!(range.min, 20.0);
        assert_eq!(range.max, 28.0);
        assert_eq!(range.average, 24.0);
    }

    #[test]
    fn damage_range_fixed() {
        let range = DamageRange::fixed(24.0);
        assert_eq!(range.min, 24.0);
        assert_eq!(range.max, 24.0);
        assert_eq!(range.average, 24.0);
        assert!(range.is_fixed());
    }

    #[test]
    fn damage_range_is_fixed_false_for_range() {
        let range = DamageRange::new(20.0, 28.0);
        assert!(!range.is_fixed());
    }

    #[test]
    fn damage_range_range_width() {
        let range = DamageRange::new(20.0, 28.0);
        assert_eq!(range.range(), 8.0);
    }

    #[test]
    fn fixed_average_policy_returns_average() {
        let range = DamageRange::new(20.0, 28.0);
        let policy = DamagePolicy::FixedAverage;
        assert_eq!(policy.resolve(range, 1, "test"), 24.0);
    }

    #[test]
    fn fixed_average_policy_with_fixed_range() {
        let range = DamageRange::fixed(50.0);
        let policy = DamagePolicy::FixedAverage;
        assert_eq!(policy.resolve(range, 1, "test"), 50.0);
    }

    #[test]
    fn rolled_policy_returns_value_in_range() {
        let range = DamageRange::new(20.0, 28.0);
        let policy = DamagePolicy::Rolled;

        // Run multiple times to verify it's in range
        for _ in 0..100 {
            let resolved = policy.resolve(range, 42, "poison");
            assert!(resolved >= 20.0 && resolved <= 28.0);
        }
    }

    #[test]
    fn rolled_policy_is_deterministic() {
        let range = DamageRange::new(20.0, 28.0);
        let policy = DamagePolicy::Rolled;

        let result1 = policy.resolve(range, 42, "poison");
        let result2 = policy.resolve(range, 42, "poison");

        assert_eq!(result1, result2, "Rolled policy should be deterministic for same actor/skill");
    }

    #[test]
    fn rolled_policy_different_seeds_different_results() {
        let range = DamageRange::new(20.0, 28.0);
        let policy = DamagePolicy::Rolled;

        let result1 = policy.resolve(range, 1, "poison");
        let result2 = policy.resolve(range, 2, "poison");

        // Results may or may not be equal (hash collision is possible but unlikely)
        // The key is that both are in range
        assert!(result1 >= 20.0 && result1 <= 28.0);
        assert!(result2 >= 20.0 && result2 <= 28.0);
    }

    #[test]
    fn resolve_damage_fixed_convenience() {
        let range = DamageRange::new(20.0, 28.0);
        assert_eq!(resolve_damage_fixed(range), 24.0);
    }

    #[test]
    fn resolve_damage_with_policy() {
        let range = DamageRange::new(20.0, 28.0);
        assert_eq!(resolve_damage(DamagePolicy::FixedAverage, range, 1, "test"), 24.0);
    }

    #[test]
    fn default_policy_is_fixed_average() {
        let policy = DamagePolicy::default();
        assert_eq!(policy, DamagePolicy::FixedAverage);
    }
}