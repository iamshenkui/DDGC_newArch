const buildingImageById: Record<string, string> = {
  stagecoach: "/original/buildings/building_perception_tower.png",
  guild: "/original/buildings/building_train_field.png",
  blacksmith: "/original/buildings/building_forging.png",
  sanitarium: "/original/buildings/building_cell_repair.png",
  abbey: "/original/buildings/building_faith_altar.png",
  tavern: "/original/buildings/building_paradise.png"
};

const heroPortraitByKey: Record<string, string> = {
  hunter: "/original/heroes/hunter_portrait_roster.png",
  white: "/original/heroes/hunter1_portrait_roster.png",
  black: "/original/heroes/hunter2_portrait_roster.png"
};

function normalizeKey(value: string | undefined): string {
  return (value ?? "").trim().toLowerCase();
}

export function resolveBuildingImage(buildingId: string): string | undefined {
  return buildingImageById[normalizeKey(buildingId)];
}

export function resolveHeroPortrait(options: {
  heroId?: string;
  classLabel?: string;
}): string | undefined {
  const heroId = normalizeKey(options.heroId);
  const classLabel = normalizeKey(options.classLabel);

  if (heroId.includes("hunter") || classLabel === "hunter") {
    return heroPortraitByKey.hunter;
  }

  if (heroId.includes("white") || classLabel === "white") {
    return heroPortraitByKey.white;
  }

  if (heroId.includes("black") || classLabel === "black") {
    return heroPortraitByKey.black;
  }

  return heroPortraitByKey[classLabel];
}