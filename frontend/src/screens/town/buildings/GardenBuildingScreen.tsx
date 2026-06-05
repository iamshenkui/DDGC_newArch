import { For, createSignal, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface GardenBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

type GardenTab = "upgrade" | "use";

/**
 * Garden (天国花园) building screen.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Garden/GardenWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_garden.png
 * GUID: c34c4012911d41c4bbed2328d7025138
 *
 * Reference image: 公会界面-天国花园-升级.png
 *   - Left panel: character image + 对话/离开 buttons
 *   - Right panel: tabs (升级设施 / 使用设施) + upgrade slots / use actions
 *   - Bottom: resource strip + construction progress
 */
export const GardenBuildingScreen: Component<GardenBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;
  const [activeTab, setActiveTab] = createSignal<GardenTab>("upgrade");

  const upgradeActions = () =>
    vm().actions.filter((a) => a.id.startsWith("upgrade-") || a.id.includes("construct"));
  const useActions = () =>
    vm().actions.filter((a) => !a.id.startsWith("upgrade-") && !a.id.includes("construct"));

  const progressPercent = () => {
    const upgrades = upgradeActions();
    if (upgrades.length === 0) return 0;
    const completed = upgrades.filter((a) => a.isAvailable).length;
    return Math.round((completed / upgrades.length) * 100);
  };

  return (
    <div class="app-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Building Header — mirrors GardenWindow/LeftPanel/Icon + Title ── */}
      <BuildingDetailHeader
        buildingId="garden"
        label={vm().label}
        status={vm().status}
        description={vm().description}
        sourcePrefabPath="Assets/Prefabs/UI/Estate/Buildings/Garden/GardenWindow.prefab"
        sourceSpritePath="Assets/Sprites/town/buildings/building_garden.png"
        sourceGuid="c34c4012911d41c4bbed2328d7025138"
      />

      {/* ── Content — mirrors GardenWindow LeftPanel + RightPanel ── */}
      <div class="building-detail-content garden-detail-content">
        {/* Left Panel — mirrors GardenWindow/LeftPanel */}
        <div
          class="building-detail-left garden-detail-left"
          data-source-hierarchy="GardenWindow/LeftPanel"
        >
          {/* Character portrait area */}
          <div class="garden-character-panel">
            <div class="garden-character-frame">
              <div class="garden-character-placeholder">
                <span class="garden-character-initial">花</span>
              </div>
            </div>
            <div class="garden-character-actions">
              <button
                class="garden-talk-btn"
                onClick={() => {
                  const talkAction = vm().actions.find((a) => a.id === "talk");
                  if (talkAction) props.onAction(talkAction.id);
                }}
                data-source-component="TalkButton"
              >
                对话
              </button>
              <button
                class="garden-leave-btn"
                onClick={props.onReturn}
                data-source-component="CloseButton"
              >
                离开
              </button>
            </div>
          </div>

          <div class="building-info-card">
            <h3 class="building-info-card-title">Building Status</h3>
            <div class="building-info-row">
              <span class="building-info-label">Status</span>
              <span class="building-info-value">
                {vm().status === "ready" ? "Operational" : vm().status === "partial" ? "Partially Available" : "Locked"}
              </span>
            </div>
            {vm().currentUpgrade && (
              <div class="building-info-row">
                <span class="building-info-label">Garden Level</span>
                <span class="building-info-value">{vm().currentUpgrade}</span>
              </div>
            )}
            {vm().upgradeRequirement && (
              <div class="building-info-row">
                <span class="building-info-label">Requirement</span>
                <span class="building-info-value">{vm().upgradeRequirement}</span>
              </div>
            )}
          </div>
        </div>

        {/* Right Panel / Actions — mirrors GardenWindow/RightPanel */}
        <div
          class="building-detail-right garden-detail-right"
          data-source-hierarchy="GardenWindow/RightPanel"
        >
          {/* Tabs — 升级设施 / 使用设施 */}
          <div class="garden-tab-bar" role="tablist">
            <button
              class={`garden-tab-btn ${activeTab() === "upgrade" ? "garden-tab-btn--active" : ""}`}
              onClick={() => setActiveTab("upgrade")}
              role="tab"
              aria-selected={activeTab() === "upgrade"}
              data-tab="upgrade"
            >
              <span class="garden-tab-check">{activeTab() === "upgrade" ? "☑" : "☐"}</span>
              升级设施
            </button>
            <button
              class={`garden-tab-btn ${activeTab() === "use" ? "garden-tab-btn--active" : ""}`}
              onClick={() => setActiveTab("use")}
              role="tab"
              aria-selected={activeTab() === "use"}
              data-tab="use"
            >
              <span class="garden-tab-check">{activeTab() === "use" ? "☑" : "☐"}</span>
              使用设施
            </button>
          </div>

          {/* Upgrade Tab Content */}
          {activeTab() === "upgrade" && (
            <div class="garden-upgrade-panel" data-tab-content="upgrade">
              {/* Upgrade slots — diamond shaped */}
              {upgradeActions().length > 0 && (
                <div class="garden-upgrade-slots">
                  <For each={upgradeActions()}>
                    {(action, index) => (
                      <div
                        class={`garden-upgrade-slot ${action.isAvailable ? "garden-upgrade-slot--available" : ""} ${action.isUnsupported ? "garden-upgrade-slot--unsupported" : ""}`}
                        data-source-prefab="Assets/Prefabs/UI/UpgradeSlot.prefab"
                        data-source-component="UpgradeSlot"
                        data-slot-index={index()}
                      >
                        <div class="garden-upgrade-slot-diamond">
                          <span class="garden-upgrade-slot-number">{index() + 1}</span>
                        </div>
                        <span class="garden-upgrade-slot-label">{action.label}</span>
                        <span class="garden-upgrade-slot-cost">{action.cost}</span>
                      </div>
                    )}
                  </For>
                </div>
              )}

              {/* Upgrade action cards */}
              <div class="building-action-section">
                <h3 class="building-action-section-title">Facility Upgrades</h3>
                {upgradeActions().length === 0 ? (
                  <div class="building-info-card">
                    <p style="margin:0;color:rgba(218,198,168,0.5);font-size:0.82rem;">
                      No upgrades currently available.
                    </p>
                  </div>
                ) : (
                  <For each={upgradeActions()}>
                    {(action) => (
                      <div class="building-action-card">
                        <div class="building-action-card-header">
                          <span class="building-action-label">{action.label}</span>
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
                        <p class="building-action-desc">{action.description}</p>
                        <div class="building-action-footer">
                          <span class={`building-action-cost ${!action.isAvailable ? "building-action-cost-unavailable" : ""}`}>
                            Cost: <strong>{action.cost}</strong>
                          </span>
                          {action.isUnsupported ? (
                            <button class="building-action-btn building-action-btn--disabled" disabled>
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
                            <div style="display:flex;gap:0.5rem;align-items:center;">
                              <button class="building-action-btn building-action-btn--disabled" disabled>
                                Prerequisites Not Met
                              </button>
                              {vm().upgradeRequirement && (
                                <span class="building-action-pill building-action-pill--info">
                                  {vm().upgradeRequirement}
                                </span>
                              )}
                            </div>
                          )}
                        </div>
                      </div>
                    )}
                  </For>
                )}
              </div>

              {/* Construction progress */}
              <div class="garden-progress-row">
                <span class="garden-progress-label">建设进度</span>
                <div class="garden-progress-bar">
                  <div
                    class="garden-progress-fill"
                    style={`width: ${progressPercent()}%`}
                  />
                </div>
                <span class="garden-progress-value">{progressPercent()}%</span>
              </div>
            </div>
          )}

          {/* Use Tab Content */}
          {activeTab() === "use" && (
            <div class="garden-use-panel" data-tab-content="use">
              {useActions().length > 0 ? (
                <div class="building-action-section">
                  <h3 class="building-action-section-title">Facility Services</h3>
                  <For each={useActions()}>
                    {(action) => (
                      <div class="building-action-card">
                        <div class="building-action-card-header">
                          <span class="building-action-label">{action.label}</span>
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
                        <p class="building-action-desc">{action.description}</p>
                        <div class="building-action-footer">
                          <span class={`building-action-cost ${!action.isAvailable ? "building-action-cost-unavailable" : ""}`}>
                            Cost: <strong>{action.cost}</strong>
                          </span>
                          {action.isUnsupported ? (
                            <button class="building-action-btn building-action-btn--disabled" disabled>
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
                            <button class="building-action-btn building-action-btn--disabled" disabled>
                              Prerequisites Not Met
                            </button>
                          )}
                        </div>
                      </div>
                    )}
                  </For>
                </div>
              ) : (
                <div class="building-info-card">
                  <p style="margin:0;color:rgba(218,198,168,0.5);font-size:0.82rem;">
                    No services currently available.
                  </p>
                </div>
              )}
            </div>
          )}
        </div>
      </div>

      {/* ── Return to Town — mirrors GardenWindow/CloseButton ── */}
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
