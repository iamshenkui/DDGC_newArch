import type { ParentComponent } from "solid-js";
import { RendererType } from "@contracts/ui-substrate";
import type { IPixiRenderer } from "@contracts/pixi-renderer";

interface PixiStageProps {
  label: string;
  rendererId?: string;
}

/**
 * Primary game viewport surface for town/meta scene rendering.
 *
 * In the current CSS-only phase this provides the landscape viewport shell
 * with a scenic estate background. When Pixi/Spine integration is wired,
 * the canvas layer hosts the actual runtime renderer.
 */
export const PixiStage: ParentComponent<PixiStageProps> = (props) => {
  const rendererType: RendererType = RendererType.PixiJS;
  const rendererContract: Pick<IPixiRenderer, "id"> | undefined = props.rendererId
    ? { id: props.rendererId }
    : undefined;

  return (
    <div
      class="game-surface"
      id={props.rendererId ?? "ddgc-stage-canvas"}
      data-renderer={rendererType}
      data-contract={rendererContract?.id ?? "ddgc-pixi-stage"}
    >
      <div class="game-surface-sky" />
      <div class="game-surface-landscape" />
      <div class="game-surface-buildings">
        {props.children}
      </div>
    </div>
  );
};
