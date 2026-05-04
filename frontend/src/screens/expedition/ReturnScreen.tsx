import { For, type Component } from "solid-js";

import type { ReturnViewModel } from "../../bridge/contractTypes";

interface ReturnScreenProps {
  viewModel: ReturnViewModel;
  onResumeTown: () => void;
}

/**
 * Return screen — landscape viewport layout.
 *
 * Shows returning heroes and expedition conclusion summary.
 * References original Unity prefab structure from:
 *   Assets/Prefabs/UI/ReturnToTownWindow.prefab (estimated)
 */
export const ReturnScreen: Component<ReturnScreenProps> = (props) => {
  return (
    <div class="expedition-viewport">
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">Expedition Concluded</span>
          <h1 class="expedition-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class="hud-pill hud-pill-accent">{props.viewModel.expeditionName}</span>
          <span class="hud-pill" style="color: #5bbd6e;">Closed</span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="expedition-content">
          {/* Summary */}
          <div class="outcome-banner outcome-banner--success">
            <h2 class="outcome-banner-title">Expedition Concluded</h2>
            <p class="outcome-banner-subtitle">{props.viewModel.summary}</p>
            <p class="outcome-banner-summary" style="opacity: 0.7;">
              The expedition log has been closed. All surviving heroes have returned to the roster.
              Visit town buildings to tend to hero conditions and prepare for the next expedition.
            </p>
          </div>

          {/* Returning heroes */}
          {props.viewModel.returningHeroes.length > 0 && (
            <div class="returning-hero-row">
              <For each={props.viewModel.returningHeroes}>
                {(hero) => (
                  <div class="returning-hero-card">
                    <div class="returning-hero-card-name">{hero.heroName}</div>
                    <div class="returning-hero-card-stats">
                      <div class="returning-hero-stat">
                        <span style="color: var(--panel-muted);">HP</span>
                        <span style="color: #5bbd6e;">{hero.hp}</span>
                      </div>
                      <div class="returning-hero-stat">
                        <span style="color: var(--panel-muted);">Stress</span>
                        <span style="color: #e8a838;">{hero.stress}</span>
                      </div>
                    </div>
                  </div>
                )}
              </For>
            </div>
          )}
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left" />
        <div class="expedition-controls-right">
          <button
            class="action-primary"
            onClick={props.onResumeTown}
            disabled={!props.viewModel.isTownResumeAvailable}
          >
            Resume Town Activities
          </button>
        </div>
      </footer>
    </div>
  );
};
