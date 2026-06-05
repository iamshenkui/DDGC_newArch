import { For, type Component } from "solid-js";

import type { CombatViewModel } from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface CombatScreenProps {
  viewModel: CombatViewModel;
  onUseSkill: (skillId: string) => void;
  onFleeCombat: () => void;
  onAutoResolveCombat: () => void;
  onNextTurn: () => void;
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
 * Combat screen — dungeon battle viewport.
 *
 * Displays the active combat encounter with party formation,
 * enemy formation, turn order, combat log, and skill controls.
 * References original Unity prefab structure from:
 *   Assets/Prefabs/UI/CombatWindow.prefab (estimated)
 *
 * Source hierarchy: UI_Combat/CombatWindow
 *   TopPanel → RoundLabel + TurnIndicator
 *   PartyPanel → HeroCombatCard × 4
 *   EnemyPanel → EnemyCombatCard × 4
 *   SkillPanel → SkillButton × N
 *   LogPanel → CombatLogEntry × N
 *   ControlPanel → FleeButton + AutoResolveButton
 */
export const CombatScreen: Component<CombatScreenProps> = (props) => {
  const turnLabel = () => {
    if (props.viewModel.turn === "party") {
      return `Party Turn — ${props.viewModel.activeActorName}`;
    }
    return `Enemy Turn — ${props.viewModel.activeActorName}`;
  };

  const turnPillClass = () => {
    if (props.viewModel.turn === "party") {
      return "hud-pill hud-pill-accent";
    }
    return "hud-pill pill-danger";
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
          <span class="eyebrow">Combat</span>
          <h1 class="combat-title">{props.viewModel.title}</h1>
        </span>
        <span class="combat-hud-center">
          <span class="hud-pill hud-pill-accent">
            Round {props.viewModel.round}
          </span>
          <span class={turnPillClass()}>
            {turnLabel()}
          </span>
          <span class="hud-pill">
            {props.viewModel.encounterName}
          </span>
        </span>
      </header>

      {/* ── Battle Surface ───────────────────────────────── */}
      <div class="combat-surface">
        <div class="combat-surface-bg" />
        <div class="combat-surface-mist" />

        <div class="combat-content">
          {/* Party vs Enemy layout */}
          <div class="combat-formation">
            {/* Party side */}
            <div class="combat-party">
              <div class="combat-side-label">Party</div>
              <For each={props.viewModel.party}>
                {(hero) => {
                  const portraitUrl = resolveHeroPortrait({
                    heroId: hero.id,
                    classLabel: hero.classLabel
                  });
                  const isActive = hero.isActive && props.viewModel.turn === "party";
                  return (
                    <div
                      class={`combat-actor-card ${isActive ? "combat-actor-card--active" : ""} ${!hero.isAlive ? "combat-actor-card--dead" : ""}`}
                      data-actor-id={hero.id}
                      data-actor-type="hero"
                    >
                      <div
                        class={`combat-actor-portrait${portraitUrl ? " combat-actor-portrait--image" : " combat-actor-portrait--fallback"}`}
                      >
                        {portraitUrl ? (
                          <img
                            class="combat-actor-portrait-image"
                            src={portraitUrl}
                            alt=""
                            aria-hidden="true"
                          />
                        ) : (
                          <span class="combat-actor-initial">
                            {hero.name[0]}
                          </span>
                        )}
                      </div>
                      <div class="combat-actor-name">{hero.name}</div>
                      <div class="combat-actor-sub">{hero.classLabel}</div>
                      {hero.isAlive && (
                        <div class="combat-actor-bars">
                          <div class="combat-bar-row">
                            <div class="combat-bar-label">HP</div>
                            <div class="combat-bar-track">
                              <div
                                class="combat-bar-fill"
                                style={{
                                  width: `${healthPercent(hero.hp)}%`,
                                  background: healthBarColor(hero.hp),
                                }}
                              />
                            </div>
                            <div class="combat-bar-value">{hero.hp}</div>
                          </div>
                          <div class="combat-bar-row">
                            <div class="combat-bar-label">ST</div>
                            <div class="combat-bar-track">
                              <div
                                class="combat-bar-fill"
                                style={{
                                  width: `${stressPercent(hero.stress, hero.maxStress)}%`,
                                  background: stressBarColor(hero.stress),
                                }}
                              />
                            </div>
                            <div class="combat-bar-value">{hero.stress}</div>
                          </div>
                        </div>
                      )}
                      {!hero.isAlive && (
                        <div class="combat-actor-dead-label">Defeated</div>
                      )}
                    </div>
                  );
                }}
              </For>
            </div>

            {/* VS divider */}
            <div class="combat-vs-divider" aria-hidden="true">
              <div class="combat-vs-line" />
              <span class="combat-vs-text">VS</span>
              <div class="combat-vs-line" />
            </div>

            {/* Enemy side */}
            <div class="combat-enemy">
              <div class="combat-side-label combat-side-label--enemy">Enemies</div>
              <For each={props.viewModel.enemies}>
                {(enemy) => {
                  const isActive = enemy.isActive && props.viewModel.turn === "enemy";
                  return (
                    <div
                      class={`combat-actor-card combat-actor-card--enemy ${isActive ? "combat-actor-card--active" : ""} ${!enemy.isAlive ? "combat-actor-card--dead" : ""}`}
                      data-actor-id={enemy.id}
                      data-actor-type="enemy"
                    >
                      <div class="combat-actor-portrait combat-actor-portrait--enemy">
                        <span class="combat-actor-initial combat-actor-initial--enemy">
                          {enemy.name[0]}
                        </span>
                      </div>
                      <div class="combat-actor-name">{enemy.name}</div>
                      {enemy.isAlive && (
                        <div class="combat-actor-bars">
                          <div class="combat-bar-row">
                            <div class="combat-bar-label">HP</div>
                            <div class="combat-bar-track">
                              <div
                                class="combat-bar-fill"
                                style={{
                                  width: `${healthPercent(enemy.hp)}%`,
                                  background: healthBarColor(enemy.hp),
                                }}
                              />
                            </div>
                            <div class="combat-bar-value">{enemy.hp}</div>
                          </div>
                        </div>
                      )}
                      {!enemy.isAlive && (
                        <div class="combat-actor-dead-label">Defeated</div>
                      )}
                    </div>
                  );
                }}
              </For>
            </div>
          </div>

          {/* Combat log */}
          {props.viewModel.combatLog.length > 0 && (
            <div class="combat-log">
              <div class="combat-log-header">
                <span class="combat-log-title">Combat Log</span>
              </div>
              <div class="combat-log-entries">
                <For each={props.viewModel.combatLog}>
                  {(entry, index) => (
                    <div
                      class={`combat-log-entry ${index() === props.viewModel.combatLog.length - 1 ? "combat-log-entry--latest" : ""}`}
                    >
                      {entry}
                    </div>
                  )}
                </For>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="combat-controls">
        <div class="combat-controls-left">
          {props.viewModel.turn === "party" && props.viewModel.availableSkills.length > 0 && (
            <div class="combat-skill-panel">
              <span class="combat-skill-label">Skills</span>
              <div class="combat-skill-row">
                <For each={props.viewModel.availableSkills}>
                  {(skill) => (
                    <button
                      class={`combat-skill-btn ${skill.isAvailable ? "" : "combat-skill-btn--disabled"}`}
                      onClick={() => skill.isAvailable && props.onUseSkill(skill.id)}
                      disabled={!skill.isAvailable}
                      title={skill.description}
                    >
                      <span class="combat-skill-name">{skill.name}</span>
                      <span class="combat-skill-target">{skill.target}</span>
                    </button>
                  )}
                </For>
              </div>
            </div>
          )}
          {props.viewModel.turn === "enemy" && (
            <span class="combat-enemy-turn-label">
              Enemies are acting...
            </span>
          )}
        </div>
        <div class="combat-controls-right">
          <button
            class="action-secondary"
            onClick={props.onFleeCombat}
            disabled={!props.viewModel.isFleeAvailable}
          >
            Flee
          </button>
          <button
            class="action-secondary"
            onClick={props.onAutoResolveCombat}
            disabled={!props.viewModel.isAutoResolveAvailable}
          >
            Auto
          </button>
          <button
            class="action-primary launch-primary"
            onClick={props.onNextTurn}
          >
            Next Turn
          </button>
        </div>
      </footer>
    </div>
  );
};
