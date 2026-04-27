import type { Component } from "solid-js";

import { AppFrame } from "../../components/layout/AppFrame";

interface StartupScreenProps {
  onReplayBoot: () => void;
  onLiveBoot: () => void;
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
          <h2 class="panel-title">Boot Path</h2>
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