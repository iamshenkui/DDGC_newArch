import { describe, expect, it } from "vitest";

import { resolveScreen, canTransition, type ScreenKey } from "./FlowController";
import type {
  DdgcFrontendSnapshot,
  DungeonHintViewModel,
  DungeonSelectViewModel,
  ExpeditionResultViewModel,
  ReturnViewModel,
  DungeonMapViewModel,
  CombatViewModel,
  DungeonInteractionViewModel,
} from "../bridge/contractTypes";
import {
  fatalSnapshot,
  unsupportedSnapshot,
  replayLoadingSnapshot,
  liveLoadingSnapshot,
  replayReadySnapshot,
  replayHeroDetailSnapshot,
  replayBuildingDetailSnapshot,
  startupSnapshot,
  dungeonSelectSnapshot,
  provisioningSnapshot,
  dungeonHintSnapshot,
  expeditionSnapshot,
  dungeonInteractionSnapshot,
  dungeonItemsSnapshot,
  dungeonAssistSnapshot,
  dungeonMapSnapshot,
  combatSnapshot,
  replayCombatViewModel,
  resultSnapshot,
  failureResultSnapshot,
  partialResultSnapshot,
  returnSnapshot,
  expeditionPlanningSnapshot,
} from "../validation/replayFixtures";

describe("FlowController", () => {
  describe("resolveScreen", () => {
    it("returns fatal screen for fatal lifecycle", () => {
      const screen = resolveScreen(fatalSnapshot);
      expect(screen).toBe("fatal");
    });

    it("returns unsupported screen for unsupported lifecycle", () => {
      const screen = resolveScreen(unsupportedSnapshot);
      expect(screen).toBe("unsupported");
    });

    it("returns loading screen for loading lifecycle", () => {
      const screen = resolveScreen(replayLoadingSnapshot);
      expect(screen).toBe("loading");
    });

    it("returns loading screen for booting lifecycle", () => {
      const bootingSnapshot: DdgcFrontendSnapshot = {
        ...replayLoadingSnapshot,
        lifecycle: "booting",
      };
      const screen = resolveScreen(bootingSnapshot);
      expect(screen).toBe("loading");
    });

    it("returns loading screen for live loading lifecycle", () => {
      const screen = resolveScreen(liveLoadingSnapshot);
      expect(screen).toBe("loading");
    });

    it("returns town screen for ready lifecycle with town flowState", () => {
      const screen = resolveScreen(replayReadySnapshot);
      expect(screen).toBe("town");
    });

    it("returns startup screen for ready lifecycle with boot flowState", () => {
      const screen = resolveScreen(startupSnapshot);
      expect(screen).toBe("startup");
    });

    it("returns building-detail screen for building detail view model", () => {
      const screen = resolveScreen(replayBuildingDetailSnapshot);
      expect(screen).toBe("building-detail");
    });

    it("returns provisioning screen for provisioning view model", () => {
      const screen = resolveScreen(provisioningSnapshot);
      expect(screen).toBe("provisioning");
    });

    it("returns dungeon-hint screen for dungeon hint view model", () => {
      const screen = resolveScreen(dungeonHintSnapshot);
      expect(screen).toBe("dungeon-hint");
    });

    it("returns expedition screen for expedition setup view model", () => {
      const screen = resolveScreen(expeditionSnapshot);
      expect(screen).toBe("expedition");
    });

    it("returns dungeon-interaction screen for dungeon interaction view model", () => {
      const screen = resolveScreen(dungeonInteractionSnapshot);
      expect(screen).toBe("dungeon-interaction");
    });

    it("returns dungeon-items screen for dungeon items view model", () => {
      const screen = resolveScreen(dungeonItemsSnapshot);
      expect(screen).toBe("dungeon-items");
    });

    it("returns dungeon-assist screen for dungeon assist view model", () => {
      const screen = resolveScreen(dungeonAssistSnapshot);
      expect(screen).toBe("dungeon-assist");
    });

    it("returns dungeon-map screen for dungeon map view model", () => {
      const screen = resolveScreen(dungeonMapSnapshot);
      expect(screen).toBe("dungeon-map");
    });

    it("returns combat screen for combat view model", () => {
      const screen = resolveScreen(combatSnapshot);
      expect(screen).toBe("combat");
    });

    it("returns result screen for result view model", () => {
      const screen = resolveScreen(resultSnapshot);
      expect(screen).toBe("result");
    });

    it("returns result screen for failure outcome result view model", () => {
      const screen = resolveScreen(failureResultSnapshot);
      expect(screen).toBe("result");
    });

    it("returns result screen for partial outcome result view model", () => {
      const screen = resolveScreen(partialResultSnapshot);
      expect(screen).toBe("result");
    });

    it("returns return screen for return view model", () => {
      const screen = resolveScreen(returnSnapshot);
      expect(screen).toBe("return");
    });
  });
});

