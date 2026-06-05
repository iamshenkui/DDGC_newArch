import { For, createSignal, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface SanitariumBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

type FacilityMode = "upgrade" | "use";

/**
 * Sanitarium (细胞修复站) building screen.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Sanitarium/SanitariumWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_cell_repair.png
 * GUID: 55375034893560044a266e905926e8ff
 *
 * Reference layout (公会界面-细胞修复站-使用空):
 *   - Left panel: NPC attendant portrait + "对话" button
 *   - Right panel: facility mode tabs (升级设施 / 使用设施)
 *   - Treatment slots: 心理疾病, 异星细胞工坊 — each with hero portrait + empty slot
 *   - Bottom resource strip
 */
export const SanitariumBuildingScreen: Component<SanitariumBuildingScreenProps> = (
  props,
) => {
  const vm = () => props.viewModel;
  const [mode, setMode] = createSignal<FacilityMode>("use");

  const treatmentSlots = () => vm().treatmentSlots ?? [];

  // Categorise actions for the "upgrade" tab
  const upgradeActions = () =>
    vm().actions.filter(
      (a) =>
        a.id.includes("slot") ||
        a.id.includes("upgrade") ||
        a.id.includes("treatment-chance"),
    );

  const otherActions = () =>
    vm().actions.filter(
      (a) =>
        !a.id.includes("slot") &&
        !a.id.includes("upgrade") &&
        !a.id.includes("treatment-chance"),
    );

  return (
    <div class="app-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Building Header — mirrors SanitariumWindow/LeftPanel/Icon + Title ── */}
      <BuildingDetailHeader
        buildingId="sanitarium"
        label={vm().label}
        status={vm().status}
        description={vm().description}
        sourcePrefabPath="Assets/Prefabs/UI/Estate/Buildings/Sanitarium/SanitariumWindow.prefab"
        sourceSpritePath="Assets/Sprites/town/buildings/building_cell_repair.png"
        sourceGuid="55375034893560044a266e905926e8ff"
      />

      {/* ── Content — mirrors SanitariumWindow LeftPanel + RightPanel ── */}
      <div class="building-detail-content">
        {/* Left Panel — NPC Portrait + Talk button */}
        <div
          class="building-detail-left sanitarium-left-panel"
          data-source-hierarchy="SanitariumWindow/LeftPanel"
        >
          {/* NPC Portrait */}
          <div class="sanitarium-npc-area">
            <div class="sanitarium-npc-portrait-frame">
              {vm().npcPortrait ? (
                <img
                  class="sanitarium-npc-portrait-img"
                  src={vm().npcPortrait}
                  alt="细胞修复站医师"
                  loading="eager"
                />
              ) : (
                <div class="sanitarium-npc-portrait-fallback">
                  <span class="sanitarium-npc-initial">医</span>
                </div>
              )}
            </div>
            <button
              class="sanitarium-talk-btn"
              onClick={() => props.onAction("talk")}
              data-source-component="TalkButton"
            >
              对话
            </button>
          </div>

          {/* Building info card (compact) */}
          <div class="building-info-card">
            <h3 class="building-info-card-title">Building Status</h3>
            <div class="building-info-row">
              <span class="building-info-label">Status</span>
              <span class="building-info-value">
                {vm().status === "ready"
                  ? "Operational"
                  : vm().status === "partial"
                    ? "Partially Available"
                    : "Locked"}
              </span>
            </div>
            {vm().currentUpgrade && (
              <div class="building-info-row">
                <span class="building-info-label">Treatment Level</span>
                <span class="building-info-value">{vm().currentUpgrade}</span>
              </div>
            )}
            {vm().upgradeRequirement && (
              <div class="building-info-row">
                <span class="building-info-label">Requirement</span>
                <span class="building-info-value">{vm().upgradeRequirement}</span>
              </div>
            )}
          </div>
        </div>

        {/* Right Panel — Facility tabs + content */}
        <div
          class="building-detail-right sanitarium-right-panel"
          data-source-hierarchy="SanitariumWindow/RightPanel"
        >
          {/* Facility mode tabs */}
          <div class="sanitarium-mode-tabs">
            <label class="sanitarium-mode-tab">
              <input
                type="checkbox"
                checked={mode() === "upgrade"}
                onChange={() => setMode("upgrade")}
              />
              <span class="sanitarium-mode-label">升级设施</span>
            </label>
            <label class="sanitarium-mode-tab">
              <input
                type="checkbox"
                checked={mode() === "use"}
                onChange={() => setMode("use")}
              />
              <span class="sanitarium-mode-label">使用设施</span>
            </label>
          </div>

          {/* Use Facility mode — Treatment Slots */}
          {mode() === "use" && (
            <div class="sanitarium-treatment-section">
              <For each={treatmentSlots()}>
                {(slot) => (
                  <div
                    class="sanitarium-treatment-slot"
                    data-source-prefab="Assets/Prefabs/UI/TreatmentHeroSlot.prefab"
                    data-source-component="TreatmentHeroSlot"
                    data-slot-id={slot.id}
                  >
                    {/* Slot label + hero thumbnail */}
                    <div class="sanitarium-slot-left">
                      <span class="sanitarium-slot-label">{slot.label}</span>
                      <span class="sanitarium-slot-desc">{slot.description}</span>
                      {slot.heroId ? (
                        <div class="sanitarium-slot-hero">
                          <div class="sanitarium-slot-hero-portrait">
                            {slot.heroPortrait ? (
                              <img
                                src={slot.heroPortrait}
                                alt={slot.heroName}
                                class="sanitarium-slot-hero-img"
                              />
                            ) : (
                              <span class="sanitarium-slot-hero-initial">
                                {slot.heroName?.[0] ?? "?"}
                              </span>
                            )}
                          </div>
                          <span class="sanitarium-slot-hero-name">{slot.heroName}</span>
                          <span class="sanitarium-slot-hero-class">
                            {slot.heroClassLabel}
                          </span>
                        </div>
                      ) : (
                        <div class="sanitarium-slot-hero sanitarium-slot-hero--empty">
                          <div class="sanitarium-slot-hero-portrait sanitarium-slot-hero-portrait--empty">
                            <span class="sanitarium-slot-empty-mark">+</span>
                          </div>
                          <span class="sanitarium-slot-hero-name sanitarium-slot-hero-name--empty">
                            空置
                          </span>
                        </div>
                      )}
                    </div>

                    {/* Treatment slot action area */}
                    <div class="sanitarium-slot-right">
                      <div
                        class={`sanitarium-slot-target ${
                          slot.heroId ? "sanitarium-slot-target--filled" : "sanitarium-slot-target--empty"
                        }`}
                      >
                        {slot.heroId ? (
                          <>
                            <span class="sanitarium-slot-status">治疗中</span>
                            <span class="sanitarium-slot-cost">
                              费用: <strong>{slot.cost}</strong>
                            </span>
                          </>
                        ) : (
                          <>
                            <span class="sanitarium-slot-status sanitarium-slot-status--empty">
                              空床位
                            </span>
                            <span class="sanitarium-slot-cost">
                              费用: <strong>{slot.cost}</strong>
                            </span>
                            <button
                              class="building-action-btn building-action-btn--primary"
                              onClick={() => props.onAction(`treat-${slot.id}`)}
                              disabled={!slot.isAvailable}
                            >
                              选择英雄
                            </button>
                          </>
                        )}
                      </div>
                    </div>
                  </div>
                )}
              </For>

              {/* Fallback: if no treatment slots defined, show actions as generic list */}
              {treatmentSlots().length === 0 && (
                <div class="building-action-section">
                  <h3 class="building-action-section-title">Actions</h3>
                  {vm().actions.length === 0 ? (
                    <div class="building-info-card">
                      <p style="margin:0;color:rgba(218,198,168,0.5);font-size:0.82rem;">
                        No actions currently available for this building.
                      </p>
                    </div>
                  ) : (
                    <For each={vm().actions}>
                      {(action) => (
                        <div class="building-action-card">
                          <div class="building-action-card-header">
                            <span class="building-action-label">{action.label}</span>
                            {action.isUnsupported && (
                              <span class="building-action-pill building-action-pill--unsupported">
                                Unsupported
                              </span>
                            )}
                            {!action.isAvailable && !action.isUnsupported && (
                              <span class="building-action-pill building-action-pill--unavailable">
                                Unavailable
                              </span>
                            )}
                          </div>
                          <p class="building-action-desc">{action.description}</p>
                          <div class="building-action-footer">
                            <span
                              class={`building-action-cost ${
                                !action.isAvailable ? "building-action-cost-unavailable" : ""
                              }`}
                            >
                              Cost: <strong>{action.cost}</strong>
                            </span>
                            {action.isUnsupported ? (
                              <button
                                class="building-action-btn building-action-btn--disabled"
                                disabled
                              >
                                Not Available
                              </button>
                            ) : action.isAvailable ? (
                              <button
                                class="building-action-btn building-action-btn--primary"
                                onClick={() => props.onAction(action.id)}
                              >
                                {action.label}
                              </button>
                            ) : (
                              <button
                                class="building-action-btn building-action-btn--disabled"
                                disabled
                              >
                                Prerequisites Not Met
                              </button>
                            )}
                          </div>
                        </div>
                      )}
                    </For>
                  )}
                </div>
              )}
            </div>
          )}

          {/* Upgrade Facility mode */}
          {mode() === "upgrade" && (
            <div class="sanitarium-upgrade-section">
              {upgradeActions().length > 0 ? (
                <div class="building-action-section">
                  <h3 class="building-action-section-title">Facility Upgrades</h3>
                  <For each={upgradeActions()}>
                    {(action) => (
                      <div class="building-action-card">
                        <div class="building-action-card-header">
                          <span class="building-action-label">{action.label}</span>
                          {action.isUnsupported && (
                            <span class="building-action-pill building-action-pill--unsupported">
                              Unsupported
                            </span>
                          )}
                          {!action.isAvailable && !action.isUnsupported && (
                            <span class="building-action-pill building-action-pill--unavailable">
                              Unavailable
                            </span>
                          )}
                        </div>
                        <p class="building-action-desc">{action.description}</p>
                        <div class="building-action-footer">
                          <span
                            class={`building-action-cost ${
                              !action.isAvailable ? "building-action-cost-unavailable" : ""
                            }`}
                          >
                            Cost: <strong>{action.cost}</strong>
                          </span>
                          {action.isUnsupported ? (
                            <button
                              class="building-action-btn building-action-btn--disabled"
                              disabled
                            >
                              Not Available
                            </button>
                          ) : action.isAvailable ? (
                            <button
                              class="building-action-btn building-action-btn--primary"
                              onClick={() => props.onAction(action.id)}
                            >
                              {action.label}
                            </button>
                          ) : (
                            <div style="display:flex;gap:0.5rem;align-items:center;">
                              <button
                                class="building-action-btn building-action-btn--disabled"
                                disabled
                              >
                                Prerequisites Not Met
                              </button>
                              {vm().upgradeRequirement && (
                                <span class="building-action-pill building-action-pill--info">
                                  {vm().upgradeRequirement}
                                </span>
                              )}
                            </div>
                          )}
                        </div>
                      </div>
                    )}
                  </For>
                </div>
              ) : (
                <div class="building-info-card">
                  <p style="margin:0;color:rgba(218,198,168,0.5);font-size:0.82rem;">
                    暂无可用升级。提升城镇等级以解锁更多设施升级。
                  </p>
                </div>
              )}

              {/* Also show other uncategorised actions in upgrade tab */}
              {otherActions().length > 0 && (
                <div class="building-action-section" style={{ "margin-top": "14px" }}>
                  <h3 class="building-action-section-title">Other Services</h3>
                  <For each={otherActions()}>
                    {(action) => (
                      <div class="building-action-card">
                        <div class="building-action-card-header">
                          <span class="building-action-label">{action.label}</span>
                          {action.isUnsupported && (
                            <span class="building-action-pill building-action-pill--unsupported">
                              Unsupported
                            </span>
                          )}
                          {!action.isAvailable && !action.isUnsupported && (
                            <span class="building-action-pill building-action-pill--unavailable">
                              Unavailable
                            </span>
                          )}
                        </div>
                        <p class="building-action-desc">{action.description}</p>
                        <div class="building-action-footer">
                          <span
                            class={`building-action-cost ${
                              !action.isAvailable ? "building-action-cost-unavailable" : ""
                            }`}
                          >
                            Cost: <strong>{action.cost}</strong>
                          </span>
                          {action.isUnsupported ? (
                            <button
                              class="building-action-btn building-action-btn--disabled"
                              disabled
                            >
                              Not Available
                            </button>
                          ) : action.isAvailable ? (
                            <button
                              class="building-action-btn building-action-btn--primary"
                              onClick={() => props.onAction(action.id)}
                            >
                              {action.label}
                            </button>
                          ) : (
                            <button
                              class="building-action-btn building-action-btn--disabled"
                              disabled
                            >
                              Prerequisites Not Met
                            </button>
                          )}
                        </div>
                      </div>
                    )}
                  </For>
                </div>
              )}
            </div>
          )}
        </div>
      </div>

      {/* ── Resource Strip (bottom) ── */}
      <div class="sanitarium-resource-strip">
        <span class="sanitarium-resource-slot" data-resource="bust">
          <span class="sanitarium-resource-value">10</span>
        </span>
        <span class="sanitarium-resource-slot" data-resource="portrait">
          <span class="sanitarium-resource-value">10</span>
        </span>
        <span class="sanitarium-resource-slot" data-resource="deed">
          <span class="sanitarium-resource-value">10</span>
        </span>
        <span class="sanitarium-resource-slot" data-resource="crest">
          <span class="sanitarium-resource-value">20</span>
        </span>
        <span class="sanitarium-resource-slot sanitarium-resource-slot--gold" data-resource="gold">
          <span class="sanitarium-resource-value">5895</span>
        </span>
      </div>

      {/* ── Return to Town — mirrors SanitariumWindow/CloseButton ── */}
      <div class="building-return-row">
        <button
          class="building-return-btn"
          onClick={props.onReturn}
          data-source-component="CloseButton"
          data-source-sprite="Assets/Sprites/ui/btn_close.png"
        >
          Return to Town
        </button>
      </div>
    </div>
  );
};
