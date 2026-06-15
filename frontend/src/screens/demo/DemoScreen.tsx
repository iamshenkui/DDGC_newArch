import { type Component, createMemo, createSignal } from "solid-js";

import type { GameAction, GameState } from "../../demo/types";
import { demoReducer } from "../../demo/reducer";
import { createSeedState } from "../../demo/seedContent";

/**
 * Isolated chaos-dungeon demo screen.
 *
 * Renders the current run phase, party summary, enemy or dungeon-event
 * summary, chaos-meter value, and at least one primary action button.
 *
 * The screen owns its own local state via a reducer — it does not touch
 * the runtime bridge or the session store.
 */

function ChaosMeterBar(props: { value: number }) {
  const pct = createMemo(() => Math.min((props.value / 20) * 100, 100));
  const barClass = createMemo(() => {
    if (props.value < 7) return "demo-chaos--low";
    if (props.value < 13) return "demo-chaos--mid";
    return "demo-chaos--high";
  });

  return (
    <div class="demo-chaos-bar">
      <span class="demo-chaos-label">Chaos: {props.value}</span>
      <div class="demo-chaos-track">
        <div
          class={`demo-chaos-fill ${barClass()}`}
          style={{ width: `${pct()}%` }}
        />
      </div>
    </div>
  );
}

