//! DDGC targeting rules — game-layer targeting overrides for DDGC-specific skill semantics.
//!
//! The framework's `TargetSelector` provides `AllEnemies` and `AllAllies` as safe defaults
//! for DDGC skill targeting, but these flatten DDGC's more expressive targeting semantics:
//!
//! - Single-target: DDGC skills marked as single-target should only affect ONE enemy/ally
//!   (e.g., `mark_skill`, `stun`), not all of them
//! - Ally-exclusive: DDGC `@1234` notation means "any ally rank, not self", but `AllAllies`
//!   includes self — need to exclude self for ally-exclusive skills
//!
//! This module provides DDGC-specific targeting rules that post-process framework target
//! resolution results, applying single-target truncation and self-exclusion filters.

use crate::encounters::targeting::{LaunchConstraint, SideAffinity, TargetRank};
use framework_combat::encounter::CombatSide;
use framework_combat::formation::FormationLayout;
use framework_rules::actor::ActorId;

/// DDGC targeting rule for a specific skill.
#[derive(Debug, Clone)]
pub struct DdgcTargetingRule {
    /// Launch constraint for the skill.
    pub launch_constraint: LaunchConstraint,
    /// Target rank restriction.
    pub target_rank: TargetRank,
    /// Side affinity for valid targets.
    pub side_affinity: SideAffinity,
    /// Whether this skill targets a single actor.
    pub single_target: bool,
    /// Whether to exclude self from ally targets (DDGC `@rank` = ally, not self).
    pub exclude_self_from_allies: bool,
}

/// Helper to create a launch constraint from DDGC rank range notation.
///
/// DDGC ranks are 1-indexed and relative to the actor's side (ally/enemy).
/// Rank 1-2 = front row, rank 3-4 = back row.
///
/// `launch_ranks` is a string like "12" (ranks 1-2), "34" (ranks 3-4), or "1234" (any).
pub fn launch_constraint_from_ddgc(launch_ranks: &str) -> LaunchConstraint {
    match launch_ranks {
        "12" => LaunchConstraint::FrontRow,
        "34" => LaunchConstraint::BackRow,
        "1234" => LaunchConstraint::Any,
        _ => LaunchConstraint::Any,
    }
}

/// Helper to create a target rank from DDGC rank notation.
///
/// DDGC target notation:
/// - "1234" = any rank (FrontAndBack)
/// - "12" or "~12" = front 2 ranks (Front)
/// - "34" = back 2 ranks (Back)
pub fn target_rank_from_ddgc(target_notation: &str) -> TargetRank {
    match target_notation {
        "1234" => TargetRank::FrontAndBack,
        "12" | "~12" | "~1234" => TargetRank::Front,
        "34" => TargetRank::Back,
        _ => TargetRank::Any,
    }
}

/// Check if an actor at the given slot satisfies a DDGC rank-based launch constraint.
///
/// DDGC ranks are 1-indexed and relative to the actor's side:
/// - Rank 1-2 = front row (slots 0 to slots_per_lane-1 within the actor's lane)
/// - Rank 3-4 = back row (slots slots_per_lane to 2*slots_per_lane-1 within the actor's lane)
pub fn check_launch_rank_constraint(
    constraint: LaunchConstraint,
    actor: ActorId,
    formation: &FormationLayout,
    _side_lookup: &std::collections::HashMap<ActorId, CombatSide>,
) -> bool {
    let Some(slot) = formation.find_actor(actor) else {
        return false;
    };

    // Compute the actor's rank within their lane
    // Allies are in lane 0, enemies are in lane 1
    // Rank is slot position within the lane (0-indexed), +1 for 1-indexed
    let slot_in_lane = slot.0 % formation.slots_per_lane;

    match constraint {
        LaunchConstraint::Any => true,
        LaunchConstraint::FrontRow => slot_in_lane < formation.slots_per_lane / 2,
        LaunchConstraint::BackRow => slot_in_lane >= formation.slots_per_lane / 2,
        LaunchConstraint::SpecificLane(lane) => formation
            .slots
            .get(&slot)
            .is_some_and(|s| s.lane == lane),
        LaunchConstraint::SlotRange { min, max } => {
            // Convert slot range (absolute) to rank-based: rank = slot_in_lane + 1
            // Rank 1 = slot 0 in lane, Rank 2 = slot 1, etc.
            let rank = slot_in_lane + 1;
            rank >= min && rank <= max
        }
    }
}

