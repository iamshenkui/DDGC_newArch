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

/// Build the encounter pack registry with common and boss packs for all four dungeons.
///
/// This function registers hall, room, and boss packs for QingLong, BaiHu, ZhuQue,
/// and XuanWu. Boss packs are added incrementally as boss families are migrated.
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

    // Boss packs — added incrementally as boss families are migrated
    for pack in packs::qinglong_boss_packs() {
        registry.register(pack);
    }

    for pack in packs::zhuque_boss_packs() {
        registry.register(pack);
    }

    for pack in packs::baihu_boss_packs() {
        registry.register(pack);
    }

    for pack in packs::xuanwu_boss_packs() {
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

    #[test]
    fn encounter_pack_registry_has_boss_packs() {
        let registry = build_packs_registry();

        let boss_packs = registry.by_type(PackType::Boss);

        assert!(
            !boss_packs.is_empty(),
            "Registry should have boss packs"
        );
    }

    #[test]
    fn qinglong_boss_azure_dragon_pack_is_correct() {
        let registry = build_packs_registry();

        let pack = registry
            .get("qinglong_boss_azure_dragon")
            .expect("qinglong_boss_azure_dragon should exist");

        assert_eq!(pack.dungeon, Dungeon::QingLong);
        assert_eq!(pack.pack_type, PackType::Boss);
        assert_eq!(pack.total_units(), 3, "azure_dragon boss pack should have 3 units");

        let family_ids: Vec<&str> = pack.family_ids().iter().map(|id| id.0.as_str()).collect();
        assert!(
            family_ids.contains(&"azure_dragon"),
            "azure_dragon boss pack must contain azure_dragon"
        );
        assert!(
            family_ids.contains(&"azure_dragon_ball_thunder"),
            "azure_dragon boss pack must contain azure_dragon_ball_thunder"
        );
        assert!(
            family_ids.contains(&"azure_dragon_ball_wind"),
            "azure_dragon boss pack must contain azure_dragon_ball_wind"
        );
    }

    #[test]
    fn zhuque_boss_vermilion_bird_pack_is_correct() {
        let registry = build_packs_registry();

        let pack = registry
            .get("zhuque_boss_vermilion_bird")
            .expect("zhuque_boss_vermilion_bird should exist");

        assert_eq!(pack.dungeon, Dungeon::ZhuQue);
        assert_eq!(pack.pack_type, PackType::Boss);
        assert_eq!(pack.total_units(), 3, "vermilion_bird boss pack should have 3 units");

        let family_ids: Vec<&str> = pack.family_ids().iter().map(|id| id.0.as_str()).collect();
        assert!(
            family_ids.contains(&"vermilion_bird"),
            "vermilion_bird boss pack must contain vermilion_bird"
        );
        assert!(
            family_ids.contains(&"vermilion_bird_tail_A"),
            "vermilion_bird boss pack must contain vermilion_bird_tail_A"
        );
        assert!(
            family_ids.contains(&"vermilion_bird_tail_B"),
            "vermilion_bird boss pack must contain vermilion_bird_tail_B"
        );
    }

    #[test]
    fn zhuque_boss_gambler_pack_is_correct() {
        let registry = build_packs_registry();

        let pack = registry
            .get("zhuque_boss_gambler")
            .expect("zhuque_boss_gambler should exist");

        assert_eq!(pack.dungeon, Dungeon::ZhuQue);
        assert_eq!(pack.pack_type, PackType::Boss);
        assert_eq!(pack.total_units(), 1, "gambler boss pack starts with 1 unit (mahjong summoned mid-fight)");

        let family_ids: Vec<&str> = pack.family_ids().iter().map(|id| id.0.as_str()).collect();
        assert!(
            family_ids.contains(&"gambler"),
            "gambler boss pack must contain gambler"
        );
    }

    #[test]
    fn baihu_boss_white_tiger_pack_is_correct() {
        let registry = build_packs_registry();

        let pack = registry
            .get("baihu_boss_white_tiger")
            .expect("baihu_boss_white_tiger should exist");

        assert_eq!(pack.dungeon, Dungeon::BaiHu);
        assert_eq!(pack.pack_type, PackType::Boss);
        assert_eq!(pack.total_units(), 3, "white_tiger boss pack should have 3 units");

        let family_ids: Vec<&str> = pack.family_ids().iter().map(|id| id.0.as_str()).collect();
        assert!(
            family_ids.contains(&"white_tiger_A"),
            "white_tiger boss pack must contain white_tiger_A"
        );
        assert!(
            family_ids.contains(&"white_tiger_B"),
            "white_tiger boss pack must contain white_tiger_B"
        );
        assert!(
            family_ids.contains(&"white_tiger_terrain"),
            "white_tiger boss pack must contain white_tiger_terrain"
        );
    }

    #[test]
    fn xuanwu_boss_black_tortoise_pack_is_correct() {
        let registry = build_packs_registry();

        let pack = registry
            .get("xuanwu_boss_black_tortoise")
            .expect("xuanwu_boss_black_tortoise should exist");

        assert_eq!(pack.dungeon, Dungeon::XuanWu);
        assert_eq!(pack.pack_type, PackType::Boss);
        assert_eq!(pack.total_units(), 2, "black_tortoise boss pack should have 2 units");

        let family_ids: Vec<&str> = pack.family_ids().iter().map(|id| id.0.as_str()).collect();
        assert!(
            family_ids.contains(&"black_tortoise_A"),
            "black_tortoise boss pack must contain black_tortoise_A"
        );
        assert!(
            family_ids.contains(&"black_tortoise_B"),
            "black_tortoise boss pack must contain black_tortoise_B"
        );
    }

    #[test]
    fn xuanwu_boss_rotvine_wraith_pack_is_correct() {
        let registry = build_packs_registry();

        let pack = registry
            .get("xuanwu_boss_rotvine_wraith")
            .expect("xuanwu_boss_rotvine_wraith should exist");

        assert_eq!(pack.dungeon, Dungeon::XuanWu);
        assert_eq!(pack.pack_type, PackType::Boss);
        assert_eq!(pack.total_units(), 3, "rotvine_wraith boss pack should have 3 units");

        let family_ids: Vec<&str> = pack.family_ids().iter().map(|id| id.0.as_str()).collect();
        assert!(
            family_ids.contains(&"rotvine_wraith"),
            "rotvine_wraith boss pack must contain rotvine_wraith"
        );
        assert!(
            family_ids.contains(&"rotten_fruit_A"),
            "rotvine_wraith boss pack must contain rotten_fruit_A"
        );
        assert!(
            family_ids.contains(&"rotten_fruit_B"),
            "rotvine_wraith boss pack must contain rotten_fruit_B"
        );
    }

    #[test]
    fn xuanwu_boss_skeletal_tiller_pack_is_correct() {
        let registry = build_packs_registry();

        let pack = registry
            .get("xuanwu_boss_skeletal_tiller")
            .expect("xuanwu_boss_skeletal_tiller should exist");

        assert_eq!(pack.dungeon, Dungeon::XuanWu);
        assert_eq!(pack.pack_type, PackType::Boss);
        assert_eq!(pack.total_units(), 2, "skeletal_tiller boss pack should have 2 units");

        let family_ids: Vec<&str> = pack.family_ids().iter().map(|id| id.0.as_str()).collect();
        assert!(
            family_ids.contains(&"skeletal_tiller"),
            "skeletal_tiller boss pack must contain skeletal_tiller"
        );
        assert!(
            family_ids.contains(&"vegetable"),
            "skeletal_tiller boss pack must contain vegetable"
        );
    }

    #[test]
    fn xuanwu_boss_necrodrake_embryosac_pack_is_correct() {
        let registry = build_packs_registry();

        let pack = registry
            .get("xuanwu_boss_necrodrake_embryosac")
            .expect("xuanwu_boss_necrodrake_embryosac should exist");

        assert_eq!(pack.dungeon, Dungeon::XuanWu);
        assert_eq!(pack.pack_type, PackType::Boss);
        assert_eq!(pack.total_units(), 3, "necrodrake_embryosac boss pack should have 3 units");

        let family_ids: Vec<&str> = pack.family_ids().iter().map(|id| id.0.as_str()).collect();
        assert!(
            family_ids.contains(&"necrodrake_embryosac"),
            "necrodrake_embryosac boss pack must contain necrodrake_embryosac"
        );
        assert!(
            family_ids.contains(&"egg_membrane_empty"),
            "necrodrake_embryosac boss pack must contain egg_membrane_empty"
        );
    }
}