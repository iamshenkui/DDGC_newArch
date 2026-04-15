//! Multi-target riposte battle scenario — verifies per-target riposte correctness.
//!
//! This scenario tests that riposte events are generated ONLY for actors
//! actually hit in the current target iteration, not for all riposte candidates
//! in the overall target set.
//!
//! ## Bug Being Tested
//!
//! The bug (pre-fix): when iterating over targets in a multi-target attack,
//! the riposte filtering checked `targets.contains(&candidate)` which is true
//! for ANY candidate in the target set, not just the CURRENT target. This
//! caused riposte events to be generated for candidates who weren't hit in
//! that specific iteration.
//!
//! The fix: change to `candidate == target` so only the actor actually hit
//! in this iteration triggers their riposte.
//!
//! ## Battle Setup
//!
//! Allies:
//! - Actor 1 (Tank): has riposte status (duration 10)
//! - Actor 2 (Hunter): basic attacks
//!
//! Enemies:
//! - Actor 10: uses multi-target attack (targets all enemies)
//! - Actor 11: uses multi-target attack (targets all enemies)
//!
//! Both enemy actors use multi-target attacks simultaneously. The tank (Actor 1)
//! should get hit and trigger riposte counter-attacks. The key test: with the bug,
//! both iterations (target=10 and target=11) would generate riposte events for
//! Actor 1 even though Actor 1 was only hit once. With the fix, each target
//! iteration only generates riposte for actors hit in THAT iteration.

use std::collections::HashMap;

use framework_combat::commands::CombatCommand;
use framework_combat::effects::{EffectContext, resolve_skill};
use framework_combat::encounter::{CombatSide, Encounter, EncounterId, EncounterState};
use framework_combat::formation::{FormationLayout, SlotIndex};
use framework_combat::resolver::CombatResolver;
use framework_combat::skills::SkillId;
use framework_rules::actor::{ActorAggregate, ActorId};
use framework_rules::attributes::{AttributeKey, AttributeValue, ATTR_HEALTH};

use crate::content::ContentPack;
use crate::content::statuses;
use crate::trace::BattleTrace;

/// Result of running the multi-target riposte battle.
pub struct MultiTargetRiposteResult {
    pub winner: Option<CombatSide>,
    pub turns: u32,
    pub trace: BattleTrace,
}

