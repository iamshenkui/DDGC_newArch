import { For, type Component } from "solid-js";

import type { ExpeditionResultViewModel } from "../../bridge/contractTypes";
import { resolveChromeAsset, resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface ResultScreenProps {
  viewModel: ExpeditionResultViewModel;
  onContinue: () => void;
  onReturnToTown: () => void;
}

/**
 * Dungeon settlement screen (副本结算) — Chinese book-style layout.
 *
 * References original Unity prefab structure from:
 *   Assets/Prefabs/UI/Raid/HeroResultSlot.prefab
 *   Assets/Prefabs/UI/Raid/LootSlot.prefab
 *
 * Layout fidelity to reference image:
 *   - Open book / diary surface with left and right pages
 *   - Left page: dungeon seal, outcome label, reward progress bar,
 *     collected treasure (gold)
 *   - Right page: heirloom counts, returning hero rows with
 *     portrait + status icons
 *   - Bottom controls: return to town + continue
 */
export const ResultScreen: Component<ResultScreenProps> = (props) => {
  const outcomeSealLabel = () => {
    switch (props.viewModel.outcome) {
      case "success":
        return "成功撤离";
      case "failure":
        return "全军覆没";
      case "partial":
        return "仓促撤离";
    }
  };

  const outcomeSealClass = () => {
    switch (props.viewModel.outcome) {
      case "success":
        return "settlement-seal settlement-seal--success";
      case "failure":
        return "settlement-seal settlement-seal--failure";
      case "partial":
        return "settlement-seal settlement-seal--partial";
    }
  };

  const heirloomCounts = () => {
    const h = props.viewModel.heirloomsGained;
    return {
      portrait: h?.portrait ?? 0,
      deed: h?.deed ?? 0,
      crest: h?.crest ?? 0,
      relic: h?.relic ?? 0,
    };
  };

  const rewardSegments = () => {
    switch (props.viewModel.outcome) {
      case "success":
        return [
          { label: "探索", active: true },
          { label: "战斗", active: true },
          { label: "宝藏", active: true },
          { label: "撤离", active: true },
        ];
      case "partial":
        return [
          { label: "探索", active: true },
          { label: "战斗", active: true },
          { label: "宝藏", active: false },
          { label: "撤离", active: true },
        ];
      case "failure":
        return [
          { label: "探索", active: true },
          { label: "战斗", active: false },
          { label: "宝藏", active: false },
          { label: "撤离", active: false },
        ];
    }
  };

  return (
    <div
      class="expedition-viewport"
      data-source-scene="UI_Raid/DungeonSettlementWindow"
      data-source-prefab="Assets/Prefabs/UI/Raid/HeroResultSlot.prefab"
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

        <div class="settlement-content">
          {/* Book-like panel */}
          <div class="settlement-book">
            {/* ── Left page ─────────────────────────────── */}
            <div class="settlement-page settlement-page--left">
              <div class="settlement-page-inner">
                {/* Seal + outcome */}
                <div class="settlement-header-left">
                  <div class={outcomeSealClass()} aria-label="outcome-seal">
                    <span class="settlement-seal-letter">C</span>
                  </div>
                  <div class="settlement-outcome-text">
                    <div class="settlement-outcome-label">{outcomeSealLabel()}</div>
                    <div class="settlement-dungeon-name">{props.viewModel.expeditionName}</div>
                  </div>
                </div>

                {/* Reward progress bar */}
                <div class="settlement-reward-section">
                  <div class="settlement-reward-bar" role="img" aria-label="reward-progress">
                    <For each={rewardSegments()}>
                      {(seg, idx) => (
                        <div
                          class={`settlement-reward-segment ${seg.active ? "settlement-reward-segment--active" : ""}`}
                          style={{ "--seg-index": idx() } as Record<string, string>}
                        >
                          <span class="settlement-reward-segment-label">{seg.label}</span>
                        </div>
                      )}
                    </For>
                  </div>
                  <div class="settlement-reward-title">收集的奖励</div>
                </div>

                {/* Treasure */}
                <div class="settlement-treasure">
                  <div class="settlement-treasure-title">收集的宝藏</div>
                  <div class="settlement-treasure-row">
                    <img
                      class="settlement-treasure-icon"
                      src={resolveChromeAsset("goldIcon")}
                      alt=""
                      aria-hidden="true"
                    />
                    <span class="settlement-treasure-value" data-testid="settlement-gold">
                      {props.viewModel.resourcesGained.gold}
                    </span>
                  </div>
                </div>
              </div>
            </div>

            {/* Spine */}
            <div class="settlement-book-spine" aria-hidden="true" />

            {/* ── Right page ────────────────────────────── */}
            <div class="settlement-page settlement-page--right">
              <div class="settlement-page-inner">
                {/* Heirloom counts */}
                <div
                  class="settlement-heirloom-section"
                  data-blocker="BLOCKER-005: original heirloom runtime counters not wired to headless bridge"
                >
                  <div class="settlement-heirloom-title">收集的传家宝</div>
                  <div class="settlement-heirloom-grid">
                    <div class="settlement-heirloom-cell">
                      <span class="settlement-heirloom-icon" aria-hidden="true">○</span>
                      <span class="settlement-heirloom-count" data-testid="heirloom-portrait">
                        {heirloomCounts().portrait}
                      </span>
                    </div>
                    <div class="settlement-heirloom-cell">
                      <span class="settlement-heirloom-icon" aria-hidden="true">□</span>
                      <span class="settlement-heirloom-count" data-testid="heirloom-deed">
                        {heirloomCounts().deed}
                      </span>
                    </div>
                    <div class="settlement-heirloom-cell">
                      <span class="settlement-heirloom-icon" aria-hidden="true">△</span>
                      <span class="settlement-heirloom-count" data-testid="heirloom-crest">
                        {heirloomCounts().crest}
                      </span>
                    </div>
                    <div class="settlement-heirloom-cell">
                      <span class="settlement-heirloom-icon" aria-hidden="true">◇</span>
                      <span class="settlement-heirloom-count" data-testid="heirloom-relic">
                        {heirloomCounts().relic}
                      </span>
                    </div>
                  </div>
                </div>

                {/* Hero list */}
                <div class="settlement-hero-list" role="list" aria-label="returning heroes">
                  <For each={props.viewModel.heroOutcomes}>
                    {(hero) => {
                      const portraitUrl = resolveHeroPortrait({ heroId: hero.heroId, classLabel: hero.classLabel });
                      const isDead = hero.status === "dead";
                      return (
                        <div class="settlement-hero-row" role="listitem">
                          {portraitUrl ? (
                            <img
                              class={`settlement-hero-portrait ${isDead ? "settlement-hero-portrait--dead" : ""}`}
                              src={portraitUrl}
                              alt={hero.heroName}
                            />
                          ) : (
                            <div class="settlement-hero-portrait settlement-hero-portrait--fallback">
                              <span class="settlement-hero-portrait-letter">{hero.heroName[0]}</span>
                            </div>
                          )}
                          <div class="settlement-hero-info">
                            <div class={`settlement-hero-name ${isDead ? "settlement-hero-name--dead" : ""}`}>
                              {hero.heroName}
                            </div>
                            <div class="settlement-hero-status-dots">
                              <span
                                class={`settlement-status-dot settlement-status-dot--hp ${hero.hpChange.startsWith("-") ? "settlement-status-dot--down" : "settlement-status-dot--up"}`}
                                title={`HP ${hero.hpChange}`}
                                aria-label={`HP change ${hero.hpChange}`}
                              />
                              <span
                                class={`settlement-status-dot settlement-status-dot--stress ${hero.stressChange.startsWith("+") ? "settlement-status-dot--up" : "settlement-status-dot--down"}`}
                                title={`Stress ${hero.stressChange}`}
                                aria-label={`Stress change ${hero.stressChange}`}
                              />
                              <span
                                class={`settlement-status-dot settlement-status-dot--state ${isDead ? "settlement-status-dot--dead" : "settlement-status-dot--alive"}`}
                                title={isDead ? "阵亡" : "存活"}
                                aria-label={isDead ? "dead" : "alive"}
                              />
                            </div>
                          </div>
                        </div>
                      );
                    }}
                  </For>
                </div>

                {/* Loot list (small, below heroes) */}
                {props.viewModel.lootAcquired.length > 0 && (
                  <div class="settlement-loot-section">
                    <div class="settlement-loot-title">获得物品</div>
                    <div class="settlement-loot-list">
                      <For each={props.viewModel.lootAcquired}>
                        {(item) => <div class="settlement-loot-item">{item}</div>}
                      </For>
                    </div>
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left" />
        <div class="expedition-controls-right">
          <button
            class="action-secondary settlement-return-btn"
            onClick={props.onReturnToTown}
            data-testid="settlement-return-to-town"
          >
            返回城镇
          </button>
          <button
            class="action-primary launch-primary settlement-continue-btn"
            onClick={props.onContinue}
            disabled={!props.viewModel.isContinueAvailable}
            data-testid="settlement-continue"
          >
            {props.viewModel.isContinueAvailable ? "继续" : "等待结算"}
          </button>
        </div>
      </footer>
    </div>
  );
};
