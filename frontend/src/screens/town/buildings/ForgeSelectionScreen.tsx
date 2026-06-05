import { For, createSignal, type Component } from "solid-js";

import type { ForgeHeroItem } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface ForgeSelectionScreenProps {
  buildingId: string;
  label: string;
  status: "ready" | "partial" | "locked";
  description: string;
  heroes: ReadonlyArray<ForgeHeroItem>;
  mode?: "upgrade" | "rent";
  currentUpgrade?: string;
  upgradeRequirement?: string;
  onReturn: () => void;
  onSelectHero: (heroId: string) => void;
  onAction: (actionId: string) => void;
}

/**
 * Forge (锻造舱) hero selection screen.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Blacksmith/BlacksmithWindow.prefab
 *
 * Reference image: 公会界面-锻造仓-选择.png
 * Layout:
 *   - Left panel: selected hero portrait + info
 *   - Center-top: mode tabs (升级设施 / 租用设施)
 *   - Center: hero selection list with checkboxes, names, levels, equipment slots
 *   - Close button (top-right)
 *   - Return to town button (bottom)
 */
export const ForgeSelectionScreen: Component<ForgeSelectionScreenProps> = (
  props
) => {
  const [selectedHeroId, setSelectedHeroId] = createSignal(
    props.heroes.find((h) => h.isSelected)?.id ?? props.heroes[0]?.id ?? ""
  );
  const [activeMode, setActiveMode] = createSignal(props.mode ?? "upgrade");

  const selectedHero = () =>
    props.heroes.find((h) => h.id === selectedHeroId());

  const handleSelectHero = (heroId: string) => {
    setSelectedHeroId(heroId);
    props.onSelectHero(heroId);
  };

  return (
    <div class="app-frame forge-selection-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Building Header ── */}
      <BuildingDetailHeader
        buildingId={props.buildingId}
        label={props.label}
        status={props.status}
        description={props.description}
        sourcePrefabPath="Assets/Prefabs/UI/Estate/Buildings/Blacksmith/BlacksmithWindow.prefab"
        sourceSpritePath="Assets/Sprites/town/buildings/building_forging.png"
        sourceGuid="23e01c10f262ddc4ba9977b91314b031"
      />

      {/* ── Section Title ── */}
      <div class="forge-section-header">
        <h2 class="forge-section-title">选择人物</h2>
        <span class="forge-section-subtitle">选择英雄进行装备锻造</span>
      </div>

      {/* ── Mode Tabs ── */}
      <div class="forge-mode-tabs">
        <button
          class={`forge-mode-tab ${activeMode() === "upgrade" ? "forge-mode-tab--active" : ""}`}
          onClick={() => setActiveMode("upgrade")}
          data-testid="tab-upgrade"
        >
          <span class="forge-mode-check">{activeMode() === "upgrade" ? "☑" : "☐"}</span>
          升级设施
        </button>
        <button
          class={`forge-mode-tab ${activeMode() === "rent" ? "forge-mode-tab--active" : ""}`}
          onClick={() => setActiveMode("rent")}
          data-testid="tab-rent"
        >
          <span class="forge-mode-check">{activeMode() === "rent" ? "☑" : "☐"}</span>
          租用设施
        </button>
      </div>

      {/* ── Main Content: Left portrait + Right hero list ── */}
      <div class="forge-selection-content">
        {/* Left Panel — Selected Hero Portrait */}
        <div class="forge-selection-left" data-source-hierarchy="BlacksmithWindow/LeftPanel">
          {selectedHero() && (
            <div class="forge-selected-hero">
              <div class="forge-hero-portrait-frame">
                <div class="forge-hero-portrait">
                  <span class="forge-hero-portrait-initial">
                    {selectedHero()!.name.charAt(0)}
                  </span>
                </div>
              </div>
              <div class="forge-selected-hero-info">
                <h3 class="forge-selected-hero-name">{selectedHero()!.name}</h3>
                <span class="forge-selected-hero-class">{selectedHero()!.classLabel}</span>
                <div class="forge-selected-hero-level">
                  <span class="forge-level-label">等级</span>
                  <span class="forge-level-stars">
                    {Array.from({ length: selectedHero()!.level }, (_, i) => (
                      <span key={i} class="forge-level-star">★</span>
                    ))}
                  </span>
                </div>
                <div class="forge-selected-hero-stats">
                  <div class="forge-stat-row">
                    <span class="forge-stat-label">生命</span>
                    <span class="forge-stat-value">{selectedHero()!.hp}</span>
                  </div>
                  <div class="forge-stat-row">
                    <span class="forge-stat-label">压力</span>
                    <span class="forge-stat-value">{selectedHero()!.stress}</span>
                  </div>
                </div>
                {(selectedHero()!.isWounded || selectedHero()!.isAfflicted) && (
                  <div class="forge-hero-status-tags">
                    {selectedHero()!.isWounded && (
                      <span class="forge-hero-status-tag forge-hero-status-tag--wounded">受伤</span>
                    )}
                    {selectedHero()!.isAfflicted && (
                      <span class="forge-hero-status-tag forge-hero-status-tag--afflicted"> affliction</span>
                    )}
                  </div>
                )}
              </div>
            </div>
          )}
        </div>

        {/* Right Panel — Hero Selection List */}
        <div class="forge-selection-right" data-source-hierarchy="BlacksmithWindow/RightPanel">
          <div class="forge-hero-list">
            <div class="forge-hero-list-header">
              <span class="forge-list-col forge-list-col-check"></span>
              <span class="forge-list-col forge-list-col-name">英雄</span>
              <span class="forge-list-col forge-list-col-level">等级</span>
              <span class="forge-list-col forge-list-col-equip">装备</span>
            </div>
            <For each={props.heroes}>
              {(hero) => (
                <div
                  class={`forge-hero-row ${selectedHeroId() === hero.id ? "forge-hero-row--selected" : ""}`}
                  onClick={() => handleSelectHero(hero.id)}
                  data-hero-id={hero.id}
                  data-testid={`forge-hero-row-${hero.id}`}
                >
                  <span class="forge-list-col forge-list-col-check">
                    <span class="forge-hero-checkbox">
                      {selectedHeroId() === hero.id ? "☑" : "☐"}
                    </span>
                  </span>
                  <span class="forge-list-col forge-list-col-name">
                    <span class="forge-hero-row-name">{hero.name}</span>
                    <span class="forge-hero-row-class">{hero.classLabel}</span>
                  </span>
                  <span class="forge-list-col forge-list-col-level">
                    <span class="forge-hero-row-stars">
                      {Array.from({ length: hero.level }, (_, i) => (
                        <span key={i} class="forge-hero-row-star">★</span>
                      ))}
                    </span>
                  </span>
                  <span class="forge-list-col forge-list-col-equip">
                    <div class="forge-equip-slots">
                      <div class="forge-equip-slot" title={hero.weaponName}>
                        <span class="forge-equip-slot-icon">⚔</span>
                        <span class="forge-equip-slot-level">{hero.weaponLevel}</span>
                      </div>
                      <div class="forge-equip-slot" title={hero.armorName}>
                        <span class="forge-equip-slot-icon">🛡</span>
                        <span class="forge-equip-slot-level">{hero.armorLevel}</span>
                      </div>
                    </div>
                  </span>
                </div>
              )}
            </For>
          </div>
        </div>
      </div>

      {/* ── Building Status Card (if upgrade info present) ── */}
      {(props.currentUpgrade || props.upgradeRequirement) && (
        <div class="forge-status-bar">
          {props.currentUpgrade && (
            <span class="forge-status-pill">
              当前: {props.currentUpgrade}
            </span>
          )}
          {props.upgradeRequirement && (
            <span class="forge-status-pill forge-status-pill--requirement">
              需求: {props.upgradeRequirement}
            </span>
          )}
        </div>
      )}

      {/* ── Action buttons ── */}
      <div class="forge-action-row">
        <button
          class="forge-action-btn forge-action-btn--primary"
          onClick={() => props.onAction("confirm-forge-selection")}
          disabled={!selectedHeroId()}
        >
          {activeMode() === "upgrade" ? "升级装备" : "租用设施"}
        </button>
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
