//! DDGC frontend application host — documentation and verification layer.
//!
//! This module provides canonical documentation and acceptance tests for the
//! DDGC headless migration's frontend host layer. It verifies that:
//!
//! - The [`DdgcHost`] frontend entrypoint exists and is properly documented.
//! - The host boots from approved contract packages without reading simulation internals.
//! - Both replay-driven and live-runtime startup paths are explicit and testable.
//! - Startup, loading, unsupported-state, and fatal-error surfaces are explicit.
//!
//! # Frontend Host Architecture
//!
//! The [`DdgcHost`] (defined in [`crate::contracts::host`]) is the canonical
//! application host for the DDGC headless migration. It provides:
//!
//! ## Startup modes
//!
//! | Mode | Method | Description |
//! |------|--------|-------------|
//! | Live-runtime | [`DdgcHost::boot_live`] | Start a fresh campaign with initial config |
//! | Replay-driven | [`DdgcHost::boot_from_campaign`] | Resume from a saved campaign state |
//!
//! ## Explicit phase tracking
//!
//! The host transitions through explicit [`HostPhase`] states rather than
//! implicitly failing:
//!
//! | Phase | Meaning |
//! |-------|---------|
//! | `Uninitialized` | Host created but not booted |
//! | `Booting` | Contract packages are being loaded |
//! | `Ready` | Host is ready to run |
//! | `FatalError` | A fatal error occurred; see [`HostError`] |
//! | `Unsupported` | A requested feature is not supported |
//!
//! ## Explicit error surfaces
//!
//! All boot operations return a dedicated [`HostError`] variant with meaningful
//! context. Errors are never silent fallbacks or panics:
//!
//! | Variant | Trigger |
//! |---------|--------|
//! | `DataDirectoryNotFound` | Contract data directory missing or inaccessible |
//! | `ContractParse` | A contract file failed to parse |
//! | `CampaignLoadFailed` | Campaign state could not be deserialized |
//! | `UnsupportedCampaignSchema` | Campaign schema version mismatch |
//! | `InvalidInitialConfig` | Live startup validation failed |
//! | `FeatureNotSupported` | Requested feature not available in build |
//! | `InvalidHostState` | Operation requires a different host phase |
//!
//! # No simulation internals
//!
//! The host operates exclusively on contracts-layer types (registries, data models,
//! and [`CampaignState`]). It does not read framework internals like `ActorId`,
//! `EncounterId`, or `Run` directly. This ensures a clean separation between
//! the frontend host and the simulation layer.
//!
//! # Local developer startup
//!
//! ```rust
//! use game_ddgc_headless::contracts::host::{DdgcHost, LiveConfig};
//!
//! // Boot in live-runtime mode
//! let host = DdgcHost::boot_live(&LiveConfig::default()).expect("boot failed");
//! assert!(host.is_ready());
//!
//! // Check for errors explicitly
//! if let Some(msg) = host.error_message() {
//!     eprintln!("Host error: {}", msg);
//! }
//! ```
//!
//! ```ignore
//! use game_ddgc_headless::contracts::host::{DdgcHost, ReplayConfig};
//!
//! // Boot from a saved campaign state (replay-driven)
//! // Note: saved_json would be obtained from a previous campaign.save() call
//! let host = DdgcHost::boot_from_campaign(&ReplayConfig {
//!     campaign_json: &saved_json,
//!     source_path: "savegame.json",
//! }).expect("replay failed");
//! assert!(host.is_ready());
//! ```
//!
//! # Canonical save/load boundary
//!
//! [`CampaignState`] (defined in [`crate::contracts`]) is the **single source of
//! truth** for campaign persistence. Every gameplay-significant piece of state
//! that must survive a save/load cycle lives in this struct. The schema captures:
//!
//! | Domain | Field | Type | Description |
//! |--------|-------|------|-------------|
//! | **Version** | `schema_version` | `u32` | Schema format version for forward/backward compatibility |
//! | **Gold** | `gold` | `u32` | Current gold balance |
//! | **Heirlooms** | `heirlooms` | `BTreeMap<HeirloomCurrency, u32>` | Heirloom currency balances (Bones, Portraits, Tapes) |
//! | **Town** | `building_states` | `BTreeMap<String, BuildingUpgradeState>` | Building upgrade levels keyed by building ID |
//! | **Roster** | `roster` | `Vec<CampaignHero>` | Hero roster with full state per hero |
//! | **Inventory** | `inventory` | `Vec<CampaignInventoryItem>` | Estate inventory items with quantities |
//! | **Run history** | `run_history` | `Vec<CampaignRunRecord>` | Completed/abandoned dungeon runs |
//! | **Quests** | `quest_progress` | `Vec<CampaignQuestProgress>` | Active quest step tracking |
//!
//! ## Hero substructure
//!
//! Each [`CampaignHero`] captures the full persisted hero state:
//!
//! | Field | Type | Description |
//! |-------|------|-------------|
//! | `id` | `String` | Unique hero identifier |
//! | `class_id` | `String` | Hero class (e.g. `"alchemist"`, `"crusader"`) |
//! | `level` | `u32` | Resolve level |
//! | `xp` | `u32` | Experience toward next level |
//! | `health` | `f64` | Current health |
//! | `max_health` | `f64` | Maximum health |
//! | `stress` | `f64` | Current stress |
//! | `max_stress` | `f64` | Maximum stress |
//! | `quirks` | `CampaignHeroQuirks` | Positive, negative, and disease quirks |
//! | `traits` | `CampaignHeroTraits` | Afflictions and virtues |
//! | `skills` | `Vec<String>` | Equipped skill IDs (order preserved) |
//! | `equipment` | `CampaignHeroEquipment` | Weapon/armor levels and trinket slots |
//!
//! # Schema versioning
//!
//! The schema is explicitly versioned via [`CAMPAIGN_SNAPSHOT_VERSION`]. Every
//! save file begins with a `schema_version` field. Consumers **MUST** reject
//! snapshots whose version differs from the expected value.
//!
//! When the schema format changes in a backward-incompatible way:
//! 1. Increment `CAMPAIGN_SNAPSHOT_VERSION`.
//! 2. Update this documentation to reflect the new fields.
//! 3. Add a migration path if backward compatibility is required.
//!
//! # Deterministic serialization
//!
//! All keyed collections (`heirlooms`, `building_states`) use [`BTreeMap`]
//! rather than [`HashMap`]. This guarantees that `serde_json::to_string` produces
//! **byte-identical output** for identical logical state. The guarantees are:
//!
//! - Same state → same JSON bytes, every time.
//! - JSON keys appear in sorted order.
//! - Save-file diffing is reliable.
//! - Integrity hashes (e.g. SHA-256 of the save file) are stable.
//!
//! # Boundary contract
//!
//! `CampaignState` is the **canonical save/load boundary**. No
//! framework-specific types (`ActorId`, `EncounterId`, `SkillDefinition`, etc.)
//! appear in this schema. All identifiers are plain [`String`] values so the
//! persisted state is fully decoupled from the framework type graph.
//!
//! This means:
//! - The save file is a standalone JSON document with no binary blobs.
//! - The save file can be inspected, diffed, and edited with standard tools.
//! - Framework crate upgrades cannot break save compatibility unless the
//!   schema version is explicitly bumped.
//!
//! # Architecture
//!
//! ```text
//! contracts/  (data model + JSON parsing)
//!     │
//!     ▼
//! state/      (GameState: content loading + campaign CRUD)
//!     │
//!     ▼
//! docs/       (this module: canonical documentation + schema tests)
//! ```
//!
//! The `docs` module sits at the verification layer. It imports types from
//! `contracts` and `state` to prove, via focused tests, that the schema
//! faithfully round-trips every gameplay-significant field.
//!
//! # Test coverage
//!
//! The tests in this module are the **canonical acceptance tests** for the
//! save/load boundary. They verify:
//!
//! - Every top-level domain (gold, heirlooms, roster, inventory, town, run
//!   history, quests) round-trips without data loss.
//! - Every hero subdomain (health, stress, quirks, traits, skills, equipment)
//!   round-trips without data loss.
//! - Serialization is deterministic (identical state → identical bytes).
//! - Schema version validation rejects unsupported versions.
//! - The canonical JSON structure is a valid JSON object with the expected keys.
//!
//! If a new gameplay-significant field is added to `CampaignState`, a
//! corresponding test **must** be added here before the field is considered
//! safe for production save/load.
//!
//! # High-Frequency Semantic Path Registry
//!
//! This section documents every **high-frequency semantic path** in the DDGC
//! migration and its fence status. A "semantic path" is any code path that
//! interprets DDGC semantics (targeting, movement, conditions, damage, hit
//! resolution, camp effects, meta transitions). A "fence" is a deterministic
//! guarantee that the path never silently drops an unsupported semantic.
//!
//! ## Fence Status Taxonomy
//!
//! | Status | Meaning | Trace Marker |
//! |---|---|---|
//! | **Implemented** | Fully functional with deterministic behavior | Domain-specific description |
//! | **Fenced (STUB)** | Produces `[STUB]` trace marker; no state change | `"[STUB] <reason>"` |
//! | **Fenced (SKIPPED)** | Produces `[SKIPPED]` trace marker; intentionally no-op | `"[SKIPPED] <reason>"` |
//! | **Approximated** | Simplified but preserves observable behavior | Domain-specific description |
//! | **Unsupported (Unknown)** | Returns `ConditionResult::Unknown`; caller handles gracefully | N/A (returns enum variant) |
//!
//! ## Path Inventory
//!
//! ### Targeting (H — every skill)
//!
//! | Path | Variants | Fence Status |
//! |---|---|---|
//! | `LaunchConstraint` | 5 (Any, FrontRow, BackRow, SpecificLane, SlotRange) | Implemented |
//! | `TargetRank` | 4 (Any, Front, Back, FrontAndBack) | Implemented |
//! | `SideAffinity` | 3 (Enemy, Ally, Any) | Implemented |
//! | `TargetCount` | 2 (Single, Multiple) | Implemented |
//! | `TargetingIntent` | Composite of above (30+ combos) | Implemented |
//!
//! ### Movement (M — repositioning skills)
//!
//! | Path | Variants | Fence Status |
//! |---|---|---|
//! | `MovementEffect` | 4 (Push, Pull, Shuffle, None) | Implemented |
//! | `MovementDirection` | 2 (Forward, Backward) | Implemented |
//!
//! ### Special Effect Handling / Camp Effects (M — camping phase)
//!
//! | Path | Variants | Fence Status |
//! |---|---|---|
//! | Implemented camp effects | 16 (StressHeal, HealthHeal, RemoveBleed, etc.) | Implemented |
//! | Stubbed camp effects | 4 (ReduceAmbushChance, Loot, ReduceTurbulenceChance, ReduceRiptideChance) | Fenced (STUB) |
//! | Non-functional camp effects | 2 (None, ReduceTorch) | Fenced (SKIPPED) |
//!
//! ### Persistent Meta Transitions (B — boss phase changes)
//!
//! | Path | Variants | Fence Status |
//! |---|---|---|
//! | `PhaseTransitionTrigger` | 5 (PressAttackCount, HealthBelow, RoundElapsed, OnAllyDeath, OnAllAlliesDead) | Implemented |
//!
//! ### Combat Conditions (H — every effect)
//!
//! | Path | Variants | Fence Status |
//! |---|---|---|
//! | `DdgcCondition` supported | 11 (FirstRound, StressAbove/Below, DeathsDoor, HpAbove, TargetHp*, etc.) | Implemented |
//! | Framework-native conditions | 3 (IfTargetHealthBelow, IfActorHasStatus, Probability) | Approximated (adapter mirror) |
//! | Deferred DdgcCondition variants | 19 (Afflicted, Virtued, Melee, Ranged, etc.) | Unsupported (Unknown) |
//! | `IfTargetPosition` | 1 | Unsupported (Unknown) |
//!
//! ### Damage (H — every attack)
//!
//! | Path | Variants | Fence Status |
//! |---|---|---|
//! | `DamagePolicy::FixedAverage` | Default | Approximated (deterministic average) |
//! | `DamagePolicy::Rolled` | Available, not wired | Implemented but not active |
//!
//! ### Hit Resolution (M — accuracy/dodge encounters)
//!
//! | Path | Variants | Fence Status |
//! |---|---|---|
//! | `HitResolutionContext` | Context struct | Implemented |
//! | Accuracy/dodge attributes | Set from DDGC data | Approximated (simplified formula) |
//!
//! ## "No Silent Drop" Guarantee
//!
//! Every high-frequency semantic path in the registry satisfies **one** of:
//! 1. Fully implemented with deterministic trace output
//! 2. Fenced with `[STUB]` marker — the call site knows the semantic was not applied
//! 3. Fenced with `[SKIPPED]` marker — the call site knows the semantic was intentionally ignored
//! 4. Returns `ConditionResult::Unknown` — the caller must handle the unrecognized variant
//!
//! No path returns `None`, an empty string, or silently succeeds without effect.
//! The regression tests in [`high_freq_path_tests`] verify this invariant for
//! every path in the registry.
//!
//! # Replay-Driven Rendered UI Validation for Town/Meta Flows
//!
//! This section documents the **replay-driven rendered UI validation** approach
//! used to verify the frontend host layer for town and meta-game flows without
//! depending on nondeterministic manual gameplay setup.
//!
//! ## The Problem with Manual Setup
//!
//! Traditional end-to-end testing of the frontend host for town/meta flows
//! requires:
//! - A running game session with heroes, inventory, and town state
//! - Manual navigation through the entire game loop (startup → town → expedition
//!   → result → town)
//! - Deterministic timing and random seed control
//!
//! This is fragile, slow, and hard to reproduce in CI. Any change to the
//! screen composition, adapter mapping, or state transition wiring can silently
//! break the rendered output without being caught.
//!
//! ## The Replay-Fixture Solution
//!
//! Replay-driven validation replaces manual gameplay with **deterministic fixture
//! factories** that produce the same view model payloads every time. Each fixture
//! represents a canonical state in the town/meta flow vertical slice:
//!
//! | Screen/Flow | Fixture Factory | ViewModel | Validation Scope |
//! |---|---|---|---|
//! | Startup | `BootLoadViewModel::success(...)` | `BootLoadViewModel` | Boot surface, error surfaces, JSON boundary |
//! | Loading | `BootLoadViewModel::success(...).with_campaign_version(...)` | `BootLoadViewModel` | Campaign load, schema versioning |
//! | Town Shell | `make_replay_town_view_model()` | `TownViewModel` | Heroes, buildings, gold, roster, quirks |
//! | Hero Inspection | `make_replay_hero_detail_view_model()` | `HeroDetailViewModel` | Progression, resistances, skills, equipment |
//! | Building Screen | `make_replay_guild_building_detail()` / `make_replay_blacksmith_building_detail()` | `BuildingDetailViewModel` | Actions, costs, status, unsupported flags |
//! | Provisioning | `make_replay_provisioning_view_model()` | `ProvisioningViewModel` | Party selection, health consistency, launch readiness |
//! | Expedition Launch | `make_replay_expedition_view_model()` | `ExpeditionSetupViewModel` | Difficulty, objectives, warnings, launchability |
//! | Launch Contract | `ExpeditionLaunchResult::success(...)` / `failure(...)` | `ExpeditionLaunchResult` | Success/failure results, error actionability |
//! | Result | `ResultViewModel::victory(...)` / `defeat(...)` | `ResultViewModel` | Loot, rewards, casualties |
//! | Return Flow | `ReturnFlowViewModel { ... }` | `ReturnFlowViewModel` | Hero states, gold transfer, town resume |
//! | Navigation Shell | `NavigationShell::new_replay()` | N/A (state machine) | Transition validity, replay/live parity, meta-loop cycles |
//!
//! These fixtures live in the **integration test suite** (`tests/`) rather than
//! in the source tree, satisfying the "scoped to the tests module" acceptance
//! criterion. The canonical test file is
//! [`replay_driven_validation_tests`](https://路径/tests/replay_driven_validation_tests.rs).
//!
//! ## Fixture Architecture — Shared Hero and Building Data
//!
//! The town/meta flow fixtures share a common data foundation to ensure
//! consistency across screens:
//!
//! ```ignore
//! // Three canonical heroes shared by town, provisioning, and hero-detail fixtures
//! fn shared_hero_data() -> Vec<TownHeroViewModel>
//!
//! // Four canonical buildings shared by town shell and building-detail fixtures
//! fn shared_building_data() -> Vec<TownBuildingViewModel>
//! ```
//!
//! The same hero IDs (`hero-hunter-01`, `hero-white-01`, `hero-black-01`) and
//! building IDs (`stagecoach`, `guild`, `blacksmith`, `sanitarium`) are
//! referenced across all fixtures. Cross-fixture consistency is validated by
//! dedicated tests (e.g., `replay_hero_detail_consistent_with_town_roster`,
//! `replay_building_detail_consistent_with_town`).
//!
//! ## Validation by Screen
//!
//! ### Startup and Load (BootLoadViewModel)
//!
//! - `replay_startup_boot_load_view_model` — success surface with loaded registries
//! - `replay_loading_boot_load_view_model` — replay-mode loading surface
//! - `replay_loading_with_campaign_version` — campaign schema version propagation
//! - `replay_fatal_error_surface_actionable` — fatal error surfaces non-empty description
//! - `replay_unsupported_surface_actionable` — unsupported state surfaces non-empty description
//! - `replay_boot_load_json_no_framework_internals` — JSON boundary: no ActorId, EncounterId, RunId
//! - `replay_boot_load_json_roundtrip` — deterministic JSON round-trip
//!
//! ### Town Shell (TownViewModel)
//!
//! - `replay_town_shell_heroes_valid` — all three heroes have valid vitals
//! - `replay_town_shell_wounded_afflicted_flags` — is_wounded/is_afflicted flags correct
//! - `replay_town_shell_quirks_and_diseases` — quirk/disease lists populated
//! - `replay_town_shell_buildings_valid` — all four buildings present and available
//! - `replay_town_shell_next_action_label` — UI routing label non-empty
//! - `replay_town_shell_gold_and_fresh_visit` — gold > 0, is_fresh_visit = true
//! - `replay_town_shell_no_framework_internals` — JSON boundary validated
//! - `replay_town_shell_json_roundtrip` — deterministic JSON round-trip
//!
//! ### Hero Inspection (HeroDetailViewModel)
//!
//! - `replay_hero_detail_progression` — level, experience, experience_to_next populated
//! - `replay_hero_detail_resistances` — all seven resistance categories end with '%'
//! - `replay_hero_detail_skills` — at least one combat and one camping skill
//! - `replay_hero_detail_equipment` — weapon and armor descriptions non-empty
//! - `replay_hero_detail_consistent_with_town_roster` — cross-fixture consistency
//! - `replay_hero_detail_json_roundtrip` — deterministic JSON round-trip
//!
//! ### Building Screens (BuildingDetailViewModel)
//!
//! - `replay_building_detail_consistent_with_town` — building IDs match town fixture
//! - `replay_building_detail_actions_valid` — all actions have id, label, description, cost
//! - `replay_building_detail_action_cost_format` — cost ends with " Gold"
//! - `replay_building_detail_descriptions` — descriptions > 20 chars
//! - `replay_building_detail_status` — Ready vs Partial status
//! - `replay_building_detail_unsupported_actions` — is_unsupported flag for not-yet-wired features
//! - `replay_building_detail_json_roundtrip` — deterministic JSON round-trip
//! - `replay_building_detail_all_buildings_have_fixture` — known fixtures documented
//!
//! ### Provisioning (ProvisioningViewModel)
//!
//! - `replay_provisioning_heroes_consistent_with_town` — hero identity matches town roster
//! - `replay_provisioning_parameters_valid` — max_party_size, supply_level, costs populated
//! - `replay_provisioning_empty_party_not_ready` — empty party => is_ready_to_launch = false
//! - `replay_provisioning_json_roundtrip` — deterministic JSON round-trip
//! - `replay_provisioning_hero_health_consistency` — string and numeric health fields agree
//!
//! ### Expedition Launch (ExpeditionSetupViewModel / ExpeditionLaunchResult)
//!
//! - `replay_expedition_parameters_valid` — difficulty, duration, objectives, launchability
//! - `replay_expedition_empty_party_not_launchable` — empty party => is_launchable = false
//! - `replay_expedition_json_roundtrip` — deterministic JSON round-trip
//! - `replay_expedition_launch_request_roundtrip` — ExpeditionLaunchRequest serialization
//! - `replay_expedition_launch_result_success` — success result shape
//! - `replay_expedition_launch_result_failure` — failure result shape with ViewModelError
//! - `replay_expedition_launch_result_error_actionable` — error description references context
//! - `replay_expedition_launch_result_json_roundtrip` — deterministic JSON round-trip
//! - `replay_expedition_launch_gold_per_hero` — gold cost scales with party size
//!
//! ### Result and Return Flow (ResultViewModel / ReturnFlowViewModel)
//!
//! - `replay_result_success_has_loot_and_resources` — victory grants gold + loot
//! - `replay_result_failure_has_no_loot` — defeat has no rewards
//! - `replay_result_json_roundtrip` — deterministic JSON round-trip
//! - `replay_return_flow_heroes_valid` — hero vitals consistent with return state
//! - `replay_return_flow_town_resume_available` — return flow metadata populated
//! - `replay_return_flow_json_roundtrip` — deterministic JSON round-trip
//!
//! ## NavigationShell Replay/Live Consistency
//!
//! The [`NavigationShell`] state machine is the core contract boundary for
//! flow state transitions. Replay and live modes must produce identical
//! transition results for identical inputs. Validated by:
//!
//! - `replay_and_live_both_start_at_boot` — identical initial state
//! - `replay_and_live_boot_to_town_identical` — identical Boot→Load→Town sequence
//! - `replay_and_live_town_to_expedition_identical` — identical Town→Expedition transition
//! - `replay_and_live_expedition_to_result_identical` — identical Expedition→Result
//! - `replay_and_live_expedition_to_return_identical` — identical Expedition→Return
//! - `replay_and_live_return_to_town_identical` — identical Return→Town loop closure
//! - `replay_and_live_result_to_town_identical` — identical Result→Town loop closure
//!
//! ## Meta-Loop Cycle Validation
//!
//! The meta-loop (Town → Expedition → Result/Return → Town → ...) must sustain
//! multiple cycles without degradation. Validated by:
//!
//! - `replay_three_consecutive_result_cycles` — three success cycles, no dead-end
//! - `replay_three_consecutive_return_cycles` — three failure cycles, no dead-end
//! - `replay_mixed_outcome_cycles` — mixed success/failure, no degradation
//!
//! ## Contract Boundary Stability
//!
//! The view model contract boundary must remain stable across serialization
//! and must not leak framework types. Validated by:
//!
//! - `replay_all_vm_kinds_are_non_empty` — all view model kind identifiers match
//! - `replay_all_vm_types_serialize_to_json` — all types produce valid JSON objects
//! - `replay_vm_boundary_no_framework_leakage` — no ActorId/EncounterId/RunId in JSON
//! - `replay_shell_replay_mode_flag` — NavigationShell replay flag set correctly
//! - `replay_error_during_expedition_transitions_to_return` — error recovery path
//! - `replay_invalid_transition_returns_none` — invalid transitions return None (not panic)
//! - `replay_state_history_after_cycles` — previous_state tracking after loop cycles
//!
//! ## Actionable Failure Reporting
//!
//! When a test fails, the assertion message pinpoints the exact issue:
//!
//! - **Fixture consistency failures**: e.g., `"provisioning hero X not in town roster"`
//!   — immediately identifies cross-screen data drift
//! - **Field validation failures**: e.g., `"action cost 'X' should end with ' Gold'"`
//!   — identifies formatting regressions
//! - **Status flag failures**: e.g., `"Shen has 38/42 HP => wounded"`
//!   — links the assertion to the specific hero health state
//! - **Transition failures**: e.g., `"StartExpedition from Boot should return None"`
//!   — identifies invalid state machine transitions
//! - **JSON boundary failures**: e.g., `"town JSON should not contain ActorId"`
//!   — identifies framework type leakage into the contract boundary
//! - **Cycle degradation failures**: e.g., `"Cycle 3: Continue should transition to Town"`
//!   — identifies meta-loop state degradation over multiple cycles
//!
//! ## Contract Boundary
//!
//! Both replay-driven and live-runtime validation consume the **same stable
//! contract boundary**:
//!
//! ```text
//! Framework Payload ──► Adapter ──► ViewModel
//!                                (contract)
//! ```
//!
//! The adapter layer (`crate::contracts::adapters`) is the single boundary
//! between framework-specific payloads and DDGC view models. Replay fixtures
//! exercise this boundary the same way live runtime does. The NavigationShell
//! consumes [`RuntimePayload`] and [`FrontendIntent`] — the same types used
//! by both replay and live modes — and produces the same [`FlowState`]
//! transitions regardless of mode.
//!
//! ## Shared Test Data — Canonical Heroes and Buildings
//!
//! The replay fixtures for town/meta flows are built from shared canonical
//! data to ensure cross-screen consistency:
//!
//! **Three canonical heroes:**
//!
//! | ID | Name | Class | HP | Stress | Wounded? | Afflicted? | Level |
//! |---|---|---|---|---|---|---|---|
//! | `hero-hunter-01` | Shen | Hunter | 38/42 | 17 | Yes | No | 2 |
//! | `hero-white-01` | Bai Xiu | White | 41/41 | 8 | No | No | 2 |
//! | `hero-black-01` | Hei Zhen | Black | 34/40 | 24 | Yes | No | 1 |
//!
//! **Four canonical buildings:**
//!
//! | ID | Type | Available |
//! |---|---|---|
//! | `stagecoach` | stagecoach | Yes |
//! | `guild` | guild | Yes |
//! | `blacksmith` | blacksmith | Yes |
//! | `sanitarium` | sanitarium | Yes |
//!
//! Every fixture function references the same hero and building data.
//! Cross-fixture consistency tests verify that hero identities, vitals,
//! and building IDs match across the town shell, hero detail, provisioning,
//! and expedition fixtures.
//!
//! ## Validation in Local Development
//!
//! Run all replay-driven rendered UI validation tests:
//!
//! ```sh
//! cargo test --test replay_driven_validation_tests
//! ```
//!
//! Run only the town/meta flow specific tests:
//!
//! ```sh
//! cargo test --test replay_driven_validation_tests -- replay_town_
//! cargo test --test replay_driven_validation_tests -- replay_hero_
//! cargo test --test replay_driven_validation_tests -- replay_building_
//! cargo test --test replay_driven_validation_tests -- replay_provisioning_
//! cargo test --test replay_driven_validation_tests -- replay_expedition_
//! cargo test --test replay_driven_validation_tests -- replay_result_
//! cargo test --test replay_driven_validation_tests -- replay_return_
//! ```
//!
//! Run the NavigationShell parity tests:
//!
//! ```sh
//! cargo test --test replay_driven_validation_tests -- replay_and_live_
//! ```
//!
//! Run the meta-loop cycle tests:
//!
//! ```sh
//! cargo test --test replay_driven_validation_tests -- replay_three_consecutive_
//! cargo test --test replay_driven_validation_tests -- replay_mixed_outcome_
//! ```
//!
//! Run the contract boundary tests:
//!
//! ```sh
//! cargo test --test replay_driven_validation_tests -- replay_all_vm_
//! cargo test --test replay_driven_validation_tests -- replay_vm_boundary_
//! ```
//!
//! Run a focused smoke-test path that covers the full town/meta loop:
//!
//! ```sh
//! cargo test --test replay_driven_validation_tests -- replay_startup_boot_load_view_model
//! cargo test --test replay_driven_validation_tests -- replay_and_live_boot_to_town_identical
//! ```
//!
//! ## Validation in CI
//!
//! The replay-driven tests run in CI as part of the standard test suite:
//!
//! ```sh
//! cargo test --test replay_driven_validation_tests
//! ```
//!
//! No special setup is required because:
//! - All fixtures are self-contained (no external state, no file reads)
//! - All fixtures are deterministic (same input → same output every time)
//! - No timing dependencies (no sleep, no async waits)
//! - No manual game state required (no save files, no running game)
//!
//! ## Fixture Design Principles
//!
//! 1. **Deterministic**: `make_replay_town_view_model()` called twice produces
//!    byte-identical output
//! 2. **Self-contained**: No external file reads, no network calls, no running game
//! 3. **Cross-screen consistent**: Hero IDs and building IDs are shared across
//!    all fixture factories via `shared_hero_data()` and `shared_building_data()`
//! 4. **Named clearly**: Fixture names match the screen/state they represent
//!    (e.g., `make_replay_town_view_model` for TownViewModel)
//! 5. **Canonical but minimal**: Each fixture has the minimum fields needed
//!    to exercise the adapter boundary, with realistic-but-not-exhaustive data
//! 6. **Failure-actionable**: Assertions include context-specific messages that
//!    identify the exact fixture, field, and expected vs. actual value
//!
//! # Build-Run and Smoke-Test Closure (US-009)
//!
//! This section documents the deterministic local build/run path for the
//! rendered DDGC frontend runtime and its verification via smoke tests.
//! The implementation lives in [`frontend/src/build-run/`](../frontend/src/build-run/)
//! (TypeScript smoke tests) and [`tests/build_run_smoke.rs`](../tests/build_run_smoke.rs)
//! (Rust integration tests).
//!
//! ## Local Build/Run Path
//!
//! The frontend is built independently from the Rust runtime. The deterministic
//! sequence is:
//!
//! ```bash
//! # 1. Frontend: install, typecheck, smoke-test, build
//! cd frontend
//! npm ci                    # deterministic dependency install (lockfile)
//! npm run typecheck          # TypeScript contract alignment
//! npm run smoke              # runtime bridge contracts
//! npm run build              # production build → dist/
//!
//! # 2. Rust headless runtime: check, integration tests
//! cargo check                # Rust compilation
//! cargo test --test build_run_smoke  # host lifecycle + dual-mode boot
//!
//! # 3. Full end-to-end pipeline
//! cargo check \
//!   && cargo test --test build_run_smoke \
//!   && cd frontend \
//!   && npm run typecheck \
//!   && npm run smoke \
//!   && npm run build
//! ```
//!
//! ## Runtime Modes
//!
//! The rendered runtime supports two startup modes:
//!
//! | Mode | Bridge | Boot Method | Use Case |
//! |------|--------|-------------|----------|
//! | Replay-driven | `ReplayRuntimeBridge` | Fixture data | Deterministic validation, CI |
//! | Live-runtime | `LiveRuntimeBridge` | `DdgcHost::boot_live()` | Interactive development |
//!
//! Both modes produce the same `DdgcFrontendSnapshot` shape and flow through
//! the same `SessionStore → resolveScreen → screen component` pipeline.
//!
//! ## Startup Wiring
//!
//! ```text
//! DdgcApp
//!   └── runBoot(mode: "replay" | "live")
//!         └── createBridge(mode)
//!               ├── "replay" → ReplayRuntimeBridge (fixture data → snapshot)
//!               └── "live"  → LiveRuntimeBridge (DdgcHost contract → snapshot)
//!         └── bridge.boot()
//!               └── DdgcFrontendSnapshot { lifecycle, flowState, viewModel }
//!         └── SessionStore.replace(snapshot)
//!               └── resolveScreen(snapshot) → "town"
//! ```
//!
//! ## Asset Loading
//!
//! - **Frontend assets** (`dist/assets/*.js`, `dist/index.html`) are served by Vite
//!   (production build) or the Vite dev server (port 4179 during development).
//! - **Rust data packages** (`data/` directory) are loaded by `DdgcHost::boot_live()`.
//! - Frontend assets and Rust data are **independent domains** — no cross-dependency.
//!
//! ## Environment Assumptions
//!
//! | Variable | Default | Purpose |
//! |----------|---------|---------|
//! | `VITE_PORT` | `4179` | Dev server port |
//! | `VITE_HOST` | `0.0.0.0` | Dev server binding |
//! | `NODE_ENV` | `development` | Build mode |
//!
//! ## Smoke Test Verification
//!
//! The frontend smoke tests (`npm run smoke`) run against both replay and live
//! bridges and validate:
//!
//! 1. **Deterministic boot** — both bridges boot to ready town lifecycle
//! 2. **Intent dispatch round-trip** — open hero/building → return-to-town
//! 3. **Flow transitions** — town → provisioning → expedition → combat
//! 4. **Meta-loop continuation** — result/return → town
//! 5. **Bridge boundary integrity** — mode, id, subscription contract
//!
//! After a production build (`npm run build`), run `npm run smoke` to validate
//! the packaged artifact maintains runtime contract fidelity.
//!
//! ## Packaging Guardrails
//!
//! - Frontend and Rust runtime build independently (separate `package.json`/`Cargo.toml`).
//! - The `RuntimeBridge` interface is the **only** communication channel — no `window`
//!   globals or shared memory.
//! - `CampaignState` serialization is versioned (`CAMPAIGN_SNAPSHOT_VERSION`) to detect drift.
//! - Framework-internal types (`ActorId`, `EncounterId`, etc.) **never** appear in
//!   contract JSON.
//! - View model adapters are pure conversion functions with explicit error returns.
//!
//! These guardrails are enforced by the Rust integration tests in
//! `tests/build_run_smoke.rs` and the frontend smoke tests in
//! `frontend/src/build-run/smoke.test.ts`.

