import { describe, expect, it } from "vitest";

import { createSessionStore } from "./SessionStore";
import type { DdgcFrontendSnapshot } from "../bridge/contractTypes";
import {
  fatalSnapshot,
  replayReadySnapshot,
  replayLoadingSnapshot,
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