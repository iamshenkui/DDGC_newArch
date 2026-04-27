import type { ParentComponent } from "solid-js";
import type { IAppOptions } from "@contracts/app-shell";

const DEFAULT_APP_OPTIONS: IAppOptions = {
  canvasId: "ddgc-stage-canvas",
  viewportWidth: 1440,
  viewportHeight: 900,
  debug: true,
  entryModule: "ddgc-product-shell"
};

export const AppProviders: ParentComponent = (props) => {
  return (
    <div class="app-shell" data-canvas-id={DEFAULT_APP_OPTIONS.canvasId}>
      {props.children}
    </div>
  );
};