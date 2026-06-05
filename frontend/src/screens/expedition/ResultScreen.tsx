import { For, type Component } from "solid-js";

import type { ExpeditionResultViewModel } from "../../bridge/contractTypes";
import { resolveChromeAsset, resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface ResultScreenProps {
  viewModel: ExpeditionResultViewModel;
  onContinue: () => void;
  onReturnToTown: () => void;
}

/**
 * Dungeon settlement screen (副本结算) — book-style two-page spread.
 *
 * Mirrors the reference layout:
 *   Left page  → outcome badge, reward tier bar, treasure total
 *   Right page → heirloom tallies, returning hero rows with status orbs
 *
 * Source hierarchy: UI_Dungeon/DungeonSettlementWindow
 *   SettlementBook → LeftPage / Spine / RightPage
 *   RewardBar      → segmented reward tiers
 *   TreasureLine   → gold icon + value
 *   HeirloomRow    → three heirloom counters (data-blocker noted)
 *   HeroList       → portrait + name + HP/Stress orbs
 *   CloseButton    → return to town / continue flow
 */
export const ResultScreen: Component<ResultScreenProps> = (props) => {
  const outcomeLabel = () => {
    switch (props.viewModel.outcome) {
      case "success":
        return "胜利";
      case "failure":
        return "失败";
      case "partial":
        return "险胜";
    }
  };

  const outcomeSub = () => {
    switch (props.viewModel.outcome) {
      case "success":
        return "副本探索成功完成";
      case "failure":
        return "队伍被击败，被迫撤离";
      case "partial":
        return "部分目标达成，代价惨重";
    }
  };

  const outcomeClass = () =>
    `settlement-outcome settlement-outcome--${props.viewModel.outcome}`;

  const outcomeIcon = () => {
    switch (props.viewModel.outcome) {
      case "success":
        return "C";
      case "failure":
        return "×";
      case "partial":
        return "!";
    }
  };

  const rewardSegments = () => {
    const total = 5;
    let filled = props.viewModel.lootAcquired.length;
    if (props.viewModel.outcome === "failure") filled = 0;
    if (props.viewModel.outcome === "partial" && filled < 1) filled = 1;
    if (props.viewModel.outcome === "success" && filled < 1) filled = 3;
    return Array.from({ length: total }, (_, i) => i < filled);
  };

  const segmentClass = (index: number, filled: boolean) => {
    const base = "settlement-reward-segment";
    if (!filled) return base;
    const tier = index < 2 ? "gold" : index < 4 ? "silver" : "bronze";
    return `${base} ${base}--${tier}`;
  };

  const heroOrbClass = (change: string, kind: "hp" | "stress") => {
    const base = `settlement-hero-orb settlement-hero-orb--${kind}`;
    const num = Number.parseInt(change, 10);
    if (Number.isNaN(num)) return base;
    if (kind === "hp" && num < 0) return `${base} settlement-hero-orb--wounded`;
    if (kind === "stress" && num > 0) return `${base} settlement-hero-orb--stressed`;
    return `${base} settlement-hero-orb--healthy`;
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
          <h1 class="expedition-title">{props.viewModel.expeditionName}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class={`hud-pill ${props.viewModel.outcome === "failure" ? "pill-error" : "hud-pill-accent"}`}>
            {outcomeLabel()}
          </span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="expedition-content settlement-content">
          <div class="settlement-book">
            {/* ── Left page ───────────────────────────────── */}
            <div class="settlement-page settlement-page--left">
              <div class={outcomeClass()}>
                <div class="settlement-outcome-icon" aria-hidden="true">
                  {outcomeIcon()}
                </div>
                <div class="settlement-outcome-text">
                  <div class="settlement-outcome-label">{outcomeLabel()}</div>
                  <div class="settlement-outcome-sub">{outcomeSub()}</div>
                </div>
              </div>

              <section class="settlement-section">
                <h2 class="settlement-section-title">收集的奖励</h2>
                <div
                  class="settlement-reward-bar"
                  role="img"
                  aria-label={`reward tiers: ${props.viewModel.lootAcquired.length} loot items`}
                >
                  <For each={rewardSegments()}>
                    {(filled, i) => (
                      <div class={segmentClass(i(), filled)} />
                    )}
                  </For>
                </div>
                <div class="settlement-reward-legend">
                  <span class="settlement-reward-tag settlement-reward-tag--gold">
                    黄金
                  </span>
                  <span class="settlement-reward-tag settlement-reward-tag--silver">
                    白银
                  </span>
                  <span class="settlement-reward-tag settlement-reward-tag--bronze">
                    青铜
                  </span>
                </div>
              </section>

              <section class="settlement-section">
                <h2 class="settlement-section-title">收集的宝藏</h2>
                <div class="settlement-treasure">
                  <img
                    class="settlement-treasure-icon"
                    src={resolveChromeAsset("goldIcon")}
                    alt=""
                    aria-hidden="true"
                  />
                  <span class="settlement-treasure-value">
                    {props.viewModel.resourcesGained.gold}
                  </span>
                </div>
              </section>
            </div>

            {/* ── Spine ───────────────────────────────────── */}
            <div class="settlement-spine" aria-hidden="true" />

            {/* ── Right page ──────────────────────────────── */}
            <div class="settlement-page settlement-page--right">
              <section class="settlement-section">
                <h2 class="settlement-section-title">收集的传家宝</h2>
                <div
                  class="settlement-heirloom-row"
                  data-blocker="heirloom-counts-not-wired"
                  title="Heirloom counts are placeholders; runtime does not yet expose heirloom totals."
                >
                  <div class="settlement-heirloom">
                    <div class="settlement-heirloom-icon settlement-heirloom-icon--crest" />
                    <span class="settlement-heirloom-value">0</span>
                  </div>
                  <div class="settlement-heirloom">
                    <div class="settlement-heirloom-icon settlement-heirloom-icon--deed" />
                    <span class="settlement-heirloom-value">0</span>
                  </div>
                  <div class="settlement-heirloom">
                    <div class="settlement-heirloom-icon settlement-heirloom-icon--portrait" />
                    <span class="settlement-heirloom-value">0</span>
                  </div>
                </div>
              </section>

              <section class="settlement-section settlement-heroes-section">
                <h2 class="settlement-section-title">英雄状态</h2>
                <div class="settlement-hero-list">
                  <For each={props.viewModel.heroOutcomes}>
                    {(hero) => {
                      const portraitUrl = resolveHeroPortrait({
                        heroId: hero.heroId,
                        classLabel: hero.classLabel,
                      });
                      return (
                        <div
                          class="settlement-hero-row"
                          data-hero-id={hero.heroId}
                          data-hero-status={hero.status}
                        >
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
                          <div class="settlement-hero-info">
                            <div class="settlement-hero-name">{hero.heroName}</div>
                            <div class="settlement-hero-orbs">
                              <span
                                class={heroOrbClass(hero.hpChange, "hp")}
                                title={`HP ${hero.hpChange}`}
                                aria-label={`HP change ${hero.hpChange}`}
                              />
                              <span
                                class={heroOrbClass(hero.stressChange, "stress")}
                                title={`Stress ${hero.stressChange}`}
                                aria-label={`Stress change ${hero.stressChange}`}
                              />
                            </div>
                          </div>
                        </div>
                      );
                    }}
                  </For>
                </div>
              </section>
            </div>
          </div>
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left" />
        <div class="expedition-controls-right">
          <button
            class="action-secondary"
            onClick={props.onReturnToTown}
            aria-label="Return to Town"
          >
            返回城镇
          </button>
          <button
            class="action-primary launch-primary"
            onClick={props.onContinue}
            disabled={!props.viewModel.isContinueAvailable}
            aria-label="Proceed to Return"
          >
            {props.viewModel.isContinueAvailable ? "继续" : "等待中"}
          </button>
        </div>
      </footer>
    </div>
  );
};
