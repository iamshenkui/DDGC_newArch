import type { Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../bridge/contractTypes";
import { AppFrame } from "../../components/layout/AppFrame";

interface BuildingDetailScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

export const BuildingDetailScreen: Component<BuildingDetailScreenProps> = (props) => {
  const statusLabel = () => {
    switch (props.viewModel.status) {
      case "ready":
        return "Operational";
      case "partial":
        return "Partially Available";
      case "locked":
        return "Locked";
    }
  };

  const statusClass = () => {
    switch (props.viewModel.status) {
      case "ready":
        return "status-ready";
      case "partial":
        return "status-partial";
      case "locked":
        return "status-locked";
    }
  };

  return (
    <AppFrame
      eyebrow="Building Detail"
      title={props.viewModel.label}
      subtitle={`Status: ${statusLabel()}`}
    >
      <section class="grid">
        <div class="stack">
          <section class="panel stack">
            <h2 class="panel-title">Building Status</h2>
            <div class="surface-card stack">
              <div class="row">
                <span class="stat-label">Status</span>
                <span class={`stat-value ${statusClass()}`}>{statusLabel()}</span>
              </div>
              <div class="row">
                <span class="stat-label">Building</span>
                <span class="stat-value">{props.viewModel.label}</span>
              </div>
              {props.viewModel.upgradeRequirement && (
                <div class="row">
                  <span class="stat-label">Upgrade Requirement</span>
                  <span class="stat-value">{props.viewModel.upgradeRequirement}</span>
                </div>
              )}
            </div>
          </section>

          <section class="panel stack">
            <h2 class="panel-title">Description</h2>
            <div class="surface-card">
              <p>{props.viewModel.description}</p>
            </div>
          </section>
        </div>

        <div class="stack">
          <section class="panel stack">
            <h2 class="panel-title">Available Actions</h2>
            <ul class="list-reset">
              {props.viewModel.actions.map((action) => (
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
                    <span class="stat-value">{action.cost}</span>
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
        </div>
      </section>

      <div class="row">
        <button class="action-secondary" onClick={props.onReturn}>
          Return to Town
        </button>
      </div>
    </AppFrame>
  );
};