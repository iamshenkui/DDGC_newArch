import { For, Show, createSignal, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";
import { sanitariumUpgradeTrees } from "../../../data/sanitariumUpgrades";

interface SanitariumBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

type TabKey = "upgrade" | "use";

interface UpgradeLevel {
  code: string;
  cost: number;
  effects: Array<{ effect_id: string; value: number }>;
}

interface UpgradeTree {
  tree_id: string;
  levels: UpgradeLevel[];
}

interface BuildingData {
  id: string;
  building_type: string;
  unlock_conditions: Array<{
    condition_type: string;
    required_count: number;
  }>;
  upgrade_trees: UpgradeTree[];
}

const SANITARIUM_UPGRADE_LABELS: Record<string, string> = {
  quirk_positive_cost: "怪癖强化费用",
  quirk_negative_cost: "怪癖移除费用",
  quirk_permanent_negative_cost: "永久怪癖移除",
  quirk_treatment_chance: "怪癖治疗成功率",
  quirk_slots: "怪癖治疗槽位",
  disease_cost: "疾病治疗费用",
  disease_cure_all_chance: "疾病全愈率",
  disease_slots: "疾病治疗槽位",
};

const SANITARIUM_NPC = {
  name: "梅玲",
  title: "细胞修复站医师",
};

function getSanitariumUpgradeTrees(): UpgradeTree[] {
  return (sanitariumUpgradeTrees as unknown as UpgradeTree[]) ?? [];
}

function getCurrentLevelForTree(
  tree: UpgradeTree,
  currentUpgrade?: string
): number {
  if (!currentUpgrade) return 0;
  const match = currentUpgrade.match(/level\s*(\d+)/i);
  if (match) {
    const lvl = parseInt(match[1], 10);
    return Math.min(Math.max(lvl - 1, 0), tree.levels.length - 1);
  }
  return 0;
}

/**
 * Sanitarium (细胞修复站) building screen with Upgrade / Use tabs.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Sanitarium/SanitariumWindow.prefab
 *
 * Reference image: 公会界面-细胞修复站-升级.png
 *   - Tab: 升级设施 (checked) / 使用设施
 *   - Left: NPC portrait (梅玲) with 离开 button
 *   - Right: upgrade tree rows with checkbox progression
 *
 * Original sprite: Assets/Sprites/town/buildings/building_cell_repair.png
 * GUID: 55375034893560044a266e905926e8ff
 */
export const SanitariumBuildingScreen: Component<SanitariumBuildingScreenProps> = (
  props
) => {
  const vm = () => props.viewModel;
  const [activeTab, setActiveTab] = createSignal<TabKey>("upgrade");

  const upgradeTrees = () => getSanitariumUpgradeTrees();

  const quirkActions = () =>
    vm().actions.filter(
      (a) =>
        a.id.includes("quirk") ||
        a.id.includes("positive") ||
        a.id.includes("negative")
    );
  const diseaseActions = () =>
    vm().actions.filter(
      (a) => a.id.includes("disease") || a.id.includes("cure")
    );
  const stressActions = () =>
    vm().actions.filter(
      (a) => a.id.includes("stress") || a.id.includes("therapy")
    );
  const otherActions = () =>
    vm().actions.filter((a) => {
      const id = a.id;
      return (
        !id.includes("quirk") &&
        !id.includes("positive") &&
        !id.includes("negative") &&
        !id.includes("disease") &&
        !id.includes("cure") &&
        !id.includes("stress") &&
        !id.includes("therapy")
      );
    });

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

      {/* ── Tab Bar — mirrors UpgradeWindow tab strip ── */}
      <div class="sanitarium-tab-bar" data-source-hierarchy="SanitariumWindow/TabBar">
        <button
          class={`sanitarium-tab-btn ${activeTab() === "upgrade" ? "sanitarium-tab-btn--active" : ""}`}
          onClick={() => setActiveTab("upgrade")}
          data-testid="sanitarium-tab-upgrade"
        >
          <span class="sanitarium-tab-check">{activeTab() === "upgrade" ? "☑" : "☐"}</span>
          升级设施
        </button>
        <button
          class={`sanitarium-tab-btn ${activeTab() === "use" ? "sanitarium-tab-btn--active" : ""}`}
          onClick={() => setActiveTab("use")}
          data-testid="sanitarium-tab-use"
        >
          <span class="sanitarium-tab-check">{activeTab() === "use" ? "☑" : "☐"}</span>
          使用设施
        </button>
      </div>

      {/* ── Content ── */}
      <div class="sanitarium-content">
        {/* Left Panel — NPC portrait + info */}
        <div
          class="sanitarium-left-panel"
          data-source-hierarchy="SanitariumWindow/LeftPanel"
        >
          <div class="sanitarium-npc-card">
            <div class="sanitarium-npc-portrait">
              <div class="sanitarium-npc-avatar">
                <span class="sanitarium-npc-initial">
                  {SANITARIUM_NPC.name[0]}
                </span>
              </div>
            </div>
            <div class="sanitarium-npc-info">
              <span class="sanitarium-npc-name">{SANITARIUM_NPC.name}</span>
              <span class="sanitarium-npc-title">{SANITARIUM_NPC.title}</span>
            </div>
            <button
              class="sanitarium-leave-btn"
              onClick={props.onReturn}
              data-source-component="CloseButton"
            >
              离开
            </button>
          </div>

          {/* Building info card */}
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
            <Show when={vm().currentUpgrade}>
              <div class="building-info-row">
                <span class="building-info-label">Treatment Level</span>
                <span class="building-info-value">{vm().currentUpgrade}</span>
              </div>
            </Show>
            <Show when={vm().upgradeRequirement}>
              <div class="building-info-row">
                <span class="building-info-label">Requirement</span>
                <span class="building-info-value">{vm().upgradeRequirement}</span>
              </div>
            </Show>
          </div>
        </div>

        {/* Right Panel */}
        <div
          class="sanitarium-right-panel"
          data-source-hierarchy="SanitariumWindow/RightPanel"
        >
          {/* ── Upgrade Tab ── */}
          <Show when={activeTab() === "upgrade"}>
            <div class="sanitarium-upgrade-view">
              <h3 class="sanitarium-upgrade-title">设施升级</h3>
              <Show
                when={upgradeTrees().length > 0}
                fallback={
                  <div class="building-info-card">
                    <p style="margin:0;color:rgba(218,198,168,0.5);font-size:0.82rem;">
                      Upgrade data not available.
                    </p>
                  </div>
                }
              >
                <For each={upgradeTrees()}>
                  {(tree) => {
                    const currentLvl = () =>
                      getCurrentLevelForTree(tree, vm().currentUpgrade);
                    const label =
                      SANITARIUM_UPGRADE_LABELS[tree.tree_id] ?? tree.tree_id;

                    return (
                      <div
                        class="sanitarium-upgrade-tree"
                        data-tree-id={tree.tree_id}
                      >
                        <div class="sanitarium-upgrade-tree-header">
                          <div class="sanitarium-upgrade-tree-icon">
                            <span>◈</span>
                          </div>
                          <span class="sanitarium-upgrade-tree-label">
                            {label}
                          </span>
                        </div>
                        <div class="sanitarium-upgrade-slots">
                          <For each={tree.levels}>
                            {(level, idx) => {
                              const isPurchased = idx() <= currentLvl();
                              const isNext = idx() === currentLvl() + 1;
                              return (
                                <div
                                  class={`sanitarium-upgrade-slot ${isPurchased ? "sanitarium-upgrade-slot--purchased" : ""} ${isNext ? "sanitarium-upgrade-slot--next" : ""}`}
                                  data-level-code={level.code}
                                  title={`${label} ${level.code.toUpperCase()}: ${level.cost > 0 ? `${level.cost} Gold` : "Free"}`}
                                >
                                  <span class="sanitarium-upgrade-slot-box">
                                    {isPurchased ? "☑" : "☐"}
                                  </span>
                                  <Show when={level.cost > 0 && !isPurchased}>
                                    <span class="sanitarium-upgrade-slot-cost">
                                      {level.cost}
                                    </span>
                                  </Show>
                                </div>
                              );
                            }}
                          </For>
                        </div>
                      </div>
                    );
                  }}
                </For>
              </Show>

              {/* Data blocker note */}
              <div class="sanitarium-blocker-note">
                <span class="sanitarium-blocker-label">Data Status</span>
                <span>
                  Upgrade tree definitions loaded from Buildings.json. Current
                  purchase levels require runtime save-data integration
                  (BLOCKER-SANITARIUM-001).
                </span>
              </div>
            </div>
          </Show>

          {/* ── Use Tab ── */}
          <Show when={activeTab() === "use"}>
            <div class="sanitarium-use-view">
              {/* Disease Treatment */}
              <Show when={diseaseActions().length > 0}>
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
              </Show>

              {/* Quirk Treatment */}
              <Show when={quirkActions().length > 0}>
                <div class="building-action-section">
                  <h3 class="building-action-section-title">Quirk Treatment</h3>
                  <For each={quirkActions()}>
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
              </Show>

              {/* Stress Treatment */}
              <Show when={stressActions().length > 0}>
                <div class="building-action-section">
                  <h3 class="building-action-section-title">Stress Treatment</h3>
                  <For each={stressActions()}>
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
              </Show>

              {/* Other actions */}
              <Show when={otherActions().length > 0}>
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
              </Show>

              {/* Fallback when no actions match any category */}
              <Show
                when={
                  diseaseActions().length === 0 &&
                  quirkActions().length === 0 &&
                  stressActions().length === 0 &&
                  otherActions().length === 0
                }
              >
                <div class="building-action-section">
                  <h3 class="building-action-section-title">Actions</h3>
                  {vm().actions.length === 0 ? (
                    <div class="building-info-card">
                      <p
                        style="margin:0;color:rgba(218,198,168,0.5);font-size:0.82rem;"
                      >
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
                  )}
                </div>
              </Show>
            </div>
          </Show>
        </div>
      </div>

      {/* ── Return to Town — mirrors SanitariumWindow/CloseButton ── */}
      <Show when={activeTab() === "use"}>
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
      </Show>
    </div>
  );
};
