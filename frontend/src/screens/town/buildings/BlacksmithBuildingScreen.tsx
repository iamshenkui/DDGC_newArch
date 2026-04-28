import type { Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { AppFrame } from "../../../components/layout/AppFrame";

interface BlacksmithBuildingScreenProps {
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

export const BlacksmithBuildingScreen: Component<BlacksmithBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;
  const weaponActions = () => vm().actions.filter((a) => a.id.includes("weapon"));
  const armorActions = () => vm().actions.filter((a) => a.id.includes("armor"));
  const otherActions = () => vm().actions.filter(
    (a) => !a.id.includes("weapon") && !a.id.includes("armor")
  );

  return (
    <AppFrame
      eyebrow="Building — Blacksmith"
      title={vm().label}
      subtitle="Forge and upgrade weapons and armor for your heroes"
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
                  <span class="stat-label">Forge Level</span>
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
          {weaponActions().length > 0 && (
            <section class="panel stack">
              <h2 class="panel-title">Weapon Upgrades</h2>
              <ul class="list-reset">
                {weaponActions().map((action) => (
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

          {armorActions().length > 0 && (
            <section class="panel stack">
              <h2 class="panel-title">Armor Upgrades</h2>
              <ul class="list-reset">
                {armorActions().map((action) => (
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

          {otherActions().length > 0 && (
            <section class="panel stack">
              <h2 class="panel-title">Other Services</h2>
              <ul class="list-reset">
                {otherActions().map((action) => (
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
