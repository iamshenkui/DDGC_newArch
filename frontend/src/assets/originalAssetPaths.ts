/**
 * Original Unity asset path resolvers.
 *
 * Maps building IDs and hero class labels to the physical PNG sprites
 * extracted from the DreamDeveloperGame-Crossover Unity project.
 *
 * Current repo status (frontend/public/original/):
 *   6 building sprites (all scoped buildings)
 *   3 hunter family portraits (base, white variant, black variant)
 *   All other hero families (alchemist, diviner, shaman, tank) — not yet extracted
 *
 * See asset-manifest.json for the full inventory with GUIDs and deferred items.
 */

// ── Building sprites ──────────────────────────────────────────────────────
// 6 scoped buildings, all present in frontend/public/original/buildings/

const buildingImageById: Record<string, string> = {
  stagecoach: "/original/buildings/building_perception_tower.png",
  guild: "/original/buildings/building_train_field.png",
  blacksmith: "/original/buildings/building_forging.png",
  sanitarium: "/original/buildings/building_cell_repair.png",
  abbey: "/original/buildings/building_faith_altar.png",
  tavern: "/original/buildings/building_paradise.png"
};

// ── Hero portraits ────────────────────────────────────────────────────────
// 5 recruitable families x 3 chaos variants each. Currently only hunter
// portraits are extracted; others fall through to the CSS-letter avatar.

type PortraitMap = Record<string, string>;

/** Families whose portrait PNGs exist in frontend/public/original/heroes/. */
const extractedPortraits: PortraitMap = {
  hunter: "/original/heroes/hunter_portrait_roster.png",
  hunter1: "/original/heroes/hunter1_portrait_roster.png",
  hunter2: "/original/heroes/hunter2_portrait_roster.png"
};

/**
 * Full portrait path patterns for all 5 families x 3 variants.
 * Families other than hunter are documented here with expected paths
 * but will resolve to undefined until extracted from the Unity project.
 */
const allPortraitPaths: PortraitMap = {
  // Extracted (present in repo)
  hunter: "/original/heroes/hunter_portrait_roster.png",
  hunter1: "/original/heroes/hunter1_portrait_roster.png",
  hunter2: "/original/heroes/hunter2_portrait_roster.png",

  // Not yet extracted (expected paths documented in asset-manifest.json)
  alchemist: "/original/heroes/alchemist_portrait_roster.png",
  alchemist1: "/original/heroes/alchemist1_portrait_roster.png",
  alchemist2: "/original/heroes/alchemist2_portrait_roster.png",
  diviner: "/original/heroes/diviner_portrait_roster.png",
  diviner1: "/original/heroes/diviner1_portrait_roster.png",
  diviner2: "/original/heroes/diviner2_portrait_roster.png",
  shaman: "/original/heroes/shaman_portrait_roster.png",
  shaman1: "/original/heroes/shaman1_portrait_roster.png",
  shaman2: "/original/heroes/shaman2_portrait_roster.png",
  tank: "/original/heroes/tank_portrait_roster.png",
  tank1: "/original/heroes/tank1_portrait_roster.png",
  tank2: "/original/heroes/tank2_portrait_roster.png"
};

// ── Chaos variant suffixes ─────────────────────────────────────────────────
// DDGC chaos ranges: stored < 50 → black variant (+2 suffix),
// 50-149 → base, >= 150 → white variant (+1 suffix)

/** White variant suffix appended to base class ID. */
const WHITE_SUFFIX = "1";
/** Black variant suffix appended to base class ID. */
const BLACK_SUFFIX = "2";

function normalizeKey(value: string | undefined): string {
  return (value ?? "").trim().toLowerCase();
}

/** Normalize a Rust display name (e.g. "Alchemist") to its base class ID. */
function classLabelToBaseId(label: string): string {
  return normalizeKey(label);
}

/**
 * Resolve the variant key for portrait lookup.
 *
 * @param baseId  The hero's base class ID (e.g. "hunter", "alchemist").
 * @param variant Optional variant suffix ("1" for white, "2" for black).
 *                Pass null/undefined for the base variant.
 * @returns The lookup key into allPortraitPaths.
 */
function variantKey(baseId: string, variant?: string | null): string {
  if (!variant) return baseId;
  return `${baseId}${variant}`;
}

// ── Public resolvers ──────────────────────────────────────────────────────

export function resolveBuildingImage(buildingId: string): string | undefined {
  return buildingImageById[normalizeKey(buildingId)];
}

export interface HeroPortraitOptions {
  /** Unique hero ID from the view model. May include variant suffix. */
  heroId?: string;
  /** Human-readable class label (e.g. "Hunter", "Alchemist"). */
  classLabel?: string;
  /**
   * Chaos variant: "base" (or undefined/null) for normal,
   * "white" for chaos >= 150, "black" for chaos < 50.
   * When omitted, inferred from heroId suffix conventions.
   */
  variant?: "base" | "white" | "black";
}

export function resolveHeroPortrait(options: HeroPortraitOptions): string | undefined {
  const classLabel = normalizeKey(options.classLabel);
  const heroId = normalizeKey(options.heroId);

  // 1. Try exact match by heroId in the full portrait map
  //    (e.g. "hunter1" or "hunter2" for variant heroes)
  if (heroId && allPortraitPaths[heroId]) {
    // Only return if the file is actually extracted (check extractedPortraits)
    if (extractedPortraits[heroId]) {
      return allPortraitPaths[heroId];
    }
  }

  // 2. Try by classLabel match
  const baseId = classLabelToBaseId(classLabel);

  // Build the variant key from the options
  let key: string;
  if (options.variant === "white") {
    key = `${baseId}${WHITE_SUFFIX}`;
  } else if (options.variant === "black") {
    key = `${baseId}${BLACK_SUFFIX}`;
  } else {
    key = baseId;
  }

  // Only return the path if the file is actually extracted
  if (extractedPortraits[key]) {
    return allPortraitPaths[key];
  }

  // 3. Fallback: if the classLabel itself is a known extracted key (e.g. "hunter")
  if (extractedPortraits[baseId]) {
    return allPortraitPaths[baseId];
  }

  // 4. No portrait available — the caller should render a CSS-letter avatar
  return undefined;
}
