# Skills Inventory Audit

## Methodology

This audit cross-references three sources:
1. **Migrated skills**: `src/content/skills.rs` — currently defined skill functions
2. **Monster registry skill references**: `src/monsters/mod.rs` — all `SkillId::new(...)` calls
3. **Original game data**: `DreamDeveloperGame-Crossover/Assets/Resources/Data/JsonCamping.json` and C# skill classes

## 1. Migrated Combat Skills (src/content/skills.rs)

| # | Skill ID | Source Class/Monster | Effects | Target | Cooldown |
|---|----------|---------------------|---------|--------|----------|
| 1 | `crusading_strike` | Crusader | damage 12 | AllEnemies | none |
| 2 | `holy_lance` | Crusader | damage 9 + heal 3 | AllEnemies | 2 turns |
| 3 | `divine_grace` | Vestal | heal 10 | AllAllies | none |
| 4 | `rend` | Bone Soldier | damage 6 + bleed 3 | AllEnemies | none |
| 5 | `skull_bash` | Necromancer | damage 14 + stun 60% | AllEnemies | 3 turns |
| 6 | `grave_bash` | Necromancer | damage 5 + damage 5 | AllEnemies | none |

**Total migrated: 6 combat skills**

## 2. Combat Skills Referenced by Monster Registry

The monster registry references **164 unique skill IDs**. These are distributed across:
- `src/content/skills.rs` — hero and legacy boss skills (6 skills)
- `src/content/monsters/` — monster family skill modules (158 skills across all families)

### Common Monster Skills — MIGRATED

All 22 common monster families have been migrated with their identity skills defined
in the Rust content layer via monster family modules (e.g., `mantis_magic_flower.rs`).
These skills are registered in `ContentPack` via `monsters::register_content()`.

#### QingLong (8 families)
| Family | Identity Skills | DDGC Source |
|--------|----------------|-------------|
| mantis_magic_flower | `poison`, `crowd_bleed` | dmg 20-28 + blight, dmg 10-14 + bleed |
| mantis_spiny_flower | `ignore_armor`, `crowd_bleed` | dmg 15-25 + armor ignore, dmg 10-14 + bleed |
| mantis_walking_flower | `weak`, `crowd_bleed` | dmg 10-14 + weak, dmg 10-14 + bleed |
| dry_tree_genie | `bleed`, `slow_crowd`, `stress` | dmg 8-12 + bleed, AoE slow, stress attack |
| moth_mimicry_a | `poison`, `stress_poison` | blight + stress DOT |
| moth_mimicry_b | `poison`, `stress_crowd` | blight + stress AoE |
| robber_melee | `smoke_bomb`, `bleed` | back attack + bleed |
| robber_ranged | `throw_stone`, `multiple_shot` | ranged + multi-hit |

#### BaiHu (6 families)
| Family | Identity Skills | DDGC Source |
|--------|----------------|-------------|
| metal_armor | `stun`, `bleed` | stun + bleed |
| tiger_sword | `normal_attack`, `pull` | dmg 25-35, pull forward |
| lizard | `stun`, `intimidate`, `stress` | stun + stress |
| unicorn_beetle_a | `bleed`, `bleed_crowd` | single + AoE bleed |
| unicorn_beetle_b | `bleed`, `stress` | bleed + stress |
| alligator_yangtze | `bleed`, `mark_riposte` | bleed + riposte setup |

#### ZhuQue (5 families)
| Family | Identity Skills | DDGC Source |
|--------|----------------|-------------|
| ghost_fire_assist | `assist`, `ghost_fire_split` | ally buff + split damage |
| ghost_fire_damage | `burn_attack`, `ghost_fire_split` | burn + split |
| fox_fire | `bite`, `protect` | melee + guard |
| moth_fire | `cocoon`, `fly_into_fire` | stun cocoon + fire transition |
| lantern | `stress`, `burn_attack` | stress + burn |

#### XuanWu (3 families)
| Family | Identity Skills | DDGC Source |
|--------|----------------|-------------|
| snake_water | `stun`, `poison_fang` | stun + poison |
| water_grass | `stun`, `puncture`, `convolve` | multi-effect controller |
| monkey_water | `rush`, `stress` | charge + stress |

### Boss Skills — MIGRATED

