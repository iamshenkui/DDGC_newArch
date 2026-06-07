import type { RuntimeMode } from "../app/runtimeMode";
import {
  replayReadySnapshot,
  replayHeroDetailViewModel,
  replayBuildingDetailViewModel,
  replayProvisioningViewModel,
  replayExpeditionViewModel,
  replayDungeonAssistViewModel,
  replayDungeonMapViewModel,
  replayAttackCombatViewModel,
  replayCombatViewModel,
  replayResultViewModel,
  replayReturnViewModel
} from "../validation/replayFixtures";
import type { RuntimeBridge, RuntimeBridgeListener } from "./RuntimeBridge";
import { canTransition } from "../session/FlowController";
import type {
  DdgcFrontendIntent,
  DdgcFrontendSnapshot,
  TownViewModel,
  ProvisioningViewModel,
  ExpeditionSetupViewModel,
  DungeonAssistViewModel,
  DungeonMapViewModel,
  ExpeditionResultViewModel,
  ReturnViewModel,
  CombatViewModel
} from "./contractTypes";

function advanceReplayCombatTurn(combatVm: CombatViewModel): CombatViewModel {
  const livingParty = combatVm.party.filter((hero) => hero.isAlive);
  const activeIndex = livingParty.findIndex((hero) => hero.id === combatVm.activeHeroId);
  const nextHero = livingParty[activeIndex + 1] ?? livingParty[0];
  const startsNewRound = activeIndex < 0 || activeIndex === livingParty.length - 1;
  const selectedSkillId =
    nextHero?.skills.find((skill) => skill.cooldownRemaining === 0)?.id ??
    nextHero?.skills[0]?.id;

  return {
    ...combatVm,
    round: startsNewRound ? combatVm.round + 1 : combatVm.round,
    activeHeroId: nextHero?.id ?? combatVm.activeHeroId,
    selectedSkillId,
    party: combatVm.party.map((hero) => ({
      ...hero,
      isActive: hero.id === nextHero?.id
    })),
    combatLog: [
      ...combatVm.combatLog,
      `${combatVm.party.find((hero) => hero.id === combatVm.activeHeroId)?.name ?? "Active hero"} ends their turn.`,
      startsNewRound
        ? `Round ${combatVm.round + 1} begins.`
        : `${nextHero?.name ?? "Next hero"} is ready.`
    ]
  };
}

function createReplayFleeResultViewModel(): ExpeditionResultViewModel {
  return {
    ...replayResultViewModel,
    outcome: "failure",
    summary: "The party fled combat before securing the objective. Regroup in town and prepare for another attempt.",
    lootAcquired: [],
    resourcesGained: {
      gold: 0,
      supplies: -20,
      experience: 20
    }
  };
}

function createReplayCharacterHitCombatViewModel(): CombatViewModel {
  return {
    ...replayCombatViewModel,
    title: "Dungeon Combat",
    dungeonName: "The Depths Await",
    roundLabel: "Round 3",
    phase: "character-hit",
    round: 3,
    selectedSkillId: undefined,
    party: replayCombatViewModel.party.map((hero) =>
      hero.id === "hero-hunter-01"
        ? { ...hero, hp: "28 / 42", stress: "24", isHit: true }
        : { ...hero, isHit: false }
    ),
    enemies: replayCombatViewModel.enemies.map((enemy) => ({ ...enemy, isHit: false })),
    hitTargetHeroId: "hero-hunter-01",
    hitDamage: "10",
    hitLog: "Cultist Acolyte strikes Shen for 10 damage.",
    combatLog: [
      "Cultist Acolyte strikes Shen for 10 damage.",
      "Acknowledge the hit before issuing the next command."
    ],
    isPlayerTurn: false,
    isFleeAvailable: true,
    turnCount: 3,
    settingsLabel: "设置"
  };
}

