# Encounter-Runtime Fidelity Inventory — Phase 3

**Branch:** `ralph/ddgc-encounter-runtime-fidelity-closure`
**Story:** US-701 (P3-0)
**Date:** 2026-04-15

This document is the checked-in inventory for Phase 3 (DDGC Targeting, Boss Runtime Mechanics, And Run-Flow Fidelity).

---

## Gap Category 1: Targeting Simplifications (Content-Model Gap)

**Description:** All migrated skills use `TargetSelector::AllEnemies` or `TargetSelector::AllAllies` regardless of DDGC's rank-based targeting semantics. Single-target, self-target, ally-only, and rank-gating are flattened to group selectors.

**Root Cause:** Framework's `ByPosition(SlotRange)` is available but side-blind (targets absolute slot indices regardless of CombatSide). Migration uses broad selectors as a safe default.

**Migrated Content Affected:**

| Content | File | Skill | DDGC Targeting | Current Framework | Gap |
|---------|------|-------|---------------|-------------------|-----|
| Hunter hero | `content/heroes/hunter.rs` | `mark_skill` | Single-target, ally with mark | AllAllies | Single-target flattened |
| Tank hero | `content/heroes/tank.rs` | `protect_skill` | Single ally guard | AllAllies | Ally-target flattened |
| Tank hero | `content/heroes/tank.rs` | `taunt_skill` | Self-only (applies to self) | SelfOnly | ✓ OK |
| Tank hero | `content/heroes/tank.rs` | `regression` | Ally targeting | AllAllies | Ally-target flattened |
| Shaman hero | `content/heroes/shaman.rs` | `buff_self` | Self-only | SelfOnly | ✓ OK |
| Diviner hero | `content/heroes/diviner.rs` | `draw_stick` | Self-only (oracle) | SelfOnly | ✓ OK |
| Alchemist hero | `content/heroes/alchemist.rs` | `heal_single` | Single ally | AllAllies | Ally-target flattened |
| Hunter | `content/heroes/hunter.rs` | `pull_skill` | Single enemy | AllEnemies | Single-target flattened |
| All monster skills | Various | Various | Rank 1-2, rank 3-4, etc. | AllEnemies | Rank-gating flattened |

**Implementation Anchors for Targeting Slices:**
- **US-703 single-target anchor:** `mark_skill` (Hunter) — a skill that should hit exactly one marked ally, currently hits all allies
- **US-703 ally-target anchor:** `protect_skill` (Tank) — a skill that should guard exactly one ally, currently affects all allies
- **US-704 rank-gating anchor:** Any monster with rank-restricted skills (e.g., mantis families use rank 1-2 attacks)

---

## Gap Category 2: Boss Runtime Mechanics (Encounter-Runtime Gaps)

**Description:** Boss packs are registered with correct family compositions, but authored runtime behaviors (summon, shared-health, multi-phase, captor) are not yet active in encounter resolution.

### 2a: Summon Runtime

| Boss | File | Expected Behavior | Current Behavior |
|------|------|-----------------|-----------------|
| rotvine_wraith | `content/monsters/rotvine_wraith.rs` | Re-summons rotten_fruit minions each phase | rotvine_wraith uses `apply_status("summon_rotten_fruit", Some(N))` — no actual spawn |
| gambler | `content/monsters/gambler.rs` | Summons mahjong tiles during fight | Skill applies summon status markers — no actual spawn |
| scorchthroat_chanteuse | `content/monsters/scorchthroat_chanteuse.rs` | Summons sc_blow/sc_bow during fight | Skill applies summon status markers — no actual spawn |

**US-707/US-708 anchor:** `rotvine_wraith` — the clearest summon pattern (re-summons rotting produce each round).

### 2b: Shared Health / Linked-Body Damage

| Boss | File | Expected Behavior | Current Behavior |
|------|------|-----------------|-----------------|
| azure_dragon | `content/monsters/azure_dragon.rs` + azure_dragon_ball_*.rs | All 3 balls share one health pool | Each ball has independent HP |
| black_tortoise | `content/monsters/black_tortoise_*.rs` | Tortoise A + Snake B share damage via `share_damage` status | Each body has independent HP; `share_damage` is a marker status only |
| vermilion_bird | `content/monsters/vermilion_bird.rs` + tail_*.rs | Bird + 2 tails share health | Each part has independent HP |

**US-709 anchor:** `azure_dragon` — the clearest shared-health case (3 balls with one pooled HP).

### 2c: Multi-Phase / Transition-State

| Boss | File | Expected Behavior | Current Behavior |
|------|------|-----------------|-----------------|
| white_tiger | `content/monsters/white_tiger_*.rs` | Transitions between terrain/phase forms with different abilities | All 3 units present simultaneously; no phase transition logic |
| frostvein_clam | `content/monsters/frostvein_clam.rs` | Enters empowered phase after pearlkin die | No phase transition triggered |

**US-710 anchor:** `white_tiger` — authored 3-phase sequence (terrain → A+B → final form).

### 2d: Captor / Release

