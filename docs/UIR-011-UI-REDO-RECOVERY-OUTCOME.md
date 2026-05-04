# UIR-011: UI Redo — Unity Asset and Layout Parity Outcome

**Date:** 2026-05-04
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
- `StageCoachWindow.cs`, `GuildHeroWindow.cs`, `BlacksmithHeroWindow.cs` — specialized building screens
- `RaidResultWindow.cs`, `ResultHeroWindow.cs`, `ResultItemWindow.cs` — result/return flow
- `ProvisioningManager.cs`, `PartyFormationManager.cs` — provisioning flow
- `StressPanel.cs`, `ResistancesPanel.cs`, `QuirksPanel.cs`, `CharEquipmentPanel.cs` — hero detail sub-panels

### 1.4 Building Sprites Reused (via Inline SVG Icons)

Six scoped buildings have inline SVG icons in `BuildingIcons.tsx` that reference the original Unity `.png` sprite paths and GUIDs. These serve as faithful abstract representations until the original sprites can be extracted from the Unity project:

| Building | Original Asset | GUID | Frontend Icon |
|----------|---------------|------|---------------|
| Stagecoach | `building_perception_tower.png` | `87a55679f12a1e6489ecdb1d6e6f6b93` | Conic spire SVG |
| Guild | `building_train_field.png` | `67a5e7aed8029d84dbf9c9e497a944d2` | Dojo gate SVG |
| Blacksmith | `building_forging.png` | `23e01c10f262ddc4ba9977b91314b031` | Anvil/forge SVG |
| Sanitarium | `building_cell_repair.png` | `55375034893560044a266e905926e8ff` | Medical cross SVG |
| Abbey | `building_faith_altar.png` | `311540f167839cf4da00305566192b4a` | Chapel spire SVG |
| Tavern | `building_paradise.png` | `c0ea280d2704bdb4a9621d6e181e0316` | Mug SVG |

Each SVG carries `data-asset-path` and `data-guid` attributes for future automated sprite replacement.

### 1.5 UI Chrome Patterns Reused (CSS-Only)

| Unity Pattern | Frontend CSS Equivalent |
|--------------|------------------------|
| `btn_*.png` button sprites | `.action-primary`, `.action-secondary` CSS buttons with accent gradient |
| `hero_slot_bg.png` | `.roster-hero` dark semi-transparent card |
| `item_slot.png` | `.surface-card` / `.party-slot` panel styling |
| `upgrade_slot.png` / `upgrade_locked.png` | CSS `.status-locked` styling |
| `stress.*.png` (3 states) | `.stress-pip--normal/stressed/overstressed` CSS classes |
| `dialog02.png`, `dialog07.png` | `.panel` background with `--panel-bg` variable |
| `char_bg.png` | `.hero-portrait-frame` gradient background |
| `lowwindow_bg.png` | `.details-overlay` backdrop blur panel |
| `building_label_bg.png`, `building_title_bg.png` | `.building-icon-label` / `.viewport-title` styled text |

---

## 2. Missing Original Assets — Deferred Gaps and Blockers

### 2.1 Critical Blockers (from UIR-004 Asset Manifest)

These are documented in the [asset manifest](../frontend/src/assets/asset-manifest.json) and remain unresolved:

| Blocker | Severity | Description | Impact |
|---------|----------|-------------|--------|
| BLOCKER-001 | Critical | Original Unity `.png` sprite files not in repository | Frontend uses CSS-only dark theme with inline SVGs; no raster sprites for buildings, items, or portraits |
| BLOCKER-002 | High | Spine runtime not configured for web | HeroDetailScreen cannot render original Spine skeletal animations; placeholder initial-letter avatars used instead |
| BLOCKER-003 | High | GUID-to-asset-path requires Unity source | Cannot automate asset extraction; manual copy needed from `DreamDeveloperGame-Crossover/Assets` |

### 2.2 Deferred Asset Enumerations

These are non-blocking for the scoped UI but tracked for future phases:

| Item | Reason | Blocker |
|------|--------|---------|
| Hero portrait full enumeration (5 families × 3 variants) | Path pattern known but extraction deferred | No |
| Trinket icon full enumeration (~10 trinkets) | Deferred until trinket UI is scoped | No |
| Provision icon full enumeration | Deferred until provisioning UI details are designed | No |
| Camping skill icons | No icon sprites scoped yet | No |
| Dungeon map assets (HallSector, Hallway, Room prefabs) | Expedition map UI not yet designed | No |
| Font (`ZhiYiSongTi-Regular.ttf`) | CSS uses system font stack with `"Noto Sans SC"` fallback; original font not extracted | No |

### 2.3 Deferred Screens (Unscoped Buildings)

Six buildings exist in EstateManagement.unity but have no dedicated frontend screen:

| Building | Unity Scene Presence | Status |
|----------|---------------------|--------|
| Market | Present in `UI_Estate` layer | Deferred |
| Graveyard | Present with `DeathRecord.prefab` | Deferred |
| Museum (Legacy Tower) | Present | Deferred |
| Provisioner | Present | Deferred |
| Sanctuary | Present | Deferred |
| Inn | Present | Deferred |

