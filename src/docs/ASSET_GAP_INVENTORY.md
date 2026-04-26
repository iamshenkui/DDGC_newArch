# DDGC Asset Gap Inventory

This document is the canonical checked-in audit of all DDGC raw asset families,
listing their migration status, parser/registry/runtime anchors, and next
implementation targets. It is source-backed against the actual registry code
in `src/heroes/families.rs` and `src/monsters/mod.rs`, not inferred from
README text.

## Source Anchors

| Asset Family Type | Registry Module | Runtime Anchor |
|---|---|---|
| Hero families | `src/heroes/families.rs::HeroFamilyRegistry` | `ContentPack::default()` |
| Monster families | `src/monsters/mod.rs::build_registry()` | `monsters::register_content()` |

## Hero Families

All five DDGC hero families are fully migrated with base, white (+1), and black (+2) variants.

| Family | Status | Parser Module | Registry | Runtime Anchor | Next Target | Source Reference |
|---|---|---|---|---|---|---|
| Alchemist | Migrated | `src/content/heroes/alchemist.rs` | `HeroFamilyRegistry` | `ContentPack::default()` | — | `Alchemist.bytes`, `Alchemist1.bytes`, `Alchemist2.bytes` |
| Diviner | Migrated | `src/content/heroes/diviner.rs` | `HeroFamilyRegistry` | `ContentPack::default()` | — | `Diviner.bytes`, `Diviner1.bytes`, `Diviner2.bytes` |
| Hunter | Migrated | `src/content/heroes/hunter.rs` | `HeroFamilyRegistry` | `ContentPack::default()` | — | `Hunter.bytes`, `Hunter1.bytes`, `Hunter2.bytes` |
| Shaman | Migrated | `src/content/heroes/shaman.rs` | `HeroFamilyRegistry` | `ContentPack::default()` | — | `Shaman.bytes`, `Shaman1.bytes`, `Shaman2.bytes` |
| Tank | Migrated | `src/content/heroes/tank.rs` | `HeroFamilyRegistry` | `ContentPack::default()` | — | `Tank.bytes`, `Tank1.bytes`, `Tank2.bytes` |

**Hero Summary:** 5 migrated, 0 partial, 0 missing

## Monster Families

### Common Monsters

| Family | Dungeon | Tier | Status | Parser Module | Registry | Next Target | Source Reference |
|---|---|---|---|---|---|---|---|
| mantis_magic_flower | QingLong | Common | Migrated | `src/content/monsters/mantis_magic_flower.rs` | `MonsterFamilyRegistry` | — | `mantis_magic_flower_1.txt` |
| mantis_spiny_flower | QingLong | Common | Migrated | `src/content/monsters/mantis_spiny_flower.rs` | `MonsterFamilyRegistry` | — | `mantis_spiny_flower_1.txt` |
| mantis_walking_flower | QingLong | Common | Migrated | `src/content/monsters/mantis_walking_flower.rs` | `MonsterFamilyRegistry` | — | `mantis_walking_flower_1.txt` |
| dry_tree_genie | QingLong | Common | Migrated | `src/content/monsters/dry_tree_genie.rs` | `MonsterFamilyRegistry` | — | `dry_tree_genie_1.txt` |
| moth_mimicry_A | QingLong | Common | Migrated | `src/content/monsters/moth_mimicry_a.rs` | `MonsterFamilyRegistry` | — | `moth_mimicry_A_1.txt` |
| moth_mimicry_B | QingLong | Common | Migrated | `src/content/monsters/moth_mimicry_b.rs` | `MonsterFamilyRegistry` | — | `moth_mimicry_B_1.txt` |
| robber_melee | QingLong | Common | Migrated | `src/content/monsters/robber_melee.rs` | `MonsterFamilyRegistry` | — | `robber_melee.txt` |
| robber_ranged | QingLong | Common | Migrated | `src/content/monsters/robber_ranged.rs` | `MonsterFamilyRegistry` | — | `robber_ranged.txt` |
| metal_armor | BaiHu | Common | Migrated | `src/content/monsters/metal_armor.rs` | `MonsterFamilyRegistry` | — | `metal_armor_1.txt` |
| tiger_sword | BaiHu | Common | Migrated | `src/content/monsters/tiger_sword.rs` | `MonsterFamilyRegistry` | — | `tiger_sword_1.txt` |
| lizard | BaiHu | Common | Migrated | `src/content/monsters/lizard.rs` | `MonsterFamilyRegistry` | — | `lizard_1.txt` |
| unicorn_beetle_A | BaiHu | Common | Migrated | `src/content/monsters/unicorn_beetle_a.rs` | `MonsterFamilyRegistry` | — | `unicorn_beetle_A_1.txt` |
| unicorn_beetle_B | BaiHu | Common | Migrated | `src/content/monsters/unicorn_beetle_b.rs` | `MonsterFamilyRegistry` | — | `unicorn_beetle_B_1.txt` |
| alligator_yangtze | BaiHu | Common | Migrated | `src/content/monsters/alligator_yangtze.rs` | `MonsterFamilyRegistry` | — | `alligator_yangtze_1.txt` |
| ghost_fire_assist | ZhuQue | Common | Migrated | `src/content/monsters/ghost_fire_assist.rs` | `MonsterFamilyRegistry` | — | `ghost_fire_assist_1.txt` |
| ghost_fire_damage | ZhuQue | Common | Migrated | `src/content/monsters/ghost_fire_damage.rs` | `MonsterFamilyRegistry` | — | `ghost_fire_damage_1.txt` |
| fox_fire | ZhuQue | Common | Migrated | `src/content/monsters/fox_fire.rs` | `MonsterFamilyRegistry` | — | `fox_fire_1.txt` |
| moth_fire | ZhuQue | Common | Migrated | `src/content/monsters/moth_fire.rs` | `MonsterFamilyRegistry` | — | `moth_fire_1.txt` |
| lantern | ZhuQue | Common | Migrated | `src/content/monsters/lantern.rs` | `MonsterFamilyRegistry` | — | `lantern_1.txt` |
| snake_water | XuanWu | Common | Migrated | `src/content/monsters/snake_water.rs` | `MonsterFamilyRegistry` | — | `snake_water_1.txt` |
| water_grass | XuanWu | Common | Migrated | `src/content/monsters/water_grass.rs` | `MonsterFamilyRegistry` | — | `water_grass_1.txt` |
| monkey_water | XuanWu | Common | Migrated | `src/content/monsters/monkey_water.rs` | `MonsterFamilyRegistry` | — | `monkey_water_1.txt` |

