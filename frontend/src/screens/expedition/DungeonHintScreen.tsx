import { For, type Component } from "solid-js";

import type { DungeonHintViewModel } from "../../bridge/contractTypes";

interface DungeonHintScreenProps {
  viewModel: DungeonHintViewModel;
  onEnterDungeon: () => void;
  onReturnToTown: () => void;
}

/**
 * Dungeon Hint screen — 位面探索-副本提示
 *
 * Displays dungeon information, tips, warnings, expected enemies,
 * and reward preview before the player commits to entering.
 *
 * Layout mirrors the expedition viewport pattern with:
 *   - Top HUD with expedition name and difficulty
 *   - Central info panels (description, tips, enemies, rewards)
 *   - Bottom controls with Enter / Return buttons
 */
export const DungeonHintScreen: Component<DungeonHintScreenProps> = (props) => {
  return (
    <div
      class="expedition-viewport"
      data-source-scene="UI_Expedition/DungeonHintWindow"
      data-source-prefab="Assets/Prefabs/UI/DungeonHintWindow.prefab"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">Dungeon Briefing</span>
          <h1 class="expedition-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class="hud-pill hud-pill-accent">{props.viewModel.expeditionName}</span>
          <span class="hud-pill hud-pill--with-icon">
            <span class="hud-pill-icon" aria-hidden="true">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7">
                <path d="M12 2L4 6v6c0 5 3.4 9.4 8 10 4.6-.6 8-5 8-10V6l-8-4z" />
              </svg>
            </span>
            Difficulty: {props.viewModel.difficulty}
          </span>
          <span class="hud-pill hud-pill--with-icon">
            <span class="hud-pill-icon" aria-hidden="true">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7">
                <circle cx="9" cy="8" r="3.5" />
                <path d="M2 20c0-3.6 3.2-6 7-6s7 2.4 7 6" />
                <circle cx="17" cy="9" r="2.5" />
                <path d="M22 19c0-2.4-1.8-4-4-4" />
              </svg>
            </span>
            Party: {props.viewModel.partySize}
          </span>
          <span class="hud-pill">
            Est. Duration: {props.viewModel.estimatedDuration}
          </span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="expedition-content dungeon-hint-content">
          {/* Dungeon description banner */}
          <div class="dungeon-hint-banner">
            <span class="dungeon-hint-banner-ornament" aria-hidden="true" />
            <h2 class="dungeon-hint-banner-title">{props.viewModel.expeditionName}</h2>
            <span class="dungeon-hint-banner-ornament" aria-hidden="true" />
            <p class="dungeon-hint-banner-desc">{props.viewModel.dungeonDescription}</p>
          </div>

          {/* Twin panels: tips + enemies */}
          <div class="dungeon-hint-panels">
            <section class="details-overlay dungeon-hint-panel">
              <header class="details-overlay-header">
                <span class="details-overlay-frame-rule" aria-hidden="true" />
                <h2 class="details-overlay-title">Dungeon Intel</h2>
                <span class="details-overlay-frame-rule" aria-hidden="true" />
              </header>
              <div class="details-overlay-row">
                <span class="details-overlay-label">Recommended Level</span>
                <span class="details-overlay-value">{props.viewModel.recommendedLevel}</span>
              </div>
              <div class="details-overlay-row">
                <span class="details-overlay-label">Party Size</span>
                <span class="details-overlay-value">{props.viewModel.partySize}</span>
              </div>
              <div class="details-overlay-row">
                <span class="details-overlay-label">Difficulty</span>
                <span class="details-overlay-value">{props.viewModel.difficulty}</span>
              </div>
              <div class="details-overlay-row">
                <span class="details-overlay-label">Est. Duration</span>
                <span class="details-overlay-value">{props.viewModel.estimatedDuration}</span>
              </div>
              <div class="details-overlay-row">
                <span class="details-overlay-label">Supply Level</span>
                <span class="details-overlay-value">{props.viewModel.supplyLevel}</span>
              </div>
              <div class="details-overlay-row">
                <span class="details-overlay-label">Provision Cost</span>
                <span class="details-overlay-value">{props.viewModel.provisionCost}</span>
              </div>

              {props.viewModel.tips.length > 0 && (
                <div class="dungeon-hint-section">
                  <div class="dungeon-hint-section-title">Tips</div>
                  <For each={props.viewModel.tips}>
                    {(tip) => (
                      <div class="dungeon-hint-tip-row">
                        <span class="dungeon-hint-tip-marker" aria-hidden="true">
                          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                            <circle cx="12" cy="12" r="10" />
                            <path d="M12 16v-4" />
                            <path d="M12 8h.01" />
                          </svg>
                        </span>
                        <span class="dungeon-hint-tip-text">{tip}</span>
                      </div>
                    )}
                  </For>
                </div>
              )}
            </section>

            <section class={`details-overlay dungeon-hint-panel ${props.viewModel.warnings.length > 0 ? "readiness-checks--cautious" : "readiness-checks--clear"}`}>
              <header class="details-overlay-header">
                <span class="details-overlay-frame-rule" aria-hidden="true" />
                <h2 class="details-overlay-title">Threat Assessment</h2>
                <span class="details-overlay-frame-rule" aria-hidden="true" />
              </header>

              {props.viewModel.warnings.length > 0 ? (
                <For each={props.viewModel.warnings}>
                  {(warning) => (
                    <div class="readiness-check-row readiness-check-row--warn">
                      <span class="readiness-check-marker readiness-check-marker--warn" aria-hidden="true">
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
                          <path d="M12 3L1 21h22L12 3z" />
                          <line x1="12" y1="10" x2="12" y2="14" />
                          <circle cx="12" cy="17" r="0.8" fill="currentColor" />
                        </svg>
                      </span>
                      <span class="readiness-check-text">{warning}</span>
                    </div>
                  )}
                </For>
              ) : (
                <div class="readiness-check-row readiness-check-row--ok">
                  <span class="readiness-check-marker readiness-check-marker--ok" aria-hidden="true">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                      <polyline points="5 12 10 17 19 7" />
                    </svg>
                  </span>
                  <span class="readiness-check-text">No immediate threats detected</span>
                </div>
              )}

              {props.viewModel.expectedEnemies.length > 0 && (
                <div class="dungeon-hint-section">
                  <div class="dungeon-hint-section-title">Expected Foes</div>
                  <div class="dungeon-hint-enemy-list">
                    <For each={props.viewModel.expectedEnemies}>
                      {(enemy) => (
                        <span class="dungeon-hint-enemy-chip">{enemy}</span>
                      )}
                    </For>
                  </div>
                </div>
              )}

              {props.viewModel.rewardPreview.length > 0 && (
                <div class="dungeon-hint-section">
                  <div class="dungeon-hint-section-title">Possible Rewards</div>
                  <For each={props.viewModel.rewardPreview}>
                    {(reward) => (
                      <div class="dungeon-hint-reward-row">
                        <span class="dungeon-hint-reward-marker" aria-hidden="true">
                          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="5 12 10 17 19 7" />
                          </svg>
                        </span>
                        <span class="dungeon-hint-reward-text">{reward}</span>
                      </div>
                    )}
                  </For>
                </div>
              )}
            </section>
          </div>
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left">
          <span class={`expedition-status-pill ${props.viewModel.isEnterable ? "expedition-status-pill--ready" : "expedition-status-pill--afflicted"}`}>
            <span class="expedition-status-pill-dot" aria-hidden="true" />
            {props.viewModel.isEnterable ? "Ready to enter" : "Requirements not met"}
          </span>
        </div>
        <div class="expedition-controls-right">
          <button class="action-secondary" onClick={props.onReturnToTown}>
            Return to Town
          </button>
          <button
            class="action-primary launch-primary"
            onClick={props.onEnterDungeon}
            disabled={!props.viewModel.isEnterable}
          >
            {props.viewModel.isEnterable ? "Enter Dungeon" : "Not Ready"}
          </button>
        </div>
      </footer>
    </div>
  );
};
