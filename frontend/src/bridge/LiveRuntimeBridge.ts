import type { RuntimeMode } from "../app/runtimeMode";
import type { RuntimeBridge, RuntimeBridgeListener } from "./RuntimeBridge";
import { canTransition } from "../session/FlowController";
import type {
  DdgcFrontendIntent,
  DdgcFrontendSnapshot,
  TownViewModel,
  TownHeroSummary,
  TownBuildingSummary,
  HeroDetailViewModel,
  BuildingDetailViewModel,
  DungeonSelectViewModel,
  ExpeditionPlanningViewModel,
  ProvisioningViewModel,
  ExpeditionSetupViewModel,
  DungeonAssistViewModel,
  DungeonMapViewModel,
  ExpeditionResultViewModel,
  ReturnViewModel,
  DungeonInteractionViewModel,
  CombatViewModel,
  ProvisioningHeroSummary
} from "./contractTypes";
import { createTownBuildingSummary } from "../town/buildingCatalog";

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

function deriveProvisioningFromExpeditionPlanning(planningVm: ExpeditionPlanningViewModel): ProvisioningViewModel {
  const selectedPlane = planningVm.planes.find((p) => p.id === planningVm.selectedPlaneId);
  const selectedIds = new Set(planningVm.partySlots.filter((s): s is NonNullable<typeof s> => s !== null).map((s) => s.heroId));
  const roster = planningVm.roster ?? [];

  const party: ProvisioningHeroSummary[] = roster.map((hero) => ({
    ...hero,
    isSelected: selectedIds.has(hero.id)
  }));
  const selectedCount = party.filter((h) => h.isSelected).length;

  return {
    kind: "provisioning",
    title: "战前补给",
    campaignName: planningVm.campaignName,
    expeditionLabel: selectedPlane?.name ?? "未知远征",
    expeditionSummary: selectedPlane?.description ?? "做好出发前的准备，合理分配补给。",
    party,
    maxPartySize: planningVm.maxPartySize,
    isReadyToLaunch: selectedCount >= 2 && selectedCount <= planningVm.maxPartySize,
    supplyLevel: "充足",
    provisionCost: planningVm.provisionCost,
    supplies: [
      { id: "supply-food", name: "干粮", icon: "🍞", qty: 8 },
      { id: "supply-torch", name: "火把", icon: "🔥", qty: 6 },
      { id: "supply-bandage", name: "绷带", icon: "🩹", qty: 4 },
      { id: "supply-antidote", name: "解毒剂", icon: "🧪", qty: 2 },
      { id: "supply-shovel", name: "铁锹", icon: "⛏", qty: 2 },
      { id: "supply-key", name: "万能钥匙", icon: "🔑", qty: 1 },
      { id: "supply-holy", name: "圣水", icon: "✨", qty: 2 }
    ]
  };
}

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

const createLiveDungeonSelectViewModel = (): DungeonSelectViewModel => ({
  kind: "dungeon-select",
  title: "副本选择人物",
  campaignName: "新档位面",
  selectedDungeonId: null,
  dungeons: [
    {
      id: "dungeon-ruins-live",
      name: "废墟遗迹",
      description: "古老的废墟中隐藏着危险的敌人和珍贵的宝藏。",
      difficulty: "简单",
      estimatedDuration: "短",
      recommendedLevel: 1,
      provisionCost: "100 Gold",
      supplyLevel: "基础",
      rewards: ["古金币", "初级装备"],
      isAvailable: true
    },
    {
      id: "dungeon-forest-live",
      name: "迷雾森林",
      description: "被浓雾笼罩的古老森林。",
      difficulty: "普通",
      estimatedDuration: "中等",
      recommendedLevel: 2,
      provisionCost: "150 Gold",
      supplyLevel: "标准",
      rewards: ["神秘宝石", "中级装备"],
      isAvailable: true
    }
  ],
  party: [
    { id: "hero-hunter-live-01", name: "Yuan", classLabel: "Hunter", hp: "42 / 42", maxHp: "42", health: 42, maxHealth: 42, stress: "0", maxStress: "200", level: 1, xp: 0, isWounded: false, isAfflicted: false, isSelected: false },
    { id: "hero-white-live-01", name: "Mei", classLabel: "White", hp: "41 / 41", maxHp: "41", health: 41, maxHealth: 41, stress: "0", maxStress: "200", level: 1, xp: 0, isWounded: false, isAfflicted: false, isSelected: false }
  ],
  maxPartySize: 4,
  isReadyToProceed: false
});

