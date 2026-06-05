import { For, type Component } from "solid-js";

import type { DungeonSettlementViewModel } from "../../bridge/contractTypes";
import { resolveChromeAsset, resolveHeroPortrait } from "../../assets/originalAssetPaths";

interface DungeonSettlementScreenProps {
  viewModel: DungeonSettlementViewModel;
  onContinueToNextRoom: () => void;
  onRetreatFromDungeon: () => void;
}

function parseHp(hp: string, maxHp: string): number {
  const current = Number(hp.split(" / ")[0]?.trim() ?? hp.trim());
  const max = Number(maxHp.trim());
  if (max <= 0) return 0;
  return Math.round((current / max) * 100);
}

function healthBarColor(hpPct: number): string {
  if (hpPct >= 80) return "#5bbd6e";
  if (hpPct >= 40) return "#e8a838";
  return "#ea7767";
}

function stressPercent(stress: string, maxStress: string): number {
  const s = Number(stress.trim());
  const m = Number(maxStress.trim() || 200);
  return Math.min(Math.round((s / m) * 100), 100);
}

function stressBarColor(stress: string): string {
  const s = Number(stress.trim());
  if (s <= 20) return "#5bbd6e";
  if (s <= 40) return "#e8a838";
  return "#ea7767";
}

/**
 * Dungeon Settlement screen — landscape viewport layout.
 *
 * Displays room completion outcome with hero status, loot, and resources.
 * References original Unity prefab structure from:
 *   Assets/Prefabs/UI/DungeonSettlementWindow.prefab
 *
 * Source hierarchy: UI_Dungeon/DungeonSettlementWindow
 *   RoomBannerPanel → RoomLabel + RoomOutcome
 *   HeroStatusPanel → HeroStatusCard × 4
 *   RewardsPanel → GoldLabel + LootGrid + ResourceLine
 *   ControlButtons → NextRoomButton + RetreatButton
 */
