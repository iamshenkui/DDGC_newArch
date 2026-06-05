import { For, Show, createSignal, type Component } from "solid-js";

import type { BuildingDetailViewModel, TownHeroSummary } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface SanitariumBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

type TabKey = "upgrade" | "usage";
type ModalState =
  | { kind: "closed" }
  | { kind: "select-hero"; slotType: "quirk" | "disease"; slotLabel: string }
  | { kind: "confirm-treatment"; slotType: "quirk" | "disease"; heroId: string; cost: string };

/**
 * Fixture heroes for the sanitarium selection modal.
 * Mirrors the town roster so the selection UI has representative data.
 *
 * NOTE: These are local fixtures until the runtime wires hero roster
 * into BuildingDetailViewModel.availableHeroes (see MIGRATION_BLOCKER.md).
 */
const fixtureHeroes: TownHeroSummary[] = [
  {
    id: "hero-hunter-01",
    name: "测试官测试1",
    classLabel: "Hunter",
    hp: "31 / 31",
    maxHp: "31",
    health: 31,
    maxHealth: 31,
    stress: "0 / 200",
    maxStress: "200",
    level: 1,
    xp: 0,
    isWounded: false,
    isAfflicted: false,
    positiveQuirks: ["义无反顾"],
    negativeQuirks: ["恍惚"],
    diseases: ["神隐之印"],
  },
  {
    id: "hero-white-01",
    name: "测试人测试1",
    classLabel: "White",
    hp: "31 / 31",
    maxHp: "31",
    health: 31,
    maxHealth: 31,
    stress: "0 / 200",
    maxStress: "200",
    level: 1,
    xp: 0,
    isWounded: false,
    isAfflicted: false,
    positiveQuirks: [],
    negativeQuirks: [],
    diseases: [],
  },
  {
    id: "hero-black-01",
    name: "测试人测试2",
    classLabel: "Black",
    hp: "31 / 31",
    maxHp: "31",
    health: 31,
    maxHealth: 31,
    stress: "0 / 200",
    maxStress: "200",
    level: 1,
    xp: 0,
    isWounded: false,
    isAfflicted: false,
    positiveQuirks: [],
    negativeQuirks: [],
    diseases: ["异常阴影"],
  },
];

/**
 * Sanitarium (细胞修复站) building screen.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Sanitarium/SanitariumWindow.prefab
 *
 * Reference frames:
 *   - 公会界面-细胞修复站-使用空.png  →  usage tab with empty slots
 *   - 公会界面-细胞修复站-选择1.png  →  quirk-treatment hero selection
 *   - 公会界面-细胞修复站-选择2.png  →  disease-treatment hero + disease detail
 *
 * Original sprite: Assets/Sprites/town/buildings/building_cell_repair.png
 * GUID: 55375034893560044a266e905926e8ff
 */
