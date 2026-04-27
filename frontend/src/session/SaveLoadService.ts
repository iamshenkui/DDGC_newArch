import type { DdgcFrontendSnapshot } from "../bridge/contractTypes";
import {
  replayReadySnapshot,
  liveLoadingSnapshot,
  replayLoadingSnapshot,
} from "../validation/replayFixtures";

const SAVED_SNAPSHOT_KEY = "ddgc_campaign_snapshot";

export interface SaveLoadService {
  save(snapshot: DdgcFrontendSnapshot): void;
  load(): DdgcFrontendSnapshot | null;
  hasSavedCampaign(): boolean;
  delete(): void;
}

function createDefaultNewCampaignSnapshot(mode: "replay" | "live"): DdgcFrontendSnapshot {
  return mode === "replay" ? replayReadySnapshot : liveLoadingSnapshot;
}

export function createSaveLoadService(mode: "replay" | "live"): SaveLoadService {
  return {
    save(snapshot: DdgcFrontendSnapshot) {
      try {
        localStorage.setItem(SAVED_SNAPSHOT_KEY, JSON.stringify(snapshot));
      } catch (e) {
        console.warn("Failed to save campaign snapshot:", e);
      }
    },

    load(): DdgcFrontendSnapshot | null {
      try {
        const saved = localStorage.getItem(SAVED_SNAPSHOT_KEY);
        if (saved) {
          return JSON.parse(saved) as DdgcFrontendSnapshot;
        }
      } catch (e) {
        console.warn("Failed to load campaign snapshot:", e);
      }
      return null;
    },

    hasSavedCampaign(): boolean {
      return localStorage.getItem(SAVED_SNAPSHOT_KEY) !== null;
    },

    delete() {
      try {
        localStorage.removeItem(SAVED_SNAPSHOT_KEY);
      } catch (e) {
        console.warn("Failed to delete campaign snapshot:", e);
      }
    }
  };
}

export function createFreshCampaignSnapshot(mode: "replay" | "live"): DdgcFrontendSnapshot {
  return createDefaultNewCampaignSnapshot(mode);
}