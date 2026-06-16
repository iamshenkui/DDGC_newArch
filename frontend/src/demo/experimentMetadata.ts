/**
 * Experiment metadata record for the DDGC H5 demo.
 *
 * This record captures the experiment design, hypothesis, target
 * metrics, and qualification criteria so that anyone reviewing the
 * demo output or telemetry can understand what was being tested.
 *
 * The metadata is consumed by the telemetry system to annotate
 * captured events and by validation tests to verify coverage.
 */

export interface MetricDefinition {
  /** Human-readable label for the metric. */
  label: string;
  /** Longer description of what the metric measures. */
  description: string;
  /** The unit of measurement. */
  unit: string;
  /** Whether higher or lower values are better. */
  direction: "higher_is_better" | "lower_is_better" | "neutral";
}

/**
 * Structured experiment metadata that accompanies the H5 demo.
 */
export interface ExperimentMetadata {
  /** Unique identifier for the experiment. */
  experimentId: string;
  /** Human-readable demo/experiment name. */
  demoName: string;
  /** The hypothesis being tested. */
  hypothesis: string;
  /** Short description of the experiment scope. */
  description: string;
  /** Ordered list of metric keys this experiment tracks. */
  targetMetrics: string[];
  /** Definition for each tracked metric. */
  metrics: Record<string, MetricDefinition>;
  /** Criteria that qualify whether the experiment is valid. */
  qualificationCriteria: string[];
  /** ISO date when this metadata record was created. */
  createdAt: string;
}

/**
 * Canonical experiment metadata for the DDGC chaos-dungeon H5 demo
 * (experiment ddgc-h5-demo-001).
 */
export const DEMO_EXPERIMENT_METADATA: ExperimentMetadata = {
  experimentId: "ddgc-h5-demo-001",
  demoName: "Chaos Dungeon H5 Demo",
  hypothesis:
    "The isolated H5 chaos-dungeon demo provides a playable, " +
    "deterministic introduction to Darkest Dungeon chaos mechanics " +
    "without requiring the full game runtime.",
  description:
    "A self-contained mobile-friendly web demo that showcases the " +
    "chaos-meter dungeon loop: party management, event choices, " +
    "turn-based combat, and escalating chaos culminating in run " +
    "resolution.  All state transitions are deterministic.",
  targetMetrics: [
    "game_loads",
    "first_actions",
    "chaos_changes",
    "run_endings",
  ],
  metrics: {
    game_loads: {
      label: "Game Loads",
      description: "Number of times the demo screen mounted successfully.",
      unit: "count",
      direction: "neutral",
    },
    first_actions: {
      label: "First Actions",
      description: "Number of first user interactions after a run starts.",
      unit: "count",
      direction: "neutral",
    },
    chaos_changes: {
      label: "Chaos Changes",
      description: "Frequency and magnitude of chaos-meter changes.",
      unit: "count",
      direction: "neutral",
    },
    run_endings: {
      label: "Run Endings",
      description: "Run outcomes (victory, defeat, retreat, catastrophe, abandoned).",
      unit: "count",
      direction: "neutral",
    },
  },
  qualificationCriteria: [
    "All telemetry events carry a valid experiment_id matching the metadata.",
    "At least one telemetry event is emitted per demo session.",
    "Telemetry events include game_loaded, first_action, chaos_changed, and run_ended at appropriate lifecycle points.",
    "Telemetry collector makes no network calls.",
    "Existing app startup outside the demo path is unaffected.",
  ],
  createdAt: "2026-06-16",
};
