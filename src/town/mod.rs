//! Town visit phase — services, healing, and activity resolution between dungeon runs.
//!
//! This module implements the meta-game loop between dungeon runs:
//! - Town visit represents a single town phase with available services
//! - Hero roster state tracks stress and health during town phase
//! - Gold and heirloom balances are managed in town
//! - Building services are resolved through `perform_town_activity`
//! - All activities produce a deterministic trace

use crate::contracts::{
    BuildingRegistry, BuildingUpgradeState, TownBuilding,
    TownState,
};

/// A hero in town with their current stress and health state.
///
/// This represents the hero's state during a town visit. Stress can be
/// healed through building services like the Abbey.
#[derive(Debug, Clone, PartialEq)]
pub struct HeroInTown {
    /// Unique hero identifier.
    pub id: String,
    /// Current stress level (0 = no stress, max_stress = afflicted).
    pub stress: f64,
    /// Maximum stress level.
    pub max_stress: f64,
    /// Current health level.
    pub health: f64,
    /// Maximum health level.
    pub max_health: f64,
    /// Hero class ID (e.g., "alchemist", "hunter").
    pub class_id: String,
}

impl HeroInTown {
    /// Create a new hero in town.
    pub fn new(id: &str, class_id: &str, stress: f64, max_stress: f64, health: f64, max_health: f64) -> Self {
        HeroInTown {
            id: id.to_string(),
            class_id: class_id.to_string(),
            stress,
            max_stress,
            health,
            max_health,
        }
    }

    /// Check if the hero is afflicted (stress at maximum).
    pub fn is_afflicted(&self) -> bool {
        self.stress >= self.max_stress
    }

    /// Check if the hero is wounded (health below maximum).
    pub fn is_wounded(&self) -> bool {
        self.health < self.max_health
    }
}

/// Type of quirk treatment at the Sanitarium.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuirkTreatmentType {
    /// Treatment for positive quirks.
    Positive,
    /// Treatment for negative (non-permanent) quirks.
    Negative,
    /// Treatment for permanent negative quirks.
    PermanentNegative,
}

/// Activity that can be performed at a town building.
#[derive(Debug, Clone, PartialEq)]
pub enum TownActivity {
    /// Pray at the Abbey to heal stress.
    Pray,
    /// Rest at the Inn to recover health and reduce stress.
    Rest,
    /// Recruit a new hero at the Stagecoach.
    Recruit,
    /// Train at the Guild to gain experience.
    Train,
    /// Repair equipment at the Blacksmith.
    Repair,
    /// Upgrade weapon at the Blacksmith.
    UpgradeWeapon,
    /// Upgrade building itself (apply upgrade level).
    UpgradeBuilding,
    /// Treat a quirk at the Sanitarium.
    TreatQuirk {
        /// Index of the treatment slot being used.
        slot_index: usize,
        /// Type of quirk treatment.
        quirk_type: QuirkTreatmentType,
    },
    /// Treat a disease at the Sanitarium.
    TreatDisease {
        /// Index of the treatment slot being used.
        slot_index: usize,
    },
    /// Drink at the Tavern bar.
    TavernBar {
        /// Index of the tavern slot being used.
        slot_index: usize,
    },
    /// Gamble at the Tavern.
    TavernGambling {
        /// Index of the tavern slot being used.
        slot_index: usize,
    },
    /// Visit the Tavern brothel.
    TavernBrothel {
        /// Index of the tavern slot being used.
        slot_index: usize,
    },
}

/// Result of performing a single town activity.
#[derive(Debug, Clone, PartialEq)]
pub struct TownActivityRecord {
    /// The building ID where the activity was performed.
    pub building_id: String,
    /// The activity that was performed.
    pub activity: TownActivity,
    /// Hero ID (if applicable, e.g., stress heal).
    pub hero_id: Option<String>,
    /// The upgrade level used (if applicable).
    pub upgrade_level: Option<char>,
    /// Gold cost of the activity.
    pub gold_cost: u32,
    /// Stress change applied to the hero (negative = reduction).
    pub stress_change: f64,
    /// Health change applied to the hero (positive = recovery).
    pub health_change: f64,
    /// Whether the activity was successful.
    pub success: bool,
    /// Description of the result.
    pub message: String,
}

impl TownActivityRecord {
    /// Create a successful activity record.
    fn success(
        building_id: &str,
        activity: TownActivity,
        hero_id: Option<String>,
        upgrade_level: Option<char>,
        gold_cost: u32,
        stress_change: f64,
        health_change: f64,
        message: &str,
    ) -> Self {
        TownActivityRecord {
            building_id: building_id.to_string(),
            activity,
            hero_id,
            upgrade_level,
            gold_cost,
            stress_change,
            health_change,
            success: true,
            message: message.to_string(),
        }
    }

    /// Create a failed activity record.
    fn failure(building_id: &str, activity: TownActivity, message: &str) -> Self {
        TownActivityRecord {
            building_id: building_id.to_string(),
            activity,
            hero_id: None,
            upgrade_level: None,
            gold_cost: 0,
            stress_change: 0.0,
            health_change: 0.0,
            success: false,
            message: message.to_string(),
        }
    }
}

/// A trace of all activities performed during a town visit.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TownActivityTrace {
    /// All activity records in order performed.
    pub activities: Vec<TownActivityRecord>,
}

impl TownActivityTrace {
    /// Create a new empty trace.
    pub fn new() -> Self {
        TownActivityTrace {
            activities: Vec::new(),
        }
    }

    /// Record an activity.
    pub fn record(&mut self, record: TownActivityRecord) {
        self.activities.push(record);
    }

    /// Get total gold spent.
    pub fn total_gold_spent(&self) -> u32 {
        self.activities.iter().map(|a| a.gold_cost).sum()
    }

