import { Match, Switch, createSignal } from "solid-js";

import { AppProviders } from "./AppProviders";
import { DEFAULT_RUNTIME_MODE, type RuntimeMode } from "./runtimeMode";
import { LiveRuntimeBridge } from "../bridge/LiveRuntimeBridge";
import { ReplayRuntimeBridge } from "../bridge/ReplayRuntimeBridge";
import type { RuntimeBridge } from "../bridge/RuntimeBridge";
import type {
  FatalErrorViewModel,
  TownViewModel,
  UnsupportedViewModel
} from "../bridge/contractTypes";
import { fatalSnapshot } from "../validation/replayFixtures";
import { createSessionStore } from "../session/SessionStore";
import { resolveScreen } from "../session/FlowController";
import { dispatchIntent } from "../session/intentDispatch";
import { FatalErrorScreen } from "../screens/errors/FatalErrorScreen";
import { UnsupportedStateScreen } from "../screens/errors/UnsupportedStateScreen";
import { StartupScreen } from "../screens/startup/StartupScreen";
import { TownShellScreen } from "../screens/town/TownShellScreen";

function createBridge(mode: RuntimeMode): RuntimeBridge {
  return mode === "live" ? new LiveRuntimeBridge() : new ReplayRuntimeBridge();
}

export function DdgcApp() {
  const session = createSessionStore(fatalSnapshot);
  const [booted, setBooted] = createSignal(false);
  const [activeMode, setActiveMode] = createSignal<RuntimeMode>(DEFAULT_RUNTIME_MODE);
  let bridge = createBridge(DEFAULT_RUNTIME_MODE);

  const runBoot = async (mode: RuntimeMode) => {
    setActiveMode(mode);
    bridge = createBridge(mode);
    const unsubscribe = bridge.subscribe((snapshot) => {
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

    unsubscribe();
  };

  const snapshot = session.snapshot();
  const screen = booted() ? resolveScreen(snapshot) : "startup";

  return (
    <AppProviders>
      <Switch>
        <Match when={screen === "startup"}>
          <StartupScreen
            onReplayBoot={() => runBoot("replay")}
            onLiveBoot={() => runBoot("live")}
          />
        </Match>
        <Match when={screen === "town" && snapshot.viewModel.kind === "town"}>
          <TownShellScreen
            viewModel={snapshot.viewModel as TownViewModel}
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
          when={screen === "unsupported" && snapshot.viewModel.kind === "unsupported"}
        >
          <UnsupportedStateScreen
            viewModel={snapshot.viewModel as UnsupportedViewModel}
            onReturn={() => {
              setBooted(false);
              setActiveMode(DEFAULT_RUNTIME_MODE);
            }}
          />
        </Match>
        <Match when={snapshot.viewModel.kind === "fatal"}>
          <FatalErrorScreen
            viewModel={snapshot.viewModel as FatalErrorViewModel}
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