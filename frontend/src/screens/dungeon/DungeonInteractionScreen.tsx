import { For, type Component } from "solid-js";

import type { DungeonInteractionViewModel } from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface DungeonInteractionScreenProps {
  viewModel: DungeonInteractionViewModel;
  onProceed: () => void;
  onInteract: (interactionId: string) => void;
  onRetreat: () => void;
  onOpenItems?: () => void;
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

function roomTypeLabel(type: string): string {
  switch (type) {
    case "combat": return "战斗房间";
    case "event": return "事件房间";
    case "corridor": return "走廊";
    case "boss": return "首领房间";
    case "treasure": return "宝藏房间";
    case "shop": return "商店";
    default: return type;
  }
}

function roomTypeClass(type: string): string {
  switch (type) {
    case "combat": return "room-type-badge room-type-badge--combat";
    case "event": return "room-type-badge room-type-badge--event";
    case "corridor": return "room-type-badge room-type-badge--corridor";
    case "boss": return "room-type-badge room-type-badge--boss";
    case "treasure": return "room-type-badge room-type-badge--treasure";
    case "shop": return "room-type-badge room-type-badge--shop";
    default: return "room-type-badge";
  }
}

/**
 * Dungeon interaction screen — landscape viewport for in-dungeon room interactions.
 *
 * Displays current room info, party vitals, available interactions, and progress.
 * References original Unity prefab structure from dungeon scene UI.
 *
 * This is the migrated "副本场景-交互" page for the dungeon-runtime target area.
 */
export const DungeonInteractionScreen: Component<DungeonInteractionScreenProps> = (props) => {
  const progressPercent = () => {
    const { currentRoom, totalRooms } = props.viewModel.progress;
    if (totalRooms <= 0) return 0;
    return Math.round((currentRoom / totalRooms) * 100);
  };

  return (
    <div
      class="expedition-viewport dungeon-interaction-viewport"
      data-source-scene="UI_Dungeon/DungeonInteractionWindow"
      data-source-prefab="Assets/Prefabs/UI/DungeonInteractionWindow.prefab"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">Dungeon Exploration</span>
          <h1 class="expedition-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class="hud-pill hud-pill-accent">{props.viewModel.dungeonName}</span>
          <span class={roomTypeClass(props.viewModel.roomType)}>
            {roomTypeLabel(props.viewModel.roomType)}
          </span>
          <span class="hud-pill hud-pill--with-icon">
            <span class="hud-pill-icon" aria-hidden="true">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7">
                <circle cx="12" cy="12" r="10" />
                <path d="M12 6v6l4 2" />
              </svg>
            </span>
            Room {props.viewModel.progress.currentRoom} / {props.viewModel.progress.totalRooms}
          </span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="expedition-content dungeon-interaction-content">
          {/* Room info panel */}
          <div class="dungeon-room-panel">
            <div class="dungeon-room-header">
              <span class="dungeon-room-label">{props.viewModel.roomLabel}</span>
              <span class={roomTypeClass(props.viewModel.roomType)}>
                {roomTypeLabel(props.viewModel.roomType)}
              </span>
            </div>
            <p class="dungeon-room-description">{props.viewModel.roomDescription}</p>

            {/* Progress bar */}
            <div class="dungeon-progress">
              <div class="dungeon-progress-header">
                <span class="dungeon-progress-label">Exploration Progress</span>
                <span class="dungeon-progress-value">
                  {props.viewModel.progress.roomsCleared} cleared · {props.viewModel.progress.currentRoom} / {props.viewModel.progress.totalRooms}
                </span>
              </div>
              <div class="dungeon-progress-track">
                <div
                  class="dungeon-progress-fill"
                  style={{ width: `${progressPercent()}%` }}
                />
              </div>
            </div>
          </div>

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
                      class="vitals-card dungeon-hero-card"
                      data-hero-id={hero.id}
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

          {/* Interaction options */}
          {props.viewModel.interactions.length > 0 && (
            <div class="dungeon-interactions-panel">
              <header class="details-overlay-header">
                <span class="details-overlay-frame-rule" aria-hidden="true" />
                <h2 class="details-overlay-title">Available Actions</h2>
                <span class="details-overlay-frame-rule" aria-hidden="true" />
              </header>
              <div class="dungeon-interactions-grid">
                <For each={props.viewModel.interactions}>
                  {(interaction) => (
                    <button
                      class={`dungeon-interaction-btn${interaction.isAvailable ? "" : " dungeon-interaction-btn--disabled"}`}
                      onClick={() => interaction.isAvailable && props.onInteract(interaction.id)}
                      disabled={!interaction.isAvailable}
                      data-interaction-id={interaction.id}
                    >
                      <span class="dungeon-interaction-label">{interaction.label}</span>
                      <span class="dungeon-interaction-desc">{interaction.description}</span>
                    </button>
                  )}
                </For>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left">
          <span class="expedition-status-pill expedition-status-pill--neutral">
            <span class="expedition-status-pill-dot" aria-hidden="true" />
            {props.viewModel.progress.roomsCleared} rooms cleared
          </span>
        </div>
        <div class="expedition-controls-right">
          <button
            class="action-secondary"
            onClick={props.onOpenItems}
            data-testid="dungeon-open-items-btn"
          >
            物品背包
          </button>
          <button
            class="action-secondary"
            onClick={props.onRetreat}
            disabled={!props.viewModel.isRetreatAvailable}
          >
            Retreat
          </button>
          <button
            class="action-primary launch-primary"
            onClick={props.onProceed}
            disabled={!props.viewModel.isProceedAvailable}
          >
            {props.viewModel.isProceedAvailable ? "Proceed" : "Blocked"}
          </button>
        </div>
      </footer>
    </div>
  );
};
