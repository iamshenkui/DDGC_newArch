import { describe, expect, it } from "vitest";

import { createSessionStore } from "./SessionStore";
import { resolveScreen, canTransition } from "./FlowController";
import type { DdgcFrontendSnapshot } from "../bridge/contractTypes";
import {
  fatalSnapshot,
  replayReadySnapshot,
  replayLoadingSnapshot,
  resultSnapshot,
  failureResultSnapshot,
  partialResultSnapshot,
  returnSnapshot,
  provisioningSnapshot,
  expeditionSnapshot,
} from "../validation/replayFixtures";

describe("SessionStore", () => {
  it("initializes with the provided snapshot", () => {
    const store = createSessionStore(replayReadySnapshot);
    expect(store.snapshot()).toBe(replayReadySnapshot);
  });

  it("replaces snapshot via replace()", () => {
    const store = createSessionStore(replayReadySnapshot);
    store.replace(replayLoadingSnapshot);
    expect(store.snapshot()).toBe(replayLoadingSnapshot);
  });

  it("fails with fatal snapshot and provided reason", () => {
    const store = createSessionStore(replayReadySnapshot);
    const errorReason = "Data directory not found";
    store.fail(errorReason);

    const snapshot = store.snapshot();
    expect(snapshot.lifecycle).toBe("fatal");
    expect(snapshot.viewModel.kind).toBe("fatal");
    if (snapshot.viewModel.kind === "fatal") {
      expect(snapshot.viewModel.reason).toBe(errorReason);
    }
  });

  it("fail() preserves fatalSnapshot structure", () => {
    const store = createSessionStore(replayReadySnapshot);
    store.fail("test failure");

    const snapshot = store.snapshot();
    expect(snapshot.flowState).toBe(fatalSnapshot.flowState);
    expect(snapshot.viewModel.kind).toBe("fatal");
  });

  it("replace() accepts any valid snapshot", () => {
    const store = createSessionStore(fatalSnapshot);

    const validSnapshots: DdgcFrontendSnapshot[] = [
      replayReadySnapshot,
      replayLoadingSnapshot,
      fatalSnapshot,
    ];

    for (const snap of validSnapshots) {
      store.replace(snap);
      expect(store.snapshot()).toBe(snap);
    }
  });
});

