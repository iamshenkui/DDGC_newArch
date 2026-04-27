import type { DdgcFrontendSnapshot } from "../bridge/contractTypes";

export type ScreenKey = "startup" | "loading" | "town" | "hero-detail" | "unsupported" | "fatal";

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

  if (snapshot.flowState === "town") {
    return "town";
  }

  return "startup";
}