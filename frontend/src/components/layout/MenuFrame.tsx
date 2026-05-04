import type { ParentComponent } from "solid-js";

interface MenuFrameProps {
  gameTitle: string;
  gameSubtitle: string;
}

export const MenuFrame: ParentComponent<MenuFrameProps> = (props) => {
  return (
    <main class="menu-frame">
      <div class="menu-backdrop" />
      <div class="menu-dialog">
        <header class="menu-header">
          <h1 class="menu-game-title">{props.gameTitle}</h1>
          <p class="menu-game-subtitle">{props.gameSubtitle}</p>
        </header>
        <nav class="menu-options">
          {props.children}
        </nav>
      </div>
    </main>
  );
};
