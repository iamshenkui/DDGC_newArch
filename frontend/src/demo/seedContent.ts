import type { GameState, PartyMember, Room } from "./types";

// ── Seed Party ─────────────────────────────────────────────────────────────

export const SEED_PARTY: PartyMember[] = [
  { id: "hero-1", name: "Aelric", class: "Crusader", hp: 28, maxHp: 32, stress: 0 },
  { id: "hero-2", name: "Maren", class: "Vestal", hp: 22, maxHp: 24, stress: 0 },
  { id: "hero-3", name: "Doran", class: "Highwayman", hp: 20, maxHp: 20, stress: 5 },
  { id: "hero-4", name: "Sylvie", class: "Grave Robber", hp: 18, maxHp: 18, stress: 3 },
];

// ── Room Templates ─────────────────────────────────────────────────────────

/**
 * A fixed pool of rooms. The reducer picks rooms by index
 * (index = turnCount % ROOM_POOL.length) so transitions are deterministic.
 */
export const ROOM_POOL: Room[] = [
  {
    id: "room-01",
    name: "Crumbling Hall",
    description:
      "Dust motes dance in the pale light filtering through shattered windows. " +
      "Cracked flagstones bear the faded symbols of an old order.",
    enemyGroup: {
      id: "enemy-01",
      name: "Cultist Acolytes",
      count: 2,
      hp: 10,
    },
    event: {
      id: "evt-01",
      title: "Desecrated Altar",
      description:
        "A blood-stained altar stands in the corner, a ghastly relic still humming with tainted energy.",
      choices: [
        {
          label: "Smash the altar",
          chaosDelta: -2,
          partyHpDelta: -3,
          outcome:
            "The party shatters the altar. A wave of rancid energy bursts forth but dissipates, leaving the air clearer.",
        },
        {
          label: "Offer a prayer",
          chaosDelta: 1,
          partyStressDelta: -2,
          outcome:
            "Maren offers a quiet prayer. The altar trembles — something heard, and it is not pleased.",
        },
        {
          label: "Ignore it and move on",
          chaosDelta: 1,
          outcome:
            "You leave the altar untouched, but the unease lingers. The dungeon grows more hostile.",
        },
      ],
    },
  },
  {
    id: "room-02",
    name: "Flooded Vault",
    description:
      "Water laps at your ankles, carrying a foul smell. Strange ripples suggest something stirs beneath the surface.",
    enemyGroup: {
      id: "enemy-02",
      name: "Swamp Leeches",
      count: 3,
      hp: 6,
    },
    event: {
      id: "evt-02",
      title: "Murky Pool",
      description:
        "At the centre of the flooded chamber, an eerie luminescent pool pulsates. " +
        "Tendrils of light dance beneath the surface, offering an unsettling warmth.",
      choices: [
        {
          label: "Drink from the pool",
          chaosDelta: 2,
          partyHpDelta: 5,
          partyStressDelta: -3,
          outcome:
            "The water is strangely sweet. A wave of vitality washes over the party, " +
            "though the dungeon seems to grow more alert to your presence.",
        },
        {
          label: "Disturb the water",
          chaosDelta: -1,
          partyHpDelta: -2,
          outcome:
            "The pool erupts in sudden fury — barbed tendrils lash out, cutting deep " +
            "before retreating. The water stills, somehow calmer than before.",
        },
      ],
    },
  },
  {
    id: "room-03",
    name: "Armoury",
    description:
      "Rusted weapons still line the walls. In the centre, a suit of armour shifts slightly, as if breathing.",
    enemyGroup: {
      id: "enemy-03",
      name: "Animated Armour",
      count: 1,
      hp: 20,
    },
    event: {
      id: "evt-03",
      title: "Cursed Blade",
      description:
        "A sword pulses with dark energy on a pedestal. It promises power — at a cost.",
      choices: [
        {
          label: "Take the blade",
          chaosDelta: 3,
          partyStressDelta: 5,
          outcome:
            "Gripping the hilt sends a jolt through your arm. Power surges, but the blade whispers dark promises.",
        },
        {
          label: "Shatter the blade",
          chaosDelta: -1,
          partyHpDelta: -2,
          outcome:
            "The blade shatters. A scream echoes through the dungeon as the curse is lifted.",
        },
      ],
    },
  },
  {
    id: "room-04",
    name: "The Heart Chamber",
    description:
      "A massive, pulsating organic mass fills the centre of this chamber. Tendrils crawl along the walls.",
    enemyGroup: {
      id: "enemy-04",
      name: "Heart Tendrils",
      count: 4,
      hp: 8,
    },
    event: {
      id: "evt-04",
      title: "Pulsing Core",
      description:
        "The heart beats in a rhythm that resonates with your own. The chaos in the room is palpable.",
      choices: [
        {
          label: "Strike the core",
          chaosDelta: -3,
          partyHpDelta: 4,
          outcome:
            "Your blow lands true. The heart spasms and a wave of calm spreads through the chamber, mending wounds.",
        },
        {
          label: "Feed it your rage",
          chaosDelta: 4,
          partyStressDelta: 4,
          outcome:
            "The heart drinks in your anger and swells. The dungeon trembles with renewed fury.",
        },
      ],
    },
  },
];

// ── Seed State ─────────────────────────────────────────────────────────────

export function createSeedState(): GameState {
  return {
    phase: "start",
    party: SEED_PARTY.map((m) => ({ ...m })),
    dungeonLevel: 1,
    currentRoom: null,
    chaosMeter: 5,
    turnCount: 0,
    runLog: ["The Darkest Dungeon awaits…"],
    resultMessage: null,
    combatEncounter: null,
  };
}
