//! Encounter pack types and DDGC common encounter pack definitions.
//!
//! An encounter pack is a fixed enemy composition drawn from the DDGC dungeon
//! encounter tables. Each pack specifies which monster families appear and how
//! many of each, organized by dungeon and pack type (hall vs room).
//!
//! Hall packs are corridor encounters (typically 1-3 enemies).
//! Room packs are room encounters (typically 2-4 enemies, higher threat).
//!
//! Boss packs are NOT included here — they will be added in K29/K30+ stories.

use std::collections::HashMap;

pub use crate::monsters::families::{Dungeon, FamilyId};

// ── Pack Type ────────────────────────────────────────────────────────────────

/// Type of encounter pack, matching DDGC dungeon generation categories.
///
/// DDGC distinguishes between "hall" (corridor), "room" encounter tables,
/// and "boss" encounters within each dungeon. This enum captures that
/// distinction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackType {
    /// Corridor encounter — typically 1-3 enemies, lighter pressure.
    Hall,
    /// Room encounter — typically 2-4 enemies, higher threat density.
    Room,
    /// Boss encounter — boss + boss parts, highest threat.
    Boss,
}

// ── Pack ID ──────────────────────────────────────────────────────────────────

/// Unique identifier for an encounter pack.
///
/// Format: `{dungeon}_{pack_type}_{index}` — e.g., `qinglong_hall_01`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackId(pub String);

impl PackId {
    pub fn new(id: impl Into<String>) -> Self {
        PackId(id.into())
    }
}

impl std::fmt::Display for PackId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ── Family Slot ───────────────────────────────────────────────────────────────

/// A single monster family slot within an encounter pack.
///
/// Each slot specifies which family appears and how many copies. Position
/// in the `slots` vec corresponds to formation position (front to back).
#[derive(Debug, Clone)]
pub struct FamilySlot {
    /// The monster family ID (e.g., "mantis_magic_flower").
    pub family_id: FamilyId,
    /// How many of this family appear in the pack.
    pub count: u32,
}

// ── Encounter Pack ───────────────────────────────────────────────────────────

/// A DDGC common encounter pack — a fixed enemy composition for a dungeon.
///
/// Packs are drawn from the DDGC dungeon encounter tables. Each pack
/// represents one possible enemy composition that can appear in a combat
/// room, organized by dungeon and pack type.
#[derive(Debug, Clone)]
pub struct EncounterPack {
    /// Unique pack identifier.
    pub id: PackId,
    /// Home dungeon region.
    pub dungeon: Dungeon,
    /// Hall (corridor) or Room encounter.
    pub pack_type: PackType,
    /// Monster family composition, ordered by formation position.
    pub slots: Vec<FamilySlot>,
}

impl EncounterPack {
    /// Total number of enemy units in this pack (sum of all slot counts).
    pub fn total_units(&self) -> u32 {
        self.slots.iter().map(|s| s.count).sum()
    }

    /// Return all family IDs referenced in this pack.
    pub fn family_ids(&self) -> Vec<&FamilyId> {
        self.slots.iter().map(|s| &s.family_id).collect()
    }
}

// ── Encounter Pack Registry ──────────────────────────────────────────────────

/// Lookup table for DDGC common encounter packs by pack ID and dungeon.
///
/// The registry is built incrementally as packs are migrated. The
/// `build_packs_registry()` function populates it with all common-tier
/// encounter packs for all four dungeons.
pub struct EncounterPackRegistry {
    packs: HashMap<String, EncounterPack>,
}

impl EncounterPackRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        EncounterPackRegistry {
            packs: HashMap::new(),
        }
    }

    /// Register an encounter pack.
    pub fn register(&mut self, pack: EncounterPack) {
        self.packs.insert(pack.id.0.clone(), pack);
    }

    /// Look up a pack by its string ID.
    pub fn get(&self, id: &str) -> Option<&EncounterPack> {
        self.packs.get(id)
    }

    /// Look up a pack by `PackId`.
    pub fn get_by_id(&self, id: &PackId) -> Option<&EncounterPack> {
        self.packs.get(&id.0)
    }

    /// Return all packs for a given dungeon.
    pub fn by_dungeon(&self, dungeon: Dungeon) -> Vec<&EncounterPack> {
        self.packs
            .values()
            .filter(|p| p.dungeon == dungeon)
            .collect()
    }

    /// Return all packs of a given type across all dungeons.
    pub fn by_type(&self, pack_type: PackType) -> Vec<&EncounterPack> {
        self.packs
            .values()
            .filter(|p| p.pack_type == pack_type)
            .collect()
    }

    /// Return packs for a given dungeon AND pack type.
    pub fn by_dungeon_and_type(&self, dungeon: Dungeon, pack_type: PackType) -> Vec<&EncounterPack> {
        self.packs
            .values()
            .filter(|p| p.dungeon == dungeon && p.pack_type == pack_type)
            .collect()
    }

    /// Iterate over all registered packs.
    pub fn iter(&self) -> impl Iterator<Item = &EncounterPack> {
        self.packs.values()
    }

    /// Return the number of registered packs.
    pub fn len(&self) -> usize {
        self.packs.len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.packs.is_empty()
    }
}

