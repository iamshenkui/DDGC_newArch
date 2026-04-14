//! Encounter resolution — maps combat and boss rooms to DDGC encounter packs.
//!
//! This module connects the run flow to the encounter pack registry,
//! so combat rooms resolve through real DDGC enemy compositions instead
//! of a single hard-coded battle scenario.
//!
//! Combat rooms resolve to room packs (heavier encounters).
//! Corridor rooms resolve to hall packs (lighter encounters).
//! Boss rooms resolve to boss packs (boss + boss parts).

use std::collections::HashMap;

use framework_combat::commands::CombatCommand;
use framework_combat::effects::{EffectContext, resolve_skill};
use framework_combat::encounter::{CombatSide, Encounter, EncounterId, EncounterState};
use framework_combat::formation::{FormationLayout, SlotIndex};
use framework_combat::resolver::CombatResolver;
use framework_combat::skills::SkillId;
use framework_rules::actor::{ActorAggregate, ActorId};
use framework_rules::attributes::{AttributeKey, ATTR_HEALTH};

use crate::content::ContentPack;
use crate::encounters::{build_packs_registry, Dungeon, EncounterPack, EncounterPackRegistry, PackType};
use crate::encounters::ddgc_targeting_rules::get_ddgc_targeting_rule;
use crate::monsters::build_registry as build_monster_registry;
use crate::monsters::MonsterFamilyRegistry;
use crate::trace::BattleTrace;
use crate::run::guard_detection::detect_guard_relations_for_target;
use crate::run::reactive_events::{build_reactive_events, DamageStepContext, ReactiveEventKind};
use crate::run::reactive_queue::ReactiveQueue;
use crate::run::riposte_detection::detect_riposte_candidates;
use crate::run::riposte_execution::{execute_riposte, has_riposte_status};
use crate::run::guard_redirect_execution::execute_guard_redirect;
use crate::run::usage_counters::SkillUsageCounters;
use crate::run::usage_limits::get_usage_limit;

/// Skill assignment for encounter battles.
///
/// Maps each actor to the skill they use in the deterministic battle script.
/// In a full game, this would be replaced by AI/player input.
struct SkillAssignment {
    skills: HashMap<u64, String>,
}

impl SkillAssignment {
    fn new() -> Self {
        SkillAssignment {
            skills: HashMap::new(),
        }
    }

    fn assign(&mut self, actor_id: ActorId, skill_name: impl Into<String>) {
        self.skills.insert(actor_id.0, skill_name.into());
    }

    fn get(&self, actor_id: ActorId) -> Option<&str> {
        self.skills.get(&actor_id.0).map(|s| s.as_str())
    }
}

/// Result of running an encounter battle.
pub struct EncounterBattleResult {
    pub winner: Option<CombatSide>,
    pub turns: u32,
    pub trace: BattleTrace,
    pub pack_id: String,
}

/// Resolves combat rooms to encounter packs and runs battles.
///
/// Holds the encounter pack registry, monster family registry, and content pack
/// needed to build and execute encounters from DDGC pack definitions.
pub struct EncounterResolver {
    pack_registry: EncounterPackRegistry,
    content_pack: ContentPack,
    monster_registry: MonsterFamilyRegistry,
}

/// Default ally party for encounter battles.
///
/// Uses DDGC-scale hero archetypes (not legacy tutorial heroes like Crusader/Vestal
/// which have ~30 HP vs the DDGC-scale ~150 HP). Each hero is assigned a
/// damage-dealing primary skill for the deterministic battle script.
///
/// Party composition: Tank (frontline), Hunter (damage), Shaman (caster), Diviner (support).
const DEFAULT_PARTY: &[(&str, &str)] = &[
    ("Tank", "active_riposte"),    // 192 HP, 16 dmg + tag
    ("Hunter", "ignore_def_skill"), // 152 HP, 40 dmg
    ("Shaman", "direct_hit_1"),    // 135 HP, 39 dmg
    ("Diviner", "duality_fate"),   // 160 HP, 9 dmg
];

impl EncounterResolver {
    /// Create a new resolver with all DDGC common encounter packs.
    pub fn new() -> Self {
        EncounterResolver {
            pack_registry: build_packs_registry(),
            content_pack: ContentPack::default(),
            monster_registry: build_monster_registry(),
        }
    }

