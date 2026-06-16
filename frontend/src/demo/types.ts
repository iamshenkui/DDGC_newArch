/**
 * Types for the isolated chaos-dungeon H5 demo.
 *
 * The demo models a minimal dungeon-crawl loop: start a run, explore rooms,
 * resolve events or combat, and track the escalating chaos-meter value.
 *
 * Combat encounters are resolved as deterministic turn-based exchanges:
 * heroes act one at a time, then enemies retaliate. No randomness or browser
 * globals are used — every transition is a pure function of (state, action).
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
  /** HP change applied to each party member (positive = heal, negative = damage). */
  partyHpDelta?: number;
  /** Stress change applied to each party member. */
  partyStressDelta?: number;
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

// ── Combat Types ────────────────────────────────────────────────────────────

/** An individual enemy combatant spawned from an EnemyGroup. */
export interface CombatEnemy {
  id: string;
  name: string;
  slot: number;
  hp: number;
  maxHp: number;
}

/** A single structured entry in the combat action log. */
export interface CombatLogEntry {
  round: number;
  actorName: string;
  actionLabel: string;
  targetName: string;
  damage: number;
  targetCurrentHp: number;
  targetMaxHp: number;
}

/** Sub-state within the "combat" phase tracking round-by-round progression. */
export interface CombatEncounter {
  round: number;
  /** Index into the party array for the hero whose turn it is. */
  activeHeroIndex: number;
  /** Array of individual enemy combatants. */
  enemies: CombatEnemy[];
  /** Structured log of every action taken this encounter. */
  log: CombatLogEntry[];
  /** Whether the encounter has been decided (win or loss). */
  resolved: boolean;
  outcome: "undecided" | "victory" | "defeat";
}

/**
 * Terminal outcome of a completed demo run.
 * - "victory": The party achieved its goal (e.g. all bosses defeated).
 * - "defeat": The party was wiped in combat.
 * - "retreat": The party fled from combat or abandoned the run early.
 * - "catastrophe": Chaos exceeded the threshold and the dungeon collapsed.
 * - "abandoned": The player explicitly ended the run without a combat resolution.
 */
export type RunOutcome =
  | "victory"
  | "defeat"
  | "retreat"
  | "catastrophe"
  | "abandoned";

export interface GameState {
  phase: DemoPhase;
  party: PartyMember[];
  dungeonLevel: number;
  currentRoom: Room | null;
  chaosMeter: number;
  turnCount: number;
  /** Number of rooms visited this run (incremented on each ENTER_ROOM). */
  roomCount: number;
  runLog: string[];
  /** Set when the run ends — explains why. */
  resultMessage: string | null;
  /** Terminal outcome of the run — non-null only when phase === "result". */
  runOutcome: RunOutcome | null;
  /** Combat sub-state — non-null only when phase === "combat". */
  combatEncounter: CombatEncounter | null;
}

// ── Actions ────────────────────────────────────────────────────────────────

export type GameAction =
  | { type: "START_RUN" }
  | { type: "ENTER_ROOM" }
  | { type: "RESOLVE_EVENT"; choiceIndex: number }
  | { type: "HERO_ATTACK" }
  | { type: "COMBAT_WIN" }
  | { type: "COMBAT_LOST" }
  | { type: "COMBAT_FLEE" }
  | { type: "ADVANCE_FLOOR" }
  | { type: "END_RUN" };