impl Default for EncounterPackRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ── DDGC Common Encounter Pack Definitions ────────────────────────────────────
//
// Each dungeon defines hall and room encounter packs drawn from the DDGC
// encounter tables. These represent the most common compositions found
// in the game's dungeon generation data.
//
// Naming convention: {dungeon}_{hall|room}_{NN}
//
// The pack compositions are derived from the DDGC .bytes dungeon config
// files (mash hall/room tables, tier 1).

/// Build common encounter packs for QingLong (青龙 — Forest/Swamp).
///
/// QingLong common pool: mantis_magic_flower, mantis_spiny_flower,
/// mantis_walking_flower, dry_tree_genie, moth_mimicry_A, moth_mimicry_B,
/// plus robber_melee and robber_ranged as cross-dungeon cameos.
pub fn qinglong_packs() -> Vec<EncounterPack> {
    vec![
        // ── Hall packs (corridor encounters) ──
        EncounterPack {
            id: PackId::new("qinglong_hall_01"),
            dungeon: Dungeon::QingLong,
            pack_type: PackType::Hall,
            slots: vec![FamilySlot {
                family_id: FamilyId::new("mantis_magic_flower"),
                count: 1,
            }],
        },
        EncounterPack {
            id: PackId::new("qinglong_hall_02"),
            dungeon: Dungeon::QingLong,
            pack_type: PackType::Hall,
            slots: vec![FamilySlot {
                family_id: FamilyId::new("mantis_spiny_flower"),
                count: 3,
            }],
        },
        EncounterPack {
            id: PackId::new("qinglong_hall_03"),
            dungeon: Dungeon::QingLong,
            pack_type: PackType::Hall,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("moth_mimicry_A"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("moth_mimicry_B"),
                    count: 1,
                },
            ],
        },
        EncounterPack {
            id: PackId::new("qinglong_hall_04"),
            dungeon: Dungeon::QingLong,
            pack_type: PackType::Hall,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("mantis_spiny_flower"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("dry_tree_genie"),
                    count: 1,
                },
            ],
        },
        EncounterPack {
            id: PackId::new("qinglong_hall_05"),
            dungeon: Dungeon::QingLong,
            pack_type: PackType::Hall,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("mantis_walking_flower"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("dry_tree_genie"),
                    count: 1,
                },
            ],
        },
        // ── Room packs (room encounters) ──
        EncounterPack {
            id: PackId::new("qinglong_room_01"),
            dungeon: Dungeon::QingLong,
            pack_type: PackType::Room,
            slots: vec![FamilySlot {
                family_id: FamilyId::new("mantis_magic_flower"),
                count: 2,
            }],
        },
        EncounterPack {
            id: PackId::new("qinglong_room_02"),
            dungeon: Dungeon::QingLong,
            pack_type: PackType::Room,
            slots: vec![FamilySlot {
                family_id: FamilyId::new("mantis_spiny_flower"),
                count: 4,
            }],
        },
        EncounterPack {
            id: PackId::new("qinglong_room_03"),
            dungeon: Dungeon::QingLong,
            pack_type: PackType::Room,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("moth_mimicry_A"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("moth_mimicry_B"),
                    count: 2,
                },
            ],
        },
        EncounterPack {
            id: PackId::new("qinglong_room_04"),
            dungeon: Dungeon::QingLong,
            pack_type: PackType::Room,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("mantis_magic_flower"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("dry_tree_genie"),
                    count: 2,
                },
            ],
        },
        EncounterPack {
            id: PackId::new("qinglong_room_05"),
            dungeon: Dungeon::QingLong,
            pack_type: PackType::Room,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("mantis_walking_flower"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("moth_mimicry_A"),
                    count: 2,
                },
            ],
        },
    ]
}

