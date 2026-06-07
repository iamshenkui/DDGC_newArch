import { For, Show, type Component } from "solid-js";

import type {
  CombatViewModel,
  CombatHeroState,
  CombatEnemyState,
  MapRoomNode,
} from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface CombatScreenProps {
  viewModel: CombatViewModel;
  onUseSkill: (skillId: string) => void;
  onContinueCombat: () => void;
  onFleeCombat: () => void;
  onOpenSettings: () => void;
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

function getRoomNodeClass(room: MapRoomNode): string {
  const base = "combat-map-node";
  if (room.isCurrent) return `${base} combat-map-node--current`;
  if (room.isCleared) return `${base} combat-map-node--cleared`;
  return `${base} combat-map-node--${room.kind}`;
}

/**
 * Combat screen — dungeon encounter with character hit focus.
 *
 * Layout mirrors the reference image:
 *   - Top HUD: dungeon name, round label, settings button
 *   - Battle stage: heroes on left, enemies on right
 *   - Bottom-left: hit character status panel (portrait, stats, skills)
 *   - Bottom-right: dungeon minimap with room connections
 *
 * Source scene: 副本场景-人物受击 (Dungeon Scene - Character Hit)
 */
export const CombatScreen: Component<CombatScreenProps> = (props) => {
  const hitHero = () =>
    props.viewModel.party.find((h) => h.id === props.viewModel.hitTargetHeroId);

  const activeHero = () =>
    props.viewModel.party.find((h) => h.id === props.viewModel.activeHeroId);

  const displayHero = () => hitHero() ?? activeHero() ?? props.viewModel.party[0];

  const isCharacterHitPhase = () => props.viewModel.phase === "character-hit";
  const canUseSkills = () => !isCharacterHitPhase();

  return (
    <div
      class="combat-viewport"
      data-source-scene="UI_Combat/CombatWindow"
      data-source-prefab="Assets/Prefabs/UI/CombatWindow.prefab"
      data-testid="combat-screen"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="combat-hud">
        <span class="combat-hud-left">
          <span class="eyebrow">{props.viewModel.dungeonName}</span>
          <h1 class="combat-title">{props.viewModel.title}</h1>
        </span>
        <span class="combat-hud-center">
          <span class="hud-pill hud-pill-accent">{props.viewModel.roundLabel}</span>
          <span class="hud-pill">Turn {props.viewModel.turnCount}</span>
          <Show when={isCharacterHitPhase()}>
            <span class="hud-pill pill-danger">Character Hit</span>
          </Show>
        </span>
        <span class="combat-hud-right">
          <button
            class="combat-settings-btn"
            onClick={props.onOpenSettings}
            aria-label={props.viewModel.settingsLabel}
            data-testid="combat-settings-btn"
          >
            {props.viewModel.settingsLabel}
          </button>
        </span>
      </header>

      {/* ── Battle Stage ─────────────────────────────────── */}
      <div class="combat-stage">
        <div class="combat-stage-bg" />
        <div class="combat-stage-fx" />

        <div class="combat-formation">
          {/* Heroes on the left */}
          <div class="combat-formation-side combat-formation-side--heroes">
            <For each={props.viewModel.party}>
              {(hero) => {
                const portraitUrl = resolveHeroPortrait({
                  heroId: hero.id,
                  classLabel: hero.classLabel,
                });
                return (
                  <div
                    class={`combat-actor combat-actor--hero${
                      hero.isHit ? " combat-actor--hit" : ""
                    }${hero.isActive ? " combat-actor--active" : ""}`}
                    data-actor-id={hero.id}
                    data-testid={`combat-hero-${hero.id}`}
                  >
                    <div class="combat-actor-portrait">
                      {portraitUrl ? (
                        <img
                          class="combat-actor-portrait-image"
                          src={portraitUrl}
                          alt=""
                          aria-hidden="true"
                        />
                      ) : (
                        <span class="combat-actor-initial">{hero.name[0]}</span>
                      )}
                    </div>
                    <div class="combat-actor-name">{hero.name}</div>
                    <div class="combat-actor-bars">
                      <div class="combat-actor-bar">
                        <div
                          class="combat-actor-bar-fill"
                          style={{
                            width: `${healthPercent(hero.hp)}%`,
                            background: healthBarColor(hero.hp),
                          }}
                        />
                      </div>
                    </div>
                  </div>
                );
              }}
            </For>
          </div>

          {/* VS indicator */}
          <div class="combat-vs-divider" aria-hidden="true">
            <span class="combat-vs-text">VS</span>
          </div>

          {/* Enemies on the right */}
          <div class="combat-formation-side combat-formation-side--enemies">
            <For each={props.viewModel.enemies}>
              {(enemy) => (
                <div
                  class={`combat-actor combat-actor--enemy${
                    enemy.isHit ? " combat-actor--hit" : ""
                  }`}
                  data-actor-id={enemy.id}
                  data-testid={`combat-enemy-${enemy.id}`}
                >
                  <div class="combat-actor-portrait combat-actor-portrait--enemy">
                    <span class="combat-actor-initial combat-actor-initial--enemy">
                      {enemy.name[0]}
                    </span>
                  </div>
                  <div class="combat-actor-name combat-actor-name--enemy">
                    {enemy.name}
                  </div>
                  <div class="combat-actor-bars">
                    <div class="combat-actor-bar">
                      <div
                        class="combat-actor-bar-fill"
                        style={{
                          width: `${healthPercent(enemy.hp)}%`,
                          background: healthBarColor(enemy.hp),
                        }}
                      />
                    </div>
                  </div>
                </div>
              )}
            </For>
          </div>
        </div>

        {/* Hit damage floater */}
        <Show when={isCharacterHitPhase() && props.viewModel.hitDamage}>
          <div class="combat-hit-floater" data-testid="combat-hit-damage">
            <span class="combat-hit-damage-value">-{props.viewModel.hitDamage}</span>
          </div>
        </Show>
      </div>

      {/* ── Bottom Panels ────────────────────────────────── */}
      <div class="combat-bottom-panels">
        {/* Left: Character status panel */}
        <div class="combat-status-panel" data-testid="combat-status-panel">
          <div class="combat-status-header">
            <span class="combat-status-eyebrow">Status</span>
            <Show when={isCharacterHitPhase()}>
              <span class="combat-status-tag combat-status-tag--hit">Hit</span>
            </Show>
          </div>

          <div class="combat-status-body">
            <div class="combat-status-portrait-wrap">
              <div class="combat-status-portrait">
                {(() => {
                  const hero = displayHero();
                  if (!hero) return null;
                  const url = resolveHeroPortrait({
                    heroId: hero.id,
                    classLabel: hero.classLabel,
                  });
                  return url ? (
                    <img
                      class="combat-status-portrait-image"
                      src={url}
                      alt=""
                      aria-hidden="true"
                    />
                  ) : (
                    <span class="combat-status-portrait-letter">
                      {hero.name[0]}
                    </span>
                  );
                })()}
              </div>
              <div class="combat-status-hero-info">
                <div class="combat-status-hero-name">{displayHero()?.name}</div>
                <div class="combat-status-hero-class">
                  {displayHero()?.classLabel}
                </div>
              </div>
            </div>

            <div class="combat-status-stats">
              <Show when={displayHero()}>
                {(hero) => (
                  <>
                    <div class="combat-stat-row">
                      <span class="combat-stat-label">HP</span>
                      <div class="combat-stat-bar">
                        <div
                          class="combat-stat-bar-fill"
                          style={{
                            width: `${healthPercent(hero().hp)}%`,
                            background: healthBarColor(hero().hp),
                          }}
                        />
                      </div>
                      <span class="combat-stat-value">{hero().hp}</span>
                    </div>
                    <div class="combat-stat-row">
                      <span class="combat-stat-label">ST</span>
                      <div class="combat-stat-bar">
                        <div
                          class="combat-stat-bar-fill"
                          style={{
                            width: `${stressPercent(hero().stress, hero().maxStress)}%`,
                            background: stressBarColor(hero().stress),
                          }}
                        />
                      </div>
                      <span class="combat-stat-value">{hero().stress}</span>
                    </div>
                  </>
                )}
              </Show>
            </div>

            {/* Skill slots */}
            <Show when={displayHero()}>
              {(hero) => (
                <div class="combat-skill-slots">
                  <For each={hero().skills}>
                    {(skill, index) => (
                      <button
                        class={`combat-skill-slot${
                          canUseSkills() && skill.isAvailable
                            ? " combat-skill-slot--available"
                            : " combat-skill-slot--cooldown"
                        }`}
                        onClick={() => {
                          if (canUseSkills() && skill.isAvailable) {
                            props.onUseSkill(`${hero().id}-skill-${index()}`);
                          }
                        }}
                        disabled={!canUseSkills() || !skill.isAvailable}
                        data-testid={`combat-skill-${index()}`}
                        title={skill.name}
                      >
                        <span class="combat-skill-slot-name">{skill.name}</span>
                      </button>
                    )}
                  </For>
                </div>
              )}
            </Show>
          </div>
        </div>

        {/* Right: Dungeon minimap */}
        <div class="combat-map-panel" data-testid="combat-map-panel">
          <div class="combat-map-header">
            <span class="combat-map-eyebrow">Dungeon Map</span>
          </div>
          <div class="combat-map-canvas">
            <div class="combat-map-grid">
              <For each={props.viewModel.roomMap.rooms}>
                {(room) => (
                  <div
                    class={getRoomNodeClass(room)}
                    style={{
                      left: `${room.x * 28 + 8}px`,
                      top: `${room.y * 28 + 8}px`,
                    }}
                    data-room-id={room.id}
                    data-room-kind={room.kind}
                    data-testid={`combat-map-room-${room.id}`}
                    title={room.kind}
                  />
                )}
              </For>
              <For each={props.viewModel.roomMap.connections}>
                {(conn) => {
                  const fromRoom = props.viewModel.roomMap.rooms.find(
                    (r) => r.id === conn.from
                  );
                  const toRoom = props.viewModel.roomMap.rooms.find(
                    (r) => r.id === conn.to
                  );
                  if (!fromRoom || !toRoom) return null;
                  const x1 = fromRoom.x * 28 + 8 + 8;
                  const y1 = fromRoom.y * 28 + 8 + 8;
                  const x2 = toRoom.x * 28 + 8 + 8;
                  const y2 = toRoom.y * 28 + 8 + 8;
                  const length = Math.sqrt(
                    (x2 - x1) ** 2 + (y2 - y1) ** 2
                  );
                  const angle =
                    (Math.atan2(y2 - y1, x2 - x1) * 180) / Math.PI;
                  return (
                    <div
                      class="combat-map-connection"
                      style={{
                        left: `${x1}px`,
                        top: `${y1}px`,
                        width: `${length}px`,
                        transform: `rotate(${angle}deg)`,
                      }}
                      data-testid={`combat-map-conn-${conn.from}-${conn.to}`}
                    />
                  );
                }}
              </For>
            </div>
          </div>
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="combat-controls">
        <div class="combat-controls-left">
          <span class="combat-log" data-testid="combat-log">
            {props.viewModel.hitLog}
          </span>
        </div>
        <div class="combat-controls-right">
          <Show when={props.viewModel.isFleeAvailable}>
            <button
              class="action-secondary"
              onClick={props.onFleeCombat}
              data-testid="combat-flee-btn"
            >
              Flee
            </button>
          </Show>
          <button
            class="action-primary launch-primary"
            onClick={props.onContinueCombat}
            data-testid="combat-continue-btn"
          >
            {isCharacterHitPhase() ? "Acknowledge" : "Next Turn"}
          </button>
        </div>
      </footer>
    </div>
  );
};
