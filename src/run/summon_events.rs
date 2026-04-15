//! Summon runtime event seam — explicit summon request events for boss summoning mechanics.
//!
//! This module provides a first-class `SummonEvent` type distinct from generic status
//! application, so boss skills that emit `apply_status("summon_XXX", N)` markers can be
//! detected and routed to real unit creation through a stable game-layer pathway.
//!
//! ## Design
//!
//! `EffectNode::apply_status("summon_rotten_fruit", Some(1))` is the marker pattern
//! used by migrated DDGC summon skills. This module detects that pattern from resolved
//! effect results and emits a structured `SummonEvent` that carries:
//! - The summoner (`ActorId`)
//! - The summon kind (which family to summon)
//! - Deterministic placement intent
//!
//! The event is created deterministically from in-progress encounter state without
//! immediately mutating that state. Actual unit materialization (US-708) will
//! consume these events in a later slice.
//!
//! ## Summon Kind Patterns
//!
//! The following status kinds are recognized as summon requests:
//! - `"summon_rotten_fruit"` — rotvine_wraith summons rotten_fruit minions
//! - `"summon_mahjong"` — gambler summons mahjong tile units
//! - `"summon_scorchthroat"` — scorchthroat_chanteuse summons sc_blow/sc_bow
//! - `"summon_split"` — ghost_fire_assist/gh ost_fire_damage split clones
//! - `"summon_vegetable"` — skeletal_tiller summons vegetable minion
//! - `"summon_pearlkin"` — frostvein_clam summons pearlkin minions
//! - `"summon_necrodrake"` — necrodrake_embryosac creates additional necrodrake units
//!
//! Any `apply_status` with a `status_kind` starting with `"summon_"` is treated
//! as a summon request. This is the stable seam: adding new summon types only
//! requires adding a new pattern match, not changing the event extraction logic.

use serde::{Deserialize, Serialize};

use framework_combat::results::EffectResult;
use framework_rules::actor::ActorId;

/// Kind of summon — maps status kind patterns to concrete DDGC summon behaviors.
///
/// Each variant corresponds to a DDGC-authored summon mechanic. The variant
/// is used to determine the correct `FamilyId` for the summoned unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SummonKind {
    /// Rotten fruit minion (rotvine_wraith primary summon).
    RottenFruit,
    /// Mahjong tile unit (gambler primary summon).
    MahjongTile,
    /// Scorchthroat blow/sc_bow minion (scorchthroat_chanteuse primary summon).
    ScorchthroatMinion,
    /// Ghost fire clone (ghost_fire_assist/gh ost_fire_damage split mechanic).
    GhostFireClone,
    /// Vegetable minion (skeletal_tiller primary summon).
    VegetableMinion,
    /// Pearlkin minion (frostvein_clam summon — also triggers empowerment phase).
    PearlkinMinion,
    /// Additional necrodrake unit (necrodrake_embryosac captor mechanic).
    NecrodrakeUnit,
    /// Cauldron egg variant (necrodrake_embryosac captor mechanic).
    CauldronEgg,
    /// Generic summon — status_kind did not match a known pattern.
    Generic,
}

impl SummonKind {
    /// Classify a status kind string as a summon kind.
    ///
    /// Returns `Some(SummonKind)` if the status is a summon request,
    /// or `None` if it is a regular (non-summon) status.
    pub fn from_status_kind(status_kind: &str) -> Option<SummonKind> {
        match status_kind {
            "summon_rotten_fruit" => Some(SummonKind::RottenFruit),
            "summon_mahjong" => Some(SummonKind::MahjongTile),
            "summon_scorchthroat" => Some(SummonKind::ScorchthroatMinion),
            "summon_split" => Some(SummonKind::GhostFireClone),
            "summon_vegetable" => Some(SummonKind::VegetableMinion),
            "summon_pearlkin" => Some(SummonKind::PearlkinMinion),
            "summon_necrodrake" => Some(SummonKind::NecrodrakeUnit),
            "summon_cauldron" => Some(SummonKind::CauldronEgg),
            k if k.starts_with("summon_") => Some(SummonKind::Generic),
            _ => None,
        }
    }

