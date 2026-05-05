import { For, createSignal, onCleanup, onMount, type Component } from "solid-js";

import { resolveHeroPortrait, resolveChromeAsset } from "../../assets/originalAssetPaths";
import type { TownBuildingSummary, TownHeroSummary, TownViewModel } from "../../bridge/contractTypes";
import { getTownBuildingCatalogEntry, mergeTownBuildings } from "../../town/buildingCatalog";
import { BuildingIcon } from "./buildings/BuildingIcons";

interface TownShellScreenProps {
  viewModel: TownViewModel;
  onOpenHero: (heroId: string) => void;
  onOpenBuilding: (buildingId: string) => void;
  onStartProvisioning: () => void;
}

const ESTATE_STAGE_WIDTH = 1920;
const ESTATE_STAGE_HEIGHT = 1080;

const BUILDING_STATUS_LABEL: Record<TownBuildingSummary["status"], string> = {
  ready: "可用",
  partial: "部分可用",
  locked: "未开放"
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
  return `${ESTATE_STAGE_WIDTH / 2 + x}px`;
}

function estateTop(y: number): string {
  return `${ESTATE_STAGE_HEIGHT / 2 - y}px`;
}

function estateWidth(w: number): string {
  return `${w}px`;
}

function estateHeight(h: number): string {
  return `${h}px`;
}

function rosterHeroes(viewModel: TownViewModel): ReadonlyArray<TownHeroSummary> {
  return viewModel.roster.length > 0 ? viewModel.roster : viewModel.heroes;
}

