import type { Component } from "solid-js";
import { Match, Switch } from "solid-js";

import type { BuildingDetailViewModel } from "../../bridge/contractTypes";
import { BuildingDetailScreen } from "./BuildingDetailScreen";
import { StagecoachBuildingScreen } from "./buildings/StagecoachBuildingScreen";
import { GuildBuildingScreen } from "./buildings/GuildBuildingScreen";
import { BlacksmithBuildingScreen } from "./buildings/BlacksmithBuildingScreen";
import { SanitariumBuildingScreen } from "./buildings/SanitariumBuildingScreen";

interface BuildingScreenRouterProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
  onSwitchToForgeUse?: () => void;
}

export const BuildingScreenRouter: Component<BuildingScreenRouterProps> = (props) => {
  const buildingId = () => props.viewModel.buildingId;

  return (
    <Switch>
      <Match when={buildingId() === "stagecoach"}>
        <StagecoachBuildingScreen
          viewModel={props.viewModel}
          onReturn={props.onReturn}
          onAction={props.onAction}
        />
      </Match>
      <Match when={buildingId() === "guild"}>
        <GuildBuildingScreen
          viewModel={props.viewModel}
          onReturn={props.onReturn}
          onAction={props.onAction}
        />
      </Match>
      <Match when={buildingId() === "blacksmith"}>
        <BlacksmithBuildingScreen
          viewModel={props.viewModel}
          onReturn={props.onReturn}
          onAction={props.onAction}
          onSwitchToUse={props.onSwitchToForgeUse}
        />
      </Match>
      <Match when={buildingId() === "sanitarium"}>
        <SanitariumBuildingScreen
          viewModel={props.viewModel}
          onReturn={props.onReturn}
          onAction={props.onAction}
        />
      </Match>
      <Match when={true}>
        <BuildingDetailScreen
          viewModel={props.viewModel}
          onReturn={props.onReturn}
          onAction={props.onAction}
        />
      </Match>
    </Switch>
  );
};
