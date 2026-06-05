import { For, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface SpatialAnalysisBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

/**
 * Spatial Analysis (空间分析 / campingtrainer) building screen.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/CampingTrainer/CampingTrainerWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_space_analysis.png
 * GUID: (not yet assigned in catalog)
 *
 * Reference image: 公会界面-空间分析-使用.png
 *   Left panel: NPC character portrait, title "空间分析", label "同负人",
 *               lore description, bottom buttons "对话" / "离开"
 *   Right panel: "解锁技能" skill unlock grid with icon slots and description.
 */
export const SpatialAnalysisBuildingScreen: Component<SpatialAnalysisBuildingScreenProps> = (
  props
) => {
  const vm = () => props.viewModel;

  const unlockActions = () => vm().actions.filter((a) => a.id === "unlock-skill");
  const dialogueActions = () => vm().actions.filter((a) => a.id === "dialogue");
  const otherActions = () =>
    vm().actions.filter((a) => a.id !== "unlock-skill" && a.id !== "dialogue");

  // Placeholder skill grid data — matches the reference image layout.
  // In a full implementation these would come from the view model.
  const skillSlots = () => [
    { id: "s1", label: "侦查", unlocked: true },
    { id: "s2", label: "隐匿", unlocked: true },
    { id: "s3", label: "空间感知", unlocked: false },
    { id: "s4", label: "相位移动", unlocked: false },
    { id: "s5", label: "维度锚定", unlocked: false },
    { id: "s6", label: "营火精通", unlocked: true },
    { id: "s7", label: "野外生存", unlocked: true },
    { id: "s8", label: "轨迹分析", unlocked: false },
    { id: "s9", label: "共鸣探测", unlocked: false },
    { id: "s10", label: "裂隙行走", unlocked: false },
  ];

  return (
    <div class="app-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Building Header — mirrors CampingTrainerWindow/LeftPanel/Icon + Title ── */}
      <BuildingDetailHeader
        buildingId="campingtrainer"
        label={vm().label}
        status={vm().status}
        description={vm().description}
        sourcePrefabPath="Assets/Prefabs/UI/Estate/Buildings/CampingTrainer/CampingTrainerWindow.prefab"
        sourceSpritePath="Assets/Sprites/town/buildings/building_space_analysis.png"
        sourceGuid=""
      />

      {/* ── Content — mirrors CampingTrainerWindow LeftPanel + RightPanel ── */}
      <div class="building-detail-content">
        {/* Left Panel — NPC character + info */}
        <div
          class="building-detail-left"
          data-source-hierarchy="CampingTrainerWindow/LeftPanel"
        >
          {/* Character portrait card */}
          <div class="building-info-card spatial-analysis-npc-card">
            <h3 class="building-info-card-title">空间分析</h3>

            {/* NPC portrait area */}
            <div
              class="spatial-analysis-portrait"
              data-source-component="NPCPortrait"
            >
              <div class="spatial-analysis-portrait-frame">
                <span class="spatial-analysis-portrait-initial">同</span>
              </div>
            </div>

            {/* NPC name / role */}
            <div class="spatial-analysis-npc-name">同负人</div>
            <p class="spatial-analysis-npc-desc">
              来自东方边境的观察者，拥有在空间中感知微小波动的天赋。
              他可以帮助你的队伍掌握新的露营技能，提升在危险地带中的生存能力。
            </p>

            {/* Bottom action buttons */}
            <div class="spatial-analysis-npc-actions">
              {dialogueActions().length > 0 && (
                <For each={dialogueActions()}>
                  {(action) => (
                    <button
                      class={`building-action-btn ${action.isAvailable ? "building-action-btn--primary" : "building-action-btn--disabled"}`}
                      disabled={!action.isAvailable}
                      onClick={() => props.onAction(action.id)}
                      data-source-component="TalkButton"
                    >
                      对话
                    </button>
                  )}
                </For>
              )}
              <button
                class="building-action-btn"
                onClick={props.onReturn}
                data-source-component="LeaveButton"
              >
                离开
              </button>
            </div>
          </div>
        </div>

        {/* Right Panel — Skill unlock grid */}
        <div
          class="building-detail-right"
          data-source-hierarchy="CampingTrainerWindow/RightPanel/SkillGrid"
        >
          <div class="building-action-section">
            <h3 class="building-action-section-title">解锁技能</h3>

            {/* Skill icon grid */}
            <div
              class="spatial-analysis-skill-grid"
              data-source-component="SkillGrid"
            >
              <For each={skillSlots()}>
                {(slot) => (
                  <div
                    class={`spatial-analysis-skill-slot ${slot.unlocked ? "spatial-analysis-skill-slot--unlocked" : "spatial-analysis-skill-slot--locked"}`}
                    title={slot.label}
                    data-skill-id={slot.id}
                  >
                    <div class="spatial-analysis-skill-icon">
                      <span class="spatial-analysis-skill-icon-text">
                        {slot.label[0]}
                      </span>
                    </div>
                    <span class="spatial-analysis-skill-label">{slot.label}</span>
                  </div>
                )}
              </For>
            </div>

            {/* Skill description area */}
            <div
              class="spatial-analysis-skill-desc"
              data-source-component="SkillDescription"
            >
              <p>
                同负人掌握着多种空间分析与露营技巧。解锁新技能需要消耗一定的金币，
                并随着建筑等级提升可解锁更高级的能力。
              </p>
            </div>

            {/* Unlock action */}
            {unlockActions().length > 0 && (
              <For each={unlockActions()}>
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
            )}

            {/* Other actions fallback */}
            {otherActions().length > 0 && (
              <For each={otherActions()}>
                {(action) => (
                  <div class="building-action-card">
                    <div class="building-action-card-header">
                      <span class="building-action-label">{action.label}</span>
                    </div>
                    <p class="building-action-desc">{action.description}</p>
                    <div class="building-action-footer">
                      <span class="building-action-cost">
                        Cost: <strong>{action.cost}</strong>
                      </span>
                      <button
                        class={`building-action-btn ${action.isAvailable ? "building-action-btn--primary" : "building-action-btn--disabled"}`}
                        disabled={!action.isAvailable}
                        onClick={() => props.onAction(action.id)}
                      >
                        {action.label}
                      </button>
                    </div>
                  </div>
                )}
              </For>
            )}
          </div>
        </div>
      </div>

      {/* ── Return to Town — mirrors CampingTrainerWindow/CloseButton ── */}
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
