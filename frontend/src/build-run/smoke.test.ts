/**
 * Build-run smoke tests — validate runtime contract after local build/package step.
 *
 * Run with: npm run smoke
 * Run after build: npm run build && npm run smoke
 *
 * These tests validate:
 * 1. Deterministic boot into town shell (replay and live modes)
 * 2. Intent dispatch round-trip (open → return-to-town)
 * 3. Flow state transitions (town → provisioning → expedition → combat)
 * 4. Meta-loop continuation (result → town, return → town)
 */

import { describe, expect, it } from "vitest";

import type {
  DdgcFrontendIntent,
  HeroDetailViewModel,
  BuildingDetailViewModel,
  ProvisioningViewModel,
  ExpeditionSetupViewModel
} from "../bridge/contractTypes";
import { LiveRuntimeBridge } from "../bridge/LiveRuntimeBridge";
import { ReplayRuntimeBridge } from "../bridge/ReplayRuntimeBridge";
import { createSessionStore } from "../session/SessionStore";
import { fatalSnapshot } from "../validation/replayFixtures";
import { resolveScreen } from "../session/FlowController";

describe("build-run smoke: deterministic boot", () => {
  it("replay bridge boots to ready town lifecycle", async () => {
    const bridge = new ReplayRuntimeBridge();
    const snapshot = await bridge.boot();

    expect(snapshot.lifecycle).toBe("ready");
    expect(snapshot.flowState).toBe("town");
  });

  it("live bridge boots to ready town lifecycle", async () => {
    const bridge = new LiveRuntimeBridge();
    const snapshot = await bridge.boot();

    expect(snapshot.lifecycle).toBe("ready");
    expect(snapshot.flowState).toBe("town");
  });

  it("smoke tests run against last smoke result (no-op validation)", () => {
    expect(true).toBe(true);
  });
});

describe("build-run smoke: intent dispatch round-trip", () => {
  it("replay: open-hero then return-to-town", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    const detailSnap = await bridge.dispatchIntent({
      type: "open-hero",
      heroId: "hero-hunter-01"
    });

    expect(detailSnap.viewModel.kind).toBe("hero-detail");
    const heroDetail = detailSnap.viewModel as HeroDetailViewModel;
    expect(heroDetail.name).toBe("Shen");

    const townSnap = await bridge.dispatchIntent({ type: "return-to-town" });
    expect(townSnap.viewModel.kind).toBe("town");
  });

  it("replay: open-building then return-to-town", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    const detailSnap = await bridge.dispatchIntent({
      type: "open-building",
      buildingId: "guild"
    });

    expect(detailSnap.viewModel.kind).toBe("building-detail");
    const buildingDetail = detailSnap.viewModel as BuildingDetailViewModel;
    expect(buildingDetail.label).toBe("Guild");

    const townSnap = await bridge.dispatchIntent({ type: "return-to-town" });
    expect(townSnap.viewModel.kind).toBe("town");
  });

  it("live: open-hero then return-to-town", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();

    const detailSnap = await bridge.dispatchIntent({
      type: "open-hero",
      heroId: "hero-hunter-live-01"
    });

    expect(detailSnap.viewModel.kind).toBe("hero-detail");
    const heroDetail = detailSnap.viewModel as HeroDetailViewModel;
    expect(heroDetail.name).toBe("Yuan");

    const townSnap = await bridge.dispatchIntent({ type: "return-to-town" });
    expect(townSnap.viewModel.kind).toBe("town");
  });
});

