//! DDGC adapters — transforms framework payloads into DDGC view models.
//!
//! This module provides adapters that convert framework payloads
//! (from `framework_viewmodels`, `framework_progression`, and `framework_combat`)
//! into DDGC-specific view models defined in [`super::viewmodels`].
//!
//! # Design principles
//!
//! - Adapters are **pure conversion functions** — no side effects, no state mutation.
//! - Each adapter takes a specific payload type and returns a [`ViewModelResult`]
//!   containing either the shaped view model or a [`ViewModelError`].
//! - Unsupported or partially-mapped states produce explicit error surfaces
//!   rather than silently returning partial data.
//! - Adapter logic remains **product-owned** and is not pushed down into `WorldEngine`.
//!
//! # Supported conversions
//!
//! | Payload type | View model | Function |
//! |---|---|---|
//! | `DdgcHost` + `HostPhase` | `BootLoadViewModel` | [`boot_load_from_host`] |
//! | `CampaignState` | `TownViewModel` | [`town_from_campaign`] |
//! | `CampaignState` + hero ID | `HeroDetailViewModel` | [`hero_detail_from_campaign`] |
//! | `CampaignState` + building ID | `BuildingDetailViewModel` | [`building_detail_from_campaign`] |
//! | `CampaignState` + building ID + registry | `BuildingEntryViewModel` | [`building_entry_from_campaign`] |
//! | `CampaignState` + `BuildingActionRequest` | `BuildingActionResult` | [`execute_building_action`] |
//! | `CampaignState` | building action status map | [`all_building_actions_status`] |
//! | `DdgcRunResult` | `DungeonViewModel` | [`dungeon_from_run_result`] |
//! | `DdgcRunResult` | `ExplorationHudViewModel` | [`exploration_hud_from_dungeon`] |
//! | `DdgcRunResult` + room index | `RoomMovementViewModel` | [`room_movement_from_run`] |
//! | `DdgcRunResult` + room index | `EncounterEntryViewModel` | [`encounter_entry_from_run`] |
//! | `framework_viewmodels::CombatViewModel` | `CombatViewModel` | [`combat_from_framework`] |
//! | `CombatViewModel` | `CombatHudViewModel` | [`combat_hud_from_combat`] |
//! | `RunResult` + rewards | `ResultViewModel` | [`result_from_run`] |
//! | `CampaignState` + selection | `ProvisioningViewModel` | [`provisioning_from_campaign`] |
//! | `CampaignState` + selection + hero | selected hero IDs | [`provisioning_hero_selection`] |
//! | `CampaignState` + selection + quest | `ExpeditionSetupViewModel` | [`expedition_setup_from_data`] |
//! | `CampaignState` + `ExpeditionLaunchRequest` | `ExpeditionLaunchResult` | [`expedition_launch`] |
//! | `DdgcRunState` + heroes | `ReturnFlowViewModel` | [`return_flow_from_state`] |

use crate::contracts::viewmodels::{
    BuildingAction, BuildingDetailViewModel, BootLoadViewModel, CombatHudViewModel,
    CombatPhase, CombatViewModel, CombatantType, CombatantViewModel, CombatPosition,
    CombatantVitalViewModel, DungeonHeroViewModel, DungeonRoomKind, DungeonRoomViewModel,
    DungeonViewModel, EncounterEntryViewModel, EncounterHeroViewModel, EncounterType,
    ExplorationHudViewModel, HeroDetailViewModel, HeroProgression, HeroResistances,
    HeroVitalViewModel, InteractionType, RoomMovementViewModel, ViewModelResult,
};
use crate::contracts::{
    CampaignState, DungeonType, HeirloomCurrency, MapSize,
};

/// Adapter: Convert `DdgcHost` state to `BootLoadViewModel`.
///
/// Takes the host phase and a flag indicating whether campaign was loaded
/// to produce the boot/load view model.
pub fn boot_load_from_host(
    host_phase: &crate::contracts::host::HostPhase,
    campaign_loaded: bool,
    campaign_schema_version: Option<u32>,
) -> ViewModelResult<BootLoadViewModel> {
    match host_phase {
        crate::contracts::host::HostPhase::Uninitialized => {
            Ok(BootLoadViewModel::success("Initialized and ready to boot", vec![]))
        }
        crate::contracts::host::HostPhase::Booting => {
            Ok(BootLoadViewModel::success("Loading contract packages...", vec![]))
        }
        crate::contracts::host::HostPhase::Ready => {
            let mut vm = BootLoadViewModel::success("Host ready", vec![]);
            if campaign_loaded {
                vm.status_message = "Campaign loaded successfully".to_string();
            }
            if let Some(version) = campaign_schema_version {
                vm = vm.with_campaign_version(version);
            }
            Ok(vm)
        }
        crate::contracts::host::HostPhase::FatalError => {
            Ok(BootLoadViewModel::failure("Fatal error during boot"))
        }
        crate::contracts::host::HostPhase::Unsupported => {
            Ok(BootLoadViewModel::failure("Feature not supported in this build"))
        }
    }
}

/// Adapter: Convert `CampaignState` to `TownViewModel`.
///
/// Takes the campaign state and produces a town visit view model
/// with buildings, roster, and available activities.
pub fn town_from_campaign(
    campaign: &CampaignState,
) -> ViewModelResult<crate::contracts::viewmodels::TownViewModel> {
    use crate::contracts::viewmodels::{TownActivityType, TownBuildingViewModel, TownHeroViewModel};

    let roster: Vec<TownHeroViewModel> = campaign
        .roster
        .iter()
        .map(|hero| {
            let is_wounded = hero.health < hero.max_health;
            let is_afflicted = hero.stress >= hero.max_stress;

            TownHeroViewModel {
                id: hero.id.clone(),
                name: hero.id.clone(), // Placeholder: use id as display name
                class_id: hero.class_id.clone(),
                class_name: hero.class_id.clone(),
                health: hero.health,
                max_health: hero.max_health,
                stress: hero.stress,
                max_stress: hero.max_stress,
                is_wounded,
                is_afflicted,
                level: hero.level,
                xp: hero.xp,
                positive_quirks: hero.quirks.positive.clone(),
                negative_quirks: hero.quirks.negative.clone(),
                diseases: hero.quirks.diseases.clone(),
            }
        })
        .collect();

    // Map building states to view models
    let buildings: Vec<TownBuildingViewModel> = campaign
        .building_states
        .keys()
        .map(|building_id| {
            let upgrade_state = campaign.building_states.get(building_id);
            TownBuildingViewModel {
                id: building_id.clone(),
                building_type: building_id.clone(),
                current_upgrade: upgrade_state.and_then(|s| s.current_level),
                available: true,
            }
        })
        .collect();

    // Determine available activities based on buildings
    let mut available_activities = Vec::new();
    for building_id in campaign.building_states.keys() {
        match building_id.as_str() {
            "stagecoach" => available_activities.push(TownActivityType::Stagecoach),
            "guild" => available_activities.push(TownActivityType::Guild),
            "blacksmith" => available_activities.push(TownActivityType::Blacksmith),
            "sanitarium" => available_activities.push(TownActivityType::Sanitarium),
            "tavern" => available_activities.push(TownActivityType::Tavern),
            "abbey" => available_activities.push(TownActivityType::Abbey),
            "campfire" => available_activities.push(TownActivityType::Camping),
            _ => {}
        }
    }

    // Convert heirlooms to string keys
    let heirlooms: std::collections::BTreeMap<String, u32> = campaign
        .heirlooms
        .iter()
        .map(|(k, v)| (format!("{:?}", k).to_lowercase(), *v))
        .collect();

    Ok(crate::contracts::viewmodels::TownViewModel {
        kind: "town".to_string(),
        title: "Town Surface".to_string(),
        campaign_name: "Campaign".to_string(),
        campaign_summary: "Town visit with roster and building access.".to_string(),
        gold: campaign.gold,
        heirlooms,
        buildings,
        heroes: roster.clone(),
        roster,
        available_activities,
        next_action_label: "Provision Expedition".to_string(),
        is_fresh_visit: true,
        error: None,
    })
}

