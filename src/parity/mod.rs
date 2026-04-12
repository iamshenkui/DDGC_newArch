//! Semantic parity module — fixtures and expectations for verifying migration fidelity.
//!
//! Each submodule defines parity fixtures for a specific game system
//! (heroes, monsters, skills, statuses). The fixtures capture the original
//! DDGC behavioral expectations that must hold in the headless migration.

pub mod heroes;
pub mod monsters;
pub mod skills;
pub mod statuses;

pub use heroes::*;
pub use monsters::*;
pub use skills::*;
pub use statuses::*;
