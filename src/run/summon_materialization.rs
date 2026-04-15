//! Summon unit materialization — converts summon events to live encounter actors.
//!
//! This module provides the materialization pathway that US-708 requires: given
//! a `SummonEvent` extracted from resolved skill effects, it creates a real
//! actor in the encounter state through the formation placement pathway.
//!
//! ## Design
//!
//! The seam is intentionally narrow: `materialize_summons()` takes extracted
//! events and encounter state, and mutates that state directly. The function
//! is called in the battle loop after `extract_summon_events()` collects events
//! from the resolved skill.
//!
//! ## Deduplication
//!
//! Some summon kinds (`needs_dedup() == true`) can produce infinite spawn
//! loops if not deduplicated. The `SummonTracker` records which summon events
//! have been materialized this battle to prevent duplicate materialization of
//! the same summon within one trigger cycle.

use std::collections::HashMap;

use framework_combat::encounter::{CombatSide, Encounter};
use framework_combat::formation::{FormationLayout, SlotIndex};
use framework_rules::actor::{ActorAggregate, ActorId};

use crate::content::ContentPack;
use crate::monsters::MonsterFamilyRegistry;
use crate::run::summon_events::{SummonEvent, SummonKind, SummonPlacement};

/// Tracks materialized summons to prevent infinite spawn loops.
///
/// Some summon kinds (RottenFruit, GhostFireClone, PearlkinMinion, NecrodrakeUnit)
/// can produce infinite loops if the same summon event is materialized multiple
/// times per trigger cycle. The tracker records which (summoner, summon_kind) pairs
/// have been materialized this battle.
#[derive(Debug, Clone, Default)]
pub struct SummonTracker {
    /// Records which (summoner, summon_kind) pairs have been materialized.
    /// Key: (summoner_id, summon_kind_name), Value: count materialized this battle
    materialized: HashMap<(u64, String), u32>,
}

impl SummonTracker {
    /// Creates a new empty tracker.
    pub fn new() -> Self {
        SummonTracker {
            materialized: HashMap::new(),
        }
    }

    /// Check if a summon event can be materialized (not over deduplication limit).
    ///
    /// For summon kinds that need deduplication, only one materialization is
    /// allowed per (summoner, summon_kind) pair per battle.
    /// For summon kinds that don't need deduplication, always returns true.
    pub fn can_materialize(&self, event: &SummonEvent) -> bool {
        if !event.summon_kind.needs_dedup() {
            return true;
        }

        let key = (event.summoner.0, format!("{:?}", event.summon_kind));
        let count = self.materialized.get(&key).copied().unwrap_or(0);
        count == 0
    }

    /// Record that a summon event has been materialized.
    pub fn record_materialization(&mut self, event: &SummonEvent) {
        let key = (event.summoner.0, format!("{:?}", event.summon_kind));
        *self.materialized.entry(key).or_insert(0) += 1;
    }
}

/// Maps a SummonKind to a specific FamilyId for materialization.
///
/// This is public so it can be tested in integration tests.
pub fn family_id_for_summon_kind(summon_kind: SummonKind, summoner: ActorId) -> Option<String> {
    match summon_kind {
        SummonKind::MahjongTile => {
            // Gambler summons mahjong tiles: red, green, or white.
            // For deterministic selection, use the summoner ID to pick:
            // even ID → red, odd ID → green, (id % 3 == 0) → white
            // This ensures the same summoner always picks the same tile.
            let tile = match summoner.0 % 3 {
                0 => "mahjong_red",
                1 => "mahjong_green",
                _ => "mahjong_white",
            };
            Some(tile.to_string())
        }
        SummonKind::RottenFruit => Some("rotten_fruit_A".to_string()),
        SummonKind::GhostFireClone => Some("ghost_fire_assist".to_string()),
        SummonKind::VegetableMinion => Some("vegetable".to_string()),
        SummonKind::PearlkinMinion => Some("pearlkin_flawed".to_string()),
        SummonKind::NecrodrakeUnit => Some("necrodrake_embryosac".to_string()),
        SummonKind::ScorchthroatMinion => Some("sc_blow".to_string()),
        SummonKind::CauldronEgg => Some("egg_membrane_full".to_string()),
        SummonKind::Generic => None,
    }
}

