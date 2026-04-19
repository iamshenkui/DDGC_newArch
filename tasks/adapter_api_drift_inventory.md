# Adapter API Drift Inventory (US-001-a)

Inventory of compile-blocking mismatches and API surface drift between the
`ConditionAdapter` (`src/run/conditions.rs`) and the framework crates it depends on.
Generated from a live `cargo check` + `cargo clippy` pass against the current codebase.

---

## Summary

| Category | Count |
|---|---|
| Currently blocking typecheck (clippy) | 0 (fixed) |
| Framework API drift — adapter handles correctly but with divergence risk | 2 |
| Framework API drift — adapter cannot reach | 2 |
| DDGC game-layer logic gaps | 2 |

---

## 1. Encounter Runtime (`src/run/encounters.rs`)

### DRIFT-001: `EffectContext::new` never wires `game_condition_evaluator`

**Classification:** Framework API drift — adapter cannot reach

**Detail:** The battle loop at `encounters.rs:515` creates `EffectContext::new(...)`
without the `game_condition_evaluator` callback. The framework provides
`EffectContext::new_with_game_condition_evaluator(...)` precisely for this purpose,
but it has zero call sites across the entire game codebase.

**Impact:** Any `EffectCondition::GameCondition(tag)` embedded in a
`SkillDefinition` effect node silently fails (returns `false`) when resolved
through the framework's `resolve_skill`. The adapter's `evaluate_by_tag` machinery
exists but is never invoked by the framework path.

**Affected content:** Hunter `opening_strike` bonus damage (`ddgc_first_round`),
Hunter `desperate_strike` bonus damage (`ddgc_deaths_door`).

**Remediation direction:** Wire `ConditionAdapter::evaluate_by_tag` (or a
closure wrapping it) as the `game_condition_evaluator` when constructing
`EffectContext` in the battle loop. Requires threading `ConditionContext` data
into the encounter loop.

---

### DRIFT-002: `IfTargetPosition` returns `Unknown` — formation not wired

**Classification:** Framework API drift — adapter cannot reach

**Detail:** `ConditionAdapter::evaluate_framework` returns `ConditionResult::Unknown`
for `EffectCondition::IfTargetPosition` because `ConditionContext` does not hold a
`FormationLayout` reference. The framework's `EffectContext` CAN evaluate this
condition because it carries `&mut FormationLayout` directly.

**Impact:** Position-based conditions routed through the adapter are
observable as `Unknown` but never resolve. The framework's own `check_condition`
handles them correctly when called through `EffectContext`.

**Remediation direction:** Add formation layout data to `ConditionContext` or
document this as a permanent limitation of the snapshot-based adapter design.

---

## 2. Adapter Module (`src/run/conditions.rs`)

### DRIFT-003: `evaluate_framework` re-implements framework logic instead of delegating

**Classification:** Framework API drift — divergence risk

**Detail:** `ConditionAdapter::evaluate_framework` duplicates the logic of
`EffectContext::check_condition` for `IfTargetHealthBelow`, `IfActorHasStatus`,
and `Probability` rather than delegating to the framework method. If the
framework changes condition semantics, the adapter will silently diverge.

**Impact:** Currently correct — both paths produce identical results. But the
duplication creates a maintenance trap: any framework semantic change requires
a corresponding manual update to the adapter.

**Remediation direction:** Either delegate to `EffectContext::check_condition`
(where possible) or add a synchronization test that compares adapter results
against framework results for all native conditions.

---

### DRIFT-004: `IfTargetHealthBelow` threshold semantics: raw HP vs fraction

**Classification:** Framework API drift — divergence risk

**Detail:** The framework docstring for `IfTargetHealthBelow` says
"Only execute if the target's health is below the given fraction (0.0–1.0)"
but the implementation compares **raw effective HP** against the threshold,
not a fraction. The adapter faithfully mirrors the implementation (raw HP),
but the docstring mismatch creates a long-term authoring trap.

