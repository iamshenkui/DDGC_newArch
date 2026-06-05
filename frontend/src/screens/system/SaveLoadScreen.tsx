import { For, type Component } from "solid-js";

import type { SaveLoadViewModel, SaveLoadSlot } from "../../bridge/contractTypes";

interface SaveLoadScreenProps {
  viewModel: SaveLoadViewModel;
  onSelectSlot: (slotId: string) => void;
  onCreateNewSave: (slotId: string) => void;
  onDeleteSave: (slotId: string) => void;
  onReturn: () => void;
}

/**
 * Save/Load selection screen — 存档界面
 *
 * Reference: 跨际元契约/1系统界面/存档界面.png
 * Layout mirrors the Unity save-selection dialog:
 *   dark stone backdrop → parchment frame → slot list → actions
 *
 * Chrome approximations used until BLOCKER-004 (original sprite extraction) is resolved.
 */
export const SaveLoadScreen: Component<SaveLoadScreenProps> = (props) => {
  return (
    <main
      class="save-load-viewport"
      data-source-prefab="Assets/Prefabs/UI/Windows/SaveLoadWindow.prefab"
      data-screen="save-load"
    >
      {/* Stone backdrop — dark textured background from reference */}
      <div class="save-load-backdrop" aria-hidden="true" />

      {/* Parchment frame — centered dialog from reference */}
      <div class="save-load-frame">
        {/* Header with back arrow and title */}
        <header class="save-load-header">
          <button
            class="save-load-back-btn"
            aria-label="返回"
            title="返回"
            onClick={props.onReturn}
            data-source-component="BackButton"
          >
            <span class="save-load-back-arrow" aria-hidden="true">←</span>
          </button>
          <h1 class="save-load-title">{props.viewModel.title}</h1>
          <span class="save-load-header-spacer" aria-hidden="true" />
        </header>

        {/* Slot list */}
        <ul class="save-load-slot-list" role="list">
          <For each={props.viewModel.slots}>
            {(slot) => (
              <li class="save-load-slot" data-slot-id={slot.id} data-slot-status={slot.status}>
                {/* Checkbox indicator */}
                <span class="save-load-slot-checkbox" aria-hidden="true">
                  <span class="save-load-slot-checkmark" />
                </span>

                {/* Slot content */}
                <div class="save-load-slot-content">
                  {slot.status === "empty" ? (
                    <span class="save-load-slot-placeholder">
                      点击开始新的冒险...
                    </span>
                  ) : (
                    <div class="save-load-slot-info">
                      <span class="save-load-slot-name">{slot.label}</span>
                      {slot.week !== undefined && (
                        <span class="save-load-slot-week">第{slot.week}周</span>
                      )}
                    </div>
                  )}
                </div>

                {/* Slot action button */}
                {slot.status === "empty" ? (
                  <button
                    class="save-load-slot-action save-load-slot-action-create"
                    aria-label={`在${slot.label}创建新存档`}
                    title="新建存档"
                    onClick={() => props.onCreateNewSave(slot.id)}
                    data-action="create"
                  >
                    <span class="save-load-slot-plus" aria-hidden="true">+</span>
                  </button>
                ) : (
                  <button
                    class="save-load-slot-action save-load-slot-action-delete"
                    aria-label={`删除${slot.label}`}
                    title="删除存档"
                    onClick={() => props.onDeleteSave(slot.id)}
                    data-action="delete"
                  >
                    <span class="save-load-slot-trash" aria-hidden="true">🗑</span>
                  </button>
                )}
              </li>
            )}
          </For>
        </ul>

        {/* Footer version */}
        <footer class="save-load-footer">
          <span class="save-load-version">V {props.viewModel.version}</span>
        </footer>
      </div>
    </main>
  );
};
