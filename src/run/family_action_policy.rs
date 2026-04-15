//! Family action policy — deterministic skill selection for monster families.
//!
//! Replaces the naive "first registered skill" fallback with authored DDGC
//! family-level action policies grounded in the JsonAI brain data.
//!
//! ## Policy Types
//!
//! - **`FirstSkill`** (default): Uses the first registered skill. This is the
//!   safe fallback for families whose AI behavior is not yet modeled.
//!
//! - **`DeterministicCycle`**: Cycles through skills in a fixed sequence.
//!   Used for families like `lizard` whose AI brain defines a
//!   `last_combat_skill_used_skill` chain: stun → intimidate → stress → repeat.
//!
//! - **`PriorityTable`**: Selects the highest-priority available skill.
//!   Used for families like `gambler` whose AI brain defines extreme weight
//!   differences (summon_mahjong weight 1000 vs others weight 0.25).

use framework_combat::skills::SkillId;
use framework_rules::actor::ActorId;

/// A skill selection policy for a monster family.
///
/// Each policy type encodes a different DDGC AI brain behavior.
/// The policy is resolved at runtime given the current actor's state.
#[derive(Debug, Clone, Default)]
pub enum FamilyActionPolicy {
    /// Default fallback: use the first registered skill.
    /// This is the safe default for families not yet migrated to a real policy.
    #[default]
    FirstSkill,

    /// Cycles through skills in a fixed sequence, restarting after the last.
    /// The sequence is defined as an ordered list of skill IDs.
    /// Transitions: sequence[i] → sequence[(i+1) % len]
    DeterministicCycle {
        /// Ordered skill sequence to cycle through.
        sequence: Vec<SkillId>,
    },

    /// Selects the highest-priority skill from a weight table.
    /// The first skill with the highest weight is selected.
    /// Used for boss families with extreme weight disparities (e.g., gambler).
    PriorityTable {
        /// (skill_id, weight) pairs. Higher weight = higher priority.
        entries: Vec<(SkillId, u32)>,
    },
}

/// Per-actor state tracked during encounter resolution for policy evaluation.
///
/// Some policies (like DeterministicCycle) need to remember the last skill
/// used to compute the next skill.
#[derive(Debug, Clone, Default)]
pub struct ActorActionState {
    /// The last skill ID this actor used, if any.
    pub last_skill_used: Option<SkillId>,
}

/// Look up the next skill for an actor given their current action state.
pub fn select_next_skill(
    policy: &FamilyActionPolicy,
    _actor_id: ActorId,
    state: &ActorActionState,
    all_skills: &[SkillId],
) -> SkillId {
    match policy {
        FamilyActionPolicy::FirstSkill => {
            // Fallback: use first registered skill
            all_skills.first().cloned().unwrap_or_else(|| SkillId::new("normal_attack"))
        }
        FamilyActionPolicy::DeterministicCycle { sequence } => {
            if sequence.is_empty() {
                return all_skills.first().cloned().unwrap_or_else(|| SkillId::new("normal_attack"));
            }

            let last = match &state.last_skill_used {
                Some(s) => s,
                None => {
                    // First turn: use the first skill in the sequence
                    return sequence[0].clone();
                }
            };

            // Find the current position in the cycle
            let current_idx = sequence.iter().position(|s| s == last);

            match current_idx {
                Some(idx) => {
                    // Next skill: cycle to (idx + 1) % len
                    let next_idx = (idx + 1) % sequence.len();
                    sequence[next_idx].clone()
                }
                None => {
                    // Last skill used is not in the cycle — reset to first
                    sequence[0].clone()
                }
            }
        }
        FamilyActionPolicy::PriorityTable { entries } => {
            if entries.is_empty() {
                return all_skills.first().cloned().unwrap_or_else(|| SkillId::new("normal_attack"));
            }

            // Find the max weight
            let max_weight = entries.iter().map(|(_, w)| *w).max().unwrap_or(0);

            // Find the first skill with max weight
            for (skill_id, weight) in entries {
                if *weight >= max_weight {
                    return skill_id.clone();
                }
            }

            entries[0].0.clone()
        }
    }
}

