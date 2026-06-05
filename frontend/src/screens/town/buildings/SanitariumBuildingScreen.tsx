import { For, createSignal, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface SanitariumBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

type SanitariumTab = "use" | "upgrade";
type TreatmentMode = "quirk" | "disease" | null;

/**
 * Sanitarium (细胞修复站) building screen — faithful to Unity prefab.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Sanitarium/SanitariumWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_cell_repair.png
 * GUID: 55375034893560044a266e905926e8ff
 *
 * Related prefabs:
 *   Assets/Prefabs/UI/TreatmentHeroSlot.prefab  — treatment slot pattern
 *   Assets/Prefabs/UI/QuirkTreatmentSlot.prefab — quirk/disease slot pattern
 *
 * Sub-windows (from EstateManagement.unity hierarchy):
 *   SanitariumQuirkWindow   → QuirkTreatmentBackdrop
 *   SanitariumDiseaseWindow → DiseaseTreatmentBackdrop
 *
 * DATA BLOCKER — HB-iamshenkui-GameMigration-25
 *   The BuildingDetailViewModel contract does not yet carry:
 *   • hero roster with quirks/diseases for treatment slot population
 *   • per-slot lock/unlock state
 *   • per-treatment-type dynamic cost values
 *   • upgrade tree node completion state
 *   Until the runtime/bridge populates these, the UI renders the correct
 *   structural slots with placeholder/empty states.
 */
export const SanitariumBuildingScreen: Component<SanitariumBuildingScreenProps> = (
  props
) => {
  const vm = () => props.viewModel;
  const [activeTab, setActiveTab] = createSignal<SanitariumTab>("use");
  const [treatmentMode, setTreatmentMode] = createSignal<TreatmentMode>("quirk");

  // Derive action categories from the generic action list
  const quirkAction = () =>
    vm().actions.find(
      (a) =>
        a.id.includes("quirk") || a.id.includes("remove-quirk") || a.id.includes("positive")
    );
  const diseaseAction = () =>
    vm().actions.find(
      (a) => a.id.includes("disease") || a.id.includes("cure")
    );
  const stressAction = () =>
    vm().actions.find((a) => a.id.includes("stress"));
  const upgradeActions = () =>
    vm().actions.filter(
      (a) => a.id.includes("slot") || a.id.includes("upgrade") || a.id.includes("cost")
    );

  // Slot placeholders mirroring Unity QuirkTreatmentSlot layout
  const NEGATIVE_QUIRK_SLOTS = 5;
  const POSITIVE_QUIRK_SLOTS = 5;
  const DISEASE_SLOTS = 3;

  return (
    <div
      class="app-frame sanitarium-window"
      data-source-scene="Assets/Scenes/EstateManagement.unity"
      data-source-prefab="Assets/Prefabs/UI/Estate/Buildings/Sanitarium/SanitariumWindow.prefab"
    >
      {/* ── Building Header — mirrors SanitariumWindow/LeftPanel/Title + Icon ── */}
      <BuildingDetailHeader
        buildingId="sanitarium"
        label={vm().label}
        status={vm().status}
        description={vm().description}
        sourcePrefabPath="Assets/Prefabs/UI/Estate/Buildings/Sanitarium/SanitariumWindow.prefab"
        sourceSpritePath="Assets/Sprites/town/buildings/building_cell_repair.png"
        sourceGuid="55375034893560044a266e905926e8ff"
      />

      {/* ── Content: left (info + npc) + right (tabs + panels) ── */}
      <div class="building-detail-content sanitarium-content">
        {/* Left Panel — mirrors SanitariumWindow/LeftPanel */}
        <div
          class="building-detail-left sanitarium-left"
          data-source-hierarchy="SanitariumWindow/LeftPanel"
        >
          {/* NPC Character — mirrors LeftPanel/Character */}
          <div
            class="sanitarium-npc-area"
            data-source-component="Character"
            data-source-sprite="Assets/Sprites/town/npc/npc_sanitarium.png"
          >
            <div class="sanitarium-npc-circle" data-source-sprite="Assets/Sprites/ui/char_bg.png" />
            <span class="sanitarium-npc-label">细胞修复站</span>
          </div>

          {/* Talk button — mirrors LeftPanel/TalkButton */}
          <button
            class="sanitarium-talk-btn"
            data-source-component="TalkButton"
            onClick={() => {
              const firstAvailable = vm().actions.find((a) => a.isAvailable);
              if (firstAvailable) props.onAction(firstAvailable.id);
            }}
          >
            对话
          </button>
        </div>

        {/* Right Panel — mirrors SanitariumWindow/RightPanel */}
        <div
          class="building-detail-right sanitarium-right"
          data-source-hierarchy="SanitariumWindow/RightPanel"
        >
          {/* Tab bar — Use / Upgrade */}
          <div class="sanitarium-tab-bar">
            <button
              class={`sanitarium-tab ${activeTab() === "use" ? "sanitarium-tab--active" : ""}`}
              onClick={() => setActiveTab("use")}
              data-source-component="UseButton"
            >
              使用设施
            </button>
            <button
              class={`sanitarium-tab ${activeTab() === "upgrade" ? "sanitarium-tab--active" : ""}`}
              onClick={() => setActiveTab("upgrade")}
              data-source-component="UpgradeButton"
            >
              升级设施
            </button>
          </div>

          {/* ── Use Tab ── */}
          {activeTab() === "use" && (
            <div class="sanitarium-use-panel">
              {/* Treatment type selector — mirrors UseWindow (Disease + Quirk cards) */}
              <div class="sanitarium-treatment-selector">
                <button
                  class={`sanitarium-treatment-card ${treatmentMode() === "disease" ? "sanitarium-treatment-card--active" : ""}`}
                  onClick={() => setTreatmentMode("disease")}
                  data-treatment="disease"
                >
                  <span class="sanitarium-treatment-icon" data-source-sprite="Assets/Sprites/town/icons/sanitarium.desease.png">
                    <span class="sanitarium-treatment-icon-fallback">异</span>
                  </span>
                  <span class="sanitarium-treatment-label">异源细胞工坊</span>
                  <span class="sanitarium-treatment-desc">后遗症抹除</span>
                </button>
                <button
                  class={`sanitarium-treatment-card ${treatmentMode() === "quirk" ? "sanitarium-treatment-card--active" : ""}`}
                  onClick={() => setTreatmentMode("quirk")}
                  data-treatment="quirk"
                >
                  <span class="sanitarium-treatment-icon" data-source-sprite="Assets/Sprites/town/icons/sanitarium.quirk.png">
                    <span class="sanitarium-treatment-icon-fallback">心</span>
                  </span>
                  <span class="sanitarium-treatment-label">心魇斋</span>
                  <span class="sanitarium-treatment-desc">消除负面神降，锁定正面神降</span>
                </button>
              </div>

              {/* ── Quirk Treatment Backdrop (Selection 1) ── */}
              {treatmentMode() === "quirk" && (
                <div
                  class="sanitarium-backdrop sanitarium-quirk-backdrop"
                  data-source-prefab="Assets/Prefabs/UI/Estate/Buildings/Sanitarium/SanitariumQuirkWindow.prefab"
                  data-source-component="SanitariumQuirkWindow"
                >
                  <h3 class="sanitarium-backdrop-label" data-source-text="选择要治疗的神降">
                    选择要治疗的神降
                  </h3>

                  {/* Negative quirks */}
                  <div class="sanitarium-quirk-section">
                    <h4 class="sanitarium-quirk-section-title">负面特质</h4>
                    <div class="sanitarium-slot-grid">
                      <For each={Array.from({ length: NEGATIVE_QUIRK_SLOTS }, (_, i) => i)}>
                        {(i) => (
                          <div
                            class="sanitarium-quirk-slot"
                            data-source-prefab="Assets/Prefabs/UI/QuirkTreatmentSlot.prefab"
                            data-slot-type="negative"
                            data-slot-index={i}
                          >
                            <span class="sanitarium-quirk-slot-label">负面特质</span>
                            <span class="sanitarium-quirk-slot-lock" />
                          </div>
                        )}
                      </For>
                    </div>
                  </div>

                  {/* Positive quirks */}
                  <div class="sanitarium-quirk-section">
                    <h4 class="sanitarium-quirk-section-title">正面特质</h4>
                    <div class="sanitarium-slot-grid">
                      <For each={Array.from({ length: POSITIVE_QUIRK_SLOTS }, (_, i) => i)}>
                        {(i) => (
                          <div
                            class="sanitarium-quirk-slot sanitarium-quirk-slot--positive"
                            data-source-prefab="Assets/Prefabs/UI/QuirkTreatmentSlot.prefab"
                            data-slot-type="positive"
                            data-slot-index={i}
                          >
                            <span class="sanitarium-quirk-slot-label">正面特质</span>
                            <span class="sanitarium-quirk-slot-lock" />
                          </div>
                        )}
                      </For>
                    </div>
                  </div>

                  {/* Cost + Action */}
                  <div class="sanitarium-backdrop-footer">
                    <span class="sanitarium-cost">
                      <span class="gold-icon-fallback" />
                      <span class="sanitarium-cost-label">花费：</span>
                      <span class="sanitarium-cost-value" data-blocker="quirk-cost-not-wired">
                        {quirkAction()?.cost ?? "1350"}
                      </span>
                    </span>
                    <button
                      class="sanitarium-activity-btn"
                      onClick={() => {
                        const action = quirkAction();
                        if (action) props.onAction(action.id);
                      }}
                      disabled={!quirkAction()?.isAvailable}
                      data-source-component="ActivityButton"
                    >
                      开始治疗
                    </button>
                  </div>
                </div>
              )}

              {/* ── Disease Treatment Backdrop ── */}
              {treatmentMode() === "disease" && (
                <div
                  class="sanitarium-backdrop sanitarium-disease-backdrop"
                  data-source-prefab="Assets/Prefabs/UI/Estate/Buildings/Sanitarium/SanitariumDiseaseWindow.prefab"
                  data-source-component="SanitariumDiseaseWindow"
                >
                  <h3 class="sanitarium-backdrop-label" data-source-text="选择要治疗的后遗症">
                    选择要治疗的后遗症
                  </h3>

                  <div class="sanitarium-slot-grid sanitarium-disease-grid">
                    <For each={Array.from({ length: DISEASE_SLOTS }, (_, i) => i)}>
                      {(i) => (
                        <div
                          class="sanitarium-disease-slot"
                          data-source-prefab="Assets/Prefabs/UI/QuirkTreatmentSlot.prefab"
                          data-slot-type="disease"
                          data-slot-index={i}
                        >
                          <span class="sanitarium-disease-slot-icon" />
                          <span class="sanitarium-disease-slot-label">疾病</span>
                        </div>
                      )}
                    </For>
                  </div>

                  {/* Cost + Action */}
                  <div class="sanitarium-backdrop-footer">
                    <span class="sanitarium-cost">
                      <span class="gold-icon-fallback" />
                      <span class="sanitarium-cost-label">花费：</span>
                      <span class="sanitarium-cost-value" data-blocker="disease-cost-not-wired">
                        {diseaseAction()?.cost ?? "750"}
                      </span>
                    </span>
                    <button
                      class="sanitarium-activity-btn"
                      onClick={() => {
                        const action = diseaseAction();
                        if (action) props.onAction(action.id);
                      }}
                      disabled={!diseaseAction()?.isAvailable}
                      data-source-component="ActivityButton"
                    >
                      开始治疗
                    </button>
                  </div>
                </div>
              )}
            </div>
          )}

          {/* ── Upgrade Tab ── */}
          {activeTab() === "upgrade" && (
            <div
              class="sanitarium-upgrade-panel"
              data-source-component="UpgradeWindow"
            >
              <div class="sanitarium-upgrade-trees">
                {/* Medical tree — 解心魔 */}
                <div class="sanitarium-upgrade-tree" data-tree-id="quirk_treatment_chance">
                  <div class="sanitarium-upgrade-tree-header">
                    <span class="sanitarium-upgrade-tree-icon" data-source-sprite="Assets/Sprites/town/icons/sanitarium.medical.png">
                      <span class="sanitarium-upgrade-tree-icon-fallback">医</span>
                    </span>
                    <span class="sanitarium-upgrade-tree-label">解心魔</span>
                  </div>
                  <div class="sanitarium-upgrade-tree-slots">
                    <For each={Array.from({ length: 5 }, (_, i) => i)}>
                      {(i) => (
                        <div
                          class={`sanitarium-upgrade-node ${i === 0 ? "sanitarium-upgrade-node--active" : ""}`}
                          data-node-index={i}
                        />
                      )}
                    </For>
                  </div>
                </div>

                {/* Cells tree — 深潜医疗舱 */}
                <div class="sanitarium-upgrade-tree" data-tree-id="quirk_slots">
                  <div class="sanitarium-upgrade-tree-header">
                    <span class="sanitarium-upgrade-tree-icon" data-source-sprite="Assets/Sprites/town/icons/sanitarium.cells.png">
                      <span class="sanitarium-upgrade-tree-icon-fallback">舱</span>
                    </span>
                    <span class="sanitarium-upgrade-tree-label">深潜医疗舱</span>
                  </div>
                  <div class="sanitarium-upgrade-tree-slots">
                    <For each={Array.from({ length: 5 }, (_, i) => i)}>
                      {(i) => (
                        <div
                          class={`sanitarium-upgrade-node ${i === 0 ? "sanitarium-upgrade-node--active" : ""}`}
                          data-node-index={i}
                        />
                      )}
                    </For>
                  </div>
                </div>

                {/* Treatment tree — 调控仪 */}
                <div class="sanitarium-upgrade-tree" data-tree-id="disease_slots">
                  <div class="sanitarium-upgrade-tree-header">
                    <span class="sanitarium-upgrade-tree-icon" data-source-sprite="Assets/Sprites/town/icons/sanitarium.treatment.png">
                      <span class="sanitarium-upgrade-tree-icon-fallback">调</span>
                    </span>
                    <span class="sanitarium-upgrade-tree-label">调控仪</span>
                  </div>
                  <div class="sanitarium-upgrade-tree-slots">
                    <For each={Array.from({ length: 5 }, (_, i) => i)}>
                      {(i) => (
                        <div
                          class={`sanitarium-upgrade-node ${i === 0 ? "sanitarium-upgrade-node--active" : ""}`}
                          data-node-index={i}
                        />
                      )}
                    </For>
                  </div>
                </div>
              </div>

              {/* Upgrade actions from view model */}
              {upgradeActions().length > 0 && (
                <div class="sanitarium-upgrade-actions">
                  <For each={upgradeActions()}>
                    {(action) => (
                      <div class="building-action-card sanitarium-upgrade-action-card">
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
                          <span class={`building-action-cost ${!action.isAvailable ? "building-action-cost-unavailable" : ""}`}>
                            Cost: <strong>{action.cost}</strong>
                          </span>
                          {action.isUnsupported ? (
                            <button class="building-action-btn building-action-btn--disabled" disabled>
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
                            <button class="building-action-btn building-action-btn--disabled" disabled>
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

      {/* ── Return to Town — mirrors SanitariumWindow/CloseButton ── */}
      <div class="building-return-row">
        <button
          class="building-return-btn sanitarium-return-btn"
          onClick={props.onReturn}
          data-source-component="CloseButton"
          data-source-sprite="Assets/Sprites/ui/btn_close.png"
        >
          离开
        </button>
      </div>
    </div>
  );
};
