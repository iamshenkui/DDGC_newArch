import { createSignal, For, type Component } from "solid-js";

import { resolveHeroPortrait } from "../../../assets/originalAssetPaths";
import type { BuildingDetailViewModel, TownHeroSummary } from "../../../bridge/contractTypes";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface GuildBuildingScreenProps {
  viewModel: BuildingDetailViewModel;
  onReturn: () => void;
  onAction: (actionId: string) => void;
}

/**
 * Guild (试炼场) building screen — Hero Archive / Training Hall.
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
 *
 * Building data (data/Buildings.json):
 *   guild_training — skill training level upgrades
 *   guild_equipment — equipment tier upgrades
 */
export const GuildBuildingScreen: Component<GuildBuildingScreenProps> = (props) => {
  const vm = () => props.viewModel;
  const heroes = () => vm().heroes ?? [];

  const [selectedHeroId, setSelectedHeroId] = createSignal<string | undefined>(
    heroes()[0]?.id
  );

  const selectedHero = () =>
    heroes().find((h) => h.id === selectedHeroId());

  const trainingActions = () => vm().actions.filter((a) => a.id.startsWith("train-"));
  const equipmentActions = () => vm().actions.filter((a) => a.id.startsWith("upgrade-"));
  const otherActions = () =>
    vm().actions.filter((a) => !a.id.startsWith("train-") && !a.id.startsWith("upgrade-"));

  function heroPortraitSrc(hero: TownHeroSummary): string | undefined {
    return resolveHeroPortrait({ heroId: hero.id, classLabel: hero.classLabel });
  }

  function hpPercent(hero: TownHeroSummary): number {
    return Math.max(0, Math.min(100, (hero.health / Math.max(1, hero.maxHealth)) * 100));
  }

  function stressPercent(hero: TownHeroSummary): number {
    const max = Number(hero.maxStress) || 200;
    const current = Number(hero.stress) || 0;
    return Math.max(0, Math.min(100, (current / max) * 100));
  }

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

      {/* ── Hero Roster Strip — mirrors GuildWindow/HeroRoster ── */}
      {heroes().length > 0 && (
        <div
          class="guild-hero-roster"
          data-source-component="GuildHeroRoster"
          data-source-hierarchy="GuildWindow/HeroRoster"
        >
          <div class="guild-roster-header">
            <span class="guild-roster-title">英雄名册</span>
            <span class="guild-roster-count">({heroes().length})</span>
          </div>
          <div class="guild-roster-scroll">
            <For each={heroes()}>
              {(hero) => {
                const isSelected = () => selectedHeroId() === hero.id;
                const portrait = () => heroPortraitSrc(hero);
                return (
                  <button
                    class={`guild-roster-card ${isSelected() ? "guild-roster-card--selected" : ""}`}
                    onClick={() => setSelectedHeroId(hero.id)}
                    data-hero-id={hero.id}
                    data-source-component="GuildHeroSlot"
                  >
                    <div class="guild-roster-portrait">
                      {portrait() ? (
                        <img
                          class="guild-roster-portrait-img"
                          src={portrait()}
                          alt={hero.name}
                          loading="lazy"
                        />
                      ) : (
                        <span class="guild-roster-portrait-letter">
                          {hero.classLabel[0]}
                        </span>
                      )}
                      <span class="guild-roster-level">Lv{hero.level}</span>
                      {hero.isWounded && (
                        <span class="guild-roster-status guild-roster-status--wounded">W</span>
                      )}
                      {hero.isAfflicted && (
                        <span class="guild-roster-status guild-roster-status--afflicted">A</span>
                      )}
                    </div>
                    <div class="guild-roster-info">
                      <span class="guild-roster-name">{hero.name}</span>
                      <span class="guild-roster-class">{hero.classLabel}</span>
                    </div>
                    <div class="guild-roster-bars">
                      <div class="guild-roster-bar-row">
                        <span class="guild-roster-bar-label">HP</span>
                        <div class="guild-roster-bar-track">
                          <div
                            class="guild-roster-bar-fill guild-roster-bar-fill--hp"
                            style={{ width: `${hpPercent(hero)}%` }}
                          />
                        </div>
                      </div>
                      <div class="guild-roster-bar-row">
                        <span class="guild-roster-bar-label">ST</span>
                        <div class="guild-roster-bar-track">
                          <div
                            class="guild-roster-bar-fill guild-roster-bar-fill--stress"
                            style={{ width: `${stressPercent(hero)}%` }}
                          />
                        </div>
                      </div>
                    </div>
                  </button>
                );
              }}
            </For>
          </div>
        </div>
      )}

      {/* ── Selected Hero Detail + Actions ── */}
      {selectedHero() ? (
        <div class="guild-hero-detail">
          {/* Left Panel — Selected Hero Profile */}
          <div
            class="guild-hero-detail-left"
            data-source-hierarchy="GuildWindow/LeftPanel/HeroProfile"
          >
            <div class="guild-hero-profile-card">
              <div class="guild-hero-profile-portrait">
                {heroPortraitSrc(selectedHero()!) ? (
                  <img
                    class="guild-hero-profile-portrait-img"
                    src={heroPortraitSrc(selectedHero()!)}
                    alt={selectedHero()!.name}
                    loading="eager"
                  />
                ) : (
                  <span class="guild-hero-profile-portrait-letter">
                    {selectedHero()!.classLabel[0]}
                  </span>
                )}
              </div>

              <div class="guild-hero-profile-info">
                <h3 class="guild-hero-profile-name">{selectedHero()!.name}</h3>
                <span class="guild-hero-profile-class">{selectedHero()!.classLabel}</span>
                <span class="guild-hero-profile-level">Level {selectedHero()!.level}</span>
              </div>

              <div class="guild-hero-profile-stats">
                <div class="guild-stat-row">
                  <span class="guild-stat-label">HP</span>
                  <div class="guild-stat-track">
                    <div
                      class="guild-stat-fill guild-stat-fill--hp"
                      style={{ width: `${hpPercent(selectedHero()!)}%` }}
                    />
                  </div>
                  <span class="guild-stat-value">{selectedHero()!.hp}</span>
                </div>
                <div class="guild-stat-row">
                  <span class="guild-stat-label">Stress</span>
                  <div class="guild-stat-track">
                    <div
                      class="guild-stat-fill guild-stat-fill--stress"
                      style={{ width: `${stressPercent(selectedHero()!)}%` }}
                    />
                  </div>
                  <span class="guild-stat-value">{selectedHero()!.stress}/{selectedHero()!.maxStress}</span>
                </div>
              </div>

              <div class="guild-hero-profile-tags">
                {selectedHero()!.isWounded && (
                  <span class="guild-hero-tag guild-hero-tag--wounded">Wounded</span>
                )}
                {selectedHero()!.isAfflicted && (
                  <span class="guild-hero-tag guild-hero-tag--afflicted">Afflicted</span>
                )}
                {selectedHero()!.diseases.length > 0 && (
                  <span class="guild-hero-tag guild-hero-tag--disease">
                    Disease ({selectedHero()!.diseases.length})
                  </span>
                )}
              </div>

              <div class="guild-hero-profile-quirks">
                {selectedHero()!.positiveQuirks.length > 0 && (
                  <div class="guild-quirk-group">
                    <span class="guild-quirk-label">Positive</span>
                    <div class="guild-quirk-list">
                      <For each={selectedHero()!.positiveQuirks}>
                        {(quirk) => (
                          <span class="guild-quirk-chip guild-quirk-chip--positive">{quirk}</span>
                        )}
                      </For>
                    </div>
                  </div>
                )}
                {selectedHero()!.negativeQuirks.length > 0 && (
                  <div class="guild-quirk-group">
                    <span class="guild-quirk-label">Negative</span>
                    <div class="guild-quirk-list">
                      <For each={selectedHero()!.negativeQuirks}>
                        {(quirk) => (
                          <span class="guild-quirk-chip guild-quirk-chip--negative">{quirk}</span>
                        )}
                      </For>
                    </div>
                  </div>
                )}
              </div>
            </div>
          </div>

          {/* Right Panel — Training Actions */}
          <div
            class="guild-hero-detail-right"
            data-source-hierarchy="GuildWindow/RightPanel/UpgradeWindow"
          >
            <div class="guild-actions-header">
              <span class="guild-actions-title">训练与升级</span>
              <span class="guild-actions-subtitle">Training &amp; Upgrades</span>
            </div>

            {/* Skill Training — mirrors SkillUpgradeSlot */}
            {trainingActions().length > 0 && (
              <div class="building-action-section">
                <h3 class="building-action-section-title">Skill Training</h3>
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
              <div class="building-action-section">
                <h3 class="building-action-section-title">Equipment Upgrades</h3>
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
      ) : heroes().length === 0 ? (
        <div class="guild-no-heroes">
          <p>No heroes available in the roster.</p>
        </div>
      ) : null}

      {/* ── Return to Town — mirrors GuildWindow/CloseButton ── */}
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
