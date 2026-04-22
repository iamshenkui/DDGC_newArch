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
//! The `progress` submodule provides hero XP and leveling progression.

pub mod base;
pub mod black;
pub mod families;
pub mod progress;
pub mod recruitment;
pub mod skills;
pub mod stats;
pub mod statuses;
pub mod white;

pub use base::*;
pub use black::*;
pub use families::*;
pub use progress::{HeroProgress, LEVEL_THRESHOLD_TABLE, RESOLVE_THRESHOLD_TABLE};
pub use recruitment::{
    hero_class_identity, hero_matches_any_class_requirement, hero_matches_class_requirement,
    is_base_recruit_class, normalize_recruit_class_id, HeroClassIdentity, RecruitPool,
};
pub use skills::FamilySkillResolver;
pub use stats::{compute_hero_stats, BASE_STATS};
pub use statuses::{FamilyStatusRegistry, FamilyStatusSemantics, VariantStatusProfile};
pub use white::*;
