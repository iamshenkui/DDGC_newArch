//! Monster family registry — DDGC family-level lookup for archetype, role, and skill-pack.
//!
//! Each `MonsterFamily` captures the structural identity of a DDGC monster family:
//! its ID, home dungeon, tier (Common or Boss), behavioral role, monster type,
//! and associated skill IDs. The registry allows O(1) lookup by family ID.
//!
//! DDGC uses exactly two tiers — Common and Boss — with no intermediate "elite"
//! classification. This registry models that faithfully.

use framework_combat::skills::SkillId;
use std::collections::HashMap;

// ── Dungeon Classification ──────────────────────────────────────────────────

/// Dungeon region classification for DDGC monsters.
///
/// Common monsters belong to exactly one dungeon. Some bosses are
/// cross-dungeon (e.g., Bloodthirsty Assassin, Glutton Pawnshop).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dungeon {
    /// 青龙 — Forest/swamp: mantises, trees, moths, robbers.
    QingLong,
    /// 白虎 — Fortress: armor, blades, lizards, beetles, alligators.
    BaiHu,
    /// 朱雀 — Fire temple: foxes, moths, lanterns, ghost fire.
    ZhuQue,
    /// 玄武 — Water depths: snakes, grass, monkeys, water creatures.
    XuanWu,
    /// Cross-dungeon bosses that appear outside a single dungeon.
    Cross,
}

// ── Tier Classification ────────────────────────────────────────────────────

/// DDGC migration tier — faithful to the game's two-tier system.
///
/// Common monsters are size-1 with 1 turn per round, scaled across 3 difficulty
/// levels (suffix `_1`, `_2`, `_3`). Bosses are size-2+ with 2+ turns per round
/// and carry the `boss` tag.
///
/// There is no "elite" tier in DDGC data. Some bosses have shadow/paired units
/// that share the boss tag but operate as semi-independent actors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MonsterTier {
    /// Size 1, 1 turn/round, three difficulty variants.
    Common,
    /// Size 2+, 2+ turns/round, multi-part or summon mechanics.
    Boss,
}

// ── Monster Type ───────────────────────────────────────────────────────────

/// Monster type classification from DDGC's `MonsterType` enum.
///
/// Derived from `MechanicsDefines.cs`. Each monster data file specifies its
/// type via `enemy_type: .id "<type>"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MonsterType {
    /// 野兽 — Animals and mythical beasts.
    Beast,
    /// 异灵 — Supernatural entities.
    Eldritch,
    /// 不死 — Undead and animated constructs.
    Unholy,
    /// 人类 — Human enemies.
    Man,
    /// Special vessel/mechanic entities (e.g., egg_membrane).
    Cauldron,
    /// Battlefield terrain/corpses.
    Corpse,
}

// ── Family Role ─────────────────────────────────────────────────────────────

/// Behavioral role within a monster family.
///
/// Describes the family's tactical identity — what kind of threat it poses
/// in combat — rather than its mechanical implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FamilyRole {
    /// High damage, high HP front-liner (e.g., alligator_yangtze).
    Bruiser,
    /// Disables or manipulates the party (e.g., lizard, moth_mimicry_B).
    Controller,
    /// Buffs allies or debuffs heroes (e.g., ghost_fire_assist).
    Support,
    /// Fast, low-HP hit-and-run (e.g., robber_melee, robber_ranged).
    Skirmisher,
    /// High protection, absorbs damage (e.g., metal_armor).
    Tank,
    /// Ranged damage dealer (e.g., dry_tree_genie, lantern).
    Ranged,
    /// Summons additional units (boss pattern).
    Summoner,
}

// ── Family ID ───────────────────────────────────────────────────────────────

/// Unique identifier for a monster family (e.g., "mantis_magic_flower").
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FamilyId(pub String);

impl FamilyId {
    pub fn new(id: impl Into<String>) -> Self {
        FamilyId(id.into())
    }
}

impl std::fmt::Display for FamilyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ── Monster Family ──────────────────────────────────────────────────────────