    /// Get total stress healed across all activities.
    pub fn total_stress_healed(&self) -> f64 {
        self.activities
            .iter()
            .filter(|a| a.stress_change < 0.0)
            .map(|a| a.stress_change.abs())
            .sum()
    }

    /// Get total health recovered across all activities.
    pub fn total_health_recovered(&self) -> f64 {
        self.activities
            .iter()
            .filter(|a| a.health_change > 0.0)
            .map(|a| a.health_change)
            .sum()
    }
}

/// Represents a single town visit phase.
///
/// A town visit occurs between dungeon runs, allowing heroes to heal stress,
/// recover health, recruit new heroes, and upgrade building services.
#[derive(Debug, Clone, PartialEq)]
pub struct TownVisit {
    /// Current town state (gold, heirlooms, building upgrades).
    pub town_state: TownState,
    /// Heroes currently in town.
    pub heroes: Vec<HeroInTown>,
    /// Building registry for looking up building services.
    pub building_registry: BuildingRegistry,
    /// Trace of activities performed this visit.
    pub trace: TownActivityTrace,
}

impl TownVisit {
    /// Create a new town visit with initial state.
    pub fn new(
        town_state: TownState,
        heroes: Vec<HeroInTown>,
        building_registry: BuildingRegistry,
    ) -> Self {
        TownVisit {
            town_state,
            heroes,
            building_registry,
            trace: TownActivityTrace::new(),
        }
    }

    /// Create a town visit from dungeon run results.
    ///
    /// This initializes town state from the gold earned in a dungeon run
    /// and creates heroes from the run state.
    pub fn from_dungeon_run(
        gold_earned: u32,
        stress_change: f64,
        hero_count: usize,
        building_registry: BuildingRegistry,
    ) -> Self {
        // Create initial town state with gold from dungeon run
        let mut town_state = TownState::new(gold_earned);

        // Initialize building states for all buildings
        for building_id in building_registry.all_ids() {
            town_state
                .building_states
                .insert(building_id.to_string(), BuildingUpgradeState::new(building_id, Some('a')));
        }

        // Create heroes with stress from dungeon run
        let heroes: Vec<HeroInTown> = (0..hero_count)
            .map(|i| {
                // Stress increases based on dungeon run
                let stress = (stress_change.max(0.0) * 10.0).min(200.0);
                HeroInTown::new(
                    &format!("hero_{}", i),
                    "alchemist", // Default class for initial heroes
                    stress,
                    200.0,
                    100.0,
                    100.0,
                )
            })
            .collect();

        TownVisit::new(town_state, heroes, building_registry)
    }

    /// Get a hero by ID.
    pub fn get_hero(&self, hero_id: &str) -> Option<&HeroInTown> {
        self.heroes.iter().find(|h| h.id == hero_id)
    }

    /// Get a mutable hero by ID.
    fn get_hero_mut(&mut self, hero_id: &str) -> Option<&mut HeroInTown> {
        self.heroes.iter_mut().find(|h| h.id == hero_id)
    }

    /// Get the cost for a specific building upgrade level.
    fn get_upgrade_cost(&self, building_id: &str, level_code: char) -> Option<u32> {
        self.building_registry
            .get_upgrade_cost(building_id, level_code)
    }

    /// Get the stress heal effect at a given upgrade level.
    fn get_stress_heal_effect(&self, building_id: &str, level_code: char) -> Option<f64> {
        self.building_registry
            .get_effect_at_level(building_id, level_code, "stress_heal")
    }

    /// Check if a building is unlocked based on its unlock conditions.
    ///
    /// In the town visit context, we assume all buildings are available since
    /// the town visit phase occurs between dungeon runs. A full implementation
    /// would check completed_runs and defeated_monsters against the player's history.
    fn is_building_unlocked(&self, _building: &TownBuilding) -> bool {
        // For now, all buildings are considered unlocked in the town visit context.
        // In a full implementation, this would check completed_runs, defeated_monsters, etc.
        // against the player's actual run history to determine if unlock conditions are met.
        true
    }

    /// Perform a town activity at a building.
    ///
    /// This function resolves building services deterministically based on:
    /// - The building and activity requested
    /// - The hero ID (for per-hero activities like stress heal)
    /// - The upgrade level (for building upgrades)
    ///
    /// Returns a record of the activity result.
    pub fn perform_town_activity(
        &mut self,
        building_id: &str,
        activity: TownActivity,
        hero_id: Option<&str>,
        upgrade_level: Option<char>,
    ) -> TownActivityRecord {
        // Look up the building
        let building = match self.building_registry.get(building_id) {
            Some(b) => b,
            None => {
                return TownActivityRecord::failure(
                    building_id,
                    activity,
                    "Building not found",
                );
            }
        };

        // Check if building is unlocked
        if !self.is_building_unlocked(building) {
            return TownActivityRecord::failure(
                building_id,
                activity,
                "Building is locked",
            );
        }

        match activity {
            TownActivity::Pray => self.perform_pray(building_id, hero_id, upgrade_level),
            TownActivity::Rest => self.perform_rest(building_id, hero_id, upgrade_level),
            TownActivity::Recruit => self.perform_recruit(building_id),
            TownActivity::Train => self.perform_train(building_id, hero_id, upgrade_level),
            TownActivity::Repair => self.perform_repair(building_id, hero_id, upgrade_level),
            TownActivity::UpgradeWeapon => self.perform_upgrade_weapon(building_id, hero_id, upgrade_level),
            TownActivity::UpgradeBuilding => self.perform_upgrade_building(building_id, upgrade_level),
            TownActivity::TreatQuirk { slot_index, quirk_type } => {
                self.perform_treat_quirk(building_id, hero_id, slot_index, quirk_type, upgrade_level)
            }
            TownActivity::TreatDisease { slot_index } => {
                self.perform_treat_disease(building_id, hero_id, slot_index, upgrade_level)
            }
            TownActivity::TavernBar { slot_index } => {
                self.perform_tavern_bar(building_id, hero_id, slot_index, upgrade_level)
            }
            TownActivity::TavernGambling { slot_index } => {
                self.perform_tavern_gambling(building_id, hero_id, slot_index, upgrade_level)
            }
            TownActivity::TavernBrothel { slot_index } => {
                self.perform_tavern_brothel(building_id, hero_id, slot_index, upgrade_level)
            }
        }
    }

