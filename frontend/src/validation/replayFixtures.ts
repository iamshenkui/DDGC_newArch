import type {
  BootLoadViewModel,
  BuildingDetailViewModel,
  DdgcFrontendSnapshot,
  DungeonSettlementViewModel,
  ExpeditionSetupViewModel,
  ExpeditionResultViewModel,
  ReturnViewModel,
  FatalErrorViewModel,
  HeroDetailViewModel,
  ProvisioningViewModel,
  TownHeroSummary,
  TownViewModel,
  UnsupportedViewModel,
  FlowState,
  FrontendLifecycle
} from "../bridge/contractTypes";
import { createTownBuildingSummary } from "../town/buildingCatalog";

// Shared roster heroes used by both town, provisioning, and hero-detail fixtures.
const townHeroes: ReadonlyArray<TownHeroSummary> = [
  {
    id: "hero-hunter-01",
    name: "Shen",
    classLabel: "Hunter",
    hp: "38 / 42",
    maxHp: "42",
    health: 38,
    maxHealth: 42,
    stress: "17",
    maxStress: "200",
    level: 2,
    xp: 240,
    isWounded: true,
    isAfflicted: false,
    positiveQuirks: ["steady", "sharp_eyes"],
    negativeQuirks: ["paranoid"],
    diseases: []
  },
  {
    id: "hero-white-01",
    name: "Bai Xiu",
    classLabel: "White",
    hp: "41 / 41",
    maxHp: "41",
    health: 41,
    maxHealth: 41,
    stress: "8",
    maxStress: "200",
    level: 2,
    xp: 180,
    isWounded: false,
    isAfflicted: false,
    positiveQuirks: ["blessed"],
    negativeQuirks: ["fragile"],
    diseases: []
  },
  {
    id: "hero-black-01",
    name: "Hei Zhen",
    classLabel: "Black",
    hp: "34 / 40",
    maxHp: "40",
    health: 34,
    maxHealth: 40,
    stress: "24",
    maxStress: "200",
    level: 1,
    xp: 60,
    isWounded: true,
    isAfflicted: false,
    positiveQuirks: [],
    negativeQuirks: ["clumsy", "fearful"],
    diseases: ["red_plague"]
  }
];

export const replayTownViewModel: TownViewModel = {
  kind: "town",
  title: "城镇界面",
  campaignName: "苍灯远征",
  campaignSummary:
    "回放快照：当前战役处于城镇整备阶段，可查看名册、建筑与远征准备。",
  heroes: townHeroes,
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
  ],
  roster: townHeroes,
  gold: 1250,
  isFreshVisit: true,
  nextActionLabel: "整备远征"
};

export const replayHeroDetailViewModel: HeroDetailViewModel = {
  kind: "hero-detail",
  heroId: "hero-hunter-01",
  name: "Shen",
  classLabel: "Hunter",
  hp: "38",
  maxHp: "42",
  stress: "17",
  resolve: "3",
  resolveLabel: "Heroic",
  maxStress: "200",
  progression: {
    level: 2,
    experience: "240",
    experienceToNext: "360",
    resolveLevel: 2,
    resolveXP: "120"
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
    dmg: "8-14",
    maxHp: "42",
    crit: "7%",
    spd: "5",
    dodge: "12%"
  },
  combatSkills: [
    { name: "Hunting Bow", level: 2, description: "Ranged attack that marks the target.", target: "Enemy", hitRating: "85%", critRating: "7%" },
    { name: "Rapid Shot", level: 2, description: "Fire two quick shots at the target.", target: "Enemy", hitRating: "75%", critRating: "5%" },
    { name: "Marked for Death", level: 1, description: "Mark a target to take increased damage.", target: "Enemy", hitRating: "100%", critRating: "0%" },
    { name: "Batty Advice", level: 1, description: "Grant a random buff to an ally.", target: "Ally", hitRating: "100%", critRating: "0%" }
  ],
  campingSkills: [
    { name: "Campfire Song", level: 2, description: "Restores party stress during camp.", target: "Party", hitRating: "100%", critRating: "0%" },
    { name: "Warrior's Restore", level: 1, description: "Heal a hero during camp rest.", target: "Self", hitRating: "100%", critRating: "0%" }
  ],
  weapon: { name: "Hunter's Bow (+2)", level: 3 },
  armor: { name: "Leather Armor (+1)", level: 2 },
  positiveQuirks: ["steady", "sharp_eyes"],
  negativeQuirks: ["paranoid"],
  diseases: [],
  isWounded: true,
  isAfflicted: false,
  heroDescription: "An expert hunter with keen eyes and deadly aim.",
  talent: "Natural Marksman"
};

