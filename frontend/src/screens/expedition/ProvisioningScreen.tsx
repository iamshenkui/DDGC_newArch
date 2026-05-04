import { For, type Component } from "solid-js";

import type { ProvisioningViewModel } from "../../bridge/contractTypes";

interface ProvisioningScreenProps {
  viewModel: ProvisioningViewModel;
  onToggleHeroSelection: (heroId: string) => void;
  onConfirmProvisioning: () => void;
  onReturnToTown: () => void;
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
 * Provisioning screen — landscape viewport layout.
 *
 * Mirrors the party/loadout preparation feel from original Unity prefabs:
 *   Assets/Prefabs/UI/PartyInventorySlot.prefab
 *   Assets/Prefabs/UI/PartyInventorySlotInDungeon.prefab
 *
 * Supply icon references:
 *   data-asset-path="Assets/Resources/Sprites/inv_supply+rattle_drum.png"
 *   data-guid="e401bf9b9275ede4aa2ff50d13cc6207"
 *
 * Gold icon reference:
 *   data-asset-path="Assets/Resources/Sprites/gold.png"
 */
export const ProvisioningScreen: Component<ProvisioningScreenProps> = (props) => {
  const selectedCount = () =>
    props.viewModel.party.filter((h) => h.isSelected).length;

  const woundedCount = () =>
    props.viewModel.party.filter((h) => h.isSelected && h.isWounded).length;

  const afflictedCount = () =>
    props.viewModel.party.filter((h) => h.isSelected && h.isAfflicted).length;

  const selectedHeroes = () =>
    props.viewModel.party.filter((h) => h.isSelected);

  const unselectedHeroes = () =>
    props.viewModel.party.filter((h) => !h.isSelected);

  // Build party formation: selected heroes + empty slots
  const partyFormation = () => {
    const slots = [...selectedHeroes()];
    while (slots.length < props.viewModel.maxPartySize) {
      slots.push(null as unknown as typeof props.viewModel.party[number]);
    }
    return slots;
  };

  const isFull = () => selectedCount() >= props.viewModel.maxPartySize;

  return (
    <div class="expedition-viewport">
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">Provisioning</span>
          <h1 class="expedition-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class="hud-pill">
            Party: {selectedCount()} / {props.viewModel.maxPartySize}
          </span>
          <span class="hud-pill">
            {props.viewModel.expeditionLabel}
          </span>
          <span
            class="hud-pill"
            data-asset-path="Assets/Resources/Sprites/inv_supply+rattle_drum.png"
            data-guid="e401bf9b9275ede4aa2ff50d13cc6207"
          >
            Supply: {props.viewModel.supplyLevel}
          </span>
          <span
            class="hud-pill"
            data-asset-path="Assets/Resources/Sprites/gold.png"
          >
            Cost: {props.viewModel.provisionCost}
          </span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="expedition-content">
          {/* Party formation slots */}
          <div class="party-formation">
            <For each={partyFormation()}>
              {(hero) => {
                if (hero === null) {
                  return (
                    <div class="party-slot party-slot--empty">
                      <div class="party-slot-empty-icon">
                        <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
                          <circle cx="12" cy="8" r="4" />
                          <path d="M4 20c0-4 3.6-7 8-7s8 3 8 7" />
                        </svg>
                      </div>
                      <span class="party-slot-empty-label">Empty</span>
                    </div>
                  );
                }

                const hpInfo = parseHp(hero.hp);
                const stressNum = Number(hero.stress);
                const stressMax = Number(hero.maxStress || 200);
                const showStatus = hero.isWounded || hero.isAfflicted;

                return (
                  <button
                    class="party-slot party-slot--selected"
                    onClick={() => props.onToggleHeroSelection(hero.id)}
                    title={`Remove ${hero.name} from party`}
                  >
                    {/* Status badge */}
                    {showStatus && (
                      <span
                        class={`party-slot-status ${
                          hero.isAfflicted
                            ? "party-slot-status--afflicted"
                            : "party-slot-status--wounded"
                        }`}
                      >
                        {hero.isAfflicted ? "A" : "W"}
                      </span>
                    )}
                    {/* Level badge */}
                    <span class="party-slot-level">Lv{hero.level}</span>
                    {/* Portrait */}
                    <div class="party-slot-portrait">
                      <span class="party-slot-initial">{hero.classLabel[0]}</span>
                    </div>
                    {/* Name */}
                    <span class="party-slot-name">{hero.name}</span>
                    <span class="party-slot-class">{hero.classLabel}</span>
                    {/* Bars */}
                    <div class="party-slot-bars">
                      <div class="party-slot-bar-row">
                        <div class="party-slot-bar-label">HP</div>
                        <div class="party-slot-bar-track">
                          <div
                            class="party-slot-bar-fill"
                            style={{
                              width: `${healthPercent(hero.hp)}%`,
                              background: healthBarColor(hero.hp),
                            }}
                          />
                        </div>
                      </div>
                      <div class="party-slot-bar-row">
                        <div class="party-slot-bar-label">ST</div>
                        <div class="party-slot-bar-track">
                          <div
                            class="party-slot-bar-fill"
                            style={{
                              width: `${stressPercent(hero.stress, hero.maxStress)}%`,
                              background: stressBarColor(hero.stress),
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

          {/* Readiness state */}
          {selectedCount() > 0 && (
            <div class="readiness-panel">
              <div class="readiness-stat">
                <span class="readiness-icon">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M20 12H4M12 4v16" />
                  </svg>
                </span>
                <span class="readiness-label">Selected</span>
                <span class="readiness-value">{selectedCount()} / {props.viewModel.maxPartySize}</span>
              </div>
              {woundedCount() > 0 && (
                <div class="readiness-stat">
                  <span class="readiness-icon">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M12 8v8M8 12h8" />
                    </svg>
                  </span>
                  <span class="readiness-label">Wounded</span>
                  <span class="readiness-value readiness-warning">{woundedCount()}</span>
                </div>
              )}
              {afflictedCount() > 0 && (
                <div class="readiness-stat">
                  <span class="readiness-icon">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M18 6L6 18M6 6l12 12" />
                    </svg>
                  </span>
                  <span class="readiness-label">Afflicted</span>
                  <span class="readiness-value readiness-danger">{afflictedCount()}</span>
                </div>
              )}
              {woundedCount() === 0 && afflictedCount() === 0 && selectedCount() > 0 && (
                <div class="readiness-stat">
                  <span class="readiness-icon">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
                      <path d="M22 4L12 14.01l-3-3" />
                    </svg>
                  </span>
                  <span class="readiness-label">Ready</span>
                  <span class="readiness-value readiness-ready">Party Fit</span>
                </div>
              )}
            </div>
          )}

          {/* Available heroes (roster) */}
          {unselectedHeroes().length > 0 && (
            <div class="readiness-panel" style="flex-direction: column; gap: 8px; padding: 12px;">
              <div style="font-size: 0.7rem; text-transform: uppercase; letter-spacing: 0.08em; color: var(--panel-muted); align-self: flex-start;">
                Available Heroes
              </div>
              <div style="display: flex; gap: 8px; flex-wrap: wrap; justify-content: center;">
                <For each={unselectedHeroes()}>
                  {(hero) => {
                    const hpInfo = parseHp(hero.hp);
                    const stressNum = Number(hero.stress);
                    const stressMax = Number(hero.maxStress || 200);
                    const showStatus = hero.isWounded || hero.isAfflicted;

                    return (
                      <button
                        class="party-slot"
                        style="min-height: 140px; width: 120px;"
                        onClick={() => props.onToggleHeroSelection(hero.id)}
                        disabled={!hero.isSelected && isFull()}
                        title={isFull() ? "Party is full" : `Add ${hero.name} to party`}
                      >
                        {/* Status badge */}
                        {showStatus && (
                          <span
                            class={`party-slot-status ${
                              hero.isAfflicted
                                ? "party-slot-status--afflicted"
                                : "party-slot-status--wounded"
                            }`}
                          >
                            {hero.isAfflicted ? "A" : "W"}
                          </span>
                        )}
                        <span class="party-slot-level">Lv{hero.level}</span>
                        <div class="party-slot-portrait">
                          <span class="party-slot-initial">{hero.classLabel[0]}</span>
                        </div>
                        <span class="party-slot-name">{hero.name}</span>
                        <span class="party-slot-class">{hero.classLabel}</span>
                        <div class="party-slot-bars">
                          <div class="party-slot-bar-row">
                            <div class="party-slot-bar-label">HP</div>
                            <div class="party-slot-bar-track">
                              <div
                                class="party-slot-bar-fill"
                                style={{
                                  width: `${healthPercent(hero.hp)}%`,
                                  background: healthBarColor(hero.hp),
                                }}
                              />
                            </div>
                          </div>
                          <div class="party-slot-bar-row">
                            <div class="party-slot-bar-label">ST</div>
                            <div class="party-slot-bar-track">
                              <div
                                class="party-slot-bar-fill"
                                style={{
                                  width: `${stressPercent(hero.stress, hero.maxStress)}%`,
                                  background: stressBarColor(hero.stress),
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
          )}
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left">
          <span class="hud-pill" style="font-size: 0.72rem;">
            {props.viewModel.expeditionSummary}
          </span>
        </div>
        <div class="expedition-controls-right">
          <button class="action-secondary" onClick={props.onReturnToTown}>
            Return to Town
          </button>
          <button
            class="action-primary"
            onClick={props.onConfirmProvisioning}
            disabled={!props.viewModel.isReadyToLaunch}
          >
            {props.viewModel.isReadyToLaunch
              ? "Confirm & Launch Expedition"
              : "Select Party Members"}
          </button>
        </div>
      </footer>
    </div>
  );
};
