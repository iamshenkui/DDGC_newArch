import { For, Show, createMemo, createSignal, type Component } from "solid-js";

import type { DungeonItemsViewModel, DungeonItemsItem, DungeonItemsHero } from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface DungeonItemsScreenProps {
  viewModel: DungeonItemsViewModel;
  onSelectItem: (itemId: string) => void;
  onSelectHero: (heroId: string) => void;
  onUseItem: (itemId: string, heroId?: string) => void;
  onClose: () => void;
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

function itemCategoryLabel(category: DungeonItemsItem["category"]): string {
  switch (category) {
    case "consumable": return "消耗品";
    case "tool": return "工具";
    case "treasure": return "宝藏";
    case "key": return "钥匙";
    default: return category;
  }
}

/**
 * Dungeon Items screen — 副本场景-物品.
 *
 * In-dungeon inventory viewport showing party-carried supplies, loot, and usable items.
 * Layout mirrors the reference image:
 *   - Top HUD: dungeon name, room indicator, item count
 *   - Left panel: party member list with vitals
 *   - Center panel: item grid with category grouping
 *   - Right panel: selected item detail + use/return actions
 */
export const DungeonItemsScreen: Component<DungeonItemsScreenProps> = (props) => {
  const [hoveredItemId, setHoveredItemId] = createSignal<string | null>(null);

  const selectedItem = createMemo(() =>
    props.viewModel.items.find((item) => item.id === props.viewModel.selectedItemId) ?? null
  );

  const selectedHero = createMemo(() =>
    props.viewModel.party.find((hero) => hero.id === props.viewModel.selectedHeroId) ?? null
  );

  const displayItem = createMemo(() => selectedItem() ?? hoveredItem() ?? props.viewModel.items[0] ?? null);

  const hoveredItem = createMemo(() =>
    props.viewModel.items.find((item) => item.id === hoveredItemId()) ?? null
  );

  const usableTargets = createMemo(() => {
    const item = displayItem();
    if (!item || !item.isUsable) return [];
    return props.viewModel.party;
  });

  const canUseSelected = createMemo(() => {
    const item = selectedItem();
    if (!item || !item.isUsable || item.qty <= 0) return false;
    if (usableTargets().length === 0) return true;
    return selectedHero() !== null;
  });

  return (
    <div
      class="expedition-viewport dungeon-items-viewport"
      data-source-scene="UI_Dungeon/DungeonItemsWindow"
      data-source-prefab="Assets/Prefabs/UI/DungeonItemsWindow.prefab"
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
                <circle cx="12" cy="12" r="10" />
                <path d="M12 6v6l4 2" />
              </svg>
            </span>
            Room {props.viewModel.roomNumber}
          </span>
          <span class="hud-pill">
            Items: {props.viewModel.items.reduce((sum, item) => sum + item.qty, 0)}
          </span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="expedition-content dungeon-items-content">
          {/* Left: Party panel */}
          <section class="dungeon-items-party-panel" data-testid="dungeon-items-party-panel">
            <header class="dungeon-items-panel-header">
              <span class="dungeon-items-panel-title">队伍状态</span>
              <span class="dungeon-items-panel-subtitle">{props.viewModel.party.length} heroes</span>
            </header>
            <div class="dungeon-items-party-list">
              <For each={props.viewModel.party}>
                {(hero) => {
                  const portraitUrl = resolveHeroPortrait({ heroId: hero.id, classLabel: hero.classLabel });
                  const isSelected = hero.id === props.viewModel.selectedHeroId;
                  return (
                    <button
                      class={`dungeon-items-hero-row${isSelected ? " dungeon-items-hero-row--selected" : ""}`}
                      onClick={() => props.onSelectHero(hero.id)}
                      data-hero-id={hero.id}
                      data-testid={`dungeon-items-hero-${hero.id}`}
                    >
                      <div
                        class={`dungeon-items-hero-portrait${portraitUrl ? " dungeon-items-hero-portrait--image" : " dungeon-items-hero-portrait--fallback"}`}
                      >
                        {portraitUrl ? (
                          <img
                            class="dungeon-items-hero-portrait-image"
                            src={portraitUrl}
                            alt=""
                            aria-hidden="true"
                          />
                        ) : (
                          <span class="dungeon-items-hero-portrait-letter">{hero.classLabel[0]}</span>
                        )}
                      </div>
                      <div class="dungeon-items-hero-info">
                        <div class="dungeon-items-hero-name">{hero.name}</div>
                        <div class="dungeon-items-hero-class">{hero.classLabel}</div>
                        <div class="dungeon-items-hero-bars">
                          <div class="dungeon-items-bar-row">
                            <span class="dungeon-items-bar-track">
                              <span
                                class="dungeon-items-bar-fill"
                                style={{
                                  width: `${healthPercent(hero.hp)}%`,
                                  background: healthBarColor(hero.hp)
                                }}
                              />
                            </span>
                            <span class="dungeon-items-bar-value">{hero.hp}</span>
                          </div>
                          <div class="dungeon-items-bar-row">
                            <span class="dungeon-items-bar-track dungeon-items-bar-track--stress">
                              <span
                                class="dungeon-items-bar-fill"
                                style={{
                                  width: `${stressPercent(hero.stress, hero.maxStress)}%`,
                                  background: stressBarColor(hero.stress)
                                }}
                              />
                            </span>
                            <span class="dungeon-items-bar-value">{hero.stress}/{hero.maxStress}</span>
                          </div>
                        </div>
                      </div>
                    </button>
                  );
                }}
              </For>
            </div>
          </section>

          {/* Center: Item grid */}
          <section class="dungeon-items-grid-panel" data-testid="dungeon-items-grid-panel">
            <header class="dungeon-items-panel-header">
              <span class="dungeon-items-panel-title">物品</span>
              <span class="dungeon-items-panel-subtitle">{props.viewModel.items.length} types</span>
            </header>
            <Show
              when={props.viewModel.items.length > 0}
              fallback={
                <div class="dungeon-items-empty" data-testid="dungeon-items-empty">
                  <span class="dungeon-items-empty-icon" aria-hidden="true">🎒</span>
                  <span class="dungeon-items-empty-text">背包空空如也</span>
                  <span class="dungeon-items-empty-sub">在探索中收集补给与宝藏</span>
                </div>
              }
            >
              <div class="dungeon-items-grid">
                <For each={props.viewModel.items}>
                  {(item) => {
                    const isSelected = item.id === props.viewModel.selectedItemId;
                    return (
                      <button
                        class={`dungeon-items-cell${isSelected ? " dungeon-items-cell--selected" : ""}${!item.isUsable ? " dungeon-items-cell--readonly" : ""}`}
                        onClick={() => props.onSelectItem(item.id)}
                        onMouseEnter={() => setHoveredItemId(item.id)}
                        onMouseLeave={() => setHoveredItemId(null)}
                        data-item-id={item.id}
                        data-testid={`dungeon-item-${item.id}`}
                      >
                        <span class="dungeon-items-cell-icon" aria-hidden="true">{item.icon}</span>
                        <span class="dungeon-items-cell-name">{item.name}</span>
                        <span class="dungeon-items-cell-qty">×{item.qty}</span>
                        <span class="dungeon-items-cell-category">{itemCategoryLabel(item.category)}</span>
                      </button>
                    );
                  }}
                </For>
              </div>
            </Show>
          </section>

          {/* Right: Item detail */}
          <section class="dungeon-items-detail-panel" data-testid="dungeon-items-detail-panel">
            <Show
              when={displayItem()}
              fallback={
                <div class="dungeon-items-detail-empty">
                  <span class="dungeon-items-detail-empty-icon" aria-hidden="true">🎒</span>
                  <span class="dungeon-items-detail-empty-text">选择物品查看详情</span>
                </div>
              }
            >
              {(item) => {
                const targets = usableTargets();
                const hasTargets = targets.length > 0;
                return (
                  <div class="dungeon-items-detail">
                    <div class="dungeon-items-detail-icon" aria-hidden="true">{item().icon}</div>
                    <h2 class="dungeon-items-detail-name">{item().name}</h2>
                    <span class="dungeon-items-detail-category">{itemCategoryLabel(item().category)}</span>
                    <p class="dungeon-items-detail-desc">{item().description}</p>
                    <div class="dungeon-items-detail-meta">
                      <span class="dungeon-items-detail-meta-item">持有: ×{item().qty}</span>
                      <span class="dungeon-items-detail-meta-item">
                        {item().isUsable ? "可使用" : "不可使用"}
                      </span>
                    </div>

                    <Show when={item().isUsable && hasTargets}>
                      <div class="dungeon-items-target-list">
                        <span class="dungeon-items-target-label">选择目标</span>
                        <For each={targets}>
                          {(hero) => {
                            const isSelected = hero.id === props.viewModel.selectedHeroId;
                            return (
                              <button
                                class={`dungeon-items-target-btn${isSelected ? " dungeon-items-target-btn--selected" : ""}`}
                                onClick={() => props.onSelectHero(hero.id)}
                                data-target-hero-id={hero.id}
                                data-testid={`dungeon-items-target-${hero.id}`}
                              >
                                <span class="dungeon-items-target-name">{hero.name}</span>
                                <span class="dungeon-items-target-hp">{hero.hp}</span>
                              </button>
                            );
                          }}
                        </For>
                      </div>
                    </Show>

                    <div class="dungeon-items-detail-actions">
                      <button
                        class="action-secondary"
                        onClick={props.onClose}
                        data-testid="dungeon-items-close-btn"
                      >
                        返回
                      </button>
                      <button
                        class="action-primary"
                        onClick={() => props.onUseItem(item().id, selectedHero()?.id)}
                        disabled={!canUseSelected()}
                        data-testid="dungeon-items-use-btn"
                      >
                        {item().isUsable ? "使用" : "查看"}
                      </button>
                    </div>
                  </div>
                );
              }}
            </Show>
          </section>
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left">
          <span class="expedition-status-pill expedition-status-pill--neutral">
            <span class="expedition-status-pill-dot" aria-hidden="true" />
            {props.viewModel.items.reduce((sum, item) => sum + item.qty, 0)} items carried
          </span>
        </div>
        <div class="expedition-controls-right">
          <button
            class="action-secondary"
            onClick={props.onClose}
            data-testid="dungeon-items-footer-close"
          >
            关闭背包
          </button>
        </div>
      </footer>
    </div>
  );
};

export type { DungeonItemsHero };