**Impact:** Any author reading the framework doc expecting fractions will
write wrong threshold values. The adapter's `actor_hp_fraction()` and
`target_hp_fraction()` APIs compute proper fractions, but these are not
exposed as `DdgcCondition` variants for HP-threshold conditions.

**Remediation direction:** Clarify the framework docstring or add a
`DdgcCondition::TargetHpBelow(fraction)` variant that uses the fraction API.

---

### DRIFT-005 (FIXED): `GameCondition` handled via `evaluate_by_tag` delegation

**Classification:** Fixed in this iteration

**Detail:** `EffectCondition::GameCondition(tag)` was previously handled by the
`#[allow(unreachable_patterns)] _ => Unknown` catch-all. Now it explicitly
delegates to `ConditionAdapter::evaluate_by_tag(tag)`, bridging the framework
dispatch to DDGC condition evaluation.

---

### DRIFT-006 (FIXED): `has_status_kind` used `.iter()` instead of `.values()`

**Classification:** Fixed in this iteration

**Detail:** The free function `has_status_kind` used `actor.statuses.active().iter().any(|s| s.1.kind.0 == kind)`
while `evaluate_framework` and the framework itself use `.values().any(|s| s.kind.0 == ...)`.
Now consistent.

---

### DRIFT-007 (FIXED): `SlotRange` import suppressed with `#[allow(unused_imports)]`

**Classification:** Fixed in this iteration

**Detail:** `SlotRange` was imported at module level with `#[allow(unused_imports)]`
but only used in test code. Moved to `#[cfg(test)]` import in `adapter_tests`.

---

### DRIFT-008 (FIXED): Misleading Probability comment

**Classification:** Fixed in this iteration

**Detail:** Comment said "probability < 1.0 always returns true" but code
checks `p > 0.0`. Corrected to "any probability > 0.0 passes in headless mode."

---

## 3. Hero Content (`src/content/heroes/hunter.rs`)

### GAP-001: Hunter `GameCondition` skills fire unconditionally

**Classification:** DDGC game-layer logic gap

**Detail:** `opening_strike` and `desperate_strike` define bonus damage nodes
with `EffectNode::with_game_condition("ddgc_first_round")` and
`with_game_condition("ddgc_deaths_door")`. The comments state these are
"evaluated by the game-layer ConditionAdapter via the `game_condition_evaluator`
set on EffectContext." But no `EffectContext` anywhere in the codebase sets
this evaluator, so the `GameCondition` check always returns `false` and the
bonus damage never applies.

**Impact:** Hunter's conditional bonus damage is dead code in all live battle
paths. Tests pass only because the unconditional damage node produces non-zero
totals.

**Remediation direction:** Wire `game_condition_evaluator` in the encounter
loop (see DRIFT-001).

---

## 4. Scenarios & Test Fixtures

### GAP-002: `deferred_effect_battle` scenario does not exercise `GameCondition` evaluation

**Classification:** DDGC game-layer logic gap

**Detail:** The scenario's stated purpose is to test that "deferred effects
(DDGC condition tags) are properly evaluated." But its battle loop creates
`EffectContext::new(...)` without the evaluator, so `GameCondition` nodes
always fail. The test `opening_strike_deferred_effect_appears_in_trace`
asserts `total_damage > 0.0`, which passes because the unconditional
20-damage node fires, but the 20-bonus conditional node never fires.

**Impact:** The test's acceptance criterion is weaker than the scenario's
stated goal. The scenario does not actually verify `GameCondition` evaluation.

**Remediation direction:** Wire `game_condition_evaluator` in the scenario
and strengthen the test assertion to verify conditional bonus damage.

---

## Classification Key

| Label | Meaning |
|---|---|
| **Framework API drift** | The adapter's assumptions about framework types/methods diverge from the framework's actual API or are at risk of diverging |
| **DDGC game-layer logic gap** | The adapter or runtime is missing wiring that would make DDGC-specific game logic actually execute in live battle paths |
