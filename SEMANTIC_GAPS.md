# Semantic Gap Ledger

This document tracks every known semantic difference between the original DDGC
game and the headless migration, organized by system. Each gap is classified
using the terminology from `SEMANTIC_PARITY.md` and cross-references the
corresponding blocker in `MIGRATION_BLOCKERS.md`.

---

## Hero Gaps

_No hero-specific semantic gaps identified. Hero archetypes (Crusader, Vestal)
preserve role identity, position preference, and resource semantics as verified
by parity tests._

---

## Monster Gaps

_No monster-specific semantic gaps identified. Monster archetypes (Bone Soldier,
Necromancer) preserve threat model, position logic, and behavioral
differentiation as verified by parity tests._

---

## Skill Gaps

### Damage Range Averaging

- **Original behavior:** Skills deal `DamageMin..DamageMax` range damage, rolled
  per use.
- **Migration behavior:** Skills deal fixed damage equal to the average of min
  and max.
- **Parity impact:** Damage output is deterministic and may differ from any
  single original roll, but matches the expected value over many rolls.
- **Tracking:** See MIGRATION_BLOCKERS.md B-006.

---

## Status Gaps

### BuffRule Condition System Downgrade

- **Original behavior:** `BuffRule` supports 35+ conditional variants
  (HpBelow, StressAbove, InMode, FirstRound, DeathsDoor, etc.).
- **Migration behavior:** Framework `EffectCondition` covers 4 variants
  (IfTargetHealthBelow, IfActorHasStatus, IfTargetPosition, Probability).
  DDGC-specific conditions not yet implemented.
- **Parity impact:** Skills with unimplemented conditions (e.g., StressAbove,
  DeathsDoor) always apply or never apply instead of conditionally applying.
- **Tracking:** See MIGRATION_BLOCKERS.md B-004.

### Reactive Hooks in Game Layer

- **Original behavior:** Riposte and guard are reactive triggers that fire
  automatically when certain events occur.
- **Migration behavior:** Riposte and guard are marker statuses detectable by
  game-layer code; the reactive trigger itself must be implemented in game-layer
  event handling.
- **Parity impact:** Reactive behaviors (counter-attacks, damage redirection)
  do not fire automatically; game-layer code must check for marker statuses
  and apply secondary actions.
- **Tracking:** See MIGRATION_BLOCKERS.md B-008.

---

## Cross-System Gaps

### Skill Usage Limits in Game State

- **Original behavior:** Skills have `LimitPerTurn` and `LimitPerBattle`
  constraints tracked per-skill.
- **Migration behavior:** Framework `SkillDefinition` has `cooldown: Option<u32>`
  but no per-turn or per-battle usage count. Usage tracking does not exist yet.
- **Parity impact:** Skills with usage limits can be used unlimited times per
  turn/battle in the current migration.
- **Tracking:** See MIGRATION_BLOCKERS.md B-005.
