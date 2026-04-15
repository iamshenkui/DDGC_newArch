# Phase 2 Inventory: DDGC Conditions, Damage Variance, and Hit Resolution

This document is the implementation artifact for **US-601 (P2-0: Inventory open conditional and hit-policy gaps)**.
It inventories all migrated content blocked by DDGC-specific conditions, damage variance, and
dodge-aware hit resolution, and selects the initial implementation subset for Phase 2.

---

## B-004: DDGC BuffRule Condition System

### DDGC BuffRule Enum (from `Buff.cs`)

The full enum has 30+ variants. Classification:

#### Framework-Native Conditions (already supported via `EffectCondition`)

| DDGC BuffRule | Framework `EffectCondition` | Status |
|---|---|---|
| `HpBelow` | `IfTargetHealthBelow(fraction)` | ✅ Supported |
| `HpAbove` | `IfTargetHealthBelow` (inverted) | ✅ Supported (approximation) |
| `TargetHpBelow` | `IfTargetHealthBelow` | ✅ Supported |
| `TargetHpAbove` | `IfTargetHealthBelow` (inverted) | ✅ Supported (approximation) |
| `Status` / `ActorStatus` | `IfActorHasStatus(String)` | ✅ Supported |
| `InRank` | `IfTargetPosition(SlotRange)` | ✅ Supported |
| `Probability` | `Probability(f64)` | ✅ Supported (used for stun chance etc.) |
| `Melee` | — | Game-gap (no range check in framework) |
| `Ranged` | — | Game-gap (no range check in framework) |

#### DDGC-Only Conditions (NOT yet implemented in game-layer)

| DDGC BuffRule | Description | Implementation Priority |
|---|---|---|
| `FirstRound` | Active only on round 0 (first round of combat) | **P2-4 (highest)** |
| `StressAbove` | Hero stress above threshold | **P2-5** |
| `StressBelow` | Hero stress below threshold | **P2-5** |
| `DeathsDoor` | Actor at death's door (HP < 50%) | P2-7 (deferred) |
| `InMode` | Actor in a specific mode | P2-7 (deferred) |
| `InCamp` | During camping phase | Out of scope (not a combat mechanic) |
| `InDungeon` | During dungeon phase | Out of scope |
| `WalkBack` | Actor walking backwards | P2-7 (rare) |
| `InActivity` | During activity | Out of scope |
| `InCorridor` | Actor in corridor | Out of scope |
| `Riposting` | Actor currently riposting | P2-7 (rare) |
| `ChaosBelow` / `ChaosAbove` | Chaos level conditions | P2-7 (deferred - chaos not modeled) |
| `LightBelow` / `LightAbove` | Light level conditions | Out of scope |
| `LightChanged` | Light changed | Out of scope |
| `Dot` | Target has damage-over-time | P2-7 (rare - needs dot detection) |
| `Size` | Actor size | P2-7 (rare) |
| `EnemyType` | Target enemy type | P2-7 (rare) |
| `Skill` | Specific skill active | P2-7 (rare) |
| `Afflicted` | Actor is afflicted | P2-7 (deferred - affliction system not modeled) |
| `Virtued` | Actor is virtuous | P2-7 (deferred - virtue system not modeled) |

### Initial Condition Subset Selection (Phase 2 Targets)

**Selected for implementation (in priority order):**

1. **`FirstRound`** — P2-4 story
   - High leverage: many DDGC opening-turn effects use this
   - Clear evaluation context: `BattleStatus == Fighting && RoundNumber == 0`
   - DDGC semantics: buff applies on round 0, reverts after round 0 ends
   - Implementation: game-layer condition check in skill resolution pipeline

2. **`StressAbove` / `StressBelow`** — P2-5 story
   - High leverage: stress-driven behavior is core to DDGC hero mechanics
   - Requires: `ATTR_STRESS` is already defined in `actors.rs`
   - DDGC semantics: `StressAbove` checks `Stress > threshold` for heroes only (monsters always fail)
   - Implementation: game-layer condition check reading `ATTR_STRESS`