export const replayBuildingDetailViewModel: BuildingDetailViewModel = {
  kind: "building-detail",
  buildingId: "guild",
  label: "试炼场",
  status: "ready",
  description: "The guild provides skill training and party capability review. Upgrade your heroes' abilities to better face the challenges ahead.",
  actions: [
    {
      id: "train-combat",
      label: "Train Combat Skill",
      description: "Improve a hero's combat skill proficiency.",
      cost: "200 Gold",
      isAvailable: true,
      isUnsupported: false
    },
    {
      id: "train-camping",
      label: "Train Camping Skill",
      description: "Enhance a hero's camping skill for better rest and recovery.",
      cost: "150 Gold",
      isAvailable: true,
      isUnsupported: false
    },
    {
      id: "upgrade-weapon",
      label: "Upgrade Weapon",
      description: "Enhance a hero's weapon to deal more damage.",
      cost: "300 Gold",
      isAvailable: false,
      isUnsupported: false
    },
    {
      id: "upgrade-armor",
      label: "Upgrade Armor",
      description: "Improve a hero's armor for better protection.",
      cost: "300 Gold",
      isAvailable: false,
      isUnsupported: false
    },
    {
      id: "rare-recruit",
      label: "Rare Hero Recruitment",
      description: "Access the rare hero recruitment pool.",
      cost: "1000 Gold",
      isAvailable: false,
      isUnsupported: true
    }
  ],
  upgradeRequirement: "Reach Town Level 2 to unlock weapon and armor upgrades."
};

export const replayBlacksmithBuildingDetailViewModel: BuildingDetailViewModel = {
  kind: "building-detail",
  buildingId: "blacksmith",
  label: "锻造舱",
  status: "ready",
  description: "The blacksmith forges and upgrades weapons and armor. Enhance your heroes' equipment to improve their combat effectiveness.",
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
    },
    {
      id: "masterwork-forge",
      label: "Masterwork Forge",
      description: "Commission a masterwork quality weapon for a hero.",
      cost: "1500 Gold",
      isAvailable: false,
      isUnsupported: true
    }
  ],
  currentUpgrade: "Forge Level 2",
  upgradeRequirement: "Reach Town Level 3 to unlock armor upgrades."
};

export const replaySanitariumBuildingDetailViewModel: BuildingDetailViewModel = {
  kind: "building-detail",
  buildingId: "sanitarium",
  label: "细胞修复站",
  status: "ready",
  description: "The sanitarium provides treatment for physical and mental afflictions. Cure diseases, reduce stress, and remove negative quirks.",
  actions: [
    {
      id: "cure-disease",
      label: "Cure Disease",
      description: "Treat a hero's diseases and restore their health.",
      cost: "250 Gold",
      isAvailable: true,
      isUnsupported: false
    },
    {
      id: "reduce-stress",
      label: "Stress Treatment",
      description: "Provide therapy to reduce a hero's stress level.",
      cost: "300 Gold",
      isAvailable: true,
      isUnsupported: false
    },
    {
      id: "remove-quirk",
      label: "Remove Negative Quirk",
      description: "Remove a negative quirk from a hero's profile.",
      cost: "500 Gold",
      isAvailable: false,
      isUnsupported: false
    }
  ],
  upgradeRequirement: "Reach Town Level 2 to unlock quirk removal."
};

export const replayStagecoachBuildingDetailViewModel: BuildingDetailViewModel = {
  kind: "building-detail",
  buildingId: "stagecoach",
  label: "次元感知塔",
  status: "ready",
  description: "The stagecoach brings new heroes to town. Recruit adventurers to expand your party roster and fill gaps in your expedition team.",
  actions: [
    {
      id: "recruit-hero",
      label: "Recruit Hero",
      description: "Recruit a new hero from the available pool to join your roster.",
      cost: "500 Gold",
      isAvailable: true,
      isUnsupported: false
    },
    {
      id: "dismiss-hero",
      label: "Dismiss Hero",
      description: "Release a hero from your roster to make room for new recruits.",
      cost: "0 Gold",
      isAvailable: true,
      isUnsupported: false
    },
    {
      id: "rare-recruit",
      label: "Rare Hero Recruitment",
      description: "Access the rare hero recruitment pool for exceptional adventurers.",
      cost: "1500 Gold",
      isAvailable: false,
      isUnsupported: true
    }
  ],
  upgradeRequirement: "Reach Town Level 2 to unlock rare recruitment."
};

export const replayAbbeyBuildingDetailViewModel: BuildingDetailViewModel = {
  kind: "building-detail",
  buildingId: "abbey",
  label: "信仰祭坛",
  status: "ready",
  description: "通过祈祷与仪式降低英雄压力，恢复精神状态。",
  actions: [
    {
      id: "pray",
      label: "祈祷",
      description: "进行一次祈祷仪式，降低英雄的压力值。",
      cost: "100 Gold",
      isAvailable: true,
      isUnsupported: false
    },
    {
      id: "meditate",
      label: "冥想",
      description: "深入冥想以恢复精神并消除负面状态。",
      cost: "200 Gold",
      isAvailable: false,
      isUnsupported: false
    }
  ],
  upgradeRequirement: "Reach Town Level 2 to unlock meditation."
};

export const replayTavernBuildingDetailViewModel: BuildingDetailViewModel = {
  kind: "building-detail",
  buildingId: "tavern",
  label: "迷情乐园",
  status: "ready",
  description: "通过酒馆活动缓解压力并恢复状态，在轻松的氛围中重整队伍。",
  actions: [
    {
      id: "drink",
      label: "畅饮",
      description: "在酒馆畅饮一番，大幅降低压力但可能带来随机效果。",
      cost: "150 Gold",
      isAvailable: true,
      isUnsupported: false
    },
    {
      id: "gamble",
      label: "博弈",
      description: "参与酒馆博弈活动，有机会获得额外金币。",
      cost: "50 Gold",
      isAvailable: true,
      isUnsupported: false
    }
  ]
};