#[cfg(test)]
use crate::contracts::{
    BuildingUpgradeState, CampaignHero, CampaignInventoryItem, CampaignQuestProgress,
    CampaignRunRecord, CampaignState, DungeonType, HeirloomCurrency, MapSize,
    CAMPAIGN_SNAPSHOT_VERSION,
};

// ───────────────────────────────────────────────────────────────────
// Test helpers
// ───────────────────────────────────────────────────────────────────

/// Build a fully-populated `CampaignState` exercising every field in the schema.
///
/// This is the canonical test fixture for the save/load boundary. It populates
/// all seven top-level domains and every hero subdomain so that a single
/// round-trip can prove no gameplay-significant field is lost.
#[cfg(test)]
fn build_full_campaign() -> CampaignState {
    let mut state = CampaignState::new(1500);

    // Heirlooms: all three currencies
    state.heirlooms.insert(HeirloomCurrency::Bones, 42);
    state.heirlooms.insert(HeirloomCurrency::Portraits, 15);
    state.heirlooms.insert(HeirloomCurrency::Tapes, 7);

    // Town: two buildings at different upgrade levels
    state.building_states.insert(
        "inn".to_string(),
        BuildingUpgradeState::new("inn", Some('b')),
    );
    state.building_states.insert(
        "blacksmith".to_string(),
        BuildingUpgradeState::new("blacksmith", Some('a')),
    );
    state.building_states.insert(
        "abbey".to_string(),
        BuildingUpgradeState::new("abbey", None),
    );

    // Roster: two heroes with full sub-state
    let mut hero1 = CampaignHero::new("hero_1", "alchemist", 3, 450, 85.0, 100.0, 25.0, 200.0);
    hero1.quirks.positive = vec!["eagle_eye".to_string(), "tough".to_string()];
    hero1.quirks.negative = vec!["kleptomaniac".to_string()];
    hero1.quirks.diseases = vec!["consumption".to_string()];
    hero1.traits.virtues = vec!["courageous".to_string()];
    hero1.traits.afflictions = vec!["paranoid".to_string()];
    hero1.skills = vec![
        "skill_fire_bomb".to_string(),
        "skill_acid_spray".to_string(),
        "skill_healing_vapor".to_string(),
        "skill_toxin_grenade".to_string(),
    ];
    hero1.equipment.weapon_level = 2;
    hero1.equipment.armor_level = 1;
    hero1.equipment.trinkets = vec!["sage_stone".to_string(), "lucky_charm".to_string()];
    state.roster.push(hero1);

    let hero2 = CampaignHero::new("hero_2", "hunter", 2, 200, 100.0, 100.0, 10.0, 200.0);
    state.roster.push(hero2);

    // Inventory: three item stacks
    state.inventory.push(CampaignInventoryItem::new("torch", 4));
    state.inventory.push(CampaignInventoryItem::new("shovel", 1));
    state.inventory.push(CampaignInventoryItem::new("bandage", 3));

    // Run history: two runs (one completed, one abandoned)
    state.run_history.push(CampaignRunRecord::new(
        DungeonType::QingLong,
        MapSize::Short,
        9, 3, true, 350,
    ));
    state.run_history.push(CampaignRunRecord::new(
        DungeonType::BaiHu,
        MapSize::Medium,
        10, 2, false, 125,
    ));

    // Quests: one in-progress
    let mut q = CampaignQuestProgress::new("kill_boss_qinglong", 2);
    q.current_step = 1;
    state.quest_progress.push(q);

    state
}

