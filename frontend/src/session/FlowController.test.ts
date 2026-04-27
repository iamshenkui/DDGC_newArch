import { describe, expect, it } from "vitest";

import { resolveScreen, canTransition, type ScreenKey } from "./FlowController";
import type { DdgcFrontendSnapshot } from "../bridge/contractTypes";
import {
  fatalSnapshot,
  unsupportedSnapshot,
  replayLoadingSnapshot,
  liveLoadingSnapshot,
  replayReadySnapshot,
  replayHeroDetailSnapshot,
  replayBuildingDetailSnapshot,
  replayProvisioningViewModel,
  replayExpeditionViewModel,
  replayResultViewModel,
  replayFailureResultViewModel,
  replayPartialResultViewModel,
  replayReturnViewModel,
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

    it("returns startup screen for unhandled lifecycle combinations", () => {
      const unknownLifecycle: DdgcFrontendSnapshot = {
        lifecycle: "ready",
        flowState: "boot",
        viewModel: replayReadySnapshot.viewModel,
      };
      const screen = resolveScreen(unknownLifecycle);
      expect(screen).toBe("startup");
    });

    it("returns building-detail screen for building detail view model", () => {
      const screen = resolveScreen(replayBuildingDetailSnapshot);
      expect(screen).toBe("building-detail");
    });

    it("returns provisioning screen for provisioning view model", () => {
      const provisioningSnapshot: DdgcFrontendSnapshot = {
        lifecycle: "ready",
        flowState: "provisioning",
        viewModel: replayProvisioningViewModel,
        debugMessage: "Replay bridge showing provisioning screen."
      };
      const screen = resolveScreen(provisioningSnapshot);
      expect(screen).toBe("provisioning");
    });

    it("returns expedition screen for expedition setup view model", () => {
      const expeditionSnapshot: DdgcFrontendSnapshot = {
        lifecycle: "ready",
        flowState: "expedition",
        viewModel: replayExpeditionViewModel,
        debugMessage: "Replay bridge showing expedition launch screen."
      };
      const screen = resolveScreen(expeditionSnapshot);
      expect(screen).toBe("expedition");
    });

    it("returns result screen for result view model", () => {
      const resultSnapshot: DdgcFrontendSnapshot = {
        lifecycle: "ready",
        flowState: "result",
        viewModel: replayResultViewModel,
        debugMessage: "Replay bridge showing result screen."
      };
      const screen = resolveScreen(resultSnapshot);
      expect(screen).toBe("result");
    });

    it("returns result screen for failure outcome result view model", () => {
      const failureSnapshot: DdgcFrontendSnapshot = {
        lifecycle: "ready",
        flowState: "result",
        viewModel: replayFailureResultViewModel,
        debugMessage: "Replay bridge showing failure result screen."
      };
      const screen = resolveScreen(failureSnapshot);
      expect(screen).toBe("result");
    });

    it("returns result screen for partial outcome result view model", () => {
      const partialSnapshot: DdgcFrontendSnapshot = {
        lifecycle: "ready",
        flowState: "result",
        viewModel: replayPartialResultViewModel,
        debugMessage: "Replay bridge showing partial result screen."
      };
      const screen = resolveScreen(partialSnapshot);
      expect(screen).toBe("result");
    });

    it("returns return screen for return view model", () => {
      const returnSnapshot: DdgcFrontendSnapshot = {
        lifecycle: "ready",
        flowState: "return",
        viewModel: replayReturnViewModel,
        debugMessage: "Replay bridge showing return screen."
      };
      const screen = resolveScreen(returnSnapshot);
      expect(screen).toBe("return");
    });
  });
});

