import type { Component } from "solid-js";

import { resolveBuildingImage } from "../../../assets/originalAssetPaths";

interface BuildingDetailHeaderProps {
  buildingId: string;
  label: string;
  status: "ready" | "partial" | "locked";
  description: string;
  sourcePrefabPath: string;
  sourceSpritePath: string;
  sourceGuid: string;
}

const STATUS_LABEL: Record<string, string> = {
  ready: "Operational",
  partial: "Partially Available",
  locked: "Locked",
};

const STATUS_CLASS: Record<string, string> = {
  ready: "status-ready",
  partial: "status-partial",
  locked: "status-locked",
};

/**
 * Building detail header — mirrors the Unity building window icon/title area.
 *
 * Unity prefab hierarchy:
 *   UI_Shared/UI_LowWindows/{Building}Window/
 *     LeftPanel/
 *       Icon             → .building-detail-sprite image
 *       Title/
 *         BuildingLabel  → .building-detail-name
 *         BuildingDesc   → .building-detail-desc
 *     TalkButton         → (mapped to first action)
 *     CloseButton        → Return to Town
 */
export const BuildingDetailHeader: Component<BuildingDetailHeaderProps> = (props) => {
  const spriteSrc = () => resolveBuildingImage(props.buildingId);
  const statusLabel = () => STATUS_LABEL[props.status] ?? props.status;
  const statusClass = () => STATUS_CLASS[props.status] ?? "status-locked";

  return (
    <div
      class="building-detail-header"
      data-source-prefab={props.sourcePrefabPath}
      data-source-hierarchy="UI_Shared/UI_LowWindows/{Building}Window/LeftPanel"
    >
      {/* Icon area — mirrors {Building}Window/LeftPanel/Icon */}
      <div
        class="building-detail-sprite"
        data-source-component="BuildingIcon"
        data-source-sprite={props.sourceSpritePath}
        data-source-guid={props.sourceGuid}
      >
        {spriteSrc() ? (
          <img
            class="building-detail-sprite-img"
            src={spriteSrc()}
            alt={props.label}
            loading="eager"
          />
        ) : (
          <div class="building-detail-sprite-fallback">
            <span>{props.label[0]?.toUpperCase() ?? "?"}</span>
          </div>
        )}
      </div>

      {/* Title area — mirrors {Building}Window/LeftPanel/Title */}
      <div
        class="building-detail-title-area"
        data-source-hierarchy="{Building}Window/LeftPanel/Title"
      >
        <span class="building-detail-eyebrow">Building</span>
        <h2
          class="building-detail-name"
          data-source-component="BuildingLabel"
          data-source-sprite="Assets/Sprites/ui/building_title_bg.png"
        >
          {props.label}
        </h2>
        <span class={`building-detail-status ${statusClass()}`}>
          {statusLabel()}
        </span>
        <p
          class="building-detail-desc"
          data-source-component="BuildingDesc"
        >
          {props.description}
        </p>
      </div>
    </div>
  );
};
