# Migration Task: Sanitarium + Town Building Services Completion

## Classification

game-gap

## Summary

Fill the remaining shell implementations in `town::TownVisit` for Sanitarium
(disease/quirk treatment), Tavern (bar/gambling/brothel with side-effects),
Blacksmith (equipment discount), and Guild (skill discount). All behavior must
be derived from the original DDGC Unity C# source and JSON building configs.

## Current State

- `TownActivity` enum has variants for Pray, Rest, Recruit, Train, Repair,
  UpgradeWeapon, UpgradeBuilding.
- Only `Pray` (Abbey) and `Recruit` (StageCoach) have real implementations.
- `Rest` exists but is a simplified placeholder (full HP restore, no building
  config lookup).
- `Train`, `Repair`, `UpgradeWeapon` are no-op stubs.
- **Sanitarium is entirely missing** from `TownActivity`.
- **Tavern is entirely missing** from `TownActivity`.

## Original Game References

### Sanitarium (`Sanitarium.cs`, `sanitarium.building.json`)

Sanitarium extends `Building` (not `ActivityBuilding`). It contains two
sub-activities:

1. **Quirk Treatment**
   - Fields: `QuirkTreatmentChance` (default 1.0), `BasePositiveQuirkCost`,
     `BaseNegativeQuirkCost`, `BasePermNegativeCost`, `BaseQuirkSlots`
   - Upgrades:
     - `positive_quirk_cost_upgrades`: 7500 -> 2500 (levels a-e)
     - `negative_quirk_cost_upgrades`: 1500 -> 750 (levels a-e)
     - `permanent_negative_quirk_cost_upgrades`: 5000 -> 2500 (levels a-e)
     - `slot_upgrades`: 1 -> 3 slots (levels b, d)
   - Creates `TreatmentSlots` (1-3) with 3 cost tiers per slot:
     - Positive quirk cost, Negative quirk cost, Permanent negative cost
   - Cost modifiers from event system (`IsActivityFree`, `ActivityCostModifier`)

2. **Disease Treatment**
   - Fields: `DiseaseTreatmentChance` (default 1.0), `BaseDiseaseTreatmentCost`,
     `BaseCureAllChance` (0.33), `BaseDiseaseSlots`
   - Upgrades:
     - `disease_quirk_cost_upgrades`: 750 -> 450 (levels a, c, e)
     - `disease_quirk_cure_all_chance_upgrades`: 0.33 -> 1.00 (levels b, d)
     - `slot_upgrades`: 1 -> 3 slots (levels a, c)
   - Creates `TreatmentSlots` based on `DiseaseSlots`

### Tavern (`Tavern.cs`, `tavern.building.json`)

Tavern extends `ActivityBuilding`. It has three activities:

1. **Bar**
   - `side_effects` chance: 0.40
   - Side effects (weighted): `activity_lock`, `go_missing` (1-2 duration),
     `add_quirk` (alcoholism, resolution), `apply_buff`
     (townHungoverAccDebuff, townHungoverDEFDebuff), `change_currency` (-500),
     `remove_trinket`
   - `cost_upgrades`: 1000 -> 700 (levels b, e)
   - `slot_upgrades`: 1 -> 3 (levels c, f)
   - `stress_upgrades`: 45 -> 100 (levels a, d)
   - `affliction_cure_upgrades`: 1.00 -> 0.10 (level g)
   - `quirk_library_names`: resolution, gambler, love_interest, enlightened,
     god_fearing, flagellant

2. **Gambling**
   - `side_effects` chance: 0.35
   - Side effects: `activity_lock`, `go_missing`, `add_quirk` (gambler,
     known_cheat, bad_gambler), `change_currency` (+500 / -500), `add_trinket`,
     `remove_trinket`
   - `cost_upgrades`: 1250 -> 900 (levels b, e)
   - `stress_upgrades`: 55 -> 86 (levels a, d)

3. **Brothel**
   - `side_effects` chance: 0.30
   - Side effects: `activity_lock`, `go_missing`, `add_quirk` (love_interest,
     syphilis, deviant_tastes), `apply_buff` (townBrothelSPDBuff,
     townBrothelSPDDebuff)
   - `cost_upgrades`: 1500 -> 1100 (levels b, e)
   - `stress_upgrades`: 65 -> 100 (levels a, d)

### Blacksmith (`blacksmith.building.json`)

- Extends `Building` (not `ActivityBuilding`)
- `equipment_cost_discount_upgrades`: 10% per level (a-e), cumulative
- In original: `DiscountUpgrades` list, discount accumulates from purchased
  upgrades
