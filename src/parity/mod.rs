//! Semantic parity module — fixtures and expectations for verifying migration fidelity.
//!
//! Each submodule defines parity fixtures for a specific game system
//! (heroes, monsters, skills, statuses). The fixtures capture the original
//! DDGC behavioral expectations that must hold in the headless migration.

pub mod heroes;

pub use heroes::*;
