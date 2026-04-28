import type { Component } from "solid-js";

import type { ExpeditionSetupViewModel } from "../../bridge/contractTypes";
import { AppFrame } from "../../components/layout/AppFrame";

interface ExpeditionScreenProps {
  viewModel: ExpeditionSetupViewModel;
  onLaunchExpedition: () => void;
  onReturnToTown: () => void;
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

function healthBarClass(hp: string): string {
  const pct = healthPercent(hp);
  if (pct >= 80) return "bar-fill bar-fill-health";
  if (pct >= 40) return "bar-fill bar-fill-health-warning";
  return "bar-fill bar-fill-health-danger";
}

function healthTextClass(hp: string): string {
  const pct = healthPercent(hp);
  if (pct >= 80) return "text-good";
  if (pct >= 40) return "text-warning";
  return "text-danger";
}

function stressPercent(stress: string, maxStress: string): number {
  const s = Number(stress);
  const m = Number(maxStress || 200);
  return Math.min(Math.round((s / m) * 100), 100);
}

function stressBarClass(stress: string): string {
  const s = Number(stress);
  if (s <= 40) return "bar-fill bar-fill-stress";
  return "bar-fill bar-fill-stress-high";
}

function stressTextClass(stress: string): string {
  const s = Number(stress);
  if (s <= 20) return "text-good";
  if (s <= 40) return "text-warning";
  return "text-danger";
}

export const ExpeditionScreen: Component<ExpeditionScreenProps> = (props) => {
  return (
    <AppFrame
      eyebrow="Expedition Launch"
      title={props.viewModel.title}
      subtitle={`Difficulty: ${props.viewModel.difficulty}`}
    >
      <section class="grid">
        <div class="stack">
          <section class="panel stack">
            <div class="row">
              <span class="pill">Flow: expedition</span>
              <span class="pill">Party: {props.viewModel.partySize} heroes</span>
              <span class="pill" style="color: var(--panel-accent);">
                {props.viewModel.expeditionName}
              </span>
            </div>
            <div class="surface-card stack">
              <h3>Expedition Ready</h3>
              <p>
                Your party is provisioned and ready to depart. Review the
                expedition details before launching.
              </p>
            </div>
          </section>

          {props.viewModel.party.length > 0 && (
            <section class="panel stack">
              <h2 class="panel-title">Party Vitals</h2>
              <ul class="list-reset">
                {props.viewModel.party.map((hero) => {
                  const hpInfo = parseHp(hero.hp);
                  return (
                    <li class="surface-card stack">
                      <div class="row">
                        <strong>{hero.name}</strong>
                        <span class="pill">{hero.classLabel}</span>
                      </div>
                      <div class="stack">
                        <div class="bar-row">
                          <span class="stat-label">HP</span>
                          <span class={`stat-value ${healthTextClass(hero.hp)}`}>
                            {hpInfo.current} / {hpInfo.max}
                          </span>
                          <div class="bar-container">
                            <div
                              class={healthBarClass(hero.hp)}
                              style={{ width: `${healthPercent(hero.hp)}%` }}
                            />
                          </div>
                        </div>
                        <div class="bar-row">
                          <span class="stat-label">Stress</span>
                          <span class={`stat-value ${stressTextClass(hero.stress)}`}>
                            {hero.stress} / {hero.maxStress}
                          </span>
                          <div class="bar-container">
                            <div
                              class={stressBarClass(hero.stress)}
                              style={{ width: `${stressPercent(hero.stress, hero.maxStress)}%` }}
                            />
                          </div>
                        </div>
                      </div>
                    </li>
                  );
                })}
              </ul>
            </section>
          )}

          <section class="panel stack">
            <h2 class="panel-title">Expedition Details</h2>
            <div class="surface-card stack">
              <div class="row">
                <span class="stat-label">Expedition</span>
                <span class="stat-value">{props.viewModel.expeditionName}</span>
              </div>
              <div class="row">
                <span class="stat-label">Party Size</span>
                <span class="stat-value">{props.viewModel.partySize}</span>
              </div>
              <div class="row">
                <span class="stat-label">Difficulty</span>
                <span class="stat-value">{props.viewModel.difficulty}</span>
              </div>
              <div class="row">
                <span class="stat-label">Est. Duration</span>
                <span class="stat-value">{props.viewModel.estimatedDuration}</span>
              </div>
              <div class="row">
                <span class="stat-label">Supply Level</span>
                <span class="stat-value">{props.viewModel.supplyLevel}</span>
              </div>
              <div class="row">
                <span class="stat-label">Provision Cost</span>
                <span class="stat-value">{props.viewModel.provisionCost}</span>
              </div>
            </div>
          </section>

          <section class="panel stack">
            <h2 class="panel-title">Objectives</h2>
            <ul class="list-reset">
              {props.viewModel.objectives.map((objective) => (
                <li class="surface-card">
                  <span>{objective}</span>
                </li>
              ))}
            </ul>
          </section>

          {props.viewModel.warnings.length > 0 && (
            <section class="panel stack">
              <h2 class="panel-title">Warnings</h2>
              <ul class="list-reset">
                {props.viewModel.warnings.map((warning) => (
                  <li class="surface-card warning-card">
                    <span class="warning-text">{warning}</span>
                  </li>
                ))}
              </ul>
            </section>
          )}
        </div>

        <div class="stack">
          <section class="panel stack">
            <div class="surface-card stack">
              <h3>Pre-Launch Summary</h3>
              <p>
                {props.viewModel.partySize} hero{props.viewModel.partySize !== 1 ? "es" : ""} selected —
                difficulty <strong>{props.viewModel.difficulty}</strong> —
                {props.viewModel.warnings.length > 0
                  ? ` ${props.viewModel.warnings.length} warning${props.viewModel.warnings.length !== 1 ? "s" : ""} active`
                  : " no warnings"}
              </p>
            </div>
            <div class="stack" style="margin-top: 8px;">
              <button
                class="action-primary"
                onClick={props.onLaunchExpedition}
                disabled={!props.viewModel.isLaunchable}
              >
                {props.viewModel.isLaunchable
                  ? "Launch Expedition"
                  : "Expedition Not Ready"}
              </button>
              <button
                class="action-secondary"
                onClick={props.onReturnToTown}
              >
                Return to Town
              </button>
            </div>
          </section>
        </div>
      </section>
    </AppFrame>
  );
};
