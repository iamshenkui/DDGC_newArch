# DDGC Monster Families

This document inventories every monster family in DDGC, providing the role,
type, tier, and dungeon classification derived from the `.txt` data files under
`Assets/Resources/Data/Monsters/` (DreamDeveloperGame-Crossover).

## Monster Type Classification

DDGC uses the `MonsterType` enum defined in `MechanicsDefines.cs`:

| Type | CN | Description |
|---|---|---|
| Beast | 野兽 | Animals and mythical beasts |
| Eldritch | 异灵 | Supernatural entities |
| Unholy | 不死 | Undead and animated constructs |
| Man | 人类 | Human enemies |
| Cauldron | — | Special vessel/mechanic entities |
| Corpse | — | Battlefield terrain/corpses |

Each monster data file specifies its type via `enemy_type: .id "<type>"`.

## Tier Classification

DDGC has **no explicit "elite" classification**. The game uses two tiers:

- **Common** — size 1, no `tag`, 1 turn per round. Scaled across 3 dungeon
  difficulty levels (suffix `_1`, `_2`, `_3`).
- **Boss** — size 2+, carries `tag: .id "boss"`, 2+ turns per round. Has
  unique multi-part mechanics, summon abilities, or special battle fields.

For migration purposes, we classify as:

| Migration Tier | DDGC Signal | Turn/Round | Notes |
|---|---|---|---|
| Common | `display: .size 1`, no tag | 1 | Three difficulty tiers per family |
| Boss | `tag: .id "boss"` or special field | 2–3 | Multi-part, summon, or control mechanics |

There is no intermediate elite tier in the data. Some bosses have shadow/paired
units that share the boss tag but operate as semi-independent actors.

## Dungeon Distribution

Common monsters are assigned to one of four mythological dungeon regions:

| Dungeon | CN | Theme |
|---|---|---|
| QingLong | 青龙 | Forest/swamp — mantises, trees, moths, robbers |
| BaiHu | 白虎 | Fortress — armor, blades, lizards, beetles, alligators |
| ZhuQue | 朱雀 | Fire temple — foxes, moths, lanterns, ghost fire |
| XuanWu | 玄武 | Water depths — snakes, grass, monkeys, water creatures |

Bosses are associated with specific dungeons but some are cross-dungeon
(Glutton Pawnshop, Bloodthirsty Assassin).

## Common Monster Family Inventory

Each common family appears in 3 difficulty tiers. Tier 1 stats are shown below;
tier 2/3 scale HP, SPD, and skill damage upward.

