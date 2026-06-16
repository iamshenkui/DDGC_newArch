/**
 * Telemetry event types for the isolated chaos-dungeon H5 demo.
 *
 * These events capture local-only observability data for the demo
 * experiment.  No events are sent over the network — the collector
 * is purely in-memory and scoped to the demo.
 *
 * Required shared fields on every event:
 *   experiment_id — identifies which experiment this run belongs to
 *   session_id    — stable for one page-load session
 *   run_id        — identifies one run attempt (multiple runs per session)
 *   event_type    — discriminator for the event kind
 *   timestamp     — ms since Unix epoch (Date.now())
 *
 * Defined event types:
 *   game_loaded    — emitted once when the demo screen mounts
 *   first_action   — emitted on the first user interaction in a run
 *   chaos_changed  — emitted whenever the chaos-meter value changes
 *   run_ended      — emitted when the run reaches a terminal result
 */

/** Unique identifier assigned to the H5 demo experiment. */
export const DEMO_EXPERIMENT_ID = "ddgc-h5-demo-001";

// ── Shared Fields ───────────────────────────────────────────────────────────

export interface TelemetryEventBase {
  experiment_id: string;
  session_id: string;
  run_id: string;
  event_type: string;
  timestamp: number;
}

// ── Event Payloads ──────────────────────────────────────────────────────────

export interface GameLoadedPayload {
  demoName: string;
  partySize: number;
  initialChaos: number;
  initialDungeonLevel: number;
}

export interface FirstActionPayload {
  actionType: string;
  turnCount: number;
  chaosMeter: number;
}

export interface ChaosChangedPayload {
  previousChaos: number;
  newChaos: number;
  delta: number;
  turnCount: number;
  phase: string;
}

export interface RunEndedPayload {
  outcome: string;
  finalChaos: number;
  dungeonLevel: number;
  roomCount: number;
  turnCount: number;
  survivalCount: number;
}

// ── Event Unions ────────────────────────────────────────────────────────────

export interface GameLoadedEvent extends TelemetryEventBase {
  event_type: "game_loaded";
  payload: GameLoadedPayload;
}

export interface FirstActionEvent extends TelemetryEventBase {
  event_type: "first_action";
  payload: FirstActionPayload;
}

export interface ChaosChangedEvent extends TelemetryEventBase {
  event_type: "chaos_changed";
  payload: ChaosChangedPayload;
}

export interface RunEndedEvent extends TelemetryEventBase {
  event_type: "run_ended";
  payload: RunEndedPayload;
}

/** Discriminated union of all known telemetry event types. */
export type TelemetryEvent =
  | GameLoadedEvent
  | FirstActionEvent
  | ChaosChangedEvent
  | RunEndedEvent;

/** Event-type string literal union for use in type guards. */
export type TelemetryEventType = TelemetryEvent["event_type"];
