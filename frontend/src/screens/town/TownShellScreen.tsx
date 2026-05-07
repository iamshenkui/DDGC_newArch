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
      "--ddgc-chrome-side-button": resolveChromeAsset("sideRealmInventory"),
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

            {/* ── QuickStart / QuickProgress — removed per KUI-P1-008 blocker
                 (Unity source has these as active:false; they do not appear
                 in the reference town.png frame). ── */}
          </section>

          {/* ═══ UI_Shared — persistent shell chrome from Unity UI_Shared ═══ */}
          <div
            class="estate-ui-shared"
            data-source-scene="EstateManagement.unity"
            data-source-prefab="UI_Shared"
            data-source-rect="anchorMin=(0,0) anchorMax=(1,1) pivot=(0.5,0.5)"
          >
            {/* ── Top-left save-slot tag (reference frame shows "存档9")
                 Hard-coded as a documented fixture until the runtime bridge
                 exposes the current save slot index. Source ownership not
                 traced in EstateManagement.unity recon — kept as a UI_Shared
                 sibling overlay anchored to top-left. ── */}
            <div
              class="estate-corner-tag estate-corner-tag-left"
              aria-hidden="true"
              data-source-scene="EstateManagement.unity"
              data-source-prefab="UI_Shared/SaveSlotTag"
              data-source-layer="save-slot-overlay"
            >
              <span class="estate-corner-tag-text">存档9</span>
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
                <span class="estate-currency-slot" data-source-sprite="bust.png">
                  <img class="estate-currency-icon" src={resolveChromeAsset("bustIcon")} alt="" aria-hidden="true" />
                  <span class="estate-currency-value">{props.viewModel.bust}</span>
                </span>
                <span class="estate-currency-slot" data-source-sprite="portrait.png">
                  <img class="estate-currency-icon" src={resolveChromeAsset("portraitIcon")} alt="" aria-hidden="true" />
                  <span class="estate-currency-value">{props.viewModel.portrait}</span>
                </span>
                <span class="estate-currency-slot" data-source-sprite="deed.png">
                  <img class="estate-currency-icon" src={resolveChromeAsset("deedIcon")} alt="" aria-hidden="true" />
                  <span class="estate-currency-value">{props.viewModel.deed}</span>
                </span>
                <span class="estate-currency-slot" data-source-sprite="crest.png">
                  <img class="estate-currency-icon" src={resolveChromeAsset("crestIcon")} alt="" aria-hidden="true" />
                  <span class="estate-currency-value">{props.viewModel.crest}</span>
                </span>
                <span class="estate-currency-slot estate-currency-slot-gold" data-source-sprite="gold.png">
                  <img class="estate-currency-icon" src={resolveChromeAsset("goldIcon")} alt="" aria-hidden="true" />
                  <span class="estate-currency-value">{props.viewModel.gold}</span>
                </span>
              </div>

              {/* ── Top-right command row — source-backed text glyphs
                   Reference 公会界面.png shows three small square text buttons
                   (饰品仓库, 英雄, 设置) floating directly on the sky. The Unity
                   source SideButtons group has six children rendered in source
                   order: ActivityLog, RealmInventory, Hero, TownEvent, Settings,
                   Glossary. Of these, only RealmInventoryButton, HeroButton, and
                   SettingsButton are active in the reference frame; the other
                   three are kept in markup with data-source-active="false" and
                   the .estate-side-button-inactive class so the source grouping
                   is preserved while the inactive quick commands remain hidden.
                   All six children share btn_white.png (GUID 5536faf88204cc54985b146b5ceb03f6)
                   as their Image sprite — a hand-drawn parchment square. ── */}
              <nav
                class="estate-side-panel"
                aria-label="顶部命令栏"
                data-source-scene="EstateManagement.unity"
                data-source-prefab="UI_Shared/UI_MidWindows/UI_Panels/BottomPanel/SideButtons"
                data-source-rect="anchorMin=(0,0) anchorMax=(1,1) pivot=(0.5,0.5)"
              >
                <button
                  class="estate-side-button-inactive"
                  type="button"
                  hidden
                  aria-hidden="true"
                  tabindex={-1}
                  data-source-prefab="UI_Shared/UI_MidWindows/UI_Panels/BottomPanel/SideButtons/ActivityLogButton"
                  data-source-rect="anchorMin=(1,1) anchorMax=(1,1) pivot=(0.5,0.5) anchoredPosition=(-100,-80) sizeDelta=(136,136)"
                  data-source-sprite="btn_white.png"
                  data-source-guid="5536faf88204cc54985b146b5ceb03f6"
                  data-source-active="false"
                />
                <button
                  class="estate-side-button"
                  type="button"
                  title="饰品仓库"
                  aria-label="饰品仓库"
                  data-source-prefab="UI_Shared/UI_MidWindows/UI_Panels/BottomPanel/SideButtons/RealmInventoryButton"
                  data-source-rect="anchorMin=(1,1) anchorMax=(1,1) pivot=(0.5,0.5) anchoredPosition=(-340,-80) sizeDelta=(136,136)"
                  data-source-sprite="btn_white.png"
                  data-source-guid="5536faf88204cc54985b146b5ceb03f6"
                  data-source-active="true"
                >
                  <span class="estate-side-button-label">饰品仓库</span>
                </button>
                <button
                  class="estate-side-button"
                  type="button"
                  title="英雄"
                  aria-label="英雄"
                  data-source-prefab="UI_Shared/UI_MidWindows/UI_Panels/BottomPanel/SideButtons/HeroButton"
                  data-source-rect="anchorMin=(1,1) anchorMax=(1,1) pivot=(0.5,0.5) anchoredPosition=(-220,-80) sizeDelta=(136,136)"
                  data-source-sprite="btn_white.png"
                  data-source-guid="5536faf88204cc54985b146b5ceb03f6"
                  data-source-active="true"
                  onClick={() => {
                    const firstHero = props.viewModel.heroes[0] ?? props.viewModel.roster[0];
                    if (firstHero) props.onOpenHero(firstHero.id);
                  }}
                >
                  <span class="estate-side-button-label">英雄</span>
                </button>
                <button
                  class="estate-side-button-inactive"
                  type="button"
                  hidden
                  aria-hidden="true"
                  tabindex={-1}
                  data-source-prefab="UI_Shared/UI_MidWindows/UI_Panels/BottomPanel/SideButtons/TownEventButton"
                  data-source-rect="anchorMin=(1,1) anchorMax=(1,1) pivot=(0.5,0.5) anchoredPosition=(-100,-80) sizeDelta=(136,136)"
                  data-source-sprite="btn_white.png"
                  data-source-guid="5536faf88204cc54985b146b5ceb03f6"
                  data-source-active="false"
                />
                <button
                  class="estate-side-button"
                  type="button"
                  title="设置"
                  aria-label="设置"
                  data-source-prefab="UI_Shared/UI_MidWindows/UI_Panels/BottomPanel/SideButtons/SettingsButton"
                  data-source-rect="anchorMin=(1,1) anchorMax=(1,1) pivot=(0.5,0.5) anchoredPosition=(-100,-80) sizeDelta=(136,136)"
                  data-source-sprite="btn_white.png"
                  data-source-guid="5536faf88204cc54985b146b5ceb03f6"
                  data-source-active="true"
                >
                  <span class="estate-side-button-label">设置</span>
                </button>
                <button
                  class="estate-side-button-inactive"
                  type="button"
                  hidden
                  aria-hidden="true"
                  tabindex={-1}
                  data-source-prefab="UI_Shared/UI_MidWindows/UI_Panels/BottomPanel/SideButtons/GlossaryButton"
                  data-source-rect="anchorMin=(1,1) anchorMax=(1,1) pivot=(0.5,0.5) anchoredPosition=(-220,-80) sizeDelta=(136,136)"
                  data-source-sprite="btn_white.png"
                  data-source-guid="5536faf88204cc54985b146b5ceb03f6"
                  data-source-active="false"
                />
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
