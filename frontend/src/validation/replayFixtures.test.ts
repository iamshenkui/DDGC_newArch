import { describe, expect, it } from "vitest";

import {
  replayTownViewModel,
  replayHeroDetailViewModel,
  replayBuildingDetailViewModel,
  replayProvisioningViewModel,
  replayExpeditionViewModel,
  replayResultViewModel,
  replayFailureResultViewModel,
  replayPartialResultViewModel,
  replayReturnViewModel,
  validateSnapshotContract,
  replayReadySnapshot,
  replayHeroDetailSnapshot,
  replayBuildingDetailSnapshot,
  replayBlacksmithBuildingSnapshot,
  replaySanitariumBuildingSnapshot,
  unsupportedSnapshot,
  fatalSnapshot,
  replayLoadingSnapshot,
  liveLoadingSnapshot,
  startupSnapshot,
  provisioningSnapshot,
  expeditionSnapshot,
  resultSnapshot,
  failureResultSnapshot,
  partialResultSnapshot,
  returnSnapshot,
  replayBlacksmithBuildingDetailViewModel,
  replaySanitariumBuildingDetailViewModel,
  replayStagecoachBuildingDetailViewModel,
  replayStagecoachBuildingSnapshot,
} from "./replayFixtures";

describe("replay fixtures — hero and campaign state consistency", () => {
  describe("town fixture roster", () => {
    it("has three heroes with valid data", () => {
      const heroes = replayTownViewModel.heroes;
      expect(heroes.length).toBe(3);

      for (const hero of heroes) {
        expect(hero.id).toBeTruthy();
        expect(hero.name).toBeTruthy();
        expect(hero.classLabel).toBeTruthy();
        expect(hero.level).toBeGreaterThanOrEqual(1);

        // HP format: "current / max"
        const hpParts = hero.hp.split("/");
        expect(hpParts.length).toBe(2);
        const current = Number(hpParts[0].trim());
        const max = Number(hpParts[1].trim());
        expect(current).toBeGreaterThan(0);
        expect(max).toBeGreaterThan(0);
        expect(current).toBeLessThanOrEqual(max);

        // Stress is a non-negative integer
        const stress = Number(hero.stress);
        expect(stress).toBeGreaterThanOrEqual(0);
        expect(Number.isInteger(stress)).toBe(true);
      }
    });

    it("exposes class label and level for roster progression signal", () => {
      for (const hero of replayTownViewModel.heroes) {
        expect(hero.classLabel).toMatch(/^[A-Z]/);
        expect(typeof hero.level).toBe("number");
      }
    });

    it("exposes HP as parseable health signals", () => {
      const shen = replayTownViewModel.heroes.find((h) => h.id === "hero-hunter-01");
      expect(shen).toBeDefined();
      const hp = shen!.hp.split("/").map((s) => Number(s.trim()));
      expect(hp[0]).toBeLessThan(hp[1]); // Shen has 38/42 — damaged
    });

    it("exposes numeric health and maxHealth fields", () => {
      for (const hero of replayTownViewModel.heroes) {
        expect(hero.health).toBeGreaterThan(0);
        expect(hero.maxHealth).toBeGreaterThan(0);
        expect(hero.health).toBeLessThanOrEqual(hero.maxHealth);
      }
    });

    it("exposes maxHp string for direct display", () => {
      for (const hero of replayTownViewModel.heroes) {
        expect(hero.maxHp).toBeTruthy();
        expect(Number(hero.maxHp)).toBeGreaterThan(0);
      }
    });

    it("exposes maxStress for stress bar context", () => {
      for (const hero of replayTownViewModel.heroes) {
        expect(hero.maxStress).toBeTruthy();
        expect(Number(hero.maxStress)).toBeGreaterThan(0);
      }
    });

    it("has correct wounded and afflicted flags", () => {
      const shen = replayTownViewModel.heroes.find((h) => h.id === "hero-hunter-01")!;
      expect(shen.isWounded).toBe(true); // 38/42
      expect(shen.isAfflicted).toBe(false); // 17/200

      const baiXiu = replayTownViewModel.heroes.find((h) => h.id === "hero-white-01")!;
      expect(baiXiu.isWounded).toBe(false); // 41/41 — full health
      expect(baiXiu.isAfflicted).toBe(false);
    });

    it("has XP for progression signal in roster view", () => {
      for (const hero of replayTownViewModel.heroes) {
        expect(hero.xp).toBeGreaterThanOrEqual(0);
      }
    });

    it("has quirk and disease lists for pre-expedition triage", () => {
      for (const hero of replayTownViewModel.heroes) {
        expect(Array.isArray(hero.positiveQuirks)).toBe(true);
        expect(Array.isArray(hero.negativeQuirks)).toBe(true);
        expect(Array.isArray(hero.diseases)).toBe(true);
      }
    });

    it("hero-hunter-01 has expected quirks and diseases", () => {
      const shen = replayTownViewModel.heroes.find((h) => h.id === "hero-hunter-01")!;
      expect(shen.positiveQuirks.length).toBeGreaterThan(0);
      expect(shen.negativeQuirks.length).toBeGreaterThan(0);
      expect(shen.diseases.length).toBe(0);
    });

    it("hero-black-01 has disease for triage signal", () => {
      const heiZhen = replayTownViewModel.heroes.find((h) => h.id === "hero-black-01")!;
      expect(heiZhen.diseases.length).toBeGreaterThan(0);
    });
  });

  describe("town view model enrichment", () => {
    it("has gold and fresh visit flag", () => {
      expect(replayTownViewModel.gold).toBeGreaterThan(0);
      expect(replayTownViewModel.isFreshVisit).toBe(true);
    });

    it("has roster field matching heroes", () => {
      expect(replayTownViewModel.roster.length).toBe(replayTownViewModel.heroes.length);
      expect(replayTownViewModel.roster[0].id).toBe("hero-hunter-01");
    });

    it("roster heroes have same health fields as heroes", () => {
      for (const hero of replayTownViewModel.roster) {
        expect(hero.health).toBeGreaterThan(0);
        expect(hero.maxHealth).toBeGreaterThan(0);
        expect(typeof hero.isWounded).toBe("boolean");
        expect(typeof hero.isAfflicted).toBe("boolean");
      }
    });
  });

  describe("provisioning hero enrichment", () => {
    it("provisioning heroes have health and status fields", () => {
      for (const ph of replayProvisioningViewModel.party) {
        expect(ph.health).toBeGreaterThan(0);
        expect(ph.maxHealth).toBeGreaterThan(0);
        expect(ph.maxHp).toBeTruthy();
        expect(ph.maxStress).toBeTruthy();
        expect(typeof ph.isWounded).toBe("boolean");
        expect(typeof ph.isAfflicted).toBe("boolean");
        expect(ph.xp).toBeGreaterThanOrEqual(0);
      }
    });

    it("provisioning hero field values are consistent with town roster", () => {
      for (const ph of replayProvisioningViewModel.party) {
        const townHero = replayTownViewModel.heroes.find((h) => h.id === ph.id);
        expect(townHero).toBeDefined();
        expect(ph.health).toBe(townHero!.health);
        expect(ph.maxHealth).toBe(townHero!.maxHealth);
        expect(ph.isWounded).toBe(townHero!.isWounded);
        expect(ph.isAfflicted).toBe(townHero!.isAfflicted);
        expect(ph.xp).toBe(townHero!.xp);
      }
    });
  });

  describe("hero-detail fixture", () => {
    it("is consistent with town roster hero", () => {
      const detail = replayHeroDetailViewModel;
      const townHero = replayTownViewModel.heroes.find((h) => h.id === detail.heroId);
      expect(townHero).toBeDefined();
      expect(detail.name).toBe(townHero!.name);
      expect(detail.classLabel).toBe(townHero!.classLabel);

      // Detail HP should match town roster HP
      const townHp = townHero!.hp.split("/").map((s) => s.trim());
      expect(detail.hp).toBe(townHp[0]);
      expect(detail.maxHp).toBe(townHp[1]);
    });

    it("has progression data for campaign decisions", () => {
      const prog = replayHeroDetailViewModel.progression;
      expect(prog.level).toBeGreaterThan(0);
      expect(prog.experience).toBeTruthy();
      expect(prog.experienceToNext).toBeTruthy();
    });

    it("has resistances for all seven categories", () => {
      const res = replayHeroDetailViewModel.resistances;
      const categories = ["stun", "bleed", "disease", "move", "death", "trap", "hazard"];
      for (const cat of categories) {
        expect(res[cat as keyof typeof res]).toMatch(/\d+%/);
      }
    });

    it("has at least one combat and camping skill", () => {
      expect(replayHeroDetailViewModel.combatSkills.length).toBeGreaterThan(0);
      expect(replayHeroDetailViewModel.campingSkills.length).toBeGreaterThan(0);
    });

    it("has equipment progression signals", () => {
      expect(replayHeroDetailViewModel.weapon).toBeTruthy();
      expect(replayHeroDetailViewModel.armor).toBeTruthy();
    });
  });

  describe("building-detail fixture", () => {
    it("is consistent with town building", () => {
      const detail = replayBuildingDetailViewModel;
      const townBuilding = replayTownViewModel.buildings.find(
        (b) => b.id === detail.buildingId,
      );
      expect(townBuilding).toBeDefined();
      expect(detail.label).toBe(townBuilding!.label);
      expect(detail.status).toBe(townBuilding!.status);
    });

    it("has available actions with cost and availability flags", () => {
      const actions = replayBuildingDetailViewModel.actions;
      expect(actions.length).toBeGreaterThan(0);

      for (const action of actions) {
        expect(action.id).toBeTruthy();
        expect(action.label).toBeTruthy();
        expect(action.description).toBeTruthy();
        expect(action.cost).toBeTruthy();
        expect(typeof action.isAvailable).toBe("boolean");
        expect(typeof action.isUnsupported).toBe("boolean");
      }
    });
  });

  describe("provisioning fixture", () => {
    it("heroes are consistent with town roster", () => {
      for (const ph of replayProvisioningViewModel.party) {
        const townHero = replayTownViewModel.heroes.find((h) => h.id === ph.id);
        expect(townHero).toBeDefined();
        expect(ph.name).toBe(townHero!.name);
        expect(ph.classLabel).toBe(townHero!.classLabel);
        expect(ph.hp).toBe(townHero!.hp);
        expect(ph.level).toBe(townHero!.level);
      }
    });

    it("has valid provisioning parameters", () => {
      const vm = replayProvisioningViewModel;
      expect(vm.maxPartySize).toBeGreaterThan(0);
      expect(vm.party.length).toBeLessThanOrEqual(vm.maxPartySize);
      expect(typeof vm.isReadyToLaunch).toBe("boolean");
      expect(vm.supplyLevel).toBeTruthy();
      expect(vm.provisionCost).toBeTruthy();
    });
  });

  describe("expedition fixture", () => {
    it("has valid expedition parameters", () => {
      const vm = replayExpeditionViewModel;
      expect(vm.partySize).toBeGreaterThan(0);
      expect(vm.difficulty).toBeTruthy();
      expect(vm.estimatedDuration).toBeTruthy();
      expect(vm.objectives.length).toBeGreaterThan(0);
      expect(typeof vm.isLaunchable).toBe("boolean");
    });
  });

  describe("result fixtures", () => {
    it("success result has loot and positive resources", () => {
      expect(replayResultViewModel.outcome).toBe("success");
      expect(replayResultViewModel.lootAcquired.length).toBeGreaterThan(0);
      expect(replayResultViewModel.resourcesGained.gold).toBeGreaterThan(0);
    });

    it("failure result has no loot and zero gold", () => {
      expect(replayFailureResultViewModel.outcome).toBe("failure");
      expect(replayFailureResultViewModel.lootAcquired.length).toBe(0);
      expect(replayFailureResultViewModel.resourcesGained.gold).toBe(0);
    });

    it("partial result has mixed outcomes", () => {
      expect(replayPartialResultViewModel.outcome).toBe("partial");
      const statuses = replayPartialResultViewModel.heroOutcomes.map((h) => h.status);
      expect(statuses).toContain("alive");
      expect(statuses).toContain("stressed");
    });

    it("all result fixtures have hero outcomes with status and HP/stress changes", () => {
      for (const result of [
        replayResultViewModel,
        replayFailureResultViewModel,
        replayPartialResultViewModel,
      ]) {
        for (const hero of result.heroOutcomes) {
          expect(hero.heroId).toBeTruthy();
          expect(hero.heroName).toBeTruthy();
          expect(["alive", "dead", "stressed"]).toContain(hero.status);
          expect(hero.hpChange).toBeTruthy();
          expect(hero.stressChange).toBeTruthy();
        }
      }
    });

    it("all result fixtures have isContinueAvailable for meta-loop", () => {
      expect(replayResultViewModel.isContinueAvailable).toBe(true);
      expect(replayFailureResultViewModel.isContinueAvailable).toBe(true);
      expect(replayPartialResultViewModel.isContinueAvailable).toBe(true);
    });
  });

  describe("return fixture", () => {
    it("heroes match town roster", () => {
      for (const hero of replayReturnViewModel.returningHeroes) {
        const townHero = replayTownViewModel.heroes.find(
          (h) => h.id === hero.heroId,
        );
        expect(townHero).toBeDefined();
        expect(hero.heroName).toBe(townHero!.name);
      }
    });

    it("has isTownResumeAvailable for meta-loop", () => {
      expect(replayReturnViewModel.isTownResumeAvailable).toBe(true);
    });
  });
});

