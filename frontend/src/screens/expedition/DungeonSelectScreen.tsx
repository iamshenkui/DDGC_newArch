import { For, type Component } from "solid-js";

import type { DungeonSelectViewModel } from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface DungeonSelectScreenProps {
  viewModel: DungeonSelectViewModel;
  onSelectDungeon: (dungeonId: string) => void;
  onToggleHeroSelection: (heroId: string) => void;
  onConfirmSelection: () => void;
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
 * Dungeon Select screen — 位面探索-副本选择人物
 *
 * Plane exploration dungeon selection with character assignment.
 * Mirrors the original Unity scene: UI_Expedition/DungeonSelectWindow
 *
 * Layout:
 *   Left panel — dungeon list with difficulty/rewards/cost
 *   Right panel — hero roster for party assignment
 *   Bottom controls — return to town / proceed to provisioning
 */
export const DungeonSelectScreen: Component<DungeonSelectScreenProps> = (props) => {
  const selectedCount = () =>
    props.viewModel.party.filter((h) => h.isSelected).length;

  const selectedHeroes = () =>
    props.viewModel.party.filter((h) => h.isSelected);

  const unselectedHeroes = () =>
    props.viewModel.party.filter((h) => !h.isSelected);

  const isFull = () => selectedCount() >= props.viewModel.maxPartySize;

  const selectedDungeon = () =>
    props.viewModel.dungeons.find((d) => d.id === props.viewModel.selectedDungeonId);

  const partyStatusLabel = () => {
    if (selectedCount() === 0) return "未选择英雄";
    if (isFull()) return "队伍已满";
    return `已选择 ${selectedCount()}/${props.viewModel.maxPartySize} 英雄`;
  };

  return (
    <div
      class="expedition-viewport"
      data-source-scene="UI_Expedition/DungeonSelectWindow"
      data-source-prefab="Assets/Prefabs/UI/DungeonSelectWindow.prefab"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">位面探索</span>
          <h1 class="expedition-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class="hud-pill hud-pill--with-icon">
            <span class="hud-pill-icon" aria-hidden="true">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7">
                <circle cx="9" cy="8" r="3.5" />
                <path d="M2 20c0-3.6 3.2-6 7-6s7 2.4 7 6" />
                <circle cx="17" cy="9" r="2.5" />
                <path d="M22 19c0-2.4-1.8-4-4-4" />
              </svg>
            </span>
            队伍: {selectedCount()}/{props.viewModel.maxPartySize}
          </span>
          {selectedDungeon() && (
            <span class="hud-pill hud-pill-accent">
              {selectedDungeon()!.name}
            </span>
          )}
        </span>
        <span class="expedition-hud-right">
          <span
            class={`expedition-status-pill ${
              isFull()
                ? "expedition-status-pill--ready"
                : selectedCount() > 0
                  ? "expedition-status-pill--wounded"
                  : "expedition-status-pill--neutral"
            }`}
          >
            <span class="expedition-status-pill-dot" aria-hidden="true" />
            {partyStatusLabel()}
          </span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="expedition-content dungeon-select-content">
          {/* ── Left: Dungeon List ───────────────────────── */}
          <section class="dungeon-list-panel">
            <header class="dungeon-list-header">
              <span class="dungeon-list-header-rule" aria-hidden="true" />
              <h2 class="dungeon-list-title">副本列表</h2>
              <span class="dungeon-list-header-rule" aria-hidden="true" />
            </header>
            <div class="dungeon-list">
              <For each={props.viewModel.dungeons}>
                {(dungeon) => {
                  const isSelected = dungeon.id === props.viewModel.selectedDungeonId;
                  return (
                    <button
                      class={`dungeon-card ${isSelected ? "dungeon-card--selected" : ""} ${!dungeon.isAvailable ? "dungeon-card--locked" : ""}`}
                      onClick={() => dungeon.isAvailable && props.onSelectDungeon(dungeon.id)}
                      disabled={!dungeon.isAvailable}
                      data-dungeon-id={dungeon.id}
                    >
                      <div class="dungeon-card-header">
                        <span class="dungeon-card-name">{dungeon.name}</span>
                        {!dungeon.isAvailable && dungeon.lockReason && (
                          <span class="dungeon-card-lock">{dungeon.lockReason}</span>
                        )}
                      </div>
                      <div class="dungeon-card-body">
                        <span class="dungeon-card-difficulty">{dungeon.difficulty}</span>
                        <span class="dungeon-card-level">推荐 Lv.{dungeon.recommendedLevel}</span>
                      </div>
                      <div class="dungeon-card-footer">
                        <span class="dungeon-card-cost">{dungeon.provisionCost}</span>
                        <span class="dungeon-card-duration">{dungeon.estimatedDuration}</span>
                      </div>
                      {dungeon.rewards.length > 0 && (
                        <div class="dungeon-card-rewards">
                          <For each={dungeon.rewards}>
                            {(reward) => (
                              <span class="dungeon-reward-tag">{reward}</span>
                            )}
                          </For>
                        </div>
                      )}
                    </button>
                  );
                }}
              </For>
            </div>
          </section>

          {/* ── Right: Selected Dungeon Detail + Hero Roster ─ */}
          <section class="dungeon-detail-panel">
            {selectedDungeon() ? (
              <div class="dungeon-detail-active">
                <header class="dungeon-detail-header">
                  <h2 class="dungeon-detail-name">{selectedDungeon()!.name}</h2>
                  <p class="dungeon-detail-desc">{selectedDungeon()!.description}</p>
                </header>
                <div class="dungeon-detail-stats">
                  <div class="dungeon-stat-row">
                    <span class="dungeon-stat-label">难度</span>
                    <span class="dungeon-stat-value">{selectedDungeon()!.difficulty}</span>
                  </div>
                  <div class="dungeon-stat-row">
                    <span class="dungeon-stat-label">推荐等级</span>
                    <span class="dungeon-stat-value">Lv.{selectedDungeon()!.recommendedLevel}</span>
                  </div>
                  <div class="dungeon-stat-row">
                    <span class="dungeon-stat-label">预计时长</span>
                    <span class="dungeon-stat-value">{selectedDungeon()!.estimatedDuration}</span>
                  </div>
                  <div class="dungeon-stat-row">
                    <span class="dungeon-stat-label">补给等级</span>
                    <span class="dungeon-stat-value">{selectedDungeon()!.supplyLevel}</span>
                  </div>
                  <div class="dungeon-stat-row">
                    <span class="dungeon-stat-label">消耗</span>
                    <span class="dungeon-stat-value">{selectedDungeon()!.provisionCost}</span>
                  </div>
                </div>
                {selectedDungeon()!.rewards.length > 0 && (
                  <div class="dungeon-detail-rewards">
                    <div class="dungeon-rewards-title">可能奖励</div>
                    <div class="dungeon-rewards-list">
                      <For each={selectedDungeon()!.rewards}>
                        {(reward) => (
                          <span class="dungeon-reward-chip">{reward}</span>
                        )}
                      </For>
                    </div>
                  </div>
                )}
              </div>
            ) : (
              <div class="dungeon-detail-empty">
                <span class="dungeon-detail-empty-icon" aria-hidden="true">
                  <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <path d="M12 2L2 7l10 5 10-5-10-5z" />
                    <path d="M2 17l10 5 10-5" />
                    <path d="M2 12l10 5 10-5" />
                  </svg>
                </span>
                <span class="dungeon-detail-empty-text">请选择一个副本</span>
              </div>
            )}

            {/* ── Party Formation (selected heroes) ──────── */}
            {selectedCount() > 0 && (
              <div class="dungeon-select-party">
                <div class="dungeon-select-party-header">
                  <span class="dungeon-select-party-rule" aria-hidden="true" />
                  <span class="dungeon-select-party-title">出征队伍</span>
                  <span class="dungeon-select-party-rule" aria-hidden="true" />
                </div>
                <div class="dungeon-select-party-slots">
                  <For each={selectedHeroes()}>
                    {(hero) => {
                      const portraitUrl = resolveHeroPortrait({
                        heroId: hero.id,
                        classLabel: hero.classLabel
                      });
                      return (
                        <button
                          class="dungeon-party-slot"
                          onClick={() => props.onToggleHeroSelection(hero.id)}
                          title={`移除 ${hero.name}`}
                        >
                          <div
                            class={`dungeon-party-portrait${portraitUrl ? " dungeon-party-portrait--image" : " dungeon-party-portrait--fallback"}`}
                          >
                            {portraitUrl ? (
                              <img
                                class="dungeon-party-portrait-image"
                                src={portraitUrl}
                                alt=""
                                aria-hidden="true"
                              />
                            ) : (
                              <span class="dungeon-party-portrait-letter">
                                {hero.classLabel[0]}
                              </span>
                            )}
                          </div>
                          <span class="dungeon-party-name">{hero.name}</span>
                          <span class="dungeon-party-class">{hero.classLabel}</span>
                          <div class="dungeon-party-bars">
                            <div class="party-slot-bar-row">
                              <div class="party-slot-bar-label">HP</div>
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
                              <div class="party-slot-bar-label">ST</div>
                              <div class="party-slot-bar-track">
                                <div
                                  class="party-slot-bar-fill"
                                  style={{
                                    width: `${stressPercent(hero.stress, hero.maxStress)}%`,
                                    background: stressBarColor(hero.stress),
                                  }}
                                />
                              </div>
                            </div>
                          </div>
                        </button>
                      );
                    }}
                  </For>
                  {Array.from({ length: props.viewModel.maxPartySize - selectedCount() }).map((_, i) => (
                    <div class="dungeon-party-slot dungeon-party-slot--empty" data-slot-index={i}>
                      <span class="dungeon-party-empty-marker" aria-hidden="true">
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                          <line x1="12" y1="5" x2="12" y2="19" />
                          <line x1="5" y1="12" x2="19" y2="12" />
                        </svg>
                      </span>
                      <span class="dungeon-party-empty-hint">空位</span>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {/* ── Available Heroes ───────────────────────── */}
            <div class="dungeon-select-roster">
              <div class="dungeon-select-roster-header">
                <span class="dungeon-select-roster-label">可用英雄</span>
                <span class="dungeon-select-roster-count">
                  {unselectedHeroes().length} 待命
                </span>
              </div>
              <div class="roster-scroll dungeon-select-roster-scroll">
                <For each={unselectedHeroes()}>
                  {(hero) => {
                    const portraitUrl = resolveHeroPortrait({
                      heroId: hero.id,
                      classLabel: hero.classLabel
                    });
                    return (
                      <button
                        class="roster-hero dungeon-select-roster-hero"
                        onClick={() => props.onToggleHeroSelection(hero.id)}
                        disabled={isFull()}
                        title={isFull() ? "队伍已满" : `添加 ${hero.name}`}
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
                              <span class="roster-portrait-letter">
                                {hero.classLabel[0]}
                              </span>
                            )}
                          </div>
                          <span class="roster-portrait-level">Lv{hero.level}</span>
                          {(hero.isWounded || hero.isAfflicted) && (
                            <span class="roster-portrait-status">
                              {hero.isAfflicted ? "A" : "W"}
                            </span>
                          )}
                        </div>
                        <div class="roster-hero-info">
                          <span class="roster-hero-name">{hero.name}</span>
                          <span class="roster-hero-class">{hero.classLabel}</span>
                        </div>
                        <div class="roster-hero-bars">
                          <div class="roster-bar-row">
                            <span class="roster-bar-label">HP</span>
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
                            <span class="roster-bar-label">ST</span>
                            <span class="roster-bar-track">
                              <span
                                class="roster-bar-fill"
                                style={{
                                  width: `${stressPercent(hero.stress, hero.maxStress)}%`,
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
            </div>
          </section>
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left">
          <span class="expedition-status-pill expedition-status-pill--neutral">
            <span class="expedition-status-pill-dot" aria-hidden="true" />
            {props.viewModel.selectedDungeonId
              ? `已选择: ${selectedDungeon()?.name ?? ""}`
              : "请选择一个副本"}
          </span>
        </div>
        <div class="expedition-controls-right">
          <button class="action-secondary" onClick={props.onReturnToTown}>
            返回城镇
          </button>
          <button
            class="action-primary launch-primary"
            onClick={props.onConfirmSelection}
            disabled={!props.viewModel.isReadyToProceed}
          >
            {props.viewModel.isReadyToProceed
              ? "确认出征"
              : "选择副本和队伍"}
          </button>
        </div>
      </footer>
    </div>
  );
};