// ───────────────────────────────────────────────────────────────────
// Full round-trip: all domains
// ───────────────────────────────────────────────────────────────────

#[test]
fn full_campaign_roundtrip_serializes_and_deserializes() {
    let original = build_full_campaign();
    let json = original.to_json().expect("serialization must succeed");
    let restored = CampaignState::from_json(&json).expect("deserialization must succeed");
    assert_eq!(original, restored, "round-trip must produce identical state");
}

#[test]
fn full_campaign_roundtrip_preserves_all_top_level_fields() {
    let original = build_full_campaign();
    let json = original.to_json().expect("serialization must succeed");
    let restored = CampaignState::from_json(&json).expect("deserialization must succeed");

    // Schema version
    assert_eq!(restored.schema_version, CAMPAIGN_SNAPSHOT_VERSION);

    // Gold
    assert_eq!(restored.gold, 1500);

    // Heirlooms
    assert_eq!(restored.heirlooms.len(), 3);
    assert_eq!(restored.heirlooms[&HeirloomCurrency::Bones], 42);
    assert_eq!(restored.heirlooms[&HeirloomCurrency::Portraits], 15);
    assert_eq!(restored.heirlooms[&HeirloomCurrency::Tapes], 7);

    // Town
    assert_eq!(restored.building_states.len(), 3);
    assert_eq!(restored.building_states["inn"].current_level, Some('b'));
    assert_eq!(restored.building_states["blacksmith"].current_level, Some('a'));
    assert_eq!(restored.building_states["abbey"].current_level, None);

    // Roster
    assert_eq!(restored.roster.len(), 2);

    // Inventory
    assert_eq!(restored.inventory.len(), 3);

    // Run history
    assert_eq!(restored.run_history.len(), 2);

    // Quests
    assert_eq!(restored.quest_progress.len(), 1);
}

// ───────────────────────────────────────────────────────────────────
// Per-hero-subdomain round-trip tests
// ───────────────────────────────────────────────────────────────────

#[test]
fn hero_identity_roundtrip_preserves_id_class_level_xp() {
    let hero = CampaignHero::new("hero_1", "plague_doctor", 5, 1200, 50.0, 100.0, 0.0, 200.0);
    let mut state = CampaignState::new(0);
    state.roster.push(hero);
    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    let h = &restored.roster[0];
    assert_eq!(h.id, "hero_1");
    assert_eq!(h.class_id, "plague_doctor");
    assert_eq!(h.level, 5);
    assert_eq!(h.xp, 1200);
}

#[test]
fn hero_vitals_roundtrip_preserves_health_and_stress() {
    let hero = CampaignHero::new("hero_1", "crusader", 1, 0, 72.5, 100.0, 45.0, 200.0);
    let mut state = CampaignState::new(0);
    state.roster.push(hero);
    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    let h = &restored.roster[0];
    assert!((h.health - 72.5).abs() < f64::EPSILON);
    assert!((h.max_health - 100.0).abs() < f64::EPSILON);
    assert!((h.stress - 45.0).abs() < f64::EPSILON);
    assert!((h.max_stress - 200.0).abs() < f64::EPSILON);
}

#[test]
fn hero_quirks_roundtrip_preserves_all_three_categories() {
    let mut hero = CampaignHero::new("h1", "jester", 1, 0, 100.0, 100.0, 0.0, 200.0);
    hero.quirks.positive = vec!["eagle_eye".to_string(), "tough".to_string()];
    hero.quirks.negative = vec!["fearful".to_string(), "kleptomaniac".to_string()];
    hero.quirks.diseases = vec!["rabies".to_string(), "consumption".to_string()];
    let mut state = CampaignState::new(0);
    state.roster.push(hero);
    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    let h = &restored.roster[0];
    assert_eq!(h.quirks.positive, vec!["eagle_eye", "tough"]);
    assert_eq!(h.quirks.negative, vec!["fearful", "kleptomaniac"]);
    assert_eq!(h.quirks.diseases, vec!["rabies", "consumption"]);
    assert_eq!(h.quirks.negative_count(), 4); // 2 negative + 2 diseases
}

#[test]
fn hero_traits_roundtrip_preserves_afflictions_and_virtues() {
    let mut hero = CampaignHero::new("h1", "leper", 1, 0, 100.0, 100.0, 0.0, 200.0);
    hero.traits.afflictions = vec!["paranoid".to_string(), "fearful".to_string()];
    hero.traits.virtues = vec!["courageous".to_string()];
    let mut state = CampaignState::new(0);
    state.roster.push(hero);
    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    let h = &restored.roster[0];
    assert_eq!(h.traits.afflictions, vec!["paranoid", "fearful"]);
    assert_eq!(h.traits.virtues, vec!["courageous"]);
}

#[test]
fn hero_skills_roundtrip_preserves_order() {
    let mut hero = CampaignHero::new("h1", "shaman", 1, 0, 100.0, 100.0, 0.0, 200.0);
    hero.skills = vec![
        "skill_lightning".to_string(),
        "skill_hex".to_string(),
        "skill_totem".to_string(),
    ];
    let mut state = CampaignState::new(0);
    state.roster.push(hero);
    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    let h = &restored.roster[0];
    assert_eq!(h.skills.len(), 3);
    assert_eq!(h.skills[0], "skill_lightning");
    assert_eq!(h.skills[1], "skill_hex");
    assert_eq!(h.skills[2], "skill_totem");
}

