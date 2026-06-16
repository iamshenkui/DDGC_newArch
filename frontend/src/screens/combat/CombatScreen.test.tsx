// @vitest-environment jsdom

import { render } from "solid-js/web";
import { afterEach, describe, expect, it, vi } from "vitest";

import { ReplayRuntimeBridge } from "../../bridge/ReplayRuntimeBridge";
import type { CombatViewModel } from "../../bridge/contractTypes";
import { firstCombatDemoViewModel, replayAttackCombatViewModel, replayCombatViewModel } from "../../validation/replayFixtures";
import { CombatScreen } from "./CombatScreen";

describe("CombatScreen skill interactions", () => {
  let dispose: (() => void) | undefined;

  afterEach(() => {
    dispose?.();
    dispose = undefined;
    document.body.innerHTML = "";
  });

  it("shows hovered skill details without changing the selected skill", () => {
    const onSelectSkill = vi.fn();
    const root = document.createElement("div");
    document.body.appendChild(root);

    dispose = render(
      () => (
        <CombatScreen
          viewModel={replayAttackCombatViewModel}
          onSelectSkill={onSelectSkill}
          onSelectTarget={vi.fn()}
          onConfirmAttack={vi.fn()}
          onFleeCombat={vi.fn()}
          onEndTurn={vi.fn()}
        />
      ),
      root
    );

    const rapidShotButton = root.querySelector<HTMLButtonElement>(
      '.combat-skill-slot[title="Rapid Shot"]'
    );
    expect(rapidShotButton).not.toBeNull();

    rapidShotButton?.dispatchEvent(new MouseEvent("mouseenter"));

    const tooltip = root.querySelector(".combat-skill-tooltip");
    expect(tooltip?.textContent).toContain("Rapid Shot");
    expect(tooltip?.textContent).toContain("Fire two quick shots at the target.");
    expect(tooltip?.textContent).not.toContain("Hunting Bow");
    expect(onSelectSkill).not.toHaveBeenCalled();

    rapidShotButton?.click();
    expect(onSelectSkill).toHaveBeenCalledWith("skill-2");
  });

  it("routes character-hit acknowledgement through continue combat", () => {
    const onContinueCombat = vi.fn();
    const onEndTurn = vi.fn();
    const onSelectSkill = vi.fn();
    const onSelectTarget = vi.fn();
    const onConfirmAttack = vi.fn();
    const root = document.createElement("div");
    document.body.appendChild(root);

    dispose = render(
      () => (
        <CombatScreen
          viewModel={replayCombatViewModel}
          onSelectSkill={onSelectSkill}
          onSelectTarget={onSelectTarget}
          onConfirmAttack={onConfirmAttack}
          onFleeCombat={vi.fn()}
          onEndTurn={onEndTurn}
          onContinueCombat={onContinueCombat}
        />
      ),
      root
    );

    const acknowledgeButton = root.querySelector<HTMLButtonElement>('[data-testid="combat-continue-btn"]');
    expect(acknowledgeButton?.textContent).toContain("Acknowledge");

    acknowledgeButton?.click();
    expect(onContinueCombat).toHaveBeenCalledOnce();

    const endTurnButton = root.querySelector<HTMLButtonElement>(".combat-end-turn-btn");
    expect(endTurnButton?.disabled).toBe(true);
    endTurnButton?.click();
    expect(onEndTurn).not.toHaveBeenCalled();

    const fleeButton = root.querySelector<HTMLButtonElement>(".combat-flee-btn");
    expect(fleeButton?.disabled).toBe(true);

    const skillButton = root.querySelector<HTMLButtonElement>(".combat-skill-slot");
    expect(skillButton?.disabled).toBe(true);
    skillButton?.click();
    expect(onSelectSkill).not.toHaveBeenCalled();

    const targetButton = root.querySelector<HTMLButtonElement>(".combat-target-cell");
    expect(targetButton?.disabled).toBe(true);
    targetButton?.click();
    expect(onSelectTarget).not.toHaveBeenCalled();

    const arenaEnemy = root.querySelector<HTMLElement>(".combat-enemy-stand");
    expect(arenaEnemy?.getAttribute("aria-disabled")).toBe("true");
    arenaEnemy?.click();
    arenaEnemy?.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter" }));
    expect(onSelectTarget).not.toHaveBeenCalled();
    expect(onConfirmAttack).not.toHaveBeenCalled();
  });

  it("renders the first-combat demo with original families and mantis flower enemies", () => {
    const onSelectSkill = vi.fn();
    const onSelectTarget = vi.fn();
    const root = document.createElement("div");
    document.body.appendChild(root);

    dispose = render(
      () => (
        <CombatScreen
          viewModel={firstCombatDemoViewModel}
          onSelectSkill={onSelectSkill}
          onSelectTarget={onSelectTarget}
          onConfirmAttack={vi.fn()}
          onFleeCombat={vi.fn()}
          onEndTurn={vi.fn()}
        />
      ),
      root
    );

    const classLabels = Array.from(root.querySelectorAll(".combat-hero-class")).map(
      (el) => el.textContent
    );
    expect(classLabels).toContain("Hunter");
    expect(classLabels).toContain("Alchemist");
    expect(classLabels).toContain("Diviner");
    expect(classLabels).toContain("Shaman");
    expect(classLabels).toContain("Tank");

    const enemyNames = Array.from(root.querySelectorAll(".combat-enemy-name")).map(
      (el) => el.textContent
    );
    expect(enemyNames).toContain("Mantis Walking Flower");
    expect(enemyNames).toContain("Mantis Spiny Flower");
    expect(enemyNames).toContain("Mantis Magic Flower");

    const activeSkillButton = root.querySelector<HTMLButtonElement>(
      '.combat-skill-slot[title="Hunting Bow"]'
    );
    activeSkillButton?.click();
    expect(onSelectSkill).toHaveBeenCalledWith("skill-hunter-1");

    const targetCell = root.querySelector<HTMLButtonElement>(
      '[data-enemy-id="enemy-mantis-spiny-01"]'
    );
    targetCell?.click();
    expect(onSelectTarget).toHaveBeenCalledWith("enemy-mantis-spiny-01");
  });

  it("reaches the first-combat demo through the replay bridge and selects a skill", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();

    const snapshot = await bridge.dispatchIntent({ type: "start-first-combat-demo" });
    expect(snapshot.flowState).toBe("combat");
    expect(snapshot.viewModel.kind).toBe("combat");

    const demoVm = snapshot.viewModel as CombatViewModel;
    const onSelectSkill = vi.fn();
    const root = document.createElement("div");
    document.body.appendChild(root);

    dispose = render(
      () => (
        <CombatScreen
          viewModel={demoVm}
          onSelectSkill={onSelectSkill}
          onSelectTarget={vi.fn()}
          onConfirmAttack={vi.fn()}
          onFleeCombat={vi.fn()}
          onEndTurn={vi.fn()}
        />
      ),
      root
    );

    const skillButton = root.querySelector<HTMLButtonElement>(
      '.combat-skill-slot[title="Hunting Bow"]'
    );
    expect(skillButton).not.toBeNull();
    skillButton?.click();
    expect(onSelectSkill).toHaveBeenCalledWith("skill-hunter-1");
  });
});
