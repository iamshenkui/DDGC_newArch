import type { RuntimeMode } from "../app/runtimeMode";
import {
  replayReadySnapshot
} from "../validation/replayFixtures";
import type { RuntimeBridge, RuntimeBridgeListener } from "./RuntimeBridge";
import type {
  DdgcFrontendIntent,
  DdgcFrontendSnapshot
} from "./contractTypes";

export class ReplayRuntimeBridge implements RuntimeBridge {
  readonly id = "ddgc-replay-bridge";
  readonly mode: RuntimeMode = "replay";

  private listeners = new Set<RuntimeBridgeListener>();
  private snapshot = replayReadySnapshot;

  async boot(): Promise<DdgcFrontendSnapshot> {
    this.emit(this.snapshot);
    return this.snapshot;
  }

  currentSnapshot(): DdgcFrontendSnapshot {
    return this.snapshot;
  }

  async dispatchIntent(intent: DdgcFrontendIntent): Promise<DdgcFrontendSnapshot> {
    switch (intent.type) {
      case "open-hero":
        this.snapshot = {
          ...this.snapshot,
          debugMessage: `Open hero intent received for ${intent.heroId}.`
        };
        break;
      case "open-building":
        this.snapshot = {
          ...this.snapshot,
          debugMessage: `Open building intent received for ${intent.buildingId}.`
        };
        break;
      case "start-provisioning":
        this.snapshot = {
          ...this.snapshot,
          debugMessage: "Provisioning flow placeholder requested."
        };
        break;
      case "launch-expedition":
        this.snapshot = {
          ...this.snapshot,
          flowState: "expedition",
          debugMessage:
            "Expedition handoff placeholder requested. Replace with real runtime transition in Phase 10."
        };
        break;
      case "return-to-town":
        this.snapshot = replayReadySnapshot;
        break;
      case "boot":
        this.snapshot = replayReadySnapshot;
        break;
    }

    this.emit(this.snapshot);
    return this.snapshot;
  }

  subscribe(listener: RuntimeBridgeListener): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  private emit(snapshot: DdgcFrontendSnapshot): void {
    for (const listener of this.listeners) {
      listener(snapshot);
    }
  }
}