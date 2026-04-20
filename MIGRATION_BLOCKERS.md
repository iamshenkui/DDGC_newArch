# DDGC Migration Blockers

Every migration blocker classified as **core-gap**, **framework-gap**, or **game-gap**.
Only core-gap and framework-gap blockers may result in patches to the framework or core crates.
Game-gap blockers are solved entirely within `game_ddgc_headless`.

---

## Classification Rules

| Label | Meaning | Resolution |
|---|---|---|
| **core-gap** | The framework's core types are missing a capability needed for DDGC | Patch `framework_rules` with regression tests |
| **framework-gap** | A framework crate (combat, progression, etc.) lacks a feature DDGC needs | Patch the specific crate with regression tests |
| **game-gap** | The framework provides the building blocks but DDGC needs game-specific logic | Implement in `game_ddgc_headless/src/` — no framework changes |

---

## Active Blockers

### B-001: Paired Attributes (HP current/max)
- **Classification:** game-gap
- **Batch:** 1 (Actors)
- **Description:** DDGC uses paired attributes (current/max) for HP, Stress, and Chaos. The framework has single `AttributeKey` entries with `Modifier` stacking.
- **Resolution:** Define two separate `AttributeKey` constants per paired attribute (e.g., `ATTR_HEALTH` + `ATTR_MAX_HEALTH`). The game layer tracks the pairing convention. No framework change needed.
- **Status:** Resolved — convention established in `actors.rs`

### B-002: Physical vs. Magic Defense Split
- **Classification:** game-gap
- **Batch:** 1 (Actors)
- **Description:** DDGC separates `DefenseRating` (physical) and `ProtectionRating` (magic) and `MagicProtectionRating`. The framework has a single `ATTR_DEFENSE`.
- **Resolution:** Define custom `AttributeKey` constants: `ATTR_PHYS_DEFENSE`, `ATTR_MAGIC_DEFENSE`. Combat resolution in the game layer reads the appropriate key based on damage type.
- **Status:** Resolved — convention established in `actors.rs`

### B-003: Stress and Chaos Mechanics
- **Classification:** game-gap
- **Batch:** 1 (Actors)
- **Description:** DDGC's Stress and Chaos are first-class mechanics with death's door, afflictions, and virtue mechanics. The framework has no concept of these.
- **Resolution:** Implement as custom `AttributeKey` entries + game-layer logic. Death's Door is a threshold check in game-layer code. No framework change needed.
- **Status:** Resolved — game-layer convention

### B-004: BuffRule Conditional System
- **Classification:** game-gap
- **Batch:** 2 (Statuses)
- **Description:** DDGC's `BuffRule` supports 35+ conditional variants (HpBelow, StressAbove, InMode, FirstRound, DeathsDoor, etc.). The framework has `EffectCondition` with 4 variants (IfTargetHealthBelow, IfActorHasStatus, IfTargetPosition, Probability).
- **Resolution:** Use `EffectCondition` where it matches. For DDGC-specific conditions (StressAbove, DeathsDoor, FirstRound), implement game-layer filtering in the skill resolution pipeline. No framework change needed for G2 slice.
- **Status:** Resolved — Phase 2 implemented `ConditionAdapter` in `src/run/conditions.rs` with `FirstRound`, `StressAbove`, `StressBelow`, `DeathsDoor`, `TargetHasStatus`, and `ActorHasStatus` conditions. Phase 4a added `HpAbove`, `TargetHpAbove`, and `TargetHpBelow` HP-threshold conditions (US-802). Framework-native conditions (`Probability`, `IfTargetHealthBelow`, `IfActorHasStatus`) are bridged through the adapter with identical behavior. `IfTargetPosition` returns `Unknown` (formation context unavailable). Remaining DDGC conditions (e.g., `InMode`) are deferred as low priority (no migrated content depends on them).

### B-005: Skill Usage Limits (LimitPerTurn, LimitPerBattle)
- **Classification:** game-gap
- **Batch:** 3 (Skills)
- **Description:** DDGC tracks how many times a skill can be used per turn or per battle. The framework's `SkillDefinition` has `cooldown: Option<u32>` but no per-turn or per-battle usage count.
- **Resolution:** Track usage counts in game-layer state alongside the combat resolver. Reset per-turn counts when `CombatResolver::end_turn()` is called; track per-battle counts for the encounter lifetime. No framework change needed.
- **Status:** Resolved — US-510 (SkillUsageCounters), US-512 (reset_battle_scope), US-513 (direct_hit_1 per-turn limit), US-514 (duality_fate per-battle limit), US-515 (regression suite)

### B-006: Damage Range (Min/Max) vs. Fixed Damage
- **Classification:** game-gap
- **Batch:** 3 (Skills)
- **Description:** DDGC skills deal `DamageMin..DamageMax` range damage. The framework's `EffectNode::damage(amount)` takes a single `f64`.
- **Resolution:** Use the average of min/max as the fixed damage value for the first migration slice. If variance is needed later, add a game-layer damage roll step before submitting combat commands.
- **Status:** Resolved — US-609 (DamagePolicy with FixedAverage default; Rolled policy available for future variance)

