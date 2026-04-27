import type { RuntimeMode } from "../app/runtimeMode";
import type { RuntimeBridge, RuntimeBridgeListener } from "./RuntimeBridge";
import type {
  DdgcFrontendIntent,
  DdgcFrontendSnapshot,
  TownViewModel,
  TownHeroSummary,
  TownBuildingSummary,
  HeroDetailViewModel,
  BuildingDetailViewModel,
} from "./contractTypes";

const createLiveTownViewModel = (): TownViewModel => ({
  kind: "town",
  title: "Town Surface — Live Mode",
  campaignName: "Fresh Campaign",
  campaignSummary:
    "Live runtime boot: DDGC host initialized with fresh campaign state. Roster and building data reflects initial campaign setup.",
  heroes: [
    {
      id: "hero-hunter-live-01",
      name: "Yuan",
      classLabel: "Hunter",
      hp: "42 / 42",
      stress: "0",
      level: 1
    },
    {
      id: "hero-white-live-01",
      name: "Mei",
      classLabel: "White",
      hp: "41 / 41",
      stress: "0",
      level: 1
    }
  ] as ReadonlyArray<TownHeroSummary>,
  buildings: [
    {
      id: "stagecoach",
      label: "Stagecoach",
      summary: "Recruit new heroes to your party.",
      status: "ready"
    },
    {
      id: "guild",
      label: "Guild",
      summary: "Skill training and party capability review.",
      status: "ready"
    }
  ] as ReadonlyArray<TownBuildingSummary>,
  nextActionLabel: "Launch Expedition"
});

const createLiveTownSnapshot = (): DdgcFrontendSnapshot => ({
  lifecycle: "ready",
  flowState: "town",
  viewModel: createLiveTownViewModel(),
  debugMessage: "Live runtime bridge booted: fresh campaign initialized through DdgcHost::boot_live()."
});

const createLiveHeroDetailViewModel = (hero: TownHeroSummary): HeroDetailViewModel => ({
  kind: "hero-detail",
  heroId: hero.id,
  name: hero.name,
  classLabel: hero.classLabel,
  hp: hero.hp.split(" / ")[0],
  maxHp: hero.hp.split(" / ")[1] ?? hero.hp.split(" / ")[0],
  stress: hero.stress,
  resolve: "3",
  progression: {
    level: hero.level,
    experience: "0",
    experienceToNext: "300"
  },
  resistances: {
    stun: "40%",
    bleed: "60%",
    disease: "30%",
    move: "50%",
    death: "0%",
    trap: "70%",
    hazard: "20%"
  },
  combatSkills: ["Skill 1", "Skill 2"],
  campingSkills: ["Campfire Song"],
  weapon: "Basic Weapon",
  armor: "Leather Armor",
  campNotes: "Hero detail view - live mode placeholder."
});

const createLiveBuildingDetailViewModel = (building: TownBuildingSummary): BuildingDetailViewModel => {
  const buildingConfigs: Record<string, {
    description: string;
    actions: Array<{
      id: string;
      label: string;
      description: string;
      cost: string;
      isAvailable: boolean;
      isUnsupported: boolean;
    }>;
    upgradeRequirement?: string;
  }> = {
    stagecoach: {
      description: "The stagecoach offers new recruits from the surrounding region. Recruit heroes to expand your party roster.",
      actions: [
        {
          id: "recruit-hero",
          label: "Recruit Hero",
          description: "Recruit a new hero to your party from available candidates.",
          cost: "500 Gold",
          isAvailable: true,
          isUnsupported: false
        },
        {
          id: "view-candidates",
          label: "View Candidates",
          description: "Browse available hero candidates without recruiting.",
          cost: "Free",
          isAvailable: true,
          isUnsupported: false
        }
      ]
    },
    guild: {
      description: "The guild provides skill training and party capability review. Upgrade your heroes' abilities.",
      actions: [
        {
          id: "train-skill",
          label: "Train Skill",
          description: "Improve a hero's combat or camping skill.",
          cost: "200 Gold",
          isAvailable: true,
          isUnsupported: false
        },
        {
          id: "upgrade-weapon",
          label: "Upgrade Weapon",
          description: "Enhance a hero's weapon.",
          cost: "300 Gold",
          isAvailable: false,
          isUnsupported: false
        },
        {
          id: "upgrade-armor",
          label: "Upgrade Armor",
          description: "Improve a hero's armor protection.",
          cost: "300 Gold",
          isAvailable: false,
          isUnsupported: false
        }
      ]
    }
  };

  const config = buildingConfigs[building.id] ?? {
    description: building.summary,
    actions: [
      {
        id: "interact",
        label: "Interact",
        description: "Interact with this building.",
        cost: "Free",
        isAvailable: true,
        isUnsupported: false
      }
    ]
  };

  return {
    kind: "building-detail",
    buildingId: building.id,
    label: building.label,
    status: building.status,
    description: config.description,
    actions: config.actions,
    upgradeRequirement: config.upgradeRequirement
  };
};

export class LiveRuntimeBridge implements RuntimeBridge {
  readonly id = "ddgc-live-bridge";
  readonly mode: RuntimeMode = "live";

  private listeners = new Set<RuntimeBridgeListener>();
  private snapshot = createLiveTownSnapshot();

  async boot(): Promise<DdgcFrontendSnapshot> {
    this.emit(this.snapshot);
    return this.snapshot;
  }

  currentSnapshot(): DdgcFrontendSnapshot {
    return this.snapshot;
  }

  async dispatchIntent(intent: DdgcFrontendIntent): Promise<DdgcFrontendSnapshot> {
    switch (intent.type) {
      case "boot":
        this.snapshot = createLiveTownSnapshot();
        break;
      case "open-hero": {
        const townVm = this.snapshot.viewModel as TownViewModel;
        const hero = townVm.heroes.find((h) => h.id === intent.heroId) ?? townVm.heroes[0];
        this.snapshot = {
          ...this.snapshot,
          flowState: "town",
          viewModel: createLiveHeroDetailViewModel(hero)
        };
        break;
      }
      case "open-building": {
        const townVm = this.snapshot.viewModel as TownViewModel;
        const building = townVm.buildings.find((b) => b.id === intent.buildingId) ?? townVm.buildings[0];
        this.snapshot = {
          ...this.snapshot,
          flowState: "town",
          viewModel: createLiveBuildingDetailViewModel(building)
        };
        break;
      }
      case "building-action":
        this.snapshot = {
          ...this.snapshot,
          debugMessage: `Live: building action intent received for ${intent.actionId}.`
        };
        break;
      case "start-provisioning":
        this.snapshot = {
          ...this.snapshot,
          debugMessage: "Live: provisioning flow placeholder requested."
        };
        break;
      case "launch-expedition":
        this.snapshot = {
          ...this.snapshot,
          flowState: "expedition",
          debugMessage:
            "Live: expedition handoff placeholder. Real runtime transition in subsequent Phase 10 iterations."
        };
        break;
      case "return-to-town":
        this.snapshot = createLiveTownSnapshot();
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