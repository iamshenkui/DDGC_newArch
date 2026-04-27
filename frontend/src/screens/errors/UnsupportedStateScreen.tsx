import type { Component } from "solid-js";

import type { UnsupportedViewModel } from "../../bridge/contractTypes";
import { AppFrame } from "../../components/layout/AppFrame";

interface UnsupportedStateScreenProps {
  viewModel: UnsupportedViewModel;
  onReturn: () => void;
}

export const UnsupportedStateScreen: Component<UnsupportedStateScreenProps> = (
  props
) => {
  return (
    <AppFrame
      eyebrow="Unsupported State"
      title={props.viewModel.title}
      subtitle={props.viewModel.reason}
    >
      <section class="panel stack">
        <div class="surface-card stack">
          <h3>Why this is visible</h3>
          <p>
            Phase 10 requires unsupported and incomplete runtime paths to fail as
            explicit product surfaces, not as silent console-only errors.
          </p>
        </div>
        <div class="row">
          <button class="action-secondary" onClick={props.onReturn}>
            Return To Replay Shell
          </button>
        </div>
      </section>
    </AppFrame>
  );
};