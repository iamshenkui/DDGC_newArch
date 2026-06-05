import { createSignal, For, Show, type Component } from "solid-js";

import { resolveHeroPortrait } from "../../assets/originalAssetPaths";
import type { HeroDetailViewModel } from "../../bridge/contractTypes";

type TabKey = "info" | "state" | "camping" | "combat" | "equipment";

interface HeroDetailScreenProps {
  viewModel: HeroDetailViewModel;
  onReturn: () => void;
  onPrevHero?: () => void;
  onNextHero?: () => void;
}

const TAB_CONFIG: ReadonlyArray<{ key: TabKey; label: string; sub: string }> = [
  { key: "info", label: "人物信息", sub: "Info" },
  { key: "state", label: "人物状态", sub: "State" },
  { key: "camping", label: "扎营技能", sub: "Camp" },
  { key: "combat", label: "战斗技能", sub: "Combat" },
  { key: "equipment", label: "装备", sub: "Equip" },
];

export const HeroDetailScreen: Component<HeroDetailScreenProps> = (props) => {
  const [activeTab, setActiveTab] = createSignal<TabKey>("info");

  const portraitSrc = () =>
    resolveHeroPortrait({
      heroId: props.viewModel.heroId,
      classLabel: props.viewModel.classLabel,
    });

  return (
    <div
      class="hero-panel-viewport"
      data-source-prefab="Assets/Prefabs/UI/Windows/CharacterWindow.prefab"
      data-source-component="CharacterWindow"
      data-testid="hero-detail-screen"
    >
      {/* ── Left sidebar ── */}
      <aside class="hero-panel-sidebar">
        {/* Back button */}
        <button
          class="hero-panel-back"
          onClick={props.onReturn}
          aria-label="返回"
          data-source-component="CloseButton"
        >
          ←
        </button>

        {/* Hero thumbnail list area */}
        <div class="hero-panel-thumb-list">
          <div
            class="hero-panel-thumb hero-panel-thumb--active"
            data-testid="hero-thumb-active"
          >
            <Show
              when={portraitSrc()}
              fallback={
                <span class="hero-panel-thumb-letter">
                  {props.viewModel.name[0]}
                </span>
              }
            >
              <img
                src={portraitSrc()!}
                alt={props.viewModel.name}
                class="hero-panel-thumb-img"
              />
            </Show>
          </div>
        </div>

        {/* Prev / Next arrows */}
        <div class="hero-panel-nav">
          <button
            class="hero-panel-nav-arrow"
            onClick={props.onPrevHero}
            disabled={!props.onPrevHero}
            aria-label="上一个英雄"
          >
            ‹
          </button>
          <button
            class="hero-panel-nav-arrow"
            onClick={props.onNextHero}
            disabled={!props.onNextHero}
            aria-label="下一个英雄"
          >
            ›
          </button>
        </div>

        {/* Hero name */}
        <h2 class="hero-panel-name">{props.viewModel.name}</h2>

        {/* Description */}
        <p class="hero-panel-desc">{props.viewModel.heroDescription}</p>

        {/* Talent */}
        <div class="hero-panel-talent">
          <span class="hero-panel-talent-label">天赋</span>
          <p class="hero-panel-talent-text">{props.viewModel.talent}</p>
        </div>

        {/* Exile button */}
        <button
          class="hero-panel-exile"
          data-source-component="ExileButton"
          data-blocker="exile-action-not-wired"
          disabled
          title="放逐功能尚未接入"
        >
          放逐
        </button>
      </aside>

      {/* ── Center: large hero artwork ── */}
      <div class="hero-panel-center">
        <div class="hero-panel-artwork">
          <Show
            when={portraitSrc()}
            fallback={
              <span class="hero-panel-artwork-letter">
                {props.viewModel.classLabel[0]}
              </span>
            }
          >
            <img
              src={portraitSrc()!}
              alt={`${props.viewModel.name} 立绘`}
              class="hero-panel-artwork-img"
              loading="eager"
            />
          </Show>
        </div>
      </div>

      {/* ── Right: vertical tabs + content ── */}
      <div class="hero-panel-right">
        {/* Vertical tab strip */}
        <div class="hero-panel-tab-strip" role="tablist" aria-label="英雄面板标签">
          <For each={TAB_CONFIG}>
            {(tab) => (
              <button
                role="tab"
                aria-selected={activeTab() === tab.key}
                class={`hero-panel-tab ${activeTab() === tab.key ? "hero-panel-tab--active" : ""}`}
                onClick={() => setActiveTab(tab.key)}
                data-tab-key={tab.key}
                data-source-component={`${tab.key}Tab`}
              >
                <span class="hero-panel-tab-label">{tab.label}</span>
              </button>
            )}
          </For>
        </div>

        {/* Tab content */}
        <div class="hero-panel-content">
          {/* ── 人物信息 (Info) ── */}
          <Show when={activeTab() === "info"}>
            <div
              class="hero-panel-content-inner"
              data-source-component="InfoPanel"
            >
              {/* Class header */}
              <div class="hero-class-header">
                <span class="hero-class-diamond" aria-hidden="true">
                  ◆
                </span>
                <span class="hero-class-name">
                  {props.viewModel.classLabel}
                </span>
              </div>

              {/* Stats grid */}
              <div class="hero-stats-grid">
                <div class="hero-stat-row">
                  <div class="hero-stat-cell">
                    <span class="hero-stat-key">生命</span>
                    <span class="hero-stat-val">
                      {props.viewModel.hp}/{props.viewModel.maxHp}
                    </span>
                  </div>
                  <div class="hero-stat-cell">
                    <span class="hero-stat-key">ACC.MOD</span>
                    <span
                      class="hero-stat-val hero-stat-val--blocker"
                      data-blocker="acc-mod-not-in-viewmodel"
                    >
                      +0
                    </span>
                  </div>
                </div>
                <div class="hero-stat-row">
                  <div class="hero-stat-cell">
                    <span class="hero-stat-key">闪避</span>
                    <span class="hero-stat-val">
                      {props.viewModel.baseStats.dodge}
                    </span>
                  </div>
                  <div class="hero-stat-cell">
                    <span class="hero-stat-key">暴击率</span>
                    <span class="hero-stat-val">
                      {props.viewModel.baseStats.crit}
                    </span>
                  </div>
                </div>
                <div class="hero-stat-row">
                  <div class="hero-stat-cell">
                    <span class="hero-stat-key">防御</span>
                    <span
                      class="hero-stat-val hero-stat-val--blocker"
                      data-blocker="defense-not-in-viewmodel"
                    >
                      0%
                    </span>
                  </div>
                  <div class="hero-stat-cell">
                    <span class="hero-stat-key">伤害</span>
                    <span class="hero-stat-val">
                      {props.viewModel.baseStats.dmg}
                    </span>
                  </div>
                </div>
                <div class="hero-stat-row">
                  <div class="hero-stat-cell">
                    <span class="hero-stat-key">速度</span>
                    <span class="hero-stat-val">
                      {props.viewModel.baseStats.spd}
                    </span>
                  </div>
                  <div class="hero-stat-cell">
                    <span class="hero-stat-key">腐蚀</span>
                    <span
                      class="hero-stat-val hero-stat-val--blocker"
                      data-blocker="blight-not-in-viewmodel"
                    >
                      0-0
                    </span>
                  </div>
                </div>
                <div class="hero-stat-row">
                  <div class="hero-stat-cell">
                    <span class="hero-stat-key">精准</span>
                    <span
                      class="hero-stat-val hero-stat-val--blocker"
                      data-blocker="accuracy-not-in-viewmodel"
                    >
                      20%
                    </span>
                  </div>
                  <div class="hero-stat-cell" />
                </div>
              </div>

              {/* Divider */}
              <div class="hero-panel-divider" />

              {/* Traits */}
              <div class="hero-traits-section">
                <h3 class="hero-traits-title">特质</h3>
                <div class="hero-traits-list">
                  <For each={props.viewModel.positiveQuirks}>
                    {(quirk) => (
                      <span class="hero-trait hero-trait--positive">
                        {quirk}
                      </span>
                    )}
                  </For>
                  <For each={props.viewModel.negativeQuirks}>
                    {(quirk) => (
                      <span class="hero-trait hero-trait--negative">
                        {quirk}
                      </span>
                    )}
                  </For>
                  <Show
                    when={
                      props.viewModel.positiveQuirks.length === 0 &&
                      props.viewModel.negativeQuirks.length === 0
                    }
                  >
                    <span class="hero-trait-empty">无特质</span>
                  </Show>
                </div>
              </div>
            </div>
          </Show>

          {/* ── 人物状态 (State) ── */}
          <Show when={activeTab() === "state"}>
            <div
              class="hero-panel-content-inner"
              data-source-component="StatePanel"
            >
              <h3 class="hero-panel-section-title">抗性</h3>
              <div class="hero-resistances">
                <div class="hero-resist-row">
                  <span class="hero-resist-key">眩晕</span>
                  <span class="hero-resist-val">
                    {props.viewModel.resistances.stun}
                  </span>
                </div>
                <div class="hero-resist-row">
                  <span class="hero-resist-key">流血</span>
                  <span class="hero-resist-val">
                    {props.viewModel.resistances.bleed}
                  </span>
                </div>
                <div class="hero-resist-row">
                  <span class="hero-resist-key">疾病</span>
                  <span class="hero-resist-val">
                    {props.viewModel.resistances.disease}
                  </span>
                </div>
                <div class="hero-resist-row">
                  <span class="hero-resist-key">位移</span>
                  <span class="hero-resist-val">
                    {props.viewModel.resistances.move}
                  </span>
                </div>
                <div class="hero-resist-row">
                  <span class="hero-resist-key">死亡</span>
                  <span class="hero-resist-val">
                    {props.viewModel.resistances.death}
                  </span>
                </div>
                <div class="hero-resist-row">
                  <span class="hero-resist-key">陷阱</span>
                  <span class="hero-resist-val">
                    {props.viewModel.resistances.trap}
                  </span>
                </div>
                <div class="hero-resist-row">
                  <span class="hero-resist-key"> hazards </span>
                  <span class="hero-resist-val">
                    {props.viewModel.resistances.hazard}
                  </span>
                </div>
              </div>

              <h3 class="hero-panel-section-title">疾病</h3>
              <div class="hero-disease-list">
                <For each={props.viewModel.diseases}>
                  {(d) => <span class="hero-disease-chip">{d}</span>}
                </For>
                <Show when={props.viewModel.diseases.length === 0}>
                  <span class="hero-trait-empty">无疾病</span>
                </Show>
              </div>

              <h3 class="hero-panel-section-title">进度</h3>
              <div class="hero-progress">
                <div class="hero-progress-row">
                  <span>等级</span>
                  <span>{props.viewModel.progression.level}</span>
                </div>
                <div class="hero-progress-row">
                  <span>经验</span>
                  <span>
                    {props.viewModel.progression.experience} /{" "}
                    {props.viewModel.progression.experienceToNext}
                  </span>
                </div>
                <div class="hero-progress-row">
                  <span>决心等级</span>
                  <span>{props.viewModel.resolve}</span>
                </div>
              </div>
            </div>
          </Show>

          {/* ── 扎营技能 (Camping) ── */}
          <Show when={activeTab() === "camping"}>
            <div
              class="hero-panel-content-inner"
              data-source-component="CampingSkillsPanel"
            >
              <h3 class="hero-panel-section-title">扎营技能</h3>
              <div class="hero-skill-list">
                <For each={props.viewModel.campingSkills}>
                  {(skill) => (
                    <div class="hero-skill-card hero-skill-card--camping">
                      <div class="hero-skill-header">
                        <span class="hero-skill-name">{skill.name}</span>
                        <span class="hero-skill-level">
                          Lv{skill.level}
                        </span>
                      </div>
                      <p class="hero-skill-desc">{skill.description}</p>
                    </div>
                  )}
                </For>
                <Show when={props.viewModel.campingSkills.length === 0}>
                  <span class="hero-trait-empty">无扎营技能</span>
                </Show>
              </div>
            </div>
          </Show>

          {/* ── 战斗技能 (Combat) ── */}
          <Show when={activeTab() === "combat"}>
            <div
              class="hero-panel-content-inner"
              data-source-component="CombatSkillsPanel"
            >
              <h3 class="hero-panel-section-title">战斗技能</h3>
              <div class="hero-skill-list">
                <For each={props.viewModel.combatSkills}>
                  {(skill) => (
                    <div class="hero-skill-card">
                      <div class="hero-skill-header">
                        <span class="hero-skill-name">{skill.name}</span>
                        <span class="hero-skill-level">
                          Lv{skill.level}
                        </span>
                      </div>
                      <p class="hero-skill-desc">{skill.description}</p>
                      <div class="hero-skill-meta">
                        <span>命中 {skill.hitRating}</span>
                        <span>暴击 {skill.critRating}</span>
                        <span>目标 {skill.target}</span>
                      </div>
                    </div>
                  )}
                </For>
                <Show when={props.viewModel.combatSkills.length === 0}>
                  <span class="hero-trait-empty">无战斗技能</span>
                </Show>
              </div>
            </div>
          </Show>

          {/* ── 装备 (Equipment) ── */}
          <Show when={activeTab() === "equipment"}>
            <div
              class="hero-panel-content-inner"
              data-source-component="EquipmentPanel"
            >
              <h3 class="hero-panel-section-title">装备</h3>
              <div class="hero-equip-grid">
                <div class="hero-equip-slot">
                  <span class="hero-equip-label">武器</span>
                  <div class="hero-equip-icon">⚔</div>
                  <span class="hero-equip-name">
                    {props.viewModel.weapon.name}
                  </span>
                  <span class="hero-equip-rank">
                    {"■".repeat(props.viewModel.weapon.level)}
                    {"□".repeat(5 - props.viewModel.weapon.level)}
                  </span>
                </div>
                <div class="hero-equip-slot">
                  <span class="hero-equip-label">护甲</span>
                  <div class="hero-equip-icon">🛡</div>
                  <span class="hero-equip-name">
                    {props.viewModel.armor.name}
                  </span>
                  <span class="hero-equip-rank">
                    {"■".repeat(props.viewModel.armor.level)}
                    {"□".repeat(5 - props.viewModel.armor.level)}
                  </span>
                </div>
                <div class="hero-equip-slot">
                  <span class="hero-equip-label">饰品左</span>
                  <div class="hero-equip-icon">💍</div>
                  <span class="hero-equip-name">
                    {props.viewModel.leftTrinket?.name ?? "—"}
                  </span>
                </div>
                <div class="hero-equip-slot">
                  <span class="hero-equip-label">饰品右</span>
                  <div class="hero-equip-icon">💍</div>
                  <span class="hero-equip-name">
                    {props.viewModel.rightTrinket?.name ?? "—"}
                  </span>
                </div>
              </div>
            </div>
          </Show>
        </div>
      </div>
    </div>
  );
};
