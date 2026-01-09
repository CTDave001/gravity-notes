<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  let {
    show = false,
    message = '',
    filePath = '',
    onclose,
  }: {
    show?: boolean;
    message?: string;
    filePath?: string;
    onclose?: () => void;
  } = $props();

  async function openLocation() {
    if (filePath) {
      await invoke('reveal_in_folder', { path: filePath });
      onclose?.();
    }
  }
</script>

{#if show}
  <div class="toast">
    <div class="toast-content">
      <div class="toast-message">
        <svg class="toast-icon" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
        </svg>
        <span>{message}</span>
      </div>
      <div class="toast-actions">
        <button class="toast-btn open" onclick={openLocation}>
          Open Location
        </button>
        <button class="toast-btn close" onclick={onclose} aria-label="Close">
          <svg viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </button>
      </div>
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
    border-radius: 10px;
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
    color: #22c55e;
    flex-shrink: 0;
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

  .toast-btn.open {
    padding: 6px 12px;
    font-size: 12px;
    font-weight: 500;
    color: #6366f1;
    background: #eef2ff;
    border-radius: 6px;
  }

  .toast-btn.open:hover {
    background: #e0e7ff;
  }

  :global(.dark) .toast-btn.open {
    color: #a5b4fc;
    background: rgba(99, 102, 241, 0.15);
  }

  :global(.dark) .toast-btn.open:hover {
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
</style>
