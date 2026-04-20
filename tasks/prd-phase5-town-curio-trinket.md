# PRD: Phase 5 — DDGC Town Meta-Loop, Curio Interaction, and Equipment Systems

**Project:** game_ddgc_headless
**Branch:** `ralph/ddgc-phase5-town-curio-equipment`
**Depends on:** Phase 4 (condition closure, damage variance, dungeon fidelity)

## Goal

Implement the DDGC meta-game loop: town buildings, curio/trap/obstacle interactions during dungeon runs, and the trinket/equipment system that drives hero stat progression. After Phase 5, a headless run produces a full gameplay cycle: town preparation → dungeon run with room interactions → town recovery → repeat.

---

## Phase 5a: Curio, Trap, and Obstacle System

### Context

DDGC's dungeon rooms and corridors contain interactable objects (curios), traps, and obstacles. The current `RoomKind` enum supports `Event`, `Treasure`, `Shop`, `Boss`, `Corridor`, and `Custom` but has no interaction mechanics. Curios use weighted result tables with item-interaction overrides; traps use 3-tier difficulty scaling; obstacles use a uniform pattern (5% HP, -20 torchlight, Stress 2).

### US-901: P5a-0: Define curio/trap/obstacle data model

**Priority:** 1

As a developer, I need data structures for curios, traps, and obstacles so the game layer can represent room and corridor interactions beyond combat.

**Acceptance Criteria:**
- `CurioDefinition` struct exists with: `id`, `dungeon_scope` (which dungeons it appears in), `results: Vec<CurioResult>`, `item_interactions: Vec<ItemInteraction>`
- `CurioResult` struct: `weight`, `chance`, `result_type` (Nothing/Loot/Quirk/Effect/Purge/Scouting/Teleport/Disease), `result_id`
- `ItemInteraction` struct: `item_id`, `overrides_result_id`
- `TrapDefinition` struct: `id`, `success_effects`, `fail_effects`, `health_fraction`, `difficulty_variations`
- `ObstacleDefinition` struct: `id`, `fail_effects`, `health_fraction`, `torchlight_penalty`
- All structs derive `Serialize`/`Deserialize`/`Clone`/`Debug`
- Focused test proves struct construction and serialization are deterministic

**Notes:** Data-model slice only. Match the schemas observed in `Curios.csv`, `Traps.json`, and `Obstacles.json`.

---

### US-902: P5a-1: Parse DDGC curio/trap/obstacle definitions

**Priority:** 2

As a developer, I need the DDGC curio, trap, and obstacle data parsed into the data model so actual game content drives interactions.

**Acceptance Criteria:**
- A `CurioRegistry` holds all DDGC curio definitions parsed from `Curios.csv`
- A `TrapRegistry` holds all DDGC trap definitions parsed from `Traps.json`
- An `ObstacleRegistry` holds all DDGC obstacle definitions parsed from `Obstacles.json`
- At least 3 curios per dungeon region are parsed (QingLong, BaiHu, ZhuQue, XuanWu)
- All 3 trap types and all 5 obstacle types are parsed
- Focused test proves: (a) registry lookup by ID works, (b) dungeon-scoped curio filtering works, (c) item interaction overrides are preserved

**Notes:** Curios.csv uses a multi-row block format per curio — requires custom parsing. Traps.json and Obstacles.json are standard JSON.

---

### US-903: P5a-2: Implement curio interaction resolution

**Priority:** 3

As a developer, I need a deterministic curio interaction resolver so room exploration produces outcomes matching DDGC's weighted result system.

**Acceptance Criteria:**
- `resolve_curio_interaction(curio_id, has_item, item_id, seed)` returns a `CurioInteractionOutcome`
- Without item: selects result from weighted result table using seeded probability
- With item: checks `item_interactions` for override; if found, uses override result; otherwise falls back to weighted table
- `CurioInteractionOutcome` includes: `result_type`, `result_id`, `applied_effects: Vec<String>`
- Focused test proves: (a) bare interaction follows weighted probabilities, (b) item interaction produces override result, (c) results are deterministic for a given seed

**Notes:** DDGC curios have `RESULT TYPES` like `Loot`, `Quirk`, `Effect`, `Disease` — the resolver dispatches by type but actual effect application (e.g., applying a quirk) is deferred to the relevant subsystem.

---

### US-904: P5a-3: Implement trap and obstacle resolution

