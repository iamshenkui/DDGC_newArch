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

### SG-001: Damage Range Averaging

- **Gap ID:** SG-001
- **Classification:** Acceptable approximation
- **Description:**
  - **Original behavior:** Skills deal `DamageMin..DamageMax` range damage, rolled per use.
  - **Migration behavior:** Skills deal fixed damage equal to the average of min and max.
- **Reason:** First slice uses fixed averages; variance can be restored via game-layer damage roll without changing parity test structure. The approximation is bounded — the fixed value matches the expected value over many rolls — and the restoration path does not require framework changes.
- **Tracking:** MIGRATION_BLOCKERS.md B-006

---

## Status Gaps

### SG-002: BuffRule Condition System — Partial Implementation

- **Gap ID:** SG-002
- **Classification:** Partially resolved
- **Description:**
  - **Original behavior:** `BuffRule` supports 35+ conditional variants (HpBelow, StressAbove, InMode, FirstRound, DeathsDoor, etc.).
  - **Phase 1 migration behavior:** Framework `EffectCondition` covers 4 variants (IfTargetHealthBelow, IfActorHasStatus, IfTargetPosition, Probability). DDGC-specific conditions not yet implemented.
  - **Phase 2 migration behavior:** Game-layer `ConditionAdapter` in `src/run/conditions.rs` provides a unified interface for both framework-native and DDGC-specific conditions.
- **Implemented DDGC conditions (Phase 2):**
  - `FirstRound` — active only on round 0 of combat
  - `StressAbove(threshold)` — actor's stress exceeds threshold (heroes only)
  - `StressBelow(threshold)` — actor's stress below threshold (heroes only)
  - `DeathsDoor` — actor HP below 50% of max
  - `TargetHasStatus(kind)` — target has a specific status active
  - `ActorHasStatus(kind)` — actor has a specific status active
- **Bridged framework-native conditions:**
  - `EffectCondition::Probability` — deterministic pass-through (>0 passes)
  - `EffectCondition::IfTargetHealthBelow` — target HP fraction threshold
  - `EffectCondition::IfActorHasStatus` — actor has a specific status
  - `EffectCondition::IfTargetPosition` — returns `Unknown` (formation context unavailable)
- **Remaining unimplemented DDGC conditions:**
  - Dungeon-specific modes (`InMode`)
  - Position-based conditions beyond `IfTargetPosition`
  - Other DDGC-specific variants not listed above
- **Reason:** `ConditionAdapter` provides a scalable interface for DDGC-specific conditions. Implemented conditions are exercised via `ConditionContext` and `evaluate_by_tag()`. Remaining conditions require future game-layer additions.
- **Tracking:** MIGRATION_BLOCKERS.md B-004 (partially resolved — see B-004 entry for updated status)

### SG-003: Reactive Hooks in Game Layer

- **Gap ID:** SG-003
- **Classification:** Acceptable approximation
- **Description:**
  - **Original behavior:** Riposte and guard are reactive triggers that fire automatically when certain events occur.
  - **Migration behavior:** Riposte and guard are marker statuses detectable by game-layer code; the reactive trigger itself must be implemented in game-layer event handling.
- **Reason:** Riposte/guard are marker statuses detectable by game-layer code; the reactive trigger itself is a game-gap not a semantic gap. The marker pattern preserves the detectability of reactive statuses, and the trigger implementation is a straightforward game-layer addition that does not affect parity test structure.
- **Tracking:** MIGRATION_BLOCKERS.md B-008

---

## Cross-System Gaps

### SG-004: Skill Usage Limits in Game State

- **Gap ID:** SG-004
- **Classification:** Deferred parity work
- **Description:**
  - **Original behavior:** Skills have `LimitPerTurn` and `LimitPerBattle` constraints tracked per-skill.
  - **Migration behavior:** Framework `SkillDefinition` has `cooldown: Option<u32>` but no per-turn or per-battle usage count. Usage tracking does not exist yet.
- **Reason:** LimitPerTurn/LimitPerBattle tracking does not exist yet; once implemented parity tests should verify usage enforcement. This is a known missing feature that requires game-layer state tracking, not a framework limitation.
- **Tracking:** MIGRATION_BLOCKERS.md B-005

---

## Classification Summary

| Gap ID | Classification | Blocker |
|---|---|---|
| SG-001 | Acceptable approximation | B-006 |
| SG-002 | Partially resolved | B-004 |
| SG-003 | Acceptable approximation | B-008 |
| SG-004 | Deferred parity work | B-005 |

