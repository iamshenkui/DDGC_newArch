import type { Component } from "solid-js";

import type { ReturnViewModel } from "../../bridge/contractTypes";
import { AppFrame } from "../../components/layout/AppFrame";

interface ReturnScreenProps {
  viewModel: ReturnViewModel;
  onResumeTown: () => void;
  onReturnToTown?: () => void;
}

export const ReturnScreen: Component<ReturnScreenProps> = (props) => {
  return (
    <AppFrame
      eyebrow="Expedition Concluded"
      title={props.viewModel.title}
      subtitle={`Expedition: ${props.viewModel.expeditionName}`}
    >
      <section class="grid">
        <div class="stack">
          <section class="panel stack">
            <div class="row">
              <span class="pill">Flow: return</span>
              <span class="pill" style="color: #5bbd6e;">Closed</span>
            </div>
            <div class="surface-card stack">
              <h3>Expedition Concluded</h3>
              <p>{props.viewModel.summary}</p>
              <p style="color: var(--panel-muted); margin-top: 4px;">
                The expedition log has been closed. All surviving heroes have returned to the roster.
                Visit town buildings to tend to hero conditions and prepare for the next expedition.
              </p>
            </div>
          </section>

          <section class="panel stack">
            <h2 class="panel-title">Returning Heroes</h2>
            <ul class="list-reset">
              {props.viewModel.returningHeroes.map((hero) => (
                <li class="surface-card stack">
                  <div class="row">
                    <strong>{hero.heroName}</strong>
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
            <div class="stack">
              <button
                class="action-primary"
                onClick={props.onResumeTown}
                disabled={!props.viewModel.isTownResumeAvailable}
              >
                Resume Town Activities
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