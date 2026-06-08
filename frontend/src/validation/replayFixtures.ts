import type {
  BootLoadViewModel,
  BuildingDetailViewModel,
  DungeonInteractionViewModel,
  DdgcFrontendSnapshot,
  DungeonAssistViewModel,
  DungeonSelectViewModel,
  ExpeditionPlanningViewModel,
  ExpeditionSetupViewModel,
  DungeonMapViewModel,
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

export const replayDungeonSelectViewModel: DungeonSelectViewModel = {
  kind: "dungeon-select",
  title: "副本选择人物",
  campaignName: "苍灯远征",
  selectedDungeonId: null,
  dungeons: [
    {
      id: "dungeon-ruins-01",
      name: "废墟遗迹",
      description: "古老的废墟中隐藏着危险的敌人和珍贵的宝藏。适合新手探险者磨练技艺。",
      difficulty: "简单",
      estimatedDuration: "短",
      recommendedLevel: 1,
      provisionCost: "100 Gold",
      supplyLevel: "基础",
      rewards: ["古金币", "初级装备", "经验值"],
      isAvailable: true
    },
    {
      id: "dungeon-forest-01",
      name: "迷雾森林",
      description: "被浓雾笼罩的古老森林，里面栖息着诡异的生物。需要一定的准备才能深入。",
      difficulty: "普通",
      estimatedDuration: "中等",
      recommendedLevel: 2,
      provisionCost: "150 Gold",
      supplyLevel: "标准",
      rewards: ["神秘宝石", "中级装备", "大量经验值"],
      isAvailable: true
    },
    {
      id: "dungeon-depths-01",
      name: "深渊裂隙",
      description: "通往未知位面的裂隙，充满了极度危险的敌人。只有经验丰富的队伍才能挑战。",
      difficulty: "困难",
      estimatedDuration: "长",
      recommendedLevel: 4,
      provisionCost: "300 Gold",
      supplyLevel: "充足",
      rewards: ["传奇遗物", "高级装备", "稀有材料"],
      isAvailable: false,
      lockReason: "需要城镇等级 3"
    }
  ],
  party: [
    { id: "hero-hunter-01", name: "Shen", classLabel: "Hunter", hp: "38 / 42", maxHp: "42", health: 38, maxHealth: 42, stress: "17", maxStress: "200", level: 2, xp: 240, isWounded: true, isAfflicted: false, isSelected: false },
    { id: "hero-white-01", name: "Bai Xiu", classLabel: "White", hp: "41 / 41", maxHp: "41", health: 41, maxHealth: 41, stress: "8", maxStress: "200", level: 2, xp: 180, isWounded: false, isAfflicted: false, isSelected: false },
    { id: "hero-black-01", name: "Hei Zhen", classLabel: "Black", hp: "34 / 40", maxHp: "40", health: 34, maxHealth: 40, stress: "24", maxStress: "200", level: 1, xp: 60, isWounded: true, isAfflicted: false, isSelected: false }
  ],
  maxPartySize: 4,
  isReadyToProceed: false
};

export const replayExpeditionPlanningViewModel: ExpeditionPlanningViewModel = {
  kind: "expedition-planning",
  title: "Plane Exploration",
  campaignName: "The Azure Lantern",
  selectedPlaneId: "qinglong",
  planes: [
    {
      id: "qinglong",
      name: "青龙",
      description: "The Azure Dragon plane — ancient forests shrouded in mist, home to mantis kin and tree spirits.",
      difficulty: "Challenging",
      difficultyPips: 3,
      estimatedDuration: "Medium",
      isLocked: false,
      rewards: ["Dragon Scale", "Ancient Wood", "Mantis Essence"],
      objectives: ["Explore the forest depths", "Defeat the Azure Dragon", "Collect dragon scales"],
      themeColor: "#4a9b8e"
    },
    {
      id: "baihu",
      name: "白虎",
      description: "The White Tiger plane — fortress ruins where armored phantoms and blade spirits roam.",
      difficulty: "Hard",
      difficultyPips: 4,
      estimatedDuration: "Long",
      isLocked: false,
      rewards: ["Tiger Fang", "Steel Fragment", "Phantom Shard"],
      objectives: [" breach the fortress gates", "Defeat the White Tiger", "Recover lost artifacts"],
      themeColor: "#c9a959"
    },
    {
      id: "zhuque",
      name: "朱雀",
      description: "The Vermilion Bird plane — fire temples where ghost flames dance and fox spirits lure travelers.",
      difficulty: "Very Hard",
      difficultyPips: 5,
      estimatedDuration: "Very Long",
      isLocked: true,
      lockReason: "Complete QingLong first",
      rewards: ["Phoenix Feather", "Fire Gem", "Fox Spirit Orb"],
      objectives: ["Navigate the burning temples", "Defeat the Vermilion Bird", "Extinguish the eternal flame"],
      themeColor: "#d4563c"
    },
    {
      id: "xuanwu",
      name: "玄武",
      description: "The Black Tortoise plane — watery depths where serpents coil and frozen corpses drift.",
      difficulty: "Extreme",
      difficultyPips: 5,
      estimatedDuration: "Very Long",
      isLocked: true,
      lockReason: "Complete BaiHu first",
      rewards: ["Turtle Shell", "Ice Crystal", "Serpent Venom"],
      objectives: ["Descend into the abyss", "Defeat the Black Tortoise", "Seal the water gate"],
      themeColor: "#4a6fa5"
    }
  ],
  partySlots: [
    { heroId: "hero-hunter-01", heroName: "Shen", classLabel: "Hunter", hp: "38 / 42", stress: "17", level: 2 },
    { heroId: "hero-white-01", heroName: "Bai Xiu", classLabel: "White", hp: "41 / 41", stress: "8", level: 2 },
    null,
    null
  ],
  maxPartySize: 4,
  isReadyToProvision: true,
  provisionCost: "150 Gold"
};

export const replayProvisioningViewModel: ProvisioningViewModel = {
  kind: "provisioning",
  title: "战前补给",
  campaignName: "苍灯远征",
  expeditionLabel: "深渊探查",
  expeditionSummary: "做好出发前的准备，合理分配补给。",
  party: [
    { id: "hero-hunter-01", name: "Shen", classLabel: "Hunter", hp: "38 / 42", maxHp: "42", health: 38, maxHealth: 42, stress: "17", maxStress: "200", level: 2, xp: 240, isWounded: true, isAfflicted: false, isSelected: true },
    { id: "hero-white-01", name: "Bai Xiu", classLabel: "White", hp: "41 / 41", maxHp: "41", health: 41, maxHealth: 41, stress: "8", maxStress: "200", level: 2, xp: 180, isWounded: false, isAfflicted: false, isSelected: true },
    { id: "hero-black-01", name: "Hei Zhen", classLabel: "Black", hp: "34 / 40", maxHp: "40", health: 34, maxHealth: 40, stress: "24", maxStress: "200", level: 1, xp: 60, isWounded: true, isAfflicted: false, isSelected: false }
  ],
  maxPartySize: 4,
  isReadyToLaunch: true,
  supplyLevel: "充足",
  provisionCost: "150 金币",
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

export const replayDungeonHintViewModel: import("../bridge/contractTypes").DungeonHintViewModel = {
  kind: "dungeon-hint",
  title: "副本提示",
  expeditionName: "The Depths Await",
  dungeonDescription: "幽暗深渊中回响着远古的低语。探险者将面对腐败的亡灵军团与潜伏在阴影中的未知恐怖。只有最坚韧的意志才能抵达深处。",
  recommendedLevel: "Level 2+",
  partySize: 2,
  difficulty: "Challenging",
  estimatedDuration: "Medium",
  tips: [
    "Bring sufficient torches — darkness increases stress",
    "Holy water is effective against undead foes",
    "Camp before the boss room to recover stress"
  ],
  warnings: [
    "High stress environment — torches burn faster",
    "Undead enemies resist bleed effects"
  ],
  expectedEnemies: [
    "Bone Soldier",
    "Necromancer",
    "Plague Bearer"
  ],
  rewardPreview: [
    "Ancient Relics",
    "Rare Gems",
    "Hero Experience"
  ],
  supplyLevel: "Adequate",
  provisionCost: "150 Gold",
  isEnterable: true
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

export const replayDungeonInteractionViewModel: DungeonInteractionViewModel = {
  kind: "dungeon-interaction",
  title: "Dungeon Interaction",
  dungeonName: "The Depths Await",
  roomType: "event",
  roomLabel: "Ancient Altar",
  roomDescription: "An ancient altar stands before you, covered in moss and faintly glowing runes. Something about it feels both inviting and dangerous.",
  progress: {
    currentRoom: 3,
    totalRooms: 9,
    roomsCleared: 2
  },
  party: [
    { id: "hero-hunter-01", name: "Shen", classLabel: "Hunter", hp: "38 / 42", maxHp: "42", stress: "17", maxStress: "200" },
    { id: "hero-white-01", name: "Bai Xiu", classLabel: "White", hp: "41 / 41", maxHp: "41", stress: "8", maxStress: "200" }
  ],
  interactions: [
    { id: "investigate", label: "Investigate", description: "Examine the altar closely for clues or hidden mechanisms.", isAvailable: true },
    { id: "use-item", label: "Use Item", description: "Attempt to use a provision or tool on the altar.", isAvailable: true },
    { id: "pray", label: "Pray", description: "Offer a prayer at the altar. The outcome is uncertain.", isAvailable: true },
    { id: "ignore", label: "Ignore", description: "Leave the altar untouched and proceed.", isAvailable: true }
  ],
  isProceedAvailable: true,
  isRetreatAvailable: true
};

export const replayDungeonAssistViewModel: DungeonAssistViewModel = {
  kind: "dungeon-assist",
  title: "Dungeon Assist",
  dungeonName: "QingLong Depths",
  roomNumber: 1,
  party: [
    { id: "hero-hunter-01", name: "Shen", classLabel: "Hunter", hp: "38 / 42", maxHp: "42", stress: "17", maxStress: "200", level: 2, isSelected: true },
    { id: "hero-white-01", name: "Bai Xiu", classLabel: "White", hp: "41 / 41", maxHp: "41", stress: "8", maxStress: "200", level: 2, isSelected: false },
    { id: "hero-black-01", name: "Hei Zhen", classLabel: "Black", hp: "34 / 40", maxHp: "40", stress: "24", maxStress: "200", level: 1, isSelected: false }
  ],
  selectedHeroId: "hero-hunter-01",
  assistActions: [
    { id: "heal-wound", label: "Heal", description: "Restore health to selected hero", iconType: "heal", isAvailable: true },
    { id: "reduce-stress", label: "Calm", description: "Reduce stress of selected hero", iconType: "calm", isAvailable: true },
    { id: "apply-buff", label: "Buff", description: "Apply a combat buff", iconType: "buff", isAvailable: false },
    { id: "remove-debuff", label: "Cleanse", description: "Remove negative status", iconType: "cleanse", isAvailable: false },
    { id: "guard-ally", label: "Guard", description: "Guard an ally", iconType: "guard", isAvailable: false }
  ],
  canContinue: false
};

export const replayDungeonMapViewModel: DungeonMapViewModel = {
  kind: "dungeon-map",
  title: "Dungeon Map",
  expeditionName: "The Depths Await",
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
    { id: "hero-hunter-01", name: "Shen", classLabel: "Hunter", hp: "38 / 42", maxHp: "42", stress: "17", maxStress: "200", isWounded: true, isAfflicted: false },
    { id: "hero-white-01", name: "Bai Xiu", classLabel: "White", hp: "41 / 41", maxHp: "41", stress: "8", maxStress: "200", isWounded: false, isAfflicted: false }
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

export const replayAttackCombatViewModel = {
  kind: "combat" as const,
  title: "副本场景-人物攻击",
  round: 1,
  turnPhase: "player" as const,
  activeHeroId: "hero-hunter-01",
  party: [
    {
      id: "hero-hunter-01",
      name: "Shen",
      classLabel: "Hunter",
      hp: "38 / 42",
      maxHp: "42",
      stress: "17",
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
      id: "hero-white-01",
      name: "Bai Xiu",
      classLabel: "White",
      hp: "41 / 41",
      maxHp: "41",
      stress: "8",
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
    },
    {
      id: "hero-black-01",
      name: "Hei Zhen",
      classLabel: "Black",
      hp: "34 / 40",
      maxHp: "40",
      stress: "24",
      maxStress: "200",
      position: 3,
      isActive: false,
      isAlive: true,
      skills: [
        { id: "skill-b1", name: "Shadow Strike", description: "Attack from the shadows.", target: "Enemy", hitRating: "85%", critRating: "10%", cooldown: 0, cooldownRemaining: 0 },
        { id: "skill-b2", name: "Smoke Bomb", description: "Blind enemies, reducing accuracy.", target: "Enemy", hitRating: "75%", critRating: "0%", cooldown: 2, cooldownRemaining: 0 },
        { id: "skill-b3", name: "Poison Blade", description: "Apply poison to the target.", target: "Enemy", hitRating: "80%", critRating: "4%", cooldown: 1, cooldownRemaining: 0 },
        { id: "skill-b4", name: "Backstab", description: "High damage if target is marked.", target: "Enemy", hitRating: "70%", critRating: "12%", cooldown: 2, cooldownRemaining: 1 },
        { id: "skill-b5", name: "Vanish", description: "Become untargetable for one turn.", target: "Self", hitRating: "100%", critRating: "0%", cooldown: 3, cooldownRemaining: 0 }
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
      size: "large" as const
    },
    {
      id: "enemy-larva-01",
      name: "Larva Swarm",
      hp: "30 / 30",
      maxHp: "30",
      position: 2,
      isAlive: true,
      isTargeted: false,
      size: "small" as const
    }
  ],
  selectedSkillId: "skill-1",
  combatLog: [
    "Round 1 begins...",
    "Shen readies Hunting Bow.",
    "Select a target to attack."
  ],
  isPlayerTurn: true,
  canFlee: true
};

export const replayCombatViewModel = {
  ...replayAttackCombatViewModel,
  title: "Dungeon Combat",
  dungeonName: "The Depths Await",
  roundLabel: "Round 3",
  phase: "character-hit" as const,
  round: 3,
  selectedSkillId: undefined,
  party: replayAttackCombatViewModel.party.map((hero) =>
    hero.id === "hero-hunter-01"
      ? {
          ...hero,
          hp: "28 / 42",
          stress: "24",
          isHit: true,
          skills: hero.skills.map((skill) => ({ ...skill, isAvailable: false }))
        }
      : {
          ...hero,
          isHit: false,
          skills: hero.skills.map((skill) => ({ ...skill, isAvailable: false }))
        }
  ),
  enemies: replayAttackCombatViewModel.enemies.map((enemy) => ({ ...enemy, isHit: false })),
  hitTargetHeroId: "hero-hunter-01",
  hitDamage: "10",
  hitLog: "Cultist Acolyte strikes Shen for 10 damage.",
  combatLog: [
    "Cultist Acolyte strikes Shen for 10 damage.",
    "Acknowledge the hit before issuing the next command."
  ],
  roomMap: {
    rooms: [
      { id: "r1", x: 0, y: 2, kind: "combat" as const, isCurrent: true, isCleared: false },
      { id: "r2", x: 1, y: 2, kind: "corridor" as const, isCurrent: false, isCleared: true },
      { id: "r3", x: 2, y: 1, kind: "event" as const, isCurrent: false, isCleared: true },
      { id: "r4", x: 2, y: 3, kind: "combat" as const, isCurrent: false, isCleared: false },
      { id: "r5", x: 3, y: 2, kind: "treasure" as const, isCurrent: false, isCleared: false },
      { id: "r6", x: 4, y: 2, kind: "boss" as const, isCurrent: false, isCleared: false }
    ],
    connections: [
      { from: "r1", to: "r2" },
      { from: "r2", to: "r3" },
      { from: "r2", to: "r4" },
      { from: "r3", to: "r5" },
      { from: "r4", to: "r5" },
      { from: "r5", to: "r6" }
    ]
  },
  isPlayerTurn: false,
  isFleeAvailable: true,
  turnCount: 3,
  settingsLabel: "设置"
};

export const replayCombatSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "combat",
  viewModel: replayAttackCombatViewModel,
  debugMessage: "Replay bridge showing combat scene - character attack phase."
};

export const combatSnapshot = replayCombatSnapshot;

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

// Dungeon select flow snapshot
export const dungeonSelectSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "dungeon-select",
  viewModel: replayDungeonSelectViewModel,
  debugMessage: "Replay bridge showing dungeon select screen."
};

// Expedition planning flow snapshot
export const expeditionPlanningSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "expedition-planning",
  viewModel: replayExpeditionPlanningViewModel,
  debugMessage: "Replay bridge showing expedition planning screen."
};

// Provisioning flow snapshot
export const provisioningSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "provisioning",
  viewModel: replayProvisioningViewModel,
  debugMessage: "Replay bridge showing provisioning screen."
};

// Dungeon hint flow snapshot
export const dungeonHintSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "dungeon-hint",
  viewModel: replayDungeonHintViewModel,
  debugMessage: "Replay bridge showing dungeon hint screen."
};

