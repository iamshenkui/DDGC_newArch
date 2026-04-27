import type { Component } from "solid-js";

import type { BootLoadViewModel } from "../../bridge/contractTypes";
import { AppFrame } from "../../components/layout/AppFrame";

interface LoadingScreenProps {
  viewModel: BootLoadViewModel;
}

export const LoadingScreen: Component<LoadingScreenProps> = (props) => {
  return (
    <AppFrame
      eyebrow="Loading"
      title={props.viewModel.title}
      subtitle={props.viewModel.summary}
    >
      <section class="panel stack">
        <div class="surface-card stack">
          <h3>Runtime Initialization</h3>
          <p>
            The DDGC runtime is initializing in {props.viewModel.mode} mode.
            Please wait while assets and state are loaded.
          </p>
        </div>
        <div class="surface-card stack">
          <div class="loading-indicator">
            <span class="spinner" />
            <span>Loading...</span>
          </div>
        </div>
      </section>
    </AppFrame>
  );
};