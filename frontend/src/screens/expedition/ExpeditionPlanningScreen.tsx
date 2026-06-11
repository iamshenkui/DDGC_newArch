import { For, Show, type Component } from "solid-js";

import type { ExpeditionPlanningViewModel, ExpeditionPlanningHeroSlot } from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface ExpeditionPlanningScreenProps {
  viewModel: ExpeditionPlanningViewModel;
  onSelectPlane: (planeId: string) => void;
  onToggleHero: (heroId: string) => void;
  onProceedToProvisioning: () => void;
  onReturnToTown: () => void;
}

function parseHp(hp: string): { current: number; max: number } {
  const parts = hp.split("/");
  if (parts.length === 2) {
    return { current: Number(parts[0].trim()), max: Number(parts[1].trim()) };
  }
  return { current: 0, max: 1 };
}

function healthPercent(hp: string): number {
  const { current, max } = parseHp(hp);
  if (max <= 0) return 0;
  return Math.round((current / max) * 100);
}

function healthBarColor(hp: string): string {
  const pct = healthPercent(hp);
  if (pct >= 80) return "#5bbd6e";
  if (pct >= 40) return "#e8a838";
  return "#ea7767";
}

function stressPercent(stress: string, maxStress: string): number {
  const s = Number(stress);
  const m = Number(maxStress || 200);
  return Math.min(Math.round((s / m) * 100), 100);
}

function stressBarColor(stress: string): string {
  const s = Number(stress);
  if (s <= 20) return "#5bbd6e";
  if (s <= 40) return "#e8a838";
  return "#ea7767";
}

/**
 * Expedition planning screen — 位面探索 (Plane Exploration).
 *
 * Dungeon/plane selection surface that mirrors the original Unity UI_Quest hierarchy:
 *   Assets/Prefabs/UI/QuestWindow.prefab (estimated)
 *   UI_Quest/SelectedQuestPanel — dungeon info, party slots, rewards
 *   UI_Quest/RaidPartyPanel — party assignment
 *   UI_Quest/Dungeons — BaiHu, ZhuQue, XuanWu, QingLong + locked states
 *
 * Source hierarchy: UI_Quest/SelectedQuestPanel + RaidPartyPanel + Dungeons
 */
