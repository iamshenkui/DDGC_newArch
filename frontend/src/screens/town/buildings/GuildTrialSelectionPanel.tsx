import { For, createSignal, type Component } from "solid-js";

import type { TrialHeroRosterItem } from "../../../bridge/contractTypes";

interface GuildTrialSelectionPanelProps {
  roster: ReadonlyArray<TrialHeroRosterItem>;
  onSelectHero: (heroId: string) => void;
  onTalk: () => void;
  onLeave: () => void;
}

/**
 * Guild Trial Selection Panel — mirrors GuildWindow trial-ground hero selection.
 *
 * Unity prefab hierarchy:
 *   Assets/Prefabs/UI/Estate/Buildings/Guild/GuildWindow.prefab
 *     RightPanel/TrialSelection/
 *       LeftColumn/  → NPC portrait + selected hero detail
 *       RightColumn/ → Hero roster list with selection checkboxes
 *
 * Reference: 公会界面-试炼场-选择.png
 *   - Top tabs: 升级设施 | 使用设施
 *   - Left: NPC portrait, "选择人物" header, selected hero stats
 *   - Right: Roster table with name, level, status orbs, selection boxes
 *   - Bottom: 对话 (Talk), 离开 (Leave) buttons
 */
export const GuildTrialSelectionPanel: Component<GuildTrialSelectionPanelProps> = (
  props
) => {
  const [selectedHeroId, setSelectedHeroId] = createSignal<string | null>(null);

  const selectedHero = () =>
    props.roster.find((h) => h.id === selectedHeroId()) ?? props.roster[0] ?? null;

  const handleSelect = (heroId: string) => {
    setSelectedHeroId(heroId);
    props.onSelectHero(heroId);
  };

  return (
    <div
      class="guild-trial-panel"
      data-source-prefab="Assets/Prefabs/UI/Estate/Buildings/Guild/GuildWindow.prefab"
      data-source-hierarchy="GuildWindow/RightPanel/TrialSelection"
    >
      {/* ── Left Column — NPC portrait + selected hero detail ── */}
      <div
        class="guild-trial-left"
        data-source-hierarchy="GuildWindow/RightPanel/TrialSelection/LeftColumn"
      >
        {/* NPC Portrait area */}
        <div
          class="guild-trial-npc-portrait"
          data-source-component="NpcPortrait"
          data-source-sprite="Assets/Sprites/ui/guild_trainer_portrait.png"
        >
          <div class="guild-trial-npc-placeholder">
            <span class="guild-trial-npc-label">试炼教官</span>
          </div>
        </div>

        {/* Selected hero detail */}
        {selectedHero() && (
          <div
            class="guild-trial-selected-hero"
            data-source-component="SelectedHeroDetail"
          >
            <h3 class="guild-trial-section-title">选择人物</h3>
            <div class="guild-trial-hero-card">
              <div class="guild-trial-hero-portrait">
                <span class="guild-trial-hero-initial">
                  {selectedHero()!.name[0]?.toUpperCase() ?? "?"}
                </span>
              </div>
              <div class="guild-trial-hero-info">
                <span class="guild-trial-hero-name">{selectedHero()!.name}</span>
                <span class="guild-trial-hero-class">{selectedHero()!.classLabel}</span>
                <span class="guild-trial-hero-level">Lv.{selectedHero()!.level}</span>
              </div>
            </div>
            <div class="guild-trial-hero-stats">
              <div class="guild-trial-stat-row">
                <span class="guild-trial-stat-label">HP</span>
                <span class="guild-trial-stat-value">
                  {selectedHero()!.hp} / {selectedHero()!.maxHp}
                </span>
              </div>
              <div class="guild-trial-stat-row">
                <span class="guild-trial-stat-label">Stress</span>
                <span class="guild-trial-stat-value">
                  {selectedHero()!.stress} / {selectedHero()!.maxStress}
                </span>
              </div>
              {selectedHero()!.isWounded && (
                <span class="guild-trial-hero-status guild-trial-hero-status--wounded">
                  Wounded
                </span>
              )}
              {selectedHero()!.isAfflicted && (
                <span class="guild-trial-hero-status guild-trial-hero-status--afflicted">
                  Afflicted
                </span>
              )}
            </div>
          </div>
        )}

        {/* Bottom action buttons */}
        <div class="guild-trial-actions">
          <button
            class="guild-trial-btn guild-trial-btn--talk"
            onClick={props.onTalk}
            data-source-component="TalkButton"
          >
            对话
          </button>
          <button
            class="guild-trial-btn guild-trial-btn--leave"
            onClick={props.onLeave}
            data-source-component="LeaveButton"
          >
            离开
          </button>
        </div>
      </div>

      {/* ── Right Column — Hero roster list ── */}
      <div
        class="guild-trial-right"
        data-source-hierarchy="GuildWindow/RightPanel/TrialSelection/RightColumn"
      >
        <div class="guild-trial-roster-header">
          <span class="guild-trial-roster-title">队伍列表</span>
          <span class="guild-trial-roster-count">
            {props.roster.length} 名英雄
          </span>
        </div>

        <div class="guild-trial-roster-list">
          <For each={props.roster}>
            {(hero) => {
              const isSelected = () => selectedHeroId() === hero.id;
              return (
                <div
                  class={`guild-trial-roster-row ${isSelected() ? "guild-trial-roster-row--selected" : ""}`}
                  onClick={() => handleSelect(hero.id)}
                  data-hero-id={hero.id}
                  data-source-component="HeroRosterRow"
                >
                  {/* Hero thumbnail */}
                  <div class="guild-trial-row-portrait">
                    <span class="guild-trial-row-initial">
                      {hero.name[0]?.toUpperCase() ?? "?"}
                    </span>
                  </div>

                  {/* Hero name + class */}
                  <div class="guild-trial-row-info">
                    <span class="guild-trial-row-name">{hero.name}</span>
                    <span class="guild-trial-row-class">{hero.classLabel}</span>
                  </div>

                  {/* Level */}
                  <span class="guild-trial-row-level">Lv.{hero.level}</span>

                  {/* Status orbs */}
                  <div class="guild-trial-row-status">
                    {hero.isWounded && (
                      <span
                        class="guild-trial-status-orb guild-trial-status-orb--wounded"
                        title="Wounded"
                      />
                    )}
                    {hero.isAfflicted && (
                      <span
                        class="guild-trial-status-orb guild-trial-status-orb--afflicted"
                        title="Afflicted"
                      />
                    )}
                    {!hero.isWounded && !hero.isAfflicted && (
                      <span
                        class="guild-trial-status-orb guild-trial-status-orb--healthy"
                        title="Healthy"
                      />
                    )}
                  </div>

                  {/* Selection checkbox */}
                  <div class="guild-trial-row-checkbox">
                    <div
                      class={`guild-trial-checkbox-box ${isSelected() ? "guild-trial-checkbox-box--checked" : ""}`}
                    >
                      {isSelected() && (
                        <svg
                          width="12"
                          height="12"
                          viewBox="0 0 12 12"
                          fill="none"
                          xmlns="http://www.w3.org/2000/svg"
                        >
                          <path
                            d="M2 6L5 9L10 3"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                          />
                        </svg>
                      )}
                    </div>
                  </div>
                </div>
              );
            }}
          </For>
        </div>
      </div>
    </div>
  );
};