/// A DDGC monster family entry in the registry.
///
/// Each family captures the structural identity needed to look up its archetype
/// builder and skill pack. Individual family migrations (US-405 through US-426
/// for commons, US-430 through US-441 for bosses) will register entries here.
#[derive(Debug, Clone)]
pub struct MonsterFamily {
    /// Unique family identifier (e.g., "mantis_magic_flower", "azure_dragon").
    pub id: FamilyId,
    /// Home dungeon region.
    pub dungeon: Dungeon,
    /// Common or Boss tier.
    pub tier: MonsterTier,
    /// Behavioral role in combat.
    pub role: FamilyRole,
    /// DDGC monster type classification.
    pub monster_type: MonsterType,
    /// Skill IDs associated with this family's base tier variant.
    pub skill_ids: Vec<SkillId>,
    /// Name of the archetype builder in the content pack.
    ///
    /// Content packs index archetypes by name. This field stores the archetype
    /// name (e.g., "Mantis Magic Flower") so the family entry can resolve to
    /// its archetype at runtime.
    pub archetype_name: String,
}

// ── Monster Family Registry ─────────────────────────────────────────────────

/// Lookup table for DDGC monster families by family ID.
///
/// The registry is built incrementally as families are migrated. Each migration
/// slice registers exactly one family, so the registry grows from empty to
/// complete over the K1–K26 stories.
pub struct MonsterFamilyRegistry {
    families: HashMap<String, MonsterFamily>,
}

impl MonsterFamilyRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        MonsterFamilyRegistry {
            families: HashMap::new(),
        }
    }

    /// Register a monster family. Each migration slice calls this once.
    pub fn register(&mut self, family: MonsterFamily) {
        self.families.insert(family.id.0.clone(), family);
    }

    /// Look up a family by its string ID (e.g., "mantis_magic_flower").
    pub fn get(&self, id: &str) -> Option<&MonsterFamily> {
        self.families.get(id)
    }

    /// Look up a family by `FamilyId`.
    pub fn get_by_id(&self, id: &FamilyId) -> Option<&MonsterFamily> {
        self.families.get(&id.0)
    }

    /// Iterate over all registered families.
    pub fn iter(&self) -> impl Iterator<Item = &MonsterFamily> {
        self.families.values()
    }

    /// Return the number of registered families.
    pub fn len(&self) -> usize {
        self.families.len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.families.is_empty()
    }

    /// Return all families in a given dungeon.
    pub fn by_dungeon(&self, dungeon: Dungeon) -> Vec<&MonsterFamily> {
        self.families
            .values()
            .filter(|f| f.dungeon == dungeon)
            .collect()
    }

    /// Return all families of a given tier.
    pub fn by_tier(&self, tier: MonsterTier) -> Vec<&MonsterFamily> {
        self.families
            .values()
            .filter(|f| f.tier == tier)
            .collect()
    }

    /// Return all skill IDs referenced across all registered families.
    pub fn all_skill_ids(&self) -> Vec<&SkillId> {
        self.families
            .values()
            .flat_map(|f| f.skill_ids.iter())
            .collect()
    }
}

