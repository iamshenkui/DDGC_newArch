import type { Component } from "solid-js";

interface StartupScreenProps {
  onReplayBoot: () => void;
  onLiveBoot: () => void;
  onNewCampaign: () => void;
  onLoadCampaign: () => void;
  hasSavedCampaign: boolean;
}

/**
 * 开始界面 (Start Screen) — rewritten from reference image:
 * reference/ref_image/跨际元契约/1系统界面/开始界面.png
 *
 * Layout mirrors the reference:
 *   - Dark grungy stone background
 *   - Title "跨纪元契约" with sword-cross ornament
 *   - Parchment-style menu with four options
 *   - Version label at bottom
 *
 * Source-backed chrome:
 *   - menu_bg.png (GUID 4e780ea66a89c2f4b8bdc4f4b76c290d)
 *   - btn_menu_primary.png (GUID b14c9d4fa66bc4d45897fba8c866125a)
 *   - Deutsch.ttf (GUID f930c4496e27b454ebc744aa4b25236e)
 *
 * Original sprites not yet extracted (BLOCKER-004); CSS approximations used.
 */
export const StartupScreen: Component<StartupScreenProps> = (props) => {
  return (
    <main
      class="start-screen"
      data-source-ref="开始界面"
      data-source-group="1系统界面"
    >
      {/* Background layer — dark grungy stone texture */}
      <div
        class="start-screen-backdrop"
        data-source-sprite="Assets/Sprites/ui/menu_bg.png"
        data-source-guid="4e780ea66a89c2f4b8bdc4f4b76c290d"
      />

      <div class="start-screen-content">
        {/* Title with sword-cross ornament */}
        <header class="start-screen-header">
          <div class="start-screen-ornament" aria-hidden="true">
            <svg
              viewBox="0 0 48 64"
              width="48"
              height="64"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
            >
              {/* Sword-cross ornament matching reference */}
              <path
                d="M24 0v64M12 20h24M24 20l-8-12M24 20l8-12M18 20l-6 8M30 20l6 8"
                stroke="currentColor"
                stroke-width="2.5"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
          </div>
          <h1 class="start-screen-game-title">跨纪元契约</h1>
        </header>

        {/* Parchment-style menu */}
        <nav
          class="start-screen-menu"
          data-source-component="MenuOptions"
          data-source-font="Assets/Fonts/Bak/Deutsch.ttf"
          data-source-button-sprite="Assets/Sprites/ui/btn_menu_primary.png"
        >
          <button
            class="start-screen-menu-button"
            data-source-sprite="Assets/Sprites/ui/btn_menu_primary.png"
            data-testid="start-new-game"
            onClick={props.onNewCampaign}
          >
            新游戏
          </button>

          <button
            class="start-screen-menu-button"
            data-source-sprite="Assets/Sprites/ui/btn_menu_primary.png"
            data-testid="start-continue"
            onClick={props.onLoadCampaign}
            disabled={!props.hasSavedCampaign}
          >
            {props.hasSavedCampaign ? "继续游戏" : "读取存档"}
          </button>

          <button
            class="start-screen-menu-button"
            data-source-sprite="Assets/Sprites/ui/btn_menu_primary.png"
            data-testid="start-settings"
            onClick={() => {
              /* Settings not yet implemented — noop with visual feedback */
            }}
            disabled
          >
            设置
          </button>

          <button
            class="start-screen-menu-button"
            data-source-sprite="Assets/Sprites/ui/btn_menu_primary.png"
            data-testid="start-exit"
            onClick={() => {
              if (typeof window !== "undefined") {
                window.close();
              }
            }}
          >
            退出游戏
          </button>
        </nav>

        {/* Version label */}
        <footer class="start-screen-version">V 1.0</footer>
      </div>

      {/* Dev tools row — secondary, outside main reference layout */}
      <div class="start-screen-dev-bar">
        <button
          class="start-screen-dev-button"
          data-testid="boot-replay"
          onClick={props.onReplayBoot}
        >
          Boot Replay
        </button>
        <button
          class="start-screen-dev-button"
          data-testid="boot-live"
          onClick={props.onLiveBoot}
        >
          Boot Live
        </button>
      </div>
    </main>
  );
};
