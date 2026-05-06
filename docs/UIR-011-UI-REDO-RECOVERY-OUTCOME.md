# UIR-011: UI Redo — Unity Asset and Layout Parity Outcome

**Date:** 2026-05-06
**Task:** UIR-011
**Phase:** Phase 10 — Town / Meta Surface Expansion
**Predecessors:** UIR-003 (hierarchy brief), UIR-004 (asset manifest), UIR-005–UIR-010 (screen migration + smoke)

---

## 1. Unity Scenes / Prefabs / Assets Inspected and Reused

### 1.1 Scenes

| Scene | File | Status |
|-------|------|--------|
| EstateManagement.unity | `Assets/Scenes/EstateManagement.unity` | **Fully inspected.** 2693 asset references, 424 text nodes, 364 prefab instances. Single-canvas scene containing all town estate, provisioning, quest-select, and shared overlay surfaces. No scene file was copied; its hierarchy was used as the layout reference for all frontend screens. |

### 1.2 Prefabs Inspected (28 total)

The following prefabs were extracted into fixture JSON at `fixtures/ui_inventory/Assets/Prefabs/` and their hierarchy patterns mapped to frontend components:

| Prefab | Frontend Mapped To |
|--------|-------------------|
| `CharacterWindow.prefab` | `HeroDetailScreen.tsx` — tabbed hero inspection (equipment, skills, info, resistances, quirks) |
| `MainMenuWindow.prefab` | `StartupScreen.tsx` — replay/live boot buttons |
| `HeroSlot.prefab` | `TownShellScreen.tsx` — roster hero cards |
| `Controls/HeroSlot.prefab` | `TownShellScreen.tsx` — detailed hero controls |
| `Controls/PartyHeroSlot.prefab` | `ProvisioningScreen.tsx` — party slot cards |
| `Controls/TrinketRow.prefab` | `HeroDetailScreen.tsx` — trinket equipment display |
| `MarketSlot.prefab` | BuildingScreenRouter generic screens |
| `ShopSlot.prefab` | BuildingScreenRouter generic screens |
| `SkillUpgradeSlot.prefab` | `GuildBuildingScreen.tsx` — skill upgrade cards |
| `UpgradeSlot.prefab` | `GuildBuildingScreen.tsx`, `BlacksmithBuildingScreen.tsx` — upgrade displays |
| `EquipmentUpgradeSlot.prefab` | `BlacksmithBuildingScreen.tsx` — equipment upgrade |
| `PartyInventorySlot.prefab` | `ProvisioningScreen.tsx` — inventory grid |
| `PartyInventorySlotInDungeon.prefab` | `ExpeditionScreen.tsx` — in-dungeon inventory |
| `HeroResultSlot.prefab` | `ResultScreen.tsx` — per-hero outcome cards |
| `LootSlot.prefab` | `ResultScreen.tsx` — loot list |
| `OverlaySlot.prefab` | `ExpeditionScreen.tsx` — expedition overlay |
| `SanitariumWindow.prefab` | `SanitariumBuildingScreen.tsx` — quirk/disease treatment UI |

**Unmapped prefabs (not in scoped screens):**
- `TreatmentHeroSlot.prefab` — Sanitarium treatment UI (not part of frontend scope)
- `DeathRecord.prefab` — Graveyard death record (not scoped)
- `Exchange.prefab` — Heirloom exchange (not scoped)
- `ActivityRecord.prefab`, `Goal.prefab`, `WeekBlock.prefab` — Activity log panels (future scope)
- `GlossaryRecord.prefab` — Glossary panel (future scope)
- `HallSectorSlot.prefab`, `HallwaySlot.prefab`, `RoomSlot.prefab` — Dungeon map prefabs (expedition map not scoped)
- `GameSetup.prefab` — Game setup controls (not scoped)

### 1.3 C# Scripts Referenced