All boss skill families have been migrated with skills defined in monster family modules
and registered in `ContentPack` via `monsters::register_content()`. The integration tests
`all_boss_family_skills_exist_in_content_pack` and `all_boss_family_skills_validate_for_encounter_use`
verify all boss skills are present and valid.

#### QingLong (Boss — Azure Dragon)
| Skill ID | DDGC Source | Effects | Game-Gap |
|----------|-------------|---------|----------|
| `bloodscale_reaping` | dmg 2-4 + bleed | damage + bleed | position-based targeting not modeled |
| `dragonfear_crash` | dmg 1-2 + stun | damage + stun | position-based targeting not modeled |
| `summit_relocation` | dmg 1-2 + pull | damage + pull | position-based targeting not modeled |
| `soulfog_enthrall` | dmg 1-3 + stress | damage + stress | position-based targeting not modeled |
| `dragoncry_storm` | AoE 1-2 | AoE damage | position-based targeting not modeled |
| `volt_tyranny` | dmg 3-6 + crit buff | damage + crit | ball-swap AI not modeled |
| `voltic_baptism` | thunder buff | buff | random buff tables not modeled |
| `capricious_skies` | wind buff | buff | random buff tables not modeled |
| `swap_dragon_ball` | summon wind | summon | ball swap as status marker only |
| `swap_dragon_ball_other` | summon thunder | summon | ball swap as status marker only |
| `swap_dragon_ball_summon` | summon wind/thunder | summon | ball-type-dependent skill selection not modeled |
| `thunder_buff_magic` | magic buff | buff | ball-type-dependent not modeled |
| `thunder_buff_stress` | stress buff | buff | ball-type-dependent not modeled |
| `thunder_stress_attack` | stress attack | stress | ball-type-dependent not modeled |
| `wind_buff_acc` | accuracy buff | buff | ball-type-dependent not modeled |
| `wind_shuffle` | shuffle | shuffle | ball-type-dependent not modeled |
| `wind_buff_physic` | phys buff | buff | ball-type-dependent not modeled |

#### ZhuQue (Boss — Vermilion Bird)
| Skill ID | DDGC Source | Effects | Game-Gap |
|----------|-------------|---------|----------|
| `singing_loudly` | dmg 2-5 + stun | damage + stun | position-based targeting not modeled |
| `ruin` | burn 2 | burn DOT | phase-dependent AI not modeled |
| `ruin1` | burn 3 | burn DOT | phase-dependent AI not modeled |
| `precise_pecking` | dmg 3-5 + suck_hp | damage + heal | life steal not modeled |
| `iron_feather` | dmg 1-3 | damage | position-based targeting not modeled |
| `bide` | dmg 2-4 | damage | self-movement not modeled |
| `calm_nerves` | heal + absorb | heal | PB charge mechanic not modeled |
| `explosion` | massive heal | heal | PB charge mechanic not modeled |
| `deterrence` | stun | stun | tail part not modeled |
| `confused` | stress | stress | tail part not modeled |
| `ignore_def` | armor ignore | ignore armor | tail part not modeled |
| `follow` | stress | stress | tail behavior not modeled |
| `follow1` | stress | stress | tail behavior not modeled |
| `run_water` | dmg 2-4 | damage | tail part not modeled |
| `run_water1` | dmg 2-4 | damage | tail part not modeled |
| `heaven_falls` | AoE 2-4 | AoE damage | tail part not modeled |
| `heaven_falls1` | AoE 3-5 | AoE damage | tail part not modeled |
| `iron_feather_with` | buff | buff | PB-related buff not modeled |

#### ZhuQue (Boss — Gambler)
| Skill ID | DDGC Source | Effects | Game-Gap |
|----------|-------------|---------|----------|
| `dice_thousand` | chaos attack | chaos | chaos system not modeled |
| `hollow_victory` | stress | stress | tag behavior not modeled |
| `card_doomsday` | AoE stress | AoE stress | tag mechanic not modeled |
| `jackpot_requiem` | damage | damage | random tile selection not modeled |
| `summon_mahjong` | summon | summon | summon materialization not wired |
| `lucky_charity` | heal | heal | gambling mechanic not modeled |
| `fortune_ante` | buff | buff | ante mechanic not modeled |
| `fa_cai_blessing` | buff | buff | prosperity mechanic not modeled |
| `high_roller` | damage + stress | damage + stress | luck mechanic not modeled |
| `joyful_bonus` | buff | buff | bonus mechanic not modeled |
| `triple_tile_invite` | summon | summon | triple tile mechanic not modeled |

