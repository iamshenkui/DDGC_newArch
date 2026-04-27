//! DDGC frontend application host — contracts-layer entrypoint.
//!
//! This module provides the [`DdgcHost`] struct, which is the canonical
//! application host for the DDGC headless migration. The host boots from
//! approved contract packages (JSON/CSV data files) and exposes a clean API
//! for starting a game in either replay-driven or live-runtime mode.
//!
//! # Startup modes
//!
//! - **Replay-driven startup**: Load a saved [`CampaignState`] from JSON and resume.
//!   Use [`DdgcHost::boot_from_campaign`] for this path.
//!
//! - **Live-runtime startup**: Start a fresh campaign with initial configuration.
//!   Use [`DdgcHost::boot_live`] for this path.
//!
//! # Error handling
//!
//! All boot operations return a dedicated [`HostError`] variant rather than
//! panicking or silently using fallback values. Call [`DdgcHost::error_message`]
//! to get a human-readable description of any boot error.
//!
//! # No simulation internals
//!
//! The host operates exclusively on contracts-layer types (registries, data models,
//! and [`CampaignState`]). It does not read framework internals like `ActorId`,
//! `EncounterId`, or `Run` directly.

use std::path::Path;

use crate::contracts::parse::{
    parse_buildings_json, parse_camping_json, parse_curios_csv, parse_obstacles_json,
    parse_quirks_json, parse_traits_json, parse_traps_json,
};
use crate::contracts::{
    BuffRegistry, BuildingRegistry, CampaignState, CampingSkillRegistry, CurioRegistry,
    DungeonEncounterRegistry, EquipmentRegistry, MapSize, ObstacleRegistry, QuestRegistry,
    QuirkRegistry, TraitRegistry, TrapRegistry, TrinketRegistry,
};

/// The runtime phase of the host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostPhase {
    /// Host is initialized but not yet booted.
    Uninitialized,
    /// Host is booting (contract packages loading).
    Booting,
    /// Host is ready to run.
    Ready,
    /// Host encountered a fatal error and cannot proceed.
    FatalError,
    /// Host is unsupported (feature not available in this build).
    Unsupported,
}

impl Default for HostPhase {
    fn default() -> Self {
        HostPhase::Uninitialized
    }
}

/// Startup mode for the DDGC host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartupMode {
    /// Boot from a saved campaign state (replay-driven).
    Replay,
    /// Boot a fresh campaign (live-runtime).
    Live,
}

impl std::fmt::Display for StartupMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StartupMode::Replay => write!(f, "replay"),
            StartupMode::Live => write!(f, "live"),
        }
    }
}

/// Error variants for host boot operations.
///
/// Each variant carries context about what went wrong so callers
/// can present meaningful diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostError {
    /// Contract data directory not found or not accessible.
    DataDirectoryNotFound {
        path: String,
        message: String,
    },
    /// Failed to parse a contract data file.
    ContractParse {
        file: String,
        message: String,
    },
    /// Campaign state could not be deserialized.
    CampaignLoadFailed {
        message: String,
    },
    /// Campaign state schema version is unsupported.
    UnsupportedCampaignSchema {
        found_version: u32,
        supported_version: u32,
    },
    /// Live startup validation failed (e.g., invalid initial config).
    InvalidInitialConfig {
        reason: String,
    },
    /// Feature is not available in this build.
    FeatureNotSupported {
        feature: String,
    },
    /// Host is in an invalid state for the requested operation.
    InvalidHostState {
        actual: HostPhase,
        expected: &'static str,
    },
}

impl HostError {
    /// Returns a human-readable description of this error.
    pub fn description(&self) -> String {
        match self {
            HostError::DataDirectoryNotFound { path, message } => {
                format!("data directory not found: {} ({})", path, message)
            }
            HostError::ContractParse { file, message } => {
                format!("failed to parse contract file '{}': {}", file, message)
            }
            HostError::CampaignLoadFailed { message } => {
                format!("failed to load campaign state: {}", message)
            }
            HostError::UnsupportedCampaignSchema {
                found_version,
                supported_version,
            } => {
                format!(
                    "unsupported campaign schema version {} (supported: {})",
                    found_version, supported_version
                )
            }
            HostError::InvalidInitialConfig { reason } => {
                format!("invalid initial configuration: {}", reason)
            }
            HostError::FeatureNotSupported { feature } => {
                format!("feature not supported: {}", feature)
            }
            HostError::InvalidHostState { actual, expected } => {
                format!("host in invalid state: got {:?}, expected {}", actual, expected)
            }
        }
    }
}

