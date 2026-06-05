import { For, createSignal, type Component } from "solid-js";

import type { BuildingDetailViewModel, TownHeroSummary } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";
import { resolveHeroPortrait } from "../../../assets/originalAssetPaths";

interface GuildBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

/**
 * Guild (试炼场) building screen.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Guild/GuildWindow.prefab
 *
 * Original sprite: Assets/Sprites/town/buildings/building_train_field.png
 * GUID: 67a5e7aed8029d84dbf9c9e497a944d2
 *
 * Related prefabs:
 *   Assets/Prefabs/UI/SkillUpgradeSlot.prefab  — skill upgrade slots
 *   Assets/Prefabs/UI/UpgradeSlot.prefab       — equipment upgrade slots
 *   Assets/Prefabs/UI/Estate/Buildings/Guild/HeroSlot.prefab — hero selection slots
 *
 * Building data (data/Buildings.json):
 *   guild_training — skill training level upgrades
 *   guild_equipment — equipment tier upgrades
 *
 * Reference layout: 公会界面.png
 *   - Top: building header + resource bar (gold)
 *   - Left: hero roster / character slots
 *   - Right: skill training + equipment upgrade panels
 *   - Bottom: return navigation
 */
export const GuildBuildingScreen: Component<GuildBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;
  const [selectedHeroId, setSelectedHeroId] = createSignal<string | undefined>(
    vm().heroes?.[0]?.id
  );

  const selectedHero = (): TownHeroSummary | undefined =>
    vm().heroes?.find((h) => h.id === selectedHeroId());

  const trainingActions = () => vm().actions.filter((a) => a.id.startsWith("train-"));
  const equipmentActions = () => vm().actions.filter((a) => a.id.startsWith("upgrade-"));
  const otherActions = () =>
    vm().actions.filter((a) => !a.id.startsWith("train-") && !a.id.startsWith("upgrade-"));

  const gold = () => vm().gold ?? 0;

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

      {/* ── Resource Bar — mirrors GuildWindow/TopBar/CurrencyPanel ── */}
      <div
        class="guild-resource-bar"
        data-source-prefab="UI_Shared/UI_TopWindows/CurrencyPanel"
        data-source-layer="resource-bar"
      >
        <div class="guild-resource-slot guild-resource-slot-gold">
          <span class="guild-resource-icon" aria-hidden="true">◆</span>
          <span class="guild-resource-label">金币</span>
          <span class="guild-resource-value">{gold()}</span>
        </div>
        {vm().currentUpgrade && (
          <div class="guild-resource-slot">
            <span class="guild-resource-label">建筑等级</span>
            <span class="guild-resource-value guild-resource-value--level">{vm().currentUpgrade}</span>
          </div>
        )}
      </div>

      {/* ── Content — mirrors GuildWindow LeftPanel + RightPanel ── */}
      <div class="building-detail-content">
        {/* Left Panel — mirrors GuildWindow/LeftPanel */}
        <div
          class="building-detail-left"
          data-source-hierarchy="GuildWindow/LeftPanel"
        >
          {/* Hero Roster — mirrors GuildWindow/HeroSlot list */}
          {vm().heroes && vm().heroes.length > 0 && (
            <div
              class="building-info-card guild-hero-roster"
              data-source-prefab="Assets/Prefabs/UI/Estate/Buildings/Guild/HeroSlot.prefab"
              data-source-component="HeroSlot"
            >
              <h3 class="building-info-card-title">英雄名册</h3>
              <div class="guild-hero-list">
                <For each={vm().heroes}>
                  {(hero) => {
                    const isSelected = () => selectedHeroId() === hero.id;
                    const portrait = () => resolveHeroPortrait({ heroId: hero.id, classLabel: hero.classLabel });
                    return (
                      <button
                        class={`guild-hero-card ${isSelected() ? "guild-hero-card--selected" : ""}`}
                        onClick={() => setSelectedHeroId(hero.id)}
                        data-hero-id={hero.id}
                        data-testid={`guild-hero-${hero.id}`}
                        aria-pressed={isSelected()}
                      >
                        <div class="guild-hero-portrait">
                          {portrait() ? (
                            <img
                              class="guild-hero-portrait-img"
                              src={portrait()!}
                              alt={hero.name}
                              loading="lazy"
                            />
                          ) : (
                            <div class="guild-hero-portrait-fallback">
                              <span>{hero.name[0]?.toUpperCase() ?? "?"}</span>
                            </div>
                          )}
                          {hero.isWounded && (
                            <span class="guild-hero-status-badge guild-hero-status-badge--wounded">
                              伤
                            </span>
                          )}
                          {hero.isAfflicted && (
                            <span class="guild-hero-status-badge guild-hero-status-badge--afflicted">
                              畸
                            </span>
                          )}
                        </div>
                        <div class="guild-hero-info">
                          <span class="guild-hero-name">{hero.name}</span>
                          <span class="guild-hero-class">{hero.classLabel}</span>
                          <div class="guild-hero-bars">
                            <div class="guild-hero-bar-row">
                              <span class="guild-hero-bar-label">HP</span>
                              <div class="guild-hero-bar-track">
                                <div
                                  class="guild-hero-bar-fill guild-hero-bar-fill--hp"
                                  style={{ width: `${Math.round((hero.health / hero.maxHealth) * 100)}%` }}
                                />
                              </div>
                            </div>
                            <div class="guild-hero-bar-row">
                              <span class="guild-hero-bar-label">压力</span>
                              <div class="guild-hero-bar-track">
                                <div
                                  class={`guild-hero-bar-fill ${parseInt(hero.stress) > 100 ? "guild-hero-bar-fill--stress-high" : "guild-hero-bar-fill--stress"}`}
                                  style={{ width: `${Math.min(Math.round((parseInt(hero.stress) / parseInt(hero.maxStress)) * 100), 100)}%` }}
                                />
                              </div>
                            </div>
                          </div>
                          <span class="guild-hero-level">Lv.{hero.level}</span>
                        </div>
                      </button>
                    );
                  }}
                </For>
              </div>
            </div>
          )}

          {/* Building Info Card */}
          <div class="building-info-card">
            <h3 class="building-info-card-title">建筑状态</h3>
            <div class="building-info-row">
              <span class="building-info-label">状态</span>
              <span class="building-info-value">
                {vm().status === "ready" ? "运营中" : vm().status === "partial" ? "部分可用" : "锁定"}
              </span>
            </div>
            {vm().currentUpgrade && (
              <div class="building-info-row">
                <span class="building-info-label">试炼场等级</span>
                <span class="building-info-value">{vm().currentUpgrade}</span>
              </div>
            )}
            {vm().upgradeRequirement && (
              <div class="building-info-row">
                <span class="building-info-label">升级要求</span>
                <span class="building-info-value">{vm().upgradeRequirement}</span>
              </div>
            )}
          </div>

          {/* Selected Hero Detail — mirrors GuildWindow/SelectedHeroPanel */}
          {selectedHero() && (
            <div
              class="building-info-card guild-selected-hero-detail"
              data-source-hierarchy="GuildWindow/SelectedHeroPanel"
            >
              <h3 class="building-info-card-title">选中英雄</h3>
              <div class="guild-selected-hero-header">
                <span class="guild-selected-hero-name">{selectedHero()!.name}</span>
                <span class="guild-selected-hero-class">{selectedHero()!.classLabel}</span>
              </div>
              <div class="guild-selected-hero-stats">
                <div class="building-info-row">
                  <span class="building-info-label">生命值</span>
                  <span class="building-info-value">{selectedHero()!.hp}</span>
                </div>
                <div class="building-info-row">
                  <span class="building-info-label">压力</span>
                  <span class="building-info-value">{selectedHero()!.stress} / {selectedHero()!.maxStress}</span>
                </div>
                <div class="building-info-row">
                  <span class="building-info-label">等级</span>
                  <span class="building-info-value">{selectedHero()!.level}</span>
                </div>
              </div>
              {(selectedHero()!.positiveQuirks.length > 0 || selectedHero()!.negativeQuirks.length > 0) && (
                <div class="guild-selected-hero-quirks">
                  <For each={selectedHero()!.positiveQuirks}>
                    {(quirk) => (
                      <span class="guild-hero-quirk guild-hero-quirk--positive">{quirk}</span>
                    )}
                  </For>
                  <For each={selectedHero()!.negativeQuirks}>
                    {(quirk) => (
                      <span class="guild-hero-quirk guild-hero-quirk--negative">{quirk}</span>
                    )}
                  </For>
                </div>
              )}
              {selectedHero()!.diseases.length > 0 && (
                <div class="guild-selected-hero-diseases">
                  <span class="building-info-label">疾病</span>
                  <div class="guild-disease-list">
                    <For each={selectedHero()!.diseases}>
                      {(disease) => (
                        <span class="guild-disease-chip">{disease}</span>
                      )}
                    </For>
                  </div>
                </div>
              )}
            </div>
          )}
        </div>

        {/* Right Panel / Actions — mirrors GuildWindow/RightPanel/UpgradeWindow */}
        <div
          class="building-detail-right"
          data-source-hierarchy="GuildWindow/RightPanel/UpgradeWindow"
        >
          {/* Skill Training — mirrors SkillUpgradeSlot */}
          {trainingActions().length > 0 && (
            <div class="building-action-section">
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
                          尚未支持
                        </span>
                      )}
                      {!action.isAvailable && !action.isUnsupported && (
                        <span class="building-action-pill building-action-pill--unavailable">
                          不可用
                        </span>
                      )}
                    </div>
                    <p class="building-action-desc">{action.description}</p>
                    <div class="building-action-footer">
                      <span class={`building-action-cost ${!action.isAvailable ? "building-action-cost-unavailable" : ""}`}>
                        费用: <strong>{action.cost}</strong>
                      </span>
                      {action.isUnsupported ? (
                        <button class="building-action-btn building-action-btn--disabled" disabled>
                          暂不可用
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
                          前置条件未满足
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
            <div class="building-action-section">
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
                          尚未支持
                        </span>
                      )}
                      {!action.isAvailable && !action.isUnsupported && (
                        <span class="building-action-pill building-action-pill--unavailable">
                          不可用
                        </span>
                      )}
                    </div>
                    <p class="building-action-desc">{action.description}</p>
                    <div class="building-action-footer">
                      <span class={`building-action-cost ${!action.isAvailable ? "building-action-cost-unavailable" : ""}`}>
                        费用: <strong>{action.cost}</strong>
                      </span>
                      {action.isUnsupported ? (
                        <button class="building-action-btn building-action-btn--disabled" disabled>
                          暂不可用
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
                            前置条件未满足
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
                          尚未支持
                        </span>
                      )}
                      {!action.isAvailable && !action.isUnsupported && (
                        <span class="building-action-pill building-action-pill--unavailable">
                          不可用
                        </span>
                      )}
                    </div>
                    <p class="building-action-desc">{action.description}</p>
                    <div class="building-action-footer">
                      <span class={`building-action-cost ${!action.isAvailable ? "building-action-cost-unavailable" : ""}`}>
                        费用: <strong>{action.cost}</strong>
                      </span>
                      {action.isUnsupported ? (
                        <button class="building-action-btn building-action-btn--disabled" disabled>
                          暂不可用
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
                          前置条件未满足
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
      <div class="building-return-row">
        <button
          class="building-return-btn"
          onClick={props.onReturn}
          data-source-component="CloseButton"
          data-source-sprite="Assets/Sprites/ui/btn_close.png"
        >
          返回城镇
        </button>
      </div>
    </div>
  );
};
