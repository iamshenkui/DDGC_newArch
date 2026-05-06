# UIR-005H — Town/Base Screenshot Parity Report

Recorded: 2026-05-06 (UTC+8) for task UIR-005H — UI Redo Step 7.

This report compares the current reconstructed town/base shell against the Unity source-of-truth reference. It is a parity *review* over already-captured artifacts, gated by the upstream UIR-005G runtime-target validation. It does not re-run live boot, does not re-capture frames, and does not perform frontend redesign or fixes.

The canonical out-of-repo location of this report (consumed by the orchestration harness) is:
`/mnt/d/GameDesign/游戏迁移/.state/artifacts/ui_validation/uir-005h-town-parity-report.md`. This file in `docs/plans/` is the in-repo committed copy.

## 1. Inputs Consumed

| Role | Path | Dimensions | Provenance disposition (per UIR-005G record) |
| --- | --- | --- | --- |
| Unity source reference | `/mnt/d/GameDesign/游戏迁移/ref_image/town.png` | 2084 × 1176 (≈1.772:1) | Authoritative source-of-truth for parity scoring. |
| Runtime-target record | `/mnt/d/GameDesign/游戏迁移/.state/artifacts/ui_validation/uir-005g-entry-runtime-record.md` | n/a | Verified preview target on `localhost:4179`; replay + live both reach `.town-viewport`. |
| Existing current render | `/mnt/d/GameDesign/游戏迁移/.state/artifacts/ui_inputs/uir-005g-current-town-render.png` | 2200 × 1122 (≈1.961:1) | **Provenance: PARTIAL — content matches, capture parameters do not.** Used as supplementary cross-reference only, not as authoritative current-side input. |
| Authoritative current render — replay | `/mnt/d/GameDesign/游戏迁移/.state/artifacts/ui_validation/uir-005g-replay-town-fresh-1440x900.png` | 1440 × 900 (≈1.6:1) | Captured 2026-05-06 03:53 +0800 from the verified preview target via `frontend/smoke-cap/townCapture.spec.ts`. Authoritative current-side input. |
| Authoritative current render — live | `/mnt/d/GameDesign/游戏迁移/.state/artifacts/ui_validation/uir-005g-live-town-fresh-1440x900.png` | 1440 × 900 (≈1.6:1) | Same capture pipeline as replay; only campaign-name data differs. Authoritative current-side input. |

## 2. Provenance Gate

The upstream UIR-005G record classifies provenance as **PARTIAL** for the legacy `ui_inputs/uir-005g-current-town-render.png` artifact and explicitly recommends the parity step route around it:

> The parity step **MUST** treat `.state/artifacts/ui_validation/uir-005g-replay-town-fresh-1440x900.png` and `.state/artifacts/ui_validation/uir-005g-live-town-fresh-1440x900.png` as the authoritative current-runtime captures … and treat `ui_inputs/uir-005g-current-town-render.png` as **provenance-uncertain reference only**.

Per the UIR-005H acceptance criterion ("If the runtime-target record indicates stale, missing, or mismatched provenance, the task treats that as a blocker and does not silently continue into parity acceptance"):

- The 2200 × 1122 artifact has mismatched capture-parameter provenance and is therefore **explicitly excluded from authoritative parity scoring**, not silently consumed. It is retained only as a higher-resolution visual cross-reference.
- The 1440 × 900 fresh captures from the verified preview target carry full provenance (entry flow verified, viewport documented, capture script known) and are used as the authoritative current-side input.
- The runtime-target record itself does **not** flag a blocking provenance failure — entry flow is verified, content matches, and canonical captures exist. Therefore parity acceptance proceeds against the canonical captures with the legacy artifact's mismatch documented and routed around.

**Provenance gate disposition: PASS (with explicit routing).** Parity findings below derive from the canonical 1440 × 900 captures vs. `ref_image/town.png`. The 2200 × 1122 artifact is referenced only where its higher detail aids interpretation of a finding already present at canonical resolution.

## 3. Anchor-by-Anchor Parity Findings

Each anchor is graded:
- **OK** — anchor present at expected position with structural and content parity (acceptable visual drift).
- **MINOR DRIFT** — anchor present and functional, but visible deviations in styling, chrome, or treatment.
- **NOTABLE DRIFT** — anchor present but treatment diverges enough that it would read as a different style choice on first glance.
- **GAP** — anchor missing or broken.

### 3.1 Top Nameplate (top-left campaign label)

| Aspect | Reference (`town.png`) | Current (canonical 1440×900) | Grade |
| --- | --- | --- | --- |
| Presence | Single low-contrast white text "新游戏" anchored top-left | Eyebrow "新页面" with heading "新档位面" (live) / "苍灯远征" (replay) anchored top-left | OK |
| Position | Top-left, inset from corner | Top-left, inset from corner — matches anchor location | OK |
| Hierarchy | Single label, single weight | Two-line eyebrow + heading composition | MINOR DRIFT |
| Typography | Thin sans, small caps feel | Sans, two weights stacked | MINOR DRIFT |