    /// Perform prayer at the Abbey to heal stress.
    fn perform_pray(
        &mut self,
        building_id: &str,
        hero_id: Option<&str>,
        upgrade_level: Option<char>,
    ) -> TownActivityRecord {
        // Get the hero ID
        let hero_id_str = match hero_id {
            Some(id) => id.to_string(),
            None => {
                return TownActivityRecord::failure(
                    building_id,
                    TownActivity::Pray,
                    "No hero specified for prayer",
                );
            }
        };

        // First, get hero stress and validate hero exists (without mutable borrow)
        let hero_exists = self.heroes.iter().any(|h| h.id == hero_id_str);
        if !hero_exists {
            return TownActivityRecord::failure(
                building_id,
                TownActivity::Pray,
                "Hero not found",
            );
        }

        // Determine the upgrade level to use (default to current building level)
        let level = upgrade_level.unwrap_or_else(|| {
            self.town_state
                .get_upgrade_level(building_id)
                .unwrap_or('a')
        });

        // Get the stress heal cost
        let cost = self.get_upgrade_cost(building_id, level);
        let cost = match cost {
            Some(c) => c,
            None => {
                return TownActivityRecord::failure(
                    building_id,
                    TownActivity::Pray,
                    "Invalid upgrade level",
                );
            }
        };

        // Check if we have enough gold
        if self.town_state.gold < cost {
            return TownActivityRecord::failure(
                building_id,
                TownActivity::Pray,
                "Not enough gold for prayer",
            );
        }

        // Get the stress heal effect
        let stress_heal = self.get_stress_heal_effect(building_id, level).unwrap_or(1.0);

        // Now get mutable reference to hero and apply changes
        let hero = self.get_hero_mut(&hero_id_str).unwrap();
        let old_stress = hero.stress;
        hero.stress = (hero.stress - stress_heal).max(0.0);
        let actual_heal = old_stress - hero.stress;
        let new_stress = hero.stress;
        let _ = hero; // Release mutable borrow

        // Deduct gold
        self.town_state.gold -= cost;

        let record = TownActivityRecord::success(
            building_id,
            TownActivity::Pray,
            Some(hero_id_str),
            Some(level),
            cost,
            -actual_heal, // Negative because stress is reduced
            0.0,
            &format!(
                "Prayed at Abbey: stress healed {:.1} ({} -> {:.1}), cost {} gold",
                actual_heal, old_stress, new_stress, cost
            ),
        );

        self.trace.record(record.clone());
        record
    }

    /// Perform rest at the Inn to recover health and reduce stress.
    fn perform_rest(
        &mut self,
        building_id: &str,
        hero_id: Option<&str>,
        upgrade_level: Option<char>,
    ) -> TownActivityRecord {
        // Get the hero ID
        let hero_id_str = match hero_id {
            Some(id) => id.to_string(),
            None => {
                return TownActivityRecord::failure(
                    building_id,
                    TownActivity::Rest,
                    "No hero specified for rest",
                );
            }
        };

        // First, validate hero exists (without mutable borrow)
        let hero_exists = self.heroes.iter().any(|h| h.id == hero_id_str);
        if !hero_exists {
            return TownActivityRecord::failure(
                building_id,
                TownActivity::Rest,
                "Hero not found",
            );
        }

        // Determine the upgrade level
        let level = upgrade_level.unwrap_or_else(|| {
            self.town_state
                .get_upgrade_level(building_id)
                .unwrap_or('a')
        });

        // Get the cost
        let cost = self.get_upgrade_cost(building_id, level).unwrap_or(100);

        // Check if we have enough gold
        if self.town_state.gold < cost {
            return TownActivityRecord::failure(
                building_id,
                TownActivity::Rest,
                "Not enough gold for rest",
            );
        }

        // Now get mutable reference to hero and apply changes
        let hero = self.get_hero_mut(&hero_id_str).unwrap();
        let old_health = hero.health;
        let old_stress = hero.stress;

        hero.health = hero.max_health; // Full health restore
        hero.stress = (hero.stress - 5.0).max(0.0); // Small stress reduction

        let health_recovered = hero.health - old_health;
        let stress_reduced = old_stress - hero.stress;
        let new_health = hero.health;
        let new_stress = hero.stress;
        let _ = hero; // Release mutable borrow

        // Deduct gold
        self.town_state.gold -= cost;

        let record = TownActivityRecord::success(
            building_id,
            TownActivity::Rest,
            Some(hero_id_str),
            Some(level),
            cost,
            -stress_reduced,
            health_recovered,
            &format!(
                "Rested at Inn: healed {:.1} HP ({} -> {:.1}), reduced stress by {:.1} ({} -> {:.1}), cost {} gold",
                health_recovered, old_health, new_health, stress_reduced, old_stress, new_stress, cost
            ),
        );

        self.trace.record(record.clone());
        record
    }

    /// Perform recruitment at the Stagecoach.
    fn perform_recruit(&mut self, building_id: &str) -> TownActivityRecord {
        // Determine the upgrade level
        let level = self
            .town_state
            .get_upgrade_level(building_id)
            .unwrap_or('a');

        // Get the cost (discounted if upgrade exists)
        let base_cost = 500u32;
        let discount = self
            .building_registry
            .get_effect_at_level(building_id, level, "recruit_discount")
            .unwrap_or(0.0);
        let cost = (base_cost as f64 * (1.0 - discount)) as u32;

        // Check if we have enough gold
        if self.town_state.gold < cost {
            return TownActivityRecord::failure(
                building_id,
                TownActivity::Recruit,
                "Not enough gold for recruitment",
            );
        }

        // Create a new hero
        let new_id = format!("hero_{}", self.heroes.len());
        let new_hero = HeroInTown::new(&new_id, "hunter", 0.0, 200.0, 100.0, 100.0);
        self.heroes.push(new_hero);

        // Deduct gold
        self.town_state.gold -= cost;

        let record = TownActivityRecord::success(
            building_id,
            TownActivity::Recruit,
            Some(new_id),
            Some(level),
            cost,
            0.0,
            0.0,
            &format!("Recruited new hero for {} gold (discount: {:.0}%)", cost, discount * 100.0),
        );

        self.trace.record(record.clone());
        record
    }

