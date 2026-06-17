// @vitest-environment jsdom

import { render } from "solid-js/web";
import { afterEach, describe, expect, it, vi } from "vitest";

import { StartupScreen } from "./StartupScreen";

describe("StartupScreen first-combat demo entry", () => {
  let dispose: (() => void) | undefined;

  afterEach(() => {
    dispose?.();
    dispose = undefined;
    document.body.innerHTML = "";
  });

  it("renders the title and main menu options", () => {
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

    expect(root.querySelector(".title-page-game-title")?.textContent).toBe("DDGC");
    expect(root.querySelector(".title-page-game-subtitle")?.textContent).toContain("暗黑地牢");

    const buttons = Array.from(root.querySelectorAll(".title-page-button"));
    const labels = buttons.map((b) => b.textContent?.trim());
    expect(labels).toEqual([
      "New Campaign",
      "Load Campaign",
      "First Combat Demo",
      "Boot Replay",
      "Boot Live",
    ]);
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

    const loadButton = Array.from(root.querySelectorAll(".title-page-button")).find(
      (b) => b.textContent?.trim() === "Load Campaign"
    ) as HTMLButtonElement | undefined;
    expect(loadButton?.disabled).toBe(true);
  });

  it("enables Load Campaign when a saved campaign exists", () => {
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
          hasSavedCampaign={true}
        />
      ),
      root
    );

    const loadButton = Array.from(root.querySelectorAll(".title-page-button")).find(
      (b) => b.textContent?.trim() === "Load Campaign"
    ) as HTMLButtonElement | undefined;
    expect(loadButton?.disabled).toBe(false);
  });

  it("invokes onStartFirstCombatDemo when the First Combat Demo button is clicked", () => {
    const onStartFirstCombatDemo = vi.fn();
    const root = document.createElement("div");
    document.body.appendChild(root);

    dispose = render(
      () => (
        <StartupScreen
          onReplayBoot={vi.fn()}
          onLiveBoot={vi.fn()}
          onNewCampaign={vi.fn()}
          onLoadCampaign={vi.fn()}
          onStartFirstCombatDemo={onStartFirstCombatDemo}
          hasSavedCampaign={false}
        />
      ),
      root
    );

    const demoButton = Array.from(root.querySelectorAll(".title-page-button")).find(
      (b) => b.textContent?.trim() === "First Combat Demo"
    ) as HTMLButtonElement | undefined;

    expect(demoButton).toBeDefined();
    expect(demoButton?.disabled).toBe(false);

    demoButton?.click();
    expect(onStartFirstCombatDemo).toHaveBeenCalledOnce();
  });
});
