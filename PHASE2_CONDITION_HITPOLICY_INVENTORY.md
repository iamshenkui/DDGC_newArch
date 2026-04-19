# Phase 2 DDGC Condition and Hit-Policy Inventory

**PRD Reference:** `tasks/prd-semantic-gap-phase-2.json` — US-601
**Branch:** `ralph/ddgc-conditional-hit-policy-closure`
**Status:** Discovery slice — inventory of open gaps for Phase 2 implementation

---

## Overview

This document catalogs all migrated DDGC content affected by:
- **B-004:** DDGC-specific conditional rules
- **B-006:** Damage range (min/max) vs. fixed-averaging
- **B-010:** Accuracy vs. dodge hit resolution

The inventory distinguishes framework-native conditions from DDGC-only conditions and selects implementation anchors for Phase 2.

---

## B-004: DDGC-Specific Conditions

### Framework-Native Conditions (Bridged via ConditionAdapter)

These conditions are already bridged through `ConditionAdapter::evaluate_framework()` in `src/run/conditions.rs` with identical behavior to `EffectContext::check_condition`:

| Framework Condition | Evaluation | Status |
|---|---|---|
| `EffectCondition::Probability(p)` | Pass if `p > 0.0` | **Implemented** — deterministic pass-through |
| `EffectCondition::IfTargetHealthBelow(threshold)` | Pass if target HP fraction < threshold | **Implemented** |
| `EffectCondition::IfActorHasStatus(kind)` | Pass if actor has active status | **Implemented** |
| `EffectCondition::IfTargetPosition(slot_range)` | Returns `Unknown` | **Not implemented** — formation context unavailable in `ConditionContext` |

### DDGC-Specific Conditions (Implemented via ConditionAdapter)

These conditions are evaluated via `ConditionAdapter::evaluate_ddgc()` in `src/run/conditions.rs`:

| DDGC Condition | Tag Format | Implementation Method | Status |
|---|---|---|---|
| `FirstRound` | `ddgc_first_round` | `ConditionContext::is_first_round()` | **Implemented** |
| `StressAbove(threshold)` | `ddgc_stress_above_<threshold>` | `ConditionContext::actor_stress_above()` | **Implemented** |
| `StressBelow(threshold)` | `ddgc_stress_below_<threshold>` | `ConditionContext::actor_stress_below()` | **Implemented** |
| `DeathsDoor` | `ddgc_deaths_door` | `ConditionContext::actor_at_deaths_door()` | **Implemented** |
| `TargetHasStatus(kind)` | `ddgc_target_has_status_<kind>` | `ConditionContext::target_has_status()` | **Implemented** |
| `ActorHasStatus(kind)` | `ddgc_actor_has_status_<kind>` | `ConditionContext::actor_has_status()` | **Implemented** |

### Migrated Skills Using DDGC Conditions

The following skills use the `ConditionAdapter` condition mechanism via the `.with_game_condition()` builder pattern:

| Skill | File | Condition Tag | Status |
|---|---|---|---|
| `opening_strike()` | `src/content/heroes/hunter.rs` | `ddgc_first_round` | **Implemented via ConditionAdapter** |
| `desperate_strike()` | `src/content/heroes/hunter.rs` | `ddgc_deaths_door` | **Implemented via ConditionAdapter** |

### Still-Blocked DDGC Conditions

The following DDGC conditions are **not yet implemented** in `ConditionAdapter`:

| DDGC Condition | DDGC Reference | Notes |
|---|---|---|
| `InMode` | Dungeon-specific modes | Requires dungeon context modeling |
| `HpAbove` | HP above threshold | Counterpart to DeathsDoor |
| `TargetHpAbove` | Target HP above threshold | Counterpart to target HP below |
| Other BuffRule variants | `BuffRule` 35+ variants | Only a subset is implemented |

**Note:** The `ConditionAdapter::evaluate_framework()` returns `ConditionResult::Unknown` for any unrecognized framework condition variant, ensuring missing implementations are observable rather than silent failures.

---

## B-006: Damage Range (Min/Max) vs. Fixed Damage

### Current State

All migrated skills use **averaged fixed damage** (min+max)/2. DDGC's min/max range information is not tracked.