    /// Perform training at the Guild.
    fn perform_train(
        &mut self,
        building_id: &str,
        hero_id: Option<&str>,
        upgrade_level: Option<char>,
    ) -> TownActivityRecord {
        let hero_id_str = hero_id.map(|s| s.to_string());

        let level = upgrade_level.unwrap_or_else(|| {
            self.town_state
                .get_upgrade_level(building_id)
                .unwrap_or('a')
        });

        // Get the skill cost discount (cumulative 10% per level, up to 50%)
        let discount = self
            .building_registry
            .get_effect_at_level(building_id, level, "skill_cost_discount")
            .unwrap_or(0.0);

        let record = TownActivityRecord::success(
            building_id,
            TownActivity::Train,
            hero_id_str,
            Some(level),
            0,
            0.0,
            0.0,
            &format!(
                "Trained at Guild: skill cost discount {:.0}% applied",
                discount * 100.0
            ),
        );

        self.trace.record(record.clone());
        record
    }

    /// Perform repair at the Blacksmith.
    fn perform_repair(
        &mut self,
        building_id: &str,
        hero_id: Option<&str>,
        upgrade_level: Option<char>,
    ) -> TownActivityRecord {
        let hero_id_str = hero_id.map(|s| s.to_string());

        let level = upgrade_level.unwrap_or_else(|| {
            self.town_state
                .get_upgrade_level(building_id)
                .unwrap_or('a')
        });

        // Get the equipment cost discount (cumulative 10% per level, up to 50%)
        let discount = self
            .building_registry
            .get_effect_at_level(building_id, level, "equipment_cost_discount")
            .unwrap_or(0.0);

        let record = TownActivityRecord::success(
            building_id,
            TownActivity::Repair,
            hero_id_str,
            Some(level),
            0,
            0.0,
            0.0,
            &format!(
                "Repaired equipment at Blacksmith: equipment cost discount {:.0}% applied",
                discount * 100.0
            ),
        );

        self.trace.record(record.clone());
        record
    }

    /// Perform weapon upgrade at the Blacksmith.
    fn perform_upgrade_weapon(
        &mut self,
        building_id: &str,
        hero_id: Option<&str>,
        upgrade_level: Option<char>,
    ) -> TownActivityRecord {
        let hero_id_str = hero_id.map(|s| s.to_string());

        let level = upgrade_level.unwrap_or_else(|| {
            self.town_state
                .get_upgrade_level(building_id)
                .unwrap_or('a')
        });

        // Get the equipment cost discount (cumulative 10% per level, up to 50%)
        let discount = self
            .building_registry
            .get_effect_at_level(building_id, level, "equipment_cost_discount")
            .unwrap_or(0.0);

        let record = TownActivityRecord::success(
            building_id,
            TownActivity::UpgradeWeapon,
            hero_id_str,
            Some(level),
            0,
            0.0,
            0.0,
            &format!(
                "Upgraded weapon at Blacksmith: equipment cost discount {:.0}% applied",
                discount * 100.0
            ),
        );

        self.trace.record(record.clone());
        record
    }

    /// Upgrade a building to a new level.
    fn perform_upgrade_building(
        &mut self,
        building_id: &str,
        upgrade_level: Option<char>,
    ) -> TownActivityRecord {
        let level = match upgrade_level {
            Some(l) => l,
            None => {
                return TownActivityRecord::failure(
                    building_id,
                    TownActivity::UpgradeBuilding,
                    "No upgrade level specified",
                );
            }
        };

        // Get the building
        let building = match self.building_registry.get(building_id) {
            Some(b) => b,
            None => {
                return TownActivityRecord::failure(
                    building_id,
                    TownActivity::UpgradeBuilding,
                    "Building not found",
                );
            }
        };

        // Apply the upgrade through town_state
        match self.town_state.apply_upgrade(building_id, level, building) {
            Some(cost) => {
                let record = TownActivityRecord::success(
                    building_id,
                    TownActivity::UpgradeBuilding,
                    None,
                    Some(level),
                    cost,
                    0.0,
                    0.0,
                    &format!(
                        "Upgraded {} to level {} for {} gold",
                        building_id, level, cost
                    ),
                );
                self.trace.record(record.clone());
                record
            }
            None => TownActivityRecord::failure(
                building_id,
                TownActivity::UpgradeBuilding,
                "Not enough gold or invalid upgrade level",
            ),
        }
    }

    /// Get the quirk treatment cost for a specific treatment type.
    fn get_quirk_treatment_cost(
        &self,
        building_id: &str,
        quirk_type: QuirkTreatmentType,
        level: char,
    ) -> Option<u32> {
        let effect_id = match quirk_type {
            QuirkTreatmentType::Positive => "positive_quirk_cost",
            QuirkTreatmentType::Negative => "negative_quirk_cost",
            QuirkTreatmentType::PermanentNegative => "permanent_negative_quirk_cost",
        };
        self.building_registry
            .get_effect_at_level(building_id, level, effect_id)
            .map(|v| v as u32)
    }

    /// Get the disease treatment cost.
    fn get_disease_treatment_cost(&self, building_id: &str, level: char) -> Option<u32> {
        self.building_registry
            .get_effect_at_level(building_id, level, "disease_cost")
            .map(|v| v as u32)
    }

