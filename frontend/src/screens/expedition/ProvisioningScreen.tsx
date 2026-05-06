import { For, type Component } from "solid-js";

import type { ProvisioningViewModel } from "../../bridge/contractTypes";
import { resolveChromeAsset, resolveHeroPortrait } from "../../assets/originalAssetPaths";

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
 * Original asset wiring:
 *   gold       — extracted, served from /original/chrome/gold.png (UIR-005B).
 *                Wired via resolveChromeAsset("goldIcon") on the cost pill.
 *   portraits  — extracted hunter family served from /original/heroes/ (UIR-005B).
 *                Wired via resolveHeroPortrait on selected party slots and the
 *                roster strip; un-extracted families fall through to a
 *                CSS-letter avatar.
 *   supply     — Assets/Resources/Sprites/inv_supply+rattle_drum.png
 *                GUID e401bf9b9275ede4aa2ff50d13cc6207.
 *                Not extracted (BLOCKER-001) — pill keeps the explicit blocker
 *                annotation and a CSS fallback swatch until the PNG is staged.
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

  const partyStatusLabel = () => {
    if (selectedCount() === 0) return "No heroes selected";
    if (afflictedCount() > 0) return `${afflictedCount()} afflicted in party`;
    if (woundedCount() > 0) return `${woundedCount()} wounded in party`;
    if (isFull()) return "Party ready to embark";
    return `${selectedCount()} of ${props.viewModel.maxPartySize} heroes selected`;
  };

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
          {/* Party count pill with party glyph */}
          <span class="hud-pill hud-pill--with-icon">
            <span class="hud-pill-icon" aria-hidden="true">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7">
                <circle cx="9" cy="8" r="3.5" />
                <path d="M2 20c0-3.6 3.2-6 7-6s7 2.4 7 6" />
                <circle cx="17" cy="9" r="2.5" />
                <path d="M22 19c0-2.4-1.8-4-4-4" />
              </svg>
            </span>
            Party: {selectedCount()}/{props.viewModel.maxPartySize}
          </span>
          {/* Expedition label */}
          <span class="hud-pill hud-pill-accent">
            {props.viewModel.expeditionLabel}
          </span>
          {/* Supply level pill with icon */}
          <span
            class="hud-pill supply-pill"
            data-source-component="ProvisionInventoryEntry"
            data-asset-path="Assets/Resources/Sprites/inv_supply+rattle_drum.png"
            data-guid="e401bf9b9275ede4aa2ff50d13cc6207"
            data-extraction-status="not-extracted"
            data-blocker="BLOCKER-001: Original Unity supply sprite not in repository"
          >
            <span class="supply-icon-fallback" aria-hidden="true" />
            Supply: {props.viewModel.supplyLevel}
          </span>
          {/* Provision cost pill with extracted gold sprite */}
          <span
            class="hud-pill gold-pill"
            data-source-component="ProvisionCostEntry"
            data-asset-path="Assets/Resources/Sprites/gold.png"
            data-extraction-status="staged"
          >
            <img
              class="gold-icon-image"
              src={resolveChromeAsset("goldIcon")}
              alt=""
              aria-hidden="true"
            />
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
          <div class="party-formation-banner">
            <span class="party-formation-banner-frame">
              <span class="party-formation-banner-rule" aria-hidden="true" />
              <span class="party-formation-banner-title">Party Formation</span>
              <span class="party-formation-banner-rule" aria-hidden="true" />
            </span>
            <span class="party-formation-banner-hint">
              Tap a hero card to remove from the formation. Add heroes from the roster below.
            </span>
          </div>

          {/* Party formation — main focal area */}
          <div class="party-formation">
            <For each={partyFormation()}>
              {(hero, index) => {
                if (hero === null) {
                  return (
                    <div class="party-slot party-slot--empty" data-slot-index={index()}>
                      <span class="party-slot-empty-marker" aria-hidden="true">
                        <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                          <line x1="12" y1="5" x2="12" y2="19" />
                          <line x1="5" y1="12" x2="19" y2="12" />
                        </svg>
                      </span>
                      <span class="party-slot-empty-hint">Open Slot</span>
                      <span class="party-slot-empty-sub">Choose a hero from the roster</span>
                    </div>
                  );
                }

                const showStatus = hero.isWounded || hero.isAfflicted;
                const portraitUrl = resolveHeroPortrait({
                  heroId: hero.id,
                  classLabel: hero.classLabel
                });

                return (
                  <button
                    class="party-slot party-slot--selected"
                    onClick={() => props.onToggleHeroSelection(hero.id)}
                    title={`Remove ${hero.name} from party`}
                    data-source-prefab="Assets/Prefabs/UI/PartyInventorySlot.prefab"
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
                    {/* Portrait — original PNG when extracted, CSS letter otherwise */}
                    <div
                      class={`party-slot-portrait${portraitUrl ? " party-slot-portrait--image" : " party-slot-portrait--fallback"}`}
                    >
                      {portraitUrl ? (
                        <img
                          class="party-slot-portrait-image"
                          src={portraitUrl}
                          alt=""
                          aria-hidden="true"
                        />
                      ) : (
                        <span
                          class="party-slot-initial"
                          data-blocker="BLOCKER-002: portrait sprite not extracted for this hero family"
                          data-class-label={hero.classLabel}
                        >
                          {hero.classLabel[0]}
                        </span>
                      )}
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
        </div>
      </div>

      {/* ── Available heroes roster strip (like town roster) ── */}
      {unselectedHeroes().length > 0 && (
        <section
          class="provisioning-roster-strip"
          data-source-hierarchy="UI_Provision/RosterPanel"
        >
          <div class="provisioning-roster-header">
            <span class="provisioning-roster-label">Available Heroes</span>
            <span class="provisioning-roster-count">
              {unselectedHeroes().length} on standby
            </span>
          </div>
          <div class="roster-scroll provisioning-roster-scroll">
            <For each={unselectedHeroes()}>
              {(hero) => {
                const hpInfo = parseHp(hero.hp);
                const stressNum = Number(hero.stress);
                const stressMax = Number(hero.maxStress || 200);
                const showStatus = hero.isWounded || hero.isAfflicted;
                const portraitUrl = resolveHeroPortrait({
                  heroId: hero.id,
                  classLabel: hero.classLabel
                });

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
                      <div
                        class={`roster-portrait-avatar${portraitUrl ? " roster-portrait-avatar--image" : ""}`}
                      >
                        {portraitUrl ? (
                          <img
                            class="roster-portrait-image"
                            src={portraitUrl}
                            alt=""
                            aria-hidden="true"
                          />
                        ) : (
                          <span
                            class="roster-portrait-letter"
                            data-blocker="BLOCKER-002: portrait sprite not extracted for this hero family"
                            data-class-label={hero.classLabel}
                          >
                            {hero.classLabel[0]}
                          </span>
                        )}
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
          <span
            class={`expedition-status-pill ${
              afflictedCount() > 0
                ? "expedition-status-pill--afflicted"
                : woundedCount() > 0
                  ? "expedition-status-pill--wounded"
                  : isFull()
                    ? "expedition-status-pill--ready"
                    : "expedition-status-pill--neutral"
            }`}
            data-state={
              afflictedCount() > 0
                ? "afflicted"
                : woundedCount() > 0
                  ? "wounded"
                  : isFull()
                    ? "ready"
                    : "incomplete"
            }
          >
            <span class="expedition-status-pill-dot" aria-hidden="true" />
            {partyStatusLabel()}
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
