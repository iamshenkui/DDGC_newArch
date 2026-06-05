import { For, type Component, createMemo } from "solid-js";

import type { CombatViewModel } from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface CombatScreenProps {
  viewModel: CombatViewModel;
  onSelectHero: (heroId: string) => void;
  onUseSkill: (skillId: string) => void;
  onRetreat: () => void;
  onFinishCombat: () => void;
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

function mapNodeIcon(type: string): string {
  switch (type) {
    case "entrance": return "◎";
    case "combat": return "⚔";
    case "boss": return "👹";
    case "treasure": return "◆";
    case "rest": return "🔥";
    case "exit": return "→";
    default: return "•";
  }
}

function mapNodeLabel(type: string): string {
  switch (type) {
    case "entrance": return "Entrance";
    case "combat": return "Combat";
    case "boss": return "Boss";
    case "treasure": return "Treasure";
    case "rest": return "Rest";
    case "exit": return "Exit";
    default: return "Room";
  }
}

/**
 * Dungeon Combat screen — mirrors the original Unity combat encounter layout.
 *
 * Reference: 副本场景-战斗.png
 * Layout:
 *   - Top HUD: round indicator, expedition name, settings
 *   - Battlefield: heroes on the left, enemies on the right
 *   - Bottom panel: selected hero stats/skills (left), dungeon map (right)
 */
export const CombatScreen: Component<CombatScreenProps> = (props) => {
  const selectedHero = createMemo(() =>
    props.viewModel.heroes.find((h) => h.id === props.viewModel.selectedHeroId) ??
    props.viewModel.heroes[0]
  );

  const torchColor = () => {
    const t = props.viewModel.torchLevel;
    if (t >= 75) return "#e8c84a";
    if (t >= 40) return "#e8a838";
    return "#ea7767";
  };

  return (
    <div
      class="combat-viewport"
      data-source-scene="UI_Combat/CombatWindow"
      data-source-prefab="Assets/Prefabs/UI/CombatWindow.prefab"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="combat-hud">
        <span class="combat-hud-left">
          <span class="eyebrow">Dungeon Combat</span>
          <h1 class="combat-title">{props.viewModel.expeditionName}</h1>
        </span>
        <span class="combat-hud-center">
          <span class="hud-pill hud-pill-accent">
            Round {props.viewModel.round}
          </span>
          <span class="hud-pill">
            Turn: {props.viewModel.turn === "player" ? "Your Turn" : "Enemy Turn"}
          </span>
          <span
            class="hud-pill"
            style={{ color: torchColor(), borderColor: torchColor() }}
          >
            Torch: {props.viewModel.torchLevel}%
          </span>
        </span>
        <span class="combat-hud-right">
          <button class="action-secondary" onClick={props.onRetreat}>
            Retreat
          </button>
        </span>
      </header>

      {/* ── Battlefield ─────────────────────────────────── */}
      <div class="combat-battlefield">
        <div class="combat-battlefield-bg" />
        <div class="combat-battlefield-mist" />

        {/* Heroes (left side) */}
        <div class="combat-formation combat-formation--heroes">
          <For each={props.viewModel.heroes}>
            {(hero) => {
              const portraitUrl = resolveHeroPortrait({
                heroId: hero.id,
                classLabel: hero.classLabel
              });
              const isSelected = hero.id === props.viewModel.selectedHeroId;
              return (
                <div
                  class={`combat-unit combat-unit--hero${isSelected ? " combat-unit--selected" : ""}`}
                  onClick={() => props.onSelectHero(hero.id)}
                  data-hero-id={hero.id}
                >
                  <div
                    class={`combat-unit-portrait${portraitUrl ? " combat-unit-portrait--image" : " combat-unit-portrait--fallback"}`}
                  >
                    {portraitUrl ? (
                      <img
                        class="combat-unit-portrait-image"
                        src={portraitUrl}
                        alt=""
                        aria-hidden="true"
                      />
                    ) : (
                      <span class="combat-unit-initial">
                        {hero.classLabel[0]}
                      </span>
                    )}
                  </div>
                  <div class="combat-unit-name">{hero.name}</div>
                  <div class="combat-unit-class">{hero.classLabel}</div>
                  <div class="combat-unit-bars">
                    <div class="combat-bar-row">
                      <div class="combat-bar-track">
                        <div
                          class="combat-bar-fill"
                          style={{
                            width: `${healthPercent(hero.hp)}%`,
                            background: healthBarColor(hero.hp),
                          }}
                        />
                      </div>
                      <span class="combat-bar-value">{hero.hp}</span>
                    </div>
                    <div class="combat-bar-row">
                      <div class="combat-bar-track">
                        <div
                          class="combat-bar-fill"
                          style={{
                            width: `${stressPercent(hero.stress, hero.maxStress)}%`,
                            background: stressBarColor(hero.stress),
                          }}
                        />
                      </div>
                      <span class="combat-bar-value combat-bar-value--stress">{hero.stress}</span>
                    </div>
                  </div>
                  {isSelected && (
                    <div class="combat-unit-turn-indicator" aria-hidden="true">
                      <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M12 2l-10 20h20L12 2z" />
                      </svg>
                    </div>
                  )}
                </div>
              );
            }}
          </For>
        </div>

        {/* VS divider */}
        <div class="combat-vs-divider" aria-hidden="true">
          <span class="combat-vs-text">VS</span>
        </div>

        {/* Enemies (right side) */}
        <div class="combat-formation combat-formation--enemies">
          <For each={props.viewModel.enemies}>
            {(enemy) => (
              <div
                class={`combat-unit combat-unit--enemy${enemy.size > 1 ? " combat-unit--large" : ""}`}
                data-enemy-id={enemy.id}
              >
                <div class="combat-unit-portrait combat-unit-portrait--enemy">
                  <span class="combat-unit-initial combat-unit-initial--enemy">
                    {enemy.name[0]}
                  </span>
                </div>
                <div class="combat-unit-name">{enemy.name}</div>
                <div class="combat-unit-bars">
                  <div class="combat-bar-row">
                    <div class="combat-bar-track">
                      <div
                        class="combat-bar-fill"
                        style={{
                          width: `${healthPercent(enemy.hp)}%`,
                          background: healthBarColor(enemy.hp),
                        }}
                      />
                    </div>
                    <span class="combat-bar-value">{enemy.hp}</span>
                  </div>
                </div>
              </div>
            )}
          </For>
        </div>
      </div>

      {/* ── Bottom Panel ────────────────────────────────── */}
      <div class="combat-bottom-panel">
        {/* Hero detail / skills (left) */}
        <div class="combat-hero-panel">
          <header class="combat-hero-panel-header">
            <span class="combat-hero-panel-title">Selected Hero</span>
            <span class="combat-hero-panel-name">
              {selectedHero()?.name} — {selectedHero()?.classLabel}
            </span>
          </header>

          <div class="combat-skills-grid">
            <For each={props.viewModel.skills}>
              {(skill) => (
                <button
                  class="combat-skill-btn"
                  onClick={() => props.onUseSkill(skill.id)}
                  disabled={skill.cooldown > 0}
                  data-skill-id={skill.id}
                >
                  <span class="combat-skill-name">{skill.name}</span>
                  <span class="combat-skill-desc">{skill.description}</span>
                  <span class="combat-skill-meta">
                    <span class="combat-skill-target">{skill.target}</span>
                    {skill.cooldown > 0 && (
                      <span class="combat-skill-cooldown">CD: {skill.cooldown}</span>
                    )}
                  </span>
                </button>
              )}
            </For>
          </div>

          <div class="combat-hero-stats">
            <div class="combat-stat">
              <span class="combat-stat-label">HP</span>
              <span class="combat-stat-value">{selectedHero()?.hp}</span>
            </div>
            <div class="combat-stat">
              <span class="combat-stat-label">Stress</span>
              <span class="combat-stat-value combat-stat-value--stress">{selectedHero()?.stress}</span>
            </div>
            <div class="combat-stat">
              <span class="combat-stat-label">Position</span>
              <span class="combat-stat-value">{selectedHero()?.position}</span>
            </div>
          </div>
        </div>

        {/* Dungeon map (right) */}
        <div class="combat-map-panel">
          <header class="combat-map-panel-header">
            <span class="combat-map-panel-title">Dungeon Map</span>
          </header>
          <div class="combat-map-track">
            <For each={props.viewModel.dungeonMap}>
              {(node, index) => (
                <>
                  <div
                    class={`combat-map-node${node.isCurrent ? " combat-map-node--current" : ""}${node.isExplored ? " combat-map-node--explored" : ""}`}
                    data-node-id={node.id}
                    title={mapNodeLabel(node.type)}
                  >
                    <span class="combat-map-node-icon">{mapNodeIcon(node.type)}</span>
                    <span class="combat-map-node-label">{mapNodeLabel(node.type)}</span>
                  </div>
                  {index() < props.viewModel.dungeonMap.length - 1 && (
                    <div class="combat-map-connector" aria-hidden="true" />
                  )}
                </>
              )}
            </For>
          </div>
        </div>
      </div>

      {/* ── Bottom Controls ─────────────────────────────── */}
      <footer class="combat-controls">
        <div class="combat-controls-left">
          <span class="hud-pill hud-pill-accent">
            {props.viewModel.heroes.length} Heroes vs {props.viewModel.enemies.length} Enemies
          </span>
        </div>
        <div class="combat-controls-right">
          <button class="action-secondary" onClick={props.onRetreat}>
            Retreat
          </button>
          <button
            class="action-primary launch-primary"
            onClick={props.onFinishCombat}
          >
            Finish Combat
          </button>
        </div>
      </footer>
    </div>
  );
};
