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
  sourcePrefabPath: string;
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
    sourcePrefabPath: "Assets/Prefabs/UI/Estate/Buildings/StageCoach/StageCoachWindow.prefab",
    labelOffsetX: -26,
    labelOffsetY: -150
  },
  guild: {
    x: 120,
    y: -50,
    width: 397,
    height: 397,
    sourceLabel: "试炼场",
    sourcePrefabPath: "Assets/Prefabs/UI/Estate/Buildings/Guild/GuildWindow.prefab",
    labelOffsetX: 30,
    labelOffsetY: -50
  },
  blacksmith: {
    x: 709,
    y: 294,
    width: 482,
    height: 482,
    sourceLabel: "锻造舱",
    sourcePrefabPath: "Assets/Prefabs/UI/Estate/Buildings/Blacksmith/BlacksmithWindow.prefab",
    labelOffsetX: 0,
    labelOffsetY: -168
  },
  sanitarium: {
    x: 390,
    y: 110,
    width: 339,
    height: 339,
    sourceLabel: "细胞修复站",
    sourcePrefabPath: "Assets/Prefabs/UI/Estate/Buildings/Sanitarium/SanitariumWindow.prefab",
    labelOffsetX: 50,
    labelOffsetY: -100
  },
  abbey: {
    x: -330,
    y: 90,
    width: 519,
    height: 519,
    sourceLabel: "信仰祭坛",
    sourcePrefabPath: "Assets/Prefabs/UI/Estate/Buildings/Abbey/AbbeyWindow.prefab",
    labelOffsetX: -10,
    labelOffsetY: 153
  },
  tavern: {
    x: -650,
    y: -220,
    width: 519,
    height: 519,
    sourceLabel: "迷情乐园",
    sourcePrefabPath: "Assets/Prefabs/UI/Estate/Buildings/Tavern/TavernWindow.prefab",
    labelOffsetX: -6,
    labelOffsetY: 160
  },
  graveyard: {
    x: 696,
    y: 96,
    width: 344,
    height: 344,
    sourceLabel: "英雄档案馆",
    sourcePrefabPath: "Assets/Prefabs/UI/Estate/Buildings/Graveyard/GraveyardWindow.prefab",
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
    sourcePrefabPath: `Assets/Prefabs/UI/Estate/Buildings/${buildingId}.prefab`,
    labelOffsetX: 0,
    labelOffsetY: 120
  };
}

function rosterHeroes(viewModel: TownViewModel): ReadonlyArray<TownHeroSummary> {
  return viewModel.roster.length > 0 ? viewModel.roster : viewModel.heroes;
}