impl std::fmt::Display for HostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl std::error::Error for HostError {}

/// Result type for host operations.
pub type HostResult<T> = Result<T, HostError>;

/// Configuration for live-runtime (fresh) campaign startup.
#[derive(Debug, Clone)]
pub struct LiveConfig {
    /// Starting gold for the new campaign.
    pub starting_gold: u32,
    /// Initial map size for dungeon runs.
    pub default_map_size: MapSize,
}

impl Default for LiveConfig {
    fn default() -> Self {
        LiveConfig {
            starting_gold: 500,
            default_map_size: MapSize::Short,
        }
    }
}

/// Configuration for replay-driven (saved state) campaign startup.
#[derive(Debug, Clone)]
pub struct ReplayConfig<'a> {
    /// The serialized campaign state JSON.
    pub campaign_json: &'a str,
    /// Path to the save file (for error reporting).
    pub source_path: &'a str,
}

/// DDGC frontend application host — boots from contract packages and exposes
/// a clean API for game startup.
///
/// The host holds all contracts-layer registries and provides explicit
/// methods for starting in either replay-driven or live-runtime mode.
///
/// # Example
///
/// ```
/// use game_ddgc_headless::contracts::host::{DdgcHost, LiveConfig};
///
/// let config = LiveConfig::default();
/// let host = DdgcHost::boot_live(&config).expect("failed to boot host");
/// assert!(host.is_ready());
/// ```
#[derive(Debug, Clone)]
pub struct DdgcHost {
    /// Current runtime phase.
    phase: HostPhase,
    /// Startup mode that was used.
    startup_mode: Option<StartupMode>,
    /// Contract registries.
    pub curio_registry: CurioRegistry,
    pub trap_registry: TrapRegistry,
    pub obstacle_registry: ObstacleRegistry,
    pub building_registry: BuildingRegistry,
    pub quest_registry: QuestRegistry,
    pub trinket_registry: TrinketRegistry,
    pub quirk_registry: QuirkRegistry,
    pub trait_registry: TraitRegistry,
    pub camping_skill_registry: CampingSkillRegistry,
    pub dungeon_encounter_registry: DungeonEncounterRegistry,
    pub equipment_registry: EquipmentRegistry,
    pub buff_registry: BuffRegistry,
    /// Campaign state (loaded or created on boot).
    campaign: Option<CampaignState>,
    /// Last error that caused a fatal error (if any).
    last_error: Option<HostError>,
}

impl Default for DdgcHost {
    fn default() -> Self {
        DdgcHost::new()
    }
}

impl DdgcHost {
    /// Create a new uninitialized host.
    pub fn new() -> Self {
        DdgcHost {
            phase: HostPhase::Uninitialized,
            startup_mode: None,
            curio_registry: CurioRegistry::new(),
            trap_registry: TrapRegistry::new(),
            obstacle_registry: ObstacleRegistry::new(),
            building_registry: BuildingRegistry::new(),
            quest_registry: QuestRegistry::new(),
            trinket_registry: TrinketRegistry::new(),
            quirk_registry: QuirkRegistry::new(),
            trait_registry: TraitRegistry::new(),
            camping_skill_registry: CampingSkillRegistry::new(),
            dungeon_encounter_registry: DungeonEncounterRegistry::new(),
            equipment_registry: EquipmentRegistry::new(),
            buff_registry: BuffRegistry::new(),
            campaign: None,
            last_error: None,
        }
    }

    /// Returns the current runtime phase of the host.
    pub fn phase(&self) -> HostPhase {
        self.phase.clone()
    }

    /// Returns the startup mode that was used to boot this host.
    pub fn startup_mode(&self) -> Option<StartupMode> {
        self.startup_mode.clone()
    }

    /// Returns whether the host is in the `Ready` phase.
    pub fn is_ready(&self) -> bool {
        self.phase == HostPhase::Ready
    }

    /// Returns the campaign state if the host is ready.
    pub fn campaign(&self) -> Option<&CampaignState> {
        self.campaign.as_ref()
    }

