import { Match, Switch, createSignal, For, type Component } from "solid-js";
import { resolveStartupAsset } from "../../assets/originalAssetPaths";
import {
  PLACEHOLDER_SAVE_SLOT,
  PLACEHOLDER_SLOT_HINT,
  SAVE_PANEL_TITLE,
  SETTINGS_PANEL_TITLE,
  SETTINGS_ROWS,
  STARTUP_VERSION_LABEL,
  TRANSITION_CONTINUE_HINT,
  TRANSITION_REGION_FLAVOR,
  TRANSITION_REGION_TITLE,
  type SaveSlotFixture,
  type SettingsRowFixture
} from "./startupFixtures";

interface StartupScreenProps {
  onReplayBoot: () => void;
  onLiveBoot: () => void;
  onNewCampaign: () => void;
  onLoadCampaign: () => void;
  onDeleteCampaign?: () => void;
  onQuit?: () => void;
  hasSavedCampaign: boolean;
  /** Optional summary of an existing campaign (e.g. "存档8"). */
  savedCampaignLabel?: string;
  /** Optional rank/week summary for an existing campaign (e.g. "第3周"). */
  savedCampaignRank?: string;
}

type StartupMenuState = "main" | "save-select" | "settings" | "transition";

/**
 * Title screen rebuilt from the original kuajiyuanqi (跨纪元契约)
 * CampaignSelection.unity scene. Phase 1 (KUI-P1-005) wires the four
 * source-faithful menu controls to their state surfaces:
 *   新游戏        → transition state → onNewCampaign
 *   继续游戏/读取存档 → save selection state (parchment frame, dialog02.png)
 *   设置          → settings state (parchment frame, dialog07.png)
 *   退出游戏      → onQuit (closes the campaign session)
 *
 * The replay/live boot affordances are retained as a small developer
 * dock at the bottom of the main menu so the runtime test harness can
 * still reach the live bridge path. Frame layout (background, parchment,
 * landscape composition) and source-backed iconography are unchanged
 * from KUI-P1-004.
 *
 * Source assets (KUI-P1-003): scene_bg.png · dialog02.png · dialog07.png ·
 * game_logo.png · btn_red.png · save_title_bg.png · ZhiYiSongTi-Regular.ttf,
 * all from CampaignSelection.unity.
 */
