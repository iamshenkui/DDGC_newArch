//! Tests for camping phase resolution (US-006-b / US-007-b).
//!
//! Verifies that `CampingPhase` correctly tracks time budget, per-skill usage,
//! participating heroes, and activity trace. Also verifies that
//! `perform_camping_skill` enforces time budget, use limits, class eligibility,
//! and target legality using DDGC's original targeting semantics.
//!
//! All tests use `game_ddgc_headless::run::camping` public APIs.

use game_ddgc_headless::run::camping::{
    CampingPhase, CampingSkill, CampEffect, CampEffectType, CampTargetSelection,
    HeroInCamp, DEFAULT_CAMP_TIME_BUDGET, perform_camping_skill,
};

// ── Test helpers ───────────────────────────────────────────────────────────────

/// Helper: create a simple camping skill.
fn make_skill(
    id: &str,
    time_cost: u32,
    use_limit: u32,
    classes: Vec<&str>,
    selection: CampTargetSelection,
) -> CampingSkill {
    CampingSkill {
        id: id.to_string(),
        time_cost,
        use_limit,
        has_individual_target: matches!(
            selection,
            CampTargetSelection::Individual | CampTargetSelection::PartyOther
        ),
        classes: classes.iter().map(|s| s.to_string()).collect(),
        effects: vec![CampEffect {
            selection,
            requirements: Vec::new(),
            chance: 1.0,
            effect_type: CampEffectType::StressHealAmount,
            sub_type: String::new(),
            amount: 10.0,
        }],
    }
}

/// Helper: create a hero in camp.
fn make_hero(hero_id: &str, class_id: &str) -> HeroInCamp {
    HeroInCamp::new(hero_id, class_id, 100.0, 100.0, 50.0, 200.0)
}

// ── CampingPhase construction tests ─────────────────────────────────────────────

#[test]
fn camping_phase_new_has_default_budget() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let phase = CampingPhase::new(heroes);
    assert_eq!(phase.time_budget, DEFAULT_CAMP_TIME_BUDGET);
    assert_eq!(phase.time_spent, 0);
    assert!(phase.skill_uses.is_empty());
    assert!(phase.trace.is_empty());
}

#[test]
fn camping_phase_with_custom_budget() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let phase = CampingPhase::with_budget(heroes, 20);
    assert_eq!(phase.time_budget, 20);
    assert_eq!(phase.time_spent, 0);
}

#[test]
fn camping_phase_tracks_heroes() {
    let heroes = vec![
        make_hero("h1", "arbalest"),
        make_hero("h2", "crusader"),
        make_hero("h3", "alchemist"),
    ];
    let phase = CampingPhase::new(heroes);
    assert_eq!(phase.heroes.len(), 3);
    assert_eq!(phase.get_hero("h1").unwrap().class_id, "arbalest");
    assert_eq!(phase.get_hero("h2").unwrap().class_id, "crusader");
    assert_eq!(phase.get_hero("h3").unwrap().class_id, "alchemist");
}

#[test]
fn camping_phase_starts_with_empty_trace() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let phase = CampingPhase::new(heroes);
    assert!(phase.trace.is_empty());
}

// ── Time budget tests ──────────────────────────────────────────────────────────

#[test]
fn has_time_for_returns_true_when_enough_time() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let phase = CampingPhase::with_budget(heroes, 12);
    assert!(phase.has_time_for(5));
    assert!(phase.has_time_for(12));
}

#[test]
fn has_time_for_returns_false_when_not_enough_time() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let phase = CampingPhase::with_budget(heroes, 12);
    assert!(!phase.has_time_for(13));
}

#[test]
fn remaining_time_decreases_as_time_spent() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let mut phase = CampingPhase::with_budget(heroes, 12);
    assert_eq!(phase.remaining_time(), 12);
    phase.time_spent = 5;
    assert_eq!(phase.remaining_time(), 7);
}

#[test]
fn time_budget_exhaustion_blocks_skill() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let mut phase = CampingPhase::with_budget(heroes, 12);

    // Use skills until time runs out
    let skill = make_skill("test_skill", 5, 10, vec![], CampTargetSelection::SelfTarget);

    // First use: 12 - 5 = 7 remaining
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success, "First use should succeed: {:?}", result.error);

    // Second use: 7 - 5 = 2 remaining
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success, "Second use should succeed: {:?}", result.error);

    // Third use: 2 - 5 = not enough
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(!result.success, "Third use should fail due to time budget");
    assert!(
        result.error.unwrap().contains("Insufficient time"),
        "Error should mention insufficient time"
    );
}

#[test]
fn time_cost_deducted_on_success() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let mut phase = CampingPhase::with_budget(heroes, 12);

    let skill = make_skill("test_skill", 3, 10, vec![], CampTargetSelection::SelfTarget);
    assert_eq!(phase.time_spent, 0);

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);
    assert_eq!(phase.time_spent, 3);

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);
    assert_eq!(phase.time_spent, 6);
}

