//! Encounter pack module — DDGC encounter compositions grouped by dungeon.
//!
//! This module provides dungeon-grouped encounter packs for common (non-boss)
//! DDGC combat rooms. Each pack specifies which monster families appear and
//! in what quantities, drawn from the DDGC dungeon encounter tables.
//!
//! The `build_packs_registry()` function populates the registry with all
//! common encounter packs for the four DDGC dungeons. Boss encounter packs
//! will be added in later migration slices (K29/K30+).

pub mod packs;

pub use packs::*;

/// Build the common encounter pack registry with packs for all four dungeons.
///
/// This function registers hall and room packs for QingLong, BaiHu, ZhuQue,
/// and XuanWu. Boss packs are NOT included here — they will be added in
/// the boss migration slices (K29/K30+).
pub fn build_packs_registry() -> EncounterPackRegistry {
    let mut registry = EncounterPackRegistry::new();

    // QingLong (青龙 — Forest/Swamp)
    for pack in packs::qinglong_packs() {
        registry.register(pack);
    }

    // BaiHu (白虎 — Fortress)
    for pack in packs::baihu_packs() {
        registry.register(pack);
    }

    // ZhuQue (朱雀 — Fire Temple)
    for pack in packs::zhuque_packs() {
        registry.register(pack);
    }

    // XuanWu (玄武 — Water Depths)
    for pack in packs::xuanwu_packs() {
        registry.register(pack);
    }

    registry
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encounter_pack_registry_builds() {
        let registry = build_packs_registry();
        assert!(!registry.is_empty(), "Registry should not be empty");
    }

    #[test]
    fn encounter_pack_registry_has_packs_for_all_dungeons() {
        let registry = build_packs_registry();

        let qinglong = registry.by_dungeon(Dungeon::QingLong);
        let baihu = registry.by_dungeon(Dungeon::BaiHu);
        let zhuque = registry.by_dungeon(Dungeon::ZhuQue);
        let xuanwu = registry.by_dungeon(Dungeon::XuanWu);

        assert!(
            !qinglong.is_empty(),
            "Registry should have QingLong packs"
        );
        assert!(!baihu.is_empty(), "Registry should have BaiHu packs");
        assert!(!zhuque.is_empty(), "Registry should have ZhuQue packs");
        assert!(!xuanwu.is_empty(), "Registry should have XuanWu packs");
    }

    #[test]
    fn encounter_pack_registry_exposes_hall_and_room_packs() {
        let registry = build_packs_registry();

        let hall_packs = registry.by_type(PackType::Hall);
        let room_packs = registry.by_type(PackType::Room);

        assert!(
            !hall_packs.is_empty(),
            "Registry should have hall packs"
        );
        assert!(
            !room_packs.is_empty(),
            "Registry should have room packs"
        );
    }

    #[test]
    fn encounter_pack_registry_looks_up_by_id() {
        let registry = build_packs_registry();

        let pack = registry
            .get("qinglong_hall_01")
            .expect("qinglong_hall_01 should exist");

        assert_eq!(pack.dungeon, Dungeon::QingLong);
        assert_eq!(pack.pack_type, PackType::Hall);
    }

    #[test]
    fn encounter_pack_registry_filters_by_dungeon_and_type() {
        let registry = build_packs_registry();

        let qinglong_halls = registry.by_dungeon_and_type(Dungeon::QingLong, PackType::Hall);
        let qinglong_rooms = registry.by_dungeon_and_type(Dungeon::QingLong, PackType::Room);

        assert!(
            !qinglong_halls.is_empty(),
            "QingLong should have hall packs"
        );
        assert!(
            !qinglong_rooms.is_empty(),
            "QingLong should have room packs"
        );

        // All QingLong halls should be hall type
        for pack in &qinglong_halls {
            assert_eq!(pack.pack_type, PackType::Hall);
            assert_eq!(pack.dungeon, Dungeon::QingLong);
        }

        // All QingLong rooms should be room type
        for pack in &qinglong_rooms {
            assert_eq!(pack.pack_type, PackType::Room);
            assert_eq!(pack.dungeon, Dungeon::QingLong);
        }
    }

    #[test]
    fn encounter_pack_registry_total_count_matches_per_dungeon() {
        let registry = build_packs_registry();

        let qinglong_count = registry.by_dungeon(Dungeon::QingLong).len();
        let baihu_count = registry.by_dungeon(Dungeon::BaiHu).len();
        let zhuque_count = registry.by_dungeon(Dungeon::ZhuQue).len();
        let xuanwu_count = registry.by_dungeon(Dungeon::XuanWu).len();

        assert_eq!(
            registry.len(),
            qinglong_count + baihu_count + zhuque_count + xuanwu_count,
            "Total pack count should equal sum of per-dungeon counts"
        );
    }

    #[test]
    fn encounter_pack_all_family_ids_are_registered_in_monster_registry() {
        // Verify that every family ID referenced in encounter packs exists
        // in the monster family registry.
        let monster_registry = crate::monsters::build_registry();
        let pack_registry = build_packs_registry();

        for pack in pack_registry.iter() {
            for slot in &pack.slots {
                assert!(
                    monster_registry.get(&slot.family_id.0).is_some(),
                    "Pack {} references family '{}' which is not registered in monster registry",
                    pack.id,
                    slot.family_id
                );
            }
        }
    }

    #[test]
    fn encounter_pack_all_packs_have_valid_archetype_names() {
        // Verify that every family referenced in packs has an archetype name
        // in the monster registry (needed for content resolution later).
        let monster_registry = crate::monsters::build_registry();
        let pack_registry = build_packs_registry();

        for pack in pack_registry.iter() {
            for slot in &pack.slots {
                let family = monster_registry
                    .get(&slot.family_id.0)
                    .expect("family should exist");
                assert!(
                    !family.archetype_name.is_empty(),
                    "Pack {} references family '{}' with empty archetype_name",
                    pack.id,
                    slot.family_id
                );
            }
        }
    }
}