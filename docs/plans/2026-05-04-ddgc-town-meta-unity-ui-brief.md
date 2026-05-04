# DDGC Town / Meta — Unity UI Hierarchy & Layout Migration Brief

**Date:** 2026-05-04
**Task:** UIR-003
**Context:** DDGC frontend migration, Phase 10 town/meta surface expansion

## 1. Assets Inspected

### Unity Scenes
- `Assets/Scenes/EstateManagement.unity` — the single town/meta scene; houses every estate building, provisioning, roster, quest select, and shared overlay surface. 1769 native GameObjects, 364 prefab instances, 424 text nodes.

### Prefabs
- `Assets/Prefabs/UI/Windows/CharacterWindow.prefab` — hero detail overlay (stress panel, hero info, model display, equipment, combat/camping skills, stats, resistances, roster scroll)
- `Assets/Prefabs/UI/Windows/MainMenuWindow.prefab` — main menu
- `Assets/Prefabs/UI/HeroSlot.prefab` — roster hero entry
- `Assets/Prefabs/UI/TreatmentHeroSlot.prefab` — sanitarium treatment slot
- `Assets/Prefabs/UI/MarketSlot.prefab` — market item slot
- `Assets/Prefabs/UI/ShopSlot.prefab` — shop item slot
- `Assets/Prefabs/UI/SkillUpgradeSlot.prefab` — skill upgrade slot
- `Assets/Prefabs/UI/UpgradeSlot.prefab` — building upgrade slot
- `Assets/Prefabs/UI/EquipmentUpgradeSlot.prefab` — equipment upgrade slot
- `Assets/Prefabs/UI/PartyInventorySlot.prefab` / `PartyInventorySlotInDungeon.prefab`
- `Assets/Prefabs/UI/Controls/HeroSlot.prefab` / `PartyHeroSlot.prefab` / `TrinketRow.prefab`
- `Assets/Prefabs/UI/Estate/Buildings/Graveyard/DeathRecord.prefab`
- `Assets/Prefabs/UI/Estate/Exchange/Exchange.prefab`
- `Assets/Prefabs/UI/Estate/PanelWindows/ActivityLog/*.prefab`
- `Assets/Prefabs/UI/Estate/PanelWindows/Glossary/GlossaryRecord.prefab`
- `Assets/Prefabs/UI/Raid/HeroResultSlot.prefab` / `LootSlot.prefab` / `OverlaySlot.prefab`
- `Assets/Prefabs/UI/Raid/Map/*.prefab` (hall sector, hallway, room)

### C# Scripts (key ones)
- **Managers:** `EstateSceneManager.cs`, `TownManager.cs`, `RaidSceneManager.cs`, `RaidPreparationManager.cs`, `PartyFormationManager.cs`, `ShopManager.cs`, `LocalizationManager.cs`, `SettingsManager.cs`, `ToolTipManager.cs`
- **UI/Windows:** `CharacterWindow.cs`, `RaidResultWindow.cs`, `ResultHeroWindow.cs`, `ResultItemWindow.cs`, `QuestCompletionWindow.cs`, `ReviveHeroWindow.cs`, `RealmInventoryWindow.cs`, `SettingsWindow.cs`, `ActivityLogWindow.cs`, `GlossaryWindow.cs`, `DialogueWindow.cs`, `TownEventWindow.cs`
- **UI/Windows/Buildings:** `BuildingWindow.cs`, `UpgradableBuildingWindow.cs`, `UpgradableHeroBuildingWindow.cs`, `StageCoachWindow.cs`, `GuildHeroWindow.cs`, `BlacksmithHeroWindow.cs`, `TavernWindow.cs`, `AbbeyWindow.cs`, `SanitariumWindow.cs` (+ quirk/disease sub-windows), `GardenWindow.cs`, `GraveyardWindow.cs`, `LegacyTowerWindow.cs`, `NomadWagonWindow.cs`, `CampingTrainerHeroWindow.cs`, `StatueWindow.cs`, `HeroOverviewWindow.cs`, `UpgradeWindow.cs`
- **UI/Panels:** `HeroRosterPanel.cs`, `EstateBottomPanel.cs`, `EstateTopPanel.cs`, `EstateCurrencyPanel.cs`, `SelectedQuestPanel.cs`, `RaidPartyPanel.cs`, `RaidQuestPanel.cs`, `RaidInventoryPanel.cs`, `StressPanel.cs`, `ResistancesPanel.cs`, `QuirksPanel.cs`, `CharStatsPanel.cs`, `CharEquipmentPanel.cs`, `CharCombatSkillPanel.cs`, `CharCampingSkillPanel.cs`, `RecruitPanel.cs`, `DungeonPanel.cs`, `HeroDiscardPanel.cs`, `DiseasePanel.cs`, `DialoguePanel.cs`, `RaidBannerPanel.cs`, `RaidHeroPanel.cs`, `RaidCharStatsPanel.cs`, `RaidCombatSkillsPanel.cs`, `RaidMapPanel.cs`, `RaidPanel.cs`, `QuestRewardPanel.cs`, `PartyCompositionPanel.cs`, `HeirloomExchangePanel.cs`, `ItemSellbackPanel.cs`, `TrayPanel.cs`, `StressOverlayPanel.cs`, `HeroDisplayPanel.cs`

