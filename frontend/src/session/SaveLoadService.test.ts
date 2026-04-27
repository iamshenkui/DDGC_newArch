import { describe, expect, it, beforeEach, afterEach } from "vitest";

import {
  createSaveLoadService,
  createFreshCampaignSnapshot
} from "./SaveLoadService";
import { replayReadySnapshot } from "../validation/replayFixtures";

describe("SaveLoadService", () => {
  const testKey = "ddgc_campaign_snapshot";

  afterEach(() => {
    // Clean up localStorage after each test
    localStorage.removeItem(testKey);
  });

  describe("save and load", () => {
    it("saves a snapshot to localStorage", () => {
      const service = createSaveLoadService("replay");
      service.save(replayReadySnapshot);

      const saved = localStorage.getItem(testKey);
      expect(saved).not.toBeNull();
      expect(JSON.parse(saved!)).toEqual(replayReadySnapshot);
    });

    it("loads a saved snapshot from localStorage", () => {
      const service = createSaveLoadService("replay");
      service.save(replayReadySnapshot);

      const loaded = service.load();
      expect(loaded).toEqual(replayReadySnapshot);
    });

    it("returns null when no saved snapshot exists", () => {
      const service = createSaveLoadService("replay");
      const loaded = service.load();
      expect(loaded).toBeNull();
    });
  });

  describe("hasSavedCampaign", () => {
    it("returns false when no campaign is saved", () => {
      const service = createSaveLoadService("replay");
      expect(service.hasSavedCampaign()).toBe(false);
    });

    it("returns true after saving a campaign", () => {
      const service = createSaveLoadService("replay");
      service.save(replayReadySnapshot);
      expect(service.hasSavedCampaign()).toBe(true);
    });
  });

  describe("delete", () => {
    it("removes the saved snapshot from localStorage", () => {
      const service = createSaveLoadService("replay");
      service.save(replayReadySnapshot);
      expect(service.hasSavedCampaign()).toBe(true);

      service.delete();
      expect(service.hasSavedCampaign()).toBe(false);
    });
  });

  describe("createFreshCampaignSnapshot", () => {
    it("creates a valid replay snapshot for new campaign", () => {
      const snapshot = createFreshCampaignSnapshot("replay");
      expect(snapshot.lifecycle).toBe("ready");
      expect(snapshot.flowState).toBe("town");
      expect(snapshot.viewModel.kind).toBe("town");
    });

    it("creates a valid live snapshot for new campaign", () => {
      const snapshot = createFreshCampaignSnapshot("live");
      expect(snapshot.lifecycle).toBe("loading");
      expect(snapshot.flowState).toBe("load");
      expect(snapshot.viewModel.kind).toBe("boot-load");
    });
  });
});

describe("Campaign entry path validation", () => {
  afterEach(() => {
    localStorage.removeItem("ddgc_campaign_snapshot");
  });

  it("proves new campaign entry produces a town snapshot", () => {
    const service = createSaveLoadService("replay");
    const freshSnapshot = createFreshCampaignSnapshot("replay");

    // Verify the fresh snapshot has town state
    expect(freshSnapshot.lifecycle).toBe("ready");
    expect(freshSnapshot.flowState).toBe("town");
    expect(freshSnapshot.viewModel.kind).toBe("town");

    // Save and verify it can be retrieved
    service.save(freshSnapshot);
    const loaded = service.load();
    expect(loaded).toEqual(freshSnapshot);
  });

  it("proves save/load cycle preserves town shell state", () => {
    const service = createSaveLoadService("replay");
    const originalSnapshot = replayReadySnapshot;

    service.save(originalSnapshot);
    const loadedSnapshot = service.load();

    expect(loadedSnapshot).not.toBeNull();
    expect(loadedSnapshot!.viewModel.kind).toBe("town");
    expect(loadedSnapshot!.flowState).toBe("town");
    expect(loadedSnapshot!.lifecycle).toBe("ready");
  });

  it("proves load campaign returns null when storage is empty", () => {
    const service = createSaveLoadService("replay");
    expect(service.load()).toBeNull();
    expect(service.hasSavedCampaign()).toBe(false);
  });
});