import { For, Show, createMemo, type Component } from "solid-js";

import type { DungeonItemsViewModel, DungeonItem } from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface DungeonItemsScreenProps {
  viewModel: DungeonItemsViewModel;
  onClose: () => void;
  onSelectItem: (itemId: string) => void;
  onSelectHero: (heroId: string) => void;
  onUseItem: (itemId: string, heroId?: string) => void;
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

function itemCategoryClass(category: DungeonItem["category"]): string {
  switch (category) {
    case "heal": return "dungeon-item-card--heal";
    case "stress": return "dungeon-item-card--stress";
    case "buff": return "dungeon-item-card--buff";
    case "tool": return "dungeon-item-card--tool";
    default: return "dungeon-item-card--misc";
  }
}

function itemCategoryLabel(category: DungeonItem["category"]): string {
  switch (category) {
    case "heal": return "治疗";
    case "stress": return "减压";
    case "buff": return "增益";
    case "tool": return "工具";
    default: return "其他";
  }
}

/**
 * Dungeon Items screen — 副本场景-物品
 *
 * Landscape game viewport for managing and using expedition supplies,
 * consumables, and loot while inside the dungeon.
 *
 * Layout mirrors the reference image:
 *   - Top HUD: dungeon name, room indicator, inventory count
 *   - Left panel: item grid with category filters and quantities
 *   - Right panel: selected item details + party target selection
 *   - Bottom controls: return to previous dungeon screen / continue
 */
export const DungeonItemsScreen: Component<DungeonItemsScreenProps> = (props) => {
  const selectedItem = createMemo(() =>
    props.viewModel.items.find((item) => item.id === props.viewModel.selectedItemId) ??
    props.viewModel.items[0]
  );

  const selectedHero = createMemo(() =>
    props.viewModel.party.find((hero) => hero.id === props.viewModel.selectedHeroId) ??
    props.viewModel.party[0]
  );

  const usableItems = createMemo(() => props.viewModel.items.filter((item) => item.isUsable && item.qty > 0));

  return (
    <div
      class="dungeon-items-viewport"
      data-source-scene="UI_Dungeon/DungeonItemsWindow"
      data-testid="dungeon-items-screen"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">Dungeon Inventory</span>
          <h1 class="expedition-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class="hud-pill hud-pill-accent">{props.viewModel.dungeonName}</span>
          <span class="hud-pill hud-pill--with-icon">
            <span class="hud-pill-icon" aria-hidden="true">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7">
                <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
              </svg>
            </span>
            {props.viewModel.roomLabel}
          </span>
          <span class="hud-pill">
            {props.viewModel.items.reduce((sum, item) => sum + item.qty, 0)} / {props.viewModel.maxItems} items
          </span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="dungeon-items-content">
          {/* Left: Item Grid */}
          <section class="dungeon-items-panel dungeon-items-panel--inventory" data-testid="items-inventory-panel">
            <header class="details-overlay-header">
              <span class="details-overlay-frame-rule" aria-hidden="true" />
              <h2 class="details-overlay-title">物品栏</h2>
              <span class="details-overlay-frame-rule" aria-hidden="true" />
            </header>

            <Show
              when={props.viewModel.items.length > 0}
              fallback={
                <div class="dungeon-items-empty">
                  <span class="dungeon-items-empty-icon">🎒</span>
                  <p>Inventory is empty.</p>
                </div>
              }
            >
              <div class="dungeon-items-grid">
                <For each={props.viewModel.items}>
                  {(item) => {
                    const isSelected = item.id === props.viewModel.selectedItemId;
                    const isUsable = item.isUsable && item.qty > 0;
                    return (
                      <button
                        class={`dungeon-item-card ${itemCategoryClass(item.category)}${isSelected ? " dungeon-item-card--selected" : ""}${!isUsable ? " dungeon-item-card--disabled" : ""}`}
                        onClick={() => isUsable && props.onSelectItem(item.id)}
                        disabled={!isUsable}
                        data-item-id={item.id}
                        data-testid={`dungeon-item-${item.id}`}
                      >
                        <span class="dungeon-item-card-icon">{item.icon}</span>
                        <span class="dungeon-item-card-qty">x{item.qty}</span>
                        <span class="dungeon-item-card-name">{item.name}</span>
                        <span class="dungeon-item-card-category">{itemCategoryLabel(item.category)}</span>
                      </button>
                    );
                  }}
                </For>
              </div>
            </Show>
          </section>

          {/* Right: Item Detail + Target Selection */}
          <section class="dungeon-items-panel dungeon-items-panel--detail" data-testid="items-detail-panel">
            <Show
              when={selectedItem()}
              fallback={
                <div class="dungeon-items-empty">
                  <span class="dungeon-items-empty-icon">🎒</span>
                  <p>Select an item to view details.</p>
                </div>
              }
            >
              {(item) => (
                <>
                  <header class="dungeon-item-detail-header">
                    <span class="dungeon-item-detail-icon">{item().icon}</span>
                    <div class="dungeon-item-detail-title-group">
                      <h2 class="dungeon-item-detail-name">{item().name}</h2>
                      <span class={`dungeon-item-detail-category ${itemCategoryClass(item().category)}`}>
                        {itemCategoryLabel(item().category)}
                      </span>
                    </div>
                    <span class="dungeon-item-detail-qty">x{item().qty}</span>
                  </header>

                  <p class="dungeon-item-detail-description">{item().description}</p>

                  <Show when={item().isUsable && item().qty > 0}>
                    <div class="dungeon-item-target-section">
                      <h3 class="dungeon-item-target-title">选择目标</h3>
                      <div class="dungeon-item-hero-list">
                        <For each={props.viewModel.party}>
                          {(hero) => {
                            const portraitUrl = resolveHeroPortrait({
                              heroId: hero.id,
                              classLabel: hero.classLabel
                            });
                            const isTarget = hero.id === props.viewModel.selectedHeroId;
                            return (
                              <button
                                class={`dungeon-item-hero-token${isTarget ? " dungeon-item-hero-token--selected" : ""}`}
                                onClick={() => props.onSelectHero(hero.id)}
                                data-hero-id={hero.id}
                                data-testid={`item-target-hero-${hero.id}`}
                              >
                                <div
                                  class={`dungeon-item-hero-portrait${portraitUrl ? " dungeon-item-hero-portrait--image" : " dungeon-item-hero-portrait--fallback"}`}
                                >
                                  {portraitUrl ? (
                                    <img
                                      class="dungeon-item-hero-portrait-image"
                                      src={portraitUrl}
                                      alt=""
                                      aria-hidden="true"
                                    />
                                  ) : (
                                    <span class="dungeon-item-hero-initial">{hero.classLabel[0]}</span>
                                  )}
                                </div>
                                <div class="dungeon-item-hero-info">
                                  <span class="dungeon-item-hero-name">{hero.name}</span>
                                  <span class="dungeon-item-hero-class">{hero.classLabel} · Lv.{hero.level}</span>
                                </div>
                                <div class="dungeon-item-hero-bars">
                                  <div class="dungeon-item-bar-row">
                                    <div class="dungeon-item-bar-track">
                                      <div
                                        class="dungeon-item-bar-fill"
                                        style={{
                                          width: `${healthPercent(hero.hp)}%`,
                                          background: healthBarColor(hero.hp)
                                        }}
                                      />
                                    </div>
                                  </div>
                                  <div class="dungeon-item-bar-row">
                                    <div class="dungeon-item-bar-track">
                                      <div
                                        class="dungeon-item-bar-fill"
                                        style={{
                                          width: `${stressPercent(hero.stress, hero.maxStress)}%`,
                                          background: stressBarColor(hero.stress)
                                        }}
                                      />
                                    </div>
                                  </div>
                                </div>
                              </button>
                            );
                          }}
                        </For>
                      </div>

                      <button
                        class="action-primary dungeon-item-use-btn"
                        onClick={() => props.onUseItem(item().id, selectedHero()?.id)}
                        disabled={!selectedHero()}
                        data-testid="use-item-btn"
                      >
                        使用 {item().name}
                      </button>
                    </div>
                  </Show>
                </>
              )}
            </Show>
          </section>
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left">
          <span class="expedition-status-pill expedition-status-pill--neutral">
            <span class="expedition-status-pill-dot" aria-hidden="true" />
            {usableItems().length} usable items
          </span>
        </div>
        <div class="expedition-controls-right">
          <button
            class="action-secondary"
            onClick={props.onClose}
            data-testid="close-items-btn"
          >
            Return
          </button>
          <button
            class="action-primary launch-primary"
            onClick={props.onClose}
            disabled={!props.viewModel.canContinue}
          >
            {props.viewModel.canContinue ? "Continue" : "Manage Items"}
          </button>
        </div>
      </footer>
    </div>
  );
};
