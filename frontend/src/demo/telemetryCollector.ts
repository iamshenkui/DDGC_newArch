/**
 * In-memory telemetry collector for the H5 demo.
 *
 * Stores events in a local array.  No network calls are made — this is
 * a purely local collector intended for experiment observability and
 * automated validation.
 *
 * The collector is a plain class so tests can instantiate isolated
 * instances.  In the DemoDisplay component a singleton is created
 * per component mount.
 */

import type { TelemetryEvent } from "./telemetry";

export class TelemetryCollector {
  /** Ordered list of captured events (newest appended). */
  private events: TelemetryEvent[] = [];

  /**
   * Record a telemetry event.
   * No side-effects beyond appending to the internal buffer.
   */
  capture(event: TelemetryEvent): void {
    this.events.push(event);
  }

  /**
   * Return a shallow copy of all captured events.
   * The caller may safely iterate or test the array.
   */
  getEvents(): TelemetryEvent[] {
    return [...this.events];
  }

  /**
   * Return events matching one or more event_type values.
   */
  getEventsByType(...types: string[]): TelemetryEvent[] {
    return this.events.filter((e) => types.includes(e.event_type));
  }

  /**
   * Return the number of captured events.
   */
  getEventCount(): number {
    return this.events.length;
  }

  /**
   * Clear all captured events — useful between runs in tests or
   * when resetting the demo state.
   */
  clear(): void {
    this.events = [];
  }
}