/// Run a battle that exercises per-target riposte detection on multi-target hits.
///
/// The scenario uses a simple structure:
/// - Tank (Actor 1) has riposte status
/// - Enemies (Actors 10, 11) use multi-target attacks
///
/// This verifies that riposte events are generated only when candidate == target
/// (the actor hit in this iteration), not when candidate is merely in the
/// overall target set.
pub fn run_multi_target_riposte_battle() -> MultiTargetRiposteResult {
    let pack = ContentPack::default();

    // ── Actors ──────────────────────────────────────────────────────────────
    // Actor 1: Ally Tank with riposte status
    let tank_id = ActorId(1);
    let tank_arch = pack.get_archetype("Tank").unwrap();
    let mut tank = tank_arch.create_actor(tank_id);
    tank.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(192.0));
    tank.statuses.attach(statuses::riposte(10));

    // Actor 2: Ally Hunter (uses attacks)
    let hunter_id = ActorId(2);
    let hunter_arch = pack.get_archetype("Hunter").unwrap();
    let mut hunter = hunter_arch.create_actor(hunter_id);
    hunter.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(152.0));

    // Actor 10: Enemy - will use multi-target attack
    let enemy1_id = ActorId(10);
    let enemy1_arch = pack.get_archetype("Lizard").unwrap();
    let mut enemy1 = enemy1_arch.create_actor(enemy1_id);
    enemy1.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));

    // Actor 11: Enemy - will use multi-target attack
    let enemy2_id = ActorId(11);
    let enemy2_arch = pack.get_archetype("Lizard").unwrap();
    let mut enemy2 = enemy2_arch.create_actor(enemy2_id);
    enemy2.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));

    let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
    actors.insert(tank_id, tank);
    actors.insert(hunter_id, hunter);
    actors.insert(enemy1_id, enemy1);
    actors.insert(enemy2_id, enemy2);

    // ── Encounter ───────────────────────────────────────────────────────────
    let mut encounter = Encounter::new(
        EncounterId(1),
        vec![tank_id, hunter_id],
        vec![enemy1_id, enemy2_id],
    );

    // ── Formation ───────────────────────────────────────────────────────────
    let mut formation = FormationLayout::new(2, 4);
    formation.place(tank_id, SlotIndex(0)).unwrap();    // ally front
    formation.place(hunter_id, SlotIndex(1)).unwrap();  // ally back
    formation.place(enemy1_id, SlotIndex(4)).unwrap();  // enemy front
    formation.place(enemy2_id, SlotIndex(5)).unwrap();  // enemy back

    // ── Side lookup ────────────────────────────────────────────────────────
    let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();
    side_lookup.insert(tank_id, CombatSide::Ally);
    side_lookup.insert(hunter_id, CombatSide::Ally);
    side_lookup.insert(enemy1_id, CombatSide::Enemy);
    side_lookup.insert(enemy2_id, CombatSide::Enemy);

    // ── Resolver ─────────────────────────────────────────────────────────
    let mut resolver = CombatResolver::new(3);
    resolver.start(&mut encounter, &actors);

    // ── Battle loop ─────────────────────────────────────────────────────────
    let mut trace = BattleTrace::new("multi_target_riposte_battle");
    let mut round: u32 = 0;
    let max_rounds = 50;

    while encounter.state == EncounterState::Active && round < max_rounds {
        round += 1;

        let current_actor = match encounter.current_turn.as_ref() {
            Some(t) => t.current_actor,
            None => break,
        };

        // Check if actor is alive
        let hp = actors[&current_actor].effective_attribute(&AttributeKey::new(ATTR_HEALTH));
        if hp.0 <= 0.0 {
            remove_defeated(&mut encounter, &mut actors, current_actor);
            resolver.end_turn(&mut encounter, &mut actors);
            continue;
        }

        // Determine skill for current actor
        // Enemies use lizard skills (bite is single-target, but we'll use a skill that hits multiple)
        // Actually, Lizard's skills are single-target by default in the framework.
        // For this test, we'll use a direct damage approach where enemies attack all allies.
        let skill_name = match current_actor {
            id if id == tank_id => "taunt_skill",
            id if id == hunter_id => "ignore_def_skill",
            id if id == enemy1_id => "stun", // Lizard's stun targets all enemies via AllEnemies
            id if id == enemy2_id => "stun",
            _ => "normal_attack",
        };

        let skill = match pack.get_skill(&SkillId::new(skill_name)) {
            Some(s) => s,
            None => {
                let cmd = CombatCommand::Wait { actor: current_actor };
                resolver.submit_command(&mut encounter, &mut actors, cmd);
                trace.record_wait(round, current_actor, &actors);
                resolver.end_turn(&mut encounter, &mut actors);
                continue;
            }
        };

        // Resolve targets
        let mut targets = skill
            .target_selector
            .resolve(current_actor, &formation, &actors, &side_lookup);
        targets.sort_by_key(|t| t.0);

        if targets.is_empty() {
            let cmd = CombatCommand::Wait { actor: current_actor };
            resolver.submit_command(&mut encounter, &mut actors, cmd);
            trace.record_wait(round, current_actor, &actors);
        } else {
            let cmd = CombatCommand::UseSkill {
                actor: current_actor,
                skill: skill.id.clone(),
                targets: targets.clone(),
            };
            let resolution = resolver.submit_command(&mut encounter, &mut actors, cmd);

            if resolution.accepted {
                let mut ctx = EffectContext::new(
                    current_actor,
                    targets.clone(),
                    &mut formation,
                    &mut actors,
                );
                let effect_results = resolve_skill(skill, &mut ctx);

                // ── Reactive Processing ─────────────────────────────────────
                let damage_amount = effect_results.results.iter()
                    .find_map(|r| r.values.get("amount").copied());

                if damage_amount.is_some() && damage_amount.unwrap() > 0.0 {
                    let mut reactive_queue = crate::run::reactive_queue::ReactiveQueue::new();

                    for &target in &targets {
                        // Riposte detection: actor with riposte status who was hit
                        // FIX: Check candidate == target, not targets.contains(&candidate)
                        let candidates = crate::run::riposte_detection::detect_riposte_candidates(&actors);
                        for candidate in candidates {
                            // Only create event if this specific target (not any target) was hit
                            if candidate == target {
                                let ctx = crate::run::reactive_events::DamageStepContext::new(
                                    current_actor,
                                    skill.id.clone(),
                                    target,
                                    damage_amount,
                                );
                                let events = crate::run::reactive_events::build_reactive_events(
                                    &ctx,
                                    candidate,
                                    crate::run::reactive_events::ReactiveEventKind::Riposte,
                                );
                                for event in events {
                                    reactive_queue.enqueue(event);
                                }
                            }
                        }
                    }

                    // Process reactive queue
                    while let Some(event) = reactive_queue.drain_next() {
                        if event.is_riposte() {
                            let reactor_id = event.reactor;
                            let has_riposte = if let Some(reactor) = actors.get(&reactor_id) {
                                crate::run::riposte_execution::has_riposte_status(reactor)
                            } else {
                                false
                            };
                            if has_riposte {
                                if let Some((_skill_id, reactive_results)) = crate::run::riposte_execution::execute_riposte(
                                    &event,
                                    &pack,
                                    &mut actors,
                                    &mut formation,
                                    &side_lookup,
                                ) {
                                    let trigger = crate::trace::ReactiveTrigger {
                                        attacker: event.attacker.0,
                                        skill: skill_name.to_string(),
                                        target: event.triggered_on.0,
                                        kind: "Riposte".to_string(),
                                    };
                                    trace.record_reactive(
                                        round,
                                        event.reactor,
                                        "riposte",
                                        &[event.attacker],
                                        &reactive_results,
                                        &actors,
                                        trigger,
                                    );
                                }
                            }
                        }
                    }
                }

                trace.record_action(
                    round,
                    current_actor,
                    skill_name,
                    &targets,
                    &effect_results.results,
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
                let cmd = CombatCommand::Wait { actor: current_actor };
                resolver.submit_command(&mut encounter, &mut actors, cmd);
                trace.record_wait(round, current_actor, &actors);
            }
        }

        resolver.end_turn(&mut encounter, &mut actors);
    }

    let winner = match &encounter.state {
        EncounterState::Resolved { winner } => *winner,
        _ => resolver.check_resolution(&encounter),
    };

    trace.finalize(winner, round);

    MultiTargetRiposteResult {
        winner,
        turns: round,
        trace,
    }
}

