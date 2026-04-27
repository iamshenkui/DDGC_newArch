import type { Component } from "solid-js";

import type { TownViewModel } from "../../bridge/contractTypes";
import { AppFrame } from "../../components/layout/AppFrame";
import { PixiStage } from "../../render/PixiStage";

interface TownShellScreenProps {
  viewModel: TownViewModel;
  onOpenHero: (heroId: string) => void;
  onOpenBuilding: (buildingId: string) => void;
  onStartProvisioning: () => void;
}

export const TownShellScreen: Component<TownShellScreenProps> = (props) => {
  return (
    <AppFrame
      eyebrow="Town / Meta Surface"
      title={props.viewModel.campaignName}
      subtitle={props.viewModel.campaignSummary}
    >
      <section class="grid">
        <div class="stack">
          <section class="panel stack">
            <div class="row">
              <span class="pill">Flow: town</span>
              <span class="pill">Next: {props.viewModel.nextActionLabel}</span>
            </div>
            <div class="surface-card stack">
              <h3>{props.viewModel.title}</h3>
              <p>
                This placeholder screen proves the rendered town shell can own
                roster, buildings, and provisioning entry without reading runtime
                internals directly.
              </p>
            </div>
            <div class="row">
              <button class="action-primary" onClick={props.onStartProvisioning}>
                {props.viewModel.nextActionLabel}
              </button>
            </div>
          </section>
          <PixiStage label="Town stage layer placeholder" rendererId="ddgc-town-stage" />
        </div>
        <div class="stack">
          <section class="panel stack">
            <h2 class="panel-title">Roster</h2>
            <ul class="list-reset">
              {props.viewModel.heroes.map((hero) => (
                <li class="surface-card stack">
                  <div class="row">
                    <strong>{hero.name}</strong>
                    <span class="pill">{hero.classLabel}</span>
                    <span class="pill">Lv {hero.level}</span>
                  </div>
                  <p>HP {hero.hp} | Stress {hero.stress}</p>
                  <div class="row">
                    <button
                      class="action-secondary"
                      onClick={() => props.onOpenHero(hero.id)}
                    >
                      Inspect Hero
                    </button>
                  </div>
                </li>
              ))}
            </ul>
          </section>
          <section class="panel stack">
            <h2 class="panel-title">Buildings</h2>
            <ul class="list-reset">
              {props.viewModel.buildings.map((building) => (
                <li class="surface-card stack">
                  <div class="row">
                    <strong>{building.label}</strong>
                    <span class="pill">{building.status}</span>
                  </div>
                  <p>{building.summary}</p>
                  <div class="row">
                    <button
                      class="action-secondary"
                      onClick={() => props.onOpenBuilding(building.id)}
                    >
                      Open Building
                    </button>
                  </div>
                </li>
              ))}
            </ul>
          </section>
        </div>
      </section>
    </AppFrame>
  );
};