/// Update the actor's action state after a skill is used.
pub fn update_actor_state(
    state: &mut ActorActionState,
    skill_id: SkillId,
) {
    state.last_skill_used = Some(skill_id);
}

/// Get the default action policy for a family based on its family ID.
///
/// This function returns the authored policy for families whose AI behavior
/// has been analyzed from JsonAI.json. Families not listed here use
/// `FamilyActionPolicy::FirstSkill` as the safe default.
pub fn get_default_policy(family_id: &str) -> FamilyActionPolicy {
    match family_id {
        // Lizard: deterministic cycle stun → intimidate → stress → repeat
        // From JsonAI.json: last_combat_skill_used chain with weight 1000
        "lizard" => FamilyActionPolicy::DeterministicCycle {
            sequence: vec![
                SkillId::new("stun"),
                SkillId::new("intimidate"),
                SkillId::new("stress"),
            ],
        },

        // Gambler: priority table with summon_mahjong weight 1000 vs others 0.25
        // From JsonAI.json: specific_skill desires with extreme weight disparity
        "gambler" => FamilyActionPolicy::PriorityTable {
            entries: vec![
                (SkillId::new("summon_mahjong"), 1000),
                (SkillId::new("dice_thousand"), 1),
                (SkillId::new("hollow_victory"), 1),
                (SkillId::new("card_doomsday"), 1),
                (SkillId::new("jackpot_requiem"), 1),
            ],
        },

        // All other families: use first skill fallback
        _ => FamilyActionPolicy::FirstSkill,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_skill_policy_returns_first_skill() {
        let policy = FamilyActionPolicy::FirstSkill;
        let skills = vec![
            SkillId::new("stun"),
            SkillId::new("intimidate"),
            SkillId::new("stress"),
        ];

        let state = ActorActionState::default();
        let result = select_next_skill(&policy, ActorId(10), &state, &skills);

        assert_eq!(result.0, "stun");
    }

    #[test]
    fn deterministic_cycle_starts_with_first_skill() {
        let policy = FamilyActionPolicy::DeterministicCycle {
            sequence: vec![
                SkillId::new("stun"),
                SkillId::new("intimidate"),
                SkillId::new("stress"),
            ],
        };
        let skills = vec![
            SkillId::new("stun"),
            SkillId::new("intimidate"),
            SkillId::new("stress"),
            SkillId::new("move"),
        ];

        // No last skill: should return first in sequence
        let state = ActorActionState::default();
        let result = select_next_skill(&policy, ActorId(10), &state, &skills);
        assert_eq!(result.0, "stun");
    }

    #[test]
    fn deterministic_cycle_transitions_correctly() {
        let policy = FamilyActionPolicy::DeterministicCycle {
            sequence: vec![
                SkillId::new("stun"),
                SkillId::new("intimidate"),
                SkillId::new("stress"),
            ],
        };
        let skills = vec![
            SkillId::new("stun"),
            SkillId::new("intimidate"),
            SkillId::new("stress"),
            SkillId::new("move"),
        ];

        // After stun → intimidate
        let state = ActorActionState { last_skill_used: Some(SkillId::new("stun")) };
        let result = select_next_skill(&policy, ActorId(10), &state, &skills);
        assert_eq!(result.0, "intimidate");

        // After intimidate → stress
        let state = ActorActionState { last_skill_used: Some(SkillId::new("intimidate")) };
        let result = select_next_skill(&policy, ActorId(10), &state, &skills);
        assert_eq!(result.0, "stress");

        // After stress → stun (cycle restarts)
        let state = ActorActionState { last_skill_used: Some(SkillId::new("stress")) };
        let result = select_next_skill(&policy, ActorId(10), &state, &skills);
        assert_eq!(result.0, "stun");
    }

    #[test]
    fn deterministic_cycle_resets_if_last_not_in_sequence() {
        let policy = FamilyActionPolicy::DeterministicCycle {
            sequence: vec![
                SkillId::new("stun"),
                SkillId::new("intimidate"),
                SkillId::new("stress"),
            ],
        };
        let skills = vec![
            SkillId::new("stun"),
            SkillId::new("intimidate"),
            SkillId::new("stress"),
            SkillId::new("move"),
        ];

        // Last skill is "move" (not in cycle) → reset to first
        let state = ActorActionState { last_skill_used: Some(SkillId::new("move")) };
        let result = select_next_skill(&policy, ActorId(10), &state, &skills);
        assert_eq!(result.0, "stun");
    }

    #[test]
    fn priority_table_selects_highest_weight() {
        let policy = FamilyActionPolicy::PriorityTable {
            entries: vec![
                (SkillId::new("summon_mahjong"), 1000),
                (SkillId::new("dice_thousand"), 1),
                (SkillId::new("hollow_victory"), 1),
            ],
        };
        let skills = vec![
            SkillId::new("summon_mahjong"),
            SkillId::new("dice_thousand"),
            SkillId::new("hollow_victory"),
        ];

        let state = ActorActionState::default();
        let result = select_next_skill(&policy, ActorId(10), &state, &skills);

        // Highest weight is summon_mahjong (1000)
        assert_eq!(result.0, "summon_mahjong");
    }

    #[test]
    fn priority_table_ignores_last_skill_state() {
        // Priority table doesn't use state — it always picks highest weight
        let policy = FamilyActionPolicy::PriorityTable {
            entries: vec![
                (SkillId::new("summon_mahjong"), 1000),
                (SkillId::new("dice_thousand"), 1),
            ],
        };
        let skills = vec![
            SkillId::new("summon_mahjong"),
            SkillId::new("dice_thousand"),
        ];

        // Even if last_skill_used is dice_thousand, should still pick summon_mahjong
        let state = ActorActionState { last_skill_used: Some(SkillId::new("dice_thousand")) };

        let result = select_next_skill(&policy, ActorId(10), &state, &skills);
        assert_eq!(result.0, "summon_mahjong");
    }

    #[test]
    fn get_default_policy_lizard_is_cycle() {
        let policy = get_default_policy("lizard");
        match policy {
            FamilyActionPolicy::DeterministicCycle { sequence } => {
                assert_eq!(sequence.len(), 3);
                assert_eq!(sequence[0].0, "stun");
                assert_eq!(sequence[1].0, "intimidate");
                assert_eq!(sequence[2].0, "stress");
            }
            _ => panic!("lizard should have DeterministicCycle policy"),
        }
    }

    #[test]
    fn get_default_policy_gambler_is_priority() {
        let policy = get_default_policy("gambler");
        match policy {
            FamilyActionPolicy::PriorityTable { entries } => {
                // summon_mahjong should have highest weight
                let max_entry = entries.iter().max_by_key(|(_, w)| *w).unwrap();
                assert_eq!(max_entry.0.0, "summon_mahjong");
                assert_eq!(max_entry.1, 1000);
            }
            _ => panic!("gambler should have PriorityTable policy"),
        }
    }

    #[test]
    fn get_default_policy_unknown_family_is_first_skill() {
        let policy = get_default_policy("mantis_magic_flower");
        match policy {
            FamilyActionPolicy::FirstSkill => {}
            _ => panic!("unknown family should use FirstSkill fallback"),
        }
    }

    #[test]
    fn update_actor_state_records_last_skill() {
        let mut state = ActorActionState::default();
        assert!(state.last_skill_used.is_none());

        update_actor_state(&mut state, SkillId::new("stun"));
        assert_eq!(state.last_skill_used.as_ref().unwrap().0, "stun");

        update_actor_state(&mut state, SkillId::new("intimidate"));
        assert_eq!(state.last_skill_used.as_ref().unwrap().0, "intimidate");
    }
}