### B-007: Launch Ranks / Target Ranks (Formation Positioning)
- **Classification:** framework-gap (partial)
- **Batch:** 3 (Skills)
- **Description:** DDGC uses rank-based formation (positions 1–4 for allies, 1–4 for enemies) with `LaunchRanks` (who can use the skill) and `TargetRanks` (who the skill can hit). The framework has `FormationLayout` with `SlotRange` and `TargetSelector::ByPosition(SlotRange)` / `RelativePosition`.
- **Resolution:** Framework's position system is sufficient for DDGC's rank system. Map DDGC ranks 1–4 to slot positions. `TargetSelector::ByPosition` covers the targeting. **No framework change needed** — reclassified from framework-gap to game-gap after analysis.
- **Status:** Resolved — direct mapping via `SlotRange`

### B-008: Riposte and Guard Ally Mechanics
- **Classification:** game-gap
- **Batch:** 2 (Statuses)
- **Description:** DDGC has "riposte" (counter-attack when hit) and "guard ally" (redirect damage to self). The framework has no built-in reactive trigger system.
- **Resolution:** Implement as `StatusEffect { kind: "riposte" }` / `StatusEffect { kind: "guard" }`. Game-layer code checks for these statuses after each damage effect and applies the secondary action. No framework change needed.
- **Status:** Resolved — US-506 (riposte counter-attack), US-507 (guard detection), US-508 (guard redirect execution)

### B-009: Multi-Hit Skills
- **Classification:** game-gap
- **Batch:** 3 (Skills)
- **Description:** DDGC's `MultiHitCount` allows a skill to hit the same target N times. The framework's `EffectNode::damage()` deals damage once per node.
- **Resolution:** Emit N `EffectNode::damage()` entries in the effects vector. Each hit is a separate node. No framework change needed.
- **Status:** Resolved — replicate damage nodes

### B-010: Skill Accuracy and Dodge
- **Classification:** game-gap
- **Batch:** 3 (Skills)
- **Description:** DDGC has an accuracy vs. dodge roll. The framework has `EffectCondition::Probability(chance)` for conditional effects.
- **Resolution:** Use `EffectCondition::Probability(chance)` for the first slice. If dodge needs to be a reactive mechanic, implement as a status check in game-layer code.
- **Status:** Resolved — US-612 (HitResolutionContext + HitPolicy), US-613 (accuracy/dodge integration in battle loop, 0.95 default accuracy on all actors)

---

## Rejected Backflow Requests

These were considered for framework patches but rejected as game-specific:

| Request | Rejection Reason |
|---|---|
| Add `PairedAttribute` type to `framework_rules` | Pairing is a DDGC convention; framework's separate keys are more flexible |
| Add `StressAttribute` to `framework_rules` | Stress is DDGC-specific; custom `AttributeKey` is the right abstraction |
| Add `LimitPerTurn` / `LimitPerBattle` to `SkillDefinition` | Usage tracking is game-state; framework's `cooldown` covers time-based limits |
| Add `BuffRule` system to `framework_combat` | Too many DDGC-specific variants; `EffectCondition` covers the generic cases |
| Add reactive triggers (on-hit, on-damage) to `framework_combat` | Reactive hooks are game-specific; status-kind strings + game-layer checks are sufficient |

---

## Feedback Loop Process

When a migration blocker is discovered:

1. **Record** the blocker in the Active Blockers section with ID, classification, batch, description, and proposed resolution.
2. **Classify** as `core-gap`, `framework-gap`, or `game-gap`:
   - `core-gap` / `framework-gap`: the framework genuinely lacks a capability that would benefit *any* consumer, not just DDGC.
   - `game-gap`: the framework provides the building blocks; DDGC-specific logic belongs in the game layer.
3. **If core-gap or framework-gap**: patch the appropriate crate *and* add a regression test in that crate's test suite. The test must verify the generic capability, not DDGC-specific behavior.
4. **If game-gap**: implement in `game_ddgc_headless/src/`. No framework changes.
5. **If a backflow request is rejected**: document it in the Rejected Backflow Requests table with the rejection reason.
6. **Verify**: no DDGC-specific constants, types, or rule branches exist in framework or core crates. The integration test `no_ddgc_content_in_framework_crates` enforces this at build time.

### Regression Test Requirement

Every core-gap or framework-gap patch **must** include a regression test in the patched crate's test suite. The test verifies the generic capability (e.g., "attributes support arbitrary string keys" not "stress attribute works"). No test in a framework crate may reference DDGC-specific names (Crusader, Vestal, bone_soldier, necromancer, DDGC, stress as a game concept, etc.).

---

## Blocker Summary

| ID | Classification | Batch | Status |
|---|---|---|---|
| B-001 | game-gap | 1 | Resolved |
| B-002 | game-gap | 1 | Resolved |
| B-003 | game-gap | 1 | Resolved |
| B-004 | game-gap | 2 | Resolved |
| B-005 | game-gap | 3 | Resolved |
| B-006 | game-gap | 3 | Resolved |
| B-007 | game-gap | 3 | Resolved |
| B-008 | game-gap | 2 | Resolved |
| B-009 | game-gap | 3 | Resolved |
| B-010 | game-gap | 3 | Resolved |