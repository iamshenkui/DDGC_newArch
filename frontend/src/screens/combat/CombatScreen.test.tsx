// @vitest-environment jsdom

import { render } from "solid-js/web";
import { afterEach, describe, expect, it, vi } from "vitest";

import { replayCombatViewModel } from "../../validation/replayFixtures";
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
          viewModel={replayCombatViewModel}
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
});
