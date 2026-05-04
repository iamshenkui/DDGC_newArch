import type { Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { AppFrame } from "../../../components/layout/AppFrame";

/**
 * Sanitarium building screen.
 *
 * References Unity prefab UI structure from:
 *   Assets/Prefabs/UI/TreatmentHeroSlot.prefab
 * Hierarchy mirrors: SanitariumWindow → LeftPanel → { Title (BuildingLabel,
 * BuildingDesc), Treatment slots } and RightPanel → { UpgradeButton, UpgradeWindow }
 *
 * Original sprite: Assets/Sprites/town/buildings/building_cell_repair.png
 * GUID: 55375034893560044a266e905926e8ff
 *
 * Treatment actions map to building upgrade trees in data/Buildings.json:
 *   quirk_positive_cost, quirk_negative_cost, quirk_treatment_chance,
 *   quirk_slots, disease_cost, disease_cure_all_chance, disease_slots
 */

interface SanitariumBuildingScreenProps {
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

export const SanitariumBuildingScreen: Component<SanitariumBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;

  // Categorise actions per the Sanitarium's Unity prefab treatment slots
  const quirkActions = () => vm().actions.filter(
    (a) => a.id.includes("quirk") || a.id.includes("positive") || a.id.includes("negative")
  );
  const diseaseActions = () => vm().actions.filter(
    (a) => a.id.includes("disease") || a.id.includes("cure")
  );
  const upgradeActions = () => vm().actions.filter(
    (a) => a.id.includes("slot") || a.id.includes("upgrade") || a.id.includes("treatment-chance")
  );

  return (
    <AppFrame
      eyebrow="Building — Sanitarium"
      title={vm().label}
      subtitle="Treat quirks, cure diseases, and restore your heroes"
      /* data-asset-path for future sprite replacement */
      /* data-asset-path="Assets/Sprites/town/buildings/building_cell_repair.png" */
    >
      <div class="grid">
        {/* ── Left Panel (mirrors SanitariumWindow/LeftPanel) ────── */}
        <div class="stack">
          <section class="panel stack">
            {/* Title → BuildingLabel + BuildingDesc */}
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
                  <span class="stat-label">Treatment Level</span>
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

        {/* ── Right Panel (mirrors SanitariumWindow/RightPanel) ───── */}
        <div class="stack">
          {/* Quirk Treatment (maps to quirk treatment tree nodes) */}
          {quirkActions().length > 0 && (
            <section class="panel stack">
              <h2 class="panel-title">Quirk Treatment</h2>
              <ul class="list-reset">
                {quirkActions().map((action) => (
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

          {/* Disease Treatment (maps to disease tree nodes) */}
          {diseaseActions().length > 0 && (
            <section class="panel stack">
              <h2 class="panel-title">Disease Treatment</h2>
              <ul class="list-reset">
                {diseaseActions().map((action) => (
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

          {/* Facility Upgrades (mirrors UpgradeWindow) */}
          {upgradeActions().length > 0 && (
            <section class="panel stack">
              <h2 class="panel-title">Facility Upgrades</h2>
              <ul class="list-reset">
                {upgradeActions().map((action) => (
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

          {/* Fallback: uncategorised actions */}
          {quirkActions().length === 0 && diseaseActions().length === 0 && upgradeActions().length === 0 && (
            <section class="panel stack">
              <h2 class="panel-title">Actions</h2>
              {vm().actions.length === 0 ? (
                <div class="surface-card">
                  <p>No actions currently available for this building.</p>
                </div>
              ) : (
                <ul class="list-reset">
                  {vm().actions.map((action) => (
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
              )}
            </section>
          )}
        </div>
      </div>

      {/* ── Return to Town (mirrors CloseButton pattern) ────────── */}
      <div class="row">
        <button class="action-secondary" onClick={props.onReturn}>
          Return to Town
        </button>
      </div>
    </AppFrame>
  );
};
