import type { Component } from "solid-js";

interface StartupScreenProps {
  onReplayBoot: () => void;
  onLiveBoot: () => void;
  onNewCampaign: () => void;
  onLoadCampaign: () => void;
  onFirstCombatDemo?: () => void;
  hasSavedCampaign: boolean;
}

/**
 * Title / main-menu page rewritten from the original DDGC MainMenuWindow.prefab
 * hierarchy (fixtures/ui_inventory/Assets/Prefabs/UI/Windows/MainMenuWindow.prefab.json).
 *
 * Layout mirrors the Unity prefab:
 *   MainMenuWindow (centered canvas) → background layer → MenuOptions → Buttons
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
      class="title-page"
      data-source-prefab="Assets/Prefabs/UI/Windows/MainMenuWindow.prefab"
    >
      {/* Background layer — mirrors MainMenuWindow root Image component (menu_bg.png) */}
      <div
        class="title-page-backdrop"
        data-source-sprite="Assets/Sprites/ui/menu_bg.png"
        data-source-guid="4e780ea66a89c2f4b8bdc4f4b76c290d"
      />

      <div
        class="title-page-dialog"
        data-source-component="MainMenuWindow"
      >
        {/* Game branding header */}
        <header class="title-page-header">
          <div class="title-page-ornament" aria-hidden="true" />
          <h1 class="title-page-game-title">DDGC</h1>
          <p class="title-page-game-subtitle">暗黑地牢: 降龙</p>
        </header>

        <div class="title-page-divider" aria-hidden="true" />

        {/* MenuOptions container — mirrors MainMenuWindow → MenuOptions LayoutGroup */}
        <nav
          class="title-page-menu"
          data-source-component="MenuOptions"
          data-source-font="Assets/Fonts/Bak/Deutsch.ttf"
          data-source-button-sprite="Assets/Sprites/ui/btn_menu_primary.png"
        >
          <span class="title-page-menu-label">Main Menu</span>

          <button
            class="title-page-button"
            data-source-sprite="Assets/Sprites/ui/btn_menu_primary.png"
            onClick={props.onNewCampaign}
          >
            New Campaign
          </button>

          <button
            class="title-page-button"
            data-source-sprite="Assets/Sprites/ui/btn_menu_primary.png"
            onClick={props.onLoadCampaign}
            disabled={!props.hasSavedCampaign}
          >
            Load Campaign
          </button>

          <button
            class="title-page-button"
            data-source-sprite="Assets/Sprites/ui/btn_menu_primary.png"
            onClick={props.onReplayBoot}
          >
            Boot Replay
          </button>

          <button
            class="title-page-button"
            data-source-sprite="Assets/Sprites/ui/btn_menu_primary.png"
            onClick={props.onFirstCombatDemo}
          >
            First Combat Demo
          </button>

          <button
            class="title-page-button"
            data-source-sprite="Assets/Sprites/ui/btn_menu_primary.png"
            onClick={props.onLiveBoot}
          >
            Boot Live
          </button>
        </nav>
      </div>
    </main>
  );
};
