# PR #101 Revision Validation

## Changes
- Fixed dungeon-select readiness gate: minimum party size raised from 1 to 2 heroes
  to match the established provisioning flow requirements.
- Added bridge-layer guard for `confirm-dungeon-selection` in both LiveRuntimeBridge
  and ReplayRuntimeBridge: intent is rejected when `isReadyToProceed` is false.
- Updated browser smoke tests to select 2 heroes before confirming.

## Review Fixes (Round 2)
- `ReplayRuntimeBridge` and `LiveRuntimeBridge` now validate that the selected dungeon
  exists and `isAvailable` before setting `selectedDungeonId` or `isReadyToProceed`.
- `toggle-dungeon-hero` also requires a valid, available dungeon before marking ready.
- `confirm-dungeon-selection` rejects if the selected dungeon is unknown or unavailable
  (defense-in-depth beyond the `isReadyToProceed` check).
- `FlowController.canTransition` now rejects `select-dungeon` for unknown or locked
  dungeon IDs, and rejects `confirm-dungeon-selection` when the selected dungeon
  does not exist or is not available.
- Updated `ResultReturnFlow.test.ts` to use a real available dungeon ID in transition
  validation tests.
- Added regression tests in `FlowController.test.ts` and `runtimeBridge.test.ts` for
  locked and unknown dungeon IDs.

## Review Fixes (Round 3)
- Fixed TS2339 narrowing error in `FlowController.ts` (`confirm-dungeon-selection`
  branch). Extracted `const vm = snapshot.viewModel` after the `kind === "dungeon-select"`
  narrowing check so `vm.selectedDungeonId` is used inside the `.find()` callback
  instead of `snapshot.viewModel.selectedDungeonId`.

## Validation Commands

### TypeScript typecheck
```bash
npm run typecheck
```
Result: **Exit 2** — the PR-specific TS2339 error in `FlowController.ts` is resolved.
Three pre-existing TS2307 module-resolution errors remain in `AppProviders.tsx` and
`PixiStage.tsx` (unable to resolve `@contracts/app-shell`, `@contracts/ui-substrate`,
`@contracts/pixi-renderer`) because the `WorldEngine` sibling repository is not present
in this sandbox. These files and their `@contracts/*` imports pre-date this PR.

### Full Frontend Test Suite
```bash
npm test
```
Result: **Exit 0 — 10 test files passed, 342 tests passed**

### Smoke Tests
```bash
npm run smoke
```
Result: **Exit 0 — 4 test files passed, 151 tests passed**

### Production Build
```bash
npm run build
```
Result: **Exit 0 — build succeeded**

### Browser Smoke Tests
```bash
npm run smoke-browser
```
Result: **Exit 0 — 2 tests passed**