| Family | Type | Dungeon | HP (T1) | SPD (T1) | Skills | Key Mechanics | Source File | Migration Status |
|---|---|---|---|---|---|---|---|---|
| mantis_magic_flower | Beast | QingLong | 88 | 7 | poison, crowd_bleed, normal_attack, move | Blight + AoE bleed | mantis_magic_flower_1/2/3.txt | Migrated |
| mantis_spiny_flower | Beast | QingLong | 88 | 7 | ignore_armor, crowd_bleed, normal_attack, move | Armor-piercing blight + AoE bleed | mantis_spiny_flower_1/2/3.txt | Migrated |
| mantis_walking_flower | Beast | QingLong | 88 | 7 | weak, crowd_bleed, normal_attack, move | Debuff + AoE bleed | mantis_walking_flower_1/2/3.txt | Migrated |
| dry_tree_genie | Eldritch | QingLong | 90 | 3 | bleed, slow_crowd, stress, move | Ranged bleed + slow + stress | dry_tree_genie_1/2/3.txt | Migrated |
| moth_mimicry_A | Eldritch | QingLong | 63 | 6 | normal_attack, poison, stress_poison | Ranged blight + stress-poison | moth_mimicry_A_1/2/3.txt | Migrated |
| moth_mimicry_B | Eldritch | QingLong | 63 | 6 | poison, stress, stress_crowd | Ranged blight + AoE stress | moth_mimicry_B_1/2/3.txt | Migrated |
| robber_melee | Man | QingLong | 15 | 5 | normal_attack, bleed, smoke_bomb, move | Low HP, stun + debuff | robber_melee.txt | Migrated |
| robber_ranged | Man | QingLong | 10 | 3 | normal_attack, multiple_shot, throw_stone, move | Low HP, ranged stun | robber_ranged.txt | Migrated |
| metal_armor | Unholy | BaiHu | 90 | 4 | stun, bleed, normal_attack, move | High prot, melee stun+bleed | metal_armor_1/2/3.txt | Migrated |
| tiger_sword | Unholy | BaiHu | 90 | 3 | normal_attack, pull, move | Heavy melee + pull | tiger_sword_1/2/3.txt | Migrated |
| lizard | Eldritch | BaiHu | 62 | 6 | stun, intimidate, stress, move | Fast, stun + stress | lizard_1/2/3.txt | Migrated |
| unicorn_beetle_A | Eldritch | BaiHu | 62 | 7 | normal_attack, bleed, bleed_crowd, move | Ranged bleed + ignore def | unicorn_beetle_A_1/2/3.txt | Migrated |
| unicorn_beetle_B | Eldritch | BaiHu | 62 | 7 | normal_attack, bleed, stress, move | Ranged bleed + stress | unicorn_beetle_B_1/2/3.txt | Migrated |
| alligator_yangtze | Beast | BaiHu | 94 | 6 | normal_attack, bleed, mark_riposte | Melee bruiser + riposte | alligator_yangtze_1/2/3.txt | Migrated |
| ghost_fire_assist | Eldritch | ZhuQue | 72 | 6 | assist, buff_self, ghost_fire_split | Ally buff + self-split on death | ghost_fire_assist_1/2/3.txt | Migrated |
| ghost_fire_damage | Eldritch | ZhuQue | 72 | 6 | stress, burn_attack, ghost_fire_split | Burn + stress + self-split on death | ghost_fire_damage_1/2/3.txt | Migrated |
| fox_fire | Beast | ZhuQue | 65 | 6 | bite, vomit, protect, move | Bleed + burn debuff + ally guard | fox_fire_1/2/3.txt | Migrated |
| moth_fire | Eldritch | ZhuQue | 65 | 7 | stress_attack, cocoon, fly_into_fire | Stress + cocoon (defend+heal) + burn | moth_fire_1/2/3.txt | Migrated |
| lantern | Eldritch | ZhuQue | 70 | 7 | stress, burn_attack | Magic burn + stress | lantern_1/2/3.txt | Migrated |
| snake_water | Eldritch | XuanWu | 62 | 6 | stun, poison_fang, move | Stun + blight | snake_water_1/2/3.txt | Migrated |
| water_grass | Eldritch | XuanWu | 62 | 3 | stun, puncture, attack_crowd, convolve, move | Stun + bleed + pull (5 skills) | water_grass_1/2/3.txt | Migrated |
| monkey_water | Unholy | XuanWu | 98 | 6 | base_melee, rush, stress, move | Stress-tag melee + charge | monkey_water_1/2/3.txt | Migrated |

## Boss Family Inventory

Bosses have unique IDs without difficulty tiers. Each boss entry describes its
multi-part mechanics and summon/minion relationships.