/// Filter targets by DDGC target rank constraint.
///
/// Given a list of candidate targets, filters to only those whose slot
/// satisfies the target rank restriction.
pub fn filter_targets_by_rank(
    rank_constraint: TargetRank,
    targets: &[ActorId],
    formation: &FormationLayout,
) -> Vec<ActorId> {
    if matches!(rank_constraint, TargetRank::Any | TargetRank::FrontAndBack) {
        return targets.to_vec();
    }

    let slots_per_lane = formation.slots_per_lane;

    targets
        .iter()
        .filter(|&&target_id| {
            let Some(slot) = formation.find_actor(target_id) else {
                return false;
            };
            let slot_in_lane = slot.0 % slots_per_lane;

            match rank_constraint {
                TargetRank::Front => slot_in_lane < slots_per_lane / 2,
                TargetRank::Back => slot_in_lane >= slots_per_lane / 2,
                _ => true,
            }
        })
        .copied()
        .collect()
}

impl DdgcTargetingRule {
    /// Create a single-target enemy rule (no launch constraint, any rank).
    pub fn single_target_enemy() -> Self {
        DdgcTargetingRule {
            launch_constraint: LaunchConstraint::Any,
            target_rank: TargetRank::Any,
            side_affinity: SideAffinity::Enemy,
            single_target: true,
            exclude_self_from_allies: false,
        }
    }

    /// Create a single-target ally rule that excludes self (DDGC `@rank` semantics).
    pub fn single_target_ally_excluding_self() -> Self {
        DdgcTargetingRule {
            launch_constraint: LaunchConstraint::Any,
            target_rank: TargetRank::Any,
            side_affinity: SideAffinity::Ally,
            single_target: true,
            exclude_self_from_allies: true,
        }
    }

    /// Create a multi-target enemy rule (no single-target restriction).
    pub fn multi_target_enemy() -> Self {
        DdgcTargetingRule {
            launch_constraint: LaunchConstraint::Any,
            target_rank: TargetRank::Any,
            side_affinity: SideAffinity::Enemy,
            single_target: false,
            exclude_self_from_allies: false,
        }
    }

    /// Create a rule for a self-only skill.
    pub fn self_only() -> Self {
        DdgcTargetingRule {
            launch_constraint: LaunchConstraint::Any,
            target_rank: TargetRank::Any,
            side_affinity: SideAffinity::Ally,
            single_target: true,
            exclude_self_from_allies: false, // Self is the only target anyway
        }
    }

    /// Create a rule with explicit launch constraint and target rank (for rank-gated skills).
    pub fn with_rank_constraint(
        launch_constraint: LaunchConstraint,
        target_rank: TargetRank,
        side_affinity: SideAffinity,
        single_target: bool,
    ) -> Self {
        DdgcTargetingRule {
            launch_constraint,
            target_rank,
            side_affinity,
            single_target,
            exclude_self_from_allies: false,
        }
    }
}

