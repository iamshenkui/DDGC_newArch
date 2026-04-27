import { describe, expect, it } from "vitest";

import type { BuildingDetailViewModel, HeroDetailViewModel, ProvisioningViewModel, ExpeditionSetupViewModel, ExpeditionResultViewModel, ReturnViewModel } from "../bridge/contractTypes";
import { LiveRuntimeBridge } from "../bridge/LiveRuntimeBridge";
import { ReplayRuntimeBridge } from "../bridge/ReplayRuntimeBridge";

describe("runtime bridge skeleton", () => {
  it("boots replay mode into the town shell placeholder", async () => {
    const bridge = new ReplayRuntimeBridge();
    const snapshot = await bridge.boot();

    expect(snapshot.lifecycle).toBe("ready");
    expect(snapshot.flowState).toBe("town");
    expect(snapshot.viewModel.kind).toBe("town");
  });

  it("surfaces live mode as ready town shell after wiring", async () => {
    const bridge = new LiveRuntimeBridge();
    const snapshot = await bridge.boot();

    expect(snapshot.lifecycle).toBe("ready");
    expect(snapshot.flowState).toBe("town");
    expect(snapshot.viewModel.kind).toBe("town");
  });

  it("replay open-hero intent shows hero detail view", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    const snapshot = await bridge.dispatchIntent({
      type: "open-hero",
      heroId: "hero-hunter-01"
    });

    expect(snapshot.viewModel.kind).toBe("hero-detail");
    const heroDetail = snapshot.viewModel as HeroDetailViewModel;
    expect(heroDetail.kind).toBe("hero-detail");
    expect(heroDetail.name).toBe("Shen");
    expect(heroDetail.classLabel).toBe("Hunter");
  });

  it("replay return-to-town intent returns to town view", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    await bridge.dispatchIntent({ type: "open-hero", heroId: "hero-hunter-01" });
    const detailSnapshot = await bridge.dispatchIntent({ type: "return-to-town" });

    expect(detailSnapshot.viewModel.kind).toBe("town");
  });

  it("live open-hero intent shows hero detail view", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();

    const snapshot = await bridge.dispatchIntent({
      type: "open-hero",
      heroId: "hero-hunter-live-01"
    });

    expect(snapshot.viewModel.kind).toBe("hero-detail");
    const heroDetail = snapshot.viewModel as HeroDetailViewModel;
    expect(heroDetail.kind).toBe("hero-detail");
    expect(heroDetail.name).toBe("Yuan");
    expect(heroDetail.classLabel).toBe("Hunter");
  });

  it("replay open-building intent shows building detail view", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    const snapshot = await bridge.dispatchIntent({
      type: "open-building",
      buildingId: "guild"
    });

    expect(snapshot.viewModel.kind).toBe("building-detail");
    const buildingDetail = snapshot.viewModel as BuildingDetailViewModel;
    expect(buildingDetail.kind).toBe("building-detail");
    expect(buildingDetail.label).toBe("Guild");
    expect(buildingDetail.status).toBe("ready");
    expect(buildingDetail.actions.length).toBeGreaterThan(0);
  });

  it("replay open-building intent for blacksmith shows partial status", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    const snapshot = await bridge.dispatchIntent({
      type: "open-building",
      buildingId: "blacksmith"
    });

    expect(snapshot.viewModel.kind).toBe("building-detail");
    const buildingDetail = snapshot.viewModel as BuildingDetailViewModel;
    expect(buildingDetail.label).toBe("Blacksmith");
    expect(buildingDetail.status).toBe("partial");
  });

  it("replay return-to-town after opening building returns to town view", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    await bridge.dispatchIntent({ type: "open-building", buildingId: "guild" });
    const detailSnapshot = await bridge.dispatchIntent({ type: "return-to-town" });

    expect(detailSnapshot.viewModel.kind).toBe("town");
  });

  it("live open-building intent shows building detail view", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();

    const snapshot = await bridge.dispatchIntent({
      type: "open-building",
      buildingId: "stagecoach"
    });

    expect(snapshot.viewModel.kind).toBe("building-detail");
    const buildingDetail = snapshot.viewModel as BuildingDetailViewModel;
    expect(buildingDetail.kind).toBe("building-detail");
    expect(buildingDetail.label).toBe("Stagecoach");
    expect(buildingDetail.status).toBe("ready");
  });

  it("live open-building intent for guild shows ready status", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();

    const snapshot = await bridge.dispatchIntent({
      type: "open-building",
      buildingId: "guild"
    });

    expect(snapshot.viewModel.kind).toBe("building-detail");
    const buildingDetail = snapshot.viewModel as BuildingDetailViewModel;
    expect(buildingDetail.label).toBe("Guild");
    expect(buildingDetail.status).toBe("ready");
  });

  it("building action intent is handled without error", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    const snapshot = await bridge.dispatchIntent({
      type: "open-building",
      buildingId: "guild"
    });

    expect(snapshot.viewModel.kind).toBe("building-detail");

    const actionSnapshot = await bridge.dispatchIntent({
      type: "building-action",
      actionId: "train-combat"
    });

    expect(actionSnapshot.debugMessage).toContain("train-combat");
  });
});

