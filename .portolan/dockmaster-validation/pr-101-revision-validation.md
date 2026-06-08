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
  dungeon IDs, and rejects `confirm-dungeon-selection` when the selected dungeon does
  not exist or is not available.
- Updated `ResultReturnFlow.test.ts` to use a real available dungeon ID in transition
  validation tests.
- Added regression tests in `FlowController.test.ts` and `runtimeBridge.test.ts` for
  locked and unknown dungeon IDs.

## Validation Commands

### Unit Tests (relevant suites)
```bash
npx vitest run src/validation src/session src/screens
```
Result: **7 test files passed, 274 tests passed**

### Full Frontend Test Suite
```bash
npm test
```
Result: **10 test files passed, 342 tests passed**

### Build + Smoke
```bash
npm run build && npm run smoke
```
Result: **Build succeeded, 4 test files passed, 146 tests passed**

## Exit Status
All commands exited with status 0.
