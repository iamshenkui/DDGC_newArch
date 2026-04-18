//! Mantis Magic Flower — QingLong controller family (poison + crowd bleed).
//!
//! DDGC reference: Beast-type controller from the QingLong dungeon.
//! Tier 1 base stats: HP 88, DEF 7.5%, SPD 7, crit 12% on normal_attack.
//! Skills: poison (blight), crowd_bleed (AoE bleed), normal_attack, move.

use framework_combat::effects::EffectNode;
use framework_combat::skills::{SkillDefinition, SkillId};
use framework_combat::targeting::TargetSelector;

use crate::content::actors::{Archetype, ArchetypeName};
use framework_combat::encounter::CombatSide;

/// Mantis Magic Flower base archetype — tier 1 stats from DDGC data.
///
/// HP 88, weapon damage 30–42 (avg 36), speed 7, dodge 0%, crit 12%.
/// Defense 7.5% mapped to `defense` field as 0.075.
/// Controller role: applies blight and AoE bleed rather than raw damage.
pub fn archetype() -> Archetype {
    Archetype {
        name: ArchetypeName::new("Mantis Magic Flower"),
        side: CombatSide::Enemy,
        health: 88.0,
        max_health: 88.0,
        attack: 36.0,
        defense: 0.075,
        speed: 7.0,
        stress: 0.0,
        max_stress: 200.0,
        crit_chance: 0.12,
        accuracy: 0.95,
        dodge: 0.0,
    }
}

// ── Mantis Magic Flower Skills ────────────────────────────────────────────

/// Poison — melee blight attack targeting all enemy ranks.
///
/// DDGC reference: dmg 20–28 (avg 24), applies "New Blight 1",
/// atk 82.5%, launch ranks 1–2, target ranks 1–4.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn poison() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("poison"),
        vec![
            EffectNode::damage(24.0),
            EffectNode::apply_status("blight", Some(1)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Crowd Bleed — melee AoE bleed targeting front 2 enemies.
///
/// DDGC reference: dmg 10–14 (avg 12), applies "New Bleed 1 2" (bleed for 2 rounds),
/// atk 82.5%, launch ranks 1–2, target ~12 (front 2).
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn crowd_bleed() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("crowd_bleed"),
        vec![
            EffectNode::damage(12.0),
            EffectNode::apply_status("bleed", Some(2)),
        ],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Normal Attack — basic melee strike targeting front ranks.
///
/// DDGC reference: dmg 30–42 (avg 36), atk 72%, crit 12%,
/// launch ranks 1–2, target ranks 1–2.
/// Game-gap: position-based targeting not modeled — targets AllEnemies.
pub fn normal_attack() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("normal_attack"),
        vec![EffectNode::damage(36.0)],
        TargetSelector::AllEnemies,
        1,
        None,
    )
}

/// Move — self-repositioning skill (move forward 1 rank).
///
/// DDGC reference: 0 dmg, atk 0%, launch ranks 3–4, moves self forward 1.
/// Uses pull(1) with SelfOnly so the actor moves forward (toward front row).
pub fn move_skill() -> SkillDefinition {
    SkillDefinition::new(
        SkillId::new("move"),
        vec![EffectNode::pull(1)],
        TargetSelector::SelfOnly,
        1,
        None,
    )
}

