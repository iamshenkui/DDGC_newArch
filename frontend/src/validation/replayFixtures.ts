import type {
  BootLoadViewModel,
  DdgcFrontendSnapshot,
  FatalErrorViewModel,
  HeroDetailViewModel,
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