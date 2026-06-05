import { For, type Component } from "solid-js";

import type { DungeonSettlementViewModel } from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface DungeonSettlementScreenProps {
  viewModel: DungeonSettlementViewModel;
  onContinue: () => void;
  onReturnToTown: () => void;
}

/**
 * Dungeon settlement screen — book-style parchment layout.
 *
 * Displays dungeon encounter outcomes with rewards, treasures, heirlooms,
 * and party member status in a twin-page journal aesthetic.
 *
 * Reference image: 副本结算.png
 *   Left page: outcome badge + reward segments + treasure gold count
 *   Right page: heirloom tallies + party outcome cards
 *   Bottom: continue prompt
 */
export const DungeonSettlementScreen: Component<DungeonSettlementScreenProps> = (props) => {
  const outcomeClass = () => {
    switch (props.viewModel.outcome) {
      case "victory":
        return "settlement-outcome settlement-outcome--victory";
      case "defeat":
        return "settlement-outcome settlement-outcome--defeat";
      case "retreat":
        return "settlement-outcome settlement-outcome--retreat";
    }
  };

  const outcomeLabel = () => {
    switch (props.viewModel.outcome) {
      case "victory":
        return "胜利";
      case "defeat":
        return "失败";
      case "retreat":
        return "撤退";
    }
  };

  const heroStatusClass = (status: string) => {
    switch (status) {
      case "dead":
        return "settlement-hero-status settlement-hero-status--dead";
      case "wounded":
        return "settlement-hero-status settlement-hero-status--wounded";
      case "stressed":
        return "settlement-hero-status settlement-hero-status--stressed";
      default:
        return "settlement-hero-status settlement-hero-status--alive";
    }
  };

  const heroStatusLabel = (status: string) => {
    switch (status) {
      case "dead":
        return "阵亡";
      case "wounded":
        return "受伤";
      case "stressed":
        return "压力";
      default:
        return "存活";
    }
  };

  const totalRewardProgress = () => {
    if (props.viewModel.rewardSegments.length === 0) return 0;
    const totalValue = props.viewModel.rewardSegments.reduce((sum, seg) => sum + seg.value, 0);
    const totalMax = props.viewModel.rewardSegments.reduce((sum, seg) => sum + seg.max, 0);
    if (totalMax <= 0) return 0;
    return Math.round((totalValue / totalMax) * 100);
  };

  return (
    <div
      class="dungeon-settlement-viewport"
      data-source-scene="UI_Dungeon/DungeonSettlementWindow"
      data-source-prefab="Assets/Prefabs/UI/DungeonSettlementWindow.prefab"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">副本结算</span>
          <h1 class="expedition-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class="hud-pill hud-pill-accent">{props.viewModel.dungeonName}</span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="dungeon-settlement-content">
          {/* Book-like twin-page layout */}
          <div class="settlement-book">
            {/* ── Left Page ───────────────────────────────── */}
            <div class="settlement-page settlement-page--left">
              {/* Outcome badge */}
              <div class={outcomeClass()}>
                <div class="settlement-outcome-icon" aria-hidden="true">
                  <span class="settlement-outcome-letter">
                    {props.viewModel.outcome === "victory" ? "C" : "X"}
                  </span>
                </div>
                <div class="settlement-outcome-text">
                  <div class="settlement-outcome-label">{outcomeLabel()}</div>
                  <div class="settlement-outcome-dungeon">{props.viewModel.dungeonName}</div>
                </div>
              </div>

              {/* Summary */}
              <p class="settlement-summary">{props.viewModel.summary}</p>

              {/* Rewards section */}
              <div class="settlement-section">
                <h2 class="settlement-section-title">{props.viewModel.rewardsLabel}</h2>
                <div class="settlement-reward-bar">
                  <For each={props.viewModel.rewardSegments}>
                    {(segment) => (
                      <div
                        class="settlement-reward-segment"
                        style={{
                          width: `${Math.round((segment.value / Math.max(segment.max, 1)) * 100)}%`,
                          "background-color": segment.color,
                        }}
                        title={`${segment.label}: ${segment.value}/${segment.max}`}
                      />
                    )}
                  </For>
                </div>
                <div class="settlement-reward-legend">
                  <For each={props.viewModel.rewardSegments}>
                    {(segment) => (
                      <div class="settlement-reward-legend-item">
                        <span
                          class="settlement-reward-legend-dot"
                          style={{ "background-color": segment.color }}
                          aria-hidden="true"
                        />
                        <span class="settlement-reward-legend-label">{segment.label}</span>
                        <span class="settlement-reward-legend-value">{segment.value}/{segment.max}</span>
                      </div>
                    )}
                  </For>
                </div>
                <div class="settlement-reward-total">
                  <span class="settlement-reward-total-label">总进度</span>
                  <span class="settlement-reward-total-value">{totalRewardProgress()}%</span>
                </div>
              </div>

              {/* Treasures section */}
              <div class="settlement-section">
                <h2 class="settlement-section-title">{props.viewModel.treasuresLabel}</h2>
                <div class="settlement-treasure-row">
                  <span class="settlement-treasure-icon" aria-hidden="true">
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <circle cx="12" cy="12" r="9" />
                      <text x="12" y="16" text-anchor="middle" fill="currentColor" font-size="10" stroke="none">G</text>
                    </svg>
                  </span>
                  <span class="settlement-treasure-label">金币</span>
                  <span class="settlement-treasure-value">{props.viewModel.goldCollected}</span>
                </div>
              </div>
            </div>

            {/* ── Right Page ──────────────────────────────── */}
            <div class="settlement-page settlement-page--right">
              {/* Heirlooms section */}
              <div class="settlement-section">
                <h2 class="settlement-section-title">{props.viewModel.heirloomsLabel}</h2>
                <div class="settlement-heirloom-grid">
                  <For each={props.viewModel.heirlooms}>
                    {(heirloom) => (
                      <div class="settlement-heirloom-item">
                        <span class="settlement-heirloom-icon" aria-hidden="true">
                          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                            <path d="M12 2L2 7l10 5 10-5-10-5z" />
                            <path d="M2 17l10 5 10-5" />
                            <path d="M2 12l10 5 10-5" />
                          </svg>
                        </span>
                        <span class="settlement-heirloom-type">{heirloom.type}</span>
                        <span class="settlement-heirloom-count">{heirloom.count}</span>
                      </div>
                    )}
                  </For>
                </div>
              </div>

              {/* Party outcomes */}
              <div class="settlement-section">
                <h2 class="settlement-section-title">队伍状态</h2>
                <div class="settlement-party-list">
                  <For each={props.viewModel.partyOutcomes}>
                    {(hero) => {
                      const portraitUrl = resolveHeroPortrait({ heroId: hero.heroId, classLabel: hero.classLabel });
                      return (
                        <div class="settlement-party-card">
                          {portraitUrl ? (
                            <img
                              class="settlement-party-portrait"
                              src={portraitUrl}
                              alt={hero.heroName}
                            />
                          ) : (
                            <div class="settlement-party-portrait settlement-party-portrait--fallback">
                              <span class="settlement-party-portrait-letter">
                                {hero.heroName[0]}
                              </span>
                            </div>
                          )}
                          <div class="settlement-party-info">
                            <div class="settlement-party-name">{hero.heroName}</div>
                            <div class="settlement-party-class">{hero.classLabel} · Lv.{hero.level}</div>
                          </div>
                          <div class="settlement-party-meta">
                            <span class={heroStatusClass(hero.status)}>
                              {heroStatusLabel(hero.status)}
                            </span>
                            {hero.xpGained > 0 && (
                              <span class="settlement-party-xp">+{hero.xpGained} XP</span>
                            )}
                          </div>
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
        <div class="expedition-controls-left">
          <button class="action-secondary" onClick={props.onReturnToTown}>
            返回城镇
          </button>
        </div>
        <div class="expedition-controls-right">
          <button
            class="action-primary launch-primary"
            onClick={props.onContinue}
            disabled={!props.viewModel.isContinueAvailable}
          >
            {props.viewModel.isContinueAvailable
              ? "点击继续"
              : "等待中..."}
          </button>
        </div>
      </footer>
    </div>
  );
};
