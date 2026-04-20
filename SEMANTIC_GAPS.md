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
- **Classification:** Resolved — Rolled policy available
- **Description:**
  - **Original behavior:** Skills deal `DamageMin..DamageMax` range damage, rolled per use.
  - **Migration behavior:** Skills deal damage resolved by `DamagePolicy`: `FixedAverage` (deterministic average) or `Rolled` (deterministic hash-based roll within range). `DamageRange` is wired into all migrated skill definitions.
- **Reason:** `DamagePolicy` with `FixedAverage` default and `Rolled` variant is implemented (US-609). `DamageRange` structs are registered for all migrated skills in `ContentPack` (US-806-a). Variance is restored via `Rolled` policy without changing parity test structure or requiring framework changes.
- **Tracking:** MIGRATION_BLOCKERS.md B-006

---

## Status Gaps

### SG-002: BuffRule Condition System Downgrade

- **Gap ID:** SG-002
- **Classification:** Resolved
- **Description:**
  - **Original behavior:** `BuffRule` supports 35+ conditional variants (HpBelow, StressAbove, InMode, FirstRound, DeathsDoor, etc.).
  - **Migration behavior:** Framework `EffectCondition` covers 5 variants (IfTargetHealthBelow, IfActorHasStatus, IfTargetPosition, Probability, GameCondition). DDGC-specific conditions are bridged through `ConditionAdapter` with 11 `DdgcCondition` variants. All condition families with migrated content are implemented.
- **Reason:** Phase 2 implemented `ConditionAdapter` with FirstRound, stress-threshold, DeathsDoor, and status-check conditions. Phase 4a added HP-threshold conditions (HpAbove, TargetHpAbove, TargetHpBelow), dungeon-mode condition (InMode), and kill-trigger condition (OnKill). All condition families referenced by migrated content are now implemented. Remaining 20+ DDGC variants are low-impact deferred (no migrated content uses them).
- **Tracking:** MIGRATION_BLOCKERS.md B-004

### SG-003: Reactive Hooks in Game Layer

- **Gap ID:** SG-003
- **Classification:** Acceptable approximation (now resolved)
- **Description:**
  - **Original behavior:** Riposte and guard are reactive triggers that fire automatically when certain events occur.
  - **Migration behavior:** Riposte and guard are marker statuses detectable by game-layer code; the reactive trigger itself must be implemented in game-layer event handling.
- **Reason:** Riposte/guard are marker statuses detectable by game-layer code; the reactive trigger itself is a game-gap not a semantic gap. The marker pattern preserves the detectability of reactive statuses, and the trigger implementation is a straightforward game-layer addition that does not affect parity test structure.
- **Resolution:** Fully implemented — US-506 (riposte counter-attack), US-507 (guard detection), US-508 (guard redirect execution). Reactive hooks are now active in the battle loop.
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
| SG-001 | Resolved — Rolled policy available | B-006 |
| SG-002 | Resolved | B-004 |
| SG-003 | Acceptable approximation (resolved) | B-008 |
| SG-004 | Deferred parity work (resolved) | B-005 |

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

**Implementation anchor for riposte (Phase 1, B-008):** `alligator_yangtze` is the primary anchor. Its `mark_riposte` skill applies a 3-turn `riposte` status to self; when hit during that window, the actor executes `riposte1` as a counter-attack via the reactive queue. US-506 implemented this. Guard redirect (US-507/US-508) is now also implemented — both reactive hooks are fully active.

### Guard Touchpoints

Guard is a reactive trigger: when an actor with the "guard" status is protecting an ally, incoming damage is redirected to the guardian.

