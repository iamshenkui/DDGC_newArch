import { For, type Component } from "solid-js";

import type { DungeonMapViewModel, DungeonMapRoomViewModel } from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface DungeonMapScreenProps {
  viewModel: DungeonMapViewModel;
  onEnterRoom: (roomId: string) => void;
  onFleeDungeon: () => void;
}

function roomKindLabel(kind: DungeonMapRoomViewModel["kind"]): string {
  switch (kind) {
    case "combat": return "Combat";
    case "boss": return "Boss";
    case "event": return "Event";
    case "corridor": return "Corridor";
    default: return "Unknown";
  }
}

function roomKindClass(kind: DungeonMapRoomViewModel["kind"]): string {
  switch (kind) {
    case "combat": return "room-node--combat";
    case "boss": return "room-node--boss";
    case "event": return "room-node--event";
    case "corridor": return "room-node--corridor";
    default: return "room-node--unknown";
  }
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
 * Dungeon map screen — landscape viewport layout.
 *
 * Shows the dungeon room grid, party vitals, and controls for room entry.
 * References original Unity prefab structure from:
 *   Assets/Prefabs/UI/Raid/Map/RoomSlot.prefab
 *
 * Source hierarchy: UI_Raid/Map
 *   RoomSlot × N → Room + Marker + Indicator
 *   Party HUD → hero vitals cards
 *   Torchlight / Gold strip
 */
export const DungeonMapScreen: Component<DungeonMapScreenProps> = (props) => {
  const progressPercent = () => {
    if (props.viewModel.totalRooms <= 0) return 0;
    return Math.round((props.viewModel.roomsCleared / props.viewModel.totalRooms) * 100);
  };

  return (
    <div
      class="dungeon-map-viewport"
      data-source-scene="UI_Raid/Map"
      data-source-prefab="Assets/Prefabs/UI/Raid/Map/RoomSlot.prefab"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">Dungeon Map</span>
          <h1 class="expedition-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class="hud-pill hud-pill-accent">{props.viewModel.dungeonType}</span>
          <span class="hud-pill">Floor {props.viewModel.floor}</span>
          <span class="hud-pill">Torch: {props.viewModel.torchlight}%</span>
          <span class="hud-pill gold-pill">
            Gold: {props.viewModel.goldCarried}
          </span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="expedition-content">
          {/* Progress banner */}
          <div class="outcome-banner">
            <div class="outcome-banner-ornament" />
            <span class="hud-pill">
              Progress: {props.viewModel.roomsCleared} / {props.viewModel.totalRooms} rooms ({progressPercent()}%)
            </span>
            {props.viewModel.isComplete && (
              <span class="hud-pill hud-pill-accent">Dungeon Complete</span>
            )}
            {props.viewModel.partyFled && (
              <span class="hud-pill hud-pill--afflicted">Party Fled</span>
            )}
          </div>

          {/* Room grid */}
          <div class="dungeon-map-room-grid">
            <For each={props.viewModel.rooms}>
              {(room) => {
                const currentRoomIndex = props.viewModel.rooms.findIndex((r) => r.isCurrent);
                const targetRoomIndex = props.viewModel.rooms.findIndex((r) => r.roomId === room.roomId);
                const isAdjacent = currentRoomIndex < 0 || Math.abs(currentRoomIndex - targetRoomIndex) === 1;
                const isEnterable = !room.cleared && !room.isCurrent && !props.viewModel.isComplete && !props.viewModel.partyFled && isAdjacent;
                return (
                  <button
                    class={`room-node ${roomKindClass(room.kind)} ${room.isCurrent ? "room-node--current" : ""} ${room.cleared ? "room-node--cleared" : ""}`}
                    data-room-id={room.roomId}
                    data-room-kind={room.kind}
                    data-source-component="RoomSlot"
                    disabled={!isEnterable}
                    onClick={() => props.onEnterRoom(room.roomId)}
                  >
                    <span class="room-node-label">{roomKindLabel(room.kind)}</span>
                    {room.isCurrent && (
                      <span class="room-node-badge room-node-badge--current">Current</span>
                    )}
                    {room.cleared && (
                      <span class="room-node-badge room-node-badge--cleared">Cleared</span>
                    )}
                    {room.curioId && !room.cleared && (
                      <span class="room-node-badge room-node-badge--curio">Curio</span>
                    )}
                    {room.trapId && !room.cleared && (
                      <span class="room-node-badge room-node-badge--trap">Trap</span>
                    )}
                  </button>
                );
              }}
            </For>
          </div>

          {/* Party vitals */}
          {props.viewModel.heroes.length > 0 && (
            <div class="expedition-hero-row">
              <For each={props.viewModel.heroes}>
                {(hero) => {
                  const portraitUrl = resolveHeroPortrait({
                    heroId: hero.id,
                    classLabel: hero.classLabel
                  });
                  return (
                    <div class="vitals-card">
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
                          <span class="vitals-card-initial">
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
                      {hero.isAtDeathsDoor && (
                        <span class="hud-pill hud-pill--afflicted">Death's Door</span>
                      )}
                      {hero.isDead && (
                        <span class="hud-pill hud-pill--dead">Dead</span>
                      )}
                    </div>
                  );
                }}
              </For>
            </div>
          )}
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left">
          <span class="hud-pill">
            {props.viewModel.currentRoom
              ? `Current: ${roomKindLabel(props.viewModel.currentRoom.kind)}`
              : "Select a room to enter"}
          </span>
        </div>
        <div class="expedition-controls-right">
          <button
            class="action-secondary"
            onClick={props.onFleeDungeon}
            disabled={props.viewModel.isComplete || props.viewModel.partyFled}
          >
            Flee Dungeon
          </button>
          <button
            class="action-primary launch-primary"
            onClick={() => {
              const currentIdx = props.viewModel.rooms.findIndex((r) => r.isCurrent);
              const nextRoom = props.viewModel.rooms.find(
                (r, idx) => idx === currentIdx + 1 && !r.cleared && !r.isCurrent
              );
              if (nextRoom) {
                props.onEnterRoom(nextRoom.roomId);
              }
            }}
            disabled={props.viewModel.isComplete || props.viewModel.partyFled}
          >
            {props.viewModel.isComplete
              ? "Dungeon Complete"
              : props.viewModel.partyFled
                ? "Party Fled"
                : "Enter Next Room"}
          </button>
        </div>
      </footer>
    </div>
  );
};