describe("ScreenKey exhaustiveness", () => {
  const allScreenKeys: ScreenKey[] = ["startup", "loading", "town", "hero-detail", "building-detail", "expedition-planning", "dungeon-select", "provisioning", "dungeon-hint", "expedition", "dungeon-assist", "dungeon-map", "combat", "dungeon-interaction", "dungeon-items", "result", "return", "unsupported", "fatal"];

  it("covers all screen keys in FlowController.resolveScreen", () => {
    const snapshotsByScreen: Record<ScreenKey, DdgcFrontendSnapshot> = {
      startup: startupSnapshot,
      loading: replayLoadingSnapshot,
      town: replayReadySnapshot,
      "hero-detail": replayHeroDetailSnapshot,
      "building-detail": replayBuildingDetailSnapshot,
      "expedition-planning": expeditionPlanningSnapshot,
      "dungeon-select": dungeonSelectSnapshot,
      provisioning: provisioningSnapshot,
      "dungeon-hint": dungeonHintSnapshot,
      expedition: expeditionSnapshot,
      "dungeon-assist": dungeonAssistSnapshot,
      "dungeon-map": dungeonMapSnapshot,
      combat: combatSnapshot,
      "dungeon-interaction": dungeonInteractionSnapshot,
      "dungeon-items": dungeonItemsSnapshot,
      result: resultSnapshot,
      return: returnSnapshot,
      unsupported: unsupportedSnapshot,
      fatal: fatalSnapshot,
    };

    for (const key of allScreenKeys) {
      const snapshot = snapshotsByScreen[key];
      const screen = resolveScreen(snapshot);
      expect(screen).toBe(key);
    }
  });
});

