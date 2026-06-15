/**
 * Deterministic reducer for the chaos-dungeon demo.
 *
 * Every transition is a pure function of (state, action) → state.
 * Room selection is driven by turnCount % ROOM_POOL.length so that
 * identical action sequences always produce identical results.
 */
import type { GameAction, GameState, Room } from "./types";
import { ROOM_POOL, createSeedState } from "./seedContent";

// ── Helpers ────────────────────────────────────────────────────────────────

function pickRoom(turnCount: number): Room {
  const idx = turnCount % ROOM_POOL.length;
  const src = ROOM_POOL[idx];
  return {
    ...src,
    enemyGroup: src.enemyGroup ? { ...src.enemyGroup } : null,
    event: src.event
      ? {
          ...src.event,
          choices: src.event.choices.map((c) => ({ ...c })),
        }
      : null,
  };
}

function damageMember(member: GameState["party"][number], amount: number) {
  return { ...member, hp: Math.max(0, member.hp - amount) };
}

/** Pure function — no randomness, no side-effects. */
export function demoReducer(state: GameState, action: GameAction): GameState {
  switch (action.type) {
    // ── Start ──────────────────────────────────────────────
    case "START_RUN": {
      if (state.phase !== "start") return state;
      const room = pickRoom(0);
      return {
        ...state,
        phase: "dungeon-room",
        currentRoom: room,
        turnCount: 1,
        runLog: [...state.runLog, `Run started. Entering ${room.name}.`],
      };
    }

    // ── Enter Room ─────────────────────────────────────────
    case "ENTER_ROOM": {
      if (state.phase !== "dungeon-room") return state;
      const nextTurn = state.turnCount + 1;
      const room = pickRoom(nextTurn);

      // Determine what happens: event rooms take priority over enemy-only rooms
      if (room.event) {
        return {
          ...state,
          phase: "event",
          currentRoom: room,
          turnCount: nextTurn,
          chaosMeter: state.chaosMeter + 1,
          runLog: [...state.runLog, `You enter ${room.name}. ${room.event.title}!`],
        };
      }

      if (room.enemyGroup) {
        return {
          ...state,
          phase: "combat",
          currentRoom: room,
          turnCount: nextTurn,
          chaosMeter: state.chaosMeter + 1,
          runLog: [...state.runLog, `Ambushed by ${room.enemyGroup.name}!`],
        };
      }

      // Empty room — push forward automatically
      return {
        ...state,
        currentRoom: room,
        turnCount: nextTurn,
        runLog: [...state.runLog, `${room.name} — nothing of interest. Moving on.`],
      };
    }

    // ── Resolve Event ──────────────────────────────────────
    case "RESOLVE_EVENT": {
      if (state.phase !== "event" || !state.currentRoom?.event) return state;
      const evt = state.currentRoom.event;
      const choice =
        evt.choices[action.choiceIndex] ?? evt.choices[0];

      const nextChaos = state.chaosMeter + choice.chaosDelta;

      // After event, go back to dungeon-room (which the player re-enters as a fresh room)
      // if chaos is still manageable, or to result if it goes critical
      const nextAfterEvent = () => {
        if (nextChaos >= 15) {
          return {
            phase: "result" as const,
            currentRoom: null,
            runLog: [
              ...state.runLog,
              choice.outcome,
              `Chaos reached ${nextChaos}! The dungeon collapses around you.`,
            ],
            resultMessage: "Chaos overwhelms the realm. The run ends in catastrophe.",
          };
        }
        return {
          phase: "dungeon-room" as const,
          currentRoom: null,
          runLog: [...state.runLog, choice.outcome, "You ready yourself for what lies ahead."],
          resultMessage: null,
        };
      };

      const continuation = nextAfterEvent();
      return {
        ...state,
        phase: continuation.phase,
        currentRoom: continuation.currentRoom,
        chaosMeter: nextChaos,
        runLog: continuation.runLog,
        resultMessage: continuation.resultMessage,
      };
    }

    // ── Combat ─────────────────────────────────────────────
    case "COMBAT_WIN": {
      if (state.phase !== "combat") return state;
      const healed = state.party.map((m) => {
        const heal = Math.min(m.maxHp - m.hp, 4);
        return { ...m, hp: m.hp + heal, stress: Math.max(0, m.stress - 2) };
      });
      const nextChaos = state.chaosMeter + 2;
      const bossRoom =
        state.turnCount > 0 && state.turnCount % 4 === 0;

      if (bossRoom) {
        // If it was a boss room, advance floor
        return {
          ...state,
          phase: "dungeon-room",
          party: healed,
          dungeonLevel: state.dungeonLevel + 1,
          chaosMeter: nextChaos,
          runLog: [
            ...state.runLog,
            `Victory! The last enemy falls. You descend to level ${state.dungeonLevel + 1}.`,
          ],
        };
      }

      return {
        ...state,
        phase: "dungeon-room",
        party: healed,
        chaosMeter: nextChaos,
        runLog: [...state.runLog, "Victory! The enemies are vanquished."],
      };
    }

    case "COMBAT_FLEE": {
      if (state.phase !== "combat") return state;
      const stressed = state.party.map((m) => ({
        ...m,
        stress: m.stress + 8,
      }));

      return {
        ...state,
        phase: "dungeon-room",
        party: stressed,
        chaosMeter: state.chaosMeter + 1,
        currentRoom: null,
        runLog: [...state.runLog, "You retreat from the fight, shaken but alive."],
      };
    }

    // ── Advance Floor ──────────────────────────────────────
    case "ADVANCE_FLOOR": {
      return {
        ...state,
        dungeonLevel: state.dungeonLevel + 1,
        chaosMeter: state.chaosMeter + 1,
        runLog: [...state.runLog, `You press deeper into darkness. Floor ${state.dungeonLevel + 1}.`],
      };
    }

    // ── End Run ────────────────────────────────────────────
    case "END_RUN": {
      return {
        ...state,
        phase: "result",
        currentRoom: null,
        chaosMeter: state.chaosMeter + 3,
        runLog: [...state.runLog, "The expedition returns, battered but alive."],
        resultMessage: `Run ended after ${state.turnCount} turns. Chaos at ${state.chaosMeter}.`,
      };
    }

    default:
      return state;
  }
}

/** Convenience: start from seed and apply actions. */
export function runDemoSequence(actions: GameAction[]): GameState {
  return actions.reduce(demoReducer, createSeedState());
}