#### BaiHu (Boss — White Tiger)
| Skill ID | DDGC Source | Effects | Game-Gap |
|----------|-------------|---------|----------|
| `drag` | dmg 6-8 + pull | damage + pull | position-based targeting not modeled |
| `angry_eyes` | stress 2 | stress | position-based targeting not modeled |
| `pounce` | dmg 4-6 | damage | self-movement not modeled |
| `pounce_bite` | dmg 4-6 + bleed | damage + bleed | self-movement not modeled |
| `jump` | dmg 5-7 | damage | self-movement not modeled |
| `thunder_lightning` | dmg 3-5 + stun | damage + stun | multi-phase not modeled |
| `paw` | dmg 5-7 | damage | clone-phase not modeled |
| `raging_fire` | burn 2 | burn | clone-phase not modeled |
| `true_strike` | dmg 7-9 | damage | clone-phase not modeled |
| `deter_stress` | stress | stress | clone-phase not modeled |
| `deter_def` | debuff | debuff | clone-phase not modeled |
| `allow_return` | buff | buff | clone behavior not modeled |
| `fire_soul_shadow` | damage | damage | clone behavior not modeled |
| `tiger_swing` | damage | damage | final-form not modeled |
| `thunder_shadow` | damage + stun | damage + stun | final-form not modeled |
| `occupy` | buff | buff | final-form not modeled |

#### XuanWu (Boss — Black Tortoise)
| Skill ID | DDGC Source | Effects | Game-Gap |
|----------|-------------|---------|----------|
| `tortoise_call_roll` | summon | summon | call roll mechanic not modeled |
| `tortoise_rain_spray` | damage | damage | ball-type-dependent not modeled |
| `ice_spike` | damage + freeze | damage + freeze | frozen status not modeled |
| `hunger_cold` | stress | stress | snake/tortoise split not modeled |
| `inner_battle` | damage | damage | tortoise AI not modeled |
| `near_mountain_river` | damage | damage | shared damage not modeled |
| `hunger_cold_1` | stress | stress | snake AI not modeled |
| `unexpectedly` | damage | damage | snake AI not modeled |
| `snake_call_roll` | summon | summon | call roll mechanic not modeled |
| `snake_rain_spray` | damage | damage | ball-type-dependent not modeled |
| `freezing_cold` | freeze | freeze | frozen status not modeled |
| `benefits_stress` | stress | stress | snake AI not modeled |
| `fangs_sprayed` | damage | damage | snake AI not modeled |
| `fangs_sprayed_1` | damage | damage | snake AI not modeled |
| `snake_bites` | damage + poison | damage + poison | snake AI not modeled |
| `armor` | buff | buff | tortoise AI not modeled |

#### XuanWu (Boss — Rotvine Wraith)
| Skill ID | DDGC Source | Effects | Game-Gap |
|----------|-------------|---------|----------|
| `cadaver_bloom` | AoE damage | AoE damage | summon not materialized |
| `rotvine_snare` | debuff | debuff | rotvine AI not modeled |
| `sepsis_strangulate` | damage + poison | damage + poison | rotvine AI not modeled |
| `telluric_resurrect` | summon | summon | resummon not modeled |
| `carrion_sowing` | summon | summon | phase-dependent not modeled |

#### XuanWu (Boss — Skeletal Tiller)
| Skill ID | DDGC Source | Effects | Game-Gap |
|----------|-------------|---------|----------|
| `bone_reforge` | damage | damage | tiller AI not modeled |
| `famine_reaping` | damage | damage | tiller AI not modeled |
| `scarecrow_shriek` | stress | stress | tiller AI not modeled |
| `grave_tug` | pull | pull | tiller AI not modeled |
| `tiller_crop_rot_claw` | damage | damage | tiller AI not modeled |

#### XuanWu (Boss — Necrodrake Embryosac)
| Skill ID | DDGC Source | Effects | Game-Gap |
|----------|-------------|---------|----------|
| `ecdysis_metamorphosis` | transform | transform | phase transition not modeled |
| `requiem_stillbirth` | damage | damage | embryosac AI not modeled |
| `placental_tap` | life drain | life drain | life drain not modeled |
| `untimely_progeny` | summon | summon | egg behavior not modeled |
| `doom_symbiosis` | debuff | debuff | shared HP not modeled |