export const replayGraveyardBuildingDetailViewModel: BuildingDetailViewModel = {
  kind: "building-detail",
  buildingId: "graveyard",
  label: "英雄档案馆",
  status: "partial",
  description: "查看阵亡英雄记录与历史档案，缅怀逝去的战士。",
  actions: [
    {
      id: "view-records",
      label: "查看档案",
      description: "浏览阵亡英雄的历史记录与战斗数据。",
      cost: "0 Gold",
      isAvailable: true,
      isUnsupported: false
    }
  ]
};

export const replayGardenBuildingDetailViewModel: BuildingDetailViewModel = {
  kind: "building-detail",
  buildingId: "garden",
  label: "天国花园",
  status: "partial",
  description: "提供特殊休整与恢复服务，在宁静的花园中治愈身心。",
  actions: [
    {
      id: "rest",
      label: "休整",
      description: "在花园中休整，恢复英雄生命值并降低压力。",
      cost: "200 Gold",
      isAvailable: true,
      isUnsupported: false
    }
  ],
  upgradeRequirement: "Reach Town Level 2 to unlock garden upgrades."
};

export const replayLegacyTowerBuildingDetailViewModel: BuildingDetailViewModel = {
  kind: "building-detail",
  buildingId: "legacytower",
  label: "维度灯塔",
  status: "partial",
  description: "查看传承与博物馆式收藏内容，回顾战役历程与成就。",
  actions: [
    {
      id: "view-legacy",
      label: "查看传承",
      description: "浏览已解锁的传承物品与战役成就。",
      cost: "0 Gold",
      isAvailable: true,
      isUnsupported: false
    },
    {
      id: "claim-reward",
      label: "领取奖励",
      description: "领取传承里程碑奖励。",
      cost: "0 Gold",
      isAvailable: false,
      isUnsupported: false
    }
  ]
};

export const replayMarketBuildingDetailViewModel: BuildingDetailViewModel = {
  kind: "building-detail",
  buildingId: "market",
  label: "交易市场",
  status: "partial",
  description: "购买补给、物资与商店类服务，为远征做好物资准备。",
  actions: [
    {
      id: "buy-supplies",
      label: "购买补给",
      description: "购买远征所需的各类补给品和物资。",
      cost: "100 Gold",
      isAvailable: true,
      isUnsupported: false
    },
    {
      id: "buy-trinket",
      label: "购买饰品",
      description: "浏览并购买英雄可装备的饰品。",
      cost: "300 Gold",
      isAvailable: false,
      isUnsupported: false
    }
  ],
  upgradeRequirement: "Reach Town Level 2 to unlock trinket shop."
};

export const replayCampingTrainerBuildingDetailViewModel: BuildingDetailViewModel = {
  kind: "building-detail",
  buildingId: "campingtrainer",
  label: "空间分析",
  status: "partial",
  description: "营火与露营训练相关服务，提升队伍在远征中的生存能力。",
  actions: [
    {
      id: "train-camping",
      label: "露营训练",
      description: "训练英雄的露营技能，提高远征中的恢复效果。",
      cost: "150 Gold",
      isAvailable: true,
      isUnsupported: false
    }
  ],
  upgradeRequirement: "Reach Town Level 2 to unlock advanced camping skills."
};

export const replayProvisioningViewModel: ProvisioningViewModel = {
  kind: "provisioning",
  title: "Provision Expedition",
  campaignName: "The Azure Lantern",
  expeditionLabel: "The Depths Await",
  expeditionSummary: "Assemble your party and provision wisely. The expedition awaits those who dare enter.",
  party: [
    { id: "hero-hunter-01", name: "Shen", classLabel: "Hunter", hp: "38 / 42", maxHp: "42", health: 38, maxHealth: 42, stress: "17", maxStress: "200", level: 2, xp: 240, isWounded: true, isAfflicted: false, isSelected: true },
    { id: "hero-white-01", name: "Bai Xiu", classLabel: "White", hp: "41 / 41", maxHp: "41", health: 41, maxHealth: 41, stress: "8", maxStress: "200", level: 2, xp: 180, isWounded: false, isAfflicted: false, isSelected: true },
    { id: "hero-black-01", name: "Hei Zhen", classLabel: "Black", hp: "34 / 40", maxHp: "40", health: 34, maxHealth: 40, stress: "24", maxStress: "200", level: 1, xp: 60, isWounded: true, isAfflicted: false, isSelected: false }
  ],
  maxPartySize: 4,
  isReadyToLaunch: true,
  supplyLevel: "Adequate",
  provisionCost: "150 Gold"
};

