# UIR-005A — Town/Base Owning Hierarchy & Screenshot Anchors Recon

**Date:** 2026-05-05
**Task:** UIR-005A — UI Redo: recon town/base owning hierarchy and screenshot anchors
**Scope:** Source-backed inventory of the owning Unity hierarchy, persistent shell chrome, building node geometry, and screenshot acceptance anchors for the DDGC town/base screen. No frontend implementation work.

---

## 1. Files Inspected

### Unity Scene JSON Extractions (`fixtures/ui_inventory/`)
| File | Kind | Size | GameObjects | Prefab Instances | Text Nodes |
|------|------|------|-------------|-------------------|------------|
| `fixtures/ui_inventory/index.json` | inventory manifest | 6.4 KB | — | — | — |
| `fixtures/ui_inventory/Assets/Scenes/EstateManagement.unity.json` | scene | 7.7 MB | 1769 native | 364 | 424 |
| `fixtures/ui_inventory/Assets/Scenes/Intro.unity.json` | scene | 22 KB | 8 | 0 | 1 |

### Unity Prefab JSON Extractions (`fixtures/ui_inventory/Assets/Prefabs/`)
| File | Size | Description |
|------|------|-------------|
| `UI/Windows/CharacterWindow.prefab.json` | 1.2 MB | Hero detail overlay (95 text nodes, 442 asset refs) |
| `UI/Windows/MainMenuWindow.prefab.json` | 47 KB | Pause/options menu (5 text nodes) |
| `UI/Estate/Buildings/Graveyard/DeathRecord.prefab.json` | 28 KB | Graveyard death record entry |
| `UI/Estate/Exchange/Exchange.prefab.json` | 16 KB | Currency exchange window |
| `UI/Estate/PanelWindows/ActivityLog/WeekBlock.prefab.json` | 501 KB | Activity log week block (36 text nodes) |
| `UI/Estate/PanelWindows/ActivityLog/ActivityRecord.prefab.json` | 15 KB | Activity log record entry |
| `UI/Estate/PanelWindows/ActivityLog/Goal.prefab.json` | 11 KB | Activity log goal entry |
| `UI/Estate/PanelWindows/Glossary/GlossaryRecord.prefab.json` | 9.5 KB | Glossary record entry |
| `UI/HeroSlot.prefab.json` | 26 KB | Roster hero slot |
| `UI/TreatmentHeroSlot.prefab.json` | 19 KB | Sanitarium treatment slot |
| `UI/MarketSlot.prefab.json` | 16 KB | Market item slot |
| `UI/ShopSlot.prefab.json` | 16 KB | Shop item slot |
| `UI/SkillUpgradeSlot.prefab.json` | 17 KB | Skill upgrade slot |
| `UI/UpgradeSlot.prefab.json` | 24 KB | Building upgrade slot |
| `UI/EquipmentUpgradeSlot.prefab.json` | 18 KB | Equipment upgrade slot |
| `UI/PartyInventorySlot.prefab.json` | 14 KB | Party inventory slot |
| `UI/PartyInventorySlotInDungeon.prefab.json` | 13 KB | Dungeon inventory slot |
| `UI/Controls/GameSetup.prefab.json` | 2.1 KB | Cursor/content bootstrap |
| `UI/Controls/HeroSlot.prefab.json` | 123 KB | Control hero slot (10 text nodes) |
| `UI/Controls/PartyHeroSlot.prefab.json` | 52 KB | Party hero slot |
| `UI/Controls/TrinketRow.prefab.json` | 55 KB | Trinket row |

### Extracted Sprite PNGs (`frontend/public/original/`)
| Path | Size | Mapped To |
|------|------|-----------|
| `buildings/building_perception_tower.png` | 189 KB | stagecoach |
| `buildings/building_train_field.png` | 265 KB | guild |
| `buildings/building_forging.png` | 385 KB | blacksmith |
| `buildings/building_cell_repair.png` | 221 KB | sanitarium |
| `buildings/building_faith_altar.png` | 248 KB | abbey |
| `buildings/building_paradise.png` | 521 KB | tavern |
| `heroes/hunter_portrait_roster.png` | 17 KB | hunter (base variant) |
| `heroes/hunter1_portrait_roster.png` | 17 KB | hunter (white variant) |
| `heroes/hunter2_portrait_roster.png` | 17 KB | hunter (black variant) |

