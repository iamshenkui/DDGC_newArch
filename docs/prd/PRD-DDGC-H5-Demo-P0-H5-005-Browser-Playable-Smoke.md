# PRD: DDGC H5 Demo P0 H5-005 Browser Playable Smoke

## Introduction

This PRD defines the fifth executable slice for the DDGC H5 demo. H5-005 should
add browser smoke coverage for the isolated demo path, including a mobile-sized
viewport and a short click-through of the first playable loop. The goal is to
catch fatal render, console, layout, and interaction regressions before the demo
is used for player-facing market validation.

This slice should not expand game scope. It should validate the demo loop that
already exists after earlier slices.

## Goals

- Add Playwright or existing browser smoke coverage for the isolated demo path.
- Cover at least one mobile viewport.
- Click through the first core loop far enough to reach event or combat state.
- Preserve existing browser smoke behavior for the default DDGC startup path.
- Keep the frontend typecheck, test, browser smoke, and build scripts passing
  where available.

## Functional Requirements

- The demo path must load without fatal page errors.
- The smoke test must watch for console or page errors where the existing test
  stack supports it.
- The test must interact with the primary action on a mobile viewport.
- The test must reach event, combat, room, or result state within a short
  timeout.
- Existing default startup smoke checks must remain valid.

## Non-Goals

- Do not add full visual parity testing.
- Do not add production analytics or external telemetry.
- Do not migrate more Unity systems.
- Do not modify Rust runtime, `WorldEngine`, or `glowing-fishstick-framwork`.
- Do not introduce a new test framework if the frontend already has one.

## Technical Considerations

- Target repository: `iamshenkui/DDGC_newArch`.
- Primary workspace: `frontend/`.
- Expected dependency: H5-004 short-run loop and result state should exist
  before this slice runs.
- Prefer existing frontend Playwright or browser smoke conventions.
- Keep smoke selectors stable and tied to user-visible controls or test-safe
  attributes.
- Preserve the isolated demo entry path.

## Success Metrics

- `npm run typecheck` passes from `frontend/`.
- `npm run test` passes from `frontend/`.
- `npm run build` passes from `frontend/`.
- The available browser smoke command passes or a focused equivalent is added
  and documented in the test file/package scripts.
- The smoke test covers a mobile viewport and clicks through the first demo
  interaction loop.

## User Stories

### US-001: Add browser smoke check for the playable H5 demo

**Description:** As the DDGC H5 demo operator, I want browser smoke automation
for the isolated demo path, so automated iteration can detect whether the demo
still opens and can be played through the first interaction loop on mobile-sized
screens.

**Acceptance Criteria:**

- A browser smoke check opens the isolated demo path.
- The smoke check uses a mobile viewport or mobile-sized viewport.
- The smoke check clicks at least one primary demo action and reaches event,
  combat, room, or result output within a short timeout.
- The smoke check fails on fatal page errors and checks console errors where the
  existing frontend harness supports it.
- Existing default startup smoke behavior remains valid.
- `npm run typecheck`, the existing frontend validation check, and
  `npm run build` pass from `frontend/`.
