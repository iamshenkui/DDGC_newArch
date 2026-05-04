import { For, type Component } from "solid-js";

import type { ReturnViewModel } from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface ReturnScreenProps {
  viewModel: ReturnViewModel;
  onResumeTown: () => void;
}

/**
 * Return screen — landscape viewport layout.
 *
 * Shows returning heroes and expedition conclusion summary.
 * References original Unity prefab structure from:
 *   Assets/Prefabs/UI/ReturnToTownWindow.prefab
 *
 * Source hierarchy: UI_Expedition/ReturnToTownWindow
 *   SummaryBannerPanel → CloseLabel + SummaryText
 *   ReturningHeroPanel → HeroReturnCard × 4
 *   ResumeButton → return to estate/town surface
 */
export const ReturnScreen: Component<ReturnScreenProps> = (props) => {
  return (
    <div
      class="expedition-viewport"
      data-source-scene="UI_Expedition/ReturnToTownWindow"
      data-source-prefab="Assets/Prefabs/UI/ReturnToTownWindow.prefab"
    >
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
          {/* Summary banner */}
          <div class="outcome-banner outcome-banner--success">
            <div class="outcome-banner-ornament" />
            <h2 class="outcome-banner-title">Expedition Log Closed</h2>
            <p class="outcome-banner-subtitle">{props.viewModel.summary}</p>
            <p class="return-summary-text">
              The expedition has concluded. All surviving heroes have returned to the Estate.
              Visit town buildings to tend to hero conditions and prepare for future expeditions.
            </p>
          </div>

          {/* Returning heroes */}
          {props.viewModel.returningHeroes.length > 0 && (
            <div class="returning-hero-row">
              <For each={props.viewModel.returningHeroes}>
                {(hero) => {
                  const portraitUrl = resolveHeroPortrait({ heroId: hero.heroId, classLabel: hero.classLabel });
                  return (
                    <div class="returning-hero-card">
                      {portraitUrl ? (
                        <img
                          class="returning-hero-portrait"
                          src={portraitUrl}
                          alt={hero.heroName}
                        />
                      ) : (
                        <div class="returning-hero-portrait returning-hero-portrait--fallback">
                          <span class="returning-hero-portrait-letter">
                            {hero.heroName[0]}
                          </span>
                        </div>
                      )}
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
                  );
                }}
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
            class="action-primary launch-primary"
            onClick={props.onResumeTown}
            disabled={!props.viewModel.isTownResumeAvailable}
          >
            {props.viewModel.isTownResumeAvailable
              ? "Resume Town Activities"
              : "Awaiting Town Handoff"}
          </button>
        </div>
      </footer>
    </div>
  );
};
