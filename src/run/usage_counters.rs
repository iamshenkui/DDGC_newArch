//! Skill usage counter state for DDGC per-turn and per-battle usage limits.
//!
//! DDGC skills can declare `LimitPerTurn` or `LimitPerBattle` usage constraints
//! that are separate from the framework's cooldown mechanism. This module provides
//! game-layer state to track and enforce those limits.
//!
//! ## Design
//!
//! - Counters are keyed by `(ActorId, SkillId, UsageScope)` — the same skill
//!   used by different actors has independent counters
//! - Turn-scoped counters reset at actor turn boundaries
//! - Battle-scoped counters persist for the full encounter
//! - Counters are isolated from cooldown logic (cooldown is a framework concern;
//!   usage limits are a DDGC game-layer concern)
//!
//! ## Usage
//!
//! ```ignore
//! let mut counters = SkillUsageCounters::new();
//!
//! // Record a skill usage
//! counters.record_usage(ActorId(1), SkillId::new("fireball"), UsageScope::Turn);
//!
//! // Check if a skill can still be used
//! if counters.can_use(ActorId(1), SkillId::new("fireball"), 3, UsageScope::Turn) {
//!     // skill is within limits
//! }
//!
//! // Reset turn counters at turn boundary
//! counters.reset_turn_scope(ActorId(1));
//! ```

use std::collections::HashMap;

use framework_combat::skills::SkillId;
use framework_rules::actor::ActorId;

/// Scope for skill usage tracking — determines when a counter resets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UsageScope {
    /// Counter resets at actor turn boundaries.
    Turn,
    /// Counter persists for the full battle/encounter.
    Battle,
}

/// A usage limit declaration for a skill.
///
/// Specifies the maximum number of times a skill can be used within a given scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UsageLimit {
    pub scope: UsageScope,
    pub max_uses: u32,
}

impl UsageLimit {
    /// Create a new per-turn usage limit.
    pub fn per_turn(max_uses: u32) -> Self {
        UsageLimit {
            scope: UsageScope::Turn,
            max_uses,
        }
    }

    /// Create a new per-battle usage limit.
    pub fn per_battle(max_uses: u32) -> Self {
        UsageLimit {
            scope: UsageScope::Battle,
            max_uses,
        }
    }
}

/// Game-layer state for tracking skill usage counts per actor.
///
/// Tracks usage counts by actor and skill for both turn and battle scopes.
/// Isolated from cooldown logic — cooldown is a framework concern, while
/// per-turn/per-battle limits are a DDGC game-layer concern.
#[derive(Debug, Clone, Default)]
pub struct SkillUsageCounters {
    /// Internal counter storage keyed by (actor, skill, scope).
    counters: HashMap<UsageKey, u32>,
}

/// Key for looking up a specific usage counter.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UsageKey {
    actor: ActorId,
    skill: SkillId,
    scope: UsageScope,
}

impl UsageKey {
    fn new(actor: ActorId, skill: SkillId, scope: UsageScope) -> Self {
        UsageKey { actor, skill, scope }
    }
}

impl SkillUsageCounters {
    /// Create a new empty counter state.
    pub fn new() -> Self {
        SkillUsageCounters {
            counters: HashMap::new(),
        }
    }

    /// Record a single use of a skill by an actor within a scope.
    ///
    /// Increments the counter. Call this when a skill is successfully used,
    /// before checking limits for the next use.
    pub fn record_usage(&mut self, actor: ActorId, skill: SkillId, scope: UsageScope) {
        let key = UsageKey::new(actor, skill, scope);
        *self.counters.entry(key).or_insert(0) += 1;
    }

    /// Get the current usage count for an actor + skill + scope.
    ///
    /// Returns 0 if the skill has not been used yet in this scope.
    pub fn get_usage_count(&self, actor: ActorId, skill: &SkillId, scope: UsageScope) -> u32 {
        let key = UsageKey::new(actor, skill.clone(), scope);
        self.counters.get(&key).copied().unwrap_or(0)
    }

    /// Check if a skill can still be used by an actor within the given limit.
    ///
    /// Returns `true` if usage is within limits (count < max_uses).
    pub fn can_use(&self, actor: ActorId, skill: &SkillId, limit: UsageLimit) -> bool {
        let current = self.get_usage_count(actor, skill, limit.scope);
        current < limit.max_uses
    }

    /// Get the remaining uses available for an actor + skill + limit.
    ///
    /// Returns `max(0, max_uses - current_count)`.
    pub fn remaining_uses(&self, actor: ActorId, skill: &SkillId, limit: UsageLimit) -> u32 {
        let current = self.get_usage_count(actor, skill, limit.scope);
        limit.max_uses.saturating_sub(current)
    }

