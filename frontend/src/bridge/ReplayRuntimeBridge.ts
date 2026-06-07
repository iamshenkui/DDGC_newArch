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
          flowState: "dungeon-map",
          viewModel: replayDungeonMapViewModel as DungeonMapViewModel
        };
        break;
      case "enter-room": {
        const mapVm = this.snapshot.viewModel as DungeonMapViewModel;
        const targetRoom = mapVm.rooms.find((r) => r.id === intent.roomId);
        if (!targetRoom) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: `enter-room rejected: room "${intent.roomId}" does not exist`
          };
          break;
        }
        const currentRoom = mapVm.rooms.find((r) => r.id === mapVm.currentRoomId);
        if (currentRoom && intent.roomId !== currentRoom.id && !currentRoom.connections.includes(intent.roomId)) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: `enter-room rejected: room "${intent.roomId}" is not connected to the current room`
          };
          break;
        }
        if (!targetRoom.isRevealed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: `enter-room rejected: room "${intent.roomId}" is not revealed`
          };
          break;
        }
        const updatedRooms = mapVm.rooms.map((room) =>
          room.id === intent.roomId
            ? { ...room, isVisited: true, isCurrent: true }
            : { ...room, isCurrent: false }
        );
        const visitedCount = updatedRooms.filter((r) => r.isVisited).length;
        const total = updatedRooms.length;
        const newCompletion = Math.round((visitedCount / total) * 100);
        this.snapshot = {
          ...this.snapshot,
          viewModel: {
            ...mapVm,
            currentRoomId: intent.roomId,
            rooms: updatedRooms,
            exploredCount: visitedCount,
            completionPercent: newCompletion,
            isComplete: newCompletion >= 80
          }
        };
        break;
      }
      case "retreat-from-dungeon":
        this.snapshot = {
          ...this.snapshot,
          flowState: "result",
          viewModel: {
            ...replayResultViewModel,
            outcome: "partial",
            title: "Expedition Partial Success",
            summary: "Your party retreated from the dungeon with what they could carry."
          } as ExpeditionResultViewModel
        };
        break;
      case "complete-dungeon": {
        const mapVm = this.snapshot.viewModel as DungeonMapViewModel;
        if (!mapVm.isComplete) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: `complete-dungeon rejected: dungeon is not complete`
          };
          break;
        }
        this.snapshot = {
          ...this.snapshot,
          flowState: "result",
          viewModel: replayResultViewModel as ExpeditionResultViewModel
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