export const TownShellScreen: Component<TownShellScreenProps> = (props) => {
  return (
    <div
      class="town-viewport estate-town-screen"
      data-source-scene="Assets/Scenes/EstateManagement.unity"
      data-source-manager="EstateSceneManager"
      data-source-hierarchy="UI_Estate | UI_Shared"
    >
      {/*── Top Panel — mirrors UI_Shared/UI_Panels/EstateNameplate + UI_TopWindows/CurrencyPanel ──*/}
      <header
        class="estate-top-panel"
        data-source-hierarchy="UI_Shared/UI_Panels/EstateNameplate | UI_Shared/UI_TopWindows/CurrencyPanel"
      >
        <div class="estate-name-card"
          data-source-component="EstateNameplate"
          data-source-sprite="Assets/Sprites/ui/building_title_bg.png"
        >
          <span class="eyebrow">Estate</span>
          <h1 class="estate-campaign-name">{props.viewModel.campaignName}</h1>
          <p class="estate-campaign-summary">{props.viewModel.campaignSummary}</p>
        </div>
        <div class="estate-status-stack" data-source-component="CurrencyPanel">
          <span class="estate-status-pill">Gold {props.viewModel.gold}</span>
          {props.viewModel.isFreshVisit && <span class="estate-status-pill estate-status-pill-accent">Fresh Visit</span>}
        </div>
      </header>

      {/*── Central Estate Surface — mirrors UI_Estate building collection ──*/}
      <section
        class="estate-stage-shell"
        aria-label="Town estate"
        data-source-hierarchy="UI_Estate/UI_Estate"
      >
        <div class="estate-stage-sky" />
        <div
          class="estate-stage-landscape"
          data-source-layer="EstateBackground"
          data-source-sprite="Assets/Sprites/EstateBackground/estate_bg.png"
        />
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
                data-source-prefab={layout.sourcePrefabPath}
                data-source-component={building.label}
              >
                <span class="building-icon-marker estate-building-art"
                  data-source-sprite="Assets/Sprites/ui/building_icon_bg.png"
                  data-source-guid="366866d49b92bc7489d973717eabaa58"
                >
                  <BuildingIcon buildingId={building.id} size={layout.width} />
                </span>
                <span
                  class="estate-building-banner"
                  style={{
                    left: `${50 + (layout.labelOffsetX / layout.width) * 100}%`,
                    top: `${50 - (layout.labelOffsetY / layout.height) * 100}%`
                  }}
                  data-source-component="BuildingLabel"
                  data-source-sprite="Assets/Sprites/town/building_label_bg01.png"
                  data-source-title-bg="Assets/Sprites/ui/building_title_bg.png"
                  data-source-info-bg="Assets/Sprites/ui/building_info_bg.png"
                >
                  <span class="estate-building-source-label"
                    data-source-sprite="Assets/Sprites/ui/building_title_bg.png"
                    data-source-guid="f25ac3da7d687e34e89e1443ea12c9a3"
                  >{layout.sourceLabel}</span>
                  <span class="estate-building-contract-label"
                    data-source-sprite="Assets/Sprites/ui/building_info_bg.png"
                    data-source-guid="1005dba8c66f9694895fbcfbc9fa59a1"
                  >{building.label}</span>
                  <span class={`estate-building-status estate-building-status-${building.status}`}>
                    {BUILDING_STATUS_LABEL[building.status]}
                  </span>
                </span>
              </button>
            );
          }}
        </For>
      </section>

      {/*── Bottom Panel — mirrors UI_Shared/UI_Panels/BottomPanel/{EmbarkButton,SideButtons} + UI_Roster/RosterPanel ──*/}
      <section
        class="estate-bottom-panel"
        data-source-hierarchy="UI_Shared/UI_Panels/BottomPanel | UI_Shared/UI_Roster/RosterPanel"
      >
        <button
          class="estate-embark-button"
          onClick={props.onStartProvisioning}
          data-source-component="EmbarkButton"
          data-source-prefab="Assets/Prefabs/UI/Estate/PanelWindows/EstateBottomPanel.prefab"
        >
          <span class="estate-embark-title">位面探索</span>
          <span class="estate-embark-subtitle">{props.viewModel.nextActionLabel}</span>
        </button>

        <div class="roster-scroll estate-roster-strip" data-source-component="RosterPanel">
          <For each={rosterHeroes(props.viewModel)}>
            {(hero) => {
              const portraitSrc = resolveHeroPortrait({ heroId: hero.id, classLabel: hero.classLabel });
              const hp = parseHp(hero.hp);
              return (
                <button
                  class="roster-hero estate-roster-hero"
                  onClick={() => props.onOpenHero(hero.id)}
                  data-source-prefab="Assets/Prefabs/UI/HeroSlot.prefab"
                  data-source-hierarchy="HeroSlot"
                  data-source-component="TownHeroSlot"
                >
                  <div class="roster-hero-portrait estate-roster-portrait" data-source-hierarchy="HeroSlot/Frame">
                    <div class="roster-portrait-frame estate-roster-frame" data-source-sprite="Assets/Resources/Sprites/hero_slot.backgroundhightlight.png" />
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
                  <div class="estate-roster-meta" data-source-hierarchy="HeroSlot/HeroLabel">
                    <span class="roster-hero-name">{hero.name}</span>
                    <span class="roster-hero-class">{hero.classLabel}</span>
                    <div class="estate-roster-bars">
                      <div class="estate-roster-bar-row">
                        <span class="estate-roster-bar-label">HP</span>
                        <span class="estate-mini-bar">
                          <span class="estate-mini-bar-fill" style={{ width: `${healthPercent(hero)}%`, background: healthBarColor(hero) }} />
                        </span>
                      </div>
                      <div class="estate-roster-bar-row">
                        <span class="estate-roster-bar-label estate-roster-bar-label--stress">ST</span>
                        <span class="estate-mini-bar estate-mini-bar-stress">
                          <span class="estate-mini-bar-fill" style={{ width: `${stressPercent(hero)}%`, background: stressBarColor(hero) }} />
                        </span>
                      </div>
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