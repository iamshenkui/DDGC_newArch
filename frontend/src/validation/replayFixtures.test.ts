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