### Frontend Source Files (cross-reference only — not modified)
| File | Purpose |
|------|---------|
| `frontend/src/town/buildingCatalog.ts` | 11-building layout catalog with display names and label offsets |
| `frontend/src/screens/town/TownShellScreen.tsx` | Town shell component rendering building grid, roster, embark button |
| `frontend/src/screens/town/buildings/BuildingIcons.tsx` | Building sprite resolver component |
| `frontend/src/townEstateLayout.css` | CSS layout for estate town screen |
| `frontend/src/assets/originalAssetPaths.ts` | Sprite path resolver for buildings and hero portraits |
| `frontend/src/assets/asset-manifest.json` | Full Unity asset inventory (99 references catalogued) |
| `frontend/src/bridge/contractTypes.ts` | TownViewModel, TownBuildingSummary, TownHeroSummary types |

### Documentation (cross-reference only)
| File | Purpose |
|------|---------|
| `docs/plans/2026-05-04-ddgc-town-meta-unity-ui-brief.md` | Prior UIR-003 Unity UI hierarchy analysis (560 lines) |

---

## 2. Owning Unity Hierarchy: EstateManagement.unity

### 2.1 Scene Root Structure

All UI lives under `EstateSceneManager` (single root manager GameObject). The scene uses a **single-camera, all-in-one-scene** approach with visibility controlled by activating/deactivating parent GameObjects rather than canvas sorting layers.

```
EstateManagement.unity
├── EventSystem
├── MainCamera (tag: "Main UI Camera")
└── EstateSceneManager
    ├── UI_Provision (active: false)
    │   └── UI_Provision
    │       ├── LeftPanel → EmbarkButton, Title, Icon, BackButton, Character
    │       └── ProvisionShop → RaidInfoGroup, PartyInventory, ShopInventory
    ├── UI_Estate (active: true — persistent town canvas)
    │   └── UI_Estate
    │       ├── Guild, Tavern, Graveyard, Blacksmith, StageCoach
    │       ├── Garden, LegacyTower, Abbey, Market, Sanitarium
    │       ├── CampingTrainer
    │       ├── QuickProgressButton, QuickStartButton
    │       └── (each building → BuildingLabel child)
    ├── UI_Shared (active: true — persistent overlays)
    │   ├── UI_MidWindows → SettingsWindow, GlossaryWindow, ActivityLogWindow, TownEventWindow
    │   ├── UI_ScrollWindow
    │   ├── UI_Panels
    │   │   ├── EstateNameplate → EstateName + EstateIcon
    │   │   └── BottomPanel
    │   │       ├── EmbarkButton → Text
    │   │       └── SideButtons → ActivityLog, RealmInventory, Hero, TownEvent, Settings, Glossary
    │   ├── UI_TopWindows → RealmInventoryWindow, CurrencyPanel, Exchange, ConfirmBox
    │   ├── UI_Roster → RosterPanel
    │   │   ├── SortButtons (Building, Level, Class, Stress)
    │   │   ├── HeroInfo (Name, Class, Stress, Quirks, Diseases)
    │   │   ├── RosterScroll → Viewport → RosterSlots → HeroSlot
    │   │   └── CloseButton
    │   └── UI_LowWindows (building modal windows)
    │       ├── GardenWindow, BlacksmithWindow, TavernWindow
    │       ├── GuildWindow, GraveyardWindow, StageCoachWindow
    │       ├── AbbeyWindow, MarketWindow, SanitariumWindow
    │       ├── CampingTrainerWindow, LegacyTowerWindow
    │       ├── DialogueWindow, ReviveHeroWindow
    │       └── (each → LeftPanel + RightPanel split)
    └── UI_Quest (active: false)
        ├── SelectedQuestPanel
        ├── RaidPartyPanel → PartySlots
        ├── HeroDiscardPanel
        └── RaidManager
```

### 2.2 Canvas Reference Resolution

The Unity UI base resolution is **1280 × 720**. All RectTransform values below are in this coordinate space. The building nodes in `UI_Estate` use center-anchored positioning (`anchorMin = (0.5, 0.5)`, `anchorMax = (0.5, 0.5)`, `pivot = (0.5, 0.5)`) — absolute positions relative to the center of the 1280×720 canvas.

