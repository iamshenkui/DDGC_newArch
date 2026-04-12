# DDGC Hero Class Families

This document inventories every recruitable hero class family in DDGC, providing the
base/white/black variant mapping derived from `CharacterHelper::GetBaseHeroClassId` and
`CharacterHelper::GetChaosHeroClassId`.

## Variant Mapping Behavior

The base/white/black mapping is defined in `CharacterHelper.cs` (DreamDeveloperGame-Crossover):

- **`GetBaseHeroClassId(classId)`** — strips any trailing digit suffix from `classId` and
  verifies the resulting base ID exists in `DarkestDungeonManager.Data.HeroClasses`. Returns
  the base ID. Example: `"alchemist1"` → `"alchemist"`, `"diviner2"` → `"diviner"`.

- **`GetChaosHeroClassId(classId, chaosMode)`** — resolves base ID, then appends a suffix
  based on `chaosMode.Id`:
  - `"white"` or `"chaos_white"` → appends `"1"` (e.g. `"alchemist1"`)
  - `"black"` or `"chaos_black"` → appends `"2"` (e.g. `"alchemist2"`)
  - Any other mode (including `null`) → returns base ID unchanged

## Chaos Mode Thresholds

Chaos value is stored 0–200 (displayed as −100 to +100). Mode switching thresholds
(`Hero.cs`):

| Chaos stored value | Display value | Mode | Variant suffix |
|---|---|---|---|
| < 50 | < −50 | `chaos_black` | `+2` |
| 50–149 | −50 to +49 | `chaos_normal` | (base, no suffix) |
| >= 150 | >= +50 | `chaos_white` | `+1` |

## Recruitment Semantics

- **StageCoach** recruits only base class IDs (no digit suffix). This is enforced by
  `CharacterHelper.IsBaseRecruitHeroClass(classId)` which returns `true` only when
  `GetBaseHeroClassId(classId) == classId`.
- **Bonus recruits** (`RestockBonus`) normalize variant IDs back to base via
  `CharacterHelper.NormalizeRecruitHeroClassId` before creating the hero.
- **Class requirement matching** (`HeroMatchesClassRequirement`) succeeds when the
  required class ID matches any of: the hero's raw class string, the base class ID, or
  the current chaos variant class ID.

## Hero Class Family Inventory

| Family | Base ID | White ID (+1) | Black ID (+2) | DDGC Data Files | Skills (base) | Migration Status |
|---|---|---|---|---|---|---|
| Alchemist | `alchemist` | `alchemist1` | `alchemist2` | Alchemist.bytes, Alchemist1.bytes, Alchemist2.bytes | heal_multi, heal_single, miss_single, stress_multi, burn_skill, push_skill, push_self | Not started |
| Diviner | `diviner` | `diviner1` | `diviner2` | Diviner.bytes, Diviner1.bytes, Diviner2.bytes | duality_fate, repel, blessed_evasion, pull_skill, survive, draw_stick, karmic_cycle | Not started |
| Hunter | `hunter` | `hunter1` | `hunter2` | Hunter.bytes, Hunter1.bytes, Hunter2.bytes | mark_skill, pull_skill, aoe_skill, stun_skill, ignore_def_skill, bleed_skill, buff_skill | Not started |
| Shaman | `shaman` | `shaman1` | `shaman2` | Shaman.bytes, Shaman1.bytes, Shaman2.bytes | frozen_skill, burn_skill, bleed_skill, direct_hit_1, direct_hit_2, stun_skill, buff_self | Not started |
| Tank | `tank` | `tank1` | `tank2` | Tank.bytes, Tank1.bytes, Tank2.bytes | protect_skill, attack_reduce, taunt_skill, active_riposte, blood_oath, stun_skill, regression | Not started |

## Variant Skill Differences

Each variant shares the same skill IDs as its base family, but with different effect
chains or stat scaling. Notable variant-specific differences observed in DDGC data:

| Family | Variant | Key Differences from Base |
|---|---|---|
| Alchemist | White (+1) | heal_multi has higher heal values per level |
| Alchemist | Black (+2) | heal_multi has additional `"Heal Stress"` effect |
| Diviner | White (+1) | duality_fate has `"Divine Bad Count"` effect |
| Diviner | Black (+2) | duality_fate has `"Ask God Not Trigger Divination"` effect |
| Hunter | White (+1) | mark_skill has additional `"Hunter Anyone Mark"` effect |
| Hunter | Black (+2) | mark_skill has `"Hunter Mark Queue False"` effect |
| Shaman | White (+1) | frozen_skill has additional `"Ch75 Burn"` effect |
| Shaman | Black (+2) | frozen_skill has reduced magic_dmg percentage (-37.5% vs -75%) |
| Tank | White (+1) | protect_skill has additional `"Tank Damage"` effect |
| Tank | Black (+2) | protect_skill has `"Remove Random Dot"` effect |

## Upgrade Trees

Each variant has its own upgrade tree JSON file under
`Assets/Resources/Data/Upgrades/Heroes/`:
- `{base}.upgrades.json` — base variant upgrade paths
- `{base}1.upgrades.json` — white variant upgrade paths
- `{base}2.upgrades.json` — black variant upgrade paths

The `StageCoach.GeneratePurchaseInfo` method registers all three variant upgrade trees
for every hero, ensuring that equipment and skill upgrades follow the current chaos mode.

## Legacy References

The IDs `plague_doctor` and `vestal` appear in `StageCoach.firstHeroClasses` (tutorial
only) and test fixtures, but have no `.bytes` data files or upgrade JSON files. They are
**not** part of the active recruitable hero class pool.