Key manager and window scripts analyzed for behavior and panel decomposition:
- `CharacterWindow.cs` — tab panel layout, stress display, stat panels
- `EstateSceneManager.cs` — scene root organization, panel activation
- `BuildingWindow.cs`, `UpgradableBuildingWindow.cs` — building window pattern
- `StageCoachWindow.cs`, `GuildHeroWindow.cs`, `BlacksmithHeroWindow.cs`, `SanitariumWindow.cs` — specialized building screens
- `RaidResultWindow.cs`, `ResultHeroWindow.cs`, `ResultItemWindow.cs` — result/return flow
- `ProvisioningManager.cs`, `PartyFormationManager.cs` — provisioning flow
- `StressPanel.cs`, `ResistancesPanel.cs`, `QuirksPanel.cs`, `CharEquipmentPanel.cs` — hero detail sub-panels

### 1.4 Extracted Original Sprites

Unity original sprites have been extracted into `frontend/public/original/` as PNG assets served directly by the Vite dev server. The extraction covers three categories: buildings (full set), shell chrome, and hero portraits (partial).

**Building sprites (11/11 town catalog entries — full set):**

| Building | Original Asset | GUID | Frontend Path |
|----------|---------------|------|---------------|
| Stagecoach | `building_perception_tower.png` | `87a55679f12a1e6489ecdb1d6e6f6b93` | `/original/buildings/building_perception_tower.png` |
| Guild | `building_train_field.png` | `67a5e7aed8029d84dbf9c9e497a944d2` | `/original/buildings/building_train_field.png` |
| Blacksmith | `building_forging.png` | `23e01c10f262ddc4ba9977b91314b031` | `/original/buildings/building_forging.png` |
| Sanitarium | `building_cell_repair.png` | `55375034893560044a266e905926e8ff` | `/original/buildings/building_cell_repair.png` |
| Abbey | `building_faith_altar.png` | `311540f167839cf4da00305566192b4a` | `/original/buildings/building_faith_altar.png` |
| Tavern | `building_paradise.png` | `c0ea280d2704bdb4a9621d6e181e0316` | `/original/buildings/building_paradise.png` |
| Market | `building_market.png` | `8c3505ec8c7e25b42aacdcaece82f821` | `/original/buildings/building_market.png` |
| Graveyard | `building_hero_archive.png` | — | `/original/buildings/building_hero_archive.png` |
| Legacy Tower (Museum) | `building_legacy_tower.png` | `ebd56d507c80af54986327ac7cb16661` | `/original/buildings/building_legacy_tower.png` |
| Camping Trainer | `building_space_analysis.png` | — | `/original/buildings/building_space_analysis.png` |
| Inn | `building_garden.png` | `c34c4012911d41c4bbed2328d7025138` | `/original/buildings/building_garden.png` |

Building icons are rendered via `BuildingIcons.tsx` which calls `resolveBuildingImage(buildingId)` from `originalAssetPaths.ts`. When the asset is present, an `<img>` element is rendered with the sprite; otherwise a CSS-letter fallback is shown. Each `<img>` carries the CSS class `building-icon-image`.

All 11 building sprites were staged in two batches:
- **UIR-007 core set (6):** stagecoach, guild, blacksmith, sanitarium, abbey, tavern — the scoped buildings with dedicated frontend screens
- **UIR-005B deferred stage (5):** market, graveyard, legacy tower, camping trainer, inn — sprites present in the repo and resolvable by `BuildingIcons.tsx`, but their screens remain generic fallback (see §2.3)

**Shell chrome sprites (10 extracted):**

| Chrome Element | Original Asset | Frontend Path |
|----------------|---------------|---------------|
| Estate nameplate background | `estate_name_bg.png` | `/original/chrome/estate_name_bg.png` |
| Gold currency icon | `gold.png` | `/original/chrome/gold.png` |
| Embark button | `btn_play.png` | `/original/chrome/btn_play.png` |
| Side nav button (save) | `btn_save_01.png` | `/original/chrome/btn_save_01.png` |
| Side nav button (save alt) | `btn_save_02.png` | `/original/chrome/btn_save_02.png` |
| Close button | `btn_close.png` | `/original/chrome/btn_close.png` |
| Building label background | `building_label_bg01.png` | `/original/chrome/building_label_bg01.png` |
| Building icon background | `building_icon_bg.png` | `/original/chrome/building_icon_bg.png` |
| Building title background | `building_title_bg.png` | `/original/chrome/building_title_bg.png` |
| Building info background | `building_info_bg.png` | `/original/chrome/building_info_bg.png` |

