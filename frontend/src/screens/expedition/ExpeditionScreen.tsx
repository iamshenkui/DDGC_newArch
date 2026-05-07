import { For, createSignal, type Component } from "solid-js";

import type { DungeonId, ExpeditionSetupViewModel } from "../../bridge/contractTypes";
import {
  resolveChromeAsset,
  resolveHeroPortrait,
  resolveDungeonMapIcon,
  resolveDungeonMapBackground,
  resolveExpeditionUiAsset
} from "../../assets/originalAssetPaths";

interface ExpeditionScreenProps {
  viewModel: ExpeditionSetupViewModel;
  onLaunchExpedition: () => void;
  onReturnToTown: () => void;
  onSelectDungeon?: (dungeonId: DungeonId) => void;
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

const DUNGEON_NODES: Array<{
  id: DungeonId;
  label: string;
  sourceLabel: string;
  positionClass: string;
}> = [
  { id: "baihu", label: "Baihu", sourceLabel: "白虎大陆", positionClass: "expedition-dungeon-node--tl" },
  { id: "qinglong", label: "Qinglong", sourceLabel: "青龙大陆", positionClass: "expedition-dungeon-node--tr" },
  { id: "xuanwu", label: "Xuanwu", sourceLabel: "玄武大陆", positionClass: "expedition-dungeon-node--bl" },
  { id: "zhuque", label: "Zhuque", sourceLabel: "朱雀大陆", positionClass: "expedition-dungeon-node--br" }
];

/**
 * Expedition launch screen — landscape game viewport for pre-launch review.
 *
 * Pre-launch review showing party vitals, expedition details, and warnings.
 * References original Unity prefab structure from:
 *   Assets/Prefabs/UI/ExpeditionWindow.prefab (estimated)
 *
 * Original asset wiring:
 *   gold       — extracted, served from /original/chrome/gold.png (UIR-005B).
 *                Wired via resolveChromeAsset("goldIcon") on the cost pill.
 *   portraits  — extracted hunter family served from /original/heroes/ (UIR-005B).
 *                Wired via resolveHeroPortrait on each vitals card; un-extracted
 *                families fall through to a CSS-letter avatar.
 *   supply     — Assets/Resources/Sprites/inv_supply+rattle_drum.png
 *                GUID e401bf9b9275ede4aa2ff50d13cc6207.
 *                Not extracted (BLOCKER-001) — pill keeps the explicit blocker
 *                annotation and a CSS fallback swatch.
 *
 * Expedition map composition (KUI-P1-014):
 *   map backgrounds — /original/expedition/ui/map[1-4].png staged in P1-013.
 *                     Selected by dungeonId via resolveDungeonMapBackground.
 *   dungeon icons   — /original/expedition/dungeon/dungeon_map_*.png staged in P1-013.
 *                     All four dungeons rendered as map hot-spot nodes.
 *   quest chrome    — quest.title.bg.png, quest.flag.png, quest.minimap.png,
 *                     quest.line.png staged in P1-013. Used as decorative chrome.
 *
 * Selection states (KUI-P1-015):
 *   hover/select    — dungeon nodes are interactive buttons. Hover state lifts
 *                     the node visually; click dispatches select-dungeon intent
 *                     to update the routed dungeon and refresh the map.
 *   route handoff   — selected node receives a "→ ROUTE" badge that links the
 *                     selection to the launch CTA, signaling which dungeon will
 *                     be entered when the launch button fires.
 */
export const ExpeditionScreen: Component<ExpeditionScreenProps> = (props) => {
  const [hoveredDungeon, setHoveredDungeon] = createSignal<DungeonId | null>(null);

  const launchStateLabel = () => {
    if (!props.viewModel.isLaunchable) return "Hold — review warnings";
    if (props.viewModel.warnings.length > 0)
      return `Cleared with ${props.viewModel.warnings.length} caution${props.viewModel.warnings.length !== 1 ? "s" : ""}`;
    return "All checks clear — ready to launch";
  };

  const launchStateClass = () => {
    if (!props.viewModel.isLaunchable) return "expedition-status-pill--afflicted";
    if (props.viewModel.warnings.length > 0) return "expedition-status-pill--wounded";
    return "expedition-status-pill--ready";
  };

  const selectedDungeonId = (): DungeonId =>
    (props.viewModel.dungeonId ?? "baihu");

  const mapBackgroundUrl = () => resolveDungeonMapBackground(selectedDungeonId());

  return (
    <div
      class="expedition-viewport"
      data-source-scene="UI_Expedition/ExpeditionWindow"
      data-source-prefab="Assets/Prefabs/UI/ExpeditionWindow.prefab"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">Expedition Launch</span>
          <h1 class="expedition-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class="hud-pill hud-pill-accent">{props.viewModel.expeditionName}</span>
          <span class="hud-pill hud-pill--with-icon">
            <span class="hud-pill-icon" aria-hidden="true">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7">
                <circle cx="9" cy="8" r="3.5" />
                <path d="M2 20c0-3.6 3.2-6 7-6s7 2.4 7 6" />
                <circle cx="17" cy="9" r="2.5" />
                <path d="M22 19c0-2.4-1.8-4-4-4" />
              </svg>
            </span>
            Party: {props.viewModel.partySize} hero{props.viewModel.partySize !== 1 ? "es" : ""}
          </span>
          <span class="hud-pill hud-pill--with-icon">
            <span class="hud-pill-icon" aria-hidden="true">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7">
                <path d="M12 2L4 6v6c0 5 3.4 9.4 8 10 4.6-.6 8-5 8-10V6l-8-4z" />
              </svg>
            </span>
            Difficulty: {props.viewModel.difficulty}
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
            Supply: {props.viewModel.supplyLevel}
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
            Cost: {props.viewModel.provisionCost}
          </span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        {/* Map background layer — source-faithful dungeon hot-spot background */}
        <div
          class="expedition-map-bg"
          style={{ "background-image": `url(${mapBackgroundUrl()})` }}
          data-source-component="UI_Quest_Match_Height/BackGround"
          data-asset-key={props.viewModel.dungeonId ?? "baihu"}
        />
        <div class="expedition-surface-mist" />

        {/* Expedition map composition — dungeon nodes + quest chrome */}
        <div class="expedition-map-composition">
          {/* Dungeon hot-spot nodes — interactive selection surface
              (KUI-P1-015). Hover lifts the node; click dispatches the
              select-dungeon intent that switches the routed dungeon. */}
          <div class="expedition-dungeon-nodes" role="radiogroup" aria-label="Expedition routes">
            <For each={DUNGEON_NODES}>
              {(node) => {
                const isSelected = () => node.id === selectedDungeonId();
                const isHovered = () => hoveredDungeon() === node.id;
                const isInteractive = () => Boolean(props.onSelectDungeon);
                const handleSelect = () => {
                  if (props.onSelectDungeon && !isSelected()) {
                    props.onSelectDungeon(node.id);
                  }
                };
                return (
                  <button
                    type="button"
                    class={`expedition-dungeon-node ${node.positionClass}${isSelected() ? " expedition-dungeon-node--selected" : ""}${isHovered() ? " expedition-dungeon-node--hovered" : ""}`}
                    data-dungeon-id={node.id}
                    data-selected={isSelected()}
                    data-hovered={isHovered()}
                    role="radio"
                    aria-checked={isSelected()}
                    aria-label={`${node.label} (${node.sourceLabel})${isSelected() ? " — selected route" : ""}`}
                    disabled={!isInteractive()}
                    onClick={handleSelect}
                    onMouseEnter={() => setHoveredDungeon(node.id)}
                    onMouseLeave={() =>
                      setHoveredDungeon((curr) => (curr === node.id ? null : curr))
                    }
                    onFocus={() => setHoveredDungeon(node.id)}
                    onBlur={() =>
                      setHoveredDungeon((curr) => (curr === node.id ? null : curr))
                    }
                  >
                    <img
                      class="expedition-dungeon-icon"
                      src={resolveDungeonMapIcon(node.id)}
                      alt=""
                      aria-hidden="true"
                    />
                    <span class="expedition-dungeon-label">{node.label}</span>
                    <span class="expedition-dungeon-source-label" lang="zh-Hans">
                      {node.sourceLabel}
                    </span>
                    {isSelected() && (
                      <span class="expedition-dungeon-marker" aria-hidden="true">
                        <img
                          src={resolveExpeditionUiAsset("questStar")}
                          alt=""
                          aria-hidden="true"
                        />
                      </span>
                    )}
                    {isSelected() && (
                      <span
                        class="expedition-dungeon-route-badge"
                        aria-hidden="true"
                        data-source-component="SelectedQuestPanel/RouteBadge"
                      >
                        <svg
                          width="10"
                          height="10"
                          viewBox="0 0 24 24"
                          fill="none"
                          stroke="currentColor"
                          stroke-width="2.4"
                          stroke-linecap="round"
                          stroke-linejoin="round"
                        >
                          <polyline points="9 6 15 12 9 18" />
                        </svg>
                        Route
                      </span>
                    )}
                  </button>
                );
              }}
            </For>
          </div>

          {/* Quest chrome — title strip, flag, minimap frame */}
          <div class="expedition-quest-chrome">
            <img
              class="expedition-title-bg"
              src={resolveExpeditionUiAsset("questTitleBg")}
              alt=""
              aria-hidden="true"
              data-source-component="SelectedQuestPanel/TitleBg"
            />
            <img
              class="expedition-flag-decoration"
              src={resolveExpeditionUiAsset("questFlag")}
              alt=""
              aria-hidden="true"
              data-source-component="UI_Quest/QuestFlag"
            />
            <img
              class="expedition-minimap-frame"
              src={resolveExpeditionUiAsset("questMinimap")}
              alt=""
              aria-hidden="true"
              data-source-component="UI_Quest/QuestMinimap"
            />
            <img
              class="expedition-line-decoration"
              src={resolveExpeditionUiAsset("questLine")}
              alt=""
              aria-hidden="true"
            />
          </div>
        </div>

        <div class="expedition-content expedition-launch-content">
          {/* Party vitals row */}
          {props.viewModel.party.length > 0 && (
            <div class="expedition-hero-row">
              <For each={props.viewModel.party}>
                {(hero) => {
                  const portraitUrl = resolveHeroPortrait({
                    heroId: hero.id,
                    classLabel: hero.classLabel
                  });
                  return (
                    <div
                      class="vitals-card"
                      data-source-prefab="Assets/Prefabs/UI/ExpeditionWindow.prefab"
                    >
                      <div
                        class={`vitals-card-portrait${portraitUrl ? " vitals-card-portrait--image" : " vitals-card-portrait--fallback"}`}
                      >
                        {portraitUrl ? (
                          <img
                            class="vitals-card-portrait-image"
                            src={portraitUrl}
                            alt=""
                            aria-hidden="true"
                          />
                        ) : (
                          <span
                            class="vitals-card-initial"
                            data-blocker="BLOCKER-002: portrait sprite not extracted for this hero family"
                            data-class-label={hero.classLabel}
                          >
                            {hero.classLabel[0]}
                          </span>
                        )}
                      </div>
                      <div class="vitals-card-name">{hero.name}</div>
                      <div class="vitals-card-class">{hero.classLabel}</div>
                      <div class="vitals-card-bars">
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
                    </div>
                  );
                }}
              </For>
            </div>
          )}

          {/* Twin panels: details + readiness checks side by side */}
          <div class="expedition-launch-panels">
            <section class="details-overlay expedition-details-overlay">
              <header class="details-overlay-header">
                <span class="details-overlay-frame-rule" aria-hidden="true" />
                <h2 class="details-overlay-title">Expedition Details</h2>
                <span class="details-overlay-frame-rule" aria-hidden="true" />
              </header>
              <div class="details-overlay-row">
                <span class="details-overlay-label">Expedition</span>
                <span class="details-overlay-value">{props.viewModel.expeditionName}</span>
              </div>
              <div class="details-overlay-row">
                <span class="details-overlay-label">Party Size</span>
                <span class="details-overlay-value">{props.viewModel.partySize}</span>
              </div>
              <div class="details-overlay-row">
                <span class="details-overlay-label">Difficulty</span>
                <span class="details-overlay-value">{props.viewModel.difficulty}</span>
              </div>
              <div class="details-overlay-row">
                <span class="details-overlay-label">Est. Duration</span>
                <span class="details-overlay-value">{props.viewModel.estimatedDuration}</span>
              </div>
              <div class="details-overlay-row">
                <span class="details-overlay-label">Supply Level</span>
                <span class="details-overlay-value">{props.viewModel.supplyLevel}</span>
              </div>
              <div class="details-overlay-row">
                <span class="details-overlay-label">Provision Cost</span>
                <span class="details-overlay-value">{props.viewModel.provisionCost}</span>
              </div>

              {props.viewModel.objectives.length > 0 && (
                <div class="details-overlay-objectives">
                  <div class="details-overlay-objectives-title">Objectives</div>
                  <For each={props.viewModel.objectives}>
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
            </section>

            <section class={`details-overlay readiness-checks ${props.viewModel.warnings.length > 0 ? "readiness-checks--cautious" : "readiness-checks--clear"}`}>
              <header class="details-overlay-header">
                <span class="details-overlay-frame-rule" aria-hidden="true" />
                <h2 class="details-overlay-title">Readiness Checks</h2>
                <span class="details-overlay-frame-rule" aria-hidden="true" />
              </header>

              <div class="readiness-check-row readiness-check-row--ok">
                <span class="readiness-check-marker readiness-check-marker--ok" aria-hidden="true">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="5 12 10 17 19 7" />
                  </svg>
                </span>
                <span class="readiness-check-text">
                  Party of {props.viewModel.partySize} ready &ndash; supply {props.viewModel.supplyLevel.toLowerCase()}
                </span>
              </div>

              {props.viewModel.warnings.length > 0 ? (
                <For each={props.viewModel.warnings}>
                  {(warning) => (
                    <div class="readiness-check-row readiness-check-row--warn">
                      <span class="readiness-check-marker readiness-check-marker--warn" aria-hidden="true">
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
                          <path d="M12 3L1 21h22L12 3z" />
                          <line x1="12" y1="10" x2="12" y2="14" />
                          <circle cx="12" cy="17" r="0.8" fill="currentColor" />
                        </svg>
                      </span>
                      <span class="readiness-check-text">{warning}</span>
                    </div>
                  )}
                </For>
              ) : (
                <div class="readiness-check-row readiness-check-row--ok">
                  <span class="readiness-check-marker readiness-check-marker--ok" aria-hidden="true">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                      <polyline points="5 12 10 17 19 7" />
                    </svg>
                  </span>
                  <span class="readiness-check-text">No active warnings &ndash; all systems nominal</span>
                </div>
              )}
            </section>
          </div>
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left">
          <span class={`expedition-status-pill ${launchStateClass()}`}>
            <span class="expedition-status-pill-dot" aria-hidden="true" />
            {launchStateLabel()}
          </span>
        </div>
        <div class="expedition-controls-right">
          <button class="action-secondary" onClick={props.onReturnToTown}>
            Return to Town
          </button>
          <button
            class="action-primary launch-primary"
            onClick={props.onLaunchExpedition}
            disabled={!props.viewModel.isLaunchable}
          >
            {props.viewModel.isLaunchable
              ? "Launch Expedition"
              : "Party Not Ready"}
          </button>
        </div>
      </footer>
    </div>
  );
};
