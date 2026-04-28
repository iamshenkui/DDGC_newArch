import type { Component } from "solid-js";

import type { ProvisioningViewModel } from "../../bridge/contractTypes";
import { AppFrame } from "../../components/layout/AppFrame";

interface ProvisioningScreenProps {
  viewModel: ProvisioningViewModel;
  onToggleHeroSelection: (heroId: string) => void;
  onConfirmProvisioning: () => void;
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
  const m = Number(maxStress) || 200;
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

export const ProvisioningScreen: Component<ProvisioningScreenProps> = (props) => {
  const selectedCount = () =>
    props.viewModel.party.filter((h) => h.isSelected).length;

  const woundedCount = () =>
    props.viewModel.party.filter((h) => h.isSelected && h.isWounded).length;

  const afflictedCount = () =>
    props.viewModel.party.filter((h) => h.isSelected && h.isAfflicted).length;

  return (
    <AppFrame
      eyebrow="Provisioning"
      title={props.viewModel.title}
      subtitle={`Expedition: ${props.viewModel.expeditionLabel}`}
    >
      <section class="grid">
        <div class="stack">
          <section class="panel stack">
            <div class="row">
              <span class="pill">Flow: provisioning</span>
              <span class="pill">
                Party: {selectedCount()} / {props.viewModel.maxPartySize}
              </span>
              {woundedCount() > 0 && (
                <span class="pill" style="color: #e8a838; border-color: rgba(232,168,56,0.3);">
                  {woundedCount()} wounded
                </span>
              )}
              {afflictedCount() > 0 && (
                <span class="pill" style="color: #ea7767; border-color: rgba(234,119,103,0.3);">
                  {afflictedCount()} afflicted
                </span>
              )}
            </div>
            <div class="surface-card stack">
              <h3>Party Selection</h3>
              <p>
                Select heroes to join the expedition. Review their status before
                departing.
              </p>
            </div>
            <ul class="list-reset">
              {props.viewModel.party.map((hero) => {
                const hpInfo = parseHp(hero.hp);
                const stressNum = Number(hero.stress);
                const stressMax = Number(hero.maxStress || 200);
                return (
                  <li class="surface-card stack">
                    <div class="row">
                      <label class="checkbox-label">
                        <input
                          type="checkbox"
                          checked={hero.isSelected}
                          onChange={() => props.onToggleHeroSelection(hero.id)}
                          disabled={
                            !hero.isSelected &&
                            selectedCount() >= props.viewModel.maxPartySize
                          }
                        />
                        <strong>{hero.name}</strong>
                      </label>
                      <span class="pill">{hero.classLabel}</span>
                      <span class="pill">Lv {hero.level}</span>
                      {hero.isWounded && (
                        <span class="pill" style="color: #e8a838; border-color: rgba(232,168,56,0.3);">Wounded</span>
                      )}
                      {hero.isAfflicted && (
                        <span class="pill" style="color: #ea7767; border-color: rgba(234,119,103,0.3);">Afflicted</span>
                      )}
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
                          {hero.stress} / {stressMax}
                        </span>
                        <div class="bar-container">
                          <div
                            class={stressBarClass(hero.stress)}
                            style={{ width: `${stressPercent(hero.stress, hero.maxStress)}%` }}
                          />
                        </div>
                      </div>
                    </div>
                    <div class="row" style="gap: 6px; font-size: 0.82rem; color: var(--panel-muted);">
                      <span>XP: {hero.xp}</span>
                    </div>
                  </li>
                );
              })}
            </ul>
          </section>
        </div>

        <div class="stack">
          <section class="panel stack">
            <h2 class="panel-title">Expedition Details</h2>
            <div class="surface-card stack">
              <div class="row">
                <span class="stat-label">Expedition</span>
                <span class="stat-value">{props.viewModel.expeditionLabel}</span>
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
            <h2 class="panel-title">Summary</h2>
            <div class="surface-card">
              <p>{props.viewModel.expeditionSummary}</p>
            </div>
          </section>

          <section class="panel stack">
            <div class="surface-card stack">
              <h3>Party Readiness</h3>
              <div class="row">
                <span class="stat-label">Selected</span>
                <span class="stat-value">{selectedCount()} / {props.viewModel.maxPartySize}</span>
              </div>
              {woundedCount() > 0 && (
                <div class="row">
                  <span class="stat-label" style="color: #e8a838;">Wounded</span>
                  <span class="stat-value" style="color: #e8a838;">{woundedCount()}</span>
                </div>
              )}
              {afflictedCount() > 0 && (
                <div class="row">
                  <span class="stat-label" style="color: #ea7767;">Afflicted</span>
                  <span class="stat-value" style="color: #ea7767;">{afflictedCount()}</span>
                </div>
              )}
            </div>
          </section>

          <section class="panel stack">
            <div class="row">
              <button
                class="action-secondary"
                onClick={props.onReturnToTown}
              >
                Return to Town
              </button>
              <button
                class="action-primary"
                onClick={props.onConfirmProvisioning}
                disabled={!props.viewModel.isReadyToLaunch}
              >
                {props.viewModel.isReadyToLaunch
                  ? "Confirm & Launch Expedition"
                  : "Select Party Members"}
              </button>
            </div>
          </section>
        </div>
      </section>
    </AppFrame>
  );
};
