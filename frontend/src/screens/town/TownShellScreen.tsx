import { For, createSignal, onCleanup, onMount, type Component } from "solid-js";

import { resolveChromeAsset } from "../../assets/originalAssetPaths";
import type { TownViewModel } from "../../bridge/contractTypes";
import { getTownBuildingCatalogEntry, mergeTownBuildings } from "../../town/buildingCatalog";
import { BuildingIcon } from "./buildings/BuildingIcons";

interface TownShellScreenProps {
  viewModel: TownViewModel;
  onOpenHero: (heroId: string) => void;
  onOpenBuilding: (buildingId: string) => void;
  onStartProvisioning: () => void;
  onOpenSaveLoad?: () => void;
}

const ESTATE_STAGE_WIDTH = 1920;
const ESTATE_STAGE_HEIGHT = 1080;

// Building status is not rendered visually in the reference frame (town.png);
// status is kept only for data-model completeness and building-detail routing.

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
                      aria-label={layout.displayName}
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
            {/* ── Top-left campaign tag (mirrors reference "新游戏" eyebrow) ── */}
            <div class="estate-corner-tag estate-corner-tag-left" aria-hidden="true">
              <span class="estate-corner-tag-text">新游戏</span>
            </div>

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

                {props.viewModel.isFreshVisit && (
                  <span class="estate-fresh-visit-badge">本周初访</span>
                )}
              </header>

              {/* ── CurrencyPanel — 5-currency strip (Unity UI_Shared/UI_TopWindows/CurrencyPanel)
                   Reference frame anchors this row at the bottom-right of the cosmic backdrop with
                   no panel chrome; placing it here lifts it out of the top nameplate so the painterly
                   sky reads as one composition rather than two stacked overlay panels. ── */}
              <div
                class="estate-currency-strip"
                data-source-scene="EstateManagement.unity"
                data-source-prefab="UI_Shared/UI_TopWindows/CurrencyPanel"
                data-source-rect="anchorMin=(1,0) anchorMax=(1,0) pivot=(1,0) anchoredPosition=(-140,20) sizeDelta=(1000,80)"
              >
                <span class="estate-currency-slot" data-source-sprite="currency_bust.png" data-source-layer="deferred-heirloom">
                  <img class="estate-currency-icon" src={resolveChromeAsset("bustIcon")} alt="" aria-hidden="true" />
                  <span class="estate-currency-value">100</span>
                </span>
                <span class="estate-currency-slot" data-source-sprite="currency_portrait.png" data-source-layer="deferred-heirloom">
                  <img class="estate-currency-icon" src={resolveChromeAsset("portraitIcon")} alt="" aria-hidden="true" />
                  <span class="estate-currency-value">100</span>
                </span>
                <span class="estate-currency-slot" data-source-sprite="currency_deed.png" data-source-layer="deferred-heirloom">
                  <img class="estate-currency-icon" src={resolveChromeAsset("deedIcon")} alt="" aria-hidden="true" />
                  <span class="estate-currency-value">100</span>
                </span>
                <span class="estate-currency-slot" data-source-sprite="currency_crest.png" data-source-layer="deferred-heirloom">
                  <img class="estate-currency-icon" src={resolveChromeAsset("crestIcon")} alt="" aria-hidden="true" />
                  <span class="estate-currency-value">200</span>
                </span>
                <span class="estate-currency-slot estate-currency-slot-gold" data-source-sprite="gold.png">
                  <img class="estate-currency-icon" src={resolveChromeAsset("goldIcon")} alt="" aria-hidden="true" />
                  <span class="estate-currency-value">{props.viewModel.gold}</span>
                </span>
              </div>

              {/* ── Top-right utility buttons — source-backed text glyphs
                   Reference town.png shows three small square text buttons
                   (饰品仓库, 英雄, 设置) floating directly on the sky.
                   Lifted out of BottomPanel so z-index stacks above the nameplate. ── */}
              <nav
                class="estate-side-panel"
                aria-label="顶部导航"
                data-source-scene="EstateManagement.unity"
                data-source-prefab="UI_Shared/UI_Panels/BottomPanel/SideButtons"
              >
                <button
                  class="estate-side-button"
                  title="饰品仓库"
                  aria-label="饰品仓库"
                  data-source-prefab="UI_Shared/UI_Panels/BottomPanel/SideButtons/RealmInventory"
                  data-source-rect="anchoredPosition=(-340,-80) sizeDelta=(136,136)"
                >
                  饰品仓库
                </button>
                <button
                  class="estate-side-button"
                  title="英雄"
                  aria-label="英雄"
                  data-source-prefab="UI_Shared/UI_Panels/BottomPanel/SideButtons/Hero"
                  data-source-rect="anchoredPosition=(-220,-80) sizeDelta=(136,136)"
                  onClick={() => {
                    const firstHero = props.viewModel.heroes[0] ?? props.viewModel.roster[0];
                    if (firstHero) props.onOpenHero(firstHero.id);
                  }}
                >
                  英雄
                </button>
                <button
                  class="estate-side-button"
                  title="设置"
                  aria-label="设置"
                  data-source-prefab="UI_Shared/UI_Panels/BottomPanel/SideButtons/Settings"
                  data-source-rect="anchoredPosition=(-100,-80) sizeDelta=(136,136)"
                  onClick={() => props.onOpenSaveLoad?.()}
                >
                  设置
                </button>
              </nav>

              {/* ── UI_Panels/BottomPanel — bottom panel grouping (Unity UI_Shared/UI_Panels/BottomPanel) ── */}
              <div
                class="estate-bottom-panel"
                data-source-scene="EstateManagement.unity"
                data-source-prefab="UI_Shared/UI_Panels/BottomPanel"
                data-source-rect="anchorMin=(0,0) anchorMax=(1,1) pivot=(0.5,0) anchoredPosition=(0,0) sizeDelta=(0,0)"
              >
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

            {/* ── Central tower beacon glow (legacytower) — reference town.png shows
                 a prominent vertical glow and faint orbital rings around the central
                 spire. This atmospheric layer recreates that effect without requiring
                 a separate glow sprite asset. ── */}
            <div
              class="estate-tower-glow"
              aria-hidden="true"
              data-source-scene="EstateManagement.unity"
              data-source-prefab="UI_Estate/UI_Estate/LegacyTowerGlow"
              data-source-layer="atmospheric-glow"
            />
          </div>
        </div>
      </div>
    </div>
  );
};
