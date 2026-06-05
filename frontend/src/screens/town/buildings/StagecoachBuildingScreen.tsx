import { For, createSignal, type Component } from "solid-js";

import type { BuildingDetailViewModel, BuildingRecruit } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface StagecoachBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

type StagecoachTab = "upgrade" | "recruit";

/**
 * Stagecoach (次元感知塔) building screen — "Use" page.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/StageCoach/StageCoachWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_perception_tower.png
 * GUID: 87a55679f12a1e6489ecdb1d6e6f6b93
 *
 * Reference image: 公会界面-次元感知塔-使用.png
 *   Left panel: tower art + Talk / Leave buttons
 *   Right panel: tabbed parchment (Upgrade Facility / Recruit Personnel)
 *   Bottom: multi-resource cost strip
 */
export const StagecoachBuildingScreen: Component<StagecoachBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;
  const [activeTab, setActiveTab] = createSignal<StagecoachTab>("recruit");

  const upgradeActions = () =>
    vm().actions.filter((a) => a.id.startsWith("upgrade-"));
  const recruitActions = () =>
    vm().actions.filter((a) => a.id === "recruit-hero" || a.id === "view-candidates" || a.id === "rare-recruit");

  const recruits = () => vm().recruits ?? [];

  const handleRecruitClick = (recruit: BuildingRecruit) => {
    if (recruit.isAvailable) {
      props.onAction("recruit-hero");
    }
  };

  return (
    <div class="app-frame stagecoach-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Building Header — mirrors StageCoachWindow/LeftPanel/Icon + Title ── */}
      <BuildingDetailHeader
        buildingId="stagecoach"
        label={vm().label}
        status={vm().status}
        description={vm().description}
        sourcePrefabPath="Assets/Prefabs/UI/Estate/Buildings/StageCoach/StageCoachWindow.prefab"
        sourceSpritePath="Assets/Sprites/town/buildings/building_perception_tower.png"
        sourceGuid="87a55679f12a1e6489ecdb1d6e6f6b93"
      />

      {/* ── Main Content — two-column layout matching reference ── */}
      <div class="stagecoach-content">
        {/* Left Panel — Tower art + Talk / Leave */}
        <div
          class="stagecoach-left"
          data-source-hierarchy="StageCoachWindow/LeftPanel"
        >
          <div class="stagecoach-tower-card">
            <h2 class="stagecoach-tower-title">次元感知塔</h2>
            <p class="stagecoach-tower-subtitle">
              感知次元裂缝，招募异世界战士
            </p>

            {/* Tower art — mirrors StageCoachWindow/LeftPanel/Icon */}
            <div
              class="stagecoach-tower-art"
              data-source-component="BuildingIcon"
              data-source-sprite="Assets/Sprites/town/buildings/building_perception_tower.png"
              data-source-guid="87a55679f12a1e6489ecdb1d6e6f6b93"
            >
              <div class="stagecoach-tower-sprite">
                <span class="stagecoach-tower-face">◠‿◠</span>
              </div>
            </div>

            {/* Talk button — mirrors StageCoachWindow/TalkButton */}
            <button
              class="stagecoach-talk-btn"
              data-source-component="TalkButton"
              data-source-sprite="Assets/Sprites/ui/btn_talk.png"
              onClick={() => props.onAction("talk")}
            >
              对话
            </button>

            {/* Leave button — mirrors StageCoachWindow/LeaveButton */}
            <button
              class="stagecoach-leave-btn"
              onClick={props.onReturn}
              data-source-component="CloseButton"
              data-source-sprite="Assets/Sprites/ui/btn_close.png"
            >
              离开
            </button>
          </div>
        </div>

        {/* Right Panel — Tabbed parchment panel */}
        <div
          class="stagecoach-right"
          data-source-hierarchy="StageCoachWindow/RightPanel"
        >
          {/* Tab bar — mirrors reference checkboxes/tabs */}
          <div class="stagecoach-tab-bar">
            <button
              class={`stagecoach-tab ${activeTab() === "upgrade" ? "stagecoach-tab--active" : ""}`}
              onClick={() => setActiveTab("upgrade")}
              data-tab="upgrade"
            >
              <span class="stagecoach-tab-check">
                {activeTab() === "upgrade" ? "☑" : "☐"}
              </span>
              <span>升级设施</span>
            </button>
            <button
              class={`stagecoach-tab ${activeTab() === "recruit" ? "stagecoach-tab--active" : ""}`}
              onClick={() => setActiveTab("recruit")}
              data-tab="recruit"
            >
              <span class="stagecoach-tab-check">
                {activeTab() === "recruit" ? "☑" : "☐"}
              </span>
              <span>招募人员</span>
            </button>
          </div>

          {/* Tab content — Upgrade Facility */}
          {activeTab() === "upgrade" && (
            <div class="stagecoach-tab-content" data-tab-content="upgrade">
              {upgradeActions().length > 0 ? (
                <div class="stagecoach-action-list">
                  <For each={upgradeActions()}>
                    {(action) => (
                      <div class="stagecoach-action-card">
                        <div class="stagecoach-action-card-header">
                          <span class="stagecoach-action-label">{action.label}</span>
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
                        <p class="stagecoach-action-desc">{action.description}</p>
                        <div class="stagecoach-action-footer">
                          <span class={`stagecoach-action-cost ${!action.isAvailable ? "stagecoach-action-cost-unavailable" : ""}`}>
                            Cost: <strong>{action.cost}</strong>
                          </span>
                          {action.isUnsupported ? (
                            <button class="stagecoach-action-btn stagecoach-action-btn--disabled" disabled>
                              Not Available
                            </button>
                          ) : action.isAvailable ? (
                            <button
                              class="stagecoach-action-btn stagecoach-action-btn--primary"
                              onClick={() => props.onAction(action.id)}
                            >
                              {action.label}
                            </button>
                          ) : (
                            <button class="stagecoach-action-btn stagecoach-action-btn--disabled" disabled>
                              Prerequisites Not Met
                            </button>
                          )}
                        </div>
                      </div>
                    )}
                  </For>
                </div>
              ) : (
                <div class="stagecoach-empty-state">
                  <p>当前没有可用的升级项目。</p>
                </div>
              )}
            </div>
          )}

          {/* Tab content — Recruit Personnel */}
          {activeTab() === "recruit" && (
            <div class="stagecoach-tab-content" data-tab-content="recruit">
              {recruits().length > 0 ? (
                <div class="stagecoach-recruit-list">
                  <For each={recruits()}>
                    {(recruit) => (
                      <div
                        class={`stagecoach-recruit-card ${!recruit.isAvailable ? "stagecoach-recruit-card--unavailable" : ""}`}
                        data-recruit-id={recruit.heroId}
                      >
                        {/* Hero portrait */}
                        <div class="stagecoach-recruit-portrait">
                          {recruit.portrait ? (
                            <img
                              class="stagecoach-recruit-portrait-img"
                              src={recruit.portrait}
                              alt={recruit.name}
                            />
                          ) : (
                            <div class="stagecoach-recruit-portrait-fallback">
                              <span>{recruit.name[0] ?? "?"}</span>
                            </div>
                          )}
                        </div>

                        {/* Hero info */}
                        <div class="stagecoach-recruit-info">
                          <div class="stagecoach-recruit-name-row">
                            <span class="stagecoach-recruit-name">{recruit.name}</span>
                            <span class="stagecoach-recruit-class">{recruit.classLabel}</span>
                          </div>
                          <div class="stagecoach-recruit-stats">
                            <span class="stagecoach-recruit-stat" title="HP">
                              {recruit.hp}
                            </span>
                            <span class="stagecoach-recruit-stat" title="Stress">
                              {recruit.stress} / {recruit.maxStress}
                            </span>
                          </div>
                        </div>

                        {/* Cost + Action */}
                        <div class="stagecoach-recruit-action">
                          <span class={`stagecoach-recruit-cost ${!recruit.isAvailable ? "stagecoach-recruit-cost--unavailable" : ""}`}>
                            {recruit.cost}
                          </span>
                          <button
                            class={`stagecoach-recruit-btn ${recruit.isAvailable ? "stagecoach-recruit-btn--primary" : "stagecoach-recruit-btn--disabled"}`}
                            disabled={!recruit.isAvailable}
                            onClick={() => handleRecruitClick(recruit)}
                          >
                            招募
                          </button>
                        </div>
                      </div>
                    )}
                  </For>
                </div>
              ) : recruitActions().length > 0 ? (
                <div class="stagecoach-action-list">
                  <For each={recruitActions()}>
                    {(action) => (
                      <div class="stagecoach-action-card">
                        <div class="stagecoach-action-card-header">
                          <span class="stagecoach-action-label">{action.label}</span>
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
                        <p class="stagecoach-action-desc">{action.description}</p>
                        <div class="stagecoach-action-footer">
                          <span class={`stagecoach-action-cost ${!action.isAvailable ? "stagecoach-action-cost-unavailable" : ""}`}>
                            Cost: <strong>{action.cost}</strong>
                          </span>
                          {action.isUnsupported ? (
                            <button class="stagecoach-action-btn stagecoach-action-btn--disabled" disabled>
                              Not Available
                            </button>
                          ) : action.isAvailable ? (
                            <button
                              class="stagecoach-action-btn stagecoach-action-btn--primary"
                              onClick={() => props.onAction(action.id)}
                            >
                              {action.label}
                            </button>
                          ) : (
                            <button class="stagecoach-action-btn stagecoach-action-btn--disabled" disabled>
                              Prerequisites Not Met
                            </button>
                          )}
                        </div>
                      </div>
                    )}
                  </For>
                </div>
              ) : (
                <div class="stagecoach-empty-state">
                  <p>当前没有可招募的人员。</p>
                </div>
              )}
            </div>
          )}
        </div>
      </div>

      {/* ── Bottom Resource Bar — mirrors reference cost strip ── */}
      <div class="stagecoach-resource-bar">
        <div class="stagecoach-resource-slot">
          <span class="stagecoach-resource-icon stagecoach-resource-icon--gem" />
          <span class="stagecoach-resource-value">10</span>
        </div>
        <div class="stagecoach-resource-slot">
          <span class="stagecoach-resource-icon stagecoach-resource-icon--bust" />
          <span class="stagecoach-resource-value">10</span>
        </div>
        <div class="stagecoach-resource-slot">
          <span class="stagecoach-resource-icon stagecoach-resource-icon--portrait" />
          <span class="stagecoach-resource-value">10</span>
        </div>
        <div class="stagecoach-resource-slot">
          <span class="stagecoach-resource-icon stagecoach-resource-icon--crest" />
          <span class="stagecoach-resource-value">20</span>
        </div>
        <div class="stagecoach-resource-slot stagecoach-resource-slot--gold">
          <span class="stagecoach-resource-icon stagecoach-resource-icon--gold" />
          <span class="stagecoach-resource-value">6895</span>
        </div>
      </div>

      {/* ── Return to Town — mirrors StageCoachWindow/CloseButton ── */}
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
