import type { Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { AppFrame } from "../../../components/layout/AppFrame";

interface GuildBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

const statusLabel: Record<string, string> = {
  ready: "Operational",
  partial: "Partially Available",
  locked: "Locked",
};

const statusClass: Record<string, string> = {
  ready: "status-ready",
  partial: "status-partial",
  locked: "status-locked",
};

export const GuildBuildingScreen: Component<GuildBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;
  const trainingActions = () => vm().actions.filter((a) => a.id.startsWith("train-"));
  const equipmentActions = () => vm().actions.filter((a) => a.id.startsWith("upgrade-"));

  return (
    <AppFrame
      eyebrow="Building — Guild"
      title={vm().label}
      subtitle="Train skills and upgrade your heroes' equipment"
    >
      <div class="grid">
        <div class="stack">
          <section class="panel stack">
            <h2 class="panel-title">Building Status</h2>
            <div class="surface-card stack">
              <div class="row">
                <span class="stat-label">Status</span>
                <span class={`stat-value ${statusClass[vm().status]}`}>
                  {statusLabel[vm().status]}
                </span>
              </div>
              {vm().currentUpgrade && (
                <div class="row">
                  <span class="stat-label">Upgrade Level</span>
                  <span class="stat-value">{vm().currentUpgrade}</span>
                </div>
              )}
            </div>
          </section>

          <section class="panel stack">
            <h2 class="panel-title">Description</h2>
            <div class="surface-card">
              <p>{vm().description}</p>
            </div>
          </section>
        </div>

        <div class="stack">
          {trainingActions().length > 0 && (
            <section class="panel stack">
              <h2 class="panel-title">Skill Training</h2>
              <ul class="list-reset">
                {trainingActions().map((action) => (
                  <li class="surface-card stack">
                    <div class="row">
                      <strong class="action-label">{action.label}</strong>
                      {action.isUnsupported && (
                        <span class="pill pill-error">Unsupported</span>
                      )}
                      {!action.isAvailable && !action.isUnsupported && (
                        <span class="pill pill-warning">Unavailable</span>
                      )}
                    </div>
                    <p>{action.description}</p>
                    <div class="row">
                      <span class="stat-label">Cost</span>
                      <span class="stat-value action-cost">{action.cost}</span>
                    </div>
                    <div class="row">
                      {action.isUnsupported ? (
                        <button class="action-secondary" disabled>
                          Not Available
                        </button>
                      ) : action.isAvailable ? (
                        <button
                          class="action-primary"
                          onClick={() => props.onAction(action.id)}
                        >
                          {action.label}
                        </button>
                      ) : (
                        <button class="action-secondary" disabled>
                          Prerequisites Not Met
                        </button>
                      )}
                    </div>
                  </li>
                ))}
              </ul>
            </section>
          )}

          {equipmentActions().length > 0 && (
            <section class="panel stack">
              <h2 class="panel-title">Equipment Upgrades</h2>
              <ul class="list-reset">
                {equipmentActions().map((action) => (
                  <li class="surface-card stack">
                    <div class="row">
                      <strong class="action-label">{action.label}</strong>
                      {action.isUnsupported && (
                        <span class="pill pill-error">Unsupported</span>
                      )}
                      {!action.isAvailable && !action.isUnsupported && (
                        <span class="pill pill-warning">Unavailable</span>
                      )}
                    </div>
                    <p>{action.description}</p>
                    <div class="row">
                      <span class="stat-label">Cost</span>
                      <span class="stat-value action-cost">{action.cost}</span>
                    </div>
                    <div class="row">
                      {action.isUnsupported ? (
                        <button class="action-secondary" disabled>
                          Not Available
                        </button>
                      ) : action.isAvailable ? (
                        <button
                          class="action-primary"
                          onClick={() => props.onAction(action.id)}
                        >
                          {action.label}
                        </button>
                      ) : (
                        <div class="stack">
                          <button class="action-secondary" disabled>
                            Prerequisites Not Met
                          </button>
                          {vm().upgradeRequirement && (
                            <span class="pill pill-info">{vm().upgradeRequirement}</span>
                          )}
                        </div>
                      )}
                    </div>
                  </li>
                ))}
              </ul>
            </section>
          )}
        </div>
      </div>

      <div class="row">
        <button class="action-secondary" onClick={props.onReturn}>
          Return to Town
        </button>
      </div>
    </AppFrame>
  );
};