Notes:
- The text content difference (`新游戏` vs. `新档位面` / `苍灯远征`) is **campaign-data driven**, confirmed by the runtime record (replay carries `苍灯远征`, live carries `新档位面`). It is not a UI-shell parity gap; the reference's `新游戏` would appear here as the "new game" campaign label only when no other campaign is active.
- The two-line eyebrow + heading layout (`新页面` over `新档位面`) is an intentional information-architecture lift not present in the reference single-label format. It is graded MINOR DRIFT rather than NOTABLE because it occupies the same anchor region and does not displace any other chrome group.

**Grade: MINOR DRIFT** (anchor present, position matches; text composition expanded from one line to two).

### 3.2 Side Buttons (top-right utility cluster)

| Aspect | Reference | Current | Grade |
| --- | --- | --- | --- |
| Buttons present | 饰品仓库 / 英雄 / 设置 (3 buttons) | 饰品仓库 / 英雄 / 设置 (3 buttons, asserted as 3 `.estate-side-button` in smoke spec) | OK |
| Order (left → right) | 饰品仓库, 英雄, 设置 | 饰品仓库, 英雄, 设置 — matches | OK |
| Position | Top-right corner, integrated into top bar | Top-right corner, integrated into top bar | OK |
| Chrome treatment | Low-contrast rounded rectangles, integrated into the top-bar plate | Higher-contrast flat rectangles with sharper border edges; reads as overlay widgets rather than scene chrome | NOTABLE DRIFT |

**Grade: NOTABLE DRIFT** (count, order, and position parity; visual chrome treatment diverges in contrast and edge weight).

### 3.3 Embark Control (bottom-left circular dial)

| Aspect | Reference | Current | Grade |
| --- | --- | --- | --- |
| Presence | Compass-style circular dial labeled `位面探索` | Circular control with Chinese text in same anchor region (1 `.estate-embark-button` asserted in smoke spec) | OK |
| Position | Bottom-left, partially overlapping the bottom edge | Bottom-left, similar overlap | OK |
| Shape | Round with concentric ornament rings and tick marks | Round with reduced ornament; rings less pronounced | MINOR DRIFT |
| Label typography | Thin sans, vertical/curved layout around the dial | Sans label, less ornament around the ring | MINOR DRIFT |

**Grade: MINOR DRIFT** (anchor present, position and function preserved; ornament richness reduced).

### 3.4 Currency Strip (bottom horizontal slot row)

| Aspect | Reference | Current | Grade |
| --- | --- | --- | --- |
| Slot count | 5 (blue orb, purple crystal, flame, silver bar, gold/coin) | 5 (`.estate-currency-slot` × 5 asserted in smoke spec) | OK |
| Order | blue orb → purple crystal → flame → silver bar → gold/coin | Same order in canonical capture | OK |
| Numeric values | 100, 100, 100, 200, 58950 (reference frame) | Numeric values present in same slots (varies by save state) | OK |
| Position | Bottom-right of frame, horizontal | Bottom-right of frame, horizontal | OK |
| Icon fidelity | Textured, slight glow on each glyph | Flatter, less glow / texture | MINOR DRIFT |

**Grade: MINOR DRIFT** (slot count, order, and anchor parity; icon texture less detailed).

### 3.5 Visible 11-Building Coverage (central estate stage)

The Unity reference shows 11 building nodes around the central spire. The smoke spec asserts `11 .estate-building-node` elements within `.estate-stage-shell`. Building label inventory (Chinese):

| # | Reference label | Approx. position | Present in current | Notes |
| --- | --- | --- | --- | --- |
| 1 | 维度灯塔 (Dimensional Beacon) | Top center, central spire | Yes | The central spire is the visual anchor; both renders show concentric orbital rings around it (current ring glow/bloom reduced). |
| 2 | 信仰祭坛 | Upper-left quadrant | Yes | Cathedral-style island, present in both. |
| 3 | 空间分析 | Far-left edge | Yes | Cliff-edge island with arch structure. |
| 4 | 迷情乐园 | Mid-left | Yes | Forested island with warm glow. |
| 5 | 天堂花园 | Lower-mid-left | Yes | Blue-green domed island. |
| 6 | 试炼场 | Center, just below spire | Yes | Small platform with central pillar. |
| 7 | 灵/细胞修复站 (Cell Repair Station) | Mid-right | Yes | Domed structure mid-right. |
| 8 | 英雄档案馆 | Upper-right | Yes | Cathedral-style structure mirroring 信仰祭坛. |
| 9 | 灰元感知塔 (Ash-Element Sense Tower) | Lower-right area | Yes | Tower among the right cluster. |
| 10 | 交易市场 | Lower-right | Yes | Market-style island. |
| 11 | 锻造铺 (Forge / 锻造) | Far-right | Yes | Forge-style structure at the right edge. |

| Aspect | Reference | Current | Grade |
| --- | --- | --- | --- |
| Building count | 11 | 11 (smoke-asserted) | OK |
| Spatial layout | Spire-centered radial cluster | Spire-centered radial cluster preserved | OK |
| Identity per node | 11 distinct silhouettes | 11 distinct silhouettes | OK |
| Label chrome | Bare white text, no backing shape | Dark pill-shaped badges behind each name | NOTABLE DRIFT |
| Atmospheric lighting | Deep blacks, pronounced rim-light, bloom on edges | Flatter mid-grey, reduced bloom | NOTABLE DRIFT |
| Texture fidelity | Crystalline texturing, fine surface detail | Compressed/down-lit reading; less surface micro-detail | NOTABLE DRIFT |