**Priority:** 4

As a developer, I need deterministic trap and obstacle interaction so corridor and room hazards produce correct damage and debuff outcomes.

**Acceptance Criteria:**
- `resolve_trap_interaction(trap_id, trap_level, resist_chance, seed)` returns `TrapOutcome` (Success or Fail with effects and HP fraction)
- Trap difficulty variations are applied based on dungeon level (3 or 5)
- `resolve_obstacle_interaction(obstacle_id)` returns `ObstacleOutcome` with fail effects, HP fraction, and torchlight penalty
- Focused tests prove: (a) trap succeeds with resist chance, (b) trap fails with correct difficulty-scaled effects, (c) obstacles apply uniform penalty pattern

**Notes:** Traps have a resist/disarm check; obstacles always apply their fail effects. DDGC currently sets hallway trap/obstacle density to 0 in map generation, but the data and resolution logic must exist for completeness.

---

### US-905: P5a-4: Wire curio/trap/obstacle into room exploration

**Priority:** 5

As a developer, I need room exploration events to trigger curio, trap, and obstacle interactions so the dungeon run produces the full exploration cycle (combat + interaction).

**Acceptance Criteria:**
- `RoomKind::Event` rooms can carry a `curio_id` reference
- `RoomKind::Corridor` rooms can carry optional `trap_id` and `curio_id`
- Run flow: entering a room with a curio/obstacle produces an interaction outcome in the run trace
- Run flow: traversing a corridor with a trap produces a trap outcome in the run trace
- At least one run slice produces curio interaction and trap interaction entries
- `cargo test` passes with interaction events in the trace

**Notes:** Integration slice. Room generation from Phase 4c should assign curios/traps based on `DungeonMapConfig` hallway/room densities.

---

## Phase 5b: Town Building System

### Context

DDGC has 10 town buildings that form the meta-progression loop between dungeon runs. Buildings offer services (stress heal, recruit heroes, upgrade skills, buy trinkets) with upgrade trees that unlock capacity and quality. The current run flow goes directly into dungeons with no town phase.

### US-906: P5b-0: Define town building data model

**Priority:** 6

As a developer, I need data structures for town buildings, their upgrade trees, and their services so the game layer can represent the between-run meta state.

**Acceptance Criteria:**
- `TownBuilding` struct: `id`, `building_type` (Abbey/Blacksmith/StageCoach/Guild/NomadWagon/Sanitarium/Tavern/Garden/LegacyTower/CampingTrainer), `unlock_conditions`, `upgrade_trees: Vec<UpgradeTree>`
- `UpgradeTree` struct: `tree_id`, `levels: Vec<UpgradeLevel>`
- `UpgradeLevel` struct: `code` (a–g), `cost`, `effects` (building-specific: slot count, stress heal range, recruit quality, etc.)
- `TownState` struct: tracks current upgrade level per building, gold, heirloom currencies
- All structs derive `Serialize`/`Deserialize`/`Clone`/`Debug`
- Focused test proves `TownState` construction and upgrade application are deterministic

**Notes:** Data-model slice. DDGC buildings have variable upgrade structures — Abbey has 3 activity sub-trees, StageCoach has 3 parallel upgrade trees (roster, recruit count, recruit quality).

---

### US-907: P5b-1: Parse DDGC building definitions

**Priority:** 7

As a developer, I need DDGC building data parsed into the town model so actual building parameters drive the meta loop.

**Acceptance Criteria:**
- A `BuildingRegistry` holds all 10 DDGC building definitions parsed from `*.building.json` files
- Each building has correct: unlock conditions, upgrade trees, service parameters
- At least 3 buildings (StageCoach, Abbey, Guild) are fully parsed with all upgrade levels
- Focused test proves: (a) building lookup by ID works, (b) upgrade tree traversal produces correct cost/effect at each level, (c) all 10 buildings are loadable

**Notes:** Each `.building.json` has a unique structure — Abbey has 3 activities, StageCoach has parallel trees. The parser must handle building-specific fields generically.

---

### US-908: P5b-2: Implement town visit cycle

**Priority:** 8

As a developer, I need a town visit phase that occurs between dungeon runs so the meta-game loop (town → dungeon → town) is complete.

