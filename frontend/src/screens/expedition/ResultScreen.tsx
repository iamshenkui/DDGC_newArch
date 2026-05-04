import { For, type Component } from "solid-js";

import type { ExpeditionResultViewModel } from "../../bridge/contractTypes";

interface ResultScreenProps {
  viewModel: ExpeditionResultViewModel;
  onContinue: () => void;
}

/**
 * Expedition result screen — landscape viewport layout.
 *
 * Displays expedition outcome with hero outcomes, loot, and resources gained.
 * References original Unity prefab structure from:
 *   Assets/Prefabs/UI/ExpeditionResultWindow.prefab (estimated)
 */
export const ResultScreen: Component<ResultScreenProps> = (props) => {
  const outcomeLabel = () => {
    switch (props.viewModel.outcome) {
      case "success":
        return "Victory";
      case "failure":
        return "Defeat";
      case "partial":
        return "Partial Success";
    }
  };

  const outcomeBannerClass = () => {
    switch (props.viewModel.outcome) {
      case "success":
        return "outcome-banner outcome-banner--success";
      case "failure":
        return "outcome-banner outcome-banner--failure";
      case "partial":
        return "outcome-banner outcome-banner--partial";
    }
  };

  const heroOutcomeStatusClass = (status: string) => {
    switch (status) {
      case "dead":
        return "hero-outcome-status--dead";
      case "stressed":
        return "hero-outcome-status--stressed";
      default:
        return "hero-outcome-status--alive";
    }
  };

  const heroOutcomeStatusLabel = (status: string) => {
    switch (status) {
      case "dead":
        return "Deceased";
      case "stressed":
        return "Stressed";
      default:
        return "Alive";
    }
  };

  const heroCardExtraClass = (status: string) => {
    return status === "dead" ? "hero-outcome-card hero-outcome-card--dead" : "hero-outcome-card";
  };

  const heroHasCasualties = () =>
    props.viewModel.heroOutcomes.some((h) => h.status === "dead");

  const isFailure = () => props.viewModel.outcome === "failure";
  const isPartial = () => props.viewModel.outcome === "partial";

  return (
    <div class="expedition-viewport">
      {/* ── Top HUD ─────────────────────────────────────── */}
      <header class="expedition-hud">
        <span class="expedition-hud-left">
          <span class="eyebrow">Expedition Complete</span>
          <h1 class="expedition-title">{props.viewModel.title}</h1>
        </span>
        <span class="expedition-hud-center">
          <span class="hud-pill hud-pill-accent">{props.viewModel.expeditionName}</span>
        </span>
      </header>

      {/* ── Game Surface ─────────────────────────────────── */}
      <div class="expedition-surface">
        <div class="expedition-surface-bg" />
        <div class="expedition-surface-mist" />

        <div class="expedition-content">
          {/* Outcome banner */}
          <div class={outcomeBannerClass()}>
            <h2 class="outcome-banner-title">{outcomeLabel()}</h2>
            <p class="outcome-banner-subtitle">{props.viewModel.summary}</p>
            {isFailure() && (
              <p class="outcome-banner-summary" style="color: #ea7767;">
                The expedition has ended in defeat. Prepare your remaining forces before venturing forth again.
              </p>
            )}
            {isPartial() && (
              <p class="outcome-banner-summary" style="color: #e8a838;">
                The expedition achieved partial objectives. Tend to your heroes before the next venture.
              </p>
            )}
            {!isFailure() && !isPartial() && (
              <p class="outcome-banner-summary" style="color: #5bbd6e;">
                The expedition concluded successfully. Your heroes stand ready for the next challenge.
              </p>
            )}
          </div>

          {/* Hero outcomes */}
          <div class="hero-outcome-row">
            <For each={props.viewModel.heroOutcomes}>
              {(hero) => (
                <div class={heroCardExtraClass(hero.status)}>
                  <div class="hero-outcome-name">{hero.heroName}</div>
                  <span class={`hero-outcome-status ${heroOutcomeStatusClass(hero.status)}`}>
                    {heroOutcomeStatusLabel(hero.status)}
                  </span>
                  {hero.status !== "dead" && (
                    <div class="hero-outcome-changes">
                      <div class="hero-outcome-change">
                        <span class="hero-outcome-change-label">HP</span>
                        <span style={hero.hpChange.startsWith("-") ? "color: #ea7767;" : "color: #5bbd6e;"}>
                          {hero.hpChange}
                        </span>
                      </div>
                      <div class="hero-outcome-change">
                        <span class="hero-outcome-change-label">Stress</span>
                        <span style={hero.stressChange.startsWith("+") ? "color: #e8a838;" : "color: #5bbd6e;"}>
                          {hero.stressChange}
                        </span>
                      </div>
                    </div>
                  )}
                </div>
              )}
            </For>
          </div>

          {/* Resources gained */}
          <div class="resources-panel">
            <div class="resource-item">
              <span class="resource-label">Gold</span>
              <span class="resource-value resource-value--positive">
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
              <p style="margin: 0; color: #ea7767; font-size: 0.78rem;">
                <strong>Casualties sustained.</strong> Some heroes did not return. Visit the Stagecoach to recruit new party members.
              </p>
            </div>
          )}
        </div>
      </div>

      {/* ── Bottom Controls ───────────────────────────────── */}
      <footer class="expedition-controls">
        <div class="expedition-controls-left" />
        <div class="expedition-controls-right">
          <button
            class="action-primary"
            onClick={props.onContinue}
            disabled={!props.viewModel.isContinueAvailable}
          >
            Continue to Town
          </button>
        </div>
      </footer>
    </div>
  );
};
