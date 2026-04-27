# DDGC Headless Migration

A real game project that migrates DDGC (Dream Developer Game Crossover) content onto the turn-based roguelike framework, proving the framework can support production gameplay without modifications.

## Purpose

This project is the DDGC migration container. Unlike the framework's consumer example (which demonstrates API usage), this project carries real DDGC game content — actors, skills, statuses, and encounters — migrated from the original DDGC codebase onto the framework's public APIs.

## Migration Principles

1. **Framework crates receive no DDGC-specific content** — all game data lives in this project
2. **Only true generic gaps are patched** in core or framework, with regression tests
3. **Every migration blocker is classified** as core-gap, framework-gap, or game-gap
4. **The game runs headless** — no UI runtime, deterministic traces for regression

## Local Developer Startup

### Prerequisites

- Rust toolchain (stable, 1.70+)
- `data/` directory with contract JSON/CSV files (Curios.csv, Traps.json, Buildings.json, etc.)

### Running the Application

```bash
# Typecheck
cargo check

# Run tests
cargo test

# Run the binary (headless encounter/run slice)
cargo run

# Run with logging
RUST_LOG=debug cargo run
```

### Using the Frontend Host

The `DdgcHost` in `src/contracts/host.rs` provides the canonical application host for starting the game in either replay-driven or live-runtime mode:

```rust
use game_ddgc_headless::contracts::host::{DdgcHost, LiveConfig};

// Boot in live-runtime mode
let host = DdgcHost::boot_live(&LiveConfig::default()).expect("boot failed");
assert!(host.is_ready());

// Or boot from a saved campaign state (replay-driven)
let host = DdgcHost::boot_from_campaign(&ReplayConfig {
    campaign_json: &saved_json,
    source_path: "savegame.json",
}).expect("replay failed");
```

### Error Handling

The host uses explicit error types rather than silent fallbacks:

```rust
use game_ddgc_headless::contracts::host::DdgcHost;

let result = DdgcHost::boot_live(&LiveConfig::default());
if let Err(e) = result {
    eprintln!("Boot error: {}", e.error_message());
}
```

## Verification Commands

## Project Structure

```
games/game_ddgc_headless/
├── Cargo.toml
├── README.md
├── MIGRATION_MAP.md       # Systems mapped for migration
├── MIGRATION_BLOCKERS.md  # Blocker ledger (core/framework/game-gap labels)
├── SEMANTIC_GAPS.md       # Resolved semantic gap ledger (SG-001 through SG-004)
├── SEMANTIC_GAP_MATRIX.md # Remaining gap matrix (SM-001 through SM-019)
├── SEMANTIC_PARITY.md     # Parity vocabulary and definitions
├── src/
│   ├── lib.rs             # Library exports
│   ├── main.rs            # Binary entry point
│   └── content/           # Migrated DDGC content (actors, skills, statuses)
└── tests/                 # Integration and migration validation tests
```

## Framework Crates Used

| Crate | Purpose |
|---|---|
| `framework_rules` | Actor aggregates, attributes, modifiers, statuses |
| `framework_combat` | Encounters, combat resolution, skills, effects, formation |
| `framework_progression` | Runs, floors, rooms, floor generation |
| `framework_viewmodels` | Read-only view models for combat and run state |
| `framework_ai` | Desire-based AI decision framework |
