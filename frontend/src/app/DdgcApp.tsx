import { Match, Switch, createMemo, createSignal } from "solid-js";

import { AppProviders } from "./AppProviders";
import { DEFAULT_RUNTIME_MODE, type RuntimeMode } from "./runtimeMode";
import { LiveRuntimeBridge } from "../bridge/LiveRuntimeBridge";
import { ReplayRuntimeBridge } from "../bridge/ReplayRuntimeBridge";
import type { RuntimeBridge } from "../bridge/RuntimeBridge";
import type {
  BuildingDetailViewModel,
  ExpeditionResultViewModel,
  ExpeditionSetupViewModel,
  FatalErrorViewModel,
  HeroDetailViewModel,
  ProvisioningViewModel,
  ReturnViewModel,
  TownViewModel,
  UnsupportedViewModel
} from "../bridge/contractTypes";
import { fatalSnapshot } from "../validation/replayFixtures";
import { createSaveLoadService, createFreshCampaignSnapshot } from "../session/SaveLoadService";
import { createSessionStore } from "../session/SessionStore";
import { resolveScreen } from "../session/FlowController";
import { dispatchIntent } from "../session/intentDispatch";
import { FatalErrorScreen } from "../screens/errors/FatalErrorScreen";
import { UnsupportedStateScreen } from "../screens/errors/UnsupportedStateScreen";
import { StartupScreen } from "../screens/startup/StartupScreen";
import { TownShellScreen } from "../screens/town/TownShellScreen";
import { HeroDetailScreen } from "../screens/town/HeroDetailScreen";
import { BuildingScreenRouter } from "../screens/town/BuildingScreenRouter";
import { ProvisioningScreen } from "../screens/expedition/ProvisioningScreen";
import { ExpeditionScreen } from "../screens/expedition/ExpeditionScreen";
import { ResultScreen } from "../screens/expedition/ResultScreen";
import { ReturnScreen } from "../screens/expedition/ReturnScreen";

function createBridge(mode: RuntimeMode): RuntimeBridge {
  return mode === "live" ? new LiveRuntimeBridge() : new ReplayRuntimeBridge();
}

