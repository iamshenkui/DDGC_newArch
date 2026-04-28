import { describe, expect, it } from "vitest";

import { resolveScreen, canTransition } from "./FlowController";
import type {
  DdgcFrontendSnapshot,
  ProvisioningViewModel,
  ExpeditionSetupViewModel,
} from "../bridge/contractTypes";
import {
  provisioningSnapshot,
  expeditionSnapshot,
  resultSnapshot,
  returnSnapshot,
  replayReadySnapshot,
  startupSnapshot,
  fatalSnapshot,
  unsupportedSnapshot,
  replayLoadingSnapshot,
  replayHeroDetailSnapshot,
  replayBuildingDetailSnapshot,
} from "../validation/replayFixtures";

describe("Provisioning → Expedition → Launch flow validation", () => {
  // ── Provisioning flow ─────────────────────────────────────────────

  it("allows start-provisioning from town screen", () => {
    const validation = canTransition(replayReadySnapshot, { type: "start-provisioning" });
    expect(validation.allowed).toBe(true);
  });

  it("rejects start-provisioning from non-town screens", () => {
    const nonTownSnapshots: DdgcFrontendSnapshot[] = [
      provisioningSnapshot,
      expeditionSnapshot,
      resultSnapshot,
      returnSnapshot,
      startupSnapshot,
      replayLoadingSnapshot,
      replayHeroDetailSnapshot,
      replayBuildingDetailSnapshot,
      fatalSnapshot,
      unsupportedSnapshot,
    ];

    for (const snap of nonTownSnapshots) {
      const validation = canTransition(snap, { type: "start-provisioning" });
      expect(validation.allowed).toBe(false);
    }
  });

  it("allows toggle-hero-selection in provisioning screen", () => {
    const validation = canTransition(provisioningSnapshot, {
      type: "toggle-hero-selection",
      heroId: "hero-hunter-01",
    });
    expect(validation.allowed).toBe(true);
  });

  it("rejects toggle-hero-selection from non-provisioning screens", () => {
    const notProvisioning: DdgcFrontendSnapshot[] = [
      replayReadySnapshot,
      expeditionSnapshot,
      resultSnapshot,
      returnSnapshot,
    ];

    for (const snap of notProvisioning) {
      const validation = canTransition(snap, {
        type: "toggle-hero-selection",
        heroId: "hero-hunter-01",
      });
      expect(validation.allowed).toBe(false);
    }
  });

  it("allows confirm-provisioning when provisioning is ready", () => {
    const validation = canTransition(provisioningSnapshot, { type: "confirm-provisioning" });
    expect(validation.allowed).toBe(true);
  });

  it("rejects confirm-provisioning when provisioning vm indicates not ready", () => {
    const notReadySnapshot: DdgcFrontendSnapshot = {
      ...provisioningSnapshot,
      viewModel: {
        ...provisioningSnapshot.viewModel,
        isReadyToLaunch: false,
      } as ProvisioningViewModel,
    };
    const validation = canTransition(notReadySnapshot, { type: "confirm-provisioning" });
    expect(validation.allowed).toBe(false);
    expect(validation.reason).toContain("not ready");
  });

  it("rejects confirm-provisioning from non-provisioning screens", () => {
    const notProvisioning: DdgcFrontendSnapshot[] = [
      replayReadySnapshot,
      expeditionSnapshot,
      resultSnapshot,
      returnSnapshot,
    ];

    for (const snap of notProvisioning) {
      const validation = canTransition(snap, { type: "confirm-provisioning" });
      expect(validation.allowed).toBe(false);
    }
  });

  // ── Expedition launch flow ────────────────────────────────────────

  it("allows launch-expedition when expedition is launchable", () => {
    const validation = canTransition(expeditionSnapshot, { type: "launch-expedition" });
    expect(validation.allowed).toBe(true);
  });

  it("rejects launch-expedition when expedition vm indicates not launchable", () => {
    const notLaunchableSnapshot: DdgcFrontendSnapshot = {
      ...expeditionSnapshot,
      viewModel: {
        ...expeditionSnapshot.viewModel,
        isLaunchable: false,
      } as ExpeditionSetupViewModel,
    };
    const validation = canTransition(notLaunchableSnapshot, { type: "launch-expedition" });
    expect(validation.allowed).toBe(false);
    expect(validation.reason).toContain("not launchable");
  });

  it("rejects launch-expedition from non-expedition screens", () => {
    const notExpedition: DdgcFrontendSnapshot[] = [
      replayReadySnapshot,
      provisioningSnapshot,
      resultSnapshot,
      returnSnapshot,
    ];

    for (const snap of notExpedition) {
      const validation = canTransition(snap, { type: "launch-expedition" });
      expect(validation.allowed).toBe(false);
    }
  });

  // ── Return-to-town from provisioning/expedition ───────────────────

  it("allows return-to-town from provisioning screen", () => {
    const validation = canTransition(provisioningSnapshot, { type: "return-to-town" });
    expect(validation.allowed).toBe(true);
  });

  it("allows return-to-town from expedition screen", () => {
    const validation = canTransition(expeditionSnapshot, { type: "return-to-town" });
    expect(validation.allowed).toBe(true);
  });

  it("rejects return-to-town from town screen", () => {
    const validation = canTransition(replayReadySnapshot, { type: "return-to-town" });
    expect(validation.allowed).toBe(false);
  });

  // ── Screen resolution ─────────────────────────────────────────────

  it("resolves provisioning screen for provisioning view model", () => {
    const screen = resolveScreen(provisioningSnapshot);
    expect(screen).toBe("provisioning");
  });

  it("resolves expedition screen for expedition view model", () => {
    const screen = resolveScreen(expeditionSnapshot);
    expect(screen).toBe("expedition");
  });

  it("proves provisioning flow from town screen", () => {
    // Start from town
    expect(resolveScreen(replayReadySnapshot)).toBe("town");

    // Verify the provisioning transition path
    const provisionValidation = canTransition(replayReadySnapshot, { type: "start-provisioning" });
    expect(provisionValidation.allowed).toBe(true);

    // Verify provisioning screen would resolve
    expect(resolveScreen(provisioningSnapshot)).toBe("provisioning");
  });

  it("proves expedition flow from provisioning screen", () => {
    // Start from provisioning
    expect(resolveScreen(provisioningSnapshot)).toBe("provisioning");

    // Verify confirm-provisioning leads to expedition
    const confirmValidation = canTransition(provisioningSnapshot, { type: "confirm-provisioning" });
    expect(confirmValidation.allowed).toBe(true);

    // Verify expedition screen would resolve
    expect(resolveScreen(expeditionSnapshot)).toBe("expedition");
  });

  it("proves complete town → provision → expedition → launch path", () => {
    // Step 1: Town can start provisioning
    expect(canTransition(replayReadySnapshot, { type: "start-provisioning" }).allowed).toBe(true);

    // Step 2: Provisioning can confirm
    expect(canTransition(provisioningSnapshot, { type: "confirm-provisioning" }).allowed).toBe(true);

    // Step 3: Expedition can launch
    expect(canTransition(expeditionSnapshot, { type: "launch-expedition" }).allowed).toBe(true);
  });

  // ── Provisioning flow edge cases ──────────────────────────────────

  it("validates provisioning view model contract fields", () => {
    const vm = provisioningSnapshot.viewModel;
    expect(vm.kind).toBe("provisioning");
    expect(vm).toHaveProperty("title");
    expect(vm).toHaveProperty("expeditionLabel");
    expect(vm).toHaveProperty("expeditionSummary");
    expect(vm).toHaveProperty("party");
    expect(vm).toHaveProperty("maxPartySize");
    expect(vm).toHaveProperty("isReadyToLaunch");
    expect(vm).toHaveProperty("supplyLevel");
    expect(vm).toHaveProperty("provisionCost");
  });

  it("validates provisioning party hero contract fields", () => {
    const vm = provisioningSnapshot.viewModel;
    if (vm.kind !== "provisioning") return;
    for (const hero of vm.party) {
      expect(hero).toHaveProperty("id");
      expect(hero).toHaveProperty("name");
      expect(hero).toHaveProperty("classLabel");
      expect(hero).toHaveProperty("hp");
      expect(hero).toHaveProperty("maxHp");
      expect(hero).toHaveProperty("stress");
      expect(hero).toHaveProperty("maxStress");
      expect(hero).toHaveProperty("level");
      expect(hero).toHaveProperty("isSelected");
      expect(hero).toHaveProperty("isWounded");
      expect(hero).toHaveProperty("isAfflicted");
    }
  });

  it("validates expedition view model contract fields", () => {
    const vm = expeditionSnapshot.viewModel;
    expect(vm.kind).toBe("expedition");
    expect(vm).toHaveProperty("title");
    expect(vm).toHaveProperty("expeditionName");
    expect(vm).toHaveProperty("partySize");
    expect(vm).toHaveProperty("party");
    expect(vm).toHaveProperty("difficulty");
    expect(vm).toHaveProperty("estimatedDuration");
    expect(vm).toHaveProperty("objectives");
    expect(vm).toHaveProperty("warnings");
    expect(vm).toHaveProperty("supplyLevel");
    expect(vm).toHaveProperty("provisionCost");
    expect(vm).toHaveProperty("isLaunchable");
  });

  it("validates expedition hero contract fields", () => {
    const vm = expeditionSnapshot.viewModel;
    if (vm.kind !== "expedition") return;
    for (const hero of vm.party) {
      expect(hero).toHaveProperty("id");
      expect(hero).toHaveProperty("name");
      expect(hero).toHaveProperty("classLabel");
      expect(hero).toHaveProperty("hp");
      expect(hero).toHaveProperty("maxHp");
      expect(hero).toHaveProperty("stress");
      expect(hero).toHaveProperty("maxStress");
    }
  });

  // ── Expedition flow edge cases ────────────────────────────────────

  it("expedition fixture has expedition view model kind", () => {
    expect(expeditionSnapshot.viewModel.kind).toBe("expedition");
  });

  it("expedition fixture is launchable by default", () => {
    const vm = expeditionSnapshot.viewModel;
    if (vm.kind !== "expedition") return;
    expect(vm.isLaunchable).toBe(true);
  });

  it("expedition fixture has party members", () => {
    const vm = expeditionSnapshot.viewModel;
    if (vm.kind !== "expedition") return;
    expect(vm.party.length).toBeGreaterThan(0);
    expect(vm.partySize).toBe(vm.party.length);
  });
});
