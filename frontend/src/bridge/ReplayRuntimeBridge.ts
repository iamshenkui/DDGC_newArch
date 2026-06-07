import type { RuntimeMode } from "../app/runtimeMode";
import {
  replayReadySnapshot,
  replayHeroDetailViewModel,
  replayBuildingDetailViewModel,
  replayProvisioningViewModel,
  replayExpeditionViewModel,
  replayResultViewModel,
  replayReturnViewModel,
  replayCombatViewModel,
  replayCombatSnapshot
} from "../validation/replayFixtures";
import type { RuntimeBridge, RuntimeBridgeListener } from "./RuntimeBridge";
import type {
  DdgcFrontendIntent,
  DdgcFrontendSnapshot,
  TownViewModel,
  ProvisioningViewModel,
  ExpeditionSetupViewModel,
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
          flowState: "combat",
          viewModel: replayCombatViewModel as CombatViewModel
        };
        break;
      case "select-skill": {
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
      case "select-target": {
        const combatVm = this.snapshot.viewModel as CombatViewModel;
        this.snapshot = {
          ...this.snapshot,
          viewModel: {
            ...combatVm,
            enemies: combatVm.enemies.map((e) => ({
              ...e,
              isTargeted: e.id === intent.enemyId
            }))
          }
        };
        break;
      }
      case "confirm-attack":
        this.snapshot = {
          ...this.snapshot,
          flowState: "result",
          viewModel: replayResultViewModel as ExpeditionResultViewModel
        };
        break;
      case "end-turn": {
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
        this.snapshot = {
          ...this.snapshot,
          flowState: "result",
          viewModel: createReplayFleeResultViewModel()
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
