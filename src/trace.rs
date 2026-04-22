//! Battle trace — structured record of combat events for debugging and regression.
//!
//! Each trace entry captures one step of the battle loop. The full trace is
//! deterministic: running the same battle with the same inputs produces the
//! same sequence of entries. Traces can be serialized to JSON for diff-based
//! regression checks.

use framework_combat::encounter::CombatSide;
use framework_combat::results::EffectResult;
#[allow(unused_imports)]
use framework_rules::actor::{ActorAggregate, ActorId};
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
    /// For reactive follow-up entries (riposte, guard redirect), this captures
    /// the trigger relationship: which actor was hit and what skill triggered
    /// the reaction. None for normal actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub triggered_by: Option<ReactiveTrigger>,
}

/// A simplified effect record for trace output.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TraceEffect {
    pub kind: String,
    pub target: u64,
    pub value: f64,
}

/// The trigger relationship for a reactive follow-up entry.
///
/// This captures which actor was hit by the original attack and what skill
/// caused the reactive follow-up (riposte counter-attack or guard redirect).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReactiveTrigger {
    /// The actor who initiated the original attack that triggered this reaction.
    pub attacker: u64,
    /// The skill used in the original attack.
    pub skill: String,
    /// The actor who was hit by the original attack (and may react).
    pub target: u64,
    /// The kind of reactive follow-up.
    pub kind: String,
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
            triggered_by: None,
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
            triggered_by: None,
        });
    }

    /// Record a miss action (attack missed due to dodge).
    pub fn record_miss(
        &mut self,
        turn: u32,
        actor: ActorId,
        targets: &[ActorId],
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
            action: "miss".to_string(),
            targets: targets.iter().map(|t| t.0).collect(),
            effects: vec![],
            snapshot,
            triggered_by: None,
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
            triggered_by: None,
        });
    }

    /// Record a reactive follow-up action (riposte counter-attack or guard redirect).
    ///
    /// Reactive events are triggered by a damage action. The `trigger` parameter
    /// captures the original attack that caused this reaction, making the
    /// trigger relationship explicit in the trace.
    // Allow: too_many_arguments — function requires all these parameters for trace structure
    #[allow(clippy::too_many_arguments)]
    pub fn record_reactive(
        &mut self,
        turn: u32,
        actor: ActorId,
        action: &str,
        targets: &[ActorId],
        effect_results: &[EffectResult],
        actors: &HashMap<ActorId, framework_rules::actor::ActorAggregate>,
        trigger: ReactiveTrigger,
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
            triggered_by: Some(trigger),
        });
    }

    /// Record a phase transition event (multi-phase boss transitioning to next phase).
    ///
    /// The `removed` parameter lists actor IDs that were removed (clone forms).
    /// The `summoned` parameter is the ActorId of the newly materialized final form.
    pub fn record_phase_transition(
        &mut self,
        turn: u32,
        removed: &[ActorId],
        summoned: ActorId,
        actors: &HashMap<ActorId, framework_rules::actor::ActorAggregate>,
    ) {
        // Snapshot: HP of every actor (BTreeMap for deterministic ordering)
        let mut snapshot = BTreeMap::new();
        for (&id, actor) in actors {
            let hp = actor.effective_attribute(&AttributeKey::new(ATTR_HEALTH));
            snapshot.insert(id.0, hp.0);
        }

        self.entries.push(TraceEntry {
            turn,
            actor: 0, // Phase transition is not attributed to a specific actor
            action: "phase_transition".to_string(),
            targets: removed.iter().map(|t| t.0).collect(),
            effects: vec![TraceEffect {
                kind: "Summon".to_string(),
                target: summoned.0,
                value: 0.0,
            }],
            snapshot,
            triggered_by: None,
        });
    }

    /// Record a capture event (hero captured by captor).
    pub fn record_capture(
        &mut self,
        turn: u32,
        captor: ActorId,
        captured: ActorId,
        actors: &HashMap<ActorId, framework_rules::actor::ActorAggregate>,
    ) {
        // Snapshot: HP of every actor (BTreeMap for deterministic ordering)
        let mut snapshot = BTreeMap::new();
        for (&id, actor) in actors {
            let hp = actor.effective_attribute(&AttributeKey::new(ATTR_HEALTH));
            snapshot.insert(id.0, hp.0);
        }

        self.entries.push(TraceEntry {
            turn,
            actor: captor.0,
            action: "capture".to_string(),
            targets: vec![captured.0],
            effects: vec![],
            snapshot,
            triggered_by: None,
        });
    }

    /// Record a release event (hero released from captor).
    ///
    /// The `reason` indicates why the release occurred: "captor_death" or "deaths_door".
    pub fn record_release(
        &mut self,
        turn: u32,
        captor: ActorId,
        released: ActorId,
        reason: &str,
        actors: &HashMap<ActorId, framework_rules::actor::ActorAggregate>,
    ) {
        // Snapshot: HP of every actor (BTreeMap for deterministic ordering)
        let mut snapshot = BTreeMap::new();
        for (&id, actor) in actors {
            let hp = actor.effective_attribute(&AttributeKey::new(ATTR_HEALTH));
            snapshot.insert(id.0, hp.0);
        }

        self.entries.push(TraceEntry {
            turn,
            actor: captor.0,
            action: format!("release_{}", reason),
            targets: vec![released.0],
            effects: vec![],
            snapshot,
            triggered_by: None,
        });
    }

    /// Record a captor passive damage event (egg_membrane_full dealing DoT to captive).
    pub fn record_captor_dot(
        &mut self,
        turn: u32,
        captor: ActorId,
        captive: ActorId,
        damage: f64,
        actors: &HashMap<ActorId, framework_rules::actor::ActorAggregate>,
    ) {
        // Snapshot: HP of every actor (BTreeMap for deterministic ordering)
        let mut snapshot = BTreeMap::new();
        for (&id, actor) in actors {
            let hp = actor.effective_attribute(&AttributeKey::new(ATTR_HEALTH));
            snapshot.insert(id.0, hp.0);
        }

        self.entries.push(TraceEntry {
            turn,
            actor: captor.0,
            action: "captor_dot".to_string(),
            targets: vec![captive.0],
            effects: vec![TraceEffect {
                kind: "captor_damage".to_string(),
                target: captive.0,
                value: damage,
            }],
            snapshot,
            triggered_by: None,
        });
    }

    /// Record an HP averaging event (paired boss crimson_duet averaging HP).
    pub fn record_hp_averaging(
        &mut self,
        turn: u32,
        actor_a: ActorId,
        actor_b: ActorId,
        avg_hp: f64,
        actors: &HashMap<ActorId, framework_rules::actor::ActorAggregate>,
    ) {
        // Snapshot: HP of every actor (BTreeMap for deterministic ordering)
        let mut snapshot = BTreeMap::new();
        for (&id, actor) in actors {
            let hp = actor.effective_attribute(&AttributeKey::new(ATTR_HEALTH));
            snapshot.insert(id.0, hp.0);
        }

        self.entries.push(TraceEntry {
            turn,
            actor: actor_a.0,
            action: "hp_averaging".to_string(),
            targets: vec![actor_b.0],
            effects: vec![TraceEffect {
                kind: "HP_Average".to_string(),
                target: actor_b.0,
                value: avg_hp,
            }],
            snapshot,
            triggered_by: None,
        });
    }

    /// Record an act-out event (afflicted hero acting out at start of turn).
    ///
    /// The `act_out` parameter is the string representation of the act-out action
    /// (e.g., "nothing", "bark_stress", "ignore_command", etc.).
    pub fn record_act_out(
        &mut self,
        turn: u32,
        actor: ActorId,
        act_out: &str,
        actors: &HashMap<ActorId, framework_rules::actor::ActorAggregate>,
    ) {
        // Snapshot: HP of every actor (BTreeMap for deterministic ordering)
        let mut snapshot = BTreeMap::new();
        for (&id, act) in actors {
            let hp = act.effective_attribute(&AttributeKey::new(ATTR_HEALTH));
            snapshot.insert(id.0, hp.0);
        }

        self.entries.push(TraceEntry {
            turn,
            actor: actor.0,
            action: format!("act_out_{}", act_out),
            targets: vec![],
            effects: vec![],
            snapshot,
            triggered_by: None,
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
            if let Some(ref trigger) = entry.triggered_by {
                out.push_str(&format!(
                    "  [REACTIVE: {} by {} on {} from {}'s {}]\n",
                    trigger.kind, entry.actor, trigger.target, trigger.attacker, trigger.skill
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

#[cfg(test)]
mod tests {
    use super::*;
    use framework_rules::actor::ActorAggregate;

    fn make_test_actors() -> HashMap<ActorId, ActorAggregate> {
        HashMap::new()
    }

    #[test]
    fn trace_serialization_is_stable_with_reactive_event() {
        // Same reactive event recorded twice must produce identical trace entries.
        // This is the core determinism guarantee for US-504.
        let mut trace1 = BattleTrace::new("test_reactive");
        let mut trace2 = BattleTrace::new("test_reactive");

        let trigger = ReactiveTrigger {
            attacker: 10,
            skill: "normal_attack".to_string(),
            target: 1,
            kind: "Riposte".to_string(),
        };

        // Record same reactive event twice in same turn
        trace1.record_reactive(
            1,
            ActorId(1),
            "riposte",
            &[ActorId(10)],
            &[],
            &make_test_actors(),
            trigger.clone(),
        );

        trace2.record_reactive(
            1,
            ActorId(1),
            "riposte",
            &[ActorId(10)],
            &[],
            &make_test_actors(),
            trigger.clone(),
        );

        // JSON serialization must be identical
        let json1 = trace1.to_json();
        let json2 = trace2.to_json();
        assert_eq!(
            json1, json2,
            "Same reactive event must produce identical JSON serialization"
        );

        // Number of entries must match
        assert_eq!(
            trace1.entries.len(),
            trace2.entries.len(),
            "Same reactive event must produce same number of entries"
        );

        // Entries must be deeply equal
        assert_eq!(
            trace1.entries[0], trace2.entries[0],
            "Same reactive event must produce identical trace entries"
        );
    }

    #[test]
    fn trace_serialization_stable_across_multiple_reactive_events() {
        // Multiple reactive events in deterministic order must serialize stably.
        let mut trace1 = BattleTrace::new("multi_reactive");
        let mut trace2 = BattleTrace::new("multi_reactive");

        let trigger1 = ReactiveTrigger {
            attacker: 10,
            skill: "normal_attack".to_string(),
            target: 1,
            kind: "Riposte".to_string(),
        };

        let trigger2 = ReactiveTrigger {
            attacker: 10,
            skill: "heavy_strike".to_string(),
            target: 2,
            kind: "GuardRedirect".to_string(),
        };

        // Record in same order
        trace1.record_reactive(
            1,
            ActorId(1),
            "riposte",
            &[ActorId(10)],
            &[],
            &make_test_actors(),
            trigger1.clone(),
        );
        trace1.record_reactive(
            1,
            ActorId(2),
            "guard_redirect",
            &[ActorId(3)],
            &[],
            &make_test_actors(),
            trigger2.clone(),
        );

        trace2.record_reactive(
            1,
            ActorId(1),
            "riposte",
            &[ActorId(10)],
            &[],
            &make_test_actors(),
            trigger1.clone(),
        );
        trace2.record_reactive(
            1,
            ActorId(2),
            "guard_redirect",
            &[ActorId(3)],
            &[],
            &make_test_actors(),
            trigger2.clone(),
        );

        let json1 = trace1.to_json();
        let json2 = trace2.to_json();
        assert_eq!(
            json1, json2,
            "Multiple reactive events must produce identical serialization"
        );
    }

    #[test]
    fn reactive_trigger_fields_are_captured_correctly() {
        let trigger = ReactiveTrigger {
            attacker: 42,
            skill: "fireball".to_string(),
            target: 7,
            kind: "Riposte".to_string(),
        };

        let mut trace = BattleTrace::new("trigger_test");
        trace.record_reactive(
            3,
            ActorId(7),
            "riposte",
            &[ActorId(42)],
            &[],
            &make_test_actors(),
            trigger.clone(),
        );

        let entry = &trace.entries[0];
        assert_eq!(entry.turn, 3);
        assert_eq!(entry.actor, 7);
        assert_eq!(entry.action, "riposte");
        assert_eq!(entry.targets, vec![42]);

        let captured = entry.triggered_by.as_ref().unwrap();
        assert_eq!(captured.attacker, 42);
        assert_eq!(captured.skill, "fireball");
        assert_eq!(captured.target, 7);
        assert_eq!(captured.kind, "Riposte");
    }

    #[test]
    fn regular_action_has_no_trigger() {
        let mut trace = BattleTrace::new("regular_action");
        trace.record_action(
            1,
            ActorId(10),
            "normal_attack",
            &[ActorId(1)],
            &[],
            &make_test_actors(),
        );

        let entry = &trace.entries[0];
        assert!(
            entry.triggered_by.is_none(),
            "Regular action should have no triggered_by"
        );
    }

    #[test]
    fn to_text_includes_reactive_trigger_info() {
        let trigger = ReactiveTrigger {
            attacker: 99,
            skill: "slash".to_string(),
            target: 5,
            kind: "Riposte".to_string(),
        };

        let mut trace = BattleTrace::new("text_reactive");
        trace.record_reactive(
            2,
            ActorId(5),
            "riposte",
            &[ActorId(99)],
            &[],
            &make_test_actors(),
            trigger,
        );

        let text = trace.to_text();
        assert!(
            text.contains("[REACTIVE: Riposte by 5 on 5 from 99's slash]"),
            "Text output should include reactive trigger info"
        );
    }
}
