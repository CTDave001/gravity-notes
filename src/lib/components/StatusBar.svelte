<script lang="ts">
  let {
    wordCount = 0,
    charCount = 0,
    line = 1,
    column = 1,
    saved = true,
  }: {
    wordCount?: number;
    charCount?: number;
    line?: number;
    column?: number;
    saved?: boolean;
  } = $props();
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
    <div class="save-indicator" class:saving={!saved}>
      <span class="save-dot"></span>
    </div>
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
