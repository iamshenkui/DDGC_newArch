import type { RuntimeMode } from "../app/runtimeMode";
import type { RuntimeBridge, RuntimeBridgeListener } from "./RuntimeBridge";
import type {
  DdgcFrontendIntent,
  DdgcFrontendSnapshot,
  TownViewModel,
  TownHeroSummary,
  TownBuildingSummary,
  HeroDetailViewModel,
} from "./contractTypes";

const createLiveTownViewModel = (): TownViewModel => ({
  kind: "town",
  title: "Town Surface — Live Mode",
  campaignName: "Fresh Campaign",
  campaignSummary:
    "Live runtime boot: DDGC host initialized with fresh campaign state. Roster and building data reflects initial campaign setup.",
  heroes: [
    {
      id: "hero-hunter-live-01",
      name: "Yuan",
      classLabel: "Hunter",
      hp: "42 / 42",
      stress: "0",
      level: 1
    },
    {
      id: "hero-white-live-01",
      name: "Mei",
      classLabel: "White",
      hp: "41 / 41",
      stress: "0",
      level: 1
    }
  ] as ReadonlyArray<TownHeroSummary>,
  buildings: [
    {
      id: "stagecoach",
      label: "Stagecoach",
      summary: "Recruit new heroes to your party.",
      status: "ready"
    },
    {
      id: "guild",
      label: "Guild",
      summary: "Skill training and party capability review.",
      status: "ready"
    }
  ] as ReadonlyArray<TownBuildingSummary>,
  nextActionLabel: "Launch Expedition"
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
  progression: {
    level: hero.level,
    experience: "0",
    experienceToNext: "300"
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
  combatSkills: ["Skill 1", "Skill 2"],
  campingSkills: ["Campfire Song"],
  weapon: "Basic Weapon",
  armor: "Leather Armor",
  campNotes: "Hero detail view - live mode placeholder."
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
      case "open-building":
        this.snapshot = {
          ...this.snapshot,
          debugMessage: `Live: open building intent received for ${intent.buildingId}.`
        };
        break;
      case "start-provisioning":
        this.snapshot = {
          ...this.snapshot,
          debugMessage: "Live: provisioning flow placeholder requested."
        };
        break;
      case "launch-expedition":
        this.snapshot = {
          ...this.snapshot,
          flowState: "expedition",
          debugMessage:
            "Live: expedition handoff placeholder. Real runtime transition in subsequent Phase 10 iterations."
        };
        break;
      case "return-to-town":
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