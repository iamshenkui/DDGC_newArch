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

## Review Fixes (Round 4) — current revision
- Fixed malformed TypeScript syntax in `frontend/smoke/browserSmoke.spec.ts`:
  - Removed the concatenated expect message that caused a parse error.
  - Repaired the live-flow smoke block by restoring the missing `await expect(` call.
  - Updated the replay and live meta-loop tests to match the actual post-dungeon-select
    flow: confirming the dungeon selection transitions directly to provisioning.
  - Updated the combat attack flow test to select a dungeon and heroes before
    confirming and launching.
  - Replaced incorrect Chinese provisioning title expectations with the actual
    English view-model title (`Provision Expedition`) and removed the obsolete
    `getByRole("button", { name: "Confirm & Launch Expedition" })` clicks in favor
    of the existing `data-testid="footer-btn-launch"` control.
- Fixed malformed CSS block boundaries in `frontend/src/styles.css`:
  - Closed the unbalanced `.dungeon-detail-active` rule before the Dungeon Interaction
    CSS comment so downstream rules are no longer swallowed.
  - Repaired the `.dungeon-party-portrait` declaration by adding the missing
    `background: linear-gradient(` opening and restoring width/height/display rules.
  - Closed `.dungeon-party-portrait-letter` and `.dungeon-party-name`, which were
    left open and caused subsequent selectors to be malformed.

## Validation Commands

### TypeScript typecheck
```bash
npm run typecheck
```
Result: **Exit 0**

### Full Frontend Test Suite
```bash
npm test
```
Result: **Exit 0 — 10 test files passed, 342 tests passed**

### Smoke Tests
```bash
npm run smoke
```
Result: **Exit 0 — 4 test files passed, 207 tests passed**

### Production Build
```bash
npm run build
```
Result: **Exit 0 — build succeeded**

### Browser Smoke Tests
```bash
npm run smoke-browser
```
Result: **Exit 0 — 3 tests passed**
