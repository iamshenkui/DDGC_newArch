# PRD: DDGC H5 Demo P0 Chaos Dungeon Run

## Introduction

This PRD starts the first H5 demo migration slice inside `DDGC_newArch/frontend`.
The goal is not to continue the full Unity-to-new-architecture migration. The
goal is to create an isolated, playable H5 demo foundation that can validate the
DDGC-like combat, dungeon-event, and chaos-meter loop in a short browser
session.

This slice must stay frontend-only. It may add a deterministic TypeScript demo
runtime under the frontend workspace, demo content, one or more demo
screens, and focused validation. It must not move gameplay truth into the
existing town/campaign flow, modify Rust runtime APIs, or migrate long-term town
systems.

## Goals

- Add an isolated DDGC H5 demo foundation under `frontend/src/demo` and `frontend/src/screens/demo`.
- Provide a deterministic reducer-driven mini runtime for a 10-minute chaos-dungeon run.
- Expose a browser entry path through `DdgcApp` without breaking the existing DDGC startup, replay, live, town, expedition, combat, result, and return flow.
- Add seed content for a minimal party, enemy group, dungeon room sequence, and chaos meter.
- Add focused validation that proves the demo runtime can advance through its first loop and that the existing frontend build path still works.

## Functional Requirements

- The demo must be reachable through a clearly isolated frontend path, such as a query parameter, mode switch, or demo launcher path.
- The demo runtime must expose explicit `GameState`, `GameAction`, and reducer-style transition functions.
- The runtime must be deterministic from a seed and must not depend on browser globals.
- The first loop must support at least: start run, enter dungeon, resolve one dungeon event, enter combat, execute at least one player combat action, update chaos, and produce a result or continue state.
- Demo code must be separated from existing DDGC production screens and session flow.
- Existing frontend scripts must remain valid: typecheck, build, and browser smoke where available.

## Non-Goals

- Do not migrate the full Steam game.
- Do not migrate or rebuild the town meta loop.
- Do not add long-term progression, persistence compatibility, large map systems, full
  story, inventory, building services, or economy.
- Do not modify `WorldEngine`, `glowing-fishstick-framwork`, or Rust gameplay
  crates for this first slice.
- Do not replace the current `DdgcApp` replay/live flow.
- Do not introduce a new framework, state library, deployment platform, or
  external telemetry provider.

## Technical Considerations

- Target repository: `iamshenkui/DDGC_newArch`.
- Primary working directory: `frontend/`.
- Prefer the existing Solid, Vite, and Playwright conventions already present in the frontend workspace.
- Anchor the browser entry in `DdgcApp` while preserving the existing `StartupScreen` default path.
- Keep demo runtime logic in TypeScript so it can be validated without a browser.
- Keep the first implementation small enough for a single iteration. If the
  worker determines the task is too broad, it should split along frontend demo
  runtime, demo screen wiring, and focused validation.
- The demo entry must not destabilize existing smoke checks that expect the
  current startup screen by default.

## Success Metrics

- `npm run typecheck` passes in `frontend/`.
- `npm run build` passes in `frontend/`.
- A focused demo runtime check proves the first chaos-dungeon loop can advance
  deterministically.
- The default app path still boots the existing DDGC startup flow.
- The demo path renders without a fatal error and shows chaos, party, dungeon, and combat/event state in the browser.

## User Stories

### US-001: Add isolated H5 demo foundation in the frontend

**Description:** As the DDGC migration operator, I want an isolated H5 demo foundation inside `DDGC_newArch/frontend`, anchored through `DdgcApp` and preserving `StartupScreen`, so the first chaos-dungeon browser slice can be iterated by agents without touching the full Unity migration, existing town meta-loop, Rust runtime, or generic engine repositories.

**Acceptance Criteria:**

- `frontend/src/demo` or an equivalent isolated folder contains typed `GameState`, `GameAction`, deterministic reducer logic, seed content, and a minimal chaos-meter value for the first demo loop.
- `frontend/src/screens/demo` or an equivalent isolated screen folder contains a demo surface that can render the current run phase, party summary, enemy or dungeon-event summary, chaos value, and at least one primary action button.
- The demo is reachable through an explicit browser entry path in `DdgcApp` that does not replace the default `StartupScreen` startup flow.
- The first deterministic loop supports starting a run, entering a dungeon room, resolving an event or combat step, changing chaos, and reaching a continuing or result state.
- A focused validation check covers the reducer loop from seed state through at least three actions and verifies deterministic transitions.
- Existing frontend default behavior remains intact: the default browser path still shows the existing DDGC startup screen.
- `npm run typecheck` and `npm run build` pass from `frontend/`.
