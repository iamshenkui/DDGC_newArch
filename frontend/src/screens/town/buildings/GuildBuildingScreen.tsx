import { For, Show, createSignal, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { resolveBuildingImage } from "../../../assets/originalAssetPaths";

interface GuildBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

type GuildTab = "upgrades" | "recruit";

/**
 * Guild (次元感知塔) building screen — Dimensional Perception Tower Upgrade.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Guild/GuildWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_train_field.png
 * GUID: 67a5e7aed8029d84dbf9c9e497a944d2
 *
 * Reference image: 公会界面-次元感知塔-升级.png
 *   - Left panel: building illustration, Talk button, Leave button
 *   - Right panel: two tabs (升级设施 / 招募人员)
 *   - Upgrade trees: 觉醒共鸣, 强力感知, 休息室
 *   - Bottom: currency strip (busts, portraits, deeds, crests, gold)
 *
 * Building data (data/Buildings.json):
 *   guild_training   — experience boost tree (觉醒共鸣)
 *   guild_skills     — skill upgrade chance tree (强力感知)
 *   guild_capacity   — training slots tree (休息室)
 */
export const GuildBuildingScreen: Component<GuildBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;
  const [activeTab, setActiveTab] = createSignal<GuildTab>("upgrades");

  const spriteSrc = () => resolveBuildingImage("guild");

  const upgradeTrees = () => vm().upgradeTrees ?? [];
  const resources = () => vm().resources;

  const talkAction = () => vm().actions[0];

  return (
    <div class="app-frame guild-building-screen" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Main Content: Left + Right ── */}
      <div class="guild-building-content">
        {/* Left Panel — building illustration + action buttons */}
        <div
          class="guild-building-left"
          data-source-hierarchy="GuildWindow/LeftPanel"
        >
          {/* Building illustration */}
          <div
            class="guild-building-art"
            data-source-component="BuildingIcon"
            data-source-sprite="Assets/Sprites/town/buildings/building_train_field.png"
            data-source-guid="67a5e7aed8029d84dbf9c9e497a944d2"
          >
            {spriteSrc() ? (
              <img
                class="guild-building-art-img"
                src={spriteSrc()}
                alt={vm().label}
                loading="eager"
              />
            ) : (
              <div class="guild-building-art-fallback">
                <span>{vm().label[0]?.toUpperCase() ?? "?"}</span>
              </div>
            )}
          </div>

          {/* Building name below illustration */}
          <h2 class="guild-building-name" data-source-component="BuildingLabel">
            {vm().label}
          </h2>

          {/* Action buttons */}
          <div class="guild-building-actions">
            <Show when={talkAction()}>
              {(action) => (
                <button
                  class="guild-building-btn guild-building-btn--talk"
                  onClick={() => props.onAction(action().id)}
                  data-source-component="TalkButton"
                  disabled={!action().isAvailable}
                >
                  对话
                </button>
              )}
            </Show>
            <button
              class="guild-building-btn guild-building-btn--leave"
              onClick={props.onReturn}
              data-source-component="CloseButton"
              data-source-sprite="Assets/Sprites/ui/btn_close.png"
            >
              离开
            </button>
          </div>
        </div>

        {/* Right Panel — tabs + upgrade trees */}
        <div
          class="guild-building-right"
          data-source-hierarchy="GuildWindow/RightPanel"
        >
          {/* Tab bar */}
          <div class="guild-tab-bar" role="tablist" aria-label="公会功能">
            <button
              class={`guild-tab-btn ${activeTab() === "upgrades" ? "guild-tab-btn--active" : ""}`}
              role="tab"
              aria-selected={activeTab() === "upgrades"}
              onClick={() => setActiveTab("upgrades")}
              data-tab-id="upgrades"
            >
              <span class="guild-tab-check">{activeTab() === "upgrades" ? "☑" : "☐"}</span>
              升级设施
            </button>
            <button
              class={`guild-tab-btn ${activeTab() === "recruit" ? "guild-tab-btn--active" : ""}`}
              role="tab"
              aria-selected={activeTab() === "recruit"}
              onClick={() => setActiveTab("recruit")}
              data-tab-id="recruit"
            >
              <span class="guild-tab-check">{activeTab() === "recruit" ? "☑" : "☐"}</span>
              招募人员
            </button>
          </div>

          {/* Tab content */}
          <div class="guild-tab-content" role="tabpanel">
            <Show when={activeTab() === "upgrades"}>
              <div class="guild-upgrade-panel">
                <For each={upgradeTrees()}>
                  {(tree) => (
                    <div
                      class="guild-upgrade-tree"
                      data-tree-id={tree.treeId}
                      data-source-prefab="Assets/Prefabs/UI/UpgradeSlot.prefab"
                    >
                      <div class="guild-upgrade-tree-header">
                        <span class="guild-upgrade-tree-icon" aria-hidden="true">
                          {tree.icon ? (
                            <img src={tree.icon} alt="" />
                          ) : (
                            <span class="guild-upgrade-tree-icon-fallback">◆</span>
                          )}
                        </span>
                        <span class="guild-upgrade-tree-label">{tree.label}</span>
                      </div>
                      <div class="guild-upgrade-levels">
                        <For each={tree.levels}>
                          {(level, index) => (
                            <button
                              class={`guild-upgrade-slot ${level.isPurchased ? "guild-upgrade-slot--purchased" : ""} ${!level.isAvailable && !level.isPurchased ? "guild-upgrade-slot--locked" : ""}`}
                              onClick={() => {
                                if (level.isAvailable && !level.isPurchased) {
                                  props.onAction(`${tree.treeId}-${level.code}`);
                                }
                              }}
                              disabled={!level.isAvailable || level.isPurchased}
                              title={`${tree.label} ${level.code.toUpperCase()} — ${level.effectSummary}${level.cost > 0 ? ` (${level.cost} Gold)` : ""}`}
                              data-level-code={level.code}
                              data-purchased={level.isPurchased}
                              data-available={level.isAvailable}
                            >
                              <span class="guild-upgrade-slot-check">
                                {level.isPurchased ? "☑" : "☐"}
                              </span>
                              <span class="guild-upgrade-slot-code">{level.code.toUpperCase()}</span>
                            </button>
                          )}
                        </For>
                      </div>
                    </div>
                  )}
                </For>

                {/* Fallback when no upgrade trees are provided */}
                <Show when={upgradeTrees().length === 0}>
                  <div class="guild-upgrade-tree">
                    <div class="guild-upgrade-tree-header">
                      <span class="guild-upgrade-tree-icon-fallback">◆</span>
                      <span class="guild-upgrade-tree-label">觉醒共鸣</span>
                    </div>
                    <div class="guild-upgrade-levels">
                      <button class="guild-upgrade-slot guild-upgrade-slot--purchased" disabled data-level-code="a" data-purchased={true}>
                        <span class="guild-upgrade-slot-check">☑</span>
                        <span class="guild-upgrade-slot-code">A</span>
                      </button>
                      <button class="guild-upgrade-slot" disabled data-level-code="b" data-purchased={false}>
                        <span class="guild-upgrade-slot-check">☐</span>
                        <span class="guild-upgrade-slot-code">B</span>
                      </button>
                      <button class="guild-upgrade-slot guild-upgrade-slot--locked" disabled data-level-code="c" data-purchased={false}>
                        <span class="guild-upgrade-slot-check">☐</span>
                        <span class="guild-upgrade-slot-code">C</span>
                      </button>
                      <button class="guild-upgrade-slot guild-upgrade-slot--locked" disabled data-level-code="d" data-purchased={false}>
                        <span class="guild-upgrade-slot-check">☐</span>
                        <span class="guild-upgrade-slot-code">D</span>
                      </button>
                    </div>
                  </div>
                  <div class="guild-upgrade-tree">
                    <div class="guild-upgrade-tree-header">
                      <span class="guild-upgrade-tree-icon-fallback">◆</span>
                      <span class="guild-upgrade-tree-label">强力感知</span>
                    </div>
                    <div class="guild-upgrade-levels">
                      <button class="guild-upgrade-slot guild-upgrade-slot--purchased" disabled data-level-code="a" data-purchased={true}>
                        <span class="guild-upgrade-slot-check">☑</span>
                        <span class="guild-upgrade-slot-code">A</span>
                      </button>
                      <button class="guild-upgrade-slot" disabled data-level-code="b" data-purchased={false}>
                        <span class="guild-upgrade-slot-check">☐</span>
                        <span class="guild-upgrade-slot-code">B</span>
                      </button>
                      <button class="guild-upgrade-slot guild-upgrade-slot--locked" disabled data-level-code="c" data-purchased={false}>
                        <span class="guild-upgrade-slot-check">☐</span>
                        <span class="guild-upgrade-slot-code">C</span>
                      </button>
                    </div>
                  </div>
                  <div class="guild-upgrade-tree">
                    <div class="guild-upgrade-tree-header">
                      <span class="guild-upgrade-tree-icon-fallback">◆</span>
                      <span class="guild-upgrade-tree-label">休息室</span>
                    </div>
                    <div class="guild-upgrade-levels">
                      <button class="guild-upgrade-slot guild-upgrade-slot--purchased" disabled data-level-code="a" data-purchased={true}>
                        <span class="guild-upgrade-slot-check">☑</span>
                        <span class="guild-upgrade-slot-code">A</span>
                      </button>
                      <button class="guild-upgrade-slot" disabled data-level-code="b" data-purchased={false}>
                        <span class="guild-upgrade-slot-check">☐</span>
                        <span class="guild-upgrade-slot-code">B</span>
                      </button>
                      <button class="guild-upgrade-slot guild-upgrade-slot--locked" disabled data-level-code="c" data-purchased={false}>
                        <span class="guild-upgrade-slot-check">☐</span>
                        <span class="guild-upgrade-slot-code">C</span>
                      </button>
                    </div>
                  </div>
                </Show>
              </div>
            </Show>

            <Show when={activeTab() === "recruit"}>
              <div class="guild-recruit-panel">
                <p class="guild-recruit-hint">
                  招募人员功能将在后续版本中开放。
                </p>
                <div class="guild-recruit-placeholder">
                  <span class="guild-recruit-placeholder-icon">👥</span>
                  <span class="guild-recruit-placeholder-text">英雄招募列表</span>
                </div>
              </div>
            </Show>
          </div>
        </div>
      </div>

      {/* ── Bottom Currency Strip ── */}
      <Show when={resources()}>
        {(res) => (
          <div class="guild-currency-strip" data-source-prefab="UI_Shared/UI_TopWindows/CurrencyPanel">
            <div class="guild-currency-slot" data-currency="busts">
              <span class="guild-currency-icon guild-currency-icon--bust" aria-hidden="true" />
              <span class="guild-currency-value">{res().busts}</span>
            </div>
            <div class="guild-currency-slot" data-currency="portraits">
              <span class="guild-currency-icon guild-currency-icon--portrait" aria-hidden="true" />
              <span class="guild-currency-value">{res().portraits}</span>
            </div>
            <div class="guild-currency-slot" data-currency="deeds">
              <span class="guild-currency-icon guild-currency-icon--deed" aria-hidden="true" />
              <span class="guild-currency-value">{res().deeds}</span>
            </div>
            <div class="guild-currency-slot" data-currency="crests">
              <span class="guild-currency-icon guild-currency-icon--crest" aria-hidden="true" />
              <span class="guild-currency-value">{res().crests}</span>
            </div>
            <div class="guild-currency-slot guild-currency-slot--gold" data-currency="gold">
              <span class="guild-currency-icon guild-currency-icon--gold" aria-hidden="true" />
              <span class="guild-currency-value">{res().gold}</span>
            </div>
          </div>
        )}
      </Show>
    </div>
  );
};
