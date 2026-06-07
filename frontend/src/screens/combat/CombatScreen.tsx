import { For, Show, type Component, createSignal } from "solid-js";

import type { CombatViewModel } from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface CombatScreenProps {
  viewModel: CombatViewModel;
  onSelectSkill: (skillId: string) => void;
  onSelectTarget: (enemyId: string) => void;
  onConfirmAttack: () => void;
  onFleeCombat: () => void;
  onEndTurn: () => void;
  onContinueCombat?: () => void;
  onOpenSettings?: () => void;
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
 * Combat screen — dungeon scene character attack phase.
 *
 * Mirrors the reference image layout:
 *   - Top: combat arena with party on left, enemies on right
 *   - Bottom-left: active character detail panel (portrait, stats, skills)
 *   - Bottom-right: target/action selection panel
 *
 * Reference: 副本场景-人物攻击.png
 */
export const CombatScreen: Component<CombatScreenProps> = (props) => {
  const [hoveredSkillId, setHoveredSkillId] = createSignal<string | null>(null);

  const activeHero = () =>
    props.viewModel.party.find((h) => h.id === props.viewModel.activeHeroId) ??
    props.viewModel.party[0];

  const hitHero = () =>
    props.viewModel.party.find((h) => h.id === props.viewModel.hitTargetHeroId);

  const displayHero = () => hitHero() ?? activeHero();

  const isCharacterHitPhase = () => props.viewModel.phase === "character-hit";

  const hoveredSkill = () =>
    displayHero()?.skills.find((s) => s.id === hoveredSkillId());

  const portraitUrl = () =>
    resolveHeroPortrait({
      heroId: displayHero()?.id ?? "",
      classLabel: displayHero()?.classLabel ?? ""
    });

  return (
    <div
      class="combat-viewport"
      data-source-scene="UI_Combat/CombatScene"
      data-source-prefab="Assets/Prefabs/UI/CombatWindow.prefab"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="combat-hud">
        <span class="combat-hud-left">
          <span class="eyebrow">
            {props.viewModel.dungeonName ?? "Combat"} — {props.viewModel.roundLabel ?? `Round ${props.viewModel.round}`}
          </span>
          <h1 class="combat-title">{props.viewModel.title}</h1>
        </span>
        <span class="combat-hud-center">
          <Show
            when={isCharacterHitPhase()}
            fallback={
              <span class="hud-pill hud-pill-accent">
                {props.viewModel.turnPhase === "player" ? "Player Turn" : "Enemy Turn"}
              </span>
            }
          >
            <span class="hud-pill hud-pill-accent pill-danger">Character Hit</span>
          </Show>
          <span class="hud-pill">
            Active: {displayHero()?.name}
          </span>
        </span>
        <span class="combat-hud-right">
          <button
            class="combat-settings-btn"
            onClick={() => props.onOpenSettings?.()}
            aria-label="设置"
            title="设置"
          >
            设置
          </button>
        </span>
      </header>

      {/* ── Combat Arena ─────────────────────────────────── */}
      <div class="combat-arena">
        <div class="combat-arena-bg" />
        <div class="combat-arena-fx" />

        {/* Party formation on the left */}
        <div class="combat-party-line">
          <For each={props.viewModel.party}>
            {(hero) => {
              const isActive = hero.id === props.viewModel.activeHeroId;
              const hpPct = healthPercent(hero.hp);
              const stPct = stressPercent(hero.stress, hero.maxStress);
              const heroPortrait = resolveHeroPortrait({
                heroId: hero.id,
                classLabel: hero.classLabel
              });
              return (
                <div
                  class={`combat-hero-stand${isActive ? " combat-hero-stand--active" : ""}${hero.isHit ? " combat-hero-stand--hit" : ""}${!hero.isAlive ? " combat-hero-stand--dead" : ""}`}
                  data-hero-id={hero.id}
                  data-testid={`combat-hero-${hero.id}`}
                >
                  <div class="combat-hero-portrait-wrap">
                    <div
                      class={`combat-hero-portrait${heroPortrait ? " combat-hero-portrait--image" : " combat-hero-portrait--fallback"}`}
                    >
                      {heroPortrait ? (
                        <img
                          class="combat-hero-portrait-image"
                          src={heroPortrait}
                          alt=""
                          aria-hidden="true"
                        />
                      ) : (
                        <span class="combat-hero-portrait-letter">
                          {hero.classLabel[0]}
                        </span>
                      )}
                    </div>
                    <Show when={isActive}>
                      <div class="combat-hero-active-ring" aria-hidden="true" />
                    </Show>
                  </div>
                  <div class="combat-hero-info">
                    <div class="combat-hero-name">{hero.name}</div>
                    <div class="combat-hero-class">{hero.classLabel}</div>
                    <div class="combat-hero-bars">
                      <div class="combat-bar-row">
                        <div class="combat-bar-track">
                          <div
                            class="combat-bar-fill"
                            style={{
                              width: `${hpPct}%`,
                              background: healthBarColor(hero.hp)
                            }}
                          />
                        </div>
                      </div>
                      <div class="combat-bar-row">
                        <div class="combat-bar-track combat-bar-track--stress">
                          <div
                            class="combat-bar-fill"
                            style={{
                              width: `${stPct}%`,
                              background: stressBarColor(hero.stress)
                            }}
                          />
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              );
            }}
          </For>
        </div>

        {/* Enemy formation on the right */}
        <div class="combat-enemy-line">
          <For each={props.viewModel.enemies}>
            {(enemy) => {
              const hpPct = healthPercent(enemy.hp);
              return (
                <div
                  class={`combat-enemy-stand${enemy.isTargeted ? " combat-enemy-stand--targeted" : ""}${enemy.isHit ? " combat-enemy-stand--hit" : ""}${!enemy.isAlive ? " combat-enemy-stand--dead" : ""}`}
                  data-enemy-id={enemy.id}
                  data-testid={`combat-enemy-${enemy.id}`}
                  onClick={() => props.onSelectTarget(enemy.id)}
                  role="button"
                  tabindex={0}
                  onKeyDown={(e) => {
                    if (e.key === "Enter" || e.key === " ") {
                      props.onSelectTarget(enemy.id);
                    }
                  }}
                >
                  <div class="combat-enemy-body">
                    <div class={`combat-enemy-sprite combat-enemy-sprite--${enemy.size}`}>
                      <span class="combat-enemy-initial">{enemy.name[0]}</span>
                    </div>
                    <Show when={enemy.isTargeted}>
                      <div class="combat-enemy-target-marker" aria-hidden="true">
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#ea7767" stroke-width="2">
                          <circle cx="12" cy="12" r="8" />
                          <line x1="12" y1="2" x2="12" y2="6" />
                          <line x1="12" y1="18" x2="12" y2="22" />
                          <line x1="2" y1="12" x2="6" y2="12" />
                          <line x1="18" y1="12" x2="22" y2="12" />
                        </svg>
                      </div>
                    </Show>
                  </div>
                  <div class="combat-enemy-info">
                    <div class="combat-enemy-name">{enemy.name}</div>
                    <div class="combat-enemy-hp-bar">
                      <div class="combat-bar-track combat-bar-track--enemy">
                        <div
                          class="combat-bar-fill"
                          style={{
                            width: `${hpPct}%`,
                            background: healthBarColor(enemy.hp)
                          }}
                        />
                      </div>
                      <span class="combat-enemy-hp-text">{enemy.hp}</span>
                    </div>
                  </div>
                </div>
              );
            }}
          </For>
        </div>

        {/* Combat log overlay */}
        <div class="combat-log-overlay">
          <For each={props.viewModel.combatLog}>
            {(entry) => (
              <div class="combat-log-entry">{entry}</div>
            )}
          </For>
        </div>

        <Show when={isCharacterHitPhase() && props.viewModel.hitDamage}>
          <div class="combat-hit-floater" data-testid="combat-hit-damage">
            <span class="combat-hit-damage-value">-{props.viewModel.hitDamage}</span>
          </div>
        </Show>
      </div>

      {/* ── Bottom Panels ────────────────────────────────── */}
      <div class="combat-bottom-panels">
        {/* Left: Active character detail */}
        <section class="combat-char-panel">
          <header class="combat-char-header">
            <span class="combat-char-frame-rule" aria-hidden="true" />
            <h2 class="combat-char-title">{displayHero()?.name} — {displayHero()?.classLabel}</h2>
            <span class="combat-char-frame-rule" aria-hidden="true" />
          </header>

          <div class="combat-char-body">
            <div class="combat-char-portrait-col">
              <div
                class={`combat-char-portrait${portraitUrl() ? " combat-char-portrait--image" : " combat-char-portrait--fallback"}`}
              >
                {portraitUrl() ? (
                  <img
                    class="combat-char-portrait-image"
                    src={portraitUrl()}
                    alt=""
                    aria-hidden="true"
                  />
                ) : (
                  <span class="combat-char-portrait-letter">
                    {displayHero()?.classLabel[0]}
                  </span>
                )}
              </div>
              <div class="combat-char-stats">
                <div class="combat-stat-row">
                  <span class="combat-stat-label">HP</span>
                  <span class="combat-stat-value">{displayHero()?.hp}</span>
                </div>
                <div class="combat-stat-row">
                  <span class="combat-stat-label">ST</span>
                  <span class="combat-stat-value">{displayHero()?.stress} / {displayHero()?.maxStress}</span>
                </div>
              </div>
            </div>

            <div class="combat-char-skills-col">
              <div class="combat-skills-title">Skills</div>
              <div class="combat-skill-slots">
                <For each={displayHero()?.skills}>
                  {(skill) => {
                    const isSelected = skill.id === props.viewModel.selectedSkillId;
                    const isOnCooldown = skill.cooldownRemaining > 0;
                    const isDisabled = isCharacterHitPhase() || isOnCooldown;
                    return (
                      <button
                        class={`combat-skill-slot${isSelected ? " combat-skill-slot--selected" : ""}${isOnCooldown ? " combat-skill-slot--cooldown" : ""}`}
                        onClick={() => !isDisabled && props.onSelectSkill(skill.id)}
                        onMouseEnter={() => setHoveredSkillId(skill.id)}
                        onMouseLeave={() => setHoveredSkillId(null)}
                        disabled={isDisabled}
                        title={`${skill.name}${isOnCooldown ? ` (CD: ${skill.cooldownRemaining})` : ""}`}
                      >
                        <span class="combat-skill-icon">{skill.name[0]}</span>
                        <Show when={isOnCooldown}>
                          <span class="combat-skill-cd-badge">{skill.cooldownRemaining}</span>
                        </Show>
                      </button>
                    );
                  }}
                </For>
              </div>

              <Show when={hoveredSkillId()}>
                <div class="combat-skill-tooltip">
                  <div class="combat-skill-tooltip-name">{hoveredSkill()?.name}</div>
                  <div class="combat-skill-tooltip-desc">{hoveredSkill()?.description}</div>
                  <div class="combat-skill-tooltip-stats">
                    <span class="combat-skill-tooltip-stat">Target: {hoveredSkill()?.target}</span>
                    <span class="combat-skill-tooltip-stat">Hit: {hoveredSkill()?.hitRating}</span>
                    <span class="combat-skill-tooltip-stat">Crit: {hoveredSkill()?.critRating}</span>
                  </div>
                </div>
              </Show>
            </div>
          </div>
        </section>

        {/* Right: Target / action panel */}
        <section class="combat-target-panel">
          <header class="combat-target-header">
            <span class="combat-target-frame-rule" aria-hidden="true" />
            <h2 class="combat-target-title">Target Selection</h2>
            <span class="combat-target-frame-rule" aria-hidden="true" />
          </header>

          <div class="combat-target-body">
            <div class="combat-target-grid">
              <For each={props.viewModel.enemies}>
                {(enemy) => {
                  const hpPct = healthPercent(enemy.hp);
                  return (
                    <button
                      class={`combat-target-cell${enemy.isTargeted ? " combat-target-cell--selected" : ""}${!enemy.isAlive ? " combat-target-cell--dead" : ""}`}
                      onClick={() => enemy.isAlive && props.onSelectTarget(enemy.id)}
                      disabled={!enemy.isAlive}
                    >
                      <div class="combat-target-cell-sprite">
                        <span class="combat-target-cell-initial">{enemy.name[0]}</span>
                      </div>
                      <div class="combat-target-cell-name">{enemy.name}</div>
                      <div class="combat-target-cell-hp">
                        <div class="combat-target-cell-hp-bar">
                          <div
                            class="combat-target-cell-hp-fill"
                            style={{
                              width: `${hpPct}%`,
                              background: healthBarColor(enemy.hp)
                            }}
                          />
                        </div>
                        <span class="combat-target-cell-hp-text">{enemy.hp}</span>
                      </div>
                    </button>
                  );
                }}
              </For>
            </div>

            <div class="combat-action-row">
              <Show
                when={isCharacterHitPhase()}
                fallback={
                  <button
                    class="action-primary combat-attack-btn"
                    onClick={props.onConfirmAttack}
                    disabled={!props.viewModel.isPlayerTurn || !props.viewModel.selectedSkillId}
                  >
                    Confirm Attack
                  </button>
                }
              >
                <button
                  class="action-primary combat-attack-btn"
                  onClick={() => props.onContinueCombat?.()}
                  data-testid="combat-continue-btn"
                >
                  Acknowledge
                </button>
              </Show>
              <button
                class="action-secondary combat-end-turn-btn"
                onClick={props.onEndTurn}
                disabled={isCharacterHitPhase() || !props.viewModel.isPlayerTurn}
              >
                End Turn
              </button>
            </div>

            <Show when={props.viewModel.hitLog}>
              <div class="combat-hit-log" data-testid="combat-log">
                {props.viewModel.hitLog}
              </div>
            </Show>

            <Show when={props.viewModel.canFlee}>
              <button
                class="combat-flee-btn"
                onClick={props.onFleeCombat}
              >
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M15 3h4a2 2 0 012 2v14a2 2 0 01-2 2h-4" />
                  <polyline points="10 17 15 12 10 7" />
                  <line x1="15" y1="12" x2="3" y2="12" />
                </svg>
                Flee
              </button>
            </Show>
          </div>
        </section>
      </div>
    </div>
  );
};