#[test]
fn time_spent_not_deducted_on_failure() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let mut phase = CampingPhase::with_budget(heroes, 12);

    let skill = make_skill("test_skill", 100, 10, vec![], CampTargetSelection::SelfTarget);
    assert_eq!(phase.time_spent, 0);

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(!result.success);
    assert_eq!(phase.time_spent, 0, "Time should not be deducted on failed skill");
}

// ── Use limit tests ────────────────────────────────────────────────────────────

#[test]
fn use_limit_enforced_per_skill() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let mut phase = CampingPhase::with_budget(heroes, 100);

    let skill = make_skill("limited_skill", 1, 2, vec![], CampTargetSelection::SelfTarget);

    // First use: allowed
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success, "First use should succeed: {:?}", result.error);
    assert_eq!(phase.skill_use_count("limited_skill"), 1);

    // Second use: allowed (at limit)
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success, "Second use should succeed: {:?}", result.error);
    assert_eq!(phase.skill_use_count("limited_skill"), 2);

    // Third use: blocked
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(!result.success, "Third use should fail due to limit");
    assert!(
        result.error.unwrap().contains("use limit reached"),
        "Error should mention use limit"
    );
}

#[test]
fn different_skills_have_independent_limits() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let mut phase = CampingPhase::with_budget(heroes, 100);

    let skill_a = make_skill("skill_a", 1, 1, vec![], CampTargetSelection::SelfTarget);
    let skill_b = make_skill("skill_b", 1, 1, vec![], CampTargetSelection::SelfTarget);

    // Use skill_a
    let result = perform_camping_skill(&mut phase, &skill_a, "h1", Some("h1"));
    assert!(result.success, "skill_a first use should succeed: {:?}", result.error);

    // skill_a is exhausted
    let result = perform_camping_skill(&mut phase, &skill_a, "h1", Some("h1"));
    assert!(!result.success, "skill_a second use should fail");

    // skill_b should still work
    let result = perform_camping_skill(&mut phase, &skill_b, "h1", Some("h1"));
    assert!(result.success, "skill_b should work independently: {:?}", result.error);
}

#[test]
fn use_limit_blocks_before_time_budget() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let mut phase = CampingPhase::with_budget(heroes, 100);

    // Skill with limit 1 but plenty of time
    let skill = make_skill("one_use", 1, 1, vec![], CampTargetSelection::SelfTarget);

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);

    // Still has time (99 remaining) but limit is 1
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(!result.success, "Limit should block even with remaining time");
    assert!(result.error.unwrap().contains("use limit"));
}

// ── Class restriction tests ─────────────────────────────────────────────────────

#[test]
fn class_specific_skill_blocks_wrong_class() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let mut phase = CampingPhase::new(heroes);

    // Arbalest-only skill
    let skill = make_skill(
        "arbalest_skill",
        2,
        10,
        vec!["arbalest"],
        CampTargetSelection::SelfTarget,
    );

    // Alchemist cannot use arbalest skill
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(!result.success, "Wrong class should be blocked");
    assert!(
        result.error.unwrap().contains("cannot use skill"),
        "Error should mention class restriction"
    );
}

#[test]
fn class_specific_skill_allows_correct_class() {
    let heroes = vec![make_hero("h1", "arbalest")];
    let mut phase = CampingPhase::new(heroes);

    // Arbalest-only skill
    let skill = make_skill(
        "arbalest_skill",
        2,
        10,
        vec!["arbalest"],
        CampTargetSelection::SelfTarget,
    );

    // Arbalest can use arbalest skill
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success, "Correct class should be allowed: {:?}", result.error);
}

#[test]
fn generic_skill_available_to_all_classes() {
    let classes = ["alchemist", "arbalest", "hunter", "crusader"];
    for class_id in classes {
        let heroes = vec![make_hero("h1", class_id)];
        let mut phase = CampingPhase::new(heroes);

        // Generic skill (empty classes list)
        let skill = make_skill("generic_skill", 2, 10, vec![], CampTargetSelection::SelfTarget);

        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
        assert!(
            result.success,
            "Generic skill should be available to class '{}': {:?}",
            class_id,
            result.error
        );
    }
}

#[test]
fn multi_class_skill_allows_any_listed_class() {
    let allowed_classes = vec!["arbalest", "musketeer"];
    for class_id in &allowed_classes {
        let heroes = vec![make_hero("h1", class_id)];
        let mut phase = CampingPhase::new(heroes);

        let skill = make_skill(
            "multi_class_skill",
            2,
            10,
            allowed_classes.clone(),
            CampTargetSelection::SelfTarget,
        );

        let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
        assert!(
            result.success,
            "Class '{}' should be allowed for multi-class skill: {:?}",
            class_id,
            result.error
        );
    }
}

#[test]
fn multi_class_skill_blocks_unlisted_class() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let mut phase = CampingPhase::new(heroes);

    // Arbalest/musketeer skill
    let skill = make_skill(
        "multi_class_skill",
        2,
        10,
        vec!["arbalest", "musketeer"],
        CampTargetSelection::SelfTarget,
    );

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(!result.success, "Unlisted class should be blocked");
    assert!(result.error.unwrap().contains("cannot use skill"));
}

