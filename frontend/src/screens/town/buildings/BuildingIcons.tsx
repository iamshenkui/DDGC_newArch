import type { Component } from "solid-js";

/**
 * Building SVG icons referencing original Unity sprites from
 * asset-manifest.json (UIR-004). Each icon maps to the original
 * `.png` asset path so future sprite extraction can replace these
 * inline SVGs by sourcing the file at the given `data-asset-path`.
 *
 * Mapping:
 *   stagecoach → building_perception_tower.png  (87a55679...)
 *   guild      → building_train_field.png        (67a5e7ae...)
 *   blacksmith → building_forging.png            (23e01c10...)
 *   sanitarium → building_cell_repair.png        (55375034...)
 *   abbey      → building_faith_altar.png        (311540f1...)
 *   tavern     → building_paradise.png           (c0ea280d...)
 */

interface BuildingIconProps {
  buildingId: string;
  size?: number;
}

export const BuildingIcon: Component<BuildingIconProps> = (props) => {
  const size = () => props.size ?? 36;

  switch (props.buildingId) {
    case "stagecoach":
      return (
        <svg
          width={size()} height={size()}
          viewBox="0 0 36 36" fill="none"
          data-asset-path="Assets/Sprites/town/buildings/building_perception_tower.png"
          data-guid="87a55679f12a1e6489ecdb1d6e6f6b93"
        >
          {/* Perception Tower — conic spire with watch platform */}
          <rect x="13" y="4" width="10" height="4" rx="1" fill="#c6d46a" opacity="0.7" />
          <polygon points="13,8 23,8 21,18 15,18" fill="#a0ccbc" opacity="0.25" stroke="#c6d46a" stroke-width="0.6" />
          <rect x="11" y="18" width="14" height="3" rx="1" fill="#c6d46a" opacity="0.4" />
          <rect x="12" y="21" width="12" height="11" rx="1" fill="#1b322c" stroke="#c6d46a" stroke-width="0.5" opacity="0.7" />
          {/* Door */}
          <rect x="15" y="26" width="6" height="6" rx="1" fill="#0e1714" stroke="#c6d46a" stroke-width="0.4" opacity="0.6" />
          {/* Window */}
          <circle cx="18" cy="12" r="2" fill="#c6d46a" opacity="0.5" />
        </svg>
      );

    case "guild":
      return (
        <svg
          width={size()} height={size()}
          viewBox="0 0 36 36" fill="none"
          data-asset-path="Assets/Sprites/town/buildings/building_train_field.png"
          data-guid="67a5e7aed8029d84dbf9c9e497a944d2"
        >
          {/* Training Field — dojo gate with roof */}
          <polygon points="6,12 18,4 30,12" fill="#c6d46a" opacity="0.3" stroke="#c6d46a" stroke-width="0.6" />
          <rect x="6" y="12" width="24" height="16" rx="1" fill="#1b322c" stroke="#c6d46a" stroke-width="0.5" opacity="0.7" />
          <rect x="8" y="14" width="20" height="4" fill="#2a4048" opacity="0.5" />
          {/* Combat dummies */}
          <circle cx="14" cy="24" r="2.5" fill="#c6d46a" opacity="0.35" />
          <circle cx="22" cy="24" r="2.5" fill="#c6d46a" opacity="0.35" />
          <rect x="13.5" y="24" width="1" height="4" fill="#c6d46a" opacity="0.3" />
          <rect x="21.5" y="24" width="1" height="4" fill="#c6d46a" opacity="0.3" />
        </svg>
      );

    case "blacksmith":
      return (
        <svg
          width={size()} height={size()}
          viewBox="0 0 36 36" fill="none"
          data-asset-path="Assets/Sprites/town/buildings/building_forging.png"
          data-guid="23e01c10f262ddc4ba9977b91314b031"
        >
          {/* Forge — anvil + chimney */}
          <rect x="10" y="14" width="16" height="14" rx="2" fill="#1b322c" stroke="#c6d46a" stroke-width="0.5" opacity="0.7" />
          <rect x="14" y="6" width="8" height="8" rx="1" fill="#2a4048" stroke="#c6d46a" stroke-width="0.4" opacity="0.6" />
          <rect x="15" y="7" width="6" height="2" fill="#c6d46a" opacity="0.3" />
          {/* Anvil shape */}
          <polygon points="13,24 14,20 22,20 23,24" fill="#c6d46a" opacity="0.4" />
          <rect x="16" y="24" width="4" height="4" rx="0.5" fill="#c6d46a" opacity="0.25" />
          {/* Fire glow */}
          <circle cx="18" cy="22" r="2" fill="#ea7767" opacity="0.3" />
        </svg>
      );

    case "sanitarium":
      return (
        <svg
          width={size()} height={size()}
          viewBox="0 0 36 36" fill="none"
          data-asset-path="Assets/Sprites/town/buildings/building_cell_repair.png"
          data-guid="55375034893560044a266e905926e8ff"
        >
          {/* Cell / Repair — medical cross + chamber */}
          <rect x="10" y="10" width="16" height="18" rx="2" fill="#1b322c" stroke="#c6d46a" stroke-width="0.5" opacity="0.7" />
          {/* Medical cross */}
          <rect x="16" y="14" width="4" height="10" rx="0.5" fill="#c6d46a" opacity="0.45" />
          <rect x="13" y="17" width="10" height="4" rx="0.5" fill="#c6d46a" opacity="0.45" />
          {/* Roof */}
          <polygon points="8,10 18,4 28,10" fill="#c6d46a" opacity="0.3" stroke="#c6d46a" stroke-width="0.5" />
        </svg>
      );

    case "abbey":
      return (
        <svg
          width={size()} height={size()}
          viewBox="0 0 36 36" fill="none"
          data-asset-path="Assets/Sprites/town/buildings/building_faith_altar.png"
          data-guid="311540f167839cf4da00305566192b4a"
        >
          {/* Faith Altar — chapel with spire */}
          <polygon points="6,12 18,2 30,12" fill="#c6d46a" opacity="0.35" stroke="#c6d46a" stroke-width="0.6" />
          <rect x="8" y="12" width="20" height="16" rx="1" fill="#1b322c" stroke="#c6d46a" stroke-width="0.5" opacity="0.7" />
          {/* Stained glass */}
          <circle cx="18" cy="17" r="3.5" fill="#2a4048" stroke="#c6d46a" stroke-width="0.4" opacity="0.6" />
          <circle cx="18" cy="17" r="2" fill="#c6d46a" opacity="0.2" />
          {/* Altar */}
          <rect x="14" y="24" width="8" height="4" rx="0.5" fill="#c6d46a" opacity="0.3" />
          {/* Cross on top */}
          <rect x="17.5" y="3" width="1" height="3" fill="#c6d46a" opacity="0.4" />
          <rect x="16" y="4" width="4" height="1" fill="#c6d46a" opacity="0.4" />
        </svg>
      );

    case "tavern":
      return (
        <svg
          width={size()} height={size()}
          viewBox="0 0 36 36" fill="none"
          data-asset-path="Assets/Sprites/town/buildings/building_paradise.png"
          data-guid="c0ea280d2704bdb4a9621d6e181e0316"
        >
          {/* Paradise Tavern — mug + cozy roof */}
          <polygon points="7,10 18,3 29,10" fill="#c6d46a" opacity="0.3" stroke="#c6d46a" stroke-width="0.6" />
          <rect x="8" y="10" width="20" height="18" rx="1" fill="#1b322c" stroke="#c6d46a" stroke-width="0.5" opacity="0.7" />
          {/* Sign */}
          <rect x="13" y="13" width="10" height="6" rx="1" fill="#2a4048" stroke="#c6d46a" stroke-width="0.4" opacity="0.5" />
          {/* Mug icon on sign */}
          <rect x="16" y="14" width="4" height="4" rx="0.5" fill="#c6d46a" opacity="0.35" />
          <rect x="17" y="16" width="2" height="1" fill="#2a4048" opacity="0.4" />
          {/* Door */}
          <rect x="15" y="22" width="6" height="6" rx="1" fill="#0e1714" stroke="#c6d46a" stroke-width="0.4" opacity="0.6" />
        </svg>
      );

    default:
      /* Generic building marker */
      return (
        <svg
          width={size()} height={size()}
          viewBox="0 0 36 36" fill="none"
          data-asset-path="unresolved"
        >
          <rect x="10" y="10" width="16" height="18" rx="2" fill="#1b322c" stroke="#c6d46a" stroke-width="0.5" opacity="0.5" />
          <polygon points="8,10 18,4 28,10" fill="#c6d46a" opacity="0.2" stroke="#c6d46a" stroke-width="0.4" />
        </svg>
      );
  }
};
