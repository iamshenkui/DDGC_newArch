# UIR-005E — Town/Base Label, Local Chrome, & Click-Region Alignment

**Date:** 2026-05-05
**Task:** UIR-005E — UI Redo Step 4B: align building labels, local chrome, and click regions
**Scope:** Polish the placed 11-building estate layer so building labels, local
background offsets, and click regions follow the source-backed town/base layout.
This slice does not reopen shell-chrome reconstruction or expand into building-
detail / hero-detail redesign.

---

## 1. Slice Goals

1. Building labels and local background offsets follow source-backed alignment
   from `EstateManagement.unity` (not generic web-card placement).
2. Building click regions line up with the visible estate layer and open the
   intended building targets (no drift between hit-area and Unity sizeDelta box).
3. Local building chrome uses the available source-backed label sprite
   (`building_label_bg01.png`) as a near-imperceptible underlay, matching the
   reference `town.png` rendering rather than a bright cream/gold band.

---

## 2. Source-Backed Catalog (verified via `buildingCatalog.test.ts`)

The 11-building catalog in `frontend/src/town/buildingCatalog.ts` mirrors
Unity's `EstateManagement.unity` scene exactly (positions, sizes, label offsets).
The `buildingCatalog.test.ts` Vitest suite asserts ±1px parity between catalog
entries and the Unity scene JSON via `fixtures/ui_inventory/Assets/Scenes/EstateManagement.unity.json`.

| Building (id → DDGC name) | Unity pos | sizeDelta | Label offset | Anchor mode |
|---|---|---|---|---|
| guild → 试炼场 | (120, -50) | 397×397 | (30, -50) | center |
| tavern → 迷情乐园 | (-650, -220) | 519×519 | (-6, 160) | center |
| graveyard → 英雄档案馆 | (695.5, 95.63) | 344×344 | (100, -100) | center |
| blacksmith → 锻造舱 | (709, 294 raw → -66 norm) | 482×482 | (0, -168) | bottom (anchor_y=0) |
| stagecoach → 次元感知塔 | (31, -267) | 384×384 | (-26, -150) | center |
| garden → 天国花园 | (-245, -211) | 371×371 | (0, -130) | center |
| legacytower → 维度灯塔 | (0, 270) | 541×541 | (-3, 166) | center |
| abbey → 信仰祭坛 | (-330, 90) | 519×519 | (-10, 153) | center |
| market → 交易市场 | (340.5, -222) | 326×339 | (0, -150) | center |
| sanitarium → 细胞修复站 | (390, 110) | 339×339 | (50, -100) | center |
| campingtrainer → 空间分析 | (-607, 220) | 266×314 | (-100, -110) | center |

All BuildingLabel sizeDelta values are the Unity-fixed `(326, 48)` constant.
The frontend banner CSS pins to the same dimensions, so the local chrome strip
reads as a 1:1 mirror of the Unity prefab regardless of the caption length.

---

## 3. Click-Region Behavior

**Source contract:** Each Unity `BuildingSlot` is a `Button + Image + BuildingSlot`
component triplet at `anchorMin=(0.5,0.5) anchorMax=(0.5,0.5) pivot=(0.5,0.5)`,
with `anchoredPosition` and `sizeDelta` from the catalog. UGUI Button hit-tests
the full RectTransform bounding box (transparent pixels of the Image are still
clickable when they fall inside `sizeDelta`).

**Frontend rendering:** `<button class="estate-building-node">` in
`TownShellScreen.tsx` is positioned at `(estateLeft(layout.x), estateTop(layout.y))`
with `width × height = layout.width × layout.height`, then translated `(-50%, -50%)`
to center on the source anchored position. The button is the click region; the
inner `.estate-building-art` and `.estate-building-banner` layers have
`pointer-events: none` so clicks always resolve at the source-anchored
button bounding box.

**Verification anchors:**
- The Playwright spec (`frontend/smoke/browserSmoke.spec.ts`) asserts
  `.estate-building-node` count = 11 and clicks the Guild slot via
  `[data-building-id="guild"]` to enter the source-faithful Guild detail
  (display name "试炼场").
- `buildingCatalog.test.ts` enforces `±1px` parity with the Unity rect for
  each of the 11 catalog entries, so the click region's bounding box matches
  Unity sizeDelta exactly.

---

## 4. Local Chrome — `building_label_bg01.png` Underlay

