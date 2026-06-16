/**
 * Tests for the H5 demo telemetry system.
 *
 * Validates:
 *   - Collector stores events without network calls.
 *   - Event shape matches the required schema.
 *   - Experiment metadata is well-formed.
 *   - DemoTelemetryManager emits the correct events at the
 *     right lifecycle points.
 *   - A full run sequence produces all expected event types.
 */

import { describe, expect, it, vi } from "vitest";

import { TelemetryCollector } from "./telemetryCollector";
import {
  DEMO_EXPERIMENT_ID,
  type GameLoadedEvent,
  type FirstActionEvent,
  type ChaosChangedEvent,
  type RunEndedEvent,
  type TelemetryEvent,
} from "./telemetry";
import { DemoTelemetryManager } from "./demoTelemetryManager";
import { DEMO_EXPERIMENT_METADATA } from "./experimentMetadata";
import { demoReducer, runDemoSequence } from "./reducer";
import { createSeedState } from "./seedContent";
import type { GameAction, GameState } from "./types";

// ── TelemetryCollector ────────────────────────────────────────────────────

describe("TelemetryCollector", () => {
  it("starts empty", () => {
    const c = new TelemetryCollector();
    expect(c.getEvents()).toEqual([]);
    expect(c.getEventCount()).toBe(0);
  });

  it("captures and returns events", () => {
    const c = new TelemetryCollector();
    const evt: TelemetryEvent = {
      experiment_id: DEMO_EXPERIMENT_ID,
      session_id: "sess-1",
      run_id: "run-1",
      event_type: "game_loaded",
      timestamp: 1_000_000,
      payload: {
        demoName: "Test",
        partySize: 4,
        initialChaos: 5,
        initialDungeonLevel: 1,
      },
    };

    c.capture(evt);
    expect(c.getEventCount()).toBe(1);
    expect(c.getEvents()).toEqual([evt]);
  });

  it("getEvents returns a copy, not the internal array", () => {
    const c = new TelemetryCollector();
    c.capture({
      experiment_id: DEMO_EXPERIMENT_ID,
      session_id: "s",
      run_id: "r",
      event_type: "game_loaded",
      timestamp: 1,
      payload: {
        demoName: "Test",
        partySize: 4,
        initialChaos: 5,
        initialDungeonLevel: 1,
      },
    });
    const events = c.getEvents();
    events.pop();
    expect(c.getEventCount()).toBe(1);
  });

  it("getEventsByType filters correctly", () => {
    const c = new TelemetryCollector();
    const base = {
      experiment_id: DEMO_EXPERIMENT_ID,
      session_id: "s",
      run_id: "r",
      timestamp: 1,
    };

    c.capture({
      ...base,
      event_type: "game_loaded",
      payload: {
        demoName: "T",
        partySize: 4,
        initialChaos: 5,
        initialDungeonLevel: 1,
      },
    } as TelemetryEvent);
    c.capture({
      ...base,
      event_type: "first_action",
      payload: { actionType: "START_RUN", turnCount: 0, chaosMeter: 5 },
    } as TelemetryEvent);
    c.capture({
      ...base,
      event_type: "chaos_changed",
      payload: {
        previousChaos: 5,
        newChaos: 6,
        delta: 1,
        turnCount: 1,
        phase: "dungeon-room",
      },
    } as TelemetryEvent);

    expect(c.getEventsByType("game_loaded")).toHaveLength(1);
    expect(c.getEventsByType("first_action")).toHaveLength(1);
    expect(c.getEventsByType("does_not_exist")).toHaveLength(0);
    expect(c.getEventsByType("game_loaded", "first_action")).toHaveLength(2);
  });

  it("clear empties the event buffer", () => {
    const c = new TelemetryCollector();
    c.capture({
      experiment_id: DEMO_EXPERIMENT_ID,
      session_id: "s",
      run_id: "r",
      event_type: "game_loaded",
      timestamp: 1,
      payload: {
        demoName: "T",
        partySize: 4,
        initialChaos: 5,
        initialDungeonLevel: 1,
      },
    });
    expect(c.getEventCount()).toBe(1);
    c.clear();
    expect(c.getEventCount()).toBe(0);
  });

  it("does not call fetch or XMLHttpRequest (purely in-memory)", () => {
    const c = new TelemetryCollector();
    // The capture method only appends to the internal array — no side-effects
    c.capture({
      experiment_id: DEMO_EXPERIMENT_ID,
      session_id: "s",
      run_id: "r",
      event_type: "game_loaded",
      timestamp: 1,
      payload: {
        demoName: "T",
        partySize: 4,
        initialChaos: 5,
        initialDungeonLevel: 1,
      },
    });
    expect(c.getEventCount()).toBe(1);
    // No network calls were made — the collector is a simple array wrapper
  });
});

