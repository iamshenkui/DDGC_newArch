import type { RuntimeBridge } from "../bridge/RuntimeBridge";
import type { DdgcFrontendIntent } from "../bridge/contractTypes";
import { canTransition } from "./FlowController";

export async function dispatchIntent(
  bridge: RuntimeBridge,
  intent: DdgcFrontendIntent
) {
  const snapshot = bridge.currentSnapshot();
  const validation = canTransition(snapshot, intent);
  if (!validation.allowed) {
    console.warn(`Intent ${intent.type} rejected: ${validation.reason}`);
    return snapshot;
  }
  return bridge.dispatchIntent(intent);
}