    /// Returns the last error if the host is in `FatalError` phase.
    pub fn last_error(&self) -> Option<&HostError> {
        self.last_error.as_ref()
    }

    /// Returns a human-readable error message for the last error.
    ///
    /// Returns `None` if there is no error.
    pub fn error_message(&self) -> Option<String> {
        self.last_error.as_ref().map(|e| e.description())
    }

    /// Boot the host in live-runtime mode with the given configuration.
    ///
    /// This loads all contract packages from the default data directory
    /// and initializes a fresh campaign state.
    pub fn boot_live(config: &LiveConfig) -> HostResult<Self> {
        let mut host = DdgcHost::new();
        host.phase = HostPhase::Booting;

        // Load contract packages from the data directory.
        host.load_contract_packages()?;

        // Create a new campaign state.
        let campaign = CampaignState::new(config.starting_gold);
        host.campaign = Some(campaign);

        host.startup_mode = Some(StartupMode::Live);
        host.phase = HostPhase::Ready;

        Ok(host)
    }

    /// Boot the host in replay-driven mode from a saved campaign state.
    ///
    /// This loads all contract packages and then deserializes the campaign
    /// state from the provided JSON string.
    pub fn boot_from_campaign(config: &ReplayConfig<'_>) -> HostResult<Self> {
        let mut host = DdgcHost::new();
        host.phase = HostPhase::Booting;

        // Load contract packages from the data directory.
        host.load_contract_packages()?;

        // Deserialize campaign state.
        let campaign: CampaignState = serde_json::from_str(config.campaign_json).map_err(|e| {
            HostError::CampaignLoadFailed {
                message: format!("{} (source: {})", e, config.source_path),
            }
        })?;

        // Validate schema version.
        if let Err(_msg) = campaign.validate_version() {
            return Err(HostError::UnsupportedCampaignSchema {
                found_version: campaign.schema_version,
                supported_version: crate::contracts::CAMPAIGN_SNAPSHOT_VERSION,
            });
        }

        host.campaign = Some(campaign);
        host.startup_mode = Some(StartupMode::Replay);
        host.phase = HostPhase::Ready;

        Ok(host)
    }

    /// Load all contract packages from the default data directory.
    ///
    /// The default data directory is `data/` relative to the current working
    /// directory.
    fn load_contract_packages(&mut self) -> HostResult<()> {
        self.load_contract_packages_from(Path::new("data"))
    }

    /// Load all contract packages from the specified directory.
    fn load_contract_packages_from(&mut self, data_dir: &Path) -> HostResult<()> {
        if !data_dir.exists() {
            return Err(HostError::DataDirectoryNotFound {
                path: data_dir.display().to_string(),
                message: "directory does not exist".to_string(),
            });
        }

        // Load Curios.csv
        let curios_path = data_dir.join("Curios.csv");
        if curios_path.exists() {
            match parse_curios_csv(&curios_path) {
                Ok(registry) => self.curio_registry = registry,
                Err(e) => {
                    return Err(HostError::ContractParse {
                        file: curios_path.display().to_string(),
                        message: e,
                    });
                }
            }
        }

        // Load Traps.json
        let traps_path = data_dir.join("Traps.json");
        if traps_path.exists() {
            match parse_traps_json(&traps_path) {
                Ok(registry) => self.trap_registry = registry,
                Err(e) => {
                    return Err(HostError::ContractParse {
                        file: traps_path.display().to_string(),
                        message: e,
                    });
                }
            }
        }

        // Load Obstacles.json
        let obstacles_path = data_dir.join("Obstacles.json");
        if obstacles_path.exists() {
            match parse_obstacles_json(&obstacles_path) {
                Ok(registry) => self.obstacle_registry = registry,
                Err(e) => {
                    return Err(HostError::ContractParse {
                        file: obstacles_path.display().to_string(),
                        message: e,
                    });
                }
            }
        }

        // Load Buildings.json
        let buildings_path = data_dir.join("Buildings.json");
        if buildings_path.exists() {
            match parse_buildings_json(&buildings_path) {
                Ok(registry) => self.building_registry = registry,
                Err(e) => {
                    return Err(HostError::ContractParse {
                        file: buildings_path.display().to_string(),
                        message: e,
                    });
                }
            }
        }

        // Load JsonQuirks.json
        let quirks_path = data_dir.join("JsonQuirks.json");
        if quirks_path.exists() {
            match parse_quirks_json(&quirks_path) {
                Ok(registry) => self.quirk_registry = registry,
                Err(e) => {
                    return Err(HostError::ContractParse {
                        file: quirks_path.display().to_string(),
                        message: e,
                    });
                }
            }
        }

        // Load JsonTraits.json
        let traits_path = data_dir.join("JsonTraits.json");
        if traits_path.exists() {
            match parse_traits_json(&traits_path) {
                Ok(registry) => self.trait_registry = registry,
                Err(e) => {
                    return Err(HostError::ContractParse {
                        file: traits_path.display().to_string(),
                        message: e,
                    });
                }
            }
        }

        // Load JsonCamping.json
        let camping_path = data_dir.join("JsonCamping.json");
        if camping_path.exists() {
            match parse_camping_json(&camping_path) {
                Ok(registry) => self.camping_skill_registry = registry,
                Err(e) => {
                    return Err(HostError::ContractParse {
                        file: camping_path.display().to_string(),
                        message: e,
                    });
                }
            }
        }

        Ok(())
    }

