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

## 2. Combat Skills Referenced by Monster Registry (NOT YET MIGRATED)

The monster registry references **164 unique skill IDs**. Of these 164, only the 6
above are defined in `content/skills.rs`. The remaining **158 are MISSING**.

### Missing Skills by Dungeon

#### QingLong (Common)
- `poison`, `crowd_bleed`, `normal_attack`, `move`, `ignore_armor`, `weak`
- `bleed`, `slow_crowd`, `stress`, `stress_poison`, `stress_crowd`
- `smoke_bomb`, `multiple_shot`, `throw_stone`

#### BaiHu (Common)
- `stun`, `pull`, `intimidate`, `bleed_crowd`
- `mark_riposte`, `riposte1`

#### ZhuQue (Common)
- `assist`, `buff_self`, `ghost_fire_split`, `burn_attack`
- `bite`, `vomit`, `protect`, `stress_attack`, `cocoon`, `fly_into_fire`

#### XuanWu (Common)
- `poison_fang`, `puncture`, `attack_crowd`, `convolve`
- `base_melee`, `rush`

#### QingLong (Boss — Azure Dragon)
- `bloodscale_reaping`, `dragonfear_crash`, `summit_relocation`, `soulfog_enthrall`
- `dragoncry_storm`, `volt_tyranny`, `voltic_baptism`, `capricious_skies`
- `swap_dragon_ball`, `swap_dragon_ball_other`, `swap_dragon_ball_summon`
- `thunder_buff_magic`, `thunder_buff_stress`, `thunder_stress_attack`
- `wind_buff_acc`, `wind_shuffle`, `wind_buff_physic`

#### ZhuQue (Boss — Vermilion Bird)
- `singing_loudly`, `ruin`, `ruin1`, `precise_pecking`, `iron_feather`, `bide`
- `calm_nerves`, `explosion`, `deterrence`, `confused`, `ignore_def`
- `follow`, `follow1`, `run_water`, `run_water1`, `heaven_falls`, `heaven_falls1`
- `iron_feather_with`

#### BaiHu (Boss — White Tiger)
- `thunder_lightning`, `paw`, `raging_fire`, `true_strike`, `jump`
- `deter_stress`, `deter_def`, `drag`, `angry_eyes`, `pounce`, `pounce_bite`
- `allow_return`, `fire_soul_shadow`, `tiger_swing`, `thunder_shadow`
- `occupy`

#### XuanWu (Boss — Black Tortoise)
- `tortoise_call_roll`, `tortoise_rain_spray`, `ice_spike`, `hunger_cold`
- `inner_battle`, `near_mountain_river`, `hunger_cold_1`, `unexpectedly`
- `snake_call_roll`, `snake_rain_spray`, `freezing_cold`, `benefits_stress`
- `fangs_sprayed`, `fangs_sprayed_1`, `snake_bites`, `armor`

#### XuanWu (Boss — Rotvine Wraith)
- `cadaver_bloom`, `rotvine_snare`, `sepsis_strangulate`, `telluric_resurrect`
- `carrion_sowing`

#### XuanWu (Boss — Skeletal Tiller)
- `bone_reforge`, `famine_reaping`, `scarecrow_shriek`, `grave_tug`
- `tiller_crop_rot_claw`

#### XuanWu (Boss — Other)
- `absorbed` (Rotten Fruit A), `fruit_explosion` (Rotten Fruit B)
- `fruit_stress_explosion` (Rotten Fruit C)
- `briar_intimidation`, `vegetable_crop_rot_claw`, `vegetable_move` (Vegetable)
- `requiem_stillbirth`, `placental_tap`, `untimely_progeny`, `doom_symbiosis`
- `ecdysis_metamorphosis` (Necrodrake Embryosac)
- `captor_empty`, `captor_full` (Egg Membranes)

#### ZhuQue (Boss — Gambler)
- `dice_thousand`, `hollow_victory`, `card_doomsday`, `jackpot_requiem`
- `summon_mahjong`, `lucky_charity`, `fortune_ante`, `fa_cai_blessing`
- `high_roller`, `joyful_bonus`, `triple_tile_invite`

#### XuanWu (Boss — Scorchthroat Chanteuse)
- `cremona_last_chord`, `pyre_resonance`, `ashen_communion`, `encore_embers`
- `grindbone_lament` (SC Blow), `crematorium_bowstring` (SC Bow)
- `ossein_arsonist_lyre` (SC Pluck)

#### XuanWu (Boss — Frostvein Clam)
- `glacial_torrent`, `abyssal_glare`, `nacreous_homunculus`, `prismatic_clench`
- `clam_riposte`, `po_debuff`, `po_damage` (Pearlkin Opalescent)
- `fracture_lure`, `shattered_revelation` (Pearlkin Flawed)

#### Cross-Dungeon (Boss)
- `bloodstrike_ambush`, `phantom_lunge`, `crimson_duet`, `scarlet_guillotine`
- `haemogorging_aura`, `phantom_resonance`, `umbral_cyclone`
- `flesh_usury_contract`, `compound_agony`, `invitation`, `foreclosed_wail`

### Gap Summary: Combat Skills

| Category | Count | Status |
|----------|-------|--------|
| Hero skills (Crusader, Vestal, etc.) | 2 classes × ~7 skills = ~14 | **2 migrated** (crusader only) |
| Common monster skills | ~40 unique IDs | **0 migrated** |
| Boss skills (4 dungeons + cross) | ~110 unique IDs | **4 migrated** (necromancer only) |
| **Total unique IDs referenced** | **164** | **6 migrated, 158 missing** |

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

### Phase 1: Core Combat Skills (Monster Blocking)

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

### Phase 2: Boss Skills (By Dungeon)

Migrate boss skills dungeon-by-dungeon as boss families are implemented:
- QingLong: Azure Dragon (11 skills) + Dragon Balls (6 skills)
- ZhuQue: Vermilion Bird (8 + 4 + 7 = 19 skills) + Gambler (11 skills)
- BaiHu: White Tiger (7 + 5 + 5 + 1 = 18 skills)
- XuanWu: Black Tortoise (8 + 8 = 16 skills) + Rotvine (6 skills) +
  Skeletal Tiller (5 skills) + Necrodrake (5 skills) + Scorchthroat (11 skills) +
  Frostvein Clam (8 skills)
- Cross: Bloodthirsty Assassin (4 + 3 = 7 skills) + Glutton Pawnshop (4 skills)

### Phase 3: Hero Combat Skills

Migrate all hero class combat skills (not yet referenced by monster registry
but needed for player party). Source: original `CombatSkill.cs` + hero data.

### Phase 4: Camping Skills

Migrate all 87 camping skills as part of the Camping System task.

## 6. Summary Table

| Skill Category | Total in Original | Migrated | Missing | Priority |
|----------------|------------------|----------|---------|----------|
| Combat — Hero | ~60+ | 2 | ~58+ | P3 |
| Combat — Common Monster | ~30 | 4 | ~26 | **P1** |
| Combat — Boss | ~110 | 0 | ~110 | P2 |
| Camping | 87 | 0 | 87 | P4 |
| **TOTAL** | **~290** | **6** | **~284** | — |

> Note: The 6 migrated skills are: `crusading_strike`, `holy_lance`,
> `divine_grace`, `rend`, `skull_bash`, `grave_bash`.