    /// Returns true if this summon kind requires checking for duplicate summons
    /// within the same trigger resolution (to prevent infinite spawn loops).
    pub fn needs_dedup(&self) -> bool {
        matches!(
            self,
            SummonKind::RottenFruit
                | SummonKind::GhostFireClone
                | SummonKind::PearlkinMinion
                | SummonKind::NecrodrakeUnit
        )
    }
}

/// Placement strategy for a summoned unit.
///
/// Determines where in the formation a new unit is placed. The strategy is
/// resolved deterministically: front empty slot first, then back empty slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SummonPlacement {
    /// Summon in the front row of the summoner's lane.
    FrontRow,
    /// Summon in the back row of the summoner's lane.
    BackRow,
    /// Summon in any empty slot in the summoner's lane.
    AnyEmpty,
}

impl SummonPlacement {
    /// Default placement for most summon skills: front row first.
    pub fn default_for_summon() -> Self {
        SummonPlacement::FrontRow
    }
}

/// A summon runtime event — the explicit seam for boss summoning mechanics.
///
/// This event is created deterministically from a resolved skill effect that
/// applied a `summon_XXX` status. It is NOT the status application itself —
/// it is the game-layer interpretation of that status as a unit creation request.
///
/// The event carries enough information to materialize the summoned unit without
/// further DDGC-specific knowledge:
/// - `summoner`: who cast the summon skill
/// - `summon_kind`: what type of unit to summon
/// - `placement`: where to place it (deterministic strategy)
/// - `count`: how many units to summon (from status duration, usually 1)
///
/// The event does NOT mutate encounter state. That happens in US-708 when the
/// event is processed by the materialization pathway.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SummonEvent {
    /// The actor who initiated the summon.
    pub summoner: ActorId,
    /// The kind of summon being requested.
    pub summon_kind: SummonKind,
    /// Where to place the summoned unit.
    pub placement: SummonPlacement,
    /// How many units to summon (derived from status duration).
    pub count: u32,
}

impl SummonEvent {
    /// Create a new summon event.
    pub fn new(
        summoner: ActorId,
        summon_kind: SummonKind,
        placement: SummonPlacement,
        count: u32,
    ) -> Self {
        SummonEvent {
            summoner,
            summon_kind,
            placement,
            count,
        }
    }

    /// Create a summon event from a rotvine_wraith summon skill.
    pub fn rotvine_wraith_summon(summoner: ActorId) -> Self {
        SummonEvent::new(
            summoner,
            SummonKind::RottenFruit,
            SummonPlacement::default_for_summon(),
            1,
        )
    }

    /// Create a summon event from a gambler summon skill.
    pub fn gambler_summon(summoner: ActorId) -> Self {
        SummonEvent::new(
            summoner,
            SummonKind::MahjongTile,
            SummonPlacement::default_for_summon(),
            1,
        )
    }

    /// Create a summon event from a ghost_fire split skill.
    pub fn ghost_fire_split(summoner: ActorId) -> Self {
        SummonEvent::new(
            summoner,
            SummonKind::GhostFireClone,
            SummonPlacement::default_for_summon(),
            1,
        )
    }
}