### Sprites & Textures
- `Assets/Sprites/ui/` — ~300 button/badge/panel sprites (btn_*.png, panel_*.png, bar_*.png, etc.)
- `Assets/Sprites/` — building icons (Abbey.png, Guild.png, Tavern.png, StageCoach.png, etc.)
- `Assets/Sprites/ui/stress.*.png` — stress pip states (normal, stressed, overstressed)
- `Assets/Sprites/ui/btn_talk.png`, `btn_close.png`, `btn_black.png` — shared button sprites
- `Assets/Sprites/ui/upgrade_locked.png` — locked upgrade state
- `Assets/Sprites/Formation/` — combat formation sprites
- `Assets/Resources/Screen/` — loading screen images per dungeon biome

### Spine Assets
- `Assets/Spine/Runtime/` — spine-csharp + spine-unity runtime; character model display (ModelDisplayPanel references 16 hero class model variants: tank, hunter, alchemist, shaman, diviner × 2 each)

### Data Files
- `Assets/Resources/Data/Buildings/*.building.json` — 13 building definitions (abbey, blacksmith, camping_trainer, garden, guild, legacytower, nomad_wagon, sanitarium, stage_coach, tavern, etc.)
- `Assets/Resources/Data/Heroes/Info/` — hero class data
- `Assets/Resources/Data/Heroes/Sprites/` — hero-specific sprite atlas
- `Assets/Resources/Data/Json*.json` — game data (buffs, camping, loot, quests, quirks, traits, trinkets, AI)
- `Assets/Resources/Data/Localization/*.xml` — 22 XML string tables (Activity.xml, Heroes.xml, Menu.xml, Monsters.xml, Quirks.xml, TownEvents.xml, etc.)
- `Assets/Resources/Data/Dialogues/` — NPC dialogue data
- `Assets/Resources/Data/Curios/` — curio definitions
- `Assets/Resources/Data/Inventory/` — item definitions
- `Assets/Resources/Data/Dungeons/` — dungeon configuration

### Fonts
- `Assets/Fonts/ZhiYiSongTi-Regular.ttf` — primary Chinese UI font

---

## 2. Original Unity Hierarchy & Layout Analysis

### 2.1 Scene Root Structure

All UI lives under `EstateSceneManager` (single root manager):

