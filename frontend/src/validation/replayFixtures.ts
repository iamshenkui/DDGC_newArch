import type {
  BootLoadViewModel,
  BuildingDetailViewModel,
  DdgcFrontendSnapshot,
  ExpeditionSetupViewModel,
  ExpeditionResultViewModel,
  ReturnViewModel,
  FatalErrorViewModel,
  HeroDetailViewModel,
  ProvisioningViewModel,
  TownViewModel,
  UnsupportedViewModel
} from "../bridge/contractTypes";

export const replayTownViewModel: TownViewModel = {
  kind: "town",
  title: "Town Surface Placeholder",
  campaignName: "The Azure Lantern",
  campaignSummary:
    "Representative Phase 10 replay snapshot for roster, building, and provisioning work.",
  heroes: [
    {
      id: "hero-hunter-01",
      name: "Shen",
      classLabel: "Hunter",
      hp: "38 / 42",
      stress: "17",
      level: 2
    },
    {
      id: "hero-white-01",
      name: "Bai Xiu",
      classLabel: "White",
      hp: "41 / 41",
      stress: "8",
      level: 2
    },
    {
      id: "hero-black-01",
      name: "Hei Zhen",
      classLabel: "Black",
      hp: "34 / 40",
      stress: "24",
      level: 1
    }
  ],
  buildings: [
    {
      id: "guild",
      label: "Guild",
      summary: "Skill training and party capability review.",
      status: "ready"
    },
    {
      id: "blacksmith",
      label: "Blacksmith",
      summary: "Equipment status is contract-backed but still visually skeletal.",
      status: "partial"
    },
    {
      id: "sanitarium",
      label: "Sanitarium",
      summary: "Needs real rendered treatment flow in Phase 10 town surface.",
      status: "partial"
    }
  ],
  nextActionLabel: "Provision Expedition"
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
  progression: {
    level: 2,
    experience: "240",
    experienceToNext: "360"
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
  combatSkills: ["Hunting Bow", "Rapid Shot", "Marked for Death", "Batty Advice"],
  campingSkills: ["Campfire Song", "Warrior's Restore"],
  weapon: "Hunter's Bow (+2)",
  armor: "Leather Armor (+1)",
  campNotes: "Excellent sustain healer with strong camp utility. Marked for Death synergy with teammates."
};

export const replayBuildingDetailViewModel: BuildingDetailViewModel = {
  kind: "building-detail",
  buildingId: "guild",
  label: "Guild",
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

export const replayProvisioningViewModel: ProvisioningViewModel = {
  kind: "provisioning",
  title: "Provision Expedition",
  campaignName: "The Azure Lantern",
  expeditionLabel: "The Depths Await",
  expeditionSummary: "Assemble your party and provision wisely. The expedition awaits those who dare enter.",
  party: [
    { id: "hero-hunter-01", name: "Shen", classLabel: "Hunter", hp: "38 / 42", stress: "17", level: 2, isSelected: true },
    { id: "hero-white-01", name: "Bai Xiu", classLabel: "White", hp: "41 / 41", stress: "8", level: 2, isSelected: true },
    { id: "hero-black-01", name: "Hei Zhen", classLabel: "Black", hp: "34 / 40", stress: "24", level: 1, isSelected: false }
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
  isLaunchable: true
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
      status: "alive",
      hpChange: "-4",
      stressChange: "+12"
    },
    {
      heroId: "hero-white-01",
      heroName: "Bai Xiu",
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
      status: "alive",
      hpChange: "-18",
      stressChange: "+25"
    },
    {
      heroId: "hero-white-01",
      heroName: "Bai Xiu",
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
      status: "alive",
      hpChange: "-12",
      stressChange: "+18"
    },
    {
      heroId: "hero-white-01",
      heroName: "Bai Xiu",
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
      hp: "34 / 42",
      stress: "29"
    },
    {
      heroId: "hero-white-01",
      heroName: "Bai Xiu",
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