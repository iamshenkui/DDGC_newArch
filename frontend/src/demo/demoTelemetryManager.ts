/**
 * Telemetry lifecycle manager for the H5 demo.
 *
 * This class encapsulates the logic for when and which telemetry
 * events to emit during a demo session.  It is designed to be
 * testable without a browser or UI framework — all inputs are plain
 * data and the outputs are plain events appended to the collector.
 *
 * Typical usage from DemoScreen:
 *   1. Create one manager instance per component mount.
 *   2. Call onGameLoaded() after the seed state is created.
 *   3. Call onAction() inside the dispatch wrapper.
 *   4. Call onRunReset() when the user starts a new run.
 *   5. After each notification, read getCollector().getEvents().
 */

import { DEMO_EXPERIMENT_ID } from "./telemetry";
import type { GameAction, GameState } from "./types";
import { TelemetryCollector } from "./telemetryCollector";
import type { TelemetryEvent } from "./telemetry";

// ── ID Generation ───────────────────────────────────────────────────────────

/** Generate a short random hex identifier (8 chars). */
function shortId(): string {
  return Math.floor(Math.random() * 0xffffffff)
    .toString(16)
    .padStart(8, "0");
}

// ── Manager ─────────────────────────────────────────────────────────────────

export class DemoTelemetryManager {
  readonly collector: TelemetryCollector;
  readonly sessionId: string;
  private _runId: string;
  private _hasFiredFirstAction: boolean;
  private _previousChaos: number;

  /**
   * @param collector  Optional shared collector.  Defaults to a new instance.
   */
  constructor(collector?: TelemetryCollector) {
    this.collector = collector ?? new TelemetryCollector();
    this.sessionId = shortId();
    this._runId = shortId();
    this._hasFiredFirstAction = false;
    this._previousChaos = 0;
  }

  // ── Run ID ──────────────────────────────────────────────────────────────

  get runId(): string {
    return this._runId;
  }

  // ── Lifecycle Hooks ──────────────────────────────────────────────────────

  /**
   * Call once when the demo screen mounts, after the seed state is created.
   * Emits a game_loaded event.
   */
  onGameLoaded(state: GameState): void {
    this._previousChaos = state.chaosMeter;
    this.capture({
      event_type: "game_loaded",
      payload: {
        demoName: "Chaos Dungeon H5 Demo",
        partySize: state.party.length,
        initialChaos: state.chaosMeter,
        initialDungeonLevel: state.dungeonLevel,
      },
    });
  }

  /**
   * Call on every action dispatch, before the reducer runs.
   * May emit first_action and/or chaos_changed events based on the
   * action and the current state.
   *
   * @param action  The action being dispatched.
   * @param before  The state before the action is applied.
   */
  onAction(action: GameAction, before: GameState): void {
    // ── First action ─────────────────────────────────────
    if (!this._hasFiredFirstAction) {
      this._hasFiredFirstAction = true;
      this.capture({
        event_type: "first_action",
        payload: {
          actionType: action.type,
          turnCount: before.turnCount,
          chaosMeter: before.chaosMeter,
        },
      });
    }
  }

  /**
   * Call after the state has been updated by the reducer.
   * Detects chaos-meter changes and terminal-state transitions.
   *
   * @param after  The state after the action was applied.
   */
  onStateChanged(after: GameState): void {
    // ── Chaos changed ─────────────────────────────────────
    if (after.chaosMeter !== this._previousChaos) {
      this.capture({
        event_type: "chaos_changed",
        payload: {
          previousChaos: this._previousChaos,
          newChaos: after.chaosMeter,
          delta: after.chaosMeter - this._previousChaos,
          turnCount: after.turnCount,
          phase: after.phase,
        },
      });
      this._previousChaos = after.chaosMeter;
    }

    // ── Run ended ─────────────────────────────────────────
    if (after.phase === "result" && after.runOutcome) {
      this.capture({
        event_type: "run_ended",
        payload: {
          outcome: after.runOutcome,
          finalChaos: after.chaosMeter,
          dungeonLevel: after.dungeonLevel,
          roomCount: after.roomCount,
          turnCount: after.turnCount,
          survivalCount: after.party.filter((m) => m.hp > 0).length,
        },
      });
    }
  }

  /**
   * Call when the run is reset (e.g. the user clicks "New Run").
   * Generates a new run_id and resets per-run flags.
   *
   * @returns The new run_id.
   */
  onRunReset(): string {
    this._runId = shortId();
    this._hasFiredFirstAction = false;
    this._previousChaos = 0;
    return this._runId;
  }

  // ── Internal ───────────────────────────────────────────────────────────

  private capture(
    partial: Omit<TelemetryEvent, "experiment_id" | "session_id" | "run_id" | "timestamp">,
  ): void {
    this.collector.capture({
      experiment_id: DEMO_EXPERIMENT_ID,
      session_id: this.sessionId,
      run_id: this._runId,
      timestamp: Date.now(),
      ...partial,
    } as TelemetryEvent);
  }
}