describe("canTransition - result and return meta-loop continuation", () => {
  describe("continue-from-result transitions", () => {
    it("allows continue-from-result when isContinueAvailable is true", () => {
      const validation = canTransition(resultSnapshot, { type: "continue-from-result" });
      expect(validation.allowed).toBe(true);
    });

    it("allows continue-from-result when isContinueAvailable is explicitly true", () => {
      const availableResultSnapshot: DdgcFrontendSnapshot = {
        ...resultSnapshot,
        viewModel: {
          ...resultSnapshot.viewModel,
          isContinueAvailable: true
        } as ExpeditionResultViewModel
      };
      const validation = canTransition(availableResultSnapshot, { type: "continue-from-result" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects continue-from-result when isContinueAvailable is false", () => {
      const unavailableResultSnapshot: DdgcFrontendSnapshot = {
        ...resultSnapshot,
        viewModel: {
          ...resultSnapshot.viewModel,
          isContinueAvailable: false
        } as ExpeditionResultViewModel
      };
      const validation = canTransition(unavailableResultSnapshot, { type: "continue-from-result" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("not available");
    });

    it("rejects continue-from-result when not on result screen", () => {
      const validation = canTransition(replayReadySnapshot, { type: "continue-from-result" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid on result screen");
    });

    it("rejects continue-from-result when not on result screen (town)", () => {
      const validation = canTransition(replayReadySnapshot, { type: "continue-from-result" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid on result screen");
    });

    it("rejects continue-from-result when not on result screen (hero-detail)", () => {
      const validation = canTransition(replayHeroDetailSnapshot, { type: "continue-from-result" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid on result screen");
    });
  });

  describe("resume-from-return transitions", () => {
    it("allows resume-from-return when isTownResumeAvailable is true", () => {
      const validation = canTransition(returnSnapshot, { type: "resume-from-return" });
      expect(validation.allowed).toBe(true);
    });

    it("allows resume-from-return when isTownResumeAvailable is explicitly true", () => {
      const availableReturnSnapshot: DdgcFrontendSnapshot = {
        ...returnSnapshot,
        viewModel: {
          ...returnSnapshot.viewModel,
          isTownResumeAvailable: true
        } as ReturnViewModel
      };
      const validation = canTransition(availableReturnSnapshot, { type: "resume-from-return" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects resume-from-return when isTownResumeAvailable is false", () => {
      const unavailableReturnSnapshot: DdgcFrontendSnapshot = {
        ...returnSnapshot,
        viewModel: {
          ...returnSnapshot.viewModel,
          isTownResumeAvailable: false
        } as ReturnViewModel
      };
      const validation = canTransition(unavailableReturnSnapshot, { type: "resume-from-return" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("not available");
    });

    it("rejects resume-from-return when not on return screen", () => {
      const validation = canTransition(replayReadySnapshot, { type: "resume-from-return" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid on return screen");
    });

    it("rejects resume-from-return when not on return screen (building-detail)", () => {
      const validation = canTransition(replayBuildingDetailSnapshot, { type: "resume-from-return" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid on return screen");
    });
  });

  describe("return-to-town transitions", () => {
    it("allows return-to-town from provisioning", () => {
      const validation = canTransition(provisioningSnapshot, { type: "return-to-town" });
      expect(validation.allowed).toBe(true);
    });

    it("allows return-to-town from expedition", () => {
      const validation = canTransition(expeditionSnapshot, { type: "return-to-town" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects return-to-town when already in town", () => {
      const validation = canTransition(replayReadySnapshot, { type: "return-to-town" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("already in town");
    });

    it("allows return-to-town from result screen as fallback action", () => {
      const validation = canTransition(resultSnapshot, { type: "return-to-town" });
      expect(validation.allowed).toBe(true);
    });

    it("allows return-to-town from return screen as fallback action", () => {
      const validation = canTransition(returnSnapshot, { type: "return-to-town" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects return-to-town from loading screen", () => {
      const validation = canTransition(replayLoadingSnapshot, { type: "return-to-town" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("already in town");
    });
  });

  describe("meta-loop continuation validation", () => {
    it("proves meta-loop can continue from result without dead-end states", () => {
      // From result screen, continue-from-result should be allowed
      const continueValidation = canTransition(resultSnapshot, { type: "continue-from-result" });
      expect(continueValidation.allowed).toBe(true);

      // After continuing, we should be in town where start-provisioning is allowed
      const provValidation = canTransition(replayReadySnapshot, { type: "start-provisioning" });
      expect(provValidation.allowed).toBe(true);
    });

    it("proves meta-loop can continue from return without dead-end states", () => {
      // From return screen, resume-from-return should be allowed
      const resumeValidation = canTransition(returnSnapshot, { type: "resume-from-return" });
      expect(resumeValidation.allowed).toBe(true);

      // After resuming, we should be in town where start-provisioning is allowed
      const provValidation = canTransition(replayReadySnapshot, { type: "start-provisioning" });
      expect(provValidation.allowed).toBe(true);
    });

    it("proves full expedition cycle can loop back to provisioning", () => {
      // Start from provisioning, go through expedition, come back via result
      expect(canTransition(provisioningSnapshot, { type: "confirm-provisioning" }).allowed).toBe(true);
      expect(canTransition(expeditionSnapshot, { type: "launch-expedition" }).allowed).toBe(true);
      expect(canTransition(resultSnapshot, { type: "continue-from-result" }).allowed).toBe(true);

      // And we should be able to start provisioning again
      const provValidation = canTransition(replayReadySnapshot, { type: "start-provisioning" });
      expect(provValidation.allowed).toBe(true);
    });

    it("proves meta-loop can continue from failure result without dead-end states", () => {
      // From failure result screen, continue-from-result should be allowed
      const continueValidation = canTransition(failureResultSnapshot, { type: "continue-from-result" });
      expect(continueValidation.allowed).toBe(true);

      // After continuing, we should be in town where start-provisioning is allowed
      const provValidation = canTransition(replayReadySnapshot, { type: "start-provisioning" });
      expect(provValidation.allowed).toBe(true);
    });

    it("proves meta-loop can continue from partial result without dead-end states", () => {
      // From partial result screen, continue-from-result should be allowed
      const continueValidation = canTransition(partialResultSnapshot, { type: "continue-from-result" });
      expect(continueValidation.allowed).toBe(true);

      // After continuing, we should be in town where start-provisioning is allowed
      const provValidation = canTransition(replayReadySnapshot, { type: "start-provisioning" });
      expect(provValidation.allowed).toBe(true);
    });
  });

  describe("dungeon-hint transitions", () => {
    it("allows accept-dungeon-hint when dungeon is enterable", () => {
      const validation = canTransition(dungeonHintSnapshot, { type: "accept-dungeon-hint" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects accept-dungeon-hint when not in dungeon-hint screen", () => {
      const validation = canTransition(replayReadySnapshot, { type: "accept-dungeon-hint" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid in dungeon-hint");
    });

    it("rejects accept-dungeon-hint when dungeon is not enterable", () => {
      const unenterableDungeonHintSnapshot: DdgcFrontendSnapshot = {
        ...dungeonHintSnapshot,
        viewModel: {
          ...dungeonHintSnapshot.viewModel,
          isEnterable: false,
        } as DungeonHintViewModel,
      };
      const validation = canTransition(unenterableDungeonHintSnapshot, { type: "accept-dungeon-hint" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("not enterable");
    });
  });

  describe("dungeon-assist flow transitions", () => {
    it("allows enter-dungeon-assist from expedition", () => {
      const validation = canTransition(expeditionSnapshot, { type: "enter-dungeon-assist" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects enter-dungeon-assist when not in expedition", () => {
      const validation = canTransition(provisioningSnapshot, { type: "enter-dungeon-assist" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid in expedition");
    });

    it("allows select-assist-hero in dungeon-assist", () => {
      const validation = canTransition(dungeonAssistSnapshot, { type: "select-assist-hero", heroId: "hero-hunter-01" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects select-assist-hero when not in dungeon-assist", () => {
      const validation = canTransition(expeditionSnapshot, { type: "select-assist-hero", heroId: "hero-hunter-01" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid in dungeon-assist");
    });

    it("allows use-assist-action in dungeon-assist", () => {
      const validation = canTransition(dungeonAssistSnapshot, { type: "use-assist-action", actionId: "heal-wound" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects select-assist-hero for unknown heroes", () => {
      const validation = canTransition(dungeonAssistSnapshot, { type: "select-assist-hero", heroId: "unknown-hero" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("does not exist");
    });

    it("rejects use-assist-action for locked assist actions", () => {
      const validation = canTransition(dungeonAssistSnapshot, { type: "use-assist-action", actionId: "apply-buff" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("not available");
    });

    it("rejects use-assist-action for unknown assist actions", () => {
      const validation = canTransition(dungeonAssistSnapshot, { type: "use-assist-action", actionId: "unknown-action" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("does not exist");
    });

    it("rejects use-assist-action when not in dungeon-assist", () => {
      const validation = canTransition(expeditionSnapshot, { type: "use-assist-action", actionId: "heal-wound" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid in dungeon-assist");
    });

    it("rejects continue-from-dungeon when canContinue is false", () => {
      const validation = canTransition(dungeonAssistSnapshot, { type: "continue-from-dungeon" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("cannot continue");
    });

    it("allows continue-from-dungeon when canContinue is true", () => {
      const readySnapshot: DdgcFrontendSnapshot = {
        ...dungeonAssistSnapshot,
        viewModel: {
          ...dungeonAssistSnapshot.viewModel,
          canContinue: true
        } as DdgcFrontendSnapshot["viewModel"]
      };
      const validation = canTransition(readySnapshot, { type: "continue-from-dungeon" });
      expect(validation.allowed).toBe(true);
    });

    it("allows return-to-town from dungeon-assist", () => {
      const validation = canTransition(dungeonAssistSnapshot, { type: "return-to-town" });
      expect(validation.allowed).toBe(true);
    });
  });

  describe("provisioning flow transitions", () => {
    it("allows start-provisioning from town", () => {
      const validation = canTransition(replayReadySnapshot, { type: "start-provisioning" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects start-provisioning when not in town", () => {
      const validation = canTransition(provisioningSnapshot, { type: "start-provisioning" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid in town");
    });

    it("allows confirm-provisioning when ready to launch", () => {
      const validation = canTransition(provisioningSnapshot, { type: "confirm-provisioning" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects confirm-provisioning when not in provisioning screen", () => {
      const validation = canTransition(replayReadySnapshot, { type: "confirm-provisioning" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid in provisioning");
    });

    it("allows launch-expedition when expedition is launchable", () => {
      const validation = canTransition(expeditionSnapshot, { type: "launch-expedition" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects launch-expedition when not in expedition screen", () => {
      const validation = canTransition(replayReadySnapshot, { type: "launch-expedition" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid in expedition");
    });

    it("allows toggle-hero-selection in provisioning", () => {
      const validation = canTransition(provisioningSnapshot, { type: "toggle-hero-selection", heroId: "hero-hunter-01" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects toggle-hero-selection when not in provisioning", () => {
      const validation = canTransition(replayReadySnapshot, { type: "toggle-hero-selection", heroId: "hero-hunter-01" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid in provisioning");
    });
  });

  describe("town screen transitions", () => {
    it("allows open-hero in town", () => {
      const validation = canTransition(replayReadySnapshot, { type: "open-hero", heroId: "hero-hunter-01" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects open-hero when not in town", () => {
      const validation = canTransition(provisioningSnapshot, { type: "open-hero", heroId: "hero-hunter-01" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid in town");
    });

    it("allows open-building in town", () => {
      const validation = canTransition(replayReadySnapshot, { type: "open-building", buildingId: "guild" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects open-building when not in town", () => {
      const validation = canTransition(expeditionSnapshot, { type: "open-building", buildingId: "guild" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid in town");
    });
  });

  describe("building-detail transitions", () => {
    it("allows building-action in building-detail", () => {
      const validation = canTransition(replayBuildingDetailSnapshot, { type: "building-action", actionId: "train-combat" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects building-action when not in building-detail", () => {
      const validation = canTransition(replayReadySnapshot, { type: "building-action", actionId: "train-combat" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid in building-detail");
    });
  });

  describe("dungeon-map transitions", () => {
    it("allows enter-room when room exists and is connected", () => {
      const validation = canTransition(dungeonMapSnapshot, { type: "enter-room", roomId: "room-combat-1" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects enter-room when not on dungeon-map screen", () => {
      const validation = canTransition(expeditionSnapshot, { type: "enter-room", roomId: "room-combat-1" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid on dungeon-map screen");
    });

    it("rejects enter-room when not on dungeon-map screen (result)", () => {
      const validation = canTransition(resultSnapshot, { type: "enter-room", roomId: "room-combat-1" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid on dungeon-map screen");
    });

    it("rejects enter-room when roomId is missing", () => {
      const validation = canTransition(dungeonMapSnapshot, { type: "enter-room", roomId: "" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("roomId is required");
    });

    it("rejects enter-room when room does not exist", () => {
      const validation = canTransition(dungeonMapSnapshot, { type: "enter-room", roomId: "nonexistent-room" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("does not exist");
    });

    it("rejects enter-room when room is not connected to current room", () => {
      // room-boss-1 is not connected to room-entrance (the current room in the fixture)
      const validation = canTransition(dungeonMapSnapshot, { type: "enter-room", roomId: "room-boss-1" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("not connected");
    });

    it("rejects enter-room when room is hidden (not revealed)", () => {
      // room-boss-1 is hidden in the fixture; make it connected to current room for this test
      const mapVm = dungeonMapSnapshot.viewModel as DungeonMapViewModel;
      const hiddenRoomSnapshot: DdgcFrontendSnapshot = {
        ...dungeonMapSnapshot,
        viewModel: {
          ...mapVm,
          rooms: mapVm.rooms.map((r) =>
            r.id === "room-entrance"
              ? { ...r, connections: [...r.connections, "room-boss-1"] }
              : r
          )
        }
      };
      const validation = canTransition(hiddenRoomSnapshot, { type: "enter-room", roomId: "room-boss-1" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("not revealed");
    });

    it("allows retreat-from-dungeon when retreat is available", () => {
      const validation = canTransition(dungeonMapSnapshot, { type: "retreat-from-dungeon" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects retreat-from-dungeon when not on dungeon-map screen", () => {
      const validation = canTransition(expeditionSnapshot, { type: "retreat-from-dungeon" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid on dungeon-map screen");
    });

    it("rejects retreat-from-dungeon when retreat is not available", () => {
      const mapVm = dungeonMapSnapshot.viewModel as DungeonMapViewModel;
      const noRetreatSnapshot: DdgcFrontendSnapshot = {
        ...dungeonMapSnapshot,
        viewModel: {
          ...mapVm,
          isRetreatAvailable: false
        }
      };
      const validation = canTransition(noRetreatSnapshot, { type: "retreat-from-dungeon" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("retreat is not available");
    });

    it("rejects complete-dungeon when dungeon is not complete", () => {
      const validation = canTransition(dungeonMapSnapshot, { type: "complete-dungeon" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("not complete");
    });

    it("allows complete-dungeon when dungeon is complete", () => {
      const mapVm = dungeonMapSnapshot.viewModel as DungeonMapViewModel;
      const completeSnapshot: DdgcFrontendSnapshot = {
        ...dungeonMapSnapshot,
        viewModel: {
          ...mapVm,
          isComplete: true
        }
      };
      const validation = canTransition(completeSnapshot, { type: "complete-dungeon" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects complete-dungeon when not on dungeon-map screen", () => {
      const validation = canTransition(resultSnapshot, { type: "complete-dungeon" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid on dungeon-map screen");
    });
  });

  describe("dungeon-interaction transitions", () => {
    it("allows interact-room when interaction exists and is available", () => {
      const validation = canTransition(dungeonInteractionSnapshot, { type: "interact-room", interactionId: "investigate" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects interact-room when not on dungeon-interaction screen", () => {
      const validation = canTransition(replayReadySnapshot, { type: "interact-room", interactionId: "investigate" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid in dungeon-interaction");
    });

    it("rejects interact-room for nonexistent interaction", () => {
      const validation = canTransition(dungeonInteractionSnapshot, { type: "interact-room", interactionId: "unknown-interaction" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("does not exist");
    });

    it("rejects interact-room for unavailable interaction", () => {
      const dungeonVm = dungeonInteractionSnapshot.viewModel as DungeonInteractionViewModel;
      const lockedInteractionSnapshot: DdgcFrontendSnapshot = {
        ...dungeonInteractionSnapshot,
        viewModel: {
          ...dungeonVm,
          interactions: dungeonVm.interactions.map((i) =>
            i.id === "investigate" ? { ...i, isAvailable: false } : i
          )
        }
      };
      const validation = canTransition(lockedInteractionSnapshot, { type: "interact-room", interactionId: "investigate" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("not available");
    });

    it("allows proceed-dungeon when proceed is available", () => {
      const validation = canTransition(dungeonInteractionSnapshot, { type: "proceed-dungeon" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects proceed-dungeon when not on dungeon-interaction screen", () => {
      const validation = canTransition(replayReadySnapshot, { type: "proceed-dungeon" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid in dungeon-interaction");
    });

    it("rejects proceed-dungeon when proceed is not available", () => {
      const dungeonVm = dungeonInteractionSnapshot.viewModel as DungeonInteractionViewModel;
      const noProceedSnapshot: DdgcFrontendSnapshot = {
        ...dungeonInteractionSnapshot,
        viewModel: {
          ...dungeonVm,
          isProceedAvailable: false
        }
      };
      const validation = canTransition(noProceedSnapshot, { type: "proceed-dungeon" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("proceed is not available");
    });

    it("allows retreat-dungeon when retreat is available", () => {
      const validation = canTransition(dungeonInteractionSnapshot, { type: "retreat-dungeon" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects retreat-dungeon when not on dungeon-interaction screen", () => {
      const validation = canTransition(replayReadySnapshot, { type: "retreat-dungeon" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("only valid in dungeon-interaction");
    });

    it("rejects retreat-dungeon when retreat is not available", () => {
      const dungeonVm = dungeonInteractionSnapshot.viewModel as DungeonInteractionViewModel;
      const noRetreatSnapshot: DdgcFrontendSnapshot = {
        ...dungeonInteractionSnapshot,
        viewModel: {
          ...dungeonVm,
          isRetreatAvailable: false
        }
      };
      const validation = canTransition(noRetreatSnapshot, { type: "retreat-dungeon" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("retreat is not available");
    });
  });

  describe("combat transitions", () => {
    it("allows valid player combat selection and attack intents", () => {
      expect(canTransition(combatSnapshot, { type: "select-skill", skillId: "skill-1" }).allowed).toBe(true);
      expect(canTransition(combatSnapshot, { type: "select-target", enemyId: "enemy-moth-01" }).allowed).toBe(true);
      expect(canTransition(combatSnapshot, { type: "confirm-attack" }).allowed).toBe(true);
    });

    it("rejects missing and cooldown skills for the active hero", () => {
      const missingSkill = canTransition(combatSnapshot, { type: "select-skill", skillId: "missing-skill" });
      expect(missingSkill.allowed).toBe(false);
      expect(missingSkill.reason).toContain("does not exist");

      const cooldownSkill = canTransition(combatSnapshot, { type: "select-skill", skillId: "skill-3" });
      expect(cooldownSkill.allowed).toBe(false);
      expect(cooldownSkill.reason).toContain("cooldown");
    });

    it("rejects missing and defeated combat targets", () => {
      const missingTarget = canTransition(combatSnapshot, { type: "select-target", enemyId: "missing-enemy" });
      expect(missingTarget.allowed).toBe(false);
      expect(missingTarget.reason).toContain("does not exist");

      const combatVm = combatSnapshot.viewModel as CombatViewModel;
      const deadTargetSnapshot: DdgcFrontendSnapshot = {
        ...combatSnapshot,
        viewModel: {
          ...combatVm,
          enemies: combatVm.enemies.map((enemy) =>
            enemy.id === "enemy-larva-01"
              ? { ...enemy, isAlive: false }
              : enemy
          )
        }
      };
      const deadTarget = canTransition(deadTargetSnapshot, { type: "select-target", enemyId: "enemy-larva-01" });
      expect(deadTarget.allowed).toBe(false);
      expect(deadTarget.reason).toContain("defeated");
    });

    it("rejects confirm-attack with no live selected target", () => {
      const combatVm = combatSnapshot.viewModel as CombatViewModel;
      const noTargetSnapshot: DdgcFrontendSnapshot = {
        ...combatSnapshot,
        viewModel: {
          ...combatVm,
          enemies: combatVm.enemies.map((enemy) => ({ ...enemy, isTargeted: false }))
        }
      };
      const noTarget = canTransition(noTargetSnapshot, { type: "confirm-attack" });
      expect(noTarget.allowed).toBe(false);
      expect(noTarget.reason).toContain("no live combat target");

      const deadSelectedTargetSnapshot: DdgcFrontendSnapshot = {
        ...combatSnapshot,
        viewModel: {
          ...combatVm,
          enemies: combatVm.enemies.map((enemy) =>
            enemy.isTargeted
              ? { ...enemy, isAlive: false }
              : enemy
          )
        }
      };
      const deadSelectedTarget = canTransition(deadSelectedTargetSnapshot, { type: "confirm-attack" });
      expect(deadSelectedTarget.allowed).toBe(false);
      expect(deadSelectedTarget.reason).toContain("defeated");
    });

    it("rejects combat-ending intents during character-hit acknowledgement", () => {
      const characterHitSnapshot: DdgcFrontendSnapshot = {
        ...combatSnapshot,
        viewModel: replayCombatViewModel
      };

      expect(canTransition(characterHitSnapshot, { type: "continue-from-combat" }).allowed).toBe(true);

      const selectSkill = canTransition(characterHitSnapshot, { type: "select-skill", skillId: "skill-1" });
      expect(selectSkill.allowed).toBe(false);
      expect(selectSkill.reason).toContain("character-hit acknowledgement");

      const selectTarget = canTransition(characterHitSnapshot, { type: "select-target", enemyId: "enemy-moth-01" });
      expect(selectTarget.allowed).toBe(false);
      expect(selectTarget.reason).toContain("character-hit acknowledgement");

      const confirmAttack = canTransition(characterHitSnapshot, { type: "confirm-attack" });
      expect(confirmAttack.allowed).toBe(false);
      expect(confirmAttack.reason).toContain("character-hit acknowledgement");

      const flee = canTransition(characterHitSnapshot, { type: "flee-combat" });
      expect(flee.allowed).toBe(false);
      expect(flee.reason).toContain("character-hit acknowledgement");

      const endTurn = canTransition(characterHitSnapshot, { type: "end-turn" });
      expect(endTurn.allowed).toBe(false);
      expect(endTurn.reason).toContain("character-hit acknowledgement");

      const openSettings = canTransition(characterHitSnapshot, { type: "open-combat-settings" });
      expect(openSettings.allowed).toBe(false);
      expect(openSettings.reason).toContain("character-hit acknowledgement");

      const returnToTown = canTransition(characterHitSnapshot, { type: "return-to-town" });
      expect(returnToTown.allowed).toBe(false);
      expect(returnToTown.reason).toContain("character-hit acknowledgement");
    });

    it("rejects continue-from-combat outside character-hit acknowledgement", () => {
      expect(canTransition(combatSnapshot, { type: "continue-from-combat" }).allowed).toBe(false);
      expect(canTransition(combatSnapshot, { type: "continue-from-combat" }).reason).toContain("character-hit acknowledgement");
    });
  });

  describe("secondary interaction stability after result/return handoff", () => {
    it("open-hero is allowed after continue-from-result handoff to town", () => {
      // Verify from a fresh town state (simulating after continue-from-result)
      expect(canTransition(replayReadySnapshot, { type: "open-hero", heroId: "hero-hunter-01" }).allowed).toBe(true);
      expect(canTransition(replayReadySnapshot, { type: "open-building", buildingId: "guild" }).allowed).toBe(true);
    });

    it("open-hero is allowed after resume-from-return handoff to town", () => {
      // Same town state is used after both continue-from-result and resume-from-return
      expect(canTransition(replayReadySnapshot, { type: "open-hero", heroId: "hero-hunter-01" }).allowed).toBe(true);
      expect(canTransition(replayReadySnapshot, { type: "open-building", buildingId: "stagecoach" }).allowed).toBe(true);
    });

    it("expedition launch sequence is accessible after all terminal flow states", () => {
      // After any terminal flow state, town → provisioning → expedition → launch should work
      const terminalFlows = [failureResultSnapshot, partialResultSnapshot, returnSnapshot];
      for (const snap of terminalFlows) {
        // From terminal state, can reach town via return-to-town
        expect(canTransition(snap, { type: "return-to-town" }).allowed).toBe(true);

        // Town can start provisioning
        expect(canTransition(replayReadySnapshot, { type: "start-provisioning" }).allowed).toBe(true);
        // Provisioning can confirm
        expect(canTransition(provisioningSnapshot, { type: "confirm-provisioning" }).allowed).toBe(true);
        // Expedition can launch
        expect(canTransition(expeditionSnapshot, { type: "launch-expedition" }).allowed).toBe(true);
      }
    });

    it("hero detail and building detail are accessible after result state", () => {
      // Verify all town screens are accessible when in the "after result" town state
      expect(resolveScreen(replayReadySnapshot)).toBe("town");
      expect(canTransition(replayReadySnapshot, { type: "open-hero", heroId: "hero-hunter-01" }).allowed).toBe(true);
      expect(canTransition(replayReadySnapshot, { type: "open-building", buildingId: "guild" }).allowed).toBe(true);

      // Hero detail and building detail screens resolve correctly
      expect(resolveScreen(replayHeroDetailSnapshot)).toBe("hero-detail");
      expect(resolveScreen(replayBuildingDetailSnapshot)).toBe("building-detail");
    });
  });

  describe("dungeon-select availability guard", () => {
    it("allows select-dungeon with an available dungeon", () => {
      const validation = canTransition(dungeonSelectSnapshot, { type: "select-dungeon", dungeonId: "dungeon-ruins-01" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects select-dungeon with a locked dungeon", () => {
      const validation = canTransition(dungeonSelectSnapshot, { type: "select-dungeon", dungeonId: "dungeon-depths-01" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("not available");
    });

    it("rejects select-dungeon with an unknown dungeon id", () => {
      const validation = canTransition(dungeonSelectSnapshot, { type: "select-dungeon", dungeonId: "dungeon-unknown-99" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("unknown");
    });

    it("allows confirm-dungeon-selection when dungeon is available and ready", () => {
      const readySnapshot: DdgcFrontendSnapshot = {
        ...dungeonSelectSnapshot,
        viewModel: {
          ...dungeonSelectSnapshot.viewModel,
          selectedDungeonId: "dungeon-ruins-01",
          isReadyToProceed: true
        } as DungeonSelectViewModel
      };
      const validation = canTransition(readySnapshot, { type: "confirm-dungeon-selection" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects confirm-dungeon-selection when selected dungeon is locked", () => {
      const lockedSnapshot: DdgcFrontendSnapshot = {
        ...dungeonSelectSnapshot,
        viewModel: {
          ...dungeonSelectSnapshot.viewModel,
          selectedDungeonId: "dungeon-depths-01",
          isReadyToProceed: true
        } as DungeonSelectViewModel
      };
      const validation = canTransition(lockedSnapshot, { type: "confirm-dungeon-selection" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("not available");
    });

    it("rejects confirm-dungeon-selection when selected dungeon is unknown", () => {
      const unknownSnapshot: DdgcFrontendSnapshot = {
        ...dungeonSelectSnapshot,
        viewModel: {
          ...dungeonSelectSnapshot.viewModel,
          selectedDungeonId: "dungeon-unknown-99",
          isReadyToProceed: true
        } as DungeonSelectViewModel
      };
      const validation = canTransition(unknownSnapshot, { type: "confirm-dungeon-selection" });
      expect(validation.allowed).toBe(false);
      expect(validation.reason).toContain("does not exist");
    });
  });
});
