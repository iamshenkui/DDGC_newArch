//! CLI tools for DDGC headless migration.
//!
//! This module provides command-line tools for inspecting and auditing
//! the migration state of DDGC asset families.

use crate::content::ContentPack;
use crate::heroes::families::HeroFamilyRegistry;
use crate::monsters::families::MonsterFamily;

/// Migration status for an asset family.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationStatus {
    /// Family is fully migrated with all content.
    Migrated,
    /// Family has partial content (some skills/statuses missing).
    Partial,
    /// Family is not yet migrated.
    Missing,
}

/// Asset family information for the audit.
#[derive(Debug, Clone)]
pub struct FamilyAuditEntry {
    /// Family identifier (e.g., "alchemist", "mantis_magic_flower").
    pub family_id: String,
    /// Type of family: "hero" or "monster".
    pub family_type: FamilyType,
    /// Migration status.
    pub status: MigrationStatus,
    /// Parser module path (e.g., "src/content/heroes/alchemist.rs").
    pub parser_module: Option<String>,
    /// Registry location (e.g., "HeroFamilyRegistry" or "MonsterFamilyRegistry").
    pub registry: Option<String>,
    /// Runtime anchor (e.g., "ContentPack::default()" or "build_registry()").
    pub runtime_anchor: Option<String>,
    /// Next implementation target for partial or missing families.
    pub next_target: Option<String>,
    /// Source data reference (DDGC original asset path).
    pub source_reference: String,
}

/// Type of asset family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FamilyType {
    Hero,
    Monster,
}

/// Generate the complete asset gap inventory audit.
///
/// This function inspects the actual content modules and registries
/// to produce a comprehensive audit of all DDGC asset families.
pub fn generate_asset_audit() -> Vec<FamilyAuditEntry> {
    let pack = ContentPack::default();
    let hero_registry = HeroFamilyRegistry::new();
    let monster_registry = crate::monsters::build_registry();

    let mut entries = Vec::new();

    // Audit hero families
    for family in hero_registry.all_families() {
        let entry = audit_hero_family(family, &pack);
        entries.push(entry);
    }

    // Audit monster families
    for family in monster_registry.iter() {
        let entry = audit_monster_family(family, &pack);
        entries.push(entry);
    }

    entries
}

fn audit_hero_family(
    family: &crate::heroes::families::HeroClassFamily,
    pack: &ContentPack,
) -> FamilyAuditEntry {
    let family_id = family.base_id.to_string();

    // Archetypes are stored with display names (e.g., "Alchemist", "Alchemist (White)", "Alchemist (Black)")
    let base_name = capitalize(&family.base_id);
    let white_name = format!("{} (White)", capitalize(&family.base_id));
    let black_name = format!("{} (Black)", capitalize(&family.base_id));

    // Check if all variants are migrated
    let base_migrated = pack.get_archetype(&base_name).is_some();
    let white_migrated = pack.get_archetype(&white_name).is_some();
    let black_migrated = pack.get_archetype(&black_name).is_some();

    let all_migrated = base_migrated && white_migrated && black_migrated;
    let any_migrated = base_migrated || white_migrated || black_migrated;

    let status = if all_migrated {
        MigrationStatus::Migrated
    } else if any_migrated {
        MigrationStatus::Partial
    } else {
        MigrationStatus::Missing
    };

    // Determine parser module based on hero type
    let parser_module = format!("src/content/heroes/{}.rs", family.base_id);

    // Registry is always HeroFamilyRegistry
    let registry = Some("HeroFamilyRegistry".to_string());

    // Runtime anchor is ContentPack
    let runtime_anchor = Some("ContentPack::default()".to_string());

    // Next target for partial or missing
    let next_target = if !all_migrated {
        let mut missing = Vec::new();
        if !base_migrated {
            missing.push(format!("{}_archetype()", family.base_id));
        }
        if !white_migrated {
            missing.push(format!("white::{}_archetype()", family.base_id));
        }
        if !black_migrated {
            missing.push(format!("black::{}_archetype()", family.base_id));
        }
        Some(format!(
            "Implement: {} in {}",
            missing.join(", "),
            parser_module
        ))
    } else {
        None
    };

    // Source reference - DDGC hero data files
    let source_reference = format!(
        "Assets/Resources/Data/Heroes/{}.bytes (base), {}.bytes (white), {}.bytes (black)",
        capitalize(&family.base_id),
        capitalize(&family.base_id),
        capitalize(&family.base_id)
    );

    FamilyAuditEntry {
        family_id,
        family_type: FamilyType::Hero,
        status,
        parser_module: Some(parser_module),
        registry,
        runtime_anchor,
        next_target,
        source_reference,
    }
}

