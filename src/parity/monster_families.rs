//! Monster family semantic parity fixtures — expectations for all migrated common families.
//!
//! These fixtures capture the original DDGC monster family identity: which dungeon
//! they belong to, their role, their monster type, and the skill IDs that define
//! their behavioral identity. Parity tests use these fixtures to verify the
//! migration preserves each family's distinct profile.

use crate::monsters::{Dungeon, FamilyRole, MonsterType};

/// Parity expectations for a single monster family.
#[derive(Debug, Clone)]
pub struct MonsterFamilyExpectation {
    pub family_id: &'static str,
    pub dungeon: Dungeon,
    pub role: FamilyRole,
    pub monster_type: MonsterType,
    /// Skill IDs that define this family's behavioral identity.
    pub identity_skills: &'static [&'static str],
}

/// Fixture bundle containing parity expectations for all migrated common families.
pub struct MonsterFamilyParityFixture {
    /// All 22 migrated common monster family expectations.
    pub families: Vec<MonsterFamilyExpectation>,
}

impl MonsterFamilyParityFixture {
    pub fn new() -> Self {
        MonsterFamilyParityFixture {
            families: vec![
                // ── QingLong ──────────────────────────────────────────────
                MonsterFamilyExpectation {
                    family_id: "mantis_magic_flower",
                    dungeon: Dungeon::QingLong,
                    role: FamilyRole::Controller,
                    monster_type: MonsterType::Beast,
                    identity_skills: &["poison", "crowd_bleed"],
                },
                MonsterFamilyExpectation {
                    family_id: "mantis_spiny_flower",
                    dungeon: Dungeon::QingLong,
                    role: FamilyRole::Controller,
                    monster_type: MonsterType::Beast,
                    identity_skills: &["ignore_armor", "crowd_bleed"],
                },
                MonsterFamilyExpectation {
                    family_id: "mantis_walking_flower",
                    dungeon: Dungeon::QingLong,
                    role: FamilyRole::Controller,
                    monster_type: MonsterType::Beast,
                    identity_skills: &["weak", "crowd_bleed"],
                },
                MonsterFamilyExpectation {
                    family_id: "dry_tree_genie",
                    dungeon: Dungeon::QingLong,
                    role: FamilyRole::Ranged,
                    monster_type: MonsterType::Eldritch,
                    identity_skills: &["bleed", "slow_crowd", "stress"],
                },
                MonsterFamilyExpectation {
                    family_id: "moth_mimicry_A",
                    dungeon: Dungeon::QingLong,
                    role: FamilyRole::Ranged,
                    monster_type: MonsterType::Eldritch,
                    identity_skills: &["poison", "stress_poison"],
                },
                MonsterFamilyExpectation {
                    family_id: "moth_mimicry_B",
                    dungeon: Dungeon::QingLong,
                    role: FamilyRole::Ranged,
                    monster_type: MonsterType::Eldritch,
                    identity_skills: &["poison", "stress_crowd"],
                },
                MonsterFamilyExpectation {
                    family_id: "robber_melee",
                    dungeon: Dungeon::QingLong,
                    role: FamilyRole::Skirmisher,
                    monster_type: MonsterType::Man,
                    identity_skills: &["smoke_bomb", "bleed"],
                },
                MonsterFamilyExpectation {
                    family_id: "robber_ranged",
                    dungeon: Dungeon::QingLong,
                    role: FamilyRole::Skirmisher,
                    monster_type: MonsterType::Man,
                    identity_skills: &["throw_stone", "multiple_shot"],
                },
                // ── BaiHu ────────────────────────────────────────────────
                MonsterFamilyExpectation {
                    family_id: "metal_armor",
                    dungeon: Dungeon::BaiHu,
                    role: FamilyRole::Tank,
                    monster_type: MonsterType::Unholy,
                    identity_skills: &["stun", "bleed"],
                },
                MonsterFamilyExpectation {
                    family_id: "tiger_sword",
                    dungeon: Dungeon::BaiHu,
                    role: FamilyRole::Bruiser,
                    monster_type: MonsterType::Unholy,
                    identity_skills: &["normal_attack", "pull"],
                },
                MonsterFamilyExpectation {
                    family_id: "lizard",
                    dungeon: Dungeon::BaiHu,
                    role: FamilyRole::Controller,
                    monster_type: MonsterType::Eldritch,
                    identity_skills: &["stun", "intimidate", "stress"],
                },
                MonsterFamilyExpectation {
                    family_id: "unicorn_beetle_A",
                    dungeon: Dungeon::BaiHu,
                    role: FamilyRole::Ranged,
                    monster_type: MonsterType::Eldritch,
                    identity_skills: &["bleed", "bleed_crowd"],
                },
                MonsterFamilyExpectation {
                    family_id: "unicorn_beetle_B",
                    dungeon: Dungeon::BaiHu,
                    role: FamilyRole::Ranged,
                    monster_type: MonsterType::Eldritch,
                    identity_skills: &["bleed", "stress"],
                },
                MonsterFamilyExpectation {
                    family_id: "alligator_yangtze",
                    dungeon: Dungeon::BaiHu,
                    role: FamilyRole::Bruiser,
                    monster_type: MonsterType::Beast,
                    identity_skills: &["bleed", "mark_riposte"],
                },
                // ── ZhuQue ────────────────────────────────────────────────
                MonsterFamilyExpectation {
                    family_id: "ghost_fire_assist",
                    dungeon: Dungeon::ZhuQue,
                    role: FamilyRole::Support,
                    monster_type: MonsterType::Eldritch,
                    identity_skills: &["assist", "ghost_fire_split"],
                },
                MonsterFamilyExpectation {
                    family_id: "ghost_fire_damage",
                    dungeon: Dungeon::ZhuQue,
                    role: FamilyRole::Ranged,
                    monster_type: MonsterType::Eldritch,
                    identity_skills: &["burn_attack", "ghost_fire_split"],
                },
                MonsterFamilyExpectation {
                    family_id: "fox_fire",
                    dungeon: Dungeon::ZhuQue,
                    role: FamilyRole::Bruiser,
                    monster_type: MonsterType::Beast,
                    identity_skills: &["bite", "protect"],
                },
                MonsterFamilyExpectation {
                    family_id: "moth_fire",
                    dungeon: Dungeon::ZhuQue,
                    role: FamilyRole::Ranged,
                    monster_type: MonsterType::Eldritch,
                    identity_skills: &["cocoon", "fly_into_fire"],
                },
                MonsterFamilyExpectation {
                    family_id: "lantern",
                    dungeon: Dungeon::ZhuQue,
                    role: FamilyRole::Ranged,
                    monster_type: MonsterType::Eldritch,
                    identity_skills: &["stress", "burn_attack"],
                },
                // ── XuanWu ────────────────────────────────────────────────
                MonsterFamilyExpectation {
                    family_id: "snake_water",
                    dungeon: Dungeon::XuanWu,
                    role: FamilyRole::Controller,
                    monster_type: MonsterType::Eldritch,
                    identity_skills: &["stun", "poison_fang"],
                },
                MonsterFamilyExpectation {
                    family_id: "water_grass",
                    dungeon: Dungeon::XuanWu,
                    role: FamilyRole::Controller,
                    monster_type: MonsterType::Eldritch,
                    identity_skills: &["stun", "puncture", "convolve"],
                },
                MonsterFamilyExpectation {
                    family_id: "monkey_water",
                    dungeon: Dungeon::XuanWu,
                    role: FamilyRole::Bruiser,
                    monster_type: MonsterType::Unholy,
                    identity_skills: &["rush", "stress"],
                },
            ],
        }
    }
}

impl Default for MonsterFamilyParityFixture {
    fn default() -> Self {
        Self::new()
    }
}