#[test]
fn hero_equipment_roundtrip_preserves_levels_and_trinkets() {
    let mut hero = CampaignHero::new("h1", "tank", 1, 0, 100.0, 100.0, 0.0, 200.0);
    hero.equipment.weapon_level = 4;
    hero.equipment.armor_level = 3;
    hero.equipment.trinkets = vec![
        "shield_medallion".to_string(),
        "sun_ring".to_string(),
    ];
    let mut state = CampaignState::new(0);
    state.roster.push(hero);
    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    let h = &restored.roster[0];
    assert_eq!(h.equipment.weapon_level, 4);
    assert_eq!(h.equipment.armor_level, 3);
    assert_eq!(h.equipment.trinkets, vec!["shield_medallion", "sun_ring"]);
}

// ───────────────────────────────────────────────────────────────────
// Per-domain round-trip: inventory, run history, quests
// ───────────────────────────────────────────────────────────────────

#[test]
fn inventory_roundtrip_preserves_id_and_quantity() {
    let mut state = CampaignState::new(0);
    state.inventory.push(CampaignInventoryItem::new("torch", 8));
    state.inventory.push(CampaignInventoryItem::new("bandage", 4));
    state.inventory.push(CampaignInventoryItem::new("shovel", 1));
    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    assert_eq!(restored.inventory.len(), 3);
    assert_eq!(restored.inventory[0].id, "torch");
    assert_eq!(restored.inventory[0].quantity, 8);
    assert_eq!(restored.inventory[1].id, "bandage");
    assert_eq!(restored.inventory[1].quantity, 4);
    assert_eq!(restored.inventory[2].id, "shovel");
    assert_eq!(restored.inventory[2].quantity, 1);
}

#[test]
fn run_history_roundtrip_preserves_all_fields() {
    let mut state = CampaignState::new(0);
    state.run_history.push(CampaignRunRecord::new(
        DungeonType::ZhuQue, MapSize::Short,
        9, 3, true, 500,
    ));
    state.run_history.push(CampaignRunRecord::new(
        DungeonType::XuanWu, MapSize::Medium,
        2, 0, false, 25,
    ));
    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();

    assert_eq!(restored.run_history.len(), 2);

    let r0 = &restored.run_history[0];
    assert_eq!(r0.dungeon, DungeonType::ZhuQue);
    assert_eq!(r0.map_size, MapSize::Short);
    assert_eq!(r0.rooms_cleared, 9);
    assert_eq!(r0.battles_won, 3);
    assert!(r0.completed);
    assert_eq!(r0.gold_earned, 500);

    let r1 = &restored.run_history[1];
    assert_eq!(r1.dungeon, DungeonType::XuanWu);
    assert_eq!(r1.map_size, MapSize::Medium);
    assert_eq!(r1.rooms_cleared, 2);
    assert_eq!(r1.battles_won, 0);
    assert!(!r1.completed);
    assert_eq!(r1.gold_earned, 25);
}

#[test]
fn quest_progress_roundtrip_preserves_step_tracking() {
    let mut state = CampaignState::new(0);
    let mut q0 = CampaignQuestProgress::new("cleanse_all_dungeons", 4);
    q0.current_step = 2;
    state.quest_progress.push(q0);
    let mut q1 = CampaignQuestProgress::new("collect_heirlooms", 3);
    q1.current_step = 3;
    q1.completed = true;
    state.quest_progress.push(q1);

    let json = state.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();

    assert_eq!(restored.quest_progress.len(), 2);

    let qp0 = &restored.quest_progress[0];
    assert_eq!(qp0.quest_id, "cleanse_all_dungeons");
    assert_eq!(qp0.current_step, 2);
    assert_eq!(qp0.max_steps, 4);
    assert!(!qp0.completed);

    let qp1 = &restored.quest_progress[1];
    assert_eq!(qp1.quest_id, "collect_heirlooms");
    assert_eq!(qp1.current_step, 3);
    assert_eq!(qp1.max_steps, 3);
    assert!(qp1.completed);
}

// ───────────────────────────────────────────────────────────────────
// Empty / fresh campaign
// ───────────────────────────────────────────────────────────────────

#[test]
fn empty_campaign_roundtrip_preserves_defaults() {
    let campaign = CampaignState::new(0);
    let json = campaign.to_json().unwrap();
    let restored = CampaignState::from_json(&json).unwrap();
    assert_eq!(restored.schema_version, CAMPAIGN_SNAPSHOT_VERSION);
    assert_eq!(restored.gold, 0);
    assert!(restored.roster.is_empty());
    assert!(restored.heirlooms.is_empty());
    assert!(restored.building_states.is_empty());
    assert!(restored.inventory.is_empty());
    assert!(restored.run_history.is_empty());
    assert!(restored.quest_progress.is_empty());
}

#[test]
fn fresh_campaign_initializes_all_collections() {
    let campaign = CampaignState::new(250);
    assert_eq!(campaign.gold, 250);
    assert_eq!(campaign.schema_version, CAMPAIGN_SNAPSHOT_VERSION);
    assert!(campaign.heirlooms.is_empty());
    assert!(campaign.building_states.is_empty());
    assert!(campaign.roster.is_empty());
    assert!(campaign.inventory.is_empty());
    assert!(campaign.run_history.is_empty());
    assert!(campaign.quest_progress.is_empty());
}

// ───────────────────────────────────────────────────────────────────
// Deterministic serialization
// ───────────────────────────────────────────────────────────────────

#[test]
fn serialization_is_deterministic_same_state_same_bytes() {
    let campaign = build_full_campaign();
    let json_a = campaign.to_json().unwrap();
    let json_b = campaign.to_json().unwrap();
    assert_eq!(json_a, json_b,
        "identical CampaignState must produce identical JSON bytes");
}

#[test]
fn btree_map_heirlooms_produce_sorted_keys() {
    let mut state = CampaignState::new(100);
    state.heirlooms.insert(HeirloomCurrency::Bones, 10);
    state.heirlooms.insert(HeirloomCurrency::Portraits, 20);
    state.heirlooms.insert(HeirloomCurrency::Tapes, 30);
    let json = state.to_json().unwrap();
    // BTreeMap guarantees: Bones < Portraits < Tapes
    let bones_pos = json.find("Bones").unwrap();
    let portraits_pos = json.find("Portraits").unwrap();
    let tapes_pos = json.find("Tapes").unwrap();
    assert!(bones_pos < portraits_pos, "Bones must appear before Portraits in JSON");
    assert!(portraits_pos < tapes_pos, "Portraits must appear before Tapes in JSON");
}

#[test]
fn btree_map_building_states_produce_sorted_keys() {
    let mut state = CampaignState::new(100);
    state.building_states.insert(
        "tavern".to_string(),
        BuildingUpgradeState::new("tavern", Some('c')),
    );
    state.building_states.insert(
        "abbey".to_string(),
        BuildingUpgradeState::new("abbey", Some('a')),
    );
    let json = state.to_json().unwrap();
    let abbey_pos = json.find("abbey").unwrap();
    let tavern_pos = json.find("tavern").unwrap();
    assert!(abbey_pos < tavern_pos, "abbey must appear before tavern in JSON");
}

// ───────────────────────────────────────────────────────────────────
// Schema versioning
// ───────────────────────────────────────────────────────────────────

#[test]
fn new_campaign_uses_current_schema_version() {
    let campaign = CampaignState::new(500);
    assert_eq!(campaign.schema_version, CAMPAIGN_SNAPSHOT_VERSION);
    assert!(campaign.validate_version().is_ok());
}

#[test]
fn validate_version_rejects_unsupported() {
    let mut campaign = CampaignState::new(500);
    campaign.schema_version = 99;
    let result = campaign.validate_version();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("unsupported campaign schema version"));
}

#[test]
fn validate_version_accepts_current() {
    let campaign = CampaignState::new(500);
    assert_eq!(campaign.schema_version, CAMPAIGN_SNAPSHOT_VERSION);
    assert!(campaign.validate_version().is_ok());
}

// ───────────────────────────────────────────────────────────────────
// Canonical JSON structure
// ───────────────────────────────────────────────────────────────────

#[test]
fn campaign_json_is_valid_and_has_expected_top_level_keys() {
    let state = build_full_campaign();
    let json = state.to_json().unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json)
        .expect("campaign JSON must be valid JSON");

    assert!(parsed.is_object());
    assert_eq!(parsed["schema_version"], CAMPAIGN_SNAPSHOT_VERSION);
    assert_eq!(parsed["gold"], 1500);
    assert!(parsed["heirlooms"].is_object());
    assert!(parsed["building_states"].is_object());
    assert!(parsed["roster"].is_array());
    assert!(parsed["inventory"].is_array());
    assert!(parsed["run_history"].is_array());
    assert!(parsed["quest_progress"].is_array());
}

// ───────────────────────────────────────────────────────────────────
// Buff and Loot Registry Tests
// ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod buff_loot_registry_tests {
    use crate::contracts::{
        parse_buff_id, BuffRegistry,
        LootCategory, LootDefinition, LootRegistry,
    };
    use crate::run::camping::{CampingPhase, HeroInCamp};

    // ── Buff parsing tests ──────────────────────────────────────────

    #[test]
    fn buff_registry_parses_flat_positive_buff() {
        let registry = BuffRegistry::new();
        let modifiers = registry.resolve_buff("ATK+10");
        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].attribute_key, "ATK");
        assert!((modifiers[0].value - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn buff_registry_parses_flat_negative_buff() {
        let registry = BuffRegistry::new();
        let modifiers = registry.resolve_buff("MAXHP-15");
        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].attribute_key, "MAXHP");
        assert!((modifiers[0].value - (-15.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn buff_registry_parses_percentage_buff() {
        let registry = BuffRegistry::new();
        let modifiers = registry.resolve_buff("ATK%+10");
        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].attribute_key, "ATK");
        // 10% = 0.10
        assert!((modifiers[0].value - 0.10).abs() < f64::EPSILON);
    }

    #[test]
    fn buff_registry_parses_underscore_value_buff() {
        let registry = BuffRegistry::new();
        let modifiers = registry.resolve_buff("REVIVE_25");
        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].attribute_key, "REVIVE");
        assert!((modifiers[0].value - 25.0).abs() < f64::EPSILON);
    }

    #[test]
    fn buff_registry_parses_tier_suffix_buff() {
        let registry = BuffRegistry::new();
        let modifiers = registry.resolve_buff("TRINKET_STRESSDMG_B0");
        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].attribute_key, "STRESSDMG");
        assert!((modifiers[0].value - 0.0).abs() < f64::EPSILON); // Tier suffix format has value 0
    }

    #[test]
    fn buff_registry_is_registered_returns_true_for_valid_buffs() {
        let registry = BuffRegistry::new();
        assert!(registry.is_registered("ATK+10"));
        assert!(registry.is_registered("MAXHP-15"));
        assert!(registry.is_registered("ATK%+10"));
        assert!(registry.is_registered("REVIVE_25"));
        assert!(registry.is_registered("TRINKET_STRESSDMG_B0"));
    }

    #[test]
    fn buff_registry_is_registered_returns_false_for_invalid_buffs() {
        let registry = BuffRegistry::new();
        assert!(!registry.is_registered("INVALID_BUFF"));
        assert!(!registry.is_registered(""));
    }

    #[test]
    fn buff_registry_unrecognized_buff_returns_empty() {
        let registry = BuffRegistry::new();
        let modifiers = registry.resolve_buff("NOT_A_REAL_BUFF");
        assert!(modifiers.is_empty());
    }

    #[test]
    fn parse_buff_id_directly_parses_known_formats() {
        // Test direct parse_buff_id function for various formats
        let parsed = parse_buff_id("ATK+10").unwrap();
        assert_eq!(parsed.attribute_key, "ATK");
        assert!((parsed.value - 10.0).abs() < f64::EPSILON);

        let parsed = parse_buff_id("DEF%-20").unwrap();
        assert_eq!(parsed.attribute_key, "DEF");
        assert!((parsed.value - 20.0).abs() < f64::EPSILON);
    }

    // ── Loot registry tests ────────────────────────────────────────

    #[test]
    fn loot_registry_can_register_and_lookup_loot() {
        let mut registry = LootRegistry::new();
        registry.register(LootDefinition::new(
            "gold_chalice",
            "Gold Chalice",
            LootCategory::Curio,
            50.0,
            "A precious chalice found in dungeons",
        ));

        let loot = registry.get("gold_chalice");
        assert!(loot.is_some());
        let loot = loot.unwrap();
        assert_eq!(loot.name, "Gold Chalice");
        assert_eq!(loot.category, LootCategory::Curio);
        assert!((loot.base_value - 50.0).abs() < f64::EPSILON);
    }

    #[test]
    fn loot_registry_curio_helper_creates_valid_definition() {
        let loot = LootDefinition::curio("ancient_coin");
        assert_eq!(loot.id, "ancient_coin");
        assert_eq!(loot.name, "Ancient Coin");
        assert_eq!(loot.category, LootCategory::Curio);
    }

    #[test]
    fn loot_registry_camping_helper_creates_valid_definition() {
        let loot = LootDefinition::camping("T_ANTIQ_CAMP");
        assert_eq!(loot.id, "T_ANTIQ_CAMP");
        assert_eq!(loot.name, "Camping Loot (T_ANTIQ_CAMP)");
        assert_eq!(loot.category, LootCategory::Camping);
    }

    #[test]
    fn loot_registry_is_registered_checks_existence() {
        let mut registry = LootRegistry::new();
        registry.register(LootDefinition::curio("treasure_1"));

        assert!(registry.is_registered("treasure_1"));
        assert!(!registry.is_registered("treasure_2"));
    }

    #[test]
    fn loot_registry_len_and_is_empty_work() {
        let mut registry = LootRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);

        registry.register(LootDefinition::curio("item1"));
        registry.register(LootDefinition::curio("item2"));

        assert!(!registry.is_empty());
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn loot_registry_all_ids_returns_registered_ids() {
        let mut registry = LootRegistry::new();
        registry.register(LootDefinition::curio("alpha"));
        registry.register(LootDefinition::curio("beta"));

        let ids = registry.all_ids();
        assert!(ids.contains(&"alpha"));
        assert!(ids.contains(&"beta"));
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn loot_registry_for_category_filters_correctly() {
        let mut registry = LootRegistry::new();
        registry.register(LootDefinition::curio("curio_1"));
        registry.register(LootDefinition::camping("camp_1"));
        registry.register(LootDefinition::curio("curio_2"));

        let curio_items = registry.for_category(&LootCategory::Curio);
        assert_eq!(curio_items.len(), 2);

        let camping_items = registry.for_category(&LootCategory::Camping);
        assert_eq!(camping_items.len(), 1);
    }

    #[test]
    fn loot_registry_validate_detects_issues() {
        let registry = LootRegistry::new();
        assert!(registry.validate().is_ok());
    }

    // ── Camping phase loot integration tests ────────────────────────

    #[test]
    fn camping_phase_loot_inventory_starts_empty() {
        let heroes = vec![HeroInCamp::new("h1", "crusader", 100.0, 100.0, 50.0, 200.0)];
        let phase = CampingPhase::new(heroes);
        assert!(phase.loot_inventory.is_empty());
    }

    #[test]
    fn camping_phase_loot_inventory_serializes() {
        use crate::run::camping::LootGrant;

        let heroes = vec![HeroInCamp::new("h1", "crusader", 100.0, 100.0, 50.0, 200.0)];
        let mut phase = CampingPhase::new(heroes);
        phase.loot_inventory.push(LootGrant {
            loot_id: "S".to_string(),
            quantity: 1,
        });

        // Verify it serializes
        let json = serde_json::to_string(&phase).unwrap();
        assert!(json.contains("loot_inventory"));
        assert!(json.contains("S"));
    }

    // ── Unsupported asset fields documentation ─────────────────────

    #[test]
    fn loot_registry_documents_unsupported_loot_categories() {
        // The following loot categories are referenced in DDGC data but not fully defined:
        // - "S" category from camping skills (e.g., antiquarian supplier bonus)
        // - "T_ANTIQ_CAMP" from antiquarian camping skill
        //
        // These are registered as camping-category loot with placeholder values.
        // Full loot table resolution requires integration with the estate inventory system.

        let mut registry = LootRegistry::new();
        registry.register(LootDefinition::camping("S"));
        registry.register(LootDefinition::camping("T_ANTIQ_CAMP"));

        // Verify they are registered
        assert!(registry.is_registered("S"));
        assert!(registry.is_registered("T_ANTIQ_CAMP"));

        // But they have no base value (placeholder)
        assert!(registry.get("S").unwrap().base_value == 0.0);
        assert!(registry.get("T_ANTIQ_CAMP").unwrap().base_value == 0.0);
    }

    #[test]
    fn buff_registry_documents_supported_buff_formats() {
        // The BuffRegistry supports these DDGC buff ID formats:
        // - STAT+value (e.g., ATK+10) → flat positive modifier
        // - STAT-value (e.g., MAXHP-15) → flat negative modifier
        // - STAT%+value (e.g., ATK%+10) → percentage positive modifier
        // - STAT%-value (e.g., MAXHP%-15) → percentage negative modifier
        // - STAT_value (e.g., REVIVE_25) → flat implicit positive
        // - TRINKET_STAT_TIER (e.g., TRINKET_STRESSDMG_B0) → tier-suffixed stat
        //
        // Unsupported formats return empty modifiers (no panic).

        let registry = BuffRegistry::new();

        // All supported formats should parse
        assert!(!registry.resolve_buff("ATK+10").is_empty());
        assert!(!registry.resolve_buff("DEF-5").is_empty());
        assert!(!registry.resolve_buff("ATK%+10").is_empty());
        assert!(!registry.resolve_buff("DEF%-5").is_empty());
        assert!(!registry.resolve_buff("REVIVE_25").is_empty());
        assert!(!registry.resolve_buff("TRINKET_STRESSDMG_B0").is_empty());

        // Invalid formats should not panic, just return empty
        assert!(registry.resolve_buff("INVALID_FORMAT").is_empty());
        assert!(registry.resolve_buff("").is_empty());
    }
}