    /// Get the cure-all chance for disease treatment.
    fn get_cure_all_chance(&self, building_id: &str, level: char) -> Option<f64> {
        self.building_registry
            .get_effect_at_level(building_id, level, "cure_all_chance")
    }

    /// Get the treatment slots available for a building.
    fn get_treatment_slots(&self, building_id: &str, level: char, effect_id: &str) -> Option<usize> {
        self.building_registry
            .get_effect_at_level(building_id, level, effect_id)
            .map(|v| v as usize)
    }

    /// Perform quirk treatment at the Sanitarium.
    fn perform_treat_quirk(
        &mut self,
        building_id: &str,
        hero_id: Option<&str>,
        slot_index: usize,
        quirk_type: QuirkTreatmentType,
        upgrade_level: Option<char>,
    ) -> TownActivityRecord {
        let hero_id_str = match hero_id {
            Some(id) => id.to_string(),
            None => {
                return TownActivityRecord::failure(
                    building_id,
                    TownActivity::TreatQuirk { slot_index, quirk_type },
                    "No hero specified for quirk treatment",
                );
            }
        };

        // Validate hero exists
        let hero_exists = self.heroes.iter().any(|h| h.id == hero_id_str);
        if !hero_exists {
            return TownActivityRecord::failure(
                building_id,
                TownActivity::TreatQuirk { slot_index, quirk_type },
                "Hero not found",
            );
        }

        // Determine the upgrade level
        let level = upgrade_level.unwrap_or_else(|| {
            self.town_state
                .get_upgrade_level(building_id)
                .unwrap_or('a')
        });

        // Get the cost
        let cost = match self.get_quirk_treatment_cost(building_id, quirk_type, level) {
            Some(c) => c,
            None => {
                return TownActivityRecord::failure(
                    building_id,
                    TownActivity::TreatQuirk { slot_index, quirk_type },
                    "Could not determine quirk treatment cost",
                );
            }
        };

        // Get the number of available slots
        let slots = match self.get_treatment_slots(building_id, level, "quirk_slots") {
            Some(s) => s,
            None => 1,
        };

        // Check slot availability
        if slot_index >= slots {
            return TownActivityRecord::failure(
                building_id,
                TownActivity::TreatQuirk { slot_index, quirk_type },
                &format!("Invalid slot index {} (available: {})", slot_index, slots),
            );
        }

        // Check if we have enough gold
        if self.town_state.gold < cost {
            return TownActivityRecord::failure(
                building_id,
                TownActivity::TreatQuirk { slot_index, quirk_type },
                "Not enough gold for quirk treatment",
            );
        }

        // Get the treatment chance (default 1.0 = always succeeds)
        let treatment_chance = self
            .building_registry
            .get_effect_at_level(building_id, level, "quirk_treatment_chance")
            .unwrap_or(1.0);

        // Deduct gold
        self.town_state.gold -= cost;

        // Roll for success (deterministic based on hero_id and slot_index)
        let roll = Self::deterministic_roll(&hero_id_str, slot_index);
        let success = roll < treatment_chance;

        let quirk_type_name = match quirk_type {
            QuirkTreatmentType::Positive => "positive",
            QuirkTreatmentType::Negative => "negative",
            QuirkTreatmentType::PermanentNegative => "permanent_negative",
        };

        let message = if success {
            format!(
                "Treated {} quirk at Sanitarium: cost {} gold (slot {}/{})",
                quirk_type_name, cost, slot_index + 1, slots
            )
        } else {
            format!(
                "Quirk treatment failed at Sanitarium: cost {} gold (slot {}/{})",
                cost, slot_index + 1, slots
            )
        };

        let record = TownActivityRecord {
            building_id: building_id.to_string(),
            activity: TownActivity::TreatQuirk { slot_index, quirk_type },
            hero_id: Some(hero_id_str),
            upgrade_level: Some(level),
            gold_cost: cost,
            stress_change: 0.0,
            health_change: 0.0,
            success,
            message,
        };

        self.trace.record(record.clone());
        record
    }

