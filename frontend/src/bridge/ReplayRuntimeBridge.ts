import type { RuntimeMode } from "../app/runtimeMode";
import {
  replayReadySnapshot,
  replayHeroDetailViewModel,
  replayBuildingDetailViewModel,
  replayProvisioningViewModel,
  replayExpeditionViewModel,
  replayDungeonMapViewModel,
  replayResultViewModel,
  replayReturnViewModel
} from "../validation/replayFixtures";
import type { RuntimeBridge, RuntimeBridgeListener } from "./RuntimeBridge";
import type {
  DdgcFrontendIntent,
  DdgcFrontendSnapshot,
  TownViewModel,
  ProvisioningViewModel,
  ExpeditionSetupViewModel,
  DungeonMapViewModel,
  ExpeditionResultViewModel,
  ReturnViewModel
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
          flowState: "dungeon",
          viewModel: replayDungeonMapViewModel as DungeonMapViewModel
        };
        break;
      case "enter-room": {
        const dungeonVm = this.snapshot.viewModel as DungeonMapViewModel;
        const updatedRooms = dungeonVm.rooms.map((room) =>
          room.roomId === intent.roomId
            ? { ...room, cleared: true, isCurrent: true }
            : { ...room, isCurrent: false }
        );
        const clearedCount = updatedRooms.filter((r) => r.cleared).length;
        const isComplete = clearedCount >= dungeonVm.totalRooms;
        this.snapshot = {
          ...this.snapshot,
          flowState: isComplete ? "result" : "dungeon",
          viewModel: isComplete
            ? replayResultViewModel as ExpeditionResultViewModel
            : {
                ...dungeonVm,
                rooms: updatedRooms,
                roomsCleared: clearedCount,
                currentRoom: updatedRooms.find((r) => r.roomId === intent.roomId) ?? dungeonVm.currentRoom,
                isComplete
              }
        };
        break;
      }
      case "flee-dungeon": {
        this.snapshot = {
          ...this.snapshot,
          flowState: "return",
          viewModel: {
            ...replayReturnViewModel as ReturnViewModel,
            title: "Retreating from Dungeon",
            summary: "The party has fled the dungeon. Survivors are returning to town."
          }
        };
        break;
      }
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