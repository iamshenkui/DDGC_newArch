import { For, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface GardenBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

/**
 * Garden (天国花园) building screen — empty use state.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Garden/GardenWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_garden.png
 * GUID: c34c4012911d41c4bbed2328d7025138
 *
 * Related prefabs:
 *   Assets/Prefabs/UI/GardenRestSlot.prefab — hero rest slot pattern
 *
 * This screen renders the "使用空" (empty use) state where no heroes
 * are currently resting in the garden slots.
 */
export const GardenBuildingScreen: Component<GardenBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;

  // Categorise garden actions
  const restActions = () =>
    vm().actions.filter((a) => a.id.includes("rest") || a.id.includes("heal") || a.id.includes("recover"));
  const upgradeActions = () =>
    vm().actions.filter(
      (a) => a.id.includes("upgrade") || a.id.includes("slot") || a.id.includes("expand"),
    );
  const otherActions = () =>
    vm().actions.filter(
      (a) =>
        !a.id.includes("rest") &&
        !a.id.includes("heal") &&
        !a.id.includes("recover") &&
        !a.id.includes("upgrade") &&
        !a.id.includes("slot") &&
        !a.id.includes("expand"),
    );

  // Empty garden slots — mirrors GardenRestSlot empty state
  const EMPTY_SLOTS = [
    { id: "slot-1", label: "花园休养位 1" },
    { id: "slot-2", label: "花园休养位 2" },
    { id: "slot-3", label: "花园休养位 3" },
  ];

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
      <div class="building-detail-content">
        {/* Left Panel — mirrors GardenWindow/LeftPanel */}
        <div
          class="building-detail-left"
          data-source-hierarchy="GardenWindow/LeftPanel"
        >
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

          {/* Garden description card */}
          <div class="building-info-card">
            <h3 class="building-info-card-title">Garden Services</h3>
            <p style="margin:0;color:rgba(218,198,168,0.7);font-size:0.82rem;line-height:1.5;">
              在宁静的花园中安置英雄进行休整，恢复生命值并降低压力值。
              升级花园可解锁更多休养位和增强恢复效果。
            </p>
          </div>
        </div>

        {/* Right Panel / Actions — mirrors GardenWindow/RightPanel */}
        <div
          class="building-detail-right"
          data-source-hierarchy="GardenWindow/RightPanel"
        >
          {/* Empty Garden Slots — mirrors GardenRestSlot empty state (使用空) */}
          <div class="building-action-section">
            <h3 class="building-action-section-title">休养位</h3>
            <For each={EMPTY_SLOTS}>
              {(slot) => (
                <div
                  class="building-action-card"
                  data-source-prefab="Assets/Prefabs/UI/GardenRestSlot.prefab"
                  data-source-component="GardenRestSlot"
                  data-slot-id={slot.id}
                  data-slot-state="empty"
                >
                  <div class="building-action-card-header">
                    <span class="building-action-label">{slot.label}</span>
                    <span class="building-action-pill building-action-pill--info">
                      空闲
                    </span>
                  </div>
                  <p class="building-action-desc">
                    暂无英雄在此休养。选择一位英雄放入花园以开始恢复。
                  </p>
                  <div class="building-action-footer">
                    <span class="building-action-cost">
                      恢复效果: <strong>HP +10 / Stress -5</strong>
                    </span>
                    <button
                      class="building-action-btn building-action-btn--primary"
                      onClick={() => props.onAction("place-hero")}
                      data-action="place-hero"
                    >
                      放置英雄
                    </button>
                  </div>
                </div>
              )}
            </For>
          </div>

          {/* Rest Actions */}
          {restActions().length > 0 && (
            <div class="building-action-section">
              <h3 class="building-action-section-title">休养服务</h3>
              <For each={restActions()}>
                {(action) => (
                  <div
                    class="building-action-card"
                    data-source-prefab="Assets/Prefabs/UI/GardenRestSlot.prefab"
                    data-source-component="GardenRestSlot"
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
          )}

          {/* Upgrade Actions */}
          {upgradeActions().length > 0 && (
            <div class="building-action-section">
              <h3 class="building-action-section-title">Facility Upgrades</h3>
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
            </div>
          )}

          {/* Other uncategorised actions */}
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
