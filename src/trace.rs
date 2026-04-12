//! Battle trace — structured record of combat events for debugging and regression.
//!
//! Each trace entry captures one step of the battle loop. The full trace is
//! deterministic: running the same battle with the same inputs produces the
//! same sequence of entries. Traces can be serialized to JSON for diff-based
//! regression checks.

use framework_combat::encounter::CombatSide;
use framework_combat::results::EffectResult;
use framework_rules::actor::ActorId;
use framework_rules::attributes::{AttributeKey, ATTR_HEALTH};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// A single step in the battle trace.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TraceEntry {
    pub turn: u32,
    pub actor: u64,
    pub action: String,
    pub targets: Vec<u64>,
    pub effects: Vec<TraceEffect>,
    pub snapshot: BTreeMap<u64, f64>,
}

/// A simplified effect record for trace output.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TraceEffect {
    pub kind: String,
    pub target: u64,
    pub value: f64,
}

/// Full battle trace including metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BattleTrace {
    pub scenario: String,
    pub winner: Option<String>,
    pub turns: u32,
    pub entries: Vec<TraceEntry>,
}

impl BattleTrace {
    pub fn new(scenario: &str) -> Self {
        BattleTrace {
            scenario: scenario.to_string(),
            winner: None,
            turns: 0,
            entries: Vec::new(),
        }
    }

    /// Record one combat action into the trace.
    pub fn record_action(
        &mut self,
        turn: u32,
        actor: ActorId,
        action: &str,
        targets: &[ActorId],
        effect_results: &[EffectResult],
        actors: &HashMap<ActorId, framework_rules::actor::ActorAggregate>,
    ) {
        let effects: Vec<TraceEffect> = effect_results
            .iter()
            .flat_map(|er| {
                let kind_str = format!("{:?}", er.kind);
                er.targets.iter().filter_map(move |&t| {
                    er.values.get("amount").map(|&v| TraceEffect {
                        kind: kind_str.clone(),
                        target: t.0,
                        value: v,
                    })
                })
            })
            .collect();

        // Snapshot: HP of every actor (BTreeMap for deterministic ordering)
        let mut snapshot = BTreeMap::new();
        for (&id, actor) in actors {
            let hp = actor.effective_attribute(&AttributeKey::new(ATTR_HEALTH));
            snapshot.insert(id.0, hp.0);
        }

        self.entries.push(TraceEntry {
            turn,
            actor: actor.0,
            action: action.to_string(),
            targets: targets.iter().map(|t| t.0).collect(),
            effects,
            snapshot,
        });
    }

    /// Record a wait action (no effects).
    pub fn record_wait(
        &mut self,
        turn: u32,
        actor: ActorId,
        actors: &HashMap<ActorId, framework_rules::actor::ActorAggregate>,
    ) {
        let mut snapshot = BTreeMap::new();
        for (&id, act) in actors {
            let hp = act.effective_attribute(&AttributeKey::new(ATTR_HEALTH));
            snapshot.insert(id.0, hp.0);
        }

        self.entries.push(TraceEntry {
            turn,
            actor: actor.0,
            action: "wait".to_string(),
            targets: vec![],
            effects: vec![],
            snapshot,
        });
    }

    /// Record a status tick event (end-of-turn status processing).
    pub fn record_status_tick(
        &mut self,
        turn: u32,
        actor: ActorId,
        damage: f64,
        actors: &HashMap<ActorId, framework_rules::actor::ActorAggregate>,
    ) {
        if damage.abs() < f64::EPSILON {
            return; // skip zero-damage ticks
        }

        let mut snapshot = BTreeMap::new();
        for (&id, act) in actors {
            let hp = act.effective_attribute(&AttributeKey::new(ATTR_HEALTH));
            snapshot.insert(id.0, hp.0);
        }

        self.entries.push(TraceEntry {
            turn,
            actor: actor.0,
            action: "status_tick".to_string(),
            targets: vec![actor.0],
            effects: vec![TraceEffect {
                kind: "status_damage".to_string(),
                target: actor.0,
                value: damage,
            }],
            snapshot,
        });
    }

    /// Finalize the trace with the battle outcome.
    pub fn finalize(&mut self, winner: Option<CombatSide>, turns: u32) {
        self.winner = winner.map(|s| format!("{:?}", s));
        self.turns = turns;
    }

    /// Render the trace as a human-readable text summary.
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("=== Battle Trace: {} ===\n", self.scenario));
        for entry in &self.entries {
            out.push_str(&format!(
                "T{} | Actor {} | {} | targets {:?}\n",
                entry.turn, entry.actor, entry.action, entry.targets
            ));
            for eff in &entry.effects {
                out.push_str(&format!(
                    "  -> {} on target {} (value: {:.1})\n",
                    eff.kind, eff.target, eff.value
                ));
            }
            out.push_str(&format!("  HP: {:?}\n", entry.snapshot));
        }
        out.push_str(&format!(
            "Result: {} after {} turns\n",
            self.winner.as_deref().unwrap_or("NONE"),
            self.turns
        ));
        out
    }

    /// Render the trace as JSON (for regression diffing).
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }
}
