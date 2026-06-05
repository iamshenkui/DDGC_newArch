import { For, type Component } from "solid-js";

import type { DungeonItemsViewModel } from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface DungeonItemsScreenProps {
  viewModel: DungeonItemsViewModel;
  onContinue: () => void;
  onReturnToTown: () => void;
}

function rarityClass(rarity: string): string {
  switch (rarity) {
    case "common":
      return "item-card--common";
    case "uncommon":
      return "item-card--uncommon";
    case "rare":
      return "item-card--rare";
    case "epic":
      return "item-card--epic";
    default:
      return "item-card--common";
  }
}

function rarityLabel(rarity: string): string {
  switch (rarity) {
    case "common":
      return "普通";
    case "uncommon":
      return "优秀";
    case "rare":
      return "稀有";
    case "epic":
      return "史诗";
    default:
      return rarity;
  }
}

function parseHp(hp: string): { current: number; max: number } {
  const parts = hp.split("/");
  if (parts.length === 2) {
    return { current: Number(parts[0].trim()), max: Number(parts[1].trim()) };
  }
  return { current: 0, max: 1 };
}

function healthPercent(hp: string): number {
  const { current, max } = parseHp(hp);
  if (max <= 0) return 0;
  return Math.round((current / max) * 100);
}

function healthBarColor(hp: string): string {
  const pct = healthPercent(hp);
  if (pct >= 80) return "#5bbd6e";
  if (pct >= 40) return "#e8a838";
  return "#ea7767";
}

function stressPercent(stress: string, maxStress: string): number {
  const s = Number(stress);
  const m = Number(maxStress || 200);
  return Math.min(Math.round((s / m) * 100), 100);
}

function stressBarColor(stress: string): string {
  const s = Number(stress);
  if (s <= 20) return "#5bbd6e";
  if (s <= 40) return "#e8a838";
  return "#ea7767";
}

/**
 * Dungeon items screen — landscape game viewport for in-dungeon inventory.
 *
 * Displays items found/collected during dungeon exploration with party status.
 * References original Unity prefab structure from:
 *   Assets/Prefabs/UI/DungeonInventoryWindow.prefab (estimated)
 *
 * Source hierarchy: UI_Dungeon/DungeonInventoryWindow
 *   TopHUD → SceneLabel + ExpeditionName + InventoryCapacity
 *   ItemGridPanel → ItemCard × N
 *   PartyStatusPanel → PartyStatusCard × 4
 *   BottomControls → ContinueButton + ReturnButton
 */