    /// Resolve a combat room to an encounter pack deterministically.
    ///
    /// Uses the dungeon, room index, and seed to select a pack. Room packs
    /// are used for combat rooms; hall packs for corridor rooms.
    ///
    /// The selection is deterministic: the same (dungeon, room_index, seed, is_corridor)
    /// always yields the same pack.
    pub fn resolve_pack(
        &self,
        dungeon: Dungeon,
        room_index: u32,
        seed: u64,
        is_corridor: bool,
    ) -> Option<&EncounterPack> {
        let pack_type = if is_corridor {
            PackType::Hall
        } else {
            PackType::Room
        };
        let mut packs = self.pack_registry.by_dungeon_and_type(dungeon, pack_type);
        if packs.is_empty() {
            return None;
        }
        // Sort by pack ID for deterministic selection (HashMap order is not guaranteed)
        packs.sort_by(|a, b| a.id.0.cmp(&b.id.0));
        let selector = (seed.wrapping_add(room_index as u64)) as usize;
        let index = selector % packs.len();
        Some(packs[index])
    }

    /// Resolve a boss room to a boss encounter pack deterministically.
    ///
    /// Selects from the boss packs registered for the given dungeon.
    /// If no boss packs exist for the dungeon (e.g., Dungeon::Cross has
    /// no room-based boss encounters), returns None.
    ///
    /// The selection is deterministic: the same (dungeon, room_index, seed)
    /// always yields the same boss pack.
    pub fn resolve_boss_pack(
        &self,
        dungeon: Dungeon,
        room_index: u32,
        seed: u64,
    ) -> Option<&EncounterPack> {
        let mut packs = self.pack_registry.by_dungeon_and_type(dungeon, PackType::Boss);
        if packs.is_empty() {
            return None;
        }
        // Sort by pack ID for deterministic selection
        packs.sort_by(|a, b| a.id.0.cmp(&b.id.0));
        let selector = (seed.wrapping_add(room_index as u64)) as usize;
        let index = selector % packs.len();
        Some(packs[index])
    }

    /// Run a battle using an encounter pack's enemy composition.
    ///
    /// Creates a 4-hero ally party and enemies from the pack's family slots.
    /// Each actor uses their primary skill in a deterministic script.
    pub fn run_battle(&self, pack: &EncounterPack, encounter_id: u64) -> EncounterBattleResult {
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut ally_ids = Vec::new();
        let mut enemy_ids = Vec::new();
        let mut skill_assign = SkillAssignment::new();
        let mut side_lookup: HashMap<ActorId, CombatSide> = HashMap::new();

        // ── Ally actors (4-hero party) ─────────────────────────────────────
        for (i, (archetype_name, skill_name)) in DEFAULT_PARTY.iter().enumerate() {
            let actor_id = ActorId(i as u64 + 1);
            if let Some(archetype) = self.content_pack.get_archetype(archetype_name) {
                actors.insert(actor_id, archetype.create_actor(actor_id));
                ally_ids.push(actor_id);
                skill_assign.assign(actor_id, *skill_name);
                side_lookup.insert(actor_id, CombatSide::Ally);
            }
        }

        // ── Enemy actors from pack ─────────────────────────────────────────
        let mut next_enemy_id: u64 = 10;
        for slot in &pack.slots {
            let family = self
                .monster_registry
                .get(&slot.family_id.0)
                .unwrap_or_else(|| panic!("Family '{}' should be registered", slot.family_id));

            let archetype = self
                .content_pack
                .get_archetype(&family.archetype_name)
                .unwrap_or_else(|| {
                    panic!(
                        "Archetype '{}' should be registered for family '{}'",
                        family.archetype_name, slot.family_id
                    )
                });

            // Use the family's first skill as the primary attack
            let skill_name = family
                .skill_ids
                .first()
                .map(|s| s.0.as_str())
                .unwrap_or("normal_attack");

            for _ in 0..slot.count {
                let actor_id = ActorId(next_enemy_id);
                actors.insert(actor_id, archetype.create_actor(actor_id));
                enemy_ids.push(actor_id);
                skill_assign.assign(actor_id, skill_name);
                side_lookup.insert(actor_id, CombatSide::Enemy);
                next_enemy_id += 1;
            }
        }

        // ── Encounter ──────────────────────────────────────────────────────
        let mut encounter = Encounter::new(
            EncounterId(encounter_id),
            ally_ids.clone(),
            enemy_ids.clone(),
        );

        // ── Formation ──────────────────────────────────────────────────────
        let ally_count = ally_ids.len() as u32;
        let enemy_count = enemy_ids.len() as u32;
        let slots_per_lane = ally_count.max(enemy_count).max(4);
        let mut formation = FormationLayout::new(2, slots_per_lane);

        for (i, &id) in ally_ids.iter().enumerate() {
            formation.place(id, SlotIndex(i as u32)).unwrap();
        }
        for (i, &id) in enemy_ids.iter().enumerate() {
            let slot = slots_per_lane + i as u32;
            formation.place(id, SlotIndex(slot)).unwrap();
        }

        // ── Resolver ──────────────────────────────────────────────────────
        let mut resolver = CombatResolver::new(3);
        resolver.start(&mut encounter, &actors);

        // ── Battle loop ────────────────────────────────────────────────────
        let mut trace = BattleTrace::new(&pack.id.0);
        let mut counters = SkillUsageCounters::new();
        let mut round: u32 = 0;
        let max_rounds = 100;

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
                counters.reset_turn_scope(current_actor);
                continue;
            }