export const TownShellScreen: Component<TownShellScreenProps> = (props) => {
  const mergedBuildings = () => mergeTownBuildings(props.viewModel.buildings);
  const [canvasScale, setCanvasScale] = createSignal(1);

  onMount(() => {
    const updateCanvasScale = () => {
      const widthScale = Math.max((window.innerWidth - 32) / ESTATE_STAGE_WIDTH, 0.1);
      const heightScale = Math.max((window.innerHeight - 32) / ESTATE_STAGE_HEIGHT, 0.1);
      setCanvasScale(Math.min(widthScale, heightScale, 1));
    };

    updateCanvasScale();
    window.addEventListener("resize", updateCanvasScale);
    onCleanup(() => window.removeEventListener("resize", updateCanvasScale));
  });

  const chromeVars = () =>
    Object.entries({
      "--ddgc-chrome-name-bg": resolveChromeAsset("estateNameBg"),
      "--ddgc-chrome-building-icon-bg": resolveChromeAsset("buildingIconBg"),
      "--ddgc-chrome-building-label-bg": resolveChromeAsset("buildingLabelBg"),
      "--ddgc-chrome-building-title-bg": resolveChromeAsset("buildingTitleBg"),
      "--ddgc-chrome-building-info-bg": resolveChromeAsset("buildingInfoBg"),
      "--ddgc-chrome-embark": resolveChromeAsset("embarkButton"),
      "--ddgc-chrome-bust": resolveChromeAsset("bustIcon"),
      "--ddgc-chrome-portrait": resolveChromeAsset("portraitIcon"),
      "--ddgc-chrome-deed": resolveChromeAsset("deedIcon"),
      "--ddgc-chrome-crest": resolveChromeAsset("crestIcon"),
      "--ddgc-chrome-gold": resolveChromeAsset("goldIcon")
    } as Record<string, string>).reduce(
      (acc, [k, v]) => {
        acc[k] = `url("${v}")`;
        return acc;
      },
      {} as Record<string, string>
    );

  return (
    <div class="town-viewport estate-town-screen">
      <div class="estate-canvas-frame">
        <div
          class="estate-reference-canvas"
          style={{
            transform: `translate(-50%, -50%) scale(${canvasScale()})`,
            ...chromeVars()
          }}
          data-source-scene="EstateManagement.unity"
          data-source-root="EstateSceneManager"
        >
          {/* ═══ UI_Estate — building layer from Unity UI_Estate/UI_Estate ═══ */}
          <section
            class="estate-ui-estate"
            aria-label="Town estate"
            data-source-scene="EstateManagement.unity"
            data-source-prefab="UI_Estate/UI_Estate"
            data-source-layer="building-surface"
            data-source-rect="anchorMin=(0,0) anchorMax=(1,1) pivot=(0.5,0.5)"
          >
            {/* ── Central estate stage surface — source-backed ground plane ── */}
            <div
              class="estate-stage-shell"
              data-source-scene="EstateManagement.unity"
              data-source-layer="stage-surface"
            >
              <div class="estate-stage-backdrop" aria-hidden="true" />
              <div class="estate-stage-grid" aria-hidden="true" />

              <For each={mergedBuildings()}>
                {(building) => {
                  const layout = getTownBuildingCatalogEntry(building.id);
                  if (!layout) return null;

                  // Click region is the building's Unity sizeDelta box, anchored at
                  // anchoredPosition (translated to CSS top-left via -50% transform).
                  // This makes the hit area line up exactly with the visible estate
                  // layer rendered by the source-backed building sprite.
                  const sourceRect =
                    `anchorMin=(0.5,0.5) anchorMax=(0.5,0.5) ` +
                    `anchoredPosition=(${layout.x},${layout.y}) ` +
                    `sizeDelta=(${layout.width},${layout.height})`;
                  const labelSourceRect =
                    `anchoredPosition=(${layout.labelOffsetX},${layout.labelOffsetY}) sizeDelta=(326,48)`;

                  return (
                    <button
                      class="estate-building-node"
                      style={{
                        left: estateLeft(layout.x),
                        top: estateTop(layout.y),
                        width: estateWidth(layout.width),
                        height: estateHeight(layout.height)
                      }}
                      onClick={() => props.onOpenBuilding(building.id)}
                      title={building.summary}
                      aria-label={`${layout.displayName} — ${BUILDING_STATUS_LABEL[building.status]}`}
                      data-building-id={building.id}
                      data-source-prefab="UI/Estate/BuildingSlot"
                      data-source-scene="EstateManagement.unity"
                      data-source-rect={sourceRect}
                      data-source-guid={layout.sourceGuid ?? undefined}
                    >
                      <span class="estate-building-art" aria-hidden="true">
                        <BuildingIcon buildingId={building.id} size={layout.width} fallbackLabel={layout.displayName} />
                      </span>
                      <span
                        class="estate-building-banner"
                        style={{
                          left: `${layout.width / 2 + layout.labelOffsetX}px`,
                          top: `${layout.height / 2 - layout.labelOffsetY}px`
                        }}
                        data-source-prefab="UI/Estate/BuildingSlot/BuildingLabel"
                        data-source-rect={labelSourceRect}
                        data-source-sprite="building_label_bg01.png"
                        data-source-guid="0beef34e329073f43bc2a495a740b0b4"
                      >
                        <span class="estate-building-source-label">{layout.displayName}</span>
                        <span class={`estate-building-status estate-building-status-${building.status}`}>
                          {BUILDING_STATUS_LABEL[building.status]}
                        </span>
                      </span>
                    </button>
                  );
                }}
              </For>
            </div>

            {/* ── QuickStart / QuickProgress — utility buttons from Unity UI_Estate/UI_Estate ── */}
            <div
              class="estate-quick-buttons"
              data-source-scene="EstateManagement.unity"
              data-source-prefab="UI_Estate/UI_Estate"
              data-source-layer="quick-buttons"
            >
              <button
                class="estate-quick-button"
                title="快速开始"
                aria-label="快速开始"
                data-source-prefab="UI_Estate/UI_Estate/QuickStartButton"
                data-source-rect="anchoredPosition=(-324,137) sizeDelta=(97,42)"
              >
                <img src={resolveChromeAsset("quickStart")} alt="" aria-hidden="true" />
                <span>快速开始</span>
              </button>
              <button
                class="estate-quick-button"
                title="快速进度"
                aria-label="快速进度"
                data-source-prefab="UI_Estate/UI_Estate/QuickProgressButton"
                data-source-rect="anchoredPosition=(389,137) sizeDelta=(97,42)"
              >
                <img src={resolveChromeAsset("quickProgress")} alt="" aria-hidden="true" />
                <span>快速进度</span>
              </button>
            </div>
          </section>

          {/* ═══ UI_Shared — persistent shell chrome from Unity UI_Shared ═══ */}
          <div
            class="estate-ui-shared"
            data-source-scene="EstateManagement.unity"
            data-source-prefab="UI_Shared"
            data-source-rect="anchorMin=(0,0) anchorMax=(1,1) pivot=(0.5,0.5)"
          >
            {/* ── UI_Panels — shell chrome panel group (Unity UI_Shared/UI_Panels) ── */}
            <div
              class="estate-ui-panels"
              data-source-scene="EstateManagement.unity"
              data-source-prefab="UI_Shared/UI_Panels"
              data-source-rect="anchorMin=(0,0) anchorMax=(1,1) pivot=(0.5,0.5) sizeDelta=(0,0)"
            >
              {/* ── EstateNameplate — top-right campaign nameplate (Unity UI_Shared/UI_Panels/EstateNameplate) ── */}
              <header
                class="estate-top-panel"
                data-source-scene="EstateManagement.unity"
                data-source-prefab="UI_Shared/UI_Panels/EstateNameplate"
                data-source-layer="top-nameplate"
                data-source-rect="anchorMin=(1,1) anchorMax=(1,1) pivot=(0.5,0.5) anchoredPosition=(-1624,-76.5) sizeDelta=(592,153)"
              >
                <div class="estate-name-card">
                  <span class="eyebrow">城镇中枢</span>
                  <h1 class="estate-campaign-name">{props.viewModel.campaignName}</h1>
                  <p class="estate-campaign-summary">{props.viewModel.campaignSummary}</p>
                </div>

                {/* ── CurrencyPanel — 5-currency strip (Unity UI_Shared/UI_TopWindows/CurrencyPanel) ── */}
                <div
                  class="estate-currency-strip"
                  data-source-scene="EstateManagement.unity"
                  data-source-prefab="UI_Shared/UI_TopWindows/CurrencyPanel"
                  data-source-rect="anchorMin=(1,0) anchorMax=(1,0) pivot=(1,0) anchoredPosition=(-140,20) sizeDelta=(1000,80)"
                >
                  <span class="estate-currency-slot" data-source-sprite="currency_bust.png" data-source-layer="deferred-heirloom">
                    <img class="estate-currency-icon" src={resolveChromeAsset("bustIcon")} alt="" aria-hidden="true" />
                    <span class="estate-currency-value">—</span>
                  </span>
                  <span class="estate-currency-slot" data-source-sprite="currency_portrait.png" data-source-layer="deferred-heirloom">
                    <img class="estate-currency-icon" src={resolveChromeAsset("portraitIcon")} alt="" aria-hidden="true" />
                    <span class="estate-currency-value">—</span>
                  </span>
                  <span class="estate-currency-slot" data-source-sprite="currency_deed.png" data-source-layer="deferred-heirloom">
                    <img class="estate-currency-icon" src={resolveChromeAsset("deedIcon")} alt="" aria-hidden="true" />
                    <span class="estate-currency-value">—</span>
                  </span>
                  <span class="estate-currency-slot" data-source-sprite="currency_crest.png" data-source-layer="deferred-heirloom">
                    <img class="estate-currency-icon" src={resolveChromeAsset("crestIcon")} alt="" aria-hidden="true" />
                    <span class="estate-currency-value">—</span>
                  </span>
                  <span class="estate-currency-slot estate-currency-slot-gold" data-source-sprite="gold.png">
                    <img class="estate-currency-icon" src={resolveChromeAsset("goldIcon")} alt="" aria-hidden="true" />
                    <span class="estate-currency-value">{props.viewModel.gold}</span>
                  </span>
                </div>

                {props.viewModel.isFreshVisit && (
                  <span class="estate-fresh-visit-badge">本周初访</span>
                )}
              </header>

              {/* ── UI_Panels/BottomPanel — bottom panel grouping (Unity UI_Shared/UI_Panels/BottomPanel) ── */}
              <div
                class="estate-bottom-panel"
                data-source-scene="EstateManagement.unity"
                data-source-prefab="UI_Shared/UI_Panels/BottomPanel"
                data-source-rect="anchorMin=(0,0) anchorMax=(1,1) pivot=(0.5,0) anchoredPosition=(0,0) sizeDelta=(0,0)"
              >
                {/* ── BottomPanel/SideButtons — 6 side navigation buttons (Unity UI_Shared/UI_Panels/BottomPanel/SideButtons) ── */}
                <nav
                  class="estate-side-panel"
                  aria-label="侧边导航"
                  data-source-scene="EstateManagement.unity"
                  data-source-prefab="UI_Shared/UI_Panels/BottomPanel/SideButtons"
                >
                  <button
                    class="estate-side-button"
                    title="活动日志"
                    aria-label="活动日志"
                    data-source-prefab="UI_Shared/UI_Panels/BottomPanel/SideButtons/ActivityLog"
                    data-source-rect="anchoredPosition=(-100,-80) sizeDelta=(136,136)"
                  >
                    <img src={resolveChromeAsset("sideActivityLog")} alt="活动日志" loading="eager" />
                  </button>
                  <button
                    class="estate-side-button"
                    title="位面背包"
                    aria-label="位面背包"
                    data-source-prefab="UI_Shared/UI_Panels/BottomPanel/SideButtons/RealmInventory"
                    data-source-rect="anchoredPosition=(-340,-80) sizeDelta=(136,136)"
                  >
                    <img src={resolveChromeAsset("sideRealmInventory")} alt="位面背包" loading="eager" />
                  </button>
                  <button
                    class="estate-side-button"
                    title="英雄"
                    aria-label="英雄"
                    data-source-prefab="UI_Shared/UI_Panels/BottomPanel/SideButtons/Hero"
                    data-source-rect="anchoredPosition=(-220,-80) sizeDelta=(136,136)"
                  >
                    <img src={resolveChromeAsset("sideHero")} alt="英雄" loading="eager" />
                  </button>
                  <button
                    class="estate-side-button"
                    title="城镇事件"
                    aria-label="城镇事件"
                    data-source-prefab="UI_Shared/UI_Panels/BottomPanel/SideButtons/TownEvent"
                    data-source-rect="anchoredPosition=(-100,-80) sizeDelta=(136,136)"
                  >
                    <img src={resolveChromeAsset("sideTownEvent")} alt="城镇事件" loading="eager" />
                  </button>
                  <button
                    class="estate-side-button"
                    title="设置"
                    aria-label="设置"
                    data-source-prefab="UI_Shared/UI_Panels/BottomPanel/SideButtons/Settings"
                    data-source-rect="anchoredPosition=(-100,-80) sizeDelta=(136,136)"
                  >
                    <img src={resolveChromeAsset("sideSettings")} alt="设置" loading="eager" />
                  </button>
                  <button
                    class="estate-side-button"
                    title="术语表"
                    aria-label="术语表"
                    data-source-prefab="UI_Shared/UI_Panels/BottomPanel/SideButtons/Glossary"
                    data-source-rect="anchoredPosition=(-220,-80) sizeDelta=(136,136)"
                  >
                    <img src={resolveChromeAsset("sideGlossary")} alt="术语表" loading="eager" />
                  </button>
                </nav>

                {/* ── BottomPanel/EmbarkButton — primary expedition CTA ── */}
                <button
                  class="estate-embark-button"
                  onClick={props.onStartProvisioning}
                  data-source-scene="EstateManagement.unity"
                  data-source-prefab="UI_Shared/UI_Panels/BottomPanel/EmbarkButton"
                  data-source-rect="anchorMin=(0,0) anchorMax=(0,0) pivot=(0.5,0.5) anchoredPosition=(20,20) sizeDelta=(968,968)"
                >
                  <span class="estate-embark-title">位面探索</span>
                  <span class="estate-embark-subtitle">{props.viewModel.nextActionLabel}</span>
                </button>
              </div>
            </div>

            {/* ── UI_Roster/RosterPanel — hero roster strip (Unity UI_Shared/UI_Roster/RosterPanel)
                 Each slot mirrors HeroSlot.prefab hierarchy:
                   HeroSlot (root: Image + TownHeroSlot)
                   ├── ActiveOverlay  — active: false (not rendered)
                   ├── Locked         — active: false (not rendered)
                   ├── HeroLabel      — Text "英雄名称"
                   ├── EventLocked    — active: false (not rendered)
                   ├── FreeOverlay    — active: false (not rendered)
                   ├── ActivityButton — (Image + Button, not rendered in roster)
                   ├── CostLabel      — gold cost (not rendered in roster)
                   └── Frame          — Image (hero_slot.backgroundhightlight.png)
            */}
            <div
              class="roster-scroll estate-roster-strip"
              data-source-scene="EstateManagement.unity"
              data-source-prefab="UI_Shared/UI_Roster/RosterPanel"
              data-source-layer="roster-strip"
            >
              <For each={rosterHeroes(props.viewModel)}>
                {(hero) => {
                  const portraitSrc = resolveHeroPortrait({ heroId: hero.id, classLabel: hero.classLabel });
                  const hp = parseHp(hero.hp);
                  const topQuirks = hero.positiveQuirks.slice(0, 2);
                  const topNegQuirks = hero.negativeQuirks.slice(0, 2);
                  const hasDiseases = hero.diseases.length > 0;
                  return (
                    <button
                      class="roster-hero estate-roster-hero"
                      onClick={() => props.onOpenHero(hero.id)}
                      data-source-prefab="UI/HeroSlot.prefab"
                      data-source-component="TownHeroSlot"
                      data-hero-id={hero.id}
                    >
                      {/* ── HeroSlot portrait surface (mirrors HeroSlot root Image component) ── */}
                      <div class="roster-hero-portrait estate-roster-portrait" data-source-component="Image">
                        {/* Frame — mirrors HeroSlot/Frame child (Image: hero_slot.backgroundhightlight.png) */}
                        <div class="roster-portrait-frame estate-roster-frame" data-source-component="Frame" />
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
                        {/* HeroLabel — mirrors HeroSlot/HeroLabel (Text: hero name) */}
                        <span class="roster-hero-name" data-source-component="HeroLabel">{hero.name}</span>
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
                        {/* Quirks & Diseases — compact inline summary */}
                        <div class="roster-status-chips">
                          <For each={topQuirks}>
                            {(quirk) => (
                              <span class="roster-chip roster-chip--positive" title={`Positive Quirk: ${quirk}`}>{quirk}</span>
                            )}
                          </For>
                          <For each={topNegQuirks}>
                            {(quirk) => (
                              <span class="roster-chip roster-chip--negative" title={`Negative Quirk: ${quirk}`}>{quirk}</span>
                            )}
                          </For>
                          {hasDiseases && (
                            <span class="roster-chip roster-chip--disease" title={`${hero.diseases.length} disease(s)`}>
                              {hero.diseases.length > 1 ? `${hero.diseases.length} diseases` : hero.diseases[0]}
                            </span>
                          )}
                        </div>
                      </div>
                    </button>
                  );
                }}
              </For>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
