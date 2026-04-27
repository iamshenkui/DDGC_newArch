import type { RuntimeMode } from "../app/runtimeMode";

export type FlowState =
  | "boot"
  | "load"
  | "town"
  | "provisioning"
  | "expedition"
  | "combat"
  | "result"
  | "return";

export type FrontendLifecycle =
  | "booting"
  | "loading"
  | "ready"
  | "unsupported"
  | "fatal";

export interface BootLoadViewModel {
  kind: "boot-load";
  title: string;
  summary: string;
  mode: RuntimeMode;
}

export interface TownHeroSummary {
  id: string;
  name: string;
  classLabel: string;
  hp: string;
  stress: string;
  level: number;
}

export interface TownBuildingSummary {
  id: string;
  label: string;
  summary: string;
  status: "ready" | "partial" | "locked";
}

export interface BuildingAction {
  id: string;
  label: string;
  description: string;
  cost: string;
  isAvailable: boolean;
  isUnsupported: boolean;
}

export interface BuildingDetailViewModel {
  kind: "building-detail";
  buildingId: string;
  label: string;
  status: "ready" | "partial" | "locked";
  description: string;
  actions: ReadonlyArray<BuildingAction>;
  upgradeRequirement?: string;
}

export interface HeroProgression {
  level: number;
  experience: string;
  experienceToNext: string;
}

export interface HeroResistances {
  stun: string;
  bleed: string;
  disease: string;
  move: string;
  death: string;
  trap: string;
  hazard: string;
}

export interface HeroDetailViewModel {
  kind: "hero-detail";
  heroId: string;
  name: string;
  classLabel: string;
  hp: string;
  maxHp: string;
  stress: string;
  resolve: string;
  progression: HeroProgression;
  resistances: HeroResistances;
  combatSkills: ReadonlyArray<string>;
  campingSkills: ReadonlyArray<string>;
  weapon: string;
  armor: string;
  campNotes: string;
}

export interface TownViewModel {
  kind: "town";
  title: string;
  campaignName: string;
  campaignSummary: string;
  heroes: ReadonlyArray<TownHeroSummary>;
  buildings: ReadonlyArray<TownBuildingSummary>;
  nextActionLabel: string;
}

export interface ProvisioningHeroSummary {
  id: string;
  name: string;
  classLabel: string;
  hp: string;
  stress: string;
  level: number;
  isSelected: boolean;
}

export interface ProvisioningViewModel {
  kind: "provisioning";
  title: string;
  campaignName: string;
  expeditionLabel: string;
  expeditionSummary: string;
  party: ReadonlyArray<ProvisioningHeroSummary>;
  maxPartySize: number;
  isReadyToLaunch: boolean;
  supplyLevel: string;
  provisionCost: string;
}

export interface ExpeditionSetupViewModel {
  kind: "expedition";
  title: string;
  expeditionName: string;
  partySize: number;
  difficulty: string;
  estimatedDuration: string;
  objectives: ReadonlyArray<string>;
  warnings: ReadonlyArray<string>;
  isLaunchable: boolean;
}

export interface ExpeditionResultViewModel {
  kind: "result";
  title: string;
  expeditionName: string;
  outcome: "success" | "failure" | "partial";
  summary: string;
  lootAcquired: ReadonlyArray<string>;
  heroOutcomes: ReadonlyArray<{
    heroId: string;
    heroName: string;
    status: "alive" | "dead" | "stressed";
    hpChange: string;
    stressChange: string;
  }>;
  resourcesGained: {
    gold: number;
    supplies: number;
    experience: number;
  };
  isContinueAvailable: boolean;
}

export interface ReturnViewModel {
  kind: "return";
  title: string;
  expeditionName: string;
  summary: string;
  returningHeroes: ReadonlyArray<{
    heroId: string;
    heroName: string;
    hp: string;
    stress: string;
  }>;
  isTownResumeAvailable: boolean;
}

export interface UnsupportedViewModel {
  kind: "unsupported";
  title: string;
  reason: string;
}

export interface FatalErrorViewModel {
  kind: "fatal";
  title: string;
  reason: string;
}

export type DdgcViewModel =
  | BootLoadViewModel
  | TownViewModel
  | HeroDetailViewModel
  | BuildingDetailViewModel
  | ProvisioningViewModel
  | ExpeditionSetupViewModel
  | ExpeditionResultViewModel
  | ReturnViewModel
  | UnsupportedViewModel
  | FatalErrorViewModel;

export interface DdgcFrontendSnapshot {
  lifecycle: FrontendLifecycle;
  flowState: FlowState;
  viewModel: DdgcViewModel;
  debugMessage?: string;
}

export type DdgcFrontendIntent =
  | { type: "boot"; mode: RuntimeMode }
  | { type: "open-hero"; heroId: string }
  | { type: "open-building"; buildingId: string }
  | { type: "building-action"; actionId: string }
  | { type: "start-provisioning" }
  | { type: "toggle-hero-selection"; heroId: string }
  | { type: "confirm-provisioning" }
  | { type: "launch-expedition" }
  | { type: "return-to-town" }
  | { type: "continue-from-result" }
  | { type: "resume-from-return" };