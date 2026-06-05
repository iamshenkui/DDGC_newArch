import { For, type Component } from "solid-js";

import type { DungeonContentViewModel } from "../../bridge/contractTypes";

interface DungeonContentScreenProps {
  viewModel: DungeonContentViewModel;
  onEnterDungeonMap: () => void;
  onReturnToTown: () => void;
}

/**
 * Dungeon content screen — 位面探索-副本内容
 *
 * Displays dungeon briefing with objectives, rewards, and stage list.
 * Mirroring the original Unity book-like layout from the reference image:
 *   Left page: dungeon info, objectives, rewards, enter button
 *   Right page: numbered stage list with status indicators
 *
 * Source scene: UI_Expedition/DungeonContentWindow (estimated)
 */
export const DungeonContentScreen: Component<DungeonContentScreenProps> = (props) => {
  const difficultyStars = () => {
    const level = props.viewModel.difficultyLevel;
    return Array.from({ length: 5 }, (_, i) => i < level);
  };

  return (
    <div
      class="expedition-viewport"
      data-source-scene="UI_Expedition/DungeonContentWindow"
      data-testid="dungeon-content-screen"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">位面探索</span>
          <h1 class="expedition-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class="hud-pill hud-pill-accent">
            {props.viewModel.dungeonName}
          </span>
          <span class="hud-pill">
            难度: {props.viewModel.difficulty}
          </span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="expedition-content dungeon-content-layout">
          {/* Book-like container with left and right pages */}
          <div class="dungeon-book">
            {/* ── Left Page: Dungeon Info ───────────────── */}
            <div class="dungeon-page dungeon-page--left">
              {/* Dungeon header with icon */}
              <div class="dungeon-header">
                <div class="dungeon-icon" aria-hidden="true">
                  <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <path d="M12 2L2 7l10 5 10-5-10-5z" />
                    <path d="M2 17l10 5 10-5" />
                    <path d="M2 12l10 5 10-5" />
                  </svg>
                </div>
                <div class="dungeon-header-text">
                  <h2 class="dungeon-name">{props.viewModel.dungeonName}</h2>
                  <p class="dungeon-subtitle">{props.viewModel.dungeonSubtitle}</p>
                  <div class="dungeon-difficulty">
                    <span class="difficulty-label">难度</span>
                    <span class="difficulty-stars">
                      <For each={difficultyStars()}>
                        {(filled) => (
                          <span class={`difficulty-star${filled ? " difficulty-star--filled" : ""}`}>
                            <svg width="14" height="14" viewBox="0 0 24 24" fill={filled ? "currentColor" : "none"} stroke="currentColor" stroke-width="1.5">
                              <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />
                            </svg>
                          </span>
                        )}
                      </For>
                    </span>
                  </div>
                </div>
              </div>

              {/* Divider */}
              <div class="dungeon-divider" aria-hidden="true" />

              {/* Objectives */}
              <section class="dungeon-section">
                <h3 class="dungeon-section-title">
                  <span class="dungeon-section-icon" aria-hidden="true">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <circle cx="12" cy="12" r="10" />
                      <path d="M12 6v6l4 2" />
                    </svg>
                  </span>
                  任务目标
                </h3>
                <ul class="dungeon-objectives">
                  <For each={props.viewModel.objectives}>
                    {(obj) => (
                      <li class="dungeon-objective">
                        <span class="objective-bullet" aria-hidden="true">
                          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="5 12 10 17 19 7" />
                          </svg>
                        </span>
                        <span class="objective-text">{obj}</span>
                      </li>
                    )}
                  </For>
                </ul>
              </section>

              {/* Rewards */}
              <section class="dungeon-section">
                <h3 class="dungeon-section-title">
                  <span class="dungeon-section-icon" aria-hidden="true">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />
                    </svg>
                  </span>
                  任务奖励
                </h3>
                <div class="dungeon-rewards">
                  <For each={props.viewModel.rewards}>
                    {(reward) => (
                      <div class="dungeon-reward">
                        <div class="dungeon-reward-icon" aria-hidden="true">
                          {reward.icon ? (
                            <img src={reward.icon} alt="" />
                          ) : (
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                              <rect x="3" y="3" width="18" height="18" rx="2" />
                              <circle cx="8.5" cy="8.5" r="1.5" />
                              <path d="M21 15l-5-5L5 21" />
                            </svg>
                          )}
                        </div>
                        <span class="dungeon-reward-name">{reward.name}</span>
                        {reward.quantity !== undefined && reward.quantity > 1 && (
                          <span class="dungeon-reward-qty">x{reward.quantity}</span>
                        )}
                      </div>
                    )}
                  </For>
                </div>
              </section>
            </div>

            {/* Book spine */}
            <div class="dungeon-book-spine" aria-hidden="true" />

            {/* ── Right Page: Stage List ────────────────── */}
            <div class="dungeon-page dungeon-page--right">
              <h3 class="dungeon-stage-list-title">副本进度</h3>
              <div class="dungeon-stages">
                <For each={props.viewModel.stages}>
                  {(stage) => (
                    <div
                      class={`dungeon-stage${stage.status === "completed" ? " dungeon-stage--completed" : stage.status === "locked" ? " dungeon-stage--locked" : ""}`}
                      data-stage-id={stage.id}
                    >
                      <span class="dungeon-stage-number">{stage.number}</span>
                      <div class="dungeon-stage-info">
                        <span class="dungeon-stage-name">{stage.name}</span>
                        <div class="dungeon-stage-tags">
                          {stage.hasCombat && (
                            <span class="stage-tag stage-tag--combat">
                              <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M14.5 17.5L3 6V3h3l11.5 11.5" />
                                <path d="M13 19l6-6" />
                                <path d="M16 16l4 4" />
                                <path d="M19 21l2-2" />
                              </svg>
                              战斗
                            </span>
                          )}
                          {stage.hasTreasure && (
                            <span class="stage-tag stage-tag--treasure">
                              <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M21 16V8a2 2 0 00-1-1.73l-7-4a2 2 0 00-2 0l-7 4A2 2 0 003 8v8a2 2 0 001 1.73l7 4a2 2 0 002 0l7-4A2 2 0 0021 16z" />
                              </svg>
                              宝藏
                            </span>
                          )}
                        </div>
                      </div>
                      <span class="dungeon-stage-status">
                        {stage.status === "completed" ? (
                          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="5 12 10 17 19 7" />
                          </svg>
                        ) : stage.status === "locked" ? (
                          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <rect x="3" y="11" width="18" height="11" rx="2" ry="2" />
                            <path d="M7 11V7a5 5 0 0110 0v4" />
                          </svg>
                        ) : (
                          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <circle cx="12" cy="12" r="10" />
                            <polyline points="12 6 12 12 16 14" />
                          </svg>
                        )}
                      </span>
                    </div>
                  )}
                </For>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left">
          <span class="expedition-status-pill expedition-status-pill--ready">
            <span class="expedition-status-pill-dot" aria-hidden="true" />
            准备进入副本
          </span>
        </div>
        <div class="expedition-controls-right">
          <button class="action-secondary" onClick={props.onReturnToTown}>
            返回城镇
          </button>
          <button
            class="action-primary launch-primary"
            onClick={props.onEnterDungeonMap}
            disabled={!props.viewModel.isEnterable}
            data-testid="enter-dungeon-map-btn"
          >
            {props.viewModel.isEnterable ? "进入地图" : "未解锁"}
          </button>
        </div>
      </footer>
    </div>
  );
};
