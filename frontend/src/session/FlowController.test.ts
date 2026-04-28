import { describe, expect, it } from "vitest";

import { resolveScreen, canTransition, type ScreenKey } from "./FlowController";
import type {
  DdgcFrontendSnapshot,
  ExpeditionResultViewModel,
  ReturnViewModel,
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
  provisioningSnapshot,
  expeditionSnapshot,
  resultSnapshot,
  failureResultSnapshot,
  partialResultSnapshot,
  returnSnapshot,
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

    it("returns expedition screen for expedition setup view model", () => {
      const screen = resolveScreen(expeditionSnapshot);
      expect(screen).toBe("expedition");
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
  const allScreenKeys: ScreenKey[] = ["startup", "loading", "town", "hero-detail", "building-detail", "provisioning", "expedition", "result", "return", "unsupported", "fatal"];

  it("covers all screen keys in FlowController.resolveScreen", () => {
    const snapshotsByScreen: Record<ScreenKey, DdgcFrontendSnapshot> = {
      startup: startupSnapshot,
      loading: replayLoadingSnapshot,
      town: replayReadySnapshot,
      "hero-detail": replayHeroDetailSnapshot,
      "building-detail": replayBuildingDetailSnapshot,
      provisioning: provisioningSnapshot,
      expedition: expeditionSnapshot,
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
});