**Deferred to future work:**
- `DeathsDoor` — depends on death's door threshold (HP < 50%)
- `InMode` — mode system not yet modeled
- `Afflicted` / `Virtued` — affliction/virtue system not modeled
- All out-of-scope conditions (camp, dungeon, light, etc.)

---

## B-006: Damage Variance

### Current State

- **Original DDGC behavior:** Skills deal `DamageMin..DamageMax` range damage, rolled per use.
- **Migration behavior:** Skills deal fixed damage equal to the average of min and max.
- **Root cause:** Framework `EffectNode::damage(amount)` takes a single `f64`, not a range.

### Example: Averaged Damage in Migrated Skills

| Skill | DDGC Range | Migration Average | Impact |
|---|---|---|---|
| `crusading_strike` | 8–15 | 12.0 | ±3.5 variance lost |
| `holy_lance` | 6–12 | 9.0 | ±3.0 variance lost |
| `rend` | 4–8 | 6.0 | ±2.0 variance lost |
| `skull_bash` | 10–18 | 14.0 | ±4.0 variance lost |
| `grave_bash` | 3–7 (×2 hits) | 5.0 (×2) | ±2.0 per hit variance lost |
| `normal_attack` (alligator) | 27–36 | 31.5 | ±4.5 variance lost |
| `bleed` (alligator) | 14–18 | 16.0 | ±2.0 variance lost |

### Variance Policy Options (P2-9, P2-10, P2-11)

- **Fixed-average (P2-9):** Keep current behavior — explicit deterministic policy, stable for regression tests
- **Rolled-damage (P2-10/P2-11):** Game-layer policy that rolls within DDGC min/max range

### Impact Assessment

- All migrated skills are affected — every skill uses averaged damage
- Variance is noticeable: ±4.5 damage on high-end skills like `normal_attack` (27–36)
- Deterministic mode is acceptable for testing; variance mode needed for "real" feel

---

## B-010: Accuracy vs. Dodge Hit Resolution

### Current State

- **Original DDGC behavior:** `hitChance = clamp(accuracy + performerAccuracy - targetDodge, 0, 0.95)`
- **Migration behavior:** Most skills use `EffectCondition::Probability(chance)` as a fixed probability placeholder
- **Framework support:** `EffectCondition::Probability(f64)` exists but is not integrated with accuracy/dodge stats

### DDGC Accuracy/Dodge System

From `BattleSolver.cs`:
```csharp
float accuracy = skill.Accuracy + performer.Accuracy;
float hitChance = Mathf.Clamp(accuracy - target.Dodge, 0, 0.95f);
```

Key points:
- Skill accuracy (from skill definition) + performer accuracy (from actor stats)
- Target dodge (from actor stats) subtracts from accuracy
- Result clamped to [0, 0.95] — minimum 0%, maximum 95% hit chance
- 5% minimum hit chance preserves "always hit" feeling for low-dodge targets

### Actor Stats Involved

From `Archetype` (migrated):
- `accuracy` — actor's base accuracy (currently all 0.0 in migrated archetypes)
- `dodge` — actor's base dodge (currently all 0.0 in migrated archetypes)

From DDGC skill definitions:
- `Accuracy` field per skill (e.g., `82.5%` = 0.825)

### Miss and Dodge Results

DDGC produces two distinct miss types:
- **`Miss`** — attack failed to connect at all
- **`Dodge`** — attack was dodged (produces `SkillResultType.Dodge` entry)

Both are treated as "did not hit" in the framework migration.

### Accuracy/Dodge Inventory (Migrated Content)

| Actor | accuracy | dodge | Notes |
|---|---|---|---|
| Crusader | 0.0 | 0.0 | Placeholder |
| Vestal | 0.0 | 0.0 | Placeholder |
| Bone Soldier | 0.0 | 0.0 | Placeholder |
| Necromancer | 0.0 | 0.0 | Placeholder |
| Alligator Yangtze | 0.0 | 0.0 | DDGC has 0.0% dodge, no accuracy stat |
| Fox Fire | 0.0 | 0.075 | DDGC: 7.5% dodge from DEF |

