import { For, createSignal, type Component } from "solid-js";

import type { BuildingDetailViewModel } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface GuildBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

/**
 * Guild (试炼场) building screen — Space Analysis Selection Interface.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Guild/GuildWindow.prefab
 *   → UseWindow [GuildHeroWindow]
 *     → HeroSlot [HeroObserverSlot] + Frame
 *     → HeroInfo [Image] + HeroClassLabel + HeroDescription
 *     → SkillScroller [Image] + Content [LayoutGroup] + SkillTree [MonoBehaviour]
 *     → HeroName [Text]
 *
 * Original sprite: Assets/Sprites/town/buildings/building_train_field.png
 * GUID: 67a5e7aed8029d84dbf9c9e497a944d2
 *
 * Related prefabs:
 *   Assets/Prefabs/UI/SkillUpgradeSlot.prefab  — skill upgrade slots
 *   Assets/Prefabs/UI/UpgradeSlot.prefab       — equipment upgrade slots
 */
export const GuildBuildingScreen: Component<GuildBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;
  const heroes = () => vm().heroes ?? [];
  const campingSkills = () => vm().campingSkills ?? [];

  const [selectedHeroId, setSelectedHeroId] = createSignal<string | null>(
    heroes()[0]?.id ?? null
  );

  const selectedHero = () =>
    heroes().find((h) => h.id === selectedHeroId()) ?? null;

  const trainingActions = () => vm().actions.filter((a) => a.id.startsWith("train-"));
  const equipmentActions = () => vm().actions.filter((a) => a.id.startsWith("upgrade-"));
  const otherActions = () =>
    vm().actions.filter((a) => !a.id.startsWith("train-") && !a.id.startsWith("upgrade-"));

  return (
    <div class="app-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Building Header — mirrors GuildWindow/LeftPanel/Icon + Title ── */}
      <BuildingDetailHeader
        buildingId="guild"
        label={vm().label}
        status={vm().status}
        description={vm().description}
        sourcePrefabPath="Assets/Prefabs/UI/Estate/Buildings/Guild/GuildWindow.prefab"
        sourceSpritePath="Assets/Sprites/town/buildings/building_train_field.png"
        sourceGuid="67a5e7aed8029d84dbf9c9e497a944d2"
      />

      {/* ── Content — mirrors GuildWindow LeftPanel + RightPanel ── */}
      <div class="building-detail-content">
        {/* Left Panel — mirrors GuildWindow/LeftPanel + GuildHeroWindow/HeroSlot + HeroInfo */}
        <div
          class="building-detail-left"
          data-source-hierarchy="GuildWindow/LeftPanel"
        >
          {/* Hero Selection — mirrors GuildHeroWindow/HeroSlot */}
          <div class="building-info-card guild-hero-selection"
            data-source-prefab="Assets/Prefabs/UI/Estate/Buildings/Guild/GuildWindow.prefab"
            data-source-component="HeroObserverSlot"
          >
            <h3 class="building-info-card-title">选择英雄</h3>
            {heroes().length > 0 ? (
              <div class="guild-hero-slots"
                data-source-hierarchy="GuildHeroWindow/HeroSlot"
              >
                <For each={heroes()}>
                  {(hero) => (
                    <button
                      class={`guild-hero-slot ${selectedHeroId() === hero.id ? "guild-hero-slot--selected" : ""}`}
                      onClick={() => setSelectedHeroId(hero.id)}
                      data-hero-id={hero.id}
                      data-source-component="HeroObserverSlot"
                    >
                      <span class="guild-hero-slot-frame" aria-hidden="true"
                        data-source-sprite="Assets/Sprites/ui/char_slot.png"
                      >
                        <span class="guild-hero-slot-initial">{hero.name[0]?.toUpperCase() ?? "?"}</span>
                      </span>
                      <span class="guild-hero-slot-name">{hero.name}</span>
                    </button>
                  )}
                </For>
              </div>
            ) : (
              <p class="guild-hero-empty">暂无可用英雄</p>
            )}
          </div>

          {/* Hero Info — mirrors GuildHeroWindow/HeroInfo */}
          {selectedHero() && (
            <div class="building-info-card guild-hero-info"
              data-source-hierarchy="GuildHeroWindow/HeroInfo"
              data-source-sprite="Assets/Sprites/ui/char_desc_frame.png"
            >
              <div class="guild-hero-info-header"
                data-source-component="HeroName"
              >
                <span class="guild-hero-info-name">{selectedHero()?.name}</span>
                <span class="guild-hero-info-class"
                  data-source-component="HeroClassLabel"
                >
                  {selectedHero()?.classLabel}
                </span>
              </div>
              <p class="guild-hero-info-desc"
                data-source-component="HeroDescription"
              >
                {selectedHero()?.isWounded
                  ? `${selectedHero()?.name} 当前受伤，需要休息恢复。`
                  : `${selectedHero()?.name} 状态良好，可以进行技能训练。`}
              </p>
              <div class="guild-hero-info-vitals"
                data-source-hierarchy="GuildHeroWindow/HeroInfo"
              >
                <div class="guild-hero-info-vital"
                  data-source-sprite="hp_icon.png"
                >
                  <span class="guild-hero-info-vital-label">生命值</span>
                  <span class="guild-hero-info-vital-value">{selectedHero()?.hp} / {selectedHero()?.maxHp}</span>
                </div>
                <div class="guild-hero-info-vital"
                  data-source-sprite="stress_icon.png"
                >
                  <span class="guild-hero-info-vital-label">压力值</span>
                  <span class="guild-hero-info-vital-value">{selectedHero()?.stress} / {selectedHero()?.maxStress}</span>
                </div>
                <div class="guild-hero-info-vital"
                >
                  <span class="guild-hero-info-vital-label">等级</span>
                  <span class="guild-hero-info-vital-value">Lv.{selectedHero()?.level}</span>
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Right Panel / Skills — mirrors GuildHeroWindow/SkillScroller + SkillTree */}
        <div
          class="building-detail-right"
          data-source-hierarchy="GuildWindow/RightPanel/SkillScroller"
        >
          {/* Camping Skills Section — mirrors SkillTree [MonoBehaviour] entries */}
          <div class="building-action-section"
            data-source-prefab="Assets/Prefabs/UI/SkillUpgradeSlot.prefab"
            data-source-component="SkillUpgradeSlot"
          >
            <h3 class="building-action-section-title"
              data-source-component="UseTitle"
            >
              空间分析技能
            </h3>
            {campingSkills().length > 0 ? (
              <div class="guild-skill-grid"
                data-source-hierarchy="GuildHeroWindow/SkillScroller/Content/LayoutGroup"
              >
                <For each={campingSkills()}>
                  {(skill, index) => (
                    <div
                      class="guild-skill-tree"
                      data-source-prefab="Assets/Prefabs/UI/SkillUpgradeSlot.prefab"
                      data-source-component="SkillTree"
                      data-source-sprite="Assets/Sprites/ui/skill_slot01.png"
                    >
                      <div class="guild-skill-tree-connector" aria-hidden="true"
                        data-source-sprite="Assets/Sprites/ui/line_short.png"
                      >
                        {index() > 0 && (
                          <span class="guild-skill-connector-line"></span>
                        )}
                      </div>
                      <div class="guild-skill-slot"
                        data-source-component="SkillPurchaseSlot"
                      >
                        <span class="guild-skill-slot-frame" aria-hidden="true"
                          data-source-sprite="Assets/Sprites/ui/skill_slot01.png"
                        >
                          <span class="guild-skill-slot-initial">{skill.name[0]?.toUpperCase() ?? "?"}</span>
                        </span>
                        <span class="guild-skill-slot-name"
                          data-source-component="Level"
                        >
                          {skill.name}
                        </span>
                        <span class={`guild-skill-slot-locker ${skill.level <= 1 ? "guild-skill-slot-locker--unlocked" : ""}`}
                          aria-hidden="true"
                          data-source-sprite="Assets/Sprites/ui/skill.locked.png"
                        >
                          {skill.level <= 1 ? "✓" : "🔒"}
                        </span>
                      </div>
                      <span class="guild-skill-desc">{skill.description}</span>
                    </div>
                  )}
                </For>
              </div>
            ) : (
              <div class="building-info-card"
                data-source-component="SkillUpgradeSlot"
              >
                <p style="margin:0;color:rgba(218,198,168,0.5);font-size:0.82rem;"
                  data-source-hierarchy="GuildHeroWindow/SkillScroller"
                >
                  暂无可用空间分析技能。
                </p>
              </div>
            )}
          </div>

          {/* Training Actions — mirrors SkillUpgradeSlot / EquipmentUpgradeSlot */}
          {trainingActions().length > 0 && (
            <div class="building-action-section"
              data-source-prefab="Assets/Prefabs/UI/SkillUpgradeSlot.prefab"
            >
              <h3 class="building-action-section-title">技能训练</h3>
              <For each={trainingActions()}>
                {(action) => (
                  <div
                    class="building-action-card"
                    data-source-prefab="Assets/Prefabs/UI/SkillUpgradeSlot.prefab"
                    data-source-component="SkillUpgradeSlot"
                  >
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

          {/* Equipment Upgrades — mirrors UpgradeSlot / EquipmentUpgradeSlot */}
          {equipmentActions().length > 0 && (
            <div class="building-action-section"
              data-source-prefab="Assets/Prefabs/UI/UpgradeSlot.prefab"
            >
              <h3 class="building-action-section-title">装备升级</h3>
              <For each={equipmentActions()}>
                {(action) => (
                  <div
                    class="building-action-card"
                    data-source-prefab="Assets/Prefabs/UI/UpgradeSlot.prefab"
                    data-source-component="UpgradeSlot"
                  >
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
                        <div style="display:flex;gap:0.5rem;align-items:center;">
                          <button class="building-action-btn building-action-btn--disabled" disabled>
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
          )}

          {/* Other uncategorised actions */}
          {otherActions().length > 0 && (
            <div class="building-action-section">
              <h3 class="building-action-section-title">其他服务</h3>
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
      </div>

      {/* ── Return to Town — mirrors GuildWindow/CloseButton ── */}
      <div class="building-return-row"
        data-source-hierarchy="GuildWindow/CloseButton"
      >
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
