# DDGC Frontend Workspace

This workspace is the product-owned rendered frontend for `DDGC_newArch`.

## Purpose

The workspace exists to keep DDGC-specific screen composition and product UI
separate from both:

- reusable `WorldEngine` presentation substrate packages, and
- Rust gameplay truth inside `DDGC_newArch/src/`.

## Guardrails

- Consume only stable contracts, replay fixtures, and DDGC view-model exports.
- Do not import private Rust runtime internals.
- Do not move DDGC screens into `WorldEngine` packages.
- Do not let frontend state become gameplay truth.

## Local Developer Startup

### Prerequisites

- Node.js 18+ and npm

### Running the Rendered Frontend

```bash
# Install dependencies
npm install

# Start development server (serves on http://localhost:4179)
npm run dev

# Typecheck
npm run typecheck

# Production build
npm run build

# Run tests
npm run test

# Run validation smoke tests
npm run smoke
```

### Runtime Modes

The frontend supports two runtime modes:

1. **Replay Mode** — Boots using stable fixture data and view-model placeholders.
   This allows the rendered UI to evolve without touching gameplay truth.
   Click "Boot Replay" on the startup screen.

2. **Live Mode** — Boots through `DdgcHost::boot_live()` contract boundary,
   initializing a fresh campaign state. Currently uses placeholder data
   pending real runtime wiring. Click "Boot Live" on the startup screen.

Both modes render the same town shell application, using the same
`TownShellScreen` component and `AppFrame` layout.

### Runtime Bridge Architecture

The runtime bridge layer (`src/bridge/`) provides a clean seam between
the frontend and the Rust runtime:

- `RuntimeBridge` — Interface defining boot, snapshot, intent dispatch, and subscription
- `ReplayRuntimeBridge` — Replay-driven implementation using fixture data
- `LiveRuntimeBridge` — Live-runtime implementation wired to `DdgcHost` contracts

## Current Scope (UIR series complete)

The rendered frontend now supports the full town → provisioning → expedition → result → return meta-loop. See the [UIR-011 outcome document](../docs/UIR-011-UI-REDO-RECOVERY-OUTCOME.md) for the complete asset and layout parity summary.

### Screens Implemented

| Screen | Route | Unity Precedent |
|--------|-------|----------------|
| StartupScreen | Boot selector | MainMenuWindow.prefab |
| TownShellScreen | Town viewport | EstateManagement.unity UI_Estate |
| HeroDetailScreen | Hero inspection | CharacterWindow.prefab |
| BuildingScreenRouter | Building detail | UpgradableBuildingWindow |
| StagecoachBuildingScreen | Stagecoach | StageCoachWindow.cs |
| GuildBuildingScreen | Guild | GuildHeroWindow.cs |
| BlacksmithBuildingScreen | Blacksmith | BlacksmithHeroWindow.cs |
| ProvisioningScreen | Provisioning | UI_Provision |
| ExpeditionScreen | Expedition launch | SelectedQuestPanel |
| ResultScreen | Expedition result | RaidResultWindow |
| ReturnScreen | Return to town | ResultHeroWindow |

### Runtime Modes

The frontend supports two runtime modes: