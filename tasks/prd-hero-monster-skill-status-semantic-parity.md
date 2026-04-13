# PRD: Hero Monster Skill Status Semantic Parity

## Introduction

This PRD derives actionable user stories from the strategic framework document (`PRD_HERO_MONSTER_SKILL_STATUS_SEMANTIC_PARITY.md`) and grounds them in the current state of `game_ddgc_headless`. The core problem: the G0–G6 migration slice produced runnable content (4 archetypes, 6 skills, 4 statuses, 1 deterministic battle), but **no mechanism exists to verify that the migrated content preserves the original game's behavioral semantics**. Without semantic parity tests, future refactors or framework changes can silently drift the game behavior away from the original, and there is no structured way to distinguish acceptable approximations from unacceptable semantic drift.

The goal is to establish a **verifiable semantic parity baseline** across the four core systems — Hero, Monster, Skill, and Status — so that future migration work proceeds under a disciplined constraint: preserve observable behavior, not class structure.

## Goals

- Define a shared terminology for "semantic parity" so all contributors judge fidelity by the same standard
- Create parity fixture modules that express original-game behavioral expectations as executable Rust code
- Create integration tests that verify each system's migrated behavior matches those expectations
- Produce a cross-system golden trace that serves as a regression baseline
- Document every known semantic gap with a classification (acceptable / deferred / unacceptable)
- Ensure no future framework refactor silently breaks parity without a test failure

## Non-Goals

- Unity UI / visual parity migration
- Full class-for-class OOP translation of the original DDGC codebase
- Numerical balance rework or rebalancing of stats
- Migrating all heroes, monsters, skills, or statuses beyond the current 4/6/4 slice
- Finalizing the abstract API shape of any framework crate

---

## User Stories

### US-201: Define semantic parity terminology

**Description:** As a developer, I need a shared vocabulary for discussing migration fidelity so that "does this count as parity?" has a clear answer, not a subjective one.

**Acceptance Criteria:**
- [ ] `SEMANTIC_PARITY.md` exists at `games/game_ddgc_headless/SEMANTIC_PARITY.md`
- [ ] Document defines: `semantic parity`, `structural translation`, `game gap`, `framework gap`, `acceptable approximation`, `unacceptable semantic drift`
- [ ] Document defines "player-observable behavior" boundary: what counts as observable (damage output, turn order, status application) vs. internal (class hierarchy, method dispatch)
- [ ] Document references `MIGRATION_BLOCKERS.md` for gap classification and avoids duplicating blocker content
- [ ] `SEMANTIC_GAPS.md` exists at `games/game_ddgc_headless/SEMANTIC_GAPS.md` with section headers for Hero, Monster, Skill, Status, and Cross-system gaps
- [ ] `SEMANTIC_GAPS.md` contains the 4 pre-identified gaps from H6 (damage range averaging, BuffRule condition downgrade, reactive hooks in game layer, skill usage limits in game state) as initial entries
- [ ] Typecheck/lint passes

---

### US-202: Create hero semantic parity fixtures and tests

**Description:** As a developer, I need parity fixtures and tests that verify hero archetypes preserve their original role identity, position preference, and resource semantics rather than being flattened into interchangeable actor templates.

**Acceptance Criteria:**
- [ ] `games/game_ddgc_headless/src/parity/heroes.rs` exists and exports a `HeroParityFixture` struct containing the Crusader and Vestal parity expectations
- [ ] `games/game_ddgc_headless/src/parity/mod.rs` exists and re-exports `heroes`
- [ ] `games/game_ddgc_headless/src/lib.rs` includes `pub mod parity;`
- [ ] `games/game_ddgc_headless/tests/hero_semantic_parity.rs` exists with the following tests:
  - [ ] `hero_role_identity_is_preserved` — Crusader has Ally side, HP > 25, DEF > 3, SPD < 6; Vestal has Ally side, HP < 30, ATK < 10, SPD > 6
  - [ ] `hero_position_preference_is_preserved` — Crusader's archetype data encodes front-line preference (defense >= 5, speed <= 5); Vestal's archetype data encodes back-line preference (defense == 0, speed >= 8)
  - [ ] `hero_resource_semantics_are_preserved` — Both heroes start with stress = 0, max_stress = 200, and `effective_attribute(ATTR_STRESS)` returns 0.0 after construction
  - [ ] `hero_skill_access_constraints_match_original_intent` — Crusader can access `crusading_strike` and `holy_lance` from `ContentPack`; Vestal can access `divine_grace`; neither can access enemy-only skills (`rend`, `skull_bash`, `grave_bash`)