// ── Targeting validation tests ─────────────────────────────────────────────────

#[test]
fn self_target_requires_performer_as_target() {
    let heroes = vec![make_hero("h1", "alchemist"), make_hero("h2", "alchemist")];
    let mut phase = CampingPhase::new(heroes);

    let skill = make_skill("self_skill", 2, 10, vec![], CampTargetSelection::SelfTarget);

    // Targeting self is allowed
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success, "Self-target should succeed: {:?}", result.error);

    // Targeting other is not allowed for SelfTarget
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h2"));
    assert!(!result.success, "Cross-target should fail for SelfTarget");
    assert!(
        result.error.unwrap().contains("must target self"),
        "Error should mention must target self"
    );
}

#[test]
fn party_other_cannot_target_self() {
    let heroes = vec![make_hero("h1", "alchemist"), make_hero("h2", "alchemist")];
    let mut phase = CampingPhase::new(heroes);

    let skill = make_skill(
        "party_other_skill",
        2,
        10,
        vec![],
        CampTargetSelection::PartyOther,
    );

    // Cannot target self
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(!result.success, "Self-target should fail for PartyOther");
    assert!(
        result.error.unwrap().contains("cannot target self"),
        "Error should mention cannot target self"
    );
}

#[test]
fn party_other_allows_other_targets() {
    let heroes = vec![make_hero("h1", "alchemist"), make_hero("h2", "alchemist")];
    let mut phase = CampingPhase::new(heroes);

    let skill = make_skill(
        "party_other_skill",
        2,
        10,
        vec![],
        CampTargetSelection::PartyOther,
    );

    // Targeting other hero is allowed
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h2"));
    assert!(result.success, "Other-target should succeed: {:?}", result.error);
}

#[test]
fn party_all_ignores_target() {
    let heroes = vec![make_hero("h1", "alchemist"), make_hero("h2", "alchemist")];
    let mut phase = CampingPhase::new(heroes);

    let skill = make_skill("party_all_skill", 2, 10, vec![], CampTargetSelection::PartyAll);

    // PartyAll should succeed even without a target
    let result = perform_camping_skill(&mut phase, &skill, "h1", None);
    assert!(result.success, "PartyAll with no target should succeed: {:?}", result.error);

    // PartyAll should also succeed with a target (target is ignored)
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h2"));
    assert!(result.success, "PartyAll with target should succeed: {:?}", result.error);
}

#[test]
fn individual_target_accepts_any_hero() {
    let heroes = vec![make_hero("h1", "alchemist"), make_hero("h2", "alchemist")];
    let mut phase = CampingPhase::new(heroes);

    let skill = make_skill(
        "individual_skill",
        2,
        10,
        vec![],
        CampTargetSelection::Individual,
    );

    // Can target self
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success, "Self-target should succeed: {:?}", result.error);

    // Can target other
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h2"));
    assert!(result.success, "Other-target should succeed: {:?}", result.error);
}

#[test]
fn nonexistent_target_fails() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let mut phase = CampingPhase::new(heroes);

    let skill = make_skill(
        "individual_skill",
        2,
        10,
        vec![],
        CampTargetSelection::Individual,
    );

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("nonexistent"));
    assert!(!result.success, "Nonexistent target should fail");
    assert!(
        result.error.unwrap().contains("not found"),
        "Error should mention target not found"
    );
}

#[test]
fn nonexistent_performer_fails() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let mut phase = CampingPhase::new(heroes);

    let skill = make_skill(
        "individual_skill",
        2,
        10,
        vec![],
        CampTargetSelection::Individual,
    );

    let result = perform_camping_skill(&mut phase, &skill, "nonexistent", Some("h1"));
    assert!(!result.success, "Nonexistent performer should fail");
    assert!(
        result.error.unwrap().contains("not found"),
        "Error should mention performer not found"
    );
}

// ── Activity trace tests ──────────────────────────────────────────────────────

#[test]
fn successful_skill_appends_trace_record() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let mut phase = CampingPhase::new(heroes);

    let skill = make_skill("trace_test", 2, 10, vec![], CampTargetSelection::SelfTarget);
    assert!(phase.trace.is_empty());

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success, "Skill should succeed: {:?}", result.error);
    assert_eq!(phase.trace.len(), 1);
    assert_eq!(phase.trace[0].skill_id, "trace_test");
    assert_eq!(phase.trace[0].performer_id, "h1");
    assert!(phase.trace[0].success);
}

#[test]
fn failed_skill_does_not_append_trace() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let mut phase = CampingPhase::new(heroes);

    // Skill that will fail due to time budget
    let skill = make_skill("trace_test", 100, 10, vec![], CampTargetSelection::SelfTarget);
    assert!(phase.trace.is_empty());

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(!result.success, "Skill should fail due to time");
    // Failed skills still have a record in the result, but don't modify phase.trace
    assert!(phase.trace.is_empty(), "Failed skills should not append to trace");
}

