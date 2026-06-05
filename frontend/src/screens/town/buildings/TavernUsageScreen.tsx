import { For, type Component, createSignal, createMemo } from "solid-js";

import type {
  FacilityUsageViewModel,
  FacilityHero,
  FacilityActivity,
} from "../../../bridge/contractTypes";
import { resolveBuildingImage } from "../../../assets/originalAssetPaths";

interface TavernUsageScreenProps {
  viewModel: FacilityUsageViewModel;
  onReturn: () => void;
  onSelectHero: (heroId: string) => void;
  onSelectActivity: (activityId: string) => void;
  onConfirmUsage: () => void;
  onSwitchTab: (tab: "upgrade" | "use") => void;
}

/**
 * Tavern (迷情乐园) facility usage screen.
 *
 * Mirrors Unity prefab:
 *   Assets/Prefabs/UI/Estate/Buildings/Tavern/TavernWindow.prefab
 *
 * Reference image: 公会界面-迷情乐园-使用选择.png
 *
 * Layout:
 *   - Top: Title + Tabs (升级设施 / 使用设施)
 *   - Left: Selected hero portrait + stats + "选择" button
 *   - Center: Hero selection list
 *   - Right: Activity grid (6 slots)
 *   - Bottom: Currency strip
 *   - Top-right: Close button
 */
