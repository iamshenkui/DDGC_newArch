import { createSignal, For, type Component } from "solid-js";

import { resolveHeroPortrait } from "../../assets/originalAssetPaths";
import type { HeroDetailViewModel } from "../../bridge/contractTypes";
import { AppFrame } from "../../components/layout/AppFrame";

type TabKey = "equipment" | "skills" | "info" | "state";

interface HeroDetailScreenProps {
  viewModel: HeroDetailViewModel;
  onReturn: () => void;
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
  const [activeTab, setActiveTab] = createSignal<TabKey>("equipment");
  const maxStress = () => Number(props.viewModel.maxStress) || 200;
  const pips = () => stressPips(Number(props.viewModel.stress), maxStress());
  const portraitSrc = () =>
    resolveHeroPortrait({
      heroId: props.viewModel.heroId,
      classLabel: props.viewModel.classLabel
    });

  return (
    <AppFrame
      eyebrow="Hero Detail"
      title={`${props.viewModel.name} — ${props.viewModel.classLabel}`}
      subtitle={`Level ${props.viewModel.progression.level} · ${props.viewModel.resolveLabel} (Resolve ${props.viewModel.resolve})`}
    >
      {/* ── Full hero detail panel ─────────────────────────── */}
      <div class="hero-detail-layout">
        {/* ── Left column: portrait area ─────────────────── */}
        <div class="hero-detail-left">
          <div class="hero-portrait-area">
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

          {/* Stress pips (CharacterWindow Style) */}
          <div class="stress-pips-panel">
            <span class="stress-pips-label">Stress</span>
            <div class="stress-pips-row">
              <For each={pips()}>
                {(pip) => (
                  <span
                    class={`stress-pip stress-pip--${pip}`}
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

        {/* ── Right column: tabbed panels ────────────────── */}
        <div class="hero-detail-right">
          {/* Tab buttons */}
          <div class="hero-tab-bar">
            <button
              class={`hero-tab-btn ${activeTab() === "equipment" ? "hero-tab-btn--active" : ""}`}
              onClick={() => setActiveTab("equipment")}
            >
              装备<br /><small>Equip</small>
            </button>
            <button
              class={`hero-tab-btn ${activeTab() === "skills" ? "hero-tab-btn--active" : ""}`}
              onClick={() => setActiveTab("skills")}
            >
              技能<br /><small>Skills</small>
            </button>
            <button
              class={`hero-tab-btn ${activeTab() === "info" ? "hero-tab-btn--active" : ""}`}
              onClick={() => setActiveTab("info")}
            >
              信息<br /><small>Info</small>
            </button>
            <button
              class={`hero-tab-btn ${activeTab() === "state" ? "hero-tab-btn--active" : ""}`}
              onClick={() => setActiveTab("state")}
            >
              状态<br /><small>State</small>
            </button>
          </div>

          {/* ── Equipment Panel ─────────────────────────── */}
          {activeTab() === "equipment" && (
            <div class="hero-panel hero-panel--equipment">
              <div class="equipment-grid">
                <div class="equipment-slot equipment-slot--weapon">
                  <span class="equipment-slot-label">Weapon</span>
                  <div class="equipment-slot-icon">
                    <span class="equipment-slot-placeholder">⚔</span>
                  </div>
                  <span class="equipment-slot-name">{props.viewModel.weapon.name}</span>
                  <span class="equipment-slot-level">
                    {rankDots(props.viewModel.weapon.level)}
                  </span>
                </div>

                <div class="equipment-slot equipment-slot--armor">
                  <span class="equipment-slot-label">Armor</span>
                  <div class="equipment-slot-icon">
                    <span class="equipment-slot-placeholder">🛡</span>
                  </div>
                  <span class="equipment-slot-name">{props.viewModel.armor.name}</span>
                  <span class="equipment-slot-level">
                    {rankDots(props.viewModel.armor.level)}
                  </span>
                </div>

                <div class="equipment-slot equipment-slot--trinket">
                  <span class="equipment-slot-label">Trinket L</span>
                  <div class="equipment-slot-icon">
                    <span class="equipment-slot-placeholder">💍</span>
                  </div>
                  <span class="equipment-slot-name">
                    {props.viewModel.leftTrinket?.name ?? "—"}
                  </span>
                </div>

                <div class="equipment-slot equipment-slot--trinket">
                  <span class="equipment-slot-label">Trinket R</span>
                  <div class="equipment-slot-icon">
                    <span class="equipment-slot-placeholder">💍</span>
                  </div>
                  <span class="equipment-slot-name">
                    {props.viewModel.rightTrinket?.name ?? "—"}
                  </span>
                </div>
              </div>

              {/* HP / Stress bars */}
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

          {/* ── Skills Panel ────────────────────────────── */}
          {activeTab() === "skills" && (
            <div class="hero-panel hero-panel--skills">
              <h3 class="hero-panel-heading">Combat Skills</h3>
              <div class="skills-list">
                <For each={props.viewModel.combatSkills}>
                  {(skill) => (
                    <div class="skill-card">
                      <div class="skill-card-header">
                        <span class="skill-card-name">{skill.name}</span>
                        <span class="skill-card-level">
                          Lv{skill.level} {rankDots(skill.level)}
                        </span>
                      </div>
                      <div class="skill-card-body">
                        <span class="skill-card-desc">{skill.description}</span>
                      </div>
                      <div class="skill-card-stats">
                        <span class="skill-card-stat">
                          Hit: {skill.hitRating}
                        </span>
                        <span class="skill-card-stat">
                          Crit: {skill.critRating}
                        </span>
                        <span class="skill-card-stat">
                          Target: {skill.target}
                        </span>
                      </div>
                    </div>
                  )}
                </For>
              </div>

              <h3 class="hero-panel-heading">Camping Skills</h3>
              <div class="skills-list">
                <For each={props.viewModel.campingSkills}>
                  {(skill) => (
                    <div class="skill-card skill-card--camping">
                      <div class="skill-card-header">
                        <span class="skill-card-name">{skill.name}</span>
                        <span class="skill-card-level">
                          Lv{skill.level}
                        </span>
                      </div>
                      <div class="skill-card-body">
                        <span class="skill-card-desc">{skill.description}</span>
                      </div>
                    </div>
                  )}
                </For>
              </div>
            </div>
          )}

          {/* ── Info Panel ──────────────────────────────── */}
          {activeTab() === "info" && (
            <div class="hero-panel hero-panel--info">
              {/* Description */}
              <div class="info-section">
                <h3 class="hero-panel-heading">Description</h3>
                <p class="hero-desc-text">{props.viewModel.heroDescription}</p>
              </div>

              {/* Talent */}
              <div class="info-section">
                <h3 class="hero-panel-heading">Talent</h3>
                <p class="hero-talent-text">{props.viewModel.talent}</p>
              </div>

              {/* Positive Quirks */}
              <div class="info-section">
                <h3 class="hero-panel-heading">
                  Positive Quirks
                  <span class="heading-count">
                    ({props.viewModel.positiveQuirks.length})
                  </span>
                </h3>
                <div class="quirk-list">
                  <For each={props.viewModel.positiveQuirks}>
                    {(quirk) => (
                      <span class="quirk-chip quirk-chip--positive">
                        {quirk}
                      </span>
                    )}
                  </For>
                  {props.viewModel.positiveQuirks.length === 0 && (
                    <span class="quirk-empty">None</span>
                  )}
                </div>
              </div>

              {/* Negative Quirks */}
              <div class="info-section">
                <h3 class="hero-panel-heading">
                  Negative Quirks
                  <span class="heading-count">
                    ({props.viewModel.negativeQuirks.length})
                  </span>
                </h3>
                <div class="quirk-list">
                  <For each={props.viewModel.negativeQuirks}>
                    {(quirk) => (
                      <span class="quirk-chip quirk-chip--negative">
                        {quirk}
                      </span>
                    )}
                  </For>
                  {props.viewModel.negativeQuirks.length === 0 && (
                    <span class="quirk-empty">None</span>
                  )}
                </div>
              </div>

              {/* Base Stats */}
              <div class="info-section">
                <h3 class="hero-panel-heading">Base Stats</h3>
                <div class="base-stats-grid">
                  <div class="base-stat-cell">
                    <span class="base-stat-label">DMG</span>
                    <span class="base-stat-value">{props.viewModel.baseStats.dmg}</span>
                  </div>
                  <div class="base-stat-cell">
                    <span class="base-stat-label">HP</span>
                    <span class="base-stat-value">{props.viewModel.baseStats.maxHp}</span>
                  </div>
                  <div class="base-stat-cell">
                    <span class="base-stat-label">CRIT</span>
                    <span class="base-stat-value">{props.viewModel.baseStats.crit}</span>
                  </div>
                  <div class="base-stat-cell">
                    <span class="base-stat-label">SPD</span>
                    <span class="base-stat-value">{props.viewModel.baseStats.spd}</span>
                  </div>
                  <div class="base-stat-cell">
                    <span class="base-stat-label">DODGE</span>
                    <span class="base-stat-value">{props.viewModel.baseStats.dodge}</span>
                  </div>
                </div>
              </div>

              {/* Resolve */}
              <div class="info-section">
                <h3 class="hero-panel-heading">Resolve</h3>
                <div class="resolve-display">
                  <div class="resolve-number">
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

          {/* ── State Panel ─────────────────────────────── */}
          {activeTab() === "state" && (
            <div class="hero-panel hero-panel--state">
              {/* Resistances */}
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

              {/* Diseases */}
              <div class="info-section">
                <h3 class="hero-panel-heading">
                  Diseases
                  <span class="heading-count">
                    ({props.viewModel.diseases.length})
                  </span>
                </h3>
                <div class="disease-list">
                  <For each={props.viewModel.diseases}>
                    {(disease) => (
                      <span class="disease-chip">{disease}</span>
                    )}
                  </For>
                  {props.viewModel.diseases.length === 0 && (
                    <span class="quirk-empty">None</span>
                  )}
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
        </div>
      </div>

      {/* ── Bottom actions ────────────────────────────────── */}
      <div class="hero-detail-actions">
        <button class="action-secondary" onClick={props.onReturn}>
          Return to Town
        </button>
      </div>
    </AppFrame>
  );
};