function acknowledgeReplayCombatHit(combatVm: CombatViewModel): CombatViewModel {
  const activeHero = combatVm.party.find((hero) => hero.id === combatVm.activeHeroId);
  const selectedSkillId =
    activeHero?.skills.find((skill) => skill.cooldownRemaining === 0)?.id ??
    activeHero?.skills[0]?.id;

  return {
    ...combatVm,
    phase: "player-turn",
    turnPhase: "player",
    selectedSkillId,
    party: combatVm.party.map((hero) => ({ ...hero, isHit: false })),
    enemies: combatVm.enemies.map((enemy) => ({ ...enemy, isHit: false })),
    hitTargetHeroId: undefined,
    hitDamage: undefined,
    hitLog: undefined,
    isPlayerTurn: true,
    combatLog: [...combatVm.combatLog, "Hit acknowledged. Player command restored."]
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
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Replay: launch-expedition rejected from current state."
          };
          break;
        }
        this.snapshot = {
          ...this.snapshot,
          flowState: "dungeon-assist",
          viewModel: replayDungeonAssistViewModel as DungeonAssistViewModel
        };
        break;
      case "enter-combat":
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Replay: enter-combat rejected outside expedition."
          };
          break;
        }
        this.snapshot = {
          ...this.snapshot,
          flowState: "combat",
          viewModel: createReplayCharacterHitCombatViewModel()
        };
        break;
      case "enter-dungeon-assist":
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Replay: enter-dungeon-assist rejected outside expedition."
          };
          break;
        }
        this.snapshot = {
          ...this.snapshot,
          flowState: "dungeon-assist",
          viewModel: replayDungeonAssistViewModel as DungeonAssistViewModel
        };
        break;
      case "select-assist-hero": {
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: `Replay: assist hero ${intent.heroId} rejected.`
          };
          break;
        }
        const assistVm = this.snapshot.viewModel as DungeonAssistViewModel;
        const updatedParty = assistVm.party.map((hero) =>
          hero.id === intent.heroId
            ? { ...hero, isSelected: true }
            : { ...hero, isSelected: false }
        );
        this.snapshot = {
          ...this.snapshot,
          viewModel: {
            ...assistVm,
            party: updatedParty,
            selectedHeroId: intent.heroId
          }
        };
        break;
      }
      case "use-assist-action": {
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: `Replay: assist action ${intent.actionId} rejected.`
          };
          break;
        }
        const assistVm = this.snapshot.viewModel as DungeonAssistViewModel;
        this.snapshot = {
          ...this.snapshot,
          viewModel: {
            ...assistVm,
            canContinue: true
          },
          debugMessage: `Replay: assist action ${intent.actionId} used.`
        };
        break;
      }
      case "continue-from-dungeon":
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Replay: continue-from-dungeon rejected until dungeon assist is ready."
          };
          break;
        }
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
        if (targetRoom && !targetRoom.isRevealed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: `enter-room rejected: room "${intent.roomId}" is not revealed`
          };
          break;
        }
        if (targetRoom.type === "combat") {
          this.snapshot = {
            ...this.snapshot,
            flowState: "combat",
            viewModel: replayAttackCombatViewModel as CombatViewModel,
            debugMessage: `Replay: entered combat room "${targetRoom.label}".`
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
      case "select-skill": {
        const validation = canTransition(this.snapshot, intent);
        if (!validation.allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: `Replay: combat skill ${intent.skillId} rejected: ${validation.reason ?? "invalid transition"}.`
          };
          break;
        }
        const combatVm = this.snapshot.viewModel as CombatViewModel;
        this.snapshot = {
          ...this.snapshot,
          viewModel: {
            ...combatVm,
            selectedSkillId: intent.skillId
          }
        };
        break;
      }
      case "use-skill": {
        const validation = canTransition(this.snapshot, intent);
        if (!validation.allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage:
              this.snapshot.viewModel.kind === "combat" && this.snapshot.viewModel.phase === "character-hit"
                ? "Replay: skill intent ignored during character-hit acknowledgement."
                : `Replay: use-skill rejected: ${validation.reason ?? "invalid transition"}.`
          };
          break;
        }
        const combatVm = this.snapshot.viewModel as CombatViewModel;
        this.snapshot = {
          ...this.snapshot,
          viewModel: {
            ...combatVm,
            selectedSkillId: intent.skillId,
            hitLog: `Skill used: ${intent.skillId}.`,
            combatLog: [...combatVm.combatLog, `Skill used: ${intent.skillId}.`]
          }
        };
        break;
      }
      case "select-target": {
        const validation = canTransition(this.snapshot, intent);
        if (!validation.allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: `Replay: combat target ${intent.enemyId} rejected: ${validation.reason ?? "invalid transition"}.`
          };
          break;
        }
        const combatVm = this.snapshot.viewModel as CombatViewModel;
        this.snapshot = {
          ...this.snapshot,
          viewModel: {
            ...combatVm,
            enemies: combatVm.enemies.map((enemy) => ({
              ...enemy,
              isTargeted: enemy.id === intent.enemyId
            }))
          }
        };
        break;
      }
      case "confirm-attack":
        {
          const validation = canTransition(this.snapshot, intent);
          if (!validation.allowed) {
            this.snapshot = {
              ...this.snapshot,
              debugMessage: `Replay: confirm-attack rejected: ${validation.reason ?? "invalid transition"}.`
            };
            break;
          }
        }
        this.snapshot = {
          ...this.snapshot,
          flowState: "result",
          viewModel: replayResultViewModel as ExpeditionResultViewModel
        };
        break;
      case "continue-from-combat":
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Replay: continue-from-combat rejected outside combat."
          };
          break;
        }
        if (this.snapshot.viewModel.kind === "combat" && this.snapshot.viewModel.phase === "character-hit") {
          this.snapshot = {
            ...this.snapshot,
            flowState: "combat",
            viewModel: acknowledgeReplayCombatHit(this.snapshot.viewModel),
            debugMessage: "Replay: character-hit acknowledgement accepted."
          };
          break;
        }
        this.snapshot = {
          ...this.snapshot,
          flowState: "result",
          viewModel: replayResultViewModel as ExpeditionResultViewModel
        };
        break;
      case "open-combat-settings":
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Replay: open-combat-settings rejected outside combat."
          };
          break;
        }
        this.snapshot = {
          ...this.snapshot,
          debugMessage: "Replay: combat settings intent received."
        };
        break;
      case "end-turn": {
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Replay: end-turn rejected outside combat."
          };
          break;
        }
        const combatVm = this.snapshot.viewModel as CombatViewModel;
        this.snapshot = {
          ...this.snapshot,
          flowState: "combat",
          viewModel: advanceReplayCombatTurn(combatVm),
          debugMessage: "Replay: combat end-turn intent advanced the active hero."
        };
        break;
      }
      case "flee-combat":
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Replay: flee-combat rejected outside combat."
          };
          break;
        }
        this.snapshot = {
          ...this.snapshot,
          flowState: "result",
          viewModel: createReplayFleeResultViewModel()
        };
        break;
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
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Replay: return-to-town rejected from current screen."
          };
          break;
        }
        this.snapshot = replayReadySnapshot;
        break;
      case "boot":
        this.snapshot = replayReadySnapshot;
        break;
      case "continue-from-result":
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Replay: continue-from-result rejected outside result."
          };
          break;
        }
        this.snapshot = {
          ...this.snapshot,
          flowState: "return",
          viewModel: replayReturnViewModel as ReturnViewModel
        };
        break;
      case "resume-from-return":
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Replay: resume-from-return rejected outside return."
          };
          break;
        }
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
