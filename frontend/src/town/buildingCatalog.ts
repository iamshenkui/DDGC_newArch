import type { TownBuildingSummary } from "../bridge/contractTypes";

export interface TownBuildingCatalogEntry {
  id: string;
  displayName: string;
  summary: string;
  x: number;
  y: number;
  width: number;
  height: number;
  labelOffsetX: number;
  labelOffsetY: number;
  /** Unity source prefab path (e.g. "UI/Estate/BuildingSlot") */
  sourcePrefab: string;
  /** Unity source scene (e.g. "EstateManagement.unity") */
  sourceScene: string;
  /** Unity asset GUID when available from the staged asset inventory */
  sourceGuid: string | null;
}

const BUILDING_SOURCE = {
  scene: "EstateManagement.unity",
  prefab: "UI/Estate/BuildingSlot"
} as const;

export const TOWN_BUILDING_CATALOG: ReadonlyArray<TownBuildingCatalogEntry> = [
  {
    id: "guild",
    displayName: "试炼场",
    summary: "训练技能并调整队伍战斗能力。",
    x: 120, y: -50,
    width: 397, height: 397,
    labelOffsetX: 30, labelOffsetY: -50,
    sourcePrefab: BUILDING_SOURCE.prefab,
    sourceScene: BUILDING_SOURCE.scene,
    sourceGuid: "67a5e7aed8029d84dbf9c9e497a944d2"
  },
  {
    id: "tavern",
    displayName: "迷情乐园",
    summary: "通过酒馆活动缓解压力并恢复状态。",
    x: -650, y: -220,
    width: 519, height: 519,
    labelOffsetX: -6, labelOffsetY: 160,
    sourcePrefab: BUILDING_SOURCE.prefab,
    sourceScene: BUILDING_SOURCE.scene,
    sourceGuid: "c0ea280d2704bdb4a9621d6e181e0316"
  },
  {
    id: "graveyard",
    displayName: "英雄档案馆",
    summary: "查看阵亡与历史记录。",
    x: 695.5, y: 95.63,
    width: 344, height: 344,
    labelOffsetX: 100, labelOffsetY: -100,
    sourcePrefab: BUILDING_SOURCE.prefab,
    sourceScene: BUILDING_SOURCE.scene,
    sourceGuid: null
  },
  {
    id: "blacksmith",
    displayName: "锻造舱",
    summary: "升级与维护武器、防具和装备。",
    // Unity source: anchorMin=(0.5,0), anchorMax=(0.5,0), anchoredPosition=(709,294)
    // Converted to center-based: y = -(1080/2) + 294 = -246
    x: 709, y: -246,
    width: 482, height: 482,
    labelOffsetX: 0, labelOffsetY: -168,
    sourcePrefab: BUILDING_SOURCE.prefab,
    sourceScene: BUILDING_SOURCE.scene,
    sourceGuid: "23e01c10f262ddc4ba9977b91314b031"
  },
  {
    id: "stagecoach",
    displayName: "次元感知塔",
    summary: "招募新英雄并扩充可用名册。",
    x: 31, y: -267,
    width: 384, height: 384,
    labelOffsetX: -26, labelOffsetY: -150,
    sourcePrefab: BUILDING_SOURCE.prefab,
    sourceScene: BUILDING_SOURCE.scene,
    sourceGuid: "87a55679f12a1e6489ecdb1d6e6f6b93"
  },
  {
    id: "garden",
    displayName: "天国花园",
    summary: "提供特殊休整与恢复服务。",
    x: -245, y: -211,
    width: 371, height: 371,
    labelOffsetX: 0, labelOffsetY: -130,
    sourcePrefab: BUILDING_SOURCE.prefab,
    sourceScene: BUILDING_SOURCE.scene,
    sourceGuid: "c34c4012911d41c4bbed2328d7025138"
  },
  {
    id: "legacytower",
    displayName: "遗留塔",
    summary: "查看传承与博物馆式收藏内容。",
    x: 0, y: 270,
    width: 541, height: 541,
    labelOffsetX: -3, labelOffsetY: 166,
    sourcePrefab: BUILDING_SOURCE.prefab,
    sourceScene: BUILDING_SOURCE.scene,
    sourceGuid: "ebd56d507c80af54986327ac7cb16661"
  },
  {
    id: "abbey",
    displayName: "信仰祭坛",
    summary: "通过祈祷与仪式降低英雄压力。",
    x: -330, y: 90,
    width: 519, height: 519,
    labelOffsetX: -10, labelOffsetY: 153,
    sourcePrefab: BUILDING_SOURCE.prefab,
    sourceScene: BUILDING_SOURCE.scene,
    sourceGuid: "311540f167839cf4da00305566192b4a"
  },
  {
    id: "market",
    displayName: "交易市场",
    summary: "购买补给、物资与商店类服务。",
    x: 340.5, y: -222,
    width: 326, height: 339,
    labelOffsetX: 0, labelOffsetY: -150,
    sourcePrefab: BUILDING_SOURCE.prefab,
    sourceScene: BUILDING_SOURCE.scene,
    sourceGuid: "8c3505ec8c7e25b42aacdcaece82f821"
  },
  {
    id: "sanitarium",
    displayName: "细胞修复站",
    summary: "治疗怪癖、疾病并处理长期异常。",
    x: 390, y: 110,
    width: 339, height: 339,
    labelOffsetX: 50, labelOffsetY: -100,
    sourcePrefab: BUILDING_SOURCE.prefab,
    sourceScene: BUILDING_SOURCE.scene,
    sourceGuid: "55375034893560044a266e905926e8ff"
  },
  {
    id: "campingtrainer",
    displayName: "空间分析",
    summary: "营火与露营训练相关服务。",
    x: -607, y: 220,
    width: 266, height: 314,
    labelOffsetX: -100, labelOffsetY: -110,
    sourcePrefab: BUILDING_SOURCE.prefab,
    sourceScene: BUILDING_SOURCE.scene,
    sourceGuid: null
  }
];

const TOWN_BUILDING_CATALOG_BY_ID = new Map(TOWN_BUILDING_CATALOG.map((entry) => [entry.id, entry]));

export function getTownBuildingCatalogEntry(buildingId: string): TownBuildingCatalogEntry | undefined {
  return TOWN_BUILDING_CATALOG_BY_ID.get(buildingId);
}

export function createTownBuildingSummary(
  buildingId: string,
  status: TownBuildingSummary["status"] = "partial"
): TownBuildingSummary {
  const entry = getTownBuildingCatalogEntry(buildingId);
  return {
    id: buildingId,
    label: entry?.displayName ?? buildingId,
    summary: entry?.summary ?? "城镇建筑服务入口。",
    status
  };
}

export function mergeTownBuildings(
  buildings: ReadonlyArray<TownBuildingSummary>
): ReadonlyArray<TownBuildingSummary> {
  const runtimeById = new Map(buildings.map((building) => [building.id, building]));

  return TOWN_BUILDING_CATALOG.map((entry) => {
    const runtimeBuilding = runtimeById.get(entry.id);
    if (runtimeBuilding) {
      return {
        ...runtimeBuilding,
        label: entry.displayName,
        summary: entry.summary
      };
    }

    return createTownBuildingSummary(entry.id, "partial");
  });
}