const createLiveExpeditionPlanningViewModel = (): ExpeditionPlanningViewModel => ({
  kind: "expedition-planning",
  title: "Plane Exploration",
  campaignName: "Fresh Campaign",
  selectedPlaneId: "qinglong",
  planes: [
    {
      id: "qinglong",
      name: "青龙",
      description: "The Azure Dragon plane — ancient forests shrouded in mist.",
      difficulty: "Challenging",
      difficultyPips: 3,
      estimatedDuration: "Medium",
      isLocked: false,
      rewards: ["Dragon Scale", "Ancient Wood"],
      objectives: ["Explore the forest depths", "Defeat the Azure Dragon"],
      themeColor: "#4a9b8e"
    },
    {
      id: "baihu",
      name: "白虎",
      description: "The White Tiger plane — fortress ruins where phantoms roam.",
      difficulty: "Hard",
      difficultyPips: 4,
      estimatedDuration: "Long",
      isLocked: false,
      rewards: ["Tiger Fang", "Steel Fragment"],
      objectives: ["Breach the fortress gates", "Defeat the White Tiger"],
      themeColor: "#c9a959"
    },
    {
      id: "zhuque",
      name: "朱雀",
      description: "The Vermilion Bird plane — fire temples where ghost flames dance.",
      difficulty: "Very Hard",
      difficultyPips: 5,
      estimatedDuration: "Very Long",
      isLocked: true,
      lockReason: "Complete QingLong first",
      rewards: ["Phoenix Feather", "Fire Gem"],
      objectives: ["Navigate the burning temples", "Defeat the Vermilion Bird"],
      themeColor: "#d4563c"
    },
    {
      id: "xuanwu",
      name: "玄武",
      description: "The Black Tortoise plane — watery depths where serpents coil.",
      difficulty: "Extreme",
      difficultyPips: 5,
      estimatedDuration: "Very Long",
      isLocked: true,
      lockReason: "Complete BaiHu first",
      rewards: ["Turtle Shell", "Ice Crystal"],
      objectives: ["Descend into the abyss", "Defeat the Black Tortoise"],
      themeColor: "#4a6fa5"
    }
  ],
  partySlots: [
    { heroId: "hero-hunter-live-01", heroName: "Yuan", classLabel: "Hunter", hp: "42 / 42", stress: "0", level: 1 },
    { heroId: "hero-white-live-01", heroName: "Mei", classLabel: "White", hp: "41 / 41", stress: "0", level: 1 },
    null,
    null
  ],
  maxPartySize: 4,
  isReadyToProvision: true,
  provisionCost: "100 Gold",
  roster: [
    { id: "hero-hunter-live-01", name: "Yuan", classLabel: "Hunter", hp: "42 / 42", maxHp: "42", health: 42, maxHealth: 42, stress: "0", maxStress: "200", level: 1, xp: 0, isWounded: false, isAfflicted: false, isSelected: true },
    { id: "hero-white-live-01", name: "Mei", classLabel: "White", hp: "41 / 41", maxHp: "41", health: 41, maxHealth: 41, stress: "0", maxStress: "200", level: 1, xp: 0, isWounded: false, isAfflicted: false, isSelected: true }
  ]
});

const createLiveProvisioningViewModel = (): ProvisioningViewModel => ({
  kind: "provisioning",
  title: "战前补给",
  campaignName: "新档位面",
  expeditionLabel: "苍灯远征",
  expeditionSummary: "做好出发前的准备，合理分配补给。",
  party: [
    { id: "hero-hunter-live-01", name: "Yuan", classLabel: "Hunter", hp: "42 / 42", maxHp: "42", health: 42, maxHealth: 42, stress: "0", maxStress: "200", level: 1, xp: 0, isWounded: false, isAfflicted: false, isSelected: true },
    { id: "hero-white-live-01", name: "Mei", classLabel: "White", hp: "41 / 41", maxHp: "41", health: 41, maxHealth: 41, stress: "0", maxStress: "200", level: 1, xp: 0, isWounded: false, isAfflicted: false, isSelected: true }
  ],
  maxPartySize: 4,
  isReadyToLaunch: true,
  supplyLevel: "充足",
  provisionCost: "100 金币",
  supplies: [
    { id: "supply-food", name: "干粮", icon: "🍞", qty: 8 },
    { id: "supply-torch", name: "火把", icon: "🔥", qty: 6 },
    { id: "supply-bandage", name: "绷带", icon: "🩹", qty: 4 },
    { id: "supply-antidote", name: "解毒剂", icon: "🧪", qty: 2 },
    { id: "supply-shovel", name: "铁锹", icon: "⛏", qty: 2 },
    { id: "supply-key", name: "万能钥匙", icon: "🔑", qty: 1 },
    { id: "supply-holy", name: "圣水", icon: "✨", qty: 2 }
  ]
});

