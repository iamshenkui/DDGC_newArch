// @vitest-environment jsdom

import { render } from "solid-js/web";
import { afterEach, describe, expect, it, vi } from "vitest";

import { StartupScreen } from "./StartupScreen";
import { DdgcApp } from "../../app/DdgcApp";

function waitFor(condition: () => boolean, timeout = 2000): Promise<void> {
  return new Promise((resolve, reject) => {
    const start = Date.now();
    const check = () => {
      if (condition()) {
        resolve();
        return;
      }
      if (Date.now() - start > timeout) {
        reject(new Error("waitFor timeout"));
        return;
      }
      setTimeout(check, 10);
    };
    check();
  });
}

function findButtonByText(root: HTMLElement, text: string): HTMLButtonElement | undefined {
  return Array.from(root.querySelectorAll("button")).find((b) =>
    b.textContent?.includes(text)
  );
}

describe("StartupScreen", () => {
  let dispose: (() => void) | undefined;

  afterEach(() => {
    dispose?.();
    dispose = undefined;
    document.body.innerHTML = "";
  });

  it("renders the main menu entries including First Combat Demo", () => {
    const root = document.createElement("div");
    document.body.appendChild(root);

    dispose = render(
      () => (
        <StartupScreen
          onReplayBoot={vi.fn()}
          onLiveBoot={vi.fn()}
          onNewCampaign={vi.fn()}
          onLoadCampaign={vi.fn()}
          onStartFirstCombatDemo={vi.fn()}
          hasSavedCampaign={false}
        />
      ),
      root
    );

    expect(root.querySelector("h1")?.textContent).toBe("DDGC");
    expect(findButtonByText(root, "New Campaign")).toBeDefined();
    expect(findButtonByText(root, "Load Campaign")).toBeDefined();
    expect(findButtonByText(root, "First Combat Demo")).toBeDefined();
    expect(findButtonByText(root, "Boot Replay")).toBeDefined();
    expect(findButtonByText(root, "Boot Live")).toBeDefined();
  });

  it("disables Load Campaign when there is no saved campaign", () => {
    const root = document.createElement("div");
    document.body.appendChild(root);

    dispose = render(
      () => (
        <StartupScreen
          onReplayBoot={vi.fn()}
          onLiveBoot={vi.fn()}
          onNewCampaign={vi.fn()}
          onLoadCampaign={vi.fn()}
          onStartFirstCombatDemo={vi.fn()}
          hasSavedCampaign={false}
        />
      ),
      root
    );

    const loadButton = findButtonByText(root, "Load Campaign");
    expect(loadButton?.disabled).toBe(true);
  });

  it("calls the correct callbacks for legacy startup actions", () => {
    const onReplayBoot = vi.fn();
    const onLiveBoot = vi.fn();
    const onNewCampaign = vi.fn();
    const onLoadCampaign = vi.fn();
    const onStartFirstCombatDemo = vi.fn();

    const root = document.createElement("div");
    document.body.appendChild(root);

    dispose = render(
      () => (
        <StartupScreen
          onReplayBoot={onReplayBoot}
          onLiveBoot={onLiveBoot}
          onNewCampaign={onNewCampaign}
          onLoadCampaign={onLoadCampaign}
          onStartFirstCombatDemo={onStartFirstCombatDemo}
          hasSavedCampaign={true}
        />
      ),
      root
    );

    findButtonByText(root, "New Campaign")?.click();
    expect(onNewCampaign).toHaveBeenCalledOnce();

    findButtonByText(root, "Load Campaign")?.click();
    expect(onLoadCampaign).toHaveBeenCalledOnce();

    findButtonByText(root, "Boot Replay")?.click();
    expect(onReplayBoot).toHaveBeenCalledOnce();

    findButtonByText(root, "Boot Live")?.click();
    expect(onLiveBoot).toHaveBeenCalledOnce();

    findButtonByText(root, "First Combat Demo")?.click();
    expect(onStartFirstCombatDemo).toHaveBeenCalledOnce();
  });

  it("launches the first-combat demo from startup and reaches a usable combat interaction", async () => {
    const root = document.createElement("div");
    document.body.appendChild(root);

    dispose = render(() => <DdgcApp />, root);

    // The app starts on the startup screen.
    await waitFor(() => findButtonByText(root, "First Combat Demo") !== undefined);
    const demoButton = findButtonByText(root, "First Combat Demo");
    expect(demoButton).toBeDefined();

    demoButton!.click();

    // The demo intent should route through the replay bridge and render the
    // existing migrated CombatScreen (not a separate /demo page).
    await waitFor(() => root.querySelector(".combat-viewport") !== null);

    const combatTitle = root.querySelector(".combat-title");
    expect(combatTitle?.textContent).toBe("初战：苍灯林地");

    expect(root.querySelectorAll(".combat-hero-stand").length).toBe(4);
    expect(root.querySelectorAll(".combat-enemy-stand").length).toBe(3);
    expect(root.textContent).toContain("Magic Mantis Flower");
    expect(root.textContent).toContain("Spiny Mantis Flower");
    expect(root.textContent).toContain("Walking Mantis Flower");

    // Use a combat interaction: select the active hero's first skill.
    const skillButton = root.querySelector<HTMLButtonElement>(
      '.combat-skill-slot[data-skill-id="hunter-mark"]'
    );
    expect(skillButton).not.toBeNull();
    skillButton!.click();

    await waitFor(() =>
      root.querySelector('.combat-skill-slot.combat-skill-slot--selected') !== null
    );

    // With a skill selected, confirm attack becomes enabled and resolves the
    // interaction into the character-hit acknowledgement phase.
    const confirmButton = root.querySelector<HTMLButtonElement>(".combat-attack-btn");
    expect(confirmButton?.disabled).toBe(false);
    confirmButton!.click();

    await waitFor(() => root.querySelector('[data-testid="combat-continue-btn"]') !== null);

    const acknowledgeButton = root.querySelector<HTMLButtonElement>(
      '[data-testid="combat-continue-btn"]'
    );
    expect(acknowledgeButton?.textContent).toContain("Acknowledge");
  });
});