Chrome sprites are wired via CSS `background-image` with CSS custom property overrides (see `frontend/src/originalAssets.css` and `frontend/src/townEstateLayout.css`). The resolver in `originalAssetPaths.ts` provides `resolveChromeAsset()` for TypeScript consumers (e.g., side nav icon rendering in `TownShellScreen.tsx`).

**Hero portrait sprites (3/15 extracted):**

| Hero | Variant | Frontend Path |
|------|---------|---------------|
| Hunter | Base (chaos 50–149) | `/original/heroes/hunter_portrait_roster.png` |
| Hunter | White variant (chaos ≥ 150) | `/original/heroes/hunter1_portrait_roster.png` |
| Hunter | Black variant (chaos < 50) | `/original/heroes/hunter2_portrait_roster.png` |

Hero portraits are resolved via `resolveHeroPortrait()`. Extracted portraits render as `<img>` with class `roster-portrait-image`; missing portraits fall through to a CSS initial-letter avatar. Only the Hunter family (3 of 15 portrait slots) has been extracted; Alchemist, Diviner, Shaman, and Tank families are deferred.

Each extracted image carries `data-asset-path` and `data-guid` attributes on its parent element for traceability.

### 1.5 UI Chrome Patterns Reused (CSS-Only)

The following Unity chrome patterns remain CSS-only (no extracted sprite in the repo):

| Unity Pattern | Frontend CSS Equivalent |
|--------------|------------------------|
| `btn_*.png` button sprites (other than btn_play/btn_save/btn_close) | `.action-primary`, `.action-secondary` CSS buttons with accent gradient |
| `hero_slot_bg.png` | `.roster-hero` dark semi-transparent card |
| `item_slot.png` | `.surface-card` / `.party-slot` panel styling |
| `upgrade_slot.png` / `upgrade_locked.png` | CSS `.status-locked` styling |
| `stress.*.png` (3 states) | `.stress-pip--normal/stressed/overstressed` CSS classes |
| `dialog02.png`, `dialog07.png` | `.panel` background with `--panel-bg` variable |
| `char_bg.png` | `.hero-portrait-frame` gradient background |
| `lowwindow_bg.png` | `.details-overlay` backdrop blur panel |
| `building_label_bg.png`, `building_title_bg.png` | `.building-icon-label` / `.viewport-title` styled text |
| Building icons (11 `building_*.png`) | **Extracted — reuses original PNG via `<img>`** (see §1.4) |
| Shell chrome (10 items) | **Extracted — wired via CSS `background-image`** (see §1.4) |
| Hero portraits (3 `*_portrait_roster.png`) | **Partially extracted — Hunter family only** (see §1.4) |

---

## 2. Missing Original Assets — Deferred Gaps and Blockers

### 2.1 Critical Blockers (from UIR-004 Asset Manifest)

These are documented in the [asset manifest](../frontend/src/assets/asset-manifest.json) and remain unresolved:

| Blocker | Severity | Description | Impact |
|---------|----------|-------------|--------|
| BLOCKER-001 | Medium (diminishing) | Original Unity `.png` sprite files incomplete — 11/11 building sprites, 10/10 staged chrome sprites, and 3/15 hero portraits extracted | All building icons use original PNGs via `<img>`; shell chrome uses original sprites via CSS `background-image`; hero portraits partially rendered (Hunter only); remaining hero families (Alchemist, Diviner, Shaman, Tank) fall back to CSS-letter avatars. Original estimate of ~300 UI chrome items no longer accurate — the 10 extracted chrome sprites cover the essential shell chrome for the scoped screens; remaining chrome is CSS-equivalent patterns (see §1.5). |
| BLOCKER-002 | High | Spine runtime not configured for web | HeroDetailScreen cannot render original Spine skeletal animations; placeholder initial-letter avatars used instead |
| BLOCKER-003 | High | GUID-to-asset-path requires Unity source | Cannot automate asset extraction; manual copy needed from `DreamDeveloperGame-Crossover/Assets` |

### 2.2 Deferred Asset Enumerations

These are non-blocking for the scoped UI but tracked for future phases:

