import { For, Show, createSignal, type Component } from "solid-js";

import type { ProvisioningViewModel, ProvisioningHeroSummary } from "../../bridge/contractTypes";
import { resolveChromeAsset, resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface ProvisioningScreenProps {
  viewModel: ProvisioningViewModel;
  onToggleHeroSelection: (heroId: string) => void;
  onConfirmProvisioning: () => void;
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
 * Provisioning screen — 战前补给 (pre-battle supply preparation).
 *
 * Mirrors the reference image layout:
 *   Left panel  → focused hero portrait + vitals + action buttons
 *   Right panel → supply item grid + detail area + resource strip
 *
 * Reference: reference/ref_image/跨际元契约/3位面探索/位面探索-战前补给.png
 *
 * Original asset wiring:
 *   gold       — extracted, served from /original/chrome/gold.png (UIR-005B).
 *   portraits  — extracted hunter family served from /original/heroes/ (UIR-005B).
 *                Wired via resolveHeroPortrait on focused hero and party strip.
 *   supply     — Assets/Resources/Sprites/inv_supply+rattle_drum.png
 *                GUID e401bf9b9275ede4aa2ff50d13cc6207.
 *                Not extracted (BLOCKER-001).
 *   supply items — wired via ProvisioningViewModel.supplies (replay fixture data).
 *                  When supplies are absent the grid renders an explicit blocked state.
 */
export const ProvisioningScreen: Component<ProvisioningScreenProps> = (props) => {
  const [focusedHeroId, setFocusedHeroId] = createSignal<string | null>(null);

  const selectedCount = () =>
    props.viewModel.party.filter((h) => h.isSelected).length;

  const selectedHeroes = () =>
    props.viewModel.party.filter((h) => h.isSelected);

  const unselectedHeroes = () =>
    props.viewModel.party.filter((h) => !h.isSelected);

  const focusedHero = (): ProvisioningHeroSummary | null => {
    const selected = selectedHeroes();
    if (selected.length === 0) return null;
    const fid = focusedHeroId();
    if (fid) {
      const found = selected.find((h) => h.id === fid);
      if (found) return found;
    }
    return selected[0];
  };

  const isFull = () => selectedCount() >= props.viewModel.maxPartySize;

  const partyStatusLabel = () => {
    if (selectedCount() === 0) return "尚未选择英雄";
    if (selectedCount() < props.viewModel.maxPartySize)
      return `已选择 ${selectedCount()} / ${props.viewModel.maxPartySize} 名英雄`;
    return "队伍已满，可以出发";
  };

  return (
    <div
      class="expedition-viewport"
      data-source-scene="UI_Provision/ProvisionShop"
      data-source-prefab="Assets/Prefabs/UI/ProvisionShop.prefab"
      data-testid="provisioning-screen"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">{props.viewModel.campaignName}</span>
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
          <span class="hud-pill hud-pill-accent">
            {props.viewModel.expeditionLabel}
          </span>
          <span
            class="hud-pill supply-pill"
            data-source-component="ProvisionInventoryEntry"
            data-asset-path="Assets/Resources/Sprites/inv_supply+rattle_drum.png"
            data-guid="e401bf9b9275ede4aa2ff50d13cc6207"
            data-extraction-status="not-extracted"
            data-blocker="BLOCKER-001: Original Unity supply sprite not in repository"
          >
            <span class="supply-icon-fallback" aria-hidden="true" />
            补给: {props.viewModel.supplyLevel}
          </span>
          <span
            class="hud-pill gold-pill"
            data-source-component="ProvisionCostEntry"
            data-asset-path="Assets/Resources/Sprites/gold.png"
            data-extraction-status="staged"
          >
            <img
              class="gold-icon-image"
              src={resolveChromeAsset("goldIcon")}
              alt=""
              aria-hidden="true"
            />
            费用: {props.viewModel.provisionCost}
          </span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="expedition-content provisioning-content">
          {/* Two-panel layout matching reference image */}
          <div class="provisioning-layout">
            {/* ── Left panel: focused hero ─────────────────── */}
            <section class="provisioning-left-panel" data-testid="provisioning-left-panel">
              <Show
                when={focusedHero()}
                fallback={
                  <div class="provisioning-no-hero">
                    <span class="provisioning-no-hero-icon" aria-hidden="true">
                      <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                        <circle cx="9" cy="8" r="3.5" />
                        <path d="M2 20c0-3.6 3.2-6 7-6s7 2.4 7 6" />
                        <circle cx="17" cy="9" r="2.5" />
                        <path d="M22 19c0-2.4-1.8-4-4-4" />
                      </svg>
                    </span>
                    <span class="provisioning-no-hero-text">请先选择英雄</span>
                    <span class="provisioning-no-hero-sub">点击下方可用英雄加入队伍</span>
                  </div>
                }
              >
                {(hero) => {
                  const h = hero();
                  const portraitUrl = resolveHeroPortrait({
                    heroId: h.id,
                    classLabel: h.classLabel
                  });
                  const hpInfo = parseHp(h.hp);
                  return (
                    <div class="provisioning-hero-focus">
                      {/* Portrait */}
                      <div
                        class={`provisioning-hero-portrait${portraitUrl ? " provisioning-hero-portrait--image" : " provisioning-hero-portrait--fallback"}`}
                        data-testid="focused-hero-portrait"
                      >
                        {portraitUrl ? (
                          <img
                            class="provisioning-hero-portrait-image"
                            src={portraitUrl}
                            alt={h.name}
                            data-testid="focused-hero-portrait-img"
                          />
                        ) : (
                          <span
                            class="provisioning-hero-portrait-letter"
                            data-blocker="BLOCKER-002: portrait sprite not extracted for this hero family"
                            data-class-label={h.classLabel}
                          >
                            {h.classLabel[0]}
                          </span>
                        )}
                      </div>

                      {/* Name & class */}
                      <div class="provisioning-hero-name" data-testid="focused-hero-name">
                        {h.name}
                      </div>
                      <div class="provisioning-hero-class">
                        {h.classLabel} · Lv{h.level}
                      </div>

                      {/* Status badges */}
                      <div class="provisioning-hero-badges">
                        <Show when={h.isWounded}>
                          <span class="provisioning-hero-badge provisioning-hero-badge--wounded">受伤</span>
                        </Show>
                        <Show when={h.isAfflicted}>
                          <span class="provisioning-hero-badge provisioning-hero-badge--afflicted">受折磨</span>
                        </Show>
                        <Show when={!h.isWounded && !h.isAfflicted}>
                          <span class="provisioning-hero-badge provisioning-hero-badge--healthy">健康</span>
                        </Show>
                      </div>

                      {/* Vitals bars */}
                      <div class="provisioning-hero-vitals">
                        <div class="provisioning-vital-row">
                          <span class="provisioning-vital-label">生命</span>
                          <span class="provisioning-vital-track">
                            <span
                              class="provisioning-vital-fill"
                              style={{
                                width: `${healthPercent(h.hp)}%`,
                                background: healthBarColor(h.hp),
                              }}
                            />
                          </span>
                          <span class="provisioning-vital-value">
                            {hpInfo.current}/{hpInfo.max}
                          </span>
                        </div>
                        <div class="provisioning-vital-row">
                          <span class="provisioning-vital-label">压力</span>
                          <span class="provisioning-vital-track">
                            <span
                              class="provisioning-vital-fill"
                              style={{
                                width: `${stressPercent(h.stress, h.maxStress)}%`,
                                background: stressBarColor(h.stress),
                              }}
                            />
                          </span>
                          <span class="provisioning-vital-value">
                            {h.stress}/{h.maxStress}
                          </span>
                        </div>
                      </div>

                      {/* Action buttons */}
                      <div class="provisioning-hero-actions">
                        <button
                          class="provisioning-btn provisioning-btn--secondary"
                          onClick={props.onReturnToTown}
                          data-testid="btn-return-town"
                        >
                          返回城镇
                        </button>
                        <button
                          class="provisioning-btn provisioning-btn--primary"
                          onClick={props.onConfirmProvisioning}
                          disabled={!props.viewModel.isReadyToLaunch}
                          data-testid="btn-start-adventure"
                        >
                          {props.viewModel.isReadyToLaunch ? "开始冒险" : "准备中"}
                        </button>
                      </div>
                    </div>
                  );
                }}
              </Show>
            </section>

            {/* ── Right panel: supply grid ─────────────────── */}
            <section class="provisioning-right-panel" data-testid="provisioning-right-panel">
              {/* Supply grid title */}
              <div class="provisioning-supply-header">
                <span class="provisioning-supply-title">补给物资</span>
                <span class="provisioning-supply-subtitle">{props.viewModel.expeditionSummary}</span>
              </div>

              {/* Supply item grid — wired from ProvisioningViewModel.supplies */}
              <div
                class="provisioning-supply-grid"
                data-testid="supply-grid"
              >
                <Show
                  when={props.viewModel.supplies && props.viewModel.supplies.length > 0}
                  fallback={
                    <div
                      class="provisioning-supply-blocked"
                      data-blocker="BLOCKER-003: Supply item inventory not provided by runtime bridge"
                      data-testid="supply-grid-blocked"
                    >
                      <span class="provisioning-supply-blocked-icon" aria-hidden="true">📦</span>
                      <span class="provisioning-supply-blocked-text">补给数据未连接</span>
                      <span class="provisioning-supply-blocked-sub">等待运行时提供补给清单</span>
                    </div>
                  }
                >
                  <For each={props.viewModel.supplies}>
                    {(item) => (
                      <div class="provisioning-supply-item" data-testid={`supply-item-${item.id}`}>
                        <span class="provisioning-supply-item-icon" aria-hidden="true">
                          {item.icon}
                        </span>
                        <span class="provisioning-supply-item-name">{item.name}</span>
                        <span class="provisioning-supply-item-qty">×{item.qty}</span>
                      </div>
                    )}
                  </For>
                </Show>
              </div>

              {/* Detail / equipment area */}
              <div class="provisioning-detail-area" data-testid="provisioning-detail-area">
                <div class="provisioning-detail-placeholder">
                  <span class="provisioning-detail-placeholder-icon" aria-hidden="true">📋</span>
                  <span class="provisioning-detail-placeholder-text">选择补给查看详情</span>
                  <span class="provisioning-detail-placeholder-sub">
                    补给品将在远征中自动分配给队伍
                  </span>
                </div>
              </div>

              {/* Resource strip */}
              <div class="provisioning-resource-strip" data-testid="resource-strip">
                <div class="provisioning-resource-item">
                  <span class="provisioning-resource-label">队伍</span>
                  <span class="provisioning-resource-value">{selectedCount()}/{props.viewModel.maxPartySize}</span>
                </div>
                <div class="provisioning-resource-item">
                  <span class="provisioning-resource-label">补给等级</span>
                  <span class="provisioning-resource-value">{props.viewModel.supplyLevel}</span>
                </div>
                <div class="provisioning-resource-item">
                  <span class="provisioning-resource-label">费用</span>
                  <span class="provisioning-resource-value provisioning-resource-value--gold">
                    <img
                      class="gold-icon-image"
                      src={resolveChromeAsset("goldIcon")}
                      alt=""
                      aria-hidden="true"
                    />
                    {props.viewModel.provisionCost}
                  </span>
                </div>
              </div>
            </section>
          </div>

          {/* ── Selected party strip ───────────────────────── */}
          <Show when={selectedHeroes().length > 0}>
            <div class="provisioning-party-strip" data-testid="party-strip">
              <span class="provisioning-party-strip-label">当前队伍</span>
              <div class="provisioning-party-strip-heroes">
                <For each={selectedHeroes()}>
                  {(hero) => {
                    const portraitUrl = resolveHeroPortrait({
                      heroId: hero.id,
                      classLabel: hero.classLabel
                    });
                    return (
                      <div
                        class={`provisioning-party-hero${focusedHero()?.id === hero.id ? " provisioning-party-hero--focused" : ""}`}
                        onClick={() => {
                          setFocusedHeroId(hero.id);
                        }}
                        onKeyDown={(e) => {
                          if (e.key === "Enter" || e.key === " ") {
                            e.preventDefault();
                            setFocusedHeroId(hero.id);
                          }
                        }}
                        title={`${hero.name} — 点击切换焦点`}
                        data-testid={`party-hero-${hero.id}`}
                        tabIndex={0}
                        role="button"
                        aria-label={`${hero.name} — 点击切换焦点`}
                      >
                        <div
                          class={`provisioning-party-hero-portrait${portraitUrl ? " provisioning-party-hero-portrait--image" : ""}`}
                        >
                          {portraitUrl ? (
                            <img
                              class="provisioning-party-hero-portrait-img"
                              src={portraitUrl}
                              alt=""
                              aria-hidden="true"
                            />
                          ) : (
                            <span class="provisioning-party-hero-portrait-letter">
                              {hero.classLabel[0]}
                            </span>
                          )}
                        </div>
                        <span class="provisioning-party-hero-name">{hero.name}</span>
                        {/* Explicit remove control — separates focus from removal */}
                        <button
                          type="button"
                          class="provisioning-party-hero-remove"
                          onClick={(e) => {
                            e.stopPropagation();
                            props.onToggleHeroSelection(hero.id);
                          }}
                          title={`移除 ${hero.name}`}
                          data-testid={`party-hero-remove-${hero.id}`}
                          aria-label={`移除 ${hero.name}`}
                        >
                          ×
                        </button>
                      </div>
                    );
                  }}
                </For>
              </div>
            </div>
          </Show>
        </div>
      </div>

      {/* ── Available heroes roster strip ──────────────────── */}
      <Show when={unselectedHeroes().length > 0}>
        <section
          class="provisioning-roster-strip"
          data-source-hierarchy="UI_Provision/RosterPanel"
        >
          <div class="provisioning-roster-header">
            <span class="provisioning-roster-label">可用英雄</span>
            <span class="provisioning-roster-count">
              {unselectedHeroes().length} 名待命中
            </span>
          </div>
          <div class="roster-scroll provisioning-roster-scroll">
            <For each={unselectedHeroes()}>
              {(hero) => {
                const portraitUrl = resolveHeroPortrait({
                  heroId: hero.id,
                  classLabel: hero.classLabel
                });

                return (
                  <button
                    class="roster-hero provisioning-roster-hero"
                    onClick={() => {
                      props.onToggleHeroSelection(hero.id);
                      setFocusedHeroId(hero.id);
                    }}
                    disabled={!hero.isSelected && isFull()}
                    title={isFull() ? "队伍已满" : `添加 ${hero.name}`}
                    data-testid={`roster-hero-${hero.id}`}
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
                      <span class="roster-hero-name">{hero.name}</span>
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
        </section>
      </Show>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left">
          <span
            class={`expedition-status-pill ${
              selectedCount() === 0
                ? "expedition-status-pill--neutral"
                : selectedCount() < props.viewModel.maxPartySize
                  ? "expedition-status-pill--wounded"
                  : "expedition-status-pill--ready"
            }`}
            data-state={
              selectedCount() === 0
                ? "incomplete"
                : selectedCount() < props.viewModel.maxPartySize
                  ? "partial"
                  : "ready"
            }
            data-testid="party-status-pill"
          >
            <span class="expedition-status-pill-dot" aria-hidden="true" />
            {partyStatusLabel()}
          </span>
        </div>
        <div class="expedition-controls-right">
          <button class="action-secondary" onClick={props.onReturnToTown} data-testid="footer-btn-return">
            返回城镇
          </button>
          <button
            class="action-primary launch-primary"
            onClick={props.onConfirmProvisioning}
            disabled={!props.viewModel.isReadyToLaunch}
            data-testid="footer-btn-launch"
          >
            {props.viewModel.isReadyToLaunch
              ? "确认出发"
              : "准备中"}
          </button>
        </div>
      </footer>
    </div>
  );
};