export const StartupScreen: Component<StartupScreenProps> = (props) => {
  const sceneBg = resolveStartupAsset("startupBackground");
  const titleWordmark = resolveStartupAsset("startupTitleWordmark");
  const dialogFrame = resolveStartupAsset("startupParchmentDialog");
  const dialogSaveFrame = resolveStartupAsset("startupSaveFrameDialog");
  const titleBg = resolveStartupAsset("startupTitleBg");

  const [menuState, setMenuState] = createSignal<StartupMenuState>("main");
  const [settingsValues, setSettingsValues] = createSignal<
    Record<string, number | string>
  >(
    SETTINGS_ROWS.reduce(
      (acc, row) => {
        acc[row.id] = row.defaultValue;
        return acc;
      },
      {} as Record<string, number | string>
    )
  );

  const goMain = () => setMenuState("main");
  const goSaveSelect = () => setMenuState("save-select");
  const goSettings = () => setMenuState("settings");
  const goTransition = () => setMenuState("transition");

  const handleNewGameClick = () => {
    goTransition();
  };

  const handleTransitionContinue = () => {
    props.onNewCampaign();
  };

  const handleQuitClick = () => {
    if (props.onQuit) {
      props.onQuit();
      return;
    }
    if (typeof window !== "undefined" && typeof window.close === "function") {
      window.close();
    }
  };

  const handleSaveSlotSelect = (slot: SaveSlotFixture) => {
    if (slot.isPlaceholder) {
      goTransition();
      return;
    }
    props.onLoadCampaign();
  };

  const handleSaveSlotDelete = (slot: SaveSlotFixture, _event: MouseEvent) => {
    if (slot.isPlaceholder) {
      // The "+" action on the placeholder slot is the same affordance as the
      // row itself — start a new campaign via the transition screen.
      goTransition();
      return;
    }
    if (props.onDeleteCampaign) {
      props.onDeleteCampaign();
    }
  };

  const handleSettingChange = (id: string, value: number | string) => {
    setSettingsValues((prev) => ({ ...prev, [id]: value }));
  };

  const saveSlots = (): ReadonlyArray<SaveSlotFixture> => {
    const slots: SaveSlotFixture[] = [PLACEHOLDER_SAVE_SLOT];
    if (props.hasSavedCampaign) {
      slots.push({
        id: "saved-1",
        label: props.savedCampaignLabel ?? "存档1",
        rankSummary: props.savedCampaignRank ?? "第1周",
        isPlaceholder: false
      });
    }
    return slots;
  };

  const handleTransitionKey = (event: KeyboardEvent) => {
    if (event.key === " " || event.code === "Space" || event.key === "Enter") {
      event.preventDefault();
      handleTransitionContinue();
    }
  };

  return (
    <main
      class="startup-screen"
      data-source-scene="Assets/Scenes/CampaignSelection.unity"
      data-source-asset-key="startupBackground"
      data-startup-state={menuState()}
      style={{ "background-image": `url(${sceneBg})` }}
    >
      <div
        class="startup-stage"
        data-source-component="CampaignSelectionCanvas"
      >
        <h1
          class="startup-title"
          data-source-sprite="Assets/Sprites/ui/game_logo.png"
          data-source-asset-key="startupTitleWordmark"
        >
          <img
            class="startup-title-wordmark"
            src={titleWordmark}
            alt="跨纪元契约"
            draggable={false}
          />
        </h1>

        <Switch>
          <Match when={menuState() === "main"}>
            <section
              class="startup-frame startup-frame-main"
              data-source-sprite="Assets/Sprites/ui/dialog07.png"
              data-source-asset-key="startupParchmentDialog"
              data-startup-panel="main"
              aria-label="跨纪元契约 主菜单 frame"
              style={{ "background-image": `url(${dialogFrame})` }}
            >
              <nav
                class="startup-menu"
                data-source-component="MenuOptions"
                data-source-button-sprite="Assets/Sprites/ui/btn_red.png"
                aria-label="跨纪元契约 主菜单"
              >
                <button
                  class="startup-menu-button"
                  data-source-sprite="Assets/Sprites/ui/btn_red.png"
                  data-source-component="NewGameButton"
                  type="button"
                  onClick={handleNewGameClick}
                >
                  新游戏
                </button>

                <button
                  class="startup-menu-button"
                  data-source-sprite="Assets/Sprites/ui/btn_red.png"
                  data-source-component="ContinueGameButton"
                  type="button"
                  onClick={goSaveSelect}
                >
                  继续游戏 / 读取存档
                </button>

                <button
                  class="startup-menu-button"
                  data-source-sprite="Assets/Sprites/ui/btn_red.png"
                  data-source-component="SettingsButton"
                  type="button"
                  onClick={goSettings}
                >
                  设置
                </button>

                <button
                  class="startup-menu-button"
                  data-source-sprite="Assets/Sprites/ui/btn_red.png"
                  data-source-component="QuitGameButton"
                  type="button"
                  onClick={handleQuitClick}
                >
                  退出游戏
                </button>
              </nav>
            </section>
          </Match>

          <Match when={menuState() === "save-select"}>
            <section
              class="startup-frame startup-frame-save"
              data-source-sprite="Assets/Sprites/ui/dialog02.png"
              data-source-asset-key="startupSaveFrameDialog"
              data-startup-panel="save-select"
              aria-label="跨纪元契约 存档选择 frame"
              style={{ "background-image": `url(${dialogSaveFrame})` }}
            >
              <header class="startup-panel-header">
                <button
                  class="startup-panel-back"
                  type="button"
                  onClick={goMain}
                  aria-label="返回主菜单"
                  data-source-component="BackButton"
                >
                  <span class="startup-panel-back-icon" aria-hidden="true">
                    ‹
                  </span>
                </button>
                <h2
                  class="startup-panel-title"
                  data-source-component="SavePanelTitle"
                  data-source-sprite="Assets/Sprites/ui/save_title_bg.png"
                  style={{ "background-image": `url(${titleBg})` }}
                >
                  {SAVE_PANEL_TITLE}
                </h2>
              </header>

              <ul
                class="startup-save-list"
                data-source-component="SaveSlotList"
                aria-label="存档列表"
              >
                <For each={saveSlots()}>
                  {(slot) => (
                    <li
                      class="startup-save-slot"
                      data-source-component="SaveSlot"
                      data-slot-placeholder={slot.isPlaceholder}
                    >
                      <div
                        class="startup-save-slot-row"
                        data-source-component="SaveSlotRow"
                      >
                        <button
                          type="button"
                          class="startup-save-slot-select"
                          onClick={() => handleSaveSlotSelect(slot)}
                          data-source-component="SaveSlotSelect"
                          aria-label={
                            slot.isPlaceholder
                              ? PLACEHOLDER_SLOT_HINT
                              : `${slot.label} ${slot.rankSummary}`
                          }
                        >
                          <span
                            class="startup-save-slot-checkbox"
                            aria-hidden="true"
                          />
                          <span class="startup-save-slot-body">
                            <span class="startup-save-slot-title">
                              {slot.isPlaceholder
                                ? PLACEHOLDER_SLOT_HINT
                                : slot.label}
                            </span>
                            {!slot.isPlaceholder && (
                              <span class="startup-save-slot-rank">
                                <span class="startup-save-slot-rank-label">
                                  Rank
                                </span>
                                <span class="startup-save-slot-rank-value">
                                  {slot.rankSummary}
                                </span>
                              </span>
                            )}
                          </span>
                        </button>
                        <button
                          type="button"
                          class="startup-save-slot-action"
                          data-source-component={
                            slot.isPlaceholder
                              ? "AddSaveButton"
                              : "DeleteSaveButton"
                          }
                          aria-label={
                            slot.isPlaceholder ? "新建存档" : `删除 ${slot.label}`
                          }
                          onClick={(event) =>
                            handleSaveSlotDelete(slot, event)
                          }
                        >
                          <span aria-hidden="true">
                            {slot.isPlaceholder ? "+" : "🗑"}
                          </span>
                        </button>
                      </div>
                    </li>
                  )}
                </For>
              </ul>
            </section>
          </Match>

          <Match when={menuState() === "settings"}>
            <section
              class="startup-frame startup-frame-settings"
              data-source-sprite="Assets/Sprites/ui/dialog07.png"
              data-source-asset-key="startupParchmentDialog"
              data-startup-panel="settings"
              aria-label="跨纪元契约 设置 frame"
              style={{ "background-image": `url(${dialogFrame})` }}
            >
              <header class="startup-panel-header">
                <h2
                  class="startup-panel-title startup-panel-title-centered"
                  data-source-component="SettingsPanelTitle"
                >
                  {SETTINGS_PANEL_TITLE}
                </h2>
                <button
                  class="startup-panel-close"
                  type="button"
                  onClick={goMain}
                  aria-label="关闭设置"
                  data-source-component="CloseButton"
                >
                  <span aria-hidden="true">✕</span>
                </button>
              </header>

              <ul
                class="startup-settings-list"
                data-source-component="SettingsRows"
                aria-label="设置选项"
              >
                <For each={SETTINGS_ROWS}>
                  {(row) => (
                    <li
                      class="startup-settings-row"
                      data-source-component="SettingsRow"
                      data-settings-key={row.id}
                    >
                      <label
                        class="startup-settings-label"
                        for={`startup-settings-${row.id}`}
                      >
                        {row.label}
                      </label>
                      {renderSettingsControl(row, settingsValues, handleSettingChange)}
                    </li>
                  )}
                </For>
              </ul>
            </section>
          </Match>

          <Match when={menuState() === "transition"}>
            <section
              class="startup-frame startup-frame-transition"
              data-startup-panel="transition"
              data-source-component="RegionTransition"
              data-source-region-id="zhuque"
              aria-label="朱雀大陆 转场"
              tabindex="0"
              onClick={handleTransitionContinue}
              onKeyDown={handleTransitionKey}
              role="button"
            >
              <h2
                class="startup-transition-title"
                data-source-component="RegionTitle"
              >
                {TRANSITION_REGION_TITLE}
              </h2>
              <p
                class="startup-transition-flavor"
                data-source-component="RegionFlavor"
              >
                {TRANSITION_REGION_FLAVOR}
              </p>
              <p
                class="startup-transition-hint"
                data-source-component="RegionContinueHint"
              >
                {TRANSITION_CONTINUE_HINT}
              </p>
            </section>
          </Match>
        </Switch>

        <footer class="startup-footer" data-source-component="StartupFooter">
          <span class="startup-version" data-source-component="VersionLabel">
            {STARTUP_VERSION_LABEL}
          </span>
        </footer>
      </div>

      <div
        class="startup-dev-dock"
        data-source-component="DevBootDock"
        aria-label="developer boot dock"
      >
        <button
          class="startup-dev-button"
          type="button"
          onClick={() => props.onReplayBoot()}
        >
          Boot Replay
        </button>
        <button
          class="startup-dev-button"
          type="button"
          onClick={() => props.onLiveBoot()}
        >
          Boot Live
        </button>
      </div>
    </main>
  );
};