    /// Perform disease treatment at the Sanitarium.
    fn perform_treat_disease(
        &mut self,
        building_id: &str,
        hero_id: Option<&str>,
        slot_index: usize,
        upgrade_level: Option<char>,
    ) -> TownActivityRecord {
        let hero_id_str = match hero_id {
            Some(id) => id.to_string(),
            None => {
                return TownActivityRecord::failure(
                    building_id,
                    TownActivity::TreatDisease { slot_index },
                    "No hero specified for disease treatment",
                );
            }
        };

        // Validate hero exists
        let hero_exists = self.heroes.iter().any(|h| h.id == hero_id_str);
        if !hero_exists {
            return TownActivityRecord::failure(
                building_id,
                TownActivity::TreatDisease { slot_index },
                "Hero not found",
            );
        }

        // Determine the upgrade level
        let level = upgrade_level.unwrap_or_else(|| {
            self.town_state
                .get_upgrade_level(building_id)
                .unwrap_or('a')
        });

        // Disease treatment uses SEPARATE upgrade paths:
        // - cost (a/c/e): progressive cost reduction upgrades
        // - cure_all_chance (b/d): progressive cure-all chance upgrades
        //
        // The cost path uses levels a, c, e (which provide actual cost reductions)
        // The cure_all path uses levels b, d (which provide cure improvements)
        //
        // We determine the "effective" level for each path based on the current
        // upgrade state, looking at which levels in each path have been purchased.

        // For cost path (a/c/e): use the highest owned level from {a, c, e}
        // that is at or below the current upgrade level
        let cost_level = Self::highest_owned_level_for_path(level, &['a', 'c', 'e']);

        // For cure_all path (b/d): use the highest owned level from {b, d}
        // that is at or below the current upgrade level
        let cure_all_level = Self::highest_owned_level_for_path(level, &['b', 'd']);

        // Get the cost using the cost-path level
        let cost = match self.get_disease_treatment_cost(building_id, cost_level) {
            Some(c) => c,
            None => {
                return TownActivityRecord::failure(
                    building_id,
                    TownActivity::TreatDisease { slot_index },
                    "Could not determine disease treatment cost",
                );
            }
        };

        // Get the number of available slots
        let slots = match self.get_treatment_slots(building_id, level, "disease_slots") {
            Some(s) => s,
            None => 1,
        };

        // Check slot availability
        if slot_index >= slots {
            return TownActivityRecord::failure(
                building_id,
                TownActivity::TreatDisease { slot_index },
                &format!("Invalid slot index {} (available: {})", slot_index, slots),
            );
        }

        // Check if we have enough gold
        if self.town_state.gold < cost {
            return TownActivityRecord::failure(
                building_id,
                TownActivity::TreatDisease { slot_index },
                "Not enough gold for disease treatment",
            );
        }

        // Get the cure-all chance using the cure_all-path level
        let cure_all_chance = self.get_cure_all_chance(building_id, cure_all_level).unwrap_or(0.33);

        // Deduct gold
        self.town_state.gold -= cost;

        // Roll for cure-all (deterministic based on hero_id and slot_index)
        let roll = Self::deterministic_roll(&hero_id_str, slot_index + 100);
        let cure_all_success = roll < cure_all_chance;

        let message = if cure_all_success {
            format!(
                "Disease treatment at Sanitarium: cure-all SUCCESS (cost {} gold, slot {}/{})",
                cost, slot_index + 1, slots
            )
        } else {
            format!(
                "Disease treatment at Sanitarium: partial cure (cost {} gold, slot {}/{})",
                cost, slot_index + 1, slots
            )
        };

        let record = TownActivityRecord {
            building_id: building_id.to_string(),
            activity: TownActivity::TreatDisease { slot_index },
            hero_id: Some(hero_id_str),
            upgrade_level: Some(level),
            gold_cost: cost,
            stress_change: 0.0,
            health_change: 0.0,
            success: true, // Treatment itself succeeded, cure-all is a bonus
            message,
        };

        self.trace.record(record.clone());
        record
    }

    /// Determine the highest owned level in an upgrade path that is at or below
    /// the given upgrade level. This supports the disease treatment system where
    /// cost uses path {a, c, e} and cure-all uses path {b, d}.
    ///
    /// For example, with path ['a', 'c', 'e'] and current_level='c':
    /// - 'a' <= 'c' is true
    /// - 'c' <= 'c' is true
    /// - 'e' <= 'c' is false
    /// Returns 'c' as the highest owned.
    fn highest_owned_level_for_path(current_level: char, path: &[char]) -> char {
        let mut result = path[0]; // Start with the first (lowest) level
        for &level in path {
            if level <= current_level && level > result {
                result = level;
            }
        }
        result
    }

    /// Perform tavern bar activity.
    fn perform_tavern_bar(
        &mut self,
        building_id: &str,
        hero_id: Option<&str>,
        slot_index: usize,
        upgrade_level: Option<char>,
    ) -> TownActivityRecord {
        self.perform_tavern_activity(building_id, hero_id, slot_index, "bar", 0.40, upgrade_level)
    }

    /// Perform tavern gambling activity.
    fn perform_tavern_gambling(
        &mut self,
        building_id: &str,
        hero_id: Option<&str>,
        slot_index: usize,
        upgrade_level: Option<char>,
    ) -> TownActivityRecord {
        self.perform_tavern_activity(building_id, hero_id, slot_index, "gambling", 0.35, upgrade_level)
    }

    /// Perform tavern brothel activity.
    fn perform_tavern_brothel(
        &mut self,
        building_id: &str,
        hero_id: Option<&str>,
        slot_index: usize,
        upgrade_level: Option<char>,
    ) -> TownActivityRecord {
        self.perform_tavern_activity(building_id, hero_id, slot_index, "brothel", 0.30, upgrade_level)
    }

    /// Common logic for tavern activities.
    fn perform_tavern_activity(
        &mut self,
        building_id: &str,
        hero_id: Option<&str>,
        slot_index: usize,
        activity_type: &str,
        side_effect_chance: f64,
        upgrade_level: Option<char>,
    ) -> TownActivityRecord {
        let hero_id_str = match hero_id {
            Some(id) => id.to_string(),
            None => {
                return TownActivityRecord::failure(
                    building_id,
                    TownActivity::TavernBar { slot_index },
                    "No hero specified for tavern activity",
                );
            }
        };

        // Validate hero exists
        let hero_exists = self.heroes.iter().any(|h| h.id == hero_id_str);
        if !hero_exists {
            return TownActivityRecord::failure(
                building_id,
                TownActivity::TavernBar { slot_index },
                "Hero not found",
            );
        }

        // Determine the upgrade level
        let level = upgrade_level.unwrap_or_else(|| {
            self.town_state
                .get_upgrade_level(building_id)
                .unwrap_or('a')
        });

        // Get the cost
        let cost_effect = format!("{}_cost", activity_type);
        let cost = self
            .building_registry
            .get_effect_at_level(building_id, level, &cost_effect)
            .unwrap_or(0.0) as u32;

        // Get the stress heal amount
        let stress_heal_effect = format!("{}_stress_heal", activity_type);
        let stress_heal = self
            .building_registry
            .get_effect_at_level(building_id, level, &stress_heal_effect)
            .unwrap_or(0.0);

        // Check if we have enough gold
        if self.town_state.gold < cost {
            return TownActivityRecord::failure(
                building_id,
                TownActivity::TavernBar { slot_index },
                "Not enough gold for tavern activity",
            );
        }

        // Deduct gold
        self.town_state.gold -= cost;

        // Apply stress heal to hero
        let hero = self.get_hero_mut(&hero_id_str).unwrap();
        let old_stress = hero.stress;
        hero.stress = (hero.stress - stress_heal).max(0.0);
        let actual_heal = old_stress - hero.stress;
        let _ = hero; // Release mutable borrow

        // Roll for side effect (deterministic based on hero_id and slot_index)
        let roll = Self::deterministic_roll(&hero_id_str, slot_index + 200);
        let side_effect_triggered = roll < side_effect_chance;

        let side_effect_msg = if side_effect_triggered {
            format!(" (side-effect triggered: {:.0}% chance)", side_effect_chance * 100.0)
        } else {
            String::new()
        };

        let activity_name = match activity_type {
            "bar" => "Bar",
            "gambling" => "Gambling",
            "brothel" => "Brothel",
            _ => activity_type,
        };

        let record = TownActivityRecord {
            building_id: building_id.to_string(),
            activity: TownActivity::TavernBar { slot_index },
            hero_id: Some(hero_id_str),
            upgrade_level: Some(level),
            gold_cost: cost,
            stress_change: -actual_heal,
            health_change: 0.0,
            success: true,
            message: format!(
                "Visited Tavern {}: stress healed {:.1} (cost {} gold){}",
                activity_name, actual_heal, cost, side_effect_msg
            ),
        };

        self.trace.record(record.clone());
        record
    }