### All Migrated Skills Affected by Damage Averaging

**Legacy Skills (`src/content/skills.rs`):**
| Skill | DDGC Range | Current Value | Notes |
|---|---|---|---|
| `crusading_strike` | 8–15 | 12 (avg) | Basic melee |
| `holy_lance` | 6–12 | 9 (avg) | Ranged + self-heal |
| `divine_grace` | 8–12 | 10 (avg) | Single-target heal |
| `rend` | 4–8 | 6 (avg) | Damage + bleed |
| `skull_bash` | 10–18 | 14 (avg) | High damage + stun |
| `grave_bash` | 3–7 | 5 (avg) | Multi-hit |

**Hero Family Skills (`src/content/heroes/`):**
| Family | Skill Count | Damage Range Example | Notes |
|---|---|---|---|
| Alchemist | 7 | 17–34 → 26 avg | Lowest damage class |
| Diviner | 7 | 27–45 → 36 avg | Magic damage |
| Hunter | 8 | 35–45 → 40 avg | Highest physical damage |
| Shaman | 7 | 32–45 → 39 avg | Magic damage |
| Tank | 7 | 27–35 → 31 avg | Moderate damage |
| White variants | 35 | Same ranges | Skill effects differ |
| Black variants | 35 | Same ranges | Skill effects differ |

**Monster Skills:** All monster families use averaged damage values.

### Damage Policy Implementation Anchors

| Anchor | Type | Description |
|--------|------|-------------|
| `opening_strike()` | Fixture skill | Demonstrates first-round bonus damage — could use damage policy |
| `desperate_strike()` | Fixture skill | Demonstrates deaths-door bonus damage — could use damage policy |
| `skull_bash()` | Migrated skill | Uses `EffectCondition::Probability` for stun — natural fit for hit-policy work |

---

## B-010: Accuracy vs. Dodge Hit Resolution

### Current State

- Actors **have** a dodge attribute (e.g., Hunter has 5% dodge, Black Tortoise A has 30% dodge)
- There is **no hit-resolution policy** — attacks always hit
- Accuracy/dodge debuffs exist as status markers but are **not evaluated**

### Migrated Content with Dodge Attributes

| Actor | Dodge | Source |
|-------|-------|--------|
| Hunter (ally) | 5% | `src/content/heroes/hunter.rs` |
| Diviner (ally) | 5% | `src/content/heroes/diviner.rs` |
| Black Tortoise A | 30% | `src/content/monsters/black_tortoise_a.rs` |
| Vermilion Bird | 30% | `src/content/monsters/vermilion_bird.rs` |
| White Tiger A | 30% | `src/content/monsters/white_tiger_a.rs` |
| Azure Dragon | 30% | `src/content/monsters/azure_dragon.rs` |
| Bloodthirsty Assassin | 15% | `src/content/monsters/bloodthirsty_assassin.rs` |
| Alligator Yangtze | 7.5% | `src/content/monsters/alligator_yangtze.rs` |
| Frostvein Clam | 15% | `src/content/monsters/frostvein_clam.rs` |
| Ghost Fire variants | 7.5% | `src/content/monsters/ghost_fire_*.rs` |
| Robber variants | 15% | `src/content/monsters/robber_*.rs` |
| Pearlkin variants | 15% | `src/content/monsters/pearlkin_*.rs` |
| Glutton Pawnshop | 15% | `src/content/monsters/glutton_pawnshop.rs` |

### Migrated Content with Accuracy/Dodge Debuff Skills

These skills apply accuracy or dodge debuffs but the debuffs are **not evaluated** in hit resolution:

| Skill | File | Debuff | Status |
|-------|------|--------|--------|
| `smoke_bomb` | `src/content/monsters/robber_melee.rs` | `accuracy_debuff` | **Not evaluated** — game-gap B-010 |
| `rain_spray` | `src/content/monsters/black_tortoise_a.rs` | `dodge_debuff` + `accuracy_debuff` | **Not evaluated** |
| `po_debuff` | `src/content/monsters/pearlkin_opalescent.rs` | `dodge_debuff` | **Not evaluated** |
| `requiem_stillbirth` | `src/content/monsters/necrodrake_embryosac.rs` | `dodge_debuff` + `accuracy_debuff` | **Not evaluated** |
| `wind_buff_acc` | `src/content/monsters/azure_dragon_ball_wind.rs` | Accuracy buff (ally) | **Not evaluated** |

