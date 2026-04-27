import { Match, Switch, createSignal, createEffect } from "solid-js";

import { AppProviders } from "./AppProviders";
import { DEFAULT_RUNTIME_MODE, type RuntimeMode } from "./runtimeMode";
import { LiveRuntimeBridge } from "../bridge/LiveRuntimeBridge";
import { ReplayRuntimeBridge } from "../bridge/ReplayRuntimeBridge";
import type { RuntimeBridge } from "../bridge/RuntimeBridge";
import type {
  BootLoadViewModel,
  FatalErrorViewModel,
  HeroDetailViewModel,
  TownViewModel,
  UnsupportedViewModel
} from "../bridge/contractTypes";
import {
  fatalSnapshot,
  replayLoadingSnapshot,
  liveLoadingSnapshot
} from "../validation/replayFixtures";
import { createSessionStore } from "../session/SessionStore";
import {
  resolveScreen,
  type ScreenKey
} from "../session/FlowController";
import { dispatchIntent } from "../session/intentDispatch";
import {
  createSaveLoadService,
  createFreshCampaignSnapshot,
  type SaveLoadService
} from "../session/SaveLoadService";
import { FatalErrorScreen } from "../screens/errors/FatalErrorScreen";
import { LoadingScreen } from "../screens/loading/LoadingScreen";
import { UnsupportedStateScreen } from "../screens/errors/UnsupportedStateScreen";
import { StartupScreen } from "../screens/startup/StartupScreen";
import { TownShellScreen } from "../screens/town/TownShellScreen";
import { HeroDetailScreen } from "../screens/town/HeroDetailScreen";

function createBridge(mode: RuntimeMode): RuntimeBridge {
  return mode === "live" ? new LiveRuntimeBridge() : new ReplayRuntimeBridge();
}

function createLoadingSnapshot(mode: RuntimeMode): typeof replayLoadingSnapshot | typeof liveLoadingSnapshot {
  return mode === "replay" ? replayLoadingSnapshot : liveLoadingSnapshot;
}

export function DdgcApp() {
  const session = createSessionStore(fatalSnapshot);
  const [booted, setBooted] = createSignal(false);
  const [activeMode, setActiveMode] = createSignal<RuntimeMode>(DEFAULT_RUNTIME_MODE);
  const [saveLoadService, setSaveLoadService] = createSignal<SaveLoadService>(
    createSaveLoadService("replay")
  );

  let bridge = createBridge(DEFAULT_RUNTIME_MODE);

  const runBoot = async (mode: RuntimeMode) => {
    setActiveMode(mode);
    bridge = createBridge(mode);
    const newSaveLoadService = createSaveLoadService(mode);
    setSaveLoadService(newSaveLoadService);

    // Show loading screen first
    const loadingSnap = createLoadingSnapshot(mode);
    session.replace(loadingSnap);
    setBooted(false);

    const unsubscribe = bridge.subscribe((snapshot) => {
      session.replace(snapshot);
    });

    try {
      const snapshot = await bridge.boot();
      // Auto-save the fresh campaign state
      newSaveLoadService.save(snapshot);
      session.replace(snapshot);
      setBooted(true);
    } catch (error) {
      session.fail(error instanceof Error ? error.message : "boot failed");
      setBooted(true);
    }

    unsubscribe();
  };

  const runNewCampaign = async () => {
    const mode: RuntimeMode = "replay";
    setActiveMode(mode);
    bridge = createBridge(mode);
    const newSaveLoadService = createSaveLoadService(mode);
    setSaveLoadService(newSaveLoadService);

    // Show loading screen first
    session.replace(replayLoadingSnapshot);
    setBooted(false);

    const unsubscribe = bridge.subscribe((snapshot) => {
      session.replace(snapshot);
    });

    try {
      const snapshot = await bridge.boot();
      // Auto-save the fresh campaign state
      newSaveLoadService.save(snapshot);
      session.replace(snapshot);
      setBooted(true);
    } catch (error) {
      session.fail(error instanceof Error ? error.message : "campaign boot failed");
      setBooted(true);
    }

    unsubscribe();
  };

  const runLoadCampaign = async () => {
    const mode: RuntimeMode = "replay";
    setActiveMode(mode);
    bridge = createBridge(mode);
    const loadedSaveLoadService = createSaveLoadService(mode);
    setSaveLoadService(loadedSaveLoadService);

    // Show loading screen first
    session.replace(replayLoadingSnapshot);
    setBooted(false);

    const unsubscribe = bridge.subscribe((snapshot) => {
      session.replace(snapshot);
    });

    try {
      // Try to load saved snapshot
      const savedSnapshot = loadedSaveLoadService.load();
      if (savedSnapshot) {
        session.replace(savedSnapshot);
        setBooted(true);
      } else {
        // No saved campaign found, start fresh
        const snapshot = await bridge.boot();
        loadedSaveLoadService.save(snapshot);
        session.replace(snapshot);
        setBooted(true);
      }
    } catch (error) {
      session.fail(error instanceof Error ? error.message : "load campaign failed");
      setBooted(true);
    }

    unsubscribe();
  };

  const snapshot = session.snapshot();
  const screen = booted() ? resolveScreen(snapshot) : "startup";

  return (
    <AppProviders>
      <Switch>
        <Match when={screen === "loading" && snapshot.viewModel.kind === "boot-load"}>
          <LoadingScreen viewModel={snapshot.viewModel as BootLoadViewModel} />
        </Match>
        <Match when={screen === "startup"}>
          <StartupScreen
            onReplayBoot={() => runBoot("replay")}
            onLiveBoot={() => runBoot("live")}
            onNewCampaign={runNewCampaign}
            onLoadCampaign={runLoadCampaign}
            hasSavedCampaign={saveLoadService().hasSavedCampaign()}
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
        <Match when={screen === "hero-detail" && snapshot.viewModel.kind === "hero-detail"}>
          <HeroDetailScreen
            viewModel={snapshot.viewModel as HeroDetailViewModel}
            onReturn={() => {
              void dispatchIntent(bridge, { type: "return-to-town" });
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