interface NamedSnapshot {
  name: string;
  snapshot: import("../bridge/contractTypes").DdgcFrontendSnapshot;
}

const allSnapshots: NamedSnapshot[] = [
  { name: "startupSnapshot", snapshot: startupSnapshot },
  { name: "replayLoadingSnapshot", snapshot: replayLoadingSnapshot },
  { name: "liveLoadingSnapshot", snapshot: liveLoadingSnapshot },
  { name: "replayReadySnapshot", snapshot: replayReadySnapshot },
  { name: "replayHeroDetailSnapshot", snapshot: replayHeroDetailSnapshot },
  { name: "replayBuildingDetailSnapshot", snapshot: replayBuildingDetailSnapshot },
  { name: "replayBlacksmithBuildingSnapshot", snapshot: replayBlacksmithBuildingSnapshot },
  { name: "replaySanitariumBuildingSnapshot", snapshot: replaySanitariumBuildingSnapshot },
  { name: "replayStagecoachBuildingSnapshot", snapshot: replayStagecoachBuildingSnapshot },
  { name: "provisioningSnapshot", snapshot: provisioningSnapshot },
  { name: "expeditionSnapshot", snapshot: expeditionSnapshot },
  { name: "resultSnapshot", snapshot: resultSnapshot },
  { name: "failureResultSnapshot", snapshot: failureResultSnapshot },
  { name: "partialResultSnapshot", snapshot: partialResultSnapshot },
  { name: "returnSnapshot", snapshot: returnSnapshot },
  { name: "unsupportedSnapshot", snapshot: unsupportedSnapshot },
  { name: "fatalSnapshot", snapshot: fatalSnapshot },
];

