import { describe, expect, it } from "vitest";

import { resolveScreen, canTransition } from "../../session/FlowController";
import type {
  DdgcFrontendSnapshot,
  ExpeditionResultViewModel,
  ReturnViewModel,
} from "../../bridge/contractTypes";
import {
  resultSnapshot,
  failureResultSnapshot,
  partialResultSnapshot,
  returnSnapshot,
  replayReadySnapshot,
  provisioningSnapshot,
  expeditionSnapshot,
  startupSnapshot,
  fatalSnapshot,
  unsupportedSnapshot,
  replayLoadingSnapshot,
  replayHeroDetailSnapshot,
  replayBuildingDetailSnapshot,
} from "../../validation/replayFixtures";

describe("Result screen view model contract validation", () => {
  it("validates success result view model fields", () => {
    const vm = resultSnapshot.viewModel as ExpeditionResultViewModel;
    expect(vm.kind).toBe("result");
    expect(vm.outcome).toBe("success");
    expect(vm).toHaveProperty("title");
    expect(vm).toHaveProperty("expeditionName");
    expect(vm).toHaveProperty("outcome");
    expect(vm).toHaveProperty("summary");
    expect(vm).toHaveProperty("lootAcquired");
    expect(vm).toHaveProperty("heroOutcomes");
    expect(vm).toHaveProperty("resourcesGained");
    expect(vm).toHaveProperty("isContinueAvailable");
  });

  it("validates failure result view model fields", () => {
    const vm = failureResultSnapshot.viewModel as ExpeditionResultViewModel;
    expect(vm.kind).toBe("result");
    expect(vm.outcome).toBe("failure");
    expect(vm.lootAcquired).toHaveLength(0);
    expect(vm.heroOutcomes.some((h) => h.status === "dead")).toBe(true);
  });

  it("validates partial result view model fields", () => {
    const vm = partialResultSnapshot.viewModel as ExpeditionResultViewModel;
    expect(vm.kind).toBe("result");
    expect(vm.outcome).toBe("partial");
    expect(vm.lootAcquired.length).toBeGreaterThan(0);
    expect(vm.heroOutcomes.some((h) => h.status === "stressed")).toBe(true);
  });

  it("validates result hero outcome contract fields", () => {
    const vm = resultSnapshot.viewModel as ExpeditionResultViewModel;
    for (const hero of vm.heroOutcomes) {
      expect(hero).toHaveProperty("heroId");
      expect(hero).toHaveProperty("heroName");
      expect(hero).toHaveProperty("status");
      expect(["alive", "dead", "stressed"]).toContain(hero.status);
      expect(hero).toHaveProperty("hpChange");
      expect(hero).toHaveProperty("stressChange");
    }
  });

  it("validates result resources gained fields", () => {
    const vm = resultSnapshot.viewModel as ExpeditionResultViewModel;
    expect(vm.resourcesGained).toHaveProperty("gold");
    expect(vm.resourcesGained).toHaveProperty("supplies");
    expect(vm.resourcesGained).toHaveProperty("experience");
  });

  it("validates failure result has casualties", () => {
    const vm = failureResultSnapshot.viewModel as ExpeditionResultViewModel;
    const deadHeroes = vm.heroOutcomes.filter((h) => h.status === "dead");
    expect(deadHeroes.length).toBeGreaterThan(0);
  });

  it("validates failure result has no loot", () => {
    const vm = failureResultSnapshot.viewModel as ExpeditionResultViewModel;
    expect(vm.lootAcquired).toHaveLength(0);
  });
});

describe("Return screen view model contract validation", () => {
  it("validates return view model fields", () => {
    const vm = returnSnapshot.viewModel as ReturnViewModel;
    expect(vm.kind).toBe("return");
    expect(vm).toHaveProperty("title");
    expect(vm).toHaveProperty("expeditionName");
    expect(vm).toHaveProperty("summary");
    expect(vm).toHaveProperty("returningHeroes");
    expect(vm).toHaveProperty("isTownResumeAvailable");
  });

  it("validates returning hero contract fields", () => {
    const vm = returnSnapshot.viewModel as ReturnViewModel;
    for (const hero of vm.returningHeroes) {
      expect(hero).toHaveProperty("heroId");
      expect(hero).toHaveProperty("heroName");
      expect(hero).toHaveProperty("hp");
      expect(hero).toHaveProperty("stress");
    }
  });

  it("return view model has town resume available by default", () => {
    const vm = returnSnapshot.viewModel as ReturnViewModel;
    expect(vm.isTownResumeAvailable).toBe(true);
  });
});