| Boss | Type | Dungeon | HP | SPD | Turns/Rd | Skills | Key Mechanics | Source File | Migration Status |
|---|---|---|---|---|---|---|---|---|---|
| azure_dragon | Beast | QingLong | 150 | 7 | 2 | bloodscale_reaping, dragonfear_crash, summit_relocation, soulfog_enthrall, dragoncry_storm, volt_tyranny, voltic_baptism, capricious_skies, swap_dragon_ball, swap_dragon_ball_other, swap_dragon_ball_summon | Shared health with ball units; summons thunder/wind balls; buff/debuff control | azure_dragon.txt | Migrated |
| vermilion_bird | Beast | ZhuQue | 160 | 7 | 2 | singing_loudly, ruin, ruin1, precise_pecking, iron_feather, bide, calm_nerves, explosion | Shared health with tail units; burn + self-heal cycle; absorb mechanic | vermilion_bird.txt | Migrated |
| white_tiger_C | Beast | BaiHu | 115 | 7 | 3 | thunder_lightning, paw, raging_fire, true_strike, jump, deter_stress, deter_def | 3 turns/round; multi-phase (A/B clones → C final form); stun+burn+stress | white_tiger_C.txt | Migrated |
| black_tortoise_A | Eldritch | XuanWu | 115 | 4 | 2 | call_roll, rain_spray, ice_spike, hunger_cold, inner_battle, near_mountain_river, hunger_cold_1, unexpectedly | Tank body; slow, frozen + stress; share damage with snake body | black_tortoise_A.txt | Migrated |
| black_tortoise_B | Eldritch | XuanWu | 115 | 7 | 2 | call_roll, rain_spray, freezing_cold, benefits_stress, fangs_sprayed, armor, fangs_sprayed_1, snake_bites | Snake body; fast, blight + disease; share damage with turtle body | black_tortoise_B.txt | Migrated |
| rotvine_wraith | Eldritch | XuanWu | 150 | 5 | 2 | cadaver_bloom, rotvine_snare, sepsis_strangulate, telluric_resurrect, carrion_sowing, move | Summon rotten_fruit A/B/C; burn + mark + stun + bleed | rotvine_wraith.txt | Migrated |
| skeletal_tiller | Eldritch | XuanWu | 150 | 6 | 2 | bone_reforge, famine_reaping, scarecrow_shriek, grave_tug, crop_rot_claw | Summon vegetable; heavy melee + frozen debuff + stress | skeletal_tiller.txt | Migrated |
| necrodrake_embryosac | Man | XuanWu | 150 | 5 | 2 | requiem_stillbirth, placental_tap, untimely_progeny, doom_symbiosis, ecdysis_metamorphosis | Life-linked to egg_membrane captor mechanic; hero capture + self-cleanse | necrodrake_embryosac.txt | Migrated |
| gambler | Eldritch | ZhuQue | 150 | 5 | 2 | dice_thousand, hollow_victory, card_doomsday, jackpot_requiem, summon_mahjong | Summon mahjong_red/green/white; random debuff + stress + bleed | gambler.txt | Migrated |
| scorchthroat_chanteuse | Eldritch | ZhuQue | 150 | 6 | 2 | cremona_last_chord, pyre_resonance, ashen_communion, encore_embers, move | Summon sc_blow/bow/pluck; magic burn + stress | scorchthroat_chanteuse.txt | Migrated |
| frostvein_clam | Eldritch | XuanWu | 150 | 5 | 1 | glacial_torrent, abyssal_glare, nacreous_homunculus, prismatic_clench | Riposte; summon pearlkin_flawed/opalescent; frozen + stress | frostvein_clam.txt | Migrated |
| bloodthirsty_assassin | Eldritch | Cross | 150 | 6 | 2 | bloodstrike_ambush, phantom_lunge, crimson_duet, scarlet_guillotine | Paired with shadow; crimson_duet averages HP; ignore-def finisher | bloodthirsty_assassin.txt | Migrated |
| glutton_pawnshop | Eldritch | Cross | 150 | 5 | 2 | flesh_usury_contract, compound_agony, invitation, foreclosed_wail | Size 3 (largest); controller mechanic; tag-based bleed/blight/debuff | glutton_pawnshop.txt | Migrated |

## Boss Part and Minion Units

These are not independent encounters. They appear only as part of a boss fight.

