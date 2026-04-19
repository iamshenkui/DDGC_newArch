# Tests API Drift Inventory (US-001-b)

Inventory of API mismatches, weak assertions, and framework drift surfaced in the
integration test suite (`tests/`) and inline `#[cfg(test)]` blocks.
Generated from a live `cargo check --tests` + `cargo clippy --tests` + `cargo test` pass.

---

## Summary

| Category | Count |
|---|---|
| Currently blocking typecheck (clippy) | 0 |
| Framework API drift — missing trait derivations requiring workaround types | 1 |
| Framework API drift — adapter gaps that propagate into test assertions | 2 |
| DDGC game-layer logic gaps — weak or vacuous test assertions | 3 |
| DDGC game-layer logic gaps — hard-coded internal assumptions in tests | 3 |
| Test hygiene — raw strings instead of constants, inconsistent imports | 4 |

---

## 1. Encounter Runtime (`tests/encounter_trace.rs`, `src/run/encounters.rs` tests)

### TDRIFT-001: Hard-coded actor ID numeric thresholds for side discrimination

**Classification:** DDGC game-layer logic gap — hard-coded internal assumption

**Detail:** `encounter_trace.rs` lines 97-98, 191-192, 285-286, 392-393 use
`.filter(|&&id| id >= 10)` to distinguish enemy actors from ally actors. This
bakes in the internal assignment convention that ally IDs are 1-4 and enemy IDs
start at 10, which is an implementation detail of `EncounterResolver::run_battle`.
There is no framework API for `ActorId::is_enemy()` or `CombatSide` lookup from
a trace snapshot.

**Impact:** If `run_battle` changes ally or enemy ID assignment, all these
assertions silently break.

**Remediation direction:** Expose a `CombatSide` field in the trace snapshot,
or provide a `side_lookup` map alongside `BattleTrace`.

---

### TDRIFT-002: DDGC single-target targeting rule not enforced for enemy skills (test documents gap)

**Classification:** Framework API drift — adapter gap propagating into tests

**Detail:** The inline test `ddgc_targeting_rule_for_enemy_skills_returns_correct_count`
in `src/run/encounters.rs` explicitly verifies that `mark_skill` (which has
`single_target: true` in the DDGC targeting rule) still returns all 4 enemies
from `AllEnemies` resolution because truncation is never applied. The DDGC
rule exists and is correct, but the framework does not apply it.

**Impact:** The test documents the gap rather than asserting correct DDGC
behavior. Any fix to the targeting pipeline must update this test.

**Remediation direction:** Wire single-target truncation in the battle loop
or add a post-resolution step that applies DDGC targeting rules.

---

### TDRIFT-003: Magic seed to select specific encounter pack in test

**Classification:** DDGC game-layer logic gap — hard-coded internal assumption

**Detail:** `lizard_uses_deterministic_cycle_not_first_skill_fallback` in
`src/run/encounters.rs` uses `seed=41` with a comment explaining the modular
arithmetic: "BaiHu hall packs: baihu_hall_01..05. Sorted by pack ID,
seed=41, room_index=0 gives index=41%5=1 -> baihu_hall_02 (lizard)."

**Impact:** If new hall packs are added for BaiHu (changing the pack count
from 5 to 6), `seed=41` no longer selects the lizard pack and the test
silently tests the wrong pack.

**Remediation direction:** Add an assertion that the resolved pack ID matches
the expected `"baihu_hall_02"`, so a pack-count change causes a hard test
failure rather than silently testing the wrong content.

---

## 2. Scenarios (`tests/semantic_parity_trace.rs`, `src/scenarios/deferred_effect_battle.rs` tests)

### TDRIFT-004: `semantic_fixture_preserves_status_timing` is a vacuous test

**Classification:** DDGC game-layer logic gap — vacuous test assertion

**Detail:** `tests/semantic_parity_trace.rs` lines 61-89: the test body
explicitly documents that `apply_status` effects from Bone Soldier's `rend`
skill are recorded but not auto-executed by the resolver. The `if
status_ticks.is_empty()` branch is always taken, meaning the test body
executes empty. It is a placeholder that always passes.

**Impact:** No test coverage for status tick timing at all.

**Remediation direction:** Either implement status auto-execution in the
battle loop or mark this test `#[ignore]` with a tracking issue reference.

---

### TDRIFT-005: `deferred_effect_battle` scenario does not exercise `GameCondition` evaluation

**Classification:** Framework API drift — adapter gap propagating into tests
(mirrors GAP-002 in the adapter inventory)

**Detail:** The scenario's stated purpose is to test that "deferred effects
(DDGC condition tags) are properly evaluated." But its battle loop creates
`EffectContext::new(...)` without the `game_condition_evaluator`, so
`GameCondition` nodes always fail. The test
`opening_strike_deferred_effect_appears_in_trace` asserts `total_damage > 0.0`,
which passes because the unconditional 20-damage node fires, but the 20-bonus
conditional node never fires.