| Content | File | Skills Using Guard | Status |
|---------|------|---------------------|--------|
| Fox Fire (ZhuQue Beast Bruiser) | `content/monsters/fox_fire.rs` | `protect` (applies `guard` + `defend`) | Migrated; redirect logic implemented (US-507/US-508) |
| Tank hero | `content/heroes/tank.rs` | `protect_skill` (applies `guard`) | Migrated; redirect logic implemented |
| White hero | `content/heroes/white.rs` | `w_protect_skill` (applies `guard` + tank damage bonus) | Migrated; redirect logic implemented |
| Black hero | `content/heroes/black.rs` | `b_protect_skill` (applies `guard` + damage reduction + DoT removal) | Migrated; redirect logic implemented |
| Status definition | `content/statuses.rs` | `guard(duration)` — marker status factory | Defined; redirect logic implemented (US-507/US-508) |

**Implementation anchor for guard (Phase 1, B-008):** `fox_fire` is the primary anchor. Its `protect` skill applies `guard` (and `defend`) to allies; when a guarded ally is attacked, damage redirects to the fox_fire actor. Tank hero's `protect_skill` provides a hero-side anchor. US-507/US-508 implemented guard detection and redirect execution.

### Usage-Limit Touchpoints (Per-Turn and Per-Battle)

DDGC skills can declare `LimitPerTurn` and `LimitPerBattle` constraints. The framework's `SkillDefinition` has `cooldown: Option<u32>` but no per-turn or per-battle usage count.

| Content | File | Limit Type | Status |
|---------|------|-----------|--------|
| Direct Hit hero skill | `content/heroes/direct_hit.rs` | Per-turn limit: 2 | Implemented — US-513 |
| Duality Fate hero skill | `content/heroes/duality_fate.rs` | Per-battle limit: 1 | Implemented — US-514 |
| SkillUsageCounters | `src/run/usage_counters.rs` | Per-turn, per-battle, per-skill tracking | Implemented — US-510, US-512 |

**Usage-limit implementation (Phase 1, B-005):** Fully implemented — US-510 (SkillUsageCounters), US-512 (reset_battle_scope), US-513 (direct_hit_1 per-turn limit of 2), US-514 (duality_fate per-battle limit of 1), US-515 (regression suite). Usage limits are now active in the battle loop.

### Summary: Already-Migrated vs. Future Content

| Mechanic | Already Migrated | Reactive Trigger Implemented | Implementation Status |
|----------|-----------------|----------------------------|----------------------|
| Riposte | Yes (alligator_yangtze, frostvein_clam, heroes) | Yes | B-008: reactive hooks fully implemented (US-506) |
| Guard | Yes (fox_fire, heroes) | Yes | B-008: redirect logic fully implemented (US-507/US-508) |
| Per-turn limits | Yes (direct_hit hero skill) | N/A | B-005: per-turn limit implementation complete (US-510, US-513) |
| Per-battle limits | Yes (duality_fate hero skill) | N/A | B-005: per-battle limit implementation complete (US-510, US-514) |

**Phase 1 status:** All reactive hooks (riposte counter-attacks, guard damage redirect) and usage limits (per-turn, per-battle) are fully implemented and regression-tested. Phase 1 is closed.

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
| `HpAbove` | `ddgc_hp_above_<threshold>` | `ConditionContext::actor_hp_fraction() > threshold` | Actor HP fraction above threshold |
| `TargetHpAbove` | `ddgc_target_hp_above_<threshold>` | `ConditionContext::target_hp_fraction() > threshold` | Target HP fraction above threshold |
| `TargetHpBelow` | `ddgc_target_hp_below_<threshold>` | `ConditionContext::target_hp_fraction() < threshold` | Target HP fraction below threshold |
| `TargetHasStatus` | `ddgc_target_has_status_<kind>` | `ConditionContext::target_has_status()` | Checks first target |
| `ActorHasStatus` | `ddgc_actor_has_status_<kind>` | `ConditionContext::actor_has_status()` | Checks performing actor |
| `InMode` | `ddgc_in_mode_<mode>` | `ConditionContext::dungeon_mode_name()` | Resolves dungeon from `ConditionContext::dungeon` |
| `OnKill` | `ddgc_on_kill` | `ConditionContext::actor_killed_enemy()` | Triggers when actor killed an enemy on previous turn |

