# Tasklist: DDGC H5 Demo P0 Chaos Dungeon Run

## Purpose

This tasklist splits the DDGC H5 Demo P0 into issue-sized slices. Salvager's
current issue-enqueue MVP supports exactly one generated task per issue, so each
slice below must become its own PRD/issue when it is ready to run.

The first active issue used:

```text
docs/prd/PRD-DDGC-H5-Demo-P0-Chaos-Dungeon-Run.md
```

The follow-up executable slice PRDs are:

```text
docs/prd/PRD-DDGC-H5-Demo-P0-H5-002-Combat-Loop.md
docs/prd/PRD-DDGC-H5-Demo-P0-H5-003-Dungeon-Events-Chaos-Choices.md
docs/prd/PRD-DDGC-H5-Demo-P0-H5-004-Ten-Minute-Run-Result.md
docs/prd/PRD-DDGC-H5-Demo-P0-H5-005-Browser-Playable-Smoke.md
docs/prd/PRD-DDGC-H5-Demo-P0-H5-006-Telemetry-Mock-Experiment-Record.md
```

## Execution Rule

- Target repo: `iamshenkui/DDGC_newArch`
- Target root: `/home/dev/GameMigration/repositories/DDGC_newArch`
- Primary workspace: `frontend/`
- First active slice: `H5-001` completed locally through issue 116.
- Follow-up slices: create one `portolan:ready` issue per slice.
- Keep `DreamDeveloperGame-Crossover` read-only.
- Do not modify `WorldEngine` or `glowing-fishstick-framwork` unless a later
  issue explicitly identifies a generic gap.

## Slice Order

### H5-001: Frontend demo foundation

Status: complete locally through issue 116 and task
`GH-iamshenkui-DDGC-newArch-116-US-001`

Scope:

- Add isolated `frontend/src/demo` runtime and seed content.
- Add isolated `frontend/src/screens/demo` rendering surface.
- Add explicit demo browser entry path without changing default startup.
- Add focused reducer test.
- Keep existing frontend scripts passing.

Acceptance:

- The default browser path still shows the existing DDGC startup screen.
- The demo path renders party, dungeon/event or combat state, chaos value, and a
  primary action button.
- Reducer test advances at least three actions deterministically.
- `npm run typecheck`, `npm run test`, and `npm run build` pass.

### H5-002: Demo combat loop

Status: complete locally through issue 117 and task
`GH-iamshenkui-DDGC-newArch-117-US-001`; commit `4c2ca421acee`

Scope:

- Expand demo runtime combat actions.
- Add four-party and enemy state enough for one encounter.
- Support target selection or deterministic default targeting.
- Produce combat log events.

Acceptance:

- A seeded combat can start and end.
- At least one hero skill changes enemy HP.
- At least one enemy action changes party HP or stress/chaos.
- Bot-style reducer test can resolve the encounter without browser APIs.

### H5-003: Dungeon event and chaos choices

Status: complete locally through issue 118 and task
`GH-iamshenkui-DDGC-newArch-118-US-001`; commit `ba35945e59be`

Scope:

- Add a small event pool.
- Add risk/reward choices that increase, reduce, or spend chaos.
- Surface the event choice in the demo screen.

Acceptance:

- At least four event definitions exist.
- Each event has at least two choices.
- Choices produce deterministic state transitions and event log entries.
- Chaos changes are visible in the UI and reducer tests.

### H5-004: Ten-minute run loop and result state

Status: complete locally through issue 119 and task
`GH-iamshenkui-DDGC-newArch-119-US-001`; commit `32556dc3cb96`

Scope:

- Add room sequence and end conditions.
- Add run completion, party wipe, or boss/end result state.
- Add result screen surface.

Acceptance:

- A seeded run can reach a terminal result.
- Result shows outcome, room count, final chaos, and party survival.
- Runtime test proves no dead-end state for the seed fixture.

### H5-005: Browser playable smoke

Status: complete locally through issue 120 and task
`GH-iamshenkui-DDGC-newArch-120-US-001`; commit `73a502f00dfd`

Scope:

- Add Playwright smoke for the demo path.
- Cover mobile viewport.
- Click through the first core loop.

Acceptance:

- Demo path loads without console or page errors.
- Primary action can be clicked on a mobile viewport.
- The test reaches event or combat state within a short timeout.
- Existing browser smoke remains valid.

### H5-006: Telemetry mock and experiment record

Status: complete locally through issue 121 and task
`GH-iamshenkui-DDGC-newArch-121-US-001`; commit `906f4867e94e`

Scope:

- Add local telemetry event schema for the demo.
- Record events to an in-memory or local mock collector.
- Add an experiment metadata file for this demo.

Acceptance:

- Events include `experiment_id`, `session_id`, `run_id`, `event_name`, and
  timestamp.
- Demo emits at least `game_loaded`, `first_action`, `chaos_changed`, and
  `run_completed` or equivalent local events.
- A focused test verifies event shape.
