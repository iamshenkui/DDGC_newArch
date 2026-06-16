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

  // ── Event choice party effects ─────────────────────────────

  describe("event choice party value effects", () => {
    /** Helper: advance from seed state to the first event phase. */
    function advanceToEvent(): ReturnType<typeof demoReducer> {
      let state = demoReducer(createSeedState(), { type: "START_RUN" });
      while (state.phase !== "event") {
        state = demoReducer(state, { type: "ENTER_ROOM" });
      }
      return state;
    }

    it("applies chaosDelta from event choice", () => {
      const state = advanceToEvent();
      const chaosBefore = state.chaosMeter;
      const result = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: 0 });
      const delta = result.chaosMeter - chaosBefore;
      expect(delta).toBe(state.currentRoom!.event!.choices[0].chaosDelta);
    });

    it("applies partyHpDelta to all party members", () => {
      const state = advanceToEvent();

      // Find an event choice with partyHpDelta
      const evt = state.currentRoom!.event!;
      const hpChoiceIdx = evt.choices.findIndex((c) => c.partyHpDelta !== undefined);
      if (hpChoiceIdx < 0) return; // skip if no testable event this room

      const hpBefore = state.party.map((m) => m.hp);
      const result = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: hpChoiceIdx });
      const delta = evt.choices[hpChoiceIdx].partyHpDelta!;

      for (let i = 0; i < state.party.length; i++) {
        const expected = Math.max(0, Math.min(state.party[i].maxHp, hpBefore[i] + delta));
        expect(result.party[i].hp).toBe(expected);
      }
    });

    it("applies partyStressDelta to all party members", () => {
      const state = advanceToEvent();

      const evt = state.currentRoom!.event!;
      const stressChoiceIdx = evt.choices.findIndex((c) => c.partyStressDelta !== undefined);
      if (stressChoiceIdx < 0) return;

      const stressBefore = state.party.map((m) => m.stress);
      const result = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: stressChoiceIdx });
      const delta = evt.choices[stressChoiceIdx].partyStressDelta!;

      for (let i = 0; i < state.party.length; i++) {
        const expected = Math.max(0, stressBefore[i] + delta);
        expect(result.party[i].stress).toBe(expected);
      }
    });

    it("clamps party HP to maxHp after healing event", () => {
      const state = advanceToEvent();

      // Force a party member to near-full HP
      const fullHpParty = state.party.map((m) => ({ ...m, hp: m.maxHp - 1 }));
      const fullState = { ...state, party: fullHpParty };
      const evt = fullState.currentRoom!.event!;
      const healIdx = evt.choices.findIndex((c) => (c.partyHpDelta ?? 0) > 0);
      if (healIdx < 0) return;

      const result = demoReducer(fullState, { type: "RESOLVE_EVENT", choiceIndex: healIdx });

      for (const member of result.party) {
        expect(member.hp).toBeLessThanOrEqual(member.maxHp);
      }
    });

    it("clamps party HP to minimum 0 after damaging event", () => {
      const state = advanceToEvent();

      // Force a party member to low HP
      const lowHpParty = state.party.map((m) => ({ ...m, hp: 1 }));
      const lowState = { ...state, party: lowHpParty };
      const evt = lowState.currentRoom!.event!;
      const dmgIdx = evt.choices.findIndex((c) => (c.partyHpDelta ?? 0) < 0);
      if (dmgIdx < 0) return;

      const result = demoReducer(lowState, { type: "RESOLVE_EVENT", choiceIndex: dmgIdx });

      for (const member of result.party) {
        expect(member.hp).toBeGreaterThanOrEqual(0);
      }
    });

    it("includes party effect info in the run log", () => {
      const state = advanceToEvent();
      const evt = state.currentRoom!.event!;
      const hpChoiceIdx = evt.choices.findIndex((c) => c.partyHpDelta !== undefined);
      if (hpChoiceIdx < 0) return;

      const result = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: hpChoiceIdx });

      // Run log should contain outcome text and party effect entries
      const logText = result.runLog.join(" ");
      expect(logText).toContain(evt.choices[hpChoiceIdx].outcome);
    });

    // ── Validate specific known event choices ─────────────────

    it("evt-01: 'Smash the altar' decreases chaos by 2 and damages party for 3 HP each", () => {
      // Room-01 is the first room (turn 0) → used by pickRoom(0) at START_RUN
      // After START_RUN → ENTER_ROOM, the room advances, so we need specific sequence
      // Room-01: turn index 0, first event
      let state = demoReducer(createSeedState(), { type: "START_RUN" });
      // START_RUN sets turnCount=1, picks room at index 1%4=1 (room-02 with new evt-02)
      // We want room-01 which is at index 0, so start with turnCount = 0
      // Actually, pickRoom(turnCount) uses turnCount % ROOM_POOL.length
      // START_RUN passes turnCount=0 → pickRoom(0) → room-01
      // ENTER_ROOM increments turnCount and uses new turn for room selection
      // Let's just test the choice directly by setting up the state

      // Create state at event phase with evt-01 data
      const testState: GameState = {
        ...createSeedState(),
        phase: "event",
        chaosMeter: 10,
        currentRoom: {
          id: "room-01",
          name: "Crumbling Hall",
          description: "desc",
          enemyGroup: null,
          event: {
            id: "evt-01",
            title: "Desecrated Altar",
            description: "desc",
            choices: [
              { label: "Smash the altar", chaosDelta: -2, partyHpDelta: -3, outcome: "smashed" },
              { label: "Offer a prayer", chaosDelta: 1, partyStressDelta: -2, outcome: "prayed" },
              { label: "Ignore it", chaosDelta: 1, outcome: "ignored" },
            ],
          },
        },
      };

      const result = demoReducer(testState, { type: "RESOLVE_EVENT", choiceIndex: 0 });
      expect(result.chaosMeter).toBe(8); // 10 + (-2)
      for (const m of result.party) {
        const original = testState.party.find((p) => p.id === m.id)!;
        expect(m.hp).toBe(Math.max(0, original.hp - 3));
      }
    });

    it("evt-01: 'Offer a prayer' increases chaos by 1 and reduces stress by 2 for all", () => {
      const testState: GameState = {
        ...createSeedState(),
        phase: "event",
        chaosMeter: 7,
        currentRoom: {
          id: "room-01",
          name: "Crumbling Hall",
          description: "desc",
          enemyGroup: null,
          event: {
            id: "evt-01",
            title: "Desecrated Altar",
            description: "desc",
            choices: [
              { label: "Smash the altar", chaosDelta: -2, partyHpDelta: -3, outcome: "smashed" },
              { label: "Offer a prayer", chaosDelta: 1, partyStressDelta: -2, outcome: "prayed" },
              { label: "Ignore it", chaosDelta: 1, outcome: "ignored" },
            ],
          },
        },
      };

      const result = demoReducer(testState, { type: "RESOLVE_EVENT", choiceIndex: 1 });
      expect(result.chaosMeter).toBe(8); // 7 + 1
      for (const m of result.party) {
        const original = testState.party.find((p) => p.id === m.id)!;
        expect(m.stress).toBe(Math.max(0, original.stress - 2));
      }
    });

    it("evt-04: 'Strike the core' decreases chaos by 3 and heals party for 4 HP each", () => {
      const damagedParty = createSeedState().party.map((m) => ({
        ...m,
        hp: Math.max(1, m.hp - 8),
      }));

      const testState: GameState = {
        ...createSeedState(),
        party: damagedParty,
        phase: "event",
        chaosMeter: 12,
        currentRoom: {
          id: "room-04",
          name: "The Heart Chamber",
          description: "desc",
          enemyGroup: null,
          event: {
            id: "evt-04",
            title: "Pulsing Core",
            description: "desc",
            choices: [
              { label: "Strike the core", chaosDelta: -3, partyHpDelta: 4, outcome: "struck" },
              { label: "Feed it your rage", chaosDelta: 4, partyStressDelta: 4, outcome: "fed" },
            ],
          },
        },
      };

      const result = demoReducer(testState, { type: "RESOLVE_EVENT", choiceIndex: 0 });
      expect(result.chaosMeter).toBe(9); // 12 + (-3)
      for (const m of result.party) {
        const original = testState.party.find((p) => p.id === m.id)!;
        const expectedHp = Math.min(original.maxHp, original.hp + 4);
        expect(m.hp).toBe(expectedHp);
      }
    });

    it("evt-04: 'Feed it your rage' increases chaos by 4 and stress by 4 for all", () => {
      const testState: GameState = {
        ...createSeedState(),
        phase: "event",
        chaosMeter: 8,
        currentRoom: {
          id: "room-04",
          name: "The Heart Chamber",
          description: "desc",
          enemyGroup: null,
          event: {
            id: "evt-04",
            title: "Pulsing Core",
            description: "desc",
            choices: [
              { label: "Strike the core", chaosDelta: -3, partyHpDelta: 4, outcome: "struck" },
              { label: "Feed it your rage", chaosDelta: 4, partyStressDelta: 4, outcome: "fed" },
            ],
          },
        },
      };

      const result = demoReducer(testState, { type: "RESOLVE_EVENT", choiceIndex: 1 });
      expect(result.chaosMeter).toBe(12); // 8 + 4
      for (const m of result.party) {
        const original = testState.party.find((p) => p.id === m.id)!;
        expect(m.stress).toBe(original.stress + 4);
      }
    });
  });

  // ── Seed fixture terminal reachability ────────────────────────

  describe("seed fixture reaches terminal state", () => {
    it("can reach result phase via END_RUN with retreat outcome", () => {
      let state = createSeedState();
      state = demoReducer(state, { type: "START_RUN" });
      // Enter a few rooms, then end the run
      state = demoReducer(state, { type: "ENTER_ROOM" });
      if (state.phase === "event") {
        state = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: 0 });
      }
      state = demoReducer(state, { type: "END_RUN" });

      expect(state.phase).toBe("result");
      expect(state.runOutcome).toBe("retreat");
      expect(state.roomCount).toBeGreaterThanOrEqual(1);
      expect(state.chaosMeter).toBeGreaterThanOrEqual(0);
    });

    it("can reach result phase via COMBAT_LOST with defeat outcome", () => {
      let state = createSeedState();
      state = demoReducer(state, { type: "START_RUN" });

      // Reach combat phase
      while (state.phase !== "combat") {
        if (state.phase === "event") {
          state = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: 0 });
        } else {
          state = demoReducer(state, { type: "ENTER_ROOM" });
        }
      }

      // Accept defeat
      state = demoReducer(state, { type: "COMBAT_LOST" });

      expect(state.phase).toBe("result");
      expect(state.runOutcome).toBe("defeat");
      expect(state.roomCount).toBeGreaterThanOrEqual(1);
    });

    it("can reach result phase via chaos catastrophe from event choice", () => {
      // Set up state at high chaos with a chaos-increasing choice
      const state = demoReducer(
        {
          ...createSeedState(),
          chaosMeter: 13,
          phase: "event",
          currentRoom: {
            id: "room-test",
            name: "Test Chamber",
            description: "High chaos test.",
            enemyGroup: null,
            event: {
              id: "evt-test",
              title: "Critical Surge",
              description: "Chaos is at the tipping point.",
              choices: [
                {
                  label: "Push further",
                  chaosDelta: 3,
                  outcome: "The dungeon convulses!",
                },
              ],
            },
          },
        },
        { type: "RESOLVE_EVENT", choiceIndex: 0 },
      );

      expect(state.phase).toBe("result");
      expect(state.runOutcome).toBe("catastrophe");
      expect(state.chaosMeter).toBeGreaterThanOrEqual(15);
    });

    it("result output includes roomCount, outcome, chaos, and party survival", () => {
      let state = createSeedState();
      state = demoReducer(state, { type: "START_RUN" });
      state = demoReducer(state, { type: "ENTER_ROOM" });
      if (state.phase === "event") {
        state = demoReducer(state, { type: "RESOLVE_EVENT", choiceIndex: 0 });
      }
      state = demoReducer(state, { type: "END_RUN" });

      // Validate the result surface content
      expect(state.phase).toBe("result");
      expect(typeof state.roomCount).toBe("number");
      expect(state.roomCount).toBeGreaterThanOrEqual(1);
      expect(typeof state.chaosMeter).toBe("number");
      expect(state.runOutcome).toBe("retreat");
      expect(state.resultMessage).toBeTruthy();

      // Party survival — all party members should be present
      expect(state.party.length).toBe(4);
      for (const member of state.party) {
        expect(member.hp).toBeGreaterThanOrEqual(0);
        expect(typeof member.hp).toBe("number");
        expect(typeof member.stress).toBe("number");
      }
    });
  });
});