### Hit-Policy Implementation Anchors

| Anchor | Type | Description |
|--------|------|-------------|
| `smoke_bomb()` | Migrated monster skill | Applies `accuracy_debuff` — primary anchor for B-010 |
| `rain_spray()` | Migrated monster skill | Applies both `dodge_debuff` and `accuracy_debuff` |
| Hunter archetype | Migrated hero | Has 5% dodge baseline — natural test subject for dodge resolution |

---

## Implementation Anchor Selection for Phase 2

### Condition Anchor Selection (B-004)

**Initial condition anchors (recommended priority):**
1. `opening_strike()` — uses `FirstRound` condition (already implemented in `conditions.rs`)
2. `desperate_strike()` — uses `DeathsDoor` condition (already implemented in `conditions.rs`)
3. `stressed_skill()` — demonstrates `StressAbove` condition (ready to implement)

**Rationale:** `FirstRound` and `DeathsDoor` are already implemented in `ConditionAdapter`. These provide immediate implementation anchors. `StressAbove` is also implemented and provides a hero-specific condition anchor.

### Hit-Policy Anchor Selection (B-010)

**Initial hit-policy anchor:**
1. `smoke_bomb()` — applies `accuracy_debuff` status; primary anchor for accuracy-vs-dodge resolution

**Rationale:** `smoke_bomb` is a concrete migrated skill that explicitly references accuracy. It provides a clear, observable behavior that can be verified: heroes affected by `accuracy_debuff` should have a higher chance to miss against dodge-capable enemies.

---

## Distinction: Framework-Native vs. DDGC-Only

| Category | Description | Resolution |
|----------|-------------|------------|
| **Framework-native** | Generic conditions supported by the framework (`Probability`, `IfTargetHealthBelow`, `IfActorHasStatus`) | Bridged through `ConditionAdapter::evaluate_framework()` — no duplication of framework logic |
| **DDGC-only** | DDGC-specific conditions requiring game-layer state (`FirstRound`, `StressAbove`, `DeathsDoor`, etc.) | Implemented via `ConditionAdapter::evaluate_ddgc()` using `ConditionContext` |

The adapter architecture ensures:
- Framework-native conditions behave identically before and after the adapter
- DDGC-only conditions are evaluated through `ConditionContext` which provides game-layer state
- Unknown conditions return `ConditionResult::Unknown` rather than silently failing

---

## Summary: Open Gaps by Blocker

| Blocker | Classification | Open Items | Ready for Implementation |
|---------|----------------|------------|--------------------------|
| **B-004** | DDGC conditions | `InMode`, `HpAbove`, `TargetHpAbove` | **Yes** — `FirstRound`, `StressAbove`, `StressBelow`, `DeathsDoor`, `TargetHasStatus`, `ActorHasStatus` implemented |
| **B-006** | Damage variance | All migrated skills use averaged damage | **Partial** — needs `DamagePolicy` interface; averaged mode is stable |
| **B-010** | Hit resolution | No accuracy-vs-dodge resolution | **Yes** — needs `HitResolutionPolicy` interface; `smoke_bomb()` anchor identified |

---

## Files Reference

| File | Purpose |
|------|---------|
| `src/run/conditions.rs` | `ConditionContext`, `DdgcCondition`, `ConditionAdapter`, `ConditionResult` |
| `src/content/heroes/hunter.rs` | `opening_strike()` (FirstRound), `desperate_strike()` (DeathsDoor) |
| `src/content/monsters/robber_melee.rs` | `smoke_bomb()` — accuracy debuff anchor |
| `src/content/statuses.rs` | Status factory functions (`bleed`, `stun`, `riposte`, `burn`, etc.) |
| `src/content/actors.rs` | Actor archetypes with dodge attributes |
| `SEMANTIC_GAPS.md` | Gap classification and tracking |
| `MIGRATION_BLOCKERS.md` | Blocker classification and resolution status |