/// Map of skill names to their DDGC targeting rules.
///
/// Only skills with non-standard DDGC targeting semantics are included here.
/// Skills not in this map use the framework's default `TargetSelector` behavior.
///
/// Hero skill examples:
/// - `mark_skill` (Hunter): single-target enemy — DDGC marks ONE target, not all
/// - `protect_skill` (Tank): ally-exclusive single-target — `@1234` means any ally, not self
///
/// Monster skill examples:
/// - `stun` (lizard): single-target enemy — DDGC stuns ONE target per use
/// - `intimidate` (lizard): AoE stress, multi-target — kept as multi-target (game-gap)
pub fn ddgc_targeting_rules() -> impl IntoIterator<Item = (&'static str, DdgcTargetingRule)> {
    vec![
        // ── Hero skills ────────────────────────────────────────────────────────
        // mark_skill: single-target enemy (marks one target, not all enemies)
        ("mark_skill", DdgcTargetingRule::single_target_enemy()),
        // protect_skill: ally-exclusive single-target (DDGC @1234 = any ally, not self)
        ("protect_skill", DdgcTargetingRule::single_target_ally_excluding_self()),
        // buff_skill: self + allies, but buff_skill in Hunter is self-only intent
        ("buff_skill", DdgcTargetingRule::self_only()),
        // active_riposte: single-target enemy (marks one enemy, not all)
        ("active_riposte", DdgcTargetingRule::single_target_enemy()),
        // taunt_skill: single-target enemy (taunts one enemy)
        ("taunt_skill", DdgcTargetingRule::single_target_enemy()),
        // attack_reduce: single-target enemy (debuffs one enemy)
        ("attack_reduce", DdgcTargetingRule::single_target_enemy()),
        // regression: single-target enemy (targets rearmost enemy)
        ("regression", DdgcTargetingRule::single_target_enemy()),
        // blood_oath: single-target enemy (bonds with one enemy)
        ("blood_oath", DdgcTargetingRule::single_target_enemy()),
        // direct_hit_1: single-target enemy (hits one target)
        ("direct_hit_1", DdgcTargetingRule::single_target_enemy()),
        // duality_fate: single-target enemy (fate links one target)
        ("duality_fate", DdgcTargetingRule::single_target_enemy()),
        // opening_strike: single-target enemy
        ("opening_strike", DdgcTargetingRule::single_target_enemy()),
        // desperate_strike: single-target enemy
        ("desperate_strike", DdgcTargetingRule::single_target_enemy()),
        // ignore_def_skill: single-target enemy (Hunter's ignore def targets one)
        ("ignore_def_skill", DdgcTargetingRule::single_target_enemy()),
        // bleed_skill: single-target enemy (applies to one target)
        ("bleed_skill", DdgcTargetingRule::single_target_enemy()),
        // pull_skill: single-target enemy (pulls one target)
        ("pull_skill", DdgcTargetingRule::single_target_enemy()),
        // aoe_skill: multi-target enemy (AoE, intentionally kept multi-target)
        // ("aoe_skill", DdgcTargetingRule::multi_target_enemy()),
        // stun_skill: single-target enemy (guaranteed stun on one target)
        ("stun_skill", DdgcTargetingRule::single_target_enemy()),
        // crusading_strike: single-target enemy (legacy hero skill)
        ("crusading_strike", DdgcTargetingRule::single_target_enemy()),
        // holy_lance: single-target enemy (legacy hero skill)
        ("holy_lance", DdgcTargetingRule::single_target_enemy()),
        // divine_grace: single-target ally (heals one ally)
        ("divine_grace", DdgcTargetingRule::single_target_ally_excluding_self()),
        // rend: single-target enemy
        ("rend", DdgcTargetingRule::single_target_enemy()),
        // skull_bash: single-target enemy
        ("skull_bash", DdgcTargetingRule::single_target_enemy()),
        // grave_bash: single-target enemy
        ("grave_bash", DdgcTargetingRule::single_target_enemy()),
        // alchemist_damage_skill: single-target enemy
        ("alchemist_damage_skill", DdgcTargetingRule::single_target_enemy()),
        // alchemist_heal_skill: single-target ally
        ("alchemist_heal_skill", DdgcTargetingRule::single_target_ally_excluding_self()),
        // black_direct: single-target enemy
        ("black_direct", DdgcTargetingRule::single_target_enemy()),
        // black_aoe: multi-target enemy (AoE)
        // white_support: ally-exclusive single-target
        ("white_support", DdgcTargetingRule::single_target_ally_excluding_self()),
        // white_heal: single-target ally
        ("white_heal", DdgcTargetingRule::single_target_ally_excluding_self()),
        // white_buff: self-only
        ("white_buff", DdgcTargetingRule::self_only()),
        // diviner_attack: single-target enemy
        ("diviner_attack", DdgcTargetingRule::single_target_enemy()),
        // diviner_debuff: single-target enemy
        ("diviner_debuff", DdgcTargetingRule::single_target_enemy()),
        // barrier_skill: single-target ally
        ("barrier_skill", DdgcTargetingRule::single_target_ally_excluding_self()),
        // shield_wall: multi-target ally (protects all)
        // ("shield_wall", DdgcTargetingRule::multi_target_ally()),
        // shrapnel: multi-target enemy (AoE)
        // ("shrapnel", DdgcTargetingRule::multi_target_enemy()),
        // entangle: single-target enemy
        ("entangle", DdgcTargetingRule::single_target_enemy()),
        // hex: single-target enemy
        ("hex", DdgcTargetingRule::single_target_enemy()),
        // mystic_blast: single-target enemy
        ("mystic_blast", DdgcTargetingRule::single_target_enemy()),
        // shaman_damage: single-target enemy
        ("shaman_damage", DdgcTargetingRule::single_target_enemy()),
        // shaman_heal: single-target ally
        ("shaman_heal", DdgcTargetingRule::single_target_ally_excluding_self()),
        // purge: single-target enemy (cleanse-like effect)
        ("purge", DdgcTargetingRule::single_target_enemy()),
        // ── Monster skills ─────────────────────────────────────────────────────
        // lizard stun: single-target enemy
        ("stun", DdgcTargetingRule::single_target_enemy()),
        // lizard stress: single-target enemy
        ("stress", DdgcTargetingRule::single_target_enemy()),
        // lizard intimidate: multi-target (AoE, keep as-is)
        // alligator_yangtze mark_riposte: single-target enemy (marks one)
        ("mark_riposte", DdgcTargetingRule::single_target_enemy()),
        // tiger_sword pull: single-target enemy
        ("pull", DdgcTargetingRule::single_target_enemy()),
        // dry_tree_genie bleed: single-target enemy
        ("bleed", DdgcTargetingRule::single_target_enemy()),
        // dry_tree_genie slow_crowd: multi-target (AoE, keep as-is)
        // dry_tree_genie stress: single-target enemy
        ("stress", DdgcTargetingRule::single_target_enemy()),
        // metal_armor stun: single-target enemy
        ("stun", DdgcTargetingRule::single_target_enemy()),
        // metal_armor bleed: single-target enemy
        ("bleed", DdgcTargetingRule::single_target_enemy()),
        // moth_mimicry_A normal_attack: single-target enemy
        ("normal_attack", DdgcTargetingRule::single_target_enemy()),
        // mantis_magic_flower poison: single-target enemy
        // DDGC: launch ranks 1-2 (front row), target ranks 1-4 (any rank)
        ("poison", DdgcTargetingRule::with_rank_constraint(
            LaunchConstraint::FrontRow,
            TargetRank::FrontAndBack,
            SideAffinity::Enemy,
            true,
        )),
        // mantis_magic_flower crowd_bleed: multi-target enemy
        // DDGC: launch ranks 1-2 (front row), target ~12 (front 2 ranks only)
        ("crowd_bleed", DdgcTargetingRule::with_rank_constraint(
            LaunchConstraint::FrontRow,
            TargetRank::Front,
            SideAffinity::Enemy,
            false,
        )),
        // mantis_spiny_flower ignore_armor: single-target enemy
        ("ignore_armor", DdgcTargetingRule::single_target_enemy()),
        // robber_melee normal_attack: single-target enemy
        ("normal_attack", DdgcTargetingRule::single_target_enemy()),
        // robber_melee bleed: single-target enemy
        ("bleed", DdgcTargetingRule::single_target_enemy()),
        // robber_ranged throw_stone: single-target enemy
        ("throw_stone", DdgcTargetingRule::single_target_enemy()),
        // ghost_fire_assist assist: ally-exclusive multi-target (not single-target)
        // (kept multi-target since DDGC assist targets all allies)
        // ghost_fire_damage stress: single-target enemy
        ("stress", DdgcTargetingRule::single_target_enemy()),
        // ghost_fire_damage burn_attack: single-target enemy
        ("burn_attack", DdgcTargetingRule::single_target_enemy()),
        // fox_fire bite: single-target enemy
        ("bite", DdgcTargetingRule::single_target_enemy()),
        // fox_fire vomit: multi-target (AoE, keep as-is)
        // lantern stress: single-target enemy
        ("stress", DdgcTargetingRule::single_target_enemy()),
        // lantern burn_attack: single-target enemy
        ("burn_attack", DdgcTargetingRule::single_target_enemy()),
        // snake_water stun: single-target enemy
        ("stun", DdgcTargetingRule::single_target_enemy()),
        // snake_water poison_fang: single-target enemy
        ("poison_fang", DdgcTargetingRule::single_target_enemy()),
        // water_grass stun: single-target enemy
        ("stun", DdgcTargetingRule::single_target_enemy()),
        // water_grass puncture: single-target enemy
        ("puncture", DdgcTargetingRule::single_target_enemy()),
        // water_grass convolve: single-target enemy (pull)
        ("convolve", DdgcTargetingRule::single_target_enemy()),
        // monkey_water base_melee: single-target enemy
        ("base_melee", DdgcTargetingRule::single_target_enemy()),
        // monkey_water rush: single-target enemy
        ("rush", DdgcTargetingRule::single_target_enemy()),
        // monkey_water stress: single-target enemy
        ("stress", DdgcTargetingRule::single_target_enemy()),
        // frostvein_clam glacial_torrent: single-target enemy
        ("glacial_torrent", DdgcTargetingRule::single_target_enemy()),
        // frostvein_clam abyssal_glare: single-target enemy
        ("abyssal_glare", DdgcTargetingRule::single_target_enemy()),
        // frostvein_clam prismatic_clench: single-target enemy
        ("prismatic_clench", DdgcTargetingRule::single_target_enemy()),
        // frostvein_clam clam_riposte: single-target enemy (counter)
        ("clam_riposte", DdgcTargetingRule::single_target_enemy()),
        // scarlet_guillotine: single-target enemy
        ("scarlet_guillotine", DdgcTargetingRule::single_target_enemy()),
        // phantom_lunge: single-target enemy
        ("phantom_lunge", DdgcTargetingRule::single_target_enemy()),
        // bloodstrike_ambush: single-target enemy
        ("bloodstrike_ambush", DdgcTargetingRule::single_target_enemy()),
        // flesh_usury_contract: single-target enemy (controller debuff)
        ("flesh_usury_contract", DdgcTargetingRule::single_target_enemy()),
        // compound_agony: single-target enemy
        ("compound_agony", DdgcTargetingRule::single_target_enemy()),
        // invitation: single-target enemy
        ("invitation", DdgcTargetingRule::single_target_enemy()),
        // foreclosed_wail: multi-target (AoE, keep as-is)
    ]
}

