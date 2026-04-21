//! Hero progression system — XP, leveling, and resolve level gates.
//!
//! Implements hero experience and leveling between dungeon runs using thresholds
//! parsed from Campaign.json.

use crate::contracts::DungeonType;

/// XP thresholds for hero level advancement (cumulative).
///
/// From Campaign.json: level_threshold_table [0, 2, 6, 10, 16, 22, 32, 42]
pub const LEVEL_THRESHOLD_TABLE: [u32; 8] = [0, 2, 6, 10, 16, 22, 32, 42];

/// XP thresholds for resolve level advancement (cumulative).
///
/// From Campaign.json: resolve_thresholds [0, 2, 8, 14, 30, 62, 94]
pub const RESOLVE_THRESHOLD_TABLE: [u32; 7] = [0, 2, 8, 14, 30, 62, 94];

/// A hero's progression state — XP, hero level, and resolve level.
///
/// Tracks hero experience and leveling between dungeon runs. Both hero_level
/// and resolve_level advance based on total XP but use different threshold tables.
#[derive(Debug, Clone, PartialEq)]
pub struct HeroProgress {
    /// Total accumulated XP.
    pub xp: u32,
    /// Current hero level (0-7 based on LEVEL_THRESHOLD_TABLE).
    pub hero_level: u32,
    /// Current resolve level (0-6 based on RESOLVE_THRESHOLD_TABLE).
    pub resolve_level: u32,
}

impl HeroProgress {
    /// Create a new hero progress at starting state (level 0, resolve 0, 0 XP).
    pub fn new() -> Self {
        HeroProgress {
            xp: 0,
            hero_level: 0,
            resolve_level: 0,
        }
    }

    /// Add XP and advance levels if thresholds are crossed.
    ///
    /// Both hero_level and resolve_level advance independently based on their
    /// respective threshold tables. Levels can only advance, not regress.
    pub fn add_xp(&mut self, amount: u32) {
        if amount == 0 {
            return;
        }

        self.xp += amount;

        // Advance hero_level based on LEVEL_THRESHOLD_TABLE
        let mut new_hero_level = 0;
        for (i, threshold) in LEVEL_THRESHOLD_TABLE.iter().enumerate() {
            if self.xp >= *threshold {
                new_hero_level = i as u32;
            } else {
                break;
            }
        }
        self.hero_level = new_hero_level;

        // Advance resolve_level based on RESOLVE_THRESHOLD_TABLE
        let mut new_resolve_level = 0;
        for (i, threshold) in RESOLVE_THRESHOLD_TABLE.iter().enumerate() {
            if self.xp >= *threshold {
                new_resolve_level = i as u32;
            } else {
                break;
            }
        }
        self.resolve_level = new_resolve_level;
    }

    /// Check if a hero can enter a given dungeon based on resolve level.
    ///
    /// Heroes with resolve_level higher than the dungeon's level cannot enter
    /// "low-level" dungeons. This enforces that high-resolve heroes are
    /// gated from beginner content.
    ///
    /// Returns true if the hero can enter the dungeon, false otherwise.
    pub fn can_enter_dungeon(&self, dungeon: DungeonType) -> bool {
        let dungeon_level = dungeon_level(dungeon);
        self.resolve_level <= dungeon_level
    }
}

