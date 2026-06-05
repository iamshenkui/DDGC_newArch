import { For, Show, type Component } from "solid-js";

import type { CombatViewModel, CombatHeroState, CombatEnemyState, CombatSkill } from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface CombatScreenProps {
  viewModel: CombatViewModel;
  onSelectHero: (heroId: string) => void;
  onUseSkill: (skillId: string) => void;
  onRetreat: () => void;
  onAdvance: () => void;
}

function parseHp(hp: string, maxHp: string): number {
  const c = Number(hp);
  const m = Number(maxHp || 1);
  if (m <= 0) return 0;
  return Math.round((c / m) * 100);
}

function healthBarColor(hpPct: number): string {
  if (hpPct >= 60) return "#5bbd6e";
  if (hpPct >= 30) return "#e8a838";
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
  if (s <= 50) return "#e8a838";
  return "#ea7767";
}

function enemyThreatClass(threat: string): string {
  switch (threat) {
    case "boss":
      return "combat-enemy-threat--boss";
    case "elite":
      return "combat-enemy-threat--elite";
    default:
      return "combat-enemy-threat--normal";
  }
}

function skillTargetIcon(targetType: string): string {
  switch (targetType) {
    case "enemy":
      return "⚔";
    case "ally":
      return "🛡";
    case "self":
      return "◆";
    case "party":
      return "✦";
    default:
      return "?";
  }
}

/**
 * Dungeon combat screen — turn-based battle viewport.
 *
 * References original Unity prefab structure from:
 *   Assets/Prefabs/UI/CombatWindow.prefab (estimated)
 *
 * Layout mirrors the reference image:
 *   Top bar: retreat button, round indicator, settings button
 *   Battle stage: player party (left) vs enemies (right) on dark terrain
 *   Bottom panels: hero detail/skills (left), action/map panel (right)
 */