### Bridged Framework-Native Conditions

Framework-native `EffectCondition` variants are evaluated through `ConditionAdapter::evaluate_framework()` with identical behavior to `EffectContext::check_condition`:

| Framework Condition | Adapter Behavior | Notes |
|--------------------|-----------------|-------|
| `Probability(p)` | Pass if `p > 0.0` | Deterministic; real randomness is game-specific |
| `IfTargetHealthBelow(threshold)` | Pass if target HP fraction < threshold | |
| `IfActorHasStatus(kind)` | Pass if actor has active status | |
| `IfTargetPosition(slot_range)` | Returns `Unknown` | Formation context not yet available in `ConditionContext` |

### Low-Impact Deferred Conditions

The following DDGC conditions are NOT implemented in `ConditionAdapter` and classified as **low-impact deferred** — no migrated content depends on them:

| Condition | DDGC Reference | Classification | Reason |
|-----------|---------------|----------------|--------|
| `InCamp` | Camping phase | Out of scope | Not a combat mechanic |
| `InDungeon` | Dungeon phase | Out of scope | Covered by encounter context |
| `InCorridor` | Corridor position | Out of scope | No corridor concept in migrated content |
| `InActivity` | Activity phase | Out of scope | Not a combat mechanic |
| `LightBelow` / `LightAbove` / `LightChanged` | Light level | Out of scope | Light system not in migrated content |
| `Afflicted` | Actor afflicted | Low-impact deferred | Affliction system not modeled |
| `Virtued` | Actor virtuous | Low-impact deferred | Virtue system not modeled |
| `WalkBack` | Walking backwards | Low-impact deferred | Rare; no migrated content uses it |
| `Dot` | Damage-over-time | Low-impact deferred | No migrated content checks DoT status |
| `Size` | Actor size | Low-impact deferred | No migrated content checks size |
| `EnemyType` | Target enemy type | Low-impact deferred | No migrated content checks enemy type |
| `Skill` | Specific skill active | Low-impact deferred | No migrated content checks skill state |
| `Riposting` | Currently riposting | Low-impact deferred | Reactive hooks cover riposte detection |
| `ChaosBelow` / `ChaosAbove` | Chaos level | Low-impact deferred | Chaos system not modeled |
| `Melee` / `Ranged` | Attack range type | Low-impact deferred | No range distinction in migrated content |

**US-604 guardrail:** Unsupported conditions return `ConditionResult::Unknown`, which is surfaced explicitly rather than silently failing. This ensures missing implementations are observable.

### Implementation Anchors

| Anchor | Type | Description |
|--------|------|-------------|
| `desperate_strike()` | Fixture skill | Demonstrates `DeathsDoor` condition in skill context |
| `stressed_skill()` | Fixture skill | Demonstrates `StressAbove` condition in skill context |
| `first_round_strike()` | Fixture skill | Demonstrates `FirstRound` condition in skill context |

### Remaining Work (B-004)

- Low-impact deferred conditions (InCamp, LightBelow/Above, Afflicted, Virtued, etc.) — no migrated content requires them
- `IfTargetPosition` returns `Unknown` — formation context not yet available in `ConditionContext`
- B-006 and B-010 (damage variance and hit resolution) are separate work streams
- **B-004 is Resolved.** All condition families with migrated content are implemented.

---

## Phase 3 Encounter-Runtime Fidelity Inventory (Closed)

This section documents the resolution of encounter-runtime fidelity gaps identified in the Phase 3 inventory. All items below were resolved in the `ralph/ddgc-encounter-runtime-fidelity-closure` branch.

### Targeting Resolution (US-702, US-703, US-704)

| Gap | Resolution | Story |
|-----|-----------|-------|
| DDGC targeting intent model | TargetingIntent, TargetingContext, TargetRank, SideAffinity in `src/encounters/targeting.rs` | US-702 |
| Single-target and ally-target selection | DdgcTargetingRule in `src/encounters/ddgc_targeting_rules.rs`; ally-exclusion in battle loop | US-703 |
| Launch-rank and target-rank gating | Rank constraint checking in battle loop; FrontRow constraint enforcement | US-704 |