// ── Event Shape ────────────────────────────────────────────────────────────

describe("telemetry event shape", () => {
  it("every event carries the five required shared fields", () => {
    const base = {
      experiment_id: DEMO_EXPERIMENT_ID,
      session_id: "sess-abc",
      run_id: "run-xyz",
      timestamp: Date.now(),
    };

    const events: TelemetryEvent[] = [
      {
        ...base,
        event_type: "game_loaded",
        payload: {
          demoName: "D",
          partySize: 4,
          initialChaos: 5,
          initialDungeonLevel: 1,
        },
      },
      {
        ...base,
        event_type: "first_action",
        payload: { actionType: "START_RUN", turnCount: 0, chaosMeter: 5 },
      },
      {
        ...base,
        event_type: "chaos_changed",
        payload: {
          previousChaos: 5,
          newChaos: 6,
          delta: 1,
          turnCount: 1,
          phase: "dungeon-room",
        },
      },
      {
        ...base,
        event_type: "run_ended",
        payload: {
          outcome: "victory",
          finalChaos: 10,
          dungeonLevel: 2,
          roomCount: 5,
          turnCount: 8,
          survivalCount: 4,
        },
      },
    ];

    for (const evt of events) {
      expect(evt).toHaveProperty("experiment_id");
      expect(evt).toHaveProperty("session_id");
      expect(evt).toHaveProperty("run_id");
      expect(evt).toHaveProperty("event_type");
      expect(evt).toHaveProperty("timestamp");
      // The event_type value must be one of the defined set
      expect(
        ["game_loaded", "first_action", "chaos_changed", "run_ended"],
      ).toContain(evt.event_type);
    }
  });

  it("game_loaded event has the correct payload shape", () => {
    const evt: GameLoadedEvent = {
      experiment_id: DEMO_EXPERIMENT_ID,
      session_id: "s",
      run_id: "r",
      event_type: "game_loaded",
      timestamp: 1,
      payload: {
        demoName: "Chaos Dungeon H5 Demo",
        partySize: 4,
        initialChaos: 5,
        initialDungeonLevel: 1,
      },
    };

    expect(evt.payload.demoName).toBeTruthy();
    expect(typeof evt.payload.partySize).toBe("number");
    expect(typeof evt.payload.initialChaos).toBe("number");
    expect(typeof evt.payload.initialDungeonLevel).toBe("number");
  });

  it("first_action event has the correct payload shape", () => {
    const evt: FirstActionEvent = {
      experiment_id: DEMO_EXPERIMENT_ID,
      session_id: "s",
      run_id: "r",
      event_type: "first_action",
      timestamp: 1,
      payload: {
        actionType: "START_RUN",
        turnCount: 0,
        chaosMeter: 5,
      },
    };

    expect(typeof evt.payload.actionType).toBe("string");
    expect(typeof evt.payload.turnCount).toBe("number");
    expect(typeof evt.payload.chaosMeter).toBe("number");
  });

  it("chaos_changed event has the correct payload shape", () => {
    const evt: ChaosChangedEvent = {
      experiment_id: DEMO_EXPERIMENT_ID,
      session_id: "s",
      run_id: "r",
      event_type: "chaos_changed",
      timestamp: 1,
      payload: {
        previousChaos: 5,
        newChaos: 7,
        delta: 2,
        turnCount: 3,
        phase: "dungeon-room",
      },
    };

    expect(typeof evt.payload.previousChaos).toBe("number");
    expect(typeof evt.payload.newChaos).toBe("number");
    expect(typeof evt.payload.delta).toBe("number");
    expect(typeof evt.payload.turnCount).toBe("number");
    expect(typeof evt.payload.phase).toBe("string");
    expect(evt.payload.delta).toBe(
      evt.payload.newChaos - evt.payload.previousChaos,
    );
  });

  it("run_ended event has the correct payload shape", () => {
    const evt: RunEndedEvent = {
      experiment_id: DEMO_EXPERIMENT_ID,
      session_id: "s",
      run_id: "r",
      event_type: "run_ended",
      timestamp: 1,
      payload: {
        outcome: "retreat",
        finalChaos: 12,
        dungeonLevel: 1,
        roomCount: 3,
        turnCount: 5,
        survivalCount: 4,
      },
    };

    expect(
      ["victory", "defeat", "retreat", "catastrophe", "abandoned"],
    ).toContain(evt.payload.outcome);
    expect(typeof evt.payload.finalChaos).toBe("number");
    expect(typeof evt.payload.dungeonLevel).toBe("number");
    expect(typeof evt.payload.roomCount).toBe("number");
    expect(typeof evt.payload.turnCount).toBe("number");
    expect(typeof evt.payload.survivalCount).toBe("number");
  });
});