export const replayExpeditionViewModel: ExpeditionSetupViewModel = {
  kind: "expedition",
  title: "Expedition Launch",
  expeditionName: "The Depths Await",
  partySize: 2,
  party: [
    { id: "hero-hunter-01", name: "Shen", classLabel: "Hunter", hp: "38 / 42", maxHp: "42", stress: "17", maxStress: "200" },
    { id: "hero-white-01", name: "Bai Xiu", classLabel: "White", hp: "41 / 41", maxHp: "41", stress: "8", maxStress: "200" }
  ],
  difficulty: "Challenging",
  estimatedDuration: "Medium",
  objectives: [
    "Explore the dungeon level",
    "Collect resources",
    "Return with treasures"
  ],
  warnings: [
    "Elevated enemy presence detected",
    "Limited camping opportunities"
  ],
  supplyLevel: "Adequate",
  provisionCost: "150 Gold",
  isLaunchable: true
};

export const replayDungeonSettlementViewModel: DungeonSettlementViewModel = {
  kind: "dungeon-settlement",
  title: "副本结算",
  dungeonName: "深渊试炼",
  outcome: "victory",
  summary: "队伍成功完成了副本探索，收集到了珍贵的奖励与宝藏。",
  rewardsLabel: "收集的奖励",
  rewardSegments: [
    { label: "战斗", value: 85, max: 100, color: "#ea7767" },
    { label: "探索", value: 60, max: 100, color: "#5bbd6e" },
    { label: "宝藏", value: 40, max: 100, color: "#e8a838" },
    { label: "特殊", value: 20, max: 100, color: "#6b9bc7" }
  ],
  treasuresLabel: "收集的宝藏",
  goldCollected: 480,
  heirloomsLabel: "收集的传家宝",
  heirlooms: [
    { type: "画像", count: 0 },
    { type: "徽章", count: 0 },
    { type: "文献", count: 0 },
    { type: "遗物", count: 0 }
  ],
  partyOutcomes: [
    {
      heroId: "hero-hunter-01",
      heroName: "Shen",
      classLabel: "Hunter",
      status: "alive",
      level: 2,
      xpGained: 120
    },
    {
      heroId: "hero-white-01",
      heroName: "Bai Xiu",
      classLabel: "White",
      status: "alive",
      level: 2,
      xpGained: 120
    },
    {
      heroId: "hero-black-01",
      heroName: "Hei Zhen",
      classLabel: "Black",
      status: "wounded",
      level: 1,
      xpGained: 80
    }
  ],
  isContinueAvailable: true
};

export const replayResultViewModel: ExpeditionResultViewModel = {
  kind: "result",
  title: "Expedition Complete",
  expeditionName: "The Depths Await",
  outcome: "success",
  summary: "Your party has returned victorious from the expedition. The depths have been conquered and valuable treasures have been recovered.",
  lootAcquired: [
    "Ancient Gold Coin x3",
    "Mysterious Gemstone",
    "Forgotten Relic"
  ],
  heroOutcomes: [
    {
      heroId: "hero-hunter-01",
      heroName: "Shen",
      classLabel: "Hunter",
      status: "alive",
      hpChange: "-4",
      stressChange: "+12"
    },
    {
      heroId: "hero-white-01",
      heroName: "Bai Xiu",
      classLabel: "White",
      status: "alive",
      hpChange: "-8",
      stressChange: "+8"
    }
  ],
  resourcesGained: {
    gold: 250,
    supplies: -50,
    experience: 180
  },
  isContinueAvailable: true
};

export const replayFailureResultViewModel: ExpeditionResultViewModel = {
  kind: "result",
  title: "Expedition Failed",
  expeditionName: "The Depths Await",
  outcome: "failure",
  summary: "Your expedition has been utterly defeated. The party was overwhelmed and forced to retreat in disarray.",
  lootAcquired: [],
  heroOutcomes: [
    {
      heroId: "hero-hunter-01",
      heroName: "Shen",
      classLabel: "Hunter",
      status: "alive",
      hpChange: "-18",
      stressChange: "+25"
    },
    {
      heroId: "hero-white-01",
      heroName: "Bai Xiu",
      classLabel: "White",
      status: "dead",
      hpChange: "-41",
      stressChange: "+40"
    }
  ],
  resourcesGained: {
    gold: 0,
    supplies: -100,
    experience: 50
  },
  isContinueAvailable: true
};

export const replayPartialResultViewModel: ExpeditionResultViewModel = {
  kind: "result",
  title: "Expedition Partial Success",
  expeditionName: "The Depths Await",
  outcome: "partial",
  summary: "Your party returned with mixed results. Some objectives were achieved but at significant cost.",
  lootAcquired: [
    "Ancient Gold Coin x1"
  ],
  heroOutcomes: [
    {
      heroId: "hero-hunter-01",
      heroName: "Shen",
      classLabel: "Hunter",
      status: "alive",
      hpChange: "-12",
      stressChange: "+18"
    },
    {
      heroId: "hero-white-01",
      heroName: "Bai Xiu",
      classLabel: "White",
      status: "stressed",
      hpChange: "-15",
      stressChange: "+22"
    }
  ],
  resourcesGained: {
    gold: 80,
    supplies: -75,
    experience: 100
  },
  isContinueAvailable: true
};

