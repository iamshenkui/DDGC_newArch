import { For, createSignal, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface GardenBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

/**
 * Garden (天国花园) building screen.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Garden/GardenWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_garden.png
 * GUID: c34c4012911d41c4bbed2328d7025138
 *
 * Reference page: 公会界面-天国花园-使用中
 *   - Two tabs: 升级设施 (Upgrade Facilities) and 使用设施 (Use Facilities)
 *   - Use Facilities shows three services with hero slots:
 *     1. 星空观测台 (Stargazing Observatory)
 *     2. 回忆长廊 (Memory Corridor)
 *     3. 美梦舱室 (Dream Chamber)
 *
 * Data blocker (HB-14-blocker-001):
 *   - Per-facility hero slot occupancy is not yet tracked in the runtime view model.
 *   - Slot occupancy requires extending BuildingDetailViewModel with facility/slot state
 *     or a dedicated Garden-specific view model shape.
 *   - Until then, slots render as empty placeholders with visual affordances.
 */
export const GardenBuildingScreen: Component<GardenBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;
  const [activeTab, setActiveTab] = createSignal<"upgrade" | "use">("use");

  // Filter actions into facility categories for the "Use Facilities" tab
  const stargazingActions = () =>
    vm().actions.filter((a) => a.id === "garden-stargazing" || a.id.includes("stargaz"));
  const memoryActions = () =>
    vm().actions.filter((a) => a.id === "garden-memory" || a.id.includes("memory"));
  const dreamActions = () =>
    vm().actions.filter((a) => a.id === "garden-dream" || a.id.includes("dream"));

  // Fallback: if no categorized actions, treat all actions as garden facilities
  const hasCategorizedActions = () =>
    stargazingActions().length > 0 || memoryActions().length > 0 || dreamActions().length > 0;

  const upgradeActions = () =>
    vm().actions.filter((a) => a.id.includes("upgrade") || a.id.includes("slot"));
  const useActions = () =>
    vm().actions.filter(
      (a) => !a.id.includes("upgrade") && !a.id.includes("slot"),
    );

  // Facility definitions for rendering when actions are not explicitly categorized
  const FACILITIES = [
    {
      key: "stargazing",
      label: "星空观测台",
      desc: "远离城市喧嚣，寻找心灵的平静。",
      actionId: "garden-stargazing",
      slotCount: 3,
    },
    {
      key: "memory",
      label: "回忆长廊",
      desc: "回顾曾经的苦痛与记忆。",
      actionId: "garden-memory",
      slotCount: 3,
    },
    {
      key: "dream",
      label: "美梦舱室",
      desc: "编织美梦。",
      actionId: "garden-dream",
      slotCount: 3,
    },
  ] as const;

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
                {vm().status === "ready"
                  ? "Operational"
                  : vm().status === "partial"
                    ? "Partially Available"
                    : "Locked"}
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

          {/* NPC illustration area — mirrors GardenWindow NPC host image */}
          <div
            class="garden-npc-area"
            data-source-component="GardenHostImage"
          >
            <div class="garden-npc-frame">
              <span class="garden-npc-label">花园引导者</span>
            </div>
          </div>

          {/* Left panel action buttons — mirrors TalkButton + LeaveButton */}
          <div class="garden-left-actions">
            <button
              class="garden-talk-btn"
              onClick={() => {
                /* Talk is visual-only in current build */
              }}
              data-source-component="TalkButton"
            >
              对话
            </button>
            <button
              class="garden-leave-btn"
              onClick={props.onReturn}
              data-source-component="LeaveButton"
            >
              离开
            </button>
          </div>
        </div>

        {/* Right Panel — mirrors GardenWindow/RightPanel */}
        <div
          class="building-detail-right"
          data-source-hierarchy="GardenWindow/RightPanel"
        >
          {/* Tab bar — mirrors UpgradeTab / UseTab */}
          <div class="garden-tab-bar" data-source-component="TabBar">
            <button
              class={`garden-tab-btn ${activeTab() === "upgrade" ? "garden-tab-btn--active" : ""}`}
              onClick={() => setActiveTab("upgrade")}
              data-tab="upgrade"
            >
              升级设施
            </button>
            <button
              class={`garden-tab-btn ${activeTab() === "use" ? "garden-tab-btn--active" : ""}`}
              onClick={() => setActiveTab("use")}
              data-tab="use"
            >
              使用设施
            </button>
          </div>

          {/* ── Upgrade tab content ── */}
          {activeTab() === "upgrade" && (
            <div class="garden-tab-panel" data-tab-panel="upgrade">
              {upgradeActions().length > 0 ? (
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
              ) : (
                <div class="building-info-card">
                  <p style="margin:0;color:rgba(218,198,168,0.5);font-size:0.82rem;">
                    No upgrade actions currently available.
                  </p>
                </div>
              )}
            </div>
          )}

          {/* ── Use Facilities tab content ── */}
          {activeTab() === "use" && (
            <div class="garden-tab-panel" data-tab-panel="use">
              {hasCategorizedActions() ? (
                <>
                  {/* Stargazing Observatory */}
                  {stargazingActions().length > 0 && (
                    <GardenFacilitySection
                      title="星空观测台"
                      subtitle="远离城市喧嚣，寻找心灵的平静。"
                      actions={stargazingActions()}
                      slotCount={3}
                      onAction={props.onAction}
                    />
                  )}
                  {/* Memory Corridor */}
                  {memoryActions().length > 0 && (
                    <GardenFacilitySection
                      title="回忆长廊"
                      subtitle="回顾曾经的苦痛与记忆。"
                      actions={memoryActions()}
                      slotCount={3}
                      onAction={props.onAction}
                    />
                  )}
                  {/* Dream Chamber */}
                  {dreamActions().length > 0 && (
                    <GardenFacilitySection
                      title="美梦舱室"
                      subtitle="编织美梦。"
                      actions={dreamActions()}
                      slotCount={3}
                      onAction={props.onAction}
                    />
                  )}
                </>
              ) : (
                /* Fallback: render predefined facilities even if actions aren't explicitly categorized */
                <>
                  <For each={FACILITIES}>
                    {(facility) => {
                      const action = () =>
                        vm().actions.find((a) => a.id === facility.actionId) ??
                        vm().actions.find((a) => a.id.includes(facility.key));
                      const matched = action();
                      return (
                        <div
                          class="garden-facility-card"
                          data-facility={facility.key}
                        >
                          <div class="garden-facility-header">
                            <div class="garden-facility-icon">
                              <span class="garden-facility-icon-letter">
                                {facility.label[0]}
                              </span>
                            </div>
                            <div class="garden-facility-info">
                              <h4 class="garden-facility-name">{facility.label}</h4>
                              <p class="garden-facility-desc">{facility.desc}</p>
                            </div>
                          </div>

                          {/* Hero slots — visual placeholder for slot occupancy
                              HB-14-blocker-001: runtime does not yet return per-slot hero occupancy */}
                          <div class="garden-slot-row" data-blocker="HB-14-blocker-001">
                            <For each={Array.from({ length: facility.slotCount }, (_, i) => i)}>
                              {(i) => (
                                <div
                                  class="garden-hero-slot garden-hero-slot--empty"
                                  data-slot-index={i}
                                  title="Empty slot — assign a hero to use this facility"
                                >
                                  <span class="garden-hero-slot-plus">+</span>
                                </div>
                              )}
                            </For>
                          </div>

                          {/* Action button */}
                          <div class="garden-facility-footer">
                            {matched ? (
                              <>
                                <span
                                  class={`building-action-cost ${!matched.isAvailable ? "building-action-cost-unavailable" : ""}`}
                                >
                                  Cost: <strong>{matched.cost}</strong>
                                </span>
                                {matched.isUnsupported ? (
                                  <button
                                    class="building-action-btn building-action-btn--disabled"
                                    disabled
                                  >
                                    Not Available
                                  </button>
                                ) : matched.isAvailable ? (
                                  <button
                                    class="building-action-btn building-action-btn--primary"
                                    onClick={() => props.onAction(matched.id)}
                                  >
                                    {matched.label}
                                  </button>
                                ) : (
                                  <button
                                    class="building-action-btn building-action-btn--disabled"
                                    disabled
                                  >
                                    Prerequisites Not Met
                                  </button>
                                )}
                              </>
                            ) : (
                              <button
                                class="building-action-btn building-action-btn--disabled"
                                disabled
                              >
                                Not Available
                              </button>
                            )}
                          </div>
                        </div>
                      );
                    }}
                  </For>
                </>
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

/* ── Sub-component: Garden Facility Section (when actions are explicitly categorized) ── */

interface GardenFacilitySectionProps {
  title: string;
  subtitle: string;
  actions: ReadonlyArray<{
    id: string;
    label: string;
    description: string;
    cost: string;
    isAvailable: boolean;
    isUnsupported: boolean;
  }>;
  slotCount: number;
  onAction: (actionId: string) => void;
}

const GardenFacilitySection: Component<GardenFacilitySectionProps> = (props) => {
  return (
    <div class="building-action-section">
      <h3 class="building-action-section-title">{props.title}</h3>
      <p style="margin:0 0 10px;color:rgba(218,198,168,0.5);font-size:0.78rem;">
        {props.subtitle}
      </p>

      {/* Hero slots — visual placeholder for slot occupancy
          HB-14-blocker-001: runtime does not yet return per-slot hero occupancy */}
      <div class="garden-slot-row" data-blocker="HB-14-blocker-001">
        <For each={Array.from({ length: props.slotCount }, (_, i) => i)}>
          {(i) => (
            <div
              class="garden-hero-slot garden-hero-slot--empty"
              data-slot-index={i}
              title="Empty slot — assign a hero to use this facility"
            >
              <span class="garden-hero-slot-plus">+</span>
            </div>
          )}
        </For>
      </div>

      <For each={props.actions}>
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
              <span
                class={`building-action-cost ${!action.isAvailable ? "building-action-cost-unavailable" : ""}`}
              >
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
  );
};
