import type { RuntimeBridge } from "../bridge/RuntimeBridge";
import type { DdgcFrontendIntent } from "../bridge/contractTypes";

export async function dispatchIntent(
  bridge: RuntimeBridge,
  intent: DdgcFrontendIntent
) {
  return bridge.dispatchIntent(intent);
}