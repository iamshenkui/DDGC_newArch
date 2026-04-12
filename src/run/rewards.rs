//! Post-battle reward logic — game-layer state applied after clearing rooms.
//!
//! The framework_progression crate has no reward hook; rewards are purely
//! game-layer logic. This module defines the reward structure and computes
//! rewards based on room kind.

use framework_progression::rooms::RoomKind;

/// Post-battle updates applied after clearing a room.
///
/// These represent the game-layer consequences of completing a room:
/// gold earned, HP recovered (camping/rest), and stress relief.
#[derive(Debug, Clone, PartialEq)]
pub struct PostBattleUpdate {
    pub gold_earned: u32,
    pub hp_recovered: f64,
    pub stress_change: f64,
}

/// Compute rewards for clearing a room of the given kind.
///
/// DDGC reward balance:
/// - Combat rooms: moderate gold, small HP recovery, mild stress relief
/// - Boss rooms: high gold, significant HP recovery, strong stress relief
/// - Other rooms: no combat rewards
pub fn compute_post_battle_update(room_kind: &RoomKind) -> PostBattleUpdate {
    match room_kind {
        RoomKind::Combat => PostBattleUpdate {
            gold_earned: 50,
            hp_recovered: 2.0,
            stress_change: -5.0,
        },
        RoomKind::Boss => PostBattleUpdate {
            gold_earned: 200,
            hp_recovered: 10.0,
            stress_change: -15.0,
        },
        _ => PostBattleUpdate {
            gold_earned: 0,
            hp_recovered: 0.0,
            stress_change: 0.0,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combat_room_gives_moderate_rewards() {
        let update = compute_post_battle_update(&RoomKind::Combat);
        assert_eq!(update.gold_earned, 50);
        assert_eq!(update.hp_recovered, 2.0);
        assert_eq!(update.stress_change, -5.0);
    }

    #[test]
    fn boss_room_gives_high_rewards() {
        let update = compute_post_battle_update(&RoomKind::Boss);
        assert_eq!(update.gold_earned, 200);
        assert_eq!(update.hp_recovered, 10.0);
        assert_eq!(update.stress_change, -15.0);
    }

    #[test]
    fn event_room_gives_no_combat_rewards() {
        let update = compute_post_battle_update(&RoomKind::Event);
        assert_eq!(update.gold_earned, 0);
        assert_eq!(update.hp_recovered, 0.0);
        assert_eq!(update.stress_change, 0.0);
    }
}
