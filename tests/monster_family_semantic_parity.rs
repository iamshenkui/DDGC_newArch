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

/// Verifies that ALL skills referenced by common monster families exist in the
/// ContentPack and can be used in encounter execution.
///
/// This test closes the common-monster combat skill gap by ensuring no skill
/// referenced in the monster family registry is missing from the content layer.
/// It validates the full skill set (not just identity skills) against the
/// source-of-truth registry in src/monsters/mod.rs.
#[test]
fn all_common_family_skills_exist_in_content_pack() {
    use game_ddgc_headless::monsters::MonsterTier;

    let pack = ContentPack::default();
    let registry = build_registry();

    // Get all common monster families from the registry
    // Get all common monster families from the registry
    let common_families: Vec<_> = registry.by_tier(MonsterTier::Common);

    // Every common family must have at least one skill
    assert!(
        !common_families.is_empty(),
        "Registry must have at least one common monster family"
    );

    let mut missing_skills = Vec::new();

    for family in common_families {
        for skill_id in &family.skill_ids {
            let skill = pack.get_skill(skill_id);
            if skill.is_none() {
                missing_skills.push(format!(
                    "{} references skill '{}' which is missing from ContentPack",
                    family.id.0, skill_id.0
                ));
            } else if skill.unwrap().effects.is_empty() {
                missing_skills.push(format!(
                    "{} has skill '{}' with no effects (empty skill)",
                    family.id.0, skill_id.0
                ));
            }
        }
    }

    assert!(
        missing_skills.is_empty(),
        "Missing or empty skills found:\n{}",
        missing_skills.join("\n")
    );
}

