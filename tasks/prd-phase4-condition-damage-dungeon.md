# PRD: Phase 4 — DDGC Condition Closure, Damage Variance, and Dungeon Fidelity

**Project:** game_ddgc_headless
**Branch:** `ralph/ddgc-phase4-condition-damage-dungeon`
**Depends on:** Phase 3 (encounter-runtime fidelity) — closed

## Goal

Close the last partially-resolved migration blocker (B-004), activate damage variance, and align dungeon map generation with DDGC's original parameters. After Phase 4, the headless run produces a combat loop whose condition coverage, damage distribution, and dungeon structure match the original game.

---

## Phase 4a: Condition System Closure (B-004)

### Context

B-004 is the only blocker still marked "Partially resolved." The `ConditionAdapter` in `src/run/conditions.rs` covers 6 DDGC conditions and 3 framework-native bridges. The original `BuffRule` system has 35+ condition variants; the most impactful missing ones are HP-threshold conditions and dungeon-mode conditions.

### US-801: P4a-0: Inventory remaining condition touchpoints

**Priority:** 1

As a developer, I need a precise inventory of which migrated DDGC skills and statuses depend on unimplemented condition variants so Phase 4a targets only the conditions with real migrated content.

**Acceptance Criteria:**
- A checked-in inventory (in SEMANTIC_GAPS.md or a dedicated file) lists every migrated skill/status that references a condition not yet implemented in `ConditionAdapter`
- The inventory maps each unimplemented condition to the skills that require it, grouped by condition family (HP-threshold, dungeon-mode, kill-trigger, etc.)
- At least 2 condition families are selected as implementation anchors with concrete skill anchors

**Notes:** Discovery slice. Scan `src/content/heroes/` and `src/content/monsters/` for `.with_game_condition()` calls that reference unimplemented tags.

---

### US-802: P4a-1: Implement HP-threshold conditions

**Priority:** 2

As a developer, I need `HpAbove`, `TargetHpAbove`, and `TargetHpBelow` DDGC conditions implemented so HP-threshold skill gating works correctly instead of being always-on or always-off.

**Acceptance Criteria:**
- `ConditionAdapter` evaluates `ddgc_hp_above_<threshold>`, `ddgc_target_hp_above_<threshold>`, and `ddgc_target_hp_below_<threshold>` condition tags
- HP-threshold evaluation reads `ATTR_HEALTH` / `ATTR_MAX_HEALTH` from the appropriate actor/target and compares against the fractional threshold
- At least one migrated or fixture skill uses an HP-threshold condition through the adapter
- Focused tests prove: (a) condition passes above threshold, (b) fails below threshold, (c) boundary at exact threshold is deterministic
- `MIGRATION_BLOCKERS.md` B-004 status is updated

**Notes:** Primary anchor: any boss or hero skill that applies bonus effects when HP is above/below a threshold. `ConditionContext` already exposes `actors` HashMap — read HP attributes directly.

---

### US-803: P4a-2: Implement dungeon-mode condition

**Priority:** 3

As a developer, I need the `InMode` DDGC condition implemented so skills that behave differently in specific dungeon modes (e.g., empowered in XuanWu, weakened in BaiHu) can diverge correctly.

**Acceptance Criteria:**
- `ConditionAdapter` evaluates `ddgc_in_mode_<mode>` condition tags
- Mode resolution reads from `ConditionContext::dungeon` (already present) and any game-layer mode state
- At least one fixture skill uses an InMode condition
- Focused test proves the condition passes when the current dungeon/mode matches and fails otherwise
- `MIGRATION_BLOCKERS.md` B-004 status is updated to reflect InMode implementation

**Notes:** DDGC's `InMode` is tied to dungeon-specific battle modifiers (e.g., `in_black_tortoise_field`). The `ConditionContext` already carries `dungeon: Dungeon`; extend with an optional mode flag.

---

### US-804: P4a-3: Implement kill-trigger condition

**Priority:** 4

As a developer, I need an `OnKill` DDGC condition implemented so skills that trigger bonus effects on enemy death (e.g., necromancer corpse interaction, shaman soul harvest) can execute correctly.

**Acceptance Criteria:**
- `ConditionAdapter` evaluates a `ddgc_on_kill` condition tag
- Kill detection reads from a game-layer kill event tracker that records which actors died this turn
- At least one fixture or migrated skill uses the OnKill condition
- Focused test proves: (a) condition passes on a turn where the actor killed an enemy, (b) fails when no kill occurred

**Notes:** Requires a lightweight kill-event accumulator in the game layer — similar pattern to `UsageCounters`. Add `KillTracker` to `src/run/`.

---

### US-805: P4a-4: Update condition coverage and close B-004

**Priority:** 5

As a developer, I need the condition system's coverage documented and B-004 fully resolved so no further condition work is tracked as a blocker.

**Acceptance Criteria:**
- SEMANTIC_GAPS.md SG-002 (BuffRule condition downgrade) is updated to reflect all newly implemented conditions
- MIGRATION_BLOCKERS.md B-004 status is changed from "Partially resolved" to "Resolved"
- A condition coverage table lists all implemented DDGC conditions vs. the original 35+ variants, with remaining ones classified as "low-impact deferred" or "not used in migrated content"
- `cargo test` passes with full condition coverage

**Notes:** Closeout slice. Remaining unimplemented conditions (if any) should be documented as low-priority, not blockers.

---

## Phase 4b: Damage Variance Activation (B-006)

### Context

`DamagePolicy` and `DamageRange` exist in `src/run/damage_policy.rs` with `FixedAverage` (default) and `Rolled` variants. All 111 hero skills and all monster skills currently use fixed-average damage. The `Rolled` policy exists but is not wired into the battle loop.

