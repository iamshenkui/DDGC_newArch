import { describe, expect, it } from "vitest";

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

  it("surfaces live mode as unsupported until wired", async () => {
    const bridge = new LiveRuntimeBridge();
    const snapshot = await bridge.boot();

    expect(snapshot.lifecycle).toBe("unsupported");
    expect(snapshot.viewModel.kind).toBe("unsupported");
  });
});