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
  CombatViewModel
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
          flowState: "combat",
          viewModel: createLiveCombatViewModel()
        };
        break;
      case "select-skill": {
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
        const combatVm = this.snapshot.viewModel as CombatViewModel;
        this.snapshot = {
          ...this.snapshot,
          viewModel: {
            ...combatVm,
            enemies: combatVm.enemies.map((e) => ({
              ...e,
              isTargeted: e.id === intent.enemyId
            }))
          }
        };
        break;
      }
      case "confirm-attack":
      case "end-turn":
      case "flee-combat":
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