/// Build common encounter packs for BaiHu (白虎 — Fortress).
///
/// BaiHu common pool: metal_armor, tiger_sword, lizard, unicorn_beetle_A,
/// unicorn_beetle_B, alligator_yangtze, plus robber_melee and robber_ranged
/// as cross-dungeon cameos.
pub fn baihu_packs() -> Vec<EncounterPack> {
    vec![
        // ── Hall packs ──
        EncounterPack {
            id: PackId::new("baihu_hall_01"),
            dungeon: Dungeon::BaiHu,
            pack_type: PackType::Hall,
            slots: vec![FamilySlot {
                family_id: FamilyId::new("metal_armor"),
                count: 1,
            }],
        },
        EncounterPack {
            id: PackId::new("baihu_hall_02"),
            dungeon: Dungeon::BaiHu,
            pack_type: PackType::Hall,
            slots: vec![FamilySlot {
                family_id: FamilyId::new("lizard"),
                count: 2,
            }],
        },
        EncounterPack {
            id: PackId::new("baihu_hall_03"),
            dungeon: Dungeon::BaiHu,
            pack_type: PackType::Hall,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("robber_melee"),
                    count: 1,
                },
                FamilySlot {
                    family_id: FamilyId::new("tiger_sword"),
                    count: 2,
                },
            ],
        },
        EncounterPack {
            id: PackId::new("baihu_hall_04"),
            dungeon: Dungeon::BaiHu,
            pack_type: PackType::Hall,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("unicorn_beetle_A"),
                    count: 1,
                },
                FamilySlot {
                    family_id: FamilyId::new("unicorn_beetle_B"),
                    count: 1,
                },
                FamilySlot {
                    family_id: FamilyId::new("unicorn_beetle_A"),
                    count: 1,
                },
            ],
        },
        EncounterPack {
            id: PackId::new("baihu_hall_05"),
            dungeon: Dungeon::BaiHu,
            pack_type: PackType::Hall,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("tiger_sword"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("alligator_yangtze"),
                    count: 1,
                },
            ],
        },
        // ── Room packs ──
        EncounterPack {
            id: PackId::new("baihu_room_01"),
            dungeon: Dungeon::BaiHu,
            pack_type: PackType::Room,
            slots: vec![FamilySlot {
                family_id: FamilyId::new("metal_armor"),
                count: 2,
            }],
        },
        EncounterPack {
            id: PackId::new("baihu_room_02"),
            dungeon: Dungeon::BaiHu,
            pack_type: PackType::Room,
            slots: vec![FamilySlot {
                family_id: FamilyId::new("tiger_sword"),
                count: 4,
            }],
        },
        EncounterPack {
            id: PackId::new("baihu_room_03"),
            dungeon: Dungeon::BaiHu,
            pack_type: PackType::Room,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("unicorn_beetle_B"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("unicorn_beetle_A"),
                    count: 2,
                },
            ],
        },
        EncounterPack {
            id: PackId::new("baihu_room_04"),
            dungeon: Dungeon::BaiHu,
            pack_type: PackType::Room,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("metal_armor"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("alligator_yangtze"),
                    count: 2,
                },
            ],
        },
        EncounterPack {
            id: PackId::new("baihu_room_05"),
            dungeon: Dungeon::BaiHu,
            pack_type: PackType::Room,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("lizard"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("alligator_yangtze"),
                    count: 2,
                },
            ],
        },
    ]
}