**Note:** Archetype `defense` (7.5%) is DDGC's "dodge" equivalent for physical attacks.
The framework does not currently model accuracy vs. dodge resolution — this is B-010.

### Hit-Policy Anchor Scenario Selection

**Selected anchor: `snake_water` (common monster)**
- From XuanWu dungeon, has a `stun` skill with accuracy check
- DDGC skill definition: `stun` has `atk 82.5%` accuracy
- Target has `dodge` from DEF
- This is the simplest accuracy-vs-dodge scenario: one attacker, one target, clear hit/miss outcome

**Alternative anchor: `frostvein_clam` (boss)**
- Has `riposte` mechanic (already partially implemented in P1)
- Complex multi-body with dodge stat

**Recommendation:** Start with `snake_water` as the P2-12 anchor — simpler case for the policy interface.

---

## Combined Inventory: All Blocked Content

### Content Blocked by DDGC-Only Conditions (B-004)

| Content | File | Blocked By | Notes |
|---|---|---|---|
| Any skill with `FirstRound` condition | — | `FirstRound` | Not yet implemented |
| Hero skills with stress conditions | — | `StressAbove`/`StressBelow` | Heroes have stress, monsters don't |
| Skills with `DeathsDoor` condition | — | `DeathsDoor` | Not yet implemented |

**Currently migrated content has NO DDGC-specific conditions applied** — all migrated skills
use unconditional effects or framework-native `EffectCondition::Probability`. The conditions
are absent because they haven't been implemented yet.

### Content Affected by Damage Variance (B-006)

**All migrated skills** are affected — every skill uses averaged damage.

### Content Affected by Dodge-Aware Hit Resolution (B-010)

| Content | File | Accuracy | Dodge | Notes |
|---|---|---|---|---|
| Alligator Yangtze `normal_attack` | `alligator_yangtze.rs` | 82.5% (skill) | 0% | Simple accuracy check |
| Alligator Yangtze `bleed` | `alligator_yangtze.rs` | 82.5% (skill) | 0% | Simple accuracy check |
| Fox Fire `bite` | `fox_fire.rs` | 82.5% (skill) | 7.5% (DEF) | Has dodge from DEF |
| Fox Fire `vomit` | `fox_fire.rs` | 82.5% (skill) | 7.5% (DEF) | Has dodge from DEF |
| All hero skills | `content/heroes/*.rs` | varies | 0% | Accuracy varies by skill |

---

## Phase 2 Story Selection Recommendations

Based on this inventory:

| Story | DDGC Gap | Rationale |
|---|---|---|
| **P2-4: FirstRound** | B-004 | Highest leverage, clear context, many effects use it |
| **P2-5: StressAbove/Below** | B-004 | Stress is core DDGC mechanic,ATTR_STRESS already exists |
| **P2-9: Fixed-average policy** | B-006 | Stabilize existing tests before adding variance |
| **P2-10: Rolled-damage policy** | B-006 | Restore variance without breaking deterministic tests |
| **P2-12: Accuracy-vs-dodge policy** | B-010 | Start with `snake_water` anchor scenario |

---

## Appendix: Framework `EffectCondition` Reference

```rust
pub enum EffectCondition {
    /// Only execute if the target's health is below the given fraction (0.0–1.0).
    IfTargetHealthBelow(f64),
    /// Only execute if the source actor has a specific status.
    IfActorHasStatus(String),
    /// Only execute if the target is in a specific position range.
    IfTargetPosition(SlotRange),
    /// Execute only with the given probability (0.0–1.0).
    Probability(f64),
}
```

**Note:** DDGC `BuffRule` has 30+ variants. The framework's `EffectCondition` covers
4 generic cases. DDGC-specific conditions (FirstRound, StressAbove, DeathsDoor, etc.)
require game-layer filtering in the skill resolution pipeline.