export const TavernUsageScreen: Component<TavernUsageScreenProps> = (props) => {
  const vm = () => props.viewModel;

  const selectedHero = createMemo(() =>
    vm().heroes.find((h) => h.id === vm().selectedHeroId)
  );

  const selectedActivity = createMemo(() =>
    vm().activities.find((a) => a.id === vm().selectedActivityId)
  );

  const isConfirmEnabled = createMemo(() =>
    Boolean(vm().selectedHeroId && vm().selectedActivityId)
  );

  const spriteSrc = () => resolveBuildingImage("tavern");

  return (
    <div class="facility-usage-viewport" data-source-scene="Assets/Scenes/EstateManagement.unity">
      {/* ── Top HUD: Title + Tabs ── */}
      <div class="facility-usage-hud">
        <div class="facility-usage-hud-left">
          <div class="facility-usage-title-area">
            <img
              class="facility-usage-title-icon"
              src={spriteSrc()}
              alt={vm().facilityLabel}
              data-source-sprite="Assets/Sprites/town/buildings/building_paradise.png"
            />
            <div class="facility-usage-title-text">
              <h1 class="facility-usage-title">{vm().facilityLabel}</h1>
              <span class="facility-usage-subtitle">选择人物并使用设施</span>
            </div>
          </div>
        </div>

        <div class="facility-usage-hud-center">
          <div class="facility-usage-tabs">
            <button
              class={`facility-usage-tab ${vm().activeTab === "upgrade" ? "facility-usage-tab--active" : ""}`}
              onClick={() => props.onSwitchTab("upgrade")}
              data-tab-id="upgrade"
            >
              <span class="facility-usage-tab-check">{vm().activeTab === "upgrade" ? "☑" : "☐"}</span>
              升级设施
            </button>
            <button
              class={`facility-usage-tab ${vm().activeTab === "use" ? "facility-usage-tab--active" : ""}`}
              onClick={() => props.onSwitchTab("use")}
              data-tab-id="use"
            >
              <span class="facility-usage-tab-check">{vm().activeTab === "use" ? "☑" : "☐"}</span>
              使用设施
            </button>
          </div>
        </div>

        <div class="facility-usage-hud-right">
          <button
            class="facility-usage-close-btn"
            onClick={props.onReturn}
            aria-label="关闭"
            data-source-component="CloseButton"
          >
            ✕
          </button>
        </div>
      </div>

      {/* ── Main Content: Left | Center | Right ── */}
      <div class="facility-usage-content">
        {/* Left Panel: Selected Hero Detail */}
        <div class="facility-usage-left-panel">
          <div class="facility-usage-panel-header">
            <h2 class="facility-usage-panel-title">选择人物</h2>
          </div>

          <div class="facility-hero-detail">
            {selectedHero() ? (
              <>
                <div class="facility-hero-portrait">
                  <div class="facility-hero-portrait-frame">
                    <span class="facility-hero-portrait-initial">
                      {selectedHero()!.name[0]?.toUpperCase() ?? "?"}
                    </span>
                  </div>
                </div>

                <div class="facility-hero-info">
                  <div class="facility-hero-name-row">
                    <span class="facility-hero-name">{selectedHero()!.name}</span>
                    <span class="facility-hero-class">{selectedHero()!.classLabel}</span>
                  </div>

                  <div class="facility-hero-stats">
                    <div class="facility-hero-stat">
                      <span class="facility-hero-stat-label">体力/力量</span>
                      <span class="facility-hero-stat-value">{selectedHero()!.hp}</span>
                    </div>
                    <div class="facility-hero-stat">
                      <span class="facility-hero-stat-label">等级</span>
                      <span class="facility-hero-stat-value">Lv.{selectedHero()!.level}</span>
                    </div>
                    <div class="facility-hero-stat">
                      <span class="facility-hero-stat-label">压力</span>
                      <span class="facility-hero-stat-value">{selectedHero()!.stress}/{selectedHero()!.maxStress}</span>
                    </div>
                    <div class="facility-hero-stat">
                      <span class="facility-hero-stat-label">状态</span>
                      <span class={`facility-hero-stat-value ${selectedHero()!.isWounded ? "text-warning" : selectedHero()!.isAfflicted ? "text-danger" : "text-good"}`}>
                        {selectedHero()!.isWounded ? "受伤" : selectedHero()!.isAfflicted ? "折磨" : "正常"}
                      </span>
                    </div>
                  </div>
                </div>
              </>
            ) : (
              <div class="facility-hero-empty">
                <div class="facility-hero-portrait-frame facility-hero-portrait-frame--empty">
                  <span class="facility-hero-portrait-initial">?</span>
                </div>
                <p class="facility-hero-empty-text">请从列表中选择人物</p>
              </div>
            )}
          </div>

          <button
            class="facility-usage-select-btn"
            disabled={!vm().selectedHeroId}
            onClick={() => {
              if (vm().selectedHeroId) props.onSelectHero(vm().selectedHeroId);
            }}
          >
            选择
          </button>
        </div>

        {/* Center Panel: Hero List */}
        <div class="facility-usage-center-panel">
          <div class="facility-hero-list">
            <For each={vm().heroes}>
              {(hero) => (
                <button
                  class={`facility-hero-list-item ${vm().selectedHeroId === hero.id ? "facility-hero-list-item--selected" : ""}`}
                  onClick={() => props.onSelectHero(hero.id)}
                  data-hero-id={hero.id}
                >
                  <div class="facility-hero-list-portrait">
                    <span class="facility-hero-list-initial">{hero.name[0]?.toUpperCase() ?? "?"}</span>
                  </div>
                  <div class="facility-hero-list-info">
                    <div class="facility-hero-list-name-row">
                      <span class="facility-hero-list-name">{hero.name}</span>
                      <span class="facility-hero-list-class">{hero.classLabel}</span>
                    </div>
                    <div class="facility-hero-list-bars">
                      <div class="facility-hero-list-bar">
                        <div
                          class="facility-hero-list-bar-fill facility-hero-list-bar-fill--hp"
                          style={{
                            width: `${Math.max(0, Math.min(100, (hero.health / hero.maxHealth) * 100))}%`
                          }}
                        />
                      </div>
                      <div class="facility-hero-list-bar">
                        <div
                          class="facility-hero-list-bar-fill facility-hero-list-bar-fill--stress"
                          style={{
                            width: `${Math.max(0, Math.min(100, (parseInt(hero.stress) / parseInt(hero.maxStress)) * 100))}%`
                          }}
                        />
                      </div>
                    </div>
                  </div>
                  <div class="facility-hero-list-indicator">
                    {vm().selectedHeroId === hero.id && (
                      <span class="facility-hero-list-check">●</span>
                    )}
                  </div>
                </button>
              )}
            </For>
          </div>
        </div>

        {/* Right Panel: Activity Grid */}
        <div class="facility-usage-right-panel">
          <div class="facility-activity-grid">
            <For each={vm().activities}>
              {(activity) => (
                <button
                  class={`facility-activity-card ${vm().selectedActivityId === activity.id ? "facility-activity-card--selected" : ""} ${!activity.isAvailable ? "facility-activity-card--unavailable" : ""}`}
                  onClick={() => props.onSelectActivity(activity.id)}
                  disabled={!activity.isAvailable}
                  data-activity-id={activity.id}
                >
                  <div class="facility-activity-card-icon">
                    <span class="facility-activity-card-initial">{activity.name[0]?.toUpperCase() ?? "?"}</span>
                  </div>
                  <div class="facility-activity-card-info">
                    <span class="facility-activity-card-name">{activity.name}</span>
                    <span class="facility-activity-card-cost">{activity.cost}</span>
                  </div>
                  {vm().selectedActivityId === activity.id && (
                    <div class="facility-activity-card-selected-mark">✓</div>
                  )}
                </button>
              )}
            </For>
          </div>

          {selectedActivity() && (
            <div class="facility-activity-detail">
              <div class="facility-activity-detail-header">
                <span class="facility-activity-detail-name">{selectedActivity()!.name}</span>
                <span class="facility-activity-detail-cost">{selectedActivity()!.cost}</span>
              </div>
              <p class="facility-activity-detail-desc">{selectedActivity()!.description}</p>
              <div class="facility-activity-detail-meta">
                <span class="facility-activity-detail-pill">压力缓解: {selectedActivity()!.stressReduction}</span>
                <span class="facility-activity-detail-pill">时长: {selectedActivity()!.duration}</span>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* ── Bottom: Currency Strip ── */}
      <div class="facility-usage-currency-strip">
        <div class="facility-usage-currency-item">
          <span class="facility-usage-currency-icon facility-usage-currency-icon--bust">B</span>
          <span class="facility-usage-currency-value">{vm().currencies.bust}</span>
        </div>
        <div class="facility-usage-currency-item">
          <span class="facility-usage-currency-icon facility-usage-currency-icon--portrait">P</span>
          <span class="facility-usage-currency-value">{vm().currencies.portrait}</span>
        </div>
        <div class="facility-usage-currency-item">
          <span class="facility-usage-currency-icon facility-usage-currency-icon--deed">D</span>
          <span class="facility-usage-currency-value">{vm().currencies.deed}</span>
        </div>
        <div class="facility-usage-currency-item">
          <span class="facility-usage-currency-icon facility-usage-currency-icon--crest">C</span>
          <span class="facility-usage-currency-value">{vm().currencies.crest}</span>
        </div>
        <div class="facility-usage-currency-item facility-usage-currency-item--gold">
          <span class="facility-usage-currency-icon facility-usage-currency-icon--gold">G</span>
          <span class="facility-usage-currency-value">{vm().currencies.gold}</span>
        </div>

        <button
          class="facility-usage-confirm-btn"
          disabled={!isConfirmEnabled()}
          onClick={props.onConfirmUsage}
        >
          确认使用
        </button>
      </div>
    </div>
  );
};