- [ ] Typecheck/lint passes

---

### US-203: Create monster semantic parity fixtures and tests

**Description:** As a developer, I need parity fixtures and tests that verify monster archetypes preserve their original threat model, position logic, and behavioral differentiation — not just their stat blocks.

**Acceptance Criteria:**
- [ ] `games/game_ddgc_headless/src/parity/monsters.rs` exists and exports a `MonsterParityFixture` struct containing Bone Soldier and Necromancer parity expectations
- [ ] `games/game_ddgc_headless/src/parity/mod.rs` includes `monsters` module
- [ ] `games/game_ddgc_headless/tests/monster_semantic_parity.rs` exists with the following tests:
  - [ ] `monster_role_identity_is_preserved` — Bone Soldier has Enemy side, HP < 25, ATK between 6–10, SPD > 5 (fragile but fast); Necromancer has Enemy side, HP > 40, ATK > 10, SPD < 5 (tanky boss)
  - [ ] `monster_skill_preference_is_preserved` — Bone Soldier's primary skill (`rend`) applies bleed status, confirming DoT threat identity; Necromancer's primary skill (`skull_bash`) has a conditional stun, confirming control-threat identity
  - [ ] `boss_behavior_is_not_flattened` — Necromancer has at least 2 distinct skills with different tactical roles (damage + stun vs. multi-hit), and its HP is > 2x Bone Soldier's HP, and its cooldown skill has cooldown >= 3, confirming boss-level threat differentiation
  - [ ] `monster_position_logic_matches_original_intent` — Bone Soldier uses `AllEnemies` targeting (frontline brawler); Necromancer uses `AllEnemies` targeting with multi-hit capability (backline boss). Both have stat profiles consistent with their implied position (Bone Soldier: moderate dodge, Necromancer: 0 dodge)
- [ ] Typecheck/lint passes

---

### US-204: Create skill semantic parity fixtures and tests

**Description:** As a developer, I need parity fixtures and tests that verify migrated skills preserve their original targeting semantics, position constraints, effect chains, and usage restrictions — not just their damage numbers.

**Acceptance Criteria:**
- [ ] `games/game_ddgc_headless/src/parity/skills.rs` exists and exports a `SkillParityFixture` struct containing expectations for all 6 migrated skills
- [ ] `games/game_ddgc_headless/src/parity/mod.rs` includes `skills` module
- [ ] `games/game_ddgc_headless/tests/skill_semantic_parity.rs` exists with the following tests:
  - [ ] `skill_targeting_semantics_are_preserved` — `crusading_strike` targets enemies; `divine_grace` targets allies; `holy_lance` targets enemies; `rend` targets enemies; `skull_bash` targets enemies; `grave_bash` targets enemies. Each test asserts `skill.target_selector` variant matches expected side
  - [ ] `skill_position_constraints_are_preserved` — Crusader's melee skill (`crusading_strike`) has `action_cost == 1` and no cooldown (frontline spam); Holy Lance has `cooldown == Some(2)` (positional cooldown skill). Vestal's heal has no cooldown (support always available). Necromancer's stun has `cooldown == Some(3)` (boss cooldown)
  - [ ] `skill_effect_chain_semantics_are_preserved` — `crusading_strike` has exactly 1 effect (pure damage); `holy_lance` has exactly 2 effects (damage + heal, confirming combined offense/self-sustain); `rend` has exactly 2 effects (damage + status, confirming DoT application); `skull_bash` has exactly 2 effects (damage + conditional status, confirming control threat); `grave_bash` has exactly 2 effects (damage + damage, confirming multi-hit)
  - [ ] `skill_usage_restrictions_match_original_intent` — All 6 skills pass `skill.validate()`. Cooldown skills (`holy_lance`, `skull_bash`) have `cooldown.is_some()`. Non-cooldown skills have `cooldown.is_none()`. All skills have `action_cost >= 1`
