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
  ProvisioningViewModel,
  ExpeditionSetupViewModel,
  ExpeditionResultViewModel,
  ReturnViewModel,
} from "./contractTypes";
import { createTownBuildingSummary } from "../town/buildingCatalog";

const createLiveTownViewModel = (): TownViewModel => ({
  kind: "town",
  title: "城镇界面",
  campaignName: "新档位面",
  campaignSummary:
    "实时模式已启动，当前展示的是新战役的城镇主界面与初始编队。",
  heroes: [
    {
      id: "hero-hunter-live-01",
      name: "Yuan",
      classLabel: "Hunter",
      hp: "42 / 42",
      maxHp: "42",
      health: 42,
      maxHealth: 42,
      stress: "0",
      maxStress: "200",
      level: 1,
      xp: 0,
      isWounded: false,
      isAfflicted: false,
      positiveQuirks: [],
      negativeQuirks: [],
      diseases: []
    },
    {
      id: "hero-white-live-01",
      name: "Mei",
      classLabel: "White",
      hp: "41 / 41",
      maxHp: "41",
      health: 41,
      maxHealth: 41,
      stress: "0",
      maxStress: "200",
      level: 1,
      xp: 0,
      isWounded: false,
      isAfflicted: false,
      positiveQuirks: [],
      negativeQuirks: [],
      diseases: []
    }
  ] as ReadonlyArray<TownHeroSummary>,
  buildings: [
    createTownBuildingSummary("stagecoach", "ready"),
    createTownBuildingSummary("guild", "ready"),
    createTownBuildingSummary("blacksmith", "ready"),
    createTownBuildingSummary("sanitarium", "ready"),
    createTownBuildingSummary("abbey", "ready"),
    createTownBuildingSummary("tavern", "ready"),
    createTownBuildingSummary("graveyard", "partial"),
    createTownBuildingSummary("garden", "partial"),
    createTownBuildingSummary("legacytower", "partial"),
    createTownBuildingSummary("market", "partial"),
    createTownBuildingSummary("campingtrainer", "partial")
  ] as ReadonlyArray<TownBuildingSummary>,
  roster: [
    {
      id: "hero-hunter-live-01",
      name: "Yuan",
      classLabel: "Hunter",
      hp: "42 / 42",
      maxHp: "42",
      health: 42,
      maxHealth: 42,
      stress: "0",
      maxStress: "200",
      level: 1,
      xp: 0,
      isWounded: false,
      isAfflicted: false,
      positiveQuirks: [],
      negativeQuirks: [],
      diseases: []
    },
    {
      id: "hero-white-live-01",
      name: "Mei",
      classLabel: "White",
      hp: "41 / 41",
      maxHp: "41",
      health: 41,
      maxHealth: 41,
      stress: "0",
      maxStress: "200",
      level: 1,
      xp: 0,
      isWounded: false,
      isAfflicted: false,
      positiveQuirks: [],
      negativeQuirks: [],
      diseases: []
    }
  ] as ReadonlyArray<TownHeroSummary>,
  gold: 500,
  isFreshVisit: true,
  nextActionLabel: "整备并出发"
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
  resolveLabel: "Heroic",
  maxStress: "200",
  progression: {
    level: hero.level,
    experience: "0",
    experienceToNext: "300",
    resolveLevel: 1,
    resolveXP: "0"
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
  baseStats: {
    dmg: "8-12",
    maxHp: hero.maxHp,
    crit: "5%",
    spd: "4",
    dodge: "10%"
  },
  combatSkills: [
    { name: "Skill 1", level: 1, description: "Combat skill description.", target: "Enemy", hitRating: "80%", critRating: "5%" },
    { name: "Skill 2", level: 1, description: "Combat skill description.", target: "Self", hitRating: "100%", critRating: "0%" }
  ],
  campingSkills: [
    { name: "Campfire Song", level: 1, description: "Camping skill description.", target: "Party", hitRating: "100%", critRating: "0%" }
  ],
  weapon: { name: "Basic Weapon", level: 1 },
  armor: { name: "Leather Armor", level: 1 },
  positiveQuirks: [],
  negativeQuirks: [],
  diseases: [],
  isWounded: hero.isWounded,
  isAfflicted: hero.isAfflicted,
  heroDescription: "A brave hero ready for adventure.",
  talent: "Versatile"
});

const createLiveBuildingDetailViewModel = (building: TownBuildingSummary): BuildingDetailViewModel => {
  const buildingConfigs: Record<string, BuildingDetailViewModel> = {
    stagecoach: {
      kind: "building-detail",
      buildingId: "stagecoach",
      label: building.label,
      status: building.status,
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
      kind: "building-detail",
      buildingId: "guild",
      label: building.label,
      status: building.status,
      description: "The guild provides skill training and party capability review. Upgrade your heroes' abilities.",
      currentUpgrade: "Training Hall Level 1",
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
    },
    blacksmith: {
      kind: "building-detail",
      buildingId: "blacksmith",
      label: building.label,
      status: building.status,
      description: "The blacksmith forges and upgrades weapons and armor. Enhance your heroes' equipment to improve their combat effectiveness.",
      currentUpgrade: "Forge Level 1",
      actions: [
        {
          id: "upgrade-weapon",
          label: "Upgrade Weapon",
          description: "Enhance a hero's weapon to deal more damage in combat.",
          cost: "400 Gold",
          isAvailable: true,
          isUnsupported: false
        },
        {
          id: "upgrade-armor",
          label: "Upgrade Armor",
          description: "Improve a hero's armor for better protection against enemy attacks.",
          cost: "350 Gold",
          isAvailable: false,
          isUnsupported: false
        }
      ],
      upgradeCategories: [
        {
          id: "weapon",
          label: "武器锻造",
          slots: [
            { level: 1, label: "初级锻造", isUnlocked: true, isCurrent: true, cost: "200 Gold" },
            { level: 2, label: "中级锻造", isUnlocked: false, isCurrent: false, cost: "400 Gold" },
            { level: 3, label: "高级锻造", isUnlocked: false, isCurrent: false, cost: "600 Gold" },
            { level: 4, label: "大师锻造", isUnlocked: false, isCurrent: false, cost: "1000 Gold" },
            { level: 5, label: "传说锻造", isUnlocked: false, isCurrent: false, cost: "2000 Gold" }
          ]
        },
        {
          id: "armor",
          label: "护甲强化",
          slots: [
            { level: 1, label: "初级强化", isUnlocked: true, isCurrent: true, cost: "150 Gold" },
            { level: 2, label: "中级强化", isUnlocked: false, isCurrent: false, cost: "350 Gold" },
            { level: 3, label: "高级强化", isUnlocked: false, isCurrent: false, cost: "550 Gold" },
            { level: 4, label: "大师强化", isUnlocked: false, isCurrent: false, cost: "900 Gold" },
            { level: 5, label: "传说强化", isUnlocked: false, isCurrent: false, cost: "1800 Gold" }
          ]
        }
      ],
      resources: [
        { type: "gem", label: "Gem", amount: 10 },
        { type: "shard", label: "Shard", amount: 10 },
        { type: "core", label: "Core", amount: 10 },
        { type: "gold", label: "Gold", amount: 20 }
      ]
    }
  };

  const config = buildingConfigs[building.id] ?? {
    kind: "building-detail",
    buildingId: building.id,
    label: building.label,
    status: building.status,
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

  return config;
};

const createLiveProvisioningViewModel = (): ProvisioningViewModel => ({
  kind: "provisioning",
  title: "Provision Expedition",
  campaignName: "Fresh Campaign",
  expeditionLabel: "The Azure Lantern Expedition",
  expeditionSummary: "Deploy your party into the dungeon. Manage supplies and party composition carefully.",
  party: [
    { id: "hero-hunter-live-01", name: "Yuan", classLabel: "Hunter", hp: "42 / 42", maxHp: "42", health: 42, maxHealth: 42, stress: "0", maxStress: "200", level: 1, xp: 0, isWounded: false, isAfflicted: false, isSelected: true },
    { id: "hero-white-live-01", name: "Mei", classLabel: "White", hp: "41 / 41", maxHp: "41", health: 41, maxHealth: 41, stress: "0", maxStress: "200", level: 1, xp: 0, isWounded: false, isAfflicted: false, isSelected: true }
  ],
  maxPartySize: 4,
  isReadyToLaunch: true,
  supplyLevel: "Adequate",
  provisionCost: "100 Gold"
});

const createLiveExpeditionViewModel = (): ExpeditionSetupViewModel => ({
  kind: "expedition",
  title: "Expedition Launch",
  expeditionName: "The Azure Lantern Expedition",
  partySize: 2,
  party: [
    { id: "hero-hunter-live-01", name: "Yuan", classLabel: "Hunter", hp: "42 / 42", maxHp: "42", stress: "0", maxStress: "200" },
    { id: "hero-white-live-01", name: "Mei", classLabel: "White", hp: "41 / 41", maxHp: "41", stress: "0", maxStress: "200" }
  ],
  difficulty: "Challenging",
  estimatedDuration: "Medium",
  objectives: ["Explore the dungeon", "Find the treasure", "Return alive"],
  warnings: ["High stress area ahead", "Limited camping spots"],
  supplyLevel: "Adequate",
  provisionCost: "100 Gold",
  isLaunchable: true
});

const createLiveResultViewModel = (): ExpeditionResultViewModel => ({
  kind: "result",
  title: "Expedition Complete",
  expeditionName: "The Azure Lantern Expedition",
  outcome: "success",
  summary: "Your expedition has returned. Review the outcomes and continue your campaign.",
  lootAcquired: ["Gold Coin x2", "Ancient Relic"],
  heroOutcomes: [
    {
      heroId: "hero-hunter-live-01",
      heroName: "Yuan",
      classLabel: "Hunter",
      status: "alive",
      hpChange: "-2",
      stressChange: "+5"
    },
    {
      heroId: "hero-white-live-01",
      heroName: "Mei",
      classLabel: "White",
      status: "alive",
      hpChange: "-3",
      stressChange: "+3"
    }
  ],
  resourcesGained: {
    gold: 150,
    supplies: -30,
    experience: 100
  },
  isContinueAvailable: true
});

const createLiveReturnViewModel = (): ReturnViewModel => ({
  kind: "return",
  title: "Returning to Town",
  expeditionName: "The Azure Lantern Expedition",
  summary: "The expedition party has returned safely. Resume town activities.",
  returningHeroes: [
    {
      heroId: "hero-hunter-live-01",
      heroName: "Yuan",
      classLabel: "Hunter",
      hp: "40 / 42",
      stress: "5"
    },
    {
      heroId: "hero-white-live-01",
      heroName: "Mei",
      classLabel: "White",
      hp: "38 / 41",
      stress: "3"
    }
  ],
  isTownResumeAvailable: true
});

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
          flowState: "provisioning",
          viewModel: createLiveProvisioningViewModel()
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
          viewModel: createLiveExpeditionViewModel()
        };
        break;
      case "launch-expedition":
        this.snapshot = {
          ...this.snapshot,
          flowState: "result",
          viewModel: createLiveResultViewModel()
        };
        break;
      case "return-to-town":
        this.snapshot = createLiveTownSnapshot();
        break;
      case "continue-from-result":
        this.snapshot = {
          ...this.snapshot,
          flowState: "return",
          viewModel: createLiveReturnViewModel()
        };
        break;
      case "resume-from-return":
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