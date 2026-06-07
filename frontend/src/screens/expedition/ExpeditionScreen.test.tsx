// @vitest-environment jsdom

import { render } from "solid-js/web";
import { afterEach, describe, expect, it, vi } from "vitest";

import { replayExpeditionViewModel } from "../../validation/replayFixtures";
import { ExpeditionScreen } from "./ExpeditionScreen";

describe("ExpeditionScreen combat entry", () => {
  let dispose: (() => void) | undefined;

  afterEach(() => {
    dispose?.();
    dispose = undefined;
    document.body.innerHTML = "";
  });

  it("exposes the combat entry intent when supplied by the app shell", () => {
    const onEnterCombat = vi.fn();
    const root = document.createElement("div");
    document.body.appendChild(root);

    dispose = render(
      () => (
        <ExpeditionScreen
          viewModel={replayExpeditionViewModel}
          onLaunchExpedition={vi.fn()}
          onEnterCombat={onEnterCombat}
          onReturnToTown={vi.fn()}
        />
      ),
      root
    );

    const enterCombatButton = root.querySelector<HTMLButtonElement>('[data-testid="enter-combat-btn"]');
    expect(enterCombatButton).not.toBeNull();

    enterCombatButton?.click();
    expect(onEnterCombat).toHaveBeenCalledOnce();
  });
});