export const DungeonItemsScreen: Component<DungeonItemsScreenProps> = (props) => {
  const inventoryPercent = () =>
    Math.min(
      Math.round((props.viewModel.inventoryUsed / props.viewModel.inventoryCapacity) * 100),
      100
    );

  const inventoryStatusClass = () => {
    const pct = inventoryPercent();
    if (pct >= 90) return "inventory-pill--full";
    if (pct >= 70) return "inventory-pill--heavy";
    return "inventory-pill--normal";
  };

  return (
    <div
      class="expedition-viewport"
      data-source-scene="UI_Dungeon/DungeonInventoryWindow"
      data-source-prefab="Assets/Prefabs/UI/DungeonInventoryWindow.prefab"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">{props.viewModel.sceneLabel}</span>
          <h1 class="expedition-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class="hud-pill hud-pill-accent">{props.viewModel.expeditionName}</span>
          <span class={`hud-pill inventory-pill ${inventoryStatusClass()}`}>
            <span class="inventory-pill-icon" aria-hidden="true">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7">
                <path d="M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z" />
                <line x1="3" y1="6" x2="21" y2="6" />
                <path d="M16 10a4 4 0 0 1-8 0" />
              </svg>
            </span>
            Inventory: {props.viewModel.inventoryUsed}/{props.viewModel.inventoryCapacity}
          </span>
          <span class="hud-pill">
            Items: {props.viewModel.items.length}
          </span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="expedition-content dungeon-items-content">
          {/* Item grid + party status side by side */}
          <div class="dungeon-items-layout">
            {/* Left: Item grid */}
            <section class="item-grid-panel">
              <header class="item-grid-header">
                <span class="item-grid-header-rule" aria-hidden="true" />
                <h2 class="item-grid-title">Found Items</h2>
                <span class="item-grid-header-rule" aria-hidden="true" />
              </header>

              {props.viewModel.items.length === 0 ? (
                <div class="item-grid-empty">
                  <span class="item-grid-empty-icon" aria-hidden="true">
                    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <path d="M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z" />
                      <line x1="3" y1="6" x2="21" y2="6" />
                      <path d="M16 10a4 4 0 0 1-8 0" />
                    </svg>
                  </span>
                  <span class="item-grid-empty-text">No items found yet</span>
                  <span class="item-grid-empty-hint">
                    Explore the dungeon to discover treasures and supplies.
                  </span>
                </div>
              ) : (
                <div class="item-grid">
                  <For each={props.viewModel.items}>
                    {(item) => (
                      <div
                        class={`item-card ${rarityClass(item.rarity)}`}
                        data-item-id={item.id}
                        data-item-rarity={item.rarity}
                      >
                        <div class="item-card-header">
                          <span class="item-card-name">{item.name}</span>
                          <span class="item-card-quantity">×{item.quantity}</span>
                        </div>
                        <div class="item-card-body">
                          <span class="item-card-description">{item.description}</span>
                        </div>
                        <div class="item-card-footer">
                          <span class={`item-card-rarity item-card-rarity--${item.rarity}`}>
                            {rarityLabel(item.rarity)}
                          </span>
                        </div>
                      </div>
                    )}
                  </For>
                </div>
              )}
            </section>

            {/* Right: Party status */}
            <aside class="party-status-panel">
              <header class="party-status-header">
                <span class="party-status-header-rule" aria-hidden="true" />
                <h2 class="party-status-title">Party Status</h2>
                <span class="party-status-header-rule" aria-hidden="true" />
              </header>

              <div class="party-status-list">
                <For each={props.viewModel.party}>
                  {(hero) => {
                    const portraitUrl = resolveHeroPortrait({
                      heroId: hero.heroId,
                      classLabel: hero.classLabel
                    });
                    return (
                      <div class="party-status-card" data-hero-id={hero.heroId}>
                        <div class="party-status-portrait">
                          {portraitUrl ? (
                            <img
                              class="party-status-portrait-image"
                              src={portraitUrl}
                              alt=""
                              aria-hidden="true"
                            />
                          ) : (
                            <span class="party-status-portrait-letter">
                              {hero.heroName[0]}
                            </span>
                          )}
                        </div>
                        <div class="party-status-info">
                          <span class="party-status-name">{hero.heroName}</span>
                          <span class="party-status-class">{hero.classLabel}</span>
                        </div>
                        <div class="party-status-bars">
                          <div class="party-status-bar-row">
                            <span class="party-status-bar-label">HP</span>
                            <span class="party-status-bar-track">
                              <span
                                class="party-status-bar-fill"
                                style={{
                                  width: `${healthPercent(hero.hp)}%`,
                                  background: healthBarColor(hero.hp),
                                }}
                              />
                            </span>
                          </div>
                          <div class="party-status-bar-row">
                            <span class="party-status-bar-label">ST</span>
                            <span class="party-status-bar-track">
                              <span
                                class="party-status-bar-fill"
                                style={{
                                  width: `${stressPercent(hero.stress, hero.maxStress)}%`,
                                  background: stressBarColor(hero.stress),
                                }}
                              />
                            </span>
                          </div>
                        </div>
                      </div>
                    );
                  }}
                </For>
              </div>
            </aside>
          </div>
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left">
          <span class={`expedition-status-pill ${inventoryStatusClass()}`}>
            <span class="expedition-status-pill-dot" aria-hidden="true" />
            Inventory {inventoryPercent()}% full
          </span>
        </div>
        <div class="expedition-controls-right">
          <button class="action-secondary" onClick={props.onReturnToTown}>
            Return to Town
          </button>
          <button
            class="action-primary launch-primary"
            onClick={props.onContinue}
            disabled={!props.viewModel.isContinueAvailable}
          >
            {props.viewModel.isContinueAvailable
              ? "Continue Expedition"
              : "Inventory Full"}
          </button>
        </div>
      </footer>
    </div>
  );
};
