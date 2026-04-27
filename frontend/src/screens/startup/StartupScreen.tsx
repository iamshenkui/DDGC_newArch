import type { Component } from "solid-js";

import { AppFrame } from "../../components/layout/AppFrame";

interface StartupScreenProps {
  onReplayBoot: () => void;
  onLiveBoot: () => void;
  onNewCampaign: () => void;
  onLoadCampaign: () => void;
  hasSavedCampaign: boolean;
}

export const StartupScreen: Component<StartupScreenProps> = (props) => {
  return (
    <AppFrame
      eyebrow="Phase 10 Frontend Skeleton"
      title="DDGC Rendered Frontend"
      subtitle="Boot the product-owned frontend shell through replay mode first. Live mode stays behind the runtime bridge seam until real integration lands."
    >
      <section class="grid">
        <div class="panel stack">
          <h2 class="panel-title">Campaign Entry</h2>
          <div class="surface-card stack">
            <h3>New Campaign</h3>
            <p>
              Start a fresh campaign with default roster and initial town state.
              Uses stable replay fixtures for development.
            </p>
            <button class="action-primary" onClick={props.onNewCampaign}>
              New Campaign
            </button>
          </div>
          <div class="surface-card stack">
            <h3>Load Campaign</h3>
            <p>
              {props.hasSavedCampaign
                ? "Continue a saved campaign from local storage."
                : "No saved campaign found. Start a new campaign to begin."}
            </p>
            <button
              class="action-secondary"
              onClick={props.onLoadCampaign}
              disabled={!props.hasSavedCampaign}
            >
              Load Campaign
            </button>
          </div>
        </div>
        <div class="panel stack">
          <h2 class="panel-title">Runtime Boot Options</h2>
          <div class="surface-card stack">
            <h3>Replay-first shell</h3>
            <p>
              Uses stable fixtures and DDGC view-model placeholders so the rendered
              UI can evolve without touching gameplay truth.
            </p>
          </div>
          <div class="surface-card stack">
            <h3>Live bridge seam</h3>
            <p>
              The live bridge exists as a boundary, but returns an explicit
              unsupported surface until runtime wiring is ready.
            </p>
          </div>
          <div class="row">
            <button class="action-primary" onClick={props.onReplayBoot}>
              Boot Replay Shell
            </button>
            <button class="action-secondary" onClick={props.onLiveBoot}>
              Boot Live Shell
            </button>
          </div>
        </div>
        <div class="panel stack">
          <h2 class="panel-title">Guardrails</h2>
          <ul class="list-reset">
            <li class="surface-card">No frontend import reaches into private Rust runtime files.</li>
            <li class="surface-card">DDGC product screens stay in the DDGC product workspace.</li>
            <li class="surface-card">Gameplay truth remains in Rust; frontend sends intents only.</li>
          </ul>
        </div>
      </section>
    </AppFrame>
  );
};