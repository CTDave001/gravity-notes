<script lang="ts">
  let {
    show = false,
    noteTitle = '',
    onundo,
    onclose,
  }: {
    show?: boolean;
    noteTitle?: string;
    onundo?: () => void;
    onclose?: () => void;
  } = $props();

  let timeoutId: ReturnType<typeof setTimeout> | null = null;
  let progressWidth = $state(100);
  let animationFrameId: number | null = null;
  let startTime: number = 0;
  const TOAST_DURATION = 5000; // 5 seconds

  $effect(() => {
    if (show) {
      startTime = Date.now();
      animateProgress();
      timeoutId = setTimeout(() => {
        onclose?.();
      }, TOAST_DURATION);
    } else {
      if (timeoutId) clearTimeout(timeoutId);
      if (animationFrameId) cancelAnimationFrame(animationFrameId);
      progressWidth = 100;
    }

    return () => {
      if (timeoutId) clearTimeout(timeoutId);
      if (animationFrameId) cancelAnimationFrame(animationFrameId);
    };
  });

  function animateProgress() {
    const elapsed = Date.now() - startTime;
    const remaining = Math.max(0, TOAST_DURATION - elapsed);
    progressWidth = (remaining / TOAST_DURATION) * 100;

    if (remaining > 0 && show) {
      animationFrameId = requestAnimationFrame(animateProgress);
    }
  }

  function handleUndo() {
    if (timeoutId) clearTimeout(timeoutId);
    if (animationFrameId) cancelAnimationFrame(animationFrameId);
    onundo?.();
  }

  function truncateTitle(title: string, maxLength: number = 30): string {
    if (title.length <= maxLength) return title;
    return title.slice(0, maxLength) + '...';
  }
</script>

{#if show}
  <div class="toast">
    <div class="toast-content">
      <div class="toast-message">
        <svg class="toast-icon" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd" />
        </svg>
        <span>Deleted "{truncateTitle(noteTitle)}"</span>
      </div>
      <div class="toast-actions">
        <button class="toast-btn undo" onclick={handleUndo}>
          Undo
        </button>
        <button class="toast-btn close" onclick={onclose} aria-label="Close">
          <svg viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </button>
      </div>
    </div>
    <div class="progress-bar">
      <div class="progress-fill" style="width: {progressWidth}%"></div>
    </div>
  </div>
{/if}

<style>
  .toast {
    position: fixed;
    bottom: 20px;
    right: 20px;
    z-index: 100;
    animation: slideIn 0.2s ease-out;
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .toast-content {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    background: white;
    border-radius: 10px 10px 0 0;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  }

  :global(.dark) .toast-content {
    background: #27272a;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
  }

  .toast-message {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    color: #3f3f46;
  }

  :global(.dark) .toast-message {
    color: #e4e4e7;
  }

  .toast-icon {
    width: 18px;
    height: 18px;
    color: #a1a1aa;
    flex-shrink: 0;
  }

  :global(.dark) .toast-icon {
    color: #71717a;
  }

  .toast-actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .toast-btn {
    border: none;
    background: transparent;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .toast-btn.undo {
    padding: 6px 12px;
    font-size: 12px;
    font-weight: 500;
    color: #6366f1;
    background: #eef2ff;
    border-radius: 6px;
  }

  .toast-btn.undo:hover {
    background: #e0e7ff;
  }

  :global(.dark) .toast-btn.undo {
    color: #a5b4fc;
    background: rgba(99, 102, 241, 0.15);
  }

  :global(.dark) .toast-btn.undo:hover {
    background: rgba(99, 102, 241, 0.25);
  }

  .toast-btn.close {
    padding: 4px;
    color: #a1a1aa;
    border-radius: 4px;
  }

  .toast-btn.close:hover {
    color: #71717a;
    background: rgba(0, 0, 0, 0.05);
  }

  :global(.dark) .toast-btn.close:hover {
    color: #d4d4d8;
    background: rgba(255, 255, 255, 0.1);
  }

  .toast-btn.close svg {
    width: 16px;
    height: 16px;
  }

  .progress-bar {
    height: 3px;
    background: rgba(0, 0, 0, 0.1);
    border-radius: 0 0 10px 10px;
    overflow: hidden;
  }

  :global(.dark) .progress-bar {
    background: rgba(255, 255, 255, 0.1);
  }

  .progress-fill {
    height: 100%;
    background: #6366f1;
    transition: width 0.1s linear;
  }
</style>
