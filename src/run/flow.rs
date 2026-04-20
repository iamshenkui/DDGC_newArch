//! DDGC run flow — room-by-room dungeon progression.
//!
//! Generates a floor using `DefaultRoomGenerator` with DDGC room weights,
//! then drives the run through each room in sequence. Combat rooms and
//! boss rooms resolve through the DDGC encounter pack registry.
//! Post-battle rewards are applied after clearing combat rooms.
//!
//! This is the Batch 5 migration: the new stack proves it can handle
//! gameplay progression rather than a single isolated encounter.

use framework_combat::encounter::CombatSide;
use framework_progression::floor::{Floor, FloorId};
use framework_progression::generation::{DefaultRoomGenerator, FloorConfig, RoomGenerator};
use framework_progression::rooms::{RoomId, RoomKind};
use framework_progression::run::{Run, RunId, RunResult};

use crate::contracts::{get_dungeon_config, DungeonType, MapSize};
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
    pub dungeon: Dungeon,
    pub map_size: MapSize,
}

impl Default for DdgcRunConfig {
    fn default() -> Self {
        DdgcRunConfig {
            seed: 42,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
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

impl Default for DdgcRunState {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of completing a DDGC run slice.
pub struct DdgcRunResult {
    pub run: Run,
    pub state: DdgcRunState,
    pub floor: Floor,
    /// Pack IDs for battles in this run slice — used to verify no fallback content.
    pub battle_pack_ids: Vec<String>,
}

/// Run a minimal DDGC run slice.
///
/// Generates a deterministic floor with DDGC room weights, then progresses
/// through each room in sequence. Combat rooms resolve through the encounter
/// pack registry; Boss rooms resolve through the boss encounter registry.
/// Post-battle rewards are applied after each combat room is cleared.
///
/// All four core DDGC dungeons (QingLong, BaiHu, ZhuQue, XuanWu) have migrated
/// encounter packs — this function will panic if a pack is missing, indicating
/// a developer error in the migration.
///
/// Room generation uses `DungeonMapConfig` parameters for the current dungeon:
/// room count comes from `base_room_number`, and connectivity drives `max_connections`.
pub fn run_ddgc_slice(config: &DdgcRunConfig) -> DdgcRunResult {
    // Look up the DungeonMapConfig for this dungeon type and size
    let dungeon_type = DungeonType::from_dungeon(config.dungeon)
        .expect("DungeonMapConfig required for core dungeons (QingLong/BaiHu/ZhuQue/XuanWu)");
    let map_config = get_dungeon_config(dungeon_type, config.map_size)
        .expect("DungeonMapConfig must exist for the given dungeon type and size");

    let room_count = map_config.base_room_number;
    let max_connections = map_config.max_connections();

    let gen = DefaultRoomGenerator;
    let floor_config = FloorConfig::new(
        room_count,
        DDGC_ROOM_WEIGHTS.to_vec(),
        max_connections,
    );

    let mut floor = gen.generate_floor(FloorId(0), config.seed, &floor_config);

    let mut run = Run::new(RunId(1), vec![FloorId(0)], config.seed);
    let mut state = DdgcRunState::new();
    let mut battle_pack_ids = Vec::new();

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
                // Resolve combat room through encounter pack registry.
                // All four core DDGC dungeons have migrated encounter packs;
                // if a pack is missing, this is a developer error — fail fast.
                let pack = resolver
                    .resolve_pack(config.dungeon, room_idx as u32, config.seed, false)
                    .expect("Combat room: migrated DDGC dungeon must have encounter packs");
                let battle_result = resolver.run_battle(pack, room_idx as u64 + 1);
                battle_pack_ids.push(battle_result.pack_id.clone());

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
                // Boss rooms resolve through the boss encounter pack registry.
                // All four core DDGC dungeons have migrated boss packs;
                // if a pack is missing, this is a developer error — fail fast.
                let pack = resolver
                    .resolve_boss_pack(config.dungeon, room_idx as u32, config.seed)
                    .expect("Boss room: migrated DDGC dungeon must have boss encounter packs");
                let battle_result = resolver.run_battle(pack, room_idx as u64 + 1);
                battle_pack_ids.push(battle_result.pack_id.clone());

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
        battle_pack_ids,
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
    use crate::contracts::{QINGLONG_MEDIUM_EXPLORE, QINGLONG_SHORT_EXPLORE};

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
        // Use medium map to get 14 rooms, guaranteeing combat rooms appear
        let config = DdgcRunConfig {
            seed: 99,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Medium,
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

        // No battles should be lost (DDGC-scale heroes should defeat all encounter packs)
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
        let room_count = result.floor.rooms.len() as u32;
        assert_eq!(
            result.state.rooms_cleared,
            room_count,
            "All {} rooms should be cleared",
            room_count
        );

        // Run should have visited all rooms
        assert_eq!(
            result.state.visited_rooms.len(),
            room_count as usize,
            "Should have visited all rooms"
        );

        // At least one battle should have been won (combat-heavy weights guarantee it)
        assert!(
            result.state.battles_won > 0,
            "Should have won at least one battle"
        );
    }

    #[test]
    fn migrated_encounter_content_is_default_path_for_all_dungeons() {
        // Proves that the four core DDGC dungeons resolve combat and boss rooms
        // through the migrated encounter pack registry (not placeholder content).
        // This is the acceptance test for K43: gameplay entrypoints no longer
        // depend on Bone Soldier or Necromancer placeholders.
        use crate::run::encounters::EncounterResolver;
        use crate::encounters::PackType;

        let resolver = EncounterResolver::new();

        for dungeon in [Dungeon::QingLong, Dungeon::BaiHu, Dungeon::ZhuQue, Dungeon::XuanWu] {
            // Combat rooms must resolve through encounter registry
            let combat_pack = resolver.resolve_pack(dungeon, 0, 42, false);
            assert!(
                combat_pack.is_some(),
                "{:?} combat room should resolve through encounter registry",
                dungeon
            );
            let pack = combat_pack.unwrap();
            assert_ne!(
                pack.pack_type,
                PackType::Boss,
                "{:?} combat room should not resolve to boss pack",
                dungeon
            );

            // Boss rooms must resolve through encounter registry
            let boss_pack = resolver.resolve_boss_pack(dungeon, 0, 42);
            assert!(
                boss_pack.is_some(),
                "{:?} boss room should resolve through encounter registry",
                dungeon
            );
            assert_eq!(
                boss_pack.unwrap().pack_type,
                PackType::Boss,
                "{:?} boss room should resolve to boss pack",
                dungeon
            );
        }
    }

    #[test]
    fn run_slice_uses_no_fallback_content() {
        // Proves that a representative run slice completes using migrated DDGC
        // encounter content only — no fallback to first_battle placeholder.
        // This is the acceptance test for US-713: "Remove transitional encounter
        // and run fallbacks."
        let config = DdgcRunConfig {
            seed: 42,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
        };
        let result = run_ddgc_slice(&config);

        // Count battle rooms
        let battle_room_count = result
            .floor
            .rooms
            .iter()
            .filter(|rid| {
                matches!(
                    result.floor.rooms_map[rid].kind,
                    RoomKind::Combat | RoomKind::Boss
                )
            })
            .count();

        // There should be battle rooms (combat-heavy weights + short map has 9 rooms)
        assert!(
            battle_room_count > 0,
            "Expected battle rooms with 9 rooms and combat-heavy weights"
        );

        // Every battle pack_id must be a real pack (not the fallback marker)
        for (i, pack_id) in result.battle_pack_ids.iter().enumerate() {
            assert_ne!(
                pack_id, "fallback_transitional",
                "Battle {} must not use fallback_transitional — should use migrated DDGC content",
                i
            );
        }

        // battle_pack_ids count should match battle room count
        assert_eq!(
            result.battle_pack_ids.len(), battle_room_count,
            "battle_pack_ids count should match number of battle rooms"
        );

        // All battles should be won (DDGC-scale heroes vs migrated encounter packs)
        assert_eq!(
            result.state.battles_won, battle_room_count as u32,
            "All {} battle rooms should be won",
            battle_room_count
        );
        assert_eq!(
            result.state.battles_lost, 0,
            "No battles should be lost"
        );
    }

    // ── US-810-a: DungeonMapConfig wiring tests ──────────────────────────────────

    #[test]
    fn qinglong_maps_have_correct_room_count() {
        // QingLong short config has base_room_number = 9, medium = 14
        let short_config = DdgcRunConfig {
            seed: 42,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Short,
        };
        let short_result = run_ddgc_slice(&short_config);
        assert_eq!(
            short_result.floor.rooms.len() as u32,
            QINGLONG_SHORT_EXPLORE.base_room_number,
            "QingLong short should have {} rooms",
            QINGLONG_SHORT_EXPLORE.base_room_number
        );

        let medium_config = DdgcRunConfig {
            seed: 42,
            dungeon: Dungeon::QingLong,
            map_size: MapSize::Medium,
        };
        let medium_result = run_ddgc_slice(&medium_config);
        assert_eq!(
            medium_result.floor.rooms.len() as u32,
            QINGLONG_MEDIUM_EXPLORE.base_room_number,
            "QingLong medium should have {} rooms",
            QINGLONG_MEDIUM_EXPLORE.base_room_number
        );
    }

    #[test]
    fn baihu_maps_have_lower_connectivity_than_zhuque() {
        // BaiHu connectivity (0.85) < ZhuQue connectivity (0.95)
        // This should produce floors with measurably fewer connections.
        // We use the same seed so the random sequence is identical,
        // only max_connections differs based on dungeon config.
        let baihu_config = DdgcRunConfig {
            seed: 3,
            dungeon: Dungeon::BaiHu,
            map_size: MapSize::Short,
        };
        let zhuque_config = DdgcRunConfig {
            seed: 3,
            dungeon: Dungeon::ZhuQue,
            map_size: MapSize::Short,
        };

        let baihu_result = run_ddgc_slice(&baihu_config);
        let zhuque_result = run_ddgc_slice(&zhuque_config);

        // Compute average connections per room for each floor
        let baihu_avg_conn = avg_connections_per_room(&baihu_result.floor);
        let zhuque_avg_conn = avg_connections_per_room(&zhuque_result.floor);

        assert!(
            baihu_avg_conn < zhuque_avg_conn,
            "BaiHu avg connections ({:.2}) should be less than ZhuQue ({:.2})",
            baihu_avg_conn, zhuque_avg_conn
        );
    }

    #[test]
    fn short_vs_medium_maps_differ_in_room_count() {
        // Short variants have 9 rooms, medium variants have 14
        for dungeon in [Dungeon::QingLong, Dungeon::BaiHu, Dungeon::ZhuQue, Dungeon::XuanWu] {
            let short_config = DdgcRunConfig {
                seed: 42,
                dungeon,
                map_size: MapSize::Short,
            };
            let medium_config = DdgcRunConfig {
                seed: 42,
                dungeon,
                map_size: MapSize::Medium,
            };

            let short_result = run_ddgc_slice(&short_config);
            let medium_result = run_ddgc_slice(&medium_config);

            assert!(
                short_result.floor.rooms.len() < medium_result.floor.rooms.len(),
                "{:?} short ({}) should have fewer rooms than medium ({})",
                dungeon,
                short_result.floor.rooms.len(),
                medium_result.floor.rooms.len()
            );
        }
    }

    /// Compute the average number of connections per room in a floor.
    fn avg_connections_per_room(floor: &Floor) -> f64 {
        if floor.rooms_map.is_empty() {
            return 0.0;
        }
        let total_connections: usize = floor.rooms_map.values().map(|r| r.connections.len()).sum();
        total_connections as f64 / floor.rooms_map.len() as f64
    }
}
