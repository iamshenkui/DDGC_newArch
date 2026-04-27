import type { Component } from "solid-js";
import { RendererType } from "@contracts/ui-substrate";
import type { IPixiRenderer } from "@contracts/pixi-renderer";

interface PixiStageProps {
  label: string;
  rendererId?: string;
}

export const PixiStage: Component<PixiStageProps> = (props) => {
  const rendererType: RendererType = RendererType.PixiJS;
  const rendererContract: Pick<IPixiRenderer, "id"> | undefined = props.rendererId
    ? { id: props.rendererId }
    : undefined;

  return (
    <section class="panel stage-shell">
      <div class="row">
        <span class="pill">Renderer: {rendererType}</span>
        <span class="pill">
          Contract: {rendererContract?.id ?? "ddgc-pixi-stage-placeholder"}
        </span>
      </div>
      <div class="stage-canvas" id="ddgc-stage-canvas">
        <div>
          <strong>{props.label}</strong>
          <div>Reserved canvas layer for Pixi and Spine-backed town/meta rendering.</div>
        </div>
      </div>
    </section>
  );
};