/// Build common encounter packs for ZhuQue (朱雀 — Fire Temple).
///
/// ZhuQue common pool: ghost_fire_assist, ghost_fire_damage, fox_fire,
/// moth_fire, lantern, plus robber_ranged and robber_melee as cameos.
pub fn zhuque_packs() -> Vec<EncounterPack> {
    vec![
        // ── Hall packs ──
        EncounterPack {
            id: PackId::new("zhuque_hall_01"),
            dungeon: Dungeon::ZhuQue,
            pack_type: PackType::Hall,
            slots: vec![FamilySlot {
                family_id: FamilyId::new("ghost_fire_assist"),
                count: 1,
            }],
        },
        EncounterPack {
            id: PackId::new("zhuque_hall_02"),
            dungeon: Dungeon::ZhuQue,
            pack_type: PackType::Hall,
            slots: vec![FamilySlot {
                family_id: FamilyId::new("ghost_fire_damage"),
                count: 3,
            }],
        },
        EncounterPack {
            id: PackId::new("zhuque_hall_03"),
            dungeon: Dungeon::ZhuQue,
            pack_type: PackType::Hall,
            slots: vec![FamilySlot {
                family_id: FamilyId::new("fox_fire"),
                count: 2,
            }],
        },
        EncounterPack {
            id: PackId::new("zhuque_hall_04"),
            dungeon: Dungeon::ZhuQue,
            pack_type: PackType::Hall,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("ghost_fire_damage"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("ghost_fire_assist"),
                    count: 1,
                },
            ],
        },
        EncounterPack {
            id: PackId::new("zhuque_hall_05"),
            dungeon: Dungeon::ZhuQue,
            pack_type: PackType::Hall,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("moth_fire"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("lantern"),
                    count: 1,
                },
            ],
        },
        // ── Room packs ──
        EncounterPack {
            id: PackId::new("zhuque_room_01"),
            dungeon: Dungeon::ZhuQue,
            pack_type: PackType::Room,
            slots: vec![FamilySlot {
                family_id: FamilyId::new("ghost_fire_assist"),
                count: 2,
            }],
        },
        EncounterPack {
            id: PackId::new("zhuque_room_02"),
            dungeon: Dungeon::ZhuQue,
            pack_type: PackType::Room,
            slots: vec![FamilySlot {
                family_id: FamilyId::new("fox_fire"),
                count: 3,
            }],
        },
        EncounterPack {
            id: PackId::new("zhuque_room_03"),
            dungeon: Dungeon::ZhuQue,
            pack_type: PackType::Room,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("ghost_fire_damage"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("ghost_fire_assist"),
                    count: 2,
                },
            ],
        },
        EncounterPack {
            id: PackId::new("zhuque_room_04"),
            dungeon: Dungeon::ZhuQue,
            pack_type: PackType::Room,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("moth_fire"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("lantern"),
                    count: 2,
                },
            ],
        },
        EncounterPack {
            id: PackId::new("zhuque_room_05"),
            dungeon: Dungeon::ZhuQue,
            pack_type: PackType::Room,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("fox_fire"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("ghost_fire_assist"),
                    count: 2,
                },
            ],
        },
    ]
}

/// Build common encounter packs for XuanWu (玄武 — Water Depths).
///
/// XuanWu common pool: snake_water, water_grass, monkey_water,
/// plus robber_ranged as a cross-dungeon cameo.
pub fn xuanwu_packs() -> Vec<EncounterPack> {
    vec![
        // ── Hall packs ──
        EncounterPack {
            id: PackId::new("xuanwu_hall_01"),
            dungeon: Dungeon::XuanWu,
            pack_type: PackType::Hall,
            slots: vec![FamilySlot {
                family_id: FamilyId::new("snake_water"),
                count: 1,
            }],
        },
        EncounterPack {
            id: PackId::new("xuanwu_hall_02"),
            dungeon: Dungeon::XuanWu,
            pack_type: PackType::Hall,
            slots: vec![FamilySlot {
                family_id: FamilyId::new("water_grass"),
                count: 3,
            }],
        },
        EncounterPack {
            id: PackId::new("xuanwu_hall_03"),
            dungeon: Dungeon::XuanWu,
            pack_type: PackType::Hall,
            slots: vec![FamilySlot {
                family_id: FamilyId::new("monkey_water"),
                count: 2,
            }],
        },
        EncounterPack {
            id: PackId::new("xuanwu_hall_04"),
            dungeon: Dungeon::XuanWu,
            pack_type: PackType::Hall,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("snake_water"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("water_grass"),
                    count: 1,
                },
            ],
        },
        EncounterPack {
            id: PackId::new("xuanwu_hall_05"),
            dungeon: Dungeon::XuanWu,
            pack_type: PackType::Hall,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("monkey_water"),
                    count: 1,
                },
                FamilySlot {
                    family_id: FamilyId::new("water_grass"),
                    count: 2,
                },
            ],
        },
        // ── Room packs ──
        EncounterPack {
            id: PackId::new("xuanwu_room_01"),
            dungeon: Dungeon::XuanWu,
            pack_type: PackType::Room,
            slots: vec![FamilySlot {
                family_id: FamilyId::new("snake_water"),
                count: 2,
            }],
        },
        EncounterPack {
            id: PackId::new("xuanwu_room_02"),
            dungeon: Dungeon::XuanWu,
            pack_type: PackType::Room,
            slots: vec![FamilySlot {
                family_id: FamilyId::new("water_grass"),
                count: 4,
            }],
        },
        EncounterPack {
            id: PackId::new("xuanwu_room_03"),
            dungeon: Dungeon::XuanWu,
            pack_type: PackType::Room,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("snake_water"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("monkey_water"),
                    count: 2,
                },
            ],
        },
        EncounterPack {
            id: PackId::new("xuanwu_room_04"),
            dungeon: Dungeon::XuanWu,
            pack_type: PackType::Room,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("snake_water"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("water_grass"),
                    count: 1,
                },
                FamilySlot {
                    family_id: FamilyId::new("monkey_water"),
                    count: 1,
                },
            ],
        },
        EncounterPack {
            id: PackId::new("xuanwu_room_05"),
            dungeon: Dungeon::XuanWu,
            pack_type: PackType::Room,
            slots: vec![FamilySlot {
                family_id: FamilyId::new("monkey_water"),
                count: 4,
            }],
        },
    ]
}

