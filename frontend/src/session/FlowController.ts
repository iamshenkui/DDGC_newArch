import type { DdgcFrontendSnapshot } from "../bridge/contractTypes";

export type ScreenKey = "startup" | "loading" | "town" | "hero-detail" | "building-detail" | "unsupported" | "fatal";

export function resolveScreen(snapshot: DdgcFrontendSnapshot): ScreenKey {
  if (snapshot.lifecycle === "fatal") {
    return "fatal";
  }

  if (snapshot.lifecycle === "unsupported") {
    return "unsupported";
  }

  if (snapshot.lifecycle === "loading" || snapshot.lifecycle === "booting") {
    return "loading";
  }

  if (snapshot.viewModel.kind === "hero-detail") {
    return "hero-detail";
  }

  if (snapshot.viewModel.kind === "building-detail") {
    return "building-detail";
  }

  if (snapshot.flowState === "town") {
    return "town";
  }

  return "startup";
}