The Unity asset `building_label_bg01.png` is a wide painterly wash (5:1 strip)
provided in `frontend/public/original/chrome/`. The reference `town.png`
renders this band as a near-imperceptible underlay over the dark starfield
rather than a bright cream/gold plate. The CSS in
`frontend/src/townEstateLayout.css` mirrors that intent:

```css
.estate-building-banner::before {
  content: "";
  position: absolute;
  inset: 0;
  background-image: var(--ddgc-chrome-building-label-bg, url("/original/chrome/building_label_bg01.png"));
  background-size: 100% 100%;
  background-repeat: no-repeat;
  opacity: 0.42;
  mix-blend-mode: overlay;
  pointer-events: none;
}
```

The `building_icon_bg.png` (square inked frame) is intentionally *not* used
at the estate layer — the reference `town.png` shows buildings without
square frames. That asset is reserved for building-detail surfaces and would
clash with the floating-island silhouette aesthetic if applied here.

---

## 5. Source-Faithful Data Attributes

Every building node and its label banner now carry source-rect documentation
inline so the rendered DOM is self-describing for screenshot diff harnesses
and Playwright recon:

| Element | Attribute | Value |
|---------|-----------|-------|
| `.estate-building-node` | `data-source-prefab` | `UI/Estate/BuildingSlot` |
| `.estate-building-node` | `data-source-scene` | `EstateManagement.unity` |
| `.estate-building-node` | `data-source-rect` | `anchorMin=(0.5,0.5) anchorMax=(0.5,0.5) anchoredPosition=(x,y) sizeDelta=(w,h)` |
| `.estate-building-node` | `data-source-guid` | per-building Unity asset GUID |
| `.estate-building-node` | `aria-label` | `${displayName} — ${statusLabel}` (e.g. "试炼场 — 部分可用") |
| `.estate-building-banner` | `data-source-prefab` | `UI/Estate/BuildingSlot/BuildingLabel` |
| `.estate-building-banner` | `data-source-rect` | `anchoredPosition=(offsetX,offsetY) sizeDelta=(326,48)` |
| `.estate-building-banner` | `data-source-sprite` | `building_label_bg01.png` |

These attributes do not affect rendering — they document the source-faithful
mapping inline so a future verification harness can compare the DOM against
the Unity scene JSON without re-deriving the catalog at runtime.

---

## 6. Quality Gates

| Gate | Command | Status |
|------|---------|--------|
| Typecheck | `npm run typecheck` (`tsc --noEmit`) | pass |
| Vitest (frontend) | `npm test` (`vitest run`) | 10 files, 315 tests pass |
| Production build | `npm run build` | 38 modules transformed, dist emitted |
| Source parity | `buildingCatalog.test.ts` (34 cases) | ±1px from Unity scene JSON |

---

## 7. Out of Scope for This Slice

This slice **does not** modify:

* The persistent shell chrome — top nameplate, currency strip, side buttons,
  embark control, roster strip, and bottom panel — beyond what UIR-005C/005D
  produced. The acceptance criteria explicitly limit shell-chrome work to the
  minimum needed for label and click-region alignment.
* Building-detail screens (`BuildingDetailScreen.tsx` and the per-building
  variants under `screens/town/buildings/`). These remain on their UIR-006/007
  rendering and are reached unchanged via the click-region targets above.
* Hero-detail (`HeroDetailScreen.tsx`) — out of scope per UIR-005D §7.
* Sprite extraction for the 5 buildings without dedicated PNGs (graveyard,
  garden, legacytower, market, campingtrainer) — they continue to use the
  staged sprites + CSS letter fallback pipeline from UIR-005B.
* The `building_icon_bg.png` and `building_info_bg.png` chrome assets — these
  are reserved for building-detail surfaces, not the estate layer.

---

## 8. Acceptance Summary

| Criterion | Status |
|-----------|--------|
| Building labels and local background offsets follow source-backed alignment | met — `buildingCatalog.test.ts` enforces ±1px Unity parity for all 11 buildings |
| Building click regions line up with the visible estate layer and open the intended building targets | met — `.estate-building-node` bounding box mirrors Unity sizeDelta; smoke spec verifies click → building detail flow |
| Local building chrome uses available source-backed label or frame assets | met — `building_label_bg01.png` rendered as a 0.42-opacity overlay underlay matching reference `town.png` |
| Slice does not reopen shell chrome reconstruction outside minimum needed | met — only TownShellScreen building-button + label banner attributes and CSS comments touched |
| Slice does not expand into building-detail or hero-detail redesign | met — no changes under `screens/town/buildings/` or `HeroDetailScreen.tsx` |