function renderSettingsControl(
  row: SettingsRowFixture,
  values: () => Record<string, number | string>,
  onChange: (id: string, value: number | string) => void
) {
  if (row.kind === "slider") {
    return (
      <div class="startup-settings-slider">
        <button
          type="button"
          class="startup-settings-slider-step startup-settings-slider-step-down"
          aria-label={`${row.label} 减小`}
          onClick={() => {
            const cur = Number(values()[row.id] ?? 0);
            onChange(row.id, Math.max(0, cur - 5));
          }}
        >
          ◂
        </button>
        <input
          id={`startup-settings-${row.id}`}
          class="startup-settings-slider-input"
          type="range"
          min="0"
          max="100"
          step="1"
          value={String(values()[row.id] ?? 0)}
          onInput={(event) =>
            onChange(row.id, Number(event.currentTarget.value))
          }
          aria-label={row.label}
        />
        <button
          type="button"
          class="startup-settings-slider-step startup-settings-slider-step-up"
          aria-label={`${row.label} 增大`}
          onClick={() => {
            const cur = Number(values()[row.id] ?? 0);
            onChange(row.id, Math.min(100, cur + 5));
          }}
        >
          ▸
        </button>
      </div>
    );
  }
  return (
    <select
      id={`startup-settings-${row.id}`}
      class="startup-settings-select"
      value={String(values()[row.id] ?? "")}
      onChange={(event) => onChange(row.id, event.currentTarget.value)}
      aria-label={row.label}
    >
      <For each={row.options ?? []}>
        {(option) => <option value={option.id}>{option.label}</option>}
      </For>
    </select>
  );
}
