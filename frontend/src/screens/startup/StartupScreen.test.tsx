// @vitest-environment jsdom

import { render } from "solid-js/web";
import { afterEach, describe, expect, it, vi } from "vitest";

import { StartupScreen } from "./StartupScreen";

describe("StartupScreen demo entry", () => {
  let dispose: (() => void) | undefined;

  afterEach(() => {
    dispose?.();
    dispose = undefined;
    document.body.innerHTML = "";
  });

  it("renders the First Combat Demo button", () => {
    const root = document.createElement("div");
    document.body.appendChild(root);

    dispose = render(
      () => (
        <StartupScreen
          onReplayBoot={vi.fn()}
          onLiveBoot={vi.fn()}
          onNewCampaign={vi.fn()}
          onLoadCampaign={vi.fn()}
          onFirstCombatDemo={vi.fn()}
          hasSavedCampaign={false}
        />
      ),
      root
    );

    const demoButton = Array.from(root.querySelectorAll("button")).find((btn) =>
      btn.textContent?.includes("First Combat Demo")
    );
    expect(demoButton).toBeDefined();
  });

  it("calls onFirstCombatDemo when the demo button is clicked", () => {
    const onFirstCombatDemo = vi.fn();
    const root = document.createElement("div");
    document.body.appendChild(root);

    dispose = render(
      () => (
        <StartupScreen
          onReplayBoot={vi.fn()}
          onLiveBoot={vi.fn()}
          onNewCampaign={vi.fn()}
          onLoadCampaign={vi.fn()}
          onFirstCombatDemo={onFirstCombatDemo}
          hasSavedCampaign={false}
        />
      ),
      root
    );

    const demoButton = Array.from(root.querySelectorAll("button")).find((btn) =>
      btn.textContent?.includes("First Combat Demo")
    );
    demoButton?.click();
    expect(onFirstCombatDemo).toHaveBeenCalledOnce();
  });
});