describe("build-run smoke: flow state transitions", () => {
  it("replay: town → provisioning → expedition → combat", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    const provSnap = await bridge.dispatchIntent({ type: "start-provisioning" });
    expect(provSnap.flowState).toBe("provisioning");
    expect(provSnap.viewModel.kind).toBe("provisioning");
    const provVm = provSnap.viewModel as ProvisioningViewModel;
    expect(provVm.isReadyToLaunch).toBe(true);

    const expSnap = await bridge.dispatchIntent({ type: "confirm-provisioning" });
    expect(expSnap.flowState).toBe("expedition");
    expect(expSnap.viewModel.kind).toBe("expedition");
    const expVm = expSnap.viewModel as ExpeditionSetupViewModel;
    expect(expVm.isLaunchable).toBe(true);

    const combatSnap = await bridge.dispatchIntent({ type: "launch-expedition" });
    expect(combatSnap.flowState).toBe("combat");
    expect(combatSnap.viewModel.kind).toBe("expedition");
  });

  it("live: town → provisioning → expedition → combat", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();

    const provSnap = await bridge.dispatchIntent({ type: "start-provisioning" });
    expect(provSnap.flowState).toBe("provisioning");
    expect(provSnap.viewModel.kind).toBe("provisioning");

    const expSnap = await bridge.dispatchIntent({ type: "confirm-provisioning" });
    expect(expSnap.flowState).toBe("expedition");
    expect(expSnap.viewModel.kind).toBe("expedition");

    const combatSnap = await bridge.dispatchIntent({ type: "launch-expedition" });
    expect(combatSnap.flowState).toBe("combat");
    expect(combatSnap.viewModel.kind).toBe("expedition");
  });

  it("replay: provisioning → return-to-town", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    await bridge.dispatchIntent({ type: "start-provisioning" });
    const townSnap = await bridge.dispatchIntent({ type: "return-to-town" });

    expect(townSnap.flowState).toBe("town");
    expect(townSnap.viewModel.kind).toBe("town");
  });
});

describe("build-run smoke: meta-loop continuation", () => {
  it("replay: continue-from-result returns to town", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    const resultSnap = await bridge.dispatchIntent({ type: "continue-from-result" });
    expect(resultSnap.flowState).toBe("town");
    expect(resultSnap.viewModel.kind).toBe("town");
  });

  it("replay: resume-from-return returns to town", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    const returnSnap = await bridge.dispatchIntent({ type: "resume-from-return" });
    expect(returnSnap.flowState).toBe("town");
    expect(returnSnap.viewModel.kind).toBe("town");
  });

  it("live: continue-from-result returns to town", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();

    const resultSnap = await bridge.dispatchIntent({ type: "continue-from-result" });
    expect(resultSnap.flowState).toBe("town");
    expect(resultSnap.viewModel.kind).toBe("town");
  });

  it("live: resume-from-return returns to town", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();

    const returnSnap = await bridge.dispatchIntent({ type: "resume-from-return" });
    expect(returnSnap.flowState).toBe("town");
    expect(returnSnap.viewModel.kind).toBe("town");
  });

  it("replay: full meta-loop cycle town → expedition → result → town", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });
    await bridge.dispatchIntent({ type: "launch-expedition" });

    const resultSnap = await bridge.dispatchIntent({ type: "continue-from-result" });
    expect(resultSnap.flowState).toBe("town");
    expect(resultSnap.viewModel.kind).toBe("town");

    const provSnap = await bridge.dispatchIntent({ type: "start-provisioning" });
    expect(provSnap.flowState).toBe("provisioning");
    expect(provSnap.viewModel.kind).toBe("provisioning");
  });
});

describe("build-run smoke: screen resolution", () => {
  it("town snapshot resolves to town screen", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();
    const snapshot = bridge.currentSnapshot();
    expect(resolveScreen(snapshot)).toBe("town");
  });

  it("session store round-trips snapshot correctly", async () => {
    const bridge = new ReplayRuntimeBridge();
    const store = createSessionStore(fatalSnapshot);

    bridge.subscribe((snap) => store.replace(snap));

    await bridge.boot();

    expect(store.snapshot().viewModel.kind).toBe("town");
    expect(resolveScreen(store.snapshot())).toBe("town");
  });
});

describe("build-run smoke: bridge boundary integrity", () => {
  it("replay bridge exposes correct mode", () => {
    const bridge = new ReplayRuntimeBridge();
    expect(bridge.mode).toBe("replay");
    expect(bridge.id).toBe("ddgc-replay-bridge");
  });

  it("live bridge exposes correct mode", () => {
    const bridge = new LiveRuntimeBridge();
    expect(bridge.mode).toBe("live");
    expect(bridge.id).toBe("ddgc-live-bridge");
  });

  it("subscription returns unsubscribe function", () => {
    const bridge = new ReplayRuntimeBridge();
    const unsubsribe = bridge.subscribe(() => {});
    expect(typeof unsubsribe).toBe("function");
    unsubsribe();
  });

  it("multiple subscriptions all receive updates", async () => {
    const bridge = new ReplayRuntimeBridge();
    const updates: string[] = [];

    bridge.subscribe(() => updates.push("a"));
    bridge.subscribe(() => updates.push("b"));

    await bridge.boot();

    expect(updates).toContain("a");
    expect(updates).toContain("b");
  });
});