/// Adapter: Convert `CampaignState` and hero ID to `HeroDetailViewModel`.
///
/// Takes the campaign state and a hero ID to produce a detailed hero view model
/// for inspection by the player when making campaign decisions.
pub fn hero_detail_from_campaign(
    campaign: &CampaignState,
    hero_id: &str,
) -> ViewModelResult<HeroDetailViewModel> {
    let hero = campaign
        .roster
        .iter()
        .find(|h| h.id == hero_id)
        .ok_or_else(|| crate::contracts::viewmodels::ViewModelError::MissingRequiredField {
            field: "hero_id".to_string(),
            context: format!("hero '{}' not found in roster", hero_id),
        })?;

    let hp = format!("{}", hero.health as u32);
    let max_hp = format!("{}", hero.max_health as u32);
    let stress = format!("{}", hero.stress as u32);
    let resolve = format!("{}", hero.level);

    // Calculate experience to next level (placeholder formula)
    let xp_for_next = hero.level * 200;
    let experience = format!("{}", hero.xp);
    let experience_to_next = format!("{}", xp_for_next);

    Ok(HeroDetailViewModel {
        kind: "hero-detail".to_string(),
        hero_id: hero.id.clone(),
        name: hero.id.clone(), // Placeholder: use id as display name
        class_label: hero.class_id.clone(),
        hp,
        max_hp,
        stress,
        resolve,
        progression: HeroProgression {
            level: hero.level,
            experience,
            experience_to_next,
        },
        resistances: HeroResistances {
            stun: "50%".to_string(),
            bleed: "50%".to_string(),
            disease: "50%".to_string(),
            mov: "50%".to_string(),
            death: "0%".to_string(),
            trap: "50%".to_string(),
            hazard: "50%".to_string(),
        },
        combat_skills: hero.skills.clone(),
        camping_skills: Vec::new(), // Placeholder: camping skills not yet implemented
        weapon: format!("{} (+0)", hero.class_id),
        armor: format!("{} (+0)", hero.class_id),
        camp_notes: "Hero detail view - campaign state derived.".to_string(),
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Provisioning Adapters — party selection and expedition provisioning
// ─────────────────────────────────────────────────────────────────────────────

/// Adapter: Convert `CampaignState` to `ProvisioningViewModel`.
///
/// This adapter creates the provisioning view model from campaign state,
/// presenting the hero roster for party selection before expedition launch.
/// Each hero is shown with vitals and a selection status for frontend rendering.
///
/// The provisioning view model represents the state when the player is
/// selecting heroes for an expedition and reviewing supply levels.
pub fn provisioning_from_campaign(
    campaign: &CampaignState,
    selected_hero_ids: &[String],
    expedition_label: &str,
    expedition_summary: &str,
) -> ViewModelResult<crate::contracts::viewmodels::ProvisioningViewModel> {
    use crate::contracts::viewmodels::ProvisioningHeroSummary;

    let party: Vec<ProvisioningHeroSummary> = campaign
        .roster
        .iter()
        .map(|hero| {
            let hp = format!("{:.0} / {:.0}", hero.health, hero.max_health);
            let max_hp = format!("{:.0}", hero.max_health);
            let stress = format!("{:.0} / {:.0}", hero.stress, hero.max_stress);
            let max_stress = format!("{:.0}", hero.max_stress);
            let is_wounded = hero.health < hero.max_health;
            let is_afflicted = hero.stress >= hero.max_stress;
            let is_selected = selected_hero_ids.contains(&hero.id);

            ProvisioningHeroSummary {
                id: hero.id.clone(),
                name: hero.id.clone(), // Placeholder: use id as display name
                class_label: hero.class_id.clone(),
                hp,
                max_hp,
                health: hero.health,
                max_health: hero.max_health,
                stress,
                max_stress: max_stress,
                level: hero.level,
                xp: hero.xp,
                is_wounded,
                is_afflicted,
                is_selected,
            }
        })
        .collect();

    let max_party_size = 4;
    let selected_count = selected_hero_ids.len() as u32;
    let is_ready_to_launch = selected_count > 0 && selected_count <= max_party_size;

    // Basic supply cost derived from party size
    let supply_level = if selected_count == 0 {
        "None".to_string()
    } else if selected_count <= 2 {
        "Minimal".to_string()
    } else {
        "Adequate".to_string()
    };

    let provision_cost = format!("{} Gold", selected_count * 50);

    Ok(crate::contracts::viewmodels::ProvisioningViewModel {
        kind: "provisioning".to_string(),
        title: "Provision Expedition".to_string(),
        campaign_name: "Campaign".to_string(),
        expedition_label: expedition_label.to_string(),
        expedition_summary: expedition_summary.to_string(),
        party,
        max_party_size,
        is_ready_to_launch,
        supply_level,
        provision_cost,
    })
}

/// Adapter: Compute updated `ProvisioningViewModel` after toggling hero selection.
///
/// This adapter handles the hero selection toggle during provisioning.
/// Given the campaign state and a hero ID to toggle, it returns a new
/// set of selected hero IDs (adding or removing as appropriate) that can
/// be passed back to `provisioning_from_campaign` for a fresh view model.
///
/// The runtime validates:
/// - The hero exists in the roster
/// - The selection stays within the party size limit
pub fn provisioning_hero_selection(
    campaign: &CampaignState,
    current_selection: &[String],
    hero_id: &str,
) -> ViewModelResult<Vec<String>> {
    // Validate the hero exists in the roster
    if !campaign.roster.iter().any(|h| h.id == hero_id) {
        return Err(crate::contracts::viewmodels::ViewModelError::MissingRequiredField {
            field: "hero_id".to_string(),
            context: format!("hero '{}' not found in roster for provisioning selection", hero_id),
        });
    }

    let mut updated = current_selection.to_vec();

    if updated.contains(&hero_id.to_string()) {
        // Deselect the hero
        updated.retain(|id| id != hero_id);
    } else {
        // Select the hero (up to max party size)
        if updated.len() >= 4 {
            return Err(crate::contracts::viewmodels::ViewModelError::UnsupportedState {
                state_type: "ProvisioningSelection".to_string(),
                detail: format!("cannot select more than 4 heroes for expedition party"),
            });
        }
        updated.push(hero_id.to_string());
    }

    Ok(updated)
}

// ─────────────────────────────────────────────────────────────────────────────
// Expedition Setup and Launch Adapters — pre-launch review and expedition entry
// ─────────────────────────────────────────────────────────────────────────────

/// Adapter: Convert selected heroes and quest info to `ExpeditionSetupViewModel`.
///
/// This adapter creates the expedition setup view model from the campaign state
/// and selected party heroes. It includes party vitals, expedition parameters,
/// objectives, warnings based on party condition, and provisioning status.
///
/// The resulting view model represents the final review state before the player
/// confirms expedition launch.
pub fn expedition_setup_from_data(
    campaign: &CampaignState,
    selected_hero_ids: &[String],
    quest: Option<&crate::contracts::QuestDefinition>,
    supply_level: &str,
    provision_cost: &str,
) -> ViewModelResult<crate::contracts::viewmodels::ExpeditionSetupViewModel> {
    use crate::contracts::viewmodels::ExpeditionHeroSummary;

    let expedition_name = quest
        .map(|q| q.quest_id.clone())
        .unwrap_or_else(|| "Expedition".to_string());

    let difficulty = quest
        .map(|q| match q.difficulty {
            crate::contracts::QuestDifficulty::Standard => "Standard",
            crate::contracts::QuestDifficulty::Hard => "Hard",
        })
        .unwrap_or("Standard");

    let estimated_duration = quest
        .map(|q| match q.map_size {
            crate::contracts::MapSize::Short => "Short",
            crate::contracts::MapSize::Medium => "Medium",
        })
        .unwrap_or("Short");

    let party: Vec<ExpeditionHeroSummary> = campaign
        .roster
        .iter()
        .filter(|hero| selected_hero_ids.contains(&hero.id))
        .map(|hero| {
            let hp = format!("{:.0} / {:.0}", hero.health, hero.max_health);
            let max_hp = format!("{:.0}", hero.max_health);
            let stress = format!("{:.0} / {:.0}", hero.stress, hero.max_stress);
            let max_stress = format!("{:.0}", hero.max_stress);

            ExpeditionHeroSummary {
                id: hero.id.clone(),
                name: hero.id.clone(), // Placeholder: use id as display name
                class_label: hero.class_id.clone(),
                hp,
                max_hp,
                stress,
                max_stress,
            }
        })
        .collect();

    let party_size = party.len() as u32;
    let objectives = quest
        .map(|q| vec![format!("{:?} — {:?}", q.quest_type, q.map_size)])
        .unwrap_or_else(|| vec!["Explore the dungeon".to_string()]);

    // Generate warnings based on party condition
    let mut warnings: Vec<String> = Vec::new();
    for hero in &campaign.roster {
        if selected_hero_ids.contains(&hero.id) {
            if hero.health < hero.max_health * 0.5 {
                warnings.push(format!(
                    "{} is severely wounded ({:.0}/{:.0} HP)",
                    hero.id, hero.health, hero.max_health
                ));
            } else if hero.health < hero.max_health {
                warnings.push(format!(
                    "{} is wounded ({:.0}/{:.0} HP)",
                    hero.id, hero.health, hero.max_health
                ));
            }
            if hero.stress >= hero.max_stress {
                warnings.push(format!(
                    "{} is afflicted (stress at {:.0})",
                    hero.id, hero.stress
                ));
            } else if hero.stress > hero.max_stress * 0.75 {
                warnings.push(format!(
                    "{} has high stress ({:.0}/{:.0})",
                    hero.id, hero.stress, hero.max_stress
                ));
            }
        }
    }

    Ok(crate::contracts::viewmodels::ExpeditionSetupViewModel {
        kind: "expedition".to_string(),
        title: "Expedition Review".to_string(),
        expedition_name,
        party_size,
        party,
        difficulty: difficulty.to_string(),
        estimated_duration: estimated_duration.to_string(),
        objectives,
        warnings,
        supply_level: supply_level.to_string(),
        provision_cost: provision_cost.to_string(),
        is_launchable: party_size > 0 && party_size <= 4,
    })
}

/// Adapter: Process an expedition launch request against campaign state.
///
/// This is the runtime contract for launching an expedition. It validates:
/// - The selected heroes exist in the roster
/// - At least one hero is selected
/// - No more than 4 heroes are selected (party size limit)
/// - The campaign has sufficient gold for provisioning
///
/// On success, it returns an `ExpeditionLaunchResult` with the transition
/// details, including the next runtime state ("dungeon") and the gold cost.
pub fn expedition_launch(
    campaign: &CampaignState,
    request: &crate::contracts::viewmodels::ExpeditionLaunchRequest,
) -> crate::contracts::viewmodels::ExpeditionLaunchResult {
    use crate::contracts::viewmodels::ExpeditionLaunchResult;

    // Validate at least one hero is selected
    if request.selected_hero_ids.is_empty() {
        return ExpeditionLaunchResult::failure(
            "No heroes selected for expedition",
            crate::contracts::viewmodels::ViewModelError::MissingRequiredField {
                field: "selected_hero_ids".to_string(),
                context: "at least one hero must be selected for expedition launch".to_string(),
            },
        );
    }

    // Validate party size limit
    if request.selected_hero_ids.len() > 4 {
        return ExpeditionLaunchResult::failure(
            &format!("Party size {} exceeds maximum of 4", request.selected_hero_ids.len()),
            crate::contracts::viewmodels::ViewModelError::UnsupportedState {
                state_type: "ExpeditionLaunch".to_string(),
                detail: format!("party size {} exceeds maximum of 4", request.selected_hero_ids.len()),
            },
        );
    }

    // Validate all selected heroes exist in the roster
    for hero_id in &request.selected_hero_ids {
        if !campaign.roster.iter().any(|h| h.id == *hero_id) {
            return ExpeditionLaunchResult::failure(
                &format!("Hero '{}' not found in campaign roster", hero_id),
                crate::contracts::viewmodels::ViewModelError::MissingRequiredField {
                    field: "hero_id".to_string(),
                    context: format!("hero '{}' not found in roster for expedition launch", hero_id),
                },
            );
        }
    }

    // Calculate provisioning cost based on party size
    let selected_count = request.selected_hero_ids.len() as u32;
    let gold_cost = selected_count * 50;

    // Check if campaign has enough gold
    if campaign.gold < gold_cost {
        return ExpeditionLaunchResult::failure(
            &format!(
                "Not enough gold for provisioning: need {}, have {}",
                gold_cost, campaign.gold
            ),
            crate::contracts::viewmodels::ViewModelError::MissingRequiredField {
                field: "gold".to_string(),
                context: format!("need {} gold for provisioning, have {}", gold_cost, campaign.gold),
            },
        );
    }

    // Determine dungeon type and map size from quest if available
    let (dungeon_type, map_size): (Option<String>, Option<String>) = if request.quest_id.is_some() {
        // Quest-based expedition — default to QingLong/Short when no specific quest data loaded
        (Some("QingLong".to_string()), Some("Short".to_string()))
    } else {
        (None, None)
    };

    let expedition_name = request
        .quest_id
        .as_ref()
        .map(|qid| qid.clone())
        .unwrap_or_else(|| "Expedition".to_string());

    ExpeditionLaunchResult::success(
        &format!(
            "Expedition launched with {} hero{} for {} gold",
            selected_count,
            if selected_count == 1 { "" } else { "es" },
            gold_cost
        ),
        &expedition_name,
        request.selected_hero_ids.clone(),
        request.quest_id.clone(),
        gold_cost,
        dungeon_type,
        map_size,
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// Building Entry and Action Execution Adapters — runtime contracts
// ─────────────────────────────────────────────────────────────────────────────

/// Adapter: Convert `CampaignState` and building ID to `BuildingEntryViewModel`.
///
/// This adapter creates the full building entry view model when a player
/// enters a building. It is a superset of `building_detail_from_campaign` that
/// includes current gold, current upgrade level, and upgrade level display info.
///
/// The resulting view model represents a player-facing building entry event
/// with all available actions, costs, and state information needed by the
/// screens layer to render the building interaction surface.
pub fn building_entry_from_campaign(
    campaign: &CampaignState,
    building_id: &str,
    registry: Option<&crate::contracts::BuildingRegistry>,
) -> ViewModelResult<crate::contracts::viewmodels::BuildingEntryViewModel> {
    use crate::contracts::viewmodels::BuildingStatus;

    // Check if the building exists in the campaign
    let building_state = campaign
        .building_states
        .get(building_id)
        .ok_or_else(|| crate::contracts::viewmodels::ViewModelError::MissingRequiredField {
            field: "building_id".to_string(),
            context: format!("building '{}' not found in campaign state for entry", building_id),
        })?;

    // Determine building status
    let status = match building_state.current_level {
        Some(_) => BuildingStatus::Ready,
        None => BuildingStatus::Locked,
    };

    // Get building label and description
    let (label, description) = building_label_and_description(building_id);

    // Generate actions based on building type
    let actions = generate_building_actions(building_id, &status, campaign.gold, building_state.current_level);

    // Get upgrade requirement hint
    let upgrade_requirement = building_upgrade_hint(building_id);

    // Build upgrade level display list from registry if available
    let current_level = building_state.current_level;
    let upgrade_levels = if let Some(reg) = registry {
        build_upgrade_level_display(reg, building_id, current_level)
    } else {
        Vec::new()
    };

    Ok(crate::contracts::viewmodels::BuildingEntryViewModel {
        kind: "building-entry".to_string(),
        building_id: building_id.to_string(),
        label,
        status,
        description,
        actions,
        current_gold: campaign.gold,
        upgrade_requirement,
        current_upgrade_level: current_level,
        upgrade_levels,
        error: None,
    })
}

/// Build upgrade level display info from a building registry.
///
/// Iterates all upgrade trees for the given building and produces
/// a sorted list of upgrade levels with cost and effects info.
fn build_upgrade_level_display(
    registry: &crate::contracts::BuildingRegistry,
    building_id: &str,
    current_level: Option<char>,
) -> Vec<crate::contracts::viewmodels::UpgradeLevelDisplay> {
    use crate::contracts::viewmodels::UpgradeLevelDisplay;

    let levels = match registry.get_upgrade_levels(building_id) {
        Some(l) => l,
        None => return Vec::new(),
    };

    levels
        .iter()
        .map(|level| {
            let is_owned = current_level.map_or(false, |cl| level.code <= cl);
            let effects_summary = if level.effects.is_empty() {
                "Free (starting level)".to_string()
            } else {
                level
                    .effects
                    .iter()
                    .map(|e| format!("{}: {:.2}", e.effect_id, e.value))
                    .collect::<Vec<_>>()
                    .join(", ")
            };

            UpgradeLevelDisplay {
                code: level.code,
                cost: level.cost,
                is_owned,
                effects_summary,
            }
        })
        .collect()
}

/// Adapter: Execute a building action request against campaign state.
///
/// This is the runtime contract for performing a building action.
/// It validates the request against current campaign state (gold, building status)
/// and returns a `BuildingActionResult` with the outcome.
///
/// Currently this is a validation-only adapter that checks:
/// - Building exists and is in Ready state
/// - Action is defined for this building type
/// - Player has sufficient gold
///
/// Full action execution (stress heal, health recovery, quirk treatment, etc.)
/// is handled by the `TownVisit` runtime in `crate::town::TownVisit`.
pub fn execute_building_action(
    campaign: &CampaignState,
    request: &crate::contracts::viewmodels::BuildingActionRequest,
) -> crate::contracts::viewmodels::BuildingActionResult {
    use crate::contracts::viewmodels::BuildingActionResult;

    // Check if the building exists
    let building_state = match campaign.building_states.get(&request.building_id) {
        Some(s) => s,
        None => {
            return BuildingActionResult::failure(
                &format!("Building '{}' not found", request.building_id),
                crate::contracts::viewmodels::ViewModelError::MissingRequiredField {
                    field: "building_id".to_string(),
                    context: format!("building '{}' not found for action '{}'", request.building_id, request.action_id),
                },
            );
        }
    };

    // Check if the building is unlocked
    if building_state.current_level.is_none() {
        return BuildingActionResult::failure(
            &format!("Building '{}' is locked", request.building_id),
            crate::contracts::viewmodels::ViewModelError::UnsupportedState {
                state_type: "BuildingInteraction".to_string(),
                detail: format!("building '{}' is locked", request.building_id),
            },
        );
    }

    // Get building status for action generation
    let status = match building_state.current_level {
        Some(_) => crate::contracts::viewmodels::BuildingStatus::Ready,
        None => crate::contracts::viewmodels::BuildingStatus::Locked,
    };

    // Generate actions and find the requested one
    let actions = generate_building_actions(&request.building_id, &status, campaign.gold, building_state.current_level);
    let action = match actions.iter().find(|a| a.id == request.action_id) {
        Some(a) => a,
        None => {
            return BuildingActionResult::failure(
                &format!("Action '{}' not available at '{}'", request.action_id, request.building_id),
                crate::contracts::viewmodels::ViewModelError::UnsupportedState {
                    state_type: "BuildingAction".to_string(),
                    detail: format!("action '{}' not defined for building '{}'", request.action_id, request.building_id),
                },
            );
        }
    };

    // Check if the action is unsupported
    if action.is_unsupported {
        return BuildingActionResult::failure(
            &format!("Action '{}' is not supported in the current build", action.label),
            crate::contracts::viewmodels::ViewModelError::UnsupportedState {
                state_type: "BuildingAction".to_string(),
                detail: format!("action '{}' at '{}' is unsupported", request.action_id, request.building_id),
            },
        );
    }

    // Check if player has enough gold
    let cost_numeric: u32 = action
        .cost
        .split_whitespace()
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    if cost_numeric > 0 && campaign.gold < cost_numeric {
        return BuildingActionResult::failure(
            &format!("Not enough gold for '{}': need {} gold, have {}", action.label, cost_numeric, campaign.gold),
            crate::contracts::viewmodels::ViewModelError::MissingRequiredField {
                field: "gold".to_string(),
                context: format!("need {} gold for '{}', have {}", cost_numeric, action.label, campaign.gold),
            },
        );
    }

    // Success — action is validated
    BuildingActionResult::success(
        &format!("Action '{}' completed successfully", action.label),
        -(cost_numeric as i32),
        0.0,
        0.0,
    )
}

/// Adapter: Convert `CampaignState` to a list of `BuildingAction` validation statuses.
///
/// This provides a quick overview of all actions across all buildings,
/// indicating which are available and which have prerequisites or are unsupported.
/// Useful for the frontend to show building summaries on the town map.
pub fn all_building_actions_status(
    campaign: &CampaignState,
) -> std::collections::BTreeMap<String, Vec<crate::contracts::viewmodels::BuildingAction>> {
    let mut result: std::collections::BTreeMap<String, Vec<crate::contracts::viewmodels::BuildingAction>> =
        std::collections::BTreeMap::new();

    for (building_id, building_state) in &campaign.building_states {
        let status = match building_state.current_level {
            Some(_) => crate::contracts::viewmodels::BuildingStatus::Ready,
            None => crate::contracts::viewmodels::BuildingStatus::Locked,
        };
        let actions = generate_building_actions(building_id, &status, campaign.gold, building_state.current_level);
        result.insert(building_id.clone(), actions);
    }

    result
}

/// Adapter: Convert `CampaignState` to `BuildingDetailViewModel`.
///
/// Takes the campaign state and a building ID to produce a detailed building view model
/// for inspection by the player when interacting with town buildings.
pub fn building_detail_from_campaign(
    campaign: &CampaignState,
    building_id: &str,
) -> ViewModelResult<BuildingDetailViewModel> {
    use crate::contracts::viewmodels::BuildingStatus;

    // Check if the building exists in the campaign
    let building_state = campaign
        .building_states
        .get(building_id)
        .ok_or_else(|| crate::contracts::viewmodels::ViewModelError::MissingRequiredField {
            field: "building_id".to_string(),
            context: format!("building '{}' not found in campaign state", building_id),
        })?;

    // Determine building status based on upgrade level
    // Level >= 'a' means Ready (building is accessible)
    // Level 'a' or higher means the building has been initialized
    // Locked only when level is None
    let status = match building_state.current_level {
        Some(_) => BuildingStatus::Ready,
        None => BuildingStatus::Locked,
    };

    // Generate building label and description based on building_id
    let (label, description) = building_label_and_description(building_id);

    // Generate actions based on building type
    let actions = generate_building_actions(building_id, &status, campaign.gold, building_state.current_level);

    // Determine upgrade requirement
    let upgrade_requirement = building_upgrade_hint(building_id);

    Ok(BuildingDetailViewModel {
        kind: "building-detail".to_string(),
        building_id: building_id.to_string(),
        label,
        status,
        description,
        actions,
        upgrade_requirement,
    })
}

/// Get the label and description for a building by ID.
fn building_label_and_description(building_id: &str) -> (String, String) {
    match building_id {
        "stagecoach" => (
            "Stagecoach".to_string(),
            "The stagecoach offers new recruits from the surrounding region. Recruit heroes to expand your party roster.".to_string(),
        ),
        "guild" => (
            "Guild".to_string(),
            "The guild provides skill training and party capability review. Upgrade your heroes' abilities.".to_string(),
        ),
        "blacksmith" => (
            "Blacksmith".to_string(),
            "The blacksmith crafts and repairs weapons and armor. Enhance your party's equipment.".to_string(),
        ),
        "sanitarium" => (
            "Sanitarium".to_string(),
            "The sanitarium treats hero quirks, diseases, and ailments. Restore heroes to full health.".to_string(),
        ),
        "tavern" => (
            "Tavern".to_string(),
            "The tavern provides food, drink, and entertainment. Reduce stress and boost morale.".to_string(),
        ),
        "abbey" => (
            "Abbey".to_string(),
            "The abbey offers spiritual respite. Reduce hero stress through prayer and meditation.".to_string(),
        ),
        "inn" => (
            "Inn".to_string(),
            "The inn offers lodging and meals. Heroes can rest and recover here.".to_string(),
        ),
        "graveyard" => (
            "Graveyard".to_string(),
            "The graveyard honors fallen heroes. Pay respects and manage deceased roster members.".to_string(),
        ),
        "museum" => (
            "Museum".to_string(),
            "The museum displays rare artifacts and trinkets collected from expeditions. Catalogue your discoveries.".to_string(),
        ),
        "provisioner" => (
            "Provisioner".to_string(),
            "The provisioner stocks supplies for expeditions. Purchase food, torches, and medicinal herbs.".to_string(),
        ),
        "sanctuary" => (
            "Sanctuary".to_string(),
            "The sanctuary offers advanced treatment for hero ailments beyond the sanitarium's capabilities.".to_string(),
        ),
        "campfire" => (
            "Campfire".to_string(),
            "The campfire provides a place to rest during expeditions. Camp to heal and buff heroes.".to_string(),
        ),
        _ => (
            format!("Building: {}", building_id),
            format!("Town building '{}' - detailed interactions to be implemented.", building_id),
        ),
    }
}

/// Get a hint string about the next upgrade for a building.
fn building_upgrade_hint(building_id: &str) -> Option<String> {
    match building_id {
        "stagecoach" => Some("Upgrade to expand recruit pool and reduce costs.".to_string()),
        "guild" => Some("Upgrade to unlock advanced training and skill upgrades.".to_string()),
        "blacksmith" => Some("Upgrade to improve equipment discounts and repair efficiency.".to_string()),
        "sanitarium" => Some("Upgrade to unlock additional treatment slots and reduce costs.".to_string()),
        "tavern" => Some("Upgrade to improve stress healing and unlock new activities.".to_string()),
        "abbey" => Some("Upgrade to deepen spiritual healing and unlock meditation.".to_string()),
        "inn" => Some("Upgrade to improve rest quality and food options.".to_string()),
        "graveyard" => Some("Upgrade to unlock memorial ceremonies and hero recovery.".to_string()),
        "museum" => Some("Upgrade to expand display capacity and artifact appraisal.".to_string()),
        "provisioner" => Some("Upgrade to expand supply stock and reduce expedition costs.".to_string()),
        "sanctuary" => Some("Upgrade to unlock advanced quirk and disease treatment.".to_string()),
        _ => None,
    }
}

/// Generate building actions based on building type and status.
///
/// This function produces player-facing action affordances for each building type.
/// Actions include costs, availability (based on gold and building status), and
/// unsupported flags for features not yet implemented in the current build.
///
/// Currently covered building types (11 total from registry):
/// - Primary: stagecoach, guild, blacksmith, sanitarium, tavern, abbey
/// - Secondary: inn, graveyard, museum, provisioner, sanctuary
/// - Special: campfire
fn generate_building_actions(
    building_id: &str,
    status: &crate::contracts::viewmodels::BuildingStatus,
    current_gold: u32,
    current_level: Option<char>,
) -> Vec<BuildingAction> {
    let is_ready = matches!(status, crate::contracts::viewmodels::BuildingStatus::Ready);

    match building_id {
        "stagecoach" => vec![
            BuildingAction {
                id: "recruit-hero".to_string(),
                label: "Recruit Hero".to_string(),
                description: "Recruit a new hero to your party from available candidates.".to_string(),
                cost: "500 Gold".to_string(),
                is_available: is_ready && current_gold >= 500,
                is_unsupported: false,
            },
            BuildingAction {
                id: "view-candidates".to_string(),
                label: "View Candidates".to_string(),
                description: "Browse available hero candidates without recruiting.".to_string(),
                cost: "Free".to_string(),
                is_available: is_ready,
                is_unsupported: false,
            },
            BuildingAction {
                id: "rare-recruit".to_string(),
                label: "Rare Recruit".to_string(),
                description: "Recruit a rare hero class from the stagecoach.".to_string(),
                cost: "1500 Gold".to_string(),
                is_available: is_ready && current_gold >= 1500,
                is_unsupported: true,
            },
        ],
        "guild" => {
            let can_upgrade_equipment = current_level.map_or(false, |l| l >= 'c');
            vec![
                BuildingAction {
                    id: "train-skill".to_string(),
                    label: "Train Skill".to_string(),
                    description: "Improve a hero's combat or camping skill.".to_string(),
                    cost: "200 Gold".to_string(),
                    is_available: is_ready && current_gold >= 200,
                    is_unsupported: false,
                },
                BuildingAction {
                    id: "upgrade-weapon".to_string(),
                    label: "Upgrade Weapon".to_string(),
                    description: "Enhance a hero's weapon. Requires higher guild level.".to_string(),
                    cost: "300 Gold".to_string(),
                    is_available: is_ready && can_upgrade_equipment && current_gold >= 300,
                    is_unsupported: false,
                },
                BuildingAction {
                    id: "upgrade-armor".to_string(),
                    label: "Upgrade Armor".to_string(),
                    description: "Improve a hero's armor protection. Requires higher guild level.".to_string(),
                    cost: "300 Gold".to_string(),
                    is_available: is_ready && can_upgrade_equipment && current_gold >= 300,
                    is_unsupported: false,
                },
            ]
        },
        "blacksmith" => vec![
            BuildingAction {
                id: "repair-weapon".to_string(),
                label: "Repair Weapon".to_string(),
                description: "Repair and maintain hero weapons.".to_string(),
                cost: "100 Gold".to_string(),
                is_available: is_ready && current_gold >= 100,
                is_unsupported: false,
            },
            BuildingAction {
                id: "repair-armor".to_string(),
                label: "Repair Armor".to_string(),
                description: "Repair and maintain hero armor.".to_string(),
                cost: "100 Gold".to_string(),
                is_available: is_ready && current_gold >= 100,
                is_unsupported: false,
            },
            BuildingAction {
                id: "upgrade-weapon".to_string(),
                label: "Upgrade Weapon Tier".to_string(),
                description: "Upgrade a hero's weapon to the next tier (blacksmith upgrade).".to_string(),
                cost: "500 Gold".to_string(),
                is_available: is_ready && current_gold >= 500,
                is_unsupported: false,
            },
        ],
        "sanitarium" => vec![
            BuildingAction {
                id: "treat-quirk".to_string(),
                label: "Treat Quirk".to_string(),
                description: "Remove a negative quirk from a hero.".to_string(),
                cost: "250 Gold".to_string(),
                is_available: is_ready && current_gold >= 250,
                is_unsupported: false,
            },
            BuildingAction {
                id: "cure-disease".to_string(),
                label: "Cure Disease".to_string(),
                description: "Treat a hero's disease.".to_string(),
                cost: "500 Gold".to_string(),
                is_available: is_ready && current_gold >= 500,
                is_unsupported: false,
            },
            BuildingAction {
                id: "lock-positive-quirk".to_string(),
                label: "Lock Positive Quirk".to_string(),
                description: "Lock a positive quirk to prevent its loss (unsupported in current build).".to_string(),
                cost: "2500 Gold".to_string(),
                is_available: false,
                is_unsupported: true,
            },
        ],
        "tavern" => vec![
            BuildingAction {
                id: "drink".to_string(),
                label: "Drink".to_string(),
                description: "Purchase food and drink at the tavern bar.".to_string(),
                cost: "50 Gold".to_string(),
                is_available: is_ready && current_gold >= 50,
                is_unsupported: false,
            },
            BuildingAction {
                id: "gamble".to_string(),
                label: "Gamble".to_string(),
                description: "Try your luck at the tavern games.".to_string(),
                cost: "100 Gold".to_string(),
                is_available: is_ready && current_gold >= 100,
                is_unsupported: false,
            },
            BuildingAction {
                id: "brothel".to_string(),
                label: "Visit Brothel".to_string(),
                description: "Visit the tavern's other services for stress relief.".to_string(),
                cost: "150 Gold".to_string(),
                is_available: is_ready && current_gold >= 150,
                is_unsupported: false,
            },
        ],
        "abbey" => vec![
            BuildingAction {
                id: "pray".to_string(),
                label: "Pray".to_string(),
                description: "Reduce hero stress through prayer.".to_string(),
                cost: "75 Gold".to_string(),
                is_available: is_ready && current_gold >= 75,
                is_unsupported: false,
            },
            BuildingAction {
                id: "meditate".to_string(),
                label: "Meditate".to_string(),
                description: "Deep meditation to significantly reduce stress.".to_string(),
                cost: "150 Gold".to_string(),
                is_available: is_ready && current_gold >= 150,
                is_unsupported: false,
            },
        ],
        "inn" => vec![
            BuildingAction {
                id: "rest".to_string(),
                label: "Rest".to_string(),
                description: "Rest at the inn to recover health and reduce fatigue.".to_string(),
                cost: "100 Gold".to_string(),
                is_available: is_ready && current_gold >= 100,
                is_unsupported: false,
            },
            BuildingAction {
                id: "dine".to_string(),
                label: "Dine".to_string(),
                description: "Purchase a meal to boost hero morale.".to_string(),
                cost: "50 Gold".to_string(),
                is_available: is_ready && current_gold >= 50,
                is_unsupported: false,
            },
        ],
        "graveyard" => vec![
            BuildingAction {
                id: "pay-respects".to_string(),
                label: "Pay Respects".to_string(),
                description: "Pay respects to fallen heroes to reduce party stress.".to_string(),
                cost: "Free".to_string(),
                is_available: is_ready,
                is_unsupported: false,
            },
            BuildingAction {
                id: "epitaph".to_string(),
                label: "Inscribe Epitaph".to_string(),
                description: "Inscribe an epitaph for a fallen hero (unsupported in current build).".to_string(),
                cost: "200 Gold".to_string(),
                is_available: false,
                is_unsupported: true,
            },
        ],
        "museum" => vec![
            BuildingAction {
                id: "view-artifacts".to_string(),
                label: "View Artifacts".to_string(),
                description: "Browse collected artifacts and trinkets from expeditions.".to_string(),
                cost: "Free".to_string(),
                is_available: is_ready,
                is_unsupported: false,
            },
            BuildingAction {
                id: "appraise-artifact".to_string(),
                label: "Appraise Artifact".to_string(),
                description: "Appraise an unidentified artifact for its true value.".to_string(),
                cost: "100 Gold".to_string(),
                is_available: is_ready && current_gold >= 100,
                is_unsupported: true,
            },
        ],
        "provisioner" => vec![
            BuildingAction {
                id: "buy-supplies".to_string(),
                label: "Buy Supplies".to_string(),
                description: "Purchase expedition supplies (food, torches, medicine).".to_string(),
                cost: "200 Gold".to_string(),
                is_available: is_ready && current_gold >= 200,
                is_unsupported: false,
            },
            BuildingAction {
                id: "buy-provisions".to_string(),
                label: "Buy Provisions".to_string(),
                description: "Stock up on extra provisions for longer expeditions.".to_string(),
                cost: "500 Gold".to_string(),
                is_available: is_ready && current_gold >= 500,
                is_unsupported: false,
            },
        ],
        "sanctuary" => vec![
            BuildingAction {
                id: "advanced-treatment".to_string(),
                label: "Advanced Treatment".to_string(),
                description: "Advanced medical treatment for severe hero ailments.".to_string(),
                cost: "1000 Gold".to_string(),
                is_available: is_ready && current_gold >= 1000,
                is_unsupported: false,
            },
            BuildingAction {
                id: "hero-revival".to_string(),
                label: "Hero Revival".to_string(),
                description: "Attempt to revive a fallen hero (unsupported in current build).".to_string(),
                cost: "5000 Gold".to_string(),
                is_available: false,
                is_unsupported: true,
            },
        ],
        "campfire" => vec![
            BuildingAction {
                id: "rest".to_string(),
                label: "Rest".to_string(),
                description: "Rest at the campfire to heal and recover.".to_string(),
                cost: "Free".to_string(),
                is_available: true,
                is_unsupported: false,
            },
            BuildingAction {
                id: "camping-skill".to_string(),
                label: "Use Camping Skill".to_string(),
                description: "Apply a camping skill for buffs during dungeon runs.".to_string(),
                cost: "Free".to_string(),
                is_available: true,
                is_unsupported: false,
            },
        ],
        _ => vec![BuildingAction {
            id: "interact".to_string(),
            label: "Interact".to_string(),
            description: "Interact with this building.".to_string(),
            cost: "Free".to_string(),
            is_available: is_ready,
            is_unsupported: false,
        }],
    }
}

/// Adapter: Convert `framework_viewmodels::CombatViewModel` to `CombatViewModel`.
///
/// Takes a framework combat view model and produces a DDGC-specific
/// combat view model. Returns an error if the framework version is
/// incompatible or the payload cannot be fully mapped.
///
/// Note: The framework `CombatViewModel` has a different structure than
/// the DDGC view model. This adapter handles the mapping, but some
/// framework fields may not map directly to DDGC equivalents.
/// Unsupported fields are represented as `None` or placeholder values.
pub fn combat_from_framework(
    framework_vm: &framework_viewmodels::CombatViewModel,
) -> ViewModelResult<CombatViewModel> {
    use framework_combat::encounter::CombatSide;

    let mut heroes = Vec::new();
    let mut monsters = Vec::new();

    // Filter actors by their formation side
    // Note: ActorSummary has fields: id, side, health, max_health, statuses
    // Some fields like slot_index, name, family_id, stress are not available
    // in the framework ActorSummary, so we use defaults/placeholders
    for actor in &framework_vm.actors {
        let health_f64 = actor.health.0;
        let max_health_f64 = actor.max_health.0;

        let combatant = CombatantViewModel {
            id: format!("{:?}", actor.id),
            combatant_type: if actor.side == CombatSide::Ally {
                CombatantType::Hero
            } else {
                CombatantType::Monster
            },
            name: format!("Actor {:?}", actor.id), // Placeholder name
            family_id: String::new(), // Not available in framework ActorSummary
            health: health_f64,
            max_health: max_health_f64,
            stress: None, // Not available in framework ActorSummary
            position: CombatPosition {
                lane: 0,
                slot: 0, // Not available in framework ActorSummary
            },
            active_statuses: actor.statuses.iter().map(|s| format!("{:?}", s.kind)).collect(),
            active_buffs: Vec::new(), // Not available in framework ActorSummary
            active_debuffs: Vec::new(),
            is_dead: health_f64 <= 0.0,
            is_at_deaths_door: health_f64 < (max_health_f64 * 0.5),
        };

        if actor.side == CombatSide::Ally {
            heroes.push(combatant);
        } else {
            monsters.push(combatant);
        }
    }

    let phase = CombatPhase::Unknown;

    Ok(CombatViewModel {
        encounter_id: format!("encounter_{:?}", framework_vm.encounter_id),
        round: framework_vm.round_number,
        heroes,
        monsters,
        selected_actor_id: None,
        current_turn_actor_id: framework_vm.turn_order.first().map(|a| format!("{:?}", a)),
        phase,
        result: None, // Result is determined externally
        error: None,
    })
}

/// Adapter: Convert `CombatViewModel` to `CombatHudViewModel`.
///
/// Takes a DDGC combat view model and produces a minimal HUD view model
/// for the combat shell, presenting only essential combat context.
pub fn combat_hud_from_combat(
    combat: &CombatViewModel,
) -> ViewModelResult<CombatHudViewModel> {
    let hero_vitals: Vec<CombatantVitalViewModel> = combat
        .heroes
        .iter()
        .map(|h| CombatantVitalViewModel {
            id: h.id.clone(),
            combatant_type: CombatantType::Hero,
            health_fraction: if h.max_health > 0.0 {
                h.health / h.max_health
            } else {
                0.0
            },
            is_at_deaths_door: h.is_at_deaths_door,
            is_dead: h.is_dead,
            status_count: h.active_statuses.len(),
        })
        .collect();

    let monster_vitals: Vec<CombatantVitalViewModel> = combat
        .monsters
        .iter()
        .map(|m| CombatantVitalViewModel {
            id: m.id.clone(),
            combatant_type: CombatantType::Monster,
            health_fraction: if m.max_health > 0.0 {
                m.health / m.max_health
            } else {
                0.0
            },
            is_at_deaths_door: m.is_at_deaths_door,
            is_dead: m.is_dead,
            status_count: m.active_statuses.len(),
        })
        .collect();

    let heroes_alive = combat.heroes.iter().filter(|h| !h.is_dead).count() as u32;
    let monsters_alive = combat.monsters.iter().filter(|m| !m.is_dead).count() as u32;

    Ok(CombatHudViewModel {
        encounter_id: combat.encounter_id.clone(),
        round: combat.round,
        phase: combat.phase.clone(),
        result: combat.result.clone(),
        current_turn_actor_id: combat.current_turn_actor_id.clone(),
        hero_vitals,
        monster_vitals,
        heroes_alive,
        monsters_alive,
        is_resolving: combat.phase == CombatPhase::Resolution,
        error: combat.error.clone(),
    })
}

/// Adapter: Convert `DdgcRunResult` to `DungeonViewModel`.
///
/// Takes a DDGC run result and produces a dungeon exploration view model.
pub fn dungeon_from_run_result(
    run_result: &crate::run::flow::DdgcRunResult,
) -> ViewModelResult<DungeonViewModel> {
    let rooms: Vec<DungeonRoomViewModel> = run_result
        .room_encounters
        .iter()
        .map(|enc| {
            use framework_progression::rooms::RoomKind;
            let kind = match &enc.room_kind {
                RoomKind::Combat => DungeonRoomKind::Combat,
                RoomKind::Boss => DungeonRoomKind::Boss,
                RoomKind::Event { .. } => DungeonRoomKind::Event,
                RoomKind::Corridor { .. } => DungeonRoomKind::Corridor,
                _ => DungeonRoomKind::Unknown,
            };

            DungeonRoomViewModel {
                room_id: format!("{:?}", enc.room_id),
                kind,
                cleared: false,
                is_current: false,
                curio_id: None,
                trap_id: None,
            }
        })
        .collect();

    let heroes: Vec<DungeonHeroViewModel> = run_result
        .heroes
        .iter()
        .map(|h| DungeonHeroViewModel {
            id: h.id.clone(),
            class_id: h.class_id.clone(),
            health: h.health,
            max_health: h.max_health,
            stress: h.stress,
            max_stress: h.max_stress,
            active_buffs: h.active_buffs.clone(),
            camping_buffs: h.camping_buffs.clone(),
            is_at_deaths_door: h.health < (h.max_health * 0.5),
            is_dead: h.health <= 0.0,
        })
        .collect();

    let current_room = rooms.first().cloned().map(|mut r| {
        r.is_current = true;
        r
    });

    Ok(DungeonViewModel {
        dungeon_type: format!("{:?}", run_result.metadata.dungeon_type),
        map_size: format!("{:?}", run_result.metadata.map_size),
        floor: 1,
        rooms,
        rooms_cleared: run_result.state.rooms_cleared,
        total_rooms: run_result.metadata.base_room_number,
        current_room,
        gold_carried: run_result.state.gold,
        torchlight: 100,
        battles_won: run_result.state.battles_won,
        battles_lost: run_result.state.battles_lost,
        heroes,
        is_complete: false,
        party_fled: false,
        error: None,
    })
}

/// Adapter: Convert `DungeonViewModel` to `ExplorationHudViewModel`.
///
/// Takes a dungeon view model and produces a minimal HUD view model
/// for the exploration shell, presenting only essential expedition context.
pub fn exploration_hud_from_dungeon(
    dungeon: &DungeonViewModel,
) -> ViewModelResult<ExplorationHudViewModel> {
    let hero_vitals: Vec<HeroVitalViewModel> = dungeon
        .heroes
        .iter()
        .map(|h| {
            let health_fraction = if h.max_health > 0.0 {
                h.health / h.max_health
            } else {
                0.0
            };
            let stress_fraction = if h.max_stress > 0.0 {
                h.stress / h.max_stress
            } else {
                0.0
            };
            HeroVitalViewModel {
                id: h.id.clone(),
                class_id: h.class_id.clone(),
                health_fraction,
                stress_fraction,
                is_at_deaths_door: h.is_at_deaths_door,
                is_dead: h.is_dead,
            }
        })
        .collect();

    Ok(ExplorationHudViewModel {
        dungeon_type: dungeon.dungeon_type.clone(),
        map_size: dungeon.map_size.clone(),
        floor: dungeon.floor,
        rooms_cleared: dungeon.rooms_cleared,
        total_rooms: dungeon.total_rooms,
        gold_carried: dungeon.gold_carried,
        torchlight: dungeon.torchlight,
        battles_won: dungeon.battles_won,
        battles_lost: dungeon.battles_lost,
        hero_vitals,
        current_room_kind: dungeon.current_room.as_ref().map(|r| r.kind.clone()),
        is_complete: dungeon.is_complete,
        error: dungeon.error.clone(),
    })
}

/// Adapter: Convert `DdgcRunResult` and room index to `RoomMovementViewModel`.
///
/// Takes a run result and room index to produce a room movement view model
/// representing the transition into a specific room.
pub fn room_movement_from_run(
    run_result: &crate::run::flow::DdgcRunResult,
    room_index: usize,
) -> ViewModelResult<RoomMovementViewModel> {
    use framework_progression::rooms::RoomKind;

    let rooms = &run_result.floor.rooms;
    if room_index >= rooms.len() {
        return Err(crate::contracts::viewmodels::ViewModelError::MissingRequiredField {
            field: "room_index".to_string(),
            context: format!("room_index {} out of range for {} rooms", room_index, rooms.len()),
        });
    }

    let current_room_id = rooms[room_index];
    let current_room = &run_result.floor.rooms_map[&current_room_id];

    // Previous room (if any)
    let (from_room_id, from_room_kind) = if room_index > 0 {
        let prev_room_id = rooms[room_index - 1];
        let prev_room = &run_result.floor.rooms_map[&prev_room_id];
        let prev_kind = match &prev_room.kind {
            RoomKind::Combat => DungeonRoomKind::Combat,
            RoomKind::Boss => DungeonRoomKind::Boss,
            RoomKind::Event { .. } => DungeonRoomKind::Event,
            RoomKind::Corridor { .. } => DungeonRoomKind::Corridor,
            _ => DungeonRoomKind::Unknown,
        };
        (Some(format!("{:?}", prev_room_id)), Some(prev_kind))
    } else {
        (None, None)
    };

    // Current room kind and interaction
    let (to_room_kind, interaction_id, interaction_type) = match &current_room.kind {
        RoomKind::Combat => {
            (DungeonRoomKind::Combat, None, InteractionType::Combat)
        }
        RoomKind::Boss => {
            (DungeonRoomKind::Boss, None, InteractionType::Boss)
        }
        RoomKind::Event { curio_id } => {
            (
                DungeonRoomKind::Event,
                curio_id.clone(),
                InteractionType::Curio,
            )
        }
        RoomKind::Corridor { trap_id, curio_id } => {
            let int_type = if trap_id.is_some() {
                InteractionType::Trap
            } else if curio_id.is_some() {
                InteractionType::Curio
            } else {
                InteractionType::None
            };
            (
                DungeonRoomKind::Corridor,
                trap_id.clone().or_else(|| curio_id.clone()),
                int_type,
            )
        }
        _ => (DungeonRoomKind::Unknown, None, InteractionType::None),
    };

    Ok(RoomMovementViewModel {
        from_room_id,
        from_room_kind,
        to_room_id: format!("{:?}", current_room_id),
        to_room_kind,
        is_cleared: matches!(current_room.state, framework_progression::rooms::RoomState::Cleared),
        interaction_id,
        interaction_type,
    })
}

/// Adapter: Convert `DdgcRunResult` and room index to `EncounterEntryViewModel`.
///
/// Takes a run result and room index to produce an encounter entry view model
/// representing entering a combat encounter from exploration.
pub fn encounter_entry_from_run(
    run_result: &crate::run::flow::DdgcRunResult,
    room_index: usize,
) -> ViewModelResult<EncounterEntryViewModel> {
    use framework_progression::rooms::RoomKind;

    if room_index >= run_result.room_encounters.len() {
        return Err(crate::contracts::viewmodels::ViewModelError::MissingRequiredField {
            field: "room_index".to_string(),
            context: format!(
                "room_index {} out of range for {} room_encounters",
                room_index,
                run_result.room_encounters.len()
            ),
        });
    }

    let encounter = &run_result.room_encounters[room_index];
    let room = &run_result.floor.rooms_map[&encounter.room_id];

    let encounter_type = match &room.kind {
        RoomKind::Combat => EncounterType::Combat,
        RoomKind::Boss => EncounterType::Boss,
        _ => {
            return Err(crate::contracts::viewmodels::ViewModelError::UnsupportedState {
                state_type: "EncounterEntry".to_string(),
                detail: format!(
                    "Room {:?} is not a combat room (kind: {:?})",
                    encounter.room_id, room.kind
                ),
            });
        }
    };

    let is_boss = matches!(encounter_type, EncounterType::Boss);

    let heroes: Vec<EncounterHeroViewModel> = run_result
        .heroes
        .iter()
        .map(|h| EncounterHeroViewModel {
            id: h.id.clone(),
            class_id: h.class_id.clone(),
            health: h.health,
            max_health: h.max_health,
            stress: h.stress,
            max_stress: h.max_stress,
            active_buffs: h.active_buffs.clone(),
            is_at_deaths_door: h.health < (h.max_health * 0.5),
        })
        .collect();

    Ok(EncounterEntryViewModel {
        encounter_id: format!("encounter_{:?}", encounter.room_id),
        room_id: format!("{:?}", encounter.room_id),
        encounter_type,
        pack_id: encounter.pack_id.clone(),
        family_ids: encounter.family_ids.iter().map(|f| f.0.clone()).collect(),
        heroes,
        is_boss,
    })
}

/// Adapter: Convert run metadata to `ResultViewModel`.
///
/// Takes dungeon run result data and produces a result view model.
pub fn result_from_run(
    dungeon_type: DungeonType,
    map_size: MapSize,
    rooms_cleared: u32,
    battles_won: u32,
    completed: bool,
    gold_earned: u32,
    xp_earned: u32,
    heirlooms_earned: &std::collections::BTreeMap<HeirloomCurrency, u32>,
    casualties: Vec<(String, String)>, // (hero_id, class_id) pairs
) -> ViewModelResult<crate::contracts::viewmodels::ResultViewModel> {
    use crate::contracts::viewmodels::{CasualtyViewModel, OutcomeType, RewardViewModel};

    let outcome = if completed {
        OutcomeType::Success
    } else if battles_won > 0 {
        OutcomeType::PartialSuccess
    } else {
        OutcomeType::Failure
    };

    let title = match outcome {
        OutcomeType::Success => "Dungeon Cleared!",
        OutcomeType::PartialSuccess => "Run Complete",
        OutcomeType::Failure => "Run Failed",
        _ => "Run Ended",
    };

    let description = match outcome {
        OutcomeType::Success => format!(
            "Your party successfully cleared {} rooms and won {} battles!",
            rooms_cleared, battles_won
        ),
        OutcomeType::PartialSuccess => format!(
            "Your party cleared {} rooms and won {} battles before retreating.",
            rooms_cleared, battles_won
        ),
        OutcomeType::Failure => {
            "Your party was defeated and retreated from the dungeon.".to_string()
        }
        _ => "The run has ended.".to_string(),
    };

    let rewards = if outcome == OutcomeType::Success || outcome == OutcomeType::PartialSuccess {
        Some(RewardViewModel {
            gold: gold_earned,
            heirlooms: heirlooms_earned
                .iter()
                .map(|(k, v)| (format!("{:?}", k).to_lowercase(), *v))
                .collect(),
            xp: xp_earned,
            loot: Vec::new(),
            trinkets: Vec::new(),
        })
    } else {
        None
    };

    let casualty_models: Vec<CasualtyViewModel> = casualties
        .iter()
        .map(|(hero_id, class_id)| CasualtyViewModel {
            hero_id: hero_id.clone(),
            class_id: class_id.clone(),
            cause: None,
        })
        .collect();

    Ok(crate::contracts::viewmodels::ResultViewModel {
        outcome,
        title: title.to_string(),
        description,
        rewards,
        casualties: casualty_models,
        dungeon_type: Some(format!("{:?}", dungeon_type)),
        map_size: Some(format!("{:?}", map_size)),
        rooms_cleared,
        battles_won,
        completed,
        error: None,
    })
}

/// Adapter: Convert `DdgcRunState` and heroes to `ReturnFlowViewModel`.
///
/// Takes the run state and hero states to produce a return flow view model.
pub fn return_flow_from_state(
    dungeon_type: DungeonType,
    map_size: MapSize,
    rooms_cleared: u32,
    battles_won: u32,
    completed: bool,
    gold_earned: u32,
    heroes: &[(String, String, f64, f64, f64, f64)], // (id, class_id, health, max_health, stress, max_stress)
    died_heroes: &[(String, String)], // (id, class_id) pairs
) -> ViewModelResult<crate::contracts::viewmodels::ReturnFlowViewModel> {
    use crate::contracts::viewmodels::{ReturnFlowHeroViewModel, ReturnFlowState};

    let return_heroes: Vec<ReturnFlowHeroViewModel> = heroes
        .iter()
        .map(
            |(id, class_id, health, max_health, stress, max_stress)| {
                let died = died_heroes.iter().any(|(did, _)| did == id);
                ReturnFlowHeroViewModel {
                    id: id.clone(),
                    class_id: class_id.clone(),
                    health: *health,
                    max_health: *max_health,
                    stress: *stress,
                    max_stress: *max_stress,
                    survived: !died && *health > 0.0,
                    died,
                    is_at_deaths_door: *health < (*max_health * 0.5),
                }
            },
        )
        .collect();

    let state = if completed {
        ReturnFlowState::ShowingResults
    } else {
        ReturnFlowState::Traveling
    };

    Ok(crate::contracts::viewmodels::ReturnFlowViewModel {
        state,
        dungeon_type: format!("{:?}", dungeon_type),
        map_size: format!("{:?}", map_size),
        completed,
        rooms_cleared,
        battles_won,
        gold_to_transfer: gold_earned,
        torchlight_remaining: 100,
        heroes: return_heroes,
        run_result: None,
        ready_for_town: completed,
        error: None,
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Utility functions
// ─────────────────────────────────────────────────────────────────────────────

/// Convert a `DungeonType` to a display string.
pub fn dungeon_type_display(dungeon: DungeonType) -> &'static str {
    match dungeon {
        DungeonType::QingLong => "Azure Dragon",
        DungeonType::BaiHu => "White Tiger",
        DungeonType::ZhuQue => "Vermilion Bird",
        DungeonType::XuanWu => "Black Tortoise",
    }
}

/// Convert a `MapSize` to a display string.
pub fn map_size_display(size: MapSize) -> &'static str {
    match size {
        MapSize::Short => "Short",
        MapSize::Medium => "Medium",
    }
}

/// Check if a dungeon type is valid for view model shaping.
pub fn is_valid_dungeon(dungeon: DungeonType) -> bool {
    matches!(
        dungeon,
        DungeonType::QingLong | DungeonType::BaiHu | DungeonType::ZhuQue | DungeonType::XuanWu
    )
}

/// Check if a map size is valid for view model shaping.
pub fn is_valid_map_size(size: MapSize) -> bool {
    matches!(size, MapSize::Short | MapSize::Medium)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::host::HostPhase;
    use crate::contracts::viewmodels::{
        OutcomeType,
        ReturnFlowState,
    };
    use crate::run::flow::RoomEncounterRecord;

    // ── boot_load_from_host tests ─────────────────────────────────────────────

    #[test]
    fn boot_load_from_host_uninitialized() {
        let result = boot_load_from_host(&HostPhase::Uninitialized, false, None);
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert!(vm.loaded);
        assert_eq!(vm.status_message, "Initialized and ready to boot");
        assert!(vm.error.is_none());
    }

    #[test]
    fn boot_load_from_host_booting() {
        let result = boot_load_from_host(&HostPhase::Booting, false, None);
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert!(vm.loaded);
        assert_eq!(vm.status_message, "Loading contract packages...");
    }

    #[test]
    fn boot_load_from_host_ready_without_campaign() {
        let result = boot_load_from_host(&HostPhase::Ready, false, None);
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert!(vm.loaded);
        assert_eq!(vm.status_message, "Host ready");
        assert!(vm.error.is_none());
    }

    #[test]
    fn boot_load_from_host_ready_with_campaign() {
        let result = boot_load_from_host(&HostPhase::Ready, true, Some(1));
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert!(vm.loaded);
        assert_eq!(vm.status_message, "Campaign loaded successfully");
        assert_eq!(vm.campaign_schema_version, Some(1));
    }

    #[test]
    fn boot_load_from_host_fatal_error() {
        let result = boot_load_from_host(&HostPhase::FatalError, false, None);
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert!(!vm.loaded);
        assert!(vm.error.is_some());
    }

    #[test]
    fn boot_load_from_host_unsupported() {
        let result = boot_load_from_host(&HostPhase::Unsupported, false, None);
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert!(!vm.loaded);
        assert!(vm.error.is_some());
    }

    // ── town_from_campaign tests ──────────────────────────────────────────────

    #[test]
    fn town_from_campaign_empty_roster() {
        use crate::contracts::{CampaignState, BuildingUpgradeState};

        let mut campaign = CampaignState::new(1000);
        campaign.building_states.insert(
            "stagecoach".to_string(),
            BuildingUpgradeState::new("stagecoach", Some('a')),
        );

        let result = town_from_campaign(&campaign);
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.gold, 1000);
        assert!(vm.roster.is_empty());
        assert!(!vm.buildings.is_empty());
        assert!(!vm.available_activities.is_empty());
    }

    #[test]
    fn town_from_campaign_with_heroes() {
        use crate::contracts::{CampaignState, CampaignHero, CampaignHeroQuirks};

        let mut campaign = CampaignState::new(500);
        campaign.gold = 500;

        let hero = CampaignHero {
            id: "hero1".to_string(),
            class_id: "crusader".to_string(),
            level: 1,
            xp: 0,
            health: 80.0,
            max_health: 100.0,
            stress: 20.0,
            max_stress: 200.0,
            quirks: CampaignHeroQuirks::new(),
            equipment: Default::default(),
            skills: Vec::new(),
            traits: Default::default(),
        };
        campaign.roster.push(hero);

        let result = town_from_campaign(&campaign);
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.roster.len(), 1);
        assert_eq!(vm.roster[0].id, "hero1");
        assert!(vm.roster[0].is_wounded); // 80 < 100 (not full health)
        assert!(!vm.roster[0].is_afflicted); // 20 < 200
    }

    #[test]
    fn town_from_campaign_wounded_hero() {
        use crate::contracts::{CampaignState, CampaignHero, CampaignHeroQuirks};

        let mut campaign = CampaignState::new(500);
        let hero = CampaignHero {
            id: "hero1".to_string(),
            class_id: "crusader".to_string(),
            level: 1,
            xp: 0,
            health: 50.0,
            max_health: 100.0,
            stress: 20.0,
            max_stress: 200.0,
            quirks: CampaignHeroQuirks::new(),
            equipment: Default::default(),
            skills: Vec::new(),
            traits: Default::default(),
        };
        campaign.roster.push(hero);

        let result = town_from_campaign(&campaign);
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert!(vm.roster[0].is_wounded); // 50 < 100
    }

    #[test]
    fn town_from_campaign_afflicted_hero() {
        use crate::contracts::{CampaignState, CampaignHero, CampaignHeroQuirks};

        let mut campaign = CampaignState::new(500);
        let hero = CampaignHero {
            id: "hero1".to_string(),
            class_id: "crusader".to_string(),
            level: 1,
            xp: 0,
            health: 100.0,
            max_health: 100.0,
            stress: 200.0,
            max_stress: 200.0,
            quirks: CampaignHeroQuirks::new(),
            equipment: Default::default(),
            skills: Vec::new(),
            traits: Default::default(),
        };
        campaign.roster.push(hero);

        let result = town_from_campaign(&campaign);
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert!(vm.roster[0].is_afflicted); // 200 >= 200
    }

    // ── hero_detail_from_campaign tests ─────────────────────────────────────

    #[test]
    fn hero_detail_from_campaign_finds_hero_by_id() {
        use crate::contracts::{CampaignState, CampaignHero, CampaignHeroQuirks};

        let mut campaign = CampaignState::new(500);
        let hero = CampaignHero {
            id: "hero1".to_string(),
            class_id: "crusader".to_string(),
            level: 2,
            xp: 300,
            health: 80.0,
            max_health: 100.0,
            stress: 30.0,
            max_stress: 200.0,
            quirks: CampaignHeroQuirks::new(),
            equipment: Default::default(),
            skills: vec!["skill1".to_string()],
            traits: Default::default(),
        };
        campaign.roster.push(hero);

        let result = hero_detail_from_campaign(&campaign, "hero1");
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.hero_id, "hero1");
        assert_eq!(vm.kind, "hero-detail");
        assert_eq!(vm.class_label, "crusader");
    }

    #[test]
    fn hero_detail_from_campaign_missing_hero_returns_error() {
        use crate::contracts::CampaignState;

        let campaign = CampaignState::new(500);

        let result = hero_detail_from_campaign(&campaign, "nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn hero_detail_from_campaign_formats_vitals_correctly() {
        use crate::contracts::{CampaignState, CampaignHero, CampaignHeroQuirks};

        let mut campaign = CampaignState::new(500);
        let hero = CampaignHero {
            id: "hero1".to_string(),
            class_id: "hunter".to_string(),
            level: 3,
            xp: 400,
            health: 42.0,
            max_health: 42.0,
            stress: 17.0,
            max_stress: 200.0,
            quirks: CampaignHeroQuirks::new(),
            equipment: Default::default(),
            skills: Vec::new(),
            traits: Default::default(),
        };
        campaign.roster.push(hero);

        let result = hero_detail_from_campaign(&campaign, "hero1");
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.hp, "42");
        assert_eq!(vm.max_hp, "42");
        assert_eq!(vm.stress, "17");
        assert_eq!(vm.resolve, "3");
    }

    // ── result_from_run tests ───────────────────────────────────────────────

    #[test]
    fn result_from_run_victory() {
        use std::collections::BTreeMap;

        let heirlooms: BTreeMap<HeirloomCurrency, u32> = BTreeMap::new();
        let casualties = Vec::new();

        let result = result_from_run(
            DungeonType::QingLong,
            MapSize::Short,
            8,
            4,
            true,  // completed
            500,
            100,
            &heirlooms,
            casualties,
        );

        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.outcome, OutcomeType::Success);
        assert_eq!(vm.title, "Dungeon Cleared!");
        assert!(vm.rewards.is_some());
        assert_eq!(vm.rewards.unwrap().gold, 500);
    }

    #[test]
    fn result_from_run_partial_success() {
        use std::collections::BTreeMap;

        let heirlooms: BTreeMap<HeirloomCurrency, u32> = BTreeMap::new();
        let casualties = Vec::new();

        let result = result_from_run(
            DungeonType::QingLong,
            MapSize::Short,
            4,
            2,
            false, // not completed
            200,
            50,
            &heirlooms,
            casualties,
        );

        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.outcome, OutcomeType::PartialSuccess);
        assert_eq!(vm.title, "Run Complete");
        assert!(vm.rewards.is_some());
    }

    #[test]
    fn result_from_run_failure() {
        use std::collections::BTreeMap;

        let heirlooms: BTreeMap<HeirloomCurrency, u32> = BTreeMap::new();
        let casualties = vec![("hero1".to_string(), "crusader".to_string())];

        let result = result_from_run(
            DungeonType::QingLong,
            MapSize::Short,
            2,
            0,
            false,
            0,
            0,
            &heirlooms,
            casualties,
        );

        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.outcome, OutcomeType::Failure);
        assert_eq!(vm.title, "Run Failed");
        assert!(vm.rewards.is_none());
        assert_eq!(vm.casualties.len(), 1);
    }

    // ── return_flow_from_state tests ────────────────────────────────────────

    #[test]
    fn return_flow_from_state_completed() {
        let heroes = vec![
            ("hero1".to_string(), "crusader".to_string(), 80.0, 100.0, 20.0, 200.0),
        ];
        let died_heroes = Vec::new();

        let result = return_flow_from_state(
            DungeonType::QingLong,
            MapSize::Short,
            8,
            4,
            true,  // completed
            500,
            &heroes,
            &died_heroes,
        );

        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.state, ReturnFlowState::ShowingResults);
        assert!(vm.completed);
        assert!(vm.ready_for_town);
        assert_eq!(vm.gold_to_transfer, 500);
    }

    #[test]
    fn return_flow_from_state_in_progress() {
        let heroes = vec![
            ("hero1".to_string(), "crusader".to_string(), 80.0, 100.0, 20.0, 200.0),
        ];
        let died_heroes = Vec::new();

        let result = return_flow_from_state(
            DungeonType::QingLong,
            MapSize::Short,
            4,
            2,
            false, // not completed
            200,
            &heroes,
            &died_heroes,
        );

        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.state, ReturnFlowState::Traveling);
        assert!(!vm.completed);
        assert!(!vm.ready_for_town);
    }

    #[test]
    fn return_flow_from_state_with_casualties() {
        let heroes = vec![
            ("hero1".to_string(), "crusader".to_string(), 80.0, 100.0, 20.0, 200.0),
            ("hero2".to_string(), "hunter".to_string(), 0.0, 100.0, 250.0, 200.0), // dead
        ];
        let died_heroes = vec![("hero2".to_string(), "hunter".to_string())];

        let result = return_flow_from_state(
            DungeonType::QingLong,
            MapSize::Short,
            3,
            1,
            false,
            100,
            &heroes,
            &died_heroes,
        );

        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.heroes.len(), 2);
        let hero1 = vm.heroes.iter().find(|h| h.id == "hero1").unwrap();
        assert!(hero1.survived);
        assert!(!hero1.died);

        let hero2 = vm.heroes.iter().find(|h| h.id == "hero2").unwrap();
        assert!(!hero2.survived);
        assert!(hero2.died);
    }

    // ── Utility function tests ─────────────────────────────────────────────

    #[test]
    fn dungeon_type_display_returns_correct_names() {
        assert_eq!(dungeon_type_display(DungeonType::QingLong), "Azure Dragon");
        assert_eq!(dungeon_type_display(DungeonType::BaiHu), "White Tiger");
        assert_eq!(dungeon_type_display(DungeonType::ZhuQue), "Vermilion Bird");
        assert_eq!(dungeon_type_display(DungeonType::XuanWu), "Black Tortoise");
    }

    #[test]
    fn map_size_display_returns_correct_names() {
        assert_eq!(map_size_display(MapSize::Short), "Short");
        assert_eq!(map_size_display(MapSize::Medium), "Medium");
    }

    #[test]
    fn is_valid_dungeon_returns_true_for_all_dungeon_types() {
        assert!(is_valid_dungeon(DungeonType::QingLong));
        assert!(is_valid_dungeon(DungeonType::BaiHu));
        assert!(is_valid_dungeon(DungeonType::ZhuQue));
        assert!(is_valid_dungeon(DungeonType::XuanWu));
    }

    #[test]
    fn is_valid_map_size_returns_true_for_both_sizes() {
        assert!(is_valid_map_size(MapSize::Short));
        assert!(is_valid_map_size(MapSize::Medium));
    }

    // ─────────────────────────────────────────────────────────────────────────
    // US-008-c: Replay-driven end-to-end validation for the adapter slice
    // ─────────────────────────────────────────────────────────────────────────

    /// Replay fixture for BootLoadViewModel — represents initial game boot state.
    fn make_replay_boot_load() -> BootLoadViewModel {
        BootLoadViewModel::success("Campaign loaded successfully", vec!["heroes", "monsters", "dungeons"])
            .with_campaign_version(1)
    }

    /// Replay fixture for TownViewModel — represents town visit with activities and roster.
    fn make_replay_town_vm() -> crate::contracts::viewmodels::TownViewModel {
        use crate::contracts::{BuildingUpgradeState, CampaignHero, CampaignHeroQuirks, CampaignState};

        let mut campaign = CampaignState::new(1500);
        campaign.roster.push(CampaignHero {
            id: "h1".to_string(),
            class_id: "crusader".to_string(),
            level: 3,
            xp: 500,
            health: 80.0,
            max_health: 100.0,
            stress: 30.0,
            max_stress: 200.0,
            quirks: CampaignHeroQuirks::new(),
            equipment: Default::default(),
            skills: Vec::new(),
            traits: Default::default(),
        });
        campaign.roster.push(CampaignHero {
            id: "h2".to_string(),
            class_id: "hunter".to_string(),
            level: 2,
            xp: 300,
            health: 95.0,
            max_health: 100.0,
            stress: 50.0,
            max_stress: 200.0,
            quirks: CampaignHeroQuirks::new(),
            equipment: Default::default(),
            skills: Vec::new(),
            traits: Default::default(),
        });
        campaign.building_states.insert(
            "stagecoach".to_string(),
            BuildingUpgradeState::new("stagecoach", Some('a')),
        );

        town_from_campaign(&campaign).expect("town_from_campaign should succeed for valid replay fixture")
    }

    /// Replay fixture for DungeonViewModel — represents active dungeon run state.
    fn make_replay_dungeon_vm() -> DungeonViewModel {
        let heroes = vec![
            make_replay_hero_state("h1", "crusader", 80.0, 100.0, 30.0, 200.0),
            make_replay_hero_state("h2", "hunter", 95.0, 100.0, 50.0, 200.0),
        ];
        let room_encounters = vec![
            RoomEncounterRecord {
                room_id: framework_progression::rooms::RoomId(1),
                room_kind: framework_progression::rooms::RoomKind::Combat,
                pack_id: "pack1".to_string(),
                family_ids: vec![],
            },
            RoomEncounterRecord {
                room_id: framework_progression::rooms::RoomId(2),
                room_kind: framework_progression::rooms::RoomKind::Boss,
                pack_id: "boss_pack".to_string(),
                family_ids: vec![],
            },
        ];

        let run_result = crate::run::flow::DdgcRunResult {
            run: make_replay_run(),
            state: crate::run::flow::DdgcRunState::new(),
            floor: make_replay_floor(),
            battle_pack_ids: vec![],
            metadata: crate::run::flow::RunMetadata {
                dungeon_type: DungeonType::QingLong,
                map_size: MapSize::Short,
                base_room_number: 9,
                base_corridor_number: 4,
                gridsize: crate::contracts::GridSize::new(5, 5),
                connectivity: 0.9,
            },
            room_encounters,
            interaction_records: vec![],
            camping_trace: vec![],
            heroes,
        };

        dungeon_from_run_result(&run_result).expect("dungeon_from_run_result should succeed for valid replay fixture")
    }

    /// Replay fixture for CombatViewModel — represents active combat state.
    fn make_replay_combat_vm() -> crate::contracts::viewmodels::CombatViewModel {
        let actors = vec![
            make_replay_actor_summary(1, framework_combat::encounter::CombatSide::Ally, 80.0, 100.0, vec![]),
            make_replay_actor_summary(2, framework_combat::encounter::CombatSide::Ally, 95.0, 100.0, vec![]),
            make_replay_actor_summary(10, framework_combat::encounter::CombatSide::Enemy, 150.0, 200.0, vec![]),
            make_replay_actor_summary(11, framework_combat::encounter::CombatSide::Enemy, 200.0, 200.0, vec![]),
        ];
        let framework_vm = make_replay_framework_combat_vm(1, 1, actors, vec![1, 2, 10, 11]);

        combat_from_framework(&framework_vm).expect("combat_from_framework should succeed for valid replay fixture")
    }

    /// Replay fixture for CombatHudViewModel — represents combat HUD state.
    fn make_replay_combat_hud_vm() -> CombatHudViewModel {
        let combat_vm = make_replay_combat_vm();
        combat_hud_from_combat(&combat_vm).expect("combat_hud_from_combat should succeed for valid replay fixture")
    }

    /// Replay fixture for ResultViewModel — represents dungeon/combat result state.
    fn make_replay_result_vm() -> crate::contracts::viewmodels::ResultViewModel {
        let heirlooms: std::collections::BTreeMap<HeirloomCurrency, u32> = std::collections::BTreeMap::new();

        result_from_run(
            DungeonType::ZhuQue,
            MapSize::Medium,
            14,
            6,
            true,
            800,
            200,
            &heirlooms,
            vec![],
        ).expect("result_from_run should succeed for valid replay fixture")
    }

    /// Replay fixture for ReturnFlowViewModel — represents return-to-town flow state.
    fn make_replay_return_flow_vm() -> crate::contracts::viewmodels::ReturnFlowViewModel {
        let heroes: Vec<(String, String, f64, f64, f64, f64)> = vec![
            ("h1".to_string(), "crusader".to_string(), 40.0, 100.0, 150.0, 200.0),
            ("h2".to_string(), "hunter".to_string(), 95.0, 100.0, 50.0, 200.0),
        ];
        let died_heroes: Vec<(String, String)> = vec![];

        return_flow_from_state(
            DungeonType::BaiHu,
            MapSize::Short,
            9,
            4,
            true,
            500,
            &heroes,
            &died_heroes,
        ).expect("return_flow_from_state should succeed for valid replay fixture")
    }

    /// Replay fixture for HeroDetailViewModel — represents hero inspection state.
    fn make_replay_hero_detail_vm() -> crate::contracts::viewmodels::HeroDetailViewModel {
        use crate::contracts::{CampaignHero, CampaignHeroQuirks, CampaignState};

        let mut campaign = CampaignState::new(1500);
        let hero = CampaignHero {
            id: "hero-hunter-01".to_string(),
            class_id: "hunter".to_string(),
            level: 2,
            xp: 240,
            health: 38.0,
            max_health: 42.0,
            stress: 17.0,
            max_stress: 200.0,
            quirks: CampaignHeroQuirks::new(),
            equipment: Default::default(),
            skills: vec!["hunting_bow".to_string(), "rapid_shot".to_string()],
            traits: Default::default(),
        };
        campaign.roster.push(hero);

        hero_detail_from_campaign(&campaign, "hero-hunter-01")
            .expect("hero_detail_from_campaign should succeed for valid replay fixture")
    }

    // ── Replay helper functions ───────────────────────────────────────────────

    /// Helper: create a HeroState for replay fixtures.
    fn make_replay_hero_state(
        id: &str,
        class_id: &str,
        health: f64,
        max_health: f64,
        stress: f64,
        max_stress: f64,
    ) -> crate::run::flow::HeroState {
        crate::run::flow::HeroState::new(id, class_id, health, max_health, stress, max_stress)
    }

    /// Helper: create a framework ActorSummary for replay fixtures.
    fn make_replay_actor_summary(
        id: u64,
        side: framework_combat::encounter::CombatSide,
        health: f64,
        max_health: f64,
        statuses: Vec<(&str, Option<u32>)>,
    ) -> framework_viewmodels::combat::ActorSummary {
        framework_viewmodels::combat::ActorSummary {
            id: framework_rules::actor::ActorId(id),
            side,
            health: framework_rules::attributes::AttributeValue(health),
            max_health: framework_rules::attributes::AttributeValue(max_health),
            statuses: statuses
                .into_iter()
                .map(|(kind, dur)| framework_viewmodels::combat::StatusSummary {
                    kind: kind.to_string(),
                    duration: dur,
                })
                .collect(),
        }
    }

    /// Helper: create a framework CombatViewModel for replay fixtures.
    fn make_replay_framework_combat_vm(
        encounter_id: u64,
        round: u32,
        actors: Vec<framework_viewmodels::combat::ActorSummary>,
        turn_order: Vec<u64>,
    ) -> framework_viewmodels::combat::CombatViewModel {
        framework_viewmodels::combat::CombatViewModel {
            encounter_id: framework_combat::encounter::EncounterId(encounter_id),
            current_turn: turn_order.first().map(|id| framework_rules::actor::ActorId(*id)),
            turn_order: turn_order
                .into_iter()
                .map(framework_rules::actor::ActorId)
                .collect(),
            actors,
            formation: framework_viewmodels::combat::FormationSummary {
                lanes: 1,
                slots_per_lane: 4,
                slots: vec![
                    framework_viewmodels::combat::FormationSlotSummary {
                        slot_index: 0,
                        lane: 0,
                        occupant: Some(framework_rules::actor::ActorId(1)),
                    },
                    framework_viewmodels::combat::FormationSlotSummary {
                        slot_index: 1,
                        lane: 0,
                        occupant: Some(framework_rules::actor::ActorId(2)),
                    },
                    framework_viewmodels::combat::FormationSlotSummary {
                        slot_index: 2,
                        lane: 0,
                        occupant: Some(framework_rules::actor::ActorId(10)),
                    },
                    framework_viewmodels::combat::FormationSlotSummary {
                        slot_index: 3,
                        lane: 0,
                        occupant: Some(framework_rules::actor::ActorId(11)),
                    },
                ],
            },
            round_number: round,
        }
    }

    /// Helper: create a minimal Run for replay fixtures.
    fn make_replay_run() -> framework_progression::run::Run {
        framework_progression::run::Run::new(
            framework_progression::run::RunId(1),
            vec![framework_progression::floor::FloorId(1)],
            42,
        )
    }

    /// Helper: create a minimal Floor for replay fixtures.
    fn make_replay_floor() -> framework_progression::floor::Floor {
        framework_progression::floor::Floor::new(
            framework_progression::floor::FloorId(1),
            vec![],
            framework_progression::rooms::RoomId(0),
        )
    }

    // ── US-008-c: Replay fixture validation tests ─────────────────────────────────

    /// Verifies BootLoadViewModel replay fixture renders without errors.
    #[test]
    fn replay_boot_load_fixture_renders_without_error() {
        let vm = make_replay_boot_load();
        assert!(vm.loaded, "BootLoad should be loaded");
        assert!(vm.error.is_none(), "BootLoad should have no error: {:?}", vm.error);
    }

    /// Verifies BootLoadViewModel replay fixture is deterministic.
    #[test]
    fn replay_boot_load_fixture_deterministic() {
        let vm1 = make_replay_boot_load();
        let vm2 = make_replay_boot_load();
        assert_eq!(vm1, vm2, "BootLoad fixture should be deterministic");
    }

    /// Verifies TownViewModel replay fixture renders without errors.
    #[test]
    fn replay_town_fixture_renders_without_error() {
        let vm = make_replay_town_vm();
        assert!(vm.error.is_none(), "Town should have no error: {:?}", vm.error);
        assert_eq!(vm.roster.len(), 2, "Town should have 2 heroes");
    }

    /// Verifies TownViewModel replay fixture is deterministic.
    #[test]
    fn replay_town_fixture_deterministic() {
        let vm1 = make_replay_town_vm();
        let vm2 = make_replay_town_vm();
        assert_eq!(vm1, vm2, "Town fixture should be deterministic");
    }

    /// Verifies DungeonViewModel replay fixture renders without errors.
    #[test]
    fn replay_dungeon_fixture_renders_without_error() {
        let vm = make_replay_dungeon_vm();
        assert!(vm.error.is_none(), "Dungeon should have no error: {:?}", vm.error);
        assert_eq!(vm.dungeon_type, "QingLong", "Dungeon type should be QingLong");
        assert_eq!(vm.map_size, "Short", "Map size should be Short");
    }

    /// Verifies DungeonViewModel replay fixture is deterministic.
    #[test]
    fn replay_dungeon_fixture_deterministic() {
        let vm1 = make_replay_dungeon_vm();
        let vm2 = make_replay_dungeon_vm();
        assert_eq!(vm1, vm2, "Dungeon fixture should be deterministic");
    }

    /// Verifies CombatViewModel replay fixture renders without errors.
    #[test]
    fn replay_combat_fixture_renders_without_error() {
        let vm = make_replay_combat_vm();
        assert!(vm.error.is_none(), "Combat should have no error: {:?}", vm.error);
        assert_eq!(vm.heroes.len(), 2, "Combat should have 2 heroes");
        assert_eq!(vm.monsters.len(), 2, "Combat should have 2 monsters");
    }

    /// Verifies CombatViewModel replay fixture is deterministic.
    #[test]
    fn replay_combat_fixture_deterministic() {
        let vm1 = make_replay_combat_vm();
        let vm2 = make_replay_combat_vm();
        assert_eq!(vm1, vm2, "Combat fixture should be deterministic");
    }

    /// Verifies CombatHudViewModel replay fixture renders without errors.
    #[test]
    fn replay_combat_hud_fixture_renders_without_error() {
        let vm = make_replay_combat_hud_vm();
        assert!(vm.is_combat_active(), "Combat HUD should show active combat");
        assert_eq!(vm.heroes_alive, 2, "Should have 2 heroes alive");
        assert_eq!(vm.monsters_alive, 2, "Should have 2 monsters alive");
    }

    /// Verifies CombatHudViewModel replay fixture is deterministic.
    #[test]
    fn replay_combat_hud_fixture_deterministic() {
        let vm1 = make_replay_combat_hud_vm();
        let vm2 = make_replay_combat_hud_vm();
        assert_eq!(vm1, vm2, "CombatHUD fixture should be deterministic");
    }

    /// Verifies ResultViewModel replay fixture renders without errors.
    #[test]
    fn replay_result_fixture_renders_without_error() {
        let vm = make_replay_result_vm();
        assert_eq!(vm.outcome, crate::contracts::viewmodels::OutcomeType::Success, "Result should be Success");
        assert!(vm.error.is_none(), "Result should have no error: {:?}", vm.error);
    }

    /// Verifies ResultViewModel replay fixture is deterministic.
    #[test]
    fn replay_result_fixture_deterministic() {
        let vm1 = make_replay_result_vm();
        let vm2 = make_replay_result_vm();
        assert_eq!(vm1, vm2, "Result fixture should be deterministic");
    }

    /// Verifies ReturnFlowViewModel replay fixture renders without errors.
    #[test]
    fn replay_return_flow_fixture_renders_without_error() {
        let vm = make_replay_return_flow_vm();
        assert!(vm.error.is_none(), "ReturnFlow should have no error: {:?}", vm.error);
        assert_eq!(vm.heroes.len(), 2, "ReturnFlow should have 2 heroes");
    }

    /// Verifies ReturnFlowViewModel replay fixture is deterministic.
    #[test]
    fn replay_return_flow_fixture_deterministic() {
        let vm1 = make_replay_return_flow_vm();
        let vm2 = make_replay_return_flow_vm();
        assert_eq!(vm1, vm2, "ReturnFlow fixture should be deterministic");
    }

    /// Verifies HeroDetailViewModel replay fixture renders without errors.
    #[test]
    fn replay_hero_detail_fixture_renders_without_error() {
        let vm = make_replay_hero_detail_vm();
        assert_eq!(vm.kind, "hero-detail", "HeroDetail should have kind 'hero-detail'");
        assert_eq!(vm.hero_id, "hero-hunter-01", "Hero ID should match");
        assert_eq!(vm.class_label, "hunter", "Class label should be hunter");
    }

    /// Verifies HeroDetailViewModel replay fixture is deterministic.
    #[test]
    fn replay_hero_detail_fixture_deterministic() {
        let vm1 = make_replay_hero_detail_vm();
        let vm2 = make_replay_hero_detail_vm();
        assert_eq!(vm1, vm2, "HeroDetail fixture should be deterministic");
    }

    /// Verifies vertical slice can be rendered end-to-end from replay fixtures.
    ///
    /// This validates that all view models in the representative slice can be
    /// created and rendered without errors from replay fixtures, proving the
    /// adapter contract is stable for frontend consumption.
    #[test]
    fn replay_vertical_slice_end_to_end_renders() {
        // Boot
        let boot_vm = make_replay_boot_load();
        assert!(boot_vm.loaded);

        // Town
        let town_vm = make_replay_town_vm();
        assert!(town_vm.error.is_none());

        // Dungeon
        let dungeon_vm = make_replay_dungeon_vm();
        assert!(dungeon_vm.error.is_none());

        // Combat
        let combat_vm = make_replay_combat_vm();
        assert!(combat_vm.error.is_none());

        // Combat HUD
        let combat_hud_vm = make_replay_combat_hud_vm();
        assert!(combat_hud_vm.is_combat_active());

        // Result
        let result_vm = make_replay_result_vm();
        assert!(result_vm.error.is_none());

        // Return flow
        let return_vm = make_replay_return_flow_vm();
        assert!(return_vm.error.is_none());

        // Hero detail
        let hero_detail_vm = make_replay_hero_detail_vm();
        assert_eq!(hero_detail_vm.kind, "hero-detail");
    }

    /// Verifies ViewModelError descriptions are actionable for debugging.
    ///
    /// When adapter mapping fails, the error should provide enough context
    /// to identify the source of the failure without requiring deep framework knowledge.
    #[test]
    fn viewmodel_error_descriptions_are_actionable_for_debugging() {
        use crate::contracts::viewmodels::ViewModelError;

        let err = ViewModelError::UnsupportedState {
            state_type: "Combat".to_string(),
            detail: "monster turn not supported".to_string(),
        };
        let desc = err.description();
        assert!(desc.contains("Combat"), "Error should mention state type");
        assert!(desc.contains("monster turn not supported"), "Error should mention detail");

        let err2 = ViewModelError::PartialMapping {
            state_type: "Town".to_string(),
            missing_fields: vec!["building_states".to_string(), "roster".to_string()],
        };
        let desc2 = err2.description();
        assert!(desc2.contains("Town"), "Error should mention state type");
        assert!(desc2.contains("building_states"), "Error should list missing fields");

        let err3 = ViewModelError::MissingRequiredField {
            field: "health".to_string(),
            context: "CampaignHero".to_string(),
        };
        let desc3 = err3.description();
        assert!(desc3.contains("health"), "Error should mention field");
        assert!(desc3.contains("CampaignHero"), "Error should mention context");

        let err4 = ViewModelError::IncompatibleSchema {
            expected: "2.0".to_string(),
            found: "1.0".to_string(),
        };
        let desc4 = err4.description();
        assert!(desc4.contains("2.0"), "Error should mention expected version");
        assert!(desc4.contains("1.0"), "Error should mention found version");
    }

    /// Verifies replay-driven and live-runtime validation consume same contract boundary.
    ///
    /// Both replay fixtures and live-constructed payloads should produce valid
    /// view models when passed through the same adapters, proving the adapter
    /// contract is stable for frontend consumption.
    #[test]
    fn replay_and_live_consume_same_contract_boundary() {
        // Create live-constructed combat VM via adapter
        let actors = vec![
            make_replay_actor_summary(1, framework_combat::encounter::CombatSide::Ally, 80.0, 100.0, vec![]),
            make_replay_actor_summary(10, framework_combat::encounter::CombatSide::Enemy, 150.0, 200.0, vec![]),
        ];
        let framework_vm = make_replay_framework_combat_vm(1, 1, actors, vec![1, 10]);
        let live_combat_vm = combat_from_framework(&framework_vm).unwrap();

        // Create replay fixture combat VM via same adapter
        let replay_combat_vm = make_replay_combat_vm();

        // Both should be valid CombatViewModels (same structure, potentially different IDs)
        assert!(live_combat_vm.error.is_none(), "Live combat VM should have no error");
        assert!(replay_combat_vm.error.is_none(), "Replay combat VM should have no error");

        // Both should have heroes and monsters
        assert!(!live_combat_vm.heroes.is_empty(), "Live combat VM should have heroes");
        assert!(!replay_combat_vm.heroes.is_empty(), "Replay combat VM should have heroes");
        assert!(!live_combat_vm.monsters.is_empty(), "Live combat VM should have monsters");
        assert!(!replay_combat_vm.monsters.is_empty(), "Replay combat VM should have monsters");
    }

    // ── Provisioning adapter tests ──────────────────────────────────────────

    fn make_provisioning_campaign(gold: u32) -> CampaignState {
        use crate::contracts::{CampaignHero, CampaignHeroQuirks};
        let mut campaign = CampaignState::new(gold);
        campaign.roster.push(CampaignHero {
            id: "h1".to_string(),
            class_id: "crusader".to_string(),
            level: 3,
            xp: 500,
            health: 80.0,
            max_health: 100.0,
            stress: 30.0,
            max_stress: 200.0,
            quirks: CampaignHeroQuirks::new(),
            equipment: Default::default(),
            skills: Vec::new(),
            traits: Default::default(),
        });
        campaign.roster.push(CampaignHero {
            id: "h2".to_string(),
            class_id: "hunter".to_string(),
            level: 2,
            xp: 300,
            health: 100.0,
            max_health: 100.0,
            stress: 200.0,
            max_stress: 200.0,
            quirks: CampaignHeroQuirks::new(),
            equipment: Default::default(),
            skills: Vec::new(),
            traits: Default::default(),
        });
        campaign
    }

    #[test]
    fn provisioning_from_campaign_empty_selection() {
        let campaign = make_provisioning_campaign(1000);
        let result = provisioning_from_campaign(&campaign, &[], "The Depths Await", "Explore the ancient ruins");
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.kind, "provisioning");
        assert_eq!(vm.party.len(), 2);
        assert!(!vm.is_ready_to_launch);
        assert_eq!(vm.supply_level, "None");
        assert_eq!(vm.max_party_size, 4);
    }

    #[test]
    fn provisioning_from_campaign_with_selection() {
        let campaign = make_provisioning_campaign(1000);
        let selected = vec!["h1".to_string()];
        let result = provisioning_from_campaign(&campaign, &selected, "The Depths Await", "Explore the ancient ruins");
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert!(vm.party[0].is_selected);
        assert!(!vm.party[1].is_selected);
        assert!(vm.is_ready_to_launch);
        assert_eq!(vm.provision_cost, "50 Gold");
    }

    #[test]
    fn provisioning_hero_selection_toggle_on() {
        let campaign = make_provisioning_campaign(1000);
        let current = vec!["h1".to_string()];
        let result = provisioning_hero_selection(&campaign, &current, "h2");
        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.len(), 2);
        assert!(updated.contains(&"h1".to_string()));
        assert!(updated.contains(&"h2".to_string()));
    }

    #[test]
    fn provisioning_hero_selection_toggle_off() {
        let campaign = make_provisioning_campaign(1000);
        let current = vec!["h1".to_string(), "h2".to_string()];
        let result = provisioning_hero_selection(&campaign, &current, "h1");
        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.len(), 1);
        assert!(updated.contains(&"h2".to_string()));
    }

    #[test]
    fn provisioning_hero_selection_nonexistent_hero() {
        let campaign = make_provisioning_campaign(1000);
        let result = provisioning_hero_selection(&campaign, &[], "nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn provisioning_hero_selection_max_party_limit() {
        use crate::contracts::CampaignHero;
        use crate::contracts::CampaignHeroQuirks;

        // Create a campaign with many heroes to test the party size limit
        let mut campaign = make_provisioning_campaign(1000);
        // Add more heroes to exceed 4
        for i in 3..=6 {
            campaign.roster.push(CampaignHero {
                id: format!("h{}", i),
                class_id: "alchemist".to_string(),
                level: 1,
                xp: 0,
                health: 100.0,
                max_health: 100.0,
                stress: 0.0,
                max_stress: 200.0,
                quirks: CampaignHeroQuirks::new(),
                equipment: Default::default(),
                skills: Vec::new(),
                traits: Default::default(),
            });
        }
        // Select 4 heroes (the max)
        let current = vec![
            "h1".to_string(), "h2".to_string(),
            "h3".to_string(), "h4".to_string(),
        ];
        // Try to select a 5th hero that exists in roster
        let result = provisioning_hero_selection(&campaign, &current, "h5");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let desc = err.description();
        assert!(desc.contains("cannot select more than 4"));
    }

    // ── Expedition setup and launch adapter tests ───────────────────────────

    #[test]
    fn expedition_setup_from_data_with_selected_heroes() {
        let campaign = make_provisioning_campaign(1000);
        let selected = vec!["h1".to_string(), "h2".to_string()];
        let result = expedition_setup_from_data(&campaign, &selected, None, "Adequate", "100 Gold");
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.kind, "expedition");
        assert_eq!(vm.party_size, 2);
        assert!(vm.is_launchable);
        assert_eq!(vm.party.len(), 2);
        // h1 is wounded (80/100)
        // h2 is afflicted (200/200)
        assert!(vm.warnings.iter().any(|w| w.contains("h1")));
        assert!(vm.warnings.iter().any(|w| w.contains("h2")));
    }

    #[test]
    fn expedition_setup_from_data_with_quest() {
        use crate::contracts::{QuestDefinition, QuestDifficulty, QuestRewards, QuestPenalties, QuestType, MapSize, DungeonType};

        let campaign = make_provisioning_campaign(1000);
        let selected = vec!["h1".to_string()];

        let quest = QuestDefinition::new(
            "kill_boss_expedition",
            QuestType::KillBoss,
            DungeonType::QingLong,
            MapSize::Medium,
            QuestDifficulty::Hard,
            2,
            QuestRewards::hard(),
            QuestPenalties::hard(),
        );

        let result = expedition_setup_from_data(&campaign, &selected, Some(&quest), "Adequate", "50 Gold");
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.expedition_name, "kill_boss_expedition");
        assert_eq!(vm.difficulty, "Hard");
        assert_eq!(vm.estimated_duration, "Medium");
        assert!(!vm.objectives.is_empty());
    }

    #[test]
    fn expedition_setup_from_data_empty_party_not_launchable() {
        let campaign = make_provisioning_campaign(1000);
        let result = expedition_setup_from_data(&campaign, &[], None, "None", "0 Gold");
        assert!(result.is_ok());
        let vm = result.unwrap();
        assert_eq!(vm.party_size, 0);
        assert!(!vm.is_launchable);
    }

    #[test]
    fn expedition_launch_success() {
        use crate::contracts::viewmodels::ExpeditionLaunchRequest;

        let campaign = make_provisioning_campaign(1000);
        let request = ExpeditionLaunchRequest::new(vec!["h1".to_string(), "h2".to_string()]);
        let result = expedition_launch(&campaign, &request);
        assert!(result.success);
        assert_eq!(result.selected_heroes.len(), 2);
        assert_eq!(result.gold_cost, 100); // 2 * 50
        assert_eq!(result.next_state, "dungeon");
    }

    #[test]
    fn expedition_launch_empty_party_fails() {
        use crate::contracts::viewmodels::ExpeditionLaunchRequest;

        let campaign = make_provisioning_campaign(1000);
        let request = ExpeditionLaunchRequest::new(vec![]);
        let result = expedition_launch(&campaign, &request);
        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[test]
    fn expedition_launch_too_many_heroes_fails() {
        use crate::contracts::viewmodels::ExpeditionLaunchRequest;

        let campaign = make_provisioning_campaign(1000);
        let request = ExpeditionLaunchRequest::new(vec![
            "h1".to_string(), "h2".to_string(),
            "h3".to_string(), "h4".to_string(), "h5".to_string(),
        ]);
        let result = expedition_launch(&campaign, &request);
        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[test]
    fn expedition_launch_nonexistent_hero_fails() {
        use crate::contracts::viewmodels::ExpeditionLaunchRequest;

        let campaign = make_provisioning_campaign(1000);
        let request = ExpeditionLaunchRequest::new(vec!["nonexistent".to_string()]);
        let result = expedition_launch(&campaign, &request);
        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[test]
    fn expedition_launch_deducts_gold_from_campaign() {
        use crate::contracts::viewmodels::ExpeditionLaunchRequest;

        let campaign = make_provisioning_campaign(1000);
        let request = ExpeditionLaunchRequest::new(vec!["h1".to_string()]);
        let result = expedition_launch(&campaign, &request);
        assert!(result.success);
        assert_eq!(result.gold_cost, 50); // 1 * 50
    }

    #[test]
    fn expedition_launch_with_quest_sets_dungeon_type() {
        use crate::contracts::viewmodels::ExpeditionLaunchRequest;

        let campaign = make_provisioning_campaign(1000);
        let request = ExpeditionLaunchRequest::new(vec!["h1".to_string()])
            .with_quest("kill_boss_qinglong_short");
        let result = expedition_launch(&campaign, &request);
        assert!(result.success);
        assert_eq!(result.quest_id, Some("kill_boss_qinglong_short".to_string()));
    }

    // ── Provisioning replay fixture tests — US-006-b ────────────────────────

    /// Replay fixture for ProvisioningViewModel.
    fn make_replay_provisioning_vm() -> crate::contracts::viewmodels::ProvisioningViewModel {
        let campaign = make_provisioning_campaign(1500);
        let selected = vec!["h1".to_string()];
        provisioning_from_campaign(&campaign, &selected, "The Depths Await", "Explore the ancient ruins")
            .expect("provisioning_from_campaign should succeed for valid replay fixture")
    }

    /// Replay fixture for ExpeditionSetupViewModel.
    fn make_replay_expedition_setup_vm() -> crate::contracts::viewmodels::ExpeditionSetupViewModel {
        let campaign = make_provisioning_campaign(1500);
        let selected = vec!["h1".to_string(), "h2".to_string()];
        expedition_setup_from_data(&campaign, &selected, None, "Adequate", "100 Gold")
            .expect("expedition_setup_from_data should succeed for valid replay fixture")
    }

    /// Replay fixture for ExpeditionLaunchResult.
    fn make_replay_expedition_launch_result() -> crate::contracts::viewmodels::ExpeditionLaunchResult {
        use crate::contracts::viewmodels::ExpeditionLaunchRequest;

        let campaign = make_provisioning_campaign(1500);
        let request = ExpeditionLaunchRequest::new(vec!["h1".to_string(), "h2".to_string()]);
        expedition_launch(&campaign, &request)
    }

    #[test]
    fn replay_provisioning_fixture_renders_without_error() {
        let vm = make_replay_provisioning_vm();
        assert_eq!(vm.kind, "provisioning");
        assert!(vm.is_ready_to_launch);
        assert_eq!(vm.party.len(), 2);
    }

    #[test]
    fn replay_provisioning_fixture_deterministic() {
        let vm1 = make_replay_provisioning_vm();
        let vm2 = make_replay_provisioning_vm();
        assert_eq!(vm1, vm2, "Provisioning fixture should be deterministic");
    }

    #[test]
    fn replay_expedition_setup_fixture_renders_without_error() {
        let vm = make_replay_expedition_setup_vm();
        assert_eq!(vm.kind, "expedition");
        assert_eq!(vm.party_size, 2);
        assert!(vm.is_launchable);
    }

    #[test]
    fn replay_expedition_setup_fixture_deterministic() {
        let vm1 = make_replay_expedition_setup_vm();
        let vm2 = make_replay_expedition_setup_vm();
        assert_eq!(vm1, vm2, "Expedition setup fixture should be deterministic");
    }

    #[test]
    fn replay_expedition_launch_fixture_succeeds() {
        let result = make_replay_expedition_launch_result();
        assert!(result.success);
        assert_eq!(result.selected_heroes.len(), 2);
    }

    #[test]
    fn replay_expedition_launch_fixture_deterministic() {
        let result1 = make_replay_expedition_launch_result();
        let result2 = make_replay_expedition_launch_result();
        assert_eq!(result1, result2, "Expedition launch fixture should be deterministic");
    }

    #[test]
    fn provisioning_vertical_slice_renders() {
        // Vertical slice: provisioning → expedition setup → launch
        let campaign = make_provisioning_campaign(1500);

        // Step 1: provisioning view
        let selected = vec!["h1".to_string()];
        let prov_vm = provisioning_from_campaign(&campaign, &selected, "The Depths Await", "Explore the ancient ruins")
            .expect("provisioning should succeed");
        assert_eq!(prov_vm.party.len(), 2);
        assert!(prov_vm.is_ready_to_launch);

        // Step 2: hero selection toggle (add h2)
        let updated_selection = provisioning_hero_selection(&campaign, &selected, "h2")
            .expect("hero selection should succeed");
        assert_eq!(updated_selection.len(), 2);

        // Step 3: expedition setup
        let setup_vm = expedition_setup_from_data(&campaign, &updated_selection, None, "Adequate", "100 Gold")
            .expect("expedition setup should succeed");
        assert_eq!(setup_vm.party_size, 2);
        assert!(setup_vm.is_launchable);

        // Step 4: launch expedition
        use crate::contracts::viewmodels::ExpeditionLaunchRequest;
        let launch_result = expedition_launch(
            &campaign,
            &ExpeditionLaunchRequest::new(updated_selection),
        );
        assert!(launch_result.success);
        assert_eq!(launch_result.selected_heroes.len(), 2);
        assert_eq!(launch_result.next_state, "dungeon");
    }
}