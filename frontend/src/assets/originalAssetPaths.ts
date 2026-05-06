/**
 * Original Unity asset path resolvers.
 *
 * Maps building IDs, hero class labels, and shell chrome elements to the
 * physical PNG sprites extracted from the DreamDeveloperGame-Crossover
 * Unity project.
 *
 * Current repo status (frontend/public/original/):
 *   buildings/  11 building sprites (all 11 town catalog entries — full set)
 *   chrome/     10 shell chrome sprites (nameplate, buttons, embark, gold,
 *               building label/icon backgrounds)
 *   heroes/     3 hunter family portraits (base, white, black variants)
 *   startup/    6 startup sprites + 1 font (background, title wordmark, title
 *               bg, parchment dialogs, menu button, CJK display font)
 *   fonts/      ZhiYiSongTi-Regular.ttf (CJK display font from CampaignSelection.unity)
 *   All other hero families (alchemist, diviner, shaman, tank) — not yet extracted
 *
 * See asset-manifest.json for the full inventory with GUIDs and deferred items.
 */

// ── Building sprites ──────────────────────────────────────────────────────
// All 11 town building catalog entries, staged in frontend/public/original/buildings/

const buildingImageById: Record<string, string> = {
  // Scoped (UIR-006/UIR-007 core set)
  stagecoach: "/original/buildings/building_perception_tower.png",
  guild: "/original/buildings/building_train_field.png",
  blacksmith: "/original/buildings/building_forging.png",
  sanitarium: "/original/buildings/building_cell_repair.png",
  abbey: "/original/buildings/building_faith_altar.png",
  tavern: "/original/buildings/building_paradise.png",
  // Deferred buildings (UIR-005B staged)
  garden: "/original/buildings/building_garden.png",
  graveyard: "/original/buildings/building_hero_archive.png",
  legacytower: "/original/buildings/building_legacy_tower.png",
  market: "/original/buildings/building_market.png",
  campingtrainer: "/original/buildings/building_space_analysis.png"
};

// ── Startup / title-screen assets (KUI-P1-003) ─────────────────────────────
// Source: Assets/Scenes/CampaignSelection.unity
// Staged in frontend/public/original/startup/

const startupPaths = {
  // Full-bleed stone-wall background
  startupBackground: "/original/startup/scene_bg.png",
  // Title wordmark "跨纪元契约" with integrated sword/cross ornament
  startupTitleWordmark: "/original/startup/game_logo.png",
  // Title background underlay
  startupTitleBg: "/original/startup/save_title_bg.png",
  // Parchment menu dialog (main menu panel)
  startupParchmentDialog: "/original/startup/dialog07.png",
  // Parchment save-frame dialog (save slot picker)
  startupSaveFrameDialog: "/original/startup/dialog02.png",
  // Menu button sprite (shared by all four menu items)
  startupMenuButton: "/original/startup/btn_red.png",
  // CJK display font used by CampaignSelection.unity text components
  startupFont: "/fonts/ZhiYiSongTi-Regular.ttf"
} as const;

export type StartupAssetKey = keyof typeof startupPaths;

export function resolveStartupAsset(key: StartupAssetKey): string {
  return startupPaths[key];
}

// ── Shell chrome sprites (UIR-005B) ────────────────────────────────────────
// Town screen shell chrome staged in frontend/public/original/chrome/.
// Covers top nameplate, side-button chrome, embark control, currency
// iconography, and building label backgrounds.

const chromePaths = {
  // Top nameplate
  estateNameBg: "/original/chrome/estate_name_bg.png",
  // Currency
  goldIcon: "/original/chrome/gold.png",
  bustIcon: "/original/chrome/gold.png",
  portraitIcon: "/original/chrome/gold.png",
  deedIcon: "/original/chrome/gold.png",
  crestIcon: "/original/chrome/gold.png",
  // Embark
  embarkButton: "/original/chrome/btn_play.png",
  // Side navigation (6 buttons from Unity UI_Shared/UI_Panels/BottomPanel/SideButtons)
  sideActivityLog: "/original/chrome/btn_save_02.png",
  sideRealmInventory: "/original/chrome/btn_save_02.png",
  sideHero: "/original/chrome/btn_save_01.png",
  sideTownEvent: "/original/chrome/btn_save_02.png",
  sideSettings: "/original/chrome/btn_save_01.png",
  sideGlossary: "/original/chrome/btn_save_02.png",
  sideClose: "/original/chrome/btn_close.png",
  // Quick action buttons
  quickProgress: "/original/chrome/btn_save_02.png",
  quickStart: "/original/chrome/btn_save_01.png",
  // Building chrome
  buildingLabelBg: "/original/chrome/building_label_bg01.png",
  buildingIconBg: "/original/chrome/building_icon_bg.png",
  buildingTitleBg: "/original/chrome/building_title_bg.png",
  buildingInfoBg: "/original/chrome/building_info_bg.png"
} as const;

export type ChromeAssetKey = keyof typeof chromePaths;

export function resolveChromeAsset(key: ChromeAssetKey): string {
  return chromePaths[key];
}

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
