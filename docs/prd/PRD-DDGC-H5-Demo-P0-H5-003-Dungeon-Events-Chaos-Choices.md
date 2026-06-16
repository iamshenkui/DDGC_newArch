# PRD: DDGC H5 Demo P0 H5-003 Dungeon Events and Chaos Choices

## Introduction

This PRD defines the third executable slice for the DDGC H5 demo. H5-003 should
add a small deterministic dungeon event pool with risk and reward choices tied
to the chaos meter. The goal is to validate the short-loop dungeon decision
experience without bringing in the full Unity dungeon, town, economy, or story
systems.

This slice must remain frontend-only and build on the isolated H5 demo runtime
created in earlier slices.

## Goals

- Add a small pool of deterministic dungeon event definitions.
- Add event choices that increase, reduce, or spend chaos.
- Surface event choices in the demo UI.
- Record deterministic event log entries for player choices.
- Keep the frontend typecheck, test, and build scripts passing.

## Functional Requirements

- At least four event definitions must exist in demo seed content or equivalent
  isolated content.
- Each event must expose at least two player choices.
- Choices must produce deterministic state changes.
- Choices must visibly affect chaos, party state, room state, or run log.
- The demo UI must present the current event and available choices.
- Event resolution must lead back into the existing demo loop without dead-end
  states.

## Non-Goals

- Do not migrate the full Steam event system.
- Do not add full narrative content, localization, inventory, economy, or town
  progression.
- Do not modify Rust runtime, `WorldEngine`, or `glowing-fishstick-framwork`.
- Do not replace the existing default DDGC startup/replay/live flow.
- Do not introduce a new frontend framework or external state library.

## Technical Considerations

- Target repository: `iamshenkui/DDGC_newArch`.
- Primary workspace: `frontend/`.
- Expected dependency: H5-002 combat loop should exist before this slice runs.
- Prefer changes under `frontend/src/demo` and `frontend/src/screens/demo`.
- Keep event and chaos logic deterministic and testable outside the browser.
- Preserve the isolated demo entry path.

## Success Metrics

- `npm run typecheck` passes from `frontend/`.
- `npm run test` passes from `frontend/`.
- `npm run build` passes from `frontend/`.
- A focused reducer test verifies event choice state changes and chaos changes.
- The demo UI shows the current event, choices, resulting log entries, and chaos
  movement.

## User Stories

### US-001: Add deterministic dungeon choices and chaos changes

**Description:** As the DDGC H5 demo operator, I want dungeon events with
chaos-linked choices in the isolated demo runtime, so the browser demo can test
the risk/reward decision loop alongside combat.

**Acceptance Criteria:**

- At least four deterministic dungeon event definitions exist in isolated demo
  content.
- Each event has at least two choices.
- Event choices deterministically change chaos, party values, room progress, or
  run log.
- Chaos changes from event choices are visible in the demo UI.
- Automated frontend validation covers at least two event choices and verifies
  deterministic outcomes.
- Existing default app startup behavior remains unchanged outside the demo path.
- `npm run typecheck`, the existing frontend validation check, and
  `npm run build` pass from `frontend/`.
