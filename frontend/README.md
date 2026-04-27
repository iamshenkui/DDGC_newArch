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

## Commands

```bash
npm install
npm run dev
npm run typecheck
npm run build
npm run test
```

## Initial Scope

The initial scaffold supports:

- replay-mode boot into a rendered town shell placeholder,
- explicit startup, unsupported, and fatal surfaces,
- a runtime bridge seam for replay/live modes,
- screen/module boundaries aligned to Phase 10 documentation.