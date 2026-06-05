import { For, createSignal, type Component } from "solid-js";

import type { BuildingDetailViewModel, BuildingAction } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface TavernBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

type TavernTab = "upgrade" | "use";

interface FacilityDef {
  id: string;
  name: string;
  description: string;
  iconLabel: string;
  actionFilter: (action: BuildingAction) => boolean;
}

const FACILITIES: FacilityDef[] = [
  {
    id: "bar",
    name: "迷幻酒吧",
    description: "在酒吧内邀请其他玩家",
    iconLabel: "酒",
    actionFilter: (a) => a.id === "drink" || a.id.includes("bar") || a.id.includes("drink"),
  },
  {
    id: "roulette",
    name: "猩红轮盘",
    description: "消耗药剂跟赌局",
    iconLabel: "轮",
    actionFilter: (a) => a.id === "gamble" || a.id.includes("gamble") || a.id.includes("roulette"),
  },
  {
    id: "dance",
    name: "秘密舞池",
    description: "在秘密舞池与欲望狂欢",
    iconLabel: "舞",
    actionFilter: (a) => a.id === "brothel" || a.id.includes("brothel") || a.id.includes("dance"),
  },
];

/**
 * Tavern (迷情乐园) building screen.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Tavern/TavernWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_paradise.png
 * GUID: c0ea280d2704bdb4a9621d6e181e0316
 *
 * Reference: 公会界面-迷情乐园-使用空.png
 *   - Left panel: character portrait, title, Talk / Leave buttons
 *   - Right panel: Upgrade / Use tabs, three facility rows with slots
 *   - Bottom: resource bar
 */