function PartySummary(props: { state: GameState }) {
  return (
    <div class="demo-party">
      <h3 class="demo-section-title">Party</h3>
      <div class="demo-party-grid">
        {props.state.party.map((m) => (
          <div class="demo-party-member" data-testid={`party-${m.id}`}>
            <span class="demo-party-name">{m.name}</span>
            <span class="demo-party-class">{m.class}</span>
            <div class="demo-party-stat">
              <span>HP: {m.hp}/{m.maxHp}</span>
              <span>Stress: {m.stress}</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

function EnemySummary(props: { state: GameState }) {
  const group = () => props.state.currentRoom?.enemyGroup;
  if (!group()) return null;

  return (
    <div class="demo-enemy" data-testid="enemy-group">
      <h3 class="demo-section-title">Enemies</h3>
      <p>
        {group()!.count}× {group()!.name} (HP: {group()!.hp} each)
      </p>
    </div>
  );
}

function EventPanel(props: { state: GameState; onChoice: (index: number) => void }) {
  const evt = () => props.state.currentRoom?.event;
  if (!evt()) return null;

  return (
    <div class="demo-event" data-testid="event-panel">
      <h3 class="demo-section-title">{evt()!.title}</h3>
      <p class="demo-event-desc">{evt()!.description}</p>
      <div class="demo-event-choices">
        {evt()!.choices.map((c, i) => (
          <button
            class="demo-btn demo-btn--choice"
            onClick={() => props.onChoice(i)}
            data-testid={`choice-${i}`}
          >
            {c.label}
            <span class="demo-choice-chaos">
              {c.chaosDelta >= 0 ? "+" : ""}
              {c.chaosDelta} chaos
            </span>
          </button>
        ))}
      </div>
    </div>
  );
}

function RunLog(props: { entries: string[] }) {
  return (
    <details class="demo-log">
      <summary class="demo-log-summary">Run Log ({props.entries.length})</summary>
      <div class="demo-log-entries">
        {props.entries.map((e) => (
          <p class="demo-log-entry">{e}</p>
        ))}
      </div>
    </details>
  );
}

export const DemoScreen: Component = () => {
  const [state, setState] = createSignal<GameState>(createSeedState());

  const dispatch = (action: GameAction) => {
    setState((prev) => demoReducer(prev, action));
  };

  const phase = createMemo(() => state().phase);

  return (
    <main class="demo-screen" data-testid="demo-screen">
      <h2 class="demo-title">Chaos Dungeon Demo</h2>
      <p class="demo-floor">Floor {state().dungeonLevel}</p>

      <ChaosMeterBar value={state().chaosMeter} />

      <PartySummary state={state()} />

      <Switch phase={phase()}>
        {/* ── Start ── */}
        <Match when="start">
          <section class="demo-phase-panel" data-testid="phase-start">
            <p class="demo-instruction">
              Prepare your party. The darkness below stirs…
            </p>
            <button
              class="demo-btn demo-btn--primary"
              onClick={() => dispatch({ type: "START_RUN" })}
              data-testid="btn-start-run"
            >
              Begin Run
            </button>
          </section>
        </Match>

        {/* ── Dungeon Room (exploration) ── */}
        <Match when="dungeon-room">
          <section class="demo-phase-panel" data-testid="phase-room">
            <h3 class="demo-section-title">
              {state().currentRoom?.name ?? "Unknown Chamber"}
            </h3>
            <p class="demo-room-desc">{state().currentRoom?.description}</p>
            <div class="demo-actions">
              <button
                class="demo-btn demo-btn--primary"
                onClick={() => dispatch({ type: "ENTER_ROOM" })}
                data-testid="btn-enter-room"
              >
                Press Forward
              </button>
              <button
                class="demo-btn demo-btn--danger"
                onClick={() => dispatch({ type: "END_RUN" })}
                data-testid="btn-end-run"
              >
                Abandon Run
              </button>
            </div>
          </section>
        </Match>

        {/* ── Event ── */}
        <Match when="event">
          <section class="demo-phase-panel" data-testid="phase-event">
            <EventPanel
              state={state()}
              onChoice={(i) => dispatch({ type: "RESOLVE_EVENT", choiceIndex: i })}
            />
          </section>
        </Match>

        {/* ── Combat ── */}
        <Match when="combat">
          <section class="demo-phase-panel" data-testid="phase-combat">
            <EnemySummary state={state()} />
            <div class="demo-actions">
              <button
                class="demo-btn demo-btn--primary"
                onClick={() => dispatch({ type: "COMBAT_WIN" })}
                data-testid="btn-combat-win"
              >
                Fight & Win
              </button>
              <button
                class="demo-btn demo-btn--danger"
                onClick={() => dispatch({ type: "COMBAT_FLEE" })}
                data-testid="btn-combat-flee"
              >
                Flee
              </button>
            </div>
          </section>
        </Match>

        {/* ── Result ── */}
        <Match when="result">
          <section class="demo-phase-panel" data-testid="phase-result">
            <h3 class="demo-section-title">Run Over</h3>
            <p class="demo-result-message">{state().resultMessage}</p>
            <p class="demo-run-stats">
              Turns survived: {state().turnCount} &middot; Final chaos: {state().chaosMeter}
            </p>
            <button
              class="demo-btn demo-btn--primary"
              onClick={() => setState(createSeedState())}
              data-testid="btn-new-run"
            >
              New Run
            </button>
          </section>
        </Match>
      </Switch>

      <RunLog entries={state().runLog} />
    </main>
  );
};

// ── Minimal helper to replace solid-js Switch/Match at the phase level ──

/**
 * Inline Switch component that renders the first child whose `when` prop
 * matches the current `phase` value.
 * Avoids pulling in a full routing or conditional library.
 */
function Switch(props: { phase: string; children: any }) {
  const children = Array.isArray(props.children)
    ? props.children
    : [props.children];
  const match = children.find(
    (c: any) => c?.when === props.phase || c?.props?.when === props.phase,
  );
  return match ?? null;
}

function Match(props: { when: string; children: any }) {
  return props.children;
}

// ── Inline CSS injected once per mount ─────────────────────────────────────

const STYLE_ID = "demo-screen-styles";

function ensureStyles() {
  if (typeof document === "undefined" || document.getElementById(STYLE_ID)) return;
  const css = `
.demo-screen {
  max-width: 720px;
  margin: 0 auto;
  padding: 1.5rem;
  font-family: 'Courier New', Courier, monospace;
  color: #ccc;
  background: #1a1a2e;
  min-height: 100vh;
}
.demo-title {
  font-size: 1.5rem;
  text-align: center;
  color: #e0a800;
  margin: 0 0 0.25rem;
  text-transform: uppercase;
  letter-spacing: 0.15em;
}
.demo-floor {
  text-align: center;
  font-size: 0.85rem;
  color: #888;
  margin: 0 0 1rem;
}
.demo-chaos-bar {
  margin-bottom: 1rem;
}
.demo-chaos-label {
  display: block;
  font-size: 0.85rem;
  margin-bottom: 0.25rem;
  font-weight: bold;
}
.demo-chaos-track {
  height: 12px;
  background: #333;
  border-radius: 6px;
  overflow: hidden;
}
.demo-chaos-fill {
  height: 100%;
  border-radius: 6px;
  transition: width 0.3s ease;
}
.demo-chaos--low { background: #4ade80; }
.demo-chaos--mid { background: #facc15; }
.demo-chaos--high { background: #ef4444; }
.demo-party { margin-bottom: 1rem; }
.demo-party-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
  gap: 0.5rem;
}
.demo-party-member {
  background: #16213e;
  border: 1px solid #333;
  padding: 0.5rem;
  border-radius: 4px;
}
.demo-party-name { font-weight: bold; color: #e0a800; display: block; }
.demo-party-class { font-size: 0.8rem; color: #aaa; display: block; }
.demo-party-stat { font-size: 0.75rem; color: #888; margin-top: 0.25rem; display: flex; gap: 0.5rem; }
.demo-enemy { margin-bottom: 1rem; }
.demo-section-title {
  font-size: 1rem;
  color: #e0a800;
  margin: 0 0 0.5rem;
  border-bottom: 1px solid #333;
  padding-bottom: 0.25rem;
}
.demo-phase-panel { margin-bottom: 1rem; }
.demo-instruction { color: #aaa; margin-bottom: 1rem; }
.demo-room-desc { color: #aaa; font-style: italic; margin-bottom: 0.75rem; }
.demo-event { margin-bottom: 1rem; }
.demo-event-desc { color: #aaa; margin-bottom: 0.75rem; }
.demo-event-choices { display: flex; flex-direction: column; gap: 0.5rem; }
.demo-choice-chaos {
  display: inline-block;
  font-size: 0.75rem;
  margin-left: 0.5rem;
  opacity: 0.7;
}
.demo-actions {
  display: flex;
  gap: 0.75rem;
  flex-wrap: wrap;
}
.demo-btn {
  padding: 0.6rem 1.2rem;
  border: 1px solid #555;
  border-radius: 4px;
  font-family: inherit;
  font-size: 0.9rem;
  cursor: pointer;
  transition: background 0.2s;
  color: #ddd;
}
.demo-btn:hover { filter: brightness(1.2); }
.demo-btn--primary { background: #0f3460; border-color: #e0a800; color: #e0a800; }
.demo-btn--danger { background: #3d0f0f; border-color: #ef4444; color: #ef4444; }
.demo-btn--choice { background: #16213e; border-color: #555; text-align: left; display: flex; justify-content: space-between; width: 100%; }
.demo-result-message { color: #ef4444; font-size: 1.1rem; margin-bottom: 0.5rem; }
.demo-run-stats { color: #aaa; margin-bottom: 1rem; }
.demo-log { margin-top: 1.5rem; }
.demo-log-summary { cursor: pointer; color: #888; font-size: 0.8rem; }
.demo-log-entries { max-height: 200px; overflow-y: auto; background: #111; padding: 0.5rem; border-radius: 4px; margin-top: 0.25rem; }
.demo-log-entry { font-size: 0.75rem; margin: 0.25rem 0; color: #666; }
`;
  const style = document.createElement("style");
  style.id = STYLE_ID;
  style.textContent = css;
  document.head.appendChild(style);
}

// Inject styles once
if (typeof document !== "undefined") ensureStyles();
