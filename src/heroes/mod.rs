//! Hero family module — registry, helpers, and variant definitions for DDGC hero class families.
//!
//! This module provides the family-aware view of DDGC hero classes, where each
//! recruitable profession is represented as a base/white/black variant set.
//! The `families` submodule contains the registry and chaos-mode resolution.
//! The `base` submodule contains base variant definitions for all recruitable families.
//! The `white` submodule contains white (+1) variant definitions.
//! The `black` submodule contains black (+2) variant definitions.
//! The `skills` submodule provides variant-aware skill pack resolution.
//! The `statuses` submodule provides variant-aware status semantics.

pub mod base;
pub mod black;
pub mod families;
pub mod skills;
pub mod statuses;
pub mod white;

pub use base::*;
pub use black::*;
pub use families::*;
pub use skills::FamilySkillResolver;
pub use statuses::{FamilyStatusRegistry, FamilyStatusSemantics, VariantStatusProfile};
pub use white::*;