/// Boss encounter packs for the QingLong dungeon.
///
/// Currently includes only the azure_dragon boss pack. Future boss migration
/// slices (US-430 onwards) will add additional boss packs.
pub fn qinglong_boss_packs() -> Vec<EncounterPack> {
    vec![
        EncounterPack {
            id: PackId::new("qinglong_boss_azure_dragon"),
            dungeon: Dungeon::QingLong,
            pack_type: PackType::Boss,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("azure_dragon_ball_thunder"),
                    count: 1,
                },
                FamilySlot {
                    family_id: FamilyId::new("azure_dragon"),
                    count: 1,
                },
                FamilySlot {
                    family_id: FamilyId::new("azure_dragon_ball_wind"),
                    count: 1,
                },
            ],
        },
    ]
}

/// ZhuQue boss encounter packs.
///
/// Currently includes only the vermilion_bird boss pack. Future boss migration
/// slices will add additional boss packs.
pub fn zhuque_boss_packs() -> Vec<EncounterPack> {
    vec![
        EncounterPack {
            id: PackId::new("zhuque_boss_vermilion_bird"),
            dungeon: Dungeon::ZhuQue,
            pack_type: PackType::Boss,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("vermilion_bird"),
                    count: 1,
                },
                FamilySlot {
                    family_id: FamilyId::new("vermilion_bird_tail_A"),
                    count: 1,
                },
                FamilySlot {
                    family_id: FamilyId::new("vermilion_bird_tail_B"),
                    count: 1,
                },
            ],
        },
        EncounterPack {
            id: PackId::new("zhuque_boss_gambler"),
            dungeon: Dungeon::ZhuQue,
            pack_type: PackType::Boss,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("gambler"),
                    count: 1,
                },
            ],
        },
    ]
}

/// BaiHu boss encounter packs.
///
/// Currently includes only the white_tiger boss pack. Future boss migration
/// slices will add additional boss packs (black_tortoise).
pub fn baihu_boss_packs() -> Vec<EncounterPack> {
    vec![
        EncounterPack {
            id: PackId::new("baihu_boss_white_tiger"),
            dungeon: Dungeon::BaiHu,
            pack_type: PackType::Boss,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("white_tiger_A"),
                    count: 1,
                },
                FamilySlot {
                    family_id: FamilyId::new("white_tiger_B"),
                    count: 1,
                },
                FamilySlot {
                    family_id: FamilyId::new("white_tiger_terrain"),
                    count: 1,
                },
            ],
        },
    ]
}

