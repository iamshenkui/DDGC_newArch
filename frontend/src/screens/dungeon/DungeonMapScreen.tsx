import { For, Show, type Component } from "solid-js";

import type { DungeonMapViewModel, DungeonMapRoom, DungeonMapHero } from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface DungeonMapScreenProps {
  viewModel: DungeonMapViewModel;
  onEnterRoom: (roomId: string) => void;
  onRetreat: () => void;
  onCompleteDungeon: () => void;
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

function roomTypeLabel(type: DungeonMapRoom["type"]): string {
  const labels: Record<string, string> = {
    entrance: "Entrance",
    combat: "Combat",
    treasure: "Treasure",
    rest: "Rest",
    boss: "Boss",
    exit: "Exit",
    empty: "Empty",
    curio: "Curio",
    shrine: "Shrine"
  };
  return labels[type] ?? type;
}

function roomTypeClass(type: DungeonMapRoom["type"]): string {
  return `dungeon-room--${type}`;
}

function isRoomAccessible(room: DungeonMapRoom, currentRoomId: string, rooms: ReadonlyArray<DungeonMapRoom>): boolean {
  if (room.id === currentRoomId) return false;
  if (!room.isRevealed) return false;
  const currentRoom = rooms.find((r) => r.id === currentRoomId);
  if (!currentRoom) return false;
  return currentRoom.connections.includes(room.id);
}

/**
 * Dungeon Map screen — landscape game viewport for in-dungeon exploration.
 *
 * Displays a grid-based dungeon map with room nodes, corridors, party position,
 * and navigation controls. References original Unity prefab structure from:
 *   Assets/Prefabs/UI/DungeonMapWindow.prefab (estimated)
 *
 * Original asset wiring:
 *   portraits  — extracted hunter family served from /original/heroes/ (UIR-005B).
 *                Wired via resolveHeroPortrait on party vitals panel; un-extracted
 *                families fall through to a CSS-letter avatar.
 */
export const DungeonMapScreen: Component<DungeonMapScreenProps> = (props) => {
  const torchPercent = () => Math.round((props.viewModel.torchLevel / props.viewModel.maxTorchLevel) * 100);

  const torchColor = () => {
    const pct = torchPercent();
    if (pct >= 60) return "#e8c84a";
    if (pct >= 30) return "#e8a838";
    return "#ea7767";
  };

  const progressPercent = () => props.viewModel.completionPercent;

  const currentRoom = () => props.viewModel.rooms.find((r) => r.id === props.viewModel.currentRoomId);

  const accessibleRooms = () =>
    props.viewModel.rooms.filter((r) => isRoomAccessible(r, props.viewModel.currentRoomId, props.viewModel.rooms));

  return (
    <div
      class="dungeon-map-viewport"
      data-source-scene="UI_Dungeon/DungeonMapWindow"
      data-source-prefab="Assets/Prefabs/UI/DungeonMapWindow.prefab"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">Dungeon Exploration</span>
          <h1 class="expedition-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class="hud-pill hud-pill-accent">{props.viewModel.expeditionName}</span>
          <span class="hud-pill hud-pill--with-icon">
            <span class="hud-pill-icon" aria-hidden="true">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7">
                <path d="M12 2L4 6v6c0 5 3.4 9.4 8 10 4.6-.6 8-5 8-10V6l-8-4z" />
              </svg>
            </span>
            {props.viewModel.dungeonName}
          </span>
          {/* Torch level pill */}
          <span
            class="hud-pill"
            style={{
              color: torchColor(),
              "border-color": `${torchColor()}40`,
              background: `${torchColor()}14`
            }}
          >
            <span class="hud-pill-icon" aria-hidden="true">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke={torchColor()} stroke-width="1.7">
                <path d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z" />
              </svg>
            </span>
            Torch: {props.viewModel.torchLevel}/{props.viewModel.maxTorchLevel}
          </span>
          {/* Progress pill */}
          <span class="hud-pill">
            <span class="hud-pill-icon" aria-hidden="true">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7">
                <circle cx="12" cy="12" r="10" />
                <path d="M12 6v6l4 2" />
              </svg>
            </span>
            {props.viewModel.exploredCount}/{props.viewModel.totalRooms} rooms
          </span>
        </span>
        <span class="expedition-hud-right">
          <span
            class="expedition-status-pill"
            style={{
              color: progressPercent() >= 80 ? "#5bbd6e" : progressPercent() >= 40 ? "#e8a838" : "var(--panel-muted)",
              "border-color": progressPercent() >= 80 ? "rgba(91,189,110,0.4)" : progressPercent() >= 40 ? "rgba(232,168,56,0.4)" : "rgba(160,204,188,0.15)"
            }}
          >
            <span class="expedition-status-pill-dot" aria-hidden="true" />
            {progressPercent()}% explored
          </span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="dungeon-map-surface">
        <div class="dungeon-map-bg" />
        <div class="dungeon-map-mist" />

        <div class="dungeon-map-layout">
          {/* ── Dungeon Map Grid ─────────────────────────── */}
          <section class="dungeon-map-grid-panel">
            <header class="dungeon-map-grid-header">
              <span class="dungeon-map-grid-title">{props.viewModel.dungeonName}</span>
              <Show when={currentRoom()}>
                {(room) => (
                  <span class={`dungeon-map-current-room-badge ${roomTypeClass(room().type)}`}>
                    {roomTypeLabel(room().type)} — {room().label}
                  </span>
                )}
              </Show>
            </header>

            <div
              class="dungeon-map-grid"
              style={{
                "grid-template-columns": `repeat(${props.viewModel.minimapCols}, 1fr)`,
                "grid-template-rows": `repeat(${props.viewModel.minimapRows}, 1fr)`
              }}
            >
              <For each={props.viewModel.rooms}>
                {(room) => {
                  const accessible = isRoomAccessible(room, props.viewModel.currentRoomId, props.viewModel.rooms);
                  const isCurrent = room.id === props.viewModel.currentRoomId;
                  return (
                    <div
                      class={`dungeon-room ${roomTypeClass(room.type)} ${isCurrent ? "dungeon-room--current" : ""} ${room.isVisited ? "dungeon-room--visited" : ""} ${room.isRevealed ? "dungeon-room--revealed" : "dungeon-room--hidden"} ${accessible ? "dungeon-room--accessible" : ""}`}
                      style={{
                        "grid-column": room.x + 1,
                        "grid-row": room.y + 1
                      }}
                      data-room-id={room.id}
                      data-room-type={room.type}
                    >
                      <button
                        class="dungeon-room-btn"
                        onClick={() => accessible && props.onEnterRoom(room.id)}
                        disabled={!accessible}
                        title={accessible ? `Enter ${room.label}` : room.isRevealed ? room.label : "Unknown"}
                      >
                        <span class="dungeon-room-marker" aria-hidden="true">
                          {room.type === "entrance" && (
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                              <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
                              <polyline points="9 22 9 12 15 12 15 22" />
                            </svg>
                          )}
                          {room.type === "combat" && (
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                              <path d="M14.5 17.5L3 6V3h3l11.5 11.5" />
                              <path d="M13 19l6-6" />
                              <path d="M16 16l4 4" />
                              <path d="M19 21l2-2" />
                            </svg>
                          )}
                          {room.type === "treasure" && (
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                              <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
                              <polyline points="3.27 6.96 12 12.01 20.73 6.96" />
                              <line x1="12" y1="22.08" x2="12" y2="12" />
                            </svg>
                          )}
                          {room.type === "rest" && (
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                              <path d="M2 22h20" />
                              <path d="M2 6h20" />
                              <path d="M2 10h20" />
                              <path d="M2 14h20" />
                              <path d="M2 18h20" />
                              <path d="M6 2v20" />
                            </svg>
                          )}
                          {room.type === "boss" && (
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                              <path d="M12 2L2 7l10 5 10-5-10-5z" />
                              <path d="M2 17l10 5 10-5" />
                              <path d="M2 12l10 5 10-5" />
                            </svg>
                          )}
                          {room.type === "exit" && (
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                              <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" />
                              <polyline points="16 17 21 12 16 7" />
                              <line x1="21" y1="12" x2="9" y2="12" />
                            </svg>
                          )}
                          {room.type === "curio" && (
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                              <circle cx="12" cy="12" r="10" />
                              <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" />
                              <line x1="12" y1="17" x2="12.01" y2="17" />
                            </svg>
                          )}
                          {room.type === "shrine" && (
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                              <path d="M12 2L2 7l10 5 10-5-10-5z" />
                              <path d="M2 17l10 5 10-5" />
                              <path d="M2 12l10 5 10-5" />
                              <path d="M12 22V12" />
                              <path d="M12 7V2" />
                            </svg>
                          )}
                          {room.type === "empty" && (
                            <span class="dungeon-room-dot" />
                          )}
                        </span>
                        <span class="dungeon-room-label">{room.isRevealed ? room.label : "?"}</span>
                        {isCurrent && (
                          <span class="dungeon-room-current-ring" aria-hidden="true" />
                        )}
                      </button>
                    </div>
                  );
                }}
              </For>
            </div>

            {/* Legend */}
            <div class="dungeon-map-legend">
              <span class="dungeon-map-legend-title">Legend</span>
              <div class="dungeon-map-legend-items">
                <span class="dungeon-map-legend-item"><span class="dungeon-legend-dot dungeon-legend-dot--entrance" />Entrance</span>
                <span class="dungeon-map-legend-item"><span class="dungeon-legend-dot dungeon-legend-dot--combat" />Combat</span>
                <span class="dungeon-map-legend-item"><span class="dungeon-legend-dot dungeon-legend-dot--treasure" />Treasure</span>
                <span class="dungeon-map-legend-item"><span class="dungeon-legend-dot dungeon-legend-dot--rest" />Rest</span>
                <span class="dungeon-map-legend-item"><span class="dungeon-legend-dot dungeon-legend-dot--boss" />Boss</span>
                <span class="dungeon-map-legend-item"><span class="dungeon-legend-dot dungeon-legend-dot--exit" />Exit</span>
                <span class="dungeon-map-legend-item"><span class="dungeon-legend-dot dungeon-legend-dot--curio" />Curio</span>
                <span class="dungeon-map-legend-item"><span class="dungeon-legend-dot dungeon-legend-dot--shrine" />Shrine</span>
              </div>
            </div>
          </section>

          {/* ── Party Panel ────────────────────────────────── */}
          <aside class="dungeon-party-panel">
            <header class="dungeon-party-panel-header">
              <span class="dungeon-party-panel-title">Party</span>
              <span class="dungeon-party-panel-count">{props.viewModel.party.length} heroes</span>
            </header>

            <div class="dungeon-party-list">
              <For each={props.viewModel.party}>
                {(hero) => {
                  const portraitUrl = resolveHeroPortrait({
                    heroId: hero.id,
                    classLabel: hero.classLabel
                  });
                  return (
                    <div class="dungeon-party-hero">
                      <div class="dungeon-party-hero-portrait">
                        {portraitUrl ? (
                          <img
                            class="dungeon-party-hero-portrait-image"
                            src={portraitUrl}
                            alt=""
                            aria-hidden="true"
                          />
                        ) : (
                          <span class="dungeon-party-hero-initial">{hero.classLabel[0]}</span>
                        )}
                        {(hero.isWounded || hero.isAfflicted) && (
                          <span class={`dungeon-party-hero-status ${hero.isAfflicted ? "dungeon-party-hero-status--afflicted" : "dungeon-party-hero-status--wounded"}`}>
                            {hero.isAfflicted ? "A" : "W"}
                          </span>
                        )}
                      </div>
                      <div class="dungeon-party-hero-info">
                        <span class="dungeon-party-hero-name">{hero.name}</span>
                        <span class="dungeon-party-hero-class">{hero.classLabel}</span>
                      </div>
                      <div class="dungeon-party-hero-bars">
                        <div class="dungeon-bar-row">
                          <span class="dungeon-bar-label">HP</span>
                          <span class="dungeon-bar-track">
                            <span
                              class="dungeon-bar-fill"
                              style={{
                                width: `${healthPercent(hero.hp)}%`,
                                background: healthBarColor(hero.hp)
                              }}
                            />
                          </span>
                        </div>
                        <div class="dungeon-bar-row">
                          <span class="dungeon-bar-label">ST</span>
                          <span class="dungeon-bar-track">
                            <span
                              class="dungeon-bar-fill"
                              style={{
                                width: `${stressPercent(hero.stress, hero.maxStress)}%`,
                                background: stressBarColor(hero.stress)
                              }}
                            />
                          </span>
                        </div>
                      </div>
                    </div>
                  );
                }}
              </For>
            </div>

            {/* Accessible rooms list */}
            <Show when={accessibleRooms().length > 0}>
              <div class="dungeon-navigate-panel">
                <span class="dungeon-navigate-title">Navigate To</span>
                <div class="dungeon-navigate-list">
                  <For each={accessibleRooms()}>
                    {(room) => (
                      <button
                        class={`dungeon-navigate-btn ${roomTypeClass(room.type)}`}
                        onClick={() => props.onEnterRoom(room.id)}
                      >
                        <span class="dungeon-navigate-marker">
                          {roomTypeLabel(room.type)[0]}
                        </span>
                        <span class="dungeon-navigate-label">{room.label}</span>
                        <Show when={room.difficulty}>
                          <span class="dungeon-navigate-difficulty">{room.difficulty}</span>
                        </Show>
                      </button>
                    )}
                  </For>
                </div>
              </div>
            </Show>
          </aside>
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left">
          <span class="expedition-status-pill expedition-status-pill--neutral">
            <span class="expedition-status-pill-dot" aria-hidden="true" />
            {props.viewModel.isComplete ? "Dungeon complete — proceed to exit" : "Explore the dungeon carefully"}
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
            title={props.viewModel.isRetreatAvailable ? "Retreat to town" : "Cannot retreat from here"}
          >
            Retreat
          </button>
          <Show when={props.viewModel.isComplete}>
            <button
              class="action-primary launch-primary"
              onClick={props.onCompleteDungeon}
            >
              Complete Expedition
            </button>
          </Show>
        </div>
      </footer>
    </div>
  );
};
