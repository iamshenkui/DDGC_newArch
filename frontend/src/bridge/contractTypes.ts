import type { RuntimeMode } from "../app/runtimeMode";

export type FlowState =
  | "boot"
  | "load"
  | "town"
  | "provisioning"
  | "expedition"
  | "dungeon-assist"
  | "combat"
  | "dungeon-map"
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
  maxHp: string;
  health: number;
  maxHealth: number;
  stress: string;
  maxStress: string;
  level: number;
  xp: number;
  isWounded: boolean;
  isAfflicted: boolean;
  positiveQuirks: ReadonlyArray<string>;
  negativeQuirks: ReadonlyArray<string>;
  diseases: ReadonlyArray<string>;
}

export interface TownBuildingSummary {
  id: string;
  label: string;
  summary: string;
  status: "ready" | "partial" | "locked";
  currentUpgrade?: string;
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
  currentUpgrade?: string;
  upgradeRequirement?: string;
}

export interface HeroProgression {
  level: number;
  experience: string;
  experienceToNext: string;
  resolveLevel: number;
  resolveXP: string;
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

export interface HeroBaseStats {
  dmg: string;
  maxHp: string;
  crit: string;
  spd: string;
  dodge: string;
}

export interface EquipmentItem {
  name: string;
  level: number;
  icon?: string;
}

export interface TrinketItem {
  name: string;
  description: string;
  icon?: string;
}

export interface SkillDetail {
  name: string;
  level: number;
  description: string;
  target: string;
  hitRating: string;
  critRating: string;
}

export interface HeroDetailViewModel {
  kind: "hero-detail";
  heroId: string;
  name: string;
  classLabel: string;
  hp: string;
  maxHp: string;
  stress: string;
  maxStress: string;
  resolve: string;
  resolveLabel: string;
  progression: HeroProgression;
  resistances: HeroResistances;
  baseStats: HeroBaseStats;
  combatSkills: ReadonlyArray<SkillDetail>;
  campingSkills: ReadonlyArray<SkillDetail>;
  weapon: EquipmentItem;
  armor: EquipmentItem;
  leftTrinket?: TrinketItem;
  rightTrinket?: TrinketItem;
  positiveQuirks: ReadonlyArray<string>;
  negativeQuirks: ReadonlyArray<string>;
  diseases: ReadonlyArray<string>;
  isWounded: boolean;
  isAfflicted: boolean;
  heroDescription: string;
  talent: string;
}

export interface TownViewModel {
  kind: "town";
  title: string;
  campaignName: string;
  campaignSummary: string;
  heroes: ReadonlyArray<TownHeroSummary>;
  roster: ReadonlyArray<TownHeroSummary>;
  buildings: ReadonlyArray<TownBuildingSummary>;
  gold: number;
  isFreshVisit: boolean;
  nextActionLabel: string;
}

export interface ProvisioningHeroSummary {
  id: string;
  name: string;
  classLabel: string;
  hp: string;
  maxHp: string;
  health: number;
  maxHealth: number;
  stress: string;
  maxStress: string;
  level: number;
  xp: number;
  isWounded: boolean;
  isAfflicted: boolean;
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

export interface ExpeditionHeroSummary {
  id: string;
  name: string;
  classLabel: string;
  hp: string;
  maxHp: string;
  stress: string;
  maxStress: string;
}

export interface ExpeditionSetupViewModel {
  kind: "expedition";
  title: string;
  expeditionName: string;
  partySize: number;
  party: ReadonlyArray<ExpeditionHeroSummary>;
  difficulty: string;
  estimatedDuration: string;
  objectives: ReadonlyArray<string>;
  warnings: ReadonlyArray<string>;
  supplyLevel: string;
  provisionCost: string;
  isLaunchable: boolean;
}

export interface DungeonAssistHero {
  id: string;
  name: string;
  classLabel: string;
  hp: string;
  maxHp: string;
  stress: string;
  maxStress: string;
  level: number;
  isSelected: boolean;
}

export interface AssistAction {
  id: string;
  label: string;
  description: string;
  iconType: string;
  isAvailable: boolean;
  targetHeroId?: string;
}

export interface DungeonAssistViewModel {
  kind: "dungeon-assist";
  title: string;
  dungeonName: string;
  roomNumber: number;
  party: ReadonlyArray<DungeonAssistHero>;
  selectedHeroId: string;
  assistActions: ReadonlyArray<AssistAction>;
  canContinue: boolean;
}

export interface DungeonMapRoom {
  id: string;
  x: number;
  y: number;
  type: "entrance" | "combat" | "treasure" | "rest" | "boss" | "exit" | "empty" | "curio" | "shrine";
  label: string;
  isRevealed: boolean;
  isVisited: boolean;
  isCurrent: boolean;
  connections: ReadonlyArray<string>;
  difficulty?: string;
  lootPreview?: string;
}

export interface DungeonMapHero {
  id: string;
  name: string;
  classLabel: string;
  hp: string;
  maxHp: string;
  stress: string;
  maxStress: string;
  isWounded: boolean;
  isAfflicted: boolean;
}

export interface DungeonMapViewModel {
  kind: "dungeon-map";
  title: string;
  expeditionName: string;
  dungeonName: string;
  currentRoomId: string;
  rooms: ReadonlyArray<DungeonMapRoom>;
  party: ReadonlyArray<DungeonMapHero>;
  torchLevel: number;
  maxTorchLevel: number;
  exploredCount: number;
  totalRooms: number;
  completionPercent: number;
  isRetreatAvailable: boolean;
  isComplete: boolean;
  minimapRows: number;
  minimapCols: number;
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
    classLabel: string;
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
    classLabel: string;
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
  | DungeonAssistViewModel
  | DungeonMapViewModel
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
  | { type: "enter-dungeon-assist" }
  | { type: "select-assist-hero"; heroId: string }
  | { type: "use-assist-action"; actionId: string }
  | { type: "continue-from-dungeon" }
  | { type: "enter-room"; roomId: string }
  | { type: "retreat-from-dungeon" }
  | { type: "complete-dungeon" }
  | { type: "return-to-town" }
  | { type: "continue-from-result" }
  | { type: "resume-from-return" };