| Item | Reason | Blocker |
|------|--------|---------|
| Hero portrait full enumeration (5 families × 3 variants) | 3/15 extracted (Hunter variants); Alchemist, Diviner, Shaman, Tank deferred | No |
| Trinket icon full enumeration (~10 trinkets) | Deferred until trinket UI is scoped | No |
| Provision icon full enumeration | Deferred until provisioning UI details are designed | No |
| Camping skill icons | No icon sprites scoped yet | No |
| Dungeon map assets (HallSector, Hallway, Room prefabs) | Expedition map UI not yet designed | No |
| Font (`ZhiYiSongTi-Regular.ttf`) | CSS uses system font stack with `"Noto Sans SC"` fallback; original font not extracted | No |

### 2.3 Deferred Screens (Unscoped Buildings)

Five buildings exist in EstateManagement.unity but have no dedicated frontend screen:

| Building | Unity Scene Presence | Sprite Status | Screen Status |
|----------|---------------------|---------------|---------------|
| Market | Present in `UI_Estate` layer | Extracted | Deferred — renders via `BuildingScreenRouter` generic fallback |
| Graveyard | Present with `DeathRecord.prefab` | Extracted | Deferred — renders via `BuildingScreenRouter` generic fallback |
| Museum (Legacy Tower) | Present | Extracted | Deferred — renders via `BuildingScreenRouter` generic fallback |
| Provisioner | Present | No distinct sprite identified | Deferred — renders via `BuildingScreenRouter` generic fallback |
| Inn | Present | Extracted | Deferred — renders via `BuildingScreenRouter` generic fallback |

Sanitarium was originally deferred but now has a dedicated `SanitariumBuildingScreen.tsx` (see screen table in §5). All 5 deferred buildings have their sprites ready in the repo (except Provisioner which has no distinct sprite in Unity source either). No specialized UI beyond `BuildingScreenRouter` generic fallback exists for these buildings.

---

## 3. Local Run, Smoke, and Browser Acceptance Commands

### 3.1 Frontend Development

```bash
cd frontend

# Install dependencies (first time or after package.json changes)
npm install

# Start dev server → http://localhost:4179
npm run dev

# Typecheck
npm run typecheck

# Production build
npm run build

# Unit tests (vitest, no browser needed)
npm run test

# Validation smoke tests (replay fixture decoding + build-run contracts)
npm run smoke

# Browser acceptance tests (Playwright, requires build first)
npm run build && npm run smoke-browser

# Full pipeline: build + validation smoke + browser smoke
npm run smoke-build
```

### 3.2 Browser Smoke Coverage

The Playwright smoke suite is split into two directories:

**`frontend/smoke/` — Fidelity gate tests (maintained by UIR-010):**

Single spec file `browserSmoke.spec.ts` covering the full replay boot → meta-loop → live boot path:

1. **Replay boot** → Town shell with campaign info, gold, roster heroes (Shen, Bai Xiu, Hei Zhen), and building labels (Stagecoach, Guild, Blacksmith, Sanitarium)
2. **Town viewport verification**: `.estate-top-panel` (EstateNameplate + CurrencyPanel), `.estate-stage-shell` (building collection), `.estate-bottom-panel` (EmbarkButton + RosterPanel)
3. **Original asset image checks**: building icons served from `/original/buildings/`, hero roster portraits from `/original/heroes/`
4. **Hero detail screen** with tab navigation (装备, 战斗技能, 状态, 信息, 扎营技能)
5. **Building detail** (Guild) with action buttons
6. **Full provisioning → expedition launch → result → return → town loop**
7. **Town re-entry** after loop (campaign persistence)
8. **Live boot** → Town shell with Fresh Campaign data
9. **Live hero detail** from live bridge
10. **Live building detail** (Stagecoach) from live bridge
11. **Full live provisioning → expedition → result → return flow**
12. **Fidelity gates**: All completed product surfaces checked against blocklist patterns: `placeholder`, `skeletal`, `skeleton`, `reserved canvas`, `text.based rendering`, `rendering completion`
13. **Landscape viewport classes**: `.town-viewport`, `.expedition-viewport`, `.viewport-hud`, `.viewport-roster`
14. **No page errors or console errors** at any phase

**`frontend/smoke-cap/` — Screenshot capture tests (UIR-005G, UIR-008, UIR-009):**

Five spec files for visual regression capture and flow verification:

