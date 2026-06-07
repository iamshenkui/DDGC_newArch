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
  DungeonMapViewModel,
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
    currentUpgrade?: string;
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
    currentUpgrade: config.currentUpgrade,
    upgradeRequirement: config.upgradeRequirement
  };
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

const createLiveDungeonMapViewModel = (): DungeonMapViewModel => ({
  kind: "dungeon-map",
  title: "Dungeon Map",
  expeditionName: "The Azure Lantern Expedition",
  dungeonName: "Azure Lantern Depths",
  currentRoomId: "room-entrance",
  rooms: [
    { id: "room-entrance", x: 2, y: 4, type: "entrance", label: "Entrance", isRevealed: true, isVisited: true, isCurrent: true, connections: ["room-empty-1", "room-combat-1"] },
    { id: "room-empty-1", x: 2, y: 3, type: "empty", label: "Hallway", isRevealed: true, isVisited: false, isCurrent: false, connections: ["room-entrance", "room-treasure-1"] },
    { id: "room-combat-1", x: 3, y: 4, type: "combat", label: "Ambush", isRevealed: true, isVisited: false, isCurrent: false, connections: ["room-entrance", "room-curio-1"], difficulty: "Easy" },
    { id: "room-treasure-1", x: 2, y: 2, type: "treasure", label: "Cache", isRevealed: true, isVisited: false, isCurrent: false, connections: ["room-empty-1", "room-rest-1"], lootPreview: "Gold + Relic" },
    { id: "room-curio-1", x: 4, y: 4, type: "curio", label: "Strange Idol", isRevealed: true, isVisited: false, isCurrent: false, connections: ["room-combat-1", "room-shrine-1"] },
    { id: "room-rest-1", x: 2, y: 1, type: "rest", label: "Safe Room", isRevealed: true, isVisited: false, isCurrent: false, connections: ["room-treasure-1", "room-boss-1"] },
    { id: "room-shrine-1", x: 5, y: 4, type: "shrine", label: "Healing Shrine", isRevealed: true, isVisited: false, isCurrent: false, connections: ["room-curio-1", "room-combat-2"] },
    { id: "room-combat-2", x: 5, y: 3, type: "combat", label: "Elite Guard", isRevealed: true, isVisited: false, isCurrent: false, connections: ["room-shrine-1", "room-exit"], difficulty: "Hard" },
    { id: "room-boss-1", x: 2, y: 0, type: "boss", label: "Depths Guardian", isRevealed: false, isVisited: false, isCurrent: false, connections: ["room-rest-1"], difficulty: "Boss" },
    { id: "room-exit", x: 5, y: 2, type: "exit", label: "Exit", isRevealed: true, isVisited: false, isCurrent: false, connections: ["room-combat-2"] }
  ],
  party: [
    { id: "hero-hunter-live-01", name: "Yuan", classLabel: "Hunter", hp: "42 / 42", maxHp: "42", stress: "0", maxStress: "200", isWounded: false, isAfflicted: false },
    { id: "hero-white-live-01", name: "Mei", classLabel: "White", hp: "41 / 41", maxHp: "41", stress: "0", maxStress: "200", isWounded: false, isAfflicted: false }
  ],
  torchLevel: 75,
  maxTorchLevel: 100,
  exploredCount: 1,
  totalRooms: 10,
  completionPercent: 10,
  isRetreatAvailable: true,
  isComplete: false,
  minimapRows: 5,
  minimapCols: 6
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
          flowState: "dungeon-map",
          viewModel: createLiveDungeonMapViewModel()
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
            ...createLiveResultViewModel(),
            outcome: "partial",
            title: "Expedition Partial Success",
            summary: "Your party retreated from the dungeon with what they could carry."
          }
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
          viewModel: createLiveResultViewModel()
        };
        break;
      }
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