// ── Snapshot contract boundary validation ─────────────────────────────────

describe("snapshot contract validation", () => {

  for (const { name, snapshot } of allSnapshots) {
    it(`"${name}" satisfies the DdgcFrontendSnapshot contract`, () => {
      const errors = validateSnapshotContract(snapshot);
      expect(errors, `${name}: ${errors.join("; ")}`).toEqual([]);
    });
  }

  it("all snapshots provide a debugMessage for actionable debugging", () => {
    for (const { name, snapshot } of allSnapshots) {
      expect(
        snapshot.debugMessage,
        `${name} is missing debugMessage`,
      ).toBeTruthy();
      expect(
        typeof snapshot.debugMessage,
        `${name} debugMessage should be a string, got ${typeof snapshot.debugMessage}`,
      ).toBe("string");
    }
  });
});

// ── Type discrimination validation ────────────────────────────────────────

describe("type discrimination", () => {
  it("fatal lifecycle always pairs with fatal view model kind", () => {
    expect(fatalSnapshot.lifecycle).toBe("fatal");
    expect(fatalSnapshot.viewModel.kind).toBe("fatal");
  });

  it("unsupported lifecycle always pairs with unsupported view model kind", () => {
    expect(unsupportedSnapshot.lifecycle).toBe("unsupported");
    expect(unsupportedSnapshot.viewModel.kind).toBe("unsupported");
  });

  it("loading lifecycle always pairs with boot-load view model kind", () => {
    expect(replayLoadingSnapshot.lifecycle).toBe("loading");
    expect(replayLoadingSnapshot.viewModel.kind).toBe("boot-load");
    expect(liveLoadingSnapshot.lifecycle).toBe("loading");
    expect(liveLoadingSnapshot.viewModel.kind).toBe("boot-load");
  });

  it("boot flowState pairs with boot-load view model kind", () => {
    expect(startupSnapshot.flowState).toBe("boot");
    expect(startupSnapshot.viewModel.kind).toBe("boot-load");
  });

  it("load flowState pairs with boot-load view model kind", () => {
    expect(replayLoadingSnapshot.flowState).toBe("load");
    expect(replayLoadingSnapshot.viewModel.kind).toBe("boot-load");
    expect(liveLoadingSnapshot.flowState).toBe("load");
    expect(liveLoadingSnapshot.viewModel.kind).toBe("boot-load");
  });

  it("ready lifecycle never pairs with fatal or unsupported kind", () => {
    const readySnapshots = allSnapshots
      .filter((s) => s.snapshot.lifecycle === "ready")
      .map((s) => s.snapshot);
    for (const snap of readySnapshots) {
      expect(snap.viewModel.kind).not.toBe("fatal");
      expect(snap.viewModel.kind).not.toBe("unsupported");
    }
  });

  it("result snapshots consistently set flowState and kind to result", () => {
    for (const snap of [resultSnapshot, failureResultSnapshot, partialResultSnapshot]) {
      expect(snap.flowState).toBe("result");
      expect(snap.viewModel.kind).toBe("result");
    }
  });

  it("provisioning snapshot sets flowState and kind to provisioning", () => {
    expect(provisioningSnapshot.flowState).toBe("provisioning");
    expect(provisioningSnapshot.viewModel.kind).toBe("provisioning");
  });

  it("expedition snapshot sets flowState and kind to expedition", () => {
    expect(expeditionSnapshot.flowState).toBe("expedition");
    expect(expeditionSnapshot.viewModel.kind).toBe("expedition");
  });

  it("return snapshot sets flowState and kind to return", () => {
    expect(returnSnapshot.flowState).toBe("return");
    expect(returnSnapshot.viewModel.kind).toBe("return");
  });
});