---

## 3. Persistent Shell Chrome Inventory

These are the always-visible UI elements that form the town/base chrome. They live under `UI_Shared → UI_Panels` and are always active.

### 3.1 Top Nameplate (`EstateNameplate`)

**Source:** `UI_Shared → UI_Panels → EstateNameplate`
**RectTransform:** anchorMin=(1,1), anchorMax=(1,1), pivot=(0.5,0.5), anchoredPosition=(-1624, -76.5), sizeDelta=(592, 153)

| Element | Role | RectTransform |
|---------|------|---------------|
| EstateNameplate | container | 592×153, top-right anchored |
| EstateName | campaign title text | pos=(-75, 25), size=(400, 80) relative to nameplate |
| EstateIcon | campaign icon/crest | pos=(234, 0), size=(168, 172) relative to nameplate |

**Current frontend mapping:** `TownShellScreen` → `header.estate-top-panel` → `div.estate-name-card` containing `h1.estate-campaign-name` + `p.estate-campaign-summary`. Positioned at top-right (right: 42px, top: 28px) on the 1920×1080 reference canvas.

### 3.2 Currency Strip (`CurrencyPanel`)

**Source:** `UI_Shared → UI_TopWindows → CurrencyPanel`
**RectTransform:** anchorMin=(1,0), anchorMax=(1,0), pivot=(1,0), anchoredPosition=(-140, 20), sizeDelta=(1000, 80)

Contains five currency amount displays and five currency icon sprites:
- CurrencyBust (busts), CurrencyPortrait (portraits), CurrencyDeed (deeds), CurrencyCrest (crests), CurrencyGold (gold)

**Current frontend mapping:** Simplified single-currency display via `span.estate-status-pill` showing gold only. The four heirloom currencies are deferred.

### 3.3 Bottom Panel (`BottomPanel`)

**Source:** `UI_Shared → UI_Panels → BottomPanel`
**RectTransform:** anchorMin=(0,0), anchorMax=(1,1), pivot=(0.5,0), anchoredPosition=(0,0), sizeDelta=(0,0) — full-width bottom-stretch

#### 3.3.1 Embark Control (`EmbarkButton`)

**Source:** `BottomPanel → EmbarkButton`
**RectTransform:** anchorMin=(0,0), anchorMax=(0,0), pivot=(0.5,0.5), anchoredPosition=(20, 20), sizeDelta=(968, 968)
**Child:** `Text` at anchoredPosition=(100, 120), size=(180, 180)

The primary CTA button that transitions from town to provisioning/expedition flow.

**Current frontend mapping:** `button.action-primary.estate-embark-button` at bottom-left of `.estate-bottom-panel`, 268px wide column. Shows title "位面探索" and subtitle from `viewModel.nextActionLabel`.

#### 3.3.2 Side Buttons (`SideButtons`)

**Source:** `BottomPanel → SideButtons`
Six navigation buttons anchored to bottom-right corner:

| Button | anchoredPosition | sizeDelta |
|--------|-----------------|-----------|
| ActivityLogButton | (-100, -80) | (136, 136) |
| RealmInventoryButton | (-340, -80) | (136, 136) |
| HeroButton | (-220, -80) | (136, 136) |
| TownEventButton | (-100, -80) | (136, 136) |
| SettingsButton | (-100, -80) | (136, 136) |
| GlossaryButton | (-220, -80) | (136, 136) |

Each button has a `Text` child.

**Current frontend mapping:** Not yet implemented in TownShellScreen. The current frontend merges the embark button and roster strip into a single bottom panel without the six side navigation buttons. These are deferred to a later phase.

### 3.4 Building Background Layers

**Source:** No dedicated background GameObject in Unity. The town background is the scene's camera clear color and any world-space 3D backdrop. The Unity UI relies on the scene camera rendering the Estate scene environment behind the UI canvas.

