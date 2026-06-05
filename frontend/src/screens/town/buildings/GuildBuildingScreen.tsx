import { For, createSignal, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface GuildBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

type GuildTab = "upgrade" | "use";

/**
 * Guild (试炼场) building screen.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Guild/GuildWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_train_field.png
 * GUID: 67a5e7aed8029d84dbf9c9e497a944d2
 *
 * Related prefabs:
 *   Assets/Prefabs/UI/SkillUpgradeSlot.prefab  — skill upgrade slots
 *   Assets/Prefabs/UI/UpgradeSlot.prefab       — equipment upgrade slots
 *
 * Building data (data/Buildings.json):
 *   guild_training — skill training level upgrades
 *   guild_equipment — equipment tier upgrades
 *
 * Reference image: 公会界面-试炼场-使用空.png
 *   - Two tabs: 升级设施 (Upgrade Facility) and 使用设施 (Use Facility)
 *   - Left panel: NPC portrait, Talk button, Leave button
 *   - Right panel: scrollable grid of training slots (empty in 使用空 state)
 *   - Bottom: resource strip (bust/portrait/deed/crest/gold)
 */
export const GuildBuildingScreen: Component<GuildBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;
  const [activeTab, setActiveTab] = createSignal<GuildTab>("use");

  const trainingActions = () => vm().actions.filter((a) => a.id.startsWith("train-"));
  const equipmentActions = () => vm().actions.filter((a) => a.id.startsWith("upgrade-"));
  const otherActions = () =>
    vm().actions.filter((a) => !a.id.startsWith("train-") && !a.id.startsWith("upgrade-"));

  const slotCount = () => vm().slotCount ?? 3;
  const trainingSlots = () =>
    vm().trainingSlots ?? Array.from({ length: slotCount() }, (_, i) => ({
      slotIndex: i,
      isOccupied: false
    }));

  const resourceStrip = () => [
    { key: "bust", label: "Bust", value: 10 },
    { key: "portrait", label: "Portrait", value: 10 },
    { key: "deed", label: "Deed", value: 10 },
    { key: "crest", label: "Crest", value: 30 },
    { key: "gold", label: "Gold", value: 6885 }
  ];

  return (
    <div class="app-frame guild-building-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Building Header — mirrors GuildWindow/LeftPanel/Icon + Title ── */}
      <BuildingDetailHeader
        buildingId="guild"
        label={vm().label}
        status={vm().status}
        description={vm().description}
        sourcePrefabPath="Assets/Prefabs/UI/Estate/Buildings/Guild/GuildWindow.prefab"
        sourceSpritePath="Assets/Sprites/town/buildings/building_train_field.png"
        sourceGuid="67a5e7aed8029d84dbf9c9e497a944d2"
      />

      {/* ── Tab Bar — mirrors GuildWindow tab toggle (升级设施 / 使用设施) ── */}
      <div class="guild-tab-bar" data-source-hierarchy="GuildWindow/TabBar">
        <button
          class={`guild-tab-btn ${activeTab() === "upgrade" ? "guild-tab-btn--active" : ""}`}
          onClick={() => setActiveTab("upgrade")}
          data-tab-id="upgrade"
          aria-pressed={activeTab() === "upgrade"}
        >
          <span class="guild-tab-check">{activeTab() === "upgrade" ? "☑" : "☐"}</span>
          <span>升级设施</span>
        </button>
        <button
          class={`guild-tab-btn ${activeTab() === "use" ? "guild-tab-btn--active" : ""}`}
          onClick={() => setActiveTab("use")}
          data-tab-id="use"
          aria-pressed={activeTab() === "use"}
        >
          <span class="guild-tab-check">{activeTab() === "use" ? "☑" : "☐"}</span>
          <span>使用设施</span>
        </button>
      </div>

      {/* ── Content: left (NPC + controls) + right (panel content) ── */}
      <div class="building-detail-content guild-building-content">
        {/* Left Panel — mirrors GuildWindow/LeftPanel */}
        <div
          class="building-detail-left guild-left-panel"
          data-source-hierarchy="GuildWindow/LeftPanel"
        >
          {/* NPC Portrait — guild master / trainer */}
          <div
            class="guild-npc-portrait"
            data-source-component="NPCPortrait"
            data-source-sprite="Assets/Sprites/ui/guild_master.png"
          >
            <div class="guild-npc-placeholder">
              <span class="guild-npc-label">训练师</span>
            </div>
          </div>

          {/* Talk button */}
          <button
            class="guild-npc-btn guild-talk-btn"
            data-source-component="TalkButton"
            onClick={() => props.onAction("talk")}
          >
            对话
          </button>

          {/* Leave button */}
          <button
            class="guild-npc-btn guild-leave-btn"
            data-source-component="LeaveButton"
            onClick={props.onReturn}
          >
            离开
          </button>
        </div>

        {/* Right Panel — tab-dependent content */}
        <div
          class="building-detail-right guild-right-panel"
          data-source-hierarchy="GuildWindow/RightPanel"
        >
          {/* ── Upgrade Facility Tab ── */}
          {activeTab() === "upgrade" && (
            <>
              {/* Skill Training — mirrors SkillUpgradeSlot */}
              {trainingActions().length > 0 && (
                <div class="building-action-section">
                  <h3 class="building-action-section-title">Skill Training</h3>
                  <For each={trainingActions()}>
                    {(action) => (
                      <div
                        class="building-action-card"
                        data-source-prefab="Assets/Prefabs/UI/SkillUpgradeSlot.prefab"
                        data-source-component="SkillUpgradeSlot"
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

              {/* Equipment Upgrades — mirrors UpgradeSlot / EquipmentUpgradeSlot */}
              {equipmentActions().length > 0 && (
                <div class="building-action-section">
                  <h3 class="building-action-section-title">Equipment Upgrades</h3>
                  <For each={equipmentActions()}>
                    {(action) => (
                      <div
                        class="building-action-card"
                        data-source-prefab="Assets/Prefabs/UI/UpgradeSlot.prefab"
                        data-source-component="UpgradeSlot"
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
            </>
          )}

          {/* ── Use Facility Tab — mirrors GuildWindow/RightPanel/TrainingWindow ── */}
          {activeTab() === "use" && (
            <div
              class="guild-use-panel"
              data-source-hierarchy="GuildWindow/RightPanel/TrainingWindow"
            >
              <div class="guild-use-panel-header">
                <h3 class="building-action-section-title">训练场</h3>
                <span class="guild-slot-info">
                  可用槽位: {slotCount()}
                </span>
              </div>

              {/* Training slot grid — empty state (使用空) */}
              <div class="guild-slot-grid" data-testid="guild-slot-grid">
                <For each={trainingSlots()}>
                  {(slot) => (
                    <div
                      class={`guild-slot-row ${slot.isOccupied ? "guild-slot-row--occupied" : "guild-slot-row--empty"}`}
                      data-slot-index={slot.slotIndex}
                      data-testid={`guild-slot-row-${slot.slotIndex}`}
                    >
                      {/* Checkbox / selection indicator */}
                      <span class="guild-slot-check">
                        {slot.isOccupied ? "☑" : "☐"}
                      </span>

                      {/* Hero name or empty placeholder */}
                      <span class="guild-slot-hero">
                        {slot.isOccupied && slot.heroName
                          ? slot.heroName
                          : "— 空槽位 —"}
                      </span>

                      {/* Skill upgrade slots — shown as locked (X) in empty state */}
                      <div class="guild-skill-slots">
                        <For each={[0, 1, 2, 3, 4]}>
                          {(skillIdx) => (
                            <span
                              class="guild-skill-slot guild-skill-slot--locked"
                              data-skill-index={skillIdx}
                              title="未解锁"
                            >
                              ✕
                            </span>
                          )}
                        </For>
                      </div>
                    </div>
                  )}
                </For>
              </div>

              {/* Empty state hint */}
              <div class="guild-empty-hint">
                <p>暂无英雄在训练中。请选择英雄进行技能训练。</p>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* ── Bottom Resource Strip ── */}
      <div class="guild-resource-strip" data-source-hierarchy="GuildWindow/ResourceStrip">
        <For each={resourceStrip()}>
          {(res) => (
            <span class="guild-resource-slot" data-resource={res.key}>
              <span class="guild-resource-label">{res.label}</span>
              <span class="guild-resource-value">{res.value}</span>
            </span>
          )}
        </For>
      </div>

      {/* ── Return to Town — mirrors GuildWindow/CloseButton ── */}
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
