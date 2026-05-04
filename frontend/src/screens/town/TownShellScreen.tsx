import { For, type Component } from "solid-js";

import type { TownViewModel } from "../../bridge/contractTypes";
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

function healthBarColor(hp: string): string {
  const pct = healthPercent(hp);
  if (pct >= 80) return "#5bbd6e";
  if (pct >= 40) return "#e8a838";
  return "#ea7767";
}

/**
 * Map conceptual building positions on the estate landscape.
 * These approximate the EstateManagement.unity scene layout:
 * stagecoach at the gate (left), guild at the training yard (right),
 * blacksmith at the forge (center-right), sanitarium at the clinic (center-left).
 */
const BUILDING_POSITIONS: Record<string, { left: string; top: string }> = {
  stagecoach:  { left: "10%",  top: "55%" },
  guild:       { left: "68%",  top: "35%" },
  blacksmith:  { left: "48%",  top: "58%" },
  sanitarium:  { left: "28%",  top: "32%" },
  abbey:       { left: "75%",  top: "62%" },
  tavern:      { left: "18%",  top: "68%" },
  market:      { left: "58%",  top: "48%" },
  graveyard:   { left: "82%",  top: "28%" },
  museum:      { left: "40%",  top: "22%" },
  provisioner: { left: "52%",  top: "70%" },
  sanctuary:   { left: "30%",  top: "48%" },
  inn:         { left: "65%",  top: "55%" },
};

const ROSTER_BUILDING_POSITIONS: Record<string, string> = {
  ready:   "status-ready",
  partial: "status-partial",
  locked:  "status-locked",
};

const buildingStatusLabel: Record<string, string> = {
  ready:   "Open",
  partial: "Visit",
  locked:  "Locked",
};

export const TownShellScreen: Component<TownShellScreenProps> = (props) => {
  return (
    <div class="town-viewport">
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="viewport-hud">
        <span class="viewport-hud-left">
          <span class="eyebrow">Town / Meta Surface</span>
          <h1 class="viewport-title">{props.viewModel.campaignName}</h1>
        </span>
        <span class="viewport-hud-center">
          <span class="hud-pill">Gold: {props.viewModel.gold}</span>
          {props.viewModel.isFreshVisit && (
            <span class="hud-pill hud-pill-accent">Fresh Visit</span>
          )}
        </span>
        <span class="viewport-hud-right">
          <span class="hud-pill">{props.viewModel.heroes.length} heroes</span>
          <button class="action-primary" onClick={props.onStartProvisioning}>
            {props.viewModel.nextActionLabel}
          </button>
        </span>
      </header>

      {/* ── Game Surface ────────────────────────────────── */}
      <PixiStage label="Town estate" rendererId="ddgc-town-stage">
        <For each={props.viewModel.buildings}>
          {(building) => {
            const pos = BUILDING_POSITIONS[building.id] ?? {
              left: `${20 + (building.id.length * 3) % 50}%`,
              top: `${30 + (building.id.length * 5) % 40}%`,
            };
            return (
              <button
                class="building-icon"
                style={{
                  left: pos.left,
                  top: pos.top,
                }}
                onClick={() => props.onOpenBuilding(building.id)}
                title={building.summary}
              >
                <span class="building-icon-marker" />
                <span class="building-icon-label">{building.label}</span>
                <span class={`building-icon-status ${ROSTER_BUILDING_POSITIONS[building.status] ?? "status-locked"}`}>
                  {buildingStatusLabel[building.status] ?? building.status}
                </span>
              </button>
            );
          }}
        </For>
      </PixiStage>

      {/* ── Roster Bar (HeroSlot style) ─────────────────── */}
      <section class="viewport-roster">
        <div class="roster-scroll">
          <For each={props.viewModel.heroes}>
            {(hero) => {
              const hpInfo = parseHp(hero.hp);
              const stressNum = Number(hero.stress);
              return (
                <button
                  class="roster-hero"
                  onClick={() => props.onOpenHero(hero.id)}
                >
                  {/* Portrait area — mirrors HeroSlot Frame + Image */}
                  <div class="roster-hero-portrait">
                    <div class="roster-portrait-frame" />
                    <div class="roster-portrait-avatar">
                      <span class="roster-portrait-letter">
                        {hero.classLabel[0]}
                      </span>
                    </div>
                    {/* Level badge */}
                    <span class="roster-portrait-level">Lv{hero.level}</span>
                    {/* Status overlay */}
                    {(hero.isWounded || hero.isAfflicted) && (
                      <div class="roster-portrait-status">
                        {hero.isAfflicted ? "A" : "W"}
                      </div>
                    )}
                  </div>

                  {/* Info area — mirrors HeroSlot.HeroLabel */}
                  <div class="roster-hero-info">
                    <span class="roster-hero-name">{hero.name}</span>
                    <span class="roster-hero-class">{hero.classLabel}</span>
                  </div>

                  {/* Bars — HP / Stress */}
                  <div class="roster-hero-bars">
                    <div class="roster-bar-row">
                      <div class="roster-bar-label">HP</div>
                      <div class="roster-bar-track">
                        <div
                          class="roster-bar-fill"
                          style={{
                            width: `${healthPercent(hero.hp)}%`,
                            background: healthBarColor(hero.hp),
                          }}
                        />
                      </div>
                    </div>
                    <div class="roster-bar-row">
                      <div class="roster-bar-label">ST</div>
                      <div class="roster-bar-track">
                        <div
                          class="roster-bar-fill"
                          style={{
                            width: `${Math.min((stressNum / Number(hero.maxStress || 200)) * 100, 100)}%`,
                            background: stressNum > 40 ? "#ea7767" : "#e8a838",
                          }}
                        />
                      </div>
                    </div>
                  </div>

                  {/* Numeric values footer */}
                  <div class="roster-hero-footer">
                    <span class="roster-footer-hp">{hpInfo.current}/{hpInfo.max}</span>
                    <span class="roster-footer-stress">{hero.stress}/{hero.maxStress}</span>
                  </div>
                </button>
              );
            }}
          </For>
        </div>
      </section>
    </div>
  );
};