/// Get the dungeon level for a given DungeonType.
///
/// Maps dungeon types to their difficulty level:
/// - QingLong: level 3 (starter dungeon)
/// - BaiHu: level 4
/// - ZhuQue: level 5
/// - XuanWu: level 6
fn dungeon_level(dungeon: DungeonType) -> u32 {
    match dungeon {
        DungeonType::QingLong => 3,
        DungeonType::BaiHu => 4,
        DungeonType::ZhuQue => 5,
        DungeonType::XuanWu => 6,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hero_starts_at_level_zero() {
        let progress = HeroProgress::new();
        assert_eq!(progress.xp, 0);
        assert_eq!(progress.hero_level, 0);
        assert_eq!(progress.resolve_level, 0);
    }

    #[test]
    fn adding_2_xp_reaches_hero_level_1() {
        // level_threshold_table[1] = 2
        let mut progress = HeroProgress::new();
        progress.add_xp(2);
        assert_eq!(progress.xp, 2);
        assert_eq!(progress.hero_level, 1);
        assert_eq!(progress.resolve_level, 1); // resolve_threshold_table[1] = 2
    }

    #[test]
    fn adding_8_xp_reaches_resolve_level_1() {
        // resolve_threshold_table[1] = 2, [2] = 8
        let mut progress = HeroProgress::new();
        progress.add_xp(8);
        assert_eq!(progress.xp, 8);
        assert_eq!(progress.hero_level, 2); // level_threshold_table[2] = 6
        assert_eq!(progress.resolve_level, 2); // resolve_threshold_table[2] = 8
    }

    #[test]
    fn all_threshold_boundaries_produce_correct_level_ups() {
        // Test hero level thresholds
        let mut progress = HeroProgress::new();
        assert_eq!(progress.hero_level, 0);

        progress.add_xp(1);
        assert_eq!(progress.hero_level, 0, "1 XP should still be level 0");

        progress.add_xp(1); // Now 2 XP total
        assert_eq!(progress.hero_level, 1, "2 XP should be level 1");

        progress.add_xp(3); // Now 5 XP total
        assert_eq!(progress.hero_level, 1, "5 XP should still be level 1");

        progress.add_xp(1); // Now 6 XP total
        assert_eq!(progress.hero_level, 2, "6 XP should be level 2");

        progress.add_xp(3); // Now 9 XP total
        assert_eq!(progress.hero_level, 2, "9 XP should still be level 2");

        progress.add_xp(1); // Now 10 XP total
        assert_eq!(progress.hero_level, 3, "10 XP should be level 3");

        // Test resolve level thresholds
        let mut progress = HeroProgress::new();
        assert_eq!(progress.resolve_level, 0);

        progress.add_xp(1);
        assert_eq!(progress.resolve_level, 0, "1 XP should still be resolve 0");

        progress.add_xp(1); // Now 2 XP total
        assert_eq!(progress.resolve_level, 1, "2 XP should be resolve 1");

        progress.add_xp(5); // Now 7 XP total
        assert_eq!(progress.resolve_level, 1, "7 XP should still be resolve 1");

        progress.add_xp(1); // Now 8 XP total
        assert_eq!(progress.resolve_level, 2, "8 XP should be resolve 2");

        progress.add_xp(5); // Now 13 XP total
        assert_eq!(progress.resolve_level, 2, "13 XP should still be resolve 2");

        progress.add_xp(1); // Now 14 XP total
        assert_eq!(progress.resolve_level, 3, "14 XP should be resolve 3");

        progress.add_xp(15); // Now 29 XP total
        assert_eq!(progress.resolve_level, 3, "29 XP should still be resolve 3");

        progress.add_xp(1); // Now 30 XP total
        assert_eq!(progress.resolve_level, 4, "30 XP should be resolve 4");
    }

    #[test]
    fn max_level_thresholds() {
        // Test reaching max hero level (7 at 42 XP)
        let mut progress = HeroProgress::new();
        progress.add_xp(42);
        assert_eq!(progress.hero_level, 7);
        assert_eq!(progress.resolve_level, 4); // resolve 4 at 42 XP (30 <= 42 < 62)

        // Add more XP to reach max resolve
        progress.add_xp(52); // Now 94 XP total
        assert_eq!(progress.hero_level, 7, "Should stay at max hero level 7");
        assert_eq!(progress.resolve_level, 6, "Should be at max resolve level 6 at 94 XP");

        // Add more XP beyond max
        progress.add_xp(100);
        assert_eq!(progress.hero_level, 7, "Should stay at max hero level 7");
        assert_eq!(progress.resolve_level, 6, "Should stay at max resolve level 6");
    }

    #[test]
    fn can_enter_dungeon_allows_matching_resolve() {
        let mut progress = HeroProgress::new();
        progress.add_xp(2); // resolve_level = 1

        assert!(progress.can_enter_dungeon(DungeonType::QingLong)); // level 3, resolve 1 <= 3
        assert!(progress.can_enter_dungeon(DungeonType::BaiHu)); // level 4, resolve 1 <= 4
        assert!(progress.can_enter_dungeon(DungeonType::ZhuQue)); // level 5, resolve 1 <= 5
        assert!(progress.can_enter_dungeon(DungeonType::XuanWu)); // level 6, resolve 1 <= 6
    }

    #[test]
    fn can_enter_dungeon_blocks_high_resolve_in_low_dungeons() {
        let mut progress = HeroProgress::new();
        progress.add_xp(30); // resolve_level = 4

        // resolve 4 > dungeon level 3 (QingLong) -> blocked
        assert!(!progress.can_enter_dungeon(DungeonType::QingLong));

        // resolve 4 <= dungeon level 4 (BaiHu) -> allowed
        assert!(progress.can_enter_dungeon(DungeonType::BaiHu));

        // resolve 4 <= dungeon level 5 (ZhuQue) -> allowed
        assert!(progress.can_enter_dungeon(DungeonType::ZhuQue));

        // resolve 4 <= dungeon level 6 (XuanWu) -> allowed
        assert!(progress.can_enter_dungeon(DungeonType::XuanWu));
    }

    #[test]
    fn can_enter_dungeon_blocks_max_resolve_from_all_except_highest() {
        let mut progress = HeroProgress::new();
        progress.add_xp(94); // resolve_level = 6 (max)

        // resolve 6 > all dungeons except XuanWu (level 6)
        assert!(!progress.can_enter_dungeon(DungeonType::QingLong)); // 6 > 3
        assert!(!progress.can_enter_dungeon(DungeonType::BaiHu)); // 6 > 4
        assert!(!progress.can_enter_dungeon(DungeonType::ZhuQue)); // 6 > 5
        assert!(progress.can_enter_dungeon(DungeonType::XuanWu)); // 6 <= 6
    }

    #[test]
    fn zero_xp_add_does_nothing() {
        let mut progress = HeroProgress::new();
        progress.add_xp(0);
        assert_eq!(progress.xp, 0);
        assert_eq!(progress.hero_level, 0);
        assert_eq!(progress.resolve_level, 0);
    }
}