| Spec | Purpose |
|------|---------|
| `townCapture.spec.ts` | Replay + live town screenshot captures at `/tmp/uir-005g-cap/` (UIR-005G parity review) |
| `uir008Capture.spec.ts` | Provisioning and expedition launch screen captures (UIR-008 visual review) |
| `uir008Detail.spec.ts` | Detailed provisioning with hero selection, expedition viewport capture (UIR-008 deep review) |
| `uir008Verify.spec.ts` | End-to-end provisioning + launch flow: verifies `.party-formation`, `.launch-primary`, roster hero interaction, expedition title, Return to Town, and asserts zero console/page errors |
| `uir009Capture.spec.ts` | Full meta-loop screenshot capture: result screen (Victory + Proceed to Return), return screen (Expedition Log Closed + Resume Town Activities), town re-entry (`城镇中枢`), and loop-complete screenshot |

### 3.3 Rust Backend Verification

```bash
# From project root

# Typecheck
cargo check

# Run all tests
cargo test

# Contracts smoke tests
cargo test --test contracts_smoke_tests

# State smoke tests
cargo test --test state_smoke_tests

# Docs smoke tests
cargo test --test docs_smoke_tests

# Build-run smoke tests
cargo test --test build_run_smoke

# Replay-driven validation tests
cargo test --test replay_driven_validation_tests

# Session surface render tests
cargo test --test session_surface_render_tests
```

### 3.4 Acceptance Criteria for Browser Tests

The Playwright suite uses the following explicit acceptance markers:
- **Timeouts:** 90s per test, 8s for viewport selectors
- **Viewport:** 1440×900 (landscape 16:9); capture tests use 1600×1000
- **Base URL:** `http://localhost:4179` (production preview server)
- **Headless:** true
- **Error collectors:** Both `console.error` and `page.error` are captured and asserted empty at each phase boundary

---

## 4. Why the Final UI No Longer Qualifies as Generic Web-Page Layout

### 4.1 Non-Negotiable Layout Rules (from UIR-003 §3.4)

The frontend explicitly rejects the following generic web patterns:

| Generic Pattern | Rejection Rationale | Frontend Enforcement |
|----------------|-------------------|---------------------|
| Sidebar nav with hamburger menu | Original Unity UI has no sidebar; persistent bottom-bar navigation mirrors `UI_Shared → BottomPanel → SideButtons` | `.viewport-hud` top bar + `.viewport-roster` bottom bar; no `<nav>` sidebar |
| Card-wall of stats widgets / KPI dashboard | The town surface is a game viewport, not a data dashboard | `.town-viewport` with landscape game surface (`PixiStage`) and positioned building icons |
| Infinite scroll / long scrolling page | Primary content fits viewport; scrolling restricted to roster lists and skill panels | `.town-viewport` = `height: 100vh; overflow: hidden`; only `.roster-scroll` has `overflow-x: auto` |
| Mobile-first vertical stack | Unity targets 1280×720 fixed canvas; frontend targets landscape-first with responsive collapse only below 920px | CSS uses `@media (max-width: 920px)` for narrow-viewport adaptation, but primary layout is horizontal |
| Spreadsheet-style `<table>` grids | Building and hero data uses panel/card composition (mirroring Unity's modal windows) | No `<table>` elements; all data rendered as `.surface-card`, `.equipment-slot`, `.panel` grid items |
| Full-page router with URL-based navigation | Screen transitions are state-machine driven, not URL-driven; each screen is a modal frame cycle | `FlowController.resolveScreen()` maps flow state → component; no React Router-style URL routing |

### 4.2 Landscape-First Design Principles

- **Viewport classes:** `.town-viewport`, `.expedition-viewport` use `display: flex; height: 100vh` — full-screen landscape game views, not web pages
- **Composition:** Top HUD (`viewport-hud`) + game surface (`game-surface`) + bottom roster bar (`viewport-roster`) mirrors Unity's `UI_Estate` + `UI_Shared` persistent shell pattern
- **Building window pattern:** Two-column left/right panel split mirrors `BuildingWindow.cs` LeftPanel (character + talk) / RightPanel (service + upgrade) convention
- **Modal overlays:** Hero detail and building screens mount/unmount via SolidJS `<Switch>`/`<Match>` — exactly matching Unity's `GameObject.SetActive` window-on-demand pattern
- **No URL routing:** Screen identity is derived from `DdgcFrontendSnapshot.flowState`, not from window.location

### 4.3 Frame-Based Composition vs. Web Page

Each screen is a **distinct frame in a lifecycle** (town → provisioning → expedition → result → return → town), not a hyperlinked page:

```
      ┌─────────┐
      │  Town   │ ◄──────────────────────┐
      └────┬────┘                        │
           │                              │
      ┌────▼────────┐                   │
      │ Provisioning │                   │
      └────┬────────┘                   │
           │                              │
      ┌────▼────────┐                   │
      │ Expedition  │                   │
      └────┬────────┘                   │
           │                              │
      ┌────▼──────┐                     │
      │  Result   │                     │
      └────┬──────┘                     │
           │                              │
      ┌────▼──────┐                     │
      │  Return   │─────────────────────┘
      └───────────┘
```

This is enforced by `FlowController.resolveScreen()` and `canTransition()` — no screen can be accessed out of order.

---

## 5. UIR Series Inventory Summary

| Task | Description | Files Created/Modified |
|------|-------------|----------------------|
| UIR-001 | *(implied by series start)* | — |
| UIR-002 | Extract Unity UI inventories | 28 prefab JSON fixtures in `fixtures/ui_inventory/` |
| UIR-003 | Write Unity hierarchy and layout migration brief | `docs/plans/2026-05-04-ddgc-town-meta-unity-ui-brief.md` |
| UIR-004 | Create original asset manifest | `frontend/src/assets/asset-manifest.json` |
| UIR-004A | Rewrite startup title page from original menu UI | `StartupScreen.tsx`, `components/layout/AppFrame.tsx`, `components/layout/MenuFrame.tsx` |
| UIR-005 | Implement landscape town game viewport | `screens/town/TownShellScreen.tsx`, `render/PixiStage.tsx`, `.town-viewport` CSS |
| UIR-005A | Implement town shell layout from Unity estate | `TownShellScreen.tsx` rewrite, `.estate-*` CSS classes |
| UIR-005B | Stage town/base shell chrome and source assets | 5 deferred building sprites + 10 chrome sprites → `frontend/public/original/`; `originalAssetPaths.ts` chromePaths resolver; `originalAssets.css` with CSS custom property wiring; asset-manifest.json updated to v1.1.0 |
| UIR-005C | Reconstruct town/base screen from Unity hierarchy | Screen composition, `EstateNameplate` + `CurrencyPanel` + `BuildingCluster` decomposition |
| UIR-005D | Validate town/base screenshot parity and entry flow | Flow tests, replay fixture alignment |
| UIR-005E | Align building labels, local chrome, and click regions | Building label positioning, click region sizing |
| UIR-005F | Integrate town/base composition and parity polish | Composition refinement, edge case handling |
| UIR-005G | Verify town/base entry flow and runtime target | Screenshot capture specs in `frontend/smoke-cap/townCapture.spec.ts` |
| UIR-005H | Review town/base screenshot parity anchors | Parity anchor review, screenshot diffing |
| UIR-006 | Migrate roster and hero detail UI | `screens/town/HeroDetailScreen.tsx`, `screens/town/BuildingDetailScreen.tsx`, `contractTypes.ts` HeroDetailViewModel |
| UIR-007 | Migrate town building UI with original assets | `screens/town/BuildingScreenRouter.tsx`, `screens/town/buildings/BuildingIcons.tsx` (rewritten with extracted PNGs), `screens/town/buildings/SanitariumBuildingScreen.tsx`, `assets/originalAssetPaths.ts`, 6 building screen components |
| UIR-008 | Migrate provisioning and launch flow | `ProvisioningScreen.tsx`, `ExpeditionScreen.tsx`; capture/verify specs in `frontend/smoke-cap/uir008*.spec.ts` |
| UIR-009 | Migrate result and return loop | `ResultScreen.tsx`, `ReturnScreen.tsx`, `ResultReturnFlow.test.ts`; capture spec in `frontend/smoke-cap/uir009Capture.spec.ts` |
| UIR-010 | Add browser smoke coverage for fidelity gates | `frontend/smoke/browserSmoke.spec.ts` — 2 tests covering replay + live meta-loop, original asset image assertions, fidelity blocklist checks, landscape viewport verification |
| **UIR-011** | **Document Unity asset and layout parity outcome** | **This document** — final reconciliation of asset extraction, layout parity, screen completion, and smoke coverage |

### Frontend Screen Completion Status

| Screen | Status | Unity Precedent | Fidelity Gate |
|--------|--------|----------------|---------------|
| `StartupScreen` | ✅ Complete | `MainMenuWindow.prefab` | Exempt (boot surface) |
| `TownShellScreen` | ✅ Complete | `UI_Estate` + `UI_Shared` | Passes fidelity blocklist |
| `HeroDetailScreen` | ✅ Complete | `CharacterWindow.prefab` | Passes fidelity blocklist |
| `BuildingScreenRouter` | ✅ Complete | `UI_LowWindows/*Window` | Passes fidelity blocklist |
| `StagecoachBuildingScreen` | ✅ Complete | `StageCoachWindow.cs` | Passes fidelity blocklist |
| `GuildBuildingScreen` | ✅ Complete | `GuildHeroWindow.cs` | Passes fidelity blocklist |
| `BlacksmithBuildingScreen` | ✅ Complete | `BlacksmithHeroWindow.cs` | Passes fidelity blocklist |
| `SanitariumBuildingScreen` | ✅ Complete | `SanitariumWindow.cs` + `TreatmentHeroSlot.prefab` | Passes fidelity blocklist |
| `ProvisioningScreen` | ✅ Complete | `UI_Provision` + `PartyFormationManager` | Passes fidelity blocklist |
| `ExpeditionScreen` | ✅ Complete | `SelectedQuestPanel` + `RaidPreparationManager` | Passes fidelity blocklist |
| `ResultScreen` | ✅ Complete | `RaidResultWindow.cs` + `HeroResultSlot` | Passes fidelity blocklist |
| `ReturnScreen` | ✅ Complete | `ResultHeroWindow.cs` + `ResultItemWindow.cs` | Passes fidelity blocklist |
| `FatalErrorScreen` | ✅ Complete | — | Exempt (error surface) |
| `UnsupportedStateScreen` | ✅ Complete | — | Exempt (error surface) |

---

## 6. UIR-004 Asset Manifest Updates

The asset manifest at `frontend/src/assets/asset-manifest.json` remains the canonical reference for all catalogued asset references, 28 inventoried prefabs, and 3 blockers. Key updates since UIR-004:

- All 11 town building catalog entries have confirmed sprite paths and GUIDs (6 scoped + 5 deferred staged in UIR-005B)
- 10 shell chrome sprites extracted and wired via CSS `background-image` (UIR-005B)
- Spine skeleton data for 5 hero families catalogued (partial GUID coverage)
- 24 UI chrome assets confirmed (10 extracted, remaining as CSS-equivalent patterns)
- 4 confirmed item icons (provisions + trinket samples)
- 3 stress pip states documented
- 5 deferred items enumerated
- **Final asset extraction status: 11 building sprites + 10 chrome sprites + 3 hero portraits extracted** — see `frontend/public/original/`

---

## 7. References

| Document | Location |
|----------|----------|
| Unity Hierarchy & Layout Migration Brief | `docs/plans/2026-05-04-ddgc-town-meta-unity-ui-brief.md` |
| Asset Manifest | `frontend/src/assets/asset-manifest.json` |
| Original Asset Paths Resolver | `frontend/src/assets/originalAssetPaths.ts` |
| Extracted Sprite Assets | `frontend/public/original/` (11 building PNGs, 10 chrome PNGs, 3 hero portrait PNGs) |
| Prefab Inventory Index | `fixtures/ui_inventory/index.json` |
| Frontend README | `frontend/README.md` |
| Browser Smoke Tests (fidelity gates) | `frontend/smoke/browserSmoke.spec.ts` |
| Capture Tests (visual regression) | `frontend/smoke-cap/` (5 specs: townCapture, uir008Capture, uir008Detail, uir008Verify, uir009Capture) |
| Frontend Source | `frontend/src/` |
| Project README | `README.md` |
| Migration Blockers | `MIGRATION_BLOCKERS.md` |
| Semantic Gap Matrix | `SEMANTIC_GAP_MATRIX.md` |