```
EstateManagement.unity
├── EventSystem
├── MainCamera (tag: "Main UI Camera", FMOD StudioListener)
└── EstateSceneManager
    ├── UI_Provision (active: false — shown on provision flow)
    │   ├── UI_Provision
    │   │   ├── LeftPanel
    │   │   │   ├── EmbarkButton, Title, Icon, BackButton, Character
    │   │   └── ProvisionShop
    │   │       ├── RaidInfoGroup (quest context)
    │   │       ├── PartyInventory (scroll view + scrollbar)
    │   │       └── ShopInventory
    │   └── UI_Provision_Background
    │       └── BackGround
    ├── UI_Estate (active: true — town surface, always on)
    │   └── UI_Estate
    │       ├── Guild, Tavern, Graveyard, Blacksmith, StageCoach
    │       ├── Garden, LegacyTower, Abbey, Market, Sanitarium
    │       ├── CampingTrainer, QuickProgressButton, QuickStartButton
    │       └── (each building → BuildingLabel child)
    ├── UI_Shared (active: true — persistent overlays/panels)
    │   ├── UI_MidWindows
    │   │   ├── SettingsWindow, GlossaryWindow
    │   │   ├── ActivityLogWindow, TownEventWindow
    │   ├── UI_ScrollWindow
    │   ├── UI_Panels
    │   │   ├── EstateNameplate (EstateName + EstateIcon)
    │   │   └── BottomPanel
    │   │       ├── EmbarkButton → Text
    │   │       └── SideButtons (ActivityLog, RealmInventory, Hero,
    │   │                         TownEvent, Settings, Glossary)
    │   ├── UI_TopWindows
    │   │   ├── RealmInventoryWindow, CurrencyPanel
    │   │   ├── Exchange, ConfirmBox
    │   ├── UI_Roster
    │   │   └── RosterPanel
    │   │       ├── SortButtons (Building, Level, Class, Stress)
    │   │       ├── HeroInfo (Name, Class, Stress, Quirks, Diseases)
    │   │       ├── RosterScroll → Viewport → RosterSlots → HeroSlot
    │   │       └── CloseButton
    │   └── UI_LowWindows
    │       ├── GardenWindow, BlacksmithWindow, TavernWindow
    │       ├── GuildWindow, GraveyardWindow, StageCoachWindow
    │       ├── AbbeyWindow, MarketWindow, SanitariumWindow
    │       ├── CampingTrainerWindow, LegacyTowerWindow
    │       ├── DialogueWindow, ReviveHeroWindow
    │       └── (each building window → LeftPanel + RightPanel)
    └── UI_Quest (active: false — shown on quest select)
        ├── UI_Quest_Match_Height
        │   ├── SelectedQuestPanel (dungeon info, party slots, rewards)
        │   ├── Background, BackButton
        │   ├── RaidPartyPanel (PartySlots with HeroSlot instances)
        │   ├── Dungeons (BaiHu, ZhuQue, XuanWu, QingLong + locked)
        │   └── HeroDiscardPanel
        └── RaidManager
```

### 2.2 Canvas & Layer Ordering

The original EstateManagement scene uses a **single-camera, all-in-one-scene** approach:
- **MainCamera** tag = "Main UI Camera" — a single orthographic camera renders all UI.
- There is no explicit canvas sorting layer separation. Instead, visibility is controlled by **activating/deactivating parent GameObjects**:
  - `UI_Estate` and `UI_Shared` are always active (persistent town shell).
  - `UI_Provision` and `UI_Quest` are toggled active=false, activated by state transitions.
  - Individual windows (`UI_LowWindows` children) start inactive and are toggled on demand.
- Building window modal stacking uses sibling activation (only one `UI_LowWindows` child active at a time).
- `CharacterWindow` prefab is instantiated dynamically and parented to the active canvas at runtime.
- The shared `BottomPanel` / `SideButtons` serve as the persistent navigation anchor.

### 2.3 RectTransform Patterns

The UI uses a **manual absolute positioning via anchored RectTransforms** pattern (no responsive auto-layout in most places):

| Pattern | Anchor | Pivot | Usage |
|---------|--------|-------|-------|
| Full-screen stretch | (0,0)–(1,1) | (0.5, 0.5) | CharacterWindow root, window backdrops |
| Center-absolute | (0.5, 0.5)–(0.5, 0.5) | (0.5, 0.5) | Most building LeftPanel/RightPanel, buttons |
| Top-left anchored | (0,1)–(0,1) | (0, 1) | EstateNameplate, title bars |
| Bottom-stretch | (0,0)–(1,0) | (0.5, 0) | BottomPanel |
| Top-stretch | (0,1)–(1,1) | (0.5, 1) | EstateNameplate parent |

**Building window typical metrics (1280×720 base):**
- LeftPanel: anchor=(0.5,0.5), pos=(-560, -80), size=(674, 834) — left half
- RightPanel: anchor=(0.5,0.5), pos=(160, -80), size=(674, 834) — right half
- CloseButton: anchor=(0.5,0.5), positioned near top-right of LeftPanel
- EmbarkButton: anchor=(0.5,0.5), pos=(-40, -320), size=(431, 55)
- Provision LeftPanel: anchor=(0.5,0.5), pos=(-560, -80), size=(674, 834)
- ShopInventory: anchored in equivalent right-half position