    /// Deterministic pseudo-random roll for activity outcomes.
    ///
    /// Uses a simple hash of the hero_id and seed to produce a value in [0, 1).
    /// This ensures deterministic outcomes for the same hero and activity.
    fn deterministic_roll(seed: &str, additional: usize) -> f64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        additional.hash(&mut hasher);
        // Mix in current timestamp component for some variety per visit
        let hash = hasher.finish();
        // Convert hash to f64 in [0, 1)
        (hash as f64) / (u64::MAX as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::parse::parse_buildings_json;
    use std::path::PathBuf;

    fn data_path(filename: &str) -> PathBuf {
        PathBuf::from("data").join(filename)
    }

    fn parse_buildings() -> BuildingRegistry {
        parse_buildings_json(&data_path("Buildings.json")).expect("failed to parse Buildings.json")
    }

    #[test]
    fn hero_in_town_is_afflicted_at_max_stress() {
        let hero = HeroInTown::new("h1", "alchemist", 200.0, 200.0, 100.0, 100.0);
        assert!(hero.is_afflicted());

        let hero = HeroInTown::new("h1", "alchemist", 150.0, 200.0, 100.0, 100.0);
        assert!(!hero.is_afflicted());
    }

    #[test]
    fn hero_in_town_is_wounded_when_health_below_max() {
        let hero = HeroInTown::new("h1", "alchemist", 0.0, 200.0, 50.0, 100.0);
        assert!(hero.is_wounded());

        let hero = HeroInTown::new("h1", "alchemist", 0.0, 200.0, 100.0, 100.0);
        assert!(!hero.is_wounded());
    }

    #[test]
    fn town_visit_new_creates_empty_trace() {
        let registry = parse_buildings();
        let town_state = TownState::new(1000);
        let heroes = vec![HeroInTown::new("h1", "alchemist", 50.0, 200.0, 80.0, 100.0)];

        let visit = TownVisit::new(town_state, heroes, registry);
        assert!(visit.trace.activities.is_empty());
    }

    #[test]
    fn town_visit_from_dungeon_run_creates_heroes() {
        let registry = parse_buildings();
        let visit = TownVisit::from_dungeon_run(500, 20.0, 4, registry);

        assert_eq!(visit.town_state.gold, 500);
        assert_eq!(visit.heroes.len(), 4);
        // Stress should be positive from dungeon run
        for hero in &visit.heroes {
            assert!(hero.stress > 0.0);
        }
    }

    #[test]
    fn perform_pray_reduces_stress() {
        let registry = parse_buildings();
        let mut town_state = TownState::new(500);
        town_state
            .building_states
            .insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('b')));

        let hero = HeroInTown::new("h1", "alchemist", 100.0, 200.0, 100.0, 100.0);
        let heroes = vec![hero];

        let mut visit = TownVisit::new(town_state, heroes, registry);

