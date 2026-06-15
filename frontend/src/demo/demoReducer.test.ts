import { describe, expect, it } from "vitest";

import type { GameState } from "./types";
import { demoReducer, runDemoSequence } from "./reducer";
import { createSeedState } from "./seedContent";

describe("demoReducer", () => {
  // ── Seed State ──────────────────────────────────────────────

  it("creates seed state with the correct initial phase", () => {
    const state = createSeedState();
    expect(state.phase).toBe("start");
    expect(state.chaosMeter).toBe(5);
    expect(state.party).toHaveLength(4);
    expect(state.runLog).toContain("The Darkest Dungeon awaits…");
  });

  // ── Single-action transitions ───────────────────────────────

  it("START_RUN transitions from start to dungeon-room", () => {
    const state = createSeedState();
    const next = demoReducer(state, { type: "START_RUN" });
    expect(next.phase).toBe("dungeon-room");
    expect(next.currentRoom).not.toBeNull();
    expect(next.turnCount).toBe(1);
    expect(next.runLog.length).toBeGreaterThan(state.runLog.length);
  });

  it("START_RUN is a no-op when phase is not start", () => {
    const state = { ...createSeedState(), phase: "dungeon-room" as const };
    const next = demoReducer(state, { type: "START_RUN" });
    expect(next).toBe(state);
  });

  it("ENTER_ROOM from dungeon-room changes to event when room has an event", () => {
    const state = demoReducer(createSeedState(), { type: "START_RUN" });
    // After START_RUN we're in dungeon-room; ENTER_ROOM will look up the next room
    // which might have an event or enemy. We just verify it transitions.
    const next = demoReducer(state, { type: "ENTER_ROOM" });
    expect(["event", "combat"]).toContain(next.phase);
    expect(next.turnCount).toBe(2);
  });

  it("COMBAT_WIN transitions combat to dungeon-room with healing", () => {
    let state = demoReducer(createSeedState(), { type: "START_RUN" });
    // Advance until we reach combat, handling events along the way
    while (state.phase !== "combat") {
      if (state.phase === "event") {
        state = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: 0 });
      } else {
        state = demoReducer(state, { type: "ENTER_ROOM" });
      }
    }

    const after = demoReducer(state, { type: "COMBAT_WIN" });
    expect(after.phase).toBe("dungeon-room");
    // Party should have healed
    for (const member of after.party) {
      expect(member.hp).toBeGreaterThanOrEqual(
        state.party.find((m) => m.id === member.id)!.hp,
      );
    }
  });

  it("COMBAT_FLEE transitions combat to dungeon-room with stress gain", () => {
    let state = demoReducer(createSeedState(), { type: "START_RUN" });
    while (state.phase !== "combat") {
      if (state.phase === "event") {
        state = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: 0 });
      } else {
        state = demoReducer(state, { type: "ENTER_ROOM" });
      }
    }

    const after = demoReducer(state, { type: "COMBAT_FLEE" });
    expect(after.phase).toBe("dungeon-room");
    for (const member of after.party) {
      expect(member.stress).toBeGreaterThanOrEqual(
        state.party.find((m) => m.id === member.id)!.stress,
      );
    }
  });

  it("END_RUN transitions to result phase", () => {
    const state = demoReducer(createSeedState(), { type: "START_RUN" });
    const result = demoReducer(state, { type: "END_RUN" });
    expect(result.phase).toBe("result");
    expect(result.resultMessage).not.toBeNull();
  });

  // ── Deterministic sequence (≥3 actions, AC-005) ────────────

  it("produces identical results when re-running the same action sequence", () => {
    const actions = [
      { type: "START_RUN" as const },
      { type: "ENTER_ROOM" as const },
      { type: "ENTER_ROOM" as const },
    ];

    const resultA = runDemoSequence(actions);
    const resultB = runDemoSequence(actions);

    // Full structural equality — deterministic reducer
    expect(resultA).toEqual(resultB);
    // Run log length and phase are identical
    expect(resultA.runLog.length).toBe(resultB.runLog.length);
    expect(resultA.phase).toBe(resultB.phase);
    expect(resultA.chaosMeter).toBe(resultB.chaosMeter);
  });

  // ── Full run loop ─────────────────────────────────────────────

  /**
   * Dispatch the correct action for whatever phase we're in.
   * This simulates a player making progress through the demo.
   */
  function dispatchForward(s: GameState): GameState {
    switch (s.phase) {
      case "start":
        return demoReducer(s, { type: "START_RUN" });
      case "dungeon-room":
        return demoReducer(s, { type: "ENTER_ROOM" });
      case "event":
        return demoReducer(s, { type: "RESOLVE_EVENT", choiceIndex: 0 });
      case "combat":
        return demoReducer(s, { type: "COMBAT_WIN" });
      case "result":
        return s;
    }
  }

  it("supports a full run loop: start → explore → combat → result", () => {
    let state = createSeedState();

    // Walk through 10 steps of game loop, handling whatever phase we land on
    for (let i = 0; i < 10; i++) {
      if (state.phase === "result") break;
      state = dispatchForward(state);
    }

    // After 10 steps we should be in a terminal or continuing phase
    expect(["dungeon-room", "event", "combat", "result"]).toContain(state.phase);
    expect(state.turnCount).toBeGreaterThan(0);

    // Force end the run
    if (state.phase !== "result") {
      state = demoReducer(state, { type: "END_RUN" });
    }
    expect(state.phase).toBe("result");
    expect(state.resultMessage).not.toBeNull();
  });

  // ── Chaos meter behaviour ────────────────────────────────────

  it("increases chaos when entering rooms", () => {
    const seed = createSeedState();
    const started = demoReducer(seed, { type: "START_RUN" });
    expect(started.chaosMeter).toBe(seed.chaosMeter); // START_RUN does not change chaos

    const entered = demoReducer(started, { type: "ENTER_ROOM" });
    expect(entered.chaosMeter).toBeGreaterThan(started.chaosMeter);
  });

  it("can trigger a result when chaos exceeds threshold via event choice", () => {
    // Start with high chaos and pick a large-positive-delta choice
    let state: ReturnType<typeof demoReducer> = {
      ...createSeedState(),
      chaosMeter: 14,
      phase: "event",
      currentRoom: {
        id: "test-room",
        name: "Test Chamber",
        description: "A test room.",
        enemyGroup: null,
        event: {
          id: "test-evt",
          title: "Critical Test",
          description: "A test event.",
          choices: [
            {
              label: "Trigger catastrophe",
              chaosDelta: 3,
              outcome: "Chaos explodes!",
            },
          ],
        },
      },
    };

    state = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: 0 });
    expect(state.phase).toBe("result");
    expect(state.chaosMeter).toBeGreaterThanOrEqual(15);
  });

  // ── Floor advancement ────────────────────────────────────────

  it("advances floor level on ADVANCE_FLOOR action", () => {
    const state = demoReducer(createSeedState(), { type: "ADVANCE_FLOOR" });
    expect(state.dungeonLevel).toBe(2);
    expect(state.chaosMeter).toBe(6);
  });
});