/// Extract summon events from resolved skill effect results.
///
/// This is the deterministic seam: given the same effect results from the same
/// encounter state, it always produces the same summon events in the same order.
///
/// # Arguments
///
/// * `actor` — the actor who used the skill (the summoner)
/// * `effects` — the resolved effect results from `resolve_skill()`
///
/// # Returns
///
/// A `Vec<SummonEvent>` containing all summon requests detected in the effects.
/// Returns an empty vec if no summon statuses were applied.
///
/// # Why this is non-mutating
///
/// This function only reads from `EffectResult` data structures. It does not
/// modify `actors`, `formation`, or any other encounter state. The returned
/// events are pure data describing a summon intent — actual unit creation
/// happens later when these events are processed by the materialization pathway.
pub fn extract_summon_events(
    actor: ActorId,
    effects: &[EffectResult],
) -> Vec<SummonEvent> {
    let mut events = Vec::new();

    for effect in effects {
        // Only ApplyStatus effects can contain summon requests
        if !matches!(
            effect.kind,
            framework_combat::results::EffectResultKind::ApplyStatus
        ) {
            continue;
        }

        for status in &effect.applied_statuses {
            let kind_str = status.kind.0.as_str();

            // Check if this status kind is a summon request
            if let Some(summon_kind) = SummonKind::from_status_kind(kind_str) {
                // Duration encodes the summon count (usually 1)
                let count = status.duration.unwrap_or(1).max(1);

                let event = SummonEvent::new(
                    actor,
                    summon_kind,
                    SummonPlacement::default_for_summon(),
                    count,
                );

                events.push(event);
            }
        }
    }

    events
}

