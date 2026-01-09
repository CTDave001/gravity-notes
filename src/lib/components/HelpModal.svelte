<script lang="ts">
  import { onMount } from 'svelte';

  let {
    show = false,
    onclose,
  }: {
    show?: boolean;
    onclose?: () => void;
  } = $props();

  let activeTab: 'shortcuts' | 'markdown' = $state('shortcuts');

  // Detect macOS for keyboard shortcut display
  let isMac = $state(false);

  onMount(() => {
    isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0 ||
            navigator.userAgent.toUpperCase().indexOf('MAC') >= 0;
  });

  // Get the correct modifier key name for the current OS
  function mod(): string {
    return isMac ? '⌘' : 'Ctrl';
  }

  function alt(): string {
    return isMac ? '⌥' : 'Alt';
  }

  function shift(): string {
    return isMac ? '⇧' : 'Shift';
  }

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

  // Shortcuts are computed based on OS
  let shortcuts = $derived([
    { category: 'Formatting', items: [
      { keys: [mod(), 'B'], action: 'Bold' },
      { keys: [mod(), 'I'], action: 'Italic' },
      { keys: [mod(), shift(), 'S'], action: 'Strikethrough' },
      { keys: [mod(), 'E'], action: 'Inline code' },
      { keys: [mod(), 'K'], action: 'Insert link' },
    ]},
    { category: 'Headings', items: [
      { keys: [mod(), shift(), '1'], action: 'Heading 1' },
      { keys: [mod(), shift(), '2'], action: 'Heading 2' },
      { keys: [mod(), shift(), '3'], action: 'Heading 3' },
    ]},
    { category: 'Lists & Blocks', items: [
      { keys: [mod(), shift(), '8'], action: 'Bullet list' },
      { keys: [mod(), shift(), '7'], action: 'Numbered list' },
      { keys: [mod(), shift(), 'C'], action: 'Checkbox' },
      { keys: [mod(), shift(), 'Q'], action: 'Quote' },
    ]},
    { category: 'General', items: [
      { keys: [mod(), 'Z'], action: 'Undo' },
      { keys: [mod(), shift(), 'Z'], action: 'Redo' },
      { keys: [mod(), 'S'], action: 'Save note' },
      { keys: [mod(), 'N'], action: 'New note' },
      { keys: [mod(), 'E'], action: 'Export note' },
      { keys: ['F1'], action: 'Show help' },
    ]},
    { category: 'Global (works anywhere)', items: [
      { keys: [mod(), alt(), 'N'], action: 'Quick capture' },
      { keys: [mod(), alt(), 'G'], action: 'Focus Gravity' },
      { keys: [mod(), alt(), 'V'], action: 'Clipboard to Markdown' },
    ]},
  ]);

  const markdownSyntax = [
    { category: 'Text Formatting', items: [
      { syntax: '**bold**', result: 'bold', type: 'bold' },
      { syntax: '*italic*', result: 'italic', type: 'italic' },
      { syntax: '~~strikethrough~~', result: 'strikethrough', type: 'strike' },
      { syntax: '`inline code`', result: 'inline code', type: 'code' },
      { syntax: '[link text](url)', result: 'link text', type: 'link' },
    ]},
    { category: 'Headings', items: [
      { syntax: '# Heading 1', result: 'Heading 1', type: 'h1' },
      { syntax: '## Heading 2', result: 'Heading 2', type: 'h2' },
      { syntax: '### Heading 3', result: 'Heading 3', type: 'h3' },
    ]},
    { category: 'Lists', items: [
      { syntax: '- Item', result: '• Item', type: 'bullet' },
      { syntax: '1. Item', result: '1. Item', type: 'number' },
      { syntax: '- [ ] Task', result: '☐ Task', type: 'checkbox' },
      { syntax: '- [x] Done', result: '☑ Done', type: 'checkbox-done' },
    ]},
    { category: 'Blocks', items: [
      { syntax: '> Quote text', result: 'Quote text', type: 'quote' },
      { syntax: '```\ncode block\n```', result: 'code block', type: 'codeblock' },
      { syntax: '---', result: '―――', type: 'hr' },
    ]},
    { category: 'Tables', items: [
      { syntax: '| Col 1 | Col 2 |\n|-------|-------|\n| A | B |', result: 'Table', type: 'table' },
    ]},
  ];
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
    onclick={handleBackdropClick}
  >
    <div class="bg-white dark:bg-zinc-900 rounded-xl shadow-2xl w-full max-w-2xl mx-4 overflow-hidden max-h-[85vh] flex flex-col">
      <!-- Header -->
      <div class="px-6 py-4 border-b border-zinc-200 dark:border-zinc-700 flex items-center justify-between shrink-0">
        <div>
          <h2 class="text-lg font-semibold text-zinc-900 dark:text-zinc-100">Help</h2>
          <p class="text-sm text-zinc-500 dark:text-zinc-400 mt-0.5">Keyboard shortcuts & Markdown reference</p>
        </div>
        <button
          class="p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-500 transition-colors"
          onclick={onclose}
          title="Close"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Tabs -->
      <div class="px-6 pt-4 border-b border-zinc-200 dark:border-zinc-700 shrink-0">
        <div class="flex gap-1">
          <button
            class="px-4 py-2 text-sm font-medium rounded-t-lg transition-colors relative {activeTab === 'shortcuts'
              ? 'text-zinc-900 dark:text-zinc-100 bg-zinc-100 dark:bg-zinc-800'
              : 'text-zinc-500 hover:text-zinc-700 dark:hover:text-zinc-300'}"
            onclick={() => activeTab = 'shortcuts'}
          >
            Keyboard Shortcuts
          </button>
          <button
            class="px-4 py-2 text-sm font-medium rounded-t-lg transition-colors relative {activeTab === 'markdown'
              ? 'text-zinc-900 dark:text-zinc-100 bg-zinc-100 dark:bg-zinc-800'
              : 'text-zinc-500 hover:text-zinc-700 dark:hover:text-zinc-300'}"
            onclick={() => activeTab = 'markdown'}
          >
            Markdown Syntax
          </button>
        </div>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-y-auto p-6">
        {#if activeTab === 'shortcuts'}
          <div class="space-y-6">
            {#each shortcuts as section}
              <div>
                <h3 class="text-xs font-semibold text-zinc-400 dark:text-zinc-500 uppercase tracking-wider mb-3">
                  {section.category}
                </h3>
                <div class="space-y-2">
                  {#each section.items as item}
                    <div class="flex items-center justify-between py-1.5">
                      <span class="text-sm text-zinc-700 dark:text-zinc-300">{item.action}</span>
                      <div class="flex items-center gap-1">
                        {#each item.keys as key, i}
                          {#if i > 0}
                            <span class="text-zinc-400 text-xs">+</span>
                          {/if}
                          <kbd class="px-2 py-1 text-xs font-mono bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-400 rounded border border-zinc-200 dark:border-zinc-700 shadow-sm">
                            {key}
                          </kbd>
                        {/each}
                      </div>
                    </div>
                  {/each}
                </div>
              </div>
            {/each}
          </div>
        {:else}
          <div class="space-y-6">
            {#each markdownSyntax as section}
              <div>
                <h3 class="text-xs font-semibold text-zinc-400 dark:text-zinc-500 uppercase tracking-wider mb-3">
                  {section.category}
                </h3>
                <div class="space-y-2">
                  {#each section.items as item}
                    <div class="flex items-center gap-4 py-1.5">
                      <code class="flex-1 px-3 py-1.5 text-sm font-mono bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-400 rounded whitespace-pre-wrap">
                        {item.syntax}
                      </code>
                      <span class="text-zinc-400 dark:text-zinc-500">→</span>
                      <span class="flex-1 text-sm
                        {item.type === 'bold' ? 'font-bold text-zinc-900 dark:text-zinc-100' : ''}
                        {item.type === 'italic' ? 'italic text-zinc-900 dark:text-zinc-100' : ''}
                        {item.type === 'strike' ? 'line-through text-zinc-500' : ''}
                        {item.type === 'code' ? 'font-mono bg-zinc-100 dark:bg-zinc-800 px-1.5 py-0.5 rounded text-pink-600 dark:text-pink-400' : ''}
                        {item.type === 'link' ? 'text-blue-600 dark:text-blue-400 underline' : ''}
                        {item.type === 'h1' ? 'text-xl font-bold text-zinc-900 dark:text-zinc-100' : ''}
                        {item.type === 'h2' ? 'text-lg font-bold text-zinc-900 dark:text-zinc-100' : ''}
                        {item.type === 'h3' ? 'text-base font-semibold text-zinc-900 dark:text-zinc-100' : ''}
                        {item.type === 'bullet' || item.type === 'number' ? 'text-zinc-700 dark:text-zinc-300' : ''}
                        {item.type === 'checkbox' ? 'text-zinc-700 dark:text-zinc-300' : ''}
                        {item.type === 'checkbox-done' ? 'text-green-600 dark:text-green-400' : ''}
                        {item.type === 'quote' ? 'italic text-zinc-500 border-l-2 border-zinc-300 dark:border-zinc-600 pl-2' : ''}
                        {item.type === 'codeblock' ? 'font-mono bg-zinc-800 text-zinc-300 px-2 py-1 rounded' : ''}
                        {item.type === 'hr' ? 'text-zinc-300 dark:text-zinc-600' : ''}
                        {item.type === 'table' ? 'text-zinc-600 dark:text-zinc-400' : ''}
                      ">
                        {item.result}
                      </span>
                    </div>
                  {/each}
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div class="px-6 py-3 border-t border-zinc-200 dark:border-zinc-700 bg-zinc-50 dark:bg-zinc-800/50 shrink-0">
        <p class="text-xs text-zinc-500 dark:text-zinc-400 text-center">
          Tip: Paste from web pages - formatting is automatically converted to Markdown
        </p>
      </div>
    </div>
  </div>
{/if}
