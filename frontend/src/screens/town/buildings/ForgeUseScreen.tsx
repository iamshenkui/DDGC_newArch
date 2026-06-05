import { For, type Component } from "solid-js";

import type { ForgeUseViewModel } from "../../../bridge/contractTypes";
import { resolveBuildingImage, resolveHeroPortrait } from "../../../assets/originalAssetPaths";
import { BuildingDetailHeader } from "./BuildingDetailHeader";

interface ForgeUseScreenProps {
  viewModel: ForgeUseViewModel;
  onReturn: () => void;
  onSwitchToUpgrade: () => void;
  onTalk: () => void;
  onUnequipWeapon: (heroId: string) => void;
  onUnequipArmor: (heroId: string) => void;
}

/**
 * Forge Use Facility screen (锻造舱 - 使用设施).
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Blacksmith/BlacksmithWindow.prefab
 *
 * Reference image: 公会界面-锻造仓-使用.png
 *
 * Layout:
 *   - Header with building icon + title
 *   - Two tabs: 升级设施 (Upgrade) | 使用设施 (Use) — Use is active
 *   - Left Panel: NPC portrait + 对话 (Talk) / 离开 (Leave) buttons
 *   - Right Panel: 角色 (Characters) list with hero equipment slots
 *   - Bottom: NPC description card
 */
export const ForgeUseScreen: Component<ForgeUseScreenProps> = (props) => {
  const vm = () => props.viewModel;

  return (
    <div class="app-frame" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Building Header ── */}
      <BuildingDetailHeader
        buildingId="blacksmith"
        label={vm().label}
        status={vm().status}
        description={vm().description}
        sourcePrefabPath="Assets/Prefabs/UI/Estate/Buildings/Blacksmith/BlacksmithWindow.prefab"
        sourceSpritePath="Assets/Sprites/town/buildings/building_forging.png"
        sourceGuid="23e01c10f262ddc4ba9977b91314b031"
      />

      {/* ── Tab Navigation ── */}
      <div class="forge-use-tabs" data-source-component="TabGroup">
        <button
          class="forge-use-tab"
          onClick={props.onSwitchToUpgrade}
          data-tab-id="upgrade"
          data-source-component="UpgradeTab"
        >
          <span class="forge-use-tab-checkbox" />
          <span class="forge-use-tab-label">升级设施</span>
        </button>
        <button
          class="forge-use-tab forge-use-tab--active"
          data-tab-id="use"
          data-source-component="UseTab"
        >
          <span class="forge-use-tab-checkbox forge-use-tab-checkbox--checked">☑</span>
          <span class="forge-use-tab-label">使用设施</span>
        </button>
      </div>

      {/* ── Main Content ── */}
      <div class="forge-use-content">
        {/* Left Panel — NPC Portrait + Actions */}
        <div class="forge-use-left" data-source-hierarchy="BlacksmithWindow/LeftPanel">
          <div class="forge-use-npc-card">
            <div class="forge-use-npc-portrait">
              {vm().npcPortrait ? (
                <img
                  class="forge-use-npc-img"
                  src={vm().npcPortrait}
                  alt={vm().npcName}
                  loading="eager"
                />
              ) : (
                <div class="forge-use-npc-fallback">
                  <span>{vm().npcName[0]?.toUpperCase() ?? "?"}</span>
                </div>
              )}
            </div>
            <div class="forge-use-npc-actions">
              <button
                class="forge-use-npc-btn forge-use-npc-btn--talk"
                onClick={props.onTalk}
                data-source-component="TalkButton"
              >
                对话
              </button>
              <button
                class="forge-use-npc-btn forge-use-npc-btn--leave"
                onClick={props.onReturn}
                data-source-component="LeaveButton"
              >
                离开
              </button>
            </div>
          </div>
        </div>

        {/* Right Panel — Hero Equipment List */}
        <div class="forge-use-right" data-source-hierarchy="BlacksmithWindow/RightPanel">
          <h3 class="forge-use-section-title">角色</h3>
          <div class="forge-use-hero-list">
            <For each={vm().heroes}>
              {(hero) => {
                const portraitSrc = () =>
                  resolveHeroPortrait({ heroId: hero.id, classLabel: hero.classLabel });
                return (
                  <div class="forge-use-hero-row" data-hero-id={hero.id}>
                    {/* Hero Portrait */}
                    <div class="forge-use-hero-portrait">
                      {portraitSrc() ? (
                        <img
                          class="forge-use-hero-portrait-img"
                          src={portraitSrc()}
                          alt={hero.name}
                          loading="lazy"
                        />
                      ) : (
                        <div class="forge-use-hero-portrait-fallback">
                          <span>{hero.name[0]?.toUpperCase() ?? "?"}</span>
                        </div>
                      )}
                    </div>

                    {/* Hero Info */}
                    <div class="forge-use-hero-info">
                      <span class="forge-use-hero-name">{hero.name}</span>
                      <span class="forge-use-hero-class">{hero.classLabel}</span>
                    </div>

                    {/* Equipment Slots */}
                    <div class="forge-use-equipment-slots">
                      {/* Weapon Slot */}
                      <div class="forge-use-slot">
                        <span class="forge-use-slot-label">武器</span>
                        <div class="forge-use-slot-item">
                          <span class="forge-use-slot-item-name">{hero.weapon.name}</span>
                          <span class="forge-use-slot-item-level">Lv.{hero.weapon.level}</span>
                        </div>
                        <button
                          class="forge-use-slot-remove"
                          onClick={() => props.onUnequipWeapon(hero.id)}
                          title="卸下武器"
                          aria-label={`卸下 ${hero.name} 的武器`}
                        >
                          ✕
                        </button>
                      </div>

                      {/* Armor Slot */}
                      <div class="forge-use-slot">
                        <span class="forge-use-slot-label">护甲</span>
                        <div class="forge-use-slot-item">
                          <span class="forge-use-slot-item-name">{hero.armor.name}</span>
                          <span class="forge-use-slot-item-level">Lv.{hero.armor.level}</span>
                        </div>
                        <button
                          class="forge-use-slot-remove"
                          onClick={() => props.onUnequipArmor(hero.id)}
                          title="卸下护甲"
                          aria-label={`卸下 ${hero.name} 的护甲`}
                        >
                          ✕
                        </button>
                      </div>
                    </div>
                  </div>
                );
              }}
            </For>
          </div>
        </div>
      </div>

      {/* ── NPC Description Card ── */}
      <div class="forge-use-npc-desc-card">
        <h4 class="forge-use-npc-desc-name">{vm().npcName}</h4>
        <p class="forge-use-npc-desc-text">{vm().npcDescription}</p>
      </div>

      {/* ── Bottom Resource Bar ── */}
      <div class="forge-use-resource-bar">
        <span class="forge-use-resource-item">
          <span class="forge-use-resource-icon">💰</span>
          <span class="forge-use-resource-value">{vm().gold}</span>
        </span>
      </div>
    </div>
  );
};
