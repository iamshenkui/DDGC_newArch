import { For, type Component } from "solid-js";

import type { ExpeditionSetupViewModel } from "../../bridge/contractTypes";

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
 * Expedition launch screen — landscape viewport layout.
 *
 * Pre-launch review showing party vitals, expedition details, and warnings.
 * References original Unity prefab structure from:
 *   Assets/Prefabs/UI/ExpeditionWindow.prefab (estimated)
 *
 * Supply icon reference:
 *   data-asset-path="Assets/Resources/Sprites/inv_supply+rattle_drum.png"
 *   data-guid="e401bf9b9275ede4aa2ff50d13cc6207"
 */
export const ExpeditionScreen: Component<ExpeditionScreenProps> = (props) => {
  return (
    <div class="expedition-viewport">
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">Expedition Launch</span>
          <h1 class="expedition-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class="hud-pill hud-pill-accent">{props.viewModel.expeditionName}</span>
          <span class="hud-pill">Party: {props.viewModel.partySize} heroes</span>
          <span class="hud-pill">Difficulty: {props.viewModel.difficulty}</span>
          <span
            class="hud-pill"
            data-asset-path="Assets/Resources/Sprites/inv_supply+rattle_drum.png"
            data-guid="e401bf9b9275ede4aa2ff50d13cc6207"
          >
            Supply: {props.viewModel.supplyLevel}
          </span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="expedition-content">
          {/* Party vitals */}
          {props.viewModel.party.length > 0 && (
            <div class="expedition-hero-row">
              <For each={props.viewModel.party}>
                {(hero) => (
                  <div class="vitals-card">
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
                )}
              </For>
            </div>
          )}

          {/* Expedition details overlay */}
          <div class="details-overlay">
            <h2 class="details-overlay-title">Expedition Details</h2>
            <div class="details-overlay-row">
              <span class="details-overlay-label">Expedition</span>
              <span class="details-overlay-value">{props.viewModel.expeditionName}</span>
            </div>
            <div class="details-overlay-row">
              <span class="details-overlay-label">Party Size</span>
              <span class="details-overlay-value">{props.viewModel.partySize}</span>
            </div>
            <div class="details-overlay-row">
              <span class="details-overlay-label">Difficulty</span>
              <span class="details-overlay-value">{props.viewModel.difficulty}</span>
            </div>
            <div class="details-overlay-row">
              <span class="details-overlay-label">Est. Duration</span>
              <span class="details-overlay-value">{props.viewModel.estimatedDuration}</span>
            </div>
            <div class="details-overlay-row">
              <span class="details-overlay-label">Supply Level</span>
              <span class="details-overlay-value">{props.viewModel.supplyLevel}</span>
            </div>
            <div class="details-overlay-row">
              <span class="details-overlay-label">Provision Cost</span>
              <span class="details-overlay-value">{props.viewModel.provisionCost}</span>
            </div>

            {props.viewModel.objectives.length > 0 && (
              <div class="details-overlay-objectives">
                <div class="details-overlay-objectives-title">Objectives</div>
                <For each={props.viewModel.objectives}>
                  {(obj) => (
                    <div class="details-overlay-objective">{obj}</div>
                  )}
                </For>
              </div>
            )}
          </div>

          {/* Warnings */}
          {props.viewModel.warnings.length > 0 && (
            <div class="details-overlay" style="border-color: rgba(234, 119, 103, 0.3);">
              <For each={props.viewModel.warnings}>
                {(warning) => (
                  <div class="details-overlay-warning">{warning}</div>
                )}
              </For>
            </div>
          )}
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left">
          <span class="hud-pill" style="font-size: 0.72rem;">
            {props.viewModel.partySize} hero{props.viewModel.partySize !== 1 ? "es" : ""} selected &mdash; difficulty <strong>{props.viewModel.difficulty}</strong>
            {props.viewModel.warnings.length > 0
              ? ` — ${props.viewModel.warnings.length} warning${props.viewModel.warnings.length !== 1 ? "s" : ""}`
              : ""}
          </span>
        </div>
        <div class="expedition-controls-right">
          <button class="action-secondary" onClick={props.onReturnToTown}>
            Return to Town
          </button>
          <button
            class="action-primary"
            onClick={props.onLaunchExpedition}
            disabled={!props.viewModel.isLaunchable}
          >
            {props.viewModel.isLaunchable
              ? "Launch Expedition"
              : "Expedition Not Ready"}
          </button>
        </div>
      </footer>
    </div>
  );
};