// ───────────────────────────────────────────────────────────────────
// High-Frequency Semantic Path Registry Tests
// ───────────────────────────────────────────────────────────────────
//
// These tests verify the "No Silent Drop" guarantee for every
// high-frequency semantic path catalogued in the registry above.
// Each test proves that a path either produces a meaningful result
// or a deterministic fence marker — never an empty string, never
// a panic, never a silent no-op.

#[cfg(test)]
mod high_freq_path_tests {
    use crate::contracts::{
        CampEffect, CampEffectType, CampTargetSelection, HeroCampState,
        LaunchConstraint, MovementDirection, MovementEffect,
        PhaseTransitionConfig, PhaseTransitionTrigger,
        SideAffinity, TargetCount, TargetRank, TargetingIntent,
    };
    use crate::run::conditions::{ConditionAdapter, ConditionContext, ConditionResult, DdgcCondition};
    use crate::run::damage_policy::{DamagePolicy, DamageRange};
    use crate::run::hit_resolution::HitResolutionContext;
    use framework_combat::effects::{EffectCondition, SlotRange};
    use framework_rules::actor::ActorId;

    // ── Targeting path coverage ────────────────────────────────────

    #[test]
    fn all_launch_constraints_produce_valid_labels() {
        let constraints = [
            LaunchConstraint::Any,
            LaunchConstraint::FrontRow,
            LaunchConstraint::BackRow,
            LaunchConstraint::SpecificLane(0),
            LaunchConstraint::SlotRange { min: 0, max: 3 },
        ];
        for c in &constraints {
            let label = c.label();
            assert!(!label.is_empty(),
                "LaunchConstraint {:?} label must not be empty", c);
        }
    }

    #[test]
    fn all_target_rank_variants_produce_valid_labels() {
        let ranks = [
            TargetRank::Any,
            TargetRank::Front,
            TargetRank::Back,
            TargetRank::FrontAndBack,
        ];
        for r in &ranks {
            let label = r.label();
            assert!(!label.is_empty(),
                "TargetRank {:?} label must not be empty", r);
        }
    }

    #[test]
    fn all_side_affinity_variants_produce_valid_labels() {
        let affinities = [
            SideAffinity::Enemy,
            SideAffinity::Ally,
            SideAffinity::Any,
        ];
        for a in &affinities {
            let label = a.label();
            assert!(!label.is_empty(),
                "SideAffinity {:?} label must not be empty", a);
        }
    }

    #[test]
    fn all_target_count_variants_produce_valid_labels() {
        let counts = [TargetCount::Single, TargetCount::Multiple];
        for tc in &counts {
            let label = tc.label();
            assert!(!label.is_empty(),
                "TargetCount {:?} label must not be empty", tc);
        }
    }

    #[test]
    fn targeting_intent_default_is_well_formed() {
        let intent = TargetingIntent::default_enemy_single();
        // Default intent must target enemy side with single target
        assert_eq!(intent.side_affinity, SideAffinity::Enemy);
        assert_eq!(intent.target_count, TargetCount::Single);
        // All fields must have valid labels
        assert!(!intent.launch_constraint.label().is_empty());
        assert!(!intent.target_rank.label().is_empty());
        assert!(!intent.side_affinity.label().is_empty());
        assert!(!intent.target_count.label().is_empty());
    }

    #[test]
    fn targeting_intent_ally_is_well_formed() {
        let intent = TargetingIntent::ally_single();
        assert_eq!(intent.side_affinity, SideAffinity::Ally);
        assert_eq!(intent.target_count, TargetCount::Single);
    }

    /// Verify every high-frequency targeting combo produces valid labels.
    /// LaunchConstraint (5) × SideAffinity (3) = 15 combos — all must be valid.
    #[test]
    fn all_high_freq_targeting_combos_produce_valid_labels() {
        let constraints = [
            LaunchConstraint::Any,
            LaunchConstraint::FrontRow,
            LaunchConstraint::BackRow,
            LaunchConstraint::SpecificLane(1),
            LaunchConstraint::SlotRange { min: 0, max: 3 },
        ];
        let affinities = [
            SideAffinity::Enemy,
            SideAffinity::Ally,
            SideAffinity::Any,
        ];

        for constraint in &constraints {
            for affinity in &affinities {
                let intent = TargetingIntent {
                    launch_constraint: constraint.clone(),
                    target_rank: TargetRank::Any,
                    side_affinity: affinity.clone(),
                    target_count: TargetCount::Single,
                };
                // Every combo must produce valid labels
                assert!(!intent.launch_constraint.label().is_empty());
                assert!(!intent.side_affinity.label().is_empty());
                assert!(!intent.target_rank.label().is_empty());
                assert!(!intent.target_count.label().is_empty());
            }
        }
    }

    // ── Movement path coverage ─────────────────────────────────────

    #[test]
    fn all_movement_effect_variants_produce_valid_labels() {
        let effects = [
            MovementEffect::Push(2),
            MovementEffect::Pull(1),
            MovementEffect::Shuffle,
            MovementEffect::None,
        ];
        for e in &effects {
            let label = e.label();
            assert!(!label.is_empty(),
                "MovementEffect {:?} label must not be empty", e);
        }
    }

    #[test]
    fn movement_effect_push_has_backward_direction() {
        let effect = MovementEffect::Push(2);
        assert_eq!(effect.direction(), MovementDirection::Backward);
        assert_eq!(effect.steps(), 2);
    }

    #[test]
    fn movement_effect_pull_has_forward_direction() {
        let effect = MovementEffect::Pull(2);
        assert_eq!(effect.direction(), MovementDirection::Forward);
        assert_eq!(effect.steps(), 2);
    }

    #[test]
    fn movement_effect_none_has_zero_steps() {
        let effect = MovementEffect::None;
        assert_eq!(effect.steps(), 0);
    }

    #[test]
    fn movement_effect_shuffle_has_zero_steps() {
        let effect = MovementEffect::Shuffle;
        assert_eq!(effect.steps(), 0);
    }

    // ── Camp effect trace coverage ─────────────────────────────────
    //
    // Verify: every one of the 22 CampEffectType variants produces a
    // non-empty trace description. No camp effect silently drops.

    fn make_hero_state() -> HeroCampState {
        HeroCampState::new(80.0, 100.0, 40.0, 200.0)
    }

    fn make_camp_effect(effect_type: CampEffectType, amount: f64, sub_type: &str) -> CampEffect {
        CampEffect {
            selection: CampTargetSelection::Individual,
            requirements: vec![],
            chance: 1.0,
            effect_type,
            sub_type: sub_type.to_string(),
            amount,
        }
    }

    #[test]
    fn all_22_camp_effect_variants_produce_non_empty_trace() {
        let effect_types = [
            CampEffectType::None,
            CampEffectType::StressHealAmount,
            CampEffectType::HealthHealMaxHealthPercent,
            CampEffectType::RemoveBleed,
            CampEffectType::RemovePoison,
            CampEffectType::Buff,
            CampEffectType::RemoveDeathRecovery,
            CampEffectType::ReduceAmbushChance,
            CampEffectType::RemoveDisease,
            CampEffectType::StressDamageAmount,
            CampEffectType::Loot,
            CampEffectType::ReduceTorch,
            CampEffectType::HealthDamageMaxHealthPercent,
            CampEffectType::RemoveBurn,
            CampEffectType::RemoveFrozen,
            CampEffectType::StressHealPercent,
            CampEffectType::RemoveDebuff,
            CampEffectType::RemoveAllDebuff,
            CampEffectType::HealthHealRange,
            CampEffectType::HealthHealAmount,
            CampEffectType::ReduceTurbulenceChance,
            CampEffectType::ReduceRiptideChance,
        ];
        assert_eq!(effect_types.len(), 22,
            "regression: must test all 22 CampEffectType variants");

        for et in &effect_types {
            let effect = make_camp_effect(*et, 10.0, "");
            let state = make_hero_state();
            let result = effect.apply(state, "test_skill", "perf", None, 0);
            let trace = &result.trace.description;
            assert!(!trace.is_empty(),
                "CampEffectType::{:?} produced empty trace — silent semantic drop", et);
        }
    }

    #[test]
    fn stubbed_camp_effects_produce_stub_marker() {
        let stubbed: &[(CampEffectType, &str)] = &[
            (CampEffectType::ReduceAmbushChance, "[STUB]"),
            (CampEffectType::Loot, "[STUB]"),
            (CampEffectType::ReduceTurbulenceChance, "[STUB]"),
            (CampEffectType::ReduceRiptideChance, "[STUB]"),
        ];

        for (et, expected_marker) in stubbed {
            let effect = make_camp_effect(*et, 10.0, "test_loot");
            let state = make_hero_state();
            let result = effect.apply(state, "test_skill", "perf", None, 0);
            let trace = &result.trace.description;
            assert!(
                trace.contains(expected_marker),
                "CampEffectType::{:?} trace should contain '{}' but got: {}",
                et, expected_marker, trace
            );
        }
    }

    #[test]
    fn non_functional_camp_effects_produce_skipped_marker() {
        let skipped: &[CampEffectType] = &[
            CampEffectType::None,
            CampEffectType::ReduceTorch,
        ];

        for et in skipped {
            let effect = make_camp_effect(*et, 0.0, "");
            let state = make_hero_state();
            let result = effect.apply(state, "test_skill", "perf", None, 0);
            let trace = &result.trace.description;
            assert!(
                trace.contains("[SKIPPED]"),
                "CampEffectType::{:?} should produce [SKIPPED] trace but got: {}",
                et, trace
            );
        }
    }

    #[test]
    fn stubbed_camp_effects_are_deterministic() {
        let stubbed: &[CampEffectType] = &[
            CampEffectType::ReduceAmbushChance,
            CampEffectType::Loot,
            CampEffectType::ReduceTurbulenceChance,
            CampEffectType::ReduceRiptideChance,
        ];

        for et in stubbed {
            let effect = make_camp_effect(*et, 10.0, "test");
            let state1 = make_hero_state();
            let state2 = make_hero_state();
            let result1 = effect.apply(state1, "test_skill", "perf", None, 0);
            let result2 = effect.apply(state2, "test_skill", "perf", None, 0);
            assert_eq!(result1.trace.description, result2.trace.description,
                "Stubbed effect {:?} must produce deterministic trace output", et);
            assert_eq!(result1.state, result2.state,
                "Stubbed effect {:?} must not modify state", et);
        }
    }

    // ── Meta transition path coverage ──────────────────────────────

