import { describe, expect, it } from "vitest";

import type { HeroDetailViewModel } from "../bridge/contractTypes";
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
});