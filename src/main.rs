//! DDGC Headless Migration — binary entry point.
//!
//! Runs the first migrated DDGC battle and emits a structured trace.

use game_ddgc_headless::scenarios::first_battle;

fn main() {
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
}