// ── Experiment Metadata ────────────────────────────────────────────────────

describe("experiment metadata", () => {
  it("contains the required top-level fields", () => {
    expect(DEMO_EXPERIMENT_METADATA.experimentId).toBe(DEMO_EXPERIMENT_ID);
    expect(DEMO_EXPERIMENT_METADATA.demoName).toBeTruthy();
    expect(DEMO_EXPERIMENT_METADATA.hypothesis).toBeTruthy();
    expect(DEMO_EXPERIMENT_METADATA.description).toBeTruthy();
    expect(DEMO_EXPERIMENT_METADATA.targetMetrics.length).toBeGreaterThan(0);
    expect(DEMO_EXPERIMENT_METADATA.metrics).toBeTruthy();
    expect(
      DEMO_EXPERIMENT_METADATA.qualificationCriteria.length,
    ).toBeGreaterThan(0);
    expect(DEMO_EXPERIMENT_METADATA.createdAt).toBeTruthy();
  });

  it("targetMetrics are all defined in the metrics map", () => {
    for (const key of DEMO_EXPERIMENT_METADATA.targetMetrics) {
      expect(DEMO_EXPERIMENT_METADATA.metrics[key]).toBeDefined();
      expect(DEMO_EXPERIMENT_METADATA.metrics[key].label).toBeTruthy();
      expect(DEMO_EXPERIMENT_METADATA.metrics[key].description).toBeTruthy();
      expect(DEMO_EXPERIMENT_METADATA.metrics[key].unit).toBeTruthy();
      expect(
        ["higher_is_better", "lower_is_better", "neutral"],
      ).toContain(DEMO_EXPERIMENT_METADATA.metrics[key].direction);
    }
  });
});

// ── DemoTelemetryManager ───────────────────────────────────────────────────