/// XuanWu boss encounter packs.
///
/// Includes the black_tortoise and rotvine_wraith boss packs. The black_tortoise
/// is a dual-body composite boss (Tortoise A + Snake B). The rotvine_wraith
/// is a summon-control boss that continuously re-summons rotten_fruit minions.
pub fn xuanwu_boss_packs() -> Vec<EncounterPack> {
    vec![
        EncounterPack {
            id: PackId::new("xuanwu_boss_black_tortoise"),
            dungeon: Dungeon::XuanWu,
            pack_type: PackType::Boss,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("black_tortoise_A"),
                    count: 1,
                },
                FamilySlot {
                    family_id: FamilyId::new("black_tortoise_B"),
                    count: 1,
                },
            ],
        },
        EncounterPack {
            id: PackId::new("xuanwu_boss_rotvine_wraith"),
            dungeon: Dungeon::XuanWu,
            pack_type: PackType::Boss,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("rotvine_wraith"),
                    count: 1,
                },
                FamilySlot {
                    family_id: FamilyId::new("rotten_fruit_A"),
                    count: 1,
                },
                FamilySlot {
                    family_id: FamilyId::new("rotten_fruit_B"),
                    count: 1,
                },
            ],
        },
        EncounterPack {
            id: PackId::new("xuanwu_boss_skeletal_tiller"),
            dungeon: Dungeon::XuanWu,
            pack_type: PackType::Boss,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("skeletal_tiller"),
                    count: 1,
                },
                FamilySlot {
                    family_id: FamilyId::new("vegetable"),
                    count: 1,
                },
            ],
        },
        EncounterPack {
            id: PackId::new("xuanwu_boss_necrodrake_embryosac"),
            dungeon: Dungeon::XuanWu,
            pack_type: PackType::Boss,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("egg_membrane_empty"),
                    count: 1,
                },
                FamilySlot {
                    family_id: FamilyId::new("necrodrake_embryosac"),
                    count: 1,
                },
                FamilySlot {
                    family_id: FamilyId::new("egg_membrane_empty"),
                    count: 1,
                },
            ],
        },
        EncounterPack {
            id: PackId::new("xuanwu_boss_scorchthroat_chanteuse"),
            dungeon: Dungeon::XuanWu,
            pack_type: PackType::Boss,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("sc_blow"),
                    count: 1,
                },
                FamilySlot {
                    family_id: FamilyId::new("scorchthroat_chanteuse"),
                    count: 1,
                },
                FamilySlot {
                    family_id: FamilyId::new("sc_bow"),
                    count: 1,
                },
            ],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qinglong_packs_have_correct_dungeon() {
        for pack in qinglong_packs() {
            assert_eq!(
                pack.dungeon,
                Dungeon::QingLong,
                "Pack {} should be QingLong",
                pack.id
            );
        }
    }

    #[test]
    fn baihu_packs_have_correct_dungeon() {
        for pack in baihu_packs() {
            assert_eq!(
                pack.dungeon,
                Dungeon::BaiHu,
                "Pack {} should be BaiHu",
                pack.id
            );
        }
    }

    #[test]
    fn zhuque_packs_have_correct_dungeon() {
        for pack in zhuque_packs() {
            assert_eq!(
                pack.dungeon,
                Dungeon::ZhuQue,
                "Pack {} should be ZhuQue",
                pack.id
            );
        }
    }

    #[test]
    fn xuanwu_packs_have_correct_dungeon() {
        for pack in xuanwu_packs() {
            assert_eq!(
                pack.dungeon,
                Dungeon::XuanWu,
                "Pack {} should be XuanWu",
                pack.id
            );
        }
    }

    #[test]
    fn all_packs_have_nonzero_units() {
        let ql = qinglong_packs();
        let bh = baihu_packs();
        let zq = zhuque_packs();
        let xw = xuanwu_packs();
        let all_packs: Vec<&EncounterPack> = ql
            .iter()
            .chain(bh.iter())
            .chain(zq.iter())
            .chain(xw.iter())
            .collect();

        for pack in all_packs {
            assert!(
                pack.total_units() > 0,
                "Pack {} should have at least 1 unit",
                pack.id
            );
        }
    }

    #[test]
    fn all_packs_have_slots() {
        let ql = qinglong_packs();
        let bh = baihu_packs();
        let zq = zhuque_packs();
        let xw = xuanwu_packs();
        let all_packs: Vec<&EncounterPack> = ql
            .iter()
            .chain(bh.iter())
            .chain(zq.iter())
            .chain(xw.iter())
            .collect();

        for pack in all_packs {
            assert!(
                !pack.slots.is_empty(),
                "Pack {} should have at least one slot",
                pack.id
            );
        }
    }

    #[test]
    fn hall_packs_have_fewer_units_than_room_packs() {
        // On average, hall packs should have fewer units than room packs.
        // This test verifies the structural expectation from DDGC data.
        for packs in [
            qinglong_packs(),
            baihu_packs(),
            zhuque_packs(),
            xuanwu_packs(),
        ] {
            let hall_avg = packs
                .iter()
                .filter(|p| p.pack_type == PackType::Hall)
                .map(|p| p.total_units())
                .sum::<u32>() as f64
                / packs.iter().filter(|p| p.pack_type == PackType::Hall).count() as f64;

            let room_avg = packs
                .iter()
                .filter(|p| p.pack_type == PackType::Room)
                .map(|p| p.total_units())
                .sum::<u32>() as f64
                / packs.iter().filter(|p| p.pack_type == PackType::Room).count() as f64;

            assert!(
                hall_avg <= room_avg,
                "Hall pack average ({}) should be <= room pack average ({})",
                hall_avg,
                room_avg
            );
        }
    }

    #[test]
    fn qinglong_packs_reference_only_native_families() {
        // QingLong native families + robber cameos
        let native = [
            "mantis_magic_flower",
            "mantis_spiny_flower",
            "mantis_walking_flower",
            "dry_tree_genie",
            "moth_mimicry_A",
            "moth_mimicry_B",
            "robber_melee",
            "robber_ranged",
        ];

        for pack in qinglong_packs() {
            for slot in &pack.slots {
                assert!(
                    native.contains(&slot.family_id.0.as_str()),
                    "QingLong pack {} references non-native family {}",
                    pack.id,
                    slot.family_id
                );
            }
        }
    }

    #[test]
    fn baihu_packs_reference_only_native_families() {
        let native = [
            "metal_armor",
            "tiger_sword",
            "lizard",
            "unicorn_beetle_A",
            "unicorn_beetle_B",
            "alligator_yangtze",
            "robber_melee",
            "robber_ranged",
        ];

        for pack in baihu_packs() {
            for slot in &pack.slots {
                assert!(
                    native.contains(&slot.family_id.0.as_str()),
                    "BaiHu pack {} references non-native family {}",
                    pack.id,
                    slot.family_id
                );
            }
        }
    }

    #[test]
    fn zhuque_packs_reference_only_native_families() {
        let native = [
            "ghost_fire_assist",
            "ghost_fire_damage",
            "fox_fire",
            "moth_fire",
            "lantern",
            "robber_ranged",
            "robber_melee",
        ];

        for pack in zhuque_packs() {
            for slot in &pack.slots {
                assert!(
                    native.contains(&slot.family_id.0.as_str()),
                    "ZhuQue pack {} references non-native family {}",
                    pack.id,
                    slot.family_id
                );
            }
        }
    }

    #[test]
    fn xuanwu_packs_reference_only_native_families() {
        let native = [
            "snake_water",
            "water_grass",
            "monkey_water",
            "robber_ranged",
        ];

        for pack in xuanwu_packs() {
            for slot in &pack.slots {
                assert!(
                    native.contains(&slot.family_id.0.as_str()),
                    "XuanWu pack {} references non-native family {}",
                    pack.id,
                    slot.family_id
                );
            }
        }
    }

    #[test]
    fn pack_ids_are_unique_within_dungeon() {
        for packs in [
            qinglong_packs(),
            baihu_packs(),
            zhuque_packs(),
            xuanwu_packs(),
        ] {
            let mut seen = std::collections::HashSet::new();
            for pack in &packs {
                assert!(
                    seen.insert(pack.id.0.clone()),
                    "Duplicate pack ID: {}",
                    pack.id
                );
            }
        }
    }

    #[test]
    fn encounter_pack_total_units_counts_correctly() {
        let pack = EncounterPack {
            id: PackId::new("test_pack"),
            dungeon: Dungeon::QingLong,
            pack_type: PackType::Hall,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("mantis_magic_flower"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("dry_tree_genie"),
                    count: 1,
                },
            ],
        };
        assert_eq!(pack.total_units(), 3);
    }

    #[test]
    fn encounter_pack_family_ids_lists_all_families() {
        let pack = EncounterPack {
            id: PackId::new("test_pack"),
            dungeon: Dungeon::QingLong,
            pack_type: PackType::Hall,
            slots: vec![
                FamilySlot {
                    family_id: FamilyId::new("mantis_magic_flower"),
                    count: 2,
                },
                FamilySlot {
                    family_id: FamilyId::new("dry_tree_genie"),
                    count: 1,
                },
            ],
        };
        let ids: Vec<&str> = pack.family_ids().iter().map(|id| id.0.as_str()).collect();
        assert!(ids.contains(&"mantis_magic_flower"));
        assert!(ids.contains(&"dry_tree_genie"));
    }
}