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
 * Provisioning screen — landscape game viewport with party/loadout layout.
 *
 * Mirrors the party inventory and preparation feel from original Unity prefabs:
 *   Assets/Prefabs/UI/PartyInventorySlot.prefab
 *   Assets/Prefabs/UI/PartyInventorySlotInDungeon.prefab
 *   Assets/Prefabs/UI/ProvisionShop.prefab (estimated)
 *
 * Supply icons reference original asset paths (not extracted — see blocker notes):
 *   BLOCKER-004: Original supply sprites not extracted from Unity project
 *   data-asset-path="Assets/Resources/Sprites/inv_supply+rattle_drum.png"
 *   data-guid="e401bf9b9275ede4aa2ff50d13cc6207"
 *
 * Gold icon:
 *   BLOCKER-004: Original gold sprite not extracted from Unity project
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
    <div
      class="expedition-viewport"
      data-source-scene="UI_Provision/ProvisionShop"
      data-source-prefab="Assets/Prefabs/UI/ProvisionShop.prefab"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">Provisioning</span>
          <h1 class="expedition-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          {/* Party count pill */}
          <span class="hud-pill">
            Party: {selectedCount()} / {props.viewModel.maxPartySize}
          </span>
          {/* Expedition label */}
          <span class="hud-pill hud-pill-accent">
            {props.viewModel.expeditionLabel}
          </span>
          {/* Supply level pill with icon */}
          <span
            class="hud-pill supply-pill"
            data-asset-path="Assets/Resources/Sprites/inv_supply+rattle_drum.png"
            data-guid="e401bf9b9275ede4aa2ff50d13cc6207"
            data-extraction-status="not-extracted"
            data-blocker="BLOCKER-004: Original Unity supply sprite not extracted"
          >
            <span class="supply-icon-fallback" aria-hidden="true" />
            Supply: {props.viewModel.supplyLevel}
          </span>
          {/* Provision cost pill with gold icon */}
          <span
            class="hud-pill gold-pill"
            data-asset-path="Assets/Resources/Sprites/gold.png"
            data-extraction-status="not-extracted"
            data-blocker="BLOCKER-004: Original Unity gold sprite not extracted"
          >
            <span class="gold-icon-fallback" aria-hidden="true" />
            Cost: {props.viewModel.provisionCost}
          </span>
        </span>
        {/* Readiness summary — compact game HUD element */}
        <span class="expedition-hud-right">
          {selectedCount() > 0 && (
            <span class="readiness-summary">
              {woundedCount() > 0 && (
                <span class="readiness-badge readiness-badge--wounded" title="Wounded">
                  {woundedCount()}W
                </span>
              )}
              {afflictedCount() > 0 && (
                <span class="readiness-badge readiness-badge--afflicted" title="Afflicted">
                  {afflictedCount()}A
                </span>
              )}
              {woundedCount() === 0 && afflictedCount() === 0 && selectedCount() > 0 && (
                <span class="readiness-badge readiness-badge--ready" title="All heroes fit">
                  Ready
                </span>
              )}
            </span>
          )}
        </span>
      </header>

      {/* ── Game Surface with party formation ──────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="expedition-content provisioning-content">
          {/* Party formation — main focal area */}
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
                      <span class="party-slot-empty-label">Open</span>
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

          {/* ── Party Readiness Panel (game HUD on surface) ── */}
          {selectedCount() > 0 && (
            <div class="readiness-panel">
              <div class="readiness-stat">
                <span class="readiness-icon">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <path d="M12 2L2 7l10 5 10-5-10-5z" />
                    <path d="M2 17l10 5 10-5" />
                    <path d="M2 12l10 5 10-5" />
                  </svg>
                </span>
                <span class="readiness-label">Supply</span>
                <span class="readiness-value">{props.viewModel.supplyLevel}</span>
              </div>
              <div class="readiness-stat">
                <span class="readiness-icon">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <circle cx="12" cy="12" r="10" />
                    <path d="M12 6v6l4 2" />
                  </svg>
                </span>
                <span class="readiness-label">Cost</span>
                <span class="readiness-value">{props.viewModel.provisionCost}</span>
              </div>
              <div class="readiness-stat">
                <span class="readiness-label">Party</span>
                <span class={`readiness-value ${woundedCount() > 0 || afflictedCount() > 0 ? "readiness-warning" : "readiness-ready"}`}>
                  {selectedCount()}/{props.viewModel.maxPartySize}
                </span>
              </div>
              {woundedCount() > 0 && (
                <div class="readiness-stat">
                  <span class="readiness-label">Wounded</span>
                  <span class="readiness-value readiness-warning">{woundedCount()}</span>
                </div>
              )}
              {afflictedCount() > 0 && (
                <div class="readiness-stat">
                  <span class="readiness-label">Afflicted</span>
                  <span class="readiness-value readiness-danger">{afflictedCount()}</span>
                </div>
              )}
            </div>
          )}
        </div>
      </div>

      {/* ── Available heroes roster strip (like town roster) ── */}
      {unselectedHeroes().length > 0 && (
        <section
          class="provisioning-roster-strip"
          data-source-hierarchy="UI_Provision/RosterPanel"
        >
          <div class="provisioning-roster-label">Available Heroes</div>
          <div class="roster-scroll provisioning-roster-scroll">
            <For each={unselectedHeroes()}>
              {(hero) => {
                const hpInfo = parseHp(hero.hp);
                const stressNum = Number(hero.stress);
                const stressMax = Number(hero.maxStress || 200);
                const showStatus = hero.isWounded || hero.isAfflicted;

                return (
                  <button
                    class="roster-hero provisioning-roster-hero"
                    onClick={() => props.onToggleHeroSelection(hero.id)}
                    disabled={!hero.isSelected && isFull()}
                    title={isFull() ? "Party is full" : `Add ${hero.name} to party`}
                    data-source-prefab="Assets/Prefabs/UI/HeroSlot.prefab"
                  >
                    <div class="roster-hero-portrait">
                      <div class="roster-portrait-frame" />
                      <div class="roster-portrait-avatar">
                        <span class="roster-portrait-letter">{hero.classLabel[0]}</span>
                      </div>
                      <span class="roster-portrait-level">Lv{hero.level}</span>
                      {showStatus && (
                        <span class="roster-portrait-status">
                          {hero.isAfflicted ? "A" : "W"}
                        </span>
                      )}
                    </div>
                    <div class="roster-hero-info">
                      <span class="roster-hero-name">{hero.name}</span>
                      <span class="roster-hero-class">{hero.classLabel}</span>
                    </div>
                    <div class="roster-hero-bars">
                      <div class="roster-bar-row">
                        <span class="roster-bar-label">HP</span>
                        <span class="roster-bar-track">
                          <span
                            class="roster-bar-fill"
                            style={{
                              width: `${healthPercent(hero.hp)}%`,
                              background: healthBarColor(hero.hp),
                            }}
                          />
                        </span>
                      </div>
                      <div class="roster-bar-row">
                        <span class="roster-bar-label">ST</span>
                        <span class="roster-bar-track">
                          <span
                            class="roster-bar-fill"
                            style={{
                              width: `${stressPercent(hero.stress, hero.maxStress)}%`,
                              background: stressBarColor(hero.stress),
                            }}
                          />
                        </span>
                      </div>
                    </div>
                    <div class="roster-hero-footer">
                      <span>HP {hpInfo.current}/{hpInfo.max}</span>
                      <span>ST {stressNum}/{stressMax}</span>
                    </div>
                  </button>
                );
              }}
            </For>
          </div>
        </section>
      )}

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
            class="action-primary launch-primary"
            onClick={props.onConfirmProvisioning}
            disabled={!props.viewModel.isReadyToLaunch}
          >
            {props.viewModel.isReadyToLaunch
              ? "Confirm & Launch Expedition"
              : "Fill Party Ranks"}
          </button>
        </div>
      </footer>
    </div>
  );
};
