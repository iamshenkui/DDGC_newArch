# UIR-005D — Town/Base Screenshot Parity & Entry Flow Validation

**Date:** 2026-05-05
**Task:** UIR-005D — UI Redo: validate town/base screenshot parity and entry flow
**Scope:** Verify replay and live entry into the reconstructed town/base screen, record the
verified runtime target, and reject the slice unless the rendered shell matches the source
screenshot anchors and hierarchy expectations from UIR-005A. No hero-detail or deeper
building-detail redesign work — those remain out of scope for this slice.

---

## 1. Verified Runtime Target

| Field | Value |
|-------|-------|
| Server type | Vite preview server (`vite preview`) |
| Source script | `npm run preview` (resolves to `vite preview` in `frontend/package.json`) |
| Port | `4179` |
| Mode | **preview** (production build), not dev |
| Bound URL | `http://localhost:4179` |
| Build artifact | `frontend/dist/` populated via `npm run build` (vite v5.4.21, 38 modules transformed) |
| Playwright config | `frontend/playwright.config.ts` — `webServer.command = "npm run preview"`, port `4179` |
| Viewport | `1440 × 900` (landscape, headless) |

The Playwright runner uses `webServer.reuseExistingServer = !process.env.CI`, so in CI it starts
a fresh `vite preview` instance against the freshly built `dist/`. Locally, an existing
`npm run preview` is reused. The runtime target is preview, not dev — validation is done
against the optimized production bundle, which matches what the published artifact will be.

---

## 2. Entry Flows Verified

Both entry paths reach the reconstructed town/base screen with no page errors and no console
errors.

### 2.1 Replay Boot

* Path: Startup → "Boot Replay" → `.town-viewport`
* Verified: replay campaign name "苍灯远征", eyebrow "城镇中枢", gold value `1250`
* Roster heroes: Shen, Bai Xiu, Hei Zhen
* All 11 building nodes visible by `data-building-id` and Chinese display name.
* No console errors / no page errors observed during the run.

### 2.2 Live Boot

* Path: Startup → "Boot Live" → `.town-viewport`
* Verified: live campaign name "新档位面", eyebrow "城镇中枢"
* Roster heroes: Yuan, Mei (live fixture)
* All 11 building nodes, 6 side buttons, 5 currency slots render against the live bridge.
* No console errors / no page errors observed during the run.

---

## 3. Screenshot Anchor Parity

The Playwright spec (`frontend/smoke/browserSmoke.spec.ts`) gates each anchor from the UIR-005A
recon against the rendered shell. All gates currently pass on the verified preview target.

| Anchor ID (UIR-005A §5.1) | Source Element | Frontend Selector | Expected | Actual (replay) | Actual (live) |
|----|----|----|----|----|----|
| **A-NAMEPLATE** | `EstateNameplate` (top-right, 592×153) | `.estate-top-panel` (1 element) | visible | visible | visible |
| **A-CAMPAIGN-NAME** | `EstateName` text | `h1.estate-campaign-name` | replay: `苍灯远征` / live: `新档位面` | `苍灯远征` | `新档位面` |
| **A-CURRENCY** | `CurrencyPanel` (5-currency strip) | `.estate-currency-slot` (5 elements) | 5 slots (bust/portrait/deed/crest/gold) | 5 | 5 |
| **A-EMBARK** | `BottomPanel/EmbarkButton` | `.estate-embark-button` (1 element) | visible | visible | visible |
| **A-SIDEBUTTONS** | `BottomPanel/SideButtons` (6 buttons) | `.estate-side-button` (6 elements) | 6 | 6 | 6 |
| **A-BUILDINGS** | 11 building nodes under `UI_Estate` | `.estate-building-node` (11 elements) | 11 | 11 | 11 |
| **A-BUILDING-LABELS** | 11 BuildingLabel display names | `.estate-building-source-label` text | DDGC Chinese 11 names | all 11 visible | all 11 visible |
| **A-ROSTER** | `UI_Roster/RosterPanel` | `.roster-scroll .roster-hero` | hero cards visible | 3 (replay) | 2 (live) |
| **A-FIDELITY** | no placeholder/skeleton text | `.town-viewport` innerText | no blocked patterns | clean | clean |

The 11 building Chinese display names verified against the rendered DOM:
次元感知塔, 试炼场, 锻造舱, 细胞修复站, 信仰祭坛, 迷情乐园, 英雄档案馆, 天国花园,
遗留塔, 交易市场, 空间分析.

---

## 4. Issues Found and Resolved in This Slice

### 4.1 Test/source label drift on building detail

**Symptom:** Initial Playwright run failed:
* `replay`: building-detail-name expected `"Guild"` but received `"试炼场"`.
* `live`: building-detail-name expected `"Stagecoach"` but received `"次元感知塔"`.