fn audit_monster_family(family: &MonsterFamily, pack: &ContentPack) -> FamilyAuditEntry {
    let family_id = family.id.0.clone();

    // Check if archetype is migrated
    let archetype_migrated = pack.get_archetype(&family.archetype_name).is_some();

    // Check if all skills are migrated
    let mut skills_missing = Vec::new();
    for skill_id in &family.skill_ids {
        if pack.get_skill(skill_id).is_none() {
            skills_missing.push(skill_id.0.clone());
        }
    }

    let status = if archetype_migrated && skills_missing.is_empty() {
        MigrationStatus::Migrated
    } else if archetype_migrated {
        MigrationStatus::Partial
    } else {
        MigrationStatus::Missing
    };

    // Parser module
    let parser_module = Some(format!("src/content/monsters/{}.rs", family_id));

    // Registry
    let registry = Some("MonsterFamilyRegistry".to_string());

    // Runtime anchor
    let runtime_anchor = Some("monsters::register_content()".to_string());

    // Next target
    let next_target = if !skills_missing.is_empty() {
        Some(format!("Implement skills: {} in src/content/monsters/{}.rs", skills_missing.join(", "), family_id))
    } else if !archetype_migrated {
        Some(format!("Implement archetype() in src/content/monsters/{}.rs", family_id))
    } else {
        None
    };

    // Source reference - DDGC monster data files
    let source_reference = format!(
        "Assets/Resources/Data/Monsters/{}_1.txt (tier 1)",
        family_id.replace('_', "_")
    );

    FamilyAuditEntry {
        family_id,
        family_type: FamilyType::Monster,
        status,
        parser_module,
        registry,
        runtime_anchor,
        next_target,
        source_reference,
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Format the audit as a markdown table.
pub fn format_audit_markdown(entries: &[FamilyAuditEntry]) -> String {
    let mut output = String::new();

    output.push_str("# DDGC Asset Gap Inventory\n\n");
    output.push_str("## Hero Families\n\n");
    output.push_str("| Family | Status | Parser | Registry | Runtime Anchor | Next Target | Source |\n");
    output.push_str("|--------|--------|--------|----------|-----------------|-------------|--------|\n");

    for entry in entries.iter().filter(|e| e.family_type == FamilyType::Hero) {
        output.push_str(&format_hero_row(entry));
    }

    output.push_str("\n## Monster Families\n\n");
    output.push_str("| Family | Dungeon | Tier | Status | Parser | Registry | Next Target | Source |\n");
    output.push_str("|--------|---------|------|--------|--------|----------|-------------|--------|\n");

    let monster_registry = crate::monsters::build_registry();
    for entry in entries.iter().filter(|e| e.family_type == FamilyType::Monster) {
        let dungeon = monster_registry
            .get(&entry.family_id)
            .map(|f| format!("{:?}", f.dungeon))
            .unwrap_or_else(|| "Unknown".to_string());
        let tier = monster_registry
            .get(&entry.family_id)
            .map(|f| format!("{:?}", f.tier))
            .unwrap_or_else(|| "Unknown".to_string());

        output.push_str(&format_monster_row(entry, &dungeon, &tier));
    }

    output.push_str("\n## Summary\n\n");

    let hero_migrated = entries.iter().filter(|e| e.family_type == FamilyType::Hero && e.status == MigrationStatus::Migrated).count();
    let hero_partial = entries.iter().filter(|e| e.family_type == FamilyType::Hero && e.status == MigrationStatus::Partial).count();
    let hero_missing = entries.iter().filter(|e| e.family_type == FamilyType::Hero && e.status == MigrationStatus::Missing).count();

    let monster_migrated = entries.iter().filter(|e| e.family_type == FamilyType::Monster && e.status == MigrationStatus::Migrated).count();
    let monster_partial = entries.iter().filter(|e| e.family_type == FamilyType::Monster && e.status == MigrationStatus::Partial).count();
    let monster_missing = entries.iter().filter(|e| e.family_type == FamilyType::Monster && e.status == MigrationStatus::Missing).count();

    output.push_str(&format!("- **Hero Families**: {} migrated, {} partial, {} missing\n", hero_migrated, hero_partial, hero_missing));
    output.push_str(&format!("- **Monster Families**: {} migrated, {} partial, {} missing\n", monster_migrated, monster_partial, monster_missing));

    output
}

fn format_hero_row(entry: &FamilyAuditEntry) -> String {
    format!(
        "| {} | {:?} | {} | {} | {} | {} | {} |\n",
        entry.family_id,
        entry.status,
        entry.parser_module.as_deref().unwrap_or("-"),
        entry.registry.as_deref().unwrap_or("-"),
        entry.runtime_anchor.as_deref().unwrap_or("-"),
        entry.next_target.as_deref().unwrap_or("-"),
        entry.source_reference,
    )
}

fn format_monster_row(entry: &FamilyAuditEntry, dungeon: &str, tier: &str) -> String {
    format!(
        "| {} | {} | {} | {:?} | {} | {} | {} | {} |\n",
        entry.family_id,
        dungeon,
        tier,
        entry.status,
        entry.parser_module.as_deref().unwrap_or("-"),
        entry.registry.as_deref().unwrap_or("-"),
        entry.next_target.as_deref().unwrap_or("-"),
        entry.source_reference,
    )
}

/// CLI command to run the asset audit and print to stdout.
pub fn run_audit() {
    let entries = generate_asset_audit();
    let markdown = format_audit_markdown(&entries);
    println!("{}", markdown);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hero_families_are_audited() {
        let entries = generate_asset_audit();
        let hero_entries: Vec<_> = entries
            .iter()
            .filter(|e| e.family_type == FamilyType::Hero)
            .collect();

        // 5 hero families
        assert_eq!(hero_entries.len(), 5);
    }

    #[test]
    fn monster_families_are_audited() {
        let entries = generate_asset_audit();
        let monster_entries: Vec<_> = entries
            .iter()
            .filter(|e| e.family_type == FamilyType::Monster)
            .collect();

        // Should have all monster families from the registry
        let registry = crate::monsters::build_registry();
        assert_eq!(monster_entries.len(), registry.len());
    }

    #[test]
    fn all_migrated_heroes_show_migrated_status() {
        let entries = generate_asset_audit();
        for entry in entries.iter().filter(|e| e.family_type == FamilyType::Hero) {
            assert_eq!(
                entry.status,
                MigrationStatus::Migrated,
                "Hero {} should be migrated",
                entry.family_id
            );
        }
    }

    #[test]
    fn audit_identifies_parser_for_each_family() {
        let entries = generate_asset_audit();
        for entry in &entries {
            assert!(
                entry.parser_module.is_some(),
                "Family {} should have parser_module",
                entry.family_id
            );
        }
    }

    #[test]
    fn audit_identifies_registry_for_each_family() {
        let entries = generate_asset_audit();
        for entry in &entries {
            assert!(
                entry.registry.is_some(),
                "Family {} should have registry",
                entry.family_id
            );
        }
    }

    #[test]
    fn audit_identifies_runtime_anchor_for_each_family() {
        let entries = generate_asset_audit();
        for entry in &entries {
            assert!(
                entry.runtime_anchor.is_some(),
                "Family {} should have runtime_anchor",
                entry.family_id
            );
        }
    }

    #[test]
    fn markdown_output_contains_all_sections() {
        let entries = generate_asset_audit();
        let markdown = format_audit_markdown(&entries);

        assert!(markdown.contains("# DDGC Asset Gap Inventory"));
        assert!(markdown.contains("## Hero Families"));
        assert!(markdown.contains("## Monster Families"));
        assert!(markdown.contains("## Summary"));
    }
}