<script lang="ts">
  import type { SaveStatus } from '../types';

  let {
    wordCount = 0,
    charCount = 0,
    line = 1,
    column = 1,
    status = 'saved',
    onretry,
  }: {
    wordCount?: number;
    charCount?: number;
    line?: number;
    column?: number;
    status?: SaveStatus;
    onretry?: () => void;
  } = $props();

  const statusLabel = $derived(
    status === 'saved' ? 'Saved' : status === 'saving' ? 'Saving…' : 'Save failed'
  );
</script>

<div class="status-bar">
  <div class="status-content">
    <div class="stats">
      <span class="stat"><span class="stat-label">Words:</span> {wordCount}</span>
      <span class="stat-divider">·</span>
      <span class="stat"><span class="stat-label">Chars:</span> {charCount}</span>
      <span class="stat-divider">·</span>
      <span class="stat"><span class="stat-label">Ln</span> {line}, <span class="stat-label">Col</span> {column}</span>
    </div>
    {#if status === 'error'}
      <button
        class="save-indicator error"
        title="Save failed — click to retry"
        aria-label="Save failed. Retry saving."
        onclick={onretry}
      >
        <span class="save-dot"></span>
        <span class="save-label">{statusLabel}</span>
      </button>
    {:else}
      <div
        class="save-indicator"
        class:saving={status === 'saving'}
        title={statusLabel}
        aria-label={statusLabel}
        role="status"
      >
        <span class="save-dot"></span>
      </div>
    {/if}
  </div>
</div>

<style>
  .status-bar {
    height: 28px;
    padding: 0 14px;
    display: flex;
    align-items: center;
    user-select: none;
    border-top: 1px solid var(--border-color);
    background: var(--bg-primary);
  }

  :global(.mobile) .status-bar {
    height: calc(30px + env(safe-area-inset-bottom));
    padding: 0 12px env(safe-area-inset-bottom);
  }

  .status-content {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    opacity: 0.6;
    transition: opacity 150ms ease;
  }

  .status-bar:hover .status-content {
    opacity: 0.9;
  }

  .stats {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 11px;
    font-weight: 450;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
  }

  .stat {
    display: flex;
    align-items: center;
    gap: 3px;
  }

  .stat-label {
    opacity: 0.7;
  }

  .stat-divider {
    opacity: 0.4;
  }

  .save-indicator {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 3px 5px;
    color: inherit;
    background: transparent;
    border: 0;
    border-radius: 5px;
  }

  button.save-indicator {
    cursor: pointer;
  }

  button.save-indicator:hover {
    background: var(--hover-bg);
  }

  button.save-indicator:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 1px;
  }

  .save-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #22c55e;
    transition: all 150ms ease;
  }

  .save-indicator.saving .save-dot {
    background: #f59e0b;
    animation: pulse 1s ease-in-out infinite;
  }

  .save-indicator.error .save-dot {
    background: #ef4444;
  }

  .save-label {
    color: #ef4444;
    font-size: 11px;
    font-weight: 500;
  }

  :global(.dark) .save-dot {
    background: #4ade80;
  }

  :global(.dark) .save-indicator.saving .save-dot {
    background: #fbbf24;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.4; }
  }
</style>