#[test]
fn trace_record_contains_skill_and_target() {
    let heroes = vec![make_hero("h1", "alchemist"), make_hero("h2", "alchemist")];
    let mut phase = CampingPhase::new(heroes);

    let skill = make_skill(
        "targeted_skill",
        2,
        10,
        vec![],
        CampTargetSelection::PartyOther,
    );

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h2"));
    assert!(result.success, "Skill should succeed: {:?}", result.error);

    let record = &phase.trace[0];
    assert_eq!(record.skill_id, "targeted_skill");
    assert_eq!(record.performer_id, "h1");
    assert_eq!(record.target_id.as_deref(), Some("h2"));
    assert_eq!(record.time_cost, 2);
}

#[test]
fn deterministic_trace_for_same_inputs() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let mut phase1 = CampingPhase::new(heroes);
    let heroes2 = vec![make_hero("h1", "alchemist")];
    let mut phase2 = CampingPhase::new(heroes2);

    let skill = make_skill("det_skill", 2, 10, vec![], CampTargetSelection::SelfTarget);

    let result1 = perform_camping_skill(&mut phase1, &skill, "h1", Some("h1"));
    let result2 = perform_camping_skill(&mut phase2, &skill, "h1", Some("h1"));

    assert_eq!(result1.success, result2.success);
    assert_eq!(result1.record.time_cost, result2.record.time_cost);
    assert_eq!(result1.record.skill_id, result2.record.skill_id);
}

#[test]
fn trace_records_accumulate() {
    let heroes = vec![make_hero("h1", "alchemist")];
    let mut phase = CampingPhase::new(heroes);

    let skill_a = make_skill("skill_a", 2, 10, vec![], CampTargetSelection::SelfTarget);
    let skill_b = make_skill("skill_b", 3, 10, vec![], CampTargetSelection::SelfTarget);

    perform_camping_skill(&mut phase, &skill_a, "h1", Some("h1"));
    perform_camping_skill(&mut phase, &skill_b, "h1", Some("h1"));

    assert_eq!(phase.trace.len(), 2);
    assert_eq!(phase.trace[0].skill_id, "skill_a");
    assert_eq!(phase.trace[1].skill_id, "skill_b");
}

// ── Hero activity lock tests ──────────────────────────────────────────────────

#[test]
fn activity_locked_hero_cannot_act() {
    let mut heroes = vec![make_hero("h1", "alchemist")];
    heroes[0].can_use_skills = false;

    let mut phase = CampingPhase::new(heroes);

    let skill = make_skill("test_skill", 2, 10, vec![], CampTargetSelection::SelfTarget);
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(!result.success, "Activity-locked hero should not act");
    assert!(
        result.error.unwrap().contains("cannot use skills"),
        "Error should mention cannot use skills"
    );
}

#[test]
fn hero_can_act_flag_defaults_true() {
    let hero = make_hero("h1", "alchemist");
    assert!(hero.can_use_skills);
}

// ── Effect application tests ───────────────────────────────────────────────────

#[test]
fn stress_heal_effect_applies() {
    let mut heroes = vec![make_hero("h1", "alchemist")];
    heroes[0].stress = 100.0;
    let mut phase = CampingPhase::new(heroes);

    // Use a stress heal skill
    let skill = make_skill("stress_heal", 2, 10, vec![], CampTargetSelection::SelfTarget);
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);

    // Check the hero's stress was reduced (amount is 10.0 from helper)
    let hero = phase.get_hero("h1").unwrap();
    assert_eq!(hero.stress, 90.0);
}

#[test]
fn health_heal_effect_applies() {
    let mut heroes = vec![make_hero("h1", "alchemist")];
    heroes[0].health = 50.0;
    let mut phase = CampingPhase::new(heroes);

    // Create skill with HealthHealAmount effect
    let skill = CampingSkill {
        id: "health_heal".to_string(),
        time_cost: 2,
        use_limit: 10,
        has_individual_target: true,
        classes: vec![],
        effects: vec![CampEffect {
            selection: CampTargetSelection::SelfTarget,
            requirements: Vec::new(),
            chance: 1.0,
            effect_type: CampEffectType::HealthHealAmount,
            sub_type: String::new(),
            amount: 25.0,
        }],
    };

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);

    // Health should be 50 + 25 = 75 (capped at max_health 100)
    let hero = phase.get_hero("h1").unwrap();
    assert_eq!(hero.health, 75.0);
}

#[test]
fn buff_effect_applies() {
    let mut heroes = vec![make_hero("h1", "alchemist")];
    heroes[0].stress = 100.0;
    let mut phase = CampingPhase::new(heroes);

    // Create skill with Buff effect
    let skill = CampingSkill {
        id: "apply_buff".to_string(),
        time_cost: 2,
        use_limit: 10,
        has_individual_target: true,
        classes: vec![],
        effects: vec![CampEffect {
            selection: CampTargetSelection::SelfTarget,
            requirements: Vec::new(),
            chance: 1.0,
            effect_type: CampEffectType::Buff,
            sub_type: "camping_buff_strength".to_string(),
            amount: 0.0,
        }],
    };

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);

    let hero = phase.get_hero("h1").unwrap();
    assert!(hero.active_buffs.contains(&"camping_buff_strength".to_string()));
}

