import { For, type Component } from "solid-js";

import type { ExpeditionResultViewModel } from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface ResultScreenProps {
  viewModel: ExpeditionResultViewModel;
  onContinue: () => void;
  onReturnToTown: () => void;
}

/**
 * Dungeon settlement screen — book/ledger layout.
 *
 * Displays expedition outcome with hero outcomes, loot, and resources gained.
 * References original Unity prefab structure from:
 *   Assets/Prefabs/UI/DungeonSettlementWindow.prefab
 *
 * Source hierarchy: UI_Dungeon/DungeonSettlementWindow
 *   LeftPage → GradeBadge + DungeonName + OutcomeLabel + RewardsBar + TreasureCount
 *   RightPage → HeirloomCounts + HeroRoster
 *   ContinueHint → navigate to return/flow
 */
export const ResultScreen: Component<ResultScreenProps> = (props) => {
  const outcomeColor = () => {
    switch (props.viewModel.outcome) {
      case "success":
        return "#5bbd6e";
      case "failure":
        return "#ea7767";
      case "partial":
        return "#e8a838";
    }
  };

  const heroStatusIcon = (status: string) => {
    switch (status) {
      case "dead":
        return "✕";
      case "stressed":
        return "!";
      default:
        return "✓";
    }
  };

  const heroStatusClass = (status: string) => {
    switch (status) {
      case "dead":
        return "settlement-hero-status settlement-hero-status--dead";
      case "stressed":
        return "settlement-hero-status settlement-hero-status--stressed";
      default:
        return "settlement-hero-status settlement-hero-status--alive";
    }
  };

  return (
    <div
      class="expedition-viewport"
      data-source-scene="UI_Dungeon/DungeonSettlementWindow"
      data-source-prefab="Assets/Prefabs/UI/DungeonSettlementWindow.prefab"
      data-testid="dungeon-settlement-screen"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">副本结算</span>
          <h1 class="expedition-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class="hud-pill hud-pill-accent">{props.viewModel.expeditionName}</span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="expedition-content settlement-content">
          {/* Book / Ledger container */}
          <div class="settlement-ledger">
            {/* Spine fold */}
            <div class="settlement-spine" />

            {/* Left Page */}
            <div class="settlement-page settlement-page--left">
              {/* Grade badge */}
              <div class="settlement-grade">
                <span
                  class="settlement-grade-letter"
                  style={{ color: outcomeColor() }}
                >
                  {props.viewModel.grade}
                </span>
              </div>

              {/* Dungeon name */}
              <div class="settlement-dungeon-name">{props.viewModel.expeditionName}</div>

              {/* Outcome label */}
              <div
                class="settlement-outcome"
                style={{ color: outcomeColor() }}
              >
                {props.viewModel.outcomeLabel}
              </div>

              {/* Rewards section */}
              <div class="settlement-section">
                <div class="settlement-section-title">获得的奖励</div>
                <div class="settlement-rewards-bar">
                  <div
                    class="settlement-rewards-fill"
                    style={{ width: "60%", background: outcomeColor() }}
                  />
                </div>
              </div>

              {/* Treasure section */}
              <div class="settlement-section">
                <div class="settlement-section-title">收集的宝藏</div>
                <div class="settlement-treasure">
                  <span class="settlement-treasure-icon" aria-hidden="true">
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <path d="M12 2L4 6v6c0 5 3.4 9.4 8 10 4.6-.6 8-5 8-10V6l-8-4z" />
                    </svg>
                  </span>
                  <span class="settlement-treasure-value">{props.viewModel.resourcesGained.gold}</span>
                </div>
              </div>
            </div>

            {/* Right Page */}
            <div class="settlement-page settlement-page--right">
              {/* Heirlooms section */}
              <div class="settlement-section">
                <div class="settlement-section-title">收集的传家宝</div>
                <div class="settlement-heirlooms">
                  <For each={props.viewModel.heirlooms}>
                    {(heirloom) => (
                      <div class="settlement-heirloom">
                        <span class="settlement-heirloom-label">{heirloom.label}</span>
                        <span class="settlement-heirloom-count">{heirloom.count}</span>
                      </div>
                    )}
                  </For>
                </div>
              </div>

              {/* Hero roster */}
              <div class="settlement-section">
                <div class="settlement-section-title settlement-heroes-title">远征队伍</div>
                <div class="settlement-heroes">
                  <For each={props.viewModel.heroOutcomes}>
                    {(hero) => {
                      const portraitUrl = resolveHeroPortrait({ heroId: hero.heroId, classLabel: hero.classLabel });
                      return (
                        <div class="settlement-hero">
                          <div class="settlement-hero-portrait-wrap">
                            {portraitUrl ? (
                              <img
                                class="settlement-hero-portrait"
                                src={portraitUrl}
                                alt={hero.heroName}
                              />
                            ) : (
                              <div class="settlement-hero-portrait settlement-hero-portrait--fallback">
                                <span class="settlement-hero-portrait-letter">
                                  {hero.heroName[0]}
                                </span>
                              </div>
                            )}
                            <span class={heroStatusClass(hero.status)}>
                              {heroStatusIcon(hero.status)}
                            </span>
                          </div>
                          <span class="settlement-hero-name">{hero.heroName}</span>
                        </div>
                      );
                    }}
                  </For>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left" />
        <div class="expedition-controls-right">
          <button class="action-secondary" onClick={props.onReturnToTown}>
            返回城镇
          </button>
          <button
            class="action-primary launch-primary"
            onClick={props.onContinue}
            disabled={!props.viewModel.isContinueAvailable}
            data-testid="settlement-continue-btn"
          >
            {props.viewModel.isContinueAvailable
              ? "点击继续"
              : "等待结算"}
          </button>
        </div>
      </footer>
    </div>
  );
};