**Root cause:** The town shell and the building detail screens both render the source DDGC
Chinese display names from `BuildingDetailViewModel.label` (UIR-005C). The smoke spec was still
asserting on legacy English labels.

**Fix:** Updated `frontend/smoke/browserSmoke.spec.ts` building detail assertions to expect the
DDGC Chinese display names — `试炼场` (Guild) for replay, `次元感知塔` (Stagecoach) for live.
This aligns the screenshot-parity gates with the actual source-faithful rendering.

### 4.2 Bottom panel grouping not exercised

**Symptom:** UIR-005C exposes `.estate-bottom-panel` as the BottomPanel hierarchy wrapper, but
the rendered shell did not have a CSS rule for it; the wrapper had no layout intent.

**Fix:** Added `.estate-bottom-panel` rule in `frontend/src/townEstateLayout.css` mirroring the
Unity RectTransform (`anchorMin=(0,0) anchorMax=(1,1) sizeDelta=(0,0)`) — full-stretch, click
pass-through, with descendants opting in to pointer events. This matches the source hierarchy
without altering the visible chrome positions.

### 4.3 Anchor coverage in smoke spec was partial

**Symptom:** The earlier smoke spec only checked `.estate-top-panel`, `.estate-stage-shell`,
`.estate-bottom-panel` presence. It did not assert anchor cardinality for side buttons,
embark control, currency slots, or all 11 building nodes.

**Fix:** Added explicit `toHaveCount` checks for `.estate-side-button` (6),
`.estate-currency-slot` (5), `.estate-building-node` (11), and presence of
`.estate-embark-button`. Replay spec also iterates all 11 Chinese display names rather than
the previous English subset of 4. Live spec mirrors the same anchor cardinality checks.

---

## 5. Stale Server / Replay-vs-Live Drift Audit

* `vite preview` reads from `frontend/dist/`. Confirmed `npm run build` produced a fresh dist
  (`dist/index.html`, `dist/assets/index-*.js`, `dist/assets/index-*.css`) before validation.
* Playwright's `webServer.reuseExistingServer = !process.env.CI` is intentional — local
  iteration reuses an existing preview instance. To avoid stale builds, the validation harness
  rebuilds before running tests via `npm run smoke-build` (`build && smoke && playwright test`).
* No drift observed between replay and live entry: both render the same shell hierarchy with
  the same anchor cardinality and pass the same fidelity gates. The only divergence is
  fixture-driven content (campaign name / roster), which is expected.

---

## 6. Quality Gate Results (verified target: `vite preview` on `:4179`)

| Gate | Command | Result |
|------|---------|--------|
| Typecheck | `npm run typecheck` (`tsc --noEmit`) | pass |
| Unit + integration vitest | `npx vitest run` | 9 files, 281 tests pass |
| Production build | `npm run build` | 38 modules transformed, dist emitted |
| Browser smoke (replay loop) | `npx playwright test` test 1 | pass |
| Browser smoke (live boot loop) | `npx playwright test` test 2 | pass |

---

## 7. Out of Scope for This Slice

This slice **does not** expand into:

* Hero-detail (`HeroDetailScreen.tsx`) redesign or new hero-detail anchors beyond the
  smoke-spec-level reachability check that already existed pre-UIR-005D.
* Deeper building-detail redesign — the building screens (Guild/Stagecoach/Blacksmith/
  Sanitarium/generic) keep their UIR-006/UIR-007 rendering. UIR-005D only updated the smoke
  assertions that compared against legacy English labels; the building detail components
  themselves are untouched.
* Sprite extraction for the 5 deferred buildings (graveyard, garden, legacytower, market,
  campingtrainer) — they continue to use the staged sprites as set up in UIR-005B.
* QuickStart/QuickProgress button rendering changes beyond what UIR-005C produced.

---

## 8. Acceptance Summary

| Criterion | Status |
|-----------|--------|
| Replay path reaches reconstructed town with no page/console errors | met |
| Live path reaches reconstructed town with no page/console errors | met |
| Verified runtime target (server type, port, dev/preview) recorded | met — §1 above |
| Top nameplate anchor verified | met — `.estate-top-panel` |
| Side buttons anchor verified (6) | met — `.estate-side-button` count = 6 |
| Embark control anchor verified | met — `.estate-embark-button` |
| Currency strip anchor verified (5) | met — `.estate-currency-slot` count = 5 |
| 11 visible buildings verified | met — `.estate-building-node` count = 11 |
| Stale-server / parity issues recorded or fixed | met — §4 above |
| Slice did not expand into hero-detail or deeper building-detail | met |
