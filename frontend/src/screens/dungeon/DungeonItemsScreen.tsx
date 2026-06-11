import { For, Show, createMemo, type Component } from "solid-js";

import type { DungeonItemsViewModel, DungeonItem, DungeonItemsHero } from "../../bridge/contractTypes";
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

function itemCategoryLabel(category: DungeonItem["category"]): string {
  switch (category) {
    case "consumable": return "消耗品";
    case "tool": return "工具";
    case "torch": return "火把";
    case "curio": return "奇物";
    case "key": return "钥匙";
    default: return category;
  }
}

/**
 * Dungeon Items screen — 副本场景-物品
 *
 * Landscape game viewport for using expedition supplies and curios.
 * Layout mirrors the reference image:
 *   - Top HUD: dungeon name, room label, torch level
 *   - Left panel: scrollable item grid with quantities and categories
 *   - Right panel: selected item details and party target selection
 *   - Bottom: usage hint, close button, and confirm use action
 */
export const DungeonItemsScreen: Component<DungeonItemsScreenProps> = (props) => {
  const selectedItem = createMemo(() =>
    props.viewModel.items.find((item) => item.id === props.viewModel.selectedItemId) ?? null
  );

  const selectedHero = createMemo(() =>
    props.viewModel.party.find((hero) => hero.id === props.viewModel.selectedHeroId) ?? null
  );

  const usableItem = createMemo(() => {
    const item = selectedItem();
    if (!item || !item.isUsable || item.qty <= 0) return null;
    if (item.targetHeroId) {
      return selectedHero() ? item : null;
    }
    return item;
  });

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
          <span class="hud-pill">{props.viewModel.roomLabel}</span>
          <span class="hud-pill hud-pill--with-icon">
            <span class="hud-pill-icon" aria-hidden="true">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7">
                <path d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z" />
              </svg>
            </span>
            Torch: {props.viewModel.torchLevel}/{props.viewModel.maxTorchLevel}
          </span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="dungeon-items-surface">
        <div class="dungeon-items-bg" />
        <div class="dungeon-items-mist" />

        <div class="dungeon-items-layout">
          {/* ── Item Grid Panel ───────────────────────────── */}
          <section class="dungeon-items-grid-panel" data-testid="dungeon-items-grid-panel">
            <header class="dungeon-items-grid-header">
              <span class="dungeon-items-grid-title">物品背包</span>
              <span class="dungeon-items-grid-count">
                {props.viewModel.items.reduce((sum, item) => sum + item.qty, 0)} items
              </span>
            </header>

            <div class="dungeon-items-grid">
              <For each={props.viewModel.items}>
                {(item) => {
                  const isSelected = item.id === props.viewModel.selectedItemId;
                  const isEmpty = item.qty <= 0;
                  return (
                    <button
                      class={`dungeon-item-card${isSelected ? " dungeon-item-card--selected" : ""}${isEmpty ? " dungeon-item-card--empty" : ""}`}
                      onClick={() => !isEmpty && props.onSelectItem(item.id)}
                      disabled={isEmpty}
                      data-testid={`dungeon-item-${item.id}`}
                      data-item-category={item.category}
                    >
                      <span class="dungeon-item-icon" aria-hidden="true">{item.icon}</span>
                      <span class="dungeon-item-name">{item.name}</span>
                      <span class="dungeon-item-category">{itemCategoryLabel(item.category)}</span>
                      <span class="dungeon-item-qty">× {item.qty}</span>
                    </button>
                  );
                }}
              </For>
            </div>
          </section>

          {/* ── Detail / Target Panel ─────────────────────── */}
          <section class="dungeon-items-detail-panel" data-testid="dungeon-items-detail-panel">
            <Show
              when={selectedItem()}
              fallback={
                <div class="dungeon-items-empty-state">
                  <div class="dungeon-items-empty-icon">🎒</div>
                  <p class="dungeon-items-empty-text">Select an item to view details and use it.</p>
                </div>
              }
            >
              {(item) => (
                <>
                  <header class="dungeon-items-detail-header">
                    <span class="dungeon-items-detail-icon" aria-hidden="true">{item().icon}</span>
                    <div class="dungeon-items-detail-titles">
                      <h2 class="dungeon-items-detail-name">{item().name}</h2>
                      <span class="dungeon-items-detail-category">{itemCategoryLabel(item().category)} · × {item().qty}</span>
                    </div>
                  </header>

                  <p class="dungeon-items-detail-description">{item().description}</p>

                  <Show when={item().targetHeroId}>
                    <div class="dungeon-items-target-section">
                      <h3 class="dungeon-items-target-title">选择目标</h3>
                      <div class="dungeon-items-target-grid">
                        <For each={props.viewModel.party}>
                          {(hero) => {
                            const portraitUrl = resolveHeroPortrait({
                              heroId: hero.id,
                              classLabel: hero.classLabel
                            });
                            const isTargeted = hero.id === props.viewModel.selectedHeroId;
                            return (
                              <button
                                class={`dungeon-items-target-hero${isTargeted ? " dungeon-items-target-hero--selected" : ""}${!hero.isAlive ? " dungeon-items-target-hero--dead" : ""}`}
                                onClick={() => hero.isAlive && props.onSelectHero(hero.id)}
                                disabled={!hero.isAlive}
                                data-testid={`dungeon-item-target-${hero.id}`}
                              >
                                <div
                                  class={`dungeon-items-target-portrait${portraitUrl ? " dungeon-items-target-portrait--image" : " dungeon-items-target-portrait--fallback"}`}
                                >
                                  {portraitUrl ? (
                                    <img
                                      class="dungeon-items-target-portrait-image"
                                      src={portraitUrl}
                                      alt=""
                                      aria-hidden="true"
                                    />
                                  ) : (
                                    <span class="dungeon-items-target-initial">{hero.classLabel[0]}</span>
                                  )}
                                </div>
                                <div class="dungeon-items-target-info">
                                  <span class="dungeon-items-target-name">{hero.name}</span>
                                  <span class="dungeon-items-target-class">{hero.classLabel}</span>
                                  <div class="dungeon-items-target-bars">
                                    <div class="dungeon-items-target-bar-track">
                                      <div
                                        class="dungeon-items-target-bar-fill"
                                        style={{
                                          width: `${healthPercent(hero.hp)}%`,
                                          background: healthBarColor(hero.hp)
                                        }}
                                      />
                                    </div>
                                    <div class="dungeon-items-target-bar-track">
                                      <div
                                        class="dungeon-items-target-bar-fill"
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
                    </div>
                  </Show>

                  <div class="dungeon-items-usage-hint" data-testid="dungeon-items-usage-hint">
                    {props.viewModel.usageHint}
                  </div>

                  <div class="dungeon-items-detail-actions">
                    <button
                      class="action-primary launch-primary"
                      onClick={() => {
                        const target = usableItem()?.targetHeroId ? props.viewModel.selectedHeroId : undefined;
                        if (usableItem()) {
                          props.onUseItem(usableItem()!.id, target ?? undefined);
                        }
                      }}
                      disabled={!usableItem()}
                      data-testid="dungeon-items-use-btn"
                    >
                      使用物品
                    </button>
                  </div>
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
            {props.viewModel.items.filter((i) => i.qty > 0).length} item types available
          </span>
        </div>
        <div class="expedition-controls-right">
          <button
            class="action-secondary"
            onClick={props.onClose}
            data-testid="dungeon-items-close-btn"
          >
            关闭背包
          </button>
        </div>
      </footer>
    </div>
  );
};
