import { describe, expect, it } from "vitest";
import { render } from "solid-js/web";

import { ResultScreen } from "./ResultScreen";
import type { ExpeditionResultViewModel } from "../../bridge/contractTypes";

const baseViewModel: ExpeditionResultViewModel = {
  kind: "result",
  title: "副本结算",
  expeditionName: "深渊试炼",
  outcome: "success",
  summary: "成功完成副本",
  lootAcquired: ["古金币 x3", "遗落宝石"],
  heroOutcomes: [
    {
      heroId: "hero-hunter-01",
      heroName: "Shen",
      classLabel: "Hunter",
      status: "alive",
      hpChange: "-4",
      stressChange: "+12",
    },
  ],
  resourcesGained: { gold: 480, supplies: -20, experience: 120 },
  isContinueAvailable: true,
};

describe("Dungeon settlement screen rendering", () => {
  it("renders the Chinese settlement labels from the reference", () => {
    const container = document.createElement("div");
    document.body.appendChild(container);

    const dispose = render(() => (
      <ResultScreen
        viewModel={baseViewModel}
        onContinue={() => {}}
        onReturnToTown={() => {}}
      />
    ), container);

    const text = container.textContent ?? "";
    expect(text).toContain("副本结算");
    expect(text).toContain("收集的奖励");
    expect(text).toContain("收集的宝藏");
    expect(text).toContain("收集的传家宝");
    expect(text).toContain("英雄状态");
    expect(text).toContain("返回城镇");
    expect(text).toContain("继续");

    dispose();
    document.body.removeChild(container);
  });

  it("renders hero outcomes and treasure value from the view model", () => {
    const container = document.createElement("div");
    document.body.appendChild(container);

    const dispose = render(() => (
      <ResultScreen
        viewModel={baseViewModel}
        onContinue={() => {}}
        onReturnToTown={() => {}}
      />
    ), container);

    const text = container.textContent ?? "";
    expect(text).toContain("Shen");
    expect(text).toContain("480");
    expect(text).toContain("深渊试炼");

    dispose();
    document.body.removeChild(container);
  });

  it("exposes the source prefab traceability attributes", () => {
    const container = document.createElement("div");
    document.body.appendChild(container);

    const dispose = render(() => (
      <ResultScreen
        viewModel={baseViewModel}
        onContinue={() => {}}
        onReturnToTown={() => {}}
      />
    ), container);

    const root = container.querySelector('[data-testid="dungeon-settlement-screen"]');
    expect(root).not.toBeNull();
    expect(root?.getAttribute("data-source-scene")).toBe("UI_Dungeon/DungeonSettlementWindow");
    expect(root?.getAttribute("data-source-prefab")).toBe(
      "Assets/Prefabs/UI/DungeonSettlementWindow.prefab"
    );

    dispose();
    document.body.removeChild(container);
  });

  it("notes the heirloom data blocker for observability", () => {
    const container = document.createElement("div");
    document.body.appendChild(container);

    const dispose = render(() => (
      <ResultScreen
        viewModel={baseViewModel}
        onContinue={() => {}}
        onReturnToTown={() => {}}
      />
    ), container);

    const row = container.querySelector('[data-blocker="heirloom-counts-not-wired"]');
    expect(row).not.toBeNull();

    dispose();
    document.body.removeChild(container);
  });
});