export function DdgcApp() {
  const session = createSessionStore(fatalSnapshot);
  const [booted, setBooted] = createSignal(false);
  const [activeMode, setActiveMode] = createSignal<RuntimeMode>(DEFAULT_RUNTIME_MODE);
  const saveLoad = createSaveLoadService(activeMode());
  let bridge = createBridge(DEFAULT_RUNTIME_MODE);
  let unsubscribeBridge: (() => void) | null = null;

  const runBoot = async (mode: RuntimeMode) => {
    // Clean up previous bridge subscription before replacing the bridge
    if (unsubscribeBridge) {
      unsubscribeBridge();
      unsubscribeBridge = null;
    }

    setActiveMode(mode);
    bridge = createBridge(mode);
    unsubscribeBridge = bridge.subscribe((snapshot) => {
      session.replace(snapshot);
    });

    try {
      const snapshot = await bridge.boot();
      session.replace(snapshot);
      setBooted(true);
    } catch (error) {
      session.fail(error instanceof Error ? error.message : "boot failed");
      setBooted(true);
    }
    // Keep subscription alive so subsequent dispatchIntent calls propagate
    // to the session store and trigger re-renders.
  };

  const handleNewCampaign = () => {
    const snapshot = createFreshCampaignSnapshot("replay");
    session.replace(snapshot);
    void runBoot("replay");
  };

  const handleLoadCampaign = () => {
    const saved = saveLoad.load();
    if (saved) {
      session.replace(saved);
      void runBoot("replay");
    } else {
      handleNewCampaign();
    }
  };

  const snapshot = createMemo(() => session.snapshot());
  const screen = createMemo(() => (booted() ? resolveScreen(snapshot()) : "startup"));

  return (
    <AppProviders>
      <Switch>
        <Match when={screen() === "startup"}>
          <StartupScreen
            onReplayBoot={() => runBoot("replay")}
            onLiveBoot={() => runBoot("live")}
            onNewCampaign={handleNewCampaign}
            onLoadCampaign={handleLoadCampaign}
            hasSavedCampaign={saveLoad.hasSavedCampaign()}
          />
        </Match>
        <Match when={screen() === "town" && snapshot().viewModel.kind === "town"}>
          <TownShellScreen
            viewModel={snapshot().viewModel as TownViewModel}
            onOpenHero={(heroId) => {
              void dispatchIntent(bridge, { type: "open-hero", heroId });
            }}
            onOpenBuilding={(buildingId) => {
              void dispatchIntent(bridge, { type: "open-building", buildingId });
            }}
            onStartProvisioning={() => {
              void dispatchIntent(bridge, { type: "start-provisioning" });
            }}
          />
        </Match>
        <Match
          when={screen() === "hero-detail" && snapshot().viewModel.kind === "hero-detail"}
        >
          <HeroDetailScreen
            viewModel={snapshot().viewModel as HeroDetailViewModel}
            onReturn={() => {
              void dispatchIntent(bridge, { type: "return-to-town" });
            }}
          />
        </Match>
        <Match
          when={screen() === "building-detail" && snapshot().viewModel.kind === "building-detail"}
        >
          <BuildingScreenRouter
            viewModel={snapshot().viewModel as BuildingDetailViewModel}
            onReturn={() => {
              void dispatchIntent(bridge, { type: "return-to-town" });
            }}
            onAction={(actionId) => {
              void dispatchIntent(bridge, { type: "building-action", actionId });
            }}
          />
        </Match>
        <Match
          when={screen() === "provisioning" && snapshot().viewModel.kind === "provisioning"}
        >
          <ProvisioningScreen
            viewModel={snapshot().viewModel as ProvisioningViewModel}
            onToggleHeroSelection={(heroId) => {
              void dispatchIntent(bridge, { type: "toggle-hero-selection", heroId });
            }}
            onConfirmProvisioning={() => {
              void dispatchIntent(bridge, { type: "confirm-provisioning" });
            }}
            onReturnToTown={() => {
              void dispatchIntent(bridge, { type: "return-to-town" });
            }}
          />
        </Match>
        <Match
          when={screen() === "expedition" && snapshot().viewModel.kind === "expedition"}
        >
          <ExpeditionScreen
            viewModel={snapshot().viewModel as ExpeditionSetupViewModel}
            onLaunchExpedition={() => {
              void dispatchIntent(bridge, { type: "launch-expedition" });
            }}
            onReturnToTown={() => {
              void dispatchIntent(bridge, { type: "return-to-town" });
            }}
          />
        </Match>
        <Match
          when={screen() === "result" && snapshot().viewModel.kind === "result"}
        >
          <ResultScreen
            viewModel={snapshot().viewModel as ExpeditionResultViewModel}
            onContinue={() => {
              void dispatchIntent(bridge, { type: "continue-from-result" });
            }}
            onReturnToTown={() => {
              void dispatchIntent(bridge, { type: "return-to-town" });
            }}
          />
        </Match>
        <Match
          when={screen() === "return" && snapshot().viewModel.kind === "return"}
        >
          <ReturnScreen
            viewModel={snapshot().viewModel as ReturnViewModel}
            onResumeTown={() => {
              void dispatchIntent(bridge, { type: "resume-from-return" });
            }}
            onReturnToTown={() => {
              void dispatchIntent(bridge, { type: "return-to-town" });
            }}
          />
        </Match>
        <Match
          when={screen() === "unsupported" && snapshot().viewModel.kind === "unsupported"}
        >
          <UnsupportedStateScreen
            viewModel={snapshot().viewModel as UnsupportedViewModel}
            onReturn={() => {
              setBooted(false);
              setActiveMode(DEFAULT_RUNTIME_MODE);
            }}
          />
        </Match>
        <Match when={snapshot().viewModel.kind === "fatal"}>
          <FatalErrorScreen
            viewModel={snapshot().viewModel as FatalErrorViewModel}
            onReturn={() => {
              setBooted(false);
              setActiveMode(DEFAULT_RUNTIME_MODE);
            }}
          />
        </Match>
      </Switch>
      <div style={{ display: "none" }}>{activeMode()}</div>
    </AppProviders>
  );
}