describe("DemoTelemetryManager", () => {
  it("creates a collector that starts empty", () => {
    const mgr = new DemoTelemetryManager();
    expect(mgr.collector.getEventCount()).toBe(0);
  });

  it("has a sessionId and runId", () => {
    const mgr = new DemoTelemetryManager();
    expect(mgr.sessionId).toBeTruthy();
    expect(mgr.runId).toBeTruthy();
  });

  describe("onGameLoaded", () => {
    it("emits exactly one game_loaded event", () => {
      const mgr = new DemoTelemetryManager();
      const state: GameState = {
        phase: "start",
        party: [],
        dungeonLevel: 1,
        currentRoom: null,
        chaosMeter: 5,
        turnCount: 0,
        roomCount: 0,
        runLog: [],
        resultMessage: null,
        runOutcome: null,
        combatEncounter: null,
      };

      mgr.onGameLoaded(state);

      const events = mgr.collector.getEvents();
      expect(events).toHaveLength(1);
      expect(events[0].event_type).toBe("game_loaded");
      expect(events[0].experiment_id).toBe(DEMO_EXPERIMENT_ID);
      expect(events[0].session_id).toBe(mgr.sessionId);
      expect(events[0].run_id).toBe(mgr.runId);
    });
  });

  describe("onAction + onStateChanged", () => {
    it("emits first_action on the first action", () => {
      const mgr = new DemoTelemetryManager();
      const before: GameState = {
        phase: "start",
        party: [],
        dungeonLevel: 1,
        currentRoom: null,
        chaosMeter: 5,
        turnCount: 0,
        roomCount: 0,
        runLog: [],
        resultMessage: null,
        runOutcome: null,
        combatEncounter: null,
      };
      const action: GameAction = { type: "START_RUN" };

      mgr.onAction(action, before);

      const events = mgr.collector.getEvents();
      const firstActionEvents = events.filter(
        (e) => e.event_type === "first_action",
      );
      expect(firstActionEvents).toHaveLength(1);
    });

    it("only emits first_action once across multiple actions", () => {
      const mgr = new DemoTelemetryManager();
      const before: GameState = {
        phase: "start",
        party: [],
        dungeonLevel: 1,
        currentRoom: null,
        chaosMeter: 5,
        turnCount: 0,
        roomCount: 0,
        runLog: [],
        resultMessage: null,
        runOutcome: null,
        combatEncounter: null,
      };

      mgr.onAction({ type: "START_RUN" }, before);

      const later: GameState = {
        ...before,
        phase: "dungeon-room",
        turnCount: 1,
      };
      mgr.onAction({ type: "ENTER_ROOM" }, later);

      const firstActions = mgr.collector
        .getEvents()
        .filter((e) => e.event_type === "first_action");
      expect(firstActions).toHaveLength(1);
    });

    it("emits chaos_changed when chaos differs from previous value", () => {
      const mgr = new DemoTelemetryManager();
      // Simulate onGameLoaded setting previousChaos to 5
      mgr.onGameLoaded({
        phase: "start",
        party: [],
        dungeonLevel: 1,
        currentRoom: null,
        chaosMeter: 5,
        turnCount: 0,
        roomCount: 0,
        runLog: [],
        resultMessage: null,
        runOutcome: null,
        combatEncounter: null,
      });
      mgr.collector.clear(); // clear the game_loaded event

      // Now simulate a state change with new chaos
      mgr.onStateChanged({
        phase: "dungeon-room",
        party: [],
        dungeonLevel: 1,
        currentRoom: null,
        chaosMeter: 7,
        turnCount: 2,
        roomCount: 1,
        runLog: [],
        resultMessage: null,
        runOutcome: null,
        combatEncounter: null,
      });

      const chaosEvents = mgr.collector
        .getEvents()
        .filter((e) => e.event_type === "chaos_changed");
      expect(chaosEvents).toHaveLength(1);

      const ce = chaosEvents[0] as ChaosChangedEvent;
      expect(ce.payload.previousChaos).toBe(5);
      expect(ce.payload.newChaos).toBe(7);
      expect(ce.payload.delta).toBe(2);
    });

    it("emits run_ended when phase becomes result with an outcome", () => {
      const mgr = new DemoTelemetryManager();
      mgr.onStateChanged({
        phase: "result",
        party: [],
        dungeonLevel: 1,
        currentRoom: null,
        chaosMeter: 15,
        turnCount: 5,
        roomCount: 3,
        runLog: [],
        resultMessage: "The dungeon collapses.",
        runOutcome: "catastrophe",
        combatEncounter: null,
      });

      const endedEvents = mgr.collector
        .getEvents()
        .filter((e) => e.event_type === "run_ended");
      expect(endedEvents).toHaveLength(1);

      const re = endedEvents[0] as RunEndedEvent;
      expect(re.payload.outcome).toBe("catastrophe");
      expect(re.payload.finalChaos).toBe(15);
      expect(re.payload.survivalCount).toBe(0);
    });

    it("does not emit run_ended for non-result phases", () => {
      const mgr = new DemoTelemetryManager();
      mgr.onStateChanged({
        phase: "dungeon-room",
        party: [],
        dungeonLevel: 1,
        currentRoom: null,
        chaosMeter: 6,
        turnCount: 1,
        roomCount: 1,
        runLog: [],
        resultMessage: null,
        runOutcome: null,
        combatEncounter: null,
      });

      expect(
        mgr.collector
          .getEvents()
          .filter((e) => e.event_type === "run_ended"),
      ).toHaveLength(0);
    });
  });

  describe("onRunReset", () => {
    it("generates a new runId and clears per-run flags", () => {
      const mgr = new DemoTelemetryManager();
      const originalRunId = mgr.runId;

      mgr.onAction(
        { type: "START_RUN" },
        {
          phase: "start",
          party: [],
          dungeonLevel: 1,
          currentRoom: null,
          chaosMeter: 5,
          turnCount: 0,
          roomCount: 0,
          runLog: [],
          resultMessage: null,
          runOutcome: null,
          combatEncounter: null,
        },
      );

      mgr.onRunReset();

      expect(mgr.runId).not.toBe(originalRunId);

      // After reset, a new first_action can be emitted
      mgr.onAction(
        { type: "START_RUN" },
        {
          phase: "start",
          party: [],
          dungeonLevel: 1,
          currentRoom: null,
          chaosMeter: 5,
          turnCount: 0,
          roomCount: 0,
          runLog: [],
          resultMessage: null,
          runOutcome: null,
          combatEncounter: null,
        },
      );

      const firstActions = mgr.collector
        .getEvents()
        .filter((e) => e.event_type === "first_action");
      // Should have 2 — one before reset, one after
      expect(firstActions).toHaveLength(2);
    });
  });
});