/// Finds a placement slot for a summoned unit based on the placement strategy.
///
/// For FrontRow: finds the first empty slot in the front half of the summoner's lane.
/// For BackRow: finds the first empty slot in the back half of the summoner's lane.
/// For AnyEmpty: finds any empty slot in the summoner's lane.
///
/// Returns the SlotIndex if a valid slot is found, or None if no slots available.
fn find_placement_slot(
    placement: SummonPlacement,
    summoner: ActorId,
    formation: &FormationLayout,
    actors: &HashMap<ActorId, ActorAggregate>,
) -> Option<SlotIndex> {
    // FormationLayout fields: lanes, slots_per_lane, slots
    let slots_per_lane = formation.slots_per_lane;
    let lanes = formation.lanes;

    // Determine which lane the summoner is in
    let summoner_slot = formation.find_actor(summoner)?;
    let summoner_lane = summoner_slot.0 / slots_per_lane;

    // Determine the row range to search
    let (row_start, row_end) = match placement {
        SummonPlacement::FrontRow => {
            // Front row is the first half of slots in the lane
            let front_boundary = slots_per_lane / 2;
            (0, front_boundary)
        }
        SummonPlacement::BackRow => {
            // Back row is the second half of slots in the lane
            let front_boundary = slots_per_lane / 2;
            (front_boundary, slots_per_lane)
        }
        SummonPlacement::AnyEmpty => (0, slots_per_lane),
    };

    // Search for an empty slot in the specified range
    for row in row_start..row_end {
        let slot_index = summoner_lane * slots_per_lane + row;
        let slot = SlotIndex(slot_index);

        // Check if slot is empty using occupant_at
        if formation.occupant_at(slot).is_none() {
            // Also verify no actor in our actors map at this slot
            let slot_occupied = actors.values().any(|a| {
                formation
                    .find_actor(a.id)
                    .map(|s| s == slot)
                    .unwrap_or(false)
            });

            if !slot_occupied {
                return Some(slot);
            }
        }
    }

    // If no slot found in summon's lane, search other lanes
    for lane in 0..lanes {
        if lane == summoner_lane {
            continue;
        }
        for row in row_start..row_end {
            let slot_index = lane * slots_per_lane + row;
            let slot = SlotIndex(slot_index);

            if formation.occupant_at(slot).is_none() {
                let slot_occupied = actors.values().any(|a| {
                    formation
                        .find_actor(a.id)
                        .map(|s| s == slot)
                        .unwrap_or(false)
                });

                if !slot_occupied {
                    return Some(slot);
                }
            }
        }
    }

    None
}

