import type { Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../bridge/contractTypes";
import { AppFrame } from "../../components/layout/AppFrame";

/**
 * Generic building detail screen — fallback for buildings without a
 * dedicated screen component.
 *
 * Layout mirrors the EstateManagement.unity.json hierarchy pattern:
 *   UI_Shared/UI_LowWindows/{Building}Window/
 *     LeftPanel/
 *       Title/
 *         BuildingLabel   → AppFrame title
 *         BuildingDesc    → subtitle / description section
 *       Icon              → building marker
 *       TalkButton        → (mapped to first action trigger)
 *       CloseButton       → Return to Town
 *     RightPanel/
 *       UpgradeButton     → primary actions
 *       UpgradeWindow     → upgrade tree display
 *
 * Original sprite reference (varies by building):
 *   See asset-manifest.json section "building_sprites" for each
 *   building's asset_path and GUID.
 */

interface BuildingDetailScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

/** Generic building SVG used as fallback icon marker */
const GenericBuildingSvg = () => (
  <svg width="48" height="48" viewBox="0 0 48 48" fill="none" class="building-detail-icon">
    <rect x="14" y="14" width="20" height="24" rx="2" fill="#1b322c" stroke="#c6d46a" stroke-width="0.6" opacity="0.6" />
    <polygon points="11,14 24,6 37,14" fill="#c6d46a" opacity="0.25" stroke="#c6d46a" stroke-width="0.5" />
    <rect x="20" y="28" width="8" height="10" rx="1" fill="#0e1714" stroke="#c6d46a" stroke-width="0.4" opacity="0.5" />
  </svg>
);

export const BuildingDetailScreen: Component<BuildingDetailScreenProps> = (props) => {
  const statusLabel = () => {
    switch (props.viewModel.status) {
      case "ready":
        return "Operational";
      case "partial":
        return "Partially Available";
      case "locked":
        return "Locked";
    }
  };

  const statusClass = () => {
    switch (props.viewModel.status) {
      case "ready":
        return "status-ready";
      case "partial":
        return "status-partial";
      case "locked":
        return "status-locked";
    }
  };

  return (
    <AppFrame
      eyebrow="Building Detail"
      title={props.viewModel.label}
      subtitle={`Status: ${statusLabel()}`}
    >
      <div class="grid">
        {/* ── LeftPanel / Title ──────────────────────────────── */}
        <div class="stack">
          {/* Building icon */}
          <div class="building-detail-icon-wrap">
            <GenericBuildingSvg />
          </div>

          <section class="panel stack">
            <h2 class="panel-title">Building Status</h2>
            <div class="surface-card stack">
              <div class="row">
                <span class="stat-label">Status</span>
                <span class={`stat-value ${statusClass()}`}>{statusLabel()}</span>
              </div>
              <div class="row">
                <span class="stat-label">Building</span>
                <span class="stat-value">{props.viewModel.label}</span>
              </div>
              {props.viewModel.upgradeRequirement && (
                <div class="row">
                  <span class="stat-label">Upgrade Requirement</span>
                  <span class="stat-value">{props.viewModel.upgradeRequirement}</span>
                </div>
              )}
            </div>
          </section>

          {/* BuildingDesc section */}
          <section class="panel stack">
            <h2 class="panel-title">Description</h2>
            <div class="surface-card">
              <p>{props.viewModel.description}</p>
            </div>
          </section>
        </div>

        {/* ── RightPanel / Actions ───────────────────────────── */}
        <div class="stack">
          <section class="panel stack">
            <h2 class="panel-title">Available Actions</h2>
            {props.viewModel.actions.length === 0 ? (
              <div class="surface-card">
                <p>No actions currently available for this building.</p>
              </div>
            ) : (
              <ul class="list-reset">
                {props.viewModel.actions.map((action) => (
                  <li class="surface-card stack">
                    <div class="row">
                      <strong class="action-label">{action.label}</strong>
                      {action.isUnsupported && (
                        <span class="pill pill-error">Unsupported</span>
                      )}
                      {!action.isAvailable && !action.isUnsupported && (
                        <span class="pill pill-warning">Unavailable</span>
                      )}
                    </div>
                    <p>{action.description}</p>
                    <div class="row">
                      <span class="stat-label">Cost</span>
                      <span class="stat-value">{action.cost}</span>
                    </div>
                    <div class="row">
                      {action.isUnsupported ? (
                        <button class="action-secondary" disabled>
                          Not Available
                        </button>
                      ) : action.isAvailable ? (
                        <button
                          class="action-primary"
                          onClick={() => props.onAction(action.id)}
                        >
                          {action.label}
                        </button>
                      ) : (
                        <button class="action-secondary" disabled>
                          Prerequisites Not Met
                        </button>
                      )}
                    </div>
                  </li>
                ))}
              </ul>
            )}
          </section>
        </div>
      </div>

      {/* ── CloseButton → Return to Town ─────────────────────── */}
      <div class="row">
        <button class="action-secondary" onClick={props.onReturn}>
          Return to Town
        </button>
      </div>
    </AppFrame>
  );
};