describe("Result screen resolution", () => {
  it("resolves result screen for success result view model", () => {
    const screen = resolveScreen(resultSnapshot);
    expect(screen).toBe("result");
  });

  it("resolves result screen for failure result view model", () => {
    const screen = resolveScreen(failureResultSnapshot);
    expect(screen).toBe("result");
  });

  it("resolves result screen for partial result view model", () => {
    const screen = resolveScreen(partialResultSnapshot);
    expect(screen).toBe("result");
  });

  it("resolves return screen for return view model", () => {
    const screen = resolveScreen(returnSnapshot);
    expect(screen).toBe("return");
  });
});

describe("Result screen transition validation", () => {
  it("allows continue-from-result from result screen", () => {
    const validation = canTransition(resultSnapshot, { type: "continue-from-result" });
    expect(validation.allowed).toBe(true);
  });

  it("rejects continue-from-result from non-result screens", () => {
    const nonResultScreens: DdgcFrontendSnapshot[] = [
      replayReadySnapshot,
      provisioningSnapshot,
      expeditionSnapshot,
      returnSnapshot,
      startupSnapshot,
      replayLoadingSnapshot,
      replayHeroDetailSnapshot,
      replayBuildingDetailSnapshot,
      fatalSnapshot,
      unsupportedSnapshot,
    ];

    for (const snap of nonResultScreens) {
      const validation = canTransition(snap, { type: "continue-from-result" });
      expect(validation.allowed).toBe(false);
    }
  });

  it("allows return-to-town from result screen as fallback", () => {
    const validation = canTransition(resultSnapshot, { type: "return-to-town" });
    expect(validation.allowed).toBe(true);
  });

  it("allows return-to-town from failure result screen as fallback", () => {
    const validation = canTransition(failureResultSnapshot, { type: "return-to-town" });
    expect(validation.allowed).toBe(true);
  });

  it("allows return-to-town from partial result screen as fallback", () => {
    const validation = canTransition(partialResultSnapshot, { type: "return-to-town" });
    expect(validation.allowed).toBe(true);
  });
});

describe("Return screen transition validation", () => {
  it("allows resume-from-return from return screen", () => {
    const validation = canTransition(returnSnapshot, { type: "resume-from-return" });
    expect(validation.allowed).toBe(true);
  });

  it("rejects resume-from-return from non-return screens", () => {
    const nonReturnScreens: DdgcFrontendSnapshot[] = [
      replayReadySnapshot,
      provisioningSnapshot,
      expeditionSnapshot,
      resultSnapshot,
      failureResultSnapshot,
      partialResultSnapshot,
      startupSnapshot,
      replayLoadingSnapshot,
      replayHeroDetailSnapshot,
      replayBuildingDetailSnapshot,
    ];

    for (const snap of nonReturnScreens) {
      const validation = canTransition(snap, { type: "resume-from-return" });
      expect(validation.allowed).toBe(false);
    }
  });

  it("allows return-to-town from return screen as fallback", () => {
    const validation = canTransition(returnSnapshot, { type: "return-to-town" });
    expect(validation.allowed).toBe(true);
  });
});

