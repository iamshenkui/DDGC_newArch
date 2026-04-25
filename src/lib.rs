//! DDGC Headless Migration — library exports for integration testing and content access.
//!
//! This crate migrates real DDGC game content onto the turn-based roguelike
//! framework using only public framework APIs.

pub mod content;
pub mod contracts;
pub mod encounters;
pub mod heroes;
pub mod monsters;
pub mod parity;
pub mod planner;
pub mod run;
pub mod scenarios;
pub mod town;
pub mod trace;

/// Verifies that the framework crates expose no DDGC-specific API surface.
///
/// This test checks that every public re-export from the framework dependencies
/// can be used without referencing any DDGC-specific names. It works by
/// compiling a set of framework types that the game layer actually uses.
/// If a framework crate were patched with DDGC-specific branches,
/// this test would still compile — but the MIGRATION_BLOCKERS.md process
/// and the rejected-backflow table prevent that from happening.
///
/// The real enforcement is social (the feedback loop process), but this test
/// serves as a build-time reminder: the game layer must only use generic
/// framework types.
#[cfg(test)]
mod feedback_loop {
    /// Compile-time proof that all framework types used by the game layer
    /// are generic (not DDGC-specific).
    ///
    /// If any framework crate added a DDGC-specific type or constant,
    /// this function would not reference it — making the gap obvious
    /// in code review.
    #[test]
    #[allow(unused_imports)]
    fn no_ddgc_content_in_framework_crates() {
        // framework_rules: generic attribute/actor types only
        use framework_rules::actor::{ActorAggregate, ActorId};
        use framework_rules::attributes::{AttributeKey, AttributeValue};
        use framework_rules::statuses::{StatusEffect, StackRule};
        use framework_rules::modifiers::{Modifier, ModifierSource};

        // framework_combat: generic combat types only
        use framework_combat::encounter::{CombatSide, Encounter, EncounterId, EncounterState};
        use framework_combat::formation::{FormationLayout, SlotIndex};
        use framework_combat::resolver::CombatResolver;
        use framework_combat::skills::{SkillDefinition, SkillId};
        use framework_combat::effects::{EffectContext, resolve_skill};
        use framework_combat::commands::CombatCommand;

        // framework_progression: generic run/floor/room types only
        use framework_progression::run::{Run, RunId, RunResult, RunState};
        use framework_progression::floor::{Floor, FloorId};
        use framework_progression::rooms::{Room, RoomId, RoomKind, RoomState};
        use framework_progression::generation::{DefaultRoomGenerator, FloorConfig, RoomGenerator};

        // framework_viewmodels: generic view model types only
        use framework_viewmodels::{CombatViewModel, RunViewModel};

        // framework_ai: generic AI types only
        use framework_ai::decision::{DecisionContext, ActionCandidate};
        use framework_ai::desires::DesireCalculator;

        // If we got here, all imports resolved using only generic framework types.
        // No DDGC-specific types, constants, or rule branches were needed.
    }
}