### 2.4 Panel Nesting Convention

Every building window follows a strict two-panel split:

```
BuildingWindow
├── LeftPanel
│   ├── Character (NPC portrait/ Spine model)
│   ├── Icon (building icon sprite)
│   ├── Title / BuildingLabel
│   ├── TalkButton → Text
│   └── CloseButton → Text
└── RightPanel
    ├── UseWindow (service content: recruit,治疗, buy, upgrade, etc.)
    ├── UpgradeWindow (upgrade details, toggled)
    ├── UseButton → Text
    └── UpgradeButton → Text
```

The CharacterWindow follows a different layout optimized for hero inspection:
```
CharacterWindow (full-screen overlay)
├── StressPanel (10 pips in LayoutGroup)
├── HeroPanel (name, description, dismiss)
├── WindowBackground
├── CloseButton
├── ModelDisplayPanel (Spine model + 16 class variants)
├── CharacterGuildHeader
├── PreviousHeroButton / NextHeroButton
├── EmptyHint
├── Panels (tabbed sub-panels)
│   ├── CampingSkillsPanel → SkillScroll → SkillInfo
│   ├── EquipmentPanel (weapon, armor, 2 trinket slots)
│   ├── CombatSkillsPanel → SkillScroll → SkillInfo
│   ├── InfoPanel
│   │   ├── QuirksPanel (positive + negative)
│   │   ├── ResolveLevelBar
│   │   ├── BaseStatsPanel (DMG, Crit, ACC, SPD, Dodge, Prot, etc.)
│   │   ├── Chaos (corruption bar)
│   │   └── StatePanel
│   │       ├── ResistancesPanel (11 resist slots)
│   │       └── DiseasesPanel
│   ├── StateButton / InfoButton / EquipButton / CombatSkillButton / CampingSkillButton
│   └── Background
└── RosterScroll → Viewport → RosterSlots → HeroSlot
```

### 2.5 Provision Layout

```
UI_Provision
├── LeftPanel (center-absolute, left half)
│   ├── Title (BuildingLabel + BuildingDesc)
│   ├── Icon → Image
│   ├── Character → Circle
│   ├── BackButton → Text
│   └── EmbarkButton → Text
└── ProvisionShop
    ├── RaidInfoGroup (QuestLength, QuestDifficulty, QuestLocation)
    ├── PartyInventory (Scroll View + Scrollbar — drag-to-equip slots)
    └── ShopInventory (buyable provisions grid)
```

### 2.6 Quest / Result / Return Layout

```
SelectedQuestPanel
├── QuestTitle, QuestDescription
├── MapIcon → Frame
├── MapName, MapSize
├── QuestDifficulty (5× Image pips)
├── QuestRewardsTitle → QuestGoalsTitle
├── QuestGoals, RewardsPanel → RewardItems
├── QuestCamps (camping selections)
├── PartySlots (hero assignment)
├── ProvisionButton → Text
├── PrevButton / NextButton / CloseButton
└── (prefab: HeroResultSlot — per-hero result card)
    ├── Portrait → Frame
    ├── NameLabel
    ├── StressPanel (10× StressPipSlot → StressPip)
    ├── XpLabel, Exp (Text, Level, Value, Progress)
    ├── Chaos (corruption), Divider
    └── ResolvePulse / RevealAnimation

RaidPartyPanel
├── PartyTitle
└── PartySlots (4× HeroSlot instances)
```

---

## 3. Target Landscape-First Frontend Layout

### 3.1 Design Principles

1. **Landscape-first, not responsive-portrait.** The original UI targets a 1280×720 fixed canvas. The frontend target is landscape-oriented (min 1280×720, designed for 16:9). Vertical scrolling is used only within scroll zones (roster, skill lists, inventory), not as the primary layout axis.
2. **Not a generic web dashboard.** No full-page data tables, no card-wall feeds, no infinite scroll, no sidebar-constrained "admin panel" layout.
3. **Frame-based composition.** Each screen is a distinct frame cycle, not a single scrolling page. Navigation is modal/panel-based, matching the original window-on-demand pattern.
4. **Left-right panel split for building surfaces.** This mirrors the original LeftPanel (character + talk) / RightPanel (service + upgrade) convention.
5. **Persistent town shell** with a dedicated bottom-nav bar (mirrors `UI_Shared → BottomPanel → SideButtons`).
6. **Full-screen modal overlays** for hero detail and shared windows, matching CharacterWindow and MidWindow patterns.

