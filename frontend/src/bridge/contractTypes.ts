import type { RuntimeMode } from "../app/runtimeMode";

export type FlowState =
  | "boot"
  | "load"
  | "town"
  | "expedition"
  | "combat"
  | "result"
  | "return";

export type FrontendLifecycle =
  | "booting"
  | "loading"
  | "ready"
  | "unsupported"
  | "fatal";

export interface BootLoadViewModel {
  kind: "boot-load";
  title: string;
  summary: string;
  mode: RuntimeMode;
}

export interface TownHeroSummary {
  id: string;
  name: string;
  classLabel: string;
  hp: string;
  stress: string;
  level: number;
}

export interface TownBuildingSummary {
  id: string;
  label: string;
  summary: string;
  status: "ready" | "partial" | "locked";
}

export interface TownViewModel {
  kind: "town";
  title: string;
  campaignName: string;
  campaignSummary: string;
  heroes: ReadonlyArray<TownHeroSummary>;
  buildings: ReadonlyArray<TownBuildingSummary>;
  nextActionLabel: string;
}

export interface UnsupportedViewModel {
  kind: "unsupported";
  title: string;
  reason: string;
}

export interface FatalErrorViewModel {
  kind: "fatal";
  title: string;
  reason: string;
}

export type DdgcViewModel =
  | BootLoadViewModel
  | TownViewModel
  | UnsupportedViewModel
  | FatalErrorViewModel;

export interface DdgcFrontendSnapshot {
  lifecycle: FrontendLifecycle;
  flowState: FlowState;
  viewModel: DdgcViewModel;
  debugMessage?: string;
}

export type DdgcFrontendIntent =
  | { type: "boot"; mode: RuntimeMode }
  | { type: "open-hero"; heroId: string }
  | { type: "open-building"; buildingId: string }
  | { type: "start-provisioning" }
  | { type: "launch-expedition" }
  | { type: "return-to-town" };