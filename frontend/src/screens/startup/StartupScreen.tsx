import type { Component } from "solid-js";
import { resolveStartupAsset } from "../../assets/originalAssetPaths";

interface StartupScreenProps {
  onReplayBoot: () => void;
  onLiveBoot: () => void;
  onNewCampaign: () => void;
  onLoadCampaign: () => void;
  hasSavedCampaign: boolean;
}

/**
 * Title screen rebuilt from the original kuajiyuanqi (跨纪元契约)
 * CampaignSelection.unity scene. Phase 1 scope: background, frame,
 * title/logo, and landscape screen sizing only — boot buttons remain
 * as the existing runtime entry points without save/settings polish.
 *
 * Source assets (KUI-P1-003): scene_bg.png · dialog07.png · game_logo.png ·
 * btn_red.png · ZhiYiSongTi-Regular.ttf, all from CampaignSelection.unity.
 */
export const StartupScreen: Component<StartupScreenProps> = (props) => {
  const sceneBg = resolveStartupAsset("startupBackground");
  const titleWordmark = resolveStartupAsset("startupTitleWordmark");
  const dialogFrame = resolveStartupAsset("startupParchmentDialog");

  return (
    <main
      class="startup-screen"
      data-source-scene="Assets/Scenes/CampaignSelection.unity"
      style={{ "background-image": `url(${sceneBg})` }}
    >
      <div
        class="startup-stage"
        data-source-component="CampaignSelectionCanvas"
      >
        <h1
          class="startup-title"
          data-source-sprite="Assets/Sprites/ui/game_logo.png"
        >
          <img
            class="startup-title-wordmark"
            src={titleWordmark}
            alt="跨纪元契约"
            draggable={false}
          />
        </h1>

        <section
          class="startup-frame"
          data-source-sprite="Assets/Sprites/ui/dialog07.png"
          style={{ "background-image": `url(${dialogFrame})` }}
        >
          <nav
            class="startup-menu"
            data-source-component="MenuOptions"
            data-source-button-sprite="Assets/Sprites/ui/btn_red.png"
            aria-label="跨纪元契约 主菜单"
          >
            <button
              class="startup-menu-button"
              data-source-sprite="Assets/Sprites/ui/btn_red.png"
              type="button"
              onClick={props.onNewCampaign}
            >
              New Campaign
            </button>

            <button
              class="startup-menu-button"
              data-source-sprite="Assets/Sprites/ui/btn_red.png"
              type="button"
              onClick={props.onLoadCampaign}
              disabled={!props.hasSavedCampaign}
            >
              Load Campaign
            </button>

            <button
              class="startup-menu-button"
              data-source-sprite="Assets/Sprites/ui/btn_red.png"
              type="button"
              onClick={props.onReplayBoot}
            >
              Boot Replay
            </button>

            <button
              class="startup-menu-button"
              data-source-sprite="Assets/Sprites/ui/btn_red.png"
              type="button"
              onClick={props.onLiveBoot}
            >
              Boot Live
            </button>
          </nav>
        </section>
      </div>
    </main>
  );
};
