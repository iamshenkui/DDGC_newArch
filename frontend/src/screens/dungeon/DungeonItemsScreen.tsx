import { For, type Component, createSignal } from "solid-js";

import type { DungeonItemsViewModel, EquipmentSlot, InventoryItem } from "../../bridge/contractTypes";
import { resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface DungeonItemsScreenProps {
  viewModel: DungeonItemsViewModel;
  onReturnToTown: () => void;
  onContinue: () => void;
  onSelectHero: (heroId: string) => void;
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

function equipmentSlotIcon(slot: EquipmentSlot): string {
  if (slot.isEmpty) return "—";
  switch (slot.slotId) {
    case "weapon": return "⚔";
    case "armor": return "🛡";
    case "trinket-1": return "💍";
    case "trinket-2": return "📿";
    case "consumable-1": return "🧪";
    case "consumable-2": return "📜";
    default: return "◆";
  }
}

/**
 * Dungeon Scene — Items screen (副本场景-物品)
 *
 * Shows the party in a dungeon environment with character equipment
 * and inventory panels. References original Unity prefab structure from:
 *   Assets/Prefabs/UI/DungeonScene/DungeonItemWindow.prefab (estimated)
 *
 * Layout mirrors the reference image:
 *   - Top HUD with dungeon name and scene label
 *   - Central dungeon backdrop with party formation
 *   - Bottom panel: hero detail (left) + inventory grid (right)
 *   - Footer controls for navigation
 */
export const DungeonItemsScreen: Component<DungeonItemsScreenProps> = (props) => {
  const selectedHero = () =>
    props.viewModel.party.find((h) => h.id === props.viewModel.selectedHeroId) ??
    props.viewModel.party[0];

  const [activeTab, setActiveTab] = createSignal<"equipment" | "inventory">("equipment");

  return (
    <div
      class="expedition-viewport"
      data-source-scene="UI_Dungeon/DungeonItemWindow"
      data-source-prefab="Assets/Prefabs/UI/DungeonScene/DungeonItemWindow.prefab"
      data-testid="dungeon-items-screen"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">Dungeon Exploration</span>
          <h1 class="expedition-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class="hud-pill hud-pill-accent">{props.viewModel.dungeonName}</span>
          <span class="hud-pill">{props.viewModel.floorLabel}</span>
          <span class="hud-pill hud-pill--with-icon">
            <span class="hud-pill-icon" aria-hidden="true">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7">
                <circle cx="9" cy="8" r="3.5" />
                <path d="M2 20c0-3.6 3.2-6 7-6s7 2.4 7 6" />
                <circle cx="17" cy="9" r="2.5" />
                <path d="M22 19c0-2.4-1.8-4-4-4" />
              </svg>
            </span>
            Party: {props.viewModel.party.length} heroes
          </span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="dungeon-scene-bg" />
        <div class="dungeon-scene-mist" />
        <div class="dungeon-scene-lanterns" aria-hidden="true" />

        {/* Party formation walking in dungeon */}
        <div class="dungeon-party-formation">
          <For each={props.viewModel.party}>
            {(hero) => {
              const portraitUrl = resolveHeroPortrait({
                heroId: hero.id,
                classLabel: hero.classLabel,
              });
              const isSelected = hero.id === props.viewModel.selectedHeroId;
              return (
                <button
                  class={`dungeon-hero-silhouette ${isSelected ? "dungeon-hero-silhouette--selected" : ""}`}
                  onClick={() => props.onSelectHero(hero.id)}
                  data-hero-id={hero.id}
                  data-testid={`dungeon-hero-${hero.id}`}
                >
                  <div
                    class={`vitals-card-portrait${portraitUrl ? " vitals-card-portrait--image" : " vitals-card-portrait--fallback"}`}
                    style={{ width: "44px", height: "44px" }}
                  >
                    {portraitUrl ? (
                      <img
                        class="vitals-card-portrait-image"
                        src={portraitUrl}
                        alt=""
                        aria-hidden="true"
                      />
                    ) : (
                      <span class="vitals-card-initial">{hero.classLabel[0]}</span>
                    )}
                  </div>
                  <span class="dungeon-hero-name">{hero.name}</span>
                  {isSelected && (
                    <span class="dungeon-hero-selected-indicator" aria-hidden="true" />
                  )}
                </button>
              );
            }}
          </For>
        </div>

        {/* ── Bottom Item Panel ──────────────────────────── */}
        <div class="dungeon-item-panel">
          {/* Left: Selected Hero Detail */}
          <div class="dungeon-hero-detail">
            <div class="dungeon-hero-detail-header">
              <span class="details-overlay-frame-rule" aria-hidden="true" />
              <span class="dungeon-panel-title">{selectedHero()?.name}</span>
              <span class="details-overlay-frame-rule" aria-hidden="true" />
            </div>

            <div class="dungeon-hero-detail-body">
              <div class="dungeon-hero-portrait-wrap">
                {(() => {
                  const hero = selectedHero();
                  if (!hero) return null;
                  const portraitUrl = resolveHeroPortrait({
                    heroId: hero.id,
                    classLabel: hero.classLabel,
                  });
                  return portraitUrl ? (
                    <img
                      class="dungeon-hero-portrait-image"
                      src={portraitUrl}
                      alt={hero.name}
                    />
                  ) : (
                    <div class="dungeon-hero-portrait-fallback">
                      <span>{hero.classLabel[0]}</span>
                    </div>
                  );
                })()}
              </div>

              <div class="dungeon-hero-stats">
                <div class="dungeon-stat-row">
                  <span class="dungeon-stat-label">HP</span>
                  <div class="dungeon-stat-bar-track">
                    <div
                      class="dungeon-stat-bar-fill"
                      style={{
                        width: `${healthPercent(selectedHero()?.hp ?? "0 / 1")}%`,
                        background: healthBarColor(selectedHero()?.hp ?? "0 / 1"),
                      }}
                    />
                  </div>
                  <span class="dungeon-stat-value">{selectedHero()?.hp}</span>
                </div>
                <div class="dungeon-stat-row">
                  <span class="dungeon-stat-label">ST</span>
                  <div class="dungeon-stat-bar-track">
                    <div
                      class="dungeon-stat-bar-fill"
                      style={{
                        width: `${stressPercent(selectedHero()?.stress ?? "0", selectedHero()?.maxStress ?? "200")}%`,
                        background: stressBarColor(selectedHero()?.stress ?? "0"),
                      }}
                    />
                  </div>
                  <span class="dungeon-stat-value">{selectedHero()?.stress}</span>
                </div>
                <div class="dungeon-stat-row">
                  <span class="dungeon-stat-label">Lv</span>
                  <span class="dungeon-stat-value">{selectedHero()?.level}</span>
                </div>
              </div>

              {/* Equipment Grid (2x3) */}
              <div class="dungeon-equipment-grid">
                <For each={props.viewModel.selectedHeroEquipment}>
                  {(slot) => (
                    <div
                      class={`dungeon-equipment-slot ${slot.isEmpty ? "dungeon-equipment-slot--empty" : ""}`}
                      data-slot-id={slot.slotId}
                      data-testid={`equipment-slot-${slot.slotId}`}
                    >
                      <span class="dungeon-equipment-slot-icon" aria-hidden="true">
                        {equipmentSlotIcon(slot)}
                      </span>
                      <span class="dungeon-equipment-slot-label">{slot.slotLabel}</span>
                      {!slot.isEmpty && slot.itemName && (
                        <span class="dungeon-equipment-slot-item">{slot.itemName}</span>
                      )}
                    </div>
                  )}
                </For>
              </div>
            </div>
          </div>

          {/* Right: Inventory Grid */}
          <div class="dungeon-inventory-panel">
            <div class="dungeon-inventory-header">
              <span class="details-overlay-frame-rule" aria-hidden="true" />
              <span class="dungeon-panel-title">Inventory</span>
              <span class="details-overlay-frame-rule" aria-hidden="true" />
            </div>

            <div class="dungeon-inventory-grid">
              <For each={props.viewModel.inventoryItems}>
                {(item) => (
                  <div
                    class="dungeon-inventory-cell"
                    data-item-id={item.itemId}
                    data-testid={`inventory-item-${item.itemId}`}
                    title={item.description}
                  >
                    <span class="dungeon-inventory-cell-icon" aria-hidden="true">
                      {item.icon ?? "📦"}
                    </span>
                    <span class="dungeon-inventory-cell-name">{item.name}</span>
                    {item.quantity > 1 && (
                      <span class="dungeon-inventory-cell-qty">x{item.quantity}</span>
                    )}
                  </div>
                )}
              </For>

              {/* Empty slots to fill grid */}
              {Array.from({ length: Math.max(0, 12 - props.viewModel.inventoryItems.length) }).map((_, i) => (
                <div
                  class="dungeon-inventory-cell dungeon-inventory-cell--empty"
                  data-testid={`inventory-empty-${i}`}
                >
                  <span class="dungeon-inventory-cell-placeholder">—</span>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left">
          <span class="expedition-status-pill expedition-status-pill--neutral">
            <span class="expedition-status-pill-dot" aria-hidden="true" />
            Items: {props.viewModel.inventoryItems.length} / 12
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
            data-testid="dungeon-continue-button"
          >
            {props.viewModel.isContinueAvailable
              ? "Continue Expedition"
              : "Awaiting Orders"}
          </button>
        </div>
      </footer>
    </div>
  );
};
