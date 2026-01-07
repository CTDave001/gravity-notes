<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { EditorState } from '@codemirror/state';
  import { EditorView, keymap, placeholder } from '@codemirror/view';
  import { markdown } from '@codemirror/lang-markdown';
  import { defaultKeymap, history, historyKeymap } from '@codemirror/commands';
  import { syntaxHighlighting, defaultHighlightStyle } from '@codemirror/language';
  import type { EditorStats } from '../types';

  // Svelte 5 props using $props() rune
  let {
    content = '',
    autofocus = false,
    onchange,
  }: {
    content?: string;
    autofocus?: boolean;
    onchange?: (data: { content: string; stats: EditorStats }) => void;
  } = $props();

  let editorContainer: HTMLDivElement | undefined = $state();
  let view: EditorView | undefined = $state();

  function countWords(text: string): number {
    return text.trim().split(/\s+/).filter(w => w.length > 0).length;
  }

  function createEditor() {
    if (!editorContainer) return;

    const updateListener = EditorView.updateListener.of((update) => {
      if (update.docChanged) {
        const newContent = update.state.doc.toString();
        onchange?.({
          content: newContent,
          stats: {
            wordCount: countWords(newContent),
            charCount: newContent.length,
          },
        });
      }
    });

    const state = EditorState.create({
      doc: content,
      extensions: [
        history(),
        keymap.of([...defaultKeymap, ...historyKeymap]),
        markdown(),
        syntaxHighlighting(defaultHighlightStyle),
        placeholder('Start writing...'),
        updateListener,
        EditorView.lineWrapping,
        EditorView.theme({
          '&': {
            height: '100%',
            fontSize: '15px',
          },
          '.cm-content': {
            fontFamily: "'JetBrains Mono', 'Consolas', monospace",
            padding: '16px',
          },
          '.cm-line': {
            padding: '0 4px',
          },
          '&.cm-focused': {
            outline: 'none',
          },
          '.cm-placeholder': {
            color: '#a1a1aa',
            fontStyle: 'italic',
          },
        }),
      ],
    });

    view = new EditorView({
      state,
      parent: editorContainer,
    });

    if (autofocus) {
      view.focus();
    }
  }

  // Exposed methods for parent components
  export function getContent(): string {
    return view?.state.doc.toString() ?? content;
  }

  export function setContent(newContent: string) {
    if (view) {
      view.dispatch({
        changes: {
          from: 0,
          to: view.state.doc.length,
          insert: newContent,
        },
      });
    }
  }

  export function focus() {
    view?.focus();
  }

  onMount(() => {
    createEditor();
  });

  onDestroy(() => {
    view?.destroy();
  });
</script>

<div bind:this={editorContainer} class="editor-container h-full w-full"></div>

<style>
  .editor-container {
    height: 100%;
    width: 100%;
  }

  .editor-container :global(.cm-editor) {
    height: 100%;
  }
</style>
