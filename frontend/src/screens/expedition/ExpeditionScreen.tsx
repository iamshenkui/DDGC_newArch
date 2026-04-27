import type { Component } from "solid-js";

import type { ExpeditionSetupViewModel } from "../../bridge/contractTypes";
import { AppFrame } from "../../components/layout/AppFrame";

interface ExpeditionScreenProps {
  viewModel: ExpeditionSetupViewModel;
  onLaunchExpedition: () => void;
  onReturnToTown: () => void;
}

export const ExpeditionScreen: Component<ExpeditionScreenProps> = (props) => {
  return (
    <AppFrame
      eyebrow="Expedition Launch"
      title={props.viewModel.title}
      subtitle={`Difficulty: ${props.viewModel.difficulty}`}
    >
      <section class="grid">
        <div class="stack">
          <section class="panel stack">
            <div class="row">
              <span class="pill">Flow: expedition</span>
              <span class="pill">Party: {props.viewModel.partySize} heroes</span>
            </div>
            <div class="surface-card stack">
              <h3>Expedition Ready</h3>
              <p>
                Your party is provisioned and ready to depart. Review the
                expedition details before launching.
              </p>
            </div>
          </section>

          <section class="panel stack">
            <h2 class="panel-title">Expedition Details</h2>
            <div class="surface-card stack">
              <div class="row">
                <span class="stat-label">Expedition</span>
                <span class="stat-value">{props.viewModel.expeditionName}</span>
              </div>
              <div class="row">
                <span class="stat-label">Party Size</span>
                <span class="stat-value">{props.viewModel.partySize}</span>
              </div>
              <div class="row">
                <span class="stat-label">Difficulty</span>
                <span class="stat-value">{props.viewModel.difficulty}</span>
              </div>
              <div class="row">
                <span class="stat-label">Est. Duration</span>
                <span class="stat-value">{props.viewModel.estimatedDuration}</span>
              </div>
            </div>
          </section>

          <section class="panel stack">
            <h2 class="panel-title">Objectives</h2>
            <ul class="list-reset">
              {props.viewModel.objectives.map((objective) => (
                <li class="surface-card">
                  <span>{objective}</span>
                </li>
              ))}
            </ul>
          </section>

          {props.viewModel.warnings.length > 0 && (
            <section class="panel stack">
              <h2 class="panel-title">Warnings</h2>
              <ul class="list-reset">
                {props.viewModel.warnings.map((warning) => (
                  <li class="surface-card warning-card">
                    <span class="warning-text">{warning}</span>
                  </li>
                ))}
              </ul>
            </section>
          )}
        </div>

        <div class="stack">
          <section class="panel stack">
            <div class="stack">
              <button
                class="action-primary"
                onClick={props.onLaunchExpedition}
                disabled={!props.viewModel.isLaunchable}
              >
                Launch Expedition
              </button>
              <button
                class="action-secondary"
                onClick={props.onReturnToTown}
              >
                Return to Town
              </button>
            </div>
          </section>
        </div>
      </section>
    </AppFrame>
  );
};