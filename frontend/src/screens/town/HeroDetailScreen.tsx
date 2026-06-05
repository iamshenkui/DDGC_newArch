import { createSignal, For, type Component } from "solid-js";

import { resolveHeroPortrait } from "../../assets/originalAssetPaths";
import type { HeroDetailViewModel, TownHeroSummary } from "../../bridge/contractTypes";
import { AppFrame } from "../../components/layout/AppFrame";

type TabKey = "equipment" | "combat-skills" | "state" | "info" | "camping-skills";

interface HeroDetailScreenProps {
  viewModel: HeroDetailViewModel;
  roster?: ReadonlyArray<TownHeroSummary>;
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

export const HeroDetailScreen: Component<HeroDetailScreenProps> = (props) => {
  const [activeTab, setActiveTab] = createSignal<TabKey>("state");
  const maxStress = () => Number(props.viewModel.maxStress) || 200;
  const pips = () => stressPips(Number(props.viewModel.stress), maxStress());
  const portraitSrc = () =>
    resolveHeroPortrait({
      heroId: props.viewModel.heroId,
      classLabel: props.viewModel.classLabel
    });

  const currentHeroId = () => props.viewModel.heroId;

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
        {/* ── Far-left: hero roster thumbnails (reference: vertical portrait list) ── */}
        {props.roster && props.roster.length > 0 && (
          <div
            class="hero-detail-roster"
            data-source-hierarchy="CharacterWindow/HeroRoster"
          >
            <button
              class="hero-roster-nav hero-roster-nav--prev"
              onClick={() => props.onPrevHero?.()}
              title="Previous hero"
              aria-label="Previous hero"
            >
              ‹
            </button>
            <div class="hero-roster-list">
              <For each={props.roster}>
                {(hero) => {
                  const isActive = () => hero.id === currentHeroId();
                  const thumbSrc = () =>
                    resolveHeroPortrait({
                      heroId: hero.id,
                      classLabel: hero.classLabel
                    });
                  return (
                    <button
                      class={`hero-roster-thumb ${isActive() ? "hero-roster-thumb--active" : ""}`}
                      onClick={() => {
                        if (!isActive()) props.onSelectHero?.(hero.id);
                      }}
                      title={`${hero.name} — ${hero.classLabel}`}
                      aria-label={`${hero.name} — ${hero.classLabel}`}
                      data-hero-id={hero.id}
                    >
                      {thumbSrc() ? (
                        <img
                          class="hero-roster-thumb-image"
                          src={thumbSrc()}
                          alt=""
                          aria-hidden="true"
                          loading="lazy"
                        />
                      ) : (
                        <span class="hero-roster-thumb-initial">
                          {hero.classLabel[0]}
                        </span>
                      )}
                      {isActive() && <span class="hero-roster-thumb-indicator" />}
                    </button>
                  );
                }}
              </For>
            </div>
            <button
              class="hero-roster-nav hero-roster-nav--next"
              onClick={() => props.onNextHero?.()}
              title="Next hero"
              aria-label="Next hero"
            >
              ›
            </button>
          </div>
        )}

        {/* ── Left column: portrait + stress pips (mirrors StressPanel + HeroPanel) ── */}
        <div
          class="hero-detail-left"
          data-source-hierarchy="CharacterWindow/StressPanel | CharacterWindow/HeroPanel"
        >
          {/* Portrait area — mirrors ModelDisplayPanel (Spine model) + Portrait frame */}
          <div
            class="hero-portrait-area"
            data-source-component="ModelDisplayPanel"
            data-source-hierarchy="CharacterWindow/ModelDisplayPanel"
          >
            <div class="hero-portrait-frame">
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

          {/* Stress pips — mirrors CharacterWindow/StressPanel with 10 StressPip children */}
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

          {/* Tags (Wounded / Afflicted) */}
          <div class="hero-tags">
            {props.viewModel.isWounded && (
              <span class="hero-tag hero-tag--wounded">Wounded</span>
            )}
            {props.viewModel.isAfflicted && (
              <span class="hero-tag hero-tag--afflicted">Afflicted</span>
            )}
          </div>
        </div>

        {/* ── Right column: tabbed panels (mirrors CharacterWindow/Panels) ── */}
        <div
          class="hero-detail-right"
          data-source-component="Panels"
          data-source-hierarchy="CharacterWindow/Panels"
        >
          {/* Tab buttons — order mirrors CharacterWindow button row:
              EquipButton | CombatSkillButton | StateButton | InfoButton | CampingSkillButton */}
          <div class="hero-tab-bar">
            <button
              class={`hero-tab-btn ${activeTab() === "equipment" ? "hero-tab-btn--active" : ""}`}
              onClick={() => setActiveTab("equipment")}
              data-source-component="EquipButton"
            >
              装备<br /><small>Equip</small>
            </button>
            <button
              class={`hero-tab-btn ${activeTab() === "combat-skills" ? "hero-tab-btn--active" : ""}`}
              onClick={() => setActiveTab("combat-skills")}
              data-source-component="CombatSkillButton"
            >
              战斗技能<br /><small>Combat</small>
            </button>
            <button
              class={`hero-tab-btn ${activeTab() === "state" ? "hero-tab-btn--active" : ""}`}
              onClick={() => setActiveTab("state")}
              data-source-component="StateButton"
            >
              状态<br /><small>State</small>
            </button>
            <button
              class={`hero-tab-btn ${activeTab() === "info" ? "hero-tab-btn--active" : ""}`}
              onClick={() => setActiveTab("info")}
              data-source-component="InfoButton"
            >
              信息<br /><small>Info</small>
            </button>
            <button
              class={`hero-tab-btn ${activeTab() === "camping-skills" ? "hero-tab-btn--active" : ""}`}
              onClick={() => setActiveTab("camping-skills")}
              data-source-component="CampingSkillButton"
            >
              扎营技能<br /><small>Camping</small>
            </button>
          </div>

          {/* ── Equipment Panel (mirrors CharacterWindow/Panels/EquipmentPanel) ── */}
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

          {/* ── Combat Skills Panel (mirrors CharacterWindow/Panels/CombatSkillsPanel) ── */}
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

          {/* ── State Panel (mirrors reference: 英雄面板-人物状态) ── */}
          {activeTab() === "state" && (
            <div
              class="hero-panel hero-panel--state"
              data-source-component="StatePanel"
              data-testid="hero-state-panel"
            >
              {/* After-effects / sequelae (后遗症) — diseases and afflictions */}
              <div class="info-section" data-source-component="AftereffectsPanel">
                <h3 class="hero-panel-heading">后遗症</h3>
                <div class="aftereffects-grid">
                  <div class="aftereffect-cell">
                    <span class="aftereffect-label">负面积淀</span>
                    <span class="aftereffect-value">
                      {props.viewModel.isAfflicted ? "是" : "否"}
                    </span>
                  </div>
                  <div class="aftereffect-cell">
                    <span class="aftereffect-label">创伤</span>
                    <span class="aftereffect-value">
                      {props.viewModel.isWounded ? "是" : "否"}
                    </span>
                  </div>
                  <div class="aftereffect-cell">
                    <span class="aftereffect-label">疾病</span>
                    <span class="aftereffect-value">
                      {props.viewModel.diseases.length > 0
                        ? props.viewModel.diseases.join(", ")
                        : "无"}
                    </span>
                  </div>
                </div>
              </div>

              {/* Personality / Traits (性格) */}
              <div class="info-section" data-source-component="PersonalityPanel">
                <h3 class="hero-panel-heading">
                  性格
                  <span class="heading-count">
                    ({props.viewModel.personalities.length})
                  </span>
                </h3>
                <div class="personality-list">
                  <For each={props.viewModel.personalities}>
                    {(p) => (
                      <div
                        class={`personality-chip personality-chip--${p.polarity}`}
                        data-source-component="PersonalitySlot"
                        title={p.description}
                      >
                        <span class="personarity-pip" />
                        <span class="personality-name">{p.name}</span>
                      </div>
                    )}
                  </For>
                  {props.viewModel.personalities.length === 0 && (
                    <span class="quirk-empty">无性格特征</span>
                  )}
                </div>
              </div>

              {/* Resistances (mirrors CharacterWindow/Panels/InfoPanel/ResistancePanel) */}
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

              {/* Progression */}
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

          {/* ── Info Panel (mirrors CharacterWindow/Panels/InfoPanel) ── */}
          {activeTab() === "info" && (
            <div
              class="hero-panel hero-panel--info"
              data-source-component="InfoPanel"
              data-source-hierarchy="CharacterWindow/Panels/InfoPanel"
            >
              {/* Description (mirrors HeroDesc) */}
              <div class="info-section" data-source-component="HeroDesc">
                <h3 class="hero-panel-heading">Description</h3>
                <p class="hero-desc-text">{props.viewModel.heroDescription}</p>
              </div>

              {/* Talent (mirrors HeroTalent) */}
              <div class="info-section" data-source-component="HeroTalent">
                <h3 class="hero-panel-heading">Talent</h3>
                <p class="hero-talent-text">{props.viewModel.talent}</p>
              </div>

              {/* Positive Quirks (mirrors InfoPanel/QuirksPanel/PositiveQuirks) */}
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

              {/* Negative Quirks (mirrors InfoPanel/QuirksPanel/NegativeQuirks) */}
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

              {/* Base Stats (mirrors InfoPanel/BaseStatsPanel) */}
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

              {/* Resolve (mirrors InfoPanel/ResolveLevelBar) */}
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

          {/* ── Camping Skills Panel (mirrors CharacterWindow/Panels/CampingSkillsPanel) ── */}
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

      {/* ── Bottom actions (mirrors CharacterWindow/CloseButton) ── */}
      <div class="hero-detail-actions" data-source-component="CloseButton">
        <button class="action-secondary" onClick={props.onReturn}>
          Return to Town
        </button>
      </div>
    </AppFrame>
  );
};