// ── HP string format validation ───────────────────────────────────────────

describe("HP string format consistency across fixtures", () => {
  function checkHpFormat(hp: string, label: string): void {
    expect(hp, `${label}: HP "${hp}" should contain "/"`).toContain("/");
    const parts = hp.split("/").map((s) => s.trim());
    expect(parts.length, `${label}: HP "${hp}" should split into 2 parts`).toBe(2);
    const current = Number(parts[0]);
    const max = Number(parts[1]);
    expect(Number.isInteger(current), `${label}: HP current "${parts[0]}" is not an integer`).toBe(true);
    expect(Number.isInteger(max), `${label}: HP max "${parts[1]}" is not an integer`).toBe(true);
    expect(current, `${label}: HP current ${current} should be > 0`).toBeGreaterThan(0);
    expect(max, `${label}: HP max ${max} should be > 0`).toBeGreaterThan(0);
    expect(current, `${label}: HP current ${current} should be ≤ max ${max}`).toBeLessThanOrEqual(max);
  }

  it("town roster heroes have valid HP strings", () => {
    for (const hero of replayTownViewModel.heroes) {
      checkHpFormat(hero.hp, `town hero ${hero.id}`);
    }
  });

  it("provisioning heroes have valid HP strings", () => {
    for (const ph of replayProvisioningViewModel.party) {
      checkHpFormat(ph.hp, `provisioning hero ${ph.id}`);
    }
  });

  it("expedition heroes have valid HP strings", () => {
    for (const eh of replayExpeditionViewModel.party) {
      checkHpFormat(eh.hp, `expedition hero ${eh.id}`);
    }
  });

  it("returning heroes have valid HP strings", () => {
    for (const rh of replayReturnViewModel.returningHeroes) {
      checkHpFormat(rh.hp, `return hero ${rh.heroId}`);
    }
  });

  it("hero-detail HP and maxHp match town roster", () => {
    const detail = replayHeroDetailViewModel;
    const townHero = replayTownViewModel.heroes.find((h) => h.id === detail.heroId)!;
    const townHpParts = townHero.hp.split("/").map((s) => s.trim());
    expect(detail.hp).toBe(townHpParts[0]);
    expect(detail.maxHp).toBe(townHpParts[1]);
  });
});