export const TavernBuildingScreen: Component<TavernBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;
  const [activeTab, setActiveTab] = createSignal<TavernTab>("use");

  const upgradeActions = () =>
    vm().actions.filter((a) => a.id.includes("upgrade") || a.id.includes("slot"));
  const facilityActions = (filter: (a: BuildingAction) => boolean) =>
    vm().actions.filter(filter);

  return (
    <div class="app-frame tavern-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Building Header — mirrors TavernWindow/LeftPanel/Icon + Title ── */}
      <BuildingDetailHeader
        buildingId="tavern"
        label={vm().label}
        status={vm().status}
        description={vm().description}
        sourcePrefabPath="Assets/Prefabs/UI/Estate/Buildings/Tavern/TavernWindow.prefab"
        sourceSpritePath="Assets/Sprites/town/buildings/building_paradise.png"
        sourceGuid="c0ea280d2704bdb4a9621d6e181e0316"
      />

      {/* ── Tavern Content — mirrors TavernWindow LeftPanel + RightPanel ── */}
      <div class="tavern-content">
        {/* Left Panel — character + quick actions */}
        <div class="tavern-left" data-source-hierarchy="TavernWindow/LeftPanel">
          <div class="tavern-character-card">
            <div class="tavern-character-portrait" aria-hidden="true">
              <span class="tavern-character-initial">迷</span>
            </div>
            <div class="tavern-character-info">
              <h3 class="tavern-character-title">{vm().label}</h3>
              <p class="tavern-character-desc">{vm().description}</p>
            </div>
          </div>

          <div class="tavern-left-actions">
            <button class="tavern-talk-btn" data-source-component="TalkButton">
              对话
            </button>
            <button
              class="tavern-leave-btn"
              onClick={props.onReturn}
              data-source-component="CloseButton"
              data-source-sprite="Assets/Sprites/ui/btn_close.png"
            >
              离开
            </button>
          </div>

          <div class="tavern-info-card">
            <h3 class="building-info-card-title">建筑状态</h3>
            <div class="building-info-row">
              <span class="building-info-label">状态</span>
              <span class="building-info-value">
                {vm().status === "ready" ? "运营中" : vm().status === "partial" ? "部分可用" : "锁定"}
              </span>
            </div>
            {vm().currentUpgrade && (
              <div class="building-info-row">
                <span class="building-info-label">设施等级</span>
                <span class="building-info-value">{vm().currentUpgrade}</span>
              </div>
            )}
            {vm().upgradeRequirement && (
              <div class="building-info-row">
                <span class="building-info-label">升级需求</span>
                <span class="building-info-value">{vm().upgradeRequirement}</span>
              </div>
            )}
          </div>
        </div>

        {/* Right Panel — tabs + facility grid */}
        <div class="tavern-right" data-source-hierarchy="TavernWindow/RightPanel">
          {/* Tab bar */}
          <div class="tavern-tab-bar" role="tablist" aria-label="设施选项">
            <button
              class={`tavern-tab-btn ${activeTab() === "upgrade" ? "tavern-tab-btn--active" : ""}`}
              role="tab"
              aria-selected={activeTab() === "upgrade"}
              onClick={() => setActiveTab("upgrade")}
            >
              <span class="tavern-tab-check">{activeTab() === "upgrade" ? "☑" : "☐"}</span>
              升级设施
            </button>
            <button
              class={`tavern-tab-btn ${activeTab() === "use" ? "tavern-tab-btn--active" : ""}`}
              role="tab"
              aria-selected={activeTab() === "use"}
              onClick={() => setActiveTab("use")}
            >
              <span class="tavern-tab-check">{activeTab() === "use" ? "☑" : "☐"}</span>
              使用设施
            </button>
          </div>

          {/* Upgrade tab content */}
          {activeTab() === "upgrade" && (
            <div class="tavern-tab-panel" role="tabpanel">
              {upgradeActions().length > 0 ? (
                <div class="building-action-section">
                  <h3 class="building-action-section-title">设施升级</h3>
                  <For each={upgradeActions()}>
                    {(action) => (
                      <div class="building-action-card">
                        <div class="building-action-card-header">
                          <span class="building-action-label">{action.label}</span>
                          {action.isUnsupported && (
                            <span class="building-action-pill building-action-pill--unsupported">
                              未支持
                            </span>
                          )}
                          {!action.isAvailable && !action.isUnsupported && (
                            <span class="building-action-pill building-action-pill--unavailable">
                              不可用
                            </span>
                          )}
                        </div>
                        <p class="building-action-desc">{action.description}</p>
                        <div class="building-action-footer">
                          <span class={`building-action-cost ${!action.isAvailable ? "building-action-cost-unavailable" : ""}`}>
                            费用: <strong>{action.cost}</strong>
                          </span>
                          {action.isUnsupported ? (
                            <button class="building-action-btn building-action-btn--disabled" disabled>
                              暂不可用
                            </button>
                          ) : action.isAvailable ? (
                            <button
                              class="building-action-btn building-action-btn--primary"
                              onClick={() => props.onAction(action.id)}
                            >
                              {action.label}
                            </button>
                          ) : (
                            <button class="building-action-btn building-action-btn--disabled" disabled>
                              条件不足
                            </button>
                          )}
                        </div>
                      </div>
                    )}
                  </For>
                </div>
              ) : (
                <div class="tavern-empty-state">
                  <p>暂无可用升级项目。</p>
                  {vm().upgradeRequirement && (
                    <p class="tavern-empty-hint">{vm().upgradeRequirement}</p>
                  )}
                </div>
              )}
            </div>
          )}

          {/* Use tab content */}
          {activeTab() === "use" && (
            <div class="tavern-tab-panel" role="tabpanel">
              <div class="tavern-facility-list">
                <For each={FACILITIES}>
                  {(facility) => {
                    const actions = facilityActions(facility.actionFilter);
                    return (
                      <div class="tavern-facility-row" data-facility-id={facility.id}>
                        <div class="tavern-facility-header">
                          <div class="tavern-facility-icon" aria-hidden="true">
                            <span>{facility.iconLabel}</span>
                          </div>
                          <div class="tavern-facility-meta">
                            <span class="tavern-facility-name">{facility.name}</span>
                            <span class="tavern-facility-desc">{facility.description}</span>
                          </div>
                        </div>
                        <div class="tavern-slot-row">
                          {/* Empty slot — mirrors unused hero slot */}
                          <div class="tavern-slot tavern-slot--empty">
                            <span class="tavern-slot-placeholder">空</span>
                          </div>
                          {/* Action slots — up to 2 actions rendered as use buttons */}
                          <For each={actions}>
                            {(action) => (
                              <div class="tavern-slot">
                                {action.isUnsupported ? (
                                  <button class="tavern-use-btn tavern-use-btn--disabled" disabled>
                                    未支持
                                  </button>
                                ) : action.isAvailable ? (
                                  <button
                                    class="tavern-use-btn"
                                    onClick={() => props.onAction(action.id)}
                                    title={`${action.label} — ${action.cost}`}
                                  >
                                    使用
                                  </button>
                                ) : (
                                  <button class="tavern-use-btn tavern-use-btn--disabled" disabled>
                                    使用
                                  </button>
                                )}
                              </div>
                            )}
                          </For>
                          {/* Pad with empty slots if fewer than 2 actions */}
                          {actions.length < 2 && (
                            <div class="tavern-slot tavern-slot--empty">
                              <span class="tavern-slot-placeholder">空</span>
                            </div>
                          )}
                          {actions.length < 1 && (
                            <div class="tavern-slot tavern-slot--empty">
                              <span class="tavern-slot-placeholder">空</span>
                            </div>
                          )}
                        </div>
                      </div>
                    );
                  }}
                </For>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* ── Return to Town — mirrors TavernWindow/CloseButton ── */}
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
