//! DDGC hit-resolution context — game-layer hit resolution policies for accuracy vs dodge.
//!
//! This module provides a hit-resolution context that allows the game layer to
//! determine whether an attack hits or misses based on attacker accuracy and
//! defender dodge ratings.
//!
//! # Architecture
//!
//! The hit-resolution context is a game-layer seam that sits between the framework's
//! skill resolution and the actual hit determination. When attacks are resolved,
//! the context provides access to accuracy and dodge values so the game can
//! implement DDGC-style hit calculations.

use framework_rules::actor::ActorId;
use framework_rules::attributes::{AttributeKey, AttributeValue};

use crate::content::ContentPack;

/// Context for resolving hits in DDGC-style combat.
///
/// This struct provides access to the data needed to resolve accuracy-vs-dodge
/// checks: attacker accuracy, defender dodge, and supporting encounter state.
///
/// The context is created from an in-progress encounter state and is used
/// by the hit-resolution policy to determine whether an attack hits.
#[derive(Debug, Clone)]
pub struct HitResolutionContext {
    /// The actor attempting the attack.
    pub attacker_id: ActorId,
    /// The actor being attacked.
    pub defender_id: ActorId,
    /// The attacker's accuracy rating.
    pub attacker_accuracy: f64,
    /// The defender's dodge rating.
    pub defender_dodge: f64,
    /// Whether the attacker has a flanking bonus (not yet implemented).
    pub has_flanking_bonus: bool,
    /// Whether the defender is marked (reduces dodge).
    pub defender_is_marked: bool,
}

impl HitResolutionContext {
    /// Create a new hit-resolution context from encounter state.
    ///
    /// # Arguments
    ///
    /// * `attacker_id` — the actor making the attack
    /// * `defender_id` — the actor being attacked
    /// * `actors` — map of all actors in the encounter
    /// * `content_pack` — content pack for looking up actor data
    ///
    /// # Note
    ///
    /// Currently reads accuracy from `accuracy` attribute and dodge from `dodge` attribute.
    /// These are set from DDGC data during actor creation.
    pub fn new(
        attacker_id: ActorId,
        defender_id: ActorId,
        actors: &std::collections::HashMap<ActorId, framework_rules::actor::ActorAggregate>,
        content_pack: &ContentPack,
    ) -> Option<Self> {
        let attacker = actors.get(&attacker_id)?;
        let defender = actors.get(&defender_id)?;

        let attacker_accuracy = attacker
            .effective_attribute(&AttributeKey::new("accuracy"))
            .0;
        let defender_dodge = defender
            .effective_attribute(&AttributeKey::new("dodge"))
            .0;

        // TODO: Check for flanking bonus from formation position
        let has_flanking_bonus = false;

        // TODO: Check if defender has "tagged" status (reduces dodge)
        let defender_is_marked = false;

        Some(HitResolutionContext {
            attacker_id,
            defender_id,
            attacker_accuracy,
            defender_dodge,
            has_flanking_bonus,
            defender_is_marked,
        })
    }

    /// Calculate the effective dodge for this attack.
    ///
    /// Returns the defender's dodge, potentially modified by:
    /// - Flanking bonus to attacker (reduces effective dodge)
    /// - Marked status on defender (reduces effective dodge)
    pub fn effective_dodge(&self) -> f64 {
        let mut effective = self.defender_dodge;

        // Marked targets have reduced dodge
        if self.defender_is_marked {
            // In DDGC, marked targets typically have -50% dodge
            effective *= 0.5;
        }

        // Flanking bonus reduces effective dodge
        if self.has_flanking_bonus {
            // In DDGC, flanking typically reduces dodge by a portion
            effective *= 0.75;
        }

        effective
    }

    /// Determine if this attack hits using DDGC-style accuracy vs dodge.
    ///
    /// DDGC uses a simple comparison: if accuracy > dodge, hit. Otherwise miss.
    /// This is a deterministic calculation, not a roll.
    ///
    /// # Returns
    ///
    /// `true` if the attack hits, `false` if it misses.
    pub fn resolve_hit(&self) -> bool {
        self.attacker_accuracy > self.effective_dodge()
    }
}