#[test]
fn party_all_effect_applies_to_all_heroes() {
    let mut heroes = vec![
        make_hero("h1", "alchemist"),
        make_hero("h2", "arbalest"),
        make_hero("h3", "crusader"),
    ];
    // Set different stress levels
    heroes[0].stress = 100.0;
    heroes[1].stress = 80.0;
    heroes[2].stress = 60.0;

    let mut phase = CampingPhase::new(heroes);

    let skill = make_skill("party_heal", 3, 10, vec![], CampTargetSelection::PartyAll);
    let result = perform_camping_skill(&mut phase, &skill, "h1", None);
    assert!(result.success);

    // All heroes should have stress reduced by 10
    assert_eq!(phase.get_hero("h1").unwrap().stress, 90.0);
    assert_eq!(phase.get_hero("h2").unwrap().stress, 70.0);
    assert_eq!(phase.get_hero("h3").unwrap().stress, 50.0);
}

// ── Percent-based healing tests ─────────────────────────────────────────────────

#[test]
fn stress_heal_percent_applies() {
    let mut heroes = vec![make_hero("h1", "alchemist")];
    heroes[0].stress = 100.0;
    heroes[0].max_stress = 200.0;
    let mut phase = CampingPhase::new(heroes);

    // Create skill with StressHealPercent effect (20% of max stress)
    let skill = CampingSkill {
        id: "stress_heal_pct".to_string(),
        time_cost: 2,
        use_limit: 10,
        has_individual_target: true,
        classes: vec![],
        effects: vec![CampEffect {
            selection: CampTargetSelection::SelfTarget,
            requirements: Vec::new(),
            chance: 1.0,
            effect_type: CampEffectType::StressHealPercent,
            sub_type: String::new(),
            amount: 0.20, // 20% of max stress
        }],
    };

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);

    // Stress should be reduced by 20% of 200 = 40, so 100 - 40 = 60
    let hero = phase.get_hero("h1").unwrap();
    assert_eq!(hero.stress, 60.0);
}

#[test]
fn health_heal_amount_applies() {
    let mut heroes = vec![make_hero("h1", "alchemist")];
    heroes[0].health = 50.0;
    heroes[0].max_health = 100.0;
    let mut phase = CampingPhase::new(heroes);

    // Create skill with HealthHealAmount effect (flat 30 HP)
    let skill = CampingSkill {
        id: "health_heal_flat".to_string(),
        time_cost: 2,
        use_limit: 10,
        has_individual_target: true,
        classes: vec![],
        effects: vec![CampEffect {
            selection: CampTargetSelection::SelfTarget,
            requirements: Vec::new(),
            chance: 1.0,
            effect_type: CampEffectType::HealthHealAmount,
            sub_type: String::new(),
            amount: 30.0,
        }],
    };

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);

    // Health should be 50 + 30 = 80 (not capped at max_health since 80 < 100)
    let hero = phase.get_hero("h1").unwrap();
    assert_eq!(hero.health, 80.0);
}

#[test]
fn health_heal_amount_caps_at_max_health() {
    let mut heroes = vec![make_hero("h1", "alchemist")];
    heroes[0].health = 90.0;
    heroes[0].max_health = 100.0;
    let mut phase = CampingPhase::new(heroes);

    // Create skill with HealthHealAmount effect (flat 30 HP)
    let skill = CampingSkill {
        id: "health_heal_flat".to_string(),
        time_cost: 2,
        use_limit: 10,
        has_individual_target: true,
        classes: vec![],
        effects: vec![CampEffect {
            selection: CampTargetSelection::SelfTarget,
            requirements: Vec::new(),
            chance: 1.0,
            effect_type: CampEffectType::HealthHealAmount,
            sub_type: String::new(),
            amount: 30.0,
        }],
    };

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);

    // Health should be 90 + 30 = 120, but capped at 100
    let hero = phase.get_hero("h1").unwrap();
    assert_eq!(hero.health, 100.0);
}

// ── Removal effect tests ────────────────────────────────────────────────────────

#[test]
fn remove_bleed_effect_applies() {
    let mut heroes = vec![make_hero("h1", "alchemist")];
    heroes[0].active_buffs = vec!["bleed".to_string(), "other_debuff".to_string()];
    let mut phase = CampingPhase::new(heroes);

    // Create skill with RemoveBleed effect
    let skill = CampingSkill {
        id: "remove_bleed".to_string(),
        time_cost: 2,
        use_limit: 10,
        has_individual_target: true,
        classes: vec![],
        effects: vec![CampEffect {
            selection: CampTargetSelection::SelfTarget,
            requirements: Vec::new(),
            chance: 1.0,
            effect_type: CampEffectType::RemoveBleed,
            sub_type: String::new(),
            amount: 0.0,
        }],
    };

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);

    // bleed should be removed, but other_debuff should remain
    let hero = phase.get_hero("h1").unwrap();
    assert!(!hero.active_buffs.contains(&"bleed".to_string()));
    assert!(hero.active_buffs.contains(&"other_debuff".to_string()));
}