These render as generic building detail fallback via `BuildingScreenRouter` when navigated to, but have no specialized UI.

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

# Unit tests (vitest)
npm run test

# Validation smoke tests (replay fixtures + build-run)
npm run smoke

# Full build-verification pipeline
npm run smoke-build

# Browser acceptance tests (Playwright, requires build first)
npm run build && npx playwright test
# or
npm run smoke-browser
```

### 3.2 Browser Smoke Coverage (UIR-010)

The Playwright smoke suite at `frontend/smoke/browserSmoke.spec.ts` covers:

**Replay boot → full meta-loop:**
1. Startup screen with Boot Replay Shell / Boot Live Shell buttons
2. Replay boot → Town shell with campaign info, gold, roster heroes (Shen, Bai Xiu, Hei Zhen), and building labels (Stagecoach, Guild, Blacksmith, Sanitarium)
3. Hero detail screen with tab navigation (装备, 技能, 信息, 状态)
4. Building detail (Guild) with action buttons
5. Full provisioning → expedition launch → result → return → town loop
6. Town re-entry after loop (campaign persistence)

**Live boot path:**
7. Live boot → Town shell with Fresh Campaign data
8. Hero detail from live bridge
9. Building detail (Stagecoach) from live bridge
10. Full live provisioning → expedition → result → return flow

**Fidelity gates:**
11. All completed product surfaces checked against blocklist patterns: `placeholder`, `skeletal`, `skeleton`, `reserved canvas`, `text.based rendering`, `rendering completion`
12. Landscape viewport classes present: `.town-viewport`, `.expedition-viewport`, `.viewport-hud`, `.viewport-roster`
13. No page errors or console errors at any phase

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
- **Viewport:** 1440×900 (landscape 16:9)
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
| UIR-005 | Implement landscape town game viewport | `TownShellScreen.tsx`, `PixiStage.tsx`, `.town-viewport` CSS |
| UIR-006 | Migrate roster and hero detail UI | `HeroDetailScreen.tsx`, `contractTypes.ts` HeroDetailViewModel |
| UIR-007 | Migrate town building UI with original assets | `BuildingScreenRouter.tsx`, `BuildingIcons.tsx`, 3 building screen components |
| UIR-008 | Migrate provisioning and launch flow | `ProvisioningScreen.tsx`, `ExpeditionScreen.tsx` |
| UIR-009 | Migrate result and return loop | `ResultScreen.tsx`, `ReturnScreen.tsx` |
| UIR-010 | Add browser smoke coverage for fidelity gates | `frontend/smoke/browserSmoke.spec.ts` |
| **UIR-011** | **Document Unity asset and layout parity outcome** | **This document** |

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
| `ProvisioningScreen` | ✅ Complete | `UI_Provision` + `PartyFormationManager` | Passes fidelity blocklist |
| `ExpeditionScreen` | ✅ Complete | `SelectedQuestPanel` + `RaidPreparationManager` | Passes fidelity blocklist |
| `ResultScreen` | ✅ Complete | `RaidResultWindow.cs` + `HeroResultSlot` | Passes fidelity blocklist |
| `ReturnScreen` | ✅ Complete | `ResultHeroWindow.cs` + `ResultItemWindow.cs` | Passes fidelity blocklist |
| `FatalErrorScreen` | ✅ Complete | — | Exempt (error surface) |
| `UnsupportedStateScreen` | ✅ Complete | — | Exempt (error surface) |

---

## 6. UIR-004 Asset Manifest Updates

The asset manifest at `frontend/src/assets/asset-manifest.json` remains the canonical reference for all 75 catalogued asset references, 28 inventoried prefabs, and 3 blockers. Key updates since UIR-004:

- All 6 scoped buildings have confirmed sprite paths and GUIDs
- Spine skeleton data for 5 hero families catalogued (partial GUID coverage)
- 24 UI chrome assets confirmed
- 4 confirmed item icons (provisions + trinket samples)
- 3 stress pip states documented
- 5 deferred items enumerated
- **No assets have been extracted** — frontend remains CSS-only with inline SVGs for building icons

---

## 7. References

| Document | Location |
|----------|----------|
| Unity Hierarchy & Layout Migration Brief | `docs/plans/2026-05-04-ddgc-town-meta-unity-ui-brief.md` |
| Asset Manifest | `frontend/src/assets/asset-manifest.json` |
| Prefab Inventory Index | `fixtures/ui_inventory/index.json` |
| Frontend README | `frontend/README.md` |
| Browser Smoke Tests | `frontend/smoke/browserSmoke.spec.ts` |
| Frontend Source | `frontend/src/` |
| Project README | `README.md` |
| Migration Blockers | `MIGRATION_BLOCKERS.md` |
| Semantic Gap Matrix | `SEMANTIC_GAP_MATRIX.md` |