describe("provisioning and expedition launch flow", () => {
  it("replay start-provisioning transitions to provisioning state", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    const snapshot = await bridge.dispatchIntent({ type: "start-provisioning" });

    expect(snapshot.flowState).toBe("provisioning");
    expect(snapshot.viewModel.kind).toBe("provisioning");
    const provVm = snapshot.viewModel as ProvisioningViewModel;
    expect(provVm.party.length).toBeGreaterThan(0);
    expect(provVm.isReadyToLaunch).toBe(true);
  });

  it("replay toggle-hero-selection updates party selection", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });

    const provSnapshot = await bridge.dispatchIntent({
      type: "toggle-hero-selection",
      heroId: "hero-hunter-01"
    });

    const provVm = provSnapshot.viewModel as ProvisioningViewModel;
    const hunter = provVm.party.find((h) => h.id === "hero-hunter-01");
    expect(hunter?.isSelected).toBe(false);
  });

  it("replay confirm-provisioning transitions to expedition state", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });

    const snapshot = await bridge.dispatchIntent({ type: "confirm-provisioning" });

    expect(snapshot.flowState).toBe("expedition");
    expect(snapshot.viewModel.kind).toBe("expedition");
    const expVm = snapshot.viewModel as ExpeditionSetupViewModel;
    expect(expVm.isLaunchable).toBe(true);
  });

  it("replay launch-expedition transitions to combat state", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });

    const snapshot = await bridge.dispatchIntent({ type: "launch-expedition" });

    expect(snapshot.flowState).toBe("combat");
    expect(snapshot.viewModel.kind).toBe("expedition");
  });

  it("replay return-to-town from provisioning returns to town", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });

    const snapshot = await bridge.dispatchIntent({ type: "return-to-town" });

    expect(snapshot.flowState).toBe("town");
    expect(snapshot.viewModel.kind).toBe("town");
  });

  it("live start-provisioning transitions to provisioning state", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();

    const snapshot = await bridge.dispatchIntent({ type: "start-provisioning" });

    expect(snapshot.flowState).toBe("provisioning");
    expect(snapshot.viewModel.kind).toBe("provisioning");
    const provVm = snapshot.viewModel as ProvisioningViewModel;
    expect(provVm.party.length).toBeGreaterThan(0);
  });

  it("live confirm-provisioning transitions to expedition state", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });

    const snapshot = await bridge.dispatchIntent({ type: "confirm-provisioning" });

    expect(snapshot.flowState).toBe("expedition");
    expect(snapshot.viewModel.kind).toBe("expedition");
  });

  it("live launch-expedition transitions to combat state", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });

    const snapshot = await bridge.dispatchIntent({ type: "launch-expedition" });

    expect(snapshot.flowState).toBe("combat");
    expect(snapshot.viewModel.kind).toBe("expedition");
  });

  it("town -> provision -> launch path is reproducible in replay", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    const townSnapshot = await bridge.currentSnapshot();
    expect(townSnapshot.flowState).toBe("town");
    expect(townSnapshot.viewModel.kind).toBe("town");

    const provSnapshot = await bridge.dispatchIntent({ type: "start-provisioning" });
    expect(provSnapshot.flowState).toBe("provisioning");
    expect(provSnapshot.viewModel.kind).toBe("provisioning");

    const expSnapshot = await bridge.dispatchIntent({ type: "confirm-provisioning" });
    expect(expSnapshot.flowState).toBe("expedition");
    expect(expSnapshot.viewModel.kind).toBe("expedition");

    const launchSnapshot = await bridge.dispatchIntent({ type: "launch-expedition" });
    expect(launchSnapshot.flowState).toBe("combat");
    expect(launchSnapshot.viewModel.kind).toBe("expedition");
  });

  it("town -> provision -> launch path is reproducible in live", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();

    const townSnapshot = await bridge.currentSnapshot();
    expect(townSnapshot.flowState).toBe("town");
    expect(townSnapshot.viewModel.kind).toBe("town");

    const provSnapshot = await bridge.dispatchIntent({ type: "start-provisioning" });
    expect(provSnapshot.flowState).toBe("provisioning");
    expect(provSnapshot.viewModel.kind).toBe("provisioning");

    const expSnapshot = await bridge.dispatchIntent({ type: "confirm-provisioning" });
    expect(expSnapshot.flowState).toBe("expedition");
    expect(expSnapshot.viewModel.kind).toBe("expedition");

    const launchSnapshot = await bridge.dispatchIntent({ type: "launch-expedition" });
    expect(launchSnapshot.flowState).toBe("combat");
    expect(launchSnapshot.viewModel.kind).toBe("expedition");
  });
});