| Unit | Parent Boss | Type | HP | Role | Source File |
|---|---|---|---|---|---|
| azure_dragon_ball_thunder | Azure Dragon | Beast | 55 | Shared-health ball (thunder) | azure_dragon_ball_thunder.txt |
| azure_dragon_ball_wind | Azure Dragon | Beast | 55 | Shared-health ball (wind) | azure_dragon_ball_wind.txt |
| vermilion_bird_tail_A | Vermilion Bird | Beast | 0 | Shared-health tail (invulnerable body) | vermilion_bird_tail_A.txt |
| vermilion_bird_tail_B | Vermilion Bird | Beast | 0 | Shared-health tail (invulnerable body) | vermilion_bird_tail_B.txt |
| white_tiger_A | White Tiger | Beast | 90 | Clone phase (2 turns/rd) | white_tiger_A.txt |
| white_tiger_B | White Tiger | Beast | 90 | Clone phase (2 turns/rd) | white_tiger_B.txt |
| white_tiger_terrain | White Tiger | Corpse | 15 | Terrain/corpse placeholder | white_tiger_terrain.txt |
| bloodthirsty_shadow | Bloodthirsty Assassin | Eldritch | 150 | Paired shadow; stress + bleed | bloodthirsty_shadow.txt |
| egg_membrane_empty | Necrodrake | Cauldron | 210 | Captor vessel (empty, life-linked) | egg_membrane_empty.txt |
| egg_membrane_full | Necrodrake | Cauldron | 10 | Captor vessel (holding hero, life-linked) | egg_membrane_full.txt |
| mahjong_red | Gambler | Eldritch | 20 | Summoned minion | mahjong_red.txt |
| mahjong_green | Gambler | Eldritch | 20 | Summoned minion | mahjong_green.txt |
| mahjong_white | Gambler | Eldritch | 20 | Summoned minion | mahjong_white.txt |
| sc_blow | Scorchthroat | Eldritch | 20 | Summoned minion | sc_blow.txt |
| sc_bow | Scorchthroat | Eldritch | 20 | Summoned minion | sc_bow.txt |
| sc_pluck | Scorchthroat | Eldritch | 20 | Summoned minion | sc_pluck.txt |
| pearlkin_flawed | Frostvein Clam | Eldritch | 30 | Summoned minion | pearlkin_flawed.txt |
| pearlkin_opalescent | Frostvein Clam | Eldritch | 30 | Summoned minion | pearlkin_opalescent.txt |
| vegetable | Skeletal Tiller | Eldritch | 20 | Summoned minion | vegetable.txt |
| rotten_fruit_A | Rotvine Wraith | Eldritch | 30 | Summoned minion | rotten_fruit_A.txt |
| rotten_fruit_B | Rotvine Wraith | Eldritch | 30 | Summoned minion | rotten_fruit_B.txt |
| rotten_fruit_C | Rotvine Wraith | Eldritch | 30 | Summoned minion | rotten_fruit_C.txt |

## Common Monster Difficulty Scaling

Each common family has 3 difficulty tiers. The `_1` suffix is the base tier,
`_2` is medium, and `_3` is the hardest. Scaling pattern observed across all
families:

| Tier | HP Multiplier | SPD Increase | Notes |
|---|---|---|---|
| `_1` | Base (1.0×) | Base | Dungeon level 1 |
| `_2` | ~1.4× | +1 | Dungeon level 3–4 |
| `_3` | ~1.8–2.0× | +2 | Dungeon level 5–6 |

Example (fox_fire family):
- fox_fire_1: HP 65, SPD 6
- fox_fire_2: HP 96, SPD 7
- fox_fire_3: HP 125, SPD 8

## Boss Encounter Structures

Bosses use several structural patterns that the encounter system must support:

### Shared Health
The Vermilion Bird shares a health pool with its tail parts via
`shared_health: .id vermilion_bird`. Damage to any part reduces the shared
pool. Azure Dragon similarly shares health with its ball units.

### Multi-Body
Black Tortoise is two independent bodies (turtle A + snake B) that act on
separate initiative tracks. They coordinate via `share_damage` skill effects
and `in_black_tortoise_field` battle modifier.

### Multi-Phase
White Tiger transitions through phases: A/B clones (2 turns each) followed by
C final form (3 turns/round, `disable_stall_penalty True`).

