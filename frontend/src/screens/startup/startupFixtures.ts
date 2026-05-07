/**
 * Phase 1 startup menu state fixtures.
 *
 * Source-faithful copy and option lists for the startup menu sub-states
 * (save selection, settings, region transition) staged from the source
 * Unity scenes:
 *   Assets/Scenes/CampaignSelection.unity     — main menu, save selector
 *   Assets/Scenes/SettingsPanel.unity         — settings panel
 *   Assets/Scenes/RegionTransition.unity      — region intro transition
 *
 * Reference screenshots (ref_image/跨际元契约/1系统界面/):
 *   开始界面.png  — main menu
 *   存档界面.png  — save selection
 *   设置界面.png  — settings panel
 *   转场.png      — region transition (Zhuque continent)
 *
 * Settings values are bound to documented fixture defaults (no runtime
 * settings store yet) — interactive fields persist within the session
 * but do not yet affect engine behavior.
 *
 * Transition copy mirrors 转场.png anchor text for the Zhuque region.
 */

// ── Save selection ────────────────────────────────────────────────────────
// Slot list pulled from saveLoad service. The first slot is always a
// "new save" placeholder (click to start new adventure). Empty save state
// renders the placeholder slot only (no existing campaigns).

export interface SaveSlotFixture {
  /** Stable slot ID — for now uses ordinal index (0 = new, 1.. = saves). */
  id: string;
  /** Display label (e.g. "存档1"). For the placeholder slot, "" — UI shows the empty hint. */
  label: string;
  /** Optional rank/week summary (e.g. "第1周"). */
  rankSummary: string;
  /** True when this slot is the empty placeholder for a new adventure. */
  isPlaceholder: boolean;
}

/** Empty-state placeholder slot — always rendered first in the save list. */
export const PLACEHOLDER_SAVE_SLOT: SaveSlotFixture = {
  id: "new",
  label: "",
  rankSummary: "",
  isPlaceholder: true
};

/** Empty-state hint text for the placeholder slot (mirrors 存档界面.png). */
export const PLACEHOLDER_SLOT_HINT = "点击开始新的冒险...";

/** Save panel title (mirrors 存档界面.png header). */
export const SAVE_PANEL_TITLE = "存档选择";

// ── Settings ──────────────────────────────────────────────────────────────
// Setting rows mirror 设置界面.png in source order. Sliders use 0-100
// integer ranges; dropdowns are bound to the source option lists.

export interface SettingsRowFixture {
  /** Stable settings key. */
  id: string;
  /** Source-faithful Chinese label. */
  label: string;
  /** Control kind — slider for audio levels, select for picker rows. */
  kind: "slider" | "select";
  /** Default value (numeric for sliders, option ID for selects). */
  defaultValue: number | string;
  /** Discrete options for selects, in display order. */
  options?: ReadonlyArray<{ id: string; label: string }>;
}

/** Settings panel title (mirrors 设置界面.png header). */
export const SETTINGS_PANEL_TITLE = "设置";

export const SETTINGS_ROWS: ReadonlyArray<SettingsRowFixture> = [
  { id: "masterVolume", label: "主音量", kind: "slider", defaultValue: 70 },
  { id: "musicVolume", label: "音乐", kind: "slider", defaultValue: 60 },
  { id: "sfxVolume", label: "音效", kind: "slider", defaultValue: 80 },
  {
    id: "resolution",
    label: "分辨率",
    kind: "select",
    defaultValue: "1920x1080",
    options: [
      { id: "1920x1080", label: "1920×1080" },
      { id: "1600x900", label: "1600×900" },
      { id: "1280x720", label: "1280×720" }
    ]
  },
  {
    id: "language",
    label: "语言",
    kind: "select",
    defaultValue: "zh-CN",
    options: [
      { id: "zh-CN", label: "简体中文" },
      { id: "zh-TW", label: "繁體中文" },
      { id: "en-US", label: "English" }
    ]
  },
  {
    id: "windowed",
    label: "窗体化",
    kind: "select",
    defaultValue: "off",
    options: [
      { id: "off", label: "关" },
      { id: "on", label: "开" }
    ]
  },
  {
    id: "vsync",
    label: "垂直同步",
    kind: "select",
    defaultValue: "off",
    options: [
      { id: "off", label: "关" },
      { id: "on", label: "开" }
    ]
  }
];

// ── Region transition ─────────────────────────────────────────────────────
// Zhuque (朱雀) continent intro transition shown after starting a new
// game. Phase 1 surfaces the region label, flavor text, and continue
// prompt over the staged scene background — the dedicated battlefield
// illustration is deferred until the transition asset is extracted.

export const TRANSITION_REGION_TITLE = "朱雀大陆";
export const TRANSITION_REGION_FLAVOR =
  "绝望的炎熔，仿佛连太阳都能被禁尽";
export const TRANSITION_CONTINUE_HINT =
  "按【空格】或【点击】继续游戏";

// ── Version label ─────────────────────────────────────────────────────────
// Mirrors V 1.0 anchor at the bottom of every menu state.
export const STARTUP_VERSION_LABEL = "V 1.0";
