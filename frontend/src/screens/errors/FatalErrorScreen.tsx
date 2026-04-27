import type { Component } from "solid-js";

import type { FatalErrorViewModel } from "../../bridge/contractTypes";
import { AppFrame } from "../../components/layout/AppFrame";

interface FatalErrorScreenProps {
  viewModel: FatalErrorViewModel;
  onReturn: () => void;
}

export const FatalErrorScreen: Component<FatalErrorScreenProps> = (props) => {
  return (
    <AppFrame
      eyebrow="Fatal Contract Surface"
      title={props.viewModel.title}
      subtitle={props.viewModel.reason}
    >
      <section class="panel stack">
        <p class="danger">
          Fatal contract drift or asset binding failures should stop the shell
          explicitly until the runtime/product boundary is repaired.
        </p>
        <div class="row">
          <button class="action-secondary" onClick={props.onReturn}>
            Return To Replay Shell
          </button>
        </div>
      </section>
    </AppFrame>
  );
};