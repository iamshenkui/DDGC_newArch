import { createSignal, For, type Component } from "solid-js";

import { resolveHeroPortrait } from "../../assets/originalAssetPaths";
import type { HeroDetailViewModel, TownHeroSummary } from "../../bridge/contractTypes";
import { AppFrame } from "../../components/layout/AppFrame";

type TabKey = "equipment" | "combat-skills" | "state" | "info" | "camping-skills";

interface HeroDetailScreenProps {
  viewModel: HeroDetailViewModel;
  onReturn: () => void;
  onSelectHero?: (heroId: string) => void;
  onPrevHero?: () => void;
  onNextHero?: () => void;
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

function resolveRosterPortrait(hero: TownHeroSummary): string | undefined {
  return resolveHeroPortrait({
    heroId: hero.id,
    classLabel: hero.classLabel
  });
}

export const HeroDetailScreen: Component<HeroDetailScreenProps> = (props) => {
  const [activeTab, setActiveTab] = createSignal<TabKey>("equipment");
  const maxStress = () => Number(props.viewModel.maxStress) || 200;
  const pips = () => stressPips(Number(props.viewModel.stress), maxStress());
  const portraitSrc = () =>
    resolveHeroPortrait({
      heroId: props.viewModel.heroId,
      classLabel: props.viewModel.classLabel
    });

  const tabs: Array<{ key: TabKey; label: string; sub: string; component: string }> = [
    { key: "equipment", label: "装备", sub: "Equip", component: "EquipButton" },
    { key: "combat-skills", label: "战斗技能", sub: "Combat", component: "CombatSkillButton" },
    { key: "state", label: "状态", sub: "State", component: "StateButton" },
    { key: "info", label: "信息", sub: "Info", component: "InfoButton" },
    { key: "camping-skills", label: "扎营技能", sub: "Camping", component: "CampingSkillButton" },
  ];

  return (
    <AppFrame
      eyebrow="Hero Detail"
      title={`${props.viewModel.name} — ${props.viewModel.classLabel}`}
      subtitle={`Level ${props.viewModel.progression.level} · ${props.viewModel.resolveLabel} (Resolve ${props.viewModel.resolve})`}
    >
      {/* ═══ Hero Detail Layout (mirrors CharacterWindow.prefab) ═══ */}
      <div
        class="hero-detail-layout"
        data-source-prefab="Assets/Prefabs/UI/Windows/CharacterWindow.prefab"
        data-source-component="CharacterWindow"
      >
        {/* ── Left sidebar: Hero roster thumbnails ── */}
        <div class="hero-detail-roster" data-source-component="HeroRosterSidebar">
          <For each={props.viewModel.roster}>
            {(hero) => {
              const isActive = hero.id === props.viewModel.heroId;
              const rPortrait = resolveRosterPortrait(hero);
              return (
                <button
                  class={`hero-roster-thumb ${isActive ? "hero-roster-thumb--active" : ""}`}
                  onClick={() => {
                    if (!isActive && props.onSelectHero) {
                      props.onSelectHero(hero.id);
                    }
                  }}
                  data-hero-id={hero.id}
                  data-source-component="HeroRosterThumb"
                >
                  {rPortrait ? (
                    <img
                      class="hero-roster-thumb-img"
                      src={rPortrait}
                      alt={hero.name}
                      loading="lazy"
                    />
                  ) : (
                    <div class="hero-roster-thumb-placeholder">
                      <span class="hero-roster-thumb-letter">{hero.classLabel[0]}</span>
                    </div>
                  )}
                  <span class="hero-roster-thumb-name">{hero.name}</span>
                </button>
              );
            }}
          </For>
        </div>

        {/* ── Center column: portrait + hero info ── */}
        <div
          class="hero-detail-center"
          data-source-hierarchy="CharacterWindow/StressPanel | CharacterWindow/HeroPanel"
        >
          {/* Portrait area — mirrors ModelDisplayPanel */}
          <div
            class="hero-portrait-area"
            data-source-component="ModelDisplayPanel"
            data-source-hierarchy="CharacterWindow/ModelDisplayPanel"
          >
            <div class="hero-portrait-frame hero-portrait-frame--large">
              {portraitSrc() ? (
                <img
                  class="hero-portrait-image"
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

          {/* Hero nameplate below portrait */}
          <div class="hero-detail-nameplate">
            <h2 class="hero-detail-name">{props.viewModel.name}</h2>
            <span class="hero-detail-class">{props.viewModel.classLabel}</span>
            <p class="hero-detail-desc">{props.viewModel.heroDescription}</p>
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
        </div>

        {/* ── Right column: parchment panel with vertical tabs ── */}
        <div
          class="hero-detail-parchment"
          data-source-component="Panels"
          data-source-hierarchy="CharacterWindow/Panels"
        >
          {/* Vertical tab bar on the right edge of parchment */}
          <div class="hero-tab-bar hero-tab-bar--vertical">
            <For each={tabs}>
              {(tab) => (
                <button
                  class={`hero-tab-btn ${activeTab() === tab.key ? "hero-tab-btn--active" : ""}`}
                  onClick={() => setActiveTab(tab.key)}
                  data-source-component={tab.component}
                >
                  {tab.label}
                </button>
              )}
            </For>
          </div>

          {/* Parchment content area */}
          <div class="hero-parchment-content">
            {/* ── Equipment Panel ── */}
            {activeTab() === "equipment" && (
              <div
                class="hero-panel hero-panel--equipment"
                data-source-component="EquipmentPanel"
                data-source-hierarchy="CharacterWindow/Panels/EquipmentPanel"
              >
                <h3 class="hero-panel-heading">Equipment</h3>
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

                {/* HP / Stress core stats */}
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

            {/* ── Combat Skills Panel ── */}
            {activeTab() === "combat-skills" && (
              <div
                class="hero-panel hero-panel--skills"
                data-source-component="CombatSkillsPanel"
                data-source-hierarchy="CharacterWindow/Panels/CombatSkillsPanel"
              >
                <h3 class="hero-panel-heading">Combat Skills</h3>
                <div class="skills-list">
                  <For each={props.viewModel.combatSkills}>
                    {(skill) => (
                      <div class="skill-card" data-source-component="Skill">
                        <div class="skill-card-header">
                          <span class="skill-card-name" data-source-component="SkillName">{skill.name}</span>
                          <span class="skill-card-level">
                            Lv{skill.level} {rankDots(skill.level)}
                          </span>
                        </div>
                        <div class="skill-card-body" data-source-component="SkillDesc">
                          <span class="skill-card-desc">{skill.description}</span>
                        </div>
                        <div class="skill-card-stats">
                          <span class="skill-card-stat">Hit: {skill.hitRating}</span>
                          <span class="skill-card-stat">Crit: {skill.critRating}</span>
                          <span class="skill-card-stat">Target: {skill.target}</span>
                        </div>
                      </div>
                    )}
                  </For>
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

      {/* ── Bottom actions ── */}
      <div class="hero-detail-actions" data-source-component="CloseButton">
        <button class="action-secondary" onClick={props.onReturn}>
          Return to Town
        </button>
      </div>
    </AppFrame>
  );
};
