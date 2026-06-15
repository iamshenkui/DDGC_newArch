/**
 * Types for the isolated chaos-dungeon H5 demo.
 *
 * The demo models a minimal dungeon-crawl loop: start a run, explore rooms,
 * resolve events or combat, and track the escalating chaos-meter value.
 */

/** Overall run phase — the screen uses this to decide what to render. */
export type DemoPhase =
  | "start"          // Before the run has begun
  | "dungeon-room"   // Inside a dungeon room (event or empty)
  | "event"          // A dungeon event is being resolved
  | "combat"         // Active combat encounter
  | "result";        // Run ended (victory / defeat / retreat)

export interface PartyMember {
  id: string;
  name: string;
  class: string;
  hp: number;
  maxHp: number;
  stress: number;
}

export interface EnemyGroup {
  id: string;
  name: string;
  count: number;
  hp: number;
}

export interface DungeonEvent {
  id: string;
  title: string;
  description: string;
  /** The choices the player can make. */
  choices: EventChoice[];
}

export interface EventChoice {
  label: string;
  /** Chaos delta applied when this choice is selected. */
  chaosDelta: number;
  /** Flavour text shown after picking this choice. */
  outcome: string;
}

export interface Room {
  id: string;
  name: string;
  description: string;
  /** Null means the room is empty and the party pushes forward. */
  enemyGroup: EnemyGroup | null;
  event: DungeonEvent | null;
}

export interface GameState {
  phase: DemoPhase;
  party: PartyMember[];
  dungeonLevel: number;
  currentRoom: Room | null;
  chaosMeter: number;
  turnCount: number;
  runLog: string[];
  /** Set when the run ends — explains why. */
  resultMessage: string | null;
}

// ── Actions ────────────────────────────────────────────────────────────────

export type GameAction =
  | { type: "START_RUN" }
  | { type: "ENTER_ROOM" }
  | { type: "RESOLVE_EVENT"; choiceIndex: number }
  | { type: "COMBAT_WIN" }
  | { type: "COMBAT_FLEE" }
  | { type: "ADVANCE_FLOOR" }
  | { type: "END_RUN" };