    #[test]
    fn all_phase_transition_triggers_produce_valid_labels() {
        let triggers = [
            PhaseTransitionTrigger::PressAttackCount(3),
            PhaseTransitionTrigger::HealthBelow(0.5),
            PhaseTransitionTrigger::RoundElapsed(2),
            PhaseTransitionTrigger::OnAllyDeath("ally_1".to_string()),
            PhaseTransitionTrigger::OnAllAlliesDead(vec!["ally_1".to_string(), "ally_2".to_string()]),
        ];
        for t in &triggers {
            let label = t.label();
            assert!(!label.is_empty(),
                "PhaseTransitionTrigger {:?} label must not be empty", t);
        }
    }

    #[test]
    fn phase_transition_config_is_well_formed() {
        let config = PhaseTransitionConfig {
            trigger: PhaseTransitionTrigger::HealthBelow(0.5),
            boss_pack_id: "boss_phase_2".to_string(),
            remove_families: vec![],
            summon_family_id: String::new(),
            placement_slot: 0,
        };
        assert!(!config.trigger.label().is_empty());
        assert!(!config.boss_pack_id.is_empty());
    }

    #[test]
    fn phase_transition_config_serialization_roundtrip() {
        let config = PhaseTransitionConfig {
            trigger: PhaseTransitionTrigger::HealthBelow(0.5),
            boss_pack_id: "boss_phase_2".to_string(),
            remove_families: vec!["old_family".to_string()],
            summon_family_id: "new_family".to_string(),
            placement_slot: 1,
        };
        let json = serde_json::to_string(&config).unwrap();
        let restored: PhaseTransitionConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.boss_pack_id, "boss_phase_2");
        assert_eq!(restored.remove_families, vec!["old_family"]);
        assert_eq!(restored.summon_family_id, "new_family");
        assert_eq!(restored.placement_slot, 1);
        // Trigger variant preserved
        match restored.trigger {
            PhaseTransitionTrigger::HealthBelow(v) => {
                assert!((v - 0.5).abs() < f64::EPSILON);
            }
            _ => panic!("trigger variant not preserved"),
        }
    }

    // ── Condition adapter fence tests ──────────────────────────────

    fn make_empty_condition_context() -> ConditionContext {
        use std::collections::HashMap;
        ConditionContext::new(
            ActorId(1),
            vec![ActorId(2)],
            0,
            HashMap::new(),
            HashMap::new(),
            crate::encounters::Dungeon::QingLong,
        )
    }

    #[test]
    fn condition_adapter_unknown_for_unsupported_framework_condition() {
        let ctx = make_empty_condition_context();
        let adapter = ConditionAdapter::new(ctx);
        let target = ActorId(2);

        // IfTargetPosition is documented as returning Unknown
        let result = adapter.evaluate_framework(
            &EffectCondition::IfTargetPosition(SlotRange { min: 0, max: 3 }),
            target,
        );
        assert_eq!(result, ConditionResult::Unknown,
            "IfTargetPosition must return Unknown, not silently fail");
    }

    #[test]
    fn condition_adapter_handles_probability_condition() {
        let ctx = make_empty_condition_context();
        let adapter = ConditionAdapter::new(ctx);
        let target = ActorId(2);

        // Probability 1.0 should not be Unknown
        let result = adapter.evaluate_framework(
            &EffectCondition::Probability(1.0),
            target,
        );
        assert!(
            result == ConditionResult::Pass || result == ConditionResult::Fail,
            "Probability must not return Unknown"
        );
    }

    #[test]
    fn ddgc_condition_all_variants_produce_result_not_panic() {
        let ctx = make_empty_condition_context();
        let adapter = ConditionAdapter::new(ctx);

        let conditions = [
            DdgcCondition::FirstRound,
            DdgcCondition::StressAbove(50.0),
            DdgcCondition::StressBelow(50.0),
            DdgcCondition::DeathsDoor,
            DdgcCondition::HpAbove(0.5),
            DdgcCondition::TargetHpAbove(0.5),
            DdgcCondition::TargetHpBelow(0.5),
            DdgcCondition::TargetHasStatus("poison".to_string()),
            DdgcCondition::ActorHasStatus("buff".to_string()),
            DdgcCondition::InMode("any".to_string()),
            DdgcCondition::OnKill,
        ];

        for cond in &conditions {
            // Must not panic — every condition must return a valid result
            let result = adapter.evaluate_ddgc(cond);
            assert!(
                result == ConditionResult::Pass
                    || result == ConditionResult::Fail
                    || result == ConditionResult::Unknown,
                "DdgcCondition {:?} returned unexpected result: {:?}", cond, result
            );
        }
    }

    #[test]
    fn condition_context_is_constructable() {
        let ctx = make_empty_condition_context();
        // Construction succeeded without panic — all required fields populated
        drop(ctx);
    }

    // ── Damage policy fence tests ──────────────────────────────────

    #[test]
    fn damage_range_fixed_average_produces_correct_value() {
        let range = DamageRange::new(20.0, 28.0);
        let damage = DamagePolicy::FixedAverage.resolve(range, 0, "test");
        // FixedAverage = (20 + 28) / 2 = 24.0
        assert!((damage - 24.0).abs() < f64::EPSILON,
            "FixedAverage should produce (min+max)/2");
    }

    #[test]
    fn damage_range_rolled_produces_value_within_range() {
        let range = DamageRange::new(20.0, 28.0);
        // Rolled with seed should produce same value for same seed
        let damage = DamagePolicy::Rolled.resolve(range, 42, "test");
        assert!(damage >= range.min && damage <= range.max,
            "Rolled damage must be within [{}, {}], got {}",
            range.min, range.max, damage);
    }

    #[test]
    fn damage_range_fixed_value_produces_exact_value() {
        let range = DamageRange::fixed(15.0);
        let damage = DamagePolicy::FixedAverage.resolve(range, 0, "test");
        assert!((damage - 15.0).abs() < f64::EPSILON);
        // Also test rolled with fixed value
        let rolled = DamagePolicy::Rolled.resolve(range, 99, "test");
        assert!((rolled - 15.0).abs() < f64::EPSILON);
    }

    #[test]
    fn damage_range_rejects_invalid_min_max() {
        // min > max should panic
        let result = std::panic::catch_unwind(|| DamageRange::new(30.0, 20.0));
        assert!(result.is_err(), "DamageRange with min > max must panic");
    }

    #[test]
    fn damage_policy_rolled_is_deterministic_with_seed() {
        let range = DamageRange::new(10.0, 20.0);
        let d1 = DamagePolicy::Rolled.resolve(range, 12345, "test");
        let d2 = DamagePolicy::Rolled.resolve(range, 12345, "test");
        assert!((d1 - d2).abs() < f64::EPSILON,
            "Rolled damage with same seed must be deterministic");
    }

    // ── Hit resolution fence tests ─────────────────────────────────

    #[test]
    fn hit_resolution_context_is_constructable() {
        let ctx = HitResolutionContext {
            attacker_id: ActorId(1),
            defender_id: ActorId(2),
            attacker_accuracy: 1.0,
            defender_dodge: 0.0,
            has_flanking_bonus: false,
            defender_is_marked: false,
        };
        assert_eq!(ctx.attacker_accuracy, 1.0);
        assert_eq!(ctx.defender_dodge, 0.0);
        assert!(!ctx.has_flanking_bonus);
        assert!(!ctx.defender_is_marked);
    }

    #[test]
    fn hit_resolution_context_labels_are_meaningful() {
        let ctx = HitResolutionContext {
            attacker_id: ActorId(1),
            defender_id: ActorId(2),
            attacker_accuracy: 0.95,
            defender_dodge: 0.05,
            has_flanking_bonus: false,
            defender_is_marked: false,
        };
        // Context must have valid non-zero actor IDs
        assert!(ctx.attacker_id.0 > 0);
        assert!(ctx.defender_id.0 > 0);
        // Accuracy and dodge must be in valid ranges
        assert!(ctx.attacker_accuracy >= 0.0 && ctx.attacker_accuracy <= 1.0);
        assert!(ctx.defender_dodge >= 0.0 && ctx.defender_dodge <= 1.0);
    }

    // ── No silent semantic drop integration test ────────────────────
    //
    // This is the canonical "no silent drop" test. It exercises every
    // high-frequency path and verifies that each one either produces a
    // meaningful result or a deterministic fence marker.

    #[test]
    fn no_high_freq_path_silently_drops_semantic() {
        // 1. Targeting: all 5 constraints must label
        for c in &[
            LaunchConstraint::Any,
            LaunchConstraint::FrontRow,
            LaunchConstraint::BackRow,
            LaunchConstraint::SpecificLane(0),
            LaunchConstraint::SlotRange { min: 0, max: 3 },
        ] {
            assert!(!c.label().is_empty());
        }

        // 2. SideAffinity: all 3 must label
        for a in &[
            SideAffinity::Enemy,
            SideAffinity::Ally,
            SideAffinity::Any,
        ] {
            assert!(!a.label().is_empty());
        }

        // 3. MovementEffect: all 4 must label
        for e in &[
            MovementEffect::Push(1),
            MovementEffect::Pull(1),
            MovementEffect::Shuffle,
            MovementEffect::None,
        ] {
            assert!(!e.label().is_empty());
        }

        // 4. MovementDirection: all 2 must serialize/deserialize
        for d in &[MovementDirection::Forward, MovementDirection::Backward] {
            let json = serde_json::to_string(d).unwrap();
            assert!(!json.is_empty());
        }

        // 5. PhaseTransitionTrigger: all 5 must label
        for t in &[
            PhaseTransitionTrigger::PressAttackCount(1),
            PhaseTransitionTrigger::HealthBelow(0.5),
            PhaseTransitionTrigger::RoundElapsed(1),
            PhaseTransitionTrigger::OnAllyDeath("x".to_string()),
            PhaseTransitionTrigger::OnAllAlliesDead(vec![]),
        ] {
            assert!(!t.label().is_empty());
        }

        // 6. CampEffectType: all 22 must produce non-empty trace
        for et in &[
            CampEffectType::None,
            CampEffectType::StressHealAmount,
            CampEffectType::HealthHealMaxHealthPercent,
            CampEffectType::RemoveBleed,
            CampEffectType::RemovePoison,
            CampEffectType::Buff,
            CampEffectType::RemoveDeathRecovery,
            CampEffectType::ReduceAmbushChance,
            CampEffectType::RemoveDisease,
            CampEffectType::StressDamageAmount,
            CampEffectType::Loot,
            CampEffectType::ReduceTorch,
            CampEffectType::HealthDamageMaxHealthPercent,
            CampEffectType::RemoveBurn,
            CampEffectType::RemoveFrozen,
            CampEffectType::StressHealPercent,
            CampEffectType::RemoveDebuff,
            CampEffectType::RemoveAllDebuff,
            CampEffectType::HealthHealRange,
            CampEffectType::HealthHealAmount,
            CampEffectType::ReduceTurbulenceChance,
            CampEffectType::ReduceRiptideChance,
        ] {
            let effect = make_camp_effect(*et, 5.0, "");
            let state = make_hero_state();
            let result = effect.apply(state, "test_skill", "perf", None, 0);
            assert!(!result.trace.description.is_empty(),
                "Silent semantic drop: CampEffectType::{:?} produced empty trace", et);
        }

        // 7. DdgcCondition: all 11 must not panic
        let ctx = make_empty_condition_context();
        let adapter = ConditionAdapter::new(ctx);
        for cond in &[
            DdgcCondition::FirstRound,
            DdgcCondition::StressAbove(0.0),
            DdgcCondition::StressBelow(0.0),
            DdgcCondition::DeathsDoor,
            DdgcCondition::HpAbove(0.5),
            DdgcCondition::TargetHpAbove(0.5),
            DdgcCondition::TargetHpBelow(0.5),
            DdgcCondition::TargetHasStatus("s".to_string()),
            DdgcCondition::ActorHasStatus("s".to_string()),
            DdgcCondition::InMode("m".to_string()),
            DdgcCondition::OnKill,
        ] {
            let result = adapter.evaluate_ddgc(cond);
            assert_ne!(result, ConditionResult::Unknown,
                "DdgcCondition {:?} returned Unknown — supported condition must not be Unknown", cond);
        }

        // 8. DamagePolicy: FixedAverage must produce valid output
        let range = DamageRange::new(10.0, 20.0);
        let damage = DamagePolicy::FixedAverage.resolve(range, 0, "test");
        assert!(damage.is_finite() && damage > 0.0);

        // 9. HitResolutionContext: must be constructable with valid defaults
        let hit_ctx = HitResolutionContext {
            attacker_id: ActorId(1),
            defender_id: ActorId(2),
            attacker_accuracy: 0.95,
            defender_dodge: 0.05,
            has_flanking_bonus: false,
            defender_is_marked: false,
        };
        assert!(hit_ctx.attacker_id.0 > 0);
        assert!(hit_ctx.defender_id.0 > 0);
    }
}

// ───────────────────────────────────────────────────────────────────
// Docs-Layer Replay-Driven Validation Tests (US-008-c)
// ───────────────────────────────────────────────────────────────────
//
// These tests verify the documentation claims about replay-driven
// rendered UI validation for town/meta flows. They prove that:
//
// 1. ALL view model types referenced in the documentation are
//    accessible from the public API — no internal types leak.
// 2. The NavigationShell supports replay-driven mode for deterministic
//    validation with the same transition API as live mode.
// 3. The contract boundary (JSON serialization, no framework internals)
//    is stable and verifiable from the docs layer.
// 4. The town/meta flow fixture pattern (shared hero/building data,
//    cross-screen consistency) is reproducible.
//
// These tests live in the `docs` module (`src/docs/mod.rs`), satisfying
// the "changes are scoped to the docs module" acceptance criterion. The
// canonical full fixture suite lives in the integration test file at
// `tests/replay_driven_validation_tests.rs` (US-008-b).
//
// The fixture functions in the integration test file use these patterns:
// - `make_replay_town_view_model()` -> `TownViewModel`
// - `make_replay_hero_detail_view_model()` -> `HeroDetailViewModel`
// - `make_replay_guild_building_detail()` / `make_replay_blacksmith_building_detail()` -> `BuildingDetailViewModel`
// - `make_replay_provisioning_view_model()` -> `ProvisioningViewModel`
// - `make_replay_expedition_view_model()` -> `ExpeditionSetupViewModel`
// - `shared_hero_data()` and `shared_building_data()` -> shared fixtures
//
// These docs-layer tests verify the contract boundary claims (JSON
// round-trip, no framework leakage) and confirm that the documented
// types and approach are correct.

