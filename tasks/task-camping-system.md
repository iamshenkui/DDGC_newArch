# Migration Task: Camping System

## Classification

game-gap

## Summary

Implement the camping phase system, including camping skill definitions,
camping phase resolution, and camp effect application. Camping occurs during
dungeon runs (not in town) and allows heroes to use camping skills for healing,
stress relief, buffs, and debuff removal. All behavior derived from original
DDGC `CampingSkill.cs`, `CampController.cs`, and `JsonCamping.json`.

## Current State

- **No camping system exists** in DDGC_newArch.
- Framework crates (`framework_combat`, `framework_progression`) have no
camping hook — this is purely game-layer logic.
- Only 6 combat skills are defined in `content/skills.rs`.

## Original Game References

### CampingSkill (`CampingSkill.cs`)

```csharp
public class CampingSkill : Skill {
    public int TimeCost { get; set; }      // time points consumed
    public int Limit { get; set; }         // use limit per camp
    public bool HasIndividualTarget { get; }
    public List<string> Classes { get; set; }    // hero classes that can use it
    public List<CampEffect> Effects { get; set; }
    public CurrencyCost CurrencyCost { get; set; }
}
```

### CampEffect structure (from `JsonCamping.json`)

Each effect in a camping skill has:
- `selection`: `"self"`, `"individual"`, `"party_other"`, `"party_all"`
- `requirements`: list of prerequisites (usually empty in data)
- `chance`: `{ code, amount }` — probability code and value (1.0 = guaranteed)
- `type`: CampEffectType enum value
- `sub_type`: buff ID string (when type is `"buff"`)
- `amount`: numeric parameter (heal amount, percent, etc.)

### CampEffectType enum (from `CampingSkillHelper.cs`)

Original types:
- `StressHealAmount` — flat stress heal
- `HealthHealMaxHealthPercent` — heal % of max HP
- `RemoveBleed` — removes bleed status
- `RemovePoison` — removes poison status
- `Buff` — applies buff (sub_type = buff ID)
- `RemoveDeathRecovery` — removes death's door recovery debuff
- `ReduceAmbushChance` — reduces ambush probability
- `RemoveDisease` — removes disease
- `StressDamageAmount` — flat stress damage
- `Loot` — gives loot
- `ReduceTorch` — **deleted from original game, DO NOT MIGRATE**
- `HealthDamageMaxHealthPercent` — damage % of max HP
- `RemoveBurn` — removes burning
- `RemoveFrozen` — removes frozen
- `StressHealPercent` — stress heal %
- `RemoveDebuff` — removes debuff
- `RemoveAllDebuff` — removes all debuffs
- `HealthHealRange` — heal random range
- `HealthHealAmount` — flat heal
- `ReduceTurbulenceChance` — reduces turbulence
- `ReduceRiptideChance` — reduces riptide

### CampController (`CampController.cs`)

- Manages `CampingPhase.Meal`
- `SwitchCamping(bool active)`:
  - Sets hero positions for camping scene
  - Applies buff rules (removes camping buffs on exit)
- Camping skills are consumed from a shared time pool (default 12 points)
- Each skill costs `TimeCost` points
- Skills can only be used `Limit` times per camp
- Only heroes with matching `Classes` can use class-specific skills
- Generic skills (`hero_classes: []` or all-classes list) are available to all

### JsonCamping.json data

- **87 camping skills** total in original game
- Shared skills: `encourage`, `first_aid`, `pep_talk`, `hobby`
- Class-specific examples:
  - Arbalest/Musketeer: `field_dressing`, `marching_plan`, `restring_crossbow`,
    `clean_musket`, `triage`
  - Bounty Hunter: `how_its_done`, `tracking`, `planned_takedown`, `scout_ahead`
  - Crusader: `unshakeable_leader`
- Upgrade requirements: each skill has level 0 with gold cost 1750 upgrade
- Skills have `cost` (time cost, 2-4 points) and `use_limit` (usually 1)

## Required Changes

### 1. New module: `src/camp/`

Create `src/camp/mod.rs` with:

```rust
/// A camping skill definition.
pub struct CampingSkill {
    pub id: String,
    pub time_cost: u32,
    pub use_limit: u32,
    pub has_individual_target: bool,
    pub classes: Vec<String>,
    pub effects: Vec<CampEffect>,
    pub upgrade_cost: u32,
}

/// A single effect within a camping skill.
pub struct CampEffect {
    pub selection: CampTargetSelection,
    pub requirements: Vec<String>,
    pub chance: f64,
    pub effect_type: CampEffectType,
    pub sub_type: String,       // buff ID when effect_type == Buff
    pub amount: f64,
}

pub enum CampTargetSelection {
    SelfTarget,
    Individual,
    PartyOther,
    PartyAll,
}

pub enum CampEffectType {
    StressHealAmount,
    HealthHealMaxHealthPercent,
    RemoveBleed,
    RemovePoison,
    Buff,
    RemoveDeathRecovery,
    ReduceAmbushChance,
    RemoveDisease,
    StressDamageAmount,
    Loot,
    HealthDamageMaxHealthPercent,
    RemoveBurn,
    RemoveFrozen,
    StressHealPercent,
    RemoveDebuff,
    RemoveAllDebuff,
    HealthHealRange,
    HealthHealAmount,
    ReduceTurbulenceChance,
    ReduceRiptideChance,
}
```

