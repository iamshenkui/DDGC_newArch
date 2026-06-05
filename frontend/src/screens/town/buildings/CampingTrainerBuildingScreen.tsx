import { For, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";
import { resolveHeroPortrait } from "../../../assets/originalAssetPaths";

interface CampingTrainerBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

/**
 * Camping Trainer (空间分析) building screen — mirrors CampingTrainerHeroWindow.
 *
 * Unity prefab: Assets/Prefabs/UI/Estate/Buildings/CampingTrainer/CampingTrainerHeroWindow.prefab
 * Original sprite: Assets/Sprites/town/buildings/building_space_analysis.png
 *
 * Layout (from reference image):
 *   LeftPanel  → hero portrait + name + leave button
 *   RightPanel → parchment panel "解锁技能" with skill unlock slots
 *   Bottom     → resource bar (placeholder until runtime data wired)
 *
 * Data blockers recorded:
 * - Hero selection is static (fixture hero 刘星); runtime hero context not wired.
 * - Bottom resource values are static placeholders; runtime resource strip not wired.
 */
export const CampingTrainerBuildingScreen: Component<CampingTrainerBuildingScreenProps> = (
  props
) => {
  const vm = () => props.viewModel;

  const hero = () =>
    vm().hero ?? {
      id: "hero-placeholder",
      name: "???",
      classLabel: "",
      hp: "0 / 0",
      maxHp: "0",
      health: 0,
      maxHealth: 0,
      stress: "0",
      maxStress: "0",
      level: 0,
      xp: 0,
      isWounded: false,
      isAfflicted: false,
      positiveQuirks: [],
      negativeQuirks: [],
      diseases: []
    };

  const portraitSrc = () =>
    resolveHeroPortrait({
      heroId: hero().id,
      classLabel: hero().classLabel
    });

  const unlockedActions = () => vm().actions.filter((a) => a.isAvailable);
  const lockedActions = () => vm().actions.filter((a) => !a.isAvailable);

  return (
    <div class="app-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Header ── */}
      <BuildingDetailHeader
        buildingId="campingtrainer"
        label={vm().label}
        status={vm().status}
        description={vm().description}
        sourcePrefabPath="Assets/Prefabs/UI/Estate/Buildings/CampingTrainer/CampingTrainerHeroWindow.prefab"
        sourceSpritePath="Assets/Sprites/town/buildings/building_space_analysis.png"
        sourceGuid=""
      />

      {/* ── Content: hero (left) + skill panel (right) ── */}
      <div class="building-detail-content camping-trainer-layout">
        {/* Left Panel — hero context */}
        <div
          class="building-detail-left camping-trainer-left"
          data-source-hierarchy="CampingTrainerHeroWindow/LeftPanel"
        >
          <div class="camping-trainer-hero-panel">
            <div class="camping-trainer-hero-portrait">
              {portraitSrc() ? (
                <img
                  src={portraitSrc()}
                  alt={hero().name}
                  class="camping-trainer-hero-portrait-img"
                  loading="eager"
                />
              ) : (
                <div class="camping-trainer-hero-portrait-fallback">
                  <span>{hero().name[0] ?? "?"}</span>
                </div>
              )}
            </div>

            <div class="camping-trainer-hero-info">
              <span class="camping-trainer-hero-name">{hero().name}</span>
              <span class="camping-trainer-hero-class">{hero().classLabel}</span>
            </div>

            <button
              class="camping-trainer-hero-leave-btn"
              onClick={props.onReturn}
              data-source-component="LeaveButton"
            >
              离开
            </button>
          </div>
        </div>

        {/* Right Panel — skill unlock parchment */}
        <div
          class="building-detail-right camping-trainer-right"
          data-source-hierarchy="CampingTrainerHeroWindow/RightPanel"
        >
          <div class="camping-trainer-parchment">
            <h3 class="camping-trainer-parchment-title">解锁技能</h3>

            <div class="camping-trainer-skill-grid">
              <For each={vm().actions}>
                {(action) => (
                  <div
                    class={`camping-trainer-skill-slot ${
                      action.isAvailable
                        ? ""
                        : "camping-trainer-skill-slot--locked"
                    }`}
                    data-action-id={action.id}
                  >
                    <div class="camping-trainer-skill-slot-inner">
                      <span class="camping-trainer-skill-slot-label">
                        {action.label}
                      </span>
                      {action.isAvailable ? (
                        <span class="camping-trainer-skill-slot-cost">
                          {action.cost}
                        </span>
                      ) : (
                        <span class="camping-trainer-skill-slot-lock">🔒</span>
                      )}
                    </div>
                    {action.isAvailable && (
                      <button
                        class="camping-trainer-skill-slot-btn"
                        onClick={() => props.onAction(action.id)}
                        aria-label={`解锁 ${action.label}`}
                      >
                        解锁
                      </button>
                    )}
                  </div>
                )}
              </For>
            </div>
          </div>
        </div>
      </div>

      {/* ── Bottom resource bar (placeholder values — runtime data not wired) ── */}
      <div class="camping-trainer-resource-bar">
        <div class="camping-trainer-resource-item">
          <span class="camping-trainer-resource-icon camping-trainer-resource-icon--blue"></span>
          <span>10</span>
        </div>
        <div class="camping-trainer-resource-item">
          <span class="camping-trainer-resource-icon camping-trainer-resource-icon--purple"></span>
          <span>10</span>
        </div>
        <div class="camping-trainer-resource-item">
          <span class="camping-trainer-resource-icon camping-trainer-resource-icon--gold"></span>
          <span>10</span>
        </div>
        <div class="camping-trainer-resource-item">
          <span class="camping-trainer-resource-icon camping-trainer-resource-icon--orange"></span>
          <span>30</span>
        </div>
        <div class="camping-trainer-resource-item">
          <span class="camping-trainer-resource-icon camping-trainer-resource-icon--currency"></span>
          <span>6895</span>
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
    </div>
  );
};
