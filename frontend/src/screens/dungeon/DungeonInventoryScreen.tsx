import { For, type Component } from "solid-js";

import type { DungeonInventoryViewModel } from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface DungeonInventoryScreenProps {
  viewModel: DungeonInventoryViewModel;
  onReturn: () => void;
  onUseItem: (itemId: string) => void;
}

function healthPercent(hp: string, maxHp: string): number {
  const c = Number(hp);
  const m = Number(maxHp);
  if (m <= 0) return 0;
  return Math.round((c / m) * 100);
}

function healthBarColor(pct: number): string {
  if (pct >= 80) return "#5bbd6e";
  if (pct >= 40) return "#e8a838";
  return "#ea7767";
}

function stressPercent(stress: string, maxStress: string): number {
  const s = Number(stress);
  const m = Number(maxStress || 200);
  return Math.min(Math.round((s / m) * 100), 100);
}

function itemCategoryLabel(category: string): string {
  switch (category) {
    case "consumable":
      return "消耗";
    case "equipment":
      return "装备";
    case "material":
      return "材料";
    case "quest":
      return "任务";
    default:
      return "其他";
  }
}

function itemCategoryClass(category: string): string {
  switch (category) {
    case "consumable":
      return "item-slot--consumable";
    case "equipment":
      return "item-slot--equipment";
    case "material":
      return "item-slot--material";
    case "quest":
      return "item-slot--quest";
    default:
      return "";
  }
}

/**
 * Dungeon inventory screen — scene banner + character equipment + item grid.
 *
 * Mirrors the original Unity dungeon scene inventory layout:
 *   Top banner with dungeon name and return button
 *   Bottom split: hero equipment panel (left) + inventory grid (right)
 *
 * Reference image: 副本场景-物品.png
 *   Source scene: UI_Dungeon/InventoryWindow
 *   Source prefabs:
 *     Assets/Prefabs/UI/DungeonSceneItemPanel.prefab
 *     Assets/Prefabs/UI/HeroEquipmentSlot.prefab
 *     Assets/Prefabs/UI/InventoryItemSlot.prefab
 */
