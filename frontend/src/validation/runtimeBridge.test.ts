import { describe, expect, it } from "vitest";

import type { BuildingDetailViewModel, HeroDetailViewModel, ProvisioningViewModel, ExpeditionSetupViewModel, DungeonAssistViewModel, DungeonMapViewModel, ExpeditionResultViewModel, ReturnViewModel } from "../bridge/contractTypes";
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
    expect(buildingDetail.label).toBe("试炼场");
    expect(buildingDetail.status).toBe("ready");
    expect(buildingDetail.actions.length).toBeGreaterThan(0);
  });

  it("replay open-building intent for blacksmith shows ready status", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    const snapshot = await bridge.dispatchIntent({
      type: "open-building",
      buildingId: "blacksmith"
    });

    expect(snapshot.viewModel.kind).toBe("building-detail");
    const buildingDetail = snapshot.viewModel as BuildingDetailViewModel;
    expect(buildingDetail.label).toBe("锻造舱");
    expect(buildingDetail.status).toBe("ready");
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
    expect(buildingDetail.label).toBe("次元感知塔");
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
    expect(buildingDetail.label).toBe("试炼场");
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

  it("replay launch-expedition transitions to dungeon-assist state", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });

    const snapshot = await bridge.dispatchIntent({ type: "launch-expedition" });

    expect(snapshot.flowState).toBe("dungeon-assist");
    expect(snapshot.viewModel.kind).toBe("dungeon-assist");
    const assistVm = snapshot.viewModel as DungeonAssistViewModel;
    expect(assistVm.party.length).toBeGreaterThan(0);
    expect(assistVm.canContinue).toBe(false);
  });

  it("replay rejects launch-expedition outside expedition", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    const snapshot = await bridge.dispatchIntent({ type: "launch-expedition" });

    expect(snapshot.flowState).toBe("town");
    expect(snapshot.viewModel.kind).toBe("town");
    expect(snapshot.debugMessage).toContain("rejected");
  });

  it("replay rejects launch-expedition when expedition is not launchable", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    const expeditionSnapshot = await bridge.dispatchIntent({ type: "confirm-provisioning" });
    (bridge as unknown as { snapshot: typeof expeditionSnapshot }).snapshot = {
      ...expeditionSnapshot,
      viewModel: {
        ...(expeditionSnapshot.viewModel as ExpeditionSetupViewModel),
        isLaunchable: false
      }
    };

    const snapshot = await bridge.dispatchIntent({ type: "launch-expedition" });

    expect(snapshot.flowState).toBe("expedition");
    expect(snapshot.viewModel.kind).toBe("expedition");
    expect(snapshot.debugMessage).toContain("rejected");
  });

  it("replay rejects enter-dungeon-assist outside expedition", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    const snapshot = await bridge.dispatchIntent({ type: "enter-dungeon-assist" });

    expect(snapshot.flowState).toBe("town");
    expect(snapshot.viewModel.kind).toBe("town");
    expect(snapshot.debugMessage).toContain("rejected");
  });

  it("replay continue-from-dungeon transitions to dungeon-map state", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });
    await bridge.dispatchIntent({ type: "launch-expedition" });
    await bridge.dispatchIntent({ type: "use-assist-action", actionId: "heal-wound" });

    const snapshot = await bridge.dispatchIntent({ type: "continue-from-dungeon" });

    expect(snapshot.flowState).toBe("dungeon-map");
    expect(snapshot.viewModel.kind).toBe("dungeon-map");
    const mapVm = snapshot.viewModel as DungeonMapViewModel;
    expect(mapVm.rooms.length).toBeGreaterThan(0);
    expect(mapVm.party.length).toBeGreaterThan(0);
  });

  it("replay rejects continue-from-dungeon before assist action is used", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });
    await bridge.dispatchIntent({ type: "launch-expedition" });

    const snapshot = await bridge.dispatchIntent({ type: "continue-from-dungeon" });

    expect(snapshot.flowState).toBe("dungeon-assist");
    expect(snapshot.viewModel.kind).toBe("dungeon-assist");
    expect(snapshot.debugMessage).toContain("rejected");
  });

  it("replay rejects locked and unknown assist actions", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });
    await bridge.dispatchIntent({ type: "launch-expedition" });

    const lockedSnapshot = await bridge.dispatchIntent({ type: "use-assist-action", actionId: "apply-buff" });
    expect(lockedSnapshot.flowState).toBe("dungeon-assist");
    expect(lockedSnapshot.viewModel.kind).toBe("dungeon-assist");
    expect((lockedSnapshot.viewModel as DungeonAssistViewModel).canContinue).toBe(false);
    expect(lockedSnapshot.debugMessage).toContain("rejected");

    const unknownSnapshot = await bridge.dispatchIntent({ type: "use-assist-action", actionId: "unknown-action" });
    expect(unknownSnapshot.flowState).toBe("dungeon-assist");
    expect(unknownSnapshot.viewModel.kind).toBe("dungeon-assist");
    expect((unknownSnapshot.viewModel as DungeonAssistViewModel).canContinue).toBe(false);
    expect(unknownSnapshot.debugMessage).toContain("rejected");
  });

  it("replay rejects unknown assist heroes", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });
    await bridge.dispatchIntent({ type: "launch-expedition" });

    const snapshot = await bridge.dispatchIntent({ type: "select-assist-hero", heroId: "unknown-hero" });

    expect(snapshot.flowState).toBe("dungeon-assist");
    expect(snapshot.viewModel.kind).toBe("dungeon-assist");
    expect((snapshot.viewModel as DungeonAssistViewModel).selectedHeroId).not.toBe("unknown-hero");
    expect(snapshot.debugMessage).toContain("rejected");
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

  it("live launch-expedition transitions to dungeon-assist state", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });

    const snapshot = await bridge.dispatchIntent({ type: "launch-expedition" });

    expect(snapshot.flowState).toBe("dungeon-assist");
    expect(snapshot.viewModel.kind).toBe("dungeon-assist");
    const assistVm = snapshot.viewModel as DungeonAssistViewModel;
    expect(assistVm.party.length).toBeGreaterThan(0);
  });

  it("live rejects launch-expedition outside expedition", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();

    const snapshot = await bridge.dispatchIntent({ type: "launch-expedition" });

    expect(snapshot.flowState).toBe("town");
    expect(snapshot.viewModel.kind).toBe("town");
    expect(snapshot.debugMessage).toContain("rejected");
  });

  it("live rejects launch-expedition when expedition is not launchable", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    const expeditionSnapshot = await bridge.dispatchIntent({ type: "confirm-provisioning" });
    (bridge as unknown as { snapshot: typeof expeditionSnapshot }).snapshot = {
      ...expeditionSnapshot,
      viewModel: {
        ...(expeditionSnapshot.viewModel as ExpeditionSetupViewModel),
        isLaunchable: false
      }
    };

    const snapshot = await bridge.dispatchIntent({ type: "launch-expedition" });

    expect(snapshot.flowState).toBe("expedition");
    expect(snapshot.viewModel.kind).toBe("expedition");
    expect(snapshot.debugMessage).toContain("rejected");
  });

  it("live rejects enter-dungeon-assist outside expedition", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();

    const snapshot = await bridge.dispatchIntent({ type: "enter-dungeon-assist" });

    expect(snapshot.flowState).toBe("town");
    expect(snapshot.viewModel.kind).toBe("town");
    expect(snapshot.debugMessage).toContain("rejected");
  });

  it("live rejects continue-from-dungeon before assist action is used", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });
    await bridge.dispatchIntent({ type: "launch-expedition" });

    const snapshot = await bridge.dispatchIntent({ type: "continue-from-dungeon" });

    expect(snapshot.flowState).toBe("dungeon-assist");
    expect(snapshot.viewModel.kind).toBe("dungeon-assist");
    expect(snapshot.debugMessage).toContain("rejected");
  });

  it("live rejects locked and unknown assist actions", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });
    await bridge.dispatchIntent({ type: "launch-expedition" });

    const lockedSnapshot = await bridge.dispatchIntent({ type: "use-assist-action", actionId: "apply-buff" });
    expect(lockedSnapshot.flowState).toBe("dungeon-assist");
    expect(lockedSnapshot.viewModel.kind).toBe("dungeon-assist");
    expect((lockedSnapshot.viewModel as DungeonAssistViewModel).canContinue).toBe(false);
    expect(lockedSnapshot.debugMessage).toContain("rejected");

    const unknownSnapshot = await bridge.dispatchIntent({ type: "use-assist-action", actionId: "unknown-action" });
    expect(unknownSnapshot.flowState).toBe("dungeon-assist");
    expect(unknownSnapshot.viewModel.kind).toBe("dungeon-assist");
    expect((unknownSnapshot.viewModel as DungeonAssistViewModel).canContinue).toBe(false);
    expect(unknownSnapshot.debugMessage).toContain("rejected");
  });

  it("live rejects unknown assist heroes", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });
    await bridge.dispatchIntent({ type: "launch-expedition" });

    const snapshot = await bridge.dispatchIntent({ type: "select-assist-hero", heroId: "unknown-hero" });

    expect(snapshot.flowState).toBe("dungeon-assist");
    expect(snapshot.viewModel.kind).toBe("dungeon-assist");
    expect((snapshot.viewModel as DungeonAssistViewModel).selectedHeroId).not.toBe("unknown-hero");
    expect(snapshot.debugMessage).toContain("rejected");
  });

  it("replay enter-room updates current room and exploration progress", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });
    await bridge.dispatchIntent({ type: "launch-expedition" });
    await bridge.dispatchIntent({ type: "use-assist-action", actionId: "heal-wound" });
    await bridge.dispatchIntent({ type: "continue-from-dungeon" });

    const snapshot = await bridge.dispatchIntent({ type: "enter-room", roomId: "room-combat-1" });

    expect(snapshot.viewModel.kind).toBe("dungeon-map");
    const mapVm = snapshot.viewModel as DungeonMapViewModel;
    expect(mapVm.currentRoomId).toBe("room-combat-1");
    expect(mapVm.exploredCount).toBeGreaterThan(1);
  });

  it("replay retreat-from-dungeon transitions to result with partial outcome", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });
    await bridge.dispatchIntent({ type: "launch-expedition" });
    await bridge.dispatchIntent({ type: "use-assist-action", actionId: "heal-wound" });
    await bridge.dispatchIntent({ type: "continue-from-dungeon" });

    const snapshot = await bridge.dispatchIntent({ type: "retreat-from-dungeon" });

    expect(snapshot.flowState).toBe("result");
    expect(snapshot.viewModel.kind).toBe("result");
    const resultVm = snapshot.viewModel as ExpeditionResultViewModel;
    expect(resultVm.outcome).toBe("partial");
  });

  it("replay complete-dungeon is rejected when dungeon is not complete", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });
    await bridge.dispatchIntent({ type: "launch-expedition" });
    await bridge.dispatchIntent({ type: "use-assist-action", actionId: "heal-wound" });
    await bridge.dispatchIntent({ type: "continue-from-dungeon" });

    const snapshot = await bridge.dispatchIntent({ type: "complete-dungeon" });

    expect(snapshot.viewModel.kind).toBe("dungeon-map");
    expect(snapshot.debugMessage).toContain("not complete");
    const mapVm = snapshot.viewModel as DungeonMapViewModel;
    expect(mapVm.isComplete).toBe(false);
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
    expect(launchSnapshot.flowState).toBe("dungeon-assist");
    expect(launchSnapshot.viewModel.kind).toBe("dungeon-assist");
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
    expect(launchSnapshot.flowState).toBe("dungeon-assist");
    expect(launchSnapshot.viewModel.kind).toBe("dungeon-assist");
  });
});