### 3.2 Screen Map

| Flow State | Unity Source | Frontend Target | Layout Pattern |
|------------|-------------|-----------------|----------------|
| `town` | `UI_Estate` + `UI_Shared` | `TownShellScreen` | Bottom-nav shell with building grid + roster summary, two-column stack |
| `hero-detail` | `CharacterWindow.prefab` | `HeroDetailScreen` | Full-screen overlay, two-column info + stats tabbed panel |
| `building` | `UI_LowWindows/*Window` | `BuildingDetailScreen` + sub-screens | Left-right panel split (character + service), modal |
| `provisioning` | `UI_Provision` | `ProvisioningScreen` | Left party panel, right shop grid, modal |
| `quest-select` | `UI_Quest` + `SelectedQuestPanel` | `ExpeditionScreen` | Full-screen quest info + party assignment, modal |
| `result` | `RaidResultWindow` + `HeroResultSlot` | `ResultScreen` | Outcome summary + hero cards + loot, full-screen |
| `return` | `ResultHeroWindow` + `ResultItemWindow` | `ReturnScreen` | Returning hero list + resources, full-screen |

### 3.3 Target Wireframe Descriptions

**Town Shell:**
```
┌─────────────────────────────────────────────────────┐
│ [Campaign Name] — Week X                    [Gold] │
├──────────┬──────────────────────────────────────────┤
│ ROSTER   │ BUILDINGS                                │
│ ┌──────┐ │ ┌────────┐ ┌────────┐ ┌────────┐       │
│ │ Hero1 │ │ │ Guild  │ │Tavern  │ │Abbey   │       │
│ │ HP:80%│ │ │ Lv 2   │ │ Lv 1   │ │ Lv 1   │       │
│ │ Stress│ │ │ Op     │ │ Op     │ │ Locked │       │
│ │ : 25  │ │ │        │ │        │ │        │       │
│ ├──────┤ │ ├────────┤ ├────────┤ ├────────┤       │
│ │ Hero2 │ │ │Black-  │ │Stage-  │ │Sanitar-│       │
│ │ ...   │ │ │ smith  │ │ coach  │ │ ium    │       │
│ └──────┘ │ └────────┘ └────────┘ └────────┘       │
│ [Inspect]│                      [Proceed →]        │
├──────────┴──────────────────────────────────────────┤
│ [Activity] [Inventory] [Heroes] [Settings] [Help]   │
└─────────────────────────────────────────────────────┘
```

**Building Detail (e.g., Stagecoach):**
```
┌─────────────────────────────────────────────────────┐
│ Building — Stagecoach                    [× Close] │
├──────────┬──────────────────────────────────────────┤
│          │ RECRUITMENT                              │
│ [Icon]   │ Recruit Hero          Cost: 1000g       │
│          │ ┌────────────────────────────────────┐   │
│          │ │ 2 heroes available for recruitment │   │
│ [NPC]    │ └────────────────────────────────────┘   │
│          │ [Recruit]                                │
│          │                                          │
│          │ UPGRADES                                 │
│          │ Upgrade to Lv 2    Cost: 2000g + Deed    │
│          │ [Upgrade]                                │
│          │                                          │
├──────────┴──────────────────────────────────────────┤
│ [Return to Town]                                    │
└─────────────────────────────────────────────────────┘
```

**Provisioning:**
```
┌─────────────────────────────────────────────────────┐
│ Provisioning — [Expedition Name]              [×]   │
├──────────┬──────────────────────────────────────────┤
│ PARTY    │ SHOP                                     │
│ ┌──────┐ │ ┌──────┐ ┌──────┐ ┌──────┐             │
│ │Slot 1│ │ │Food  │ │Band- │ │Torch │             │
│ │ HeroA │ │ │×10   │ │ages  │ │×8    │             │
│ ├──────┤ │ ├──────┤ ├──────┤ ├──────┤             │
│ │Slot 2│ │ │Shov- │ │HolyW │ │Anti- │             │
│ │ HeroB │ │ │el    │ │ater  │ │toxin │             │
│ ├──────┤ │ ├──────┤ ├──────┤ ├──────┤             │
│ │Slot 3│ │ │...   │ │...   │ │...   │             │
│ ├──────┤ │ └──────┘ └──────┘ └──────┘             │
│ │Slot 4│ │                                          │
│ └──────┘ │              Supplies: Ample             │
│          │              Cost: 2500g                 │
│          │                              [Embark →]  │
├──────────┴──────────────────────────────────────────┤
│ [Back to Town]                                      │
└─────────────────────────────────────────────────────┘
```