/// Verifies that all common monster skills pass validation and can participate
/// in encounter execution by checking they have valid effect chains.
///
/// This proves migrated common-monster skills are not just defined but are
/// actually usable in encounter resolution.
#[test]
fn all_common_family_skills_validate_for_encounter_use() {
    use game_ddgc_headless::monsters::MonsterTier;

    let pack = ContentPack::default();
    let registry = build_registry();

    let common_families: Vec<_> = registry.by_tier(MonsterTier::Common);

    let mut invalid_skills = Vec::new();

    for family in common_families {
        for skill_id in &family.skill_ids {
            if let Some(skill) = pack.get_skill(skill_id) {
                // Skill must validate (proper effect chain, targeting, etc.)
                if let Err(e) = skill.validate() {
                    invalid_skills.push(format!(
                        "{} skill '{}' failed validation: {:?}",
                        family.id.0, skill_id.0, e
                    ));
                }

                // Skill must have at least one effect for encounter participation
                if skill.effects.is_empty() {
                    invalid_skills.push(format!(
                        "{} skill '{}' has no effects and cannot participate in encounters",
                        family.id.0, skill_id.0
                    ));
                }
            }
        }
    }

    assert!(
        invalid_skills.is_empty(),
        "Skills that cannot participate in encounters:\n{}",
        invalid_skills.join("\n")
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

/// Verifies that movement skills (push/pull effects on self) use SelfOnly targeting.
///
/// DDGC movement skills (.move) use push/pull to reposition the actor itself,
/// targeting @23 (self/allies in DDGC terms). This is distinct from attack
/// skills that also have pull effects on targets (e.g., tiger_sword pull,
/// water_grass convolve). This test ensures pure movement skills correctly
/// use SelfOnly targeting per DDGC source data.
#[test]
fn movement_skills_use_self_only_targeting() {
    use game_ddgc_headless::monsters::MonsterTier;
    use framework_combat::targeting::TargetSelector;

    let pack = ContentPack::default();
    let registry = build_registry();
    let common_families: Vec<_> = registry.by_tier(MonsterTier::Common);

    let mut movement_with_wrong_targeting = Vec::new();

    for family in common_families {
        for skill_id in &family.skill_ids {
            // Only check skills named "move" - these are the pure movement skills
            // that use push/pull for self-repositioning
            if skill_id.0 != "move" {
                continue;
            }

            if let Some(skill) = pack.get_skill(skill_id) {
                // Movement skills must target SelfOnly
                if !matches!(skill.target_selector, TargetSelector::SelfOnly) {
                    movement_with_wrong_targeting.push(format!(
                        "{} movement skill '{}' targets {:?}, expected SelfOnly",
                        family.id.0, skill_id.0, skill.target_selector
                    ));
                }
            }
        }
    }

    assert!(
        movement_with_wrong_targeting.is_empty(),
        "Movement skills with incorrect targeting:\n{}",
        movement_with_wrong_targeting.join("\n")
    );
}

/// Verifies that all common monster skills have valid action costs.
///
/// DDGC skills have action_cost >= 1 (one action per turn for common monsters).
/// This test ensures the framework correctly captures DDGC action semantics.
#[test]
fn all_common_skill_action_costs_are_valid() {
    use game_ddgc_headless::monsters::MonsterTier;

    let pack = ContentPack::default();
    let registry = build_registry();
    let common_families: Vec<_> = registry.by_tier(MonsterTier::Common);

    let mut invalid_action_costs = Vec::new();

    for family in common_families {
        for skill_id in &family.skill_ids {
            if let Some(skill) = pack.get_skill(skill_id) {
                if skill.action_cost < 1 {
                    invalid_action_costs.push(format!(
                        "{} skill '{}' has action_cost {} (must be >= 1)",
                        family.id.0, skill_id.0, skill.action_cost
                    ));
                }
            }
        }
    }

    assert!(
        invalid_action_costs.is_empty(),
        "Skills with invalid action costs:\n{}",
        invalid_action_costs.join("\n")
    );
}

/// Verifies that each common monster family has identity skills defined
/// in ContentPack matching the parity fixture expectations.
///
/// This test closes the loop between the source-backed inventory
/// (MonsterFamilyParityFixture) and the actual skill definitions,
/// ensuring DDGC source data is correctly migrated.
#[test]
fn identity_skills_match_parity_fixture_expectations() {
    let pack = ContentPack::default();
    let fixture = MonsterFamilyParityFixture::new();

    let mut missing_identity = Vec::new();

    for exp in &fixture.families {
        for &skill_id_str in exp.identity_skills {
            let skill_id = framework_combat::skills::SkillId::new(skill_id_str);
            if let None = pack.get_skill(&skill_id) {
                missing_identity.push(format!(
                    "{} identity skill '{}' (from parity fixture) not found in ContentPack",
                    exp.family_id, skill_id_str
                ));
            }
        }
    }

    assert!(
        missing_identity.is_empty(),
        "Missing identity skills:\n{}",
        missing_identity.join("\n")
    );
}

/// Verifies that ALL skills referenced by boss monster families exist in the
/// ContentPack and can be used in encounter execution.
///
/// This test closes the boss combat skill gap by ensuring no skill
/// referenced in the boss family registry is missing from the content layer.
/// It validates the full skill set against the source-of-truth registry
/// in src/monsters/mod.rs, covering QingLong, ZhuQue, BaiHu, XuanWu,
/// and cross-dungeon boss families.
#[test]
fn all_boss_family_skills_exist_in_content_pack() {
    use game_ddgc_headless::monsters::MonsterTier;

    let pack = ContentPack::default();
    let registry = build_registry();

    // Get all boss monster families from the registry
    let boss_families: Vec<_> = registry.by_tier(MonsterTier::Boss);

    // Every boss family must have at least one skill
    assert!(
        !boss_families.is_empty(),
        "Registry must have at least one boss monster family"
    );

    let mut missing_skills = Vec::new();

    for family in boss_families {
        for skill_id in &family.skill_ids {
            let skill = pack.get_skill(skill_id);
            if skill.is_none() {
                missing_skills.push(format!(
                    "{} references skill '{}' which is missing from ContentPack",
                    family.id.0, skill_id.0
                ));
            } else if skill.unwrap().effects.is_empty() {
                missing_skills.push(format!(
                    "{} has skill '{}' with no effects (empty skill)",
                    family.id.0, skill_id.0
                ));
            }
        }
    }

    assert!(
        missing_skills.is_empty(),
        "Missing or empty skills found:\n{}",
        missing_skills.join("\n")
    );
}

/// Verifies that all boss monster skills pass validation and can participate
/// in encounter execution by checking they have valid effect chains.
///
/// This proves migrated boss skills are not just defined but are actually
/// usable in encounter resolution. Covers QingLong (azure_dragon),
/// ZhuQue (vermilion_bird, gambler), BaiHu (white_tiger),
/// XuanWu (black_tortoise, rotvine_wraith, skeletal_tiller, necrodrake,
/// frostvein_clam, scorchthroat), and cross-dungeon (bloodthirsty_assassin,
/// glutton_pawnshop) boss families.
#[test]
fn all_boss_family_skills_validate_for_encounter_use() {
    use game_ddgc_headless::monsters::MonsterTier;

    let pack = ContentPack::default();
    let registry = build_registry();

    let boss_families: Vec<_> = registry.by_tier(MonsterTier::Boss);

    let mut invalid_skills = Vec::new();

    for family in boss_families {
        for skill_id in &family.skill_ids {
            if let Some(skill) = pack.get_skill(skill_id) {
                // Skill must validate (proper effect chain, targeting, etc.)
                if let Err(e) = skill.validate() {
                    invalid_skills.push(format!(
                        "{} skill '{}' failed validation: {:?}",
                        family.id.0, skill_id.0, e
                    ));
                }

                // Skill must have at least one effect for encounter participation
                if skill.effects.is_empty() {
                    invalid_skills.push(format!(
                        "{} skill '{}' has no effects and cannot participate in encounters",
                        family.id.0, skill_id.0
                    ));
                }
            }
        }
    }

    assert!(
        invalid_skills.is_empty(),
        "Skills that cannot participate in encounters:\n{}",
        invalid_skills.join("\n")
    );
}