/// Look up a DDGC targeting rule by skill name.
///
/// Returns `None` if the skill has no DDGC-specific targeting rule and should
/// use the framework's default `TargetSelector` behavior.
pub fn get_ddgc_targeting_rule(skill_name: &str) -> Option<DdgcTargetingRule> {
    ddgc_targeting_rules()
        .into_iter()
        .find(|(name, _)| *name == skill_name)
        .map(|(_, rule)| rule)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mark_skill_has_single_target_enemy_rule() {
        let rule = get_ddgc_targeting_rule("mark_skill");
        assert!(rule.is_some());
        let rule = rule.unwrap();
        assert!(rule.single_target);
        assert!(matches!(rule.side_affinity, SideAffinity::Enemy));
    }

    #[test]
    fn protect_skill_has_ally_excluding_self_rule() {
        let rule = get_ddgc_targeting_rule("protect_skill");
        assert!(rule.is_some());
        let rule = rule.unwrap();
        assert!(rule.single_target);
        assert!(rule.exclude_self_from_allies);
        assert!(matches!(rule.side_affinity, SideAffinity::Ally));
    }

    #[test]
    fn lizard_stun_has_single_target_enemy_rule() {
        let rule = get_ddgc_targeting_rule("stun");
        assert!(rule.is_some());
        let rule = rule.unwrap();
        assert!(rule.single_target);
        assert!(matches!(rule.side_affinity, SideAffinity::Enemy));
    }

    #[test]
    fn buff_skill_is_self_only() {
        let rule = get_ddgc_targeting_rule("buff_skill");
        assert!(rule.is_some());
        let rule = rule.unwrap();
        assert!(rule.single_target);
        // buff_skill is SelfOnly in DDGC (self-buff + reposition)
    }

    #[test]
    fn unknown_skill_returns_none() {
        let rule = get_ddgc_targeting_rule("nonexistent_skill");
        assert!(rule.is_none());
    }

    #[test]
    fn multiple_skills_have_targeting_rules() {
        let rules = ddgc_targeting_rules();
        let count = rules.into_iter().count();
        assert!(
            count >= 20,
            "Should have targeting rules for at least 20 skills, had {}",
            count
        );
    }
}