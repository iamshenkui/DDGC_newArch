import { createSignal, For, type Component } from "solid-js";

import type { SkillDetail } from "../../bridge/contractTypes";

interface CampingSkillsPanelProps {
  skills: ReadonlyArray<SkillDetail>;
}

export const CampingSkillsPanel: Component<CampingSkillsPanelProps> = (props) => {
  const [selectedIndex, setSelectedIndex] = createSignal(0);

  const selectedSkill = () => props.skills[selectedIndex()];

  function skillInitial(name: string): string {
    return name.charAt(0).toUpperCase();
  }

  return (
    <div
      class="camping-skills-panel"
      data-source-component="CampingSkillsPanel"
      data-source-hierarchy="CharacterWindow/Panels/CampingSkillsPanel"
    >
      {/* ── Left: skill icon list (mirrors CampingSkillsPanel skill slot row) ── */}
      <div class="camping-skills-list">
        <For each={props.skills}>
          {(skill, index) => (
            <button
              class={`camping-skill-slot ${selectedIndex() === index() ? "camping-skill-slot--selected" : ""}`}
              onClick={() => setSelectedIndex(index())}
              data-source-component="CampingSkillSlot"
              title={skill.name}
            >
              <div class="camping-skill-icon">
                {skill.icon ? (
                  <img
                    class="camping-skill-icon-image"
                    src={skill.icon}
                    alt={skill.name}
                    loading="lazy"
                  />
                ) : (
                  <span class="camping-skill-icon-initial">
                    {skillInitial(skill.name)}
                  </span>
                )}
              </div>
              <span class="camping-skill-slot-name">{skill.name}</span>
              <span class="camping-skill-slot-level">Lv{skill.level}</span>
            </button>
          )}
        </For>
      </div>

      {/* ── Right: selected skill detail (mirrors CampingSkillsPanel detail pane) ── */}
      <div class="camping-skill-detail">
        {selectedSkill() ? (
          <>
            <div class="camping-skill-detail-header">
              <div class="camping-skill-detail-icon">
                {selectedSkill().icon ? (
                  <img
                    class="camping-skill-icon-image"
                    src={selectedSkill().icon}
                    alt={selectedSkill().name}
                  />
                ) : (
                  <span class="camping-skill-icon-initial">
                    {skillInitial(selectedSkill().name)}
                  </span>
                )}
              </div>
              <div class="camping-skill-detail-titles">
                <span class="camping-skill-detail-name" data-source-component="SkillName">
                  {selectedSkill().name}
                </span>
                <span class="camping-skill-detail-meta">
                  Lv{selectedSkill().level} · {selectedSkill().target}
                </span>
              </div>
            </div>

            <div class="camping-skill-detail-body" data-source-component="SkillDesc">
              <p class="camping-skill-detail-desc">{selectedSkill().description}</p>
            </div>

            <div class="camping-skill-detail-stats">
              {selectedSkill().timeCost !== undefined && (
                <span class="camping-skill-stat" data-source-component="SkillTimeCost">
                  Time: {selectedSkill().timeCost}
                </span>
              )}
              {selectedSkill().useLimit !== undefined && (
                <span class="camping-skill-stat" data-source-component="SkillUseLimit">
                  Uses: {selectedSkill().useLimit}
                </span>
              )}
              <span class="camping-skill-stat" data-source-component="SkillTarget">
                Target: {selectedSkill().target}
              </span>
            </div>
          </>
        ) : (
          <div class="camping-skill-empty">
            <span>No camping skills available</span>
          </div>
        )}
      </div>
    </div>
  );
};
