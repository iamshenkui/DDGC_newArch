import { For, createSignal, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface BlacksmithBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

type TabKey = "upgrade" | "use";

const UPGRADE_SLOT_COUNT = 5;

/**
 * Blacksmith (锻造舱) building screen — upgrade view.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Blacksmith/BlacksmithWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_forging.png
 * GUID: 23e01c10f262ddc4ba9977b91314b031
 *
 * Reference image: 公会界面-锻造仓-升级.png
 *   Left panel: NPC portrait + Talk / Leave buttons
 *   Top tabs: 升级设施 (Upgrade) / 使用设施 (Use)
 *   Right panel: upgrade slot grid (武器锻造 / 护甲强化)
 *   Bottom: resource cost bar
 */
export const BlacksmithBuildingScreen: Component<BlacksmithBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;
  const [activeTab, setActiveTab] = createSignal<TabKey>("upgrade");

  const upgradeCategories = () => vm().upgradeCategories ?? deriveUpgradeCategories(vm().actions);
  const resources = () => vm().resources ?? deriveResources(vm().actions);

  const handleTalk = () => {
    const firstAction = vm().actions[0];
    if (firstAction) props.onAction(firstAction.id);
  };

  return (
    <div class="app-frame blacksmith-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
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

      {/* ── Main Content: left (NPC + buttons) + right (tabs + upgrade grid) ── */}
      <div class="blacksmith-content">
        {/* Left Panel — NPC portrait + Talk / Leave buttons */}
        <div
          class="blacksmith-left"
          data-source-hierarchy="BlacksmithWindow/LeftPanel"
        >
          <div class="blacksmith-npc-card">
            <div class="blacksmith-npc-portrait">
              <span class="blacksmith-npc-initial">锻</span>
            </div>
            <div class="blacksmith-npc-actions">
              <button
                class="blacksmith-npc-btn blacksmith-npc-btn--talk"
                onClick={handleTalk}
                data-source-component="TalkButton"
              >
                对话
              </button>
              <button
                class="blacksmith-npc-btn blacksmith-npc-btn--leave"
                onClick={props.onReturn}
                data-source-component="CloseButton"
                data-source-sprite="Assets/Sprites/ui/btn_close.png"
              >
                离开
              </button>
            </div>
          </div>
        </div>

        {/* Right Panel — tabs + upgrade grid */}
        <div
          class="blacksmith-right"
          data-source-hierarchy="BlacksmithWindow/RightPanel/UpgradeWindow"
        >
          {/* Tab bar — 升级设施 / 使用设施 */}
          <div class="blacksmith-tab-bar">
            <button
              class={`blacksmith-tab-btn ${activeTab() === "upgrade" ? "blacksmith-tab-btn--active" : ""}`}
              onClick={() => setActiveTab("upgrade")}
              data-tab="upgrade"
            >
              <span class="blacksmith-tab-check">{activeTab() === "upgrade" ? "☑" : "☐"}</span>
              升级设施
            </button>
            <button
              class={`blacksmith-tab-btn ${activeTab() === "use" ? "blacksmith-tab-btn--active" : ""}`}
              onClick={() => setActiveTab("use")}
              data-tab="use"
            >
              <span class="blacksmith-tab-check">{activeTab() === "use" ? "☑" : "☐"}</span>
              使用设施
            </button>
          </div>

          {/* Upgrade panel */}
          {activeTab() === "upgrade" && (
            <div class="blacksmith-upgrade-panel">
              <For each={upgradeCategories()}>
                {(category) => (
                  <div
                    class="blacksmith-upgrade-category"
                    data-category-id={category.id}
                  >
                    <div class="blacksmith-upgrade-category-header">
                      <span class="blacksmith-upgrade-category-icon" aria-hidden="true">
                        {category.id.includes("weapon") ? "⚔️" : category.id.includes("armor") ? "🛡️" : "🔧"}
                      </span>
                      <span class="blacksmith-upgrade-category-label">{category.label}</span>
                    </div>
                    <div class="blacksmith-upgrade-slots">
                      <For each={category.slots}>
                        {(slot) => (
                          <button
                            class={`blacksmith-upgrade-slot ${slot.isCurrent ? "blacksmith-upgrade-slot--current" : ""} ${slot.isUnlocked ? "blacksmith-upgrade-slot--unlocked" : "blacksmith-upgrade-slot--locked"}`}
                            disabled={!slot.isUnlocked}
                            onClick={() => {
                              if (slot.isUnlocked) {
                                props.onAction(`${category.id}-level-${slot.level}`);
                              }
                            }}
                            data-level={slot.level}
                            data-unlocked={slot.isUnlocked}
                            data-current={slot.isCurrent}
                            title={slot.isUnlocked ? `${slot.label} — ${slot.cost ?? "Free"}` : `${slot.label} — Locked`}
                          >
                            {slot.isCurrent && <span class="blacksmith-upgrade-slot-dot" />}
                          </button>
                        )}
                      </For>
                    </div>
                  </div>
                )}
              </For>

              {upgradeCategories().length === 0 && (
                <div class="blacksmith-upgrade-empty">
                  <p>暂无可用升级项目</p>
                </div>
              )}
            </div>
          )}

          {/* Use panel */}
          {activeTab() === "use" && (
            <div class="blacksmith-use-panel">
              <div class="building-action-section">
                <h3 class="building-action-section-title">设施服务</h3>
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
              </div>
            </div>
          )}
        </div>
      </div>

      {/* ── Resource bar — mirrors BlacksmithWindow/BottomPanel/ResourceCost ── */}
      {resources().length > 0 && (
        <div class="blacksmith-resource-bar">
          <For each={resources()}>
            {(resource) => (
              <span class="blacksmith-resource-slot" data-resource-type={resource.type}>
                <span class="blacksmith-resource-icon" aria-hidden="true">
                  {resource.type === "gem" && "💎"}
                  {resource.type === "shard" && "🔮"}
                  {resource.type === "core" && "⚙️"}
                  {resource.type === "gold" && "🪙"}
                  {resource.type === "deed" && "📜"}
                </span>
                <span class="blacksmith-resource-amount">{resource.amount}</span>
              </span>
            )}
          </For>
        </div>
      )}

      {/* ── Return to Town — fallback navigation ── */}
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

/**
 * Derive upgrade categories from the legacy action list when the view model
 * does not carry explicit upgrade slot data. This preserves backward
 * compatibility with runtime bridges that only populate `actions`.
 */
function deriveUpgradeCategories(actions: ReadonlyArray<{ id: string; label: string; cost: string; isAvailable: boolean }>) {
  const weaponAction = actions.find((a) => a.id.includes("weapon"));
  const armorAction = actions.find((a) => a.id.includes("armor"));

  const categories = [];

  if (weaponAction) {
    categories.push({
      id: "weapon",
      label: "武器锻造",
      slots: generateSlots(weaponAction.isAvailable, weaponAction.cost)
    });
  }

  if (armorAction) {
    categories.push({
      id: "armor",
      label: "护甲强化",
      slots: generateSlots(armorAction.isAvailable, armorAction.cost)
    });
  }

  return categories;
}

function generateSlots(isAvailable: boolean, cost: string) {
  return Array.from({ length: UPGRADE_SLOT_COUNT }, (_, i) => ({
    level: i + 1,
    label: `Level ${i + 1}`,
    isUnlocked: isAvailable || i === 0,
    isCurrent: i === 0,
    cost
  }));
}

function deriveResources(actions: ReadonlyArray<{ cost: string }>) {
  const costs = actions
    .filter((a) => a.cost.includes("Gold"))
    .map((a) => {
      const match = a.cost.match(/(\d+)/);
      return match ? parseInt(match[1], 10) : 0;
    });

  const maxCost = costs.length > 0 ? Math.max(...costs) : 0;

  if (maxCost === 0) return [];

  return [
    { type: "gem", label: "Gem", amount: 10 },
    { type: "shard", label: "Shard", amount: 10 },
    { type: "core", label: "Core", amount: 10 },
    { type: "gold", label: "Gold", amount: Math.floor(maxCost / 20) },
  ];
}
