//! DDGC game-layer usage limits for skills.
//!
//! DDGC skills can declare `LimitPerTurn` or `LimitPerBattle` usage constraints
//! that are separate from the framework's cooldown mechanism. This module provides
//! the game-layer metadata for which skills have usage limits.
//!
//! The actual counter state and enforcement lives in `usage_counters.rs`.
//! This module only provides the limit declarations.

use framework_combat::skills::SkillId;
use std::collections::HashMap;

use crate::run::usage_counters::UsageLimit;

/// Map of skill IDs to their DDGC usage limits.
///
/// Add entries here for skills that have per-turn or per-battle limits
/// in the DDGC design.
pub fn ddgc_usage_limits() -> HashMap<String, UsageLimit> {
    let mut limits = HashMap::new();

    // Example: direct_hit_1 has a per-turn limit of 2 uses
    // This simulates DDGC skills that are powerful but limited per turn
    limits.insert(
        "direct_hit_1".to_string(),
        UsageLimit::per_turn(2),
    );

    // Add more skill limits here as migrated content reveals them
    // limits.insert("fireball".to_string(), UsageLimit::per_battle(1));

    limits
}

/// Look up the usage limit for a skill, if any.
pub fn get_usage_limit(skill_id: &SkillId) -> Option<UsageLimit> {
    ddgc_usage_limits().get(&skill_id.0).copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run::usage_counters::UsageScope;

    #[test]
    fn direct_hit_has_per_turn_limit() {
        let limit = get_usage_limit(&SkillId::new("direct_hit_1"));
        assert!(limit.is_some());
        let limit = limit.unwrap();
        assert_eq!(limit.scope, UsageScope::Turn);
        assert_eq!(limit.max_uses, 2);
    }

    #[test]
    fn unknown_skill_has_no_limit() {
        let limit = get_usage_limit(&SkillId::new("nonexistent_skill"));
        assert!(limit.is_none());
    }
}