describe("result and return meta-loop continuation", () => {
  it("continue-from-result transitions to return state", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });
    await bridge.dispatchIntent({ type: "launch-expedition" });
    await bridge.dispatchIntent({ type: "use-assist-action", actionId: "heal-wound" });
    await bridge.dispatchIntent({ type: "continue-from-dungeon" });
    await bridge.dispatchIntent({ type: "retreat-from-dungeon" });

    const snapshot = await bridge.dispatchIntent({ type: "continue-from-result" });
    expect(snapshot.flowState).toBe("return");
    expect(snapshot.viewModel.kind).toBe("return");
    const returnVm = snapshot.viewModel as ReturnViewModel;
    expect(returnVm.isTownResumeAvailable).toBe(true);
  });

  it("resume-from-return intent returns to town", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });
    await bridge.dispatchIntent({ type: "launch-expedition" });
    await bridge.dispatchIntent({ type: "use-assist-action", actionId: "heal-wound" });
    await bridge.dispatchIntent({ type: "continue-from-dungeon" });
    await bridge.dispatchIntent({ type: "retreat-from-dungeon" });
    await bridge.dispatchIntent({ type: "continue-from-result" });

    const snapshot = await bridge.dispatchIntent({ type: "resume-from-return" });
    expect(snapshot.flowState).toBe("town");
    expect(snapshot.viewModel.kind).toBe("town");
  });

  it("continue-from-result is handled in live bridge without error", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });
    await bridge.dispatchIntent({ type: "launch-expedition" });
    await bridge.dispatchIntent({ type: "use-assist-action", actionId: "heal-wound" });
    await bridge.dispatchIntent({ type: "continue-from-dungeon" });
    await bridge.dispatchIntent({ type: "retreat-from-dungeon" });

    const snapshot = await bridge.dispatchIntent({ type: "continue-from-result" });
    expect(snapshot.flowState).toBe("return");
    expect(snapshot.viewModel.kind).toBe("return");
    const returnVm = snapshot.viewModel as ReturnViewModel;
    expect(returnVm.isTownResumeAvailable).toBe(true);
  });

  it("resume-from-return is handled in live bridge without error", async () => {
    const bridge = new LiveRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });
    await bridge.dispatchIntent({ type: "launch-expedition" });
    await bridge.dispatchIntent({ type: "use-assist-action", actionId: "heal-wound" });
    await bridge.dispatchIntent({ type: "continue-from-dungeon" });
    await bridge.dispatchIntent({ type: "retreat-from-dungeon" });
    await bridge.dispatchIntent({ type: "continue-from-result" });

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
    await bridge.dispatchIntent({ type: "use-assist-action", actionId: "heal-wound" });

    // Continue from dungeon-assist -> dungeon-map -> result
    const mapSnap = await bridge.dispatchIntent({ type: "continue-from-dungeon" });
    expect(mapSnap.flowState).toBe("dungeon-map");
    expect(mapSnap.viewModel.kind).toBe("dungeon-map");

    const resultSnap = await bridge.dispatchIntent({ type: "retreat-from-dungeon" });
    expect(resultSnap.flowState).toBe("result");
    expect(resultSnap.viewModel.kind).toBe("result");

    // Continue from result — transitions to return screen
    const returnSnap = await bridge.dispatchIntent({ type: "continue-from-result" });
    expect(returnSnap.flowState).toBe("return");
    expect(returnSnap.viewModel.kind).toBe("return");

    // Resume from return — back to town
    const townSnap = await bridge.dispatchIntent({ type: "resume-from-return" });
    expect(townSnap.flowState).toBe("town");
    expect(townSnap.viewModel.kind).toBe("town");

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
    await bridge.dispatchIntent({ type: "use-assist-action", actionId: "heal-wound" });

    // Continue from dungeon-assist -> dungeon-map -> result -> return -> town
    await bridge.dispatchIntent({ type: "continue-from-dungeon" });
    await bridge.dispatchIntent({ type: "retreat-from-dungeon" });
    await bridge.dispatchIntent({ type: "continue-from-result" });
    const townSnapshot = await bridge.dispatchIntent({ type: "resume-from-return" });
    expect(townSnapshot.flowState).toBe("town");
    expect(townSnapshot.viewModel.kind).toBe("town");

    // Can restart provisioning after returning
    const provSnapshot = await bridge.dispatchIntent({ type: "start-provisioning" });
    expect(provSnapshot.flowState).toBe("provisioning");
    expect(provSnapshot.viewModel.kind).toBe("provisioning");
  });
});