/// Materializes summon events by creating real actors in the encounter state.
///
/// This function is the core of US-708: it consumes `SummonEvent` data created
/// by `extract_summon_events()` and actually creates actors, adds them to the
/// formation, and registers them with the encounter turn order.
///
/// # Arguments
///
/// * `events` — summon events extracted from resolved skill effects
/// * `actors` — the encounter's actor map (mutated in place)
/// * `formation` — the encounter's formation layout (mutated in place)
/// * `encounter` — the encounter state (mutated to add new actors to turn order)
/// * `content_pack` — access to archetype definitions for summoned families
/// * `monster_registry` — access to monster family definitions
/// * `tracker` — deduplication tracker (mutated to record materializations)
/// * `next_enemy_id` — the next available enemy ActorId (mutated if new actors added)
///
/// # Returns
///
/// The updated next_enemy_id value (incremented for each new actor created).
///
/// # Deduplication
///
/// If a summon event's kind has `needs_dedup() == true`, it will only be
/// materialized once per (summoner, summon_kind) pair per battle. Subsequent
/// events with the same key are silently skipped.
// Allow: too_many_arguments — function requires all these parameters for current architecture
// Refactoring to reduce arguments would require significant changes to call sites
#[allow(clippy::too_many_arguments)]
pub fn materialize_summons(
    events: &[SummonEvent],
    actors: &mut HashMap<ActorId, ActorAggregate>,
    formation: &mut FormationLayout,
    encounter: &mut Encounter,
    content_pack: &ContentPack,
    monster_registry: &MonsterFamilyRegistry,
    tracker: &mut SummonTracker,
    mut next_enemy_id: u64,
) -> u64 {
    for event in events {
        // Check deduplication
        if !tracker.can_materialize(event) {
            continue;
        }

        // Check if summoner still exists
        if !actors.contains_key(&event.summoner) {
            continue;
        }

        // Map summon kind to a specific family ID
        let Some(family_id) = family_id_for_summon_kind(event.summon_kind, event.summoner) else {
            // Generic summon kind - skip
            continue;
        };

        // Look up the family in the monster registry
        let family = match monster_registry.get(&family_id) {
            Some(f) => f,
            None => continue,
        };

        // Get the archetype from the content pack
        let archetype = match content_pack.get_archetype(&family.archetype_name) {
            Some(a) => a,
            None => continue,
        };

        // Determine how many units to summon (respect count from event)
        let count = event.count.min(3); // Cap at 3 to prevent abuse

        for _ in 0..count {
            // Find a placement slot
            let Some(slot) = find_placement_slot(event.placement, event.summoner, formation, actors)
            else {
                // No available slot
                break;
            };

            // Create the actor
            let actor_id = ActorId(next_enemy_id);
            let actor = archetype.create_actor(actor_id);

            // Place in formation
            if formation.place(actor_id, slot).is_err() {
                // Slot became occupied - try next ID
                next_enemy_id += 1;
                continue;
            }

            // Add to actors map
            actors.insert(actor_id, actor);

            // Add to encounter's enemy side directly (encounter.sides is public)
            // Summoned units are always enemies (same side as summoner)
            encounter
                .sides
                .entry(CombatSide::Enemy)
                .or_default()
                .push(actor_id);

            // Add to turn order queue if turn order exists (for deterministic ordering)
            // Insert at the front so they act soon (after current actor finishes)
            if let Some(ref mut turn_order) = encounter.turn_order {
                // Insert at the front of the queue so they act soon
                // This maintains deterministic order: newest summoned actors act first among summoned
                turn_order.queue.push_front(actor_id);
                // Also add to original_order for round-reset logic
                turn_order.original_order.push(actor_id);
            }

            // Record materialization for deduplication
            tracker.record_materialization(event);

            next_enemy_id += 1;
        }
    }

    next_enemy_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summon_tracker_new_is_empty() {
        let tracker = SummonTracker::new();
        assert!(tracker.materialized.is_empty());
    }

    #[test]
    fn summon_tracker_allows_non_dedup_kinds() {
        let mut tracker = SummonTracker::new();
        let event = SummonEvent::new(
            ActorId(10),
            SummonKind::MahjongTile,
            SummonPlacement::FrontRow,
            1,
        );

        // MahjongTile doesn't need dedup - should always be allowed
        assert!(tracker.can_materialize(&event));
        tracker.record_materialization(&event);
        assert!(tracker.can_materialize(&event)); // Should still be allowed
    }

    #[test]
    fn summon_tracker_blocks_dedup_kinds_after_first() {
        let mut tracker = SummonTracker::new();
        let event = SummonEvent::new(
            ActorId(10),
            SummonKind::RottenFruit,
            SummonPlacement::FrontRow,
            1,
        );

        // RottenFruit needs dedup - should be allowed once
        assert!(tracker.can_materialize(&event));
        tracker.record_materialization(&event);

        // Second materialization should be blocked
        assert!(!tracker.can_materialize(&event));
    }

    #[test]
    fn summon_tracker_tracks_by_summoner() {
        let mut tracker = SummonTracker::new();
        let event1 = SummonEvent::new(
            ActorId(10),
            SummonKind::RottenFruit,
            SummonPlacement::FrontRow,
            1,
        );
        let event2 = SummonEvent::new(
            ActorId(20),
            SummonKind::RottenFruit,
            SummonPlacement::FrontRow,
            1,
        );

        // Different summoners - both should be allowed
        assert!(tracker.can_materialize(&event1));
        tracker.record_materialization(&event1);
        assert!(!tracker.can_materialize(&event1)); // First summoner blocked

        assert!(tracker.can_materialize(&event2)); // Different summoner allowed
    }

    #[test]
    fn family_id_for_summon_kind_mahjong_tile() {
        // id % 3 == 0 → red
        let family = family_id_for_summon_kind(SummonKind::MahjongTile, ActorId(9));
        assert_eq!(family.as_deref(), Some("mahjong_red"));

        // id % 3 == 1 → green
        let family = family_id_for_summon_kind(SummonKind::MahjongTile, ActorId(10));
        assert_eq!(family.as_deref(), Some("mahjong_green"));

        // id % 3 == 2 → white
        let family = family_id_for_summon_kind(SummonKind::MahjongTile, ActorId(11));
        assert_eq!(family.as_deref(), Some("mahjong_white"));
    }

    #[test]
    fn family_id_for_summon_kind_rotten_fruit() {
        let family = family_id_for_summon_kind(SummonKind::RottenFruit, ActorId(5));
        assert_eq!(family.as_deref(), Some("rotten_fruit_A"));
    }

    #[test]
    fn family_id_for_summon_kind_ghost_fire_clone() {
        let family = family_id_for_summon_kind(SummonKind::GhostFireClone, ActorId(5));
        assert_eq!(family.as_deref(), Some("ghost_fire_assist"));
    }

    #[test]
    fn family_id_for_summon_kind_generic_returns_none() {
        let family = family_id_for_summon_kind(SummonKind::Generic, ActorId(5));
        assert_eq!(family, None);
    }
}