#### XuanWu (Boss — Egg Membranes)
| Skill ID | DDGC Source | Effects | Game-Gap |
|----------|-------------|---------|----------|
| `captor_empty` | capture | capture | captor/release not modeled |
| `captor_full` | capture | capture | captor/release not modeled |

#### XuanWu (Boss — Rotten Fruit)
| Skill ID | DDGC Source | Effects | Game-Gap |
|----------|-------------|---------|----------|
| `absorbed` | buff | buff | absorption not modeled |
| `fruit_explosion` | AoE damage | AoE damage | fruit AI not modeled |
| `fruit_stress_explosion` | AoE stress | AoE stress | fruit AI not modeled |

#### XuanWu (Boss — Vegetable)
| Skill ID | DDGC Source | Effects | Game-Gap |
|----------|-------------|---------|----------|
| `briar_intimidation` | stress | stress | vegetable AI not modeled |
| `vegetable_crop_rot_claw` | damage | damage | vegetable AI not modeled |
| `vegetable_move` | move | move | self-movement not modeled |

#### XuanWu (Boss — Scorchthroat Chanteuse)
| Skill ID | DDGC Source | Effects | Game-Gap |
|----------|-------------|---------|----------|
| `cremona_last_chord` | damage | damage | scorchthroat AI not modeled |
| `pyre_resonance` | burn | burn | scorchthroat AI not modeled |
| `ashen_communion` | buff | buff | scorchthroat AI not modeled |
| `encore_embers` | damage | damage | encore mechanic not modeled |
| `grindbone_lament` | damage | damage | SC Blow AI not modeled |
| `crematorium_bowstring` | damage | damage | SC Bow AI not modeled |
| `ossein_arsonist_lyre` | damage | damage | SC Pluck AI not modeled |

#### XuanWu (Boss — Frostvein Clam)
| Skill ID | DDGC Source | Effects | Game-Gap |
|----------|-------------|---------|----------|
| `glacial_torrent` | damage | damage | clam AI not modeled |
| `abyssal_glare` | debuff | debuff | clam AI not modeled |
| `nacreous_homunculus` | summon | summon | pearlkin not materialized |
| `prismatic_clench` | damage + guard | damage + guard | pearlkin AI not modeled |
| `clam_riposte` | riposte | riposte | pearlkin riposte not modeled |
| `po_debuff` | debuff | debuff | pearlkin AI not modeled |
| `po_damage` | damage | damage | pearlkin AI not modeled |
| `fracture_lure` | damage | damage | flawed pearlkin not modeled |
| `shattered_revelation` | damage | damage | flawed pearlkin not modeled |

#### Cross-Dungeon (Boss — Bloodthirsty Assassin)
| Skill ID | DDGC Source | Effects | Game-Gap |
|----------|-------------|---------|----------|
| `bloodstrike_ambush` | damage | damage | ambush mechanic not modeled |
| `phantom_lunge` | damage | damage | shadow pair not modeled |
| `crimson_duet` | damage | damage | HP averaging not modeled |
| `scarlet_guillotine` | damage | damage | paired boss not modeled |

#### Cross-Dungeon (Boss — Glutton Pawnshop)
| Skill ID | DDGC Source | Effects | Game-Gap |
|----------|-------------|---------|----------|
| `haemogorging_aura` | debuff | debuff | aura mechanic not modeled |
| `phantom_resonance` | damage | damage | tag cycling not modeled |
| `umbral_cyclone` | damage | damage | tag mechanic not modeled |
| `flesh_usury_contract` | debuff | debuff | contract mechanic not modeled |
| `compound_agony` | tag | tag | tag mechanic not modeled |
| `invitation` | summon | summon | summon not modeled |
| `foreclosed_wail` | stress | stress | debuff cycling not modeled |

**Note:** All migrated boss skills map to existing runtime capabilities (damage, apply_status, pull, heal, summon). Remaining gaps are documented game-gaps per `MIGRATION_BLOCKERS.md`.

### Gap Summary: Combat Skills

