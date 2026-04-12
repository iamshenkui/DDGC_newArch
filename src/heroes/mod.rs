//! Hero family module — registry, helpers, and variant definitions for DDGC hero class families.
//!
//! This module provides the family-aware view of DDGC hero classes, where each
//! recruitable profession is represented as a base/white/black variant set.
//! The `families` submodule contains the registry and chaos-mode resolution.
//! The `base` submodule contains base variant definitions for all recruitable families.

pub mod base;
pub mod families;

pub use base::*;
pub use families::*;
