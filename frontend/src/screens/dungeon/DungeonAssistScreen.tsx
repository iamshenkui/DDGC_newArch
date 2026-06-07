import { For, Show, createMemo, type Component } from "solid-js";

import type { DungeonAssistViewModel } from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface DungeonAssistScreenProps {
  viewModel: DungeonAssistViewModel;
  onSelectHero: (heroId: string) => void;
  onUseAssistAction: (actionId: string) => void;
  onContinue: () => void;
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
 * Dungeon Assist screen — 副本场景-人物辅助
 *
 * Landscape game viewport showing the dungeon scene with party members
 * and an assist interface for buffing/healing selected heroes.
 *
 * Layout mirrors the reference image:
 *   - Top HUD: dungeon name, room indicator, settings
 *   - Scene area: dungeon background with party formation
 *   - Bottom left: selected hero detail panel (portrait, stats, equipment, skills)
 *   - Bottom right: assist action grid
 */
export const DungeonAssistScreen: Component<DungeonAssistScreenProps> = (props) => {
  const selectedHero = createMemo(() =>
    props.viewModel.party.find((h) => h.id === props.viewModel.selectedHeroId) ??
    props.viewModel.party[0]
  );

  const selectedPortraitUrl = createMemo(() => {
    const hero = selectedHero();
    if (!hero) return null;
    return resolveHeroPortrait({ heroId: hero.id, classLabel: hero.classLabel });
  });

  return (
    <div
      class="dungeon-assist-viewport"
      data-source-scene="UI_Dungeon/DungeonAssistWindow"
      data-testid="dungeon-assist-screen"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="dungeon-assist-hud">
        <span class="dungeon-assist-hud-left">
          <span class="eyebrow">Dungeon Assist</span>
          <h1 class="dungeon-assist-title">{props.viewModel.dungeonName}</h1>
        </span>
        <span class="dungeon-assist-hud-center">
          <span class="hud-pill hud-pill-accent room-indicator">
            <span class="hud-pill-icon" aria-hidden="true">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7">
                <circle cx="12" cy="12" r="10" />
                <text x="12" y="16" text-anchor="middle" fill="currentColor" font-size="10" font-weight="bold">
                  {props.viewModel.roomNumber}
                </text>
              </svg>
            </span>
            Room {props.viewModel.roomNumber}
          </span>
        </span>
        <span class="dungeon-assist-hud-right">
          <button class="action-secondary settings-btn" aria-label="Settings">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="3" />
              <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.67 15 1.65 1.65 0 0 0 3.13 14H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.67 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.67V4.67A1.65 1.65 0 0 0 9 3.13V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1.51 1 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
            </svg>
            设置
          </button>
        </span>
      </header>

      {/* ── Dungeon Scene Area ──────────────────────────── */}
      <div class="dungeon-assist-scene">
        <div class="dungeon-assist-scene-bg" />
        <div class="dungeon-assist-scene-mist" />

        {/* Party formation in the dungeon */}
        <div class="dungeon-assist-party-row">
          <For each={props.viewModel.party}>
            {(hero) => {
              const portraitUrl = resolveHeroPortrait({
                heroId: hero.id,
                classLabel: hero.classLabel
              });
              const isSelected = hero.id === props.viewModel.selectedHeroId;
              return (
                <button
                  class={`dungeon-assist-hero-token${isSelected ? " dungeon-assist-hero-token--selected" : ""}`}
                  onClick={() => props.onSelectHero(hero.id)}
                  data-hero-id={hero.id}
                  data-testid={`dungeon-hero-token-${hero.id}`}
                >
                  <div
                    class={`dungeon-assist-token-portrait${portraitUrl ? " dungeon-assist-token-portrait--image" : " dungeon-assist-token-portrait--fallback"}`}
                  >
                    {portraitUrl ? (
                      <img
                        class="dungeon-assist-token-portrait-image"
                        src={portraitUrl}
                        alt=""
                        aria-hidden="true"
                      />
                    ) : (
                      <span class="dungeon-assist-token-initial">
                        {hero.classLabel[0]}
                      </span>
                    )}
                  </div>
                  <div class="dungeon-assist-token-name">{hero.name}</div>
                  <div class="dungeon-assist-token-bars">
                    <div class="dungeon-assist-bar-row">
                      <div class="dungeon-assist-bar-track">
                        <div
                          class="dungeon-assist-bar-fill"
                          style={{
                            width: `${healthPercent(hero.hp)}%`,
                            background: healthBarColor(hero.hp),
                          }}
                        />
                      </div>
                    </div>
                    <div class="dungeon-assist-bar-row">
                      <div class="dungeon-assist-bar-track">
                        <div
                          class="dungeon-assist-bar-fill"
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
        </div>
      </div>

      {/* ── Bottom Panels ───────────────────────────────── */}
      <div class="dungeon-assist-panels">
        {/* Left: Selected Hero Details */}
        <Show when={selectedHero()}>
          {(hero) => (
            <section class="dungeon-assist-detail-panel" data-testid="hero-detail-panel">
              <div class="dungeon-assist-detail-header">
                <div
                  class={`dungeon-assist-detail-portrait${selectedPortraitUrl() ? " dungeon-assist-detail-portrait--image" : " dungeon-assist-detail-portrait--fallback"}`}
                >
                  {selectedPortraitUrl() ? (
                    <img
                      class="dungeon-assist-detail-portrait-image"
                      src={selectedPortraitUrl()!}
                      alt=""
                      aria-hidden="true"
                    />
                  ) : (
                    <span class="dungeon-assist-detail-initial">
                      {hero().classLabel[0]}
                    </span>
                  )}
                </div>
                <div class="dungeon-assist-detail-info">
                  <div class="dungeon-assist-detail-name">{hero().name}</div>
                  <div class="dungeon-assist-detail-class">{hero().classLabel} · Lv.{hero().level}</div>
                  <div class="dungeon-assist-detail-bars">
                    <div class="dungeon-assist-detail-bar-row">
                      <span class="dungeon-assist-detail-bar-label">HP</span>
                      <span class="dungeon-assist-detail-bar-value">{hero().hp}</span>
                      <div class="dungeon-assist-detail-bar-track">
                        <div
                          class="dungeon-assist-detail-bar-fill"
                          style={{
                            width: `${healthPercent(hero().hp)}%`,
                            background: healthBarColor(hero().hp),
                          }}
                        />
                      </div>
                    </div>
                    <div class="dungeon-assist-detail-bar-row">
                      <span class="dungeon-assist-detail-bar-label">ST</span>
                      <span class="dungeon-assist-detail-bar-value">{hero().stress}</span>
                      <div class="dungeon-assist-detail-bar-track">
                        <div
                          class="dungeon-assist-detail-bar-fill"
                          style={{
                            width: `${stressPercent(hero().stress, hero().maxStress)}%`,
                            background: stressBarColor(hero().stress),
                          }}
                        />
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              {/* Equipment slots */}
              <div class="dungeon-assist-equipment-row">
                <div class="dungeon-assist-equipment-slot">
                  <div class="dungeon-assist-equipment-icon">
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <path d="M14.5 17.5L3 6V3h3l11.5 11.5" />
                      <path d="M13 19l6-6" />
                      <path d="M16 16l4 4" />
                      <path d="M19 21l2-2" />
                    </svg>
                  </div>
                  <span class="dungeon-assist-equipment-label">Weapon</span>
                </div>
                <div class="dungeon-assist-equipment-slot">
                  <div class="dungeon-assist-equipment-icon">
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
                    </svg>
                  </div>
                  <span class="dungeon-assist-equipment-label">Armor</span>
                </div>
                <div class="dungeon-assist-equipment-slot">
                  <div class="dungeon-assist-equipment-icon">
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />
                    </svg>
                  </div>
                  <span class="dungeon-assist-equipment-label">Trinket</span>
                </div>
                <div class="dungeon-assist-equipment-slot">
                  <div class="dungeon-assist-equipment-icon">
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
                    </svg>
                  </div>
                  <span class="dungeon-assist-equipment-label">Item</span>
                </div>
              </div>

              {/* Skill icons row */}
              <div class="dungeon-assist-skills-row">
                <For each={[1, 2, 3, 4]}>
                  {(i) => (
                    <div class="dungeon-assist-skill-icon" data-skill-slot={i}>
                      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                        <circle cx="12" cy="12" r="10" />
                        <path d="M12 8v8" />
                        <path d="M8 12h8" />
                      </svg>
                    </div>
                  )}
                </For>
              </div>
            </section>
          )}
        </Show>

        {/* Right: Assist Action Grid */}
        <section class="dungeon-assist-action-panel" data-testid="assist-action-panel">
          <header class="dungeon-assist-action-header">
            <span class="dungeon-assist-action-frame-rule" aria-hidden="true" />
            <h2 class="dungeon-assist-action-title">人物辅助</h2>
            <span class="dungeon-assist-action-frame-rule" aria-hidden="true" />
          </header>

          <div class="dungeon-assist-action-grid">
            {/* Center active action */}
            <div class="dungeon-assist-action-center">
              <Show when={props.viewModel.assistActions.find((a) => a.isAvailable)}>
                {(action) => (
                  <button
                    class="dungeon-assist-action-node dungeon-assist-action-node--active"
                    onClick={() => props.onUseAssistAction(action().id)}
                    data-action-id={action().id}
                    data-testid={`assist-action-${action().id}`}
                  >
                    <div class="dungeon-assist-action-node-icon">
                      <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                        <path d="M12 2L4 6v6c0 5.5 3.4 10.4 8 12 4.6-1.6 8-6.5 8-12V6l-8-4z" />
                      </svg>
                    </div>
                    <span class="dungeon-assist-action-node-label">{action().label}</span>
                  </button>
                )}
              </Show>
            </div>

            {/* Surrounding action slots */}
            <For each={props.viewModel.assistActions.slice(1)}>
              {(action, index) => (
                <button
                  class={`dungeon-assist-action-node${action.isAvailable ? "" : " dungeon-assist-action-node--locked"}`}
                  onClick={() => action.isAvailable && props.onUseAssistAction(action.id)}
                  data-action-id={action.id}
                  data-testid={`assist-action-${action.id}`}
                  disabled={!action.isAvailable}
                >
                  <div class="dungeon-assist-action-node-icon">
                    {action.isAvailable ? (
                      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                        <path d="M12 2L4 6v6c0 5.5 3.4 10.4 8 12 4.6-1.6 8-6.5 8-12V6l-8-4z" />
                      </svg>
                    ) : (
                      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                        <rect x="3" y="11" width="18" height="11" rx="2" ry="2" />
                        <path d="M7 11V7a5 5 0 0 1 10 0v4" />
                      </svg>
                    )}
                  </div>
                  <span class="dungeon-assist-action-node-label">{action.label}</span>
                </button>
              )}
            </For>
          </div>
        </section>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="dungeon-assist-controls">
        <div class="dungeon-assist-controls-left">
          <span class="expedition-status-pill expedition-status-pill--ready">
            <span class="expedition-status-pill-dot" aria-hidden="true" />
            Party in dungeon — Room {props.viewModel.roomNumber}
          </span>
        </div>
        <div class="dungeon-assist-controls-right">
          <button class="action-secondary" onClick={props.onReturnToTown} data-testid="return-to-town-btn">
            Return to Town
          </button>
          <button
            class="action-primary launch-primary"
            onClick={props.onContinue}
            disabled={!props.viewModel.canContinue}
            data-testid="continue-dungeon-btn"
          >
            {props.viewModel.canContinue ? "Continue Expedition" : "Assist Required"}
          </button>
        </div>
      </footer>
    </div>
  );
};