const createLiveDungeonHintViewModel = (): import("./contractTypes").DungeonHintViewModel => ({
  kind: "dungeon-hint",
  title: "副本提示",
  expeditionName: "The Azure Lantern Expedition",
  dungeonDescription: "An ancient dungeon filled with corrupted spirits and forgotten treasures. Only the prepared will survive its depths.",
  recommendedLevel: "Level 1+",
  partySize: 2,
  difficulty: "Challenging",
  estimatedDuration: "Medium",
  tips: [
    "Bring torches to reduce stress accumulation",
    "Healing supplies are essential for longer expeditions"
  ],
  warnings: [
    "High stress environment detected",
    "Enemy ambushes possible in corridors"
  ],
  expectedEnemies: [
    "Bone Soldier",
    "Shadow Wisp"
  ],
  rewardPreview: [
    "Gold Coins",
    "Equipment Upgrades",
    "Experience"
  ],
  supplyLevel: "Adequate",
  provisionCost: "100 Gold",
  isEnterable: true
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

const createLiveDungeonInteractionViewModel = (): DungeonInteractionViewModel => ({
  kind: "dungeon-interaction",
  title: "Dungeon Interaction",
  dungeonName: "The Azure Lantern Expedition",
  roomType: "event",
  roomLabel: "Ancient Altar",
  roomDescription: "An ancient altar stands before you, covered in moss and faintly glowing runes. Something about it feels both inviting and dangerous.",
  progress: {
    currentRoom: 3,
    totalRooms: 9,
    roomsCleared: 2
  },
  party: [
    { id: "hero-hunter-live-01", name: "Yuan", classLabel: "Hunter", hp: "42 / 42", maxHp: "42", stress: "0", maxStress: "200" },
    { id: "hero-white-live-01", name: "Mei", classLabel: "White", hp: "41 / 41", maxHp: "41", stress: "0", maxStress: "200" }
  ],
  interactions: [
    { id: "investigate", label: "Investigate", description: "Examine the altar closely for clues or hidden mechanisms.", isAvailable: true },
    { id: "use-item", label: "Use Item", description: "Attempt to use a provision or tool on the altar.", isAvailable: true },
    { id: "pray", label: "Pray", description: "Offer a prayer at the altar. The outcome is uncertain.", isAvailable: true },
    { id: "ignore", label: "Ignore", description: "Leave the altar untouched and proceed.", isAvailable: true }
  ],
  isProceedAvailable: true,
  isRetreatAvailable: true
});

const createLiveDungeonAssistViewModel = (): DungeonAssistViewModel => ({
  kind: "dungeon-assist",
  title: "Dungeon Assist",
  dungeonName: "The Azure Lantern Expedition",
  roomNumber: 1,
  party: [
    { id: "hero-hunter-live-01", name: "Yuan", classLabel: "Hunter", hp: "42 / 42", maxHp: "42", stress: "0", maxStress: "200", level: 1, isSelected: true },
    { id: "hero-white-live-01", name: "Mei", classLabel: "White", hp: "41 / 41", maxHp: "41", stress: "0", maxStress: "200", level: 1, isSelected: false }
  ],
  selectedHeroId: "hero-hunter-live-01",
  assistActions: [
    { id: "heal-wound", label: "Heal", description: "Restore health to selected hero", iconType: "heal", isAvailable: true },
    { id: "reduce-stress", label: "Calm", description: "Reduce stress of selected hero", iconType: "calm", isAvailable: true },
    { id: "apply-buff", label: "Buff", description: "Apply a combat buff", iconType: "buff", isAvailable: false },
    { id: "remove-debuff", label: "Cleanse", description: "Remove negative status", iconType: "cleanse", isAvailable: false },
    { id: "guard-ally", label: "Guard", description: "Guard an ally", iconType: "guard", isAvailable: false }
  ],
  canContinue: false
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

const createLiveFleeResultViewModel = (): ExpeditionResultViewModel => ({
  ...createLiveResultViewModel(),
  outcome: "failure",
  summary: "The expedition party fled combat before completing the objective. Return to town and recover before trying again.",
  lootAcquired: [],
  resourcesGained: {
    gold: 0,
    supplies: -25,
    experience: 15
  }
});

const createLiveCombatViewModel = (): CombatViewModel => ({
  kind: "combat",
  title: "副本场景-人物攻击",
  round: 1,
  turnPhase: "player",
  activeHeroId: "hero-hunter-live-01",
  party: [
    {
      id: "hero-hunter-live-01",
      name: "Yuan",
      classLabel: "Hunter",
      hp: "42 / 42",
      maxHp: "42",
      stress: "0",
      maxStress: "200",
      position: 1,
      isActive: true,
      isAlive: true,
      skills: [
        { id: "skill-1", name: "Hunting Bow", description: "Ranged attack that marks the target.", target: "Enemy", hitRating: "85%", critRating: "7%", cooldown: 0, cooldownRemaining: 0 },
        { id: "skill-2", name: "Rapid Shot", description: "Fire two quick shots at the target.", target: "Enemy", hitRating: "75%", critRating: "5%", cooldown: 1, cooldownRemaining: 0 },
        { id: "skill-3", name: "Marked for Death", description: "Mark a target to take increased damage.", target: "Enemy", hitRating: "100%", critRating: "0%", cooldown: 2, cooldownRemaining: 1 },
        { id: "skill-4", name: "Batty Advice", description: "Grant a random buff to an ally.", target: "Ally", hitRating: "100%", critRating: "0%", cooldown: 3, cooldownRemaining: 0 },
        { id: "skill-5", name: "Dodge Stance", description: "Increase dodge for one turn.", target: "Self", hitRating: "100%", critRating: "0%", cooldown: 2, cooldownRemaining: 0 }
      ]
    },
    {
      id: "hero-white-live-01",
      name: "Mei",
      classLabel: "White",
      hp: "41 / 41",
      maxHp: "41",
      stress: "0",
      maxStress: "200",
      position: 2,
      isActive: false,
      isAlive: true,
      skills: [
        { id: "skill-w1", name: "Holy Light", description: "Deal light damage to an enemy.", target: "Enemy", hitRating: "90%", critRating: "3%", cooldown: 0, cooldownRemaining: 0 },
        { id: "skill-w2", name: "Heal", description: "Restore health to an ally.", target: "Ally", hitRating: "100%", critRating: "0%", cooldown: 1, cooldownRemaining: 0 },
        { id: "skill-w3", name: "Bless", description: "Increase an ally's accuracy.", target: "Ally", hitRating: "100%", critRating: "0%", cooldown: 2, cooldownRemaining: 0 },
        { id: "skill-w4", name: "Smite", description: "Heavy damage to marked targets.", target: "Enemy", hitRating: "80%", critRating: "8%", cooldown: 2, cooldownRemaining: 1 },
        { id: "skill-w5", name: "Prayer", description: "Reduce party stress.", target: "Party", hitRating: "100%", critRating: "0%", cooldown: 3, cooldownRemaining: 0 }
      ]
    }
  ],
  enemies: [
    {
      id: "enemy-moth-01",
      name: "Moth Guardian",
      hp: "120 / 150",
      maxHp: "150",
      position: 1,
      isAlive: true,
      isTargeted: true,
      size: "large"
    },
    {
      id: "enemy-larva-01",
      name: "Larva Swarm",
      hp: "30 / 30",
      maxHp: "30",
      position: 2,
      isAlive: true,
      isTargeted: false,
      size: "small"
    }
  ],
  selectedSkillId: "skill-1",
  combatLog: [
    "Round 1 begins...",
    "Yuan readies Hunting Bow.",
    "Select a target to attack."
  ],
  isPlayerTurn: true,
  canFlee: true
});

const createLiveCharacterHitCombatViewModel = (combatVm: CombatViewModel): CombatViewModel => {
  const hitHeroId = combatVm.activeHeroId;
  const hitHero = combatVm.party.find((hero) => hero.id === hitHeroId);
  const targetedEnemy = combatVm.enemies.find((enemy) => enemy.isTargeted);
  const hitDamage = "10";
  const hitLog = `${targetedEnemy?.name ?? "Enemy"} retaliates and strikes ${hitHero?.name ?? "hero"} for ${hitDamage} damage.`;

  return {
    ...combatVm,
    phase: "character-hit",
    turnPhase: "enemy",
    isPlayerTurn: false,
    selectedSkillId: undefined,
    party: combatVm.party.map((hero) =>
      hero.id === hitHeroId
        ? { ...hero, isHit: true }
        : { ...hero, isHit: false }
    ),
    enemies: combatVm.enemies.map((enemy) => ({ ...enemy, isHit: false })),
    hitTargetHeroId: hitHeroId,
    hitDamage,
    hitLog,
    combatLog: [
      ...combatVm.combatLog,
      `${hitHero?.name ?? "Hero"} attacks ${targetedEnemy?.name ?? "target"}.`,
      hitLog,
      "Acknowledge the hit before issuing the next command."
    ]
  };
};

const advanceLiveCombatTurn = (combatVm: CombatViewModel): CombatViewModel => {
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
};

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
      case "start-dungeon-select":
        this.snapshot = {
          ...this.snapshot,
          flowState: "dungeon-select",
          viewModel: createLiveDungeonSelectViewModel()
        };
        break;
      case "start-expedition-planning":
        this.snapshot = {
          ...this.snapshot,
          flowState: "expedition-planning",
          viewModel: createLiveExpeditionPlanningViewModel()
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
      case "select-plane": {
        const planningVm = this.snapshot.viewModel as ExpeditionPlanningViewModel;
        this.snapshot = {
          ...this.snapshot,
          viewModel: {
            ...planningVm,
            selectedPlaneId: intent.planeId
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
      case "toggle-planning-hero": {
        const planningVm = this.snapshot.viewModel as ExpeditionPlanningViewModel;
        const updatedSlots = planningVm.partySlots.map((slot) => {
          if (slot === null) return null;
          if (slot.heroId === intent.heroId) return null;
          return slot;
        });
        const filledCount = updatedSlots.filter((s) => s !== null).length;
        this.snapshot = {
          ...this.snapshot,
          viewModel: {
            ...planningVm,
            partySlots: updatedSlots,
            isReadyToProvision: filledCount >= 1 && filledCount <= planningVm.maxPartySize
          }
        };
        break;
      }
      case "confirm-dungeon-selection": {
        const dsVm = this.snapshot.viewModel as DungeonSelectViewModel;
        if (!dsVm.isReadyToProceed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Live: confirm-dungeon-selection rejected — selection is not ready to proceed."
          };
          break;
        }
        const selectedDungeon = dsVm.dungeons.find((d) => d.id === dsVm.selectedDungeonId);
        if (!selectedDungeon || !selectedDungeon.isAvailable) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Live: confirm-dungeon-selection rejected — selected dungeon is not available."
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
      case "proceed-to-provisioning": {
        const planningVm = this.snapshot.viewModel as ExpeditionPlanningViewModel;
        this.snapshot = {
          ...this.snapshot,
          flowState: "provisioning",
          viewModel: deriveProvisioningFromExpeditionPlanning(planningVm)
        };
        break;
      }
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
      case "confirm-provisioning": {
        const validation = canTransition(this.snapshot, intent);
        if (!validation.allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: `Live: confirm-provisioning rejected: ${validation.reason ?? "invalid transition"}.`
          };
          break;
        }
        this.snapshot = {
          ...this.snapshot,
          flowState: "dungeon-hint",
          viewModel: createLiveDungeonHintViewModel()
        };
        break;
      }
      case "accept-dungeon-hint": {
        const validation = canTransition(this.snapshot, intent);
        if (!validation.allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: `Live: accept-dungeon-hint rejected: ${validation.reason ?? "invalid transition"}.`
          };
          break;
        }
        this.snapshot = {
          ...this.snapshot,
          flowState: "expedition",
          viewModel: createLiveExpeditionViewModel()
        };
        break;
      }
      case "launch-expedition":
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Live: launch-expedition rejected from current state."
          };
          break;
        }
        this.snapshot = {
          ...this.snapshot,
          flowState: "dungeon-assist",
          viewModel: createLiveDungeonAssistViewModel()
        };
        break;
      case "enter-dungeon-assist":
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Live: enter-dungeon-assist rejected outside expedition."
          };
          break;
        }
        this.snapshot = {
          ...this.snapshot,
          flowState: "dungeon-assist",
          viewModel: createLiveDungeonAssistViewModel()
        };
        break;
      case "select-assist-hero": {
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: `Live: assist hero ${intent.heroId} rejected.`
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
            debugMessage: `Live: assist action ${intent.actionId} rejected.`
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
          debugMessage: `Live: assist action ${intent.actionId} used.`
        };
        break;
      }
      case "continue-from-dungeon":
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Live: continue-from-dungeon rejected until dungeon assist is ready."
          };
          break;
        }
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
            viewModel: createLiveCombatViewModel(),
            debugMessage: `Live: entered combat room "${targetRoom.label}".`
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
            debugMessage: `Live: combat skill ${intent.skillId} rejected: ${validation.reason ?? "invalid transition"}.`
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
      case "select-target": {
        const validation = canTransition(this.snapshot, intent);
        if (!validation.allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: `Live: combat target ${intent.enemyId} rejected: ${validation.reason ?? "invalid transition"}.`
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
              debugMessage: `Live: confirm-attack rejected: ${validation.reason ?? "invalid transition"}.`
            };
            break;
          }
        }
        if (this.snapshot.viewModel.kind === "combat") {
          this.snapshot = {
            ...this.snapshot,
            flowState: "combat",
            viewModel: createLiveCharacterHitCombatViewModel(this.snapshot.viewModel),
            debugMessage: "Live: combat resolved into character-hit phase."
          };
        }
        break;
      case "continue-from-combat":
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Live: continue-from-combat rejected: not in character-hit acknowledgement."
          };
          break;
        }
        this.snapshot = {
          ...this.snapshot,
          flowState: "result",
          viewModel: createLiveResultViewModel(),
          debugMessage: "Live: character-hit acknowledged; transitioning to result."
        };
        break;
      case "open-combat-settings":
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Live: open-combat-settings rejected outside combat."
          };
          break;
        }
        this.snapshot = {
          ...this.snapshot,
          debugMessage: "Live: combat settings intent received."
        };
        break;
      case "end-turn": {
        const validation = canTransition(this.snapshot, intent);
        if (!validation.allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: `Live: end-turn rejected: ${validation.reason ?? "invalid transition"}.`
          };
          break;
        }
        const combatVm = this.snapshot.viewModel as CombatViewModel;
        this.snapshot = {
          ...this.snapshot,
          flowState: "combat",
          viewModel: advanceLiveCombatTurn(combatVm),
          debugMessage: "Live: combat end-turn intent advanced the active hero."
        };
        break;
      }
      case "flee-combat":
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Live: flee-combat rejected outside combat."
          };
          break;
        }
        this.snapshot = {
          ...this.snapshot,
          flowState: "result",
          viewModel: createLiveFleeResultViewModel()
        };
        break;
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
          flowState: "dungeon-interaction",
          viewModel: createLiveDungeonInteractionViewModel()
        };
        break;
      }
      case "proceed-dungeon": {
        const validation = canTransition(this.snapshot, intent);
        if (!validation.allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: `Live: proceed-dungeon rejected: ${validation.reason ?? "invalid transition"}.`
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
      case "interact-room": {
        const validation = canTransition(this.snapshot, intent);
        if (!validation.allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: `Live: interact-room rejected: ${validation.reason ?? "invalid transition"}.`
          };
          break;
        }
        this.snapshot = {
          ...this.snapshot,
          debugMessage: `Live: room interaction intent received for ${intent.interactionId}.`
        };
        break;
      }
      case "retreat-dungeon": {
        const validation = canTransition(this.snapshot, intent);
        if (!validation.allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: `Live: retreat-dungeon rejected: ${validation.reason ?? "invalid transition"}.`
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
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Live: return-to-town rejected from current screen."
          };
          break;
        }
        this.snapshot = createLiveTownSnapshot();
        break;
      case "continue-from-result":
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Live: continue-from-result rejected outside result."
          };
          break;
        }
        this.snapshot = {
          ...this.snapshot,
          flowState: "return",
          viewModel: createLiveReturnViewModel()
        };
        break;
      case "resume-from-return":
        if (!canTransition(this.snapshot, intent).allowed) {
          this.snapshot = {
            ...this.snapshot,
            debugMessage: "Live: resume-from-return rejected outside return."
          };
          break;
        }
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