- [ ] Typecheck/lint passes

---

### US-205: Create status semantic parity fixtures and tests

**Description:** As a developer, I need parity fixtures and tests that verify migrated statuses preserve their stacking rules, tick timing, reactive semantics, and resource interactions — not just their modifier values.

**Acceptance Criteria:**
- [ ] `games/game_ddgc_headless/src/parity/statuses.rs` exists and exports a `StatusParityFixture` struct containing expectations for all 4 migrated statuses
- [ ] `games/game_ddgc_headless/src/parity/mod.rs` includes `statuses` module
- [ ] `games/game_ddgc_headless/tests/status_semantic_parity.rs` exists with the following tests:
  - [ ] `status_stack_rules_are_preserved` — `bleed` uses `StackRule::Stack { max: 3 }` and 3 stacks accumulate correctly (effective HP reduced by 3x tick damage). `stun` uses `StackRule::Replace` and re-applying stun replaces (doesn't stack). `riposte` uses `StackRule::Replace`. `horror` uses `StackRule::Stack { max: 3 }` and 3 stacks accumulate correctly (effective stress increased by 3x tick value)
  - [ ] `status_tick_timing_is_preserved` — A bleed(5.0, 3) applied to an actor with 100 HP: after `statuses.tick()`, duration decreases by 1, and the status is still active (not expired). After 3 ticks, the bleed status has expired and is returned by `tick()`. A stun(1) expires after exactly 1 tick
  - [ ] `reactive_status_semantics_are_preserved` — `riposte` is a marker status with no modifiers (kind = "riposte", `modifiers.is_empty()`, `StackRule::Replace`). Its reactive behavior is documented as a game-gap in `SEMANTIC_GAPS.md` (B-008): game-layer code must check for `StatusKind::new("riposte")` after each damage event and apply the counter-attack. Test verifies the marker is present and detectable via `statuses.active()` iteration
  - [ ] `status_interacts_with_resources_as_expected` — `bleed(5.0, 3)` reduces `effective_attribute(ATTR_HEALTH)` by 5.0 per stack. `horror(10.0, 3)` increases `effective_attribute(ATTR_STRESS)` by 10.0 per stack. When bleed and horror are both active on the same actor, both attribute modifications apply simultaneously (HP reduced AND stress increased)
- [ ] Typecheck/lint passes

---

### US-206: Create cross-system golden trace and regression baseline

**Description:** As a developer, I need a deterministic battle trace that exercises hero, monster, skill, and status systems together so that future changes that break cross-system parity are caught by a regression test.

**Acceptance Criteria:**
- [ ] `games/game_ddgc_headless/fixtures/semantic_battles/` directory exists with at least one JSON fixture file (e.g., `first_battle_trace.json`)
- [ ] `games/game_ddgc_headless/tests/semantic_parity_trace.rs` exists with the following tests:
  - [ ] `semantic_fixture_battle_trace_is_stable` — Running `run_first_battle()` twice produces byte-identical JSON traces. The golden trace JSON is checked into `fixtures/semantic_battles/first_battle_trace.json` and the test asserts the live trace matches it
  - [ ] `semantic_fixture_preserves_role_and_skill_identity` — In the golden trace, Crusader uses only Crusader skills (`crusading_strike`), Bone Soldier uses only Bone Soldier skills (`rend`), and no actor uses a skill from the wrong side (no ally using enemy skills or vice versa)
  - [ ] `semantic_fixture_preserves_status_timing` — In the golden trace, if bleed is applied, subsequent turns show status_tick events that reduce the affected actor's HP. The trace records these as `status_tick` actions with non-zero damage values
- [ ] The golden trace JSON file is committed to git and treated as a regression baseline (not regenerated on each test run)
- [ ] Typecheck/lint passes

---

### US-207: Document semantic gaps with classification and acceptable approximation boundaries

**Description:** As a developer, I need a structured ledger of every known semantic difference between the original DDGC game and the migrated headless version, classified by severity, so that I don't mistake a known approximation for full parity.

**Acceptance Criteria:**
- [ ] `SEMANTIC_GAPS.md` has entries for all 4 pre-identified gaps, each with:
  - [ ] **Gap ID** (e.g., SG-001, SG-002, ...)
  - [ ] **Classification**: `acceptable approximation`, `deferred parity work`, or `unacceptable semantic drift`
  - [ ] **Description**: what the original game does vs. what the migration does
  - [ ] **Reason**: why the classification was chosen
  - [ ] **Tracking**: link to `MIGRATION_BLOCKERS.md` blocker ID if applicable
- [ ] The 4 initial gaps are classified as follows:
  - [ ] SG-001: Damage range averaging (`B-006`) — classified as `acceptable approximation` with reason "first slice uses fixed averages; variance can be restored via game-layer damage roll without changing parity test structure"
  - [ ] SG-002: BuffRule condition system downgrade (`B-004`) — classified as `deferred parity work` with reason "framework's `EffectCondition` covers 4 of 35+ DDGC variants; remaining conditions require game-layer filtering that hasn't been implemented yet"
  - [ ] SG-003: Reactive hooks in game layer (`B-008`) — classified as `acceptable approximation` with reason "riposte/guard are implemented as marker statuses detectable by game-layer code; the reactive trigger itself is a game-gap, not a semantic gap"
  - [ ] SG-004: Skill usage limits in game state (`B-005`) — classified as `deferred parity work` with reason "LimitPerTurn/LimitPerBattle tracking doesn't exist yet; once implemented, parity tests should verify usage enforcement"
- [ ] Any future semantic gap discovered during parity test implementation must be added to `SEMANTIC_GAPS.md` before the PR is merged
- [ ] Typecheck/lint passes

---

## Functional Requirements

- FR-1: `SEMANTIC_PARITY.md` must define 6 core terms: semantic parity, structural translation, game gap, framework gap, acceptable approximation, unacceptable semantic drift
- FR-2: `SEMANTIC_GAPS.md` must contain at least 4 gap entries with ID, classification, description, reason, and blocker tracking
- FR-3: `src/parity/heroes.rs` must export a `HeroParityFixture` containing role, position, resource, and skill-access expectations for Crusader and Vestal
- FR-4: `src/parity/monsters.rs` must export a `MonsterParityFixture` containing role, skill preference, boss differentiation, and position expectations for Bone Soldier and Necromancer
- FR-5: `src/parity/skills.rs` must export a `SkillParityFixture` containing targeting, position constraint, effect chain, and usage restriction expectations for all 6 migrated skills
- FR-6: `src/parity/statuses.rs` must export a `StatusParityFixture` containing stack rule, tick timing, reactive marker, and resource interaction expectations for bleed, stun, riposte, and horror
- FR-7: All parity fixture data must be expressed in terms of the current framework's public API types (`Archetype`, `SkillDefinition`, `StatusEffect`, `StackRule`, etc.) — no reference to original DDGC OOP types
- FR-8: Integration tests in `tests/` must import from `game_ddgc_headless::parity::*` and `game_ddgc_headless::content::*` — they must not duplicate fixture data
- FR-9: Golden trace fixture (`first_battle_trace.json`) must be a committed JSON file that the regression test loads and compares against
- FR-10: No test in the parity test suite may reference framework crate internal APIs — only public re-exports from `game_ddgc_headless`

## Non-Goals

- No migration of additional heroes, monsters, skills, or statuses beyond the current 4/6/4 slice
- No framework crate modifications — all parity work stays in `game_ddgc_headless`
- No numerical rebalancing — parity tests verify that migrated values match the averaging conventions established in G2/G3, not that they match original DDGC ranges
- No UI or visual regression testing
- No performance benchmarks or load testing
- No change to the existing `ContentPack`, `BattleTrace`, or `first_battle` scenario code — parity fixtures and tests read from existing content, they don't modify it

## Design Considerations

### Parity Fixture Structure

Each fixture module (`heroes.rs`, `monsters.rs`, `skills.rs`, `statuses.rs`) should contain:
1. A fixture struct (e.g., `HeroParityFixture`) with named fields for each expectation
2. A `Default` impl that populates the fixture from `ContentPack::default()` and the content factory functions — this couples fixture expectations to actual migrated content, making drift impossible
3. Helper methods that return bool/vec for test assertions

This design ensures that if content changes without updating the fixture, the `Default` impl either panics (missing content) or the test fails (changed values).

### Golden Trace Format

The golden trace is a `BattleTrace` serialized to JSON via `serde_json::to_string_pretty()`. This format is already used in the `first_battle_trace_is_deterministic` test. The parity trace test should:
1. Load `fixtures/semantic_battles/first_battle_trace.json`
2. Run `run_first_battle()` to produce a live trace
3. Assert `live_trace.to_json() == golden_trace_json`

This makes the golden trace a committed artifact that fails the build on any behavioral change.

### Test Organization

```
games/game_ddgc_headless/
├── src/parity/
│   ├── mod.rs          # re-exports
│   ├── heroes.rs       # HeroParityFixture
│   ├── monsters.rs     # MonsterParityFixture
│   ├── skills.rs       # SkillParityFixture
│   └── statuses.rs     # StatusParityFixture
├── tests/
│   ├── hero_semantic_parity.rs
│   ├── monster_semantic_parity.rs
│   ├── skill_semantic_parity.rs
│   ├── status_semantic_parity.rs
│   └── semantic_parity_trace.rs
├── fixtures/
│   └── semantic_battles/
│       └── first_battle_trace.json
├── SEMANTIC_PARITY.md
└── SEMANTIC_GAPS.md
```

## Technical Considerations

- The current `ContentPack::default()` registers all 4 archetypes and 6 skills. Parity fixtures should derive expectations from `ContentPack::default()` rather than hardcoding values, so that content changes automatically update fixture expectations.
- The `BattleTrace` struct already supports JSON serialization via `serde`. The golden trace should be generated by running `run_first_battle().trace.to_json()` once and committing the result.
- Integration tests in `tests/` run against the library crate. They import via `use game_ddgc_headless::...`.
- The `Archetype` struct's `side: CombatSide` field is the sole mechanism for hero/monster differentiation. Parity tests should assert `side == CombatSide::Ally` for heroes and `side == CombatSide::Enemy` for monsters.
- Position preference is currently implicit in stat profiles (e.g., Crusader's high defense and low speed imply frontline). The parity tests encode these implicit expectations as explicit assertions. When `PositionRule`-based launch ranks are added to skills (per B-007), these tests should be updated.
- Status tick timing in the current framework is handled by `CombatResolver::end_turn()` calling `statuses.tick()`. The parity tests should test tick timing directly via `StatusContainer::tick()` to avoid coupling to the resolver's turn loop.

## Success Metrics

- All 4 parity test files (`hero_semantic_parity.rs`, `monster_semantic_parity.rs`, `skill_semantic_parity.rs`, `status_semantic_parity.rs`) pass with specific, named test functions
- Golden trace regression test passes and the committed JSON matches live execution
- `SEMANTIC_PARITY.md` and `SEMANTIC_GAPS.md` are complete and referenced in `MIGRATION_MAP.md`
- No parity test imports framework crate internals — all imports go through `game_ddgc_headless::content` and `game_ddgc_headless::parity`
- Any future change to migrated content (stats, skill effects, status rules) that breaks a parity test requires updating the fixture AND the gap ledger, not just the test

## Open Questions

- **Position enforcement:** Should parity tests assert position *constraints* on skills (e.g., Crusader can only use `crusading_strike` from rank 1–2) once `TargetSelector::ByPosition` or `PositionRule` is adopted? Currently all skills use `AllEnemies`/`AllAllies` (see B-006/B-007). Answer: defer to the skill parity test update when position-based targeting is implemented.
- **Fixture data source:** Should `HeroParityFixture::default()` read from `ContentPack` (coupled to content) or define independent expected values (decoupled but risks drift)? Answer: use `ContentPack::default()` for archetype data and factory functions for skill/status data, so fixtures auto-update with content changes.
- **Golden trace scope:** Should the golden trace include status tick events (currently not recorded by `run_first_battle`)? Answer: the current `first_battle` scenario doesn't apply bleed/stun/horror, so the trace won't include status ticks. A richer scenario that exercises statuses should be added as a separate fixture when reactive hooks are implemented (B-008).