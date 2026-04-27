import type { Component } from "solid-js";

import type { ProvisioningViewModel } from "../../bridge/contractTypes";
import { AppFrame } from "../../components/layout/AppFrame";

interface ProvisioningScreenProps {
  viewModel: ProvisioningViewModel;
  onToggleHeroSelection: (heroId: string) => void;
  onConfirmProvisioning: () => void;
  onReturnToTown: () => void;
}

export const ProvisioningScreen: Component<ProvisioningScreenProps> = (props) => {
  const selectedCount = () =>
    props.viewModel.party.filter((h) => h.isSelected).length;

  return (
    <AppFrame
      eyebrow="Provisioning"
      title={props.viewModel.title}
      subtitle={`Expedition: ${props.viewModel.expeditionLabel}`}
    >
      <section class="grid">
        <div class="stack">
          <section class="panel stack">
            <div class="row">
              <span class="pill">Flow: provisioning</span>
              <span class="pill">
                Party: {selectedCount()} / {props.viewModel.maxPartySize}
              </span>
            </div>
            <div class="surface-card stack">
              <h3>Party Selection</h3>
              <p>
                Select heroes to join the expedition. Review their status before
                departing.
              </p>
            </div>
            <ul class="list-reset">
              {props.viewModel.party.map((hero) => (
                <li class="surface-card stack">
                  <div class="row">
                    <label class="checkbox-label">
                      <input
                        type="checkbox"
                        checked={hero.isSelected}
                        onChange={() => props.onToggleHeroSelection(hero.id)}
                        disabled={
                          !hero.isSelected &&
                          selectedCount() >= props.viewModel.maxPartySize
                        }
                      />
                      <strong>{hero.name}</strong>
                    </label>
                    <span class="pill">{hero.classLabel}</span>
                    <span class="pill">Lv {hero.level}</span>
                  </div>
                  <div class="row">
                    <span class="stat-label">HP</span>
                    <span class="stat-value">{hero.hp}</span>
                    <span class="stat-label">Stress</span>
                    <span class="stat-value">{hero.stress}</span>
                  </div>
                </li>
              ))}
            </ul>
          </section>
        </div>

        <div class="stack">
          <section class="panel stack">
            <h2 class="panel-title">Expedition Details</h2>
            <div class="surface-card stack">
              <div class="row">
                <span class="stat-label">Expedition</span>
                <span class="stat-value">{props.viewModel.expeditionLabel}</span>
              </div>
              <div class="row">
                <span class="stat-label">Supply Level</span>
                <span class="stat-value">{props.viewModel.supplyLevel}</span>
              </div>
              <div class="row">
                <span class="stat-label">Provision Cost</span>
                <span class="stat-value">{props.viewModel.provisionCost}</span>
              </div>
            </div>
          </section>

          <section class="panel stack">
            <h2 class="panel-title">Summary</h2>
            <div class="surface-card">
              <p>{props.viewModel.expeditionSummary}</p>
            </div>
          </section>

          <section class="panel stack">
            <div class="row">
              <button
                class="action-secondary"
                onClick={props.onReturnToTown}
              >
                Return to Town
              </button>
              <button
                class="action-primary"
                onClick={props.onConfirmProvisioning}
                disabled={!props.viewModel.isReadyToLaunch}
              >
                {props.viewModel.isReadyToLaunch
                  ? "Confirm & Launch Expedition"
                  : "Select Party Members"}
              </button>
            </div>
          </section>
        </div>
      </section>
    </AppFrame>
  );
};