**Acceptance Criteria:**
- `TownVisit` struct represents a single town phase with: available services, hero roster state, gold/heirloom balances
- `perform_town_activity(building_id, activity, hero_id, upgrade_level)` resolves a building service (stress heal, recruit, upgrade skill, etc.)
- Town visit produces a trace of activities performed
- At least one end-to-end test proves: (a) entering town after a dungeon run, (b) performing a stress heal at the Abbey, (c) hero stress is reduced, (d) gold is deducted
- Town visit is deterministic for given state and activity choices

**Notes:** Start with Abbey (stress heal) and StageCoach (recruit) as the two most critical buildings. Other buildings can be added incrementally.

---

### US-909: P5b-3: Wire town visit into run flow

**Priority:** 9

As a developer, I need the run flow to include town visits between dungeon runs so the full game loop operates headlessly.

**Acceptance Criteria:**
- After a dungeon run completes, a town visit phase begins
- Town visit reads: hero roster (surviving heroes, their HP/stress), gold, heirloom currencies
- Town visit allows at least: Abbey stress heal, StageCoach recruit
- Run trace records town activities alongside dungeon run events
- A full-loop run slice (town → dungeon → town) completes with heroes progressing between runs
- `cargo test` passes with town phase active

**Notes:** Integration slice. The run loop becomes: `initialize → town_visit → dungeon_run → town_visit → dungeon_run → ...` until victory or defeat.

---

### US-910: P5b-4: Implement hero XP and leveling

**Priority:** 10

As a developer, I need hero experience and leveling so heroes can progress through resolve levels between dungeon runs.

**Acceptance Criteria:**
- `HeroProgress` struct: `xp: u32`, `resolve_level: u32`, `hero_level: u32`
- XP thresholds match `Campaign.json`: `level_threshold_table: [0, 2, 6, 10, 16, 22, 32, 42]`, `resolve_thresholds: [0, 2, 8, 14, 30, 62, 94]`
- `add_xp(amount)` advances hero level and resolve level when thresholds are crossed
- Resolve level gates dungeon access (heroes above resolve level cannot enter low-level dungeons)
- Focused test proves leveling at each threshold boundary

**Notes:** DDGC's level system is separate from resolve level. Both use XP from `Campaign.json`.

---

## Phase 5c: Trinket and Equipment System

### Context

DDGC hero stats come entirely from equipment (weapon + armor at a given upgrade level). The current migration uses hardcoded level-0 stats from equipment `.bytes` files. Trinkets add buff modifiers via `JsonBuffs.json` references. The trinket system has 6+ rarity tiers and class requirements.

### US-911: P5c-0: Define trinket and equipment data model

**Priority:** 11

As a developer, I need data structures for trinkets and equipment upgrades so hero stats can be computed from gear rather than hardcoded.

**Acceptance Criteria:**
- `TrinketDefinition` struct: `id`, `buffs: Vec<String>`, `hero_class_requirements: Vec<String>`, `rarity`, `price`, `limit`, `origin_dungeon`
- `EquipmentDefinition` struct: `id`, `hero_class_id`, `slot` (weapon/armor), `upgrade_level`, `stat_modifiers: Vec<AttributeModifier>`
- `AttributeModifier` struct: `attribute_key`, `value`
- All structs derive `Serialize`/`Deserialize`/`Clone`/`Debug`
- Focused test proves construction and lookup are deterministic

**Notes:** Data-model slice. Trinket buffs reference `JsonBuffs.json` buff IDs — these map to attribute modifiers in the game layer.

---

### US-912: P5c-1: Parse DDGC trinket definitions

**Priority:** 12

As a developer, I need DDGC trinket data parsed into the trinket model so the loot and shop systems can use actual trinket content.

**Acceptance Criteria:**
- A `TrinketRegistry` holds all DDGC trinket definitions parsed from `JsonTrinkets.json`
- Trinkets are indexed by: ID, class requirement, rarity, origin dungeon
- At least 10 trinkets across multiple rarity tiers are parsed and verified
- Class-requirement filtering works: `trinkets_for_class("hunter")` returns only hunter-eligible trinkets
- Focused test proves lookup, filtering, and rarity categorization

**Notes:** `JsonTrinkets.json` references buff IDs from `JsonBuffs.json`. The buff-resolution layer is a separate concern (US-913).

---

### US-913: P5c-2: Implement buff resolution from trinket references

**Priority:** 13