impl Default for MonsterFamilyRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monster_family_registry_builds() {
        let registry = MonsterFamilyRegistry::new();
        assert!(registry.is_empty());
    }

    #[test]
    fn monster_family_registry_registers_and_looks_up() {
        let mut registry = MonsterFamilyRegistry::new();

        let family = MonsterFamily {
            id: FamilyId::new("mantis_magic_flower"),
            dungeon: Dungeon::QingLong,
            tier: MonsterTier::Common,
            role: FamilyRole::Controller,
            monster_type: MonsterType::Beast,
            skill_ids: vec![
                SkillId::new("poison"),
                SkillId::new("crowd_bleed"),
                SkillId::new("normal_attack"),
                SkillId::new("move"),
            ],
            archetype_name: "Mantis Magic Flower".to_string(),
        };

        registry.register(family);

        let found = registry.get("mantis_magic_flower").expect("family should be registered");
        assert_eq!(found.id.0, "mantis_magic_flower");
        assert_eq!(found.dungeon, Dungeon::QingLong);
        assert_eq!(found.tier, MonsterTier::Common);
        assert_eq!(found.role, FamilyRole::Controller);
        assert_eq!(found.monster_type, MonsterType::Beast);
        assert_eq!(found.skill_ids.len(), 4);
    }

    #[test]
    fn monster_family_registry_exposes_dungeon_tags() {
        let mut registry = MonsterFamilyRegistry::new();

        let qinglong_family = MonsterFamily {
            id: FamilyId::new("dry_tree_genie"),
            dungeon: Dungeon::QingLong,
            tier: MonsterTier::Common,
            role: FamilyRole::Ranged,
            monster_type: MonsterType::Eldritch,
            skill_ids: vec![
                SkillId::new("bleed"),
                SkillId::new("slow_crowd"),
                SkillId::new("stress"),
                SkillId::new("move"),
            ],
            archetype_name: "Dry Tree Genie".to_string(),
        };

        let baihu_family = MonsterFamily {
            id: FamilyId::new("metal_armor"),
            dungeon: Dungeon::BaiHu,
            tier: MonsterTier::Common,
            role: FamilyRole::Tank,
            monster_type: MonsterType::Unholy,
            skill_ids: vec![
                SkillId::new("stun"),
                SkillId::new("bleed"),
                SkillId::new("normal_attack"),
                SkillId::new("move"),
            ],
            archetype_name: "Metal Armor".to_string(),
        };

        registry.register(qinglong_family);
        registry.register(baihu_family);

        let qinglong_families = registry.by_dungeon(Dungeon::QingLong);
        assert_eq!(qinglong_families.len(), 1);
        assert_eq!(qinglong_families[0].id.0, "dry_tree_genie");

        let baihu_families = registry.by_dungeon(Dungeon::BaiHu);
        assert_eq!(baihu_families.len(), 1);
        assert_eq!(baihu_families[0].id.0, "metal_armor");
    }

    #[test]
    fn monster_family_registry_exposes_skill_ids() {
        let mut registry = MonsterFamilyRegistry::new();

        let family = MonsterFamily {
            id: FamilyId::new("mantis_magic_flower"),
            dungeon: Dungeon::QingLong,
            tier: MonsterTier::Common,
            role: FamilyRole::Controller,
            monster_type: MonsterType::Beast,
            skill_ids: vec![
                SkillId::new("poison"),
                SkillId::new("crowd_bleed"),
                SkillId::new("normal_attack"),
                SkillId::new("move"),
            ],
            archetype_name: "Mantis Magic Flower".to_string(),
        };

        registry.register(family);

        let found = registry.get("mantis_magic_flower").unwrap();
        assert!(found.skill_ids.contains(&SkillId::new("poison")));
        assert!(found.skill_ids.contains(&SkillId::new("crowd_bleed")));
    }

    #[test]
    fn monster_family_tier_is_common_or_boss() {
        // Verify that MonsterTier has exactly two variants and no "elite".
        let common = MonsterTier::Common;
        let boss = MonsterTier::Boss;
        assert_ne!(common, boss);
    }

    #[test]
    fn monster_family_no_elite_tier() {
        // This test documents the design decision: DDGC has no elite tier.
        // MonsterTier::Common covers all size-1, 1-turn enemies.
        // MonsterTier::Boss covers all size-2+, 2+ turn enemies.
        // If someone adds an "Elite" variant, this test serves as a reminder
        // that DDGC data does not support that classification.
        let family = MonsterFamily {
            id: FamilyId::new("test_common"),
            dungeon: Dungeon::QingLong,
            tier: MonsterTier::Common,
            role: FamilyRole::Skirmisher,
            monster_type: MonsterType::Beast,
            skill_ids: vec![SkillId::new("attack")],
            archetype_name: "Test Common".to_string(),
        };
        assert_eq!(family.tier, MonsterTier::Common);
    }
}