- Game-layer use: reduces equipment upgrade/repair costs

### Guild (`guild.building.json`)

- Same pattern as Blacksmith
- `combat_skill_cost_discount_upgrades`: 10% per level (a-e), cumulative
- Game-layer use: reduces combat skill purchase costs

## Required Changes

### 1. Extend `TownActivity` enum (`src/town/mod.rs`)

Add variants:
- `TreatQuirk { slot_index: usize, quirk_type: QuirkTreatmentType }`
- `TreatDisease { slot_index: usize }`
- `TavernBar { slot_index: usize }`
- `TavernGambling { slot_index: usize }`
- `TavernBrothel { slot_index: usize }`

Where `QuirkTreatmentType` is:
```rust
pub enum QuirkTreatmentType {
    Positive,
    Negative,
    PermanentNegative,
}
```

### 2. Add slot tracking to `TownVisit`

Treatment slots and tavern activity slots are consumed per-hero per-visit.
Add:
```rust
pub struct TownSlotState {
    pub building_id: String,
    pub activity_type: String,
    pub max_slots: usize,
    pub used_slots: usize,
}
```

### 3. Implement Sanitarium activities

`perform_treat_quirk`:
- Look up cost from building registry based on upgrade level and quirk type
- Check gold, check slot availability
- Deduct gold, consume slot, record activity
- Success if `QuirkTreatmentChance` roll passes (default 1.0 = always)

`perform_treat_disease`:
- Look up cost and `cure_all_chance` from registry
- Check gold, check slot availability
- Deduct gold, consume slot
- Roll `cure_all_chance` for cure-all vs single disease

### 4. Implement Tavern activities

Each tavern activity:
- Look up cost, stress heal range, slot count from registry
- Check gold, check slot availability
- Apply stress heal (use `heal_low`..`heal_high` range, or fixed value)
- Roll side-effect chance (40% bar, 35% gambling, 30% brothel)
- If side-effect triggers, weighted-select from result table and apply

Side-effect outcomes to model in game-layer:
- `activity_lock`: hero cannot perform further activities this visit
- `go_missing`: hero unavailable for N weeks (out of scope for single visit)
- `add_quirk`: add quirk to hero (requires quirk system — stub or defer)
- `apply_buff`: apply buff/debuff (requires buff system — stub or defer)
- `change_currency`: modify gold
- `remove_trinket`: remove random trinket (requires trinket system — stub)

### 5. Implement Blacksmith/Guild discount plumbing

- `perform_repair` / `perform_upgrade_weapon` / `perform_train`:
  - Look up discount percent from building registry
  - Apply discount to base costs (equipment/skill costs are game-layer)
  - Record discounted cost in activity trace

### 6. Building registry data extension

`contracts/parse.rs` and `contracts/mod.rs`:
- Add parsing for sanitarium JSON sections (`treatment`, `disease_treatment`)
- Add parsing for tavern JSON sections (`bar`, `gambling`, `brothel`)
- Add parsing for blacksmith/guild discount upgrades
- Extend `TownBuilding` or add building-type-specific config structs

## Acceptance Criteria

- [ ] `TownActivity` covers all original game building activities
- [ ] Sanitarium quirk treatment resolves cost, slots, and success chance
- [ ] Sanitarium disease treatment resolves cost, slots, and cure-all chance
- [ ] Tavern bar/gambling/brothel resolve cost, stress heal, side-effect rolls
- [ ] Blacksmith equipment discount is looked up and applied to repair/upgrade
- [ ] Guild skill discount is looked up and applied to training
- [ ] All activities produce deterministic `TownActivityRecord` traces
- [ ] Unit tests cover gold deduction, slot exhaustion, and side-effect rolls
- [ ] Building JSON configs are fully parsed (no ignored fields)

## Blockers / Dependencies

- **B-Quirk**: Quirk system not yet migrated (quirk add/remove is stub)
- **B-Buff**: Town buff/debuff system not migrated (tavern side-effect buffs)
- **B-Trinket**: Trinket system not migrated (remove_trinket side-effect)
- **B-Equipment**: Equipment cost base values not yet defined (blacksmith)
- **B-SkillPurchase**: Combat skill purchase costs not yet defined (guild)

## Estimated Effort

Medium-Large (3-5 days). Sanitarium and Tavern are the bulk; Blacksmith/Guild
are mostly plumbing existing discount data into stub methods.
