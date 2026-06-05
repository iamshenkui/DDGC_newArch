import { For, type Component, createMemo } from "solid-js";

import type { CombatViewModel, CombatantVital } from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface CombatScreenProps {
  viewModel: CombatViewModel;
  onResolveCombat: () => void;
  onReturnToExpedition: () => void;
  onCombatAction?: (actionId: string) => void;
}

function healthPercent(healthFraction: number): number {
  return Math.round(healthFraction * 100);
}

function healthBarColor(healthFraction: number): string {
  const pct = healthPercent(healthFraction);
  if (pct >= 80) return "#5bbd6e";
  if (pct >= 40) return "#e8a838";
  return "#ea7767";
}

function phaseLabel(phase: CombatViewModel["phase"]): string {
  switch (phase) {
    case "PreBattle":
      return "战斗准备";
    case "HeroTurn":
      return "英雄回合";
    case "MonsterTurn":
      return "敌方回合";
    case "Resolution":
      return "结算中";
    case "PostBattle":
      return "战斗结束";
    default:
      return "未知阶段";
  }
}

function phaseClass(phase: CombatViewModel["phase"]): string {
  switch (phase) {
    case "HeroTurn":
      return "combat-phase--hero";
    case "MonsterTurn":
      return "combat-phase--monster";
    case "Resolution":
      return "combat-phase--resolution";
    case "PostBattle":
      return "combat-phase--ended";
    default:
      return "combat-phase--neutral";
  }
}

function resultLabel(result: NonNullable<CombatViewModel["result"]>): string {
  switch (result) {
    case "Victory":
      return "胜利";
    case "Defeat":
      return "失败";
    case "Fled":
      return "撤退";
    case "Draw":
      return "平局";
    default:
      return String(result);
  }
}

function resultClass(result: NonNullable<CombatViewModel["result"]>): string {
  switch (result) {
    case "Victory":
      return "combat-result--victory";
    case "Defeat":
      return "combat-result--defeat";
    case "Fled":
      return "combat-result--fled";
    default:
      return "combat-result--draw";
  }
}

const CombatantCard: Component<{
  vital: CombatantVital;
  index: number;
  isHero: boolean;
}> = (props) => {
  const portraitUrl = createMemo(() =>
    props.isHero
      ? resolveHeroPortrait({
          heroId: props.vital.id,
          classLabel: props.vital.id.split("-")[2] ?? "Hunter"
        })
      : undefined
  );

  const cardClass = () => {
    const base = "combatant-card";
    if (props.vital.isDead) return `${base} combatant-card--dead`;
    if (props.vital.isAtDeathsDoor) return `${base} combatant-card--dying`;
    return base;
  };

  const initial = () => {
    const parts = props.vital.id.split("-");
    return parts[2]?.[0]?.toUpperCase() ?? "?";
  };

  return (
    <div class={cardClass()} data-combatant-id={props.vital.id} data-combatant-type={props.vital.combatantType}>
      <div
        class={`combatant-portrait${portraitUrl() ? " combatant-portrait--image" : " combatant-portrait--fallback"}`}
      >
        {portraitUrl() ? (
          <img class="combatant-portrait-image" src={portraitUrl()} alt="" aria-hidden="true" />
        ) : (
          <span class="combatant-initial">{initial()}</span>
        )}
      </div>
      <div class="combatant-name">{props.vital.id}</div>
      <div class="combatant-hp-bar">
        <div class="combatant-hp-track">
          <div
            class="combatant-hp-fill"
            style={{
              width: `${healthPercent(props.vital.healthFraction)}%`,
              background: healthBarColor(props.vital.healthFraction)
            }}
          />
        </div>
        <span class="combatant-hp-label">{healthPercent(props.vital.healthFraction)}%</span>
      </div>
      {props.vital.statusCount > 0 && (
        <div class="combatant-status-badges">
          <span class="combatant-status-badge">{props.vital.statusCount} 状态</span>
        </div>
      )}
      {props.vital.isAtDeathsDoor && !props.vital.isDead && (
        <div class="combatant-death-door-badge">濒死</div>
      )}
      {props.vital.isDead && (
        <div class="combatant-dead-overlay">
          <span>阵亡</span>
        </div>
      )}
    </div>
  );
};

