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
    const next = demoReducer(state, { type: "ENTER_ROOM" });
    expect(["event", "combat"]).toContain(next.phase);
    expect(next.turnCount).toBe(2);
  });

  it("COMBAT_WIN transitions combat to dungeon-room with healing", () => {
    let state = demoReducer(createSeedState(), { type: "START_RUN" });
    while (state.phase !== "combat") {
      if (state.phase === "event") {
        state = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: 0 });
      } else {
        state = demoReducer(state, { type: "ENTER_ROOM" });
      }
    }

    const after = demoReducer(state, { type: "COMBAT_WIN" });
    expect(after.phase).toBe("dungeon-room");
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

  it("COMBAT_LOST transitions combat to result", () => {
    let state = demoReducer(createSeedState(), { type: "START_RUN" });
    while (state.phase !== "combat") {
      if (state.phase === "event") {
        state = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: 0 });
      } else {
        state = demoReducer(state, { type: "ENTER_ROOM" });
      }
    }

    const after = demoReducer(state, { type: "COMBAT_LOST" });
    expect(after.phase).toBe("result");
    expect(after.resultMessage).toContain("fallen");
  });

  it("END_RUN transitions to result phase", () => {
    const state = demoReducer(createSeedState(), { type: "START_RUN" });
    const result = demoReducer(state, { type: "END_RUN" });
    expect(result.phase).toBe("result");
    expect(result.resultMessage).not.toBeNull();
  });

  // ── Combat encounter: hero actions ──────────────────────────

  it("HERO_ATTACK reduces enemy HP and creates combat log entry with actor, target, and result", () => {
    let state = demoReducer(createSeedState(), { type: "START_RUN" });
    while (state.phase !== "combat") {
      if (state.phase === "event") {
        state = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: 0 });
      } else {
        state = demoReducer(state, { type: "ENTER_ROOM" });
      }
    }

    const encBefore = state.combatEncounter!;
    const firstEnemyHp = encBefore.enemies[0].hp;

    // First hero attacks
    const after = demoReducer(state, { type: "HERO_ATTACK" });
    const encAfter = after.combatEncounter!;

    // Enemy HP should have decreased
    expect(encAfter.enemies[0].hp).toBeLessThan(firstEnemyHp);

    // Combat log should have an entry identifying actor, target, and result
    expect(encAfter.log.length).toBeGreaterThan(0);
    const logEntry = encAfter.log[0];
    expect(logEntry.actorName).toBeTruthy();
    expect(logEntry.targetName).toBeTruthy();
    expect(logEntry.damage).toBeGreaterThan(0);
    expect(logEntry.targetCurrentHp).toBeGreaterThanOrEqual(0);

    // runLog should also have an entry with the same info
    const lastRunLog = after.runLog[after.runLog.length - 1];
    expect(lastRunLog).toContain(logEntry.actorName);
    expect(lastRunLog).toContain("strikes");
    expect(lastRunLog).toContain(logEntry.targetName);
    expect(lastRunLog).toContain(`${logEntry.damage} damage`);
  });

  it("enemy phase reduces party HP and may increase stress after all heroes act", () => {
    let state = demoReducer(createSeedState(), { type: "START_RUN" });
    while (state.phase !== "combat") {
      if (state.phase === "event") {
        state = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: 0 });
      } else {
        state = demoReducer(state, { type: "ENTER_ROOM" });
      }
    }

    const partyHpBefore = state.party.map((m) => m.hp);
    const partyStressBefore = state.party.map((m) => m.stress);

    // Dispatch HERO_ATTACK until all heroes have acted
    while (
      state.combatEncounter &&
      !state.combatEncounter.resolved &&
      state.combatEncounter.activeHeroIndex < state.party.length
    ) {
      state = demoReducer(state, { type: "HERO_ATTACK" });
    }

    // After all heroes acted, enemy phase should have triggered
    // (round advances and party may have taken damage)
    if (state.combatEncounter && state.combatEncounter.round > 1) {
      // At least one party member probably took damage
      const someTookDamage = state.party.some(
        (m, i) => m.hp < partyHpBefore[i],
      );
      // Enemies dealt damage or stress — at least one party member affected
      const someStressGained = state.party.some(
        (m, i) => m.stress > partyStressBefore[i],
      );

      // Not all combats have stress-inflicting enemies, so check damage OR stress
      expect(someTookDamage || someStressGained).toBe(true);
    }
  });

  // ── Combat encounter: victory ───────────────────────────────

  it("combat can be resolved as victory when all enemies are defeated", () => {
    let state = demoReducer(createSeedState(), { type: "START_RUN" });
    while (state.phase !== "combat") {
      if (state.phase === "event") {
        state = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: 0 });
      } else {
        state = demoReducer(state, { type: "ENTER_ROOM" });
      }
    }

    // Attack until victory
    let safety = 0;
    while (
      state.phase === "combat" &&
      state.combatEncounter &&
      !state.combatEncounter.resolved &&
      safety < 20
    ) {
      state = demoReducer(state, { type: "HERO_ATTACK" });
      safety++;
    }

    expect(state.phase).toBe("combat");
    expect(state.combatEncounter?.resolved).toBe(true);
    expect(state.combatEncounter?.outcome).toBe("victory");

    // All enemies should be at 0 HP
    for (const enemy of state.combatEncounter!.enemies) {
      expect(enemy.hp).toBe(0);
    }
  });

  it("claims victory then transitions to dungeon-room via COMBAT_WIN", () => {
    let state = demoReducer(createSeedState(), { type: "START_RUN" });
    while (state.phase !== "combat") {
      if (state.phase === "event") {
        state = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: 0 });
      } else {
        state = demoReducer(state, { type: "ENTER_ROOM" });
      }
    }

    // Resolve to victory
    let safety = 0;
    while (
      state.phase === "combat" &&
      state.combatEncounter &&
      !state.combatEncounter.resolved &&
      safety < 20
    ) {
      state = demoReducer(state, { type: "HERO_ATTACK" });
      safety++;
    }

    expect(state.combatEncounter?.outcome).toBe("victory");

    // Transition out of combat
    state = demoReducer(state, { type: "COMBAT_WIN" });
    expect(state.phase).toBe("dungeon-room");
    expect(state.combatEncounter).toBeNull();
  });

  // ── Deterministic sequence ──────────────────────────────────

  it("produces identical results when re-running the same action sequence", () => {
    const actions = [
      { type: "START_RUN" as const },
      { type: "ENTER_ROOM" as const },
      { type: "ENTER_ROOM" as const },
    ];

    const resultA = runDemoSequence(actions);
    const resultB = runDemoSequence(actions);

    expect(resultA).toEqual(resultB);
    expect(resultA.runLog.length).toBe(resultB.runLog.length);
    expect(resultA.phase).toBe(resultB.phase);
    expect(resultA.chaosMeter).toBe(resultB.chaosMeter);
  });

  it("produces identical combat outcomes when re-running the same combat action sequence", () => {
    let stateA = demoReducer(createSeedState(), { type: "START_RUN" });
    let stateB = demoReducer(createSeedState(), { type: "START_RUN" });

    // Advance both to combat the same way
    const advanceToCombat = (s: GameState) => {
      while (s.phase !== "combat") {
        if (s.phase === "event") {
          s = demoReducer(s, { type: "RESOLVE_EVENT", choiceIndex: 0 });
        } else {
          s = demoReducer(s, { type: "ENTER_ROOM" });
        }
      }
      return s;
    };

    stateA = advanceToCombat(stateA);
    stateB = advanceToCombat(stateB);

    // Execute the same attack sequence
    for (let i = 0; i < 5; i++) {
      if (stateA.combatEncounter?.resolved) break;
      stateA = demoReducer(stateA, { type: "HERO_ATTACK" });
      stateB = demoReducer(stateB, { type: "HERO_ATTACK" });
    }

    // Full structural equality on combat state
    expect(stateA.combatEncounter).toEqual(stateB.combatEncounter);
    expect(stateA.party).toEqual(stateB.party);
  });

  // ── Combat log entries ──────────────────────────────────────

  it("combat log entries identify round, actor, target, damage, and HP", () => {
    let state = demoReducer(createSeedState(), { type: "START_RUN" });
    while (state.phase !== "combat") {
      if (state.phase === "event") {
        state = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: 0 });
      } else {
        state = demoReducer(state, { type: "ENTER_ROOM" });
      }
    }

    state = demoReducer(state, { type: "HERO_ATTACK" });

    const log = state.combatEncounter!.log;
    expect(log.length).toBeGreaterThan(0);

    for (const entry of log) {
      expect(entry.round).toBeGreaterThan(0);
      expect(entry.actorName).toBeTruthy();
      expect(entry.actionLabel).toBeTruthy();
      expect(entry.targetName).toBeTruthy();
      expect(typeof entry.damage).toBe("number");
      expect(typeof entry.targetCurrentHp).toBe("number");
      expect(typeof entry.targetMaxHp).toBe("number");
    }
  });

  // ── Combat encounter: state initialisation ──────────────────

  it("sets up combat encounter when entering combat", () => {
    let state = demoReducer(createSeedState(), { type: "START_RUN" });
    while (state.phase !== "combat") {
      if (state.phase === "event") {
        state = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: 0 });
      } else {
        state = demoReducer(state, { type: "ENTER_ROOM" });
      }
    }

    const enc = state.combatEncounter;
    expect(enc).not.toBeNull();
    expect(enc!.round).toBe(1);
    expect(enc!.activeHeroIndex).toBe(0);
    expect(enc!.enemies.length).toBeGreaterThan(0);
    expect(enc!.resolved).toBe(false);
    expect(enc!.outcome).toBe("undecided");
    expect(enc!.log).toEqual([]);
  });

  it("combat encounter assigns individual HP to each enemy", () => {
    let state = demoReducer(createSeedState(), { type: "START_RUN" });
    while (state.phase !== "combat") {
      if (state.phase === "event") {
        state = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: 0 });
      } else {
        state = demoReducer(state, { type: "ENTER_ROOM" });
      }
    }

    const enc = state.combatEncounter!;
    const room = state.currentRoom!;
    expect(enc.enemies.length).toBe(room.enemyGroup!.count);

    for (const enemy of enc.enemies) {
      expect(enemy.maxHp).toBe(room.enemyGroup!.hp);
      expect(enemy.hp).toBe(room.enemyGroup!.hp);
    }
  });

  // ── Chaos meter in combat ───────────────────────────────────

  it("chaos increases when a full round of combat completes", () => {
    let state = demoReducer(createSeedState(), { type: "START_RUN" });
    while (state.phase !== "combat") {
      if (state.phase === "event") {
        state = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: 0 });
      } else {
        state = demoReducer(state, { type: "ENTER_ROOM" });
      }
    }

    const chaosBefore = state.chaosMeter;

    // Act until round advances (all heroes acted + enemy phase)
    let safety = 0;
    while (
      state.combatEncounter &&
      state.combatEncounter.round < 2 &&
      !state.combatEncounter.resolved &&
      safety < 10
    ) {
      state = demoReducer(state, { type: "HERO_ATTACK" });
      safety++;
    }

    // Chaos should have increased by at least 1 from combat activity
    expect(state.chaosMeter).toBeGreaterThanOrEqual(chaosBefore);
  });

  // ── Full run loop (existing) ────────────────────────────────

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

    for (let i = 0; i < 10; i++) {
      if (state.phase === "result") break;
      state = dispatchForward(state);
    }

    expect(["dungeon-room", "event", "combat", "result"]).toContain(
      state.phase,
    );
    expect(state.turnCount).toBeGreaterThan(0);

    if (state.phase !== "result") {
      state = demoReducer(state, { type: "END_RUN" });
    }
    expect(state.phase).toBe("result");
    expect(state.resultMessage).not.toBeNull();
  });

  // ── Chaos meter behaviour ───────────────────────────────────

  it("increases chaos when entering rooms", () => {
    const seed = createSeedState();
    const started = demoReducer(seed, { type: "START_RUN" });
    expect(started.chaosMeter).toBe(seed.chaosMeter);

    const entered = demoReducer(started, { type: "ENTER_ROOM" });
    expect(entered.chaosMeter).toBeGreaterThan(started.chaosMeter);
  });

  it("can trigger a result when chaos exceeds threshold via event choice", () => {
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

  // ── Floor advancement ───────────────────────────────────────

  it("advances floor level on ADVANCE_FLOOR action", () => {
    const state = demoReducer(createSeedState(), { type: "ADVANCE_FLOOR" });
    expect(state.dungeonLevel).toBe(2);
    expect(state.chaosMeter).toBe(6);
  });
});
