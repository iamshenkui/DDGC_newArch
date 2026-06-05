import { For, type Component } from "solid-js";

import type { ExpeditionResultViewModel } from "../../bridge/contractTypes";
import { resolveChromeAsset, resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface DungeonSettlementScreenProps {
  viewModel: ExpeditionResultViewModel;
  onContinue: () => void;
  onReturnToTown: () => void;
}

/**
 * Dungeon settlement screen — 副本结算
 *
 * Displays dungeon completion outcome with hero statuses, loot, and resources.
 * References original Unity prefab structure from:
 *   Assets/Prefabs/UI/DungeonSettlementWindow.prefab (estimated)
 *
 * Source hierarchy: UI_Dungeon/DungeonSettlementWindow
 *   SettlementBannerPanel → OutcomeLabel + OutcomeIcon
 *   HeroOutcomePanel → HeroOutcomeCard × 4
 *   RewardsPanel → GoldLabel + LootGrid + ResourceLine
 *   CloseButton → navigate to return/flow
 */
export const DungeonSettlementScreen: Component<DungeonSettlementScreenProps> = (props) => {
  const outcomeLabel = () => {
    switch (props.viewModel.outcome) {
      case "success":
        return "通关";
      case "failure":
        return "战败";
      case "partial":
        return "部分通关";
    }
  };

  const outcomeBannerClass = () => {
    switch (props.viewModel.outcome) {
      case "success":
        return "outcome-banner outcome-banner--success";
      case "failure":
        return "outcome-banner outcome-banner--failure";
      case "partial":
        return "outcome-banner outcome-banner--partial";
    }
  };

  const heroOutcomeStatusClass = (status: string) => {
    switch (status) {
      case "dead":
        return "hero-outcome-status--dead";
      case "stressed":
        return "hero-outcome-status--stressed";
      default:
        return "hero-outcome-status--alive";
    }
  };

  const heroOutcomeStatusLabel = (status: string) => {
    switch (status) {
      case "dead":
        return "阵亡";
      case "stressed":
        return "受压";
      default:
        return "存活";
    }
  };

  const heroCardExtraClass = (status: string) => {
    return status === "dead" ? "hero-outcome-card hero-outcome-card--dead" : "hero-outcome-card";
  };

  const heroHasCasualties = () =>
    props.viewModel.heroOutcomes.some((h) => h.status === "dead");

  const isFailure = () => props.viewModel.outcome === "failure";
  const isPartial = () => props.viewModel.outcome === "partial";

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
          <span class="eyebrow" data-testid="settlement-eyebrow">副本结算</span>
          <h1 class="expedition-title" data-testid="settlement-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class="hud-pill hud-pill-accent">{props.viewModel.expeditionName}</span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="expedition-content">
          {/* Outcome banner */}
          <div class={outcomeBannerClass()} data-testid="outcome-banner">
            <div class="outcome-banner-ornament" />
            <h2 class="outcome-banner-title" data-testid="outcome-title">{outcomeLabel()}</h2>
            <p class="outcome-banner-subtitle">{props.viewModel.summary}</p>
            {isFailure() && (
              <p class="outcome-banner-detail outcome-detail--failure">
                远征以失败告终。重整剩余力量——领地仍在坚守。
              </p>
            )}
            {isPartial() && (
              <p class="outcome-banner-detail outcome-detail--partial">
                远征达成了部分目标。在下次出征前，先治疗伤员。
              </p>
            )}
            {!isFailure() && !isPartial() && (
              <p class="outcome-banner-detail outcome-detail--success">
                远征成功结束。你的英雄们已准备好迎接下一个挑战。
              </p>
            )}
          </div>

          {/* Hero outcomes */}
          <div class="hero-outcome-row" data-testid="hero-outcome-row">
            <For each={props.viewModel.heroOutcomes}>
              {(hero) => {
                const portraitUrl = resolveHeroPortrait({ heroId: hero.heroId, classLabel: hero.classLabel });
                return (
                  <div class={heroCardExtraClass(hero.status)} data-testid={`hero-outcome-${hero.heroId}`}>
                    {portraitUrl ? (
                      <img
                        class="hero-outcome-portrait"
                        src={portraitUrl}
                        alt={hero.heroName}
                      />
                    ) : (
                      <div class="hero-outcome-portrait hero-outcome-portrait--fallback">
                        <span class="hero-outcome-portrait-letter">
                          {hero.heroName[0]}
                        </span>
                      </div>
                    )}
                    <div class="hero-outcome-name">{hero.heroName}</div>
                    <span class={`hero-outcome-status ${heroOutcomeStatusClass(hero.status)}`}>
                      {heroOutcomeStatusLabel(hero.status)}
                    </span>
                    {hero.status !== "dead" && (
                      <div class="hero-outcome-changes">
                        <div class="hero-outcome-change">
                          <span class="hero-outcome-change-label">生命</span>
                          <span class={`hero-outcome-change-value ${hero.hpChange.startsWith("-") ? "change-negative" : "change-positive"}`}>
                            {hero.hpChange}
                          </span>
                        </div>
                        <div class="hero-outcome-change">
                          <span class="hero-outcome-change-label">压力</span>
                          <span class={`hero-outcome-change-value ${hero.stressChange.startsWith("+") ? "change-stress" : "change-positive"}`}>
                            {hero.stressChange}
                          </span>
                        </div>
                      </div>
                    )}
                  </div>
                );
              }}
            </For>
          </div>

          {/* Resources gained */}
          <div class="resources-panel" data-testid="resources-panel">
            <div class="resource-item">
              <span class="resource-label">金币</span>
              <span class="resource-value resource-value--positive result-gold-value" data-testid="gold-value">
                <img
                  class="gold-icon-image"
                  src={resolveChromeAsset("goldIcon")}
                  alt=""
                  aria-hidden="true"
                />
                +{props.viewModel.resourcesGained.gold}
              </span>
            </div>
            <div class="resource-item">
              <span class="resource-label">补给</span>
              <span class={`resource-value ${props.viewModel.resourcesGained.supplies >= 0 ? "resource-value--positive" : "resource-value--negative"}`} data-testid="supplies-value">
                {props.viewModel.resourcesGained.supplies >= 0 ? "+" : ""}{props.viewModel.resourcesGained.supplies}
              </span>
            </div>
            <div class="resource-item">
              <span class="resource-label">经验</span>
              <span class="resource-value resource-value--positive" data-testid="experience-value">
                +{props.viewModel.resourcesGained.experience}
              </span>
            </div>
          </div>

          {/* Loot */}
          {props.viewModel.lootAcquired.length > 0 && (
            <div class="loot-panel" data-testid="loot-panel">
              <div class="loot-title">获得战利品</div>
              <For each={props.viewModel.lootAcquired}>
                {(item) => (
                  <div class="loot-item" data-testid="loot-item">{item}</div>
                )}
              </For>
            </div>
          )}

          {/* Casualty warning */}
          {heroHasCasualties() && (
            <div class="details-overlay" style="border-color: rgba(234, 119, 103, 0.3);">
              <p class="casualty-message">
                <strong>出现伤亡。</strong>部分英雄未能归来。前往次元感知塔招募新的队员。
              </p>
            </div>
          )}
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left" />
        <div class="expedition-controls-right">
          <button class="action-secondary" onClick={props.onReturnToTown} data-testid="return-to-town-btn">
            返回城镇
          </button>
          <button
            class="action-primary launch-primary"
            onClick={props.onContinue}
            disabled={!props.viewModel.isContinueAvailable}
            data-testid="continue-btn"
          >
            {props.viewModel.isContinueAvailable
              ? "继续返程"
              : "等待结算"}
          </button>
        </div>
      </footer>
    </div>
  );
};