**Current frontend mapping:** CSS layers in `townEstateLayout.css`:
- `.estate-town-screen`: radial + linear gradient backdrop (#211a19 → #120f13 → #09090b)
- `.estate-stage-backdrop`: elliptical glow (inset: 72px 20px 112px) with warm-tone radial gradient
- `.estate-stage-grid`: semi-transparent grid overlay (inset: 86px 112px 118px) with vertical/horizontal line pattern
- `.estate-town-screen::before`: top-rim light wash + bottom vignette

### 3.5 Building Label Layer

**Source:** Each building node under `UI_Estate` has a `BuildingLabel` child GameObject.
**RectTransform pattern:** All BuildingLabel instances share sizeDelta=(326, 48) — a fixed-width label banner.
**Position:** anchored relative to the parent building node (center-anchored, positioned as a child).

The label offset from the building center is captured per-building below.

**Current frontend mapping:** `span.estate-building-banner` positioned at `(width/2 + labelOffsetX, height/2 - labelOffsetY)` relative to the building node. Contains `estate-building-source-label` (display name) and `estate-building-status` (ready/partial/locked badge).

---

## 4. Building Node Geometry (Source: `EstateManagement.unity.json` → `UI_Estate → UI_Estate`)

All building nodes use center-anchored RectTransforms with anchor=(0.5,0.5), pivot=(0.5,0.5) unless noted otherwise. The `anchoredPosition` is relative to the 1280×720 canvas center. The `BuildingLabel` is a child with its own anchored position relative to the building node.

### 4.1 Building Node Positions & Sizes (Unity Source)

| Building | anchoredPosition (x, y) | sizeDelta (w, h) | Label Pos (x, y) | Label Size |
|----------|------------------------|-------------------|-------------------|------------|
| Guild | (120, -50) | (397, 397) | (30, -50) | (326, 48) |
| Tavern | (-650, -220) | (519, 519) | (-6, 160) | (326, 48) |
| Graveyard | (695.5, 95.63) | (344, 344) | (100, -100) | (326, 48) |
| Blacksmith | (709, 294) ⚠️ | (482, 482) | (0, -168) | (326, 48) |
| StageCoach | (31, -267) | (384, 384) | (-26, -150) | (326, 48) |
| Garden | (-245, -211) | (371, 371) | (0, -130) | (326, 48) |
| LegacyTower | (0, 270) | (541, 541) | (-3, 166) | (326, 48) |
| Abbey | (-330, 90) | (519, 519) | (-10, 153) | (326, 48) |
| Market | (340.5, -222) | (326, 339) | (0, -150) | (326, 48) |
| Sanitarium | (390, 110) | (339, 339) | (50, -100) | (326, 48) |
| CampingTrainer | (-607, 220) | (266, 314) | (-100, -110) | (326, 48) |

> ⚠️ **Blacksmith anchor anomaly:** Blacksmith uses anchorMin=(0.5,0), anchorMax=(0.5,0) unlike all other buildings which use center anchors. This means its Y-position is measured from the bottom edge of the canvas, not the center. In a 720px tall canvas, the bottom-anchored Y=294 translates to approximately Y=-66 from center (294 - 720/2 = -66).

### 4.2 Building ID → Display Name Mapping (DDGC Chinese)

| Building ID | DDGC Display Name | Function |
|-------------|-------------------|----------|
| stagecoach | 次元感知塔 | Recruit new heroes |
| guild | 试炼场 | Train skills, adjust combat abilities |
| blacksmith | 锻造舱 | Upgrade/maintain weapons and equipment |
| sanitarium | 细胞修复站 | Treat quirks, diseases, long-term conditions |
| abbey | 信仰祭坛 | Reduce stress through prayer and ritual |
| tavern | 迷情乐园 | Relieve stress, restore status via tavern activities |
| graveyard | 英雄档案馆 | View death records and history |
| garden | 天国花园 | Special rest and recovery services |
| legacytower | 遗留塔 | Legacy and museum-style collection |
| market | 交易市场 | Buy supplies, provisions, and shop services |
| campingtrainer | 空间分析 | Campfire and camping training services |

### 4.3 Building Node → Frontend Layout Fidelity

The frontend `buildingCatalog.ts` translates Unity center-relative positions (1280×720) to a larger 1920×1080 reference canvas. The coordinate mapping is:
- `estateLeft(x) = 1920/2 + x` (x=0 is center of 1920px)
- `estateTop(y) = 1080/2 - y` (y=0 is center of 1080px, positive Y is up in Unity)

This means building positions are directly transferred from Unity to frontend with the same numeric values — the frontend simply uses a larger canvas. All building nodes maintain their relative spacing. The building `width` and `height` from Unity `sizeDelta` are used as the frontend building node dimensions.

**Fidelity notes:**
1. Building positions are source-faithful — taken directly from `anchoredPosition` in the Unity scene JSON.
2. Building sizes are source-faithful — taken directly from `sizeDelta`.
3. Label offsets are source-faithful — `BuildingLabel.anchoredPosition` relative to building node.
4. The frontend reference canvas (1920×1080) is larger than the Unity canvas (1280×720), so buildings appear slightly smaller relative to the frame, but their relative positions are preserved.
5. The Blacksmith building anchor anomaly (bottom-anchored instead of center-anchored) means its effective Y position differs from other buildings. The catalog value of y=294 should be interpreted as 294 - 360 = -66 in center-relative terms on the 720px canvas. The catalog records the raw anchoredPosition value (294) which corresponds to a different effective center offset.

---

## 5. Screenshot Acceptance Anchors

### 5.1 Anchor Point Definitions

For parity comparison between Unity source renders and the frontend reconstruction, the following anchor points serve as screenshot acceptance checkpoints:

| Anchor ID | Unity Source Element | Frontend Target | Check Criteria |
|-----------|---------------------|-----------------|----------------|
| **A-NAMEPLATE** | `EstateNameplate` (top-right, 592×153) | `.estate-name-card` (top-right, 592×153) | Position, size, text content match |
| **A-CURRENCY** | `CurrencyPanel` (bottom-right stripe) | `.estate-status-pill` (gold display) | Gold value matches; heirloom currencies deferred |
| **A-EMBARK** | `EmbarkButton` (bottom-left) | `.estate-embark-button` | Button label, position, active state match |
| **A-SIDEBUTTONS** | `SideButtons` (6 buttons, bottom-right) | (deferred) | Not yet in frontend |
| **A-BUILDING-POSITIONS** | All 11 building `anchoredPosition` values | All 11 building `left`/`top` via `estateLeft`/`estateTop` | Each building node within 2px of expected position at 1920×1080 |
| **A-BUILDING-SIZES** | All 11 building `sizeDelta` values | All 11 building `width`/`height` | Each building node size matches Unity sizeDelta |
| **A-LABEL-OFFSETS** | `BuildingLabel` child positions | `.estate-building-banner` positions | Label banner offset from building center matches BuildingLabel offset |
| **A-SPRITE-RESOLVE** | Building sprite GUIDs → PNG | `resolveBuildingImage(id)` → PNG | Each of 6 scoped buildings resolves a non-fallback sprite |
| **A-ROSTER** | `RosterPanel → RosterScroll` | `.estate-roster-strip` | Hero cards with HP/stress bars, portrait, level |
| **A-BACKGROUND** | Camera clear color / 3D environment | CSS gradient layers | Dark warm-tone backdrop with elliptical glow |

### 5.2 Parity Gate Criteria

For each anchor point:

1. **Positional parity:** Frontend element position matches Unity source position (±2px at 1280×720 reference, ±4px at 1920×1080 reference).
2. **Dimensional parity:** Frontend element size matches Unity source sizeDelta (±2px).
3. **Label parity:** Building display names match the DDGC Chinese labels catalogued in section 4.2.
4. **Sprite parity:** For the 6 scoped buildings, the correct sprite PNG is loaded (not a fallback letter).
5. **State parity:** Building status (ready/partial/locked) and hero status (wounded/afflicted) map correctly from the TownViewModel.

---

## 6. Asset Inventory Status

### 6.1 Extracted (present in `frontend/public/original/`)
- 6 of 6 scoped building sprites: stagecoach, guild, blacksmith, sanitarium, abbey, tavern
- 3 of 15 hero portraits: hunter family only (base, white variant, black variant)

### 6.2 Missing (deferred)
- 5 additional building sprites: graveyard, garden, legacytower, market, campingtrainer — use CSS letter fallback in frontend
- 12 of 15 hero portraits: alchemist, diviner, shaman, tank families (all 3 variants each) — use CSS letter avatars
- ~300 UI chrome sprites (buttons, panels, bars, badges from `Assets/Sprites/ui/`)
- All Spine model assets (16 hero class model variants)
- 5 heirloom currency icons (bust, portrait, deed, crest)
- Title menu sprites (background, button textures)
- Stress pip sprites

### 6.3 Blockers
1. **BLOCKER-001:** Original Unity `.png`/`.asset` files not in repository — requires Unity source project access for extraction
2. **BLOCKER-002:** Spine runtime integration not configured for web — character model display deferred
3. **BLOCKER-003:** GUID-to-asset-path resolution requires Unity source project to resolve asset references
4. **BLOCKER-004:** Menu background and button PNG sprites not extracted

---

## 7. Hierarchy Fidelity Summary

| Aspect | Unity Source | Frontend (from buildingCatalog.ts) | Fidelity |
|--------|-------------|-------------------------------------|----------|
| Guild pos | (120, -50) | (120, -50) | exact |
| Tavern pos | (-650, -220) | (-650, -220) | exact |
| Graveyard pos | (695.5, 95.63) | (696, 96) | rounded |
| Blacksmith pos | (709, 294) ⚠️ | (709, 294) | exact (anchor anomaly) |
| StageCoach pos | (31, -267) | (31, -267) | exact |
| Garden pos | (-245, -211) | (-245, -211) | exact |
| LegacyTower pos | (0, 270) | (0, 270) | exact |
| Abbey pos | (-330, 90) | (-330, 90) | exact |
| Market pos | (340.5, -222) | (340.5, -222) | exact |
| Sanitarium pos | (390, 110) | (390, 110) | exact |
| CampingTrainer pos | (-607, 220) | (-607, 220) | exact |
| Label offsets | per-building | per-building (from BuildingLabel) | exact |
| Label banner size | (326, 48) Unity standard | variable (min 9rem, max 13rem) CSS | approximate |
| Reference canvas | 1280×720 Unity native | 1920×1080 frontend | scaled |
| Building sprite (6 scoped) | GUID-referenced PNGs | extracted PNGs in `/original/buildings/` | source-faithful |
| Building sprite (5 deferred) | GUID-referenced PNGs | CSS letter fallback | fallback |
| Hero portrait (hunter) | 3 variant PNGs | extracted PNGs in `/original/heroes/` | source-faithful |
| Hero portrait (other) | not extracted | CSS letter avatar | fallback |
| SideButtons (6 nav) | present in Unity | not implemented in frontend | gap |
| Currency (heirlooms) | 5-currency panel | single gold pill | gap |
| QuickProgressButton | (389, 137), 97×42 | not implemented | gap |
| QuickStartButton | (-324, 137), 97×42 | not implemented | gap |

---

## 8. Recommendations for Reconstruction

1. **Blacksmith anchor fix:** The Blacksmith uses a bottom-anchored RectTransform in Unity (anchorMin=(0.5,0)). On the 1280×720 canvas, its effective center-relative Y is -66 (294 - 360). The frontend catalog records the raw value y=294. Either normalize this to the center-relative coordinate system or document the anchor exception explicitly.

2. **Building label banner:** The Unity BuildingLabel has a fixed sizeDelta of (326, 48) across all buildings. The frontend CSS uses flexible sizing (min-width: 9rem, max-width: 13rem). For pixel-perfect parity, constrain the banner to 326×48 at the 1920×1080 reference scale.

3. **QuickStart/QuickProgress buttons:** These two utility buttons (at (389, 137) and (-324, 137)) are not yet in the frontend catalog. They appear to be fast-path buttons for experienced players.

4. **Six side navigation buttons:** The SideButtons panel (ActivityLog, RealmInventory, Hero, TownEvent, Settings, Glossary) is present in Unity but not yet reflected in the frontend town shell.

5. **Five deferred building sprites:** The buildings graveyard, garden, legacytower, market, and campingtrainer have no extracted sprites. They currently render as CSS letter fallbacks in the frontend. Their Unity sprite GUIDs are known from the asset manifest and can be extracted when Unity source project access is available.

6. **Heirloom currency panel:** The full 5-currency display (Bust, Portrait, Deed, Crest, Gold) should be documented as a deferred feature. The current single-gold-pill is a simplification for the scoped UI redo.
