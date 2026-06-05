import { For, createSignal, type Component } from "solid-js";

import type { GuildUpgradeViewModel } from "../../../bridge/contractTypes";
import { resolveBuildingImage } from "../../../assets/originalAssetPaths";

interface GuildTrialUpgradeScreenProps {
  viewModel: GuildUpgradeViewModel;
  onReturnToTown: () => void;
  onReturnToGuild: () => void;
  onUpgradeAction: (rowId: string) => void;
}

/**
 * Guild Trial Ground Upgrade screen (公会界面-试炼场-升级).
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Guild/GuildUpgradeWindow.prefab
 *
 * Reference image: 公会界面-试炼场-升级.png
 * Layout:
 *   LeftPanel: NPC portrait + building icon + Talk/Leave buttons
 *   RightPanel: Tab bar (升级设施 / 使用设施) + upgrade tree + completion % + resources
 */
export const GuildTrialUpgradeScreen: Component<GuildTrialUpgradeScreenProps> = (
  props
) => {
  const vm = () => props.viewModel;
  const spriteSrc = () => resolveBuildingImage("guild");
  const [activeTab, setActiveTab] = createSignal(vm().activeTab);

  return (
    <div class="app-frame guild-upgrade-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ═══ Guild Upgrade Layout ═══ */}
      <div class="guild-upgrade-layout">
        {/* ── Left Panel ── */}
        <div
          class="guild-upgrade-left"
          data-source-hierarchy="GuildUpgradeWindow/LeftPanel"
        >
          {/* Building icon + title */}
          <div class="guild-upgrade-building-header">
            <div
              class="guild-upgrade-building-icon"
              data-source-component="BuildingIcon"
              data-source-sprite="Assets/Sprites/town/buildings/building_train_field.png"
            >
              {spriteSrc() ? (
                <img
                  class="guild-upgrade-building-icon-img"
                  src={spriteSrc()}
                  alt={vm().label}
                  loading="eager"
                />
              ) : (
                <div class="guild-upgrade-building-icon-fallback">
                  <span>{vm().label[0]?.toUpperCase() ?? "?"}</span>
                </div>
              )}
            </div>
            <div class="guild-upgrade-building-title-area">
              <span class="guild-upgrade-building-eyebrow">Building</span>
              <h2 class="guild-upgrade-building-name">{vm().label}</h2>
              <p class="guild-upgrade-building-desc">{vm().description}</p>
            </div>
          </div>

          {/* NPC Portrait area */}
          <div
            class="guild-upgrade-npc-area"
            data-source-component="NpcPortrait"
          >
            <div class="guild-upgrade-npc-portrait">
              <span class="guild-upgrade-npc-initial">
                {vm().npcName[0]?.toUpperCase() ?? "?"}
              </span>
            </div>
            <div class="guild-upgrade-npc-info">
              <span class="guild-upgrade-npc-name">{vm().npcName}</span>
              <span class="guild-upgrade-npc-title">{vm().npcTitle}</span>
            </div>
          </div>

          {/* Action buttons */}
          <div class="guild-upgrade-left-actions">
            <button
              class="guild-upgrade-talk-btn"
              data-source-component="TalkButton"
              onClick={() => props.onUpgradeAction("talk")}
            >
              对话
            </button>
            <button
              class="guild-upgrade-leave-btn"
              data-source-component="LeaveButton"
              onClick={props.onReturnToTown}
            >
              离开
            </button>
          </div>
        </div>

        {/* ── Right Panel ── */}
        <div
          class="guild-upgrade-right"
          data-source-hierarchy="GuildUpgradeWindow/RightPanel"
        >
          {/* Tab bar */}
          <div class="guild-upgrade-tabs">
            <button
              class={`guild-upgrade-tab ${activeTab() === "upgrade" ? "guild-upgrade-tab--active" : ""}`}
              data-tab="upgrade"
              onClick={() => setActiveTab("upgrade")}
            >
              <span class="guild-upgrade-tab-check">{activeTab() === "upgrade" ? "☑" : "☐"}</span>
              升级设施
            </button>
            <button
              class={`guild-upgrade-tab ${activeTab() === "use" ? "guild-upgrade-tab--active" : ""}`}
              data-tab="use"
              onClick={() => setActiveTab("use")}
            >
              <span class="guild-upgrade-tab-check">{activeTab() === "use" ? "☑" : "☐"}</span>
              使用设施
            </button>
          </div>

          {/* Upgrade tree content */}
          <div class="guild-upgrade-content">
            {activeTab() === "upgrade" ? (
              <div class="guild-upgrade-tree">
                <For each={vm().upgradeRows}>
                  {(row) => (
                    <div
                      class="guild-upgrade-row"
                      data-upgrade-row-id={row.id}
                    >
                      <div class="guild-upgrade-row-header">
                        <span class="guild-upgrade-row-icon" aria-hidden="true">
                          <span class="guild-upgrade-row-icon-inner">⚔</span>
                        </span>
                        <span class="guild-upgrade-row-label">{row.label}</span>
                      </div>
                      <div class="guild-upgrade-row-levels">
                        <For each={Array.from({ length: row.maxLevel }, (_, i) => i + 1)}>
                          {(level) => (
                            <button
                              class={`guild-upgrade-level-box ${level <= row.currentLevel ? "guild-upgrade-level-box--unlocked" : ""} ${level === row.currentLevel + 1 && row.isAvailable ? "guild-upgrade-level-box--next" : ""}`}
                              disabled={level > row.currentLevel + 1 || !row.isAvailable}
                              onClick={() =>
                                level === row.currentLevel + 1 && row.isAvailable
                                  ? props.onUpgradeAction(row.id)
                                  : undefined
                              }
                              title={
                                level <= row.currentLevel
                                  ? `${row.label} - Level ${level} (Unlocked)`
                                  : level === row.currentLevel + 1 && row.isAvailable
                                    ? `${row.label} - Level ${level} (Cost: ${row.cost})`
                                    : `${row.label} - Level ${level} (Locked)`
                              }
                            >
                              {level <= row.currentLevel ? "✓" : level === row.currentLevel + 1 && row.isAvailable ? "+" : ""}
                            </button>
                          )}
                        </For>
                      </div>
                      {row.benefits.length > 0 && (
                        <div class="guild-upgrade-row-benefits">
                          <For each={row.benefits}>
                            {(benefit) => (
                              <span class="guild-upgrade-benefit-tag">{benefit}</span>
                            )}
                          </For>
                        </div>
                      )}
                    </div>
                  )}
                </For>
              </div>
            ) : (
              <div class="guild-upgrade-use-panel">
                <div class="guild-upgrade-use-placeholder">
                  <p>设施使用功能将在后续版本开放。</p>
                  <p class="guild-upgrade-use-hint">当前可在此训练英雄技能与升级装备。</p>
                </div>
              </div>
            )}
          </div>

          {/* Footer: completion + resources */}
          <div class="guild-upgrade-footer">
            <div class="guild-upgrade-completion">
              <span class="guild-upgrade-completion-label">完成度</span>
              <div class="guild-upgrade-completion-bar">
                <div
                  class="guild-upgrade-completion-fill"
                  style={{ width: `${vm().completionPercent}%` }}
                />
              </div>
              <span class="guild-upgrade-completion-value">{vm().completionPercent}%</span>
            </div>

            <div class="guild-upgrade-resources">
              <span class="guild-upgrade-resource-slot" data-resource="bust">
                <span class="guild-upgrade-resource-icon guild-upgrade-resource-icon--bust" />
                <span class="guild-upgrade-resource-value">{vm().resources.bust}</span>
              </span>
              <span class="guild-upgrade-resource-slot" data-resource="portrait">
                <span class="guild-upgrade-resource-icon guild-upgrade-resource-icon--portrait" />
                <span class="guild-upgrade-resource-value">{vm().resources.portrait}</span>
              </span>
              <span class="guild-upgrade-resource-slot" data-resource="deed">
                <span class="guild-upgrade-resource-icon guild-upgrade-resource-icon--deed" />
                <span class="guild-upgrade-resource-value">{vm().resources.deed}</span>
              </span>
              <span class="guild-upgrade-resource-slot" data-resource="crest">
                <span class="guild-upgrade-resource-icon guild-upgrade-resource-icon--crest" />
                <span class="guild-upgrade-resource-value">{vm().resources.crest}</span>
              </span>
              <span class="guild-upgrade-resource-slot guild-upgrade-resource-slot--gold" data-resource="gold">
                <span class="guild-upgrade-resource-icon guild-upgrade-resource-icon--gold" />
                <span class="guild-upgrade-resource-value">{vm().resources.gold}</span>
              </span>
            </div>
          </div>
        </div>
      </div>

      {/* ── Return to Town (secondary) ── */}
      <div class="building-return-row">
        <button
          class="building-return-btn"
          onClick={props.onReturnToTown}
          data-source-component="CloseButton"
          data-source-sprite="Assets/Sprites/ui/btn_close.png"
        >
          Return to Town
        </button>
      </div>
    </div>
  );
};