export const DungeonInventoryScreen: Component<DungeonInventoryScreenProps> = (
  props
) => {
  const hpPct = () => healthPercent(props.viewModel.hero.hp, props.viewModel.hero.maxHp);
  const stPct = () => stressPercent(props.viewModel.hero.stress, props.viewModel.hero.maxStress);
  const portraitUrl = () =>
    resolveHeroPortrait({
      heroId: props.viewModel.hero.heroId,
      classLabel: props.viewModel.hero.classLabel
    });

  const emptySlots = () => {
    const slots = [];
    const filled = props.viewModel.inventory.length;
    for (let i = 0; i < props.viewModel.maxInventorySlots - filled; i++) {
      slots.push(i);
    }
    return slots;
  };

  return (
    <div
      class="dungeon-inventory-viewport"
      data-source-scene="UI_Dungeon/InventoryWindow"
      data-source-prefab="Assets/Prefabs/UI/DungeonSceneItemPanel.prefab"
    >
      {/* ── Top Scene Banner ────────────────────────────── */}
      <header class="dungeon-scene-banner">
        <div class="dungeon-scene-banner-bg" />
        <div class="dungeon-scene-banner-content">
          <span class="dungeon-scene-banner-left">
            <span class="dungeon-scene-name">{props.viewModel.dungeonName}</span>
          </span>
          <span class="dungeon-scene-banner-right">
            <button class="dungeon-return-btn" onClick={props.onReturn}>
              <svg
                width="14"
                height="14"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <line x1="19" y1="12" x2="5" y2="12" />
                <polyline points="12 19 5 12 12 5" />
              </svg>
              返回
            </button>
          </span>
        </div>
      </header>

      {/* ── Main Content: Equipment + Inventory ─────────── */}
      <div class="dungeon-inventory-surface">
        <div class="dungeon-inventory-surface-bg" />

        <div class="dungeon-inventory-content">
          {/* Left: Hero Equipment Panel */}
          <section
            class="dungeon-hero-panel"
            data-source-component="HeroEquipmentPanel"
          >
            <div class="dungeon-hero-panel-header">
              <span class="dungeon-hero-panel-title">英雄</span>
            </div>

            {/* Hero portrait and basic info */}
            <div class="dungeon-hero-info">
              <div
                class={`dungeon-hero-portrait${portraitUrl() ? " dungeon-hero-portrait--image" : " dungeon-hero-portrait--fallback"}`}
              >
                {portraitUrl() ? (
                  <img
                    class="dungeon-hero-portrait-image"
                    src={portraitUrl()}
                    alt={props.viewModel.hero.heroName}
                  />
                ) : (
                  <span class="dungeon-hero-portrait-letter">
                    {props.viewModel.hero.heroName[0]}
                  </span>
                )}
              </div>
              <div class="dungeon-hero-meta">
                <span class="dungeon-hero-name">{props.viewModel.hero.heroName}</span>
                <span class="dungeon-hero-class">
                  Lv{props.viewModel.hero.level} · {props.viewModel.hero.classLabel}
                </span>
              </div>
            </div>

            {/* HP Bar */}
            <div class="dungeon-hero-bars">
              <div class="dungeon-bar-row">
                <span class="dungeon-bar-label">HP</span>
                <span class="dungeon-bar-track">
                  <span
                    class="dungeon-bar-fill"
                    style={{
                      width: `${hpPct()}%`,
                      background: healthBarColor(hpPct()),
                    }}
                  />
                </span>
                <span class="dungeon-bar-value">
                  {props.viewModel.hero.hp}/{props.viewModel.hero.maxHp}
                </span>
              </div>
              <div class="dungeon-bar-row">
                <span class="dungeon-bar-label">ST</span>
                <span class="dungeon-bar-track">
                  <span
                    class="dungeon-bar-fill"
                    style={{
                      width: `${stPct()}%`,
                      background: "#e8a838",
                    }}
                  />
                </span>
                <span class="dungeon-bar-value">
                  {props.viewModel.hero.stress}/{props.viewModel.hero.maxStress}
                </span>
              </div>
            </div>

            {/* Equipment Grid */}
            <div class="dungeon-equipment-grid">
              <div
                class={`dungeon-equipment-slot${props.viewModel.hero.equipment.weapon ? " dungeon-equipment-slot--filled" : ""}`}
                data-slot-type="weapon"
              >
                <span class="dungeon-equipment-slot-label">武器</span>
                {props.viewModel.hero.equipment.weapon ? (
                  <>
                    <span class="dungeon-equipment-slot-icon" aria-hidden="true">
                      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                        <path d="M14.5 17.5L3 6V3h3l11.5 11.5" />
                        <path d="M13 19l6-6" />
                        <path d="M16 16l4 4" />
                        <path d="M19 21l2-2" />
                      </svg>
                    </span>
                    <span class="dungeon-equipment-slot-name">
                      {props.viewModel.hero.equipment.weapon.name}
                    </span>
                    <span class="dungeon-equipment-slot-level">
                      +{props.viewModel.hero.equipment.weapon.level}
                    </span>
                  </>
                ) : (
                  <span class="dungeon-equipment-slot-empty">空</span>
                )}
              </div>

              <div
                class={`dungeon-equipment-slot${props.viewModel.hero.equipment.armor ? " dungeon-equipment-slot--filled" : ""}`}
                data-slot-type="armor"
              >
                <span class="dungeon-equipment-slot-label">护甲</span>
                {props.viewModel.hero.equipment.armor ? (
                  <>
                    <span class="dungeon-equipment-slot-icon" aria-hidden="true">
                      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                        <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
                      </svg>
                    </span>
                    <span class="dungeon-equipment-slot-name">
                      {props.viewModel.hero.equipment.armor.name}
                    </span>
                    <span class="dungeon-equipment-slot-level">
                      +{props.viewModel.hero.equipment.armor.level}
                    </span>
                  </>
                ) : (
                  <span class="dungeon-equipment-slot-empty">空</span>
                )}
              </div>

              <div
                class={`dungeon-equipment-slot${props.viewModel.hero.equipment.trinket1 ? " dungeon-equipment-slot--filled" : ""}`}
                data-slot-type="trinket1"
              >
                <span class="dungeon-equipment-slot-label">饰品Ⅰ</span>
                {props.viewModel.hero.equipment.trinket1 ? (
                  <>
                    <span class="dungeon-equipment-slot-icon" aria-hidden="true">
                      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                        <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />
                      </svg>
                    </span>
                    <span class="dungeon-equipment-slot-name">
                      {props.viewModel.hero.equipment.trinket1.name}
                    </span>
                    <span class="dungeon-equipment-slot-level">
                      +{props.viewModel.hero.equipment.trinket1.level}
                    </span>
                  </>
                ) : (
                  <span class="dungeon-equipment-slot-empty">空</span>
                )}
              </div>

              <div
                class={`dungeon-equipment-slot${props.viewModel.hero.equipment.trinket2 ? " dungeon-equipment-slot--filled" : ""}`}
                data-slot-type="trinket2"
              >
                <span class="dungeon-equipment-slot-label">饰品Ⅱ</span>
                {props.viewModel.hero.equipment.trinket2 ? (
                  <>
                    <span class="dungeon-equipment-slot-icon" aria-hidden="true">
                      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                        <circle cx="12" cy="12" r="10" />
                        <circle cx="12" cy="12" r="4" />
                      </svg>
                    </span>
                    <span class="dungeon-equipment-slot-name">
                      {props.viewModel.hero.equipment.trinket2.name}
                    </span>
                    <span class="dungeon-equipment-slot-level">
                      +{props.viewModel.hero.equipment.trinket2.level}
                    </span>
                  </>
                ) : (
                  <span class="dungeon-equipment-slot-empty">空</span>
                )}
              </div>
            </div>
          </section>

          {/* Right: Inventory Grid Panel */}
          <section
            class="dungeon-inventory-panel"
            data-source-component="InventoryItemPanel"
          >
            <div class="dungeon-inventory-panel-header">
              <span class="dungeon-inventory-panel-title">道具</span>
              <span class="dungeon-inventory-panel-count">
                {props.viewModel.inventory.length}/{props.viewModel.maxInventorySlots}
              </span>
            </div>

            <div class="dungeon-inventory-grid">
              <For each={props.viewModel.inventory}>
                {(item) => (
                  <button
                    class={`dungeon-item-slot ${itemCategoryClass(item.category)}${item.isUsable ? " dungeon-item-slot--usable" : ""}`}
                    onClick={() => item.isUsable && props.onUseItem(item.id)}
                    disabled={!item.isUsable}
                    title={item.description}
                    data-item-id={item.id}
                    data-item-category={item.category}
                  >
                    <span class="dungeon-item-slot-category">
                      {itemCategoryLabel(item.category)}
                    </span>
                    <span class="dungeon-item-slot-icon" aria-hidden="true">
                      {item.category === "consumable" && (
                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                          <path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" />
                        </svg>
                      )}
                      {item.category === "equipment" && (
                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                          <path d="M14.5 17.5L3 6V3h3l11.5 11.5" />
                          <path d="M13 19l6-6" />
                          <path d="M16 16l4 4" />
                        </svg>
                      )}
                      {item.category === "material" && (
                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                          <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />
                        </svg>
                      )}
                      {item.category === "quest" && (
                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                          <polyline points="14 2 14 8 20 8" />
                        </svg>
                      )}
                    </span>
                    <span class="dungeon-item-slot-name">{item.name}</span>
                    {item.quantity > 1 && (
                      <span class="dungeon-item-slot-quantity">x{item.quantity}</span>
                    )}
                  </button>
                )}
              </For>

              {/* Empty inventory slots */}
              <For each={emptySlots()}>
                {(idx) => (
                  <div
                    class="dungeon-item-slot dungeon-item-slot--empty"
                    data-empty-index={idx}
                  >
                    <span class="dungeon-item-slot-empty-marker">—</span>
                  </div>
                )}
              </For>
            </div>

            {/* Gold display */}
            <div class="dungeon-inventory-footer">
              <span class="dungeon-gold-display">
                <span class="gold-icon-fallback" aria-hidden="true" />
                <span class="dungeon-gold-amount">{props.viewModel.gold}</span>
              </span>
            </div>
          </section>
        </div>
      </div>
    </div>
  );
};