export const CombatScreen: Component<CombatScreenProps> = (props) => {
  const selectedHero = () => {
    if (!props.viewModel.selectedHeroId) return null;
    return props.viewModel.party.find((h) => h.heroId === props.viewModel.selectedHeroId) ?? null;
  };

  const aliveParty = () => props.viewModel.party.filter((h) => h.isAlive);
  const aliveEnemies = () => props.viewModel.enemies.filter((e) => e.isAlive);

  return (
    <div
      class="combat-viewport"
      data-source-scene="UI_Combat/CombatWindow"
      data-source-prefab="Assets/Prefabs/UI/CombatWindow.prefab"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="combat-hud">
        <span class="combat-hud-left">
          <button
            class="combat-retreat-btn"
            onClick={props.onRetreat}
            disabled={!props.viewModel.isRetreatAvailable}
            data-testid="combat-retreat-btn"
          >
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M19 12H5" />
              <polyline points="12 19 5 12 12 5" />
            </svg>
            撤退
          </button>
        </span>

        <span class="combat-hud-center">
          <span class="combat-round-badge" data-testid="combat-round-badge">
            <span class="combat-round-label">Round</span>
            <span class="combat-round-number">{props.viewModel.round}</span>
          </span>
          <span class="combat-phase-pill">
            {props.viewModel.turnPhase === "player" ? "我方回合" : props.viewModel.turnPhase === "enemy" ? "敌方回合" : "结算中"}
          </span>
          <span class="hud-pill hud-pill-accent">{props.viewModel.expeditionName}</span>
        </span>

        <span class="combat-hud-right">
          <button class="combat-settings-btn" data-testid="combat-settings-btn">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="3" />
              <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
            </svg>
            设置
          </button>
        </span>
      </header>

      {/* ── Battle Stage ─────────────────────────────────── */}
      <div class="combat-stage">
        <div class="combat-stage-bg" />
        <div class="combat-stage-terrain" />

        <div class="combat-formation">
          {/* Player party — left side */}
          <div class="combat-party">
            <For each={aliveParty()}>
              {(hero) => {
                const portraitUrl = resolveHeroPortrait({
                  heroId: hero.heroId,
                  classLabel: hero.classLabel
                });
                const hpPct = parseHp(hero.hp, hero.maxHp);
                const stPct = stressPercent(hero.stress, hero.maxStress);
                const isSelected = hero.heroId === props.viewModel.selectedHeroId;
                return (
                  <div
                    class={`combat-hero-tile${isSelected ? " combat-hero-tile--selected" : ""}`}
                    onClick={() => props.onSelectHero(hero.heroId)}
                    data-hero-id={hero.heroId}
                    data-testid={`combat-hero-tile-${hero.heroId}`}
                  >
                    <div class="combat-hero-portrait-wrap">
                      {portraitUrl ? (
                        <img
                          class="combat-hero-portrait"
                          src={portraitUrl}
                          alt={hero.name}
                        />
                      ) : (
                        <div class="combat-hero-portrait combat-hero-portrait--fallback">
                          <span class="combat-hero-initial">{hero.name[0]}</span>
                        </div>
                      )}
                    </div>
                    <div class="combat-hero-name">{hero.name}</div>
                    <div class="combat-hero-bars">
                      <div class="combat-bar-row">
                        <div class="combat-bar-track">
                          <div
                            class="combat-bar-fill"
                            style={{
                              width: `${hpPct}%`,
                              background: healthBarColor(hpPct),
                            }}
                          />
                        </div>
                        <span class="combat-bar-value">{hero.hp}</span>
                      </div>
                      <div class="combat-bar-row">
                        <div class="combat-bar-track combat-bar-track--stress">
                          <div
                            class="combat-bar-fill"
                            style={{
                              width: `${stPct}%`,
                              background: stressBarColor(hero.stress),
                            }}
                          />
                        </div>
                        <span class="combat-bar-value combat-bar-value--stress">{hero.stress}</span>
                      </div>
                    </div>
                    <Show when={!hero.isAlive}>
                      <div class="combat-hero-overlay">倒下</div>
                    </Show>
                  </div>
                );
              }}
            </For>
          </div>

          {/* VS divider */}
          <div class="combat-vs-divider" aria-hidden="true">
            <div class="combat-vs-line" />
            <span class="combat-vs-mark">VS</span>
            <div class="combat-vs-line" />
          </div>

          {/* Enemy party — right side */}
          <div class="combat-enemy-party">
            <For each={aliveEnemies()}>
              {(enemy) => {
                const hpPct = parseHp(enemy.hp, enemy.maxHp);
                return (
                  <div
                    class={`combat-enemy-tile${enemy.threatLevel === "boss" ? " combat-enemy-tile--boss" : ""}`}
                    data-enemy-id={enemy.enemyId}
                    data-testid={`combat-enemy-tile-${enemy.enemyId}`}
                  >
                    <div class="combat-enemy-portrait-wrap">
                      <div class="combat-enemy-portrait">
                        <span class="combat-enemy-initial">{enemy.name[0]}</span>
                      </div>
                    </div>
                    <div class="combat-enemy-name">{enemy.name}</div>
                    <span class={`combat-enemy-threat ${enemyThreatClass(enemy.threatLevel)}`}>
                      {enemy.threatLevel === "boss" ? "BOSS" : enemy.threatLevel === "elite" ? "ELITE" : "NORMAL"}
                    </span>
                    <div class="combat-enemy-bars">
                      <div class="combat-bar-row">
                        <div class="combat-bar-track">
                          <div
                            class="combat-bar-fill"
                            style={{
                              width: `${hpPct}%`,
                              background: healthBarColor(hpPct),
                            }}
                          />
                        </div>
                        <span class="combat-bar-value">{enemy.hp}</span>
                      </div>
                    </div>
                  </div>
                );
              }}
            </For>
          </div>
        </div>

        {/* Combat log strip */}
        <Show when={props.viewModel.combatLog.length > 0}>
          <div class="combat-log-strip">
            <For each={props.viewModel.combatLog.slice(-3)}>
              {(entry) => (
                <span class="combat-log-entry">{entry}</span>
              )}
            </For>
          </div>
        </Show>
      </div>

      {/* ── Bottom Panels ────────────────────────────────── */}
      <div class="combat-bottom-panels">
        {/* Left panel: selected hero detail + skills */}
        <div class="combat-panel combat-panel--hero">
          <Show
            when={selectedHero()}
            fallback={
              <div class="combat-panel-empty">
                <span class="combat-panel-empty-text">选择一名英雄进行操作</span>
              </div>
            }
          >
            {(hero) => {
              const portraitUrl = resolveHeroPortrait({
                heroId: hero.heroId,
                classLabel: hero.classLabel
              });
              return (
                <>
                  <div class="combat-hero-detail">
                    <div class="combat-hero-detail-portrait">
                      {portraitUrl ? (
                        <img
                          class="combat-hero-detail-portrait-image"
                          src={portraitUrl}
                          alt={hero.name}
                        />
                      ) : (
                        <div class="combat-hero-detail-portrait--fallback">
                          <span>{hero.name[0]}</span>
                        </div>
                      )}
                    </div>
                    <div class="combat-hero-detail-info">
                      <div class="combat-hero-detail-name">{hero.name}</div>
                      <div class="combat-hero-detail-class">{hero.classLabel}</div>
                      <div class="combat-hero-detail-stats">
                        <span class="combat-stat">HP {hero.hp}/{hero.maxHp}</span>
                        <span class="combat-stat">ST {hero.stress}/{hero.maxStress}</span>
                      </div>
                    </div>
                  </div>

                  <div class="combat-skills-grid">
                    <For each={props.viewModel.availableSkills}>
                      {(skill) => (
                        <button
                          class={`combat-skill-btn${!skill.isAvailable ? " combat-skill-btn--disabled" : ""}${skill.cooldownRemaining > 0 ? " combat-skill-btn--cooldown" : ""}`}
                          onClick={() => skill.isAvailable && skill.cooldownRemaining === 0 && props.onUseSkill(skill.skillId)}
                          disabled={!skill.isAvailable || skill.cooldownRemaining > 0}
                          data-skill-id={skill.skillId}
                          data-testid={`combat-skill-${skill.skillId}`}
                        >
                          <span class="combat-skill-icon">{skillTargetIcon(skill.targetType)}</span>
                          <span class="combat-skill-name">{skill.name}</span>
                          <Show when={skill.cooldownRemaining > 0}>
                            <span class="combat-skill-cooldown">{skill.cooldownRemaining}</span>
                          </Show>
                        </button>
                      )}
                    </For>
                  </div>
                </>
              );
            }}
          </Show>
        </div>

        {/* Right panel: action / terrain / advance */}
        <div class="combat-panel combat-panel--action">
          <div class="combat-action-header">
            <span class="combat-action-title">战场情报</span>
            <span class="combat-action-terrain">{props.viewModel.terrainLabel}</span>
          </div>

          <div class="combat-action-summary">
            <div class="combat-summary-row">
              <span class="combat-summary-label">我方存活</span>
              <span class="combat-summary-value">{aliveParty().length} / {props.viewModel.party.length}</span>
            </div>
            <div class="combat-summary-row">
              <span class="combat-summary-label">敌方存活</span>
              <span class="combat-summary-value">{aliveEnemies().length} / {props.viewModel.enemies.length}</span>
            </div>
            <div class="combat-summary-row">
              <span class="combat-summary-label">当前回合</span>
              <span class="combat-summary-value">{props.viewModel.round}</span>
            </div>
          </div>

          <div class="combat-action-controls">
            <button
              class="action-primary combat-advance-btn"
              onClick={props.onAdvance}
              disabled={props.viewModel.turnPhase !== "player"}
              data-testid="combat-advance-btn"
            >
              {props.viewModel.turnPhase === "player" ? "结束回合" : "等待中…"}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