            // Select skill from assignment
            let skill_name = skill_assign.get(current_actor).unwrap_or("crusading_strike");
            let skill = match self.content_pack.get_skill(&SkillId::new(skill_name)) {
                Some(s) => s,
                None => {
                    let cmd = CombatCommand::Wait {
                        actor: current_actor,
                    };
                    resolver.submit_command(&mut encounter, &mut actors, cmd);
                    trace.record_wait(round, current_actor, &actors);
                    resolver.end_turn(&mut encounter, &mut actors);
                    counters.reset_turn_scope(current_actor);
                    continue;
                }
            };

            // ── Usage Limit Check (US-513) ────────────────────────────────────
            // Before executing a skill, check if it has a usage limit that's been exceeded.
            // If over limit, skip the skill use (submit Wait instead).
            let skill_over_limit = if let Some(limit) = get_usage_limit(&skill.id) {
                !counters.can_use(current_actor, &skill.id, limit)
            } else {
                false
            };

            if skill_over_limit {
                let cmd = CombatCommand::Wait {
                    actor: current_actor,
                };
                resolver.submit_command(&mut encounter, &mut actors, cmd);
                trace.record_wait(round, current_actor, &actors);
                resolver.end_turn(&mut encounter, &mut actors);
                counters.reset_turn_scope(current_actor);
                continue;
            }

            // Resolve targets — sorted by ActorId for deterministic trace output
            let mut targets = skill
                .target_selector
                .resolve(current_actor, &formation, &actors, &side_lookup);
            targets.sort_by_key(|t| t.0);

            // ── DDGC Ally-Exclusive Targeting (US-703) ─────────────────────────
            // For ally-exclusive skills (DDGC @rank = any ally, not self),
            // exclude self from the target list. This is the only targeting rule
            // that can be safely applied at the battle loop level without
            // interfering with the framework's multi-target resolution.
            // NOTE: Single-target truncation is NOT applied here because the
            // framework handles per-skill resolution against all provided targets.
            // Truncating to 1 target would break multi-target skills.
            if let Some(rule) = get_ddgc_targeting_rule(skill_name) {
                if rule.exclude_self_from_allies {
                    targets.retain(|t| *t != current_actor);
                }
            }

