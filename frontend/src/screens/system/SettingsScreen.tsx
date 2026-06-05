import { For, createSignal, type Component } from "solid-js";

import type { SettingsViewModel, SettingsOption, SettingsCategory } from "../../bridge/contractTypes";

interface SettingsScreenProps {
  viewModel: SettingsViewModel;
  onClose: () => void;
  onChangeOption?: (optionId: string, value: string) => void;
  onSwitchCategory?: (category: SettingsCategory["key"]) => void;
}

export const SettingsScreen: Component<SettingsScreenProps> = (props) => {
  const [activeCategory, setActiveCategory] = createSignal<SettingsCategory["key"]>(
    props.viewModel.activeCategory
  );

  const filteredOptions = () =>
    props.viewModel.options.filter((opt) => opt.category === activeCategory());

  const handleCategorySwitch = (category: SettingsCategory["key"]) => {
    setActiveCategory(category);
    props.onSwitchCategory?.(category);
  };

  return (
    <div class="settings-viewport" data-source-prefab="UI_Settings/UI_Settings">
      {/* ═══ Settings chrome — header bar ═══ */}
      <header class="settings-header">
        <h1 class="settings-title">{props.viewModel.title}</h1>
        <button
          class="settings-close-btn"
          onClick={props.onClose}
          aria-label="关闭设置"
          data-source-prefab="UI_Settings/UI_Settings/CloseButton"
        >
          <span aria-hidden="true">✕</span>
        </button>
      </header>

      <div class="settings-body">
        {/* ═══ Category sidebar ═══ */}
        <nav
          class="settings-category-list"
          aria-label="设置分类"
          data-source-prefab="UI_Settings/UI_Settings/CategoryPanel"
        >
          <For each={props.viewModel.categories}>
            {(category) => (
              <button
                class="settings-category-btn"
                classList={{ active: activeCategory() === category.key }}
                onClick={() => handleCategorySwitch(category.key)}
                aria-pressed={activeCategory() === category.key}
                data-category={category.key}
              >
                <span class="settings-category-label">{category.label}</span>
                <span class="settings-category-desc">{category.description}</span>
              </button>
            )}
          </For>
        </nav>

        {/* ═══ Options panel ═══ */}
        <section
          class="settings-options-panel"
          aria-label={`${activeCategory()} 设置选项`}
          data-source-prefab="UI_Settings/UI_Settings/OptionsPanel"
        >
          <For each={filteredOptions()}>
            {(option) => (
              <div class="settings-option-row" data-option-id={option.id}>
                <div class="settings-option-info">
                  <span class="settings-option-label">{option.label}</span>
                </div>
                <div class="settings-option-control">
                  {option.type === "toggle" && (
                    <ToggleControl
                      value={option.value === "true"}
                      onChange={(val) => props.onChangeOption?.(option.id, String(val))}
                    />
                  )}
                  {option.type === "slider" && (
                    <SliderControl
                      value={Number(option.value)}
                      min={option.min ?? 0}
                      max={option.max ?? 100}
                      onChange={(val) => props.onChangeOption?.(option.id, String(val))}
                    />
                  )}
                  {option.type === "select" && option.options && (
                    <SelectControl
                      value={option.value}
                      options={option.options}
                      onChange={(val) => props.onChangeOption?.(option.id, val)}
                    />
                  )}
                </div>
              </div>
            )}
          </For>
        </section>
      </div>

      {/* ═══ Footer actions ═══ */}
      <footer class="settings-footer">
        {props.viewModel.hasUnsavedChanges && (
          <span class="settings-unsaved-badge">有未保存的更改</span>
        )}
        <button
          class="settings-action-btn settings-action-primary"
          onClick={props.onClose}
          aria-label="保存并关闭"
        >
          保存并关闭
        </button>
        <button
          class="settings-action-btn settings-action-secondary"
          onClick={props.onClose}
          aria-label="取消"
        >
          取消
        </button>
      </footer>
    </div>
  );
};

// ── Sub-components ───────────────────────────────────────────────────────────

interface ToggleControlProps {
  value: boolean;
  onChange: (value: boolean) => void;
}

const ToggleControl: Component<ToggleControlProps> = (props) => {
  return (
    <button
      class="settings-toggle"
      classList={{ on: props.value }}
      onClick={() => props.onChange(!props.value)}
      aria-pressed={props.value}
      role="switch"
    >
      <span class="settings-toggle-thumb" aria-hidden="true" />
    </button>
  );
};

interface SliderControlProps {
  value: number;
  min: number;
  max: number;
  onChange: (value: number) => void;
}

const SliderControl: Component<SliderControlProps> = (props) => {
  const percentage = () =>
    Math.round(((props.value - props.min) / (props.max - props.min)) * 100);

  return (
    <div class="settings-slider-group">
      <input
        class="settings-slider"
        type="range"
        min={props.min}
        max={props.max}
        value={props.value}
        onInput={(e) => props.onChange(Number(e.currentTarget.value))}
      />
      <span class="settings-slider-value">{props.value}</span>
    </div>
  );
};

interface SelectControlProps {
  value: string;
  options: ReadonlyArray<string>;
  onChange: (value: string) => void;
}

const SelectControl: Component<SelectControlProps> = (props) => {
  return (
    <select
      class="settings-select"
      value={props.value}
      onChange={(e) => props.onChange(e.currentTarget.value)}
    >
      <For each={props.options}>
        {(opt) => <option value={opt}>{opt}</option>}
      </For>
    </select>
  );
};