**Unacceptable semantic drift:** None. The current migration has no instances of unacceptable semantic drift.

---

## Phase 1 Reactive and Usage-Limit Touchpoints Inventory

This section tracks migrated DDGC content that depends on reactive hooks (riposte, guard) or usage limits, serving as the implementation anchor for Phase 1 work.

### Riposte Touchpoints

Riposte is a reactive trigger: when an actor with the "riposte" status is hit, they automatically execute a counter-attack.

| Content | File | Skills Using Riposte | Status |
|---------|------|---------------------|--------|
| Alligator Yangtze (BaiHu Beast Bruiser) | `content/monsters/alligator_yangtze.rs` | `mark_riposte` (applies `riposte` status), `riposte1` (counter-attack skill) | Migrated; riposte counter-attack implemented (US-506) |
| Frostvein Clam (XuanWu Eldritch Summoner Boss) | `content/monsters/frostvein_clam.rs` | `prismatic_clench`/`riposte1` (applies `riposte` + prot_buff) | Migrated; riposte counter-attack implemented (US-506) |
| Tank hero | `content/heroes/tank.rs` | `active_riposte` (applies `tagged`, NOT `riposte` — game-gap) | Migrated; riposte NOT modeled |
| White hero | `content/heroes/white.rs` | `w_tank_active_riposte` (applies `tagged`, NOT `riposte` — game-gap) | Migrated; riposte NOT modeled |
| Black hero | `content/heroes/black.rs` | `b_tank_active_riposte` (applies `tagged`, NOT `riposte` — game-gap) | Migrated; riposte NOT modeled |
| Status definition | `content/statuses.rs` | `riposte(duration)` — marker status factory | Defined; trigger logic implemented (US-506) |

**Implementation anchor for riposte (Phase 1, B-008):** `alligator_yangtze` is the primary anchor. Its `mark_riposte` skill applies a 3-turn `riposte` status to self; when hit during that window, the actor executes `riposte1` as a counter-attack via the reactive queue. US-506 implemented this. Guard redirect (US-507/US-508) is still pending.

### Guard Touchpoints

Guard is a reactive trigger: when an actor with the "guard" status is protecting an ally, incoming damage is redirected to the guardian.

| Content | File | Skills Using Guard | Status |
|---------|------|---------------------|--------|
| Fox Fire (ZhuQue Beast Bruiser) | `content/monsters/fox_fire.rs` | `protect` (applies `guard` + `defend`) | Migrated; redirect logic NOT yet implemented (B-008) |
| Tank hero | `content/heroes/tank.rs` | `protect_skill` (applies `guard`) | Migrated; redirect logic NOT yet implemented |
| White hero | `content/heroes/white.rs` | `w_protect_skill` (applies `guard` + tank damage bonus) | Migrated; redirect logic NOT yet implemented |
| Black hero | `content/heroes/black.rs` | `b_protect_skill` (applies `guard` + damage reduction + DoT removal) | Migrated; redirect logic NOT yet implemented |
| Status definition | `content/statuses.rs` | `guard(duration)` — marker status factory | Defined; redirect logic NOT yet implemented |

**Implementation anchor for guard (Phase 1, B-008):** `fox_fire` is the primary anchor. Its `protect` skill applies `guard` (and `defend`) to allies; when a guarded ally is attacked, damage should redirect to the fox_fire actor. Tank hero's `protect_skill` provides a hero-side anchor.

### Usage-Limit Touchpoints (Per-Turn and Per-Battle)

DDGC skills can declare `LimitPerTurn` and `LimitPerBattle` constraints. The framework's `SkillDefinition` has `cooldown: Option<u32>` but no per-turn or per-battle usage count.

| Content | File | Limit Type | Status |
|---------|------|-----------|--------|
| None yet migrated | — | — | No migrated content uses usage limits; this is a game-gap (B-005) |

**Usage-limit implementation anchors (Phase 1, B-005):** No concrete migrated anchor exists yet. The inventory above confirms this is a greenfield addition. Future DDGC migration slices (not yet scheduled) will provide concrete per-turn and per-battle skills. The implementation must track usage counts in game-layer state, reset per-turn counts at turn boundaries, and reset per-battle counts at encounter boundaries.

---

## Phase 2 DDGC Condition Support Inventory

