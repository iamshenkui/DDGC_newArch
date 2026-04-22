//! Integration test for trait registry (US-019).
//!
//! Validates:
//! - TraitRegistry holds all DDGC trait definitions parsed from JsonTraits.json
//! - At least 2 afflictions (fearful, hopeless) and 1 virtue (courageous) are parsed
//! - Focused test proves trait lookup by ID works
//! - Focused test proves act-out weight tables are preserved
//! - Focused test proves reaction probabilities are preserved

use game_ddgc_headless::contracts::{
    parse::parse_traits_json, ActOutAction, OverstressType, ReactionEffect, ReactionTrigger,
    TraitRegistry,
};

fn data_path(filename: &str) -> std::path::PathBuf {
    std::path::PathBuf::from("data").join(filename)
}

fn parse_all() -> TraitRegistry {
    parse_traits_json(&data_path("JsonTraits.json"))
        .expect("failed to parse JsonTraits.json")
}

// ── US-019: Registry lookup by ID ───────────────────────────────────────────

#[test]
fn trait_registry_lookup_by_id_works() {
    let traits = parse_all();

    // Verify specific trait IDs exist
    assert!(traits.get("fearful").is_some(), "fearful should exist");
    assert!(traits.get("hopeless").is_some(), "hopeless should exist");
    assert!(traits.get("courageous").is_some(), "courageous should exist");
}

#[test]
fn trait_registry_returns_none_for_unknown_id() {
    let traits = parse_all();

    assert!(traits.get("nonexistent_trait").is_none(), "unknown trait should return None");
}

// ── US-019: Affliction/Virtue classification ────────────────────────────────

#[test]
fn trait_registry_has_at_least_2_afflictions() {
    let traits = parse_all();

    let afflictions = traits.afflictions();
    assert!(
        afflictions.len() >= 2,
        "At least 2 afflictions should exist, got {}",
        afflictions.len()
    );

    // All returned traits should be afflictions
    for trait_def in &afflictions {
        assert_eq!(
            trait_def.overstress_type,
            OverstressType::Affliction,
            "Trait {} should be an affliction",
            trait_def.id
        );
    }
}

#[test]
fn trait_registry_has_at_least_1_virtue() {
    let traits = parse_all();

    let virtues = traits.virtues();
    assert!(
        !virtues.is_empty(),
        "At least 1 virtue should exist, got {}",
        virtues.len()
    );

    // All returned traits should be virtues
    for trait_def in &virtues {
        assert_eq!(
            trait_def.overstress_type,
            OverstressType::Virtue,
            "Trait {} should be a virtue",
            trait_def.id
        );
    }
}

#[test]
fn trait_overstress_type_by_id() {
    let traits = parse_all();

    // fearful is an affliction
    let fearful = traits.get("fearful").unwrap();
    assert_eq!(fearful.overstress_type, OverstressType::Affliction);

    // hopeless is an affliction
    let hopeless = traits.get("hopeless").unwrap();
    assert_eq!(hopeless.overstress_type, OverstressType::Affliction);

    // courageous is a virtue
    let courageous = traits.get("courageous").unwrap();
    assert_eq!(courageous.overstress_type, OverstressType::Virtue);
}

// ── US-019: Act-out weight tables ──────────────────────────────────────────

#[test]
fn trait_act_out_weight_tables_preserved_fearful() {
    let traits = parse_all();

    let fearful = traits.get("fearful").unwrap();
    let act_outs = &fearful.combat_start_turn_act_outs;

    // fearful has 4 act-outs: nothing(40), bark_stress(30), change_pos(20), ignore_command(10)
    assert_eq!(act_outs.len(), 4);

    let nothing_weight = act_outs
        .iter()
        .find(|a| a.action == ActOutAction::Nothing)
        .map(|a| a.weight);
    assert_eq!(nothing_weight, Some(40), "fearful nothing weight should be 40");

    let bark_stress_weight = act_outs
        .iter()
        .find(|a| a.action == ActOutAction::BarkStress)
        .map(|a| a.weight);
    assert_eq!(bark_stress_weight, Some(30), "fearful bark_stress weight should be 30");

    let change_pos_weight = act_outs
        .iter()
        .find(|a| a.action == ActOutAction::ChangePos)
        .map(|a| a.weight);
    assert_eq!(change_pos_weight, Some(20), "fearful change_pos weight should be 20");

    let ignore_command_weight = act_outs
        .iter()
        .find(|a| a.action == ActOutAction::IgnoreCommand)
        .map(|a| a.weight);
    assert_eq!(ignore_command_weight, Some(10), "fearful ignore_command weight should be 10");
}

#[test]
fn trait_act_out_weight_tables_preserved_hopeless() {
    let traits = parse_all();

    let hopeless = traits.get("hopeless").unwrap();
    let act_outs = &hopeless.combat_start_turn_act_outs;

    // hopeless has 4 act-outs: nothing(50), bark_stress(35), change_pos(10), ignore_command(5)
    assert_eq!(act_outs.len(), 4);

    let nothing_weight = act_outs
        .iter()
        .find(|a| a.action == ActOutAction::Nothing)
        .map(|a| a.weight);
    assert_eq!(nothing_weight, Some(50), "hopeless nothing weight should be 50");

    let bark_stress_weight = act_outs
        .iter()
        .find(|a| a.action == ActOutAction::BarkStress)
        .map(|a| a.weight);
    assert_eq!(bark_stress_weight, Some(35), "hopeless bark_stress weight should be 35");
}

