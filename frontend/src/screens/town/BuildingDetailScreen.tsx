import { For, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../bridge/contractTypes";
import { BuildingDetailHeader } from "./buildings/BuildingDetailHeader";

/**
 * Generic building detail screen — fallback for buildings without a
 * dedicated screen component.
 *
 * Mirrors Unity building window prefab hierarchy:
 *   UI_Shared/UI_LowWindows/{Building}Window/
 *     LeftPanel/
 *       Title/  → BuildingLabel, BuildingDesc
 *       Icon    → building marker sprite
 *       TalkButton → (mapped to first action trigger)
 *     RightPanel/
 *       UpgradeButton    → primary actions
 *       UpgradeWindow    → upgrade tree / action slots
 *     CloseButton → Return to Town
 *
 * @see assets/asset-manifest.json building_sprites for GUIDs
 */

interface BuildingDetailScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

const BUILDING_PREFAB_MAP: Record<string, { prefab: string; sprite: string; guid: string }> = {
  market: {
    prefab: "Assets/Prefabs/UI/Estate/Buildings/Market/MarketWindow.prefab",
    sprite: "Assets/Sprites/town/buildings/building_market.png",
    guid: "8c3505ec8c7e25b42aacdcaece82f821",
  },
  graveyard: {
    prefab: "Assets/Prefabs/UI/Estate/Buildings/Graveyard/GraveyardWindow.prefab",
    sprite: "Assets/Sprites/town/buildings/building_graveyard.png",
    guid: "",
  },
  abbey: {
    prefab: "Assets/Prefabs/UI/Estate/Buildings/Abbey/AbbeyWindow.prefab",
    sprite: "Assets/Sprites/town/buildings/building_faith_altar.png",
    guid: "311540f167839cf4da00305566192b4a",
  },
  tavern: {
    prefab: "Assets/Prefabs/UI/Estate/Buildings/Tavern/TavernWindow.prefab",
    sprite: "Assets/Sprites/town/buildings/building_paradise.png",
    guid: "c0ea280d2704bdb4a9621d6e181e0316",
  },
};

const FALLBACK_PREFAB = {
  prefab: "Assets/Prefabs/UI/Estate/Buildings/GenericBuildingWindow.prefab",
  sprite: "Assets/Sprites/town/buildings/building_generic.png",
  guid: "",
};

function prefabFor(buildingId: string): { prefab: string; sprite: string; guid: string } {
  return BUILDING_PREFAB_MAP[buildingId] ?? FALLBACK_PREFAB;
}

export const BuildingDetailScreen: Component<BuildingDetailScreenProps> = (props) => {
  const vm = () => props.viewModel;
  const prefab = () => prefabFor(vm().buildingId);

  return (
    <div class="app-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Building Header (Icon + Title + Desc) ── */}
      <BuildingDetailHeader
        buildingId={vm().buildingId}
        label={vm().label}
        status={vm().status}
        description={vm().description}
        sourcePrefabPath={prefab().prefab}
        sourceSpritePath={prefab().sprite}
        sourceGuid={prefab().guid}
      />

      {/* ── Content: left (info) + right (actions) — mirrors LeftPanel + RightPanel ── */}
      <div class="building-detail-content">
        {/* Left Panel — mirrors {Building}Window/LeftPanel */}
        <div
          class="building-detail-left"
          data-source-hierarchy="{Building}Window/LeftPanel"
        >
          <div class="building-info-card">
            <h3 class="building-info-card-title">Status</h3>
            {vm().currentUpgrade && (
              <div class="building-info-row">
                <span class="building-info-label">Upgrade Level</span>
                <span class="building-info-value">{vm().currentUpgrade}</span>
              </div>
            )}
            {vm().upgradeRequirement && (
              <div class="building-info-row">
                <span class="building-info-label">Requirement</span>
                <span class="building-info-value">{vm().upgradeRequirement}</span>
              </div>
            )}
          </div>
        </div>

        {/* Right Panel / Actions — mirrors {Building}Window/RightPanel */}
        <div
          class="building-detail-right"
          data-source-hierarchy="{Building}Window/RightPanel/UpgradeWindow"
        >
          {vm().actions.length === 0 ? (
            <div class="building-info-card">
              <p style="margin:0;color:rgba(218,198,168,0.5);font-size:0.82rem;">
                No actions currently available for this building.
              </p>
            </div>
          ) : (
            <div class="building-action-section">
              <h3 class="building-action-section-title">Actions</h3>
              <For each={vm().actions}>
                {(action) => (
                  <div class="building-action-card">
                    <div class="building-action-card-header">
                      <span class="building-action-label">{action.label}</span>
                      {action.isUnsupported && (
                        <span class="building-action-pill building-action-pill--unsupported">
                          Unsupported
                        </span>
                      )}
                      {!action.isAvailable && !action.isUnsupported && (
                        <span class="building-action-pill building-action-pill--unavailable">
                          Unavailable
                        </span>
                      )}
                    </div>
                    <p class="building-action-desc">{action.description}</p>
                    <div class="building-action-footer">
                      <span class={`building-action-cost ${!action.isAvailable ? "building-action-cost-unavailable" : ""}`}>
                        Cost: <strong>{action.cost}</strong>
                      </span>
                      {action.isUnsupported ? (
                        <button class="building-action-btn building-action-btn--disabled" disabled>
                          Not Available
                        </button>
                      ) : action.isAvailable ? (
                        <button
                          class="building-action-btn building-action-btn--primary"
                          onClick={() => props.onAction(action.id)}
                        >
                          {action.label}
                        </button>
                      ) : (
                        <button class="building-action-btn building-action-btn--disabled" disabled>
                          Prerequisites Not Met
                        </button>
                      )}
                    </div>
                  </div>
                )}
              </For>
            </div>
          )}
        </div>
      </div>

      {/* ── Return to Town (mirrors CloseButton) ── */}
      <div class="building-return-row">
        <button
          class="building-return-btn"
          onClick={props.onReturn}
          data-source-component="CloseButton"
          data-source-sprite="Assets/Sprites/ui/btn_close.png"
        >
          Return to Town
        </button>
      </div>
    </div>
  );
};