    /// Reset all turn-scoped counters for a specific actor.
    ///
    /// Called at actor turn boundaries. Does NOT affect battle-scoped counters.
    pub fn reset_turn_scope(&mut self, actor: ActorId) {
        self.counters.retain(|key, _| {
            // Keep counters that are not turn-scoped OR are for different actors
            key.scope != UsageScope::Turn || key.actor != actor
        });
    }

    /// Reset all counters (both turn and battle scoped) for a specific actor.
    ///
    /// Called at battle/encounter end. Removes all counters for the actor.
    pub fn reset_all_scope(&mut self, actor: ActorId) {
        self.counters.retain(|key, _| key.actor != actor);
    }

    /// Reset only battle-scoped counters for a specific actor.
    ///
    /// Called when a new encounter/battle starts for the actor.
    /// Does NOT affect turn-scoped counters.
    pub fn reset_battle_scope(&mut self, actor: ActorId) {
        self.counters.retain(|key, _| {
            // Keep counters that are not battle-scoped OR are for different actors
            key.scope != UsageScope::Battle || key.actor != actor
        });
    }

    /// Returns the number of tracked counter entries.
    ///
    /// Useful for debugging and testing.
    pub fn len(&self) -> usize {
        self.counters.len()
    }

    /// Returns `true` if no counters are being tracked.
    pub fn is_empty(&self) -> bool {
        self.counters.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fireball() -> SkillId {
        SkillId::new("fireball")
    }

    fn heal() -> SkillId {
        SkillId::new("heal")
    }

    // ── Basic counter tests ─────────────────────────────────────────────────────

    #[test]
    fn new_counters_is_empty() {
        let counters = SkillUsageCounters::new();
        assert!(counters.is_empty());
        assert_eq!(counters.len(), 0);
    }

    #[test]
    fn record_usage_increments_counter() {
        let mut counters = SkillUsageCounters::new();
        let actor = ActorId(1);

        counters.record_usage(actor, fireball(), UsageScope::Turn);
        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Turn), 1);

