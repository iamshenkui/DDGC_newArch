import { For, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface StagecoachBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

/**
 * Stagecoach (次元感知塔) building screen.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/StageCoach/StageCoachWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_perception_tower.png
 * GUID: 87a55679f12a1e6489ecdb1d6e6f6b93
 *
 * Building data (data/Buildings.json):
 *   stagecoach_recruit — recruit discount upgrade tree
 *   stagecoach_heroes  — max hero slots upgrade tree
 */
export const StagecoachBuildingScreen: Component<StagecoachBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;

  const recruitmentActions = () =>
    vm().actions.filter((a) => a.id === "recruit-hero" || a.id === "view-candidates");
  const upgradeActions = () =>
    vm().actions.filter((a) => a.id !== "recruit-hero" && a.id !== "view-candidates");

  return (
    <div class="app-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Building Header — mirrors StageCoachWindow/LeftPanel/Icon + Title ── */}
      <BuildingDetailHeader
        buildingId="stagecoach"
        label={vm().label}
        status={vm().status}
        description={vm().description}
        sourcePrefabPath="Assets/Prefabs/UI/Estate/Buildings/StageCoach/StageCoachWindow.prefab"
        sourceSpritePath="Assets/Sprites/town/buildings/building_perception_tower.png"
        sourceGuid="87a55679f12a1e6489ecdb1d6e6f6b93"
      />

      {/* ── Content — mirrors StageCoachWindow LeftPanel + RightPanel ── */}
      <div class="building-detail-content">
        {/* Left Panel — mirrors StageCoachWindow/LeftPanel */}
        <div
          class="building-detail-left"
          data-source-hierarchy="StageCoachWindow/LeftPanel"
        >
          <div class="building-info-card">
            <h3 class="building-info-card-title">Building Status</h3>
            <div class="building-info-row">
              <span class="building-info-label">Status</span>
              <span class="building-info-value">{vm().status === "ready" ? "Operational" : vm().status === "partial" ? "Partially Available" : "Locked"}</span>
            </div>
            {vm().currentUpgrade && (
              <div class="building-info-row">
                <span class="building-info-label">Stagecoach Level</span>
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

        {/* Right Panel / Actions — mirrors StageCoachWindow/RightPanel/UpgradeWindow */}
        <div
          class="building-detail-right"
          data-source-hierarchy="StageCoachWindow/RightPanel/UpgradeWindow"
        >
          {/* Recruitment Actions */}
          {recruitmentActions().length > 0 && (
            <div class="building-action-section">
              <h3 class="building-action-section-title">Recruitment</h3>
              <For each={recruitmentActions()}>
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

          {/* Upgrade Actions */}
          {upgradeActions().length > 0 && (
            <div class="building-action-section">
              <h3 class="building-action-section-title">Upgrades</h3>
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
        </div>
      </div>

      {/* ── Return to Town — mirrors StageCoachWindow/CloseButton ── */}
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