#[test]
fn remove_poison_effect_applies() {
    let mut heroes = vec![make_hero("h1", "alchemist")];
    heroes[0].active_buffs = vec!["poison".to_string(), "bleed".to_string()];
    let mut phase = CampingPhase::new(heroes);

    // Create skill with RemovePoison effect
    let skill = CampingSkill {
        id: "remove_poison".to_string(),
        time_cost: 2,
        use_limit: 10,
        has_individual_target: true,
        classes: vec![],
        effects: vec![CampEffect {
            selection: CampTargetSelection::SelfTarget,
            requirements: Vec::new(),
            chance: 1.0,
            effect_type: CampEffectType::RemovePoison,
            sub_type: String::new(),
            amount: 0.0,
        }],
    };

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);

    // poison should be removed, but bleed should remain
    let hero = phase.get_hero("h1").unwrap();
    assert!(!hero.active_buffs.contains(&"poison".to_string()));
    assert!(hero.active_buffs.contains(&"bleed".to_string()));
}

#[test]
fn remove_burn_effect_applies() {
    let mut heroes = vec![make_hero("h1", "alchemist")];
    heroes[0].active_buffs = vec!["burning".to_string(), "poison".to_string()];
    let mut phase = CampingPhase::new(heroes);

    // Create skill with RemoveBurn effect
    let skill = CampingSkill {
        id: "remove_burn".to_string(),
        time_cost: 2,
        use_limit: 10,
        has_individual_target: true,
        classes: vec![],
        effects: vec![CampEffect {
            selection: CampTargetSelection::SelfTarget,
            requirements: Vec::new(),
            chance: 1.0,
            effect_type: CampEffectType::RemoveBurn,
            sub_type: String::new(),
            amount: 0.0,
        }],
    };

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);

    // burning should be removed, but poison should remain
    let hero = phase.get_hero("h1").unwrap();
    assert!(!hero.active_buffs.contains(&"burning".to_string()));
    assert!(hero.active_buffs.contains(&"poison".to_string()));
}

#[test]
fn remove_frozen_effect_applies() {
    let mut heroes = vec![make_hero("h1", "alchemist")];
    heroes[0].active_buffs = vec!["frozen".to_string(), "bleed".to_string()];
    let mut phase = CampingPhase::new(heroes);

    // Create skill with RemoveFrozen effect
    let skill = CampingSkill {
        id: "remove_frozen".to_string(),
        time_cost: 2,
        use_limit: 10,
        has_individual_target: true,
        classes: vec![],
        effects: vec![CampEffect {
            selection: CampTargetSelection::SelfTarget,
            requirements: Vec::new(),
            chance: 1.0,
            effect_type: CampEffectType::RemoveFrozen,
            sub_type: String::new(),
            amount: 0.0,
        }],
    };

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);

    // frozen should be removed, but bleed should remain
    let hero = phase.get_hero("h1").unwrap();
    assert!(!hero.active_buffs.contains(&"frozen".to_string()));
    assert!(hero.active_buffs.contains(&"bleed".to_string()));
}

#[test]
fn remove_disease_effect_applies() {
    let mut heroes = vec![make_hero("h1", "alchemist")];
    heroes[0].active_buffs = vec![
        "disease_foo".to_string(),
        "disease_bar".to_string(),
        "bleed".to_string(),
    ];
    let mut phase = CampingPhase::new(heroes);

    // Create skill with RemoveDisease effect
    let skill = CampingSkill {
        id: "remove_disease".to_string(),
        time_cost: 2,
        use_limit: 10,
        has_individual_target: true,
        classes: vec![],
        effects: vec![CampEffect {
            selection: CampTargetSelection::SelfTarget,
            requirements: Vec::new(),
            chance: 1.0,
            effect_type: CampEffectType::RemoveDisease,
            sub_type: String::new(),
            amount: 0.0,
        }],
    };

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);

    // All disease_* buffs should be removed, but bleed should remain
    let hero = phase.get_hero("h1").unwrap();
    assert!(!hero.active_buffs.iter().any(|b| b.starts_with("disease_")));
    assert!(hero.active_buffs.contains(&"bleed".to_string()));
}

#[test]
fn remove_debuff_removes_first_debuff() {
    let mut heroes = vec![make_hero("h1", "alchemist")];
    heroes[0].active_buffs = vec![
        "debuff_attack".to_string(),
        "debuff_defense".to_string(),
        "bleed".to_string(),
    ];
    let mut phase = CampingPhase::new(heroes);

    // Create skill with RemoveDebuff effect
    let skill = CampingSkill {
        id: "remove_debuff".to_string(),
        time_cost: 2,
        use_limit: 10,
        has_individual_target: true,
        classes: vec![],
        effects: vec![CampEffect {
            selection: CampTargetSelection::SelfTarget,
            requirements: Vec::new(),
            chance: 1.0,
            effect_type: CampEffectType::RemoveDebuff,
            sub_type: String::new(),
            amount: 0.0,
        }],
    };

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);

    // First debuff should be removed (debuff_attack), others remain
    let hero = phase.get_hero("h1").unwrap();
    assert!(!hero.active_buffs.contains(&"debuff_attack".to_string()));
    assert!(hero.active_buffs.contains(&"debuff_defense".to_string()));
    assert!(hero.active_buffs.contains(&"bleed".to_string()));
}

