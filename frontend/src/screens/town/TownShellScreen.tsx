import { For, type Component } from "solid-js";

import { resolveHeroPortrait } from "../../assets/originalAssetPaths";
import type { TownBuildingSummary, TownHeroSummary, TownViewModel } from "../../bridge/contractTypes";
import { BuildingIcon } from "./buildings/BuildingIcons";

interface TownShellScreenProps {
  viewModel: TownViewModel;
  onOpenHero: (heroId: string) => void;
  onOpenBuilding: (buildingId: string) => void;
  onStartProvisioning: () => void;
}

interface EstateBuildingLayout {
  x: number;
  y: number;
  width: number;
  height: number;
  sourceLabel: string;
  labelOffsetX: number;
  labelOffsetY: number;
}

const ESTATE_STAGE_WIDTH = 1600;
const ESTATE_STAGE_HEIGHT = 900;

const ESTATE_BUILDING_LAYOUT: Record<string, EstateBuildingLayout> = {
  stagecoach: {
    x: 31,
    y: -267,
    width: 384,
    height: 384,
    sourceLabel: "次元感知塔",
    labelOffsetX: -26,
    labelOffsetY: -150
  },
  guild: {
    x: 120,
    y: -50,
    width: 397,
    height: 397,
    sourceLabel: "试炼场",
    labelOffsetX: 30,
    labelOffsetY: -50
  },
  blacksmith: {
    x: 709,
    y: 294,
    width: 482,
    height: 482,
    sourceLabel: "锻造舱",
    labelOffsetX: 0,
    labelOffsetY: -168
  },
  sanitarium: {
    x: 390,
    y: 110,
    width: 339,
    height: 339,
    sourceLabel: "细胞修复站",
    labelOffsetX: 50,
    labelOffsetY: -100
  },
  abbey: {
    x: -330,
    y: 90,
    width: 519,
    height: 519,
    sourceLabel: "信仰祭坛",
    labelOffsetX: -10,
    labelOffsetY: 153
  },
  tavern: {
    x: -650,
    y: -220,
    width: 519,
    height: 519,
    sourceLabel: "迷情乐园",
    labelOffsetX: -6,
    labelOffsetY: 160
  },
  graveyard: {
    x: 696,
    y: 96,
    width: 344,
    height: 344,
    sourceLabel: "英雄档案馆",
    labelOffsetX: 100,
    labelOffsetY: -100
  }
};

const BUILDING_STATUS_LABEL: Record<TownBuildingSummary["status"], string> = {
  ready: "Open",
  partial: "Visit",
  locked: "Locked"
};

function parseHp(hp: string): { current: number; max: number } {
  const parts = hp.split("/").map((part) => Number(part.trim()));
  if (parts.length === 2 && parts.every((part) => Number.isFinite(part))) {
    return { current: parts[0], max: parts[1] };
  }
  return { current: 0, max: 1 };
}

function healthPercent(hero: TownHeroSummary): number {
  if (hero.maxHealth <= 0) return 0;
  return Math.max(0, Math.min(Math.round((hero.health / hero.maxHealth) * 100), 100));
}

function stressPercent(hero: TownHeroSummary): number {
  const maxStress = Number(hero.maxStress) || 200;
  const stress = Number(hero.stress) || 0;
  return Math.max(0, Math.min(Math.round((stress / maxStress) * 100), 100));
}

function healthBarColor(hero: TownHeroSummary): string {
  const pct = healthPercent(hero);
  if (pct >= 80) return "#72be71";
  if (pct >= 40) return "#d5a64a";
  return "#c65f52";
}

function stressBarColor(hero: TownHeroSummary): string {
  const pct = stressPercent(hero);
  if (pct >= 50) return "#b4504b";
  return "#8b6d3a";
}

function estateLeft(x: number): string {
  return `${50 + (x / ESTATE_STAGE_WIDTH) * 100}%`;
}

function estateTop(y: number): string {
  return `${50 - (y / ESTATE_STAGE_HEIGHT) * 100}%`;
}

function estateWidth(width: number): string {
  return `${(width / ESTATE_STAGE_WIDTH) * 100}%`;
}

function estateHeight(height: number): string {
  return `${(height / ESTATE_STAGE_HEIGHT) * 100}%`;
}

function buildingLayout(buildingId: string): EstateBuildingLayout {
  return ESTATE_BUILDING_LAYOUT[buildingId] ?? {
    x: 0,
    y: 0,
    width: 360,
    height: 360,
    sourceLabel: buildingId,
    labelOffsetX: 0,
    labelOffsetY: 120
  };
}