| Category | Count | Status |
|----------|-------|--------|
| Hero skills (Crusader, Vestal, etc.) | 2 classes × ~7 skills = ~14 | **6 migrated** (in skills.rs) |
| Common monster skills | ~40 unique IDs | **~40 migrated** (in monster family modules) |
| Boss skills (4 dungeons + cross) | ~110 unique IDs | **~110 migrated** (in monster family modules) |
| **Total unique IDs referenced** | **~164** | **~156 migrated** |

> Note: Boss combat skill gap is **CLOSED** (US-010-a/US-010-b).
> Boss skills are migrated in monster family modules (azure_dragon, vermilion_bird,
> white_tiger_*, black_tortoise_*, rotvine_wraith, skeletal_tiller, necrodrake_embryosac,
> gambler, scorchthroat_chanteuse, frostvein_clam, bloodthirsty_assassin, glutton_pawnshop,
> and their minion/part units). All skills validate and participate in encounter resolution
> via `all_boss_family_skills_exist_in_content_pack` and `all_boss_family_skills_validate_for_encounter_use`.

## 3. Camping Skills (Original Game)

Source: `JsonCamping.json`

| Category | Count | Migrated? |
|----------|-------|-----------|
| Shared (all classes) | 4 (`encourage`, `first_aid`, `pep_talk`, `hobby`) | **NO** |
| Arbalest/Musketeer | 5 (`field_dressing`, `marching_plan`, `restring_crossbow`, `clean_musket`, `triage`) | **NO** |
| Bounty Hunter | 4 (`how_its_done`, `tracking`, `planned_takedown`, `scout_ahead`) | **NO** |
| Crusader | ~3 | **NO** |
| Hellion | ~3 | **NO** |
| Highwayman | ~3 | **NO** |
| Jester | ~4 | **NO** |
| Leper | ~3 | **NO** |
| Man-at-Arms | ~4 | **NO** |
| Occultist | ~4 | **NO** |
| Plague Doctor | ~3 | **NO** |
| Vestal | ~3 | **NO** |
| Grave Robber | ~3 | **NO** |
| Houndmaster | ~3 | **NO** |
| Abomination | ~3 | **NO** |
| Antiquarian | ~4 | **NO** |
| **Total** | **87** | **0 migrated** |

## 4. Framework Effect Type Coverage

Original DDGC `EffectSubType` enum has 60+ entries. The migrated skills use only:
- `Damage` (via `EffectNode::damage`)
- `Heal` (via `EffectNode::heal`)
- `ApplyStatus` (via `EffectNode::apply_status`) — used for "bleed", "stun"

### Missing EffectSubTypes Needed for Referenced Skills

From `MechanicsDefines.cs`, these sub-types are referenced by the 158 missing
skills and require framework or game-layer support:

| EffectSubType | Example Skills | Framework Support? |
|---------------|---------------|-------------------|
| `Stress` | `stress`, `stress_attack`, `stress_crowd`, `stress_poison` | Partial (`stress_change` in rewards only) |
| `StressHeal` | `calm_nerves` | No |
| `Buff` / `StatBuff` | `buff_self`, `protect`, `armor`, `assist` | Partial (buff system exists?) |
| `Debuff` | `po_debuff`, `weak`, `slow_crowd` | Unknown |
| `Stun` | `stun` | Yes (`apply_status("stun")`) |
| `Frozen` | `freezing_cold`, `ice_spike` | No |
| `Poison` / `Bleed` / `Burning` | `poison`, `bleed`, `burn_attack` | Partial (status names exist) |
| `Heal` / `Cure` | various boss heals | Yes (`EffectNode::heal`) |
| `LifeDrain` / `SuckHp` | `placental_tap` | No |
| `Pull` / `Push` | `pull`, `drag` | No |
| `Summon` | `summon_mahjong`, `nacreous_homunculus` | No |
| `Shuffle` | `wind_shuffle` | No |
| `GuardAlly` / `ClearGuard` | `protect` | No |
| `Riposte` | `mark_riposte`, `riposte1`, `clam_riposte` | No |
| `Tag` / `Untag` | `compound_agony` | No |
| `Mode` | `ruin` / `ruin1` pairs | No |
| `Capture` | `captor_empty` / `captor_full` | No |
| `ApplyEffects` | `buff_self` | Unknown |
| `Chaos` / `ChaosHeal` | `dice_thousand` | No |
| `AverageHp` | `crimson_duet` | No |
| `CallRoll` | `tortoise_call_roll`, `snake_call_roll` | No |
| `VermilionBirdPb` | `iron_feather_with` | No (boss-specific) |
| `AzureDragonBallSwap` | `swap_dragon_ball*` | No (boss-specific) |
| `AzureDragonBallActiveBuff` | `thunder_buff_magic` | No (boss-specific) |
| `ShadowSwordTest` / `NextShadowSwordDamage` | various | No (boss-specific) |
| `DivinerTalentEffect` / `AskGod` / `DivineBad` | various | No (boss-specific) |
| `TankTalentEffect` | various | No |
| `Purge` | various | No |
| `CondiEffect` | various | No |
| `Damp` | various | No |
| `ShareDamage` | `near_mountain_river` | No |
| `ClearTargetRanks` | various | No |
| `Rank` | various | No |
| `Control` | various | No |
| `Immobilize` | various | No |
| `Kill` | various | No |

