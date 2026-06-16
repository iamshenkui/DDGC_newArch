/**
 * Deterministic reducer for the chaos-dungeon demo.
 *
 * Every transition is a pure function of (state, action) → state.
 * Room selection is driven by turnCount % ROOM_POOL.length so that
 * identical action sequences always produce identical results.
 *
 * Combat is resolved as deterministic turn-based exchanges: heroes act
 * one at a time via HERO_ATTACK, then enemies retaliate automatically
 * once all heroes in the round have acted.
 */
import type {
  CombatEncounter,
  CombatEnemy,
  CombatLogEntry,
  EnemyGroup,
  GameAction,
  GameState,
  PartyMember,
  Room,
} from "./types";
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

// ── Combat Helpers ──────────────────────────────────────────────────────────

/**
 * Fixed hero damage per class — deterministic, no randomness.
 * Values chosen so combat lasts several rounds for the demo.
 */
const HERO_DAMAGE: Record<string, number> = {
  Crusader: 8,
  Vestal: 4,
  Highwayman: 7,
  "Grave Robber": 6,
};

/**
 * Enemy combat stats — damage dealt per attack + stress inflicted.
 */
const ENEMY_STATS: Record<string, { damage: number; stress: number }> = {
  "Cultist Acolytes": { damage: 3, stress: 0 },
  "Swamp Leeches": { damage: 2, stress: 1 },
  "Animated Armour": { damage: 6, stress: 0 },
  "Heart Tendrils": { damage: 3, stress: 0 },
};

function getHeroDamage(className: string): number {
  return HERO_DAMAGE[className] ?? 5;
}

function getEnemyStats(
  enemyName: string,
): { damage: number; stress: number } {
  return ENEMY_STATS[enemyName] ?? { damage: 3, stress: 0 };
}

/** Find the first living enemy — the hero's target this turn. */
function findFirstLivingEnemy(
  enemies: CombatEnemy[],
): number {
  for (let i = 0; i < enemies.length; i++) {
    if (enemies[i].hp > 0) return i;
  }
  return -1;
}

/**
 * Deterministic enemy targeting: attack the party member with the lowest
 * current HP. Ties go to the lower index. Skips dead members (hp <= 0).
 */
function findTargetHero(party: PartyMember[]): number {
  let bestIdx = -1;
  let bestHp = Infinity;
  for (let i = 0; i < party.length; i++) {
    if (party[i].hp <= 0) continue;
    if (party[i].hp < bestHp) {
      bestHp = party[i].hp;
      bestIdx = i;
    }
  }
  // Fallback to first living member (or 0 if all dead — checked upstream)
  return bestIdx >= 0 ? bestIdx : 0;
}

/** Check whether the entire party is at 0 HP. */
function isPartyWiped(party: PartyMember[]): boolean {
  return party.every((m) => m.hp <= 0);
}

/** Create a CombatEncounter from an EnemyGroup definition. */
function createCombatEncounter(group: EnemyGroup): CombatEncounter {
  const enemies: CombatEnemy[] = [];
  for (let i = 0; i < group.count; i++) {
    enemies.push({
      id: `${group.id}-s${i}`,
      name: group.name,
      slot: i,
      hp: group.hp,
      maxHp: group.hp,
    });
  }
  return {
    round: 1,
    activeHeroIndex: 0,
    enemies,
    log: [],
    resolved: false,
    outcome: "undecided",
  };
}

/**
 * Resolve the enemy phase: each living enemy attacks the party's most
 * wounded hero. Returns the updated party and combat-log entries.
 */
function resolveEnemyPhase(
  party: PartyMember[],
  enemies: CombatEnemy[],
  round: number,
): { party: PartyMember[]; entries: CombatLogEntry[] } {
  let currentParty = party;
  const entries: CombatLogEntry[] = [];

  for (const enemy of enemies) {
    if (enemy.hp <= 0) continue;

    const tgtIdx = findTargetHero(currentParty);
    if (tgtIdx < 0) break; // no valid target (all dead)

    const stats = getEnemyStats(enemy.name);
    const hero = currentParty[tgtIdx];
    const newHp = Math.max(0, hero.hp - stats.damage);

    currentParty = currentParty.map((m, i) =>
      i === tgtIdx
        ? {
            ...m,
            hp: newHp,
            stress: m.stress + stats.stress,
          }
        : m,
    );

    entries.push({
      round,
      actorName: enemy.name,
      actionLabel: "attacks",
      targetName: hero.name,
      damage: stats.damage,
      targetCurrentHp: newHp,
      targetMaxHp: hero.maxHp,
    });
  }

  return { party: currentParty, entries };
}