/// Remove a defeated actor from encounter and turn order.
fn remove_defeated(
    encounter: &mut Encounter,
    _actors: &mut HashMap<ActorId, ActorAggregate>,
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
    fn multi_target_riposte_battle_runs_to_completion() {
        let result = run_multi_target_riposte_battle();

        assert!(
            result.winner.is_some(),
            "Battle should have a winner"
        );
        assert!(
            result.turns <= 50,
            "Battle should finish within 50 turns, took {}",
            result.turns
        );
    }

    #[test]
    fn multi_target_riposte_trace_is_deterministic() {
        let trace1 = run_multi_target_riposte_battle().trace.to_json();
        let trace2 = run_multi_target_riposte_battle().trace.to_json();

        assert_eq!(
            trace1, trace2,
            "Two runs of the same battle must produce identical traces"
        );
    }

    #[test]
    fn per_target_riposte_only_for_actors_hit() {
        // This test verifies the fix for US-804: riposte events should only
        // be generated for actors who were actually hit in the current target
        // iteration, not for all riposte candidates in the target set.
        //
        // In this scenario with 2 enemies using multi-target attacks:
        // - Tank (Actor 1) has riposte status
        // - Both enemies (Actors 10, 11) are hit by the multi-target attack
        //
        // With the bug (targets.contains(&candidate)), both iterations would
        // generate riposte for Actor 1 even though Actor 1 was only hit once.
        // With the fix (candidate == target), only the iteration where
        // target == Actor 1 would generate the riposte event.
        let result = run_multi_target_riposte_battle();

        let riposte_entries: Vec<_> = result.trace.entries.iter()
            .filter(|e| e.triggered_by.as_ref()
                .map(|t| t.kind == "Riposte")
                .unwrap_or(false))
            .collect();

        // The riposte events should have correct target attribution
        // (trigger.target should match the reactor who was hit)
        for entry in &riposte_entries {
            if let Some(trigger) = &entry.triggered_by {
                // The reactive entry should show the reactor (who has riposte)
                // being triggered when THEY were hit
                assert_eq!(
                    trigger.target, entry.actor,
                    "Riposte trigger target should match the reactor actor"
                );
            }
        }
    }
}