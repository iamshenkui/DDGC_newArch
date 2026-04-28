import type { Component } from "solid-js";

import type { ExpeditionResultViewModel } from "../../bridge/contractTypes";
import { AppFrame } from "../../components/layout/AppFrame";

interface ResultScreenProps {
  viewModel: ExpeditionResultViewModel;
  onContinue: () => void;
  onReturnToTown?: () => void;
}

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

  const outcomeClass = () => {
    switch (props.viewModel.outcome) {
      case "success":
        return "outcome-success";
      case "failure":
        return "outcome-failure";
      case "partial":
        return "outcome-partial";
    }
  };

  const heroStatusClass = (status: string) => {
    switch (status) {
      case "dead":
        return "text-danger";
      case "stressed":
        return "text-warning";
      default:
        return "text-good";
    }
  };

  const heroStatusLabel = (status: string) => {
    switch (status) {
      case "dead":
        return "DECEASED";
      case "stressed":
        return "Stressed";
      default:
        return "Alive";
    }
  };

  const heroCardClass = (status: string) => {
    return status === "dead" ? "surface-card dead-hero" : "surface-card";
  };

  const isFailure = () => props.viewModel.outcome === "failure";
  const isPartial = () => props.viewModel.outcome === "partial";

  const heroHasCasualties = () =>
    props.viewModel.heroOutcomes.some((h) => h.status === "dead");

  return (
    <AppFrame
      eyebrow="Expedition Complete"
      title={props.viewModel.title}
      subtitle={`Outcome: ${outcomeLabel()}`}
    >
      <section class="grid">
        <div class="stack">
          <section class="panel stack">
            <div class="row">
              <span class="pill">Flow: result</span>
              <span class={`pill ${outcomeClass()}`}>{outcomeLabel()}</span>
            </div>
            <div class="surface-card stack">
              <h3>{props.viewModel.expeditionName}</h3>
              <p>{props.viewModel.summary}</p>
              {isFailure() && (
                <p class="danger" style="margin-top: 4px;">
                  The expedition has ended in defeat. Prepare your remaining forces before venturing forth again.
                </p>
              )}
              {isPartial() && (
                <p style="color: #e8a838; margin-top: 4px;">
                  The expedition achieved partial objectives. Tend to your heroes before the next venture.
                </p>
              )}
              {!isFailure() && !isPartial() && (
                <p style="color: #5bbd6e; margin-top: 4px;">
                  The expedition concluded successfully. Your heroes stand ready for the next challenge.
                </p>
              )}
            </div>
          </section>

          <section class="panel stack">
            <h2 class="panel-title">Hero Outcomes</h2>
            <ul class="list-reset">
              {props.viewModel.heroOutcomes.map((hero) => (
                <li class={heroCardClass(hero.status)}>
                  <div class="row">
                    <strong>{hero.heroName}</strong>
                    <span class={`pill ${heroStatusClass(hero.status)}`}>
                      {heroStatusLabel(hero.status)}
                    </span>
                    {hero.status === "dead" && (
                      <span class="pill" style="background: rgba(234,119,103,0.25); border-color: #ea7767; color: #ea7767;">
                        LOST
                      </span>
                    )}
                  </div>
                  <div class="row">
                    <span class="stat-label">HP</span>
                    <span class={`stat-value ${hero.status === "dead" ? "text-danger" : ""}`}>
                      {hero.status === "dead" ? "--" : hero.hpChange}
                    </span>
                    <span class="stat-label">Stress</span>
                    <span class={`stat-value ${hero.status === "dead" ? "text-danger" : ""}`}>
                      {hero.status === "dead" ? "--" : hero.stressChange}
                    </span>
                  </div>
                </li>
              ))}
            </ul>
          </section>

          {props.viewModel.lootAcquired.length > 0 && (
            <section class="panel stack">
              <h2 class="panel-title">Loot Acquired</h2>
              <ul class="list-reset">
                {props.viewModel.lootAcquired.map((item) => (
                  <li class="surface-card">
                    <span>{item}</span>
                  </li>
                ))}
              </ul>
            </section>
          )}
        </div>

        <div class="stack">
          <section class="panel stack">
            <h2 class="panel-title">Resources Gained</h2>
            <div class="surface-card stack">
              <div class="row">
                <span class="stat-label">Gold</span>
                <span class="stat-value">+{props.viewModel.resourcesGained.gold}</span>
              </div>
              <div class="row">
                <span class="stat-label">Supplies</span>
                <span class="stat-value">+{props.viewModel.resourcesGained.supplies}</span>
              </div>
              <div class="row">
                <span class="stat-label">Experience</span>
                <span class="stat-value">+{props.viewModel.resourcesGained.experience}</span>
              </div>
            </div>
          </section>

          <section class="panel stack">
            <div class="stack">
              {heroHasCasualties() && (
                <div class="surface-card" style="border-color: rgba(234,119,103,0.3);">
                  <p class="danger" style="margin: 0;">
                    <strong>Casualties sustained.</strong> Some heroes did not return. Visit the Stagecoach to recruit new party members.
                  </p>
                </div>
              )}
              <button
                class="action-primary"
                onClick={props.onContinue}
                disabled={!props.viewModel.isContinueAvailable}
              >
                Continue to Town
              </button>
              {props.onReturnToTown && (
                <button
                  class="action-secondary"
                  onClick={props.onReturnToTown}
                >
                  Return to Town (Fallback)
                </button>
              )}
            </div>
          </section>
        </div>
      </section>
    </AppFrame>
  );
};