describe("SessionStore meta-loop continuation", () => {
  it("supports result screen snapshot then continuation to town", () => {
    const store = createSessionStore(resultSnapshot);

    // Initially on result screen
    expect(resolveScreen(store.snapshot())).toBe("result");

    // Can transition from result
    expect(canTransition(store.snapshot(), { type: "continue-from-result" }).allowed).toBe(true);

    // Replace with town snapshot (simulating continue-from-result)
    store.replace(replayReadySnapshot);
    expect(resolveScreen(store.snapshot())).toBe("town");

    // Can now start provisioning
    expect(canTransition(store.snapshot(), { type: "start-provisioning" }).allowed).toBe(true);
  });

  it("supports return screen snapshot then continuation to town", () => {
    const store = createSessionStore(returnSnapshot);

    // Initially on return screen
    expect(resolveScreen(store.snapshot())).toBe("return");

    // Can transition from return
    expect(canTransition(store.snapshot(), { type: "resume-from-return" }).allowed).toBe(true);

    // Replace with town snapshot (simulating resume-from-return)
    store.replace(replayReadySnapshot);
    expect(resolveScreen(store.snapshot())).toBe("town");

    // Town interactions are stable
    expect(canTransition(store.snapshot(), { type: "open-hero", heroId: "hero-hunter-01" }).allowed).toBe(true);
    expect(canTransition(store.snapshot(), { type: "open-building", buildingId: "guild" }).allowed).toBe(true);
    expect(canTransition(store.snapshot(), { type: "start-provisioning" }).allowed).toBe(true);
  });

  it("supports full result → town → provisioning cycle through session store", () => {
    const store = createSessionStore(resultSnapshot);
    expect(resolveScreen(store.snapshot())).toBe("result");

    // Simulate continue-from-result: bridge replaces snapshot
    store.replace(replayReadySnapshot);
    expect(resolveScreen(store.snapshot())).toBe("town");

    // Simulate start-provisioning
    store.replace(provisioningSnapshot);
    expect(resolveScreen(store.snapshot())).toBe("provisioning");
  });

  it("handles failure result → town → provisioning cycle", () => {
    const store = createSessionStore(failureResultSnapshot);
    expect(resolveScreen(store.snapshot())).toBe("result");

    // continue-from-result is allowed even from failure
    expect(canTransition(store.snapshot(), { type: "continue-from-result" }).allowed).toBe(true);

    store.replace(replayReadySnapshot);
    expect(resolveScreen(store.snapshot())).toBe("town");

    // Can still provision after failure
    expect(canTransition(store.snapshot(), { type: "start-provisioning" }).allowed).toBe(true);
  });

  it("handles partial result → town → provisioning cycle", () => {
    const store = createSessionStore(partialResultSnapshot);
    expect(resolveScreen(store.snapshot())).toBe("result");

    expect(canTransition(store.snapshot(), { type: "continue-from-result" }).allowed).toBe(true);

    store.replace(replayReadySnapshot);
    expect(resolveScreen(store.snapshot())).toBe("town");
    expect(canTransition(store.snapshot(), { type: "start-provisioning" }).allowed).toBe(true);
  });

  it("supports fallback return-to-town from result screen", () => {
    const store = createSessionStore(resultSnapshot);
    expect(canTransition(store.snapshot(), { type: "return-to-town" }).allowed).toBe(true);

    store.replace(replayReadySnapshot);
    expect(resolveScreen(store.snapshot())).toBe("town");
  });

  it("supports fallback return-to-town from return screen", () => {
    const store = createSessionStore(returnSnapshot);
    expect(canTransition(store.snapshot(), { type: "return-to-town" }).allowed).toBe(true);

    store.replace(replayReadySnapshot);
    expect(resolveScreen(store.snapshot())).toBe("town");
  });

  it("no dead-end states: all terminal flow states can return to town", () => {
    const terminalSnapshots: Array<{ label: string; snapshot: DdgcFrontendSnapshot }> = [
      { label: "result (success)", snapshot: resultSnapshot },
      { label: "result (failure)", snapshot: failureResultSnapshot },
      { label: "result (partial)", snapshot: partialResultSnapshot },
      { label: "return", snapshot: returnSnapshot },
    ];

    for (const { label, snapshot } of terminalSnapshots) {
      const store = createSessionStore(snapshot);
      expect(resolveScreen(store.snapshot())).not.toBe("startup");

      // Fallback return-to-town is always available
      expect(canTransition(store.snapshot(), { type: "return-to-town" }).allowed).toBe(true);

      // Replacing with town snapshot works
      store.replace(replayReadySnapshot);
      expect(resolveScreen(store.snapshot())).toBe("town");
    }
  });

  it("proves full expedition cycle loop through session store", () => {
    const store = createSessionStore(replayReadySnapshot);
    expect(resolveScreen(store.snapshot())).toBe("town");

    // town → provisioning
    expect(canTransition(store.snapshot(), { type: "start-provisioning" }).allowed).toBe(true);
    store.replace(provisioningSnapshot);
    expect(resolveScreen(store.snapshot())).toBe("provisioning");

    // provisioning → expedition
    expect(canTransition(store.snapshot(), { type: "confirm-provisioning" }).allowed).toBe(true);
    store.replace(expeditionSnapshot);
    expect(resolveScreen(store.snapshot())).toBe("expedition");

    // expedition → result
    expect(canTransition(store.snapshot(), { type: "launch-expedition" }).allowed).toBe(true);
    store.replace(resultSnapshot);
    expect(resolveScreen(store.snapshot())).toBe("result");

    // result → town (loop closes)
    expect(canTransition(store.snapshot(), { type: "continue-from-result" }).allowed).toBe(true);
    store.replace(replayReadySnapshot);
    expect(resolveScreen(store.snapshot())).toBe("town");

    // Can start provisioning again (loop is closed)
    expect(canTransition(store.snapshot(), { type: "start-provisioning" }).allowed).toBe(true);
  });

  it("supports multiple consecutive full expedition cycles through session store", () => {
    const store = createSessionStore(replayReadySnapshot);

    for (let cycle = 1; cycle <= 3; cycle++) {
      // town → provisioning
      expect(canTransition(store.snapshot(), { type: "start-provisioning" }).allowed).toBe(true);
      store.replace(provisioningSnapshot);
      expect(resolveScreen(store.snapshot())).toBe("provisioning");

      // provisioning → expedition
      expect(canTransition(store.snapshot(), { type: "confirm-provisioning" }).allowed).toBe(true);
      store.replace(expeditionSnapshot);
      expect(resolveScreen(store.snapshot())).toBe("expedition");

      // expedition → result
      expect(canTransition(store.snapshot(), { type: "launch-expedition" }).allowed).toBe(true);
      store.replace(resultSnapshot);
      expect(resolveScreen(store.snapshot())).toBe("result");

      // result → town (loop closes)
      expect(canTransition(store.snapshot(), { type: "continue-from-result" }).allowed).toBe(true);
      store.replace(replayReadySnapshot);
      expect(resolveScreen(store.snapshot())).toBe("town");
    }

    // After 3 cycles, all town interactions still work
    expect(canTransition(store.snapshot(), { type: "start-provisioning" }).allowed).toBe(true);
    expect(canTransition(store.snapshot(), { type: "open-hero", heroId: "hero-hunter-01" }).allowed).toBe(true);
    expect(canTransition(store.snapshot(), { type: "open-building", buildingId: "guild" }).allowed).toBe(true);
  });

  it("supports failure result → town → success result → town mixed cycles", () => {
    const store = createSessionStore(failureResultSnapshot);
    expect(resolveScreen(store.snapshot())).toBe("result");

    // Failure result → town
    expect(canTransition(store.snapshot(), { type: "continue-from-result" }).allowed).toBe(true);
    store.replace(replayReadySnapshot);
    expect(resolveScreen(store.snapshot())).toBe("town");

    // town → provisioning → expedition → success result
    store.replace(provisioningSnapshot);
    store.replace(expeditionSnapshot);
    store.replace(resultSnapshot);
    expect(resolveScreen(store.snapshot())).toBe("result");

    // Success result → town
    expect(canTransition(store.snapshot(), { type: "continue-from-result" }).allowed).toBe(true);
    store.replace(replayReadySnapshot);
    expect(resolveScreen(store.snapshot())).toBe("town");

    // After mixed outcome cycles, all town interactions work
    expect(canTransition(store.snapshot(), { type: "open-hero", heroId: "hero-hunter-01" }).allowed).toBe(true);
    expect(canTransition(store.snapshot(), { type: "open-building", buildingId: "guild" }).allowed).toBe(true);
    expect(canTransition(store.snapshot(), { type: "start-provisioning" }).allowed).toBe(true);
  });

  it("proves return-to-town fallback works from all terminal flow states after multiple cycles", () => {
    const terminalSnapshots: Array<{ label: string; snapshot: DdgcFrontendSnapshot }> = [
      { label: "result (success)", snapshot: resultSnapshot },
      { label: "result (failure)", snapshot: failureResultSnapshot },
      { label: "result (partial)", snapshot: partialResultSnapshot },
      { label: "return", snapshot: returnSnapshot },
    ];

    // Run 2 full cycles from each terminal state via return-to-town fallback
    for (let cycle = 1; cycle <= 2; cycle++) {
      for (const { label, snapshot } of terminalSnapshots) {
        const store = createSessionStore(snapshot);
        expect(canTransition(store.snapshot(), { type: "return-to-town" }).allowed).toBe(true);
        store.replace(replayReadySnapshot);
        expect(resolveScreen(store.snapshot())).toBe("town");

        // Town interactions are stable after fallback return
        expect(canTransition(store.snapshot(), { type: "start-provisioning" }).allowed).toBe(true);
        expect(canTransition(store.snapshot(), { type: "open-hero", heroId: "hero-hunter-01" }).allowed).toBe(true);
      }
    }
  });
});