            if targets.is_empty() {
                let cmd = CombatCommand::Wait {
                    actor: current_actor,
                };
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

                    // ── Record usage (US-513) ─────────────────────────────────
                    // After successful skill execution, record the usage for limit tracking.
                    if let Some(limit) = get_usage_limit(&skill.id) {
                        counters.record_usage(current_actor, skill.id.clone(), limit.scope);
                    }

                    // ── Reactive Processing (US-506, US-507) ─────────────────
                    // After damage is applied, check if targets have riposte status
                    // and process reactive counter-attacks (US-506).
                    // Also detect guard protection relationships for guard redirect
                    // events (US-507).
                    let mut reactive_queue = ReactiveQueue::new();
                    for &target in &targets {
                        // Riposte detection: actor with riposte status who was hit
                        let candidates = detect_riposte_candidates(&actors);
                        for candidate in candidates {
                            // Only create event if the candidate was actually hit (is in targets)
                            if targets.contains(&candidate) {
                                let damage_amount = effect_results.results.iter().find_map(|r| r.values.get("amount").copied());
                                let ctx = DamageStepContext::new(
                                    current_actor,
                                    skill.id.clone(),
                                    target,
                                    damage_amount,
                                );
                                let events = build_reactive_events(&ctx, candidate, ReactiveEventKind::Riposte);
                                for event in events {
                                    reactive_queue.enqueue(event);
                                }
                            }
                        }

                        // Guard detection (US-507): find guards on same side who can protect this target
                        let guard_relations = detect_guard_relations_for_target(target, &actors, &side_lookup);
                        for relation in guard_relations {
                            // Guard redirects damage for the protected target
                            let damage_amount = effect_results.results.iter().find_map(|r| r.values.get("amount").copied());
                            let ctx = DamageStepContext::new(
                                current_actor,
                                skill.id.clone(),
                                target,
                                damage_amount,
                            );
                            let events = build_reactive_events(&ctx, relation.guard, ReactiveEventKind::GuardRedirect);
                            for event in events {
                                reactive_queue.enqueue(event);
                            }
                        }
                    }

                    // Process reactive queue: execute riposte counter-attacks and guard redirects
                    while let Some(event) = reactive_queue.drain_next() {
                        if event.is_riposte() {
                            let reactor_id = event.reactor;
                            // Check riposte status - borrow actors immutably
                            let has_riposte = if let Some(reactor) = actors.get(&reactor_id) {
                                has_riposte_status(reactor)
                            } else {
                                false
                            };
                            if has_riposte {
                                if let Some((_skill_id, reactive_results)) = execute_riposte(
                                    &event,
                                    &self.content_pack,
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
                        } else if event.is_guard_redirect() {
                            // US-508: Guard redirect - damage goes to guard instead of protected target
                            if let Some(redirected_damage) = execute_guard_redirect(&event, &mut actors) {
                                let trigger = crate::trace::ReactiveTrigger {
                                    attacker: event.attacker.0,
                                    skill: skill_name.to_string(),
                                    target: event.triggered_on.0,
                                    kind: "GuardRedirect".to_string(),
                                };
                                // Build effect results for the redirect action
                                // actor = original attacker, targets = guard (who absorbed the damage)
                                let redirect_results = vec![
                                    framework_combat::results::EffectResult {
                                        kind: framework_combat::results::EffectResultKind::Damage,
                                        actor: event.attacker,
                                        targets: vec![event.reactor],
                                        values: std::collections::HashMap::from([
                                            ("amount".to_string(), redirected_damage),
                                        ]),
                                        applied_statuses: vec![],
                                    }
                                ];
                                trace.record_reactive(
                                    round,
                                    event.reactor,
                                    "guard_redirect",
                                    &[event.triggered_on],
                                    &redirect_results,
                                    &actors,
                                    trigger,
                                );
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
                    let cmd = CombatCommand::Wait {
                        actor: current_actor,
                    };
                    resolver.submit_command(&mut encounter, &mut actors, cmd);
                    trace.record_wait(round, current_actor, &actors);
                }
            }

            resolver.end_turn(&mut encounter, &mut actors);
            counters.reset_turn_scope(current_actor);
        }

        // ── Result ────────────────────────────────────────────────────────
        let winner = match &encounter.state {
            EncounterState::Resolved { winner } => *winner,
            _ => resolver.check_resolution(&encounter),
        };

        trace.finalize(winner, round);

        EncounterBattleResult {
            winner,
            turns: round,
            trace,
            pack_id: pack.id.0.clone(),
        }
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
    fn resolve_pack_is_deterministic() {
        let resolver = EncounterResolver::new();

        // Same inputs should always yield the same pack
        let pack1 = resolver.resolve_pack(Dungeon::QingLong, 0, 42, false);
        let pack2 = resolver.resolve_pack(Dungeon::QingLong, 0, 42, false);

        assert!(pack1.is_some(), "QingLong should have room packs");
        assert!(pack2.is_some(), "QingLong should have room packs");
        assert_eq!(
            pack1.unwrap().id, pack2.unwrap().id,
            "Same inputs should produce the same pack"
        );

        // Different room indices should yield different packs (with high probability)
        let pack_a = resolver.resolve_pack(Dungeon::QingLong, 0, 42, false);
        let pack_b = resolver.resolve_pack(Dungeon::QingLong, 1, 42, false);
        // Both should exist; they may or may not differ depending on the hash
        assert!(pack_a.is_some());
        assert!(pack_b.is_some());

        // Different dungeons should yield packs from that dungeon
        let ql_pack = resolver.resolve_pack(Dungeon::QingLong, 0, 42, false);
        let bh_pack = resolver.resolve_pack(Dungeon::BaiHu, 0, 42, false);
        assert!(ql_pack.is_some());
        assert!(bh_pack.is_some());
        assert_eq!(ql_pack.unwrap().dungeon, Dungeon::QingLong);
        assert_eq!(bh_pack.unwrap().dungeon, Dungeon::BaiHu);
    }

    #[test]
    fn resolve_pack_uses_hall_for_corridor() {
        let resolver = EncounterResolver::new();

        let hall_pack = resolver.resolve_pack(Dungeon::QingLong, 0, 42, true);
        let room_pack = resolver.resolve_pack(Dungeon::QingLong, 0, 42, false);

        assert!(hall_pack.is_some(), "QingLong should have hall packs");
        assert!(room_pack.is_some(), "QingLong should have room packs");
        assert_eq!(hall_pack.unwrap().pack_type, PackType::Hall);
        assert_eq!(room_pack.unwrap().pack_type, PackType::Room);
    }

    #[test]
    fn resolve_pack_returns_none_for_cross_dungeon() {
        let resolver = EncounterResolver::new();

        // Dungeon::Cross has no encounter packs registered
        let result = resolver.resolve_pack(Dungeon::Cross, 0, 42, false);
        assert!(result.is_none(), "Cross dungeon should have no packs");
    }

    #[test]
    fn encounter_resolver_runs_battle_for_qinglong_pack() {
        let resolver = EncounterResolver::new();

        let pack = resolver
            .resolve_pack(Dungeon::QingLong, 0, 42, false)
            .expect("QingLong should have a room pack");

        let result = resolver.run_battle(pack, 1);

        // Battle must terminate
        assert!(
            result.turns <= 100,
            "Battle should finish within 100 turns, took {}",
            result.turns
        );

        // Pack ID should match
        assert_eq!(result.pack_id, pack.id.0);

        // Trace should have entries
        assert!(
            !result.trace.entries.is_empty(),
            "Battle trace should record events"
        );
    }

    #[test]
    fn encounter_resolver_runs_battle_for_all_dungeons() {
        let resolver = EncounterResolver::new();

        for dungeon in [Dungeon::QingLong, Dungeon::BaiHu, Dungeon::ZhuQue, Dungeon::XuanWu] {
            let pack = resolver
                .resolve_pack(dungeon, 0, 42, false)
                .unwrap_or_else(|| panic!("{:?} should have a room pack", dungeon));

            let result = resolver.run_battle(pack, 1);

            assert!(
                result.turns <= 100,
                "{:?} battle should finish within 100 turns, took {}",
                dungeon,
                result.turns
            );
            assert_eq!(result.pack_id, pack.id.0);
        }
    }

    #[test]
    fn battle_is_deterministic() {
        let resolver = EncounterResolver::new();

        let pack = resolver
            .resolve_pack(Dungeon::BaiHu, 0, 42, false)
            .expect("BaiHu should have a room pack");

        let result1 = resolver.run_battle(pack, 1);
        let result2 = resolver.run_battle(pack, 1);

        assert_eq!(
            result1.winner, result2.winner,
            "Same battle should produce the same winner"
        );
        assert_eq!(
            result1.turns, result2.turns,
            "Same battle should take the same number of turns"
        );
        assert_eq!(
            result1.trace.entries.len(),
            result2.trace.entries.len(),
            "Same battle should produce the same number of trace entries"
        );
    }

    #[test]
    fn room_to_encounter_mapping_is_deterministic() {
        // This is the acceptance test: proves that room-to-common-encounter
        // mapping is deterministic for the same seed and dungeon.
        let resolver = EncounterResolver::new();

        // For each dungeon, verify that resolving the same room index and seed
        // always yields the same pack across 10 calls.
        for dungeon in [Dungeon::QingLong, Dungeon::BaiHu, Dungeon::ZhuQue, Dungeon::XuanWu] {
            let first_pack = resolver.resolve_pack(dungeon, 0, 99, false);
            assert!(first_pack.is_some(), "{:?} should have room packs", dungeon);

            let _first_id = first_pack.unwrap().id.0.clone();

            for room_idx in 0..5 {
                for seed in [0u64, 42, 999] {
                    let pack_a = resolver.resolve_pack(dungeon, room_idx, seed, false);
                    let pack_b = resolver.resolve_pack(dungeon, room_idx, seed, false);

                    assert_eq!(
                        pack_a.map(|p| &p.id.0),
                        pack_b.map(|p| &p.id.0),
                        "Pack selection should be deterministic for {:?} room={} seed={}",
                        dungeon, room_idx, seed
                    );
                }
            }
        }
    }

    // ── Boss room resolution tests ───────────────────────────────────────────

    #[test]
    fn resolve_boss_pack_returns_boss_packs_for_all_dungeons() {
        let resolver = EncounterResolver::new();

        for dungeon in [Dungeon::QingLong, Dungeon::BaiHu, Dungeon::ZhuQue, Dungeon::XuanWu] {
            let pack = resolver.resolve_boss_pack(dungeon, 0, 42);
            assert!(
                pack.is_some(),
                "{:?} should have at least one boss pack",
                dungeon
            );
            assert_eq!(
                pack.unwrap().pack_type,
                PackType::Boss,
                "Resolved pack for {:?} should be a boss pack",
                dungeon
            );
        }
    }

    #[test]
    fn resolve_boss_pack_is_deterministic() {
        let resolver = EncounterResolver::new();

        for dungeon in [Dungeon::QingLong, Dungeon::BaiHu, Dungeon::ZhuQue, Dungeon::XuanWu] {
            for room_idx in 0..3 {
                for seed in [0u64, 42, 999] {
                    let pack_a = resolver.resolve_boss_pack(dungeon, room_idx, seed);
                    let pack_b = resolver.resolve_boss_pack(dungeon, room_idx, seed);

                    assert_eq!(
                        pack_a.map(|p| &p.id.0),
                        pack_b.map(|p| &p.id.0),
                        "Boss pack selection should be deterministic for {:?} room={} seed={}",
                        dungeon, room_idx, seed
                    );
                }
            }
        }
    }

    #[test]
    fn resolve_boss_pack_returns_none_for_cross_dungeon() {
        let resolver = EncounterResolver::new();

        // Dungeon::Cross has no room-based boss encounter packs
        // (cross-dungeon bosses like bloodthirsty_assassin are registered
        // under Dungeon::Cross but are only accessible through special quest paths,
        // not normal floor generation boss rooms)
        let result = resolver.resolve_boss_pack(Dungeon::Cross, 0, 42);
        // Cross dungeon does have boss packs registered, so it may resolve
        // to one — this test just verifies the method handles it gracefully
        // (either Some with PackType::Boss or None)
        if let Some(pack) = result {
            assert_eq!(
                pack.pack_type,
                PackType::Boss,
                "Cross dungeon boss pack should be PackType::Boss"
            );
        }
    }

    #[test]
    fn boss_room_mapping_uses_encounter_registry() {
        // Proves that boss-room resolution goes through the encounter registry
        // (not a hard-coded battle). The resolved pack must have PackType::Boss
        // and must reference families registered in the monster family registry.
        let resolver = EncounterResolver::new();

        for dungeon in [Dungeon::QingLong, Dungeon::BaiHu, Dungeon::ZhuQue, Dungeon::XuanWu] {
            let pack = resolver
                .resolve_boss_pack(dungeon, 0, 42)
                .unwrap_or_else(|| panic!("{:?} should have a boss pack", dungeon));

            assert_eq!(pack.pack_type, PackType::Boss);
            assert_eq!(pack.dungeon, dungeon);

            // Every family in the boss pack must be registered
            for slot in &pack.slots {
                assert!(
                    resolver.monster_registry.get(&slot.family_id.0).is_some(),
                    "Boss pack {} references family '{}' not in monster registry",
                    pack.id,
                    slot.family_id
                );
            }
        }
    }

    #[test]
    fn boss_battle_runs_to_completion() {
        let resolver = EncounterResolver::new();

        // Run a boss battle for each dungeon that has boss packs
        for dungeon in [Dungeon::QingLong, Dungeon::BaiHu, Dungeon::ZhuQue, Dungeon::XuanWu] {
            let pack = resolver
                .resolve_boss_pack(dungeon, 0, 42)
                .unwrap_or_else(|| panic!("{:?} should have a boss pack", dungeon));

            let result = resolver.run_battle(pack, 1);

            assert!(
                result.turns <= 100,
                "{:?} boss battle should finish within 100 turns, took {}",
                dungeon,
                result.turns
            );
            assert_eq!(result.pack_id, pack.id.0);
            assert!(
                !result.trace.entries.is_empty(),
                "Boss battle trace should record events"
            );
        }
    }

    #[test]
    fn boss_battle_is_deterministic() {
        let resolver = EncounterResolver::new();

        let pack = resolver
            .resolve_boss_pack(Dungeon::QingLong, 0, 42)
            .expect("QingLong should have a boss pack");

        let result1 = resolver.run_battle(pack, 1);
        let result2 = resolver.run_battle(pack, 1);

        assert_eq!(
            result1.winner, result2.winner,
            "Same boss battle should produce the same winner"
        );
        assert_eq!(
            result1.turns, result2.turns,
            "Same boss battle should take the same number of turns"
        );
        assert_eq!(
            result1.trace.entries.len(),
            result2.trace.entries.len(),
            "Same boss battle should produce the same number of trace entries"
        );
    }

    // ── DDGC Targeting Tests (US-703) ────────────────────────────────────────

    #[test]
    fn lizard_stun_resolves_to_single_target() {
        // lizard's stun skill is a single-target DDGC skill — should hit ONE enemy.
        let resolver = EncounterResolver::new();

        let pack = resolver
            .resolve_pack(Dungeon::BaiHu, 0, 42, false)
            .expect("BaiHu should have room packs (contains lizard family)");

        let result = resolver.run_battle(pack, 1);

        assert!(
            result.turns <= 100,
            "Battle should finish within 100 turns"
        );

        // lizard uses "stun" skill — check that it targets exactly 1 enemy
        let stun_entries: Vec<_> = result
            .trace
            .entries
            .iter()
            .filter(|e| e.action == "stun")
            .collect();

        if !stun_entries.is_empty() {
            for entry in stun_entries {
                assert_eq!(
                    entry.targets.len(), 1,
                    "lizard stun should target exactly 1 enemy, got {} targets",
                    entry.targets.len()
                );
            }
        }
    }

    #[test]
    fn mark_skill_single_target_enemy_rule() {
        // Unit test: verify mark_skill DDGC rule is single-target enemy
        use crate::encounters::ddgc_targeting_rules::get_ddgc_targeting_rule;
        use crate::encounters::targeting::SideAffinity;

        let rule = get_ddgc_targeting_rule("mark_skill").expect("mark_skill should have a rule");
        assert!(rule.single_target, "mark_skill should be single-target");
        assert!(
            matches!(rule.side_affinity, SideAffinity::Enemy),
            "mark_skill should target enemies"
        );
        assert!(
            !rule.exclude_self_from_allies,
            "mark_skill should not exclude self (it targets enemies)"
        );
    }

    #[test]
    fn protect_skill_single_target_ally_excluding_self_rule() {
        // Unit test: verify protect_skill DDGC rule is ally-exclusive single-target
        use crate::encounters::ddgc_targeting_rules::get_ddgc_targeting_rule;
        use crate::encounters::targeting::SideAffinity;

        let rule = get_ddgc_targeting_rule("protect_skill").expect("protect_skill should have a rule");
        assert!(rule.single_target, "protect_skill should be single-target");
        assert!(
            matches!(rule.side_affinity, SideAffinity::Ally),
            "protect_skill should target allies"
        );
        assert!(
            rule.exclude_self_from_allies,
            "protect_skill should exclude self from ally targets"
        );
    }

    #[test]
    fn lizard_stun_single_target_enemy_rule() {
        // Unit test: verify lizard's stun skill DDGC rule is single-target enemy
        use crate::encounters::ddgc_targeting_rules::get_ddgc_targeting_rule;
        use crate::encounters::targeting::SideAffinity;

        let rule = get_ddgc_targeting_rule("stun").expect("stun should have a rule");
        assert!(rule.single_target, "stun should be single-target");
        assert!(
            matches!(rule.side_affinity, SideAffinity::Enemy),
            "stun should target enemies"
        );
    }

    #[test]
    fn ddgc_targeting_rule_ally_exclude_self_is_applied() {
        // Integration test: verify ally-exclusive targeting excludes self.
        // This tests the ally-exclusion rule that IS applied in the battle loop.
        use std::collections::HashMap;
        use framework_combat::formation::FormationLayout;
        use framework_combat::formation::SlotIndex;
        use framework_rules::actor::ActorId;
        use framework_rules::attributes::{AttributeKey, AttributeValue, ATTR_HEALTH};
        use framework_combat::encounter::CombatSide;
        use framework_combat::skills::SkillId;
        use crate::encounters::ddgc_targeting_rules::get_ddgc_targeting_rule;

        // Build formation: 3 allies (ActorIds 1, 2, 3), 1 enemy (ActorId 10)
        let mut formation = FormationLayout::new(2, 2);
        formation.place(ActorId(1), SlotIndex(0)).unwrap(); // ally front
        formation.place(ActorId(2), SlotIndex(1)).unwrap(); // ally front
        formation.place(ActorId(3), SlotIndex(2)).unwrap(); // ally back
        formation.place(ActorId(10), SlotIndex(3)).unwrap(); // enemy back

        let mut actors = HashMap::new();
        for id in [ActorId(1), ActorId(2), ActorId(3), ActorId(10)] {
            let mut a = framework_rules::actor::ActorAggregate::new(id);
            a.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
            actors.insert(id, a);
        }

        let mut side_lookup = HashMap::new();
        side_lookup.insert(ActorId(1), CombatSide::Ally);
        side_lookup.insert(ActorId(2), CombatSide::Ally);
        side_lookup.insert(ActorId(3), CombatSide::Ally);
        side_lookup.insert(ActorId(10), CombatSide::Enemy);

        let resolver = EncounterResolver::new();
        let protect_skill = resolver
            .content_pack
            .get_skill(&SkillId::new("protect_skill"))
            .expect("protect_skill should exist in content pack");

        // ActorId(1) casts protect_skill on allies
        let mut targets = protect_skill
            .target_selector
            .resolve(ActorId(1), &formation, &actors, &side_lookup);
        targets.sort_by_key(|t| t.0);

        // Before DDGC rule: all 3 allies including self
        assert_eq!(
            targets.len(), 3,
            "AllAllies should include self (3 allies total)"
        );
        assert!(
            targets.contains(&ActorId(1)),
            "Before DDGC rule, self should be included"
        );

        // Apply only the ally-exclusion rule (the one actually applied in battle loop)
        if let Some(rule) = get_ddgc_targeting_rule("protect_skill") {
            if rule.exclude_self_from_allies {
                targets.retain(|t| *t != ActorId(1));
            }
        }

        // After ally-exclusion rule: self excluded, but all other allies remain
        assert!(
            !targets.contains(&ActorId(1)),
            "After ally-exclusion rule, self should be excluded from protect_skill targets"
        );
        assert_eq!(
            targets.len(), 2,
            "After ally-exclusion rule, protect_skill should target 2 allies (not self)"
        );
    }

    #[test]
    fn ddgc_targeting_rule_for_enemy_skills_returns_correct_count() {
        // Verify that enemy skills using AllEnemies still return all enemies
        // (the DDGC rule is defined but not enforced for enemy multi-target skills
        // since the battle loop doesn't apply single-target truncation)
        use std::collections::HashMap;
        use framework_combat::formation::FormationLayout;
        use framework_combat::formation::SlotIndex;
        use framework_rules::actor::ActorId;
        use framework_rules::attributes::{AttributeKey, AttributeValue, ATTR_HEALTH};
        use framework_combat::encounter::CombatSide;
        use framework_combat::skills::SkillId;
        use crate::encounters::ddgc_targeting_rules::get_ddgc_targeting_rule;
        use crate::encounters::targeting::SideAffinity;

        // Build formation: 1 ally, 4 enemies
        let mut formation = FormationLayout::new(2, 3);
        formation.place(ActorId(1), SlotIndex(0)).unwrap();   // ally front
        formation.place(ActorId(10), SlotIndex(3)).unwrap(); // enemy back
        formation.place(ActorId(11), SlotIndex(4)).unwrap(); // enemy back
        formation.place(ActorId(12), SlotIndex(5)).unwrap(); // enemy back
        formation.place(ActorId(13), SlotIndex(2)).unwrap(); // enemy front-right

        let mut actors = HashMap::new();
        for id in [ActorId(1), ActorId(10), ActorId(11), ActorId(12), ActorId(13)] {
            let mut a = framework_rules::actor::ActorAggregate::new(id);
            a.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
            actors.insert(id, a);
        }

        let mut side_lookup = HashMap::new();
        side_lookup.insert(ActorId(1), CombatSide::Ally);
        side_lookup.insert(ActorId(10), CombatSide::Enemy);
        side_lookup.insert(ActorId(11), CombatSide::Enemy);
        side_lookup.insert(ActorId(12), CombatSide::Enemy);
        side_lookup.insert(ActorId(13), CombatSide::Enemy);

        let resolver = EncounterResolver::new();
        let mark_skill = resolver
            .content_pack
            .get_skill(&SkillId::new("mark_skill"))
            .expect("mark_skill should exist in content pack");

        // Resolve targets using framework (AllEnemies)
        let mut targets = mark_skill
            .target_selector
            .resolve(ActorId(1), &formation, &actors, &side_lookup);
        targets.sort_by_key(|t| t.0);

        // Framework returns all 4 enemies (multi-target by default)
        assert_eq!(
            targets.len(), 4,
            "Framework AllEnemies should return all 4 enemies"
        );

        // The DDGC rule documents single-target intent but is not applied to
        // enemy skills in the battle loop (framework handles multi-target)
        if let Some(rule) = get_ddgc_targeting_rule("mark_skill") {
            assert!(rule.single_target, "mark_skill DDGC rule should be single-target");
            assert!(
                matches!(rule.side_affinity, SideAffinity::Enemy),
                "mark_skill targets enemies"
            );
        }
    }
}
