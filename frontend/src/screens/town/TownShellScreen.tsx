import type { Component } from "solid-js";

import type { TownViewModel } from "../../bridge/contractTypes";
import { AppFrame } from "../../components/layout/AppFrame";
import { PixiStage } from "../../render/PixiStage";

interface TownShellScreenProps {
  viewModel: TownViewModel;
  onOpenHero: (heroId: string) => void;
  onOpenBuilding: (buildingId: string) => void;
  onStartProvisioning: () => void;
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

function healthClass(hp: string): string {
  const pct = healthPercent(hp);
  if (pct >= 80) return "text-good";
  if (pct >= 40) return "text-warning";
  return "text-danger";
}

function healthBarClass(hp: string): string {
  const pct = healthPercent(hp);
  if (pct >= 80) return "bar-fill bar-fill-health";
  if (pct >= 40) return "bar-fill bar-fill-health-warning";
  return "bar-fill bar-fill-health-danger";
}

function stressLevelClass(stress: string): string {
  const s = Number(stress);
  if (s <= 20) return "text-good";
  if (s <= 40) return "text-warning";
  return "text-danger";
}

function stressBarClass(stress: string): string {
  const s = Number(stress);
  if (s <= 40) return "bar-fill bar-fill-stress";
  return "bar-fill bar-fill-stress-high";
}

export const TownShellScreen: Component<TownShellScreenProps> = (props) => {
  return (
    <AppFrame
      eyebrow="Town / Meta Surface"
      title={props.viewModel.campaignName}
      subtitle={props.viewModel.campaignSummary}
    >
      <section class="grid">
        <div class="stack">
          <section class="panel stack">
            <div class="row">
              <span class="pill">Flow: town</span>
              <span class="pill">Gold: {props.viewModel.gold}</span>
              {props.viewModel.isFreshVisit && (
                <span class="pill" style="color: #c6d46a;">Fresh Visit</span>
              )}
              <span class="pill">Next: {props.viewModel.nextActionLabel}</span>
            </div>
            <div class="row">
              <button class="action-primary" onClick={props.onStartProvisioning}>
                {props.viewModel.nextActionLabel}
              </button>
            </div>
          </section>

          <section class="panel stack">
            <h2 class="panel-title">Roster Summary</h2>
            <div class="surface-card">
              <p>
                {props.viewModel.heroes.length} hero{props.viewModel.heroes.length !== 1 ? "es" : ""} available in the roster.
                Review hero status before provisioning for expedition.
              </p>
            </div>
            <ul class="list-reset">
              {props.viewModel.heroes.map((hero) => {
                const hpInfo = parseHp(hero.hp);
                const stressNum = Number(hero.stress);
                return (
                  <li class="surface-card stack">
                    <div class="row">
                      <strong>{hero.name}</strong>
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
                        <span class={`stat-value ${healthClass(hero.hp)}`}>
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
                        <span class={`stat-value ${stressLevelClass(hero.stress)}`}>
                          {hero.stress} / {hero.maxStress}
                        </span>
                        <div class="bar-container">
                          <div
                            class={stressBarClass(hero.stress)}
                            style={{ width: `${Math.min((stressNum / Number(hero.maxStress || 200)) * 100, 100)}%` }}
                          />
                        </div>
                      </div>
                    </div>
                    <div class="row" style="gap: 6px; font-size: 0.82rem; color: var(--panel-muted);">
                      <span>XP: {hero.xp}</span>
                      {hero.positiveQuirks.length > 0 && (
                        <span>+{hero.positiveQuirks.length} quirk{hero.positiveQuirks.length !== 1 ? "s" : ""}</span>
                      )}
                      {hero.negativeQuirks.length > 0 && (
                        <span style="color: #ea7767;">-{hero.negativeQuirks.length} quirk{hero.negativeQuirks.length !== 1 ? "s" : ""}</span>
                      )}
                      {hero.diseases.length > 0 && (
                        <span style="color: #ea7767;">{hero.diseases.length} disease{hero.diseases.length !== 1 ? "s" : ""}</span>
                      )}
                    </div>
                    <div class="row">
                      <button
                        class="action-secondary"
                        onClick={() => props.onOpenHero(hero.id)}
                      >
                        Inspect Hero
                      </button>
                    </div>
                  </li>
                );
              })}
            </ul>
          </section>

          <PixiStage label="Town stage layer" rendererId="ddgc-town-stage" />
        </div>
        <div class="stack">
          <section class="panel stack">
            <h2 class="panel-title">Buildings</h2>
            <ul class="list-reset">
              {props.viewModel.buildings.map((building) => (
                <li class="surface-card stack">
                  <div class="row">
                    <strong>{building.label}</strong>
                    <span class="pill">{building.status}</span>
                  </div>
                  <p>{building.summary}</p>
                  <div class="row">
                    <button
                      class="action-secondary"
                      onClick={() => props.onOpenBuilding(building.id)}
                    >
                      Open Building
                    </button>
                  </div>
                </li>
              ))}
            </ul>
          </section>
        </div>
      </section>
    </AppFrame>
  );
};