/// Check whether a status kind string represents a summon request.
pub fn is_summon_status(status_kind: &str) -> bool {
    SummonKind::from_status_kind(status_kind).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: make a minimal EffectResult with an ApplyStatus effect and given status kind.
    fn make_summon_effect(
        actor: ActorId,
        status_kind: &str,
        duration: Option<u32>,
    ) -> EffectResult {
        use framework_combat::results::EffectResultKind;
        use framework_rules::statuses::{StackRule, StatusEffect};

        let status = StatusEffect::new(
            framework_rules::statuses::StatusKind::new(status_kind),
            duration,
            vec![],
            StackRule::Replace,
        );

        EffectResult::new(
            EffectResultKind::ApplyStatus,
            actor,
            vec![actor], // self-targeted
        )
        .with_status(status)
    }

    // Helper: make a damage effect (should be ignored by summon extraction).
    fn make_damage_effect(actor: ActorId, target: ActorId) -> EffectResult {
        use framework_combat::results::EffectResultKind;

        EffectResult::new(EffectResultKind::Damage, actor, vec![target])
            .with_value("amount", 25.0)
    }

    #[test]
    fn extract_summon_events_detects_rotten_fruit_summon() {
        let summoner = ActorId(10);
        let effects = vec![make_summon_effect(summoner, "summon_rotten_fruit", Some(1))];

        let events = extract_summon_events(summoner, &effects);

        assert_eq!(events.len(), 1, "Should produce exactly one summon event");
        assert_eq!(events[0].summoner, summoner);
        assert!(matches!(events[0].summon_kind, SummonKind::RottenFruit));
        assert_eq!(events[0].count, 1);
    }

    #[test]
    fn extract_summon_events_detects_ghost_fire_split() {
        let summoner = ActorId(5);
        let effects = vec![make_summon_effect(summoner, "summon_split", Some(1))];

        let events = extract_summon_events(summoner, &effects);

        assert_eq!(events.len(), 1);
        assert!(matches!(events[0].summon_kind, SummonKind::GhostFireClone));
        assert_eq!(events[0].count, 1);
    }

    #[test]
    fn extract_summon_events_ignores_damage_effects() {
        let summoner = ActorId(10);
        let target = ActorId(1);
        // Damage effects should be completely ignored
        let effects = vec![make_damage_effect(summoner, target)];

        let events = extract_summon_events(summoner, &effects);

        assert!(events.is_empty(), "Damage effects should not produce summon events");
    }

    #[test]
    fn extract_summon_events_ignores_non_summon_statuses() {
        let summoner = ActorId(10);
        // A regular status (burn, poison, etc.) should not produce a summon event
        let effects = vec![
            make_summon_effect(summoner, "burn", Some(2)),
            make_summon_effect(summoner, "poison", Some(3)),
            make_summon_effect(summoner, "stun", Some(1)),
        ];

        let events = extract_summon_events(summoner, &effects);

        assert!(
            events.is_empty(),
            "Non-summon statuses should not produce summon events"
        );
    }

    #[test]
    fn extract_summon_events_is_deterministic() {
        let summoner = ActorId(10);
        let effects = vec![
            make_summon_effect(summoner, "summon_rotten_fruit", Some(1)),
            make_summon_effect(summoner, "burn", Some(2)), // non-summon, should be ignored
        ];

        let events1 = extract_summon_events(summoner, &effects);
        let events2 = extract_summon_events(summoner, &effects);

        assert_eq!(events1, events2, "Same effects must produce identical events");
    }

    #[test]
    fn extract_summon_events_handles_multiple_summons() {
        let summoner = ActorId(10);
        // Multiple summon statuses in the same effect list
        let effects = vec![
            make_summon_effect(summoner, "summon_rotten_fruit", Some(1)),
            make_summon_effect(summoner, "summon_split", Some(1)),
        ];

        let events = extract_summon_events(summoner, &effects);

        assert_eq!(events.len(), 2, "Should produce two summon events");
        assert!(matches!(events[0].summon_kind, SummonKind::RottenFruit));
        assert!(matches!(events[1].summon_kind, SummonKind::GhostFireClone));
    }

    #[test]
    fn extract_summon_events_uses_duration_as_count() {
        let summoner = ActorId(10);
        // Duration of 3 should produce count of 3
        let effects = vec![make_summon_effect(summoner, "summon_split", Some(3))];

        let events = extract_summon_events(summoner, &effects);

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].count, 3, "Duration should be used as summon count");
    }

    #[test]
    fn extract_summon_events_defaults_missing_duration_to_one() {
        let summoner = ActorId(10);
        // No duration specified
        let effects = vec![make_summon_effect(summoner, "summon_mahjong", None)];

        let events = extract_summon_events(summoner, &effects);

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].count, 1, "Missing duration should default to 1");
    }

    #[test]
    fn summon_event_rotvine_wraith_factory() {
        let summoner = ActorId(42);
        let event = SummonEvent::rotvine_wraith_summon(summoner);

        assert_eq!(event.summoner, summoner);
        assert!(matches!(event.summon_kind, SummonKind::RottenFruit));
        assert!(matches!(event.placement, SummonPlacement::FrontRow));
        assert_eq!(event.count, 1);
    }

    #[test]
    fn summon_event_gambler_factory() {
        let summoner = ActorId(99);
        let event = SummonEvent::gambler_summon(summoner);

        assert!(matches!(event.summon_kind, SummonKind::MahjongTile));
    }

    #[test]
    fn summon_event_ghost_fire_split_factory() {
        let summoner = ActorId(7);
        let event = SummonEvent::ghost_fire_split(summoner);

        assert!(matches!(event.summon_kind, SummonKind::GhostFireClone));
    }

    #[test]
    fn summon_kind_detects_all_known_patterns() {
        let cases = [
            ("summon_rotten_fruit", SummonKind::RottenFruit),
            ("summon_mahjong", SummonKind::MahjongTile),
            ("summon_scorchthroat", SummonKind::ScorchthroatMinion),
            ("summon_split", SummonKind::GhostFireClone),
            ("summon_vegetable", SummonKind::VegetableMinion),
            ("summon_pearlkin", SummonKind::PearlkinMinion),
            ("summon_necrodrake", SummonKind::NecrodrakeUnit),
            ("summon_cauldron", SummonKind::CauldronEgg),
            ("summon_unknown_type", SummonKind::Generic),
        ];

        for (status_kind, expected) in cases {
            let result = SummonKind::from_status_kind(status_kind);
            assert!(
                result.is_some(),
                "Expected {} to be recognized as summon",
                status_kind
            );
            assert_eq!(
                result.unwrap(),
                expected,
                "summon_kind classification mismatch for {}",
                status_kind
            );
        }
    }

    #[test]
    fn summon_kind_rejects_non_summon_statuses() {
        let non_summon = ["burn", "poison", "stun", "bleed", "mark", "riposte", "guard"];
        for status_kind in non_summon {
            let result = SummonKind::from_status_kind(status_kind);
            assert!(
                result.is_none(),
                "{} should NOT be recognized as a summon kind",
                status_kind
            );
        }
    }

    #[test]
    fn is_summon_status_returns_true_for_summon_kinds() {
        assert!(is_summon_status("summon_rotten_fruit"));
        assert!(is_summon_status("summon_split"));
        assert!(is_summon_status("summon_mahjong"));
    }

    #[test]
    fn is_summon_status_returns_false_for_regular_statuses() {
        assert!(!is_summon_status("burn"));
        assert!(!is_summon_status("poison"));
        assert!(!is_summon_status("stun"));
    }

    #[test]
    fn summon_kind_needs_dedup_for_loop_prone_summons() {
        assert!(
            SummonKind::RottenFruit.needs_dedup(),
            "RottenFruit needs dedup to prevent infinite spawns"
        );
        assert!(
            SummonKind::GhostFireClone.needs_dedup(),
            "GhostFireClone needs dedup to prevent infinite spawns"
        );
        assert!(
            SummonKind::PearlkinMinion.needs_dedup(),
            "PearlkinMinion needs dedup"
        );
        assert!(
            SummonKind::NecrodrakeUnit.needs_dedup(),
            "NecrodrakeUnit needs dedup"
        );

        assert!(
            !SummonKind::MahjongTile.needs_dedup(),
            "MahjongTile is finite and does not need dedup"
        );
        assert!(
            !SummonKind::ScorchthroatMinion.needs_dedup(),
            "ScorchthroatMinion does not need dedup"
        );
    }

    #[test]
    fn summon_events_are_serializable() {
        let event = SummonEvent::rotvine_wraith_summon(ActorId(42));

        let json = serde_json::to_string(&event).unwrap();
        let restored: SummonEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(restored, event);
    }

    #[test]
    fn extract_summon_events_with_mixed_effects() {
        // A skill that applies both damage AND a summon status
        // (e.g., a summon skill that also deals damage on summon)
        let summoner = ActorId(10);
        let target = ActorId(1);
        let effects = vec![
            make_damage_effect(summoner, target), // should be ignored
            make_summon_effect(summoner, "summon_rotten_fruit", Some(1)), // should be captured
            make_summon_effect(summoner, "burn", Some(2)), // should be ignored
        ];

        let events = extract_summon_events(summoner, &effects);

        assert_eq!(
            events.len(),
            1,
            "Only summon statuses should produce events"
        );
        assert!(matches!(events[0].summon_kind, SummonKind::RottenFruit));
    }

    #[test]
    fn extract_summon_events_empty_input() {
        let summoner = ActorId(10);
        let effects: Vec<EffectResult> = vec![];

        let events = extract_summon_events(summoner, &effects);

        assert!(events.is_empty());
    }

    #[test]
    fn summon_event_equality() {
        let summoner = ActorId(10);
        let event1 = SummonEvent::rotvine_wraith_summon(summoner);
        let event2 = SummonEvent::rotvine_wraith_summon(summoner);
        let event3 = SummonEvent::ghost_fire_split(summoner);

        assert_eq!(event1, event2, "Same inputs must produce equal events");
        assert_ne!(event1, event3, "Different summon kinds must produce different events");
    }

    #[test]
    fn summon_kind_from_unknown_summon_prefix_is_generic() {
        // A status like "summon_new_type" should be recognized as a summon
        // (starts with "summon_") but classified as Generic
        let result = SummonKind::from_status_kind("summon_new_boss_minion");
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), SummonKind::Generic));
    }
}
