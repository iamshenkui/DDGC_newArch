# PRD: DDGC H5 Demo P0 H5-002 Combat Loop

## Introduction

This PRD defines the second executable slice for the DDGC H5 demo. H5-001 added
an isolated frontend demo foundation. H5-002 should expand that foundation into
a deterministic combat loop that can resolve one encounter without depending on
browser globals or the full Unity migration.

This slice must remain frontend-only and scoped to the H5 demo path. It should
build on the existing `frontend/src/demo` runtime and `frontend/src/screens/demo`
surface, while keeping the default DDGC startup flow unchanged.

## Goals

- Expand the isolated H5 demo runtime with concrete combat actions.
- Represent party and enemy combat state enough to resolve one encounter.
- Support deterministic targeting or an explicit target selection path.
- Produce combat log entries that make player and enemy actions inspectable.
- Keep the frontend typecheck, test, and build scripts passing.

## Functional Requirements

- Combat state must include party member HP or equivalent survival state.
- Combat state must include enemy HP or equivalent defeat state.
- At least one hero action must change enemy state.
- At least one enemy action must change party state, stress, or chaos.
- The reducer must be able to start and finish a seeded combat encounter.
- The demo screen must show enough combat state for a player to understand the
  current encounter and take the next primary action.

## Non-Goals

- Do not migrate the full Steam combat system.
- Do not add long-term progression, inventory, equipment, town services, or
  persistence compatibility.
- Do not modify Rust runtime, `WorldEngine`, or `glowing-fishstick-framwork`.
- Do not replace the existing default DDGC startup/replay/live flow.
- Do not introduce a new frontend framework or external state library.

## Technical Considerations

- Target repository: `iamshenkui/DDGC_newArch`.
- Primary workspace: `frontend/`.
- Expected dependency: H5-001 foundation from issue 116 is complete locally.
- Prefer changes under `frontend/src/demo` and `frontend/src/screens/demo`.
- Keep reducer logic deterministic and testable outside the browser.
- Preserve the `?demo=true` or equivalent isolated demo entry path.

## Success Metrics

- `npm run typecheck` passes from `frontend/`.
- `npm run test` passes from `frontend/`.
- `npm run build` passes from `frontend/`.
- A focused reducer test can resolve a seeded encounter.
- The demo UI shows party state, enemy state, combat log, chaos, and a primary
  combat action.

## User Stories

### US-001: Add deterministic H5 demo combat encounter

**Description:** As the DDGC H5 demo operator, I want the isolated demo runtime
to resolve a complete seeded combat encounter, so the first playable browser
demo can validate the core battle experience before adding more dungeon choices
and run-end logic.

**Acceptance Criteria:**

- A seeded combat can start from the existing demo fixture and reach a won,
  lost, fled, or continuing combat outcome without browser APIs.
- At least one hero action changes enemy HP or equivalent enemy combat values.
- At least one enemy action changes party HP, stress, chaos, or equivalent party
  combat values.
- Combat transitions append deterministic log entries that identify the actor,
  target, and result.
- The demo screen renders party combat values, enemy combat values, chaos, and a
  primary combat action.
- Existing default app startup behavior remains unchanged outside the demo path.
- `npm run typecheck`, the existing frontend validation check, and
  `npm run build` pass from `frontend/`.