#[test]
fn trait_act_out_weight_tables_preserved_courageous() {
    let traits = parse_all();

    let courageous = traits.get("courageous").unwrap();
    let act_outs = &courageous.combat_start_turn_act_outs;

    // courageous has 4 act-outs: nothing(20), bark_stress(60), change_pos(15), ignore_command(5)
    assert_eq!(act_outs.len(), 4);

    let bark_stress_weight = act_outs
        .iter()
        .find(|a| a.action == ActOutAction::BarkStress)
        .map(|a| a.weight);
    assert_eq!(bark_stress_weight, Some(60), "courageous bark_stress weight should be 60");
}

// ── US-019: Reaction probabilities ─────────────────────────────────────────

#[test]
fn trait_reaction_probabilities_preserved_fearful() {
    let traits = parse_all();

    let fearful = traits.get("fearful").unwrap();
    let reactions = &fearful.reaction_act_outs;

    assert_eq!(reactions.len(), 2);

    // ally_hit -> flee (probability 0.3)
    let ally_hit = reactions
        .iter()
        .find(|r| r.trigger == ReactionTrigger::AllyHit);
    assert!(ally_hit.is_some(), "fearful should have ally_hit reaction");
    let ally_hit = ally_hit.unwrap();
    assert_eq!(ally_hit.probability, 0.3);
    assert_eq!(ally_hit.effect, ReactionEffect::Flee);

    // ally_killed -> panic (probability 0.6)
    let ally_killed = reactions
        .iter()
        .find(|r| r.trigger == ReactionTrigger::AllyKilled);
    assert!(ally_killed.is_some(), "fearful should have ally_killed reaction");
    let ally_killed = ally_killed.unwrap();
    assert_eq!(ally_killed.probability, 0.6);
    assert_eq!(ally_killed.effect, ReactionEffect::Panic);
}

#[test]
fn trait_reaction_probabilities_preserved_hopeless() {
    let traits = parse_all();

    let hopeless = traits.get("hopeless").unwrap();
    let reactions = &hopeless.reaction_act_outs;

    assert_eq!(reactions.len(), 2);

    // ally_hit -> despair (probability 0.5)
    let ally_hit = reactions
        .iter()
        .find(|r| r.trigger == ReactionTrigger::AllyHit);
    assert!(ally_hit.is_some(), "hopeless should have ally_hit reaction");
    let ally_hit = ally_hit.unwrap();
    assert_eq!(ally_hit.probability, 0.5);
    assert_eq!(ally_hit.effect, ReactionEffect::Despair);

    // enemy_killed -> motivate (probability 0.2)
    let enemy_killed = reactions
        .iter()
        .find(|r| r.trigger == ReactionTrigger::EnemyKilled);
    assert!(enemy_killed.is_some(), "hopeless should have enemy_killed reaction");
    let enemy_killed = enemy_killed.unwrap();
    assert_eq!(enemy_killed.probability, 0.2);
    assert_eq!(enemy_killed.effect, ReactionEffect::Motivate);
}

#[test]
fn trait_reaction_probabilities_preserved_courageous() {
    let traits = parse_all();

    let courageous = traits.get("courageous").unwrap();
    let reactions = &courageous.reaction_act_outs;

    assert_eq!(reactions.len(), 2);

    // ally_hit -> rally (probability 0.7)
    let ally_hit = reactions
        .iter()
        .find(|r| r.trigger == ReactionTrigger::AllyHit);
    assert!(ally_hit.is_some(), "courageous should have ally_hit reaction");
    let ally_hit = ally_hit.unwrap();
    assert_eq!(ally_hit.probability, 0.7);
    assert_eq!(ally_hit.effect, ReactionEffect::Rally);

    // ally_stressed -> calm (probability 0.4)
    let ally_stressed = reactions
        .iter()
        .find(|r| r.trigger == ReactionTrigger::AllyStressed);
    assert!(ally_stressed.is_some(), "courageous should have ally_stressed reaction");
    let ally_stressed = ally_stressed.unwrap();
    assert_eq!(ally_stressed.probability, 0.4);
    assert_eq!(ally_stressed.effect, ReactionEffect::Calm);
}

// ── US-019: Buff IDs ────────────────────────────────────────────────────────

#[test]
fn trait_buff_ids_preserved() {
    let traits = parse_all();

    let fearful = traits.get("fearful").unwrap();
    assert!(fearful.buff_ids.contains(&"SPD-2".to_string()));
    assert!(fearful.buff_ids.contains(&"DODGE-3".to_string()));
    assert!(fearful.buff_ids.contains(&"ACC-5".to_string()));

    let courageous = traits.get("courageous").unwrap();
    assert!(courageous.buff_ids.contains(&"ATK+5".to_string()));
    assert!(courageous.buff_ids.contains(&"DEF+3".to_string()));
    assert!(courageous.buff_ids.contains(&"STRESSRES+15".to_string()));
}

// ── US-019: Registry size ───────────────────────────────────────────────────

#[test]
fn trait_registry_has_at_least_3_traits() {
    let traits = parse_all();
    assert!(
        traits.len() >= 3,
        "At least 3 traits should be parsed, got {}",
        traits.len()
    );
}