### 2. Camping phase state

```rust
pub struct CampingPhase {
    pub time_budget: u32,           // default 12
    pub time_spent: u32,
    pub skill_uses: HashMap<String, u32>,  // skill_id -> uses this camp
    pub heroes: Vec<HeroInCamp>,
    pub trace: Vec<CampActivityRecord>,
}

pub struct HeroInCamp {
    pub hero_id: String,
    pub class_id: String,
    pub health: f64,
    pub max_health: f64,
    pub stress: f64,
    pub max_stress: f64,
    pub active_buffs: Vec<String>,
    pub can_use_skills: bool,       // activity_lock side-effect
}
```

### 3. Camping skill registry

- Parse `JsonCamping.json` into a `CampingSkillRegistry`
- Provide lookup by skill ID
- Provide filtering by hero class

### 4. Skill resolution

`perform_camping_skill(skill_id, target_hero_id)`:
1. Look up skill
2. Check time budget: `time_spent + time_cost <= time_budget`
3. Check use limit: `skill_uses[id] < use_limit`
4. Check class eligibility: hero class in `classes` list (or generic)
5. Check target selection validity:
   - `SelfTarget` → target must be performer
   - `Individual` → target must be valid individual
   - `PartyOther` → target must be other hero
   - `PartyAll` → no individual target needed
6. For each effect:
   - Roll `chance`
   - Apply effect to target(s) based on `selection`
   - Record outcome in trace
7. Deduct time, increment use counter

### 5. Effect application mapping

| CampEffectType | Game-layer action |
|----------------|-------------------|
| StressHealAmount | `hero.stress = max(0, hero.stress - amount)` |
| HealthHealMaxHealthPercent | `hero.health = min(max_health, health + max_health * amount)` |
| RemoveBleed | Remove "bleed" status if present |
| RemovePoison | Remove "poison" status if present |
| Buff | Add `sub_type` buff to hero active_buffs |
| RemoveDeathRecovery | Remove death recovery debuff |
| ReduceAmbushChance | Set ambush chance modifier (game-layer) |
| RemoveDisease | Remove disease from hero |
| StressDamageAmount | `hero.stress += amount` |
| Loot | Add loot to party inventory (requires loot system) |
| HealthDamageMaxHealthPercent | `hero.health -= max_health * amount` |
| RemoveBurn | Remove "burning" status |
| RemoveFrozen | Remove "frozen" status |
| StressHealPercent | `hero.stress = max(0, hero.stress - max_stress * amount)` |
| RemoveDebuff | Remove one debuff |
| RemoveAllDebuff | Remove all debuffs |
| HealthHealRange | Heal random amount (requires RNG seeding) |
| HealthHealAmount | `hero.health = min(max_health, health + amount)` |
| ReduceTurbulenceChance | Set turbulence modifier |
| ReduceRiptideChance | Set riptide modifier |

### 6. Integration with dungeon run

- Add `CampingPhase` to dungeon run state
- Trigger camping at designated rooms or player choice
- After camping, remove all camping buffs from heroes (original behavior)

### 7. Content file: `src/content/camping_skills.rs`

Migrate the 87 camping skills from `JsonCamping.json`.
Group by class or function:
```rust
// Shared skills
pub fn encourage() -> CampingSkill
pub fn first_aid() -> CampingSkill
pub fn pep_talk() -> CampingSkill
pub fn hobby() -> CampingSkill

// Arbalest / Musketeer
pub fn field_dressing() -> CampingSkill
pub fn marching_plan() -> CampingSkill
// ... etc
```

## Acceptance Criteria

- [ ] `CampingSkill` and `CampEffect` structs match original game fields
- [ ] `JsonCamping.json` is fully parseable (all 87 skills)
- [ ] Time budget enforcement works (12 points default, skills cost 2-4)
- [ ] Use limit enforcement works (most skills limit = 1)
- [ ] Class restriction works (class-specific vs shared skills)
- [ ] All 20 CampEffectTypes have game-layer application logic
- [ ] Buff application/removal works for camping buffs
- [ ] Camping phase produces deterministic trace
- [ ] Unit tests for time budget exhaustion, class restriction, effect rolls

## Blockers / Dependencies

- **B-Loot**: `Loot` effect type requires loot system (can stub)
- **B-Status**: Bleed/Poison/Burn/Frozen removal requires status system
- **B-Disease**: `RemoveDisease` requires disease system
- **B-Buff**: Camping buffs require buff framework integration
- **B-Debuff**: `RemoveDebuff`/`RemoveAllDebuff` requires debuff system

## Estimated Effort

Large (5-7 days). 87 skills to migrate, 20 effect types to implement,
and integration with dungeon run flow.