// ── Cross-fixture building consistency ────────────────────────────────────

describe("building fixture consistency with town roster", () => {
  const townBuildings = replayTownViewModel.buildings;

  it.each([
    ["replayBuildingDetailViewModel", replayBuildingDetailViewModel, "guild"],
    ["replayBlacksmithBuildingDetailViewModel", replayBlacksmithBuildingDetailViewModel, "blacksmith"],
    ["replaySanitariumBuildingDetailViewModel", replaySanitariumBuildingDetailViewModel, "sanitarium"],
    ["replayStagecoachBuildingDetailViewModel", replayStagecoachBuildingDetailViewModel, "stagecoach"],
  ] as const)("%s references an existing town building", (_name, detail, expectedId) => {
    const townB = townBuildings.find((b) => b.id === expectedId);
    expect(townB, `town building "${expectedId}" not found in town view model`).toBeDefined();
    expect(detail.buildingId).toBe(expectedId);
    expect(detail.label).toBe(townB!.label);
    expect(detail.status).toBe(townB!.status);
  });

  it("all town buildings have a corresponding detail fixture", () => {
    const knownBuildingFixtures = new Set([
      replayBuildingDetailViewModel.buildingId,
      replayBlacksmithBuildingDetailViewModel.buildingId,
      replaySanitariumBuildingDetailViewModel.buildingId,
      replayStagecoachBuildingDetailViewModel.buildingId,
    ]);
    for (const tb of townBuildings) {
      expect(
        knownBuildingFixtures.has(tb.id),
        `town building "${tb.id}" (${tb.label}) has no detail fixture`,
      ).toBe(true);
    }
  });

  it("building detail fixtures have descriptive non-empty descriptions", () => {
    for (const detail of [
      replayBuildingDetailViewModel,
      replayBlacksmithBuildingDetailViewModel,
      replaySanitariumBuildingDetailViewModel,
      replayStagecoachBuildingDetailViewModel,
    ]) {
      expect(detail.description.length).toBeGreaterThan(20);
      expect(detail.actions.length).toBeGreaterThan(0);
    }
  });

  it("building detail actions have valid cost strings", () => {
    for (const detail of [
      replayBuildingDetailViewModel,
      replayBlacksmithBuildingDetailViewModel,
      replaySanitariumBuildingDetailViewModel,
      replayStagecoachBuildingDetailViewModel,
    ]) {
      for (const action of detail.actions) {
        expect(action.cost).toMatch(/^\d+ Gold$/);
      }
    }
  });
});

