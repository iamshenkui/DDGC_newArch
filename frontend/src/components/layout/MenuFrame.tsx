import type { ParentComponent } from "solid-js";

interface MenuFrameProps {
  gameTitle: string;
  gameSubtitle: string;
  /** Optional source prefab path for traceability (e.g. MainMenuWindow.prefab) */
  sourcePrefab?: string;
  /** Optional menu section label (e.g. "Main Menu") */
  menuLabel?: string;
}

/**
 * Reusable menu dialog frame that mirrors the original DDGC MainMenuWindow.prefab
 * hierarchy. Used for menus and dialogs that need the game's centered-panel layout.
 *
 * Source-backed chrome:
 *   - menu_bg.png (GUID 4e780ea66a89c2f4b8bdc4f4b76c290d)
 *   - btn_menu_primary.png (GUID b14c9d4fa66bc4d45897fba8c866125a)
 *   - Deutsch.ttf (GUID f930c4496e27b454ebc744aa4b25236e)
 */
export const MenuFrame: ParentComponent<MenuFrameProps> = (props) => {
  return (
    <main
      class="menu-frame"
      data-source-prefab={props.sourcePrefab ?? "Assets/Prefabs/UI/Windows/MainMenuWindow.prefab"}
    >
      <div
        class="menu-backdrop"
        data-source-sprite="Assets/Sprites/ui/menu_bg.png"
        data-source-guid="4e780ea66a89c2f4b8bdc4f4b76c290d"
      />
      <div class="menu-dialog">
        <header class="menu-header">
          <h1 class="menu-game-title">{props.gameTitle}</h1>
          <p class="menu-game-subtitle">{props.gameSubtitle}</p>
        </header>
        <nav class="menu-options" data-source-component="MenuOptions">
          {props.menuLabel && (
            <span class="menu-options-label">{props.menuLabel}</span>
          )}
          {props.children}
        </nav>
      </div>
    </main>
  );
};
