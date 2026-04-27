import { createSignal } from "solid-js";

import type { DdgcFrontendSnapshot } from "../bridge/contractTypes";
import { fatalSnapshot } from "../validation/replayFixtures";

export function createSessionStore(initialSnapshot: DdgcFrontendSnapshot) {
  const [snapshot, setSnapshot] = createSignal(initialSnapshot);

  return {
    snapshot,
    replace(nextSnapshot: DdgcFrontendSnapshot) {
      setSnapshot(nextSnapshot);
    },
    fail(reason: string) {
      setSnapshot({
        ...fatalSnapshot,
        viewModel: {
          ...fatalSnapshot.viewModel,
          reason
        }
      });
    }
  };
}