export const replayReturnViewModel: ReturnViewModel = {
  kind: "return",
  title: "Returning to Town",
  expeditionName: "The Depths Await",
  summary: "The expedition party has returned to town. Review your heroes' conditions and prepare for future expeditions.",
  returningHeroes: [
    {
      heroId: "hero-hunter-01",
      heroName: "Shen",
      classLabel: "Hunter",
      hp: "34 / 42",
      stress: "29"
    },
    {
      heroId: "hero-white-01",
      heroName: "Bai Xiu",
      classLabel: "White",
      hp: "33 / 41",
      stress: "16"
    }
  ],
  isTownResumeAvailable: true
};

export const replayReadySnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "town",
  viewModel: replayTownViewModel,
  debugMessage: "Replay bridge loaded the representative town/meta snapshot."
};

export const replayHeroDetailSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "town",
  viewModel: replayHeroDetailViewModel,
  debugMessage: "Replay bridge showing hero detail for inspection."
};

export const replayBuildingDetailSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "town",
  viewModel: replayBuildingDetailViewModel,
  debugMessage: "Replay bridge showing building detail for interaction."
};

export const replayBlacksmithBuildingSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "town",
  viewModel: replayBlacksmithBuildingDetailViewModel,
  debugMessage: "Replay bridge showing blacksmith building detail."
};

export const replaySanitariumBuildingSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "town",
  viewModel: replaySanitariumBuildingDetailViewModel,
  debugMessage: "Replay bridge showing sanitarium building detail."
};

export const replayStagecoachBuildingSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "town",
  viewModel: replayStagecoachBuildingDetailViewModel,
  debugMessage: "Replay bridge showing stagecoach building detail."
};

export const replayAbbeyBuildingSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "town",
  viewModel: replayAbbeyBuildingDetailViewModel,
  debugMessage: "Replay bridge showing abbey building detail."
};

export const replayTavernBuildingSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "town",
  viewModel: replayTavernBuildingDetailViewModel,
  debugMessage: "Replay bridge showing tavern building detail."
};

export const replayGraveyardBuildingSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "town",
  viewModel: replayGraveyardBuildingDetailViewModel,
  debugMessage: "Replay bridge showing graveyard building detail."
};

export const replayGardenBuildingSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "town",
  viewModel: replayGardenBuildingDetailViewModel,
  debugMessage: "Replay bridge showing garden building detail."
};

export const replayLegacyTowerBuildingSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "town",
  viewModel: replayLegacyTowerBuildingDetailViewModel,
  debugMessage: "Replay bridge showing legacy tower building detail."
};

export const replayMarketBuildingSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "town",
  viewModel: replayMarketBuildingDetailViewModel,
  debugMessage: "Replay bridge showing market building detail."
};

export const replayCampingTrainerBuildingSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "town",
  viewModel: replayCampingTrainerBuildingDetailViewModel,
  debugMessage: "Replay bridge showing camping trainer building detail."
};

const unsupportedViewModel: UnsupportedViewModel = {
  kind: "unsupported",
  title: "Live Runtime Not Wired Yet",
  reason:
    "Phase 10 scaffold boots replay mode first. Live bridge wiring stays behind the RuntimeBridge seam."
};

export const unsupportedSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "unsupported",
  flowState: "load",
  viewModel: unsupportedViewModel,
  debugMessage: "Live runtime bridge is intentionally stubbed until the replay shell is stable."
};

const fatalViewModel: FatalErrorViewModel = {
  kind: "fatal",
  title: "Frontend Contract Drift",
  reason:
    "Use this surface when runtime/view-model schemas drift or required assets cannot be resolved safely."
};

export const fatalSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "fatal",
  flowState: "boot",
  viewModel: fatalViewModel,
  debugMessage: "Fatal fallback fixture."
};

const replayLoadingViewModel: BootLoadViewModel = {
  kind: "boot-load",
  title: "Loading Replay Shell",
  summary: "Initializing the DDGC replay runtime and loading fixture data.",
  mode: "replay"
};

export const replayLoadingSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "loading",
  flowState: "load",
  viewModel: replayLoadingViewModel,
  debugMessage: "Replay runtime is loading fixture data and initializing the game state."
};

const liveLoadingViewModel: BootLoadViewModel = {
  kind: "boot-load",
  title: "Loading Live Shell",
  summary: "Connecting to the DDGC live runtime bridge and bootstrapping campaign state.",
  mode: "live"
};

export const liveLoadingSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "loading",
  flowState: "load",
  viewModel: liveLoadingViewModel,
  debugMessage: "Live runtime is establishing the bridge connection and loading campaign state."
};

// ── Startup, Provisioning, Expedition, Result, and Return snapshots ─────────────

// Startup screen fixture: FlowController resolves "startup" when lifecycle=ready, flowState=boot.
// StartupScreen uses direct callbacks rather than a view model, so BootLoadViewModel is used
// as a placeholder in the snapshot structure (the viewModel is not consumed by StartupScreen).
const startupViewModel: BootLoadViewModel = {
  kind: "boot-load",
  title: "DDGC Rendered Frontend",
  summary: "Boot the product-owned frontend shell through replay mode first.",
  mode: "replay"
};

export const startupSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "boot",
  viewModel: startupViewModel,
  debugMessage: "Startup screen fixture - ready to boot into replay or live mode."
};

