// @vitest-environment jsdom

import { render } from "solid-js/web";
import { afterEach, describe, expect, it, vi } from "vitest";

import { replayAttackCombatViewModel, replayCombatViewModel } from "../../validation/replayFixtures";
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
});