This section tracks DDGC condition implementations delivered in Phase 2 (US-601 through US-607), serving as the implementation anchor for B-004.

### Condition Context Architecture

`src/run/conditions.rs` provides:
- `ConditionContext` — read-only context exposing actor, target, turn-state, and encounter-state data
- `DdgcCondition` enum — DDGC-specific condition variants
- `ConditionAdapter` — unified interface bridging framework-native and DDGC-specific conditions
- `ConditionResult` — distinguishes Pass, Fail, and Unknown (unsupported) outcomes

### Implemented DDGC Conditions

| Condition | Tag Format | Implementation | Notes |
|-----------|-----------|---------------|-------|
| `FirstRound` | `ddgc_first_round` | `ConditionContext::is_first_round()` | Active only on round 0 |
| `StressAbove` | `ddgc_stress_above_<threshold>` | `ConditionContext::actor_stress_above()` | Heroes only; monsters always fail |
| `StressBelow` | `ddgc_stress_below_<threshold>` | `ConditionContext::actor_stress_below()` | Heroes only; monsters always fail |
| `DeathsDoor` | `ddgc_deaths_door` | `ConditionContext::actor_at_deaths_door()` | Actor HP < 50% of max |
| `TargetHasStatus` | `ddgc_target_has_status_<kind>` | `ConditionContext::target_has_status()` | Checks first target |
| `ActorHasStatus` | `ddgc_actor_has_status_<kind>` | `ConditionContext::actor_has_status()` | Checks performing actor |

### Bridged Framework-Native Conditions

Framework-native `EffectCondition` variants are evaluated through `ConditionAdapter::evaluate_framework()` with identical behavior to `EffectContext::check_condition`:

| Framework Condition | Adapter Behavior | Notes |
|--------------------|-----------------|-------|
| `Probability(p)` | Pass if `p > 0.0` | Deterministic; real randomness is game-specific |
| `IfTargetHealthBelow(threshold)` | Pass if target HP fraction < threshold | |
| `IfActorHasStatus(kind)` | Pass if actor has active status | |
| `IfTargetPosition(slot_range)` | Returns `Unknown` | Formation context not yet available in `ConditionContext` |

### Unsupported Conditions

The following are known DDGC conditions not yet implemented in `ConditionAdapter`:

| Condition | DDGC Reference | Notes |
|-----------|---------------|-------|
| `InMode` | Dungeon-specific modes | Not yet modeled |
| `HpAbove` | HP above threshold | Not yet implemented |
| `TargetHpAbove` | Target HP above threshold | Not yet implemented |
| `TargetHpBelow` (DDGC variant) | Target HP below threshold | Framework `IfTargetHealthBelow` is bridged |
| Other DDGC-specific variants | `BuffRule` 35+ variants | Only a subset is implemented |

**US-604 guardrail:** Unsupported conditions return `ConditionResult::Unknown`, which is surfaced explicitly rather than silently failing. This ensures missing implementations are observable.

### Implementation Anchors

| Anchor | Type | Description |
|--------|------|-------------|
| `desperate_strike()` | Fixture skill | Demonstrates `DeathsDoor` condition in skill context |
| `stressed_skill()` | Fixture skill | Demonstrates `StressAbove` condition in skill context |
| `first_round_strike()` | Fixture skill | Demonstrates `FirstRound` condition in skill context |

### Remaining Work (B-004)

- Additional DDGC condition families beyond FirstRound, stress thresholds, and DeathsDoor
- `InMode` dungeon-specific conditions
- Full `BuffRule` coverage for remaining DDGC condition variants
- B-006 and B-010 (damage variance and hit resolution) are separate work streams

### Summary: Already-Migrated vs. Future Content

| Mechanic | Already Migrated | Reactive Trigger Implemented | Implementation Status |
|----------|-----------------|----------------------------|----------------------|
| Riposte | Yes (alligator_yangtze, frostvein_clam, heroes) | No | B-008: reactive hooks NOT yet in game layer |
| Guard | Yes (fox_fire, heroes) | No | B-008: redirect logic NOT yet in game layer |
| Per-turn limits | No | N/A | B-005: greenfield — no migrated content uses this |
| Per-battle limits | No | N/A | B-005: greenfield — no migrated content uses this |

**Phase 1 scope:** Reactive hooks (riposte counter-attacks, guard damage redirect) are the primary target. Usage limits are documented for completeness but are greenfield — no migrated DDGC content yet exercises them.
