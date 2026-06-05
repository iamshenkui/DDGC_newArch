import { For, type Component, createSignal } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface BlacksmithBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
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
 * Reference page: 公会界面-锻造仓-使用空
 *   - Left panel: NPC portrait + building info
 *   - Right panel: Tab switcher (升级设施 / 使用设施) + forge slot grid
 *   - Bottom: resource counters
 */
export const BlacksmithBuildingScreen: Component<BlacksmithBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;
  const [activeTab, setActiveTab] = createSignal(vm().activeTab ?? "use");

  const weaponActions = () => vm().actions.filter((a) => a.id.includes("weapon"));
  const armorActions = () => vm().actions.filter((a) => a.id.includes("armor"));
  const otherActions = () =>
    vm().actions.filter((a) => !a.id.includes("weapon") && !a.id.includes("armor"));

  const forgeSlots = () => vm().forgeSlots ?? [];
  const resources = () => vm().resources ?? [];

  return (
    <div class="app-frame forge-viewport" data-source-scene="Assets/Scenes/EstateManagement.unity">
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

      {/* ── Content: left (NPC + info) + right (tabs + slots) ── */}
      <div class="building-detail-content">
        {/* Left Panel — mirrors BlacksmithWindow/LeftPanel */}
        <div
          class="building-detail-left"
          data-source-hierarchy="BlacksmithWindow/LeftPanel"
        >
          {/* NPC Portrait — mirrors guild/blacksmith NPC art */}
          <div class="forge-npc-panel">
            <div class="forge-npc-portrait">
              {vm().npcPortrait ? (
                <img
                  class="forge-npc-image"
                  src={vm().npcPortrait}
                  alt={vm().npcName ?? "Blacksmith"}
                />
              ) : (
                <div class="forge-npc-fallback">
                  <span class="forge-npc-initial">
                    {(vm().npcName ?? "铁")[0]}
                  </span>
                </div>
              )}
            </div>
            <div class="forge-npc-info">
              <span class="forge-npc-name">{vm().npcName ?? "铁匠"}</span>
              <span class="forge-npc-title">{vm().npcTitle ?? "锻造师"}</span>
            </div>
          </div>

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

        {/* Right Panel — mirrors BlacksmithWindow/RightPanel/UpgradeWindow */}
        <div
          class="building-detail-right"
          data-source-hierarchy="BlacksmithWindow/RightPanel/UpgradeWindow"
        >
          {/* Tab Switcher — 升级设施 / 使用设施 */}
          <div class="forge-tabs">
            <button
              class={`forge-tab ${activeTab() === "upgrade" ? "forge-tab--active" : ""}`}
              onClick={() => setActiveTab("upgrade")}
              data-tab="upgrade"
            >
              <span class="forge-tab-checkbox">
                {activeTab() === "upgrade" ? "☑" : "☐"}
              </span>
              <span class="forge-tab-label">升级设施</span>
            </button>
            <button
              class={`forge-tab ${activeTab() === "use" ? "forge-tab--active" : ""}`}
              onClick={() => setActiveTab("use")}
              data-tab="use"
            >
              <span class="forge-tab-checkbox">
                {activeTab() === "use" ? "☑" : "☐"}
              </span>
              <span class="forge-tab-label">使用设施</span>
            </button>
          </div>

          {/* Upgrade Tab Content */}
          {activeTab() === "upgrade" && (
            <div class="forge-tab-content">
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
          )}

          {/* Use Facility Tab Content — Forge Slot Grid */}
          {activeTab() === "use" && (
            <div class="forge-tab-content">
              <div class="forge-slot-grid">
                <For each={forgeSlots()}>
                  {(slot) => (
                    <div
                      class={`forge-slot forge-slot--${slot.status}`}
                      data-slot-id={slot.id}
                      title={slot.itemName ?? slot.label ?? "Empty Slot"}
                    >
                      {slot.status === "empty" && (
                        <div class="forge-slot-empty" />
                      )}
                      {slot.status === "occupied" && (
                        <div class="forge-slot-occupied">
                          <span class="forge-slot-item">{slot.itemName ?? "?"}</span>
                          {slot.itemLevel !== undefined && (
                            <span class="forge-slot-level">+{slot.itemLevel}</span>
                          )}
                        </div>
                      )}
                      {slot.status === "locked" && (
                        <div class="forge-slot-locked">
                          <span class="forge-slot-x">✕</span>
                        </div>
                      )}
                    </div>
                  )}
                </For>
              </div>

              {/* Empty state message when no slots are occupied (使用空) */}
              {forgeSlots().length > 0 && !forgeSlots().some((s) => s.status === "occupied") && (
                <div class="forge-empty-state">
                  <p class="forge-empty-hint">
                    当前没有正在锻造的装备。选择空槽位开始锻造新装备。
                  </p>
                </div>
              )}

              {/* Detail area — mirrors reference large empty rectangle below slots */}
              <div class="forge-detail-area">
                <p class="forge-detail-placeholder">
                  选择槽位查看锻造详情
                </p>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* ── Resource Bar — mirrors reference bottom resource strip ── */}
      {resources().length > 0 && (
        <div class="forge-resources-bar">
          <For each={resources()}>
            {(resource) => (
              <div class="forge-resource-item" data-resource-id={resource.id}>
                {resource.icon ? (
                  <img class="forge-resource-icon" src={resource.icon} alt={resource.label} />
                ) : (
                  <div class="forge-resource-icon-fallback" />
                )}
                <span class="forge-resource-label">{resource.label}</span>
                <span class="forge-resource-value">{resource.value}</span>
              </div>
            )}
          </For>
        </div>
      )}

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
