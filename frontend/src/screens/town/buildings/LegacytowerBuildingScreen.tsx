import { For, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface LegacytowerBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

/**
 * Legacy Tower (维度灯塔) building screen.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/LegacyTower/LegacyTowerWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_legacy_tower.png
 * GUID: ebd56d507c80af54986327ac7cb16661
 *
 * Related prefabs:
 *   Assets/Prefabs/UI/CollectionSlot.prefab    — collection slot pattern
 *   Assets/Prefabs/UI/DonationSlot.prefab      — donation slot pattern
 *
 * Building data (data/Buildings.json):
 *   museum_collection  — trinket slot upgrade tree
 *   museum_donation    — heirloom bonus upgrade tree
 */
export const LegacytowerBuildingScreen: Component<LegacytowerBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;

  const collectionActions = () =>
    vm().actions.filter((a) => a.id.includes("collection") || a.id.includes("trinket") || a.id.includes("slot"));
  const donationActions = () =>
    vm().actions.filter((a) => a.id.includes("donation") || a.id.includes("heirloom"));
  const otherActions = () =>
    vm().actions.filter(
      (a) =>
        !a.id.includes("collection") &&
        !a.id.includes("trinket") &&
        !a.id.includes("slot") &&
        !a.id.includes("donation") &&
        !a.id.includes("heirloom"),
    );

  return (
    <div class="app-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Building Header — mirrors LegacyTowerWindow/LeftPanel/Icon + Title ── */}
      <BuildingDetailHeader
        buildingId="legacytower"
        label={vm().label}
        status={vm().status}
        description={vm().description}
        sourcePrefabPath="Assets/Prefabs/UI/Estate/Buildings/LegacyTower/LegacyTowerWindow.prefab"
        sourceSpritePath="Assets/Sprites/town/buildings/building_legacy_tower.png"
        sourceGuid="ebd56d507c80af54986327ac7cb16661"
      />

      {/* ── Content — mirrors LegacyTowerWindow LeftPanel + RightPanel ── */}
      <div class="building-detail-content">
        {/* Left Panel — mirrors LegacyTowerWindow/LeftPanel */}
        <div
          class="building-detail-left"
          data-source-hierarchy="LegacyTowerWindow/LeftPanel"
        >
          <div class="building-info-card">
            <h3 class="building-info-card-title">Building Status</h3>
            <div class="building-info-row">
              <span class="building-info-label">Status</span>
              <span class="building-info-value">{vm().status === "ready" ? "Operational" : vm().status === "partial" ? "Partially Available" : "Locked"}</span>
            </div>
            {vm().currentUpgrade && (
              <div class="building-info-row">
                <span class="building-info-label">Tower Level</span>
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

        {/* Right Panel / Actions — mirrors LegacyTowerWindow/RightPanel/UpgradeWindow */}
        <div
          class="building-detail-right"
          data-source-hierarchy="LegacyTowerWindow/RightPanel/UpgradeWindow"
        >
          {/* Collection — mirrors CollectionSlot */}
          {collectionActions().length > 0 && (
            <div class="building-action-section">
              <h3 class="building-action-section-title">Collection</h3>
              <For each={collectionActions()}>
                {(action) => (
                  <div
                    class="building-action-card"
                    data-source-prefab="Assets/Prefabs/UI/CollectionSlot.prefab"
                    data-source-component="CollectionSlot"
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

          {/* Donation — mirrors DonationSlot */}
          {donationActions().length > 0 && (
            <div class="building-action-section">
              <h3 class="building-action-section-title">Donation</h3>
              <For each={donationActions()}>
                {(action) => (
                  <div
                    class="building-action-card"
                    data-source-prefab="Assets/Prefabs/UI/DonationSlot.prefab"
                    data-source-component="DonationSlot"
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

          {/* Other Services */}
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

          {/* Fallback when no actions at all */}
          {collectionActions().length === 0 &&
            donationActions().length === 0 &&
            otherActions().length === 0 && (
              <div class="building-info-card">
                <p style="margin:0;color:rgba(218,198,168,0.5);font-size:0.82rem;">
                  No actions currently available for this building.
                </p>
              </div>
            )}
        </div>
      </div>

      {/* ── Return to Town — mirrors LegacyTowerWindow/CloseButton ── */}
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
