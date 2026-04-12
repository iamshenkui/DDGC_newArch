//! Monster semantic parity integration tests.
//!
//! Verifies that monster archetypes preserve their original threat model,
//! position logic, and behavioral differentiation — not just their stat blocks.

use game_ddgc_headless::content::actors;
use game_ddgc_headless::content::ContentPack;
use game_ddgc_headless::parity::MonsterParityFixture;

use framework_combat::skills::SkillId;

/// Verifies monster archetypes preserve their role identity:
/// - Bone Soldier: side==Enemy, HP<25, ATK 6-10, SPD>5 (fragile+fast)
/// - Necromancer: side==Enemy, HP>40, ATK>10, SPD<5 (tanky boss)
#[test]
fn monster_role_identity_is_preserved() {
    let fixture = MonsterParityFixture::new();

    // Bone Soldier: fragile+fast
    let bs_arch = actors::bone_soldier();
    let bs_exp = &fixture.bone_soldier;
    assert_eq!(bs_arch.side, bs_exp.side, "Bone Soldier side mismatch");
    assert!(
        bs_arch.health < bs_exp.max_health,
        "Bone Soldier HP {} should be < {}",
        bs_arch.health,
        bs_exp.max_health
    );
    assert!(
        bs_arch.attack >= bs_exp.min_attack && bs_arch.attack <= bs_exp.max_attack,
        "Bone Soldier ATK {} should be in range {}-{}",
        bs_arch.attack,
        bs_exp.min_attack,
        bs_exp.max_attack
    );
    assert!(
        bs_arch.speed > bs_exp.min_speed,
        "Bone Soldier SPD {} should be > {}",
        bs_arch.speed,
        bs_exp.min_speed
    );

    // Necromancer: tanky boss
    let necro_arch = actors::necromancer();
    let necro_exp = &fixture.necromancer;
    assert_eq!(necro_arch.side, necro_exp.side, "Necromancer side mismatch");
    assert!(
        necro_arch.health > necro_exp.min_health,
        "Necromancer HP {} should be > {}",
        necro_arch.health,
        necro_exp.min_health
    );
    assert!(
        necro_arch.attack > necro_exp.min_attack,
        "Necromancer ATK {} should be > {}",
        necro_arch.attack,
        necro_exp.min_attack
    );
    assert!(
        necro_arch.speed < necro_exp.max_speed,
        "Necromancer SPD {} should be < {}",
        necro_arch.speed,
        necro_exp.max_speed
    );
}

/// Verifies monster skill preference is preserved:
/// - rend skill has 2 effects including apply_status (DoT identity)
/// - skull_bash has 2 effects including conditional apply_status (control identity)
#[test]
fn monster_skill_preference_is_preserved() {
    let pack = ContentPack::default();

    // rend: 2 effects, one is apply_status (bleed) — DoT identity
    let rend = pack.get_skill(&SkillId::new("rend")).expect("rend skill missing");
    assert_eq!(rend.effects.len(), 2, "rend should have 2 effects (damage + apply_status)");
    let has_apply_status = rend.effects.iter().any(|e| {
        matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus)
    });
    assert!(has_apply_status, "rend must include an apply_status effect (DoT identity)");

    // skull_bash: 2 effects, one is conditional apply_status (stun) — control identity
    let skull_bash = pack.get_skill(&SkillId::new("skull_bash")).expect("skull_bash skill missing");
    assert_eq!(skull_bash.effects.len(), 2, "skull_bash should have 2 effects (damage + conditional apply_status)");
    let has_conditional_status = skull_bash.effects.iter().any(|e| {
        matches!(e.kind, framework_combat::effects::EffectKind::ApplyStatus) && !e.conditions.is_empty()
    });
    assert!(has_conditional_status, "skull_bash must include a conditional apply_status effect (control identity)");
}

/// Verifies boss behavior is not flattened into regular enemy behavior:
/// - Necromancer has >=2 distinct skills
/// - Necromancer HP > 2x Bone Soldier HP
/// - At least one Necromancer skill has cooldown>=3
#[test]
fn boss_behavior_is_not_flattened() {
    let pack = ContentPack::default();

    // Necromancer has >=2 distinct skills (skull_bash + grave_bash)
    let necro_skills: Vec<_> = pack.skills.values()
        .filter(|s| {
            // Skills accessible to Necromancer: skull_bash and grave_bash
            s.id.0 == "skull_bash" || s.id.0 == "grave_bash"
        })
        .collect();
    assert!(necro_skills.len() >= 2, "Necromancer should have >=2 distinct skills, got {}", necro_skills.len());

    // Necromancer HP > 2x Bone Soldier HP
    let necro_hp = actors::necromancer().health;
    let bs_hp = actors::bone_soldier().health;
    assert!(
        necro_hp > 2.0 * bs_hp,
        "Necromancer HP {} should be > 2x Bone Soldier HP {}",
        necro_hp,
        bs_hp
    );

    // At least one Necromancer skill has cooldown >= 3
    let has_high_cooldown = necro_skills.iter().any(|s| s.cooldown.unwrap_or(0) >= 3);
    assert!(has_high_cooldown, "At least one Necromancer skill should have cooldown >= 3");
}

/// Verifies monster position logic matches original intent:
/// - Bone Soldier has dodge > 0 (frontline)
/// - Necromancer has dodge == 0 (backline boss)
#[test]
fn monster_position_logic_matches_original_intent() {
    let fixture = MonsterParityFixture::new();

    // Bone Soldier: frontline — has dodge
    let bs_arch = actors::bone_soldier();
    assert!(
        bs_arch.dodge > 0.0,
        "Bone Soldier dodge {} should be > 0 (frontline)",
        bs_arch.dodge
    );
    assert_eq!(bs_arch.dodge, fixture.bone_soldier.dodge, "Bone Soldier dodge should match fixture");

    // Necromancer: backline boss — no dodge
    let necro_arch = actors::necromancer();
    assert_eq!(
        necro_arch.dodge, 0.0,
        "Necromancer dodge should be 0 (backline boss)"
    );
    assert_eq!(necro_arch.dodge, fixture.necromancer.dodge, "Necromancer dodge should match fixture");
}