describe("result and return meta-loop continuation", () => {
  it("continue-from-result intent returns to town", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    const snapshot = await bridge.dispatchIntent({ type: "continue-from-result" });
    expect(snapshot.flowState).toBe("town");
    expect(snapshot.viewModel.kind).toBe("town");
  });

  it("resume-from-return intent returns to town", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    const snapshot = await bridge.dispatchIntent({ type: "resume-from-return" });
    expect(snapshot.flowState).toBe("town");
    expect(snapshot.viewModel.kind).toBe("town");
  });

  it("continue-from-result is handled in live bridge without error", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();

    const snapshot = await bridge.dispatchIntent({ type: "continue-from-result" });
    expect(snapshot.flowState).toBe("town");
    expect(snapshot.viewModel.kind).toBe("town");
  });

  it("resume-from-return is handled in live bridge without error", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();

    const snapshot = await bridge.dispatchIntent({ type: "resume-from-return" });
    expect(snapshot.flowState).toBe("town");
    expect(snapshot.viewModel.kind).toBe("town");
  });

  it("meta-loop can cycle through result and back to town in replay", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    // Go through expedition flow
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });
    await bridge.dispatchIntent({ type: "launch-expedition" });

    // Continue from result
    const resultSnapshot = await bridge.dispatchIntent({ type: "continue-from-result" });
    expect(resultSnapshot.flowState).toBe("town");
    expect(resultSnapshot.viewModel.kind).toBe("town");

    // Can restart provisioning after returning
    const provSnapshot = await bridge.dispatchIntent({ type: "start-provisioning" });
    expect(provSnapshot.flowState).toBe("provisioning");
    expect(provSnapshot.viewModel.kind).toBe("provisioning");
  });

  it("meta-loop can cycle through return and back to town in replay", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    // Go through expedition flow
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });
    await bridge.dispatchIntent({ type: "launch-expedition" });

    // Resume from return
    const returnSnapshot = await bridge.dispatchIntent({ type: "resume-from-return" });
    expect(returnSnapshot.flowState).toBe("town");
    expect(returnSnapshot.viewModel.kind).toBe("town");

    // Can restart provisioning after returning
    const provSnapshot = await bridge.dispatchIntent({ type: "start-provisioning" });
    expect(provSnapshot.flowState).toBe("provisioning");
    expect(provSnapshot.viewModel.kind).toBe("provisioning");
  });
});