// ── Cross-fixture hero consistency ────────────────────────────────────────

describe("hero data consistency across fixtures", () => {
  it("town roster heroes match provisioning and expedition hero IDs", () => {
    const townIds = new Set(replayTownViewModel.heroes.map((h) => h.id));
    for (const ph of replayProvisioningViewModel.party) {
      expect(townIds.has(ph.id), `provisioning hero ${ph.id} not in town roster`).toBe(true);
    }
    for (const eh of replayExpeditionViewModel.party) {
      expect(townIds.has(eh.id), `expedition hero ${eh.id} not in town roster`).toBe(true);
    }
  });

  it("result hero outcomes reference known town heroes", () => {
    const townIds = new Set(replayTownViewModel.heroes.map((h) => h.id));
    for (const result of [replayResultViewModel, replayFailureResultViewModel, replayPartialResultViewModel]) {
      for (const ho of result.heroOutcomes) {
        expect(townIds.has(ho.heroId), `result hero ${ho.heroId} not in town roster`).toBe(true);
        const townHero = replayTownViewModel.heroes.find((h) => h.id === ho.heroId)!;
        expect(ho.heroName).toBe(townHero.name);
      }
    }
  });

  it("result hero outcomes include both selected party heroes", () => {
    for (const result of [replayResultViewModel, replayFailureResultViewModel, replayPartialResultViewModel]) {
      const outcomeIds = new Set(result.heroOutcomes.map((h) => h.heroId));
      for (const eh of replayExpeditionViewModel.party) {
        expect(
          outcomeIds.has(eh.id),
          `result missing outcome for expedition hero ${eh.id}`,
        ).toBe(true);
      }
    }
  });
});

// ── HP/health numeric consistency ─────────────────────────────────────────

describe("health and stress numeric consistency", () => {
  it("hero health field matches parsed HP current value", () => {
    for (const hero of replayTownViewModel.heroes) {
      const hpCurrent = Number(hero.hp.split("/")[0].trim());
      expect(hero.health, `${hero.id}: health ${hero.health} should match parsed HP ${hpCurrent}`).toBe(hpCurrent);
    }
  });

  it("hero maxHealth field matches parsed HP max value", () => {
    for (const hero of replayTownViewModel.heroes) {
      const hpMax = Number(hero.hp.split("/")[1].trim());
      expect(hero.maxHealth, `${hero.id}: maxHealth ${hero.maxHealth} should match parsed max HP ${hpMax}`).toBe(hpMax);
    }
  });

  it("provisioning hero health matches town hero health", () => {
    for (const ph of replayProvisioningViewModel.party) {
      const townHero = replayTownViewModel.heroes.find((h) => h.id === ph.id)!;
      expect(ph.health).toBe(townHero.health);
      expect(ph.maxHealth).toBe(townHero.maxHealth);
    }
  });
});
