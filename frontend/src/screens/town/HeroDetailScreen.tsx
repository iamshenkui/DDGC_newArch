import type { Component } from "solid-js";

import type { HeroDetailViewModel } from "../../bridge/contractTypes";
import { AppFrame } from "../../components/layout/AppFrame";

interface HeroDetailScreenProps {
  viewModel: HeroDetailViewModel;
  onReturn: () => void;
}

export const HeroDetailScreen: Component<HeroDetailScreenProps> = (props) => {
  return (
    <AppFrame
      eyebrow="Hero Detail"
      title={props.viewModel.name}
      subtitle={`${props.viewModel.classLabel} — Level ${props.viewModel.progression.level}`}
    >
      <section class="grid">
        <div class="stack">
          <section class="panel stack">
            <h2 class="panel-title">Core Status</h2>
            <div class="surface-card stack">
              <div class="row">
                <span class="stat-label">Health</span>
                <span class="stat-value">
                  {props.viewModel.hp} / {props.viewModel.maxHp}
                </span>
              </div>
              <div class="row">
                <span class="stat-label">Stress</span>
                <span class="stat-value">{props.viewModel.stress}</span>
              </div>
              <div class="row">
                <span class="stat-label">Resolve</span>
                <span class="stat-value">{props.viewModel.resolve}</span>
              </div>
            </div>
          </section>

          <section class="panel stack">
            <h2 class="panel-title">Progression</h2>
            <div class="surface-card stack">
              <div class="row">
                <span class="stat-label">Level</span>
                <span class="stat-value">{props.viewModel.progression.level}</span>
              </div>
              <div class="row">
                <span class="stat-label">Experience</span>
                <span class="stat-value">
                  {props.viewModel.progression.experience} /{" "}
                  {props.viewModel.progression.experienceToNext}
                </span>
              </div>
            </div>
          </section>

          <section class="panel stack">
            <h2 class="panel-title">Equipment</h2>
            <div class="surface-card stack">
              <div class="row">
                <span class="stat-label">Weapon</span>
                <span class="stat-value">{props.viewModel.weapon}</span>
              </div>
              <div class="row">
                <span class="stat-label">Armor</span>
                <span class="stat-value">{props.viewModel.armor}</span>
              </div>
            </div>
          </section>
        </div>

        <div class="stack">
          <section class="panel stack">
            <h2 class="panel-title">Resistances</h2>
            <div class="surface-card grid">
              <div class="row">
                <span class="stat-label">Stun</span>
                <span class="stat-value">{props.viewModel.resistances.stun}</span>
              </div>
              <div class="row">
                <span class="stat-label">Bleed</span>
                <span class="stat-value">{props.viewModel.resistances.bleed}</span>
              </div>
              <div class="row">
                <span class="stat-label">Disease</span>
                <span class="stat-value">{props.viewModel.resistances.disease}</span>
              </div>
              <div class="row">
                <span class="stat-label">Move</span>
                <span class="stat-value">{props.viewModel.resistances.move}</span>
              </div>
              <div class="row">
                <span class="stat-label">Death</span>
                <span class="stat-value">{props.viewModel.resistances.death}</span>
              </div>
              <div class="row">
                <span class="stat-label">Trap</span>
                <span class="stat-value">{props.viewModel.resistances.trap}</span>
              </div>
              <div class="row">
                <span class="stat-label">Hazard</span>
                <span class="stat-value">{props.viewModel.resistances.hazard}</span>
              </div>
            </div>
          </section>

          <section class="panel stack">
            <h2 class="panel-title">Combat Skills</h2>
            <ul class="list-reset">
              {props.viewModel.combatSkills.map((skill) => (
                <li class="surface-card">
                  <span class="skill-name">{skill}</span>
                </li>
              ))}
            </ul>
          </section>

          <section class="panel stack">
            <h2 class="panel-title">Camping Skills</h2>
            <ul class="list-reset">
              {props.viewModel.campingSkills.map((skill) => (
                <li class="surface-card">
                  <span class="skill-name">{skill}</span>
                </li>
              ))}
            </ul>
          </section>

          <section class="panel stack">
            <h2 class="panel-title">Camp Notes</h2>
            <div class="surface-card">
              <p>{props.viewModel.campNotes}</p>
            </div>
          </section>
        </div>
      </section>

      <div class="row">
        <button class="action-secondary" onClick={props.onReturn}>
          Return to Town
        </button>
      </div>
    </AppFrame>
  );
};