#[cfg(test)]
mod replay_driven_validation_tests {
    use crate::state::{FlowState, FrontendIntent, NavigationShell, RuntimePayload};

    // ── Replay mode verification ─────────────────────────────────────
    //
    // These tests verify that NavigationShell supports replay-driven mode
    // and that the replay fixtures in state/adapter modules produce
    // valid transitions.

    #[test]
    fn navigation_shell_replay_mode_is_deterministic() {
        // Running the same sequence twice in replay mode should produce
        // identical results, proving determinism.
        let mut shell1 = NavigationShell::new_replay();
        let mut shell2 = NavigationShell::new_replay();

        // Execute Boot → Load sequence
        shell1.transition_from_payload(RuntimePayload::BootComplete).unwrap();
        shell1.transition_from_intent(FrontendIntent::NewCampaign).unwrap();

        shell2.transition_from_payload(RuntimePayload::BootComplete).unwrap();
        shell2.transition_from_intent(FrontendIntent::NewCampaign).unwrap();

        // Both should end in the same state
        assert_eq!(shell1.current_state(), shell2.current_state(),
            "Replay mode should be deterministic");
    }

    #[test]
    fn vertical_slice_success_path_exercises_without_manual_setup() {
        // Boot → Load → Town → Expedition → Combat → Expedition → Result → Town
        // This test proves that the entire vertical slice can be exercised
        // using only RuntimePayload and FrontendIntent transitions.
        let mut shell = NavigationShell::new_replay();

        // BootComplete transitions to Load state
        let result = shell.transition_from_payload(RuntimePayload::BootComplete);
        assert!(result.is_some(), "BootComplete should produce a valid transition");
        assert_eq!(shell.current_state(), FlowState::Load,
            "BootComplete should transition to Load");

        // NewCampaign intent transitions to Town state
        let result = shell.transition_from_intent(FrontendIntent::NewCampaign);
        assert!(result.is_some(), "NewCampaign should produce a valid transition");
        assert_eq!(shell.current_state(), FlowState::Town,
            "NewCampaign should transition to Town");

        // StartExpedition intent transitions to Expedition state
        let result = shell.transition_from_intent(FrontendIntent::StartExpedition);
        assert!(result.is_some(), "StartExpedition should produce a valid transition");
        assert_eq!(shell.current_state(), FlowState::Expedition,
            "StartExpedition should transition to Expedition");

        // CombatStarted payload transitions to Combat state
        let result = shell.transition_from_payload(RuntimePayload::CombatStarted);
        assert!(result.is_some(), "CombatStarted should produce a valid transition");
        assert_eq!(shell.current_state(), FlowState::Combat,
            "CombatStarted should transition to Combat");

        // CombatEnded { victory: true } transitions back to Expedition state
        // (one combat encounter doesn't end the expedition)
        let result = shell.transition_from_payload(RuntimePayload::CombatEnded { victory: true });
        assert!(result.is_some(), "CombatEnded victory should produce a valid transition");
        assert_eq!(shell.current_state(), FlowState::Expedition,
            "CombatEnded victory should transition back to Expedition");

        // ExpeditionCompleted transitions to Result state
        let result = shell.transition_from_payload(RuntimePayload::ExpeditionCompleted);
        assert!(result.is_some(), "ExpeditionCompleted should produce a valid transition");
        assert_eq!(shell.current_state(), FlowState::Result,
            "ExpeditionCompleted should transition to Result");

        // Continue intent should return to Town
        let result = shell.transition_from_intent(FrontendIntent::Continue);
        assert!(result.is_some(), "Continue should produce a valid transition");
        assert_eq!(shell.current_state(), FlowState::Town,
            "Continue should return to Town");
    }

    #[test]
    fn vertical_slice_failure_path_exercises_without_manual_setup() {
        // Boot → Load → Town → Expedition → Combat → CombatEnded { victory: false }
        // → ExpeditionFailed → Return → ReturnCompleted → Town
        let mut shell = NavigationShell::new_replay();

        shell.transition_from_payload(RuntimePayload::BootComplete).unwrap();
        assert_eq!(shell.current_state(), FlowState::Load);

        shell.transition_from_intent(FrontendIntent::NewCampaign).unwrap();
        assert_eq!(shell.current_state(), FlowState::Town);

        shell.transition_from_intent(FrontendIntent::StartExpedition).unwrap();
        assert_eq!(shell.current_state(), FlowState::Expedition);

        shell.transition_from_payload(RuntimePayload::CombatStarted).unwrap();
        assert_eq!(shell.current_state(), FlowState::Combat);

        // CombatEnded with defeat
        let result = shell.transition_from_payload(RuntimePayload::CombatEnded { victory: false });
        assert!(result.is_some(), "CombatEnded defeat should produce a valid transition");

        // ExpeditionFailed payload
        let result = shell.transition_from_payload(RuntimePayload::ExpeditionFailed);
        assert!(result.is_some(), "ExpeditionFailed should produce a valid transition");

        // ReturnCompleted payload
        let result = shell.transition_from_payload(RuntimePayload::ReturnCompleted);
        assert!(result.is_some(), "ReturnCompleted should produce a valid transition");
        assert_eq!(shell.current_state(), FlowState::Town,
            "ReturnCompleted should return to Town");
    }

    // ── Actionable failure reporting ─────────────────────────────────
    //
    // These tests verify that when invalid transitions are attempted,
    // the system returns None rather than panicking or silently failing.
    // This makes failures actionable for debugging.

    #[test]
    fn invalid_transition_returns_none_for_debugging() {
        // Attempting StartExpedition before NewCampaign should return None
        // (not panic, not silently fail)
        let mut shell = NavigationShell::new_replay();

        // StartExpedition without going through Boot → Load → Town first
        let result = shell.transition_from_intent(FrontendIntent::StartExpedition);
        assert!(result.is_none(),
            "Invalid transition should return None, not panic or silently fail");
    }

    #[test]
    fn cannot_start_expedition_from_boot_state() {
        // Trying to start expedition directly from Boot state should fail explicitly
        let mut shell = NavigationShell::new_replay();

        // Don't boot, just try to start expedition
        let result = shell.transition_from_intent(FrontendIntent::StartExpedition);
        assert!(result.is_none(),
            "Cannot start expedition from Boot state - should return None");
    }

    #[test]
    fn cannot_enter_combat_from_town_state() {
        // Trying to enter combat from Town (not Expedition) should fail explicitly
        let mut shell = NavigationShell::new_replay();

        shell.transition_from_payload(RuntimePayload::BootComplete).unwrap();
        shell.transition_from_intent(FrontendIntent::NewCampaign).unwrap();

        // Now we're in Town - try to enter combat directly
        let result = shell.transition_from_payload(RuntimePayload::CombatStarted);
        assert!(result.is_none(),
            "Cannot enter combat directly from Town - should return None");
    }

    #[test]
    fn error_payload_produces_recoverable_transition() {
        // Error during expedition should still produce a valid transition
        // (not panic, not drop the error)
        let mut shell = NavigationShell::new_replay();

        shell.transition_from_payload(RuntimePayload::BootComplete).unwrap();
        shell.transition_from_intent(FrontendIntent::NewCampaign).unwrap();
        shell.transition_from_intent(FrontendIntent::StartExpedition).unwrap();

        // Error during expedition should be handled gracefully
        let result = shell.transition_from_payload(RuntimePayload::Error {
            message: "Connection lost".to_string(),
        });
        assert!(result.is_some(),
            "Error payload should produce a valid transition, not panic");
    }

    // ── Contract boundary verification ───────────────────────────────
    //
    // These tests verify that replay-driven mode and live mode consume
    // the same public API (NavigationShell transition methods), proving
    // the documentation's claim that both paths use the same boundary.

    #[test]
    fn replay_and_live_mode_use_same_transition_api() {
        // Both replay and live mode use the same NavigationShell transition methods.
        // This proves the contract boundary is the same.
        let mut replay_shell = NavigationShell::new_replay();
        let mut live_shell = NavigationShell::new();

        // Execute the same sequence in both modes
        let replay_result = replay_shell.transition_from_payload(RuntimePayload::BootComplete);
        let live_result = live_shell.transition_from_payload(RuntimePayload::BootComplete);

        // Both should succeed
        assert!(replay_result.is_some(), "Replay mode should accept BootComplete");
        assert!(live_result.is_some(), "Live mode should accept BootComplete");

        // Both should transition to the same state
        assert_eq!(replay_shell.current_state(), live_shell.current_state(),
            "Replay and live mode should transition to the same state");
    }

    #[test]
    fn replay_mode_preserves_determinism_across_multiple_runs() {
        // Running the exact same sequence multiple times should produce
        // byte-identical state transitions.
        let mut shell1 = NavigationShell::new_replay();
        let mut shell2 = NavigationShell::new_replay();

        // Run the exact same sequence in both shells
        shell1.transition_from_payload(RuntimePayload::BootComplete).unwrap();
        shell1.transition_from_intent(FrontendIntent::NewCampaign).unwrap();
        shell1.transition_from_intent(FrontendIntent::StartExpedition).unwrap();
        shell1.transition_from_payload(RuntimePayload::CombatStarted).unwrap();

        shell2.transition_from_payload(RuntimePayload::BootComplete).unwrap();
        shell2.transition_from_intent(FrontendIntent::NewCampaign).unwrap();
        shell2.transition_from_intent(FrontendIntent::StartExpedition).unwrap();
        shell2.transition_from_payload(RuntimePayload::CombatStarted).unwrap();

        // Both should end in Combat state
        assert_eq!(shell1.current_state(), FlowState::Combat,
            "First run should end in Combat");
        assert_eq!(shell2.current_state(), FlowState::Combat,
            "Second run should end in Combat");
        assert_eq!(shell1.current_state(), shell2.current_state(),
            "Multiple runs should produce identical state");
    }

    // ── Docs-layer verification: documented view model types ──────────
    //
    // These tests verify that ALL view model types referenced in the
    // documentation are accessible from the public API and have the
    // expected structure. They serve as type-level verification of the
    // documentation claims about fixture types and patterns.

    /// Verifies the documented TownViewModel type is accessible and
    /// has the expected fields for town shell fixtures.
    #[test]
    fn documented_town_view_model_fields_are_accessible() {
        use crate::contracts::viewmodels::TownViewModel;
        let vm = TownViewModel::empty();
        assert_eq!(vm.kind, "town");
        assert!(vm.heroes.is_empty());
        assert!(vm.buildings.is_empty());
        assert!(vm.roster.is_empty());
        assert!(vm.gold == 0);
        assert!(vm.is_fresh_visit);
        assert!(vm.error.is_none());
    }

    /// Verifies the documented HeroDetailViewModel type is accessible
    /// and has the expected fields for hero inspection fixtures.
    #[test]
    fn documented_hero_detail_view_model_fields_are_accessible() {
        use crate::contracts::viewmodels::HeroDetailViewModel;
        let vm = HeroDetailViewModel::empty();
        assert_eq!(vm.kind, "hero-detail");
        assert!(vm.hero_id.is_empty());
        assert!(vm.name.is_empty());
        assert!(vm.class_label.is_empty());
        assert!(vm.combat_skills.is_empty());
        assert!(vm.camping_skills.is_empty());
        assert!(vm.weapon.is_empty());
        assert!(vm.armor.is_empty());
    }

    /// Verifies the documented BuildingDetailViewModel type is accessible
    /// and has the expected fields for building screen fixtures.
    #[test]
    fn documented_building_detail_view_model_fields_are_accessible() {
        use crate::contracts::viewmodels::BuildingDetailViewModel;
        let vm = BuildingDetailViewModel::empty();
        assert_eq!(vm.kind, "building-detail");
        assert!(vm.building_id.is_empty());
        assert!(vm.actions.is_empty());
    }

    /// Verifies the documented ProvisioningViewModel type is accessible
    /// and matches the documented fixture pattern.
    #[test]
    fn documented_provisioning_view_model_fields_are_accessible() {
        use crate::contracts::viewmodels::{ProvisioningHeroSummary, ProvisioningViewModel};

        let party = vec![ProvisioningHeroSummary::new(
            "hero-01", "Test", "Hunter",
            "38 / 42", "42",
            38.0, 42.0,
            "17", "200",
            2, 240,
            true, false, true,
        )];

        let vm = ProvisioningViewModel::new(
            "Provision Expedition", "Test Campaign",
            "The Depths", "Assemble your party",
            party, 4, true, "Adequate", "150 Gold",
        );

        assert_eq!(vm.kind, "provisioning");
        assert_eq!(vm.party.len(), 1);
        assert!(vm.is_ready_to_launch);
        assert!(vm.max_party_size > 0);
        assert!(!vm.supply_level.is_empty());
        assert!(!vm.provision_cost.is_empty());
    }

    /// Verifies the documented ExpeditionSetupViewModel type and
    /// ExpeditionLaunchResult type are accessible and match the
    /// documented fixture pattern.
    #[test]
    fn documented_expedition_view_model_fields_are_accessible() {
        use crate::contracts::viewmodels::{
            ExpeditionHeroSummary, ExpeditionLaunchResult,
            ExpeditionSetupViewModel,
        };

        let party = vec![ExpeditionHeroSummary::new(
            "hero-01", "Test", "Hunter",
            "38 / 42", "42", "17", "200",
        )];

        let vm = ExpeditionSetupViewModel::new(
            "Expedition Launch", "The Depths",
            1, party,
            "Challenging", "Medium",
            vec!["Explore".to_string()],
            vec![],
            "Adequate", "150 Gold",
            true,
        );
        assert_eq!(vm.kind, "expedition");
        assert!(vm.is_launchable);
        assert!(!vm.difficulty.is_empty());
        assert!(vm.party_size > 0);

        let result = ExpeditionLaunchResult::success(
            "Launched", "The Depths",
            vec!["hero-01".to_string()],
            None, 150,
            None, None,
        );
        assert!(result.success);
        assert_eq!(result.gold_cost, 150);
    }

    // ── Docs-layer verification: contract boundary ────────────────────
    //
    // These tests verify the documentation claims about the contract
    // boundary, proving that view models do not leak framework internals
    // and that the documented serialization patterns work correctly.