// Provisioning flow snapshot
export const provisioningSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "provisioning",
  viewModel: replayProvisioningViewModel,
  debugMessage: "Replay bridge showing provisioning screen."
};

// Expedition launch flow snapshot
export const expeditionSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "expedition",
  viewModel: replayExpeditionViewModel,
  debugMessage: "Replay bridge showing expedition launch screen."
};

// Dungeon settlement snapshot
export const dungeonSettlementSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "dungeon-settlement",
  viewModel: replayDungeonSettlementViewModel,
  debugMessage: "Replay bridge showing dungeon settlement screen."
};

// Result snapshots (success, failure, partial)
export const resultSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "result",
  viewModel: replayResultViewModel,
  debugMessage: "Replay bridge showing successful result screen."
};

export const failureResultSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "result",
  viewModel: replayFailureResultViewModel,
  debugMessage: "Replay bridge showing failure result screen."
};

export const partialResultSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "result",
  viewModel: replayPartialResultViewModel,
  debugMessage: "Replay bridge showing partial result screen."
};

// Return flow snapshot
export const returnSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "return",
  viewModel: replayReturnViewModel,
  debugMessage: "Replay bridge showing return screen."
};

// ── Snapshot contract validation ───────────────────────────────────────────────

/**
 * Validate a DdgcFrontendSnapshot against the contract boundary.
 * Returns an array of error messages; empty array means valid.
 * Each error pinpoints a specific contract violation for actionable debugging.
 */
export function validateSnapshotContract(snapshot: DdgcFrontendSnapshot): string[] {
  const errors: string[] = [];

  // Lifecycle must be a valid FrontendLifecycle
  const validLifecycles: FrontendLifecycle[] = ["booting", "loading", "ready", "unsupported", "fatal"];
  if (!validLifecycles.includes(snapshot.lifecycle as FrontendLifecycle)) {
    errors.push(
      `lifecycle "${String(snapshot.lifecycle)}" is not a valid FrontendLifecycle. ` +
      `Expected one of: ${validLifecycles.join(", ")}`
    );
  }

  // FlowState must be a valid FlowState
  const validFlowStates: FlowState[] = ["boot", "load", "town", "provisioning", "expedition", "combat", "dungeon-settlement", "result", "return"];
  if (!validFlowStates.includes(snapshot.flowState as FlowState)) {
    errors.push(
      `flowState "${String(snapshot.flowState)}" is not a valid FlowState. ` +
      `Expected one of: ${validFlowStates.join(", ")}`
    );
  }

  // viewModel must be a non-null object with a kind property
  const vm: Record<string, unknown> | undefined = snapshot.viewModel as unknown as Record<string, unknown> | undefined;
  if (!vm || typeof vm !== "object" || Array.isArray(vm)) {
    errors.push("viewModel is missing, null, or not an object");
    return errors;
  }
  if (typeof vm.kind !== "string" || !vm.kind) {
    errors.push("viewModel.kind is missing or not a non-empty string");
    return errors;
  }

  // Type discrimination: lifecycle/flowState must align with viewModel.kind
  errors.push(...validateKindDiscrimination(snapshot.lifecycle as string, snapshot.flowState as string, vm.kind));

  // Per-kind required field validation
  errors.push(...validateRequiredFields(vm.kind, vm));

  return errors;
}

function validateKindDiscrimination(lifecycle: string, flowState: string, kind: string): string[] {
  const e: string[] = [];

  // Lifecycle-based rules take precedence over flowState rules.
  // When lifecycle is fatal/unsupported/loading/booting, flowState is secondary.
  const lifecycleKindMap: Record<string, string> = {
    fatal: "fatal",
    unsupported: "unsupported",
    loading: "boot-load",
    booting: "boot-load",
  };
  const expectedKind = lifecycleKindMap[lifecycle];
  if (expectedKind !== undefined) {
    if (kind !== expectedKind) {
      e.push(`lifecycle is "${lifecycle}" but viewModel.kind is "${kind}"; expected "${expectedKind}"`);
    }
    // Lifecycle determines the kind; skip flowState-based discrimination.
    return e;
  }

  // Ready/other lifecycle: flowState determines valid viewModel kinds.
  const flowStateKindMap: Record<string, string[]> = {
    boot: ["boot-load"],
    load: ["boot-load"],
    town: ["town", "hero-detail", "building-detail"],
    provisioning: ["provisioning"],
    expedition: ["expedition"],
    combat: ["expedition"],
    "dungeon-settlement": ["dungeon-settlement"],
    result: ["result"],
    return: ["return"],
  };
  const allowedKinds = flowStateKindMap[flowState];
  if (allowedKinds && !allowedKinds.includes(kind)) {
    e.push(`flowState is "${flowState}" but viewModel.kind is "${kind}"; expected one of: ${allowedKinds.join(", ")}`);
  }

  return e;
}

