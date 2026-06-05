import { createSignal, For, Show, type Component } from "solid-js";

import { resolveHeroPortrait } from "../../assets/originalAssetPaths";
import type { HeroDetailViewModel, HeroRosterEntry } from "../../bridge/contractTypes";
import { AppFrame } from "../../components/layout/AppFrame";

type TabKey = "equipment" | "combat-skills" | "state" | "info" | "camping-skills";

interface HeroDetailScreenProps {
  viewModel: HeroDetailViewModel;
  onReturn: () => void;
  onPrevHero?: () => void;
  onNextHero?: () => void;
  onSelectHero?: (heroId: string) => void;
}

function stressPips(stress: number, maxStress: number): Array<"normal" | "stressed" | "overstressed"> {
  const ratio = stress / maxStress;
  const totalPips = 10;
  const pips: Array<"normal" | "stressed" | "overstressed"> = [];
  for (let i = 0; i < totalPips; i++) {
    const threshold = (i + 1) / totalPips;
    if (ratio >= threshold * 0.9) {
      pips.push(ratio > 0.7 ? "overstressed" : "stressed");
    } else {
      pips.push("normal");
    }
  }
  return pips;
}

function rankDots(level: number, max: number = 5): string {
  return "■".repeat(level) + "□".repeat(max - level);
}

const TAB_DEFS: Array<{ key: TabKey; labelCn: string; labelEn: string; component: string }> = [
  { key: "info", labelCn: "人物信息", labelEn: "Info", component: "InfoButton" },
  { key: "state", labelCn: "人物状态", labelEn: "State", component: "StateButton" },
  { key: "camping-skills", labelCn: "托管技能", labelEn: "Camping", component: "CampingSkillButton" },
  { key: "combat-skills", labelCn: "战斗技能", labelEn: "Combat", component: "CombatSkillButton" },
  { key: "equipment", labelCn: "装备", labelEn: "Equip", component: "EquipButton" },
];

