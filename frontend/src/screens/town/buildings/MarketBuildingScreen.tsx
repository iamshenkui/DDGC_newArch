import { For, Show, createSignal, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface MarketBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

type MarketTab = "upgrade" | "buy" | "sell";

const TABS: { id: MarketTab; label: string }[] = [
  { id: "upgrade", label: "升级设施" },
  { id: "buy", label: "购买物品" },
  { id: "sell", label: "售卖物品" },
];

/**
 * Market (交易市场) building screen.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Market/MarketWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_market.png
 * GUID: 8c3505ec8c7e25b42aacdcaece82f821
 *
 * Reference frame: 公会界面-交易市场-售出.png
 *   - Left panel: character portrait, name, leave button
 *   - Top tabs: 升级设施 / 购买物品 / 售卖物品
 *   - Right panel: parchment-style item list
 *   - Bottom: currency strip (gems + gold)
 */
export const MarketBuildingScreen: Component<MarketBuildingScreenProps> = (
  props
) => {
  const vm = () => props.viewModel;
  const [activeTab, setActiveTab] = createSignal<MarketTab>("sell");

  const upgradeActions = () =>
    vm().actions.filter((a) => a.id.startsWith("upgrade-"));
  const buyActions = () =>
    vm().actions.filter(
      (a) => !a.id.startsWith("upgrade-") && !a.id.startsWith("sell-")
    );
  const hasCurrencies = () => {
    const c = vm().currencies;
    return c !== undefined && c.length > 0;
  };
  const currenciesList = () => vm().currencies ?? [];

  return (
    <div class="app-frame market-building-screen" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Building Header — mirrors MarketWindow title area ── */}
      <BuildingDetailHeader
        buildingId="market"
        label={vm().label}
        status={vm().status}
        description={vm().description}
        sourcePrefabPath="Assets/Prefabs/UI/Estate/Buildings/Market/MarketWindow.prefab"
        sourceSpritePath="Assets/Sprites/town/buildings/building_market.png"
        sourceGuid="8c3505ec8c7e25b42aacdcaece82f821"
      />

      {/* ── Content: left (portrait + info) + right (tabs + parchment) ── */}
      <div class="building-detail-content market-content">
        {/* Left Panel — mirrors MarketWindow/LeftPanel */}
        <div
          class="building-detail-left market-left"
          data-source-hierarchy="MarketWindow/LeftPanel"
        >
          {/* NPC Portrait */}
          <div class="market-portrait-frame">
            <div class="market-portrait-placeholder">
              <span class="market-portrait-initial">商</span>
            </div>
          </div>

          {/* NPC Name */}
          <div class="market-npc-name">刘洋</div>

          {/* Leave button */}
          <button
            class="market-leave-btn"
            onClick={props.onReturn}
            data-source-component="CloseButton"
          >
            离开
          </button>

          {/* Building info card */}
          <div class="building-info-card market-info-card">
            <h3 class="building-info-card-title">Building Status</h3>
            <div class="building-info-row">
              <span class="building-info-label">Status</span>
              <span class="building-info-value">
                {vm().status === "ready"
                  ? "Operational"
                  : vm().status === "partial"
                    ? "Partially Available"
                    : "Locked"}
              </span>
            </div>
            <Show when={vm().currentUpgrade}>
              <div class="building-info-row">
                <span class="building-info-label">Market Level</span>
                <span class="building-info-value">{vm().currentUpgrade}</span>
              </div>
            </Show>
            <Show when={vm().upgradeRequirement}>
              <div class="building-info-row">
                <span class="building-info-label">Requirement</span>
                <span class="building-info-value">
                  {vm().upgradeRequirement}
                </span>
              </div>
            </Show>
          </div>
        </div>

        {/* Right Panel — mirrors MarketWindow/RightPanel */}
        <div
          class="building-detail-right market-right"
          data-source-hierarchy="MarketWindow/RightPanel"
        >
          {/* Tab bar */}
          <div class="market-tab-bar" role="tablist" aria-label="交易市场功能">
            <For each={TABS}>
              {(tab) => (
                <button
                  class={`market-tab-btn ${activeTab() === tab.id ? "market-tab-btn--active" : ""}`}
                  role="tab"
                  aria-selected={activeTab() === tab.id}
                  onClick={() => setActiveTab(tab.id)}
                  data-tab-id={tab.id}
                >
                  {tab.label}
                </button>
              )}
            </For>
          </div>

          {/* Parchment content area */}
          <div class="market-parchment">
            {/* Upgrade Facilities Tab */}
            <Show when={activeTab() === "upgrade"}>
              <div class="market-tab-content" data-tab-content="upgrade">
                <Show
                  when={upgradeActions().length > 0}
                  fallback={
                    <div class="market-empty-state">
                      当前无可升级设施。
                    </div>
                  }
                >
                  <div class="building-action-section">
                    <h3 class="building-action-section-title">设施升级</h3>
                    <For each={upgradeActions()}>
                      {(action) => (
                        <div class="building-action-card">
                          <div class="building-action-card-header">
                            <span class="building-action-label">
                              {action.label}
                            </span>
                            {action.isUnsupported && (
                              <span class="building-action-pill building-action-pill--unsupported">
                                Unsupported
                              </span>
                            )}
                            {!action.isAvailable && !action.isUnsupported && (
                              <span class="building-action-pill building-action-pill--unavailable">
                                Unavailable
                              </span>
                            )}
                          </div>
                          <p class="building-action-desc">
                            {action.description}
                          </p>
                          <div class="building-action-footer">
                            <span
                              class={`building-action-cost ${!action.isAvailable ? "building-action-cost-unavailable" : ""}`}
                            >
                              Cost: <strong>{action.cost}</strong>
                            </span>
                            {action.isUnsupported ? (
                              <button
                                class="building-action-btn building-action-btn--disabled"
                                disabled
                              >
                                Not Available
                              </button>
                            ) : action.isAvailable ? (
                              <button
                                class="building-action-btn building-action-btn--primary"
                                onClick={() => props.onAction(action.id)}
                              >
                                {action.label}
                              </button>
                            ) : (
                              <button
                                class="building-action-btn building-action-btn--disabled"
                                disabled
                              >
                                Prerequisites Not Met
                              </button>
                            )}
                          </div>
                        </div>
                      )}
                    </For>
                  </div>
                </Show>
              </div>
            </Show>

            {/* Buy Items Tab */}
            <Show when={activeTab() === "buy"}>
              <div class="market-tab-content" data-tab-content="buy">
                <Show
                  when={vm().buyItems && vm().buyItems!.length > 0}
                  fallback={
                    <div class="market-empty-state">
                      当前无可购买物品。
                    </div>
                  }
                >
                  <div class="market-item-grid">
                    <For each={vm().buyItems}>
                      {(item) => (
                        <div class="market-item-card" data-item-id={item.id}>
                          <div class="market-item-icon">
                            {item.icon ? (
                              <img src={item.icon} alt={item.name} />
                            ) : (
                              <span class="market-item-icon-fallback">
                                {item.name.charAt(0)}
                              </span>
                            )}
                          </div>
                          <div class="market-item-name">{item.name}</div>
                          <div class="market-item-price">
                            {item.buyPrice ?? item.sellPrice} Gold
                          </div>
                          <button
                            class="market-item-action-btn"
                            onClick={() =>
                              props.onAction(`buy-${item.id}`)
                            }
                          >
                            购买
                          </button>
                        </div>
                      )}
                    </For>
                  </div>
                </Show>
              </div>
            </Show>

            {/* Sell Items Tab — reference frame shows this active */}
            <Show when={activeTab() === "sell"}>
              <div class="market-tab-content" data-tab-content="sell">
                <Show
                  when={vm().sellItems && vm().sellItems!.length > 0}
                  fallback={
                    <div class="market-empty-state">
                      当前无可售卖物品。
                    </div>
                  }
                >
                  <div class="market-item-grid">
                    <For each={vm().sellItems}>
                      {(item) => (
                        <div class="market-item-card" data-item-id={item.id}>
                          <div class="market-item-icon">
                            {item.icon ? (
                              <img src={item.icon} alt={item.name} />
                            ) : (
                              <span class="market-item-icon-fallback">
                                {item.name.charAt(0)}
                              </span>
                            )}
                          </div>
                          <div class="market-item-name">{item.name}</div>
                          <Show when={item.count && item.count > 1}>
                            <div class="market-item-count">x{item.count}</div>
                          </Show>
                          <div class="market-item-price">
                            {item.sellPrice} Gold
                          </div>
                          <button
                            class="market-item-action-btn market-item-action-btn--sell"
                            onClick={() =>
                              props.onAction(`sell-${item.id}`)
                            }
                          >
                            售卖
                          </button>
                        </div>
                      )}
                    </For>
                  </div>
                </Show>
              </div>
            </Show>
          </div>
        </div>
      </div>

      {/* ── Currency Strip — mirrors reference bottom bar ── */}
      <Show when={hasCurrencies()}>
        <div class="market-currency-strip" data-source-layer="currency-strip">
          <For each={currenciesList()}>
            {(currency) => (
              <span
                class="market-currency-slot"
                data-currency-label={currency.label}
              >
                <span class="market-currency-icon">
                  {currency.icon ? (
                    <img src={currency.icon} alt="" aria-hidden="true" />
                  ) : (
                    <span class="market-currency-icon-fallback" />
                  )}
                </span>
                <span class="market-currency-value">{currency.amount}</span>
              </span>
            )}
          </For>
        </div>
      </Show>

      {/* ── Return to Town ── */}
      <div class="building-return-row">
        <button
          class="building-return-btn"
          onClick={props.onReturn}
          data-source-component="CloseButton"
          data-source-sprite="Assets/Sprites/ui/btn_close.png"
        >
          Return to Town
        </button>
      </div>
    </div>
  );
};
