import { For, Show, type Component, createSignal, createMemo } from "solid-js";

import type { BuildingDetailViewModel, TownHeroSummary } from "../../../bridge/contractTypes";
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
 * Reference page: 公会界面-试炼场-使用
 *   - Tab: 使用设施 (Use Facility)
 *   - Left: Hero portrait preview + 离开 button
 *   - Right: Training slot grid with hero assignments
 *   - Bottom: Resource cost strip
 */
export const GuildBuildingScreen: Component<GuildBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;
  const [activeTab, setActiveTab] = createSignal<GuildTab>("use");
  const [selectedSlotIndex, setSelectedSlotIndex] = createSignal<number>(0);

  const trainingActions = () => vm().actions.filter((a) => a.id.startsWith("train-"));
  const equipmentActions = () => vm().actions.filter((a) => a.id.startsWith("upgrade-"));
  const otherActions = () =>
    vm().actions.filter((a) => !a.id.startsWith("train-") && !a.id.startsWith("upgrade-"));

  const heroes = () => vm().heroes ?? [];
  const trainingSlots = () => vm().trainingSlots ?? [];
  const resources = () => vm().resources;

  const selectedSlot = createMemo(() => {
    const slots = trainingSlots();
    const idx = selectedSlotIndex();
    return slots[idx] ?? slots[0];
  });

  const selectedHero = createMemo(() => {
    const slot = selectedSlot();
    if (!slot?.heroId) return undefined;
    return heroes().find((h) => h.id === slot.heroId);
  });

  const resolveHeroPortrait = (hero: TownHeroSummary | undefined) => {
    if (!hero) return undefined;
    const classKey = hero.classLabel.toLowerCase();
    return `/original/heroes/${classKey}_portrait_roster.png`;
  };

  return (
    <div class="app-frame guild-building-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Building Header ── */}
      <BuildingDetailHeader
        buildingId="guild"
        label={vm().label}
        status={vm().status}
        description={vm().description}
        sourcePrefabPath="Assets/Prefabs/UI/Estate/Buildings/Guild/GuildWindow.prefab"
        sourceSpritePath="Assets/Sprites/town/buildings/building_train_field.png"
        sourceGuid="67a5e7aed8029d84dbf9c9e497a944d2"
      />

      {/* ── Tabs ── */}
      <div class="guild-tabs" data-source-component="GuildTabBar">
        <button
          class={`guild-tab ${activeTab() === "upgrade" ? "guild-tab--active" : ""}`}
          onClick={() => setActiveTab("upgrade")}
          data-tab-id="upgrade"
        >
          升级设施
        </button>
        <button
          class={`guild-tab ${activeTab() === "use" ? "guild-tab--active" : ""}`}
          onClick={() => setActiveTab("use")}
          data-tab-id="use"
        >
          使用设施
        </button>
      </div>

      {/* ── Upgrade Facility Tab ── */}
      <Show when={activeTab() === "upgrade"}>
        <div class="building-detail-content">
          <div class="building-detail-left" data-source-hierarchy="GuildWindow/LeftPanel">
            <div class="building-info-card">
              <h3 class="building-info-card-title">Building Status</h3>
              <div class="building-info-row">
                <span class="building-info-label">Status</span>
                <span class="building-info-value">
                  {vm().status === "ready" ? "Operational" : vm().status === "partial" ? "Partially Available" : "Locked"}
                </span>
              </div>
              {vm().currentUpgrade && (
                <div class="building-info-row">
                  <span class="building-info-label">Guild Level</span>
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

          <div class="building-detail-right" data-source-hierarchy="GuildWindow/RightPanel/UpgradeWindow">
            {trainingActions().length > 0 && (
              <div class="building-action-section">
                <h3 class="building-action-section-title">Skill Training</h3>
                <For each={trainingActions()}>
                  {(action) => (
                    <div
                      class="building-action-card"
                      data-source-prefab="Assets/Prefabs/UI/SkillUpgradeSlot.prefab"
                    >
                      <div class="building-action-card-header">
                        <span class="building-action-label">{action.label}</span>
                        {action.isUnsupported && (
                          <span class="building-action-pill building-action-pill--unsupported">Unsupported</span>
                        )}
                        {!action.isAvailable && !action.isUnsupported && (
                          <span class="building-action-pill building-action-pill--unavailable">Unavailable</span>
                        )}
                      </div>
                      <p class="building-action-desc">{action.description}</p>
                      <div class="building-action-footer">
                        <span class={`building-action-cost ${!action.isAvailable ? "building-action-cost-unavailable" : ""}`}>
                          Cost: <strong>{action.cost}</strong>
                        </span>
                        {action.isUnsupported ? (
                          <button class="building-action-btn building-action-btn--disabled" disabled>Not Available</button>
                        ) : action.isAvailable ? (
                          <button
                            class="building-action-btn building-action-btn--primary"
                            onClick={() => props.onAction(action.id)}
                          >
                            {action.label}
                          </button>
                        ) : (
                          <button class="building-action-btn building-action-btn--disabled" disabled>Prerequisites Not Met</button>
                        )}
                      </div>
                    </div>
                  )}
                </For>
              </div>
            )}

            {equipmentActions().length > 0 && (
              <div class="building-action-section">
                <h3 class="building-action-section-title">Equipment Upgrades</h3>
                <For each={equipmentActions()}>
                  {(action) => (
                    <div
                      class="building-action-card"
                      data-source-prefab="Assets/Prefabs/UI/UpgradeSlot.prefab"
                    >
                      <div class="building-action-card-header">
                        <span class="building-action-label">{action.label}</span>
                        {action.isUnsupported && (
                          <span class="building-action-pill building-action-pill--unsupported">Unsupported</span>
                        )}
                        {!action.isAvailable && !action.isUnsupported && (
                          <span class="building-action-pill building-action-pill--unavailable">Unavailable</span>
                        )}
                      </div>
                      <p class="building-action-desc">{action.description}</p>
                      <div class="building-action-footer">
                        <span class={`building-action-cost ${!action.isAvailable ? "building-action-cost-unavailable" : ""}`}>
                          Cost: <strong>{action.cost}</strong>
                        </span>
                        {action.isUnsupported ? (
                          <button class="building-action-btn building-action-btn--disabled" disabled>Not Available</button>
                        ) : action.isAvailable ? (
                          <button
                            class="building-action-btn building-action-btn--primary"
                            onClick={() => props.onAction(action.id)}
                          >
                            {action.label}
                          </button>
                        ) : (
                          <div style="display:flex;gap:0.5rem;align-items:center;">
                            <button class="building-action-btn building-action-btn--disabled" disabled>Prerequisites Not Met</button>
                            {vm().upgradeRequirement && (
                              <span class="building-action-pill building-action-pill--info">{vm().upgradeRequirement}</span>
                            )}
                          </div>
                        )}
                      </div>
                    </div>
                  )}
                </For>
              </div>
            )}

            {otherActions().length > 0 && (
              <div class="building-action-section">
                <h3 class="building-action-section-title">Other Services</h3>
                <For each={otherActions()}>
                  {(action) => (
                    <div class="building-action-card">
                      <div class="building-action-card-header">
                        <span class="building-action-label">{action.label}</span>
                        {action.isUnsupported && (
                          <span class="building-action-pill building-action-pill--unsupported">Unsupported</span>
                        )}
                        {!action.isAvailable && !action.isUnsupported && (
                          <span class="building-action-pill building-action-pill--unavailable">Unavailable</span>
                        )}
                      </div>
                      <p class="building-action-desc">{action.description}</p>
                      <div class="building-action-footer">
                        <span class={`building-action-cost ${!action.isAvailable ? "building-action-cost-unavailable" : ""}`}>
                          Cost: <strong>{action.cost}</strong>
                        </span>
                        {action.isUnsupported ? (
                          <button class="building-action-btn building-action-btn--disabled" disabled>Not Available</button>
                        ) : action.isAvailable ? (
                          <button
                            class="building-action-btn building-action-btn--primary"
                            onClick={() => props.onAction(action.id)}
                          >
                            {action.label}
                          </button>
                        ) : (
                          <button class="building-action-btn building-action-btn--disabled" disabled>Prerequisites Not Met</button>
                        )}
                      </div>
                    </div>
                  )}
                </For>
              </div>
            )}
          </div>
        </div>
      </Show>

      {/* ── Use Facility Tab ── */}
      <Show when={activeTab() === "use"}>
        <div class="guild-use-content" data-source-component="GuildUseFacilityPanel">
          {/* Left Panel — Hero Preview */}
          <div class="guild-use-left" data-source-hierarchy="GuildWindow/UsePanel/Left">
            <div class="guild-hero-preview">
              <Show
                when={selectedHero()}
                fallback={
                  <div class="guild-hero-preview-empty">
                    <span class="guild-hero-preview-placeholder">Select a hero</span>
                  </div>
                }
              >
                {(hero) => (
                  <>
                    <div class="guild-hero-preview-portrait">
                      {resolveHeroPortrait(hero()) ? (
                        <img
                          class="guild-hero-preview-img"
                          src={resolveHeroPortrait(hero())!}
                          alt={hero().name}
                          loading="eager"
                        />
                      ) : (
                        <div class="guild-hero-preview-fallback">
                          <span>{hero().name[0]?.toUpperCase() ?? "?"}</span>
                        </div>
                      )}
                    </div>
                    <div class="guild-hero-preview-info">
                      <span class="guild-hero-preview-name">{hero().name}</span>
                      <span class="guild-hero-preview-class">{hero().classLabel}</span>
                      <span class="guild-hero-preview-level">Lv.{hero().level}</span>
                    </div>
                  </>
                )}
              </Show>
            </div>
            <button
              class="guild-leave-btn"
              onClick={props.onReturn}
              data-source-component="LeaveButton"
            >
              离开
            </button>
          </div>

          {/* Right Panel — Training Slots */}
          <div class="guild-use-right" data-source-hierarchy="GuildWindow/UsePanel/Right">
            <div class="guild-slots-header">
              <span class="guild-slots-title">Training Slots</span>
              <span class="guild-slots-subtitle">Assign heroes to training</span>
            </div>
            <div class="guild-slots-list">
              <For each={trainingSlots()}>
                {(slot) => (
                  <div
                    class={`guild-slot-row ${selectedSlotIndex() === slot.slotIndex ? "guild-slot-row--selected" : ""} ${slot.isLocked ? "guild-slot-row--locked" : ""}`}
                    onClick={() => setSelectedSlotIndex(slot.slotIndex)}
                    data-slot-index={slot.slotIndex}
                  >
                    {/* Hero portrait / class icon */}
                    <div class="guild-slot-hero">
                      <Show
                        when={slot.heroId}
                        fallback={
                          <div class="guild-slot-hero-empty">
                            <span class="guild-slot-class-label">{slot.heroClassLabel ?? "?"}</span>
                          </div>
                        }
                      >
                        <div class="guild-slot-hero-portrait">
                          {resolveHeroPortrait(heroes().find((h) => h.id === slot.heroId)) ? (
                            <img
                              class="guild-slot-hero-img"
                              src={resolveHeroPortrait(heroes().find((h) => h.id === slot.heroId))!}
                              alt={slot.heroName ?? ""}
                              loading="lazy"
                            />
                          ) : (
                            <div class="guild-slot-hero-fallback">
                              <span>{slot.heroName?.[0]?.toUpperCase() ?? "?"}</span>
                            </div>
                          )}
                        </div>
                      </Show>
                    </div>

                    {/* Training slots */}
                    <div class="guild-slot-trainings">
                      <For each={slot.trainings}>
                        {(training) => (
                          <div
                            class={`guild-training-cell ${training.isLocked ? "guild-training-cell--locked" : training.isAvailable ? "guild-training-cell--available" : ""}`}
                            title={training.name || "Locked"}
                          >
                            {training.isLocked ? (
                              <span class="guild-training-lock">✕</span>
                            ) : training.name ? (
                              <span class="guild-training-name">{training.name}</span>
                            ) : (
                              <span class="guild-training-empty">—</span>
                            )}
                          </div>
                        )}
                      </For>
                    </div>

                    {/* Cost */}
                    <div class="guild-slot-cost">
                      <span>{slot.cost}</span>
                    </div>
                  </div>
                )}
              </For>
            </div>
          </div>
        </div>
      </Show>

      {/* ── Resource Strip ── */}
      <Show when={activeTab() === "use" && resources()}>
        <div class="guild-resource-strip" data-source-component="ResourceStrip">
          <div class="guild-resource-item">
            <span class="guild-resource-icon">💎</span>
            <span class="guild-resource-value">{resources()?.shards}</span>
          </div>
          <div class="guild-resource-item">
            <span class="guild-resource-icon">📜</span>
            <span class="guild-resource-value">{resources()?.deeds}</span>
          </div>
          <div class="guild-resource-item">
            <span class="guild-resource-icon">🏅</span>
            <span class="guild-resource-value">{resources()?.crests}</span>
          </div>
          <div class="guild-resource-item">
            <span class="guild-resource-icon">🖼️</span>
            <span class="guild-resource-value">{resources()?.portraits}</span>
          </div>
          <div class="guild-resource-item guild-resource-item--gold">
            <span class="guild-resource-icon">🪙</span>
            <span class="guild-resource-value">{resources()?.gold}</span>
          </div>
        </div>
      </Show>

      {/* ── Return to Town (shown on upgrade tab only) ── */}
      <Show when={activeTab() === "upgrade"}>
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
