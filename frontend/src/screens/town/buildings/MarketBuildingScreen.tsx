import { For, Match, Switch, createMemo, createSignal, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface MarketBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

type MarketTab = "supplies" | "equipment" | "trinkets" | "special";

const TAB_LABELS: Record<MarketTab, string> = {
  supplies: "补给",
  equipment: "装备",
  trinkets: "饰品",
  special: "特殊",
};

function categorizeMarketAction(actionId: string): MarketTab {
  const id = actionId.toLowerCase();
  if (id.includes("supply") || id.includes("food") || id.includes("medicine") || id.includes("torch") || id.includes("provision")) {
    return "supplies";
  }
  if (id.includes("equipment") || id.includes("weapon") || id.includes("armor") || id.includes("gear")) {
    return "equipment";
  }
  if (id.includes("trinket") || id.includes("accessory") || id.includes("charm")) {
    return "trinkets";
  }
  return "special";
}

/**
 * Market (交易市场) building screen — Trading Market Purchase page.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Market/MarketWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_market.png
 * GUID: 8c3505ec8c7e25b42aacdcaece82f821
 *
 * Related prefabs:
 *   Assets/Prefabs/UI/ShopItemSlot.prefab    — purchase item slots
 *   Assets/Prefabs/UI/ShopCategoryTab.prefab — category tab buttons
 *
 * Building data (data/Buildings.json):
 *   market_supplies   — consumable expedition supplies
 *   market_equipment  — weapons and armor
 *   market_trinkets   — equippable accessories
 *   market_special    — limited-time or rare items
 */
export const MarketBuildingScreen: Component<MarketBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;
  const [activeTab, setActiveTab] = createSignal<MarketTab>("supplies");

  const categorizedActions = createMemo(() => {
    const cats: Record<MarketTab, BuildingDetailViewModel["actions"]> = {
      supplies: [],
      equipment: [],
      trinkets: [],
      special: [],
    };
    for (const action of vm().actions) {
      const tab = categorizeMarketAction(action.id);
      cats[tab] = [...cats[tab], action];
    }
    return cats;
  });

  const tabsWithContent = createMemo(() => {
    const cats = categorizedActions();
    return (Object.keys(cats) as MarketTab[]).filter((tab) => cats[tab].length > 0);
  });

  const currentTabActions = createMemo(() => categorizedActions()[activeTab()]);

  return (
    <div class="app-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Building Header — mirrors MarketWindow/LeftPanel/Icon + Title ── */}
      <BuildingDetailHeader
        buildingId="market"
        label={vm().label}
        status={vm().status}
        description={vm().description}
        sourcePrefabPath="Assets/Prefabs/UI/Estate/Buildings/Market/MarketWindow.prefab"
        sourceSpritePath="Assets/Sprites/town/buildings/building_market.png"
        sourceGuid="8c3505ec8c7e25b42aacdcaece82f821"
      />

      {/* ── Content — mirrors MarketWindow LeftPanel + RightPanel ── */}
      <div class="building-detail-content">
        {/* Left Panel — mirrors MarketWindow/LeftPanel */}
        <div
          class="building-detail-left"
          data-source-hierarchy="MarketWindow/LeftPanel"
        >
          <div class="building-info-card">
            <h3 class="building-info-card-title">商店信息</h3>
            <div class="building-info-row">
              <span class="building-info-label">状态</span>
              <span class="building-info-value">{vm().status === "ready" ? "营业中" : vm().status === "partial" ? "部分营业" : "未解锁"}</span>
            </div>
            {vm().currentUpgrade && (
              <div class="building-info-row">
                <span class="building-info-label">商店等级</span>
                <span class="building-info-value">{vm().currentUpgrade}</span>
              </div>
            )}
            {vm().upgradeRequirement && (
              <div class="building-info-row">
                <span class="building-info-label">升级条件</span>
                <span class="building-info-value">{vm().upgradeRequirement}</span>
              </div>
            )}
          </div>

          <div class="building-info-card" style={{ "margin-top": "12px" }}>
            <h3 class="building-info-card-title">商人说明</h3>
            <p style={{ margin: 0, color: "rgba(218,198,168,0.7)", "font-size": "0.82rem", "line-height": "1.5" }}>
              这里出售远征所需的各类物资。商品库存会随商店等级提升而扩充。购买前请确认背包空间与资金充足。
            </p>
          </div>
        </div>

        {/* Right Panel / Purchase Grid — mirrors MarketWindow/RightPanel/ShopGrid */}
        <div
          class="building-detail-right"
          data-source-hierarchy="MarketWindow/RightPanel/ShopGrid"
        >
          {/* Category Tabs — mirrors ShopCategoryTab */}
          {tabsWithContent().length > 0 && (
            <div
              class="market-tab-bar"
              data-source-prefab="Assets/Prefabs/UI/ShopCategoryTab.prefab"
              data-source-component="ShopCategoryTab"
            >
              <For each={tabsWithContent()}>
                {(tab) => (
                  <button
                    class={`market-tab-btn ${activeTab() === tab ? "market-tab-btn--active" : ""}`}
                    onClick={() => setActiveTab(tab)}
                    data-tab-id={tab}
                  >
                    {TAB_LABELS[tab]}
                  </button>
                )}
              </For>
            </div>
          )}

          {/* Purchase Item Grid — mirrors ShopItemSlot */}
          {currentTabActions().length > 0 ? (
            <div
              class="market-item-grid"
              data-source-prefab="Assets/Prefabs/UI/ShopItemSlot.prefab"
              data-source-component="ShopItemSlot"
            >
              <For each={currentTabActions()}>
                {(action) => (
                  <div class="market-item-card">
                    <div class="market-item-card-header">
                      <span class="market-item-name">{action.label}</span>
                      <div class="market-item-badges">
                        {action.isUnsupported && (
                          <span class="building-action-pill building-action-pill--unsupported">
                            未开放
                          </span>
                        )}
                        {!action.isAvailable && !action.isUnsupported && (
                          <span class="building-action-pill building-action-pill--unavailable">
                            缺货
                          </span>
                        )}
                      </div>
                    </div>
                    <p class="market-item-desc">{action.description}</p>
                    <div class="market-item-footer">
                      <span class={`market-item-cost ${!action.isAvailable ? "market-item-cost--unavailable" : ""}`}>
                        <span class="market-item-cost-label">价格:</span>
                        <strong>{action.cost}</strong>
                      </span>
                      {action.isUnsupported ? (
                        <button class="market-item-btn market-item-btn--disabled" disabled>
                          未开放
                        </button>
                      ) : action.isAvailable ? (
                        <button
                          class="market-item-btn market-item-btn--primary"
                          onClick={() => props.onAction(action.id)}
                          data-action-id={action.id}
                        >
                          购买
                        </button>
                      ) : (
                        <button class="market-item-btn market-item-btn--disabled" disabled>
                          无法购买
                        </button>
                      )}
                    </div>
                  </div>
                )}
              </For>
            </div>
          ) : (
            <div class="building-info-card">
              <p style={{ margin: 0, color: "rgba(218,198,168,0.5)", "font-size": "0.82rem" }}>
                该分类暂无商品。
              </p>
            </div>
          )}
        </div>
      </div>

      {/* ── Return to Town — mirrors MarketWindow/CloseButton ── */}
      <div class="building-return-row">
        <button
          class="building-return-btn"
          onClick={props.onReturn}
          data-source-component="CloseButton"
          data-source-sprite="Assets/Sprites/ui/btn_close.png"
        >
          返回城镇
        </button>
      </div>
    </div>
  );
};
