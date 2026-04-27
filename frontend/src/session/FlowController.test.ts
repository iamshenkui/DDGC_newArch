import { describe, expect, it } from "vitest";

import { resolveScreen, type ScreenKey } from "./FlowController";
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
  });
});

describe("ScreenKey exhaustiveness", () => {
  const allScreenKeys: ScreenKey[] = ["startup", "loading", "town", "hero-detail", "building-detail", "provisioning", "expedition", "unsupported", "fatal"];

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

  it("covers all screen keys in FlowController.resolveScreen", () => {
    const snapshotsByScreen: Record<ScreenKey, DdgcFrontendSnapshot> = {
      startup: { lifecycle: "ready", flowState: "boot", viewModel: replayReadySnapshot.viewModel },
      loading: replayLoadingSnapshot,
      town: replayReadySnapshot,
      "hero-detail": replayHeroDetailSnapshot,
      "building-detail": replayBuildingDetailSnapshot,
      provisioning: replayProvisioningSnapshot,
      expedition: replayExpeditionSnapshot,
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