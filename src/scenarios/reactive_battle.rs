//! Reactive battle scenario — demonstrates riposte and guard redirect mechanics.
//!
//! This scenario runs a battle that triggers reactive follow-up events:
//! - Riposte: when an actor with the riposte status is hit, they counter-attack
//! - Guard redirect: when a guarded target is hit, the guard absorbs the damage
//!
//! The scenario manually pre-attaches riposte and guard statuses to actors
//! because the framework's apply_status EffectNode does not auto-attach
//! statuses during skill resolution (game-layer concern).
//!
//! ## Battle Setup
//!
//! Allies:
//! - Actor 1 (Tank): has riposte status — will counter-attack when hit
//! - Actor 2 (Hunter): uses direct damage attacks
//!
//! Enemies:
//! - Actor 10 (Frostvein Clam): uses glacial_torrent (damage + frozen)
//! - Actor 11 (Frostvein Clam): uses glacial_torrent
//!
//! The enemies attack the tank (actor 1), triggering riposte counter-attacks.
//! Guard relationships are not used in this scenario to keep the trace clear.

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

/// Result of running the reactive battle.
pub struct ReactiveBattleResult {
    pub winner: Option<CombatSide>,
    pub turns: u32,
    pub trace: BattleTrace,
}

/// Run a battle that triggers reactive riposte events.
///
/// The scenario manually pre-attaches the riposte status to the tank
/// because the framework's apply_status EffectNode does not auto-attach
/// statuses during skill resolution.
pub fn run_reactive_battle() -> ReactiveBattleResult {
    let pack = ContentPack::default();

    // ── Actors ──────────────────────────────────────────────────────────────
    // Actor 1: Ally Tank with riposte status (will counter-attack when hit)
    let tank_id = ActorId(1);
    let tank_arch = pack.get_archetype("Tank").unwrap();
    let mut tank = tank_arch.create_actor(tank_id);
    tank.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(192.0));
    // Manually attach riposte status (framework apply_status doesn't auto-attach)
    tank.statuses.attach(statuses::riposte(10));

    // Actor 2: Ally Hunter (uses direct damage attacks)
    let hunter_id = ActorId(2);
    let hunter_arch = pack.get_archetype("Hunter").unwrap();
    let mut hunter = hunter_arch.create_actor(hunter_id);
    hunter.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(152.0));

    // Actor 10: Enemy (Frostvein Clam) - attacks the tank, triggering riposte
    let enemy1_id = ActorId(10);
    let enemy1_arch = pack.get_archetype("Frostvein Clam").unwrap();
    let mut enemy1 = enemy1_arch.create_actor(enemy1_id);
    enemy1.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(150.0));

    // Actor 11: Enemy (Frostvein Clam) - also attacks
    let enemy2_id = ActorId(11);
    let enemy2_arch = pack.get_archetype("Frostvein Clam").unwrap();
    let mut enemy2 = enemy2_arch.create_actor(enemy2_id);
    enemy2.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(150.0));

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
    formation.place(tank_id, SlotIndex(0)).unwrap();      // ally front
    formation.place(hunter_id, SlotIndex(1)).unwrap();   // ally back
    formation.place(enemy1_id, SlotIndex(4)).unwrap();  // enemy front
    formation.place(enemy2_id, SlotIndex(5)).unwrap();  // enemy back

    // ── Side lookup ────────────────────────────────────────────────────────
    let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();
    side_lookup.insert(tank_id, CombatSide::Ally);
    side_lookup.insert(hunter_id, CombatSide::Ally);
    side_lookup.insert(enemy1_id, CombatSide::Enemy);
    side_lookup.insert(enemy2_id, CombatSide::Enemy);

    // ── Resolver ───────────────────────────────────────────────────────────
    let mut resolver = CombatResolver::new(3);
    resolver.start(&mut encounter, &actors);

    // ── Battle loop ─────────────────────────────────────────────────────────
    let mut trace = BattleTrace::new("reactive_battle");
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
        let skill_name = match current_actor {
            id if id == tank_id => "taunt_skill",      // Tank uses damage skill
            id if id == hunter_id => "ignore_def_skill", // Hunter uses damage skill
            id if id == enemy1_id => "glacial_torrent", // Enemy uses damage skill
            id if id == enemy2_id => "glacial_torrent", // Enemy uses damage skill
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
                // Only process reactive events if there was actual damage
                let damage_amount = effect_results.results.iter()
                    .find_map(|r| r.values.get("amount").copied());

                if damage_amount.is_some() && damage_amount.unwrap() > 0.0 {
                    let mut reactive_queue = crate::run::reactive_queue::ReactiveQueue::new();

                    for &target in &targets {
                        // Riposte detection: actors with riposte status who were hit
                        // FIX: Check target == candidate, not just candidate in targets
                        let candidates = crate::run::riposte_detection::detect_riposte_candidates(&actors);
                        for candidate in candidates {
                            // Only create event if THIS target (not any target) has riposte and was hit
                            if target == candidate {
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

                    // Process reactive queue: execute riposte counter-attacks
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

    ReactiveBattleResult {
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
    fn reactive_battle_runs_to_completion() {
        let result = run_reactive_battle();

        assert!(
            result.winner.is_some(),
            "Battle should have a winner"
        );
        assert!(
            result.turns <= 50,
            "Battle should finish within 50 turns, took {}",
            result.turns
        );
        assert!(
            !result.trace.entries.is_empty(),
            "Trace should record battle events"
        );
    }

    #[test]
    fn reactive_battle_trace_is_deterministic() {
        let trace1 = run_reactive_battle().trace.to_json();
        let trace2 = run_reactive_battle().trace.to_json();

        assert_eq!(
            trace1, trace2,
            "Two runs of the same battle must produce identical traces"
        );
    }

    #[test]
    fn reactive_battle_contains_reactive_entries() {
        let result = run_reactive_battle();

        let reactive_entries: Vec<_> = result.trace.entries.iter()
            .filter(|e| e.triggered_by.is_some())
            .collect();

        assert!(
            !reactive_entries.is_empty(),
            "Reactive battle should have at least one reactive entry"
        );
    }

    #[test]
    fn reactive_battle_contains_riposte() {
        let result = run_reactive_battle();

        let riposte_entries: Vec<_> = result.trace.entries.iter()
            .filter(|e| e.triggered_by.as_ref()
                .map(|t| t.kind == "Riposte")
                .unwrap_or(false))
            .collect();

        assert!(
            !riposte_entries.is_empty(),
            "Reactive battle should have at least one riposte entry"
        );
    }
}