### Movement and AI Resolution (US-705, US-706)

| Gap | Resolution | Story |
|-----|-----------|-------|
| Movement direction semantics | EffectNode::pull for forward self-move (not push); formation slot update wired | US-705 |
| Family action policy (AI) | FamilyActionPolicy with DeterministicCycle (lizard) and PriorityTable (gambler); actor state tracking | US-706 |

### Boss Runtime Resolution (US-707 through US-712)

| Gap | Resolution | Story |
|-----|-----------|-------|
| Summon runtime event seam | SummonEvent, SummonKind, extract_summon_events() in `src/run/summon_events.rs` | US-707 |
| Summon materialization | SummonTracker, SummonKind mapping, materialize_summons() in `src/run/summon_materialization.rs` | US-708 |
| Shared-health linking | SharedHealthPool, SharedHealthTracker, azure_dragon golden trace | US-709 |
| Multi-phase transitions | white_tiger phase progression (terrain → A/B → final form) | US-710 |
| Captor/release | CaptorTracker, CaptureEvent, captive state in turn order in `src/run/captor_state.rs`, `src/run/capture_events.rs` | US-711 |
| Controller/paired-boss | PairedBossTracker, HP averaging, crimson_duet in `src/run/paired_boss.rs` | US-712 |

### Phase 3 Closeout

| Gap | Status | Notes |
|-----|--------|-------|
| Remove run fallbacks (US-713) | Resolved (P3) | `src/run/flow.rs` no longer falls back to `first_battle`; `run_slice_uses_no_fallback_content` verifies representative run slices use migrated DDGC content only |

---

## Phase 4a DDGC Condition Touchpoints Inventory (US-801-b)

This section inventories migrated DDGC skills and statuses that reference condition variants, serving as the implementation anchor for Phase 4a work targeting only conditions with real migrated content.

### Condition Family Classification

DDGC conditions are grouped into families. Each family may have implemented variants (supported by `ConditionAdapter`) and unimplemented variants (not yet supported).

| Condition Family | Implemented Variants | Unimplemented Variants | Migrated Content Using Family |
|-----------------|---------------------|----------------------|------------------------------|
| **HP-threshold** | `DeathsDoor` (HP < 50%), `HpAbove`, `TargetHpAbove`, `TargetHpBelow`, `IfTargetHealthBelow` (framework) | None | `desperate_strike` (DeathsDoor), `retribution_strike` (HpAbove, fixture) |
| **Stress-threshold** | `StressAbove`, `StressBelow` | None | None |
| **Round-trigger** | `FirstRound` | None | `opening_strike` (FirstRound) |
| **Status-check** | `ActorHasStatus`, `TargetHasStatus`, `IfActorHasStatus` (framework) | None | None |
| **Dungeon-mode** | `InMode` (US-803) | None | `xuanwu_strike` (Hunter, InMode) |
| **Kill-trigger** | `OnKill` (US-804) | None | `executioner_strike` (Hunter, OnKill) |
| **Position** | `IfTargetPosition` (returns Unknown — formation not wired) | None | None |
| **Probability** | `Probability` (framework) | None | `skull_bash` (stun chance) |

### Migrated Skills Referencing DDGC Condition Tags

The following migrated skills use `EffectNode::with_game_condition("ddgc_...")` tags evaluated by `ConditionAdapter`:

| Skill | File | Condition Tag | Condition Family | Adapter Status |
|-------|------|--------------|-----------------|----------------|
| `opening_strike` | `src/content/heroes/hunter.rs:154` | `ddgc_first_round` | Round-trigger | IMPLEMENTED |
| `desperate_strike` | `src/content/heroes/hunter.rs:180` | `ddgc_deaths_door` | HP-threshold | IMPLEMENTED |
| `retribution_strike` | `src/content/heroes/hunter.rs:205` | `ddgc_hp_above_0.5` | HP-threshold | IMPLEMENTED |
| `xuanwu_strike` | `src/content/heroes/hunter.rs:225` | `ddgc_in_mode_xuanwu` | Dungeon-mode | IMPLEMENTED |
| `executioner_strike` | `src/content/heroes/hunter.rs:246` | `ddgc_on_kill` | Kill-trigger | IMPLEMENTED |

**Note:** `desperate_strike` and `xuanwu_strike` are defined but NOT registered in `skill_pack()` (line 260), making them unreachable in current battle paths. `opening_strike`, `retribution_strike`, and `executioner_strike` are registered and reachable. See GAP-003 in `tasks/adapter_api_drift_inventory.md`.

### Migrated Skills Using Framework EffectConditions

| Skill | File | Framework Condition | Adapter Status |
|-------|------|--------------------|----------------|
| `skull_bash` | `src/content/skills.rs:91` | `EffectCondition::Probability(0.60)` | IMPLEMENTED |

### Condition Coverage: Implemented vs. Original DDGC Variants

All condition families referenced by migrated content are now implemented. The following table provides full coverage of the original 35+ DDGC `BuffRule` variants:

| DDGC BuffRule Variant | ConditionAdapter Status | Classification | Migrated Content Using It |
|-----------------------|------------------------|----------------|--------------------------|
| `HpBelow` / `TargetHpBelow` | `TargetHpBelow`, `IfTargetHealthBelow` (framework) | Implemented | `retribution_strike` |
| `HpAbove` / `TargetHpAbove` | `HpAbove`, `TargetHpAbove` | Implemented | `retribution_strike` (fixture) |
| `DeathsDoor` | `DeathsDoor` | Implemented | `desperate_strike` |
| `StressAbove` | `StressAbove` | Implemented | — |
| `StressBelow` | `StressBelow` | Implemented | — |
| `FirstRound` | `FirstRound` | Implemented | `opening_strike` |
| `Status` / `ActorStatus` | `ActorHasStatus`, `IfActorHasStatus` (framework) | Implemented | — |
| `TargetHasStatus` | `TargetHasStatus` | Implemented | — |
| `InMode` | `InMode` | Implemented | `xuanwu_strike` |
| `OnKill` | `OnKill` | Implemented | `executioner_strike` |
| `InRank` | `IfTargetPosition` (framework) | Partial (returns Unknown) | — |
| `Probability` | `Probability` (framework) | Implemented | `skull_bash` |
| `InCamp` | — | Out of scope | — |
| `InDungeon` | — | Out of scope | — |
| `InCorridor` | — | Out of scope | — |
| `InActivity` | — | Out of scope | — |
| `LightBelow` / `LightAbove` | — | Out of scope | — |
| `LightChanged` | — | Out of scope | — |
| `Afflicted` | — | Low-impact deferred | — |
| `Virtued` | — | Low-impact deferred | — |
| `WalkBack` | — | Low-impact deferred | — |
| `Dot` | — | Low-impact deferred | — |
| `Size` | — | Low-impact deferred | — |
| `EnemyType` | — | Low-impact deferred | — |
| `Skill` | — | Low-impact deferred | — |
| `Riposting` | — | Low-impact deferred | — |
| `ChaosBelow` / `ChaosAbove` | — | Low-impact deferred | — |
| `Melee` / `Ranged` | — | Low-impact deferred | — |

**Coverage summary:** 15 of 34 distinct DDGC BuffRule variants implemented through `ConditionAdapter` (11 `DdgcCondition` + 4 framework bridges), with 1 partial (`IfTargetPosition` returns Unknown). 19 remaining variants classified as out-of-scope (7: InCamp, InDungeon, InCorridor, InActivity, LightBelow, LightAbove, LightChanged) or low-impact deferred (12: Afflicted, Virtued, WalkBack, Dot, Size, EnemyType, Skill, Riposting, ChaosBelow, ChaosAbove, Melee, Ranged) — none are used in migrated content. Total enumerated variants (34) is consistent with the original "35+" approximation.