/// All 4 Mantis Magic Flower skills.
pub fn skill_pack() -> Vec<SkillDefinition> {
    vec![poison(), crowd_bleed(), normal_attack(), move_skill()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use framework_combat::effects::{resolve_skill, EffectContext};
    use framework_combat::formation::{FormationLayout, SlotIndex};
    use framework_rules::actor::{ActorAggregate, ActorId};
    use framework_rules::attributes::{AttributeKey, AttributeValue, ATTR_HEALTH};
    use std::collections::HashMap;

    #[test]
    fn mantis_magic_flower_archetype_is_enemy_beast() {
        let arch = archetype();
        assert_eq!(arch.name.0, "Mantis Magic Flower");
        assert_eq!(arch.side, CombatSide::Enemy);
        assert_eq!(arch.health, 88.0);
        assert_eq!(arch.max_health, 88.0);
        assert_eq!(arch.speed, 7.0);
        assert!(arch.defense > 0.0, "should have nonzero defense");
    }

    #[test]
    fn mantis_magic_flower_poison_applies_blight() {
        let skill = poison();
        assert_eq!(skill.id.0, "poison");
        // Must have damage + blight status
        assert!(skill.effects.len() >= 2, "poison should have damage + blight");
    }

    #[test]
    fn mantis_magic_flower_crowd_bleed_applies_bleed() {
        let skill = crowd_bleed();
        assert_eq!(skill.id.0, "crowd_bleed");
        // Must have damage + bleed status
        assert!(skill.effects.len() >= 2, "crowd_bleed should have damage + bleed");
    }

    #[test]
    fn mantis_magic_flower_skill_pack_has_four_skills() {
        let pack = skill_pack();
        assert_eq!(pack.len(), 4);
        let ids: Vec<&str> = pack.iter().map(|s| s.id.0.as_str()).collect();
        assert!(ids.contains(&"poison"), "missing poison skill");
        assert!(ids.contains(&"crowd_bleed"), "missing crowd_bleed skill");
        assert!(ids.contains(&"normal_attack"), "missing normal_attack skill");
        assert!(ids.contains(&"move"), "missing move skill");
    }

    #[test]
    fn mantis_magic_flower_poison_plus_crowd_bleed_identity() {
        // The core identity of mantis_magic_flower is poison (blight)
        // combined with crowd_bleed (AoE bleed). This test preserves that.
        let pack = skill_pack();
        let has_blight = pack.iter().any(|s| {
            s.effects.iter().any(|e| {
                e.status_kind.as_deref() == Some("blight")
            })
        });
        let has_bleed = pack.iter().any(|s| {
            s.effects.iter().any(|e| {
                e.status_kind.as_deref() == Some("bleed")
            })
        });
        assert!(has_blight, "mantis_magic_flower must apply blight (poison)");
        assert!(has_bleed, "mantis_magic_flower must apply bleed (crowd_bleed)");
    }

    /// Test that move_skill uses pull(1) to move self forward (US-705).
    ///
    /// DDGC self-move: actor moves forward 1 rank (toward front).
    /// The skill uses pull(1) which moves the actor toward lane 0 (front).
    /// This test verifies the movement is deterministic and preserves formation ordering.
    #[test]
    fn mantis_magic_flower_move_skill_moves_self_forward() {
        // Setup: 4 lanes, 2 slots per lane (matches DDGC 4-rank enemy formation)
        // Slot layout: 0-1 (lane 0, rank 1), 2-3 (lane 1, rank 2),
        //              4-5 (lane 2, rank 3), 6-7 (lane 3, rank 4)
        let mut formation = FormationLayout::new(4, 2);
        let mantis_id = ActorId(1);

        // Place mantis at slot 5 (lane 2 = DDGC rank 3)
        formation.place(mantis_id, SlotIndex(5)).unwrap();
        assert_eq!(formation.find_actor(mantis_id), Some(SlotIndex(5)));

        // Create actor aggregate
        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut actor = ActorAggregate::new(mantis_id);
        actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(88.0));
        actors.insert(mantis_id, actor);

        // Execute move_skill (pull(1) self)
        let skill = move_skill();
        let mut ctx = EffectContext::new(
            mantis_id,
            vec![mantis_id], // SelfOnly: actor targets themselves
            &mut formation,
            &mut actors,
        );
        let results = resolve_skill(&skill, &mut ctx);

        // Verify movement occurred (pull result)
        // resolve_skill returns Vec<EffectResult> directly
        assert!(!results.is_empty());
        assert_eq!(results[0].kind, framework_combat::results::EffectResultKind::Pull);

        // Verify mantis moved forward 1 lane: slot 5 (lane 2) -> slot 3 (lane 1)
        assert_eq!(formation.find_actor(mantis_id), Some(SlotIndex(3)));
    }

    /// Test that repeated self-moves are deterministic and preserve formation stability.
    ///
    /// Running move_skill twice from lane 2 should: lane 2 -> lane 1 -> lane 0.
    /// The movement is deterministic (same input -> same output) and the final
    /// slot is stable (no corruption or duplication).
    #[test]
    fn mantis_magic_flower_repeated_move_preserves_formation_stability() {
        let mut formation = FormationLayout::new(4, 2);
        let mantis_id = ActorId(1);

        // Start at lane 2 (slot 5)
        formation.place(mantis_id, SlotIndex(5)).unwrap();

        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut actor = ActorAggregate::new(mantis_id);
        actor.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(88.0));
        actors.insert(mantis_id, actor);

        let skill = move_skill();

        // First move: lane 2 -> lane 1
        {
            let mut ctx = EffectContext::new(
                mantis_id,
                vec![mantis_id],
                &mut formation,
                &mut actors,
            );
            resolve_skill(&skill, &mut ctx);
            assert_eq!(formation.find_actor(mantis_id), Some(SlotIndex(3))); // lane 1
        }

        // Second move: lane 1 -> lane 0 (front)
        {
            let mut ctx = EffectContext::new(
                mantis_id,
                vec![mantis_id],
                &mut formation,
                &mut actors,
            );
            resolve_skill(&skill, &mut ctx);
            assert_eq!(formation.find_actor(mantis_id), Some(SlotIndex(1))); // lane 0
        }

        // Third move: lane 0 is already at front boundary, cannot move further
        {
            let mut ctx = EffectContext::new(
                mantis_id,
                vec![mantis_id],
                &mut formation,
                &mut actors,
            );
            resolve_skill(&skill, &mut ctx);
            // Stays at lane 0 (boundary stops movement)
            assert_eq!(formation.find_actor(mantis_id), Some(SlotIndex(1)));
        }
    }

    /// Test that pull(1) on an enemy target moves them forward (US-705 target-reposition).
    ///
    /// This tests the target-reposition case: when a hero uses pull on an enemy,
    /// the enemy moves forward toward the front row.
    #[test]
    fn mantis_magic_flower_target_pull_moves_enemy_forward() {
        let mut formation = FormationLayout::new(4, 2);
        let hero_id = ActorId(1);
        let enemy_id = ActorId(2);

        // Place enemy at slot 6 (lane 3, back row)
        // Place hero at slot 0 (lane 0, front row)
        formation.place(enemy_id, SlotIndex(6)).unwrap();
        formation.place(hero_id, SlotIndex(0)).unwrap();

        let mut actors: HashMap<ActorId, ActorAggregate> = HashMap::new();
        let mut enemy = ActorAggregate::new(enemy_id);
        enemy.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
        actors.insert(enemy_id, enemy);
        let mut hero = ActorAggregate::new(hero_id);
        hero.set_base(AttributeKey::new(ATTR_HEALTH), AttributeValue(100.0));
        actors.insert(hero_id, hero);

        // Create a skill that pulls the target forward
        let pull_skill = SkillDefinition::new(
            SkillId::new("pull_test"),
            vec![EffectNode::pull(1)],
            TargetSelector::AllEnemies, // pulls any/all enemies
            1,
            None,
        );

        // Hero uses pull on enemy
        let mut ctx = EffectContext::new(
            hero_id,
            vec![enemy_id], // targeted enemy
            &mut formation,
            &mut actors,
        );
        let results = resolve_skill(&pull_skill, &mut ctx);

        // Verify pull occurred
        // resolve_skill returns Vec<EffectResult> directly
        assert!(!results.is_empty());
        assert_eq!(results[0].kind, framework_combat::results::EffectResultKind::Pull);

        // Enemy moved from lane 3 (slot 6) to lane 2 (slot 4)
        assert_eq!(formation.find_actor(enemy_id), Some(SlotIndex(4)));
    }
}