## 5. Recommended Migration Priority

### Phase 1: Core Combat Skills (Monster Blocking) — CLOSED

Migrate the ~30 common monster skills first so that the 18 common monster
families (K4-K8, K10-K11, K12-K16, K17, K18-K19, K20-K21, K22, K23-K26)
have functional combat logic:

1. `normal_attack` — basic damage (all families)
2. `move` — reposition (most families)
3. `poison`, `bleed`, `burn_attack` — DOT statuses
4. `stress`, `stress_attack`, `stress_crowd`, `stress_poison` — stress damage
5. `stun` — stun status
6. `pull`, `drag` — position manipulation
7. `buff_self`, `protect`, `armor` — self buffs
8. `assist` — ally buff
9. `smoke_bomb`, `throw_stone`, `multiple_shot` — robber skills
10. `ignore_armor`, `weak`, `slow_crowd` — debuffs
11. `intimidate`, `puncture`, `attack_crowd`, `convolve`
12. `base_melee`, `rush`, `mark_riposte`, `riposte1`
13. `bite`, `vomit`, `cocoon`, `fly_into_fire`
14. `ghost_fire_split`

**Status:** CLOSED (US-009-a/US-009-b/US-009-c)

### Phase 2: Boss Skills (By Dungeon) — CLOSED

Migrate boss skills dungeon-by-dungeon as boss families are implemented:
- QingLong: Azure Dragon (11 skills) + Dragon Balls (6 skills)
- ZhuQue: Vermilion Bird (8 + 4 + 7 = 19 skills) + Gambler (11 skills)
- BaiHu: White Tiger (7 + 5 + 5 + 1 = 18 skills)
- XuanWu: Black Tortoise (8 + 8 = 16 skills) + Rotvine (6 skills) +
  Skeletal Tiller (5 skills) + Necrodrake (5 skills) + Scorchthroat (11 skills) +
  Frostvein Clam (8 skills)
- Cross: Bloodthirsty Assassin (4 + 3 = 7 skills) + Glutton Pawnshop (4 skills)

**Status:** CLOSED (US-010-a/US-010-b)

### Phase 3: Hero Combat Skills — PENDING

Migrate all hero class combat skills (not yet referenced by monster registry
but needed for player party). Source: original `CombatSkill.cs` + hero data.

### Phase 4: Camping Skills — PENDING

Migrate all 87 camping skills as part of the Camping System task.

## 6. Summary Table

| Skill Category | Total in Original | Migrated | Missing | Priority |
|----------------|------------------|----------|---------|----------|
| Combat — Hero | ~60+ | 6 | ~54+ | P3 |
| Combat — Common Monster | ~40 | **~40** | 0 | **CLOSED** |
| Combat — Boss | ~110 | **~110** | 0 | **CLOSED** |
| Camping | 87 | 0 | 87 | P4 |
| **TOTAL** | **~290** | **~156** | **~134** | — |

> Note: Common monster combat skill gap is **CLOSED** (US-009-a/US-009-b/US-009-c).
> Boss combat skill gap is **CLOSED** (US-010-a/US-010-b).
> Hero skills are in `src/content/skills.rs`. Common monster skills are in
> `src/content/monsters/<family>.rs` and registered via `monsters::register_content()`.
> Boss skills are in `src/content/monsters/<boss_family>.rs` and validated via
> `all_boss_family_skills_exist_in_content_pack` and `all_boss_family_skills_validate_for_encounter_use`.
