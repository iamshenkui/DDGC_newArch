//! DDGC run flow — migrated dungeon progression on framework_progression.
//!
//! Implements Batch 5 of the migration map: run flow using
//! `DefaultRoomGenerator` with DDGC-appropriate room weights,
//! room-by-room progression, and post-battle reward application.

pub mod encounters;
pub mod flow;
pub mod guard_detection;
pub mod guard_redirect_execution;
pub mod reactive_events;
pub mod reactive_queue;
pub mod rewards;
pub mod riposte_detection;
pub mod riposte_execution;