/**
 * Combat screen — dungeon battle scene with heroes, monsters, and action panel.
 *
 * Layout mirrors the reference image:
 *   - Top HUD: round indicator, phase label, return/settings
 *   - Battle arena: heroes on left, monsters on right
 *   - Bottom panel: hero details left, action/skills right
 *
 * References original Unity prefab structure from:
 *   Assets/Prefabs/UI/CombatWindow.prefab (estimated)
 */
export const CombatScreen: Component<CombatScreenProps> = (props) => {
  const isCombatEnded = () =>
    props.viewModel.result !== undefined || props.viewModel.phase === "PostBattle";

  const turnLabel = () => {
    if (isCombatEnded()) {
      return props.viewModel.result ? resultLabel(props.viewModel.result) : "战斗结束";
    }
    return phaseLabel(props.viewModel.phase);
  };

  const turnClass = () => {
    if (isCombatEnded() && props.viewModel.result) {
      return resultClass(props.viewModel.result);
    }
    return phaseClass(props.viewModel.phase);
  };

  const currentActor = () => {
    if (!props.viewModel.currentTurnActorId) return undefined;
    return (
      props.viewModel.heroVitals.find((h) => h.id === props.viewModel.currentTurnActorId) ??
      props.viewModel.monsterVitals.find((m) => m.id === props.viewModel.currentTurnActorId)
    );
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
          <button class="combat-return-btn" onClick={props.onReturnToExpedition} aria-label="返回">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M19 12H5" />
              <path d="M12 19l-7-7 7-7" />
            </svg>
            返回
          </button>
        </span>

        <span class="combat-hud-center">
          <span class="combat-round-pill">
            <span class="combat-round-icon" aria-hidden="true">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10" />
                <polyline points="12 6 12 12 16 14" />
              </svg>
            </span>
            第 {props.viewModel.round} 回合
          </span>
          <span class={`combat-phase-pill ${turnClass()}`}>{turnLabel()}</span>
          {isCombatEnded() && props.viewModel.result && (
            <span class={`combat-result-banner ${turnClass()}`}>
              {resultLabel(props.viewModel.result!)}
            </span>
          )}
        </span>

        <span class="combat-hud-right">
          <span class="combat-alive-count">
            <span class="combat-alive-heroes">{props.viewModel.heroesAlive} 英雄</span>
            <span class="combat-alive-divider" aria-hidden="true">/</span>
            <span class="combat-alive-monsters">{props.viewModel.monstersAlive} 敌人</span>
          </span>
        </span>
      </header>

      {/* ── Battle Arena ─────────────────────────────────── */}
      <div class="combat-arena">
        <div class="combat-arena-bg" />
        <div class="combat-arena-mist" />

        {/* Heroes side */}
        <div class="combat-formation combat-formation--heroes">
          <div class="combat-formation-label">英雄</div>
          <div class="combat-formation-row">
            <For each={props.viewModel.heroVitals}>
              {(vital, index) => (
                <CombatantCard vital={vital} index={index()} isHero={true} />
              )}
            </For>
          </div>
        </div>

        {/* VS divider */}
        <div class="combat-vs-divider" aria-hidden="true">
          <span class="combat-vs-icon">VS</span>
        </div>

        {/* Monsters side */}
        <div class="combat-formation combat-formation--monsters">
          <div class="combat-formation-label">敌人</div>
          <div class="combat-formation-row">
            <For each={props.viewModel.monsterVitals}>
              {(vital, index) => (
                <CombatantCard vital={vital} index={index()} isHero={false} />
              )}
            </For>
          </div>
        </div>
      </div>

      {/* ── Bottom Panel ─────────────────────────────────── */}
      <div class="combat-bottom-panel">
        {/* Left: current actor info */}
        <section class="combat-actor-panel">
          <header class="combat-panel-header">
            <span class="combat-panel-frame-rule" aria-hidden="true" />
            <h2 class="combat-panel-title">
              {currentActor()
                ? `${currentActor()!.id} — ${currentActor()!.combatantType === "hero" ? "英雄" : "敌人"}`
                : "战斗信息"}
            </h2>
            <span class="combat-panel-frame-rule" aria-hidden="true" />
          </header>

          {currentActor() && (
            <div class="combat-actor-info">
              <div class="combat-actor-stat-row">
                <span class="combat-actor-stat-label">生命值</span>
                <span class="combat-actor-stat-value">
                  {Math.round(currentActor()!.healthFraction * 100)}%
                </span>
              </div>
              <div class="combat-actor-stat-row">
                <span class="combat-actor-stat-label">状态</span>
                <span class="combat-actor-stat-value">
                  {currentActor()!.isDead
                    ? "阵亡"
                    : currentActor()!.isAtDeathsDoor
                    ? "濒死"
                    : currentActor()!.statusCount > 0
                    ? `${currentActor()!.statusCount} 个状态效果`
                    : "正常"}
                </span>
              </div>
              <div class="combat-actor-stat-row">
                <span class="combat-actor-stat-label">回合</span>
                <span class="combat-actor-stat-value">{props.viewModel.round}</span>
              </div>
            </div>
          )}

          {!currentActor() && (
            <div class="combat-actor-info">
              <div class="combat-actor-stat-row">
                <span class="combat-actor-stat-label">当前回合</span>
                <span class="combat-actor-stat-value">第 {props.viewModel.round} 回合</span>
              </div>
              <div class="combat-actor-stat-row">
                <span class="combat-actor-stat-label">阶段</span>
                <span class="combat-actor-stat-value">{phaseLabel(props.viewModel.phase)}</span>
              </div>
            </div>
          )}
        </section>

        {/* Right: action panel */}
        <section class="combat-action-panel">
          <header class="combat-panel-header">
            <span class="combat-panel-frame-rule" aria-hidden="true" />
            <h2 class="combat-panel-title">行动</h2>
            <span class="combat-panel-frame-rule" aria-hidden="true" />
          </header>

          <div class="combat-action-grid">
            {props.viewModel.phase === "HeroTurn" && !isCombatEnded() && (
              <>
                <button
                  class="combat-action-btn combat-action-btn--attack"
                  onClick={() => props.onCombatAction?.("attack")}
                  disabled={props.viewModel.isResolving}
                >
                  <span class="combat-action-icon" aria-hidden="true">⚔️</span>
                  <span class="combat-action-label">攻击</span>
                </button>
                <button
                  class="combat-action-btn combat-action-btn--defend"
                  onClick={() => props.onCombatAction?.("defend")}
                  disabled={props.viewModel.isResolving}
                >
                  <span class="combat-action-icon" aria-hidden="true">🛡️</span>
                  <span class="combat-action-label">防御</span>
                </button>
                <button
                  class="combat-action-btn combat-action-btn--skill"
                  onClick={() => props.onCombatAction?.("skill")}
                  disabled={props.viewModel.isResolving}
                >
                  <span class="combat-action-icon" aria-hidden="true">✨</span>
                  <span class="combat-action-label">技能</span>
                </button>
                <button
                  class="combat-action-btn combat-action-btn--item"
                  onClick={() => props.onCombatAction?.("item")}
                  disabled={props.viewModel.isResolving}
                >
                  <span class="combat-action-icon" aria-hidden="true">🎒</span>
                  <span class="combat-action-label">物品</span>
                </button>
              </>
            )}

            {props.viewModel.phase === "MonsterTurn" && !isCombatEnded() && (
              <div class="combat-action-waiting">
                <span class="combat-action-waiting-icon" aria-hidden="true">💀</span>
                <span class="combat-action-waiting-text">敌方行动中…</span>
              </div>
            )}

            {props.viewModel.phase === "Resolution" && !isCombatEnded() && (
              <div class="combat-action-waiting">
                <span class="combat-action-waiting-icon" aria-hidden="true">⚡</span>
                <span class="combat-action-waiting-text">结算中…</span>
              </div>
            )}

            {isCombatEnded() && (
              <div class="combat-action-ended">
                <span class={`combat-action-ended-result ${turnClass()}`}>
                  {props.viewModel.result ? resultLabel(props.viewModel.result) : "战斗结束"}
                </span>
                <button class="action-primary combat-resolve-btn" onClick={props.onResolveCombat}>
                  继续
                </button>
              </div>
            )}

            {props.viewModel.phase === "PreBattle" && (
              <div class="combat-action-waiting">
                <span class="combat-action-waiting-icon" aria-hidden="true">🛡️</span>
                <span class="combat-action-waiting-text">战斗准备中…</span>
              </div>
            )}
          </div>
        </section>
      </div>
    </div>
  );
};
