# PRD: DDGC H5 Demo P0 H5-006 Telemetry Mock and Experiment Record

## Introduction

This PRD defines the sixth executable slice for the DDGC H5 demo. H5-006 should
add a local telemetry event schema, a mock collector, and an experiment metadata
record for the demo. The goal is not to integrate a production analytics
provider. The goal is to make the demo ready for low-cost market experiments by
standardizing the local event shape that later pipelines can forward or export.

This slice must remain frontend-only and use local or in-memory telemetry.

## Goals

- Add a local telemetry event schema for the H5 demo.
- Add an in-memory or local mock collector.
- Emit key demo lifecycle and run events.
- Add an experiment metadata record for this H5 demo.
- Keep the frontend typecheck, test, browser smoke, and build scripts passing
  where available.

## Functional Requirements

- Telemetry events must include `experiment_id`, `session_id`, `run_id`,
  `event_name`, and timestamp fields.
- The demo must emit at least `game_loaded`, `first_action`, `chaos_changed`,
  and `run_completed` or equivalent local events.
- The mock collector must be local and must not send data to an external
  provider.
- A focused test must verify event shape and at least one emitted event sequence.
- An experiment metadata file must identify the demo, slice, hypothesis, target
  metrics, and local telemetry event names.

## Non-Goals

- Do not integrate a production analytics SDK.
- Do not send data over the network.
- Do not add paid acquisition tooling.
- Do not add a dashboard or market pipeline in this slice.
- Do not modify Rust runtime, `WorldEngine`, or `glowing-fishstick-framwork`.

## Technical Considerations

- Target repository: `iamshenkui/DDGC_newArch`.
- Primary workspace: `frontend/`.
- Expected dependency: H5-005 browser smoke should exist before this slice runs.
- Prefer isolated files under `frontend/src/demo` or an equivalent demo-only
  folder.
- Keep telemetry deterministic enough to test by injecting ids or using local
  factories.
- Preserve the isolated demo entry path and default DDGC startup path.

## Success Metrics

- `npm run typecheck` passes from `frontend/`.
- `npm run test` passes from `frontend/`.
- `npm run build` passes from `frontend/`.
- A focused test verifies local event shape and event emission.
- The experiment metadata file documents the demo hypothesis and target metrics.

## User Stories

### US-001: Add local telemetry capture and experiment metadata

**Description:** As the DDGC H5 demo operator, I want local telemetry events and
an experiment metadata record, so the demo can support future market validation
without coupling this slice to an external analytics provider.

**Acceptance Criteria:**

- Local telemetry events include `experiment_id`, `session_id`, `run_id`,
  `event_name`, and timestamp fields.
- The demo emits at least `game_loaded`, `first_action`, `chaos_changed`, and
  `run_completed` or equivalent local events.
- The telemetry collector is local or in-memory and does not send network calls.
- Automated frontend validation verifies event shape and at least one emitted
  event sequence.
- An experiment metadata record captures demo name, hypothesis, target metrics,
  and event names.
- Existing default app startup behavior remains unchanged outside the demo path.
- `npm run typecheck`, the existing frontend validation check, and
  `npm run build` pass from `frontend/`.
