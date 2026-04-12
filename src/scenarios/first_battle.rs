//! First deterministic DDGC battle — Crusader vs Bone Soldier.
//!
//! This scenario runs a minimal but real DDGC battle using only migrated
//! content (archetypes, skills, statuses from the content module).
//! The battle is fully deterministic: no randomness, no UI runtime.
//!
//! Game-layer responsibilities handled here:
//! - Turn-order stepping (fastest actor first)
//! - Skill selection (deterministic script: allies attack, enemies attack)
//! - Target resolution (positional)
//! - Effect application via `resolve_skill`
//! - Defeated-actor cleanup
//! - Status tick tracking for trace output

use std::collections::HashMap;

use framework_combat::commands::CombatCommand;
use framework_combat::effects::{EffectContext, resolve_skill};
use framework_combat::encounter::{CombatSide, Encounter, EncounterId, EncounterState};
use framework_combat::formation::{FormationLayout, SlotIndex};
use framework_combat::resolver::CombatResolver;
use framework_combat::skills::SkillId;
use framework_rules::actor::ActorId;
use framework_rules::attributes::{AttributeKey, ATTR_HEALTH};

use crate::content::ContentPack;
use crate::trace::BattleTrace;

/// Skill script for one side: which skill each actor uses each turn.
///
/// In a full game this would come from AI / player input.
/// For the deterministic battle, it's a fixed sequence.
struct SkillScript {
    /// Map from ActorId to the skill they always use.
    skills: HashMap<u64, &'static str>,
}

impl SkillScript {
    fn new() -> Self {
        SkillScript {
            skills: HashMap::new(),
        }
    }

    fn assign(&mut self, actor_id: u64, skill_name: &'static str) {
        self.skills.insert(actor_id, skill_name);
    }

    fn get(&self, actor_id: ActorId) -> Option<&'static str> {
        self.skills.get(&actor_id.0).copied()
    }
}

/// Result of running the first battle.
pub struct BattleResult {
    pub winner: Option<CombatSide>,
    pub turns: u32,
    pub trace: BattleTrace,
}