/**
 * Execution log added to runLog for combat transitions.  These identify the
 * actor, target, and result as required by the acceptance criteria.
 */
function runLogEntry(
  actor: string,
  action: string,
  target: string,
  damage: number,
  hpAfter: number,
): string {
  return `${actor} ${action} ${target} for ${damage} damage (${hpAfter} HP remaining).`;
}

function enemyLogEntry(
  actor: string,
  action: string,
  target: string,
  damage: number,
  hpAfter: number,
  stress: number,
): string {
  const base = `${actor} ${action} ${target} for ${damage} damage (${hpAfter} HP remaining).`;
  return stress > 0 ? `${base} Stress +${stress}.` : base;
}

// ── Reducer ─────────────────────────────────────────────────────────────────

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
          roomCount: state.roomCount + 1,
          chaosMeter: state.chaosMeter + 1,
          runLog: [
            ...state.runLog,
            `You enter ${room.name}. ${room.event.title}!`,
          ],
        };
      }

      if (room.enemyGroup) {
        return {
          ...state,
          phase: "combat",
          currentRoom: room,
          turnCount: nextTurn,
          roomCount: state.roomCount + 1,
          chaosMeter: state.chaosMeter + 1,
          combatEncounter: createCombatEncounter(room.enemyGroup),
          runLog: [
            ...state.runLog,
            `Ambushed by ${room.enemyGroup.name}!`,
          ],
        };
      }

      // Empty room — push forward automatically
      return {
        ...state,
        currentRoom: room,
        turnCount: nextTurn,
        roomCount: state.roomCount + 1,
        runLog: [
          ...state.runLog,
          `${room.name} — nothing of interest. Moving on.`,
        ],
      };
    }

    // ── Resolve Event ──────────────────────────────────────
    case "RESOLVE_EVENT": {
      if (state.phase !== "event" || !state.currentRoom?.event) return state;
      const evt = state.currentRoom.event;
      const choice =
        evt.choices[action.choiceIndex] ?? evt.choices[0];

      const nextChaos = state.chaosMeter + choice.chaosDelta;

      // Apply party value changes from the event choice
      let partyEffectLog: string[] = [];
      const updatedParty = state.party.map((member) => {
        let { hp, stress } = member;

        if (choice.partyHpDelta !== undefined) {
          hp = Math.max(0, Math.min(member.maxHp, hp + choice.partyHpDelta));
        }
        if (choice.partyStressDelta !== undefined) {
          stress = Math.max(0, stress + choice.partyStressDelta);
        }

        const changed =
          hp !== member.hp || stress !== member.stress;

        if (changed) {
          const parts: string[] = [];
          if (hp !== member.hp) {
            const diff = hp - member.hp;
            parts.push(`HP ${diff > 0 ? "+" : ""}${diff}`);
          }
          if (stress !== member.stress) {
            const diff = stress - member.stress;
            parts.push(`Stress ${diff > 0 ? "+" : ""}${diff}`);
          }
          partyEffectLog.push(
            `${member.name}: ${parts.join(", ")}.`,
          );
        }

        return { ...member, hp, stress };
      });

      const nextAfterEvent = () => {
        if (nextChaos >= 15) {
          return {
            phase: "result" as const,
            currentRoom: null,
            combatEncounter: null as CombatEncounter | null,
            runLog: [
              ...state.runLog,
              choice.outcome,
              ...partyEffectLog,
              `Chaos reached ${nextChaos}! The dungeon collapses around you.`,
            ],
            resultMessage:
              "Chaos overwhelms the realm. The run ends in catastrophe.",
            runOutcome: "catastrophe" as const,
          };
        }
        // If the room has enemies, transition to combat after the event
        if (state.currentRoom?.enemyGroup) {
          return {
            phase: "combat" as const,
            currentRoom: state.currentRoom,
            combatEncounter: createCombatEncounter(state.currentRoom!.enemyGroup),
            runLog: [
              ...state.runLog,
              choice.outcome,
              ...partyEffectLog,
              `The ${state.currentRoom!.enemyGroup.name} attack!`,
            ],
            resultMessage: null,
            runOutcome: null,
          };
        }
        return {
          phase: "dungeon-room" as const,
          currentRoom: null,
          combatEncounter: null as CombatEncounter | null,
          runLog: [
            ...state.runLog,
            choice.outcome,
            ...partyEffectLog,
            "You ready yourself for what lies ahead.",
          ],
          resultMessage: null,
          runOutcome: null,
        };
      };

      const continuation = nextAfterEvent();
      return {
        ...state,
        party: updatedParty,
        phase: continuation.phase,
        currentRoom: continuation.currentRoom,
        chaosMeter: nextChaos,
        combatEncounter: continuation.combatEncounter,
        runLog: continuation.runLog,
        resultMessage: continuation.resultMessage,
        runOutcome: continuation.runOutcome ?? null,
      };
    }

    // ── Combat: Hero Attack ────────────────────────────────
    case "HERO_ATTACK": {
      if (state.phase !== "combat" || !state.combatEncounter) return state;
      const enc = state.combatEncounter;

      // Can't act in a resolved encounter
      if (enc.resolved) return state;

      // Find the next living hero to act
      let heroIdx = enc.activeHeroIndex;
      while (heroIdx < state.party.length && state.party[heroIdx].hp <= 0) {
        heroIdx++;
      }

      // If no living hero left, the party is wiped
      if (heroIdx >= state.party.length) {
        return {
          ...state,
          combatEncounter: {
            ...enc,
            resolved: true,
            outcome: "defeat",
          },
          runLog: [
            ...state.runLog,
            "No heroes remain standing…",
          ],
        };
      }

      const hero = state.party[heroIdx];
      const targetIdx = findFirstLivingEnemy(enc.enemies);

      // If no enemy left, mark victory
      if (targetIdx < 0) {
        return {
          ...state,
          combatEncounter: {
            ...enc,
            resolved: true,
            outcome: "victory",
          },
          runLog: [
            ...state.runLog,
            "All enemies have been vanquished!",
          ],
        };
      }

      // Resolve the hero's attack
      const damage = getHeroDamage(hero.class);
      const target = enc.enemies[targetIdx];
      const newEnemyHp = Math.max(0, target.hp - damage);

      const updatedEnemies = enc.enemies.map((e, i) =>
        i === targetIdx ? { ...e, hp: newEnemyHp } : e,
      );

      const combatLogEntry: CombatLogEntry = {
        round: enc.round,
        actorName: hero.name,
        actionLabel: "strikes",
        targetName: target.name,
        damage,
        targetCurrentHp: newEnemyHp,
        targetMaxHp: target.maxHp,
      };

      const newCombatLog = [...enc.log, combatLogEntry];

      // Check if all enemies are dead after this attack
      const allEnemiesDead = updatedEnemies.every((e) => e.hp <= 0);
      if (allEnemiesDead) {
        return {
          ...state,
          combatEncounter: {
            ...enc,
            enemies: updatedEnemies,
            log: newCombatLog,
            resolved: true,
            outcome: "victory",
          },
          runLog: [
            ...state.runLog,
            runLogEntry(
              hero.name,
              "strikes",
              target.name,
              damage,
              newEnemyHp,
            ),
            `${hero.name} delivers the final blow! Victory!`,
          ],
        };
      }

      // Move to next hero
      const nextHeroIdx = heroIdx + 1;

      // If all heroes have acted, resolve enemy phase
      if (nextHeroIdx >= state.party.length) {
        const { party: afterEnemies, entries } = resolveEnemyPhase(
          state.party.map((m) =>
            m.id === hero.id ? { ...m } : m,
          ),
          updatedEnemies,
          enc.round,
        );

        // Check for party wipe after enemy phase
        const wipe = isPartyWiped(afterEnemies);

        return {
          ...state,
          party: afterEnemies,
          chaosMeter: state.chaosMeter + 1,
          combatEncounter: {
            ...enc,
            activeHeroIndex: 0,
            round: enc.round + 1,
            enemies: updatedEnemies,
            log: [...newCombatLog, ...entries],
            resolved: wipe,
            outcome: wipe ? "defeat" : "undecided",
          },
          runLog: [
            ...state.runLog,
            runLogEntry(
              hero.name,
              "strikes",
              target.name,
              damage,
              newEnemyHp,
            ),
            ...entries.map((e) =>
              enemyLogEntry(
                e.actorName,
                e.actionLabel,
                e.targetName,
                e.damage,
                e.targetCurrentHp,
                e.damage > 0
                  ? afterEnemies.find(
                      (m) =>
                        m.name === e.targetName &&
                        m.hp === e.targetCurrentHp,
                    )?.stress ?? 0
                  : 0,
              ),
            ),
            `Round ${enc.round} ends. Chaos intensifies.`,
          ],
        };
      }

      // Hero's turn done, next hero's turn (no enemy phase yet)
      // Apply damage to hero... wait, no — the hero already attacked.
      // We only update the encounter state for the next hero.
      // Party state doesn't change on hero attack — only enemy HP.

      return {
        ...state,
        combatEncounter: {
          ...enc,
          activeHeroIndex: nextHeroIdx,
          enemies: updatedEnemies,
          log: newCombatLog,
        },
        runLog: [
          ...state.runLog,
          runLogEntry(
            hero.name,
            "strikes",
            target.name,
            damage,
            newEnemyHp,
          ),
        ],
      };
    }

    // ── Combat: Win → leave combat ─────────────────────────
    case "COMBAT_WIN": {
      if (state.phase !== "combat") return state;
      const healed = state.party.map((m) => {
        const heal = Math.min(m.maxHp - m.hp, 4);
        return {
          ...m,
          hp: m.hp + heal,
          stress: Math.max(0, m.stress - 2),
        };
      });
      const nextChaos = state.chaosMeter + 2;
      const bossRoom =
        state.turnCount > 0 && state.turnCount % 4 === 0;

      if (bossRoom) {
        return {
          ...state,
          phase: "dungeon-room",
          party: healed,
          dungeonLevel: state.dungeonLevel + 1,
          chaosMeter: nextChaos,
          combatEncounter: null,
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
        combatEncounter: null,
        runLog: [
          ...state.runLog,
          "Victory! The enemies are vanquished.",
        ],
      };
    }

    // ── Combat: Lost → end run ─────────────────────────────
    case "COMBAT_LOST": {
      if (state.phase !== "combat") return state;
      return {
        ...state,
        phase: "result",
        currentRoom: null,
        combatEncounter: null,
        runOutcome: "defeat",
        resultMessage:
          "Your party has fallen. The dungeon claims another soul.",
        runLog: [
          ...state.runLog,
          "The party has been defeated in combat…",
        ],
      };
    }

    // ── Combat: Flee ───────────────────────────────────────
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
        combatEncounter: null,
        runLog: [
          ...state.runLog,
          "You retreat from the fight, shaken but alive.",
        ],
      };
    }

    // ── Advance Floor ──────────────────────────────────────
    case "ADVANCE_FLOOR": {
      return {
        ...state,
        dungeonLevel: state.dungeonLevel + 1,
        chaosMeter: state.chaosMeter + 1,
        runLog: [
          ...state.runLog,
          `You press deeper into darkness. Floor ${state.dungeonLevel + 1}.`,
        ],
      };
    }

    // ── End Run ────────────────────────────────────────────
    case "END_RUN": {
      return {
        ...state,
        phase: "result",
        currentRoom: null,
        chaosMeter: state.chaosMeter + 3,
        combatEncounter: null,
        runOutcome: "retreat",
        runLog: [
          ...state.runLog,
          "The expedition returns, battered but alive.",
        ],
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
