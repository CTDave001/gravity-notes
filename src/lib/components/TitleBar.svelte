<script lang="ts">
  import { onDestroy } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  let {
    viewMode = 'list',
    sidebarVisible = true,
    onhelp,
    onsettings,
    onsidebar,
    ontoggleview,
    onnewnote,
  }: {
    viewMode?: 'list' | 'grid';
    sidebarVisible?: boolean;
    onhelp?: () => void;
    onsettings?: () => void;
    onsidebar?: () => void;
    ontoggleview?: () => void;
    onnewnote?: () => void;
  } = $props();

  let currentWindow = getCurrentWindow();
  let isMaximized = $state(false);

  // Check initial maximized state
  async function checkMaximized() {
    isMaximized = await currentWindow.isMaximized();
  }
  checkMaximized();

  // Listen for window resize to update maximized state
  const resizeListener = currentWindow.onResized(async () => {
    isMaximized = await currentWindow.isMaximized();
  });

  onDestroy(() => {
    void resizeListener.then((unlisten) => unlisten());
  });

  function startDrag(e: MouseEvent) {
    if (e.button === 0 && !(e.target as HTMLElement).closest('button')) {
      currentWindow.startDragging();
    }
  }

  async function handleDoubleClick(e: MouseEvent) {
    if (!(e.target as HTMLElement).closest('button')) {
      await toggleMaximize();
    }
  }

  async function minimizeWindow() {
    await currentWindow.minimize();
  }

  async function toggleMaximize() {
    await currentWindow.toggleMaximize();
    isMaximized = await currentWindow.isMaximized();
  }

  async function closeWindow() {
    await currentWindow.close();
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="title-bar"
  onmousedown={startDrag}
  ondblclick={handleDoubleClick}
>
  <div class="title-bar-left">
    <span class="app-name">Gravity</span>

    <div class="title-bar-actions">
      <button
        class="action-btn"
        class:active={sidebarVisible && viewMode === 'list'}
        onclick={onsidebar}
        title={sidebarVisible ? 'Hide notes list' : 'Show notes list'}
        aria-label={sidebarVisible ? 'Hide notes list' : 'Show notes list'}
        aria-pressed={sidebarVisible}
      >
        <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <rect x="3.5" y="4" width="17" height="16" rx="2"/>
          <path d="M9 4v16"/>
        </svg>
      </button>

      <!-- Help button -->
      <button
        class="action-btn"
        onclick={onhelp}
        title="Help (F1)"
        aria-label="Help"
      >
        <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path stroke-linecap="round" stroke-linejoin="round" d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9 5.25h.008v.008H12v-.008z" />
        </svg>
      </button>

      <!-- View toggle -->
      <button
        class="action-btn"
        onclick={ontoggleview}
        title={viewMode === 'list' ? 'Grid view' : 'List view'}
        aria-label={viewMode === 'list' ? 'Switch to grid view' : 'Switch to list view'}
      >
        {#if viewMode === 'list'}
          <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z" />
          </svg>
        {:else}
          <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
          </svg>
        {/if}
      </button>

      <button
        class="action-btn"
        onclick={onsettings}
        title="Settings"
        aria-label="Settings"
      >
        <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path stroke-linecap="round" stroke-linejoin="round" d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.592c.55 0 1.02.398 1.11.94l.214 1.286c.063.377.318.69.66.858.077.038.153.077.228.118.337.183.738.24 1.094.1l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.245c.275.476.162 1.08-.273 1.419l-1.028.802c-.3.234-.437.613-.424.994a6.7 6.7 0 010 .408c-.013.381.125.76.424.994l1.028.802c.435.338.548.943.273 1.419l-1.296 2.245a1.125 1.125 0 01-1.37.49l-1.217-.456c-.356-.14-.757-.083-1.094.1a6.86 6.86 0 01-.228.118c-.342.168-.597.481-.66.858l-.214 1.286c-.09.542-.56.94-1.11.94h-2.592c-.55 0-1.02-.398-1.11-.94l-.214-1.286c-.063-.377-.318-.69-.66-.858a6.663 6.663 0 01-.228-.118c-.337-.183-.738-.24-1.094-.1l-1.217.456a1.125 1.125 0 01-1.37-.49l-1.296-2.245a1.125 1.125 0 01.273-1.419l1.028-.802c.3-.234.437-.613.424-.994a6.681 6.681 0 010-.408c.013-.381-.125-.76-.424-.994L3.788 10a1.125 1.125 0 01-.273-1.419l1.296-2.245a1.125 1.125 0 011.37-.49l1.217.456c.356.14.757.083 1.094-.1.075-.041.151-.08.228-.118.342-.168.597-.481.66-.858l.214-1.286z"/>
          <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
        </svg>
      </button>

      <!-- New note -->
      <button
        class="action-btn accent"
        onclick={onnewnote}
        title="New note"
        aria-label="New note"
      >
        <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
        </svg>
      </button>
    </div>
  </div>

  <div class="title-bar-controls">
    <button
      class="control-btn minimize"
      onclick={minimizeWindow}
      aria-label="Minimize"
    >
      <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
        <path d="M2 6h8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
      </svg>
    </button>

    <button
      class="control-btn maximize"
      onclick={toggleMaximize}
      aria-label={isMaximized ? 'Restore' : 'Maximize'}
    >
      {#if isMaximized}
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
          <path d="M3 4.5h5.5V10H3V4.5z" stroke="currentColor" stroke-width="1.5"/>
          <path d="M4.5 4.5V3H10v5.5H8.5" stroke="currentColor" stroke-width="1.5"/>
        </svg>
      {:else}
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
          <rect x="2" y="2" width="8" height="8" stroke="currentColor" stroke-width="1.5" rx="0.5"/>
        </svg>
      {/if}
    </button>

    <button
      class="control-btn close"
      onclick={closeWindow}
      aria-label="Close"
    >
      <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
        <path d="M2.5 2.5l7 7M9.5 2.5l-7 7" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
      </svg>
    </button>
  </div>
</div>

<style>
  .title-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    height: 40px;
    padding: 0 12px;
    box-sizing: border-box;
    background: var(--bg-sidebar);
    user-select: none;
    flex-shrink: 0;
    border-bottom: 1px solid var(--border-color);
  }

  .title-bar-left {
    display: flex;
    align-items: center;
    gap: 16px;
    min-width: 0;
  }

  .app-name {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-secondary);
    letter-spacing: -0.01em;
  }

  .title-bar-actions {
    display: flex;
    align-items: center;
    gap: 2px;
  }

  .action-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 28px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: 6px;
    transition: all 150ms ease;
  }

  .action-btn:hover {
    background: var(--hover-bg);
    color: var(--text-primary);
  }

  .action-btn.accent {
    color: var(--accent);
  }

  .action-btn.active {
    color: var(--text-primary);
    background: var(--hover-bg);
  }

  .action-btn .icon {
    width: 16px;
    height: 16px;
  }

  .title-bar-controls {
    display: flex;
    align-items: center;
    gap: 2px;
    margin-left: auto;
    margin-right: -8px;
    flex-shrink: 0;
  }

  .control-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 32px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: 6px;
    transition: all 150ms ease;
  }

  .control-btn:hover {
    background: var(--hover-bg);
    color: var(--text-primary);
  }

  .control-btn.close:hover {
    background: #e81123;
    color: white;
  }

  .control-btn:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: -2px;
  }
</style>
