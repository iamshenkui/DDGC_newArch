# API Drift Repair Closeout (US-005)

Verification and status report for the DDGC_newArch Framework API Drift Repair effort.
Generated 2026-04-19.

---

## Verification Summary

| Check | Result |
|---|---|
| `cargo check` | PASS (0 errors) |
| `cargo test` | PASS (893 tests, 0 failures, 7 doc-tests ignored) |
| `cargo clippy` | PASS (0 warnings) |
| Typecheck | PASS |

---

## What Was Repaired

### US-001: Inventory The API Drift Surface
- **US-001-a (adapter):** Produced `tasks/adapter_api_drift_inventory.md` documenting 8 drift items (DRIFT-001 through DRIFT-008) and 3 game-layer logic gaps (GAP-001 through GAP-003). 4 items fixed (DRIFT-005 through DRIFT-008).
- **US-001-b (tests):** Produced `tasks/tests_api_drift_inventory.md` documenting 15 test drift items (TDRIFT-001 through TDRIFT-015). 2 items fixed (TDRIFT-008, TDRIFT-009).

### US-002: Realign Runtime Effect Resolution To Public Framework APIs
- **US-002-a (runner):** `src/run/encounters.rs` no longer imports private framework functions. Runtime code compiles against current `resolve_skill` and effect-result APIs. Riposte, deferred-effect, and battle-loop plumbing compile without referencing removed `results` or `deferred` wrapper fields.
- **US-002-b (adapter):** Adapter-layer changes aligned with public framework APIs.

### US-003: Reconnect DDGC Conditional Effects Through Supported Condition Plumbing
- **US-003-a (contracts):** Condition contract types compile without referencing removed `EffectNode` fields.
- **US-003-b (adapter):** `ConditionAdapter` bridges framework-native and DDGC-specific conditions. `GameCondition(tag)` dispatches to `evaluate_by_tag`. Implemented DDGC conditions: `FirstRound`, `StressAbove`, `StressBelow`, `DeathsDoor`, `TargetHasStatus`, `ActorHasStatus`.
- **US-003-c (docs):** `SEMANTIC_GAPS.md` updated with Phase 2 condition inventory and bridged framework conditions.
- **US-003-d (tests):** Condition adapter tests pass; unknown conditions return `ConditionResult::Unknown` explicitly.

### US-004: Repair Scenario And Regression Fixtures Against The Current Runtime Shape
- **US-004-a (state):** Scenario files under `src/scenarios/` compile against repaired runtime interfaces.
- **US-004-b (tests):** Focused tests updated to supported observable outputs. Deterministic trace-oriented coverage remains in place.

---

## What Remains Deferred

### DRIFT-001: `EffectContext::new` never wires `game_condition_evaluator`
- Hunter `opening_strike` and `desperate_strike` conditional bonus damage is dead code in live battle paths because the encounter loop creates `EffectContext` without the evaluator callback.
- **Impact:** `GameCondition` nodes always return `false` through the framework path.
- **Remediation:** Wire `ConditionAdapter::evaluate_by_tag` as the `game_condition_evaluator` when constructing `EffectContext` in `src/run/encounters.rs`. Requires threading `ConditionContext` data into the encounter loop.
- **Tracking:** DRIFT-001, GAP-001, GAP-002, TDRIFT-005

### DRIFT-002: `IfTargetPosition` returns `Unknown` - formation not wired
- `ConditionContext` does not hold a `FormationLayout` reference, so position-based conditions routed through the adapter never resolve.
- **Remediation:** Add formation layout data to `ConditionContext`.
- **Tracking:** DRIFT-002, TDRIFT-013

### GAP-003: `desperate_strike` not registered in `skill_pack()`
- The skill exists in code but is not included in the Hunter skill pack, making it unreachable.
- **Remediation:** Add `desperate_strike()` to `skill_pack()` return value.
- **Tracking:** GAP-003

### Additional DDGC condition families
- `InMode` dungeon-specific conditions, `HpAbove`, `TargetHpAbove`, and remaining `BuffRule` variants not yet implemented.
- **Tracking:** B-004

### TargetSelector missing PartialEq
- `framework_combat::targeting::TargetSelector` does not derive `PartialEq`, forcing workaround types in parity tests.
- **Tracking:** TDRIFT-007, TDRIFT-010

### Probability always passes if p > 0.0
- Sub-100% proc chances are collapsed to 100% in headless mode.
- **Tracking:** TDRIFT-014

---

## Residual Risks Tied To Framework Evolution

1. **DRIFT-003: Adapter re-implements framework logic.** `ConditionAdapter::evaluate_framework` duplicates `EffectContext::check_condition` for native conditions. If the framework changes condition semantics, the adapter will silently diverge. A synchronization test comparing adapter results against framework results would mitigate this risk.

2. **DRIFT-004: IfTargetHealthBelow threshold semantics ambiguity.** The framework docstring says "fraction (0.0-1.0)" but the implementation compares raw HP. The adapter mirrors the implementation, but the docstring mismatch creates an authoring trap.

3. **Hard-coded actor ID thresholds in tests (TDRIFT-001).** Tests use `id >= 10` to distinguish enemy vs ally actors with no framework API backing. Framework changes to ID assignment would silently break assertions.

4. **Magic seed values in deterministic tests (TDRIFT-003).** Pack-count changes in encounter registries can silently cause tests to exercise the wrong content.

5. **Test fixture duplication from content code (TDRIFT-012).** `StatusParityFixture` values are manually duplicated from `src/content/statuses.rs` with no compile-time linkage, creating drift risk.

6. **ContentPack.skills public field coupling (TDRIFT-011).** Tests iterate internal `HashMap` storage directly instead of using `get_skill` API.

---

## Task Decomposition Summary

The API Drift Repair PRD was decomposed into 5 user stories with 13 subtasks:

| Task ID | Title | Status |
|---|---|---|
| US-001-a | Inventory API Drift Surface (adapter) | verified_done |
| US-001-b | Inventory API Drift Surface (tests) | verified_done |
| US-002-a | Realign Runtime Effect Resolution (runner) | verified_done |
| US-002-b | Realign Runtime Effect Resolution (adapter) | verified_done |
| US-003-a | Reconnect DDGC Conditional Effects (contracts) | verified_done |
| US-003-b | Reconnect DDGC Conditional Effects (adapter) | verified_done |
| US-003-c | Reconnect DDGC Conditional Effects (docs) | verified_done |
| US-003-d | Reconnect DDGC Conditional Effects (tests) | verified_done |
| US-004-a | Repair Scenario/Regression Fixtures (state) | verified_done |
| US-004-b | Repair Scenario/Regression Fixtures (tests) | verified_done |
| US-005-a | Close Out With Green Verification (state) | verified_done |
| US-005-b | Close Out With Green Verification (planner) | ready |
| US-005-c | Close Out With Green Verification (tests) | ready |

**10 of 13 subtasks verified done.** US-005-b and US-005-c remain ready for future execution.
