import { For, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface SanitariumBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

/**
 * Sanitarium (细胞修复站) building screen.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Sanitarium/SanitariumWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_cell_repair.png
 * GUID: 55375034893560044a266e905926e8ff
 *
 * Related prefab:
 *   Assets/Prefabs/UI/TreatmentHeroSlot.prefab — treatment slot pattern
 *
 * Building data (data/Buildings.json):
 *   quirk_positive_cost, quirk_negative_cost, quirk_treatment_chance,
 *   quirk_slots, disease_cost, disease_cure_all_chance, disease_slots
 */
export const SanitariumBuildingScreen: Component<SanitariumBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;

  // Categorise actions per Sanitarium's Unity prefab treatment slots
  const quirkActions = () =>
    vm().actions.filter(
      (a) => a.id.includes("quirk") || a.id.includes("positive") || a.id.includes("negative"),
    );
  const diseaseActions = () =>
    vm().actions.filter((a) => a.id.includes("disease") || a.id.includes("cure"));
  const upgradeActions = () =>
    vm().actions.filter(
      (a) => a.id.includes("slot") || a.id.includes("upgrade") || a.id.includes("treatment-chance"),
    );

  return (
    <div class="app-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Building Header — mirrors SanitariumWindow/LeftPanel/Icon + Title ── */}
      <BuildingDetailHeader
        buildingId="sanitarium"
        label={vm().label}
        status={vm().status}
        description={vm().description}
        sourcePrefabPath="Assets/Prefabs/UI/Estate/Buildings/Sanitarium/SanitariumWindow.prefab"
        sourceSpritePath="Assets/Sprites/town/buildings/building_cell_repair.png"
        sourceGuid="55375034893560044a266e905926e8ff"
      />

      {/* ── Content — mirrors SanitariumWindow LeftPanel + RightPanel ── */}
      <div class="building-detail-content">
        {/* Left Panel — mirrors SanitariumWindow/LeftPanel */}
        <div
          class="building-detail-left"
          data-source-hierarchy="SanitariumWindow/LeftPanel"
        >
          <div class="building-info-card">
            <h3 class="building-info-card-title">Building Status</h3>
            <div class="building-info-row">
              <span class="building-info-label">Status</span>
              <span class="building-info-value">{vm().status === "ready" ? "Operational" : vm().status === "partial" ? "Partially Available" : "Locked"}</span>
            </div>
            {vm().currentUpgrade && (
              <div class="building-info-row">
                <span class="building-info-label">Treatment Level</span>
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

        {/* Right Panel / Actions — mirrors SanitariumWindow/RightPanel/UpgradeWindow */}
        <div
          class="building-detail-right"
          data-source-hierarchy="SanitariumWindow/RightPanel/UpgradeWindow"
        >
          {/* Quirk Treatment — mirrors TreatmentHeroSlot */}
          {quirkActions().length > 0 && (
            <div class="building-action-section">
              <h3 class="building-action-section-title">Quirk Treatment</h3>
              <For each={quirkActions()}>
                {(action) => (
                  <div
                    class="building-action-card"
                    data-source-prefab="Assets/Prefabs/UI/TreatmentHeroSlot.prefab"
                    data-source-component="TreatmentHeroSlot"
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

          {/* Disease Treatment */}
          {diseaseActions().length > 0 && (
            <div class="building-action-section">
              <h3 class="building-action-section-title">Disease Treatment</h3>
              <For each={diseaseActions()}>
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

          {/* Facility Upgrades — mirrors UpgradeWindow */}
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

          {/* Fallback: uncategorised actions */}
          {quirkActions().length === 0 &&
            diseaseActions().length === 0 &&
            upgradeActions().length === 0 && (
              <div class="building-action-section">
                <h3 class="building-action-section-title">Actions</h3>
                {vm().actions.length === 0 ? (
                  <div class="building-info-card">
                    <p style="margin:0;color:rgba(218,198,168,0.5);font-size:0.82rem;">
                      No actions currently available for this building.
                    </p>
                  </div>
                ) : (
                  <For each={vm().actions}>
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
                )}
              </div>
            )}
        </div>
      </div>

      {/* ── Return to Town — mirrors SanitariumWindow/CloseButton ── */}
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