describe("ScreenKey exhaustiveness", () => {
  const allScreenKeys: ScreenKey[] = ["startup", "loading", "town", "hero-detail", "building-detail", "provisioning", "expedition", "result", "return", "unsupported", "fatal"];

  const replayProvisioningSnapshot: DdgcFrontendSnapshot = {
    lifecycle: "ready",
    flowState: "town",
    viewModel: replayProvisioningViewModel,
    debugMessage: "Replay bridge showing provisioning screen."
  };

  const replayExpeditionSnapshot: DdgcFrontendSnapshot = {
    lifecycle: "ready",
    flowState: "expedition",
    viewModel: replayExpeditionViewModel,
    debugMessage: "Replay bridge showing expedition launch screen."
  };

  const replayResultSnapshot: DdgcFrontendSnapshot = {
    lifecycle: "ready",
    flowState: "result",
    viewModel: replayResultViewModel,
    debugMessage: "Replay bridge showing expedition result screen."
  };

  const replayReturnSnapshot: DdgcFrontendSnapshot = {
    lifecycle: "ready",
    flowState: "return",
    viewModel: replayReturnViewModel,
    debugMessage: "Replay bridge showing return screen."
  };

  it("covers all screen keys in FlowController.resolveScreen", () => {
    const snapshotsByScreen: Record<ScreenKey, DdgcFrontendSnapshot> = {
      startup: { lifecycle: "ready", flowState: "boot", viewModel: replayReadySnapshot.viewModel },
      loading: replayLoadingSnapshot,
      town: replayReadySnapshot,
      "hero-detail": replayHeroDetailSnapshot,
      "building-detail": replayBuildingDetailSnapshot,
      provisioning: replayProvisioningSnapshot,
      expedition: replayExpeditionSnapshot,
      result: replayResultSnapshot,
      return: replayReturnSnapshot,
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
  const resultSnapshot: DdgcFrontendSnapshot = {
    lifecycle: "ready",
    flowState: "result",
    viewModel: replayResultViewModel,
    debugMessage: "Replay bridge showing result screen."
  };

  const returnSnapshot: DdgcFrontendSnapshot = {
    lifecycle: "ready",
    flowState: "return",
    viewModel: replayReturnViewModel,
    debugMessage: "Replay bridge showing return screen."
  };

  describe("continue-from-result transitions", () => {
    it("allows continue-from-result when isContinueAvailable is true", () => {
      const validation = canTransition(resultSnapshot, { type: "continue-from-result" });
      expect(validation.allowed).toBe(true);
    });

    it("allows continue-from-result when isContinueAvailable is explicitly true", () => {
      const availableResultSnapshot: DdgcFrontendSnapshot = {
        ...resultSnapshot,
        viewModel: { ...replayResultViewModel, isContinueAvailable: true }
      };
      const validation = canTransition(availableResultSnapshot, { type: "continue-from-result" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects continue-from-result when isContinueAvailable is false", () => {
      const unavailableResultSnapshot: DdgcFrontendSnapshot = {
        ...resultSnapshot,
        viewModel: { ...replayResultViewModel, isContinueAvailable: false }
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
        viewModel: { ...replayReturnViewModel, isTownResumeAvailable: true }
      };
      const validation = canTransition(availableReturnSnapshot, { type: "resume-from-return" });
      expect(validation.allowed).toBe(true);
    });

    it("rejects resume-from-return when isTownResumeAvailable is false", () => {
      const unavailableReturnSnapshot: DdgcFrontendSnapshot = {
        ...returnSnapshot,
        viewModel: { ...replayReturnViewModel, isTownResumeAvailable: false }
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
      const provisioningSnapshot: DdgcFrontendSnapshot = {
        lifecycle: "ready",
        flowState: "provisioning",
        viewModel: replayProvisioningViewModel,
      };
      const validation = canTransition(provisioningSnapshot, { type: "return-to-town" });
      expect(validation.allowed).toBe(true);
    });

    it("allows return-to-town from expedition", () => {
      const expeditionSnapshot: DdgcFrontendSnapshot = {
        lifecycle: "ready",
        flowState: "expedition",
        viewModel: replayExpeditionViewModel,
      };
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
      const provisioningSnapshot: DdgcFrontendSnapshot = {
        lifecycle: "ready",
        flowState: "town",
        viewModel: replayReadySnapshot.viewModel,
      };
      const provValidation = canTransition(provisioningSnapshot, { type: "start-provisioning" });
      expect(provValidation.allowed).toBe(true);
    });

    it("proves meta-loop can continue from return without dead-end states", () => {
      // From return screen, resume-from-return should be allowed
      const resumeValidation = canTransition(returnSnapshot, { type: "resume-from-return" });
      expect(resumeValidation.allowed).toBe(true);

      // After resuming, we should be in town where start-provisioning is allowed
      const provisioningSnapshot: DdgcFrontendSnapshot = {
        lifecycle: "ready",
        flowState: "town",
        viewModel: replayReadySnapshot.viewModel,
      };
      const provValidation = canTransition(provisioningSnapshot, { type: "start-provisioning" });
      expect(provValidation.allowed).toBe(true);
    });

    it("proves full expedition cycle can loop back to provisioning", () => {
      const provisioningSnapshot: DdgcFrontendSnapshot = {
        lifecycle: "ready",
        flowState: "provisioning",
        viewModel: replayProvisioningViewModel,
      };
      const expeditionSnapshot: DdgcFrontendSnapshot = {
        lifecycle: "ready",
        flowState: "expedition",
        viewModel: replayExpeditionViewModel,
      };

      // Start from provisioning, go through expedition, come back via result
      expect(canTransition(provisioningSnapshot, { type: "confirm-provisioning" }).allowed).toBe(true);
      expect(canTransition(expeditionSnapshot, { type: "launch-expedition" }).allowed).toBe(true);
      expect(canTransition(resultSnapshot, { type: "continue-from-result" }).allowed).toBe(true);

      // And we should be able to start provisioning again
      const townSnapshot: DdgcFrontendSnapshot = {
        lifecycle: "ready",
        flowState: "town",
        viewModel: replayReadySnapshot.viewModel,
      };
      expect(canTransition(townSnapshot, { type: "start-provisioning" }).allowed).toBe(true);
    });

    it("proves meta-loop can continue from failure result without dead-end states", () => {
      const failureResultSnapshot: DdgcFrontendSnapshot = {
        lifecycle: "ready",
        flowState: "result",
        viewModel: replayFailureResultViewModel,
        debugMessage: "Replay bridge showing failure result screen."
      };

      // From failure result screen, continue-from-result should be allowed
      const continueValidation = canTransition(failureResultSnapshot, { type: "continue-from-result" });
      expect(continueValidation.allowed).toBe(true);

      // After continuing, we should be in town where start-provisioning is allowed
      const townSnapshot: DdgcFrontendSnapshot = {
        lifecycle: "ready",
        flowState: "town",
        viewModel: replayReadySnapshot.viewModel,
      };
      const provValidation = canTransition(townSnapshot, { type: "start-provisioning" });
      expect(provValidation.allowed).toBe(true);
    });

    it("proves meta-loop can continue from partial result without dead-end states", () => {
      const partialResultSnapshot: DdgcFrontendSnapshot = {
        lifecycle: "ready",
        flowState: "result",
        viewModel: replayPartialResultViewModel,
        debugMessage: "Replay bridge showing partial result screen."
      };

      // From partial result screen, continue-from-result should be allowed
      const continueValidation = canTransition(partialResultSnapshot, { type: "continue-from-result" });
      expect(continueValidation.allowed).toBe(true);

      // After continuing, we should be in town where start-provisioning is allowed
      const townSnapshot: DdgcFrontendSnapshot = {
        lifecycle: "ready",
        flowState: "town",
        viewModel: replayReadySnapshot.viewModel,
      };
      const provValidation = canTransition(townSnapshot, { type: "start-provisioning" });
      expect(provValidation.allowed).toBe(true);
    });
  });
});