export const DungeonSettlementScreen: Component<DungeonSettlementScreenProps> = (props) => {
  const outcomeLabel = () => {
    switch (props.viewModel.outcome) {
      case "cleared":
        return "Room Cleared";
      case "retreat":
        return "Retreating";
      case "defeated":
        return "Party Defeated";
    }
  };

  const outcomeBannerClass = () => {
    switch (props.viewModel.outcome) {
      case "cleared":
        return "outcome-banner outcome-banner--success";
      case "retreat":
        return "outcome-banner outcome-banner--partial";
      case "defeated":
        return "outcome-banner outcome-banner--failure";
    }
  };

  const heroStatusClass = (status: string) => {
    switch (status) {
      case "dead":
        return "hero-outcome-status--dead";
      case "wounded":
        return "hero-outcome-status--stressed";
      case "stressed":
        return "hero-outcome-status--stressed";
      default:
        return "hero-outcome-status--alive";
    }
  };

  const heroStatusLabel = (status: string) => {
    switch (status) {
      case "dead":
        return "Deceased";
      case "wounded":
        return "Wounded";
      case "stressed":
        return "Stressed";
      default:
        return "Fit";
    }
  };

  const heroCardExtraClass = (status: string) => {
    return status === "dead" ? "hero-outcome-card hero-outcome-card--dead" : "hero-outcome-card";
  };

  const heroHasCasualties = () =>
    props.viewModel.heroOutcomes.some((h) => h.status === "dead");

  const isDefeated = () => props.viewModel.outcome === "defeated";

  return (
    <div
      class="expedition-viewport dungeon-settlement-viewport"
      data-source-scene="UI_Dungeon/DungeonSettlementWindow"
      data-source-prefab="Assets/Prefabs/UI/DungeonSettlementWindow.prefab"
    >
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">Dungeon Settlement</span>
          <h1 class="expedition-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class="hud-pill hud-pill-accent">{props.viewModel.expeditionName}</span>
          <span class="hud-pill">
            Room {props.viewModel.roomNumber} / {props.viewModel.totalRooms}
          </span>
          <span class="hud-pill hud-pill--with-icon">
            <span class="hud-pill-icon" aria-hidden="true">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7">
                <path d="M12 2L4 6v6c0 5 3.4 9.4 8 10 4.6-.6 8-5 8-10V6l-8-4z" />
              </svg>
            </span>
            {props.viewModel.roomName}
          </span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="expedition-content">
          {/* Outcome banner */}
          <div class={outcomeBannerClass()}>
            <div class="outcome-banner-ornament" />
            <h2 class="outcome-banner-title">{outcomeLabel()}</h2>
            <p class="outcome-banner-subtitle">{props.viewModel.summary}</p>
            {isDefeated() && (
              <p class="outcome-banner-detail outcome-detail--failure">
                Your party has been overwhelmed. Those who survive must retreat and recover.
              </p>
            )}
            {!isDefeated() && (
              <p class="outcome-banner-detail outcome-detail--success">
                The room has been cleared. Gather your spoils and decide whether to press on or retreat.
              </p>
            )}
          </div>

          {/* Hero outcomes */}
          <div class="hero-outcome-row dungeon-settlement-hero-row">
            <For each={props.viewModel.heroOutcomes}>
              {(hero) => {
                const portraitUrl = resolveHeroPortrait({ heroId: hero.heroId, classLabel: hero.classLabel });
                const hpPct = parseHp(hero.hp, hero.maxHp);
                const stPct = stressPercent(hero.stress, hero.maxStress);
                return (
                  <div class={heroCardExtraClass(hero.status)} data-hero-id={hero.heroId}>
                    {portraitUrl ? (
                      <img
                        class="hero-outcome-portrait"
                        src={portraitUrl}
                        alt={hero.heroName}
                      />
                    ) : (
                      <div class="hero-outcome-portrait hero-outcome-portrait--fallback">
                        <span class="hero-outcome-portrait-letter">
                          {hero.heroName[0]}
                        </span>
                      </div>
                    )}
                    <div class="hero-outcome-name">{hero.heroName}</div>
                    <span class={`hero-outcome-status ${heroStatusClass(hero.status)}`}>
                      {heroStatusLabel(hero.status)}
                    </span>

                    {/* HP bar */}
                    <div class="dungeon-settlement-bar-row">
                      <span class="dungeon-settlement-bar-label">HP</span>
                      <div class="dungeon-settlement-bar-track">
                        <div
                          class="dungeon-settlement-bar-fill"
                          style={{
                            width: `${hpPct}%`,
                            background: healthBarColor(hpPct),
                          }}
                        />
                      </div>
                      <span class="dungeon-settlement-bar-value">{hero.hp}</span>
                    </div>

                    {/* Stress bar */}
                    <div class="dungeon-settlement-bar-row">
                      <span class="dungeon-settlement-bar-label">ST</span>
                      <div class="dungeon-settlement-bar-track">
                        <div
                          class="dungeon-settlement-bar-fill"
                          style={{
                            width: `${stPct}%`,
                            background: stressBarColor(hero.stress),
                          }}
                        />
                      </div>
                      <span class="dungeon-settlement-bar-value">{hero.stress}</span>
                    </div>

                    {/* Changes */}
                    {hero.status !== "dead" && (
                      <div class="hero-outcome-changes">
                        <div class="hero-outcome-change">
                          <span class="hero-outcome-change-label">HP</span>
                          <span class={`hero-outcome-change-value ${hero.hpChange.startsWith("-") ? "change-negative" : "change-positive"}`}>
                            {hero.hpChange}
                          </span>
                        </div>
                        <div class="hero-outcome-change">
                          <span class="hero-outcome-change-label">Stress</span>
                          <span class={`hero-outcome-change-value ${hero.stressChange.startsWith("+") ? "change-stress" : "change-positive"}`}>
                            {hero.stressChange}
                          </span>
                        </div>
                      </div>
                    )}
                  </div>
                );
              }}
            </For>
          </div>

          {/* Resources gained */}
          <div class="resources-panel">
            <div class="resource-item">
              <span class="resource-label">Gold</span>
              <span class="resource-value resource-value--positive result-gold-value">
                <img
                  class="gold-icon-image"
                  src={resolveChromeAsset("goldIcon")}
                  alt=""
                  aria-hidden="true"
                />
                +{props.viewModel.resourcesGained.gold}
              </span>
            </div>
            <div class="resource-item">
              <span class="resource-label">Supplies</span>
              <span class={`resource-value ${props.viewModel.resourcesGained.supplies >= 0 ? "resource-value--positive" : "resource-value--negative"}`}>
                {props.viewModel.resourcesGained.supplies >= 0 ? "+" : ""}{props.viewModel.resourcesGained.supplies}
              </span>
            </div>
            <div class="resource-item">
              <span class="resource-label">Experience</span>
              <span class="resource-value resource-value--positive">
                +{props.viewModel.resourcesGained.experience}
              </span>
            </div>
          </div>

          {/* Loot */}
          {props.viewModel.lootAcquired.length > 0 && (
            <div class="loot-panel">
              <div class="loot-title">Loot Acquired</div>
              <For each={props.viewModel.lootAcquired}>
                {(item) => (
                  <div class="loot-item">{item}</div>
                )}
              </For>
            </div>
          )}

          {/* Casualty warning */}
          {heroHasCasualties() && (
            <div class="details-overlay" style="border-color: rgba(234, 119, 103, 0.3);">
              <p class="casualty-message">
                <strong>Casualties sustained.</strong> Some heroes did not survive the encounter. Their sacrifice will not be forgotten.
              </p>
            </div>
          )}
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left">
          <span class="hud-pill">
            Room {props.viewModel.roomNumber} / {props.viewModel.totalRooms}
          </span>
        </div>
        <div class="expedition-controls-right">
          <button
            class="action-secondary"
            onClick={props.onRetreatFromDungeon}
            disabled={!props.viewModel.isRetreatAvailable}
          >
            {props.viewModel.isRetreatAvailable
              ? "Retreat to Town"
              : "Cannot Retreat"}
          </button>
          <button
            class="action-primary launch-primary"
            onClick={props.onContinueToNextRoom}
            disabled={!props.viewModel.isNextRoomAvailable}
          >
            {props.viewModel.isNextRoomAvailable
              ? "Continue to Next Room"
              : "No Further Rooms"}
          </button>
        </div>
      </footer>
    </div>
  );
};