/// Hit-resolution policy enum.
///
/// Defines how hits are resolved in DDGC-style combat.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HitPolicy {
    /// Standard DDGC hit resolution: accuracy vs dodge comparison.
    /// Hit if accuracy > effective_dodge.
    AccuracyVsDodge,

    /// Always hits (for testing or certain skill effects).
    AlwaysHit,

    /// Always misses (for testing or certain debuffs).
    AlwaysMiss,
}

impl Default for HitPolicy {
    fn default() -> Self {
        HitPolicy::AccuracyVsDodge
    }
}

impl HitPolicy {
    /// Resolve a hit using this policy.
    ///
    /// # Arguments
    ///
    /// * `ctx` — the hit-resolution context
    ///
    /// # Returns
    ///
    /// `true` if the attack hits, `false` if it misses.
    pub fn resolve(self, ctx: &HitResolutionContext) -> bool {
        match self {
            HitPolicy::AccuracyVsDodge => ctx.resolve_hit(),
            HitPolicy::AlwaysHit => true,
            HitPolicy::AlwaysMiss => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_resolution_context_calculates_effective_dodge() {
        let ctx = HitResolutionContext {
            attacker_id: ActorId(1),
            defender_id: ActorId(2),
            attacker_accuracy: 0.95,
            defender_dodge: 0.10,
            has_flanking_bonus: false,
            defender_is_marked: false,
        };

        assert_eq!(ctx.effective_dodge(), 0.10);
    }

    #[test]
    fn marked_target_has_reduced_effective_dodge() {
        let ctx = HitResolutionContext {
            attacker_id: ActorId(1),
            defender_id: ActorId(2),
            attacker_accuracy: 0.95,
            defender_dodge: 0.10,
            has_flanking_bonus: false,
            defender_is_marked: true, // Marked
        };

        assert_eq!(ctx.effective_dodge(), 0.05); // 50% of 0.10
    }

    #[test]
    fn flanking_reduces_effective_dodge() {
        let ctx = HitResolutionContext {
            attacker_id: ActorId(1),
            defender_id: ActorId(2),
            attacker_accuracy: 0.95,
            defender_dodge: 0.10,
            has_flanking_bonus: true, // Flanking
            defender_is_marked: false,
        };

        let effective = ctx.effective_dodge();
        assert!(
            (effective - 0.075).abs() < 0.0001,
            "Expected ~0.075, got {}",
            effective
        );
    }

    #[test]
    fn accuracy_vs_dodge_hits_when_accuracy_higher() {
        let ctx = HitResolutionContext {
            attacker_id: ActorId(1),
            defender_id: ActorId(2),
            attacker_accuracy: 0.95,
            defender_dodge: 0.10,
            has_flanking_bonus: false,
            defender_is_marked: false,
        };

        assert!(ctx.resolve_hit(), "Should hit when accuracy > dodge");
    }

    #[test]
    fn accuracy_vs_dodge_misses_when_dodge_higher() {
        let ctx = HitResolutionContext {
            attacker_id: ActorId(1),
            defender_id: ActorId(2),
            attacker_accuracy: 0.05,
            defender_dodge: 0.10,
            has_flanking_bonus: false,
            defender_is_marked: false,
        };

        assert!(!ctx.resolve_hit(), "Should miss when accuracy <= dodge");
    }

    #[test]
    fn default_hit_policy_is_accuracy_vs_dodge() {
        let policy = HitPolicy::default();
        assert_eq!(policy, HitPolicy::AccuracyVsDodge);
    }

    #[test]
    fn always_hit_policy_always_hits() {
        let ctx = HitResolutionContext {
            attacker_id: ActorId(1),
            defender_id: ActorId(2),
            attacker_accuracy: 0.0,
            defender_dodge: 1.0,
            has_flanking_bonus: false,
            defender_is_marked: false,
        };

        assert!(HitPolicy::AlwaysHit.resolve(&ctx));
    }

    #[test]
    fn always_miss_policy_always_misses() {
        let ctx = HitResolutionContext {
            attacker_id: ActorId(1),
            defender_id: ActorId(2),
            attacker_accuracy: 1.0,
            defender_dodge: 0.0,
            has_flanking_bonus: false,
            defender_is_marked: false,
        };

        assert!(!HitPolicy::AlwaysMiss.resolve(&ctx));
    }
}