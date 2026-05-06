import { describe, expect, it } from "vitest";

import { TOWN_BUILDING_CATALOG } from "./buildingCatalog";
import sceneJson from "../../../fixtures/ui_inventory/Assets/Scenes/EstateManagement.unity.json";

interface SceneNode {
  name: string;
  rect_transform?: {
    anchor_min?: { x: number; y: number };
    anchor_max?: { x: number; y: number };
    pivot?: { x: number; y: number };
    anchored_position?: { x: number; y: number };
    size_delta?: { x: number; y: number };
  } | null;
  children?: SceneNode[];
}

/** Resolve a child node by following the path of names. */
function resolveByPath(roots: SceneNode[], path: ReadonlyArray<string>): SceneNode | undefined {
  let cursor: SceneNode[] | undefined = roots;
  let found: SceneNode | undefined;
  for (const segment of path) {
    if (!cursor) return undefined;
    found = cursor.find((node) => node.name === segment);
    if (!found) return undefined;
    cursor = found.children;
  }
  return found;
}

function findBuildingLabel(node: SceneNode): SceneNode | undefined {
  if (!node.children) return undefined;
  return node.children.find((c) => c.name === "BuildingLabel");
}

/** Convert a bottom-anchored Unity Y to an equivalent center-anchored Y on the 720 canvas. */
function bottomToCenterY(unityY: number): number {
  // bottom-anchored Y is measured from canvas bottom (720 px tall)
  // equivalent center-anchored = unityY - 720/2
  return unityY - 720 / 2;
}

describe("building catalog — source-backed Unity parity (UIR-005E)", () => {
  const scene = sceneJson as unknown as { hierarchy: SceneNode[] };

  // Map frontend catalog ID → Unity GameObject name on the UI_Estate building layer.
  // Names alone are not unique in the scene (e.g. LegacyTower also appears under
  // UpgradeWindow). Lookups must follow the EstateSceneManager/UI_Estate/UI_Estate
  // path to land on the building-layer instance and not a homonym elsewhere.
  const idToUnityName: Record<string, string> = {
    guild: "Guild",
    tavern: "Tavern",
    graveyard: "Graveyard",
    blacksmith: "Blacksmith",
    stagecoach: "StageCoach",
    garden: "Garden",
    legacytower: "LegacyTower",
    abbey: "Abbey",
    market: "Market",
    sanitarium: "Sanitarium",
    campingtrainer: "CampingTrainer",
  };

  const ESTATE_BUILDING_LAYER_PATH = ["EstateSceneManager", "UI_Estate", "UI_Estate"] as const;

  function resolveBuilding(unityName: string): SceneNode | undefined {
    return resolveByPath(scene.hierarchy, [...ESTATE_BUILDING_LAYER_PATH, unityName]);
  }

  it("covers all 11 Unity building GameObjects", () => {
    expect(TOWN_BUILDING_CATALOG.length).toBe(11);
    for (const entry of TOWN_BUILDING_CATALOG) {
      const unityName = idToUnityName[entry.id];
      expect(unityName, `no Unity name mapped for catalog id "${entry.id}"`).toBeTruthy();
      expect(
        resolveBuilding(unityName),
        `Unity scene missing UI_Estate/UI_Estate/${unityName}`
      ).toBeTruthy();
    }
  });

  it.each(
    TOWN_BUILDING_CATALOG.map((e) => [e.id, e] as const)
  )("catalog entry %s matches Unity RectTransform", (_id, entry) => {
    const unityName = idToUnityName[entry.id];
    const node = resolveBuilding(unityName)!;

    const rt = node.rect_transform!;
    const isBottomAnchored =
      rt.anchor_min!.y === 0 && rt.anchor_max!.y === 0;

    // Position parity: ±1 px tolerance
    let expectedY: number;
    if (isBottomAnchored) {
      expectedY = bottomToCenterY(rt.anchored_position!.y);
    } else {
      expectedY = rt.anchored_position!.y;
    }
    expect(entry.x).toBeCloseTo(rt.anchored_position!.x, 1);
    expect(entry.y).toBeCloseTo(expectedY, 1);

    // Size parity
    expect(entry.width).toBeCloseTo(rt.size_delta!.x, 1);
    expect(entry.height).toBeCloseTo(rt.size_delta!.y, 1);

    // Anchor documentation
    if (isBottomAnchored) {
      expect(entry.y).toBe(bottomToCenterY(rt.anchored_position!.y));
    }
  });

  it.each(
    TOWN_BUILDING_CATALOG.map((e) => [e.id, e] as const)
  )("catalog entry %s matches Unity BuildingLabel offset", (_id, entry) => {
    const unityName = idToUnityName[entry.id];
    const node = resolveBuilding(unityName)!;
    const labelNode = findBuildingLabel(node);

    expect(labelNode, `Unity "${unityName}" missing BuildingLabel child`).toBeDefined();
    const lrt = labelNode!.rect_transform!;

    // Label position: offset from building center
    // Unity BuildingLabel uses center anchor (0.5,0.5)
    const expectedOffsetX = lrt.anchored_position!.x;
    const expectedOffsetY = lrt.anchored_position!.y;

    expect(entry.labelOffsetX).toBeCloseTo(expectedOffsetX, 1);
    expect(entry.labelOffsetY).toBeCloseTo(expectedOffsetY, 1);
  });

  it.each(
    TOWN_BUILDING_CATALOG.map((e) => [e.id, e] as const)
  )("catalog entry %s BuildingLabel uses the source-fixed 326×48 banner", (_id, entry) => {
    const unityName = idToUnityName[entry.id];
    const node = resolveBuilding(unityName)!;
    const labelNode = findBuildingLabel(node);
    expect(labelNode).toBeDefined();

    const lrt = labelNode!.rect_transform!;
    expect(lrt.size_delta!.x).toBeCloseTo(326, 1);
    expect(lrt.size_delta!.y).toBeCloseTo(48, 1);
  });
});
