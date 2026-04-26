# DDGC Headless Migration

A real game project that migrates DDGC (Dream Developer Game Crossover) content onto the turn-based roguelike framework, proving the framework can support production gameplay without modifications.

## Purpose

This project is the DDGC migration container. Unlike the framework's consumer example (which demonstrates API usage), this project carries real DDGC game content — actors, skills, statuses, and encounters — migrated from the original DDGC codebase onto the framework's public APIs.

## Migration Principles

1. **Framework crates receive no DDGC-specific content** — all game data lives in this project
2. **Only true generic gaps are patched** in core or framework, with regression tests
3. **Every migration blocker is classified** as core-gap, framework-gap, or game-gap
4. **The game runs headless** — no UI runtime, deterministic traces for regression

## Verification Commands

```bash
# Typecheck
cargo check --manifest-path games/game_ddgc_headless/Cargo.toml

# Run
cargo run --manifest-path games/game_ddgc_headless/Cargo.toml

# Test
cargo test --manifest-path games/game_ddgc_headless/Cargo.toml
```

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