**Result:**
```
┌─────────────────────────────────────────────────────┐
│ Expedition Complete — Victory!              [×]     │
├──────────┬──────────────────────────────────────────┤
│ SUMMARY  │ HERO OUTCOMES                            │
│ ┌──────┐ │ ┌────────────────────────────────────┐   │
│ │Dungeon│ │ │ HeroA    Alive    HP: 45/52       │   │
│ │Ruins  │ │ │          Stress: 67/100            │   │
│ │       │ │ ├────────────────────────────────────┤   │
│ │Loot:  │ │ │ HeroB    Stressed HP: 30/44        │   │
│ │Gold   │ │ │          Stress: 98/100            │   │
│ │+3000  │ │ ├────────────────────────────────────┤   │
│ │Supp+5 │ │ │ HeroC    [DECEASED]                │   │
│ │XP+500 │ │ └────────────────────────────────────┘   │
│ └──────┘ │                                          │
│          │ LOOT ACQUIRED                            │
│          │ Ruby, Scroll, Emerald x2                 │
│          │                              [Continue →]│
├──────────┴──────────────────────────────────────────┤
└─────────────────────────────────────────────────────┘
```

### 3.4 Layout Rejection Criteria

The following layout patterns **must not** be used as completion:

- **Generic web dashboard** — no sidebar nav with hamburger menu, no card-wall of stats widgets, no KPI-metric grid, no data-table-only views.
- **Long scrolling page** — primary content must fit the viewport without scroll. Scrolling is permitted only inside explicit scroll zones (roster lists, inventory).
- **Mobile-first vertical stack** — town surface is landscape-first. The button grid and building panels should not collapse into a single-column phone layout.
- **Full-list table view for town details** — use panel/card composition (mirroring the original modal windows), not spreadsheet-style `<table>` grids.

---

## 4. Layout Mapping: Unity → Frontend

### 4.1 Building Window Decomposition

Every building window decomposes into isomorphic frontend panels:

| Unity Element | Frontend Equivalent | Notes |
|--------------|-------------------|-------|
| `LeftPanel → Character` | (omitted or placeholder) | NPC portrait — sprite/Spine asset pending |
| `LeftPanel → Icon` | Headline area in left column | Building icon sprite |
| `LeftPanel → Title` | `panel-title` / `AppFrame eyebrow` | Text from view model |
| `LeftPanel → TalkButton` | Not yet wired | Dialogue system pending |
| `LeftPanel → CloseButton` | `onReturn` callback | Modal dismiss |
| `RightPanel → UseWindow` | Primary action card(s) | Slot grid / service list |
| `RightPanel → UpgradeWindow` | Upgrade section | Toggle visibility |
| `RightPanel → UseButton` / `UpgradeButton` | `action-primary` / `action-secondary` | Per-action buttons |

### 4.2 Town Shell Decomposition

| Unity Element | Frontend Equivalent | Notes |
|--------------|-------------------|-------|
| `UI_Estate → Guild/Tavern/...` | Building card grid in `TownShellScreen` | Click → `onOpenBuilding(id)` |
| `UI_Panels → BottomPanel → EmbarkButton` | `onStartProvisioning` CTA | Flows into provisioning |
| `UI_Panels → BottomPanel → SideButtons` | Bottom nav bar | `Activity`, `Inventory`, `Heroes`, `Settings`, `Help` |
| `UI_Roster → RosterPanel` | Roster summary cards in left column | Expand into full `RosterPanel` |
| `UI_TopWindows → CurrencyPanel` | Gold display in shell header | Inline pill |
| `UI_MidWindows` | Full-screen modal overlays | Future phase |

### 4.3 Hero Detail Decomposition

