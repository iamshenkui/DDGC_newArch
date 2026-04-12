//! Monster family semantic parity integration tests.
//!
//! Verifies that every migrated common monster family preserves its original
//! identity: dungeon assignment, role, monster type, and identity-defining skills.

use game_ddgc_headless::content::ContentPack;
use game_ddgc_headless::monsters::build_registry;
use game_ddgc_headless::parity::MonsterFamilyParityFixture;

use framework_combat::skills::SkillId;

/// Verifies that every migrated common family resolves in the registry
/// with the correct dungeon, role, and monster type.
#[test]
fn all_common_families_resolve_with_correct_metadata() {
    let registry = build_registry();
    let fixture = MonsterFamilyParityFixture::new();

    for exp in &fixture.families {
        let family = registry
            .get(exp.family_id)
            .unwrap_or_else(|| panic!("{} should be registered in the monster family registry", exp.family_id));

        assert_eq!(
            family.dungeon, exp.dungeon,
            "{} dungeon mismatch: expected {:?}, got {:?}",
            exp.family_id, exp.dungeon, family.dungeon
        );
        assert_eq!(
            family.role, exp.role,
            "{} role mismatch: expected {:?}, got {:?}",
            exp.family_id, exp.role, family.role
        );
        assert_eq!(
            family.monster_type, exp.monster_type,
            "{} monster_type mismatch: expected {:?}, got {:?}",
            exp.family_id, exp.monster_type, family.monster_type
        );
    }
}

/// Verifies that every identity-defining skill for each family exists in the
/// ContentPack and is registered under the family's skill list.
#[test]
fn all_identity_skills_exist_in_content_pack_and_registry() {
    let pack = ContentPack::default();
    let registry = build_registry();
    let fixture = MonsterFamilyParityFixture::new();

    for exp in &fixture.families {
        let family = registry.get(exp.family_id).unwrap();

        for &skill_id in exp.identity_skills {
            // Skill must exist in ContentPack
            let skill = pack
                .get_skill(&SkillId::new(skill_id))
                .unwrap_or_else(|| panic!("{} skill {} should exist in ContentPack", exp.family_id, skill_id));

            // Skill must be listed in family's skill_ids
            let registered = family.skill_ids.iter().any(|s| s.0 == skill_id);
            assert!(
                registered,
                "{} must list skill {} in its registry skill_ids",
                exp.family_id, skill_id
            );

            // Skill must have at least one effect (no empty skills)
            assert!(
                !skill.effects.is_empty(),
                "{} skill {} must have at least one effect",
                exp.family_id,
                skill_id
            );
        }
    }
}

/// Verifies that all 22 common families are covered by the parity fixture.
#[test]
fn parity_fixture_covers_all_common_families() {
    let fixture = MonsterFamilyParityFixture::new();
    assert_eq!(
        fixture.families.len(),
        22,
        "parity fixture must cover all 22 migrated common families"
    );
}

/// Verifies dungeon distribution is correct: each dungeon has the expected
/// number of common families.
#[test]
fn dungeon_distribution_matches_original_design() {
    let fixture = MonsterFamilyParityFixture::new();

    use game_ddgc_headless::monsters::Dungeon;

    let qinglong: Vec<_> = fixture.families.iter().filter(|f| f.dungeon == Dungeon::QingLong).collect();
    let baihu: Vec<_> = fixture.families.iter().filter(|f| f.dungeon == Dungeon::BaiHu).collect();
    let zhuque: Vec<_> = fixture.families.iter().filter(|f| f.dungeon == Dungeon::ZhuQue).collect();
    let xuanwu: Vec<_> = fixture.families.iter().filter(|f| f.dungeon == Dungeon::XuanWu).collect();

    assert_eq!(qinglong.len(), 8, "QingLong should have 8 common families");
    assert_eq!(baihu.len(), 6, "BaiHu should have 6 common families");
    assert_eq!(zhuque.len(), 5, "ZhuQue should have 5 common families");
    assert_eq!(xuanwu.len(), 3, "XuanWu should have 3 common families");
}