    /// Mark the host as unsupported with the given feature name.
    ///
    /// This transitions the host to `HostPhase::Unsupported`.
    pub fn mark_unsupported(mut self, feature: &str) -> Self {
        self.phase = HostPhase::Unsupported;
        self.last_error = Some(HostError::FeatureNotSupported {
            feature: feature.to_string(),
        });
        self
    }

    /// Mark the host as having encountered a fatal error.
    ///
    /// This transitions the host to `HostPhase::FatalError` and stores
    /// the error for later retrieval via [`DdgcHost::error_message`].
    pub fn mark_fatal(mut self, error: HostError) -> Self {
        self.phase = HostPhase::FatalError;
        self.last_error = Some(error);
        self
    }

    /// Assert that the host is in the `Ready` phase, returning an error if not.
    pub fn assert_ready(&self) -> HostResult<()> {
        if self.phase != HostPhase::Ready {
            return Err(HostError::InvalidHostState {
                actual: self.phase.clone(),
                expected: "Ready",
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_starts_uninitialized() {
        let host = DdgcHost::new();
        assert_eq!(host.phase(), HostPhase::Uninitialized);
        assert!(host.startup_mode().is_none());
        assert!(!host.is_ready());
        assert!(host.campaign().is_none());
        assert!(host.error_message().is_none());
    }

    #[test]
    fn host_error_descriptions_are_meaningful() {
        let errors = vec![
            HostError::DataDirectoryNotFound {
                path: "/nonexistent".to_string(),
                message: "permission denied".to_string(),
            },
            HostError::ContractParse {
                file: "Curios.csv".to_string(),
                message: "missing required field".to_string(),
            },
            HostError::CampaignLoadFailed {
                message: "JSON parse error".to_string(),
            },
            HostError::UnsupportedCampaignSchema {
                found_version: 99,
                supported_version: 1,
            },
            HostError::InvalidInitialConfig {
                reason: "starting gold cannot be negative".to_string(),
            },
            HostError::FeatureNotSupported {
                feature: "multiplayer".to_string(),
            },
            HostError::InvalidHostState {
                actual: HostPhase::Uninitialized,
                expected: "Ready",
            },
        ];

        for error in errors {
            let desc = error.description();
            assert!(!desc.is_empty(), "error {:?} had empty description", error);
        }
    }

    #[test]
    fn live_config_has_sensible_defaults() {
        let config = LiveConfig::default();
        assert_eq!(config.starting_gold, 500);
        assert_eq!(config.default_map_size, MapSize::Short);
    }

    #[test]
    fn startup_mode_display() {
        assert_eq!(StartupMode::Replay.to_string(), "replay");
        assert_eq!(StartupMode::Live.to_string(), "live");
    }

    #[test]
    fn host_phase_default_is_uninitialized() {
        let phase = HostPhase::default();
        assert_eq!(phase, HostPhase::Uninitialized);
    }

    #[test]
    fn host_clone_is_independent() {
        let host1 = DdgcHost::new();
        let host2 = host1.clone();
        assert_eq!(host1.phase(), host2.phase());
        assert_eq!(host1.startup_mode(), host2.startup_mode());
    }
}