// Expedition launch flow snapshot
export const expeditionSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "expedition",
  viewModel: replayExpeditionViewModel,
  debugMessage: "Replay bridge showing expedition launch screen."
};

// Dungeon interaction flow snapshot
export const dungeonInteractionSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "dungeon-interaction",
  viewModel: replayDungeonInteractionViewModel,
  debugMessage: "Replay bridge showing dungeon interaction screen."
};

// Dungeon assist flow snapshot
export const dungeonAssistSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "dungeon-assist",
  viewModel: replayDungeonAssistViewModel,
  debugMessage: "Replay bridge showing dungeon assist screen."
};

// Dungeon map flow snapshot
export const dungeonMapSnapshot: DdgcFrontendSnapshot = {
  lifecycle: "ready",
  flowState: "dungeon-map",
  viewModel: replayDungeonMapViewModel,
  debugMessage: "Replay bridge showing dungeon map screen."
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
  const validFlowStates: FlowState[] = ["boot", "load", "town", "dungeon-select", "expedition-planning", "provisioning", "dungeon-hint", "expedition", "dungeon-assist", "dungeon-map", "combat", "dungeon-interaction", "result", "return"];
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
    "dungeon-select": ["dungeon-select"],
    "expedition-planning": ["expedition-planning"],
    provisioning: ["provisioning"],
    "dungeon-hint": ["dungeon-hint"],
    expedition: ["expedition"],
    "dungeon-assist": ["dungeon-assist"],
    combat: ["combat"],
    "dungeon-map": ["dungeon-map"],
    "dungeon-interaction": ["dungeon-interaction"],
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
    case "dungeon-select": {
      if (!vm.title || typeof vm.title !== "string") e.push("DungeonSelectViewModel: title is missing");
      if (!Array.isArray(vm.dungeons)) { e.push("DungeonSelectViewModel: dungeons is not an array"); } else if (vm.dungeons.length === 0) { e.push("DungeonSelectViewModel: dungeons array is empty"); }
      if (!Array.isArray(vm.party)) { e.push("DungeonSelectViewModel: party is not an array"); }
      if (typeof vm.maxPartySize !== "number") e.push("DungeonSelectViewModel: maxPartySize is not a number");
      if (typeof vm.isReadyToProceed !== "boolean") e.push("DungeonSelectViewModel: isReadyToProceed is not a boolean");
      if (!vm.campaignName || typeof vm.campaignName !== "string") e.push("DungeonSelectViewModel: campaignName is missing");
      break;
    }
    case "expedition-planning": {
      if (!vm.title || typeof vm.title !== "string") e.push("ExpeditionPlanningViewModel: title is missing");
      if (!vm.campaignName || typeof vm.campaignName !== "string") e.push("ExpeditionPlanningViewModel: campaignName is missing");
      if (!vm.selectedPlaneId || typeof vm.selectedPlaneId !== "string") e.push("ExpeditionPlanningViewModel: selectedPlaneId is missing");
      if (!Array.isArray(vm.planes)) { e.push("ExpeditionPlanningViewModel: planes is not an array"); } else if (vm.planes.length === 0) { e.push("ExpeditionPlanningViewModel: planes array is empty"); }
      if (!Array.isArray(vm.partySlots)) e.push("ExpeditionPlanningViewModel: partySlots is not an array");
      if (typeof vm.maxPartySize !== "number") e.push("ExpeditionPlanningViewModel: maxPartySize is not a number");
      if (typeof vm.isReadyToProvision !== "boolean") e.push("ExpeditionPlanningViewModel: isReadyToProvision is not a boolean");
      if (!vm.provisionCost || typeof vm.provisionCost !== "string") e.push("ExpeditionPlanningViewModel: provisionCost is missing");
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
    case "dungeon-hint": {
      if (!vm.title || typeof vm.title !== "string") e.push("DungeonHintViewModel: title is missing");
      if (!vm.expeditionName || typeof vm.expeditionName !== "string") e.push("DungeonHintViewModel: expeditionName is missing");
      if (!vm.dungeonDescription || typeof vm.dungeonDescription !== "string") e.push("DungeonHintViewModel: dungeonDescription is missing");
      if (!vm.recommendedLevel || typeof vm.recommendedLevel !== "string") e.push("DungeonHintViewModel: recommendedLevel is missing");
      if (typeof vm.partySize !== "number") e.push("DungeonHintViewModel: partySize is not a number");
      if (!vm.difficulty || typeof vm.difficulty !== "string") e.push("DungeonHintViewModel: difficulty is missing");
      if (!Array.isArray(vm.tips)) e.push("DungeonHintViewModel: tips is not an array");
      if (!Array.isArray(vm.warnings)) e.push("DungeonHintViewModel: warnings is not an array");
      if (!Array.isArray(vm.expectedEnemies)) e.push("DungeonHintViewModel: expectedEnemies is not an array");
      if (!Array.isArray(vm.rewardPreview)) e.push("DungeonHintViewModel: rewardPreview is not an array");
      if (!vm.supplyLevel || typeof vm.supplyLevel !== "string") e.push("DungeonHintViewModel: supplyLevel is missing");
      if (!vm.provisionCost || typeof vm.provisionCost !== "string") e.push("DungeonHintViewModel: provisionCost is missing");
      if (typeof vm.isEnterable !== "boolean") e.push("DungeonHintViewModel: isEnterable is not a boolean");
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
    case "dungeon-interaction": {
      if (!vm.title || typeof vm.title !== "string") e.push("DungeonInteractionViewModel: title is missing");
      if (!vm.dungeonName || typeof vm.dungeonName !== "string") e.push("DungeonInteractionViewModel: dungeonName is missing");
      if (!vm.roomType || typeof vm.roomType !== "string") e.push("DungeonInteractionViewModel: roomType is missing");
      if (!vm.roomLabel || typeof vm.roomLabel !== "string") e.push("DungeonInteractionViewModel: roomLabel is missing");
      if (!vm.roomDescription || typeof vm.roomDescription !== "string") e.push("DungeonInteractionViewModel: roomDescription is missing");
      if (!vm.progress || typeof vm.progress !== "object") e.push("DungeonInteractionViewModel: progress is missing");
      if (!Array.isArray(vm.party)) { e.push("DungeonInteractionViewModel: party is not an array"); } else if (vm.party.length === 0) { e.push("DungeonInteractionViewModel: party array is empty"); }
      if (!Array.isArray(vm.interactions)) e.push("DungeonInteractionViewModel: interactions is not an array");
      if (typeof vm.isProceedAvailable !== "boolean") e.push("DungeonInteractionViewModel: isProceedAvailable is not a boolean");
      if (typeof vm.isRetreatAvailable !== "boolean") e.push("DungeonInteractionViewModel: isRetreatAvailable is not a boolean");
      break;
    }
    case "dungeon-assist": {
      if (!vm.title || typeof vm.title !== "string") e.push("DungeonAssistViewModel: title is missing");
      if (!vm.dungeonName || typeof vm.dungeonName !== "string") e.push("DungeonAssistViewModel: dungeonName is missing");
      if (typeof vm.roomNumber !== "number") e.push("DungeonAssistViewModel: roomNumber is not a number");
      if (!Array.isArray(vm.party)) { e.push("DungeonAssistViewModel: party is not an array"); } else if (vm.party.length === 0) { e.push("DungeonAssistViewModel: party array is empty"); }
      if (!vm.selectedHeroId || typeof vm.selectedHeroId !== "string") e.push("DungeonAssistViewModel: selectedHeroId is missing");
      if (!Array.isArray(vm.assistActions)) e.push("DungeonAssistViewModel: assistActions is not an array");
      if (typeof vm.canContinue !== "boolean") e.push("DungeonAssistViewModel: canContinue is not a boolean");
      break;
    }
    case "dungeon-map": {
      if (!vm.title || typeof vm.title !== "string") e.push("DungeonMapViewModel: title is missing");
      if (!vm.expeditionName || typeof vm.expeditionName !== "string") e.push("DungeonMapViewModel: expeditionName is missing");
      if (!vm.dungeonName || typeof vm.dungeonName !== "string") e.push("DungeonMapViewModel: dungeonName is missing");
      if (!vm.currentRoomId || typeof vm.currentRoomId !== "string") e.push("DungeonMapViewModel: currentRoomId is missing");
      if (!Array.isArray(vm.rooms)) { e.push("DungeonMapViewModel: rooms is not an array"); } else if (vm.rooms.length === 0) { e.push("DungeonMapViewModel: rooms array is empty"); }
      if (!Array.isArray(vm.party)) { e.push("DungeonMapViewModel: party is not an array"); } else if (vm.party.length === 0) { e.push("DungeonMapViewModel: party array is empty"); }
      if (typeof vm.torchLevel !== "number") e.push("DungeonMapViewModel: torchLevel is not a number");
      if (typeof vm.maxTorchLevel !== "number") e.push("DungeonMapViewModel: maxTorchLevel is not a number");
      if (typeof vm.exploredCount !== "number") e.push("DungeonMapViewModel: exploredCount is not a number");
      if (typeof vm.totalRooms !== "number") e.push("DungeonMapViewModel: totalRooms is not a number");
      if (typeof vm.completionPercent !== "number") e.push("DungeonMapViewModel: completionPercent is not a number");
      if (typeof vm.isRetreatAvailable !== "boolean") e.push("DungeonMapViewModel: isRetreatAvailable is not a boolean");
      if (typeof vm.isComplete !== "boolean") e.push("DungeonMapViewModel: isComplete is not a boolean");
      if (typeof vm.minimapRows !== "number") e.push("DungeonMapViewModel: minimapRows is not a number");
      if (typeof vm.minimapCols !== "number") e.push("DungeonMapViewModel: minimapCols is not a number");
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
    case "combat": {
      if (!vm.title || typeof vm.title !== "string") e.push("CombatViewModel: title is missing");
      if (typeof vm.round !== "number") e.push("CombatViewModel: round is not a number");
      if (vm.turnPhase !== "player" && vm.turnPhase !== "enemy") e.push(`CombatViewModel: turnPhase is "${String(vm.turnPhase)}", expected "player" or "enemy"`);
      if (!vm.activeHeroId || typeof vm.activeHeroId !== "string") e.push("CombatViewModel: activeHeroId is missing");
      if (!Array.isArray(vm.party)) { e.push("CombatViewModel: party is not an array"); } else if (vm.party.length === 0) { e.push("CombatViewModel: party array is empty"); }
      if (!Array.isArray(vm.enemies)) { e.push("CombatViewModel: enemies is not an array"); } else if (vm.enemies.length === 0) { e.push("CombatViewModel: enemies array is empty"); }
      if (!Array.isArray(vm.combatLog)) e.push("CombatViewModel: combatLog is not an array");
      if (typeof vm.isPlayerTurn !== "boolean") e.push("CombatViewModel: isPlayerTurn is not a boolean");
      if (typeof vm.canFlee !== "boolean") e.push("CombatViewModel: canFlee is not a boolean");
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