#[test]
fn remove_all_debuff_removes_all_debuffs() {
    let mut heroes = vec![make_hero("h1", "alchemist")];
    heroes[0].active_buffs = vec![
        "debuff_attack".to_string(),
        "debuff_defense".to_string(),
        "bleed".to_string(),
    ];
    let mut phase = CampingPhase::new(heroes);

    // Create skill with RemoveAllDebuff effect
    let skill = CampingSkill {
        id: "remove_all_debuff".to_string(),
        time_cost: 2,
        use_limit: 10,
        has_individual_target: true,
        classes: vec![],
        effects: vec![CampEffect {
            selection: CampTargetSelection::SelfTarget,
            requirements: Vec::new(),
            chance: 1.0,
            effect_type: CampEffectType::RemoveAllDebuff,
            sub_type: String::new(),
            amount: 0.0,
        }],
    };

    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);

    // All debuff_* buffs should be removed, but bleed should remain
    let hero = phase.get_hero("h1").unwrap();
    assert!(!hero.active_buffs.iter().any(|b| b.starts_with("debuff_")));
    assert!(hero.active_buffs.contains(&"bleed".to_string()));
}

// ── Chance-based effect tests ──────────────────────────────────────────────────

#[test]
fn chance_effect_triggers_when_roll_is_below_chance() {
    let mut heroes = vec![make_hero("h1", "alchemist")];
    heroes[0].health = 50.0;
    heroes[0].max_health = 100.0;
    let mut phase = CampingPhase::new(heroes);

    // Create skill with 75% chance health heal
    let skill = CampingSkill {
        id: "chance_heal".to_string(),
        time_cost: 2,
        use_limit: 10,
        has_individual_target: true,
        classes: vec![],
        effects: vec![CampEffect {
            selection: CampTargetSelection::SelfTarget,
            requirements: Vec::new(),
            chance: 0.75,
            effect_type: CampEffectType::HealthHealAmount,
            sub_type: String::new(),
            amount: 30.0,
        }],
    };

    // deterministic_chance_roll is deterministic, so we can test the result
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);

    // The effect should have been applied (the roll is deterministic based on inputs)
    // Check that the record shows the effect was processed
    assert_eq!(result.record.effects_applied.len(), 1);
}

#[test]
fn deterministic_chance_roll_produces_consistent_results() {
    // The deterministic_chance_roll function uses hash-based pseudo-random
    // so the same inputs always produce the same output
    let skill_id = "test_skill";
    let performer_id = "h1";
    let target_id = Some("h1");

    // Create two identical phases
    let heroes = vec![make_hero("h1", "alchemist")];
    let mut phase1 = CampingPhase::new(heroes);
    let heroes2 = vec![make_hero("h1", "alchemist")];
    let mut phase2 = CampingPhase::new(heroes2);

    let skill = CampingSkill {
        id: skill_id.to_string(),
        time_cost: 2,
        use_limit: 10,
        has_individual_target: true,
        classes: vec![],
        effects: vec![CampEffect {
            selection: CampTargetSelection::SelfTarget,
            requirements: Vec::new(),
            chance: 0.5,
            effect_type: CampEffectType::HealthHealAmount,
            sub_type: String::new(),
            amount: 30.0,
        }],
    };

    let result1 = perform_camping_skill(&mut phase1, &skill, performer_id, target_id);
    let result2 = perform_camping_skill(&mut phase2, &skill, performer_id, target_id);

    // Both should have the same trigger outcome
    assert_eq!(
        result1.record.effects_applied[0].triggered,
        result2.record.effects_applied[0].triggered
    );
}

// ── Representative skill tests (first_aid, pep_talk) ─────────────────────────

/// Helper to create a CampingSkill that mirrors the first_aid skill from JsonCamping.json.
/// first_aid: health_heal_max_health_percent (15%), remove_bleeding, remove_poison
fn make_first_aid_skill() -> CampingSkill {
    CampingSkill {
        id: "first_aid".to_string(),
        time_cost: 2,
        use_limit: 1,
        has_individual_target: true,
        classes: vec![
            "bounty_hunter".to_string(),
            "crusader".to_string(),
            "vestal".to_string(),
            "occultist".to_string(),
            "hellion".to_string(),
            "grave_robber".to_string(),
            "highwayman".to_string(),
            "plague_doctor".to_string(),
            "jester".to_string(),
            "leper".to_string(),
            "arbalest".to_string(),
            "man_at_arms".to_string(),
            "houndmaster".to_string(),
            "abomination".to_string(),
            "antiquarian".to_string(),
            "musketeer".to_string(),
        ],
        effects: vec![
            // Effect 0: 15% max HP heal
            CampEffect {
                selection: CampTargetSelection::Individual,
                requirements: Vec::new(),
                chance: 1.0,
                effect_type: CampEffectType::HealthHealMaxHealthPercent,
                sub_type: String::new(),
                amount: 0.15,
            },
            // Effect 1: remove bleeding
            CampEffect {
                selection: CampTargetSelection::Individual,
                requirements: Vec::new(),
                chance: 1.0,
                effect_type: CampEffectType::RemoveBleed,
                sub_type: String::new(),
                amount: 0.0,
            },
            // Effect 2: remove poison
            CampEffect {
                selection: CampTargetSelection::Individual,
                requirements: Vec::new(),
                chance: 1.0,
                effect_type: CampEffectType::RemovePoison,
                sub_type: String::new(),
                amount: 0.0,
            },
        ],
    }
}

