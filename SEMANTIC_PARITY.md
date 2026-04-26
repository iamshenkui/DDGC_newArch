# Semantic Parity Definitions

This document establishes the shared vocabulary for discussing migration fidelity
between the original DDGC game and the headless migration. When the question
"does this count as parity?" arises, the definitions below provide the answer.

---

## Terminology

### Semantic Parity

A migrated feature has **semantic parity** with the original when every
player-observable behavior produces identical outcomes for identical inputs.
The internal implementation may differ (different class hierarchy, different
method dispatch, different data structures) as long as observable results match.

### Structural Translation

A **structural translation** re-expresses the original game's logic using the
framework's generic types without changing behavioral outcomes. The mapping
from original concepts to framework types is documented in `MIGRATION_MAP.md`.

Examples:
- DDGC `DefenseRating` → framework `ATTR_DEFENSE` with game-layer convention
- DDGC `BuffRule` → framework `StatusEffect` + `Modifier` + `StackRule`

### Game Gap

A **game gap** exists when the framework provides the building blocks but the
game layer has not yet implemented the DDGC-specific logic. No framework
change is needed — only game-layer code.

Example: Skill usage limits (`LimitPerTurn`, `LimitPerBattle`) can be tracked
in game-layer state alongside the combat resolver; the framework's
`SkillDefinition` does not need modification.

### Framework Gap

A **framework gap** exists when the framework genuinely lacks a capability
that would benefit any consumer, not just DDGC. Resolution requires patching
a framework crate with a regression test.

The current migration has no framework gaps. See `MIGRATION_BLOCKERS.md` for
the full classification of all blockers; every active blocker is a game-gap.

### Acceptable Approximation

An **acceptable approximation** is a known behavioral difference where the
migration's deviation from the original is bounded, documented, and can be
restored without changing the parity test structure. Acceptable approximations
are tracked in `SEMANTIC_GAPS.md` with their rationale.

Example: Damage range averaging (B-006) uses fixed averages instead of
min/max rolls. Variance can be restored via a game-layer damage roll step
without changing any parity test.

### Unacceptable Semantic Drift

**Unacceptable semantic drift** occurs when the migration produces a
qualitatively different player experience that cannot be restored by adding
game-layer code alone. Any instance of unacceptable semantic drift is a
release blocker.

The current migration has no instances of unacceptable semantic drift.

---

## Player-Observable Behavior Boundary

The parity boundary distinguishes **observable** from **internal** concerns:

### Observable (must match for parity)

- **Damage output**: The effective damage dealt by a skill, after all modifiers
  and conditions, must match the original's expected range.
- **Turn order**: Actors must act in the same speed-based priority as the
  original.
- **Status application**: Statuses must apply, stack, tick, and expire with
  the same rules and timing as the original.
- **Resource changes**: HP, stress, and other resources must change by the
  expected amounts at the expected times.
- **Skill availability**: Skills must be available under the same conditions
  (cooldowns, position constraints) as the original.
- **Combat outcomes**: Battles must resolve with the same winner given the
  same inputs.

### Internal (may differ without breaking parity)

- **Class hierarchy**: The migration uses framework types (`ActorAggregate`,
  `StatusEffect`, `SkillDefinition`) instead of DDGC-specific classes.
- **Method dispatch**: The migration uses framework APIs (`resolve_skill`,
  `CombatResolver`) instead of DDGC's original dispatch mechanisms.
- **Data layout**: Attribute storage uses `AttributeKey` + `Modifier` instead
  of DDGC's paired-attribute structs.
- **Status implementation**: Marker statuses (stun, riposte) use empty
  modifier lists with game-layer detection instead of DDGC's typed status
  classes.

---

## Relationship to MIGRATION_BLOCKERS.md

`MIGRATION_BLOCKERS.md` is the authoritative source for blocker classification
(core-gap, framework-gap, game-gap) and resolution status. This document
defines the *parity vocabulary* used to discuss those blockers; it does not
duplicate blocker content.

When a blocker affects parity, `SEMANTIC_GAPS.md` cross-references the
blocker ID and classifies the parity impact using the terms defined here.

For the complete inventory of remaining unresolved gaps, see
**[`SEMANTIC_GAP_MATRIX.md`](SEMANTIC_GAP_MATRIX.md)** — the canonical matrix
of all 19 remaining gaps (SM-001 through SM-019) classified by subsystem,
gameplay impact, frequency tier, and owning code path.
