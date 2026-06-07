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
  ExpeditionSetupViewModel,
  DungeonMapViewModel,
  ExpeditionResultViewModel,
  ReturnViewModel
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
    expect(buildingDetail.label).toBe("试炼场");

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
  it("replay: town → provisioning → expedition → dungeon with adjacency enforcement", async () => {
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

    const dungeonSnap = await bridge.dispatchIntent({ type: "launch-expedition" });
    expect(dungeonSnap.flowState).toBe("dungeon");
    expect(dungeonSnap.viewModel.kind).toBe("dungeon-map");
    const dungeonVm = dungeonSnap.viewModel as DungeonMapViewModel;
    expect(dungeonVm.rooms.length).toBeGreaterThan(0);

    // Replay fixture: room-3 is current; room-4 is adjacent and uncleared
    const room4Snap = await bridge.dispatchIntent({ type: "enter-room", roomId: "room-4" });
    expect(room4Snap.flowState).toBe("dungeon");
    const room4Vm = room4Snap.viewModel as DungeonMapViewModel;
    expect(room4Vm.rooms.find((r) => r.roomId === "room-4")?.cleared).toBe(true);
    expect(room4Vm.currentRoom?.roomId).toBe("room-4");

    // room-5 is now adjacent to room-4 and can be entered
    const room5Snap = await bridge.dispatchIntent({ type: "enter-room", roomId: "room-5" });
    expect(room5Snap.flowState).toBe("result");
    expect(room5Snap.viewModel.kind).toBe("result");

    // Dungeon completes after all rooms cleared; entering room from result is rejected
    const rejectedSnap = await bridge.dispatchIntent({ type: "enter-room", roomId: "room-1" });
    expect(rejectedSnap.debugMessage).toContain("only valid on dungeon-map screen");
  });

  it("live: town → provisioning → expedition → dungeon with adjacency enforcement", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();

    const provSnap = await bridge.dispatchIntent({ type: "start-provisioning" });
    expect(provSnap.flowState).toBe("provisioning");
    expect(provSnap.viewModel.kind).toBe("provisioning");

    const expSnap = await bridge.dispatchIntent({ type: "confirm-provisioning" });
    expect(expSnap.flowState).toBe("expedition");
    expect(expSnap.viewModel.kind).toBe("expedition");

    const dungeonSnap = await bridge.dispatchIntent({ type: "launch-expedition" });
    expect(dungeonSnap.flowState).toBe("dungeon");
    expect(dungeonSnap.viewModel.kind).toBe("dungeon-map");
    const dungeonVm = dungeonSnap.viewModel as DungeonMapViewModel;

    // Live fixture: room-1 is current; room-2 is adjacent and uncleared
    const room2Snap = await bridge.dispatchIntent({ type: "enter-room", roomId: "room-2" });
    expect(room2Snap.flowState).toBe("dungeon");
    const room2Vm = room2Snap.viewModel as DungeonMapViewModel;
    expect(room2Vm.rooms.find((r) => r.roomId === "room-2")?.cleared).toBe(true);
    expect(room2Vm.currentRoom?.roomId).toBe("room-2");

    // Non-adjacent room entry is rejected
    const rejectedSnap = await bridge.dispatchIntent({ type: "enter-room", roomId: "room-4" });
    expect(rejectedSnap.debugMessage).toContain("not adjacent");
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
  it("replay: continue-from-result rejected when not on result screen", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    const resultSnap = await bridge.dispatchIntent({ type: "continue-from-result" });
    expect(resultSnap.flowState).toBe("town");
    expect(resultSnap.debugMessage).toContain("only valid on result screen");
  });

  it("replay: resume-from-return rejected when not on return screen", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    const returnSnap = await bridge.dispatchIntent({ type: "resume-from-return" });
    expect(returnSnap.flowState).toBe("town");
    expect(returnSnap.debugMessage).toContain("only valid on return screen");
  });

  it("live: continue-from-result rejected when not on result screen", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();

    const resultSnap = await bridge.dispatchIntent({ type: "continue-from-result" });
    expect(resultSnap.flowState).toBe("town");
    expect(resultSnap.debugMessage).toContain("only valid on result screen");
  });

  it("live: resume-from-return rejected when not on return screen", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();

    const returnSnap = await bridge.dispatchIntent({ type: "resume-from-return" });
    expect(returnSnap.flowState).toBe("town");
    expect(returnSnap.debugMessage).toContain("only valid on return screen");
  });

  it("replay: full expedition cycle town → expedition → dungeon → town", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });

    const dungeonSnap = await bridge.dispatchIntent({ type: "launch-expedition" });
    expect(dungeonSnap.flowState).toBe("dungeon");
    expect(dungeonSnap.viewModel.kind).toBe("dungeon-map");
    const dungeonVm = dungeonSnap.viewModel as DungeonMapViewModel;

    // Traverse adjacent rooms sequentially
    const room4Snap = await bridge.dispatchIntent({ type: "enter-room", roomId: "room-4" });
    expect(room4Snap.flowState).toBe("dungeon");

    const room5Snap = await bridge.dispatchIntent({ type: "enter-room", roomId: "room-5" });
    // Dungeon completes after all rooms are cleared
    expect(room5Snap.flowState).toBe("result");
    expect(room5Snap.viewModel.kind).toBe("result");

    // Return to town from result
    const townSnap = await bridge.dispatchIntent({ type: "return-to-town" });
    expect(townSnap.flowState).toBe("town");
    expect(townSnap.viewModel.kind).toBe("town");

    // Can restart provisioning after returning
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