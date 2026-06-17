// @vitest-environment jsdom

import { render } from "solid-js/web";
import { afterEach, describe, expect, it, vi } from "vitest";

import { ReplayRuntimeBridge } from "../../bridge/ReplayRuntimeBridge";
import type { CombatViewModel } from "../../bridge/contractTypes";
import {
  replayAttackCombatViewModel,
  replayCombatViewModel,
  replayFirstCombatViewModel
} from "../../validation/replayFixtures";
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

  it("reaches character-hit through the replay bridge and renders the hit UI", async () => {
    const bridge = new ReplayRuntimeBridge();
    await bridge.boot();
    await bridge.dispatchIntent({ type: "start-provisioning" });
    await bridge.dispatchIntent({ type: "confirm-provisioning" });
    await bridge.dispatchIntent({ type: "accept-dungeon-hint" });
    await bridge.dispatchIntent({ type: "launch-expedition" });
    await bridge.dispatchIntent({ type: "enter-dungeon-assist" });
    await bridge.dispatchIntent({ type: "use-assist-action", actionId: "heal-wound" });
    await bridge.dispatchIntent({ type: "continue-from-dungeon" });
    await bridge.dispatchIntent({ type: "enter-room", roomId: "room-combat-1" });

    const hitSnapshot = await bridge.dispatchIntent({ type: "confirm-attack" });
    expect(hitSnapshot.viewModel.kind).toBe("combat");
    const hitVm = hitSnapshot.viewModel as CombatViewModel;
    expect(hitVm.phase).toBe("character-hit");

    const onContinueCombat = vi.fn();
    const root = document.createElement("div");
    document.body.appendChild(root);

    dispose = render(
      () => (
        <CombatScreen
          viewModel={hitVm}
          onSelectSkill={vi.fn()}
          onSelectTarget={vi.fn()}
          onConfirmAttack={vi.fn()}
          onFleeCombat={vi.fn()}
          onEndTurn={vi.fn()}
          onContinueCombat={onContinueCombat}
        />
      ),
      root
    );

    const hudPill = root.querySelector(".pill-danger");
    expect(hudPill?.textContent).toContain("Character Hit");

    const damageFloater = root.querySelector('[data-testid="combat-hit-damage"]');
    expect(damageFloater).not.toBeNull();
    expect(damageFloater?.textContent).toContain(hitVm.hitDamage ?? "");

    const acknowledgeBtn = root.querySelector<HTMLButtonElement>('[data-testid="combat-continue-btn"]');
    expect(acknowledgeBtn?.textContent).toContain("Acknowledge");

    acknowledgeBtn?.click();
    expect(onContinueCombat).toHaveBeenCalledOnce();
  });
});

describe("CombatScreen first-combat demo fixture", () => {
  let dispose: (() => void) | undefined;

  afterEach(() => {
    dispose?.();
    dispose = undefined;
    document.body.innerHTML = "";
  });

  it("renders the first-combat encounter title and QingLong mantis flower enemies", () => {
    const root = document.createElement("div");
    document.body.appendChild(root);

    dispose = render(
      () => (
        <CombatScreen
          viewModel={replayFirstCombatViewModel}
          onSelectSkill={vi.fn()}
          onSelectTarget={vi.fn()}
          onConfirmAttack={vi.fn()}
          onFleeCombat={vi.fn()}
          onEndTurn={vi.fn()}
        />
      ),
      root
    );

    const title = root.querySelector(".combat-title");
    expect(title?.textContent).toBe("初战：苍灯林地");

    expect(root.querySelectorAll(".combat-hero-stand").length).toBe(4);
    expect(root.querySelectorAll(".combat-enemy-stand").length).toBe(3);

    expect(root.textContent).toContain("Magic Mantis Flower");
    expect(root.textContent).toContain("Spiny Mantis Flower");
    expect(root.textContent).toContain("Walking Mantis Flower");
  });

  it("keeps skill selection usable for the first-combat active hero", () => {
    const onSelectSkill = vi.fn();
    const root = document.createElement("div");
    document.body.appendChild(root);

    dispose = render(
      () => (
        <CombatScreen
          viewModel={replayFirstCombatViewModel}
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
      '.combat-skill-slot[title="Hunter\'s Mark"]'
    );
    expect(skillButton).not.toBeNull();
    expect(skillButton?.disabled).toBe(false);

    skillButton?.click();
    expect(onSelectSkill).toHaveBeenCalledWith("hunter-mark");
  });

  it("keeps target selection usable for the first-combat enemies", () => {
    const onSelectTarget = vi.fn();
    const root = document.createElement("div");
    document.body.appendChild(root);

    dispose = render(
      () => (
        <CombatScreen
          viewModel={replayFirstCombatViewModel}
          onSelectSkill={vi.fn()}
          onSelectTarget={onSelectTarget}
          onConfirmAttack={vi.fn()}
          onFleeCombat={vi.fn()}
          onEndTurn={vi.fn()}
        />
      ),
      root
    );

    const targetButton = Array.from(
      root.querySelectorAll<HTMLButtonElement>(".combat-target-cell")
    ).find((b) => b.textContent?.includes("Spiny Mantis Flower"));
    expect(targetButton).not.toBeUndefined();
    expect(targetButton?.disabled).toBe(false);

    targetButton?.click();
    expect(onSelectTarget).toHaveBeenCalledWith("enemy-mantis-spiny-01");
  });
});