### Implemented Condition Families — Implementation Anchors

Phase 4a implementation anchors are migrated skills that serve as concrete proof-of-concept for each condition family:

#### Anchor 1: HP-threshold Family (`DeathsDoor`)

- **Skill anchor:** `desperate_strike()` in `src/content/heroes/hunter.rs:180`
- **Condition tag:** `ddgc_deaths_door`
- **Condition variant:** `DdgcCondition::DeathsDoor` (actor HP < 50%)
- **Implementation:** `ConditionContext::actor_at_deaths_door()` evaluates HP fraction
- **Status:** IMPLEMENTED in adapter; `desperate_strike` NOT registered in `skill_pack()`

#### Anchor 2: Round-trigger Family (`FirstRound`)

- **Skill anchor:** `opening_strike()` in `src/content/heroes/hunter.rs:154`
- **Condition tag:** `ddgc_first_round`
- **Condition variant:** `DdgcCondition::FirstRound`
- **Implementation:** `ConditionContext::is_first_round()` checks `current_round == 0`
- **Status:** IMPLEMENTED in adapter; wiring to `game_condition_evaluator` confirmed in `encounters.rs:531`

#### Anchor 3: Dungeon-mode Family (`InMode`)

- **Skill anchor:** `xuanwu_strike()` in Hunter hero content
- **Condition tag:** `ddgc_in_mode_xuanwu`
- **Condition variant:** `DdgcCondition::InMode("xuanwu")`
- **Implementation:** `ConditionContext::dungeon_mode_name()` resolves current dungeon
- **Status:** IMPLEMENTED (US-803)

#### Anchor 4: Kill-trigger Family (`OnKill`)

- **Skill anchor:** `executioner_strike()` in Hunter hero content
- **Condition tag:** `ddgc_on_kill`
- **Condition variant:** `DdgcCondition::OnKill`
- **Implementation:** `ConditionContext::actor_killed_enemy()` checks kill records
- **Status:** IMPLEMENTED (US-804)

### Adapter Wiring Status for Condition-Evaluated Skills

Even though conditions are implemented in `ConditionAdapter`, the skills that use them depend on the `game_condition_evaluator` being wired to `EffectContext`:

| Skill | Evaluator Wired? | File:Line | Notes |
|-------|----------------|------------|-------|
| `opening_strike` | YES | `encounters.rs:531` | `game_condition_evaluator` set via `create_game_condition_evaluator()` |
| `desperate_strike` | YES (if reached) | `encounters.rs:531` | Same wiring path, but skill not in `skill_pack()` |
| `retribution_strike` | YES | `encounters.rs:531` | Same wiring path; registered in `skill_pack()` |
| `xuanwu_strike` | YES (if reached) | `encounters.rs:531` | Same wiring path, but skill not in `skill_pack()` |
| `executioner_strike` | YES | `encounters.rs:531` | Same wiring path; registered in `skill_pack()` |

### Summary: Migrated Content Referencing Unimplemented Conditions

**Result: Zero migrated skills or statuses reference unimplemented condition variants.**

All migrated content that uses DDGC condition tags uses variants that ARE implemented in `ConditionAdapter`. All condition families with migrated content (HP-threshold, stress-threshold, round-trigger, status-check, dungeon-mode, kill-trigger) are now fully implemented. Remaining DDGC variants (InCamp, LightBelow/Above, Afflicted, Virtued, etc.) are classified as out-of-scope or low-impact deferred — no migrated content depends on them.

**B-004 is Resolved.**

---

*Generated by US-801-b task: P4a-0: Inventory remaining condition touchpoints (adapter)*
*Reviewed and finalized by US-801-c: P4a-0: Inventory remaining condition touchpoints (docs)*
