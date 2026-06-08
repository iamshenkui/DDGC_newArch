import type { RuntimeMode } from "../app/runtimeMode";
import {
  replayReadySnapshot,
  replayHeroDetailViewModel,
  replayBuildingDetailViewModel,
  replayDungeonSelectViewModel,
  replayProvisioningViewModel,
  replayExpeditionViewModel,
  replayResultViewModel,
  replayReturnViewModel
} from "../validation/replayFixtures";
import type { RuntimeBridge, RuntimeBridgeListener } from "./RuntimeBridge";
import type {
  DdgcFrontendIntent,
  DdgcFrontendSnapshot,
  TownViewModel,
  DungeonSelectViewModel,
  ProvisioningViewModel,
  ExpeditionSetupViewModel,
  ExpeditionResultViewModel,
  ReturnViewModel
} from "./contractTypes";

function deriveProvisioningFromDungeonSelect(dsVm: DungeonSelectViewModel): ProvisioningViewModel {
  const selectedDungeon = dsVm.dungeons.find((d) => d.id === dsVm.selectedDungeonId);
  const provisionParty = dsVm.party.map((hero) => ({
    ...hero,
    isSelected: hero.isSelected
  }));
  const selectedCount = provisionParty.filter((h) => h.isSelected).length;

  return {
    kind: "provisioning",
    title: "Provision Expedition",
    campaignName: dsVm.campaignName,
    expeditionLabel: selectedDungeon?.name ?? "Unknown Expedition",
    expeditionSummary: selectedDungeon?.description ?? "No description available.",
    party: provisionParty,
    maxPartySize: dsVm.maxPartySize,
    isReadyToLaunch: selectedCount >= 2 && selectedCount <= dsVm.maxPartySize,
    supplyLevel: selectedDungeon?.supplyLevel ?? "Basic",
    provisionCost: selectedDungeon?.provisionCost ?? "0 Gold"
  };
}

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
      case "open-hero": {
        const townVm = this.snapshot.viewModel as TownViewModel;
        const hero = townVm.heroes.find((h) => h.id === intent.heroId) ?? townVm.heroes[0];
        this.snapshot = {
          ...this.snapshot,
          flowState: "town",
          viewModel: {
            ...replayHeroDetailViewModel,
            heroId: hero.id,
            name: hero.name,
            classLabel: hero.classLabel,
            hp: hero.hp.split(" / ")[0],
            maxHp: hero.hp.split(" / ")[1] ?? hero.hp.split(" / ")[0],
            stress: hero.stress,
            maxStress: hero.maxStress,
            positiveQuirks: hero.positiveQuirks,
            negativeQuirks: hero.negativeQuirks,
            diseases: hero.diseases,
            isWounded: hero.isWounded,
            isAfflicted: hero.isAfflicted
          }
        };
        break;
      }
      case "open-building": {
        const townVm = this.snapshot.viewModel as TownViewModel;
        const building = townVm.buildings.find((b) => b.id === intent.buildingId) ?? townVm.buildings[0];
        this.snapshot = {
          ...this.snapshot,
          flowState: "town",
          viewModel: {
            ...replayBuildingDetailViewModel,
            buildingId: building.id,
            label: building.label,
            status: building.status
          }
        };
        break;
      }
      case "building-action":
        this.snapshot = {
          ...this.snapshot,
          debugMessage: `Replay: building action intent received for ${intent.actionId}.`
        };
        break;
      case "start-dungeon-select":
        this.snapshot = {
          ...this.snapshot,
          flowState: "dungeon-select",
          viewModel: replayDungeonSelectViewModel as DungeonSelectViewModel
        };
        break;
      case "select-dungeon": {
        const dsVm = this.snapshot.viewModel as DungeonSelectViewModel;
        const selectedDungeon = dsVm.dungeons.find((d) => d.id === intent.dungeonId);
        const isDungeonValid = selectedDungeon !== undefined && selectedDungeon.isAvailable;
        const selectedCount = dsVm.party.filter((h) => h.isSelected).length;
        this.snapshot = {
          ...this.snapshot,
          viewModel: {
            ...dsVm,
            selectedDungeonId: isDungeonValid ? intent.dungeonId : null,
            isReadyToProceed:
              isDungeonValid &&
              selectedCount >= 2 &&
              selectedCount <= dsVm.maxPartySize
          }
        };
        break;
      }
      case "toggle-dungeon-hero": {
        const dsVm2 = this.snapshot.viewModel as DungeonSelectViewModel;
        const updatedParty = dsVm2.party.map((hero) =>
          hero.id === intent.heroId
            ? { ...hero, isSelected: !hero.isSelected }
            : hero
        );
        const selectedCount = updatedParty.filter((h) => h.isSelected).length;
        const selectedDungeon = dsVm2.dungeons.find((d) => d.id === dsVm2.selectedDungeonId);
        const isDungeonValid = selectedDungeon !== undefined && selectedDungeon.isAvailable;
        this.snapshot = {
          ...this.snapshot,
          viewModel: {
            ...dsVm2,
            party: updatedParty,
            isReadyToProceed:
              isDungeonValid &&
              selectedCount >= 2 &&
              selectedCount <= dsVm2.maxPartySize
          }
        };
        break;
      }
      case "confirm-dungeon-selection": {
        const dsVm = this.snapshot.viewModel as DungeonSelectViewModel;
        if (!dsVm.isReadyToProceed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Replay: confirm-dungeon-selection rejected — selection is not ready to proceed."
          };
          break;
        }
        const selectedDungeon = dsVm.dungeons.find((d) => d.id === dsVm.selectedDungeonId);
        if (!selectedDungeon || !selectedDungeon.isAvailable) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Replay: confirm-dungeon-selection rejected — selected dungeon is not available."
          };
          break;
        }
        this.snapshot = {
          ...this.snapshot,
          flowState: "provisioning",
          viewModel: deriveProvisioningFromDungeonSelect(dsVm)
        };
        break;
      }
      case "start-provisioning":
        this.snapshot = {
          ...this.snapshot,
          flowState: "provisioning",
          viewModel: replayProvisioningViewModel as ProvisioningViewModel
        };
        break;
      case "toggle-hero-selection": {
        const provVm = this.snapshot.viewModel as ProvisioningViewModel;
        const updatedParty = provVm.party.map((hero) =>
          hero.id === intent.heroId
            ? { ...hero, isSelected: !hero.isSelected }
            : hero
        );
        const selectedCount = updatedParty.filter((h) => h.isSelected).length;
        this.snapshot = {
          ...this.snapshot,
          viewModel: {
            ...provVm,
            party: updatedParty,
            isReadyToLaunch: selectedCount >= 2 && selectedCount <= provVm.maxPartySize
          }
        };
        break;
      }
      case "confirm-provisioning":
        this.snapshot = {
          ...this.snapshot,
          flowState: "expedition",
          viewModel: replayExpeditionViewModel as ExpeditionSetupViewModel
        };
        break;
      case "launch-expedition":
        this.snapshot = {
          ...this.snapshot,
          flowState: "result",
          viewModel: replayResultViewModel as ExpeditionResultViewModel
        };
        break;
      case "return-to-town":
        this.snapshot = replayReadySnapshot;
        break;
      case "boot":
        this.snapshot = replayReadySnapshot;
        break;
      case "continue-from-result":
        this.snapshot = {
          ...this.snapshot,
          flowState: "return",
          viewModel: replayReturnViewModel as ReturnViewModel
        };
        break;
      case "resume-from-return":
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