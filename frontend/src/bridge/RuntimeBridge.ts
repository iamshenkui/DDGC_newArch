import type { RuntimeMode } from "../app/runtimeMode";
import type {
  DdgcFrontendIntent,
  DdgcFrontendSnapshot
} from "./contractTypes";

export type RuntimeBridgeListener = (snapshot: DdgcFrontendSnapshot) => void;

export interface RuntimeBridge {
  readonly id: string;
  readonly mode: RuntimeMode;
  boot(): Promise<DdgcFrontendSnapshot>;
  currentSnapshot(): DdgcFrontendSnapshot;
  dispatchIntent(intent: DdgcFrontendIntent): Promise<DdgcFrontendSnapshot>;
  subscribe(listener: RuntimeBridgeListener): () => void;
}