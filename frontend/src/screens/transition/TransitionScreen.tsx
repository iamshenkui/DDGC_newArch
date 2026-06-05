import { type Component, onCleanup, onMount } from "solid-js";

import type { TransitionViewModel } from "../../bridge/contractTypes";

interface TransitionScreenProps {
  viewModel: TransitionViewModel;
  onDismiss: () => void;
  onReturn: () => void;
}

/**
 * Transition screen — cinematic interstitial between major game states.
 *
 * Mirrors the original DDGC region-transition window:
 *   Assets/Prefabs/UI/TransitionWindow.prefab (estimated)
 *
 * Layout (from reference image):
 *   - Full-bleed atmospheric background (ruined landscape / biome art)
 *   - Ornate circular emblem top-left (compass/region seal)
 *   - Region name centered near top (e.g. 朱雀大陆)
 *   - Flavor text bottom-right (descriptive atmosphere line)
 *   - Prompt bar bottom-center: press [Space] or click to continue
 *   - Back button bottom-left to return to provisioning
 *
 * Reference image:
 *   reference/ref_image/跨际元契约/1系统界面/转场.png
 */
export const TransitionScreen: Component<TransitionScreenProps> = (props) => {
  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.code === "Space" && props.viewModel.isDismissible) {
      e.preventDefault();
      props.onDismiss();
    }
  };

  onMount(() => {
    window.addEventListener("keydown", handleKeyDown);
  });

  onCleanup(() => {
    window.removeEventListener("keydown", handleKeyDown);
  });

  return (
    <div
      class="transition-viewport"
      data-source-scene="UI_Transition/TransitionWindow"
      data-source-prefab="Assets/Prefabs/UI/TransitionWindow.prefab"
      onClick={() => {
        if (props.viewModel.isDismissible) {
          props.onDismiss();
        }
      }}
    >
      {/* ── Atmospheric Background Layer ─────────────────── */}
      <div class="transition-backdrop" aria-hidden="true">
        <div class="transition-backdrop-scorch" />
        <div class="transition-backdrop-ruins" />
        <div class="transition-backdrop-vignette" />
      </div>

      {/* ── Top Ornament (compass / region seal) ─────────── */}
      <div class="transition-ornament" aria-hidden="true">
        <div class="transition-ornament-ring" />
        <div class="transition-ornament-core" />
        <div class="transition-ornament-spokes">
          <span />
          <span />
          <span />
          <span />
        </div>
      </div>

      {/* ── Center Title ─────────────────────────────────── */}
      <div class="transition-title-cluster">
        <div class="transition-title-rule" aria-hidden="true" />
        <h1 class="transition-title">{props.viewModel.regionName}</h1>
        <div class="transition-title-rule" aria-hidden="true" />
        {props.viewModel.subtitle && (
          <p class="transition-subtitle">{props.viewModel.subtitle}</p>
        )}
      </div>

      {/* ── Bottom Flavor Text ───────────────────────────── */}
      {props.viewModel.flavorText && (
        <div class="transition-flavor">
          <p class="transition-flavor-text">{props.viewModel.flavorText}</p>
        </div>
      )}

      {/* ── Bottom Prompt Bar ────────────────────────────── */}
      <div class="transition-prompt-bar">
        {props.viewModel.canReturn && (
          <button
            class="action-secondary transition-back-btn"
            onClick={(e) => {
              e.stopPropagation();
              props.onReturn();
            }}
          >
            Return
          </button>
        )}
        <span class="transition-prompt-hint">
          {props.viewModel.promptText}
        </span>
        <div class="transition-prompt-spacer" />
      </div>
    </div>
  );
};
