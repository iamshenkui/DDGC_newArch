import type { Component } from "solid-js";

import { resolveBuildingImage } from "../../../assets/originalAssetPaths";

interface BuildingIconProps {
  buildingId: string;
  size?: number;
}

export const BuildingIcon: Component<BuildingIconProps> = (props) => {
  const size = () => props.size ?? 36;
  const src = () => resolveBuildingImage(props.buildingId);

  if (src()) {
    return (
      <img
        class="building-icon-image"
        src={src()}
        alt=""
        width={size()}
        height={size()}
        loading="eager"
      />
    );
  }

  return (
    <span class="building-icon-fallback" aria-hidden="true">
      {props.buildingId[0]?.toUpperCase() ?? "?"}
    </span>
  );
};