### Captor Mechanic
Necrodrake Embryosac can capture heroes via `CaptureNe` effect, placing them
inside `egg_membrane_full` (Cauldron type). Heroes are released when the egg
is destroyed or the embryosac uses `ecdysis_metamorphosis`.

### Summon Pattern
Multiple bosses summon minion units at runtime:
- Gambler → mahjong_red/green/white
- Scorchthroat Chanteuse → sc_blow/bow/pluck
- Frostvein Clam → pearlkin_flawed/opalescent
- Skeletal Tiller → vegetable
- Rotvine Wraith → rotten_fruit_A/B/C

### Controller Mechanic
Glutton Pawnshop has a `controller:` block and uses tag-based debuff
application (`gp_control`, `gp_tag_bleed`, `gp_tag_blight`, `gp_tag_debuff`).

### Paired Boss
Bloodthirsty Assassin + Shadow operate as a pair. `crimson_duet` averages
their HP pools, making them a single tactical unit despite separate actors.

### Riposte
Frostvein Clam has a `riposte_skill` that triggers on being attacked, and
Alligator Yangtze (common) also has a `riposte_skill`.

## Monster Data File Format

Each monster is defined in a `.txt` file under `Assets/Resources/Data/Monsters/`
using a custom DSL. Key fields:

```
name: <id>
type: <family_id>

art:
skill: .id "<skill_id>" .anim "<animation>"
.end

info:
display: .size <1|2|3>
enemy_type: .id "<type>"
stats: .hp <N> .def <N%> .prot <N> .spd <N> .stun_resist <N%> ...
skill: .id "<id>" .type "<melee|ranged>" .atk <N%> .dmg <min> <max> .crit <N%> .effect "<effect>" ...
personality: .prefskill <N>
initiative: .number_of_turns_per_round <N>
monster_brain: .id <brain_id>
tag: .id "boss"              # only on boss-tagged enemies
shared_health: .id <pool>    # only on shared-health bosses
riposte_skill: .id "<id>"    # only on riposte-capable enemies
controller:                   # only on controller bosses
battle_modifier: .in_<boss>_field True  # special battle field flag
.end
```

## Migration Priority

The following order balances dungeon progression (enemies before bosses) with
structural complexity (simpler families first):

1. **Common QingLong** — mantis (3 families), dry_tree_genie, moth (2 families), robber (2 families)
2. **Common BaiHu** — metal_armor, tiger_sword, lizard, unicorn_beetle (2 families), alligator_yangtze
3. **Common ZhuQue** — ghost_fire (2 families), fox_fire, moth_fire, lantern
4. **Common XuanWu** — snake_water, water_grass, monkey_water
5. **Boss dungeon** — Azure Dragon, Vermilion Bird, White Tiger, Black Tortoise
6. **Boss summon** — Rotvine Wraith, Skeletal Tiller, Necrodrake Embryosac
7. **Boss cross-dungeon** — Gambler, Scorchthroat Chanteuse, Frostvein Clam, Bloodthirsty Assassin, Glutton Pawnshop

## Already Migrated Content

The monster-content situation has moved well beyond the original placeholder
state. The DDGC headless crate now ships migrated monster content under
`src/content/monsters/`, and those entries are wired into the runtime through
both `src/content/monsters/mod.rs` and `src/monsters/mod.rs`.

Current code state:

- All common families listed in the inventory above have migrated content
  modules and are registered in the content pack.
- The primary boss families listed above are also migrated, along with their
  required boss-part and summon/minion units.
- Encounter/runtime mechanics needed to execute those bosses headlessly were
  closed in Phase 3 and the post-merge stabilization follow-up.

What remains is no longer "monster migration is outstanding" in bulk. The
remaining work is narrower:

- keep the family inventory in sync with actual registered content,
- close the remaining DDGC-only condition families tracked under B-004,
- continue parity and encounter verification for migrated content.

Legacy placeholder enemies still exist for older parity scenarios:

- `bone_soldier`
- `necromancer`

They are legacy fixtures, not the source of truth for DDGC monster migration.