**Grade: NOTABLE DRIFT** (full 11-of-11 coverage and correct radial layout; significant divergence in label chrome, lighting depth, and texture fidelity).

### 3.6 Shell Hierarchy / Chrome Grouping

The Unity reference has a clear four-region chrome grouping:
1. **Top bar** — campaign nameplate (left) + utility buttons (right).
2. **Center stage** — radial estate composition with the 11 buildings around the central spire.
3. **Bottom-left** — circular embark control overlapping the lower edge.
4. **Bottom-right** — horizontal currency strip.

| Aspect | Reference | Current | Grade |
| --- | --- | --- | --- |
| Number of chrome regions | 4 | 4 (`.estate-stage-shell` containing the building cluster, plus the three peripheral chrome anchors) | OK |
| Region anchors (corners) | Top-left, top-right, bottom-left, bottom-right | Same four anchors | OK |
| Layering | Center stage is the primary surface; chrome floats on top with low contrast | Center stage primary; chrome floats on top with elevated contrast | MINOR DRIFT |
| Chrome / scene blend | Top bar chrome blends into the scene gradient | Top bar chrome reads as a separate UI layer with sharper edges | MINOR DRIFT |
| Edge framing | Scene fills full frame; no peripheral bezel | Subtle dark peripheral band visible (more pronounced at 2200×1122; mild at 1440×900) | MINOR DRIFT |

**Grade: MINOR DRIFT** (region count, anchors, and ordering match; chrome-to-scene contrast/blend is heavier in the current render).

## 4. Aspect-Ratio Note

| Capture | Aspect | Vs. reference (1.772:1) |
| --- | --- | --- |
| Reference `town.png` (2084×1176) | 1.772:1 | — |
| Canonical fresh `…1440x900.png` | 1.600:1 | Narrower (taller); slight vertical compression of horizontal cluster spacing. |
| Cross-ref `…current-town-render.png` (2200×1122) | 1.961:1 | Wider; cluster appears horizontally stretched with extra horizontal whitespace. |

This is consistent with the runtime record's documentation of the verified runtime target's viewport (1440 × 900). The 2200 × 1122 artifact's wider ratio is the same provenance mismatch flagged in UIR-005G; it does not affect this report's anchor grading because grading uses the canonical 1440 × 900 captures. If a future slice wants to score parity at a wider viewport closer to the reference's 1.77 ratio, that requires re-running `frontend/smoke-cap/townCapture.spec.ts` with an updated viewport — that is **not** in scope for UIR-005H.

## 5. Summary

| Anchor | Grade |
| --- | --- |
| 3.1 Top nameplate | MINOR DRIFT |
| 3.2 Side buttons | NOTABLE DRIFT |
| 3.3 Embark control | MINOR DRIFT |
| 3.4 Currency strip | MINOR DRIFT |
| 3.5 11-building coverage | NOTABLE DRIFT |
| 3.6 Shell hierarchy / chrome grouping | MINOR DRIFT |

- **Structural parity (presence, count, position, ordering): full pass across all six anchors.** All six anchor groups exist at the expected screen positions; the 11-building inventory is complete; the 5 currency slots are present; the 3 side buttons are present; the embark control is present.
- **Visual-treatment parity: partial.** The two NOTABLE DRIFT items (side-button chrome contrast, and the 11-building cluster's label chrome + lighting + texture) are the dominant remaining gaps. The other four anchors are MINOR drift consistent with cross-engine asset/rendering differences.
- **Provenance routing: clean.** The mismatched 2200 × 1122 artifact is documented and routed around. Authoritative grading used the 1440 × 900 canonical captures from the verified preview runtime target.

No anchor was graded GAP; no anchor was scored as missing or broken. The shell composition built in UIR-005A through UIR-005G covers the full screen-shell parity surface from the Unity source.

## 6. Out of Scope (Not Performed in This Slice)

Per the UIR-005H acceptance criteria, this slice does **not**:

- Reopen browser entry-flow verification (already verified in UIR-005G).
- Run a fresh replay or live validation pass.
- Re-capture canonical or higher-resolution renders.
- Make frontend code changes to close the NOTABLE DRIFT items.
- Perform pixel-level numeric parity scoring (e.g., SSIM / pixel-diff). This is qualitative anchor parity only.

Recommended successors (not implemented here):

1. A targeted slice to soften top-right side-button chrome (lower contrast, integrate edges into the top-bar plate) — addresses 3.2.
2. A targeted slice to remove or de-emphasize the dark pill backings on building labels — addresses one of the two NOTABLE drift drivers in 3.5.
3. An optional slice to evaluate post-processing/bloom treatment on the building sprites — addresses the lighting and texture portions of 3.5; may be deferred if it requires renderer-level work beyond UI scope.
