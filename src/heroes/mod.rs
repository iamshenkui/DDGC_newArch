//! Hero family module — registry and helpers for DDGC hero class families.
//!
//! This module provides the family-aware view of DDGC hero classes, where each
//! recruitable profession is represented as a base/white/black variant set.
//! The `families` submodule contains the registry and chaos-mode resolution.

pub mod families;

pub use families::*;