function rosterHeroes(viewModel: TownViewModel): ReadonlyArray<TownHeroSummary> {
  return viewModel.roster.length > 0 ? viewModel.roster : viewModel.heroes;
}

export const TownShellScreen: Component<TownShellScreenProps> = (props) => {
  return (
    <div class="town-viewport estate-town-screen">
      <header class="viewport-hud estate-top-panel">
        <div class="estate-name-card">
          <span class="eyebrow">Town / Meta Surface</span>
          <h1 class="estate-campaign-name">{props.viewModel.campaignName}</h1>
          <p class="estate-campaign-summary">{props.viewModel.campaignSummary}</p>
        </div>
        <div class="estate-status-stack">
          <span class="estate-status-pill">Gold {props.viewModel.gold}</span>
          {props.viewModel.isFreshVisit && <span class="estate-status-pill estate-status-pill-accent">Fresh Visit</span>}
        </div>
      </header>

      <section class="estate-stage-shell" aria-label="Town estate">
        <div class="estate-stage-backdrop" />
        <div class="estate-stage-grid" />
        <For each={props.viewModel.buildings}>
          {(building) => {
            const layout = buildingLayout(building.id);
            return (
              <button
                class="building-icon estate-building-node"
                style={{
                  left: estateLeft(layout.x),
                  top: estateTop(layout.y),
                  width: estateWidth(layout.width),
                  height: estateHeight(layout.height)
                }}
                onClick={() => props.onOpenBuilding(building.id)}
                title={building.summary}
                data-building-id={building.id}
              >
                <span class="building-icon-marker estate-building-art">
                  <BuildingIcon buildingId={building.id} size={layout.width} />
                </span>
                <span
                  class="estate-building-banner"
                  style={{
                    left: `${50 + (layout.labelOffsetX / layout.width) * 100}%`,
                    top: `${50 - (layout.labelOffsetY / layout.height) * 100}%`
                  }}
                >
                  <span class="estate-building-source-label">{layout.sourceLabel}</span>
                  <span class="estate-building-contract-label">{building.label}</span>
                  <span class={`estate-building-status estate-building-status-${building.status}`}>
                    {BUILDING_STATUS_LABEL[building.status]}
                  </span>
                </span>
              </button>
            );
          }}
        </For>
      </section>

      <section class="viewport-roster estate-bottom-panel">
        <button class="action-primary estate-embark-button" onClick={props.onStartProvisioning}>
          <span class="estate-embark-title">位面探索</span>
          <span class="estate-embark-subtitle">{props.viewModel.nextActionLabel}</span>
        </button>

        <div class="roster-scroll estate-roster-strip">
          <For each={rosterHeroes(props.viewModel)}>
            {(hero) => {
              const portraitSrc = resolveHeroPortrait({ heroId: hero.id, classLabel: hero.classLabel });
              const hp = parseHp(hero.hp);
              return (
                <button class="roster-hero estate-roster-hero" onClick={() => props.onOpenHero(hero.id)}>
                  <div class="roster-hero-portrait estate-roster-portrait">
                    <div class="roster-portrait-frame estate-roster-frame" />
                    {portraitSrc ? (
                      <img
                        class="roster-portrait-image"
                        src={portraitSrc}
                        alt={`${hero.name} portrait`}
                        loading="eager"
                      />
                    ) : (
                      <div class="roster-portrait-avatar">
                        <span class="roster-portrait-letter">{hero.classLabel[0]}</span>
                      </div>
                    )}
                    <span class="roster-portrait-level">Lv{hero.level}</span>
                    {(hero.isWounded || hero.isAfflicted) && (
                      <span class="roster-portrait-status">{hero.isAfflicted ? "A" : "W"}</span>
                    )}
                  </div>
                  <div class="estate-roster-meta">
                    <span class="roster-hero-name">{hero.name}</span>
                    <span class="roster-hero-class">{hero.classLabel}</span>
                    <div class="estate-roster-bars">
                      <span class="estate-mini-bar">
                        <span class="estate-mini-bar-fill" style={{ width: `${healthPercent(hero)}%`, background: healthBarColor(hero) }} />
                      </span>
                      <span class="estate-mini-bar estate-mini-bar-stress">
                        <span class="estate-mini-bar-fill" style={{ width: `${stressPercent(hero)}%`, background: stressBarColor(hero) }} />
                      </span>
                    </div>
                    <span class="estate-roster-stats">HP {hp.current}/{hp.max} · ST {hero.stress}/{hero.maxStress}</span>
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