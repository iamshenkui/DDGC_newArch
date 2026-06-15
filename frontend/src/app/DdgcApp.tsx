import { Match, Switch, createMemo, createSignal } from "solid-js";

import { DemoScreen } from "../screens/demo/DemoScreen";

import { AppProviders } from "./AppProviders";
import { DEFAULT_RUNTIME_MODE, type RuntimeMode } from "./runtimeMode";
import { LiveRuntimeBridge } from "../bridge/LiveRuntimeBridge";
import { ReplayRuntimeBridge } from "../bridge/ReplayRuntimeBridge";
import type { RuntimeBridge } from "../bridge/RuntimeBridge";
import type {
  BuildingDetailViewModel,
  CombatViewModel,
  DungeonAssistViewModel,
  DungeonHintViewModel,
  DungeonInteractionViewModel,
  DungeonMapViewModel,
  ExpeditionPlanningViewModel,
  ExpeditionResultViewModel,
  ExpeditionSetupViewModel,
  FatalErrorViewModel,
  HeroDetailViewModel,
  DungeonSelectViewModel,
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
import { DungeonSelectScreen } from "../screens/expedition/DungeonSelectScreen";
import { ExpeditionPlanningScreen } from "../screens/expedition/ExpeditionPlanningScreen";
import { ProvisioningScreen } from "../screens/expedition/ProvisioningScreen";
import { DungeonHintScreen } from "../screens/expedition/DungeonHintScreen";
import { ExpeditionScreen } from "../screens/expedition/ExpeditionScreen";
import { DungeonInteractionScreen } from "../screens/dungeon/DungeonInteractionScreen";
import { DungeonAssistScreen } from "../screens/dungeon/DungeonAssistScreen";
import { DungeonMapScreen } from "../screens/dungeon/DungeonMapScreen";
import { ResultScreen } from "../screens/expedition/ResultScreen";
import { ReturnScreen } from "../screens/expedition/ReturnScreen";
import { CombatScreen } from "../screens/combat/CombatScreen";

function createBridge(mode: RuntimeMode): RuntimeBridge {
  return mode === "live" ? new LiveRuntimeBridge() : new ReplayRuntimeBridge();
}

/**
 * Check for a `?demo=true` URL parameter to render the isolated chaos-dungeon demo
 * instead of the normal startup / session flow.
 */
const DEMO_URL_FLAG = "demo";

function hasDemoParam(): boolean {
  if (typeof window === "undefined") return false;
  const params = new URLSearchParams(window.location.search);
  return params.has(DEMO_URL_FLAG);
}

export function DdgcApp() {
  const [showDemo] = createSignal<boolean>(hasDemoParam());

  // When the demo param is present, render the isolated demo and skip the full app
  if (showDemo()) {
    return (
      <AppProviders>
        <DemoScreen />
      </AppProviders>
    );
  }

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
            onStartDungeonSelect={() => {
              void dispatchIntent(bridge, { type: "start-dungeon-select" });
            }}
            onStartProvisioning={() => {
              void dispatchIntent(bridge, { type: "start-expedition-planning" });
            }}
          />
        </Match>
        <Match
          when={screen() === "expedition-planning" && snapshot().viewModel.kind === "expedition-planning"}
        >
          <ExpeditionPlanningScreen
            viewModel={snapshot().viewModel as ExpeditionPlanningViewModel}
            onSelectPlane={(planeId) => {
              void dispatchIntent(bridge, { type: "select-plane", planeId });
            }}
            onToggleHero={(heroId) => {
              void dispatchIntent(bridge, { type: "toggle-planning-hero", heroId });
            }}
            onProceedToProvisioning={() => {
              void dispatchIntent(bridge, { type: "proceed-to-provisioning" });
            }}
            onReturnToTown={() => {
              void dispatchIntent(bridge, { type: "return-to-town" });
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
          when={screen() === "dungeon-select" && snapshot().viewModel.kind === "dungeon-select"}
        >
          <DungeonSelectScreen
            viewModel={snapshot().viewModel as DungeonSelectViewModel}
            onSelectDungeon={(dungeonId) => {
              void dispatchIntent(bridge, { type: "select-dungeon", dungeonId });
            }}
            onToggleHeroSelection={(heroId) => {
              void dispatchIntent(bridge, { type: "toggle-dungeon-hero", heroId });
            }}
            onConfirmSelection={() => {
              void dispatchIntent(bridge, { type: "confirm-dungeon-selection" });
            }}
            onReturnToTown={() => {
              void dispatchIntent(bridge, { type: "return-to-town" });
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
          when={screen() === "dungeon-hint" && snapshot().viewModel.kind === "dungeon-hint"}
        >
          <DungeonHintScreen
            viewModel={snapshot().viewModel as DungeonHintViewModel}
            onEnterDungeon={() => {
              void dispatchIntent(bridge, { type: "accept-dungeon-hint" });
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
          when={screen() === "dungeon-interaction" && snapshot().viewModel.kind === "dungeon-interaction"}
        >
          <DungeonInteractionScreen
            viewModel={snapshot().viewModel as DungeonInteractionViewModel}
            onProceed={() => {
              void dispatchIntent(bridge, { type: "proceed-dungeon" });
            }}
            onInteract={(interactionId) => {
              void dispatchIntent(bridge, { type: "interact-room", interactionId });
            }}
            onRetreat={() => {
              void dispatchIntent(bridge, { type: "retreat-dungeon" });
            }}
          />
        </Match>
        <Match
          when={screen() === "dungeon-assist" && snapshot().viewModel.kind === "dungeon-assist"}
        >
          <DungeonAssistScreen
            viewModel={snapshot().viewModel as DungeonAssistViewModel}
            onSelectHero={(heroId) => {
              void dispatchIntent(bridge, { type: "select-assist-hero", heroId });
            }}
            onUseAssistAction={(actionId) => {
              void dispatchIntent(bridge, { type: "use-assist-action", actionId });
            }}
            onContinue={() => {
              void dispatchIntent(bridge, { type: "continue-from-dungeon" });
            }}
            onReturnToTown={() => {
              void dispatchIntent(bridge, { type: "return-to-town" });
            }}
          />
        </Match>
        <Match
          when={screen() === "dungeon-map" && snapshot().viewModel.kind === "dungeon-map"}
        >
          <DungeonMapScreen
            viewModel={snapshot().viewModel as DungeonMapViewModel}
            onEnterRoom={(roomId) => {
              void dispatchIntent(bridge, { type: "enter-room", roomId });
            }}
            onRetreat={() => {
              void dispatchIntent(bridge, { type: "retreat-from-dungeon" });
            }}
            onCompleteDungeon={() => {
              void dispatchIntent(bridge, { type: "complete-dungeon" });
            }}
          />
        </Match>
        <Match
          when={screen() === "combat" && snapshot().viewModel.kind === "combat"}
        >
          <CombatScreen
            viewModel={snapshot().viewModel as CombatViewModel}
            onSelectSkill={(skillId) => {
              void dispatchIntent(bridge, { type: "select-skill", skillId });
            }}
            onSelectTarget={(enemyId) => {
              void dispatchIntent(bridge, { type: "select-target", enemyId });
            }}
            onConfirmAttack={() => {
              void dispatchIntent(bridge, { type: "confirm-attack" });
            }}
            onFleeCombat={() => {
              void dispatchIntent(bridge, { type: "flee-combat" });
            }}
            onEndTurn={() => {
              void dispatchIntent(bridge, { type: "end-turn" });
            }}
            onContinueCombat={() => {
              void dispatchIntent(bridge, { type: "continue-from-combat" });
            }}
            onOpenSettings={() => {
              void dispatchIntent(bridge, { type: "open-combat-settings" });
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
