//! DDGC run flow — room-by-room dungeon progression.
//!
//! Generates a floor using `DefaultRoomGenerator` with DDGC room weights,
//! then drives the run through each room in sequence. Combat/Boss rooms
//! trigger a battle (using the first_battle scenario as a combat proxy).
//! Post-battle rewards are applied after clearing combat rooms.
//!
//! This is the Batch 5 migration: the new stack proves it can handle
//! gameplay progression rather than a single isolated encounter.

use framework_combat::encounter::CombatSide;
use framework_progression::floor::{Floor, FloorId};
use framework_progression::generation::{DefaultRoomGenerator, FloorConfig, RoomGenerator};
use framework_progression::rooms::{RoomId, RoomKind};
use framework_progression::run::{Run, RunId, RunResult};

use crate::encounters::Dungeon;
use crate::run::encounters::EncounterResolver;
use crate::run::rewards::{self, PostBattleUpdate};

/// DDGC-appropriate room weights for floor generation.
///
/// Combat-heavy distribution matching DDGC dungeon style:
/// - Combat rooms dominate (weight 5)
/// - Event rooms are secondary (weight 2)
/// - Boss rooms cap the floor (weight 1)
const DDGC_ROOM_WEIGHTS: &[(RoomKind, f64)] = &[
    (RoomKind::Combat, 5.0),
    (RoomKind::Event, 2.0),
    (RoomKind::Boss, 1.0),
];

/// Configuration for a DDGC run slice.
pub struct DdgcRunConfig {
    pub seed: u64,
    pub room_count: u32,
    pub dungeon: Dungeon,
}

impl Default for DdgcRunConfig {
    fn default() -> Self {
        DdgcRunConfig {
            seed: 42,
            room_count: 3,
            dungeon: Dungeon::QingLong,
        }
    }
}

/// Game-layer state tracked across the run.
///
/// The framework manages room/floor progression; this struct holds
/// game-specific state that the framework doesn't know about.
pub struct DdgcRunState {
    pub gold: u32,
    pub rooms_cleared: u32,
    pub battles_won: u32,
    pub battles_lost: u32,
    pub hp_recovered: f64,
    pub stress_change: f64,
    /// Ordered record of room IDs visited (for progression verification).
    pub visited_rooms: Vec<RoomId>,
}

impl DdgcRunState {
    pub fn new() -> Self {
        DdgcRunState {
            gold: 0,
            rooms_cleared: 0,
            battles_won: 0,
            battles_lost: 0,
            hp_recovered: 0.0,
            stress_change: 0.0,
            visited_rooms: Vec::new(),
        }
    }
}

/// Result of completing a DDGC run slice.
pub struct DdgcRunResult {
    pub run: Run,
    pub state: DdgcRunState,
    pub floor: Floor,
}

/// Run a minimal DDGC run slice.
///
/// Generates a deterministic floor with DDGC room weights, then progresses
/// through each room in sequence. Combat rooms resolve through the encounter
/// pack registry; Boss rooms still use a combat proxy (boss packs not wired yet).
/// Post-battle rewards are applied after each combat room is cleared.
pub fn run_ddgc_slice(config: &DdgcRunConfig) -> DdgcRunResult {
    let gen = DefaultRoomGenerator;
    let floor_config = FloorConfig::new(
        config.room_count,
        DDGC_ROOM_WEIGHTS.to_vec(),
        2, // max_connections
    );

    let mut floor = gen.generate_floor(FloorId(0), config.seed, &floor_config);

    let mut run = Run::new(RunId(1), vec![FloorId(0)], config.seed);
    let mut state = DdgcRunState::new();

    // Build the encounter resolver once — reuse for all combat rooms
    let resolver = EncounterResolver::new();

    // Progress through rooms in floor order
    let room_ids = floor.rooms.clone();
    for (room_idx, room_id) in room_ids.iter().enumerate() {
        // Enter the room
        run.enter_room(&mut floor, *room_id).unwrap();
        state.visited_rooms.push(*room_id);

        let room_kind = floor.rooms_map[room_id].kind.clone();

        // Handle room by type
        match &room_kind {
            RoomKind::Combat => {
                // Resolve combat room through encounter pack registry
                let battle_result = match resolver.resolve_pack(
                    config.dungeon,
                    room_idx as u32,
                    config.seed,
                    false, // Combat rooms use room packs
                ) {
                    Some(pack) => resolver.run_battle(pack, room_idx as u64 + 1),
                    None => {
                        // Fallback: no pack found for this dungeon — should not happen
                        // for the four core dungeons but handled gracefully
                        let fallback = crate::scenarios::first_battle::run_first_battle();
                        crate::run::encounters::EncounterBattleResult {
                            winner: fallback.winner,
                            turns: fallback.turns,
                            trace: fallback.trace,
                            pack_id: "fallback".to_string(),
                        }
                    }
                };

                if battle_result.winner == Some(CombatSide::Ally) {
                    state.battles_won += 1;
                } else {
                    state.battles_lost += 1;
                }

                // Clear the room
                run.clear_room(&mut floor).unwrap();

                // Apply post-battle rewards
                let update = rewards::compute_post_battle_update(&room_kind);
                apply_reward(&mut state, &update);
            }
            RoomKind::Boss => {
                // Boss rooms still use the first_battle proxy until boss packs are wired (K30+)
                let battle_result = crate::scenarios::first_battle::run_first_battle();

                if battle_result.winner == Some(CombatSide::Ally) {
                    state.battles_won += 1;
                } else {
                    state.battles_lost += 1;
                }

                // Clear the room
                run.clear_room(&mut floor).unwrap();

                // Apply post-battle rewards
                let update = rewards::compute_post_battle_update(&room_kind);
                apply_reward(&mut state, &update);
            }
            _ => {
                // Event and other rooms: auto-clear (no combat)
                run.clear_room(&mut floor).unwrap();
            }
        }

        state.rooms_cleared += 1;
    }

    // Finish the run
    let run_result = if state.battles_lost > 0 {
        RunResult::Defeat
    } else {
        RunResult::Victory
    };
    run.finish(run_result);

    DdgcRunResult {
        run,
        state,
        floor,
    }
}