| Boss | File | Expected Behavior | Current Behavior |
|------|------|-----------------|-----------------|
| necrodrake_embryosac | `content/monsters/necrodrake_embryosac.rs` + egg_membrane_*.rs | Heroes captured in egg_membrane, released after conditions | Heroes not placed in captive state; egg_membrane is a normal enemy unit |

**US-711 anchor:** `necrodrake_embryosac` + `egg_membrane_full` — captor mechanic (hero imprisoned in egg, released when egg destroyed).

### 2e: Controller / Paired-Boss

| Boss | File | Expected Behavior | Current Behavior |
|------|------|-----------------|-----------------|
| glutton_pawnshop | `content/monsters/glutton_pawnshop.rs` | Controller mechanic: tag-debuff cycle | `gp_control` status applied; no conditional tag behavior |
| bloodthirsty_assassin | `content/monsters/bloodthirsty_assassin.rs` + bloodthirsty_shadow.rs | Assassin + shadow paired; crimson_duet averages HP | Each unit has independent HP; `average_hp` is a marker status only |

**US-712 anchor:** `bloodthirsty_assassin` + `bloodthirsty_shadow` — HP-averaging and paired coordination.

---

## Gap Category 3: Run-Flow Transitional Fallbacks (Run-Flow Gap)

**Description:** Encounter resolution has deterministic paths for all 4 dungeons, but the battle execution uses a hard-coded skill assignment script rather than authored family action policies.

### 3a: Skill Assignment Script (vs. Family Action Policy)

| Component | File | Current | Gap |
|-----------|------|---------|-----|
| `EncounterResolver::run_battle` | `run/encounters.rs` | Uses `DEFAULT_PARTY` (fixed 4-hero team with fixed skills) + family first-skill | No per-family AI priority; heroes always use same skill |
| Ally party | `run/encounters.rs` | Hardcoded `DEFAULT_PARTY` | No hero chaos variant selection |

**US-706 anchor:** Any monster family with an authored AI priority that differs from "always use first registered skill." Example: `lizard` (controller: stun → intimidate → stress combo cycle) — current script would use first skill only.

### 3b: Fallback Battle Path

| Path | File | Trigger | Gap |
|------|------|---------|-----|
| `first_battle` fallback | `run/flow.rs` lines 128-139, 163-174 | `resolve_pack` or `resolve_boss_pack` returns `None` | Currently all 4 dungeons have packs — fallback never triggers for core dungeons |
| `first_battle` | `scenarios/first_battle.rs` | Standalone scenario | Uses legacy Crusader/Vestal heroes (~30 HP) vs DDGC-scale heroes (135-192 HP) |

**US-713 anchor:** `flow.rs` fallback block — while it doesn't trigger for core dungeons, it must be removed so migrated content is the only path.

---

## Already Migrated vs. Still Gapped

| Mechanic | Status | Notes |
|----------|--------|-------|
| Encounter pack registry | Migrated | All 4 dungeons have hall/room/boss packs |
| Boss pack resolution | Migrated | Deterministic via seed+room_index |
| Riposte detection + execution | Migrated (P1) | US-506: alligator_yangtze riposte works |
| Guard detection + redirect | Migrated (P1) | US-507/US-508: guard redirect works |
| Skill usage counters/limits | Migrated (P1) | US-510-514: per-turn/per-battle limits work |
| Reactive event tracing | Migrated (P1) | US-504: trace entries for reactive events |
| Hero families (5 families, 3 variants each) | Migrated | Full registry + skill packs |
| Monster families (22 common + 12 boss) | Migrated | Full registry + skill packs |
| Formation placement | Migrated | Actors placed by slot index |
| Target resolution (broad selectors) | Migrated | AllEnemies/AllAllies work for damage resolution |
| Single-target selection | **GAME-GAP** | B-007 reclassified as game-gap; needs game-layer fix |
| Ally-target selection | **GAME-GAP** | Same as above |
| Rank-gating (launch/target ranks) | **GAME-GAP** | Same as above |
| Movement/reposition effects | **GAME-GAP** | push/pull modeled as EffectNode; formation slot update not wired |
| Summon materialize | **GAME-GAP** | US-707/US-708 pending |
| Shared-health linking | **GAME-GAP** | US-709 pending |
| Multi-phase transitions | **GAME-GAP** | US-710 pending |
| Captor/release | **GAME-GAP** | US-711 pending |
| Controller/paired-boss | **GAME-GAP** | US-712 pending |
| Family action policy (AI) | **GAME-GAP** | US-706 pending |
| Remove run fallbacks | **GAME-GAP** | US-713 pending |

---

## Priority Ordering for First Implementation Slices

Per US-701 acceptance criteria — at least one anchor per category:

1. **Targeting anchor (US-703):** `mark_skill` (Hunter) — clearest single-target case; or `protect_skill` (Tank) for ally-target
2. **Boss-runtime anchor (US-708):** `rotvine_wraith` summon materialize — self-contained, observable in trace
3. **Run-flow anchor (US-706):** `lizard` family action policy — authored combo (stun → intimidate → stress) is distinct from first-skill fallback
