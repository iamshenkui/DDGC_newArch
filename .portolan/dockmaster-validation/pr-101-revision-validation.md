# PR #101 Revision Validation

## Changes
- Fixed dungeon-select readiness gate: minimum party size raised from 1 to 2 heroes
  to match the established provisioning flow requirements.
- Added bridge-layer guard for `confirm-dungeon-selection` in both LiveRuntimeBridge
  and ReplayRuntimeBridge: intent is rejected when `isReadyToProceed` is false.
- Updated browser smoke tests to select 2 heroes before confirming.

## Validation Commands

### Unit Tests (relevant suites)
```bash
npx vitest run src/validation src/session src/screens/expedition
```
Result: **10 test files passed, 263 tests passed**

### Full Frontend Test Suite
```bash
npm test
```
Result: **10 test files passed, 331 tests passed**

### Build + Smoke
```bash
npm run build && npm run smoke
```
Result: **Build succeeded, 4 test files passed, 146 tests passed**

## Exit Status
All commands exited with status 0.