function validateRequiredFields(kind: string, vm: Record<string, unknown>): string[] {
  const e: string[] = [];

  switch (kind) {
    case "boot-load": {
      if (!vm.title || typeof vm.title !== "string") e.push("BootLoadViewModel: title is missing or not a string");
      if (!vm.summary || typeof vm.summary !== "string") e.push("BootLoadViewModel: summary is missing or not a string");
      if (vm.mode !== "replay" && vm.mode !== "live") e.push(`BootLoadViewModel: mode is "${String(vm.mode)}", expected "replay" or "live"`);
      break;
    }
    case "town": {
      if (!vm.title || typeof vm.title !== "string") e.push("TownViewModel: title is missing");
      if (!Array.isArray(vm.heroes)) { e.push("TownViewModel: heroes is not an array"); } else if (vm.heroes.length === 0) { e.push("TownViewModel: heroes array is empty"); }
      if (!Array.isArray(vm.buildings)) { e.push("TownViewModel: buildings is not an array"); } else if (vm.buildings.length === 0) { e.push("TownViewModel: buildings array is empty"); }
      if (!Array.isArray(vm.roster)) e.push("TownViewModel: roster is not an array");
      if (typeof vm.gold !== "number") e.push("TownViewModel: gold is not a number");
      if (typeof vm.isFreshVisit !== "boolean") e.push("TownViewModel: isFreshVisit is not a boolean");
      if (!vm.campaignName || typeof vm.campaignName !== "string") e.push("TownViewModel: campaignName is missing");
      if (vm.nextActionLabel === undefined || typeof vm.nextActionLabel !== "string") e.push("TownViewModel: nextActionLabel is missing or not a string");
      break;
    }
    case "hero-detail": {
      if (!vm.heroId || typeof vm.heroId !== "string") e.push("HeroDetailViewModel: heroId is missing");
      if (!vm.name || typeof vm.name !== "string") e.push("HeroDetailViewModel: name is missing");
      if (!vm.classLabel || typeof vm.classLabel !== "string") e.push("HeroDetailViewModel: classLabel is missing");
      if (!vm.hp || typeof vm.hp !== "string") e.push("HeroDetailViewModel: hp is missing");
      if (!vm.maxHp || typeof vm.maxHp !== "string") e.push("HeroDetailViewModel: maxHp is missing");
      if (!vm.stress || typeof vm.stress !== "string") e.push("HeroDetailViewModel: stress is missing");
      if (!vm.resolve || typeof vm.resolve !== "string") e.push("HeroDetailViewModel: resolve is missing");
      if (!vm.progression || typeof vm.progression !== "object") e.push("HeroDetailViewModel: progression is missing");
      if (!vm.resistances || typeof vm.resistances !== "object") e.push("HeroDetailViewModel: resistances is missing");
      if (!Array.isArray(vm.combatSkills)) e.push("HeroDetailViewModel: combatSkills is not an array");
      if (!Array.isArray(vm.campingSkills)) e.push("HeroDetailViewModel: campingSkills is not an array");
      if (!vm.weapon || typeof vm.weapon !== "object" || typeof (vm.weapon as Record<string,unknown>).name !== "string") e.push("HeroDetailViewModel: weapon is missing or not an EquipmentItem");
      if (!vm.armor || typeof vm.armor !== "object" || typeof (vm.armor as Record<string,unknown>).name !== "string") e.push("HeroDetailViewModel: armor is missing or not an EquipmentItem");
      break;
    }
    case "building-detail": {
      if (!vm.buildingId || typeof vm.buildingId !== "string") e.push("BuildingDetailViewModel: buildingId is missing");
      if (!vm.label || typeof vm.label !== "string") e.push("BuildingDetailViewModel: label is missing");
      if (!["ready", "partial", "locked"].includes(vm.status as string)) e.push(`BuildingDetailViewModel: status is "${String(vm.status)}", expected "ready", "partial", or "locked"`);
      if (!vm.description || typeof vm.description !== "string") e.push("BuildingDetailViewModel: description is missing");
      if (!Array.isArray(vm.actions)) { e.push("BuildingDetailViewModel: actions is not an array"); } else if (vm.actions.length === 0) { e.push("BuildingDetailViewModel: actions array is empty"); }
      break;
    }
    case "provisioning": {
      if (!vm.title || typeof vm.title !== "string") e.push("ProvisioningViewModel: title is missing");
      if (!Array.isArray(vm.party)) { e.push("ProvisioningViewModel: party is not an array"); } else if (vm.party.length === 0) { e.push("ProvisioningViewModel: party array is empty"); }
      if (typeof vm.maxPartySize !== "number") e.push("ProvisioningViewModel: maxPartySize is not a number");
      if (typeof vm.isReadyToLaunch !== "boolean") e.push("ProvisioningViewModel: isReadyToLaunch is not a boolean");
      if (!vm.supplyLevel || typeof vm.supplyLevel !== "string") e.push("ProvisioningViewModel: supplyLevel is missing");
      if (!vm.provisionCost || typeof vm.provisionCost !== "string") e.push("ProvisioningViewModel: provisionCost is missing");
      if (!vm.campaignName || typeof vm.campaignName !== "string") e.push("ProvisioningViewModel: campaignName is missing");
      if (!vm.expeditionLabel || typeof vm.expeditionLabel !== "string") e.push("ProvisioningViewModel: expeditionLabel is missing");
      break;
    }
    case "expedition": {
      if (!vm.title || typeof vm.title !== "string") e.push("ExpeditionSetupViewModel: title is missing");
      if (!vm.expeditionName || typeof vm.expeditionName !== "string") e.push("ExpeditionSetupViewModel: expeditionName is missing");
      if (typeof vm.partySize !== "number") e.push("ExpeditionSetupViewModel: partySize is not a number");
      if (!Array.isArray(vm.party)) { e.push("ExpeditionSetupViewModel: party is not an array"); } else if (vm.party.length === 0) { e.push("ExpeditionSetupViewModel: party array is empty"); }
      if (!vm.difficulty || typeof vm.difficulty !== "string") e.push("ExpeditionSetupViewModel: difficulty is missing");
      if (!Array.isArray(vm.objectives)) { e.push("ExpeditionSetupViewModel: objectives is not an array"); } else if (vm.objectives.length === 0) { e.push("ExpeditionSetupViewModel: objectives array is empty"); }
      if (typeof vm.isLaunchable !== "boolean") e.push("ExpeditionSetupViewModel: isLaunchable is not a boolean");
      if (!vm.supplyLevel || typeof vm.supplyLevel !== "string") e.push("ExpeditionSetupViewModel: supplyLevel is missing");
      if (!vm.provisionCost || typeof vm.provisionCost !== "string") e.push("ExpeditionSetupViewModel: provisionCost is missing");
      break;
    }
    case "dungeon-settlement": {
      if (!vm.title || typeof vm.title !== "string") e.push("DungeonSettlementViewModel: title is missing");
      if (!vm.dungeonName || typeof vm.dungeonName !== "string") e.push("DungeonSettlementViewModel: dungeonName is missing");
      if (vm.outcome !== "victory" && vm.outcome !== "defeat" && vm.outcome !== "retreat") e.push(`DungeonSettlementViewModel: outcome is "${String(vm.outcome)}", expected "victory", "defeat", or "retreat"`);
      if (!vm.summary || typeof vm.summary !== "string") e.push("DungeonSettlementViewModel: summary is missing");
      if (!Array.isArray(vm.rewardSegments)) e.push("DungeonSettlementViewModel: rewardSegments is not an array");
      if (typeof vm.goldCollected !== "number") e.push("DungeonSettlementViewModel: goldCollected is not a number");
      if (!Array.isArray(vm.heirlooms)) e.push("DungeonSettlementViewModel: heirlooms is not an array");
      if (!Array.isArray(vm.partyOutcomes)) { e.push("DungeonSettlementViewModel: partyOutcomes is not an array"); } else if (vm.partyOutcomes.length === 0) { e.push("DungeonSettlementViewModel: partyOutcomes array is empty"); }
      if (typeof vm.isContinueAvailable !== "boolean") e.push("DungeonSettlementViewModel: isContinueAvailable is not a boolean");
      break;
    }
    case "result": {
      if (!vm.title || typeof vm.title !== "string") e.push("ExpeditionResultViewModel: title is missing");
      if (vm.outcome !== "success" && vm.outcome !== "failure" && vm.outcome !== "partial") e.push(`ExpeditionResultViewModel: outcome is "${String(vm.outcome)}", expected "success", "failure", or "partial"`);
      if (!vm.summary || typeof vm.summary !== "string") e.push("ExpeditionResultViewModel: summary is missing");
      if (!Array.isArray(vm.heroOutcomes)) { e.push("ExpeditionResultViewModel: heroOutcomes is not an array"); } else if (vm.heroOutcomes.length === 0) { e.push("ExpeditionResultViewModel: heroOutcomes array is empty"); }
      if (!vm.resourcesGained || typeof vm.resourcesGained !== "object") e.push("ExpeditionResultViewModel: resourcesGained is missing");
      if (typeof vm.isContinueAvailable !== "boolean") e.push("ExpeditionResultViewModel: isContinueAvailable is not a boolean");
      break;
    }
    case "return": {
      if (!vm.title || typeof vm.title !== "string") e.push("ReturnViewModel: title is missing");
      if (!vm.expeditionName || typeof vm.expeditionName !== "string") e.push("ReturnViewModel: expeditionName is missing");
      if (!vm.summary || typeof vm.summary !== "string") e.push("ReturnViewModel: summary is missing");
      if (!Array.isArray(vm.returningHeroes)) { e.push("ReturnViewModel: returningHeroes is not an array"); } else if (vm.returningHeroes.length === 0) { e.push("ReturnViewModel: returningHeroes array is empty"); }
      if (typeof vm.isTownResumeAvailable !== "boolean") e.push("ReturnViewModel: isTownResumeAvailable is not a boolean");
      break;
    }
    case "fatal": {
      if (!vm.title || typeof vm.title !== "string") e.push("FatalErrorViewModel: title is missing");
      if (!vm.reason || typeof vm.reason !== "string") e.push("FatalErrorViewModel: reason is missing");
      break;
    }
    case "unsupported": {
      if (!vm.title || typeof vm.title !== "string") e.push("UnsupportedViewModel: title is missing");
      if (!vm.reason || typeof vm.reason !== "string") e.push("UnsupportedViewModel: reason is missing");
      break;
    }
    default: {
      e.push(`Unknown viewModel.kind: "${kind}"`);
      break;
    }
  }

  return e;
}