/// Run the first deterministic DDGC battle.
///
/// Setup: Crusader (Ally, rank 1) vs Bone Soldier (Enemy, rank 1).
/// Both actors use their basic attack every turn until one is defeated.
/// The battle is deterministic because damage values are fixed (no randomness).
pub fn run_first_battle() -> BattleResult {
    let pack = ContentPack::default();

    // ── Actors ──────────────────────────────────────────────────────────────
    let crusader_id = ActorId(1);
    let bone_soldier_id = ActorId(10);

    let crusader_arch = pack.get_archetype("Crusader").unwrap();
    let bone_soldier_arch = pack.get_archetype("Bone Soldier").unwrap();

    let mut actors: HashMap<ActorId, framework_rules::actor::ActorAggregate> = HashMap::new();
    actors.insert(crusader_id, crusader_arch.create_actor(crusader_id));
    actors.insert(bone_soldier_id, bone_soldier_arch.create_actor(bone_soldier_id));

    // ── Encounter ───────────────────────────────────────────────────────────
    let mut encounter = Encounter::new(
        EncounterId(1),
        vec![crusader_id],
        vec![bone_soldier_id],
    );

    // ── Formation ───────────────────────────────────────────────────────────
    let mut formation = FormationLayout::new(2, 2);
    formation.place(crusader_id, SlotIndex(0)).unwrap(); // ally front
    formation.place(bone_soldier_id, SlotIndex(2)).unwrap(); // enemy front

    // ── Side lookup for target resolution ───────────────────────────────────
    let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();
    side_lookup.insert(crusader_id, CombatSide::Ally);
    side_lookup.insert(bone_soldier_id, CombatSide::Enemy);

    // ── Skill script ────────────────────────────────────────────────────────
    let mut script = SkillScript::new();
    script.assign(crusader_id.0, "crusading_strike");
    script.assign(bone_soldier_id.0, "rend");

    // ── Resolver ────────────────────────────────────────────────────────────
    let mut resolver = CombatResolver::new(3);
    resolver.start(&mut encounter, &actors);

    // ── Battle loop ─────────────────────────────────────────────────────────
    let mut trace = BattleTrace::new("first_battle");
    let mut round: u32 = 0;
    let max_rounds = 50; // safety limit

    while encounter.state == EncounterState::Active && round < max_rounds {
        round += 1;

        let current_actor = encounter
            .current_turn
            .as_ref()
            .map(|t| t.current_actor)
            .unwrap();

        // Check if actor is alive (may have been killed by status tick)
        let hp = actors[&current_actor].effective_attribute(&AttributeKey::new(ATTR_HEALTH));
        if hp.0 <= 0.0 {
            remove_defeated(&mut encounter, &mut actors, current_actor);
            resolver.end_turn(&mut encounter, &mut actors);
            continue;
        }

        // Select skill from script
        let skill_name = script.get(current_actor).unwrap_or("crusading_strike");
        let skill = pack.get_skill(&SkillId::new(skill_name)).unwrap();

        // Resolve targets
        let targets = skill
            .target_selector
            .resolve(current_actor, &formation, &actors, &side_lookup);

        if targets.is_empty() {
            // No valid targets — wait
            let cmd = CombatCommand::Wait {
                actor: current_actor,
            };
            resolver.submit_command(&mut encounter, &mut actors, cmd);
            trace.record_wait(round, current_actor, &actors);
        } else {
            // Submit command
            let cmd = CombatCommand::UseSkill {
                actor: current_actor,
                skill: skill.id.clone(),
                targets: targets.clone(),
            };
            let resolution = resolver.submit_command(&mut encounter, &mut actors, cmd);

            if resolution.accepted {
                // Apply effects manually (resolver validates budget but doesn't execute)
                let mut ctx = EffectContext::new(
                    current_actor,
                    targets.clone(),
                    &mut formation,
                    &mut actors,
                );
                let effect_results = resolve_skill(skill, &mut ctx);

                // Record in trace
                trace.record_action(
                    round,
                    current_actor,
                    skill_name,
                    &targets,
                    &effect_results,
                    &actors,
                );

                // Remove defeated actors
                let all_ids: Vec<ActorId> = encounter
                    .allies()
                    .iter()
                    .chain(encounter.enemies().iter())
                    .copied()
                    .collect();
                for id in all_ids {
                    let hp = actors[&id].effective_attribute(&AttributeKey::new(ATTR_HEALTH));
                    if hp.0 <= 0.0 {
                        remove_defeated(&mut encounter, &mut actors, id);
                    }
                }
            } else {
                // Command rejected — wait instead
                let cmd = CombatCommand::Wait {
                    actor: current_actor,
                };
                resolver.submit_command(&mut encounter, &mut actors, cmd);
                trace.record_wait(round, current_actor, &actors);
            }
        }

        // End turn (ticks statuses, advances turn, checks resolution)
        resolver.end_turn(&mut encounter, &mut actors);
    }

    // ── Result ──────────────────────────────────────────────────────────────
    let winner = match &encounter.state {
        EncounterState::Resolved { winner } => *winner,
        _ => resolver.check_resolution(&encounter),
    };

    trace.finalize(winner, round);

    BattleResult {
        winner,
        turns: round,
        trace,
    }
}

/// Remove a defeated actor from encounter and turn order.
fn remove_defeated(
    encounter: &mut Encounter,
    _actors: &mut HashMap<ActorId, framework_rules::actor::ActorAggregate>,
    id: ActorId,
) {
    encounter.remove_actor(id);
    if let Some(ref mut to) = encounter.turn_order {
        to.remove(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_battle_runs_to_completion() {
        let result = run_first_battle();

        // Battle must terminate with a winner
        assert!(
            result.winner.is_some(),
            "Battle should have a winner, got None"
        );

        // Must finish within reasonable turns
        assert!(
            result.turns <= 20,
            "Battle took {} turns — expected <= 20",
            result.turns
        );

        // Trace must have entries
        assert!(
            !result.trace.entries.is_empty(),
            "Trace should record battle events"
        );
    }

    #[test]
    fn first_battle_trace_is_deterministic() {
        // Run twice and compare JSON traces
        let trace1 = run_first_battle().trace.to_json();
        let trace2 = run_first_battle().trace.to_json();

        assert_eq!(
            trace1, trace2,
            "Two runs of the same battle must produce identical traces"
        );
    }

    #[test]
    fn first_battle_uses_only_migrated_content() {
        let result = run_first_battle();

        // Every action in the trace must reference a migrated skill or "wait"
        let valid_actions = [
            "crusading_strike",
            "rend",
            "wait",
            "status_tick",
        ];

        for entry in &result.trace.entries {
            assert!(
                valid_actions.contains(&entry.action.as_str()),
                "Unknown action '{}' — battle should only use migrated content",
                entry.action
            );
        }

        // Winner must be Ally or Enemy (no "NONE" in resolved state)
        if let Some(winner) = result.winner {
            assert!(
                matches!(winner, CombatSide::Ally | CombatSide::Enemy),
                "Winner should be Ally or Enemy, got {:?}",
                winner
            );
        }
    }
}