**Impact:** The test's acceptance criterion is weaker than the scenario's
stated goal. The scenario does not actually verify `GameCondition` evaluation.

**Remediation direction:** Wire `game_condition_evaluator` in the scenario
(see DRIFT-001 in adapter inventory) and strengthen the test assertion to
verify conditional bonus damage.

---

### TDRIFT-006: Hard-coded actor IDs in semantic parity trace tests

**Classification:** DDGC game-layer logic gap — hard-coded internal assumption

**Detail:** `tests/semantic_parity_trace.rs` lines 45-53 use `entry.actor == 1`
for Crusader and `entry.actor == 10` for Bone Soldier. These IDs are
established in `first_battle.rs` but are not exported as named constants.

**Impact:** If `run_first_battle` changes actor ID assignment, assertions
silently stop covering the intended behavior.

**Remediation direction:** Export named constants from `first_battle` module
(e.g., `pub const CRUSADER_ID: ActorId = ActorId(1)`) or use a lookup
function.

---

## 3. Hero Content (`tests/hero_base_variants.rs`, `tests/hero_white_variants.rs`, `tests/hero_black_variants.rs`)

### TDRIFT-007: `TargetSelector` missing `PartialEq` — workaround type in parity module

**Classification:** Framework API drift — missing trait derivation

**Detail:** `framework_combat::targeting::TargetSelector` derives `Debug,
Clone, Serialize, Deserialize` but NOT `PartialEq`. This forces the game layer
to define a parallel `TargetSelectorPattern` enum in `src/parity/skills.rs`
and use a paired `matches!` macro instead of `assert_eq!`.

**Impact:** `TargetSelectorPattern` is dead code if `TargetSelector` ever
gains `PartialEq`. More importantly, the `matches!` pattern only covers
`AllEnemies` and `AllAllies` — if a skill uses `SelfOnly`, `ByPosition`,
`Conditional`, or `RelativePosition`, the check produces `false` with an
unhelpful message.

**Remediation direction:** Request `PartialEq` derivation on `TargetSelector`
in the framework crate (non-breaking addition), then replace
`TargetSelectorPattern` with direct `assert_eq!`.

---

### TDRIFT-008 (FIXED): Raw string `"stress"` instead of `ATTR_STRESS` constant

**Classification:** Test hygiene — fixed in this iteration

**Detail:** `hero_base_variants.rs`, `hero_white_variants.rs`, and
`hero_black_variants.rs` used `AttributeKey::new("stress")` instead of
`AttributeKey::new(ATTR_STRESS)`. All other DDGC attributes (`ATTR_HEALTH`,
`ATTR_SPEED`, `ATTR_MAX_HEALTH`) were already referenced via their constants.
If `ATTR_STRESS` were renamed, these tests would silently query a missing
attribute and get 0.0, which equals the expected 0.0 and always passes.

**Fix applied:** Replaced with `AttributeKey::new(ATTR_STRESS)` in all three
files, added `ATTR_STRESS` to imports.

---

### TDRIFT-009 (FIXED): `display_name.contains("Hunter")` instead of `class_id == "hunter"`

**Classification:** Test hygiene — fixed in this iteration

**Detail:** `hero_base_variants.rs` used `variant.display_name.contains("Hunter")`
to detect the Hunter family. This uses the human-readable display name rather
than the canonical class ID. If `display_name` were changed to `"Ranger"` or
localized, the Hunter special-case would silently fall through.

**Fix applied:** Replaced with `variant.class_id == "hunter"`.

---

## 4. Test Fixtures (`tests/skill_semantic_parity.rs`, `tests/monster_semantic_parity.rs`, `tests/status_semantic_parity.rs`)

### TDRIFT-010: `matches!` only covers `AllEnemies` and `AllAllies` targeting

**Classification:** Framework API drift — incomplete workaround coverage

**Detail:** Consequence of TDRIFT-007. The `matches!` in
`skill_targeting_semantics_are_preserved` only handles two `TargetSelector`
variants. If a skill uses any other variant, the check produces `false`
without identifying which variant was actually present.

**Impact:** When new skills with different targeting semantics are added,
the test must be updated to handle additional variants — but the failure
message does not help identify the mismatch.

**Remediation direction:** Same as TDRIFT-007 — add `PartialEq` to
`TargetSelector` and switch to `assert_eq!`.

---

### TDRIFT-011: `ContentPack.skills` accessed as public field instead of via `get_skill` API

**Classification:** DDGC game-layer logic gap — test couples to internal storage