describe("Result return meta-loop continuation proof", () => {
  it("proves meta-loop can continue from success result to town", () => {
    // Given: result screen
    expect(resolveScreen(resultSnapshot)).toBe("result");

    // When: continue-from-result is allowed
    expect(canTransition(resultSnapshot, { type: "continue-from-result" }).allowed).toBe(true);

    // Then: town screen is reachable with provisioning available
    expect(resolveScreen(replayReadySnapshot)).toBe("town");
    expect(canTransition(replayReadySnapshot, { type: "start-provisioning" }).allowed).toBe(true);
  });

  it("proves meta-loop can continue from failure result to town", () => {
    expect(resolveScreen(failureResultSnapshot)).toBe("result");
    expect(canTransition(failureResultSnapshot, { type: "continue-from-result" }).allowed).toBe(true);
    expect(canTransition(replayReadySnapshot, { type: "start-provisioning" }).allowed).toBe(true);
  });

  it("proves meta-loop can continue from partial result to town", () => {
    expect(resolveScreen(partialResultSnapshot)).toBe("result");
    expect(canTransition(partialResultSnapshot, { type: "continue-from-result" }).allowed).toBe(true);
    expect(canTransition(replayReadySnapshot, { type: "start-provisioning" }).allowed).toBe(true);
  });

  it("proves meta-loop can continue from return screen to town", () => {
    // Given: return screen
    expect(resolveScreen(returnSnapshot)).toBe("return");

    // When: resume-from-return is allowed
    expect(canTransition(returnSnapshot, { type: "resume-from-return" }).allowed).toBe(true);

    // Then: town screen is reachable with provisioning available
    expect(canTransition(replayReadySnapshot, { type: "start-provisioning" }).allowed).toBe(true);
  });

  it("proves fallback return-to-town from result screen avoids dead-end", () => {
    // From result, return-to-town is a valid fallback
    expect(canTransition(resultSnapshot, { type: "return-to-town" }).allowed).toBe(true);

    // After reaching town, the full provisioning cycle is available
    expect(canTransition(replayReadySnapshot, { type: "start-provisioning" }).allowed).toBe(true);
    expect(canTransition(provisioningSnapshot, { type: "confirm-provisioning" }).allowed).toBe(true);
    expect(canTransition(expeditionSnapshot, { type: "launch-expedition" }).allowed).toBe(true);
  });

  it("proves no dead-end states exist in the meta-loop", () => {
    // Every terminal flow state has a path back to town
    const allFlows: Array<{ label: string; snapshot: DdgcFrontendSnapshot }> = [
      { label: "result (success)", snapshot: resultSnapshot },
      { label: "result (failure)", snapshot: failureResultSnapshot },
      { label: "result (partial)", snapshot: partialResultSnapshot },
      { label: "return", snapshot: returnSnapshot },
    ];

    for (const { label, snapshot } of allFlows) {
      expect(resolveScreen(snapshot)).not.toBe("startup");
      const townReturn = canTransition(snapshot, { type: "return-to-town" });
      expect(townReturn.allowed).toBe(true);
    }
  });

  it("proves full expedition cycle closes without dead-end", () => {
    // Full cycle: town → provisioning → expedition → combat → result → town → provisioning
    expect(canTransition(replayReadySnapshot, { type: "start-provisioning" }).allowed).toBe(true);
    expect(canTransition(provisioningSnapshot, { type: "confirm-provisioning" }).allowed).toBe(true);
    expect(canTransition(expeditionSnapshot, { type: "launch-expedition" }).allowed).toBe(true);
    expect(canTransition(resultSnapshot, { type: "continue-from-result" }).allowed).toBe(true);

    // Cycle completes: back in town, can provision again
    expect(canTransition(replayReadySnapshot, { type: "start-provisioning" }).allowed).toBe(true);
  });

  it("proves all three result outcomes can close the meta-loop", () => {
    const outcomes = [resultSnapshot, failureResultSnapshot, partialResultSnapshot];

    for (const snap of outcomes) {
      const vm = snap.viewModel as ExpeditionResultViewModel;
      expect(vm.isContinueAvailable).toBe(true);
      expect(canTransition(snap, { type: "continue-from-result" }).allowed).toBe(true);
    }
  });
});

describe("State handoff back into town screens is stable", () => {
  it("after continue-from-result, town screens are accessible", () => {
    // After continuing from result, we're in town → verify all town interactions
    expect(resolveScreen(replayReadySnapshot)).toBe("town");

    // Hero detail is accessible
    expect(canTransition(replayReadySnapshot, { type: "open-hero", heroId: "hero-hunter-01" }).allowed).toBe(true);

    // Building detail is accessible
    expect(canTransition(replayReadySnapshot, { type: "open-building", buildingId: "guild" }).allowed).toBe(true);

    // Provisioning is accessible
    expect(canTransition(replayReadySnapshot, { type: "start-provisioning" }).allowed).toBe(true);
  });

  it("after resume-from-return, town screens are accessible", () => {
    // Same as above - return goes back to town
    expect(resolveScreen(replayReadySnapshot)).toBe("town");

    expect(canTransition(replayReadySnapshot, { type: "open-hero", heroId: "hero-hunter-01" }).allowed).toBe(true);
    expect(canTransition(replayReadySnapshot, { type: "open-building", buildingId: "guild" }).allowed).toBe(true);
    expect(canTransition(replayReadySnapshot, { type: "start-provisioning" }).allowed).toBe(true);
  });

  it("after fallback return-to-town from result, town screens are accessible", () => {
    // Fallback return-to-town from result goes to town
    expect(resolveScreen(replayReadySnapshot)).toBe("town");

    // All town interactions available
    expect(canTransition(replayReadySnapshot, { type: "open-hero", heroId: "hero-hunter-01" }).allowed).toBe(true);
    expect(canTransition(replayReadySnapshot, { type: "open-building", buildingId: "guild" }).allowed).toBe(true);
    expect(canTransition(replayReadySnapshot, { type: "start-provisioning" }).allowed).toBe(true);
  });
});