        counters.record_usage(actor, fireball(), UsageScope::Turn);
        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Turn), 2);
    }

    #[test]
    fn usage_count_starts_at_zero() {
        let counters = SkillUsageCounters::new();
        assert_eq!(
            counters.get_usage_count(ActorId(99), &fireball(), UsageScope::Turn),
            0
        );
    }

    // ── Scope isolation tests ───────────────────────────────────────────────────

    #[test]
    fn turn_and_battle_counters_are_independent() {
        let mut counters = SkillUsageCounters::new();
        let actor = ActorId(1);

        counters.record_usage(actor, fireball(), UsageScope::Turn);
        counters.record_usage(actor, fireball(), UsageScope::Turn);
        counters.record_usage(actor, fireball(), UsageScope::Battle);

        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Turn), 2);
        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Battle), 1);
    }

    #[test]
    fn reset_turn_scope_does_not_affect_battle_counters() {
        let mut counters = SkillUsageCounters::new();
        let actor = ActorId(1);

        counters.record_usage(actor, fireball(), UsageScope::Turn);
        counters.record_usage(actor, fireball(), UsageScope::Battle);

        counters.reset_turn_scope(actor);

        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Turn), 0);
        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Battle), 1);
    }

    #[test]
    fn reset_turn_scope_only_affects_target_actor() {
        let mut counters = SkillUsageCounters::new();
        let actor1 = ActorId(1);
        let actor2 = ActorId(2);

        counters.record_usage(actor1, fireball(), UsageScope::Turn);
        counters.record_usage(actor2, fireball(), UsageScope::Turn);

        counters.reset_turn_scope(actor1);

        assert_eq!(counters.get_usage_count(actor1, &fireball(), UsageScope::Turn), 0);
        assert_eq!(counters.get_usage_count(actor2, &fireball(), UsageScope::Turn), 1);
    }

    #[test]
    fn reset_all_scope_removes_all_counters_for_actor() {
        let mut counters = SkillUsageCounters::new();
        let actor1 = ActorId(1);
        let actor2 = ActorId(2);

        counters.record_usage(actor1, fireball(), UsageScope::Turn);
        counters.record_usage(actor1, fireball(), UsageScope::Battle);
        counters.record_usage(actor2, fireball(), UsageScope::Turn);

        counters.reset_all_scope(actor1);

        assert_eq!(counters.get_usage_count(actor1, &fireball(), UsageScope::Turn), 0);
        assert_eq!(counters.get_usage_count(actor1, &fireball(), UsageScope::Battle), 0);
        assert_eq!(counters.get_usage_count(actor2, &fireball(), UsageScope::Turn), 1);
    }

    // ── Actor isolation tests ─────────────────────────────────────────────────

    #[test]
    fn different_actors_have_independent_counters() {
        let mut counters = SkillUsageCounters::new();
        let actor1 = ActorId(1);
        let actor2 = ActorId(2);

        counters.record_usage(actor1, fireball(), UsageScope::Turn);
        counters.record_usage(actor1, fireball(), UsageScope::Turn);
        counters.record_usage(actor2, fireball(), UsageScope::Turn);

        assert_eq!(counters.get_usage_count(actor1, &fireball(), UsageScope::Turn), 2);
        assert_eq!(counters.get_usage_count(actor2, &fireball(), UsageScope::Turn), 1);
    }

    // ── Skill isolation tests (core acceptance criterion) ───────────────────────

    #[test]
    fn unrelated_skills_do_not_affect_each_others_counters() {
        let mut counters = SkillUsageCounters::new();
        let actor = ActorId(1);

        // Use fireball multiple times
        counters.record_usage(actor, fireball(), UsageScope::Turn);
        counters.record_usage(actor, fireball(), UsageScope::Turn);

        // Use heal — should not affect fireball counter
        counters.record_usage(actor, heal(), UsageScope::Turn);

        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Turn), 2);
        assert_eq!(counters.get_usage_count(actor, &heal(), UsageScope::Turn), 1);
    }

    #[test]
    fn skill_isolation_across_both_scopes() {
        let mut counters = SkillUsageCounters::new();
        let actor = ActorId(1);

        // Fireball usage in turn scope
        counters.record_usage(actor, fireball(), UsageScope::Turn);

        // Fireball usage in battle scope
        counters.record_usage(actor, fireball(), UsageScope::Battle);

        // Heal usage — should not affect fireball counters
        counters.record_usage(actor, heal(), UsageScope::Turn);
        counters.record_usage(actor, heal(), UsageScope::Battle);

        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Turn), 1);
        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Battle), 1);
        assert_eq!(counters.get_usage_count(actor, &heal(), UsageScope::Turn), 1);
        assert_eq!(counters.get_usage_count(actor, &heal(), UsageScope::Battle), 1);
    }

    #[test]
    fn skill_isolation_multiple_actors_multiple_skills() {
        let mut counters = SkillUsageCounters::new();
        let actor1 = ActorId(1);
        let actor2 = ActorId(2);

        // Actor 1 uses fireball
        counters.record_usage(actor1, fireball(), UsageScope::Turn);
        counters.record_usage(actor1, fireball(), UsageScope::Turn);

        // Actor 2 uses heal
        counters.record_usage(actor2, heal(), UsageScope::Turn);

        // Actor 1 uses heal
        counters.record_usage(actor1, heal(), UsageScope::Turn);

        // Verify isolation: each (actor, skill) pair is independent
        assert_eq!(counters.get_usage_count(actor1, &fireball(), UsageScope::Turn), 2);
        assert_eq!(counters.get_usage_count(actor1, &heal(), UsageScope::Turn), 1);
        assert_eq!(counters.get_usage_count(actor2, &heal(), UsageScope::Turn), 1);
        assert_eq!(counters.get_usage_count(actor2, &fireball(), UsageScope::Turn), 0);
    }

    // ── Limit checking tests ───────────────────────────────────────────────────

    #[test]
    fn can_use_returns_true_when_under_limit() {
        let counters = SkillUsageCounters::new();
        let actor = ActorId(1);

        // No usage yet — should be able to use
        assert!(counters.can_use(actor, &fireball(), UsageLimit::per_turn(3)));
    }

    #[test]
    fn can_use_returns_true_at_exact_limit() {
        let mut counters = SkillUsageCounters::new();
        let actor = ActorId(1);

        // Record 3 uses (limit is 3)
        counters.record_usage(actor, fireball(), UsageScope::Turn);
        counters.record_usage(actor, fireball(), UsageScope::Turn);
        counters.record_usage(actor, fireball(), UsageScope::Turn);

        // At exact limit — one more use should NOT be allowed
        assert!(!counters.can_use(actor, &fireball(), UsageLimit::per_turn(3)));
    }

    #[test]
    fn can_use_returns_false_when_over_limit() {
        let mut counters = SkillUsageCounters::new();
        let actor = ActorId(1);

        // Record 4 uses (limit is 3)
        for _ in 0..4 {
            counters.record_usage(actor, fireball(), UsageScope::Turn);
        }

        assert!(!counters.can_use(actor, &fireball(), UsageLimit::per_turn(3)));
    }

    #[test]
    fn can_use_respects_scope() {
        let mut counters = SkillUsageCounters::new();
        let actor = ActorId(1);

        // Use fireball 3 times in turn scope
        counters.record_usage(actor, fireball(), UsageScope::Turn);
        counters.record_usage(actor, fireball(), UsageScope::Turn);
        counters.record_usage(actor, fireball(), UsageScope::Turn);

        // Turn scope is at limit
        assert!(!counters.can_use(actor, &fireball(), UsageLimit::per_turn(3)));

        // Battle scope should still be usable
        assert!(counters.can_use(actor, &fireball(), UsageLimit::per_battle(3)));
    }

    // ── Remaining uses tests ────────────────────────────────────────────────────

    #[test]
    fn remaining_uses_calculation() {
        let mut counters = SkillUsageCounters::new();
        let actor = ActorId(1);

        counters.record_usage(actor, fireball(), UsageScope::Turn);
        counters.record_usage(actor, fireball(), UsageScope::Turn);

        assert_eq!(counters.remaining_uses(actor, &fireball(), UsageLimit::per_turn(3)), 1);
        assert_eq!(counters.remaining_uses(actor, &fireball(), UsageLimit::per_battle(3)), 3);
    }

    #[test]
    fn remaining_uses_at_zero() {
        let mut counters = SkillUsageCounters::new();
        let actor = ActorId(1);

        for _ in 0..3 {
            counters.record_usage(actor, fireball(), UsageScope::Turn);
        }

        assert_eq!(counters.remaining_uses(actor, &fireball(), UsageLimit::per_turn(3)), 0);
    }

    #[test]
    fn remaining_uses_never_negative() {
        let mut counters = SkillUsageCounters::new();
        let actor = ActorId(1);

        // Record 5 uses against a limit of 3
        for _ in 0..5 {
            counters.record_usage(actor, fireball(), UsageScope::Turn);
        }

        // remaining_uses should not go negative
        assert_eq!(counters.remaining_uses(actor, &fireball(), UsageLimit::per_turn(3)), 0);
    }

    // ── UsageLimit construction tests ──────────────────────────────────────────

    #[test]
    fn usage_limit_per_turn() {
        let limit = UsageLimit::per_turn(2);
        assert_eq!(limit.scope, UsageScope::Turn);
        assert_eq!(limit.max_uses, 2);
    }

    #[test]
    fn usage_limit_per_battle() {
        let limit = UsageLimit::per_battle(1);
        assert_eq!(limit.scope, UsageScope::Battle);
        assert_eq!(limit.max_uses, 1);
    }

    // ── Counter persistence tests ───────────────────────────────────────────────

    #[test]
    fn counters_persist_until_reset() {
        let mut counters = SkillUsageCounters::new();
        let actor = ActorId(1);

        // Record multiple uses
        counters.record_usage(actor, fireball(), UsageScope::Turn);
        counters.record_usage(actor, fireball(), UsageScope::Turn);

        // Verify count persists across multiple reads
        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Turn), 2);
        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Turn), 2);

        // Only reset clears it
        counters.reset_turn_scope(actor);
        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Turn), 0);
    }

    #[test]
    fn len_reflects_counter_count() {
        let mut counters = SkillUsageCounters::new();
        let actor1 = ActorId(1);
        let actor2 = ActorId(2);

        assert_eq!(counters.len(), 0);

        counters.record_usage(actor1, fireball(), UsageScope::Turn);
        assert_eq!(counters.len(), 1);

        counters.record_usage(actor1, heal(), UsageScope::Turn);
        assert_eq!(counters.len(), 2);

        counters.record_usage(actor2, fireball(), UsageScope::Turn);
        assert_eq!(counters.len(), 3);

        // Same actor + same scope + different skill = new counter
        counters.record_usage(actor1, fireball(), UsageScope::Battle);
        assert_eq!(counters.len(), 4);
    }

    // ── Battle scope reset tests (US-512) ─────────────────────────────────────────

    #[test]
    fn reset_battle_scope_clears_battle_counters() {
        let mut counters = SkillUsageCounters::new();
        let actor = ActorId(1);

        counters.record_usage(actor, fireball(), UsageScope::Battle);
        counters.record_usage(actor, fireball(), UsageScope::Battle);
        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Battle), 2);

        counters.reset_battle_scope(actor);
        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Battle), 0);
    }

    #[test]
    fn reset_battle_scope_does_not_affect_turn_counters() {
        let mut counters = SkillUsageCounters::new();
        let actor = ActorId(1);

        counters.record_usage(actor, fireball(), UsageScope::Turn);
        counters.record_usage(actor, fireball(), UsageScope::Battle);

        counters.reset_battle_scope(actor);

        // Turn counter should be unaffected
        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Turn), 1);
        // Battle counter should be cleared
        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Battle), 0);
    }

    #[test]
    fn reset_battle_scope_only_affects_target_actor() {
        let mut counters = SkillUsageCounters::new();
        let actor1 = ActorId(1);
        let actor2 = ActorId(2);

        counters.record_usage(actor1, fireball(), UsageScope::Battle);
        counters.record_usage(actor2, fireball(), UsageScope::Battle);

        counters.reset_battle_scope(actor1);

        // Actor1's battle counter should be cleared
        assert_eq!(counters.get_usage_count(actor1, &fireball(), UsageScope::Battle), 0);
        // Actor2's battle counter should be unaffected
        assert_eq!(counters.get_usage_count(actor2, &fireball(), UsageScope::Battle), 1);
    }

    #[test]
    fn reset_battle_scope_allows_new_encounter_with_fresh_counters() {
        let mut counters = SkillUsageCounters::new();
        let actor = ActorId(1);

        // First encounter: use skill multiple times
        counters.record_usage(actor, fireball(), UsageScope::Battle);
        counters.record_usage(actor, fireball(), UsageScope::Battle);
        counters.record_usage(actor, fireball(), UsageScope::Battle);
        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Battle), 3);

        // New encounter starts - reset battle scope
        counters.reset_battle_scope(actor);

        // Battle counter should be fresh (0)
        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Battle), 0);

        // New encounter uses can proceed
        counters.record_usage(actor, fireball(), UsageScope::Battle);
        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Battle), 1);
    }

    #[test]
    fn reset_all_scope_includes_battle_scope() {
        let mut counters = SkillUsageCounters::new();
        let actor = ActorId(1);

        counters.record_usage(actor, fireball(), UsageScope::Turn);
        counters.record_usage(actor, fireball(), UsageScope::Battle);

        counters.reset_all_scope(actor);

        // Both turn and battle should be cleared
        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Turn), 0);
        assert_eq!(counters.get_usage_count(actor, &fireball(), UsageScope::Battle), 0);
    }

    // ── Per-turn enforcement tests (US-513) ──────────────────────────────────────

    #[test]
    fn per_turn_limit_allows_up_to_limit() {
        let mut counters = SkillUsageCounters::new();
        let actor = ActorId(1);
        let skill = SkillId::new("direct_hit_1");
        let limit = UsageLimit::per_turn(2);

        // First use - should be allowed
        assert!(counters.can_use(actor, &skill, limit), "First use should be allowed");
        counters.record_usage(actor, skill.clone(), UsageScope::Turn);

        // Second use - should be allowed (at limit)
        assert!(counters.can_use(actor, &skill, limit), "Second use should be allowed");
        counters.record_usage(actor, skill.clone(), UsageScope::Turn);

        // Third use - should be blocked (over limit)
        assert!(!counters.can_use(actor, &skill, limit), "Third use should be blocked");
    }

    #[test]
    fn per_turn_limit_resets_after_turn_boundary() {
        let mut counters = SkillUsageCounters::new();
        let actor = ActorId(1);
        let skill = SkillId::new("direct_hit_1");
        let limit = UsageLimit::per_turn(2);

        // Use skill up to limit
        counters.record_usage(actor, skill.clone(), UsageScope::Turn);
        counters.record_usage(actor, skill.clone(), UsageScope::Turn);
        assert!(!counters.can_use(actor, &skill, limit), "Should be at limit");

        // Simulate turn boundary - reset turn scope
        counters.reset_turn_scope(actor);

        // After reset, should be able to use again
        assert!(counters.can_use(actor, &skill, limit), "Should be able to use after turn reset");
    }

    #[test]
    fn per_turn_limit_does_not_affect_unrelated_skills() {
        let mut counters = SkillUsageCounters::new();
        let actor = ActorId(1);
        let fireball = SkillId::new("fireball");
        let heal = SkillId::new("heal");

        // Use fireball up to its limit
        let fireball_limit = UsageLimit::per_turn(2);
        counters.record_usage(actor, fireball.clone(), UsageScope::Turn);
        counters.record_usage(actor, fireball.clone(), UsageScope::Turn);
        assert!(!counters.can_use(actor, &fireball, fireball_limit), "Fireball at limit");

        // Heal should not be affected
        let heal_limit = UsageLimit::per_turn(3);
        assert!(counters.can_use(actor, &heal, heal_limit), "Heal should not be affected by fireball limit");
    }
}