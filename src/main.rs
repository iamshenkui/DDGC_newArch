//! DDGC Headless Migration — binary entry point.
//!
//! Runs an encounter-pack battle and a dungeon run slice,
//! emitting structured traces for both. Uses migrated DDGC
//! encounter content (not placeholder Bone Soldier/Necromancer).

use game_ddgc_headless::encounters::Dungeon;
use game_ddgc_headless::run::encounters::EncounterResolver;
use game_ddgc_headless::run::flow::{self, DdgcRunConfig};

fn main() {
    // ── Encounter Battle Slice ────────────────────────────────────────────
    println!("=== DDGC Headless: Encounter Battle Slice ===\n");

    let resolver = EncounterResolver::new();

    // Run a combat room encounter for QingLong dungeon
    let pack = resolver
        .resolve_pack(Dungeon::QingLong, 0, 42, false)
        .expect("QingLong should have room encounter packs");

    let result = resolver.run_battle(pack, 1);

    println!("Encounter pack: {}", result.pack_id);
    println!("{}", result.trace.to_text());

    match result.winner {
        Some(side) => println!("Winner: {:?}", side),
        None => println!("Winner: NONE (battle did not resolve)"),
    }
    println!("Total turns: {}", result.turns);

    // Also emit JSON trace for regression diffing
    let json = result.trace.to_json();
    println!("\n--- JSON Trace ---");
    println!("{}", json);

    // ── Boss Encounter Slice ──────────────────────────────────────────────
    println!("\n=== DDGC Headless: Boss Encounter Slice ===\n");

    let boss_pack = resolver
        .resolve_boss_pack(Dungeon::QingLong, 0, 42)
        .expect("QingLong should have boss encounter packs");

    let boss_result = resolver.run_battle(boss_pack, 2);

    println!("Boss pack: {}", boss_result.pack_id);
    println!("Boss battle turns: {}", boss_result.turns);

    match boss_result.winner {
        Some(side) => println!("Boss winner: {:?}", side),
        None => println!("Boss winner: NONE"),
    }

    // ── Run Slice ─────────────────────────────────────────────────────────
    println!("\n=== DDGC Headless: Run Slice ===\n");

    let config = DdgcRunConfig::default();
    let run_result = flow::run_ddgc_slice(&config);

    println!("Run state: {:?}", run_result.run.state);
    println!("Rooms cleared: {}", run_result.state.rooms_cleared);
    println!("Battles won: {}", run_result.state.battles_won);
    println!("Battles lost: {}", run_result.state.battles_lost);
    println!("Gold earned: {}", run_result.state.gold);
    println!("HP recovered: {:.1}", run_result.state.hp_recovered);
    println!("Stress change: {:.1}", run_result.state.stress_change);

    println!("\nRoom progression:");
    for room_id in &run_result.state.visited_rooms {
        let room = &run_result.floor.rooms_map[room_id];
        println!("  Room {:?} — {:?} ({:?})", room.id, room.kind, room.state);
    }
}
