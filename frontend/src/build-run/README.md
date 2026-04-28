# Build-Run Module

This module captures the deterministic build/run path for the DDGC rendered frontend,
asset loading assumptions, startup wiring, and environment contract.

## Local Build Run Path

### Prerequisites

- Node.js 18+ and npm
- Rust toolchain (for the headless runtime)
- Platform requirements: Linux, macOS, or Windows with WSL2

### Deterministic Build Steps (Frontend)

```bash
# 1. Navigate to frontend workspace
cd frontend

# 2. Install dependencies (ci-friendly, no network variance)
npm ci

# 3. Typecheck (validates TypeScript contract alignment)
npm run typecheck

# 4. Run smoke tests (validates runtime bridge contracts)
npm run smoke

# 5. Production build (outputs to dist/)
npm run build
```

### Deterministic Build Steps (Rust Runtime)

```bash
# From repository root

# 1. Check compilation (validates Rust contract alignment)
cargo check

# 2. Run Rust integration tests (validates contracts layer)
cargo test --test build_run_smoke
cargo test --test contracts_smoke_tests

# 3. Run all Rust tests (validates full runtime)
cargo test
```

### Development Server

```bash
# Frontend dev server (from frontend/ directory)
cd frontend
npm run dev
# Serves on http://localhost:4179

# Rust binary (from repository root, separate terminal)
cargo run
```

### End-to-End Integration Pipeline

```bash
# Full pipeline: Rust check → Rust tests → Frontend typecheck → Frontend tests → Build
cargo check && \
  cargo test --test build_run_smoke && \
  cargo test --test contracts_smoke_tests && \
  cd frontend && \
  npm run typecheck && \
  npm run smoke && \
  npm run build
```

This validates that:
1. The Rust runtime compiles and its integration tests pass (contract boundary intact)
2. TypeScript contract types align (no drift in view model shapes)
3. Runtime bridge contracts are satisfied (both replay and live modes)
4. The production build artifact is generated from passing tests

## Asset Loading

### Static Assets

The frontend loads the following asset categories:

1. **CSS** — `src/styles.css` (application styles)
2. **HTML Entry** — `index.html` (mounts `#root` container)
3. **No external image/font assets** — current phase uses placeholder/text-based rendering

### Asset Resolution

Vite handles asset resolution via:
- Path aliases in `vite.config.ts`
- Direct module imports for local assets

### Environment Assumptions

| Variable | Default | Purpose |
|----------|---------|---------|
| `VITE_PORT` | `4179` | Dev server port |
| `VITE_HOST` | `0.0.0.0` | Dev server binding |
| `NODE_ENV` | `development` | Build mode |

## Startup Wiring

### Boot Sequence

1. `main.tsx` renders `<DdgcApp />` into `#root`
2. `DdgcApp` initializes `SessionStore` with `fatalSnapshot`
3. `runBoot(mode)` is called with runtime mode (`"replay"` or `"live"`)
4. A `RuntimeBridge` is created based on mode
5. Bridge `boot()` is called, returning initial `DdgcFrontendSnapshot`
6. Snapshot propagates through `SessionStore.replace()`
7. `resolveScreen(snapshot)` determines active screen

### Runtime Mode Resolution

```
DdgcApp
  └── runBoot(mode: "replay" | "live")
        └── createBridge(mode)
              ├── "replay" → ReplayRuntimeBridge (fixture data)
              └── "live" → LiveRuntimeBridge (DdgcHost contract)
```

### Boundary: Rust Runtime ↔ Frontend

The `RuntimeBridge` interface (`src/bridge/RuntimeBridge.ts`) is the **only** allowed
communication channel between the Rust runtime and the frontend. The boundary is:

- **Intent dispatch** — `dispatchIntent(bridge, intent)` flows frontend → Rust
- **Snapshot publication** — `bridge.subscribe(listener)` flows Rust → frontend
- **No direct state sharing** — frontend never reads Rust memory directly

## Smoke Test Contract

The smoke test (`npm run smoke`) validates:

1. **Replay bridge boots** to town shell with correct lifecycle
2. **Live bridge boots** to town shell with correct lifecycle
3. **Intent dispatch** works for hero/building open and return
4. **Flow transitions** (town → provisioning → expedition → combat) work
5. **Meta-loop continuation** (result/return → town) works

### Post-Build Validation

After `npm run build`, run smoke tests against the production build:

```bash
npm run build && npm run smoke
```

This validates that the packaged artifact maintains runtime contract fidelity.

## Packaging Guardrails

- **Do not** bundle Rust runtime — frontend builds independently
- **Do not** expose Rust internals via `window` or global scope
- **Do not** mutate `process.env` in ways that affect the Rust runtime
- **Do** treat the `RuntimeBridge` interface as the stable seam

## Build Artifacts

| Artifact | Location | Purpose |
|----------|----------|---------|
| JS bundles | `frontend/dist/assets/` | Production runtime |
| HTML entry | `frontend/dist/index.html` | Entry point |
| Source maps | `frontend/dist/assets/*.map` | Debugging (dev only) |
