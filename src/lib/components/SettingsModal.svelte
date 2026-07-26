<script lang="ts">
  import { get } from 'svelte/store';
  import { tick } from 'svelte';
  import { getNotesPath } from '../api';
  import { focusTrap } from '../focusTrap';
  import { saveSettings, settings, type Settings } from '../stores/settings';

  let {
    show = false,
    onclose,
    onsave,
  }: {
    show?: boolean;
    onclose?: () => void;
    onsave?: (value: Settings) => void;
  } = $props();

  let draft: Settings = $state({ ...get(settings) });
  let notesPath = $state('');
  let copied = $state(false);
  let saveError = $state('');
  let saving = $state(false);
  let closeButton: HTMLButtonElement | undefined = $state();
  let wasOpen = false;

  $effect(() => {
    if (show && !wasOpen) {
      draft = { ...get(settings) };
      copied = false;
      saveError = '';
      void getNotesPath().then((path) => notesPath = path);
      void tick().then(() => closeButton?.focus());
    }
    wasOpen = show;
  });

  async function handleSave() {
    saving = true;
    saveError = '';
    try {
      await saveSettings({ ...draft });
      onsave?.({ ...draft });
      onclose?.();
    } catch (error) {
      saveError = error instanceof Error ? error.message : 'Could not save settings.';
    } finally {
      saving = false;
    }
  }

  async function copyPath() {
    if (!notesPath) return;
    await navigator.clipboard.writeText(notesPath);
    copied = true;
    window.setTimeout(() => copied = false, 1500);
  }

  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) onclose?.();
  }

  function handleKeydown(event: KeyboardEvent) {
    if (!show) return;
    if (event.key === 'Escape') {
      event.preventDefault();
      onclose?.();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-backdrop" onclick={handleBackdropClick}>
    <div class="settings-dialog" role="dialog" aria-modal="true" aria-labelledby="settings-title" tabindex="-1" use:focusTrap>
      <header>
        <div>
          <h2 id="settings-title">Settings</h2>
          <p>Make Gravity feel right for the way you write.</p>
        </div>
        <button bind:this={closeButton} class="icon-button" onclick={onclose} aria-label="Close settings">×</button>
      </header>

      <div class="settings-content">
        <fieldset>
          <legend>Appearance</legend>
          <label>
            <span>Theme</span>
            <select bind:value={draft.theme}>
              <option value="system">Follow system</option>
              <option value="light">Light</option>
              <option value="dark">Dark</option>
            </select>
          </label>
        </fieldset>

        <fieldset>
          <legend>Editor</legend>
          <label>
            <span>Writing font</span>
            <select bind:value={draft.editorFontFamily}>
              <option value="sans">Clean sans serif</option>
              <option value="mono">Monospace</option>
            </select>
          </label>
          <label>
            <span>Font size <output>{draft.editorFontSize}px</output></span>
            <input type="range" min="12" max="22" step="1" bind:value={draft.editorFontSize} />
          </label>
          <label>
            <span>Line spacing <output>{draft.editorLineHeight.toFixed(2)}</output></span>
            <input type="range" min="1.35" max="2" step="0.05" bind:value={draft.editorLineHeight} />
          </label>
        </fieldset>

        <fieldset>
          <legend>Workspace</legend>
          <label class="checkbox-row">
            <span>
              <strong>Open notes list on launch</strong>
              <small>You can always toggle it from the title bar.</small>
            </span>
            <input type="checkbox" bind:checked={draft.sidebarDefaultOpen} />
          </label>
          <div class="path-row">
            <span>
              <strong>Notes folder</strong>
              <small title={notesPath}>{notesPath || 'Loading…'}</small>
            </span>
            <button class="secondary-button" onclick={copyPath} disabled={!notesPath}>
              {copied ? 'Copied' : 'Copy path'}
            </button>
          </div>
        </fieldset>
      </div>

      {#if saveError}<p class="save-error" role="alert">{saveError}</p>{/if}
      <footer>
        <button class="secondary-button" onclick={onclose} disabled={saving}>Cancel</button>
        <button class="primary-button" onclick={handleSave} disabled={saving}>
          {saving ? 'Saving…' : 'Save changes'}
        </button>
      </footer>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    z-index: 60;
    display: grid;
    place-items: center;
    padding: 20px;
    background: rgb(0 0 0 / 0.52);
    backdrop-filter: blur(5px);
  }

  .settings-dialog {
    width: min(520px, 100%);
    max-height: min(720px, 90vh);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 16px;
    box-shadow: 0 24px 80px rgb(0 0 0 / 0.35);
  }

  header, footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    padding: 18px 20px;
  }

  header {
    border-bottom: 1px solid var(--border-color);
  }

  footer {
    justify-content: flex-end;
    border-top: 1px solid var(--border-color);
    background: var(--bg-sidebar);
  }

  .save-error {
    margin: 0;
    padding: 10px 20px;
    color: #dc2626;
    background: rgb(239 68 68 / .08);
    font-size: 12px;
  }

  h2 {
    margin: 0;
    font-size: 18px;
  }

  p {
    margin: 3px 0 0;
    color: var(--text-muted);
    font-size: 13px;
  }

  .settings-content {
    padding: 8px 20px 20px;
    overflow-y: auto;
  }

  fieldset {
    display: grid;
    gap: 14px;
    margin: 18px 0 0;
    padding: 0;
    border: 0;
  }

  legend {
    margin-bottom: 3px;
    color: var(--text-muted);
    font-size: 11px;
    font-weight: 700;
    letter-spacing: .08em;
    text-transform: uppercase;
  }

  label, .path-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 20px;
    color: var(--text-secondary);
    font-size: 13px;
  }

  label:not(.checkbox-row) > span {
    flex: 1;
  }

  output {
    margin-left: 6px;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
  }

  select {
    min-width: 160px;
    padding: 7px 30px 7px 10px;
    color: var(--text-primary);
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 7px;
  }

  input[type='range'] {
    width: 180px;
    accent-color: var(--accent);
  }

  input[type='checkbox'] {
    width: 18px;
    height: 18px;
    accent-color: var(--accent);
  }

  .checkbox-row strong, .path-row strong {
    display: block;
    color: var(--text-primary);
    font-weight: 500;
  }

  small {
    display: block;
    max-width: 330px;
    margin-top: 3px;
    overflow: hidden;
    color: var(--text-muted);
    font-size: 12px;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  button {
    border: 0;
    cursor: pointer;
  }

  button:focus-visible, select:focus-visible, input:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }

  .icon-button {
    width: 34px;
    height: 34px;
    color: var(--text-muted);
    background: transparent;
    border-radius: 8px;
    font-size: 24px;
    line-height: 1;
  }

  .icon-button:hover, .secondary-button:hover {
    color: var(--text-primary);
    background: var(--hover-bg);
  }

  .secondary-button, .primary-button {
    padding: 8px 13px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 500;
  }

  .secondary-button {
    color: var(--text-secondary);
    background: var(--bg-surface);
  }

  .primary-button {
    color: white;
    background: var(--accent);
  }

  .primary-button:hover {
    filter: brightness(1.08);
  }

  button:disabled {
    cursor: default;
    opacity: .5;
  }

  @media (max-width: 560px) {
    .modal-backdrop { padding: 10px; }
    label:not(.checkbox-row) { align-items: stretch; flex-direction: column; gap: 8px; }
    select, input[type='range'] { width: 100%; }
  }
</style>
