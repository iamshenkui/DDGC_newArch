import { For, createSignal, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface TavernBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

type TavernTab = "upgrade" | "use";

/**
 * Tavern (迷情乐园) building screen.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Tavern/TavernWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_paradise.png
 * GUID: c0ea280d2704bdb4a9621d6e181e0316
 *
 * Related prefabs:
 *   Assets/Prefabs/UI/UpgradeSlot.prefab       — facility upgrade slots
 *
 * Building data (data/Buildings.json):
 *   tavern_facility — tavern level upgrades
 *   tavern_service  — tavern services (drink, gamble, etc.)
 */
export const TavernBuildingScreen: Component<TavernBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;
  const [activeTab, setActiveTab] = createSignal<TavernTab>("upgrade");

  const upgradeActions = () => vm().actions.filter((a) => a.id.startsWith("upgrade-"));
  const useActions = () => vm().actions.filter((a) => a.id.startsWith("use-"));
  const otherActions = () =>
    vm().actions.filter((a) => !a.id.startsWith("upgrade-") && !a.id.startsWith("use-"));

  const currentActions = () => {
    const tab = activeTab();
    if (tab === "upgrade") return upgradeActions();
    return useActions();
  };

  return (
    <div class="app-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
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

      {/* ── Content — mirrors TavernWindow LeftPanel + RightPanel ── */}
      <div class="building-detail-content">
        {/* Left Panel — mirrors TavernWindow/LeftPanel */}
        <div
          class="building-detail-left"
          data-source-hierarchy="TavernWindow/LeftPanel"
        >
          <div class="building-info-card">
            <h3 class="building-info-card-title">Building Status</h3>
            <div class="building-info-row">
              <span class="building-info-label">Status</span>
              <span class="building-info-value">{vm().status === "ready" ? "Operational" : vm().status === "partial" ? "Partially Available" : "Locked"}</span>
            </div>
            {vm().currentUpgrade && (
              <div class="building-info-row">
                <span class="building-info-label">Tavern Level</span>
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

          {/* Tavern Host Portrait — mirrors NPC portrait in reference */}
          <div
            class="tavern-host-portrait"
            data-source-component="TavernHostPortrait"
          >
            <div class="tavern-host-frame">
              <div class="tavern-host-avatar">
                <span class="tavern-host-initial">?</span>
              </div>
            </div>
            <span class="tavern-host-label">Tavern Keeper</span>
          </div>

          {/* Talk / Leave buttons — mirrors reference dialogue buttons */}
          <div class="tavern-action-buttons">
            <button
              class="tavern-action-btn tavern-action-btn--talk"
              onClick={() => props.onAction("talk")}
              data-source-component="TalkButton"
            >
              对话
            </button>
            <button
              class="tavern-action-btn tavern-action-btn--leave"
              onClick={props.onReturn}
              data-source-component="LeaveButton"
            >
              离开
            </button>
          </div>
        </div>

        {/* Right Panel / Actions — mirrors TavernWindow/RightPanel */}
        <div
          class="building-detail-right"
          data-source-hierarchy="TavernWindow/RightPanel"
        >
          {/* Tab bar — mirrors reference "升级设施" / "使用设施" tabs */}
          <div class="tavern-tab-bar" data-source-component="TavernTabBar">
            <button
              class={`tavern-tab-btn ${activeTab() === "upgrade" ? "tavern-tab-btn--active" : ""}`}
              onClick={() => setActiveTab("upgrade")}
              data-tab-id="upgrade"
            >
              <span class="tavern-tab-check">{activeTab() === "upgrade" ? "☑" : "☐"}</span>
              升级设施
            </button>
            <button
              class={`tavern-tab-btn ${activeTab() === "use" ? "tavern-tab-btn--active" : ""}`}
              onClick={() => setActiveTab("use")}
              data-tab-id="use"
            >
              <span class="tavern-tab-check">{activeTab() === "use" ? "☑" : "☐"}</span>
              使用设施
            </button>
          </div>

          {/* Upgrade panel — mirrors reference upgrade slots */}
          {activeTab() === "upgrade" && (
            <div class="tavern-upgrade-panel" data-source-component="UpgradePanel">
              {/* Building upgrade progression — mirrors reference slot row */}
              <div class="tavern-upgrade-header">
                <div class="tavern-upgrade-icon">
                  <img
                    src="/original/buildings/building_paradise.png"
                    alt="Tavern"
                    class="tavern-upgrade-icon-img"
                  />
                </div>
                <div class="tavern-upgrade-slots">
                  <For each={[0, 1, 2, 3]}>
                    {(idx) => (
                      <div
                        class={`tavern-upgrade-slot ${idx === 0 ? "tavern-upgrade-slot--active" : "tavern-upgrade-slot--locked"}`}
                        data-slot-index={idx}
                      >
                        {idx === 0 ? "✓" : "✕"}
                      </div>
                    )}
                  </For>
                </div>
              </div>

              <div class="tavern-upgrade-level">
                <span class="tavern-upgrade-level-label">酒馆等级</span>
                <span class="tavern-upgrade-level-value">{vm().currentUpgrade ?? "Level 1"}</span>
              </div>

              {/* Upgrade actions */}
              {upgradeActions().length > 0 ? (
                <div class="building-action-section">
                  <h3 class="building-action-section-title">Facility Upgrades</h3>
                  <For each={upgradeActions()}>
                    {(action) => (
                      <div
                        class="building-action-card"
                        data-source-prefab="Assets/Prefabs/UI/UpgradeSlot.prefab"
                        data-source-component="UpgradeSlot"
                      >
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
                    No facility upgrades currently available.
                  </p>
                </div>
              )}
            </div>
          )}

          {/* Use facilities panel */}
          {activeTab() === "use" && (
            <div class="tavern-use-panel" data-source-component="UsePanel">
              {useActions().length > 0 ? (
                <div class="building-action-section">
                  <h3 class="building-action-section-title">Tavern Services</h3>
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
                    No tavern services currently available.
                  </p>
                </div>
              )}
            </div>
          )}

          {/* Fallback: uncategorised actions */}
          {otherActions().length > 0 && (
            <div class="building-action-section">
              <h3 class="building-action-section-title">Other Services</h3>
              <For each={otherActions()}>
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
          Return to Town
        </button>
      </div>
    </div>
  );
};
