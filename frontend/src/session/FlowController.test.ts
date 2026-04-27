import { describe, expect, it } from "vitest";

import { resolveScreen, type ScreenKey } from "./FlowController";
import type { DdgcFrontendSnapshot } from "../bridge/contractTypes";
import {
  fatalSnapshot,
  unsupportedSnapshot,
  replayLoadingSnapshot,
  liveLoadingSnapshot,
  replayReadySnapshot,
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
  });
});

describe("ScreenKey exhaustiveness", () => {
  const allScreenKeys: ScreenKey[] = ["startup", "loading", "town", "unsupported", "fatal"];

  it("covers all screen keys in FlowController.resolveScreen", () => {
    const snapshotsByScreen: Record<ScreenKey, DdgcFrontendSnapshot> = {
      startup: { lifecycle: "ready", flowState: "boot", viewModel: replayReadySnapshot.viewModel },
      loading: replayLoadingSnapshot,
      town: replayReadySnapshot,
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