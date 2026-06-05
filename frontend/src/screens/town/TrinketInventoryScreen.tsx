import { For, type Component } from "solid-js";

import type { InventoryViewModel, TrinketItem } from "../../bridge/contractTypes";

interface TrinketInventoryScreenProps {
  viewModel: InventoryViewModel;
  onReturn: () => void;
  onFilterChange?: (filter: string) => void;
}

function rarityClass(rarity?: string): string {
  if (!rarity) return "trinket-rarity-common";
  const r = rarity.toLowerCase();
  if (r.includes("legendary") || r.includes("传说")) return "trinket-rarity-legendary";
  if (r.includes("epic") || r.includes("史诗")) return "trinket-rarity-epic";
  if (r.includes("rare") || r.includes("稀有")) return "trinket-rarity-rare";
  if (r.includes("uncommon") || r.includes("优良")) return "trinket-rarity-uncommon";
  return "trinket-rarity-common";
}

function trinketCardSource(trinket: TrinketItem): string {
  return trinket.isEquipped
    ? "Assets/Prefabs/UI/PartyInventorySlot.prefab"
    : "Assets/Prefabs/UI/ShopSlot.prefab";
}

/**
 * 饰品仓库 (Trinket Inventory) screen.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Windows/TrinketInventoryWindow.prefab
 *
 * Related prefabs:
 *   Assets/Prefabs/UI/ShopSlot.prefab            — unequipped trinket slot
 *   Assets/Prefabs/UI/PartyInventorySlot.prefab  — equipped trinket slot
 *   Assets/Prefabs/UI/HeroSlot.prefab            — hero portrait + trinket pairing
 */
export const TrinketInventoryScreen: Component<TrinketInventoryScreenProps> = (props) => {
  const vm = () => props.viewModel;

  const unequippedTrinkets = () => vm().trinkets.filter((t) => !t.isEquipped);
  const equippedTrinkets = () => vm().trinkets.filter((t) => t.isEquipped);

  return (
    <div class="app-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Header — mirrors TrinketInventoryWindow/TitlePanel ── */}
      <header
        class="inventory-header"
        data-source-prefab="Assets/Prefabs/UI/Windows/TrinketInventoryWindow.prefab"
        data-source-hierarchy="TrinketInventoryWindow/TitlePanel"
      >
        <h1 class="inventory-title">{vm().title}</h1>
        <span class="inventory-subtitle">
          共 {vm().trinkets.length} 件饰品 · 已装备 {equippedTrinkets().length} 件
        </span>
      </header>

      {/* ── Filter bar — mirrors TrinketInventoryWindow/FilterPanel ── */}
      <div
        class="inventory-filter-bar"
        data-source-hierarchy="TrinketInventoryWindow/FilterPanel"
      >
        <For each={vm().filterOptions}>
          {(filter) => (
            <button
              class={`inventory-filter-pill ${vm().activeFilter === filter ? "inventory-filter-pill--active" : ""}`}
              onClick={() => props.onFilterChange?.(filter)}
              data-filter={filter}
            >
              {filter}
            </button>
          )}
        </For>
      </div>

      {/* ── Content: hero trinket pairs + trinket grid ── */}
      <div class="inventory-content">
        {/* Left Panel — Hero Trinket Slots — mirrors HeroSlot + PartyInventorySlot */}
        <div
          class="inventory-left"
          data-source-hierarchy="TrinketInventoryWindow/LeftPanel"
        >
          <h3 class="inventory-section-title">英雄饰品</h3>
          <For each={vm().heroes}>
            {(hero) => (
              <div
                class="inventory-hero-card"
                data-source-prefab="Assets/Prefabs/UI/HeroSlot.prefab"
                data-source-component="HeroSlot"
              >
                <div class="inventory-hero-info">
                  <span class="inventory-hero-name">{hero.heroName}</span>
                  <span class="inventory-hero-class">{hero.classLabel}</span>
                </div>
                <div class="inventory-hero-trinkets">
                  <div
                    class={`inventory-trinket-slot ${hero.leftTrinket ? rarityClass(hero.leftTrinket.rarity) : "inventory-trinket-slot--empty"}`}
                    data-source-prefab="Assets/Prefabs/UI/PartyInventorySlot.prefab"
                    data-slot="left"
                  >
                    {hero.leftTrinket ? (
                      <>
                        <span class="inventory-trinket-name">{hero.leftTrinket.name}</span>
                        <span class="inventory-trinket-desc">{hero.leftTrinket.description}</span>
                      </>
                    ) : (
                      <span class="inventory-trinket-placeholder">空槽位</span>
                    )}
                  </div>
                  <div
                    class={`inventory-trinket-slot ${hero.rightTrinket ? rarityClass(hero.rightTrinket.rarity) : "inventory-trinket-slot--empty"}`}
                    data-source-prefab="Assets/Prefabs/UI/PartyInventorySlot.prefab"
                    data-slot="right"
                  >
                    {hero.rightTrinket ? (
                      <>
                        <span class="inventory-trinket-name">{hero.rightTrinket.name}</span>
                        <span class="inventory-trinket-desc">{hero.rightTrinket.description}</span>
                      </>
                    ) : (
                      <span class="inventory-trinket-placeholder">空槽位</span>
                    )}
                  </div>
                </div>
              </div>
            )}
          </For>
        </div>

        {/* Right Panel — Trinket Grid — mirrors ShopSlot grid */}
        <div
          class="inventory-right"
          data-source-hierarchy="TrinketInventoryWindow/RightPanel"
        >
          <h3 class="inventory-section-title">仓库库存</h3>
          {unequippedTrinkets().length === 0 ? (
            <div class="inventory-empty-state">
              <p>仓库中暂无未装备饰品。</p>
            </div>
          ) : (
            <div class="inventory-trinket-grid">
              <For each={unequippedTrinkets()}>
                {(trinket) => (
                  <div
                    class={`inventory-trinket-card ${rarityClass(trinket.rarity)}`}
                    data-source-prefab={trinketCardSource(trinket)}
                    data-source-component="ShopSlot"
                  >
                    <div class="inventory-trinket-card-header">
                      <span class="inventory-trinket-card-name">{trinket.name}</span>
                      {trinket.rarity && (
                        <span class={`inventory-trinket-rarity-badge ${rarityClass(trinket.rarity)}`}>
                          {trinket.rarity}
                        </span>
                      )}
                    </div>
                    <p class="inventory-trinket-card-desc">{trinket.description}</p>
                    {trinket.classRestriction && (
                      <span class="inventory-trinket-restriction">
                        限制: {trinket.classRestriction}
                      </span>
                    )}
                  </div>
                )}
              </For>
            </div>
          )}
        </div>
      </div>

      {/* ── Return to Town — mirrors TrinketInventoryWindow/CloseButton ── */}
      <div class="building-return-row">
        <button
          class="building-return-btn"
          onClick={props.onReturn}
          data-source-component="CloseButton"
          data-source-sprite="Assets/Sprites/ui/btn_close.png"
        >
          Return to Town
        </button>
      </div>
    </div>
  );
};