export const ExpeditionPlanningScreen: Component<ExpeditionPlanningScreenProps> = (
  props
) => {
  const selectedPlane = () =>
    props.viewModel.planes.find((p) => p.id === props.viewModel.selectedPlaneId) ??
    props.viewModel.planes[0];

  const filledSlots = () =>
    props.viewModel.partySlots.filter((s) => s !== null).length;

  const isFull = () => filledSlots() >= props.viewModel.maxPartySize;

  const assignedHeroIds = () =>
    new Set(props.viewModel.partySlots.filter((s): s is ExpeditionPlanningHeroSlot => s !== null).map((s) => s.heroId));

  const availableRoster = () =>
    props.viewModel.roster.filter((h) => !assignedHeroIds().has(h.heroId));

  const planeStatusClass = (plane: typeof props.viewModel.planes[number]) => {
    if (plane.isLocked) return "plane-card plane-card--locked";
    if (plane.id === props.viewModel.selectedPlaneId) return "plane-card plane-card--selected";
    return "plane-card";
  };

  const difficultyPips = (count: number, max = 5) => {
    const pips: Array<{ filled: boolean }> = [];
    for (let i = 0; i < max; i++) {
      pips.push({ filled: i < count });
    }
    return pips;
  };

  return (
    <div
      class="expedition-viewport"
      data-source-scene="UI_Quest/SelectedQuestPanel"
      data-source-prefab="Assets/Prefabs/UI/QuestWindow.prefab"
      data-testid="expedition-planning-screen"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">位面探索</span>
          <h1 class="expedition-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class="hud-pill hud-pill-accent">
            {props.viewModel.campaignName}
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
            队伍: {filledSlots()}/{props.viewModel.maxPartySize}
          </span>
          <span class="hud-pill gold-pill">
            费用: {props.viewModel.provisionCost}
          </span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="expedition-content expedition-planning-content">
          {/* Plane selection strip — horizontal scroll of available dungeons */}
          <section
            class="plane-selection-strip"
            data-source-component="Dungeons"
            data-source-hierarchy="UI_Quest/Dungeons"
            data-testid="plane-selection-strip"
          >
            <div class="plane-strip-header">
              <span class="plane-strip-title">可选位面</span>
              <span class="plane-strip-hint">
                选择一个位面进行探索。锁定位面需要战役进度解锁。
              </span>
            </div>
            <div class="plane-strip-scroll">
              <For each={props.viewModel.planes}>
                {(plane) => (
                  <button
                    class={planeStatusClass(plane)}
                    onClick={() => {
                      if (!plane.isLocked) props.onSelectPlane(plane.id);
                    }}
                    disabled={plane.isLocked}
                    data-plane-id={plane.id}
                    data-locked={plane.isLocked}
                    data-testid={`plane-card-${plane.id}`}
                  >
                    <div
                      class="plane-card-orb"
                      style={{ "background-color": plane.themeColor }}
                      aria-hidden="true"
                    />
                    <span class="plane-card-name">{plane.name}</span>
                    <div class="plane-card-difficulty">
                      <For each={difficultyPips(plane.difficultyPips)}>
                        {(pip) => (
                          <span
                            class={`difficulty-pip${pip.filled ? " difficulty-pip--filled" : ""}`}
                            aria-hidden="true"
                          />
                        )}
                      </For>
                    </div>
                    {plane.isLocked && (
                      <span class="plane-card-lock">
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                          <rect x="3" y="11" width="18" height="11" rx="2" ry="2" />
                          <path d="M7 11V7a5 5 0 0 1 10 0v4" />
                        </svg>
                        {plane.lockReason}
                      </span>
                    )}
                  </button>
                )}
              </For>
            </div>
          </section>

          {/* Two-column layout: selected plane details + party panel */}
          <div class="expedition-planning-panels">
            {/* Selected plane details */}
            <section
              class="details-overlay expedition-planning-details"
              data-source-component="SelectedQuestPanel"
              data-testid="plane-details-panel"
            >
              <header class="details-overlay-header">
                <span class="details-overlay-frame-rule" aria-hidden="true" />
                <h2 class="details-overlay-title">位面详情</h2>
                <span class="details-overlay-frame-rule" aria-hidden="true" />
              </header>

              <div class="plane-detail-header">
                <div
                  class="plane-detail-orb"
                  style={{ "background-color": selectedPlane().themeColor }}
                  aria-hidden="true"
                />
                <div class="plane-detail-header-text">
                  <h3 class="plane-detail-name">{selectedPlane().name}</h3>
                  <span class="plane-detail-difficulty">
                    难度: {selectedPlane().difficulty}
                  </span>
                </div>
              </div>

              <p class="plane-detail-description">{selectedPlane().description}</p>

              <div class="details-overlay-row">
                <span class="details-overlay-label">预计时长</span>
                <span class="details-overlay-value">{selectedPlane().estimatedDuration}</span>
              </div>

              {selectedPlane().objectives.length > 0 && (
                <div class="details-overlay-objectives">
                  <div class="details-overlay-objectives-title">目标</div>
                  <For each={selectedPlane().objectives}>
                    {(obj) => (
                      <div class="details-overlay-objective">
                        <span class="objective-marker" aria-hidden="true">
                          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="5 12 10 17 19 7" />
                          </svg>
                        </span>
                        <span class="objective-text">{obj}</span>
                      </div>
                    )}
                  </For>
                </div>
              )}

              {selectedPlane().rewards.length > 0 && (
                <div class="plane-detail-rewards">
                  <div class="plane-detail-rewards-title">可能奖励</div>
                  <div class="plane-detail-reward-list">
                    <For each={selectedPlane().rewards}>
                      {(reward) => (
                        <span class="plane-detail-reward-tag">{reward}</span>
                      )}
                    </For>
                  </div>
                </div>
              )}
            </section>

            {/* Party assignment panel */}
            <section
              class="details-overlay expedition-party-panel"
              data-source-component="RaidPartyPanel"
              data-source-hierarchy="UI_Quest/RaidPartyPanel"
              data-testid="party-panel"
            >
              <header class="details-overlay-header">
                <span class="details-overlay-frame-rule" aria-hidden="true" />
                <h2 class="details-overlay-title">远征队伍</h2>
                <span class="details-overlay-frame-rule" aria-hidden="true" />
              </header>

              <div class="party-formation">
                <For each={props.viewModel.partySlots}>
                  {(hero, index) => {
                    if (hero === null) {
                      return (
                        <div
                          class="party-slot party-slot--empty"
                          data-slot-index={index()}
                          data-testid={`party-slot-empty-${index()}`}
                        >
                          <span class="party-slot-empty-marker" aria-hidden="true">
                            <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                              <line x1="12" y1="5" x2="12" y2="19" />
                              <line x1="5" y1="12" x2="19" y2="12" />
                            </svg>
                          </span>
                          <span class="party-slot-empty-hint">空位</span>
                          <span class="party-slot-empty-sub">点击下方英雄编入队伍</span>
                        </div>
                      );
                    }

                    const portraitUrl = resolveHeroPortrait({
                      heroId: hero.heroId,
                      classLabel: hero.classLabel
                    });

                    return (
                      <button
                        type="button"
                        class="party-slot party-slot--selected"
                        data-hero-id={hero.heroId}
                        data-testid={`party-slot-hero-${hero.heroId}`}
                        onClick={() => props.onToggleHero(hero.heroId)}
                        title={`点击移除 ${hero.heroName}`}
                      >
                        <span class="party-slot-level">Lv{hero.level}</span>
                        <div
                          class={`party-slot-portrait${portraitUrl ? " party-slot-portrait--image" : " party-slot-portrait--fallback"}`}
                        >
                          {portraitUrl ? (
                            <img
                              class="party-slot-portrait-image"
                              src={portraitUrl}
                              alt=""
                              aria-hidden="true"
                            />
                          ) : (
                            <span
                              class="party-slot-initial"
                              data-blocker="BLOCKER-002: portrait sprite not extracted for this hero family"
                              data-class-label={hero.classLabel}
                            >
                              {hero.classLabel[0]}
                            </span>
                          )}
                        </div>
                        <span class="party-slot-name">{hero.heroName}</span>
                        <span class="party-slot-class">{hero.classLabel}</span>
                        <div class="party-slot-bars">
                          <div class="party-slot-bar-row">
                            <div class="party-slot-bar-label">生命</div>
                            <div class="party-slot-bar-track">
                              <div
                                class="party-slot-bar-fill"
                                style={{
                                  width: `${healthPercent(hero.hp)}%`,
                                  background: healthBarColor(hero.hp),
                                }}
                              />
                            </div>
                          </div>
                          <div class="party-slot-bar-row">
                            <div class="party-slot-bar-label">压力</div>
                            <div class="party-slot-bar-track">
                              <div
                                class="party-slot-bar-fill"
                                style={{
                                  width: `${stressPercent(hero.stress, hero.maxStress ?? "200")}%`,
                                  background: stressBarColor(hero.stress),
                                }}
                              />
                            </div>
                          </div>
                        </div>
                        {(hero.isWounded || hero.isAfflicted) && (
                          <span
                            class={`party-slot-status ${hero.isAfflicted ? "party-slot-status--afflicted" : "party-slot-status--wounded"}`}
                          >
                            {hero.isAfflicted ? "受折磨" : "受伤"}
                          </span>
                        )}
                      </button>
                    );
                  }}
                </For>
              </div>

              <div class="party-panel-status">
                <span
                  class={`expedition-status-pill ${
                    props.viewModel.isReadyToProvision
                      ? "expedition-status-pill--ready"
                      : "expedition-status-pill--neutral"
                  }`}
                  data-testid="party-panel-status"
                >
                  <span class="expedition-status-pill-dot" aria-hidden="true" />
                  {props.viewModel.isReadyToProvision
                    ? "队伍已就绪，可前往战前补给"
                    : `${filledSlots()} / ${props.viewModel.maxPartySize} 名英雄已编入队伍`}
                </span>
              </div>
            </section>
          </div>
        </div>
      </div>

      {/* ── Available heroes roster strip ──────────────────── */}
      <Show when={availableRoster().length > 0}>
        <section
          class="provisioning-roster-strip"
          data-source-hierarchy="UI_Quest/RosterPanel"
          data-testid="expedition-roster-strip"
        >
          <div class="provisioning-roster-header">
            <span class="provisioning-roster-label">可用英雄</span>
            <span class="provisioning-roster-count">
              {availableRoster().length} 名待命中
            </span>
          </div>
          <div class="roster-scroll provisioning-roster-scroll">
            <For each={availableRoster()}>
              {(hero) => {
                const portraitUrl = resolveHeroPortrait({
                  heroId: hero.heroId,
                  classLabel: hero.classLabel
                });

                return (
                  <button
                    type="button"
                    class="roster-hero provisioning-roster-hero"
                    onClick={() => props.onToggleHero(hero.heroId)}
                    disabled={isFull()}
                    title={isFull() ? "队伍已满" : `添加 ${hero.heroName}`}
                    data-testid={`roster-hero-${hero.heroId}`}
                  >
                    <div class="roster-hero-portrait">
                      <div class="roster-portrait-frame" />
                      <div
                        class={`roster-portrait-avatar${portraitUrl ? " roster-portrait-avatar--image" : ""}`}
                      >
                        {portraitUrl ? (
                          <img
                            class="roster-portrait-image"
                            src={portraitUrl}
                            alt=""
                            aria-hidden="true"
                          />
                        ) : (
                          <span
                            class="roster-portrait-letter"
                            data-blocker="BLOCKER-002: portrait sprite not extracted for this hero family"
                            data-class-label={hero.classLabel}
                          >
                            {hero.classLabel[0]}
                          </span>
                        )}
                      </div>
                      <span class="roster-portrait-level">Lv{hero.level}</span>
                      {(hero.isWounded || hero.isAfflicted) && (
                        <span class="roster-portrait-status">
                          {hero.isAfflicted ? "A" : "伤"}
                        </span>
                      )}
                    </div>
                    <div class="roster-hero-info">
                      <span class="roster-hero-name">{hero.heroName}</span>
                      <span class="roster-hero-class">{hero.classLabel}</span>
                    </div>
                    <div class="roster-hero-bars">
                      <div class="roster-bar-row">
                        <span class="roster-bar-label">生命</span>
                        <span class="roster-bar-track">
                          <span
                            class="roster-bar-fill"
                            style={{
                              width: `${healthPercent(hero.hp)}%`,
                              background: healthBarColor(hero.hp),
                            }}
                          />
                        </span>
                      </div>
                      <div class="roster-bar-row">
                        <span class="roster-bar-label">压力</span>
                        <span class="roster-bar-track">
                          <span
                            class="roster-bar-fill"
                            style={{
                              width: `${stressPercent(hero.stress, hero.maxStress ?? "200")}%`,
                              background: stressBarColor(hero.stress),
                            }}
                          />
                        </span>
                      </div>
                    </div>
                  </button>
                );
              }}
            </For>
          </div>
        </section>
      </Show>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left">
          <span class="expedition-status-pill expedition-status-pill--neutral">
            <span class="expedition-status-pill-dot" aria-hidden="true" />
            选择位面并编入队伍
          </span>
        </div>
        <div class="expedition-controls-right">
          <button
            class="action-secondary"
            onClick={props.onReturnToTown}
            data-testid="expedition-btn-return"
          >
            返回城镇
          </button>
          <button
            class="action-primary launch-primary"
            onClick={props.onProceedToProvisioning}
            disabled={!props.viewModel.isReadyToProvision}
            data-testid="expedition-btn-proceed"
          >
            {props.viewModel.isReadyToProvision
              ? "前往战前补给"
              : "先编入队伍"}
          </button>
        </div>
      </footer>
    </div>
  );
};