/// Apply a post-battle reward to the game-layer run state.
fn apply_reward(state: &mut DdgcRunState, update: &PostBattleUpdate) {
    state.gold += update.gold_earned;
    state.hp_recovered += update.hp_recovered;
    state.stress_change += update.stress_change;
}

#[cfg(test)]
mod tests {
    use super::*;
    use framework_progression::rooms::RoomState;
    use framework_progression::run::RunState;

    #[test]
    fn ddgc_run_slice_progresses_room_by_room() {
        let config = DdgcRunConfig::default();
        let result = run_ddgc_slice(&config);

        // Every room on the floor should be Cleared
        for room_id in &result.floor.rooms {
            assert_eq!(
                result.floor.rooms_map[room_id].state,
                RoomState::Cleared,
                "Room {:?} should be Cleared",
                room_id
            );
        }

        // visited_rooms should match the floor's room order
        assert_eq!(
            result.state.visited_rooms,
            result.floor.rooms,
            "Rooms should be visited in floor order"
        );

        // All rooms should have been cleared
        assert_eq!(
            result.state.rooms_cleared,
            result.floor.rooms.len() as u32,
            "All rooms should be cleared"
        );
    }

    #[test]
    fn ddgc_run_slice_applies_post_battle_updates() {
        // Use enough rooms to guarantee combat rooms appear
        let config = DdgcRunConfig {
            seed: 99,
            room_count: 10,
            dungeon: Dungeon::QingLong,
        };
        let result = run_ddgc_slice(&config);

        // Count combat and boss rooms on the generated floor
        let combat_count = result
            .floor
            .rooms
            .iter()
            .filter(|rid| matches!(result.floor.rooms_map[rid].kind, RoomKind::Combat))
            .count();
        let boss_count = result
            .floor
            .rooms
            .iter()
            .filter(|rid| matches!(result.floor.rooms_map[rid].kind, RoomKind::Boss))
            .count();
        let battle_count = combat_count + boss_count;

        // At least one battle room should exist with 10 rooms and combat-heavy weights
        assert!(
            battle_count > 0,
            "Expected at least one combat or boss room with 10 rooms and combat-heavy weights"
        );

        // Battles won should match combat + boss room count
        assert_eq!(
            result.state.battles_won, battle_count as u32,
            "Should have won one battle per combat/boss room"
        );

        // Gold should match expected rewards
        let expected_gold = (combat_count as u32 * 50) + (boss_count as u32 * 200);
        assert_eq!(
            result.state.gold, expected_gold,
            "Gold should match combat ({}) + boss ({}) rewards",
            combat_count, boss_count
        );

        // HP recovery should match expected
        let expected_hp = (combat_count as f64 * 2.0) + (boss_count as f64 * 10.0);
        assert_eq!(
            result.state.hp_recovered, expected_hp,
            "HP recovery should match combat + boss rewards"
        );

        // Stress change should match expected
        let expected_stress = (combat_count as f64 * -5.0) + (boss_count as f64 * -15.0);
        assert_eq!(
            result.state.stress_change, expected_stress,
            "Stress change should match combat + boss rewards"
        );

        // No battles should be lost (first_battle always results in Ally victory)
        assert_eq!(
            result.state.battles_lost, 0,
            "No battles should be lost in this scenario"
        );
    }

    #[test]
    fn ddgc_run_slice_finishes_with_expected_outcome() {
        let config = DdgcRunConfig::default();
        let result = run_ddgc_slice(&config);

        // Run should be in Victory state (all battles won)
        assert_eq!(
            result.run.state,
            RunState::Victory,
            "Run should end in Victory when all battles are won"
        );

        // All rooms should be cleared
        assert_eq!(
            result.state.rooms_cleared,
            config.room_count,
            "All {} rooms should be cleared",
            config.room_count
        );

        // Run should have visited all rooms
        assert_eq!(
            result.state.visited_rooms.len(),
            config.room_count as usize,
            "Should have visited all rooms"
        );

        // At least one battle should have been won (combat-heavy weights guarantee it)
        assert!(
            result.state.battles_won > 0,
            "Should have won at least one battle"
        );
    }
}
