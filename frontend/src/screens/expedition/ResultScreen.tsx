import type { Component } from "solid-js";

import type { ExpeditionResultViewModel } from "../../bridge/contractTypes";
import { AppFrame } from "../../components/layout/AppFrame";

interface ResultScreenProps {
  viewModel: ExpeditionResultViewModel;
  onContinue: () => void;
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
            </div>
          </section>

          <section class="panel stack">
            <h2 class="panel-title">Hero Outcomes</h2>
            <ul class="list-reset">
              {props.viewModel.heroOutcomes.map((hero) => (
                <li class="surface-card stack">
                  <div class="row">
                    <strong>{hero.heroName}</strong>
                    <span class="pill">{hero.status}</span>
                  </div>
                  <div class="row">
                    <span class="stat-label">HP</span>
                    <span class="stat-value">{hero.hpChange}</span>
                    <span class="stat-label">Stress</span>
                    <span class="stat-value">{hero.stressChange}</span>
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
              <button
                class="action-primary"
                onClick={props.onContinue}
                disabled={!props.viewModel.isContinueAvailable}
              >
                Continue to Town
              </button>
            </div>
          </section>
        </div>
      </section>
    </AppFrame>
  );
};