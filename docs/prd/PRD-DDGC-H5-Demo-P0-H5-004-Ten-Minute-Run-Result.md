# PRD: DDGC H5 Demo P0 H5-004 Ten Minute Run and Result State

## Introduction

This PRD defines the fourth executable slice for the DDGC H5 demo. H5-004 should
turn the isolated demo loop into a short complete run with room progression,
terminal outcomes, and a result state. The goal is to make a player finish a
small browser run in one sitting without adding the full town meta-loop or
long-term campaign systems.

This slice must remain frontend-only and build on the existing H5 demo runtime.

## Goals

- Add a deterministic room sequence for a short run.
- Add terminal conditions for completion, party wipe, retreat, or equivalent
  result outcomes.
- Add a result state and result screen surface.
- Ensure seeded runs cannot get stuck in a dead-end state.
- Keep the frontend typecheck, test, and build scripts passing.

## Functional Requirements

- The runtime must track room count or equivalent run progress.
- A seeded run must be able to reach a terminal result state.
- The result state must summarize outcome, room count, final chaos, and party
  survival.
- The demo UI must render a result surface when the run ends.
- Reducer tests must prove the seed fixture can reach a terminal result without
  browser APIs.
- The default DDGC startup flow must remain unchanged outside the demo path.

## Non-Goals

- Do not add the full town meta-loop.
- Do not add long-term progression, saves, inventory, economy, buildings, or
  campaign compatibility.
- Do not migrate the full Steam dungeon map.
- Do not modify Rust runtime, `WorldEngine`, or `glowing-fishstick-framwork`.
- Do not introduce external telemetry or deployment systems in this slice.

## Technical Considerations

- Target repository: `iamshenkui/DDGC_newArch`.
- Primary workspace: `frontend/`.
- Expected dependency: H5-003 event and chaos choice loop should exist before
  this slice runs.
- Prefer changes under `frontend/src/demo` and `frontend/src/screens/demo`.
- Keep run simulation deterministic and testable outside the browser.
- Preserve the isolated demo entry path.

## Success Metrics

- `npm run typecheck` passes from `frontend/`.
- `npm run test` passes from `frontend/`.
- `npm run build` passes from `frontend/`.
- A reducer test reaches a terminal result for the seeded run.
- The demo UI renders a result state with outcome, room count, final chaos, and
  party survival.

## User Stories

### US-001: Add complete short DDGC demo outcome

**Description:** As the DDGC H5 demo operator, I want the isolated demo runtime
to complete a short dungeon run and show a result screen, so the browser demo
can validate a full 10-minute loop instead of only individual combat or event
steps.

**Acceptance Criteria:**

- The runtime tracks room count or equivalent short-run progress.
- A seeded run can reach a terminal completion, defeat, retreat, or equivalent
  result outcome.
- The result output includes outcome, room count, final chaos, and party survival
  information.
- The demo screen renders a clear result surface when the run ends.
- Automated frontend validation proves the seed fixture can reach a terminal
  result without a dead-end.
- Existing default app startup behavior remains unchanged outside the demo path.
- `npm run typecheck`, the existing frontend validation check, and
  `npm run build` pass from `frontend/`.
