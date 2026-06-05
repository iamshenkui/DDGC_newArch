import { For, Show, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface TavernBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

/** Facility configuration matching the reference image. */
const FACILITY_CONFIG: Record<
  string,
  { name: string; description: string; icon: string; slotCount: number }
> = {
  bar: {
    name: "迷幻酒吧",
    description: "百闻不如一醉",
    icon: "🍷",
    slotCount: 4,
  },
  gambling: {
    name: "猩红轮盘",
    description: "在酒局中缓解焦虑",
    icon: "🎰",
    slotCount: 4,
  },
  brothel: {
    name: "秘密舞池",
    description: "在舞池中寻找片刻宁静",
    icon: "💃",
    slotCount: 4,
  },
};

function categorizeAction(actionId: string): string {
  if (actionId.includes("bar") || actionId === "drink") return "bar";
  if (actionId.includes("gambl") || actionId === "gamble") return "gambling";
  if (actionId.includes("dance") || actionId.includes("brothel")) return "brothel";
  return "bar";
}

/**
 * Tavern (迷情乐园) building screen — "Use Facility" view.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Tavern/TavernWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_paradise.png
 * GUID: c0ea280d2704bdb4a9621d6e181e0316
 *
 * Reference image: 公会界面-迷情乐园-使用中.png
 *   Left panel: hostess portrait + Talk / Leave buttons
 *   Right panel: "使用设施" tab with three facilities (bar, roulette, dance floor)
 *   Each facility shows hero slots — some occupied, some empty/locked.
 *
 * Data blocker: hero slot assignments and per-slot costs are not yet
 * wired through the BuildingDetailViewModel contract. Slots render as
 * placeholders until the runtime exposes tavern-occupancy data.
 */
export const TavernBuildingScreen: Component<TavernBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;

  const barActions = () => vm().actions.filter((a) => categorizeAction(a.id) === "bar");
  const gamblingActions = () => vm().actions.filter((a) => categorizeAction(a.id) === "gambling");
  const brothelActions = () => vm().actions.filter((a) => categorizeAction(a.id) === "brothel");

  const facilityGroups = () => [
    { key: "bar" as const, actions: barActions() },
    { key: "gambling" as const, actions: gamblingActions() },
    { key: "brothel" as const, actions: brothelActions() },
  ];

  // Placeholder hero slots to match reference visual density.
  // TODO(HB-33-blocker): Replace with real occupancy data from runtime.
  const placeholderSlots = (count: number) =>
    Array.from({ length: count }, (_, i) => ({
      id: `slot-${i}`,
      occupied: i === 0, // first slot occupied as in reference
      heroName: i === 0 ? "调查者测试1" : undefined,
      cost: i === 0 ? "1250" : undefined,
    }));

  return (
    <div class="app-frame tavern-building-screen" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Building Header ── */}
      <BuildingDetailHeader
        buildingId="tavern"
        label={vm().label}
        status={vm().status}
        description={vm().description}
        sourcePrefabPath="Assets/Prefabs/UI/Estate/Buildings/Tavern/TavernWindow.prefab"
        sourceSpritePath="Assets/Sprites/town/buildings/building_paradise.png"
        sourceGuid="c0ea280d2704bdb4a9621d6e181e0316"
      />

      {/* ── Content: left (hostess + buttons) + right (facilities) ── */}
      <div class="building-detail-content tavern-content">
        {/* Left Panel — mirrors TavernWindow/LeftPanel */}
        <div
          class="building-detail-left tavern-left"
          data-source-hierarchy="TavernWindow/LeftPanel"
        >
          {/* Hostess portrait area */}
          <div class="tavern-hostess-card">
            <div class="tavern-hostess-portrait" data-source-component="HostessImage">
              <span class="tavern-hostess-initial">迷</span>
            </div>
            <p class="tavern-hostess-desc">
              饮酒消遣以安抚疲惫紧张的身心
            </p>
          </div>

          {/* Talk / Leave buttons */}
          <div class="tavern-left-actions">
            <button
              class="tavern-talk-btn"
              onClick={() => props.onAction("talk")}
              data-source-component="TalkButton"
            >
              对话
            </button>
            <button
              class="tavern-leave-btn"
              onClick={props.onReturn}
              data-source-component="LeaveButton"
            >
              离开
            </button>
          </div>

          {/* Building info */}
          <div class="building-info-card tavern-info-card">
            <h3 class="building-info-card-title">Building Status</h3>
            <div class="building-info-row">
              <span class="building-info-label">Status</span>
              <span class="building-info-value">
                {vm().status === "ready" ? "Operational" : vm().status === "partial" ? "Partially Available" : "Locked"}
              </span>
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
        </div>

        {/* Right Panel / Facilities — mirrors TavernWindow/RightPanel */}
        <div
          class="building-detail-right tavern-right"
          data-source-hierarchy="TavernWindow/RightPanel/FacilityWindow"
        >
          {/* Tab row — "Upgrade Facility" vs "Use Facility" */}
          <div class="tavern-tab-bar">
            <span class="tavern-tab tavern-tab--inactive">
              <span class="tavern-tab-checkbox">☐</span>
              升级设施
            </span>
            <span class="tavern-tab tavern-tab--active">
              <span class="tavern-tab-checkbox">☑</span>
              使用设施
            </span>
          </div>

          {/* Facility list */}
          <div class="tavern-facility-list">
            <For each={facilityGroups()}>
              {(group) => {
                const config = FACILITY_CONFIG[group.key];
                const slots = placeholderSlots(config.slotCount);
                const primaryAction = group.actions[0];

                return (
                  <div
                    class="tavern-facility-card"
                    data-source-prefab={`Assets/Prefabs/UI/Tavern${config.name}Slot.prefab`}
                    data-source-component="FacilitySlot"
                  >
                    {/* Facility header: icon + name + description */}
                    <div class="tavern-facility-header">
                      <div class="tavern-facility-icon">{config.icon}</div>
                      <div class="tavern-facility-meta">
                        <span class="tavern-facility-name">{config.name}</span>
                        <span class="tavern-facility-desc">{config.description}</span>
                      </div>
                    </div>

                    {/* Hero slots row */}
                    <div class="tavern-slot-row">
                      <For each={slots}>
                        {(slot) => (
                          <div
                            class={`tavern-slot ${slot.occupied ? "tavern-slot--occupied" : "tavern-slot--empty"}`}
                            data-slot-id={slot.id}
                          >
                            <Show
                              when={slot.occupied}
                              fallback={
                                <span class="tavern-slot-lock">🔒</span>
                              }
                            >
                              <div class="tavern-slot-hero">
                                <div class="tavern-slot-hero-portrait">
                                  <span class="tavern-slot-hero-initial">
                                    {slot.heroName?.[0] ?? "?"}
                                  </span>
                                </div>
                                <span class="tavern-slot-hero-name">{slot.heroName}</span>
                                <span class="tavern-slot-hero-cost">{slot.cost}</span>
                              </div>
                            </Show>
                          </div>
                        )}
                      </For>
                    </div>

                    {/* Action footer — mirrors the cost + button pattern */}
                    <Show when={primaryAction}>
                      <div class="tavern-facility-footer">
                        <span
                          class={`building-action-cost ${!primaryAction.isAvailable ? "building-action-cost-unavailable" : ""}`}
                        >
                          Cost: <strong>{primaryAction.cost}</strong>
                        </span>
                        {primaryAction.isUnsupported ? (
                          <button class="building-action-btn building-action-btn--disabled" disabled>
                            Not Available
                          </button>
                        ) : primaryAction.isAvailable ? (
                          <button
                            class="building-action-btn building-action-btn--primary"
                            onClick={() => props.onAction(primaryAction.id)}
                          >
                            {primaryAction.label}
                          </button>
                        ) : (
                          <button class="building-action-btn building-action-btn--disabled" disabled>
                            Prerequisites Not Met
                          </button>
                        )}
                      </div>
                    </Show>
                  </div>
                );
              }}
            </For>
          </div>

          {/* Fallback when no actions match any facility */}
          {facilityGroups().every((g) => g.actions.length === 0) && (
            <div class="building-info-card">
              <p style="margin:0;color:rgba(218,198,168,0.5);font-size:0.82rem;">
                暂无可用设施。请在升级界面解锁更多设施槽位。
              </p>
            </div>
          )}
        </div>
      </div>

      {/* ── Return to Town ── */}
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

      {/* Data blocker note (hidden from UI, machine-detectable) */}
      <div
        data-blocker="tavern-occupancy"
        data-blocker-desc="Hero slot assignments and per-slot costs are not wired through BuildingDetailViewModel. Slots render as placeholders."
        style="display:none"
      />
    </div>
  );
};