### US-806: P4b-0: Wire DamageRange into migrated skill definitions

**Priority:** 6

As a developer, I need migrated skill definitions to carry `DamageRange` (min/max) metadata instead of pre-averaged values so the damage policy can choose between fixed and rolled modes.

**Acceptance Criteria:**
- All hero and monster skill definitions that deal damage store `DamageRange::new(min, max)` alongside the current `EffectNode::damage(average)` call
- A lookup or mapping exists from skill ID to its `DamageRange`
- Existing tests continue passing under `FixedAverage` policy (no behavior change)
- At least 5 migrated skills have verified min/max values matching DDGC source data

**Notes:** Do not change `EffectNode::damage()` calls yet. Add a parallel `DamageRange` registry keyed by skill ID.

---

### US-807: P4b-1: Activate Rolled damage policy in battle loop

**Priority:** 7

As a developer, I need the `Rolled` damage policy active in the battle loop so combat produces variance matching the original DDGC damage range behavior.

**Acceptance Criteria:**
- `EncounterResolver` or battle loop accepts a `DamagePolicy` parameter
- When `DamagePolicy::Rolled` is selected, each damage effect resolves to a value within `[min, max]` using a seeded RNG
- When `DamagePolicy::FixedAverage` is selected, behavior is identical to current (all existing tests pass unchanged)
- A deterministic test using a fixed seed proves rolled damage stays within range bounds
- A variance test proves rolled damage distribution includes both near-min and near-max values over sufficient samples

**Notes:** The `Rolled` policy must use the same seed source as other deterministic systems (encounter selection, room generation). Use `DamageRange::roll(&mut rng)`.

---

### US-808: P4b-2: Update damage variance semantic gap status

**Priority:** 8

As a developer, I need SG-001 (damage range averaging) reclassified so the semantic gap ledger accurately reflects that variance is restored.

**Acceptance Criteria:**
- SEMANTIC_GAPS.md SG-001 classification is updated from "Acceptable approximation" to "Resolved — Rolled policy available"
- MIGRATION_BLOCKERS.md B-006 notes updated to reflect `DamageRange` wired into all skill definitions
- `cargo test` passes with both `FixedAverage` and `Rolled` modes

**Notes:** Closeout slice for B-006.

---

## Phase 4c: Dungeon Map Fidelity

### Context

The current run flow uses `DefaultRoomGenerator` with generic parameters. DDGC's `MapGenerator.txt` defines per-dungeon map parameters (room count, corridor count, connectivity, curio/trap densities, grid size). The dungeon `.bytes` files define encounter packs with weighted monster compositions per room type.

### US-809: P4c-0: Parse DDGC MapGenerator parameters

**Priority:** 9

As a developer, I need the DDGC map generation parameters parsed and available at runtime so each dungeon generates maps matching the original's structure.

**Acceptance Criteria:**
- A `DungeonMapConfig` struct exists with fields matching `MapGenerator.txt`: `base_room_number`, `base_corridor_number`, `gridsize`, `connectivity`, `min_final_distance`, hallway densities (battle/curio/hunger), room densities (battle/curio/treasure/guarded_curio/guarded_treasure)
- 4 dungeon configs are defined (QingLong, BaiHu, ZhuQue, XuanWu) with parameters extracted from `MapGenerator.txt`
- At least 2 size variants per dungeon (short, medium) are supported
- Focused test proves each dungeon config matches the corresponding `MapGenerator.txt` values

**Notes:** Data-model slice only. Do not integrate with `DefaultRoomGenerator` yet.

---

### US-810: P4c-1: Wire dungeon map configs into room generation

**Priority:** 10

As a developer, I need `DungeonMapConfig` parameters fed into the framework's room generation so generated maps reflect DDGC's dungeon structure.

**Acceptance Criteria:**
- `Run` or `Floor` creation reads the appropriate `DungeonMapConfig` for the current dungeon
- Room generation uses per-dungeon room/corridor counts, connectivity, and grid size
- A deterministic test proves: (a) QingLong maps have the correct room count, (b) BaiHu maps have lower connectivity than ZhuQue, (c) short vs medium maps differ in room count
- `cargo test` passes with DDGC-configured generation

**Notes:** May require extending `DefaultRoomGenerator` parameters or using a custom `RoomGenerator` impl. Prefer configuration over code changes.

---

### US-811: P4c-2: Parse DDGC encounter pack weights from dungeon .bytes

**Priority:** 11

As a developer, I need the encounter pack weights from DDGC dungeon `.bytes` files available at runtime so monster composition in rooms matches the original game.

**Acceptance Criteria:**
- A `DungeonEncounterConfig` struct captures the weighted encounter definitions from dungeon `.bytes` files
- Each dungeon's hall/room/boss encounter packs are registered with correct monster IDs and chances
- At least one dungeon (QingLong) has fully parsed encounter data matching its `.bytes` source
- Focused test proves encounter selection produces the expected monster composition for a given seed

**Notes:** The dungeon `.bytes` format uses `mash:` blocks with `.chance` and `.types` fields. Parse into the existing `EncounterPackRegistry` structure.

---

### US-812: P4c-3: Validate dungeon fidelity end-to-end

**Priority:** 12

As a developer, I need an end-to-end run slice that uses DDGC-configured dungeon maps and encounter packs so the full Phase 4c integration is verified.

**Acceptance Criteria:**
- A run slice using a DDGC dungeon (e.g., QingLong short) completes with: correct room count, correct encounter types, correct monster families per room
- The run trace records dungeon type, map parameters, and encounter composition
- Existing run slices continue passing (backward compatibility)
- `cargo test` passes with DDGC dungeon configs active

**Notes:** Integration validation slice. This is the Phase 4c closeout.