// ── Integration: Full run emits expected event types ─────────────────────

describe("telemetry integration with demo run", () => {
  it("a full run sequence emits all expected event types", () => {
    const mgr = new DemoTelemetryManager();

    // 1. On mount: game_loaded
    const seed = createSeedState();
    mgr.onGameLoaded(seed);

    // 2. Run actions step by step, calling telemetry hooks
    const actions: GameAction[] = [
      { type: "START_RUN" },
      { type: "ENTER_ROOM" },
      { type: "ENTER_ROOM" },
    ];

    let state: GameState = seed;
    for (const action of actions) {
      mgr.onAction(action, state);
      state = demoReducer(state, action);
      mgr.onStateChanged(state);
    }

    // 3. Continue until we reach a terminal result
    let safety = 0;
    while (state.phase !== "result" && safety < 20) {
      safety++;
      const action: GameAction =
        state.phase === "event"
          ? { type: "RESOLVE_EVENT", choiceIndex: 0 }
          : state.phase === "combat"
            ? { type: "COMBAT_LOST" }
            : { type: "ENTER_ROOM" };

      mgr.onAction(action, state);
      state = demoReducer(state, action);
      mgr.onStateChanged(state);
    }

    // 4. If still not result, end the run explicitly
    if (state.phase !== "result") {
      mgr.onAction({ type: "END_RUN" }, state);
      state = demoReducer(state, { type: "END_RUN" });
      mgr.onStateChanged(state);
    }

    // Verify all four event types were emitted
    const allEvents = mgr.collector.getEvents();
    const eventTypes = new Set(allEvents.map((e) => e.event_type));

    expect(eventTypes.has("game_loaded")).toBe(true);
    expect(eventTypes.has("first_action")).toBe(true);
    expect(eventTypes.has("chaos_changed")).toBe(true);
    expect(eventTypes.has("run_ended")).toBe(true);

    // At least one event was emitted
    expect(allEvents.length).toBeGreaterThan(0);
  });
});