| Unity Element | Frontend Equivalent | Notes |
|--------------|-------------------|-------|
| `CharacterWindow` root | `HeroDetailScreen` full-screen overlay | `AppFrame` wrapper |
| `StressPanel` (10 pips) | Stress bar with numeric | Simplified from 10-pip to bar |
| `HeroPanel → HeroName` | `AppFrame title` | View model name |
| `ModelDisplayPanel` | `PixiStage` | Spine model pending |
| `Panels → EquipmentPanel` | Equipment section | Text summary (slots pending) |
| `Panels → CombatSkillsPanel` | Combat skills list | Name-only (details pending) |
| `Panels → CampingSkillsPanel` | Camping skills list | Name-only |
| `InfoPanel → BaseStatsPanel` | Stats grid | ACC, DMG, Crit, SPD, Dodge, Prot |
| `InfoPanel → ResistancesPanel` | Resistances grid | Stun, Bleed, Disease, Move, Death, Trap, etc. |
| `InfoPanel → QuirksPanel` | Quirks list | Positive + negative |
| `Panels → tab buttons` | `Switch`/`Match` tab routing | Button group for panel selection |
| `RosterScroll` | Not yet wired | Hero list navigation |

### 4.4 Result/Return Decomposition

| Unity Element | Frontend Equivalent | Notes |
|--------------|-------------------|-------|
| `RaidResultWindow.cs` | `ResultScreen` | Outcome + hero status + loot |
| `HeroResultSlot.prefab` | Hero outcome card | Per-hero status, HP change, stress change |
| `LootSlot.prefab` | Loot list items | Each acquired item |
| `ResultHeroWindow.cs` | `ReturnScreen` | Returning hero list |
| `QuestCompletionWindow.cs` | Result + return flow | Redirects to town resume |

---

## 5. Key Layout Differences from Unity

| Aspect | Unity Original | Frontend Target |
|--------|---------------|-----------------|
| Canvas | 1280×720 fixed, single camera | Dynamic viewport, CSS-based layout |
| Positioning | Absolute anchored RectTransforms | Flex/grid CSS with column-stack panels |
| Font | `ZhiYiSongTi-Regular.ttf` sprite-based | System/web font with CSS fallback |
| Building windows | LeftPanel (character) + RightPanel (service) | Same split, but character portrait optional |
| Stress display | 10-pip LayoutGroup | Numeric bar (faster to read) |
| Tab panel switching | Button + GameObject.SetActive | SolidJS `<Switch>`/`<Match>` routing |
| Hero model | Spine SkeletonAnimation in 3D view | `PixiStage` renderer (Spine via TS) |
| Modal stacking | SetActive toggle on window parents | Component mount/unmount via flow state |
| Save/load UI | Unity GUI system | In-browser persisted session |
| Localization | XML string tables (`Localization/*.xml`) | i18n strings mapped from Rust contracts |

---

## 6. Migration Recommendations

### Priority Order
1. **Town shell** with persistent navigation — already has `TownShellScreen`.
2. **Building screens** — generic `BuildingDetailScreen` exists; specialization for Stagecoach, Guild, Blacksmith done; extend to Tavern, Sanitarium, Abbey.
3. **Hero detail** — `HeroDetailScreen` exists; add stat details, quirks, resistances, equipment.
4. **Provisioning** — `ProvisioningScreen` exists; add party slot assignment + shop grid.
5. **Quest/expedition launch** — `ExpeditionScreen` exists; wire party composition + dungeon selection.
6. **Result/return** — `ResultScreen` + `ReturnScreen` exist; wire loot + hero outcome display.

### Layout Rules (hard constraints)
- No `<table>` layouts for building or roster data.
- No infinite-scroll or pull-to-refresh patterns.
- No sidebar-navigation-as-primary-layout (no admin-dashboard frame).
- Town viewport must present a building grid that fits at 1280×720 without vertical scroll.
- Building windows must use a two-column split; single-column is acceptable only for hero detail info tabs.

---

## Post-Migration Outcome

The completed UI Redo (UIR-005 through UIR-011) implements all screens described in this brief. See the **[UIR-011 outcome document](../UIR-011-UI-REDO-RECOVERY-OUTCOME.md)** for:

- The full inventory of scenes, prefabs, and assets inspected and mapped
- Asset blockers and deferred work (original `.png` sprites not in repo, Spine runtime not configured)
- Local run, smoke, and browser acceptance commands
- Why the final frontend is frame-based game UI, not a generic web layout
- Screen completion status with fidelity gate results