**Detail:** `tests/monster_semantic_parity.rs` lines 100-105 iterate
`pack.skills.values().filter(...)` to find Necromancer skills instead of
calling `pack.get_skill(&SkillId::new("skull_bash"))`. This works because
`ContentPack.skills` is `pub`, but it means the test is coupled to the
internal `HashMap` storage format.

**Impact:** If `ContentPack` changes its internal storage (e.g., to `BTreeMap`
or adds indirection), this test breaks while tests using `get_skill` do not.

**Remediation direction:** Use `get_skill` for individual lookups. Keep the
iteration pattern only where the test genuinely needs to enumerate all skills.

---

### TDRIFT-012: `StatusParityFixture` values are manually duplicated from content code

**Classification:** DDGC game-layer logic gap — fixture can drift from implementation

**Detail:** `StatusParityFixture` captures `stack_rule`, `kind`, `has_modifiers`
etc. as manually written constants. The same values are also in
`src/content/statuses.rs`. There is no compile-time linkage between fixture
and content.

**Impact:** If `statuses::bleed()` changes its `StackRule` from `Stack{max:3}`
to `Stack{max:5}`, the fixture field must be updated separately. The test
does re-assert fixture values against live results, which catches drift —
but the fixture itself can become stale if the test author forgets to update it.

**Remediation direction:** Generate fixture values from live content
constructors, or add a test that asserts fixture constants match live content.

---

## 5. Inline Test Blocks (`src/run/conditions.rs` tests, `src/run/encounters.rs` tests)

### TDRIFT-013: `IfTargetPosition` returns `Unknown` — test asserts gap is handled, not fixed

**Classification:** Framework API drift — adapter gap propagating into tests
(mirrors DRIFT-002 in the adapter inventory)

**Detail:** The `IfTargetPosition(SlotRange)` arm in `evaluate_framework`
returns `ConditionResult::Unknown`. The inline tests
`unsupported_framework_conditions_return_unknown`,
`unknown_condition_is_deterministic`, and
`unified_evaluate_propagates_unknown_from_framework` explicitly assert
`ConditionResult::Unknown`. The tests validate that the gap is handled
consistently, not that position-based conditions work.

**Impact:** Any skill that uses `EffectCondition::IfTargetPosition` has no
actual position-gating in headless mode.

**Remediation direction:** Add formation layout data to `ConditionContext`
(see DRIFT-002 in adapter inventory).

---

### TDRIFT-014: `Probability` always passes if `p > 0.0` — sub-100% chances collapsed

**Classification:** Framework API drift — deterministic approximation

**Detail:** `evaluate_framework` for `Probability(p)` returns `Pass` when
`*p > 0.0`. Tests explicitly verify this at lines 843-852. This means
`Probability(0.001)` and `Probability(1.0)` produce the same result.

**Impact:** Any skill with a sub-100% proc chance has that chance collapsed
to 100% in headless mode. Tests pass but do not reflect actual game behavior
for probabilistic effects.

**Remediation direction:** If probabilistic fidelity is needed, implement
a seeded RNG-based evaluation path. Document the current deterministic
approximation as intentional for headless mode.

---

### TDRIFT-015: `ConditionContext::new` takes ownership, forcing `.clone()` in ~15 test sites

**Classification:** Test hygiene — API friction

**Detail:** `ConditionContext::new` takes ownership of both the actors map
and the side_lookup map. Every test that needs multiple contexts from the
same base state must clone both maps. This creates ~15 clone sites in the
inline tests.

**Impact:** No correctness issue, but the clone friction discourages writing
tests with multiple condition evaluations from the same base state.

**Remediation direction:** Consider accepting `&HashMap` borrows in
`ConditionContext::new` or providing a `ConditionContext::clone_with_modification`
helper for test use.

---

## Classification Key

| Label | Meaning |
|---|---|
| **Framework API drift** | The test code's assumptions about framework types/methods diverge from the framework's actual API, or the framework is missing a capability that forces workaround code in tests |
| **DDGC game-layer logic gap** | Test assertions are weaker than intended, tests couple to internal implementation details, or tests document known gaps rather than asserting correct behavior |
| **Test hygiene** | The test code uses raw strings, inconsistent import paths, or patterns that create maintenance risk but do not affect correctness |

---

## Cross-Reference to Adapter Inventory (US-001-a)

| Tests item | Adapter item | Relationship |
|---|---|---|
| TDRIFT-005 | DRIFT-001 / GAP-001 / GAP-002 | Same root cause: `game_condition_evaluator` not wired |
| TDRIFT-013 | DRIFT-002 | Same root cause: formation layout not in `ConditionContext` |
| TDRIFT-014 | DRIFT-008 (fixed) area | Same area: `Probability` deterministic approximation |
| TDRIFT-007 | N/A (tests-only) | `TargetSelector` missing `PartialEq` |
| TDRIFT-002 | N/A (tests-only) | DDGC single-target rule not enforced |