### Boss Monsters

| Family | Dungeon | Tier | Status | Parser Module | Registry | Next Target | Source Reference |
|---|---|---|---|---|---|---|---|
| azure_dragon | QingLong | Boss | Migrated | `src/content/monsters/azure_dragon.rs` | `MonsterFamilyRegistry` | — | `azure_dragon.txt` |
| azure_dragon_ball_thunder | QingLong | Boss | Migrated | `src/content/monsters/azure_dragon_ball_thunder.rs` | `MonsterFamilyRegistry` | — | `azure_dragon_ball_thunder.txt` |
| azure_dragon_ball_wind | QingLong | Boss | Migrated | `src/content/monsters/azure_dragon_ball_wind.rs` | `MonsterFamilyRegistry` | — | `azure_dragon_ball_wind.txt` |
| vermilion_bird | ZhuQue | Boss | Migrated | `src/content/monsters/vermilion_bird.rs` | `MonsterFamilyRegistry` | — | `vermilion_bird.txt` |
| vermilion_bird_tail_A | ZhuQue | Boss | Migrated | `src/content/monsters/vermilion_bird_tail_a.rs` | `MonsterFamilyRegistry` | — | `vermilion_bird_tail_A.txt` |
| vermilion_bird_tail_B | ZhuQue | Boss | Migrated | `src/content/monsters/vermilion_bird_tail_b.rs` | `MonsterFamilyRegistry` | — | `vermilion_bird_tail_B.txt` |
| white_tiger_C | BaiHu | Boss | Migrated | `src/content/monsters/white_tiger_c.rs` | `MonsterFamilyRegistry` | — | `white_tiger_C.txt` |
| white_tiger_A | BaiHu | Boss | Migrated | `src/content/monsters/white_tiger_a.rs` | `MonsterFamilyRegistry` | — | `white_tiger_A.txt` |
| white_tiger_B | BaiHu | Boss | Migrated | `src/content/monsters/white_tiger_b.rs` | `MonsterFamilyRegistry` | — | `white_tiger_B.txt` |
| white_tiger_terrain | BaiHu | Boss | Migrated | `src/content/monsters/white_tiger_terrain.rs` | `MonsterFamilyRegistry` | — | `white_tiger_terrain.txt` |
| black_tortoise_A | XuanWu | Boss | Migrated | `src/content/monsters/black_tortoise_a.rs` | `MonsterFamilyRegistry` | — | `black_tortoise_A.txt` |
| black_tortoise_B | XuanWu | Boss | Migrated | `src/content/monsters/black_tortoise_b.rs` | `MonsterFamilyRegistry` | — | `black_tortoise_B.txt` |
| rotvine_wraith | XuanWu | Boss | Migrated | `src/content/monsters/rotvine_wraith.rs` | `MonsterFamilyRegistry` | — | `rotvine_wraith.txt` |
| rotten_fruit_A | XuanWu | Boss | Migrated | `src/content/monsters/rotten_fruit_a.rs` | `MonsterFamilyRegistry` | — | `rotten_fruit_A.txt` |
| rotten_fruit_B | XuanWu | Boss | Migrated | `src/content/monsters/rotten_fruit_b.rs` | `MonsterFamilyRegistry` | — | `rotten_fruit_B.txt` |
| rotten_fruit_C | XuanWu | Boss | Migrated | `src/content/monsters/rotten_fruit_c.rs` | `MonsterFamilyRegistry` | — | `rotten_fruit_C.txt` |
| skeletal_tiller | XuanWu | Boss | Migrated | `src/content/monsters/skeletal_tiller.rs` | `MonsterFamilyRegistry` | — | `skeletal_tiller.txt` |
| vegetable | XuanWu | Boss | Migrated | `src/content/monsters/vegetable.rs` | `MonsterFamilyRegistry` | — | `vegetable.txt` |
| necrodrake_embryosac | XuanWu | Boss | Migrated | `src/content/monsters/necrodrake_embryosac.rs` | `MonsterFamilyRegistry` | — | `necrodrake_embryosac.txt` |
| egg_membrane_empty | XuanWu | Boss | Migrated | `src/content/monsters/egg_membrane_empty.rs` | `MonsterFamilyRegistry` | — | `egg_membrane_empty.txt` |
| egg_membrane_full | XuanWu | Boss | Migrated | `src/content/monsters/egg_membrane_full.rs` | `MonsterFamilyRegistry` | — | `egg_membrane_full.txt` |
| gambler | ZhuQue | Boss | Migrated | `src/content/monsters/gambler.rs` | `MonsterFamilyRegistry` | — | `gambler.txt` |
| mahjong_red | ZhuQue | Boss | Migrated | `src/content/monsters/mahjong_red.rs` | `MonsterFamilyRegistry` | — | `mahjong_red.txt` |
| mahjong_green | ZhuQue | Boss | Migrated | `src/content/monsters/mahjong_green.rs` | `MonsterFamilyRegistry` | — | `mahjong_green.txt` |
| mahjong_white | ZhuQue | Boss | Migrated | `src/content/monsters/mahjong_white.rs` | `MonsterFamilyRegistry` | — | `mahjong_white.txt` |
| scorchthroat_chanteuse | XuanWu | Boss | Migrated | `src/content/monsters/scorchthroat_chanteuse.rs` | `MonsterFamilyRegistry` | — | `scorchthroat_chanteuse.txt` |
| sc_blow | XuanWu | Boss | Migrated | `src/content/monsters/sc_blow.rs` | `MonsterFamilyRegistry` | — | `sc_blow.txt` |
| sc_bow | XuanWu | Boss | Migrated | `src/content/monsters/sc_bow.rs` | `MonsterFamilyRegistry` | — | `sc_bow.txt` |
| sc_pluck | XuanWu | Boss | Migrated | `src/content/monsters/sc_pluck.rs` | `MonsterFamilyRegistry` | — | `sc_pluck.txt` |
| frostvein_clam | XuanWu | Boss | Migrated | `src/content/monsters/frostvein_clam.rs` | `MonsterFamilyRegistry` | — | `frostvein_clam.txt` |
| pearlkin_opalescent | XuanWu | Boss | Migrated | `src/content/monsters/pearlkin_opalescent.rs` | `MonsterFamilyRegistry` | — | `pearlkin_opalescent.txt` |
| pearlkin_flawed | XuanWu | Boss | Migrated | `src/content/monsters/pearlkin_flawed.rs` | `MonsterFamilyRegistry` | — | `pearlkin_flawed.txt` |
| bloodthirsty_assassin | Cross | Boss | Migrated | `src/content/monsters/bloodthirsty_assassin.rs` | `MonsterFamilyRegistry` | — | `bloodthirsty_assassin.txt` |
| bloodthirsty_shadow | Cross | Boss | Migrated | `src/content/monsters/bloodthirsty_shadow.rs` | `MonsterFamilyRegistry` | — | `bloodthirsty_shadow.txt` |
| glutton_pawnshop | Cross | Boss | Migrated | `src/content/monsters/glutton_pawnshop.rs` | `MonsterFamilyRegistry` | — | `glutton_pawnshop.txt` |

**Monster Summary:** 52 migrated, 0 partial, 0 missing

## Summary

| Category | Migrated | Partial | Missing |
|---|---|---|---|
| Hero Families | 5 | 0 | 0 |
| Monster Families | 52 | 0 | 0 |
| **Total** | **57** | **0** | **0** |

All DDGC asset families are fully migrated. No partial or missing families remain.

## Maintenance

This document is automatically derivable from the CLI tool in `src/cli.rs::generate_asset_audit()`.
Run `cargo run -- generate-asset-audit` to regenerate the table programmatically.
The CLI tool cross-references `HeroFamilyRegistry::all_families()` and
`monsters::build_registry()` to produce the authoritative status for each family.