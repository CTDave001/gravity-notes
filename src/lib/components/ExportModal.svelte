<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-shell';

  // Svelte 5 props using $props() rune
  let {
    show = false,
    noteTitle = 'Untitled',
    noteContent = '',
    onclose,
  }: {
    show?: boolean;
    noteTitle?: string;
    noteContent?: string;
    onclose?: () => void;
  } = $props();

  // Svelte 5 state using $state() rune
  let selectedFormat: 'pdf' | 'md' | 'txt' = $state('md');
  let isExporting: boolean = $state(false);
  let error: string | null = $state(null);

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      onclose?.();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      onclose?.();
    }
  }

  async function handleExport() {
    error = null;
    isExporting = true;

    try {
      if (selectedFormat === 'pdf') {
        // Use window.print() for PDF export
        window.print();
        onclose?.();
      } else {
        // Get downloads directory
        const downloadsDir: string = await invoke('get_downloads_dir');

        // Sanitize filename
        const safeFilename = (noteTitle || 'Untitled')
          .replace(/[<>:"/\\|?*]/g, '_')
          .slice(0, 100);

        // Export the note file
        const outputPath: string = await invoke('export_note_file', {
          content: noteContent,
          filename: safeFilename,
          format: selectedFormat,
          destination: downloadsDir,
        });

        // Open the downloads folder
        await open(downloadsDir);

        onclose?.();
      }
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    } finally {
      isExporting = false;
    }
  }

  function selectFormat(format: 'pdf' | 'md' | 'txt') {
    selectedFormat = format;
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
    onclick={handleBackdropClick}
  >
    <div class="bg-white dark:bg-surface-900 rounded-xl shadow-xl w-full max-w-md mx-4 overflow-hidden">
      <!-- Header -->
      <div class="px-6 py-4 border-b border-surface-200 dark:border-surface-700">
        <h2 class="text-lg font-semibold text-surface-900 dark:text-surface-100">Export Note</h2>
        <p class="text-sm text-surface-500 dark:text-surface-400 mt-1">Choose a format to export your note</p>
      </div>

      <!-- Format selection -->
      <div class="px-6 py-4">
        <div class="grid grid-cols-3 gap-3">
          <!-- PDF button -->
          <button
            class="flex flex-col items-center p-4 rounded-lg border-2 transition-all {selectedFormat === 'pdf'
              ? 'border-accent bg-accent/10 text-accent'
              : 'border-surface-200 dark:border-surface-700 hover:border-surface-300 dark:hover:border-surface-600 text-surface-600 dark:text-surface-400'}"
            onclick={() => selectFormat('pdf')}
          >
            <svg class="w-8 h-8 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 13h6m-6 3h4" />
            </svg>
            <span class="text-sm font-medium">PDF</span>
            <span class="text-xs text-surface-400 dark:text-surface-500 mt-1">Print dialog</span>
          </button>

          <!-- Markdown button -->
          <button
            class="flex flex-col items-center p-4 rounded-lg border-2 transition-all {selectedFormat === 'md'
              ? 'border-accent bg-accent/10 text-accent'
              : 'border-surface-200 dark:border-surface-700 hover:border-surface-300 dark:hover:border-surface-600 text-surface-600 dark:text-surface-400'}"
            onclick={() => selectFormat('md')}
          >
            <svg class="w-8 h-8 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 9h1l1 2 1-2h1M9 15h6" />
            </svg>
            <span class="text-sm font-medium">Markdown</span>
            <span class="text-xs text-surface-400 dark:text-surface-500 mt-1">.md file</span>
          </button>

          <!-- Text button -->
          <button
            class="flex flex-col items-center p-4 rounded-lg border-2 transition-all {selectedFormat === 'txt'
              ? 'border-accent bg-accent/10 text-accent'
              : 'border-surface-200 dark:border-surface-700 hover:border-surface-300 dark:hover:border-surface-600 text-surface-600 dark:text-surface-400'}"
            onclick={() => selectFormat('txt')}
          >
            <svg class="w-8 h-8 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 9h6M9 12h6M9 15h4" />
            </svg>
            <span class="text-sm font-medium">Plain Text</span>
            <span class="text-xs text-surface-400 dark:text-surface-500 mt-1">.txt file</span>
          </button>
        </div>

        {#if error}
          <div class="mt-4 p-3 rounded-lg bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 text-sm">
            {error}
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div class="px-6 py-4 border-t border-surface-200 dark:border-surface-700 flex justify-end gap-3">
        <button
          class="px-4 py-2 text-sm font-medium text-surface-600 dark:text-surface-400 hover:bg-surface-100 dark:hover:bg-surface-800 rounded-lg transition-colors"
          onclick={onclose}
          disabled={isExporting}
        >
          Cancel
        </button>
        <button
          class="px-4 py-2 text-sm font-medium text-white bg-accent hover:bg-accent-hover disabled:opacity-50 disabled:cursor-not-allowed rounded-lg transition-colors"
          onclick={handleExport}
          disabled={isExporting}
        >
          {#if isExporting}
            Exporting...
          {:else}
            Export
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}
