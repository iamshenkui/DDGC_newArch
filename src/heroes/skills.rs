//! Family-aware skill pack resolution for DDGC hero class families.
//!
//! Each hero family has three distinct skill packs (base, white, black) that
//! differ in effect chains. The `FamilySkillResolver` maps (family, chaos_mode)
//! to the correct skill pack, preventing the game layer from needing to know
//! about variant content paths or treating base/white/black as unrelated
//! skill sets.

use crate::heroes::families::{ChaosMode, HeroFamilyRegistry};
use framework_combat::effects::EffectKind;
use framework_combat::skills::SkillDefinition;

// ── Skill Pack Resolver ────────────────────────────────────────────────────

/// Resolves variant-aware skill packs for hero class families.
///
/// Each family has three distinct skill packs (base, white, black) that
/// differ in effect chains. The resolver maps (family, chaos_mode) to
/// the correct skill pack without the game layer needing to know about
/// variant content paths.
pub struct FamilySkillResolver {
    registry: HeroFamilyRegistry,
}

impl FamilySkillResolver {
    /// Create a new resolver with the hero family registry.
    pub fn new() -> Self {
        FamilySkillResolver {
            registry: HeroFamilyRegistry::new(),
        }
    }

    /// Resolve the skill pack for a family at a given chaos mode.
    ///
    /// Returns `None` if the family is not found in the registry.
    /// Returns the variant-specific skill pack for the given mode.
    pub fn resolve_skill_pack(&self, base_id: &str, mode: ChaosMode) -> Option<Vec<SkillDefinition>> {
        // Verify the family exists in the registry
        self.registry.get_family_by_base(base_id)?;
        match mode {
            ChaosMode::Normal => resolve_base_skills(base_id),
            ChaosMode::White => resolve_white_skills(base_id),
            ChaosMode::Black => resolve_black_skills(base_id),
        }
    }

    /// Resolve the skill pack for a family given a chaos stored value.
    ///
    /// Convenience wrapper that resolves chaos mode first, then dispatches.
    pub fn resolve_skill_pack_by_chaos(&self, base_id: &str, chaos_value: u32) -> Option<Vec<SkillDefinition>> {
        let mode = ChaosMode::from_chaos_value(chaos_value);
        self.resolve_skill_pack(base_id, mode)
    }

    /// Get a reference to the underlying family registry.
    pub fn registry(&self) -> &HeroFamilyRegistry {
        &self.registry
    }
}

impl Default for FamilySkillResolver {
    fn default() -> Self {
        Self::new()
    }
}

// ── Internal Dispatch ──────────────────────────────────────────────────────

fn resolve_base_skills(base_id: &str) -> Option<Vec<SkillDefinition>> {
    match base_id {
        "alchemist" => Some(crate::content::heroes::alchemist::skill_pack()),
        "diviner" => Some(crate::content::heroes::diviner::skill_pack()),
        "hunter" => Some(crate::content::heroes::hunter::skill_pack()),
        "shaman" => Some(crate::content::heroes::shaman::skill_pack()),
        "tank" => Some(crate::content::heroes::tank::skill_pack()),
        _ => None,
    }
}

fn resolve_white_skills(base_id: &str) -> Option<Vec<SkillDefinition>> {
    match base_id {
        "alchemist" => Some(crate::content::heroes::white::alchemist_skill_pack()),
        "diviner" => Some(crate::content::heroes::white::diviner_skill_pack()),
        "hunter" => Some(crate::content::heroes::white::hunter_skill_pack()),
        "shaman" => Some(crate::content::heroes::white::shaman_skill_pack()),
        "tank" => Some(crate::content::heroes::white::tank_skill_pack()),
        _ => None,
    }
}

fn resolve_black_skills(base_id: &str) -> Option<Vec<SkillDefinition>> {
    match base_id {
        "alchemist" => Some(crate::content::heroes::black::alchemist_skill_pack()),
        "diviner" => Some(crate::content::heroes::black::diviner_skill_pack()),
        "hunter" => Some(crate::content::heroes::black::hunter_skill_pack()),
        "shaman" => Some(crate::content::heroes::black::shaman_skill_pack()),
        "tank" => Some(crate::content::heroes::black::tank_skill_pack()),
        _ => None,
    }
}

// ── Skill Pack Analysis Helpers ───────────────────────────────────────────

/// Extract the set of status kind names from a skill pack.
///
/// Inspects all `ApplyStatus` effect nodes across all skills in the pack
/// and returns the unique status kind names, sorted for determinism.
pub fn extract_status_kinds(skills: &[SkillDefinition]) -> Vec<String> {
    let mut kinds = std::collections::HashSet::new();
    for skill in skills {
        for effect in &skill.effects {
            if effect.kind == EffectKind::ApplyStatus {
                if let Some(ref kind) = effect.status_kind {
                    kinds.insert(kind.clone());
                }
            }
        }
    }
    let mut result: Vec<String> = kinds.into_iter().collect();
    result.sort();
    result
}

/// Count the number of `ApplyStatus` effect nodes in a skill pack.
pub fn count_apply_status(skills: &[SkillDefinition]) -> usize {
    skills
        .iter()
        .flat_map(|s| &s.effects)
        .filter(|e| e.kind == EffectKind::ApplyStatus)
        .count()
}

/// Count the total number of effect nodes across all skills in a pack.
pub fn total_effect_count(skills: &[SkillDefinition]) -> usize {
    skills.iter().map(|s| s.effects.len()).sum()
}
