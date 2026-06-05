import { For, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface BlacksmithBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
  onSwitchToUse?: () => void;
}

/**
 * Blacksmith (锻造舱) building screen.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Blacksmith/BlacksmithWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_forging.png
 * GUID: 23e01c10f262ddc4ba9977b91314b031
 *
 * Related prefabs:
 *   Assets/Prefabs/UI/EquipmentUpgradeSlot.prefab  — weapon/armor upgrade slots
 *   Assets/Prefabs/UI/UpgradeSlot.prefab           — general upgrade slots
 *
 * Building data (data/Buildings.json):
 *   blacksmith_weapon  — weapon upgrade tree
 *   blacksmith_armor   — armor upgrade tree
 */
export const BlacksmithBuildingScreen: Component<BlacksmithBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;

  const weaponActions = () => vm().actions.filter((a) => a.id.includes("weapon"));
  const armorActions = () => vm().actions.filter((a) => a.id.includes("armor"));
  const otherActions = () =>
    vm().actions.filter((a) => !a.id.includes("weapon") && !a.id.includes("armor"));

  return (
    <div class="app-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Building Header — mirrors BlacksmithWindow/LeftPanel/Icon + Title ── */}
      <BuildingDetailHeader
        buildingId="blacksmith"
        label={vm().label}
        status={vm().status}
        description={vm().description}
        sourcePrefabPath="Assets/Prefabs/UI/Estate/Buildings/Blacksmith/BlacksmithWindow.prefab"
        sourceSpritePath="Assets/Sprites/town/buildings/building_forging.png"
        sourceGuid="23e01c10f262ddc4ba9977b91314b031"
      />

      {/* ── Tab Navigation ── */}
      <div class="forge-use-tabs" data-source-component="TabGroup">
        <button
          class="forge-use-tab forge-use-tab--active"
          data-tab-id="upgrade"
          data-source-component="UpgradeTab"
        >
          <span class="forge-use-tab-checkbox forge-use-tab-checkbox--checked">☑</span>
          <span class="forge-use-tab-label">升级设施</span>
        </button>
        <button
          class="forge-use-tab"
          onClick={props.onSwitchToUse}
          data-tab-id="use"
          data-source-component="UseTab"
        >
          <span class="forge-use-tab-checkbox" />
          <span class="forge-use-tab-label">使用设施</span>
        </button>
      </div>

      {/* ── Content — mirrors BlacksmithWindow LeftPanel + RightPanel ── */}
      <div class="building-detail-content">
        {/* Left Panel — mirrors BlacksmithWindow/LeftPanel */}
        <div
          class="building-detail-left"
          data-source-hierarchy="BlacksmithWindow/LeftPanel"
        >
          <div class="building-info-card">
            <h3 class="building-info-card-title">Building Status</h3>
            <div class="building-info-row">
              <span class="building-info-label">Status</span>
              <span class="building-info-value">{vm().status === "ready" ? "Operational" : vm().status === "partial" ? "Partially Available" : "Locked"}</span>
            </div>
            {vm().currentUpgrade && (
              <div class="building-info-row">
                <span class="building-info-label">Forge Level</span>
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

        {/* Right Panel / Actions — mirrors BlacksmithWindow/RightPanel/UpgradeWindow */}
        <div
          class="building-detail-right"
          data-source-hierarchy="BlacksmithWindow/RightPanel/UpgradeWindow"
        >
          {/* Weapon Upgrades — mirrors EquipmentUpgradeSlot */}
          {weaponActions().length > 0 && (
            <div class="building-action-section">
              <h3 class="building-action-section-title">Weapon Upgrades</h3>
              <For each={weaponActions()}>
                {(action) => (
                  <div
                    class="building-action-card"
                    data-source-prefab="Assets/Prefabs/UI/EquipmentUpgradeSlot.prefab"
                    data-source-component="EquipmentUpgradeSlot"
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

          {/* Armor Upgrades — mirrors EquipmentUpgradeSlot */}
          {armorActions().length > 0 && (
            <div class="building-action-section">
              <h3 class="building-action-section-title">Armor Upgrades</h3>
              <For each={armorActions()}>
                {(action) => (
                  <div
                    class="building-action-card"
                    data-source-prefab="Assets/Prefabs/UI/EquipmentUpgradeSlot.prefab"
                    data-source-component="EquipmentUpgradeSlot"
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
        </div>
      </div>

      {/* ── Return to Town — mirrors BlacksmithWindow/CloseButton ── */}
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