export const HeroDetailScreen: Component<HeroDetailScreenProps> = (props) => {
  const [activeTab, setActiveTab] = createSignal<TabKey>("combat-skills");
  const [selectedSkillIndex, setSelectedSkillIndex] = createSignal(0);
  const maxStress = () => Number(props.viewModel.maxStress) || 200;
  const pips = () => stressPips(Number(props.viewModel.stress), maxStress());
  const portraitSrc = () =>
    resolveHeroPortrait({
      heroId: props.viewModel.heroId,
      classLabel: props.viewModel.classLabel
    });

  const roster = () => props.viewModel.roster ?? [];
  const resources = () => props.viewModel.resources;
  const selectedSkill = () => props.viewModel.combatSkills[selectedSkillIndex()] ?? props.viewModel.combatSkills[0];

  return (
    <AppFrame
      eyebrow="Hero Detail"
      title={`${props.viewModel.name} — ${props.viewModel.classLabel}`}
      subtitle={`Level ${props.viewModel.progression.level} · ${props.viewModel.resolveLabel} (Resolve ${props.viewModel.resolve})`}
    >
      {/* ── Full hero detail panel (mirrors CharacterWindow.prefab) ── */}
      <div
        class="hero-detail-layout"
        data-source-prefab="Assets/Prefabs/UI/Windows/CharacterWindow.prefab"
        data-source-component="CharacterWindow"
      >
        {/* ── Left sidebar: hero roster list (mirrors hero selection panel) ── */}
        <Show when={roster().length > 0}>
          <div
            class="hero-detail-sidebar"
            data-source-hierarchy="CharacterWindow/HeroListPanel"
          >
            <div class="hero-sidebar-list">
              <For each={roster()}>
                {(hero: HeroRosterEntry) => {
                  const heroPortrait = resolveHeroPortrait({ heroId: hero.id, classLabel: hero.classLabel });
                  return (
                    <button
                      class={`hero-sidebar-item ${hero.isSelected ? "hero-sidebar-item--active" : ""}`}
                      onClick={() => props.onSelectHero?.(hero.id)}
                      data-hero-id={hero.id}
                      data-source-component="HeroListItem"
                    >
                      <div class="hero-sidebar-portrait">
                        {heroPortrait ? (
                          <img src={heroPortrait} alt={hero.name} class="hero-sidebar-portrait-img" />
                        ) : (
                          <span class="hero-sidebar-portrait-letter">{hero.classLabel[0]}</span>
                        )}
                      </div>
                      {hero.isSelected && (
                        <span class="hero-sidebar-check" data-source-component="SelectedIndicator">✓</span>
                      )}
                    </button>
                  );
                }}
              </For>
            </div>
          </div>
        </Show>

        {/* ── Center column: portrait + hero info (mirrors ModelDisplayPanel + HeroPanel) ── */}
        <div
          class="hero-detail-center"
          data-source-hierarchy="CharacterWindow/StressPanel | CharacterWindow/HeroPanel"
        >
          {/* Navigation arrows */}
          <div class="hero-nav-arrows">
            <button
              class="hero-nav-arrow hero-nav-arrow--prev"
              onClick={props.onPrevHero}
              disabled={!props.onPrevHero}
              aria-label="Previous hero"
              data-source-component="PrevHeroButton"
            >
              ◀
            </button>
            <button
              class="hero-nav-arrow hero-nav-arrow--next"
              onClick={props.onNextHero}
              disabled={!props.onNextHero}
              aria-label="Next hero"
              data-source-component="NextHeroButton"
            >
              ▶
            </button>
          </div>

          {/* Portrait area — mirrors ModelDisplayPanel */}
          <div
            class="hero-portrait-area hero-portrait-area--large"
            data-source-component="ModelDisplayPanel"
            data-source-hierarchy="CharacterWindow/ModelDisplayPanel"
          >
            <div class="hero-portrait-frame hero-portrait-frame--large">
              {portraitSrc() ? (
                <img
                  class="hero-portrait-image hero-portrait-image--large"
                  src={portraitSrc()}
                  alt={`${props.viewModel.name} portrait`}
                  loading="eager"
                />
              ) : (
                <div class="hero-portrait-placeholder">
                  <span class="hero-portrait-initial">
                    {props.viewModel.classLabel[0]}
                  </span>
                </div>
              )}
            </div>
          </div>

          {/* Stress pips */}
          <div
            class="stress-pips-panel"
            data-source-component="StressPanel"
            data-source-prefab="Assets/Prefabs/UI/Windows/CharacterWindow.prefab"
          >
            <span class="stress-pips-label">Stress</span>
            <div class="stress-pips-row">
              <For each={pips()}>
                {(pip) => (
                  <span
                    class={`stress-pip stress-pip--${pip}`}
                    data-source-component="StressPip"
                    data-source-sprite="Assets/Sprites/ui/stress.{normal,stressed,overstressed}.png"
                    title={`${props.viewModel.stress} / ${props.viewModel.maxStress}`}
                  />
                )}
              </For>
            </div>
            <span class="stress-pips-value">
              {props.viewModel.stress}/{props.viewModel.maxStress}
            </span>
          </div>

          {/* Tags */}
          <div class="hero-tags">
            {props.viewModel.isWounded && (
              <span class="hero-tag hero-tag--wounded">Wounded</span>
            )}
            {props.viewModel.isAfflicted && (
              <span class="hero-tag hero-tag--afflicted">Afflicted</span>
            )}
          </div>

          {/* Hero info panel (mirrors HeroPanel: HeroName, HeroDesc, HeroTalent) */}
          <div class="hero-info-panel" data-source-component="HeroPanel">
            <h3 class="hero-info-name" data-source-component="HeroName">
              {props.viewModel.name}
            </h3>
            <p class="hero-info-desc" data-source-component="HeroDesc">
              {props.viewModel.heroDescription}
            </p>
            <div class="hero-talent-section" data-source-component="HeroTalent">
              <span class="hero-talent-label">天赋</span>
              <p class="hero-talent-text">{props.viewModel.talent}</p>
            </div>
            <button
              class="hero-exile-btn"
              data-source-component="ExileButton"
              onClick={() => { /* Exile action - requires backend intent */ }}
            >
              放逐
            </button>
          </div>
        </div>

        {/* ── Right area: vertical tabs + panels (mirrors CharacterWindow/Panels) ── */}
        <div
          class="hero-detail-right"
          data-source-component="Panels"
          data-source-hierarchy="CharacterWindow/Panels"
        >
          {/* Vertical tab bar — mirrors CharacterWindow button column */}
          <div class="hero-tab-bar hero-tab-bar--vertical">
            <For each={TAB_DEFS}>
              {(tab) => (
                <button
                  class={`hero-tab-btn hero-tab-btn--vertical ${activeTab() === tab.key ? "hero-tab-btn--active" : ""}`}
                  onClick={() => setActiveTab(tab.key)}
                  data-source-component={tab.component}
                >
                  <span class="hero-tab-label-cn">{tab.labelCn}</span>
                  <span class="hero-tab-label-en">{tab.labelEn}</span>
                </button>
              )}
            </For>
          </div>

          {/* ── Panel content ── */}
          <div class="hero-panel-wrapper">
            {/* ── Equipment Panel ── */}
            {activeTab() === "equipment" && (
              <div
                class="hero-panel hero-panel--equipment"
                data-source-component="EquipmentPanel"
                data-source-hierarchy="CharacterWindow/Panels/EquipmentPanel"
              >
                <div class="equipment-grid">
                  <div class="equipment-slot equipment-slot--weapon" data-source-component="WeaponSlot">
                    <span class="equipment-slot-label">Weapon</span>
                    <div class="equipment-slot-icon" data-source-sprite="Assets/Sprites/ui/item_slot.png">
                      <span class="equipment-slot-placeholder">⚔</span>
                    </div>
                    <span class="equipment-slot-name">{props.viewModel.weapon.name}</span>
                    <span class="equipment-slot-level">
                      {rankDots(props.viewModel.weapon.level)}
                    </span>
                  </div>

                  <div class="equipment-slot equipment-slot--armor" data-source-component="ArmorSlot">
                    <span class="equipment-slot-label">Armor</span>
                    <div class="equipment-slot-icon" data-source-sprite="Assets/Sprites/ui/item_slot.png">
                      <span class="equipment-slot-placeholder">🛡</span>
                    </div>
                    <span class="equipment-slot-name">{props.viewModel.armor.name}</span>
                    <span class="equipment-slot-level">
                      {rankDots(props.viewModel.armor.level)}
                    </span>
                  </div>

                  <div class="equipment-slot equipment-slot--trinket" data-source-component="LeftTrinketSlot">
                    <span class="equipment-slot-label">Trinket L</span>
                    <div class="equipment-slot-icon" data-source-sprite="Assets/Sprites/ui/item_slot.png">
                      <span class="equipment-slot-placeholder">💍</span>
                    </div>
                    <span class="equipment-slot-name">
                      {props.viewModel.leftTrinket?.name ?? "—"}
                    </span>
                  </div>

                  <div class="equipment-slot equipment-slot--trinket" data-source-component="RightTrinketSlot">
                    <span class="equipment-slot-label">Trinket R</span>
                    <div class="equipment-slot-icon" data-source-sprite="Assets/Sprites/ui/item_slot.png">
                      <span class="equipment-slot-placeholder">💍</span>
                    </div>
                    <span class="equipment-slot-name">
                      {props.viewModel.rightTrinket?.name ?? "—"}
                    </span>
                  </div>
                </div>

                <div class="equipment-core-stats">
                  <div class="core-stat-row">
                    <span class="core-stat-label">HP</span>
                    <div class="core-stat-track">
                      <div
                        class="core-stat-fill core-stat-fill--hp"
                        style={{
                          width: `${(Number(props.viewModel.hp) / Number(props.viewModel.maxHp || 1)) * 100}%`,
                        }}
                      />
                    </div>
                    <span class="core-stat-value">
                      {props.viewModel.hp}/{props.viewModel.maxHp}
                    </span>
                  </div>
                  <div class="core-stat-row">
                    <span class="core-stat-label">STR</span>
                    <div class="core-stat-track">
                      <div
                        class="core-stat-fill core-stat-fill--stress"
                        style={{
                          width: `${Math.min((Number(props.viewModel.stress) / maxStress()) * 100, 100)}%`,
                        }}
                      />
                    </div>
                    <span class="core-stat-value">
                      {props.viewModel.stress}/{props.viewModel.maxStress}
                    </span>
                  </div>
                </div>
              </div>
            )}

            {/* ── Combat Skills Panel (reference-matched layout) ── */}
            {activeTab() === "combat-skills" && (
              <div
                class="hero-panel hero-panel--skills hero-panel--combat-skills"
                data-source-component="CombatSkillsPanel"
                data-source-hierarchy="CharacterWindow/Panels/CombatSkillsPanel"
              >
                <div class="combat-skills-layout">
                  {/* Skill icon list */}
                  <div class="skill-icon-list" data-source-component="SkillIconList">
                    <For each={props.viewModel.combatSkills}>
                      {(skill, index) => (
                        <button
                          class={`skill-icon-btn ${selectedSkillIndex() === index() ? "skill-icon-btn--active" : ""} ${skill.isLocked ? "skill-icon-btn--locked" : ""}`}
                          onClick={() => setSelectedSkillIndex(index())}
                          data-source-component="SkillIcon"
                          data-skill-name={skill.name}
                          disabled={skill.isLocked}
                          title={skill.name}
                        >
                          <div class="skill-icon-box">
                            {skill.isLocked ? (
                              <span class="skill-icon-lock">🔒</span>
                            ) : (
                              <span class="skill-icon-letter">{skill.name[0]}</span>
                            )}
                          </div>
                          <Show when={selectedSkillIndex() === index()}>
                            <span class="skill-icon-active-bar" />
                          </Show>
                        </button>
                      )}
                    </For>
                  </div>

                  {/* Skill detail card */}
                  <div class="skill-detail-card" data-source-component="SkillDetailCard">
                    <Show when={selectedSkill()}>
                      {(skill) => (
                        <>
                          <h3 class="skill-detail-name" data-source-component="SkillName">
                            {skill().name}
                          </h3>
                          <Show when={skill().rankPips && skill().rankPips! > 0}>
                            <div class="skill-detail-rank" data-source-component="SkillRank">
                              <span class="skill-detail-rank-label">技能等级</span>
                              <div class="skill-rank-pips">
                                <For each={Array.from({ length: skill().rankPips || 0 })}>
                                  {() => <span class="skill-rank-pip" />}
                                </For>
                              </div>
                            </div>
                          </Show>
                          <Show when={skill().statBonuses && skill().statBonuses!.length > 0}>
                            <div class="skill-detail-stats" data-source-component="SkillStatBonuses">
                              <For each={skill().statBonuses}>
                                {(stat) => (
                                  <div class="skill-detail-stat-row">
                                    <span class="skill-detail-stat-label">{stat.label}</span>
                                    <span class="skill-detail-stat-value">{stat.value}</span>
                                  </div>
                                )}
                              </For>
                            </div>
                          </Show>
                          <Show when={skill().effectDescription}>
                            <div class="skill-detail-effect" data-source-component="SkillEffect">
                              <span class="skill-detail-effect-label">效果：</span>
                              <span class="skill-detail-effect-text">{skill().effectDescription}</span>
                            </div>
                          </Show>
                          <div class="skill-detail-base">
                            <span class="skill-detail-base-stat">命中: {skill().hitRating}</span>
                            <span class="skill-detail-base-stat">暴击: {skill().critRating}</span>
                            <span class="skill-detail-base-stat">目标: {skill().target}</span>
                          </div>
                          <p class="skill-detail-desc" data-source-component="SkillDesc">
                            {skill().description}
                          </p>
                        </>
                      )}
                    </Show>
                  </div>
                </div>
              </div>
            )}

            {/* ── State Panel ── */}
            {activeTab() === "state" && (
              <div
                class="hero-panel hero-panel--state"
                data-source-component="StatePanel"
              >
                <div class="info-section">
                  <h3 class="hero-panel-heading">Resistances</h3>
                  <div class="resistances-grid">
                    <div class="resistance-cell">
                      <span class="resistance-label">Stun</span>
                      <span class="resistance-value">{props.viewModel.resistances.stun}</span>
                    </div>
                    <div class="resistance-cell">
                      <span class="resistance-label">Bleed</span>
                      <span class="resistance-value">{props.viewModel.resistances.bleed}</span>
                    </div>
                    <div class="resistance-cell">
                      <span class="resistance-label">Disease</span>
                      <span class="resistance-value">{props.viewModel.resistances.disease}</span>
                    </div>
                    <div class="resistance-cell">
                      <span class="resistance-label">Move</span>
                      <span class="resistance-value">{props.viewModel.resistances.move}</span>
                    </div>
                    <div class="resistance-cell">
                      <span class="resistance-label">Death</span>
                      <span class="resistance-value">{props.viewModel.resistances.death}</span>
                    </div>
                    <div class="resistance-cell">
                      <span class="resistance-label">Trap</span>
                      <span class="resistance-value">{props.viewModel.resistances.trap}</span>
                    </div>
                    <div class="resistance-cell">
                      <span class="resistance-label">Hazard</span>
                      <span class="resistance-value">{props.viewModel.resistances.hazard}</span>
                    </div>
                  </div>
                </div>

                <div class="info-section" data-source-component="DiseasesPanel">
                  <h3 class="hero-panel-heading">
                    Diseases
                    <span class="heading-count">
                      ({props.viewModel.diseases.length})
                    </span>
                  </h3>
                  <div class="disease-list">
                    <For each={props.viewModel.diseases}>
                      {(disease) => (
                        <span class="disease-chip" data-source-component="DiseaseSlot">{disease}</span>
                      )}
                    </For>
                    {props.viewModel.diseases.length === 0 && (
                      <span class="quirk-empty">None</span>
                    )}
                  </div>
                </div>

                <div class="info-section">
                  <h3 class="hero-panel-heading">Progression</h3>
                  <div class="progression-stats">
                    <div class="progression-row">
                      <span class="progression-label">Level</span>
                      <span class="progression-value">
                        {props.viewModel.progression.level}
                      </span>
                    </div>
                    <div class="progression-row">
                      <span class="progression-label">Experience</span>
                      <span class="progression-value">
                        {props.viewModel.progression.experience} /{" "}
                        {props.viewModel.progression.experienceToNext}
                      </span>
                    </div>
                  </div>
                </div>
              </div>
            )}

            {/* ── Info Panel ── */}
            {activeTab() === "info" && (
              <div
                class="hero-panel hero-panel--info"
                data-source-component="InfoPanel"
                data-source-hierarchy="CharacterWindow/Panels/InfoPanel"
              >
                <div class="info-section" data-source-component="HeroDesc">
                  <h3 class="hero-panel-heading">Description</h3>
                  <p class="hero-desc-text">{props.viewModel.heroDescription}</p>
                </div>

                <div class="info-section" data-source-component="HeroTalent">
                  <h3 class="hero-panel-heading">Talent</h3>
                  <p class="hero-talent-text">{props.viewModel.talent}</p>
                </div>

                <div class="info-section" data-source-component="PositiveQuirks">
                  <h3 class="hero-panel-heading">
                    Positive Quirks
                    <span class="heading-count">
                      ({props.viewModel.positiveQuirks.length})
                    </span>
                  </h3>
                  <div class="quirk-list">
                    <For each={props.viewModel.positiveQuirks}>
                      {(quirk) => (
                        <span class="quirk-chip quirk-chip--positive" data-source-component="PositiveQuirkSlot">
                          {quirk}
                        </span>
                      )}
                    </For>
                    {props.viewModel.positiveQuirks.length === 0 && (
                      <span class="quirk-empty">None</span>
                    )}
                  </div>
                </div>

                <div class="info-section" data-source-component="NegativeQuirks">
                  <h3 class="hero-panel-heading">
                    Negative Quirks
                    <span class="heading-count">
                      ({props.viewModel.negativeQuirks.length})
                    </span>
                  </h3>
                  <div class="quirk-list">
                    <For each={props.viewModel.negativeQuirks}>
                      {(quirk) => (
                        <span class="quirk-chip quirk-chip--negative" data-source-component="NegativeQuirkSlot">
                          {quirk}
                        </span>
                      )}
                    </For>
                    {props.viewModel.negativeQuirks.length === 0 && (
                      <span class="quirk-empty">None</span>
                    )}
                  </div>
                </div>

                <div class="info-section" data-source-component="BaseStatsPanel">
                  <h3 class="hero-panel-heading">Base Stats</h3>
                  <div class="base-stats-grid">
                    <div class="base-stat-cell" data-source-component="DMGLabel">
                      <span class="base-stat-label">DMG</span>
                      <span class="base-stat-value">{props.viewModel.baseStats.dmg}</span>
                    </div>
                    <div class="base-stat-cell" data-source-component="MaxHPLabel">
                      <span class="base-stat-label">HP</span>
                      <span class="base-stat-value">{props.viewModel.baseStats.maxHp}</span>
                    </div>
                    <div class="base-stat-cell" data-source-component="CritLabel">
                      <span class="base-stat-label">CRIT</span>
                      <span class="base-stat-value">{props.viewModel.baseStats.crit}</span>
                    </div>
                    <div class="base-stat-cell" data-source-component="SPDLabel">
                      <span class="base-stat-label">SPD</span>
                      <span class="base-stat-value">{props.viewModel.baseStats.spd}</span>
                    </div>
                    <div class="base-stat-cell" data-source-component="DODGELabel">
                      <span class="base-stat-label">DODGE</span>
                      <span class="base-stat-value">{props.viewModel.baseStats.dodge}</span>
                    </div>
                  </div>
                </div>

                <div class="info-section" data-source-component="ResolveLevelBar">
                  <h3 class="hero-panel-heading">Resolve</h3>
                  <div class="resolve-display">
                    <div class="resolve-number" data-source-component="ResolveNumber">
                      {props.viewModel.resolve}
                    </div>
                    <div class="resolve-details">
                      <span class="resolve-label">{props.viewModel.resolveLabel}</span>
                      <span class="resolve-xp">
                        XP: {props.viewModel.progression.resolveXP}
                      </span>
                    </div>
                  </div>
                </div>
              </div>
            )}

            {/* ── Camping Skills Panel ── */}
            {activeTab() === "camping-skills" && (
              <div
                class="hero-panel hero-panel--skills"
                data-source-component="CampingSkillsPanel"
                data-source-hierarchy="CharacterWindow/Panels/CampingSkillsPanel"
              >
                <h3 class="hero-panel-heading">Camping Skills</h3>
                <div class="skills-list">
                  <For each={props.viewModel.campingSkills}>
                    {(skill) => (
                      <div class="skill-card skill-card--camping" data-source-component="Skill">
                        <div class="skill-card-header">
                          <span class="skill-card-name" data-source-component="SkillName">{skill.name}</span>
                          <span class="skill-card-level">
                            Lv{skill.level}
                          </span>
                        </div>
                        <div class="skill-card-body" data-source-component="SkillDesc">
                          <span class="skill-card-desc">{skill.description}</span>
                        </div>
                      </div>
                    )}
                  </For>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* ── Bottom resource bar ── */}
      <Show when={resources()}>
        {(res) => (
          <div class="hero-resource-bar" data-source-component="ResourceBar">
            <div class="resource-slot" data-source-sprite="gem_icon.png">
              <span class="resource-slot-icon resource-slot-icon--gem">💎</span>
              <span class="resource-slot-value">{res().gems}</span>
            </div>
            <div class="resource-slot" data-source-sprite="crystal_icon.png">
              <span class="resource-slot-icon resource-slot-icon--crystal">🔮</span>
              <span class="resource-slot-value">{res().crystals}</span>
            </div>
            <div class="resource-slot" data-source-sprite="shard_icon.png">
              <span class="resource-slot-icon resource-slot-icon--shard">⚡</span>
              <span class="resource-slot-value">{res().shards}</span>
            </div>
            <div class="resource-slot resource-slot--gold" data-source-sprite="gold.png">
              <span class="resource-slot-icon resource-slot-icon--gold">🪙</span>
              <span class="resource-slot-value">{res().gold}</span>
            </div>
          </div>
        )}
      </Show>

      {/* ── Bottom actions ── */}
      <div class="hero-detail-actions" data-source-component="CloseButton">
        <button class="action-secondary" onClick={props.onReturn}>
          Return to Town
        </button>
      </div>
    </AppFrame>
  );
};