export const SanitariumBuildingScreen: Component<SanitariumBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;
  const [activeTab, setActiveTab] = createSignal<TabKey>("usage");
  const [modal, setModal] = createSignal<ModalState>({ kind: "closed" });
  const [selectedHeroId, setSelectedHeroId] = createSignal<string | null>(null);

  const slots = () => vm().slots ?? [];

  const quirkSlot = () => slots().find((s) => s.slotType === "quirk");
  const diseaseSlot = () => slots().find((s) => s.slotType === "disease");

  const quirkAction = () => vm().actions.find((a) => a.id === "treat-quirk" || a.id === "remove-quirk");
  const diseaseAction = () => vm().actions.find((a) => a.id === "cure-disease" || a.id === "reduce-stress");

  function openSlotModal(slotType: "quirk" | "disease") {
    const slot = slotType === "quirk" ? quirkSlot() : diseaseSlot();
    if (!slot) return;
    setSelectedHeroId(null);
    setModal({ kind: "select-hero", slotType, slotLabel: slot.label });
  }

  function closeModal() {
    setModal({ kind: "closed" });
    setSelectedHeroId(null);
  }

  function confirmHeroSelection() {
    const m = modal();
    if (m.kind !== "select-hero" || !selectedHeroId()) return;
    const hero = fixtureHeroes.find((h) => h.id === selectedHeroId());
    if (!hero) return;

    if (m.slotType === "disease") {
      const cost = diseaseAction()?.cost ?? "1000 Gold";
      setModal({ kind: "confirm-treatment", slotType: "disease", heroId: selectedHeroId()!, cost });
    } else {
      // For quirk treatment, dispatch action directly
      const actionId = quirkAction()?.id ?? "treat-quirk";
      props.onAction(actionId);
      closeModal();
    }
  }

  function startTreatment() {
    const m = modal();
    if (m.kind !== "confirm-treatment") return;
    const actionId = diseaseAction()?.id ?? "cure-disease";
    props.onAction(actionId);
    closeModal();
  }

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

      {/* ── Tab Navigation — mirrors reference upgrade/usage tabs ── */}
      <div class="sanitarium-tabs" data-source-hierarchy="SanitariumWindow/TabBar">
        <button
          class={`sanitarium-tab ${activeTab() === "upgrade" ? "sanitarium-tab--active" : ""}`}
          onClick={() => setActiveTab("upgrade")}
          data-testid="tab-upgrade"
        >
          <span class="sanitarium-tab-checkbox">{activeTab() === "upgrade" ? "☑" : "☐"}</span>
          升级设施
        </button>
        <button
          class={`sanitarium-tab ${activeTab() === "usage" ? "sanitarium-tab--active" : ""}`}
          onClick={() => setActiveTab("usage")}
          data-testid="tab-usage"
        >
          <span class="sanitarium-tab-checkbox">{activeTab() === "usage" ? "☑" : "☐"}</span>
          使用设施
        </button>
      </div>

      {/* ── Content ── */}
      <div class="building-detail-content">
        {/* Left Panel — NPC portrait + building info */}
        <div class="building-detail-left" data-source-hierarchy="SanitariumWindow/LeftPanel">
          <div class="building-info-card">
            <h3 class="building-info-card-title">建筑状态</h3>
            <div class="building-info-row">
              <span class="building-info-label">状态</span>
              <span class="building-info-value">
                {vm().status === "ready" ? "运行中" : vm().status === "partial" ? "部分可用" : "锁定"}
              </span>
            </div>
            {vm().currentUpgrade && (
              <div class="building-info-row">
                <span class="building-info-label">治疗等级</span>
                <span class="building-info-value">{vm().currentUpgrade}</span>
              </div>
            )}
            {vm().upgradeRequirement && (
              <div class="building-info-row">
                <span class="building-info-label">升级需求</span>
                <span class="building-info-value">{vm().upgradeRequirement}</span>
              </div>
            )}
          </div>

          {/* NPC portrait area — mirrors reference left-side character */}
          <div class="sanitarium-npc-area" data-source-prefab="SanitariumWindow/NPC">
            <div class="sanitarium-npc-portrait">
              <span class="sanitarium-npc-label">对话</span>
            </div>
          </div>
        </div>

        {/* Right Panel */}
        <div class="building-detail-right" data-source-hierarchy="SanitariumWindow/RightPanel">
          <Show when={activeTab() === "upgrade"}>
            {/* Upgrade tab — reuse existing action cards for upgrade actions */}
            <div class="building-action-section">
              <h3 class="building-action-section-title">设施升级</h3>
              <For each={vm().actions.filter((a) => a.id.includes("upgrade"))}>
                {(action) => (
                  <div class="building-action-card">
                    <div class="building-action-card-header">
                      <span class="building-action-label">{action.label}</span>
                      {action.isUnsupported && (
                        <span class="building-action-pill building-action-pill--unsupported">Unsupported</span>
                      )}
                      {!action.isAvailable && !action.isUnsupported && (
                        <span class="building-action-pill building-action-pill--unavailable">Unavailable</span>
                      )}
                    </div>
                    <p class="building-action-desc">{action.description}</p>
                    <div class="building-action-footer">
                      <span class={`building-action-cost ${!action.isAvailable ? "building-action-cost-unavailable" : ""}`}>
                        Cost: <strong>{action.cost}</strong>
                      </span>
                      {action.isUnsupported ? (
                        <button class="building-action-btn building-action-btn--disabled" disabled>Not Available</button>
                      ) : action.isAvailable ? (
                        <button class="building-action-btn building-action-btn--primary" onClick={() => props.onAction(action.id)}>
                          {action.label}
                        </button>
                      ) : (
                        <button class="building-action-btn building-action-btn--disabled" disabled>Prerequisites Not Met</button>
                      )}
                    </div>
                  </div>
                )}
              </For>
              {vm().actions.filter((a) => a.id.includes("upgrade")).length === 0 && (
                <div class="building-info-card">
                  <p style="margin:0;color:rgba(218,198,168,0.5);font-size:0.82rem;">暂无可用升级。</p>
                </div>
              )}
            </div>
          </Show>

          <Show when={activeTab() === "usage"}>
            {/* Usage tab — treatment slots */}
            <div class="sanitarium-usage-panel" data-testid="sanitarium-usage-panel">
              {/* Quirk Treatment Slot */}
              <Show when={quirkSlot()}>
                {(slot) => (
                  <div class="sanitarium-slot-card" data-testid={`slot-${slot().id}`}>
                    <div class="sanitarium-slot-header">
                      <span class="sanitarium-slot-icon">🧠</span>
                      <div class="sanitarium-slot-title-area">
                        <span class="sanitarium-slot-title">{slot().label}</span>
                        <span class="sanitarium-slot-desc">{slot().description}</span>
                      </div>
                    </div>
                    <div class="sanitarium-slot-grid">
                      {/* Occupied / empty slot indicators */}
                      <For each={Array.from({ length: slot().capacity })}>
                        {(_, idx) => (
                          <button
                            class={`sanitarium-slot-cell ${idx() < slot().occupied ? "sanitarium-slot-cell--occupied" : "sanitarium-slot-cell--empty"}`}
                            onClick={() => openSlotModal("quirk")}
                            data-testid={`slot-quirk-cell-${idx()}`}
                          >
                            {idx() < slot().occupied ? (
                              <span class="sanitarium-slot-hero">👤</span>
                            ) : (
                              <span class="sanitarium-slot-placeholder">+</span>
                            )}
                          </button>
                        )}
                      </For>
                    </div>
                  </div>
                )}
              </Show>

              {/* Disease Treatment Slot */}
              <Show when={diseaseSlot()}>
                {(slot) => (
                  <div class="sanitarium-slot-card" data-testid={`slot-${slot().id}`}>
                    <div class="sanitarium-slot-header">
                      <span class="sanitarium-slot-icon">💊</span>
                      <div class="sanitarium-slot-title-area">
                        <span class="sanitarium-slot-title">{slot().label}</span>
                        <span class="sanitarium-slot-desc">{slot().description}</span>
                      </div>
                    </div>
                    <div class="sanitarium-slot-grid">
                      <For each={Array.from({ length: slot().capacity })}>
                        {(_, idx) => (
                          <button
                            class={`sanitarium-slot-cell ${idx() < slot().occupied ? "sanitarium-slot-cell--occupied" : "sanitarium-slot-cell--empty"}`}
                            onClick={() => openSlotModal("disease")}
                            data-testid={`slot-disease-cell-${idx()}`}
                          >
                            {idx() < slot().occupied ? (
                              <span class="sanitarium-slot-hero">👤</span>
                            ) : (
                              <span class="sanitarium-slot-placeholder">+</span>
                            )}
                          </button>
                        )}
                      </For>
                    </div>
                  </div>
                )}
              </Show>

              {/* Fallback when no slots defined yet */}
              {slots().length === 0 && (
                <div class="building-info-card">
                  <p style="margin:0;color:rgba(218,198,168,0.5);font-size:0.82rem;">
                    槽位数据尚未加载。
                  </p>
                </div>
              )}
            </div>
          </Show>
        </div>
      </div>

      {/* ── Return to Town ── */}
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

      {/* ── Modal / Overlay ── */}
      <Show when={modal().kind !== "closed"}>
        <div class="sanitarium-modal-overlay" onClick={closeModal} data-testid="sanitarium-modal">
          <div class="sanitarium-modal" onClick={(e) => e.stopPropagation()}>
            {/* Close button */}
            <button class="sanitarium-modal-close" onClick={closeModal} data-testid="modal-close">
              ✕
            </button>

            {/* Select Hero modal (both quirk and disease) */}
            <Show when={modal().kind === "select-hero"}>
              {(m) => (
                <div class="sanitarium-modal-body">
                  <h3 class="sanitarium-modal-title">
                    {m().slotType === "quirk" ? "选择人物" : "选择要治疗的神降"}
                  </h3>

                  {/* Hero list */}
                  <div class="sanitarium-hero-list">
                    <For each={fixtureHeroes}>
                      {(hero) => (
                        <button
                          class={`sanitarium-hero-row ${selectedHeroId() === hero.id ? "sanitarium-hero-row--selected" : ""}`}
                          onClick={() => setSelectedHeroId(hero.id)}
                          data-testid={`hero-row-${hero.id}`}
                        >
                          <div class="sanitarium-hero-avatar">🧑</div>
                          <div class="sanitarium-hero-info">
                            <span class="sanitarium-hero-name">
                              {hero.name}
                              <span class="sanitarium-hero-class">{hero.classLabel}</span>
                            </span>
                            <div class="sanitarium-hero-vitals">
                              <span>HP {hero.hp}</span>
                              <span>压力 {hero.stress}</span>
                            </div>
                            <Show when={hero.positiveQuirks.length > 0 || hero.negativeQuirks.length > 0}>
                              <div class="sanitarium-hero-quirks">
                                <For each={hero.positiveQuirks}>
                                  {(q) => <span class="sanitarium-quirk sanitarium-quirk--positive">{q}</span>}
                                </For>
                                <For each={hero.negativeQuirks}>
                                  {(q) => <span class="sanitarium-quirk sanitarium-quirk--negative">{q}</span>}
                                </For>
                              </div>
                            </Show>
                          </div>
                        </button>
                      )}
                    </For>
                  </div>

                  {/* Disease detail preview (select-2 specific) */}
                  {(() => {
                    const currentModal = modal();
                    if (currentModal.kind !== "select-hero" || currentModal.slotType !== "disease" || !selectedHeroId()) return null;
                    const hero = fixtureHeroes.find((h) => h.id === selectedHeroId());
                    if (!hero) return null;
                    return (
                      <div class="sanitarium-disease-preview" data-testid="disease-preview">
                        {hero.diseases.length > 0 && (
                          <>
                            <div class="sanitarium-disease-row">
                              <span class="sanitarium-disease-label">异常状态</span>
                              <span class="sanitarium-disease-value">{hero.diseases[0]}</span>
                            </div>
                            <div class="sanitarium-disease-row">
                              <span class="sanitarium-disease-label">可治愈异常</span>
                              <span class="sanitarium-disease-value">{hero.diseases[0]}</span>
                            </div>
                          </>
                        )}
                        <div class="sanitarium-disease-cost">
                          费用: <strong>{diseaseAction()?.cost ?? "1000 Gold"}</strong>
                        </div>
                      </div>
                    );
                  })()}

                  {/* Confirm button */}
                  <div class="sanitarium-modal-actions">
                    <button
                      class="building-action-btn building-action-btn--primary"
                      disabled={!selectedHeroId()}
                      onClick={confirmHeroSelection}
                      data-testid="modal-confirm"
                    >
                      {m().slotType === "quirk" ? "确认" : "开始治疗"}
                    </button>
                  </div>
                </div>
              )}
            </Show>

            {/* Confirm Treatment modal (disease final confirmation) */}
            <Show when={modal().kind === "confirm-treatment"}>
              {(m) => (
                <div class="sanitarium-modal-body">
                  <h3 class="sanitarium-modal-title">确认治疗</h3>
                  {(() => {
                    const hero = fixtureHeroes.find((h) => h.id === m().heroId);
                    if (!hero) return null;
                    return (
                      <div class="sanitarium-confirm-body">
                        <div class="sanitarium-confirm-hero">
                          <span class="sanitarium-confirm-name">{hero.name}</span>
                          <Show when={hero.diseases.length > 0}>
                            <span class="sanitarium-confirm-disease">{hero.diseases[0]}</span>
                          </Show>
                        </div>
                        <div class="sanitarium-confirm-cost">
                          费用: <strong>{m().cost}</strong>
                        </div>
                        <div class="sanitarium-modal-actions">
                          <button class="building-action-btn" onClick={closeModal}>取消</button>
                          <button
                            class="building-action-btn building-action-btn--primary"
                            onClick={startTreatment}
                            data-testid="modal-start-treatment"
                          >
                            开始治疗
                          </button>
                        </div>
                      </div>
                    );
                  })()}
                </div>
              )}
            </Show>
          </div>
        </div>
      </Show>
    </div>
  );
};