#[test]
fn first_aid_heals_15_percent_max_health() {
    let mut heroes = vec![make_hero("h1", "vestal")];
    heroes[0].health = 50.0;
    heroes[0].max_health = 100.0;
    let mut phase = CampingPhase::new(heroes);

    let skill = make_first_aid_skill();
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);

    // 15% of 100 max health = 15 HP heal
    // 50 + 15 = 65
    let hero = phase.get_hero("h1").unwrap();
    assert_eq!(hero.health, 65.0);
}

#[test]
fn first_aid_removes_bleed_and_poison() {
    let mut heroes = vec![make_hero("h1", "vestal")];
    heroes[0].health = 100.0;
    heroes[0].active_buffs = vec![
        "bleed".to_string(),
        "poison".to_string(),
        "other_buff".to_string(),
    ];
    let mut phase = CampingPhase::new(heroes);

    let skill = make_first_aid_skill();
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);

    let hero = phase.get_hero("h1").unwrap();
    // bleed and poison should be removed
    assert!(!hero.active_buffs.contains(&"bleed".to_string()));
    assert!(!hero.active_buffs.contains(&"poison".to_string()));
    // other_buff should remain
    assert!(hero.active_buffs.contains(&"other_buff".to_string()));
}

#[test]
fn first_aid_skill_has_correct_effects() {
    let skill = make_first_aid_skill();

    // Verify first_aid has the correct effects structure
    assert_eq!(skill.effects.len(), 3);
    assert_eq!(skill.effects[0].effect_type, CampEffectType::HealthHealMaxHealthPercent);
    assert!((skill.effects[0].amount - 0.15).abs() < f64::EPSILON);
    assert_eq!(skill.effects[1].effect_type, CampEffectType::RemoveBleed);
    assert_eq!(skill.effects[2].effect_type, CampEffectType::RemovePoison);
}

/// Helper to create a CampingSkill that mirrors the pep_talk skill from JsonCamping.json.
/// pep_talk: Buff (campingStressResistBuff)
fn make_pep_talk_skill() -> CampingSkill {
    CampingSkill {
        id: "pep_talk".to_string(),
        time_cost: 2,
        use_limit: 1,
        has_individual_target: true,
        classes: vec![
            "bounty_hunter".to_string(),
            "crusader".to_string(),
            "vestal".to_string(),
            "occultist".to_string(),
            "hellion".to_string(),
            "grave_robber".to_string(),
            "highwayman".to_string(),
            "plague_doctor".to_string(),
            "jester".to_string(),
            "leper".to_string(),
            "arbalest".to_string(),
            "man_at_arms".to_string(),
            "houndmaster".to_string(),
            "abomination".to_string(),
            "antiquarian".to_string(),
            "musketeer".to_string(),
        ],
        effects: vec![CampEffect {
            selection: CampTargetSelection::SelfTarget,
            requirements: Vec::new(),
            chance: 1.0,
            effect_type: CampEffectType::Buff,
            sub_type: "campingStressResistBuff".to_string(),
            amount: 0.0,
        }],
    }
}

#[test]
fn pep_talk_applies_stress_resist_buff() {
    let mut heroes = vec![make_hero("h1", "jester")];
    heroes[0].stress = 50.0;
    let mut phase = CampingPhase::new(heroes);

    let skill = make_pep_talk_skill();
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);

    let hero = phase.get_hero("h1").unwrap();
    assert!(hero.active_buffs.contains(&"campingStressResistBuff".to_string()));
}

#[test]
fn pep_talk_skill_has_correct_effects() {
    let skill = make_pep_talk_skill();

    // Verify pep_talk has the correct effects structure
    assert_eq!(skill.effects.len(), 1);
    assert_eq!(skill.effects[0].effect_type, CampEffectType::Buff);
    assert_eq!(skill.effects[0].sub_type, "campingStressResistBuff");
}

#[test]
fn pep_talk_does_not_affect_health_or_stress() {
    let mut heroes = vec![make_hero("h1", "jester")];
    heroes[0].health = 80.0;
    heroes[0].stress = 60.0;
    let mut phase = CampingPhase::new(heroes);

    let skill = make_pep_talk_skill();
    let result = perform_camping_skill(&mut phase, &skill, "h1", Some("h1"));
    assert!(result.success);

    // pep_talk only applies a buff, doesn't change health or stress
    let hero = phase.get_hero("h1").unwrap();
    assert_eq!(hero.health, 80.0);
    assert_eq!(hero.stress, 60.0);
}