import type { Component } from "solid-js";

import { MenuFrame } from "../../components/layout/MenuFrame";

interface StartupScreenProps {
  onReplayBoot: () => void;
  onLiveBoot: () => void;
  onNewCampaign: () => void;
  onLoadCampaign: () => void;
  hasSavedCampaign: boolean;
}

export const StartupScreen: Component<StartupScreenProps> = (props) => {
  return (
    <MenuFrame gameTitle="DDGC" gameSubtitle="暗黑地牢: 降龙">
      <button class="menu-button" onClick={props.onNewCampaign}>
        New Campaign
      </button>
      <button
        class="menu-button"
        onClick={props.onLoadCampaign}
        disabled={!props.hasSavedCampaign}
      >
        Load Campaign
      </button>
      <button class="menu-button" onClick={props.onReplayBoot}>
        Boot Replay
      </button>
      <button class="menu-button" onClick={props.onLiveBoot}>
        Boot Live
      </button>
    </MenuFrame>
  );
};
