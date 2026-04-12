//! DDGC Headless Migration — binary entry point.
//!
//! Runs the first migrated DDGC battle and one migrated run slice,
//! emitting structured traces for both.

use game_ddgc_headless::run::flow::{self, DdgcRunConfig};
use game_ddgc_headless::scenarios::first_battle;

fn main() {
    // ── Battle Slice ───────────────────────────────────────────────────────
    println!("=== DDGC Headless: Battle Slice ===\n");

    let result = first_battle::run_first_battle();

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
