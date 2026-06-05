import { For, Show, createSignal, type Component } from "solid-js";

import type { BuildingDetailViewModel, TownHeroSummary } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";
import { resolveHeroPortrait } from "../../../assets/originalAssetPaths";

interface GardenBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

/** Resolve portrait path for a hero; falls back to undefined. */
function heroPortrait(hero: TownHeroSummary): string | undefined {
  return resolveHeroPortrait({ heroId: hero.id, classLabel: hero.classLabel });
}

/** Parse "38 / 42" style HP into current and max. */
function parseHp(hp: string): { current: number; max: number } {
  const parts = hp.split("/").map((s) => parseInt(s.trim(), 10));
  return { current: parts[0] ?? 0, max: parts[1] ?? 0 };
}

function hpPercent(hero: TownHeroSummary): number {
  const { current, max } = parseHp(hero.hp);
  return max > 0 ? Math.round((current / max) * 100) : 0;
}

function stressPercent(hero: TownHeroSummary): number {
  const current = parseInt(hero.stress, 10) ?? 0;
  const max = parseInt(hero.maxStress, 10) ?? 200;
  return max > 0 ? Math.round((current / max) * 100) : 0;
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
 * Layout (reference frame "公会界面-天国花园-选择.png"):
 *   - Tabs: 升级设施 | 使用设施
 *   - Use Facilities tab:
 *       LeftPanel  → selected hero portrait + info + talk/leave buttons
 *       CenterPanel → "选择人物" hero roster list
 *       RightPanel → activity cards grid
 */
export const GardenBuildingScreen: Component<GardenBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;
  const [activeTab, setActiveTab] = createSignal<"upgrade" | "use">("use");
  const [selectedHeroId, setSelectedHeroId] = createSignal<string | null>(
    vm().heroes?.[0]?.id ?? null
  );

  const selectedHero = () => vm().heroes?.find((h) => h.id === selectedHeroId()) ?? null;

  const upgradeActions = () => vm().actions.filter((a) => a.id.startsWith("upgrade-"));
  const useActions = () => vm().actions.filter((a) => !a.id.startsWith("upgrade-"));

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

      {/* ── Tabs — 升级设施 / 使用设施 ── */}
      <div class="garden-tab-bar" data-source-hierarchy="GardenWindow/TabBar">
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

      {/* ── Upgrade Facilities Tab ── */}
      <Show when={activeTab() === "upgrade"}>
        <div class="building-detail-content">
          <div class="building-detail-left">
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
          </div>
          <div class="building-detail-right">
            {upgradeActions().length === 0 ? (
              <div class="building-info-card">
                <p style="margin:0;color:rgba(218,198,168,0.5);font-size:0.82rem;">
                  No upgrades currently available.
                </p>
              </div>
            ) : (
              <div class="building-action-section">
                <h3 class="building-action-section-title">Upgrades</h3>
                <For each={upgradeActions()}>
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
                          <button class="building-action-btn building-action-btn--primary" onClick={() => props.onAction(action.id)}>
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

      {/* ── Use Facilities Tab ── */}
      <Show when={activeTab() === "use"}>
        <div class="garden-use-content">
          {/* Left Panel — Selected Hero Portrait */}
          <div class="garden-left-panel" data-source-hierarchy="GardenWindow/LeftPanel">
            <Show when={selectedHero()} fallback={
              <div class="garden-hero-empty">
                <span class="garden-hero-empty-text">请选择人物</span>
              </div>
            }>
              {(hero) => {
                const portrait = heroPortrait(hero());
                const hpPct = hpPercent(hero());
                const stressPct = stressPercent(hero());
                return (
                  <>
                    <div class="garden-hero-portrait-large">
                      {portrait ? (
                        <img src={portrait} alt={hero().name} class="garden-hero-portrait-img" />
                      ) : (
                        <div class="garden-hero-portrait-fallback">
                          <span>{hero().name[0]?.toUpperCase() ?? "?"}</span>
                        </div>
                      )}
                    </div>
                    <div class="garden-hero-info">
                      <h3 class="garden-hero-name">{hero().name}</h3>
                      <span class="garden-hero-class">{hero().classLabel} · Lv.{hero().level}</span>
                      <div class="garden-hero-bars">
                        <div class="bar-row">
                          <span class="bar-label">HP</span>
                          <div class="bar-container">
                            <div
                              class={`bar-fill ${hpPct < 30 ? "bar-fill-health-danger" : hpPct < 60 ? "bar-fill-health-warning" : "bar-fill-health"}`}
                              style={{ width: `${hpPct}%` }}
                            />
                          </div>
                          <span class="bar-value">{hero().hp}</span>
                        </div>
                        <div class="bar-row">
                          <span class="bar-label">Stress</span>
                          <div class="bar-container">
                            <div
                              class={`bar-fill ${stressPct > 75 ? "bar-fill-stress-high" : "bar-fill-stress"}`}
                              style={{ width: `${stressPct}%` }}
                            />
                          </div>
                          <span class="bar-value">{hero().stress}</span>
                        </div>
                      </div>
                    </div>
                    <div class="garden-hero-actions">
                      <button class="garden-hero-btn garden-hero-btn--talk" onClick={() => props.onAction(`talk-${hero().id}`)}>
                        对话
                      </button>
                      <button class="garden-hero-btn garden-hero-btn--leave" onClick={() => setSelectedHeroId(null)}>
                        离开
                      </button>
                    </div>
                  </>
                );
              }}
            </Show>
          </div>

          {/* Center Panel — Hero Selection List */}
          <div class="garden-center-panel" data-source-hierarchy="GardenWindow/CenterPanel">
            <h3 class="garden-panel-title">
              选择人物
              <span class="garden-panel-count">{vm().heroes?.length ?? 0}</span>
            </h3>
            <div class="garden-hero-list">
              <For each={vm().heroes ?? []}>
                {(hero) => {
                  const isSelected = () => selectedHeroId() === hero.id;
                  const portrait = heroPortrait(hero);
                  return (
                    <button
                      class={`garden-hero-list-item ${isSelected() ? "garden-hero-list-item--selected" : ""}`}
                      onClick={() => setSelectedHeroId(hero.id)}
                      data-hero-id={hero.id}
                    >
                      <div class="garden-hero-list-portrait">
                        {portrait ? (
                          <img src={portrait} alt={hero.name} class="garden-hero-list-portrait-img" />
                        ) : (
                          <span class="garden-hero-list-portrait-letter">{hero.name[0]?.toUpperCase() ?? "?"}</span>
                        )}
                      </div>
                      <div class="garden-hero-list-meta">
                        <span class="garden-hero-list-name">{hero.name}</span>
                        <span class="garden-hero-list-class">{hero.classLabel} Lv.{hero.level}</span>
                        <span class="garden-hero-list-hp">{hero.hp}</span>
                      </div>
                    </button>
                  );
                }}
              </For>
            </div>
          </div>

          {/* Right Panel — Activity Grid */}
          <div class="garden-right-panel" data-source-hierarchy="GardenWindow/RightPanel">
            <h3 class="garden-panel-title">花园服务</h3>
            <div class="garden-activity-grid">
              <For each={useActions()}>
                {(action) => (
                  <div class={`garden-activity-card ${!action.isAvailable ? "garden-activity-card--unavailable" : ""}`}>
                    <div class="garden-activity-card-header">
                      <span class="garden-activity-label">{action.label}</span>
                      {!action.isAvailable && (
                        <span class="building-action-pill building-action-pill--unavailable">Unavailable</span>
                      )}
                    </div>
                    <p class="garden-activity-desc">{action.description}</p>
                    <div class="garden-activity-footer">
                      <span class={`building-action-cost ${!action.isAvailable ? "building-action-cost-unavailable" : ""}`}>
                        Cost: <strong>{action.cost}</strong>
                      </span>
                      {action.isAvailable ? (
                        <button
                          class="building-action-btn building-action-btn--primary"
                          onClick={() => props.onAction(action.id)}
                          disabled={!selectedHeroId()}
                        >
                          选择
                        </button>
                      ) : (
                        <button class="building-action-btn building-action-btn--disabled" disabled>
                          未解锁
                        </button>
                      )}
                    </div>
                  </div>
                )}
              </For>
            </div>
          </div>
        </div>
      </Show>

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
