import type {
  DdgcFrontendSnapshot,
  DdgcFrontendIntent,
  ExpeditionResultViewModel,
  ReturnViewModel,
  TownViewModel,
  ProvisioningViewModel,
  ExpeditionSetupViewModel,
  HeroDetailViewModel,
  BuildingDetailViewModel
} from "../bridge/contractTypes";

export type ScreenKey = "startup" | "loading" | "town" | "hero-detail" | "building-detail" | "provisioning" | "expedition" | "result" | "return" | "unsupported" | "fatal";

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

  if (snapshot.viewModel.kind === "provisioning") {
    return "provisioning";
  }

  if (snapshot.viewModel.kind === "expedition") {
    return "expedition";
  }

  if (snapshot.viewModel.kind === "result") {
    return "result";
  }

  if (snapshot.viewModel.kind === "return") {
    return "return";
  }

  if (snapshot.flowState === "town") {
    return "town";
  }

  return "startup";
}

export interface TransitionValidation {
  allowed: boolean;
  reason?: string;
}

export function canTransition(
  snapshot: DdgcFrontendSnapshot,
  intent: DdgcFrontendIntent
): TransitionValidation {
  const screen = resolveScreen(snapshot);

  switch (intent.type) {
    case "continue-from-result":
      if (screen !== "result") {
        return { allowed: false, reason: "continue-from-result is only valid on result screen" };
      }
      if (snapshot.viewModel.kind !== "result") {
        return { allowed: false, reason: "viewModel is not a result view model" };
      }
      if (!snapshot.viewModel.isContinueAvailable) {
        return { allowed: false, reason: "continue is not available" };
      }
      return { allowed: true };

    case "resume-from-return":
      if (screen !== "return") {
        return { allowed: false, reason: "resume-from-return is only valid on return screen" };
      }
      if (snapshot.viewModel.kind !== "return") {
        return { allowed: false, reason: "viewModel is not a return view model" };
      }
      if (!snapshot.viewModel.isTownResumeAvailable) {
        return { allowed: false, reason: "town resume is not available" };
      }
      return { allowed: true };

    case "return-to-town":
      if (screen === "town" || screen === "startup" || screen === "loading") {
        return { allowed: false, reason: "already in town or transitioning" };
      }
      return { allowed: true };

    case "start-provisioning":
      if (screen !== "town") {
        return { allowed: false, reason: "start-provisioning is only valid in town" };
      }
      return { allowed: true };

    case "confirm-provisioning":
      if (screen !== "provisioning") {
        return { allowed: false, reason: "confirm-provisioning is only valid in provisioning" };
      }
      if (snapshot.viewModel.kind !== "provisioning") {
        return { allowed: false, reason: "viewModel is not a provisioning view model" };
      }
      if (!snapshot.viewModel.isReadyToLaunch) {
        return { allowed: false, reason: "not ready to launch expedition" };
      }
      return { allowed: true };

    case "launch-expedition":
      if (screen !== "expedition") {
        return { allowed: false, reason: "launch-expedition is only valid in expedition" };
      }
      if (snapshot.viewModel.kind !== "expedition") {
        return { allowed: false, reason: "viewModel is not an expedition view model" };
      }
      if (!snapshot.viewModel.isLaunchable) {
        return { allowed: false, reason: "expedition is not launchable" };
      }
      return { allowed: true };

    case "open-hero":
      if (screen !== "town") {
        return { allowed: false, reason: "open-hero is only valid in town" };
      }
      return { allowed: true };

    case "open-building":
      if (screen !== "town") {
        return { allowed: false, reason: "open-building is only valid in town" };
      }
      return { allowed: true };

    case "building-action":
      if (screen !== "building-detail") {
        return { allowed: false, reason: "building-action is only valid in building-detail" };
      }
      return { allowed: true };

    case "toggle-hero-selection":
      if (screen !== "provisioning") {
        return { allowed: false, reason: "toggle-hero-selection is only valid in provisioning" };
      }
      return { allowed: true };

    case "boot":
      return { allowed: true };

    default:
      return { allowed: true };
  }
}