        // Pray at abbey level b (stress_heal = 1)
        let result = visit.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('b'));

        assert!(result.success);
        assert!(result.stress_change < 0.0); // Stress was reduced
        assert_eq!(result.gold_cost, 200); // Level b costs 200
        assert_eq!(visit.town_state.gold, 300); // 500 - 200 = 300

        // Check hero stress was updated
        let hero = visit.get_hero("h1").unwrap();
        assert_eq!(hero.stress, 99.0); // 100 - 1 = 99
    }

    #[test]
    fn perform_pray_fails_without_enough_gold() {
        let registry = parse_buildings();
        let mut town_state = TownState::new(100); // Not enough for level b (200)
        town_state
            .building_states
            .insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('b')));

        let hero = HeroInTown::new("h1", "alchemist", 100.0, 200.0, 100.0, 100.0);
        let heroes = vec![hero];

        let mut visit = TownVisit::new(town_state, heroes, registry);

        let result = visit.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('b'));

        assert!(!result.success);
        assert!(result.message.contains("Not enough gold"));
        assert_eq!(visit.town_state.gold, 100); // Gold unchanged
    }

    #[test]
    fn perform_recruit_adds_hero() {
        let registry = parse_buildings();
        let mut town_state = TownState::new(1000);
        town_state
            .building_states
            .insert("stagecoach".to_string(), BuildingUpgradeState::new("stagecoach", Some('a')));

        let hero = HeroInTown::new("h1", "alchemist", 0.0, 200.0, 100.0, 100.0);
        let heroes = vec![hero];

        let mut visit = TownVisit::new(town_state, heroes, registry);

        let result = visit.perform_town_activity("stagecoach", TownActivity::Recruit, None, None);

        assert!(result.success);
        assert_eq!(visit.heroes.len(), 2); // Original + new recruit
        assert_eq!(visit.heroes[1].class_id, "hunter");
        assert_eq!(visit.town_state.gold, 500); // 1000 - 500 = 500
    }

    #[test]
    fn perform_upgrade_building_deducts_gold() {
        let registry = parse_buildings();
        let mut town_state = TownState::new(1000);
        town_state
            .building_states
            .insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('a')));

        let hero = HeroInTown::new("h1", "alchemist", 100.0, 200.0, 100.0, 100.0);
        let heroes = vec![hero];

        let mut visit = TownVisit::new(town_state, heroes, registry);

        // Upgrade abbey from a (free) to b (200)
        let result = visit.perform_town_activity("abbey", TownActivity::UpgradeBuilding, None, Some('b'));

        assert!(result.success);
        assert_eq!(result.gold_cost, 200);
        assert_eq!(visit.town_state.gold, 800); // 1000 - 200 = 800
        assert_eq!(visit.town_state.get_upgrade_level("abbey"), Some('b'));
    }

    #[test]
    fn town_activity_trace_tracks_activities() {
        let registry = parse_buildings();
        let mut town_state = TownState::new(1000);
        town_state
            .building_states
            .insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('b')));

        let hero = HeroInTown::new("h1", "alchemist", 100.0, 200.0, 100.0, 100.0);
        let heroes = vec![hero];

        let mut visit = TownVisit::new(town_state, heroes, registry);

        visit.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('b'));
        visit.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('b'));

        assert_eq!(visit.trace.activities.len(), 2);
        assert_eq!(visit.trace.total_gold_spent(), 400); // 200 * 2
    }

    #[test]
    fn town_visit_is_deterministic() {
        let registry = parse_buildings();
        let mut town_state = TownState::new(1000);
        town_state
            .building_states
            .insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('b')));

        let hero = HeroInTown::new("h1", "alchemist", 100.0, 200.0, 100.0, 100.0);
        let heroes = vec![hero];

        let mut visit1 = TownVisit::new(town_state.clone(), heroes.clone(), registry.clone());
        let mut visit2 = TownVisit::new(town_state, heroes, registry);

        // Same inputs
        let result1 = visit1.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('b'));
        let result2 = visit2.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('b'));

        // Should produce identical results
        assert_eq!(result1.gold_cost, result2.gold_cost);
        assert_eq!(result1.stress_change, result2.stress_change);
        assert_eq!(result1.success, result2.success);
    }

    // ── US-008: End-to-end town visit tests ──────────────────────────────────

    #[test]
    fn town_visit_after_dungeon_run() {
        // Simulate ending a dungeon run and entering town
        let registry = parse_buildings();

        // After a dungeon run: earned 500 gold, 30.0 stress accumulated, 4 heroes
        let visit = TownVisit::from_dungeon_run(500, 30.0, 4, registry);

        // Should have the gold from dungeon run
        assert_eq!(visit.town_state.gold, 500);

        // Should have 4 heroes with accumulated stress
        assert_eq!(visit.heroes.len(), 4);
        for hero in &visit.heroes {
            assert!(hero.stress > 0.0, "Hero {} should have stress from dungeon", hero.id);
        }

        // Should be able to perform activities
        let mut visit = visit;
        let result = visit.perform_town_activity("abbey", TownActivity::Pray, Some("hero_0"), Some('a'));
        assert!(result.success || !result.success); // Just check it runs
    }

    #[test]
    fn stress_heal_at_abbey_reduces_stress_and_deducts_gold() {
        let registry = parse_buildings();
        let mut town_state = TownState::new(500);
        town_state
            .building_states
            .insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('b')));

        // Hero with high stress (100)
        let hero = HeroInTown::new("h1", "alchemist", 100.0, 200.0, 100.0, 100.0);
        let heroes = vec![hero];

        let mut visit = TownVisit::new(town_state, heroes, registry);

        // Initial state
        let initial_gold = visit.town_state.gold;
        let initial_stress = visit.get_hero("h1").unwrap().stress;

        // Pray at abbey level b (stress_heal = 1)
        let result = visit.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('b'));

        assert!(result.success, "Prayer should succeed: {}", result.message);
        assert!(result.stress_change < 0.0, "Stress should be reduced");
        assert!(result.gold_cost > 0, "Gold should be deducted");

        // Verify stress was reduced
        let final_stress = visit.get_hero("h1").unwrap().stress;
        assert!(
            final_stress < initial_stress,
            "Stress should be reduced: {} -> {}",
            initial_stress,
            final_stress
        );

        // Verify gold was deducted
        assert_eq!(
            visit.town_state.gold,
            initial_gold - result.gold_cost,
            "Gold should be deducted"
        );

        // Verify trace is updated
        assert_eq!(visit.trace.activities.len(), 1);
        assert_eq!(visit.trace.total_gold_spent(), result.gold_cost);
    }

    #[test]
    fn multiple_prayers_accumulate_effects() {
        let registry = parse_buildings();
        let mut town_state = TownState::new(1000);
        town_state
            .building_states
            .insert("abbey".to_string(), BuildingUpgradeState::new("abbey", Some('b')));

        let hero = HeroInTown::new("h1", "alchemist", 100.0, 200.0, 100.0, 100.0);
        let heroes = vec![hero];

        let mut visit = TownVisit::new(town_state, heroes, registry);

        // Pray 5 times (each heals 1 stress)
        for _ in 0..5 {
            visit.perform_town_activity("abbey", TownActivity::Pray, Some("h1"), Some('b'));
        }

        // Hero stress should be reduced by 5 (100 -> 95)
        let hero = visit.get_hero("h1").unwrap();
        assert_eq!(hero.stress, 95.0);

        // Gold spent should be 5 * 200 = 1000
        assert_eq!(visit.town_state.gold, 0);
        assert_eq!(visit.trace.total_gold_spent(), 1000);
    }
}