As a developer, I need trinket buff references resolved to concrete attribute modifiers so equipped trinkets modify hero stats correctly.

**Acceptance Criteria:**
- A `BuffRegistry` maps buff IDs (e.g., `"TRINKET_STRESSDMG_B0"`, `"MAXHP-15"`) to `Vec<AttributeModifier>`
- Buff IDs follow DDGC naming conventions: stat abbreviations with sign/suffix
- `resolve_buffs(trinket)` returns all `AttributeModifier` entries for that trinket's buff list
- At least 5 buff IDs are resolved to correct attribute modifications
- Focused test proves: (a) positive and negative modifiers, (b) percentage-based vs flat modifiers, (c) multi-buff trinkets aggregate correctly

**Notes:** DDGC buff IDs use mixed naming: `MAXHP-15` (flat -15% max HP), `GIVE_STRESSDMG20` (+20% stress damage), `DMGL-25` / `DMGH-25` (damage low/high -25%). Need a buff ID → modifier parser.

---

### US-914: P5c-3: Compute hero stats from equipment

**Priority:** 14

As a developer, I need hero combat stats computed from base + equipment + trinkets instead of hardcoded values so the equipment progression system actually changes hero power.

**Acceptance Criteria:**
- `compute_hero_stats(hero_class, weapon_level, armor_level, trinkets)` returns a full attribute map
- Level-0 equipment + no trinkets produces the same stats as current hardcoded values (backward compatibility)
- Higher weapon levels increase ATK; higher armor levels increase DEF/HP
- Equipping a trinket applies its resolved modifiers to the attribute map
- Focused test proves: (a) level 0 = current stats, (b) level 1 weapon > level 0 weapon ATK, (c) trinket modifies correct attribute

**Notes:** DDGC hero stats come entirely from equipment `.bytes` data. The current hardcoded stats in `src/content/heroes/` are level-0 weapon + armor values. Parse upgrade data from `Upgrades/Heroes/` JSON files.

---

### US-915: P5c-4: Wire equipment into hero archetype creation

**Priority:** 15

As a developer, I need hero archetype creation to accept equipment parameters so the same hero class can have different power levels based on gear.

**Acceptance Criteria:**
- `Archetype` creation for heroes accepts optional `(weapon_level, armor_level, trinkets)` parameters
- Default (no parameters) produces current level-0 stats (backward compatibility)
- All existing tests continue passing without modification
- At least one test proves a level-1 equipped hero has different stats than level-0
- `cargo test` passes with equipment-aware archetype creation

**Notes:** Integration slice. This is the point where hero stats stop being hardcoded and start being computed. All existing code that creates hero archetypes must work unchanged (default = level 0).

---

## Phase 5d: Quirk and Disease System

### Context

DDGC quirks (positive/negative) and diseases modify hero attributes and behavior. Quirks are acquired from curios, combat, and town events. Diseases are a subset of negative quirks with `is_disease: true`. The sanitarium building treats diseases and manages quirks.

### US-916: P5d-0: Define quirk data model and parse DDGC quirks

**Priority:** 16

As a developer, I need quirk definitions parsed from `JsonQuirks.json` so heroes can acquire and be affected by quirks and diseases.

**Acceptance Criteria:**
- `QuirkDefinition` struct: `id`, `is_positive`, `is_disease`, `classification`, `buffs: Vec<String>`, `incompatible_quirks`, `curio_tag`
- A `QuirkRegistry` holds all DDGC quirk definitions
- At least 10 quirks (mix of positive, negative, disease) are parsed
- Buffs are resolved through the `BuffRegistry` (from US-913)
- Focused test proves: (a) quirk lookup by ID, (b) positive/negative/disease classification, (c) buff resolution produces correct modifiers

**Notes:** Quirks use the same buff ID system as trinkets. This story depends on US-913 (BuffRegistry).

---

### US-917: P5d-1: Implement quirk application to heroes

**Priority:** 17

As a developer, I need heroes to carry active quirks that modify their attributes so the quirk system actually affects gameplay.

**Acceptance Criteria:**
- `HeroQuirkState` tracks a hero's active quirks: `positive: Vec<QuirkId>`, `negative: Vec<QuirkId>`, `diseases: Vec<QuirkId>`
- `apply_quirk(hero, quirk_id)` adds the quirk, enforcing: (a) incompatible quirk replacement, (b) maximum quirk slots per category (DDGC limits)
- Quirk modifiers are applied alongside equipment/trinket modifiers in `compute_hero_stats`
- Focused test proves: (a) quirk modifies hero attributes, (b) incompatible quirk replaces existing, (c) disease quirks are tracked separately