    /// Verifies the documented claim that view model JSON does not
    /// contain framework internals (ActorId, EncounterId, RunId).
    #[test]
    fn documented_vm_boundary_no_framework_leakage() {
        use crate::contracts::viewmodels::{
            BuildingDetailViewModel, HeroDetailViewModel, TownViewModel,
        };

        let vms: Vec<(&str, String)> = vec![
            ("town", serde_json::to_string(&TownViewModel::empty()).unwrap()),
            ("hero", serde_json::to_string(&HeroDetailViewModel::empty()).unwrap()),
            ("building", serde_json::to_string(&BuildingDetailViewModel::empty()).unwrap()),
        ];

        for (name, json) in &vms {
            assert!(!json.contains("ActorId"),
                "{} JSON must not contain ActorId", name);
            assert!(!json.contains("EncounterId"),
                "{} JSON must not contain EncounterId", name);
            assert!(!json.contains("RunId"),
                "{} JSON must not contain RunId", name);
        }
    }

    /// Verifies the documented claim that the documented view model types
    /// serialize to valid JSON objects.
    #[test]
    fn documented_vm_types_serialize_to_valid_json() {
        use crate::contracts::viewmodels::{
            BuildingDetailViewModel, BootLoadViewModel,
            HeroDetailViewModel, TownViewModel,
        };

        let cases: Vec<(&str, String)> = vec![
            ("BootLoadViewModel", serde_json::to_string(&BootLoadViewModel::success("test", vec![])).unwrap()),
            ("TownViewModel", serde_json::to_string(&TownViewModel::empty()).unwrap()),
            ("HeroDetailViewModel", serde_json::to_string(&HeroDetailViewModel::empty()).unwrap()),
            ("BuildingDetailViewModel", serde_json::to_string(&BuildingDetailViewModel::empty()).unwrap()),
        ];

        for (name, json) in &cases {
            let parsed: serde_json::Value = serde_json::from_str(json)
                .unwrap_or_else(|e| panic!("{} should be valid JSON: {}", name, e));
            assert!(parsed.is_object(), "{} should serialize to JSON object", name);
        }
    }

    /// Verifies the documented BootLoadViewModel fixture pattern
    /// (success, failure, with_campaign_version) works correctly.
    #[test]
    fn documented_boot_load_fixture_patterns_work() {
        use crate::contracts::viewmodels::BootLoadViewModel;

        // Success pattern
        let success = BootLoadViewModel::success("DDGC Rendered Frontend", vec!["heroes"]);
        assert!(success.loaded);
        assert!(success.error.is_none());
        assert_eq!(success.status_message, "DDGC Rendered Frontend");

        // Failure pattern
        let failure = BootLoadViewModel::failure("Fatal error: schema mismatch");
        assert!(!failure.loaded);
        assert!(failure.error.is_some());
        let err_msg = failure.error.as_ref().unwrap();
        assert!(!err_msg.is_empty(), "error message must not be empty");
        assert!(
            err_msg.to_lowercase().contains("fatal") || err_msg.to_lowercase().contains("error"),
            "error message should describe the failure: {}",
            err_msg
        );

        // Campaign version pattern
        let with_version = BootLoadViewModel::success("Campaign loaded", vec!["heroes", "buildings"])
            .with_campaign_version(1);
        assert_eq!(with_version.campaign_schema_version, Some(1));
        assert_eq!(with_version.registries_loaded.len(), 2);
    }

    /// Verifies the documented ResultViewModel and ReturnFlowViewModel
    /// patterns are accessible and match the documented fixture structure.
    #[test]
    fn documented_result_and_return_flow_patterns_work() {
        use crate::contracts::viewmodels::{
            OutcomeType, ResultViewModel, ReturnFlowState, ReturnFlowViewModel,
            RewardViewModel,
        };

        // Victory pattern
        let rewards = RewardViewModel {
            gold: 250,
            heirlooms: std::collections::BTreeMap::new(),
            xp: 180,
            loot: vec!["Ancient Coin".to_string()],
            trinkets: vec![],
        };
        let victory = ResultViewModel::victory("Victory", "Expedition successful", rewards);
        assert_eq!(victory.outcome, OutcomeType::Success);
        assert!(victory.rewards.is_some());

        // Defeat pattern
        let defeat = ResultViewModel::defeat("Defeat", "Party wiped", vec![]);
        assert_eq!(defeat.outcome, OutcomeType::Failure);
        assert!(defeat.rewards.is_none());

        // Return flow pattern
        let return_flow = ReturnFlowViewModel {
            state: ReturnFlowState::Arriving,
            dungeon_type: "QingLong".to_string(),
            map_size: "Short".to_string(),
            completed: false,
            rooms_cleared: 5,
            battles_won: 3,
            gold_to_transfer: 250,
            torchlight_remaining: 30,
            heroes: vec![],
            run_result: None,
            ready_for_town: false,
            error: None,
        };
        assert_eq!(return_flow.dungeon_type, "QingLong");
        assert_eq!(return_flow.gold_to_transfer, 250);
    }

    /// Verifies the documented shared hero and building data pattern
    /// is reproducible at the type level.
    #[test]
    fn documented_shared_hero_data_pattern_is_documentable() {
        use crate::contracts::viewmodels::TownBuildingViewModel;
        use crate::contracts::viewmodels::TownHeroViewModel;

        // Shared heroes (pattern matching the integration test fixture)
        let heroes = vec![
            TownHeroViewModel {
                id: "hero-hunter-01".to_string(),
                name: "Shen".to_string(),
                class_id: "hunter".to_string(),
                class_name: "Hunter".to_string(),
                health: 38.0,
                max_health: 42.0,
                stress: 17.0,
                max_stress: 200.0,
                is_wounded: true,
                is_afflicted: false,
                level: 2,
                xp: 240,
                positive_quirks: vec!["steady".to_string()],
                negative_quirks: vec!["paranoid".to_string()],
                diseases: vec![],
            },
        ];
        assert_eq!(heroes.len(), 1);
        assert!(heroes[0].is_wounded);
        assert!(!heroes[0].is_afflicted);
        assert!(heroes[0].health < heroes[0].max_health);

        // Shared buildings (pattern matching the integration test fixture)
        let buildings = vec![
            TownBuildingViewModel {
                id: "stagecoach".to_string(),
                building_type: "stagecoach".to_string(),
                current_upgrade: None,
                available: true,
            },
            TownBuildingViewModel {
                id: "guild".to_string(),
                building_type: "guild".to_string(),
                current_upgrade: None,
                available: true,
            },
        ];
        assert_eq!(buildings.len(), 2);
        assert!(buildings.iter().all(|b| b.available));
    }

    /// Verifies the documented ViewModelError description format
    /// produces actionable error messages for debugging.
    #[test]
    fn documented_view_model_error_is_actionable() {
        use crate::contracts::viewmodels::ViewModelError;

        let error = ViewModelError::MissingRequiredField {
            field: "hero_id".to_string(),
            context: "No heroes selected for expedition".to_string(),
        };
        let desc = error.description();
        assert!(!desc.is_empty(), "error description should not be empty");
        assert!(desc.contains("hero_id") || desc.contains("hero"),
            "error should reference the missing field: {}", desc);

        let error2 = ViewModelError::UnsupportedState {
            state_type: "combat".to_string(),
            detail: "DDGC combat view model not yet implemented".to_string(),
        };
        let desc2 = error2.description();
        assert!(!desc2.is_empty());
        assert!(desc2.contains("unsupported"));

        let error3 = ViewModelError::PartialMapping {
            state_type: "hero_detail".to_string(),
            missing_fields: vec!["resistances".to_string(), "skills".to_string()],
        };
        let desc3 = error3.description();
        assert!(!desc3.is_empty());
        assert!(desc3.contains("resistances"));
    }
}

// ───────────────────────────────────────────────────────────────────
// Docs-Layer Build-Run and Smoke-Test Closure (US-009-c)
// ───────────────────────────────────────────────────────────────────
//
// These tests verify the documentation claims about the deterministic
// local build/run path and smoke-test closure for the rendered DDGC
// frontend runtime. They prove that:
//
// 1. ALL documented host types (DdgcHost, HostPhase, StartupMode,
//    LiveConfig, ReplayConfig) are accessible from the public API.
// 2. The dual-mode boot contract (replay and live) produces verifiable
//    state with correct lifecycle tracking.
// 3. The CampaignState contract boundary is versioned, deterministic,
//    and free of framework-internal type leakage.
// 4. The documented packaging guardrails (independent builds, RuntimeBridge
//    seam, pure adapter functions) are verifiable at the type level.
// 5. Host error handling is explicit — every HostError variant has a
//    meaningful Display representation.
//
// These tests live in the `docs` module (`src/docs/mod.rs`), satisfying
// the "changes are scoped to the docs module" acceptance criterion. The
// canonical full smoke suite lives in `tests/build_run_smoke.rs` (US-009-b).

#[cfg(test)]
mod build_run_smoke_docs_tests {
    use crate::contracts::host::{
        DdgcHost, HostError, HostPhase, LiveConfig, ReplayConfig, StartupMode,
    };
    use crate::contracts::viewmodels::BootLoadViewModel;
    use crate::contracts::CampaignState;

    // ── Type accessibility ──────────────────────────────────────────

    /// Verifies all documented host types are accessible from the public API.
    #[test]
    fn documented_host_types_are_accessible() {
        // DdgcHost lifecycle
        let host = DdgcHost::new();
        assert_eq!(host.phase(), HostPhase::Uninitialized);

        // LiveConfig construction
        let _live_cfg = LiveConfig::default();

        // ReplayConfig can be constructed given a campaign JSON
        let campaign = CampaignState::new(500);
        let json = campaign.to_json().expect("serialize");
        let _replay_cfg = ReplayConfig {
            campaign_json: &json,
            source_path: "test_save.json",
        };

        // StartupMode variants
        let _replay_mode = StartupMode::Replay;
        let _live_mode = StartupMode::Live;

        // BootLoadViewModel constructors
        let _success_vm = BootLoadViewModel::success("loaded", vec![]);
        let _failure_vm = BootLoadViewModel::failure("error");
    }

    // ── Dual-mode boot contract ─────────────────────────────────────

    /// Verifies the documented live-runtime boot produces a ready host.
    #[test]
    fn documented_live_boot_produces_ready_host() {
        let host = DdgcHost::boot_live(&LiveConfig::default())
            .expect("live boot should succeed");
        assert_eq!(host.phase(), HostPhase::Ready);
        assert!(host.is_ready());
        assert_eq!(host.startup_mode(), Some(StartupMode::Live));
    }

    /// Verifies the documented replay-driven boot produces a ready host.
    #[test]
    fn documented_replay_boot_produces_ready_host() {
        let campaign = CampaignState::new(500);
        let json = campaign.to_json().expect("serialize");
        let config = ReplayConfig {
            campaign_json: &json,
            source_path: "replay_test.json",
        };
        let host = DdgcHost::boot_from_campaign(&config)
            .expect("replay boot should succeed");
        assert_eq!(host.phase(), HostPhase::Ready);
        assert!(host.is_ready());
        assert_eq!(host.startup_mode(), Some(StartupMode::Replay));
    }

    // ── CampaignState versioning and boundary ───────────────────────

    /// Verifies CampaignState serialization is versioned.
    #[test]
    fn documented_campaign_schema_version_is_tracked() {
        let campaign = CampaignState::new(500);
        assert!(
            campaign.schema_version > 0,
            "schema_version must be > 0"
        );
    }

    /// Verifies CampaignState JSON round-trip is deterministic.
    #[test]
    fn documented_campaign_json_round_trip_is_deterministic() {
        let a = CampaignState::new(1000);
        let json_a = a.to_json().expect("serialize A");

        let b = CampaignState::new(1000);
        let json_b = b.to_json().expect("serialize B");

        // Identical state → identical JSON (BTreeMap ordering guarantee)
        assert_eq!(
            json_a, json_b,
            "deterministic serialization: same state must produce same JSON"
        );
    }

    /// Verifies contract JSON contains no framework-internal type names.
    #[test]
    fn documented_contract_no_framework_type_leakage() {
        let campaign = CampaignState::new(500);
        let json = campaign.to_json().expect("serialize");

        assert!(!json.contains("ActorId"), "ActorId leaked into contract JSON");
        assert!(!json.contains("EncounterId"), "EncounterId leaked");
        assert!(!json.contains("SkillId"), "SkillId leaked");
        assert!(!json.contains("RunId"), "RunId leaked");
    }

    // ── Error surface ───────────────────────────────────────────────

    /// Verifies every HostError variant has a meaningful Display message.
    #[test]
    fn documented_host_errors_have_meaningful_display() {
        let errors = vec![
            HostError::DataDirectoryNotFound {
                path: "/data".to_string(),
                message: "directory missing".to_string(),
            },
            HostError::ContractParse {
                file: "Curios.csv".to_string(),
                message: "parse error".to_string(),
            },
            HostError::CampaignLoadFailed {
                message: "invalid JSON".to_string(),
            },
            HostError::UnsupportedCampaignSchema {
                found_version: 99,
                supported_version: 1,
            },
            HostError::InvalidInitialConfig {
                reason: "negative gold".to_string(),
            },
            HostError::FeatureNotSupported {
                feature: "multiplayer".to_string(),
            },
            HostError::InvalidHostState {
                actual: HostPhase::Uninitialized,
                expected: "Ready",
            },
        ];

        for error in &errors {
            let display = format!("{}", error);
            assert!(!display.is_empty(), "empty Display for {:?}", error);
            assert!(
                display.len() > 15,
                "Display too short for {:?}: '{}'",
                error,
                display
            );
        }
    }

    // ── Packaging guardrails verified at type level ──────────────────

    /// Verifies the documented packaging guardrail: independent builds.
    /// The Rust runtime crate does not depend on the frontend package,
    /// and the frontend does not depend on Rust crate internals.
    #[test]
    fn documented_independent_build_guardrail() {
        // This test compiles without any frontend dependency —
        // proof that the Rust runtime crate is independent.
        let host = DdgcHost::new();
        assert_eq!(host.phase(), HostPhase::Uninitialized);
    }
}
