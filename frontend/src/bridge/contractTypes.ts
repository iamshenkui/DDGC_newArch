import type { RuntimeMode } from "../app/runtimeMode";

export type FlowState =
  | "boot"
  | "load"
  | "town"
  | "expedition-planning"
  | "dungeon-select"
  | "provisioning"
  | "dungeon-hint"
  | "expedition"
  | "dungeon-assist"
  | "dungeon-map"
  | "combat"
  | "dungeon-interaction"
  | "dungeon-items"
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

export interface SupplyItem {
  id: string;
  name: string;
  icon: string;
  qty: number;
}

export interface ExpeditionPlanningHeroSlot {
  heroId: string;
  heroName: string;
  classLabel: string;
  hp: string;
  stress: string;
  level: number;
}

export interface DungeonSelectHeroSummary {
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

export interface ExpeditionPlane {
  id: string;
  name: string;
  description: string;
  difficulty: string;
  difficultyPips: number;
  estimatedDuration: string;
  isLocked: boolean;
  lockReason?: string;
  rewards: ReadonlyArray<string>;
  objectives: ReadonlyArray<string>;
  themeColor: string;
}

export interface DungeonOption {
  id: string;
  name: string;
  description: string;
  difficulty: string;
  estimatedDuration: string;
  recommendedLevel: number;
  provisionCost: string;
  supplyLevel: string;
  rewards: ReadonlyArray<string>;
  isAvailable: boolean;
  lockReason?: string;
}

export interface ExpeditionPlanningViewModel {
  kind: "expedition-planning";
  title: string;
  campaignName: string;
  selectedPlaneId: string;
  planes: ReadonlyArray<ExpeditionPlane>;
  partySlots: ReadonlyArray<ExpeditionPlanningHeroSlot | null>;
  maxPartySize: number;
  isReadyToProvision: boolean;
  provisionCost: string;
}

export interface DungeonSelectViewModel {
  kind: "dungeon-select";
  title: string;
  campaignName: string;
  selectedDungeonId: string | null;
  dungeons: ReadonlyArray<DungeonOption>;
  party: ReadonlyArray<DungeonSelectHeroSummary>;
  maxPartySize: number;
  isReadyToProceed: boolean;
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
  supplies?: ReadonlyArray<SupplyItem>;
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

export interface DungeonHintViewModel {
  kind: "dungeon-hint";
  title: string;
  expeditionName: string;
  dungeonDescription: string;
  recommendedLevel: string;
  partySize: number;
  difficulty: string;
  estimatedDuration: string;
  tips: ReadonlyArray<string>;
  warnings: ReadonlyArray<string>;
  expectedEnemies: ReadonlyArray<string>;
  rewardPreview: ReadonlyArray<string>;
  supplyLevel: string;
  provisionCost: string;
  isEnterable: boolean;
}

export interface DungeonInteractionSummary {
  id: string;
  label: string;
  description: string;
  isAvailable: boolean;
}

export interface DungeonInteractionViewModel {
  kind: "dungeon-interaction";
  title: string;
  dungeonName: string;
  roomType: "combat" | "event" | "corridor" | "boss" | "treasure" | "shop";
  roomLabel: string;
  roomDescription: string;
  progress: {
    currentRoom: number;
    totalRooms: number;
    roomsCleared: number;
  };
  party: ReadonlyArray<ExpeditionHeroSummary>;
  interactions: ReadonlyArray<DungeonInteractionSummary>;
  isProceedAvailable: boolean;
  isRetreatAvailable: boolean;
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

export type CombatPhase = "player-turn" | "enemy-turn" | "character-hit" | "resolution";

export interface CombatSkill {
  id: string;
  name: string;
  description: string;
  target: string;
  hitRating: string;
  critRating: string;
  cooldown: number;
  cooldownRemaining: number;
}

export interface CombatHero {
  id: string;
  name: string;
  classLabel: string;
  hp: string;
  maxHp: string;
  stress: string;
  maxStress: string;
  position: number;
  isActive: boolean;
  isAlive: boolean;
  isHit?: boolean;
  skills: ReadonlyArray<CombatSkill>;
  portrait?: string;
}

export interface CombatEnemy {
  id: string;
  name: string;
  hp: string;
  maxHp: string;
  position: number;
  isAlive: boolean;
  isTargeted: boolean;
  isHit?: boolean;
  size: "small" | "medium" | "large";
}

export interface CombatViewModel {
  kind: "combat";
  title: string;
  dungeonName?: string;
  roundLabel?: string;
  phase?: CombatPhase;
  round: number;
  turnPhase: "player" | "enemy";
  activeHeroId: string;
  party: ReadonlyArray<CombatHero>;
  enemies: ReadonlyArray<CombatEnemy>;
  selectedSkillId?: string;
  combatLog: ReadonlyArray<string>;
  hitTargetHeroId?: string;
  hitDamage?: string;
  hitLog?: string;
  isPlayerTurn: boolean;
  canFlee: boolean;
  isFleeAvailable?: boolean;
  turnCount?: number;
  settingsLabel?: string;
}

export interface DungeonItemsHero {
  id: string;
  name: string;
  classLabel: string;
  hp: string;
  maxHp: string;
  stress: string;
  maxStress: string;
  level: number;
}

export interface DungeonItem {
  id: string;
  name: string;
  icon: string;
  description: string;
  qty: number;
  isUsable: boolean;
  category: "heal" | "stress" | "buff" | "tool" | "misc";
}

export interface DungeonItemsViewModel {
  kind: "dungeon-items";
  title: string;
  dungeonName: string;
  roomLabel: string;
  party: ReadonlyArray<DungeonItemsHero>;
  items: ReadonlyArray<DungeonItem>;
  selectedItemId: string | null;
  selectedHeroId: string | null;
  maxItems: number;
  canContinue: boolean;
  returnFlowState: "dungeon-interaction" | "dungeon-map" | "combat";
}

export type DdgcViewModel =
  | BootLoadViewModel
  | TownViewModel
  | HeroDetailViewModel
  | BuildingDetailViewModel
  | ExpeditionPlanningViewModel
  | DungeonSelectViewModel
  | ProvisioningViewModel
  | DungeonHintViewModel
  | ExpeditionSetupViewModel
  | DungeonAssistViewModel
  | DungeonMapViewModel
  | CombatViewModel
  | DungeonInteractionViewModel
  | DungeonItemsViewModel
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
  | { type: "start-expedition-planning" }
  | { type: "select-plane"; planeId: string }
  | { type: "toggle-planning-hero"; heroId: string }
  | { type: "proceed-to-provisioning" }
  | { type: "start-dungeon-select" }
  | { type: "select-dungeon"; dungeonId: string }
  | { type: "toggle-dungeon-hero"; heroId: string }
  | { type: "confirm-dungeon-selection" }
  | { type: "start-provisioning" }
  | { type: "toggle-hero-selection"; heroId: string }
  | { type: "confirm-provisioning" }
  | { type: "accept-dungeon-hint" }
  | { type: "launch-expedition" }
  | { type: "proceed-dungeon" }
  | { type: "interact-room"; interactionId: string }
  | { type: "retreat-dungeon" }
  | { type: "enter-dungeon-assist" }
  | { type: "select-assist-hero"; heroId: string }
  | { type: "use-assist-action"; actionId: string }
  | { type: "continue-from-dungeon" }
  | { type: "enter-room"; roomId: string }
  | { type: "retreat-from-dungeon" }
  | { type: "complete-dungeon" }
  | { type: "open-dungeon-items"; returnFlowState?: "dungeon-interaction" | "dungeon-map" | "combat" }
  | { type: "close-dungeon-items" }
  | { type: "select-dungeon-item"; itemId: string }
  | { type: "select-dungeon-item-target"; heroId: string }
  | { type: "use-dungeon-item"; itemId: string; heroId?: string }
  | { type: "return-to-town" }
  | { type: "continue-from-result" }
  | { type: "resume-from-return" }
  | { type: "continue-from-combat" }
  | { type: "open-combat-settings" }
  | { type: "select-skill"; skillId: string }
  | { type: "select-target"; enemyId: string }
  | { type: "confirm-attack" }
  | { type: "flee-combat" }
  | { type: "end-turn" };