**Notes:** DDGC limits heroes to a fixed number of positive and negative quirks (typically 5 each). Exceeding the limit triggers random replacement.

---

### US-918: P5d-2: Wire quirk acquisition into curio and combat events

**Priority:** 18

As a developer, I need quirks to be acquired from curio interactions and combat events so the hero's quirk profile evolves during runs.

**Acceptance Criteria:**
- Curio result type `Quirk` triggers `apply_quirk()` with the result's quirk ID
- Curio result type `Disease` triggers `apply_quirk()` with the disease quirk
- Combat events that cause disease (e.g., certain monster skills) trigger `apply_quirk()`
- At least one end-to-end test proves: (a) curio interaction adds a quirk, (b) disease from combat is applied, (c) quirk modifiers affect subsequent combat stats
- `cargo test` passes with quirk acquisition active

**Notes:** Integration slice connecting the curio system (Phase 5a), combat system, and quirk system.

---

## Phase 5e: Affliction and Virtue System

### Context

When a hero's stress exceeds maximum, DDGC rolls an affliction (negative) or virtue (positive) outcome. Afflictions modify hero behavior in combat (random actions, blocking commands) and add stat modifiers. This is the "overstress" system that drives DDGC's psychological pressure mechanic.

### US-919: P5e-0: Define trait/affliction data model and parse DDGC traits

**Priority:** 19

As a developer, I need trait (affliction/virtue) definitions parsed from `JsonTraits.json` so the overstress system can resolve correctly.

**Acceptance Criteria:**
- `TraitDefinition` struct: `id`, `overstress_type` (affliction/virtue), `buff_ids`, `combat_start_turn_act_outs`, `reaction_act_outs`
- `ActOutEntry` struct: `action` (nothing/bark_stress/change_pos/ignore_command/etc.), `weight`
- `ReactionEntry` struct: `trigger`, `probability`, `effect`
- A `TraitRegistry` holds all DDGC trait definitions
- At least 2 afflictions (e.g., fearful, hopeless) and 1 virtue (e.g., courageous) are parsed
- Focused test proves: (a) trait lookup, (b) act-out weight tables are preserved, (c) reaction probabilities are preserved

**Notes:** `JsonTraits.json` uses nested arrays for act-outs and reactions. Parse carefully.

---

### US-920: P5e-1: Implement overstress resolution

**Priority:** 20

As a developer, I need the overstress resolution (affliction/virtue roll) implemented so heroes who exceed max stress get an appropriate trait instead of being stuck at max stress.

**Acceptance Criteria:**
- `resolve_overstress(hero_id, seed)` rolls between affliction (weighted probability) and virtue (low probability)
- Affliction selection uses DDGC's weighted trait table
- The selected trait is applied to the hero as an active affliction/virtue
- Trait buffs modify hero attributes (same modifier pipeline as quirks/trinkets)
- Focused test proves: (a) overstress produces an affliction with high probability, (b) virtue is possible but rare, (c) trait buffs modify hero stats

**Notes:** DDGC's virtue chance is approximately 5–10%. Use seeded RNG for deterministic resolution.

---

### US-921: P5e-2: Implement affliction combat behavior

**Priority:** 21

As a developer, I need affliction combat behavior (act-outs) implemented so afflicted heroes occasionally disobey orders during combat.

**Acceptance Criteria:**
- At the start of an afflicted hero's turn, `resolve_act_out(affliction_id, seed)` rolls against the weighted act-out table
- Possible outcomes: `Nothing` (obey command normally), `IgnoreCommand` (skip turn), `ChangePosition` (random move), `BarkStress` (stress ally), `AttackFriendly`, `MarkSelf`
- Act-out result is recorded in the battle trace
- Focused test proves: (a) `Nothing` is the most common outcome (highest weight), (b) other outcomes are possible, (c) act-out resolution is deterministic for a given seed

**Notes:** This is the most visible player-facing effect of the affliction system. DDGC's act-out weights make `nothing` dominant (weight ~6 out of ~10) with smaller chances for disruptive actions.

---

### US-922: P5e-3: Wire overstress into battle loop

**Priority:** 22

As a developer, I need the battle loop to check for overstress and resolve it so the stress mechanic produces consequences during combat.

**Acceptance Criteria:**
- When a hero's stress exceeds `ATTR_MAX_STRESS`, `resolve_overstress` is called
- The resulting affliction/virtue takes effect immediately (buffs applied, act-outs enabled)
- Battle trace records overstress events with the resulting trait
- At least one end-to-end test proves: (a) stress damage pushes hero over threshold, (b) affliction is applied, (c) act-out occurs on subsequent turn
- `cargo test` passes with overstress resolution active

**Notes:** Integration slice. This completes the stress → affliction → combat behavior chain, which is the core psychological pressure loop in DDGC.

---

## Dependency Order

```
Phase 4 (prerequisite):
  US-801 → US-802 → US-803 → US-804 → US-805  (conditions)
  US-806 → US-807 → US-808                       (damage)
  US-809 → US-810 → US-811 → US-812              (dungeon)

Phase 5 (after Phase 4):
  US-901 → US-902 → US-903 → US-904 → US-905     (curio/trap/obstacle)
  US-906 → US-907 → US-908 → US-909 → US-910     (town buildings)
  US-911 → US-912 → US-913 → US-914 → US-915     (trinket/equipment)
  US-916 → US-917 → US-918                        (quirk) [depends on US-913]
  US-919 → US-920 → US-921 → US-922              (affliction/virtue) [depends on US-918]

Cross-phase dependencies:
  US-905 (wire curio into rooms) depends on US-810 (dungeon map configs)
  US-918 (quirk from curio) depends on US-903 (curio resolution) and US-917 (quirk application)
  US-913 (buff resolution) is shared by trinkets (US-911+) and quirks (US-916+)
  US-922 (overstress in battle) depends on US-920 (overstress resolution)
```

## User Story Summary

| US ID | Title | Priority | Phase |
|-------|-------|----------|-------|
| US-801 | Inventory remaining condition touchpoints | 1 | 4a |
| US-802 | Implement HP-threshold conditions | 2 | 4a |
| US-803 | Implement dungeon-mode condition | 3 | 4a |
| US-804 | Implement kill-trigger condition | 4 | 4a |
| US-805 | Close B-004 condition blocker | 5 | 4a |
| US-806 | Wire DamageRange into skill definitions | 6 | 4b |
| US-807 | Activate Rolled damage policy | 7 | 4b |
| US-808 | Close B-006 damage variance gap | 8 | 4b |
| US-809 | Parse DDGC MapGenerator parameters | 9 | 4c |
| US-810 | Wire dungeon map configs into room generation | 10 | 4c |
| US-811 | Parse DDGC encounter pack weights | 11 | 4c |
| US-812 | Validate dungeon fidelity end-to-end | 12 | 4c |
| US-901 | Define curio/trap/obstacle data model | 13 | 5a |
| US-902 | Parse DDGC curio/trap/obstacle definitions | 14 | 5a |
| US-903 | Implement curio interaction resolution | 15 | 5a |
| US-904 | Implement trap and obstacle resolution | 16 | 5a |
| US-905 | Wire curio/trap/obstacle into room exploration | 17 | 5a |
| US-906 | Define town building data model | 18 | 5b |
| US-907 | Parse DDGC building definitions | 19 | 5b |
| US-908 | Implement town visit cycle | 20 | 5b |
| US-909 | Wire town visit into run flow | 21 | 5b |
| US-910 | Implement hero XP and leveling | 22 | 5b |
| US-911 | Define trinket and equipment data model | 23 | 5c |
| US-912 | Parse DDGC trinket definitions | 24 | 5c |
| US-913 | Implement buff resolution from trinket references | 25 | 5c |
| US-914 | Compute hero stats from equipment | 26 | 5c |
| US-915 | Wire equipment into hero archetype creation | 27 | 5c |
| US-916 | Define quirk data model and parse DDGC quirks | 28 | 5d |
| US-917 | Implement quirk application to heroes | 29 | 5d |
| US-918 | Wire quirk acquisition into events | 30 | 5d |
| US-919 | Define trait/affliction data model | 31 | 5e |
| US-920 | Implement overstress resolution | 32 | 5e |
| US-921 | Implement affliction combat behavior | 33 | 5e |
| US-922 | Wire overstress into battle loop | 34 | 5e |
