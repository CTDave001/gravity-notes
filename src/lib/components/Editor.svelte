<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { EditorState } from '@codemirror/state';
  import { EditorView, keymap, placeholder, drawSelection, hoverTooltip, type Tooltip } from '@codemirror/view';
  import { markdown, markdownLanguage } from '@codemirror/lang-markdown';
  import { languages } from '@codemirror/language-data';
  import { defaultKeymap, history, historyKeymap, undo } from '@codemirror/commands';
  import { syntaxHighlighting, defaultHighlightStyle } from '@codemirror/language';
  import { invoke } from '@tauri-apps/api/core';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { getCurrentWebview } from '@tauri-apps/api/webview';
  import { readFile } from '@tauri-apps/plugin-fs';
  import TurndownService from 'turndown';
  import { gfm } from '@joplin/turndown-plugin-gfm';
  import type { EditorStats } from '../types';

  // Svelte 5 props using $props() rune
  let {
    content = '',
    autofocus = false,
    onchange,
    oncursorchange,
  }: {
    content?: string;
    autofocus?: boolean;
    onchange?: (data: { content: string; stats: EditorStats }) => void;
    oncursorchange?: (data: { line: number; column: number }) => void;
  } = $props();

  let editorContainer: HTMLDivElement | undefined = $state();
  let view: EditorView | undefined = $state();
  let imagesPathValue: string = '';
  let unlistenDragDrop: (() => void) | null = null;

  // Paste toast state
  let showPasteToast = $state(false);
  let pasteToastTimeout: ReturnType<typeof setTimeout> | null = null;
  let lastPastedPlainText = '';
  let lastPastePosition = 0;
  let lastPastedLength = 0;

  // Initialize Turndown for HTML to Markdown conversion
  const turndownService = new TurndownService({
    headingStyle: 'atx',
    codeBlockStyle: 'fenced',
    bulletListMarker: '-',
    br: '\n',
  });
  // Add GFM support (tables, strikethrough, etc.)
  turndownService.use(gfm);

  // Custom rules for better HTML conversion
  turndownService.addRule('lineBreak', {
    filter: 'br',
    replacement: () => '\n'
  });

  turndownService.addRule('paragraph', {
    filter: 'p',
    replacement: (content) => '\n\n' + content + '\n\n'
  });

  turndownService.addRule('div', {
    filter: 'div',
    replacement: (content) => content + '\n'
  });

  turndownService.addRule('span', {
    filter: 'span',
    replacement: (content) => content
  });

  turndownService.addRule('underline', {
    filter: 'u',
    replacement: (content) => '_' + content + '_'
  });

  turndownService.addRule('subscript', {
    filter: 'sub',
    replacement: (content) => '~' + content + '~'
  });

  turndownService.addRule('superscript', {
    filter: 'sup',
    replacement: (content) => '^' + content + '^'
  });

  turndownService.addRule('mark', {
    filter: 'mark',
    replacement: (content) => '==' + content + '=='
  });

  // Remove any remaining inline styles/scripts
  turndownService.remove(['style', 'script', 'noscript', 'iframe', 'object', 'embed']);

  // Keep content from unknown tags
  turndownService.keep(['abbr', 'address', 'cite', 'dfn', 'kbd', 'samp', 'var', 'time']);

  // Post-process markdown to clean up any remaining HTML tags
  function cleanMarkdown(md: string): string {
    return md
      // Replace nbsp with regular spaces
      .replace(/\u00A0/g, ' ')
      .replace(/&nbsp;/gi, ' ')
      // Remove any remaining HTML tags but keep their content
      .replace(/<\/?[a-zA-Z][^>]*>/g, '')
      // Decode common HTML entities
      .replace(/&amp;/g, '&')
      .replace(/&lt;/g, '<')
      .replace(/&gt;/g, '>')
      .replace(/&quot;/g, '"')
      .replace(/&#39;/g, "'")
      // Clean up excessive newlines
      .replace(/\n{3,}/g, '\n\n')
      // Remove leading/trailing whitespace from lines
      .split('\n')
      .map(line => line.trimEnd())
      .join('\n')
      .trim();
  }

  // Use a getter so the tooltip always gets the current value
  function getImagesPath(): string {
    return imagesPathValue;
  }

  function countWords(text: string): number {
    return text.trim().split(/\s+/).filter(w => w.length > 0).length;
  }

  // Handle dropped images from File object (paste)
  async function handleImageDrop(file: File): Promise<string | null> {
    console.log('[Image Drop] Processing file:', file.name, file.type);
    const extension = file.name.split('.').pop()?.toLowerCase() || 'png';
    const validExtensions = ['png', 'jpg', 'jpeg', 'gif', 'webp', 'svg'];

    if (!validExtensions.includes(extension)) {
      console.log('[Image Drop] Invalid extension:', extension);
      return null;
    }

    const buffer = await file.arrayBuffer();
    const data = Array.from(new Uint8Array(buffer));
    console.log('[Image Drop] File size:', data.length, 'bytes');

    try {
      const filename: string = await invoke('save_image', { data, extension });
      console.log('[Image Drop] Saved as:', filename);
      return `![${file.name}](gravity-image://${filename})`;
    } catch (err) {
      console.error('[Image Drop] Failed to save image:', err);
      return null;
    }
  }

  // Handle dropped images from file path (Tauri drag-drop)
  async function handleImagePathDrop(filePath: string): Promise<string | null> {
    console.log('[Image Path Drop] Processing path:', filePath);
    const extension = filePath.split('.').pop()?.toLowerCase() || 'png';
    const validExtensions = ['png', 'jpg', 'jpeg', 'gif', 'webp', 'svg'];

    if (!validExtensions.includes(extension)) {
      console.log('[Image Path Drop] Not an image:', extension);
      return null;
    }

    try {
      // Read file using Tauri's fs plugin
      const fileData = await readFile(filePath);
      const data = Array.from(fileData);
      console.log('[Image Path Drop] File size:', data.length, 'bytes');

      const filename: string = await invoke('save_image', { data, extension });
      console.log('[Image Path Drop] Saved as:', filename);

      // Get original filename for alt text
      const originalName = filePath.split(/[/\\]/).pop() || 'image';
      return `![${originalName}](gravity-image://${filename})`;
    } catch (err) {
      console.error('[Image Path Drop] Failed:', err);
      return null;
    }
  }

  // Show paste toast with auto-dismiss
  function showPasteNotification() {
    showPasteToast = true;
    if (pasteToastTimeout) clearTimeout(pasteToastTimeout);
    pasteToastTimeout = setTimeout(() => {
      showPasteToast = false;
    }, 4000);
  }

  // Swap markdown paste with plain text
  function swapToPlainText() {
    if (!view) return;

    // Validate we have something to swap
    if (!lastPastedPlainText || lastPastedLength === 0) {
      console.warn('[Paste] No plain text available to swap');
      dismissToast();
      return;
    }

    // Validate positions are still valid
    const docLength = view.state.doc.length;
    const from = lastPastePosition;
    const to = lastPastePosition + lastPastedLength;

    if (from < 0 || to > docLength) {
      console.warn('[Paste] Position out of bounds, cannot swap');
      dismissToast();
      return;
    }

    // Replace the pasted markdown with plain text
    view.dispatch({
      changes: { from, to, insert: lastPastedPlainText },
      selection: { anchor: from + lastPastedPlainText.length }
    });

    // Reset state
    lastPastedPlainText = '';
    lastPastedLength = 0;
    dismissToast();
  }

  // Dismiss toast
  function dismissToast() {
    showPasteToast = false;
    if (pasteToastTimeout) clearTimeout(pasteToastTimeout);
  }

  // Check if HTML conversion would produce meaningfully different output than plain text
  function isPlainTextHtml(html: string, plainText: string): boolean {
    // Convert HTML to markdown
    const markdown = cleanMarkdown(turndownService.turndown(html));

    // Normalize both for comparison (collapse whitespace, trim)
    const normalizedMarkdown = markdown.trim().replace(/\s+/g, ' ');
    const normalizedPlain = plainText.trim().replace(/\s+/g, ' ');

    // If the markdown output is essentially the same as plain text, no point showing toast
    return normalizedMarkdown === normalizedPlain;
  }

  // Markdown formatting commands
  function wrapSelection(editorView: EditorView, prefix: string, suffix: string = prefix): boolean {
    const { from, to } = editorView.state.selection.main;
    const hasSelection = from !== to;

    if (hasSelection) {
      // Wrap selected text
      const selectedText = editorView.state.doc.sliceString(from, to);
      editorView.dispatch({
        changes: { from, to, insert: prefix + selectedText + suffix },
        selection: { anchor: from + prefix.length, head: to + prefix.length }
      });
    } else {
      // Insert markers and place cursor between them
      editorView.dispatch({
        changes: { from, insert: prefix + suffix },
        selection: { anchor: from + prefix.length }
      });
    }
    return true;
  }

  function insertAtLineStart(editorView: EditorView, prefix: string): boolean {
    const { from } = editorView.state.selection.main;
    const line = editorView.state.doc.lineAt(from);
    const lineText = line.text;

    // Check if line already has the prefix
    if (lineText.startsWith(prefix)) {
      // Remove the prefix
      editorView.dispatch({
        changes: { from: line.from, to: line.from + prefix.length, insert: '' }
      });
    } else {
      // Add the prefix
      editorView.dispatch({
        changes: { from: line.from, insert: prefix }
      });
    }
    return true;
  }

  // Formatting command functions for keymap
  const formatBold = (editorView: EditorView) => wrapSelection(editorView, '**');
  const formatItalic = (editorView: EditorView) => wrapSelection(editorView, '*');
  const formatStrikethrough = (editorView: EditorView) => wrapSelection(editorView, '~~');
  const formatCode = (editorView: EditorView) => wrapSelection(editorView, '`');
  const formatLink = (editorView: EditorView) => {
    const { from, to } = editorView.state.selection.main;
    const hasSelection = from !== to;

    if (hasSelection) {
      const selectedText = editorView.state.doc.sliceString(from, to);
      // Check if selection looks like a URL
      if (selectedText.startsWith('http://') || selectedText.startsWith('https://')) {
        editorView.dispatch({
          changes: { from, to, insert: '[]('+selectedText+')' },
          selection: { anchor: from + 1 }
        });
      } else {
        editorView.dispatch({
          changes: { from, to, insert: '['+selectedText+'](url)' },
          selection: { anchor: from + selectedText.length + 3, head: from + selectedText.length + 6 }
        });
      }
    } else {
      editorView.dispatch({
        changes: { from, insert: '[](url)' },
        selection: { anchor: from + 1 }
      });
    }
    return true;
  };
  const formatHeading1 = (editorView: EditorView) => insertAtLineStart(editorView, '# ');
  const formatHeading2 = (editorView: EditorView) => insertAtLineStart(editorView, '## ');
  const formatHeading3 = (editorView: EditorView) => insertAtLineStart(editorView, '### ');
  const formatQuote = (editorView: EditorView) => insertAtLineStart(editorView, '> ');
  const formatBulletList = (editorView: EditorView) => insertAtLineStart(editorView, '- ');
  const formatNumberedList = (editorView: EditorView) => insertAtLineStart(editorView, '1. ');
  const formatCheckbox = (editorView: EditorView) => insertAtLineStart(editorView, '- [ ] ');

  // Formatting keymap
  const formattingKeymap = keymap.of([
    { key: 'Mod-b', run: formatBold, preventDefault: true },
    { key: 'Mod-i', run: formatItalic, preventDefault: true },
    { key: 'Mod-Shift-s', run: formatStrikethrough, preventDefault: true },
    { key: 'Mod-e', run: formatCode, preventDefault: true },
    { key: 'Mod-k', run: formatLink, preventDefault: true },
    { key: 'Mod-Shift-1', run: formatHeading1, preventDefault: true },
    { key: 'Mod-Shift-2', run: formatHeading2, preventDefault: true },
    { key: 'Mod-Shift-3', run: formatHeading3, preventDefault: true },
    { key: 'Mod-Shift-q', run: formatQuote, preventDefault: true },
    { key: 'Mod-Shift-8', run: formatBulletList, preventDefault: true },
    { key: 'Mod-Shift-7', run: formatNumberedList, preventDefault: true },
    { key: 'Mod-Shift-c', run: formatCheckbox, preventDefault: true },
  ]);

  // Image hover tooltip extension
  const imageHoverTooltip = hoverTooltip((view, pos) => {
    const line = view.state.doc.lineAt(pos);
    const text = line.text;

    // Match markdown image syntax: ![alt](url)
    const imageRegex = /!\[([^\]]*)\]\(([^)]+)\)/g;
    let match: RegExpExecArray | null;

    while ((match = imageRegex.exec(text)) !== null) {
      const start = line.from + match.index;
      const end = start + match[0].length;

      if (pos >= start && pos <= end) {
        const imagePath = match[2];
        const altText = match[1] || 'Image preview';

        return {
          pos: start,
          end: end,
          above: true,
          create() {
            const dom = document.createElement('div');
            dom.className = 'cm-image-preview';

            const img = document.createElement('img');

            // Handle gravity-image:// protocol
            if (imagePath.startsWith('gravity-image://')) {
              const filename = imagePath.replace('gravity-image://', '');
              const currentImagesPath = getImagesPath();
              console.log('[Image Hover] filename:', filename, 'imagesPath:', currentImagesPath);
              if (currentImagesPath) {
                // Use proper path separator for Windows
                const separator = currentImagesPath.includes('\\') ? '\\' : '/';
                const fullPath = `${currentImagesPath}${separator}${filename}`;
                const assetUrl = convertFileSrc(fullPath);
                console.log('[Image Hover] fullPath:', fullPath, 'assetUrl:', assetUrl);
                img.src = assetUrl;
              } else {
                dom.textContent = 'Loading...';
              }
            } else {
              img.src = imagePath;
            }

            img.alt = altText;
            img.style.maxWidth = '300px';
            img.style.maxHeight = '200px';
            img.style.borderRadius = '8px';
            img.onerror = (e) => {
              console.error('[Image Hover] Image load error:', img.src, e);
              dom.textContent = 'Image not found';
            };
            img.onload = () => {
              console.log('[Image Hover] Image loaded successfully');
            };

            dom.appendChild(img);
            return { dom };
          }
        } as Tooltip;
      }
    }

    return null;
  });

  function createEditor() {
    if (!editorContainer) return;

    const updateListener = EditorView.updateListener.of((update) => {
      if (update.docChanged) {
        const newContent = update.state.doc.toString();
        const pos = update.state.selection.main.head;
        const lineInfo = update.state.doc.lineAt(pos);
        onchange?.({
          content: newContent,
          stats: {
            wordCount: countWords(newContent),
            charCount: newContent.length,
            line: lineInfo.number,
            column: pos - lineInfo.from + 1,
          },
        });
      }
      // Track cursor/selection changes
      if (update.selectionSet) {
        const pos = update.state.selection.main.head;
        const lineInfo = update.state.doc.lineAt(pos);
        oncursorchange?.({
          line: lineInfo.number,
          column: pos - lineInfo.from + 1,
        });
      }
    });

    // Drop handler for images
    const dropHandler = EditorView.domEventHandlers({
      dragover(event) {
        // Must prevent default to allow drop
        if (event.dataTransfer?.types.includes('Files')) {
          event.preventDefault();
          event.dataTransfer.dropEffect = 'copy';
          return true;
        }
        return false;
      },
      drop(event, editorView) {
        console.log('[Drop Event] Triggered');
        const files = event.dataTransfer?.files;
        if (!files || files.length === 0) {
          console.log('[Drop Event] No files');
          return false;
        }

        console.log('[Drop Event] Files:', Array.from(files).map(f => f.name));
        const imageFiles = Array.from(files).filter(f =>
          f.type.startsWith('image/')
        );

        if (imageFiles.length === 0) {
          console.log('[Drop Event] No image files');
          return false;
        }

        event.preventDefault();

        // Get cursor position from drop location
        const pos = editorView.posAtCoords({ x: event.clientX, y: event.clientY });
        console.log('[Drop Event] Position:', pos);
        if (pos === null) {
          console.log('[Drop Event] Could not get position');
          return false;
        }

        // Handle async image processing
        (async () => {
          for (const file of imageFiles) {
            const md = await handleImageDrop(file);
            if (md) {
              editorView.dispatch({
                changes: { from: pos, insert: md + '\n' },
                selection: { anchor: pos + md.length + 1 }
              });
            }
          }
        })();

        return true;
      },
      paste(event, editorView) {
        const clipboardData = event.clipboardData;
        if (!clipboardData) return false;

        const items = clipboardData.items;

        // First check for images
        if (items) {
          for (const item of items) {
            if (item.type.startsWith('image/')) {
              event.preventDefault();
              const file = item.getAsFile();
              if (file) {
                const pos = editorView.state.selection.main.head;
                (async () => {
                  const md = await handleImageDrop(file);
                  if (md) {
                    editorView.dispatch({
                      changes: { from: pos, insert: md + '\n' },
                      selection: { anchor: pos + md.length + 1 }
                    });
                  }
                })();
              }
              return true;
            }
          }
        }

        // Check for HTML content (web clipping)
        if (clipboardData.types.includes('text/html')) {
          const html = clipboardData.getData('text/html');
          let plainText = clipboardData.getData('text/plain') || '';

          // Extract plain text from HTML as fallback
          if (!plainText && html) {
            const tempDiv = document.createElement('div');
            tempDiv.innerHTML = html;
            plainText = tempDiv.textContent || tempDiv.innerText || '';
          }

          // Skip if HTML is trivial (just plain text wrapped in tags)
          if (html && html.includes('<') && !isPlainTextHtml(html, plainText)) {
            event.preventDefault();

            // Convert HTML to Markdown and clean up
            const markdownText = cleanMarkdown(turndownService.turndown(html));

            // Insert at cursor position
            const pos = editorView.state.selection.main.head;

            editorView.dispatch({
              changes: { from: pos, insert: markdownText },
              selection: { anchor: pos + markdownText.length }
            });

            // Store info for potential swap to plain text
            lastPastedPlainText = plainText.trim();
            lastPastePosition = pos;
            lastPastedLength = markdownText.length;

            console.log('[Paste] Converted HTML to Markdown:', {
              markdownLength: markdownText.length,
              plainTextLength: lastPastedPlainText.length,
              position: pos
            });

            // Show toast
            showPasteNotification();

            return true;
          }
        }

        return false;
      }
    });

    const state = EditorState.create({
      doc: content,
      extensions: [
        history(),
        formattingKeymap,
        keymap.of([...defaultKeymap, ...historyKeymap]),
        markdown({
          base: markdownLanguage,
          codeLanguages: languages,
        }),
        syntaxHighlighting(defaultHighlightStyle),
        placeholder('Start writing...'),
        updateListener,
        EditorView.lineWrapping,
        drawSelection(),
        imageHoverTooltip,
        dropHandler,
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
          '.cm-cursor': {
            borderLeftColor: '#18181b',
            borderLeftWidth: '2px',
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

  // Direct DOM event handlers for drag+drop (more reliable in Tauri)
  function handleContainerDragOver(e: DragEvent) {
    if (e.dataTransfer?.types.includes('Files')) {
      e.preventDefault();
      e.stopPropagation();
      e.dataTransfer.dropEffect = 'copy';
    }
  }

  async function handleContainerDrop(e: DragEvent) {
    console.log('[Container Drop] Event triggered');
    const files = e.dataTransfer?.files;
    if (!files || files.length === 0) {
      console.log('[Container Drop] No files');
      return;
    }

    const imageFiles = Array.from(files).filter(f => f.type.startsWith('image/'));
    console.log('[Container Drop] Image files:', imageFiles.map(f => f.name));

    if (imageFiles.length === 0) return;

    e.preventDefault();
    e.stopPropagation();

    if (!view) {
      console.log('[Container Drop] No editor view');
      return;
    }

    // Insert at current cursor position
    const pos = view.state.selection.main.head;
    console.log('[Container Drop] Insert position:', pos);

    for (const file of imageFiles) {
      const md = await handleImageDrop(file);
      console.log('[Container Drop] Generated markdown:', md);
      if (md && view) {
        view.dispatch({
          changes: { from: pos, insert: md + '\n' },
          selection: { anchor: pos + md.length + 1 }
        });
      }
    }
  }

  onMount(async () => {
    // Get images path for hover previews
    try {
      imagesPathValue = await invoke('get_images_path');
      console.log('[Editor] Images path:', imagesPathValue);
    } catch (err) {
      console.error('[Editor] Failed to get images path:', err);
    }

    createEditor();

    // Set up Tauri drag-drop listener
    try {
      const webview = getCurrentWebview();
      const unlisten = await webview.onDragDropEvent(async (event) => {
        console.log('[Tauri DragDrop] Event:', event.payload.type);

        if (event.payload.type === 'drop') {
          const paths = event.payload.paths;
          console.log('[Tauri DragDrop] Dropped paths:', paths);

          if (!view || !paths || paths.length === 0) return;

          const pos = view.state.selection.main.head;

          for (const filePath of paths) {
            const md = await handleImagePathDrop(filePath);
            if (md && view) {
              view.dispatch({
                changes: { from: pos, insert: md + '\n' },
                selection: { anchor: pos + md.length + 1 }
              });
              view.focus();
            }
          }
        }
      });
      unlistenDragDrop = unlisten;
    } catch (err) {
      console.error('[Editor] Failed to set up drag-drop:', err);
    }
  });

  onDestroy(() => {
    view?.destroy();
    unlistenDragDrop?.();
  });
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="editor-wrapper h-full w-full relative">
  <div
    bind:this={editorContainer}
    class="editor-container h-full w-full"
    ondragover={handleContainerDragOver}
    ondrop={handleContainerDrop}
  ></div>

  <!-- Paste toast notification -->
  {#if showPasteToast}
    <div class="paste-toast">
      <span class="toast-text">Pasted as Markdown</span>
      <button class="toast-btn" onclick={swapToPlainText}>Plain text</button>
      <button class="toast-dismiss" onclick={dismissToast}>×</button>
    </div>
  {/if}
</div>

<style>
  .editor-wrapper {
    position: relative;
  }

  .editor-container {
    height: 100%;
    width: 100%;
  }

  /* Paste toast notification */
  .paste-toast {
    position: absolute;
    bottom: 20px;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 16px;
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    border: 1px solid rgba(0, 0, 0, 0.06);
    border-radius: 10px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08), 0 1px 3px rgba(0, 0, 0, 0.05);
    font-size: 13px;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
    z-index: 100;
    animation: toast-slide-in 0.25s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes toast-slide-in {
    from {
      opacity: 0;
      transform: translateX(-50%) translateY(12px) scale(0.96);
    }
    to {
      opacity: 1;
      transform: translateX(-50%) translateY(0) scale(1);
    }
  }

  .toast-text {
    color: #52525b;
    font-weight: 450;
  }

  .toast-btn {
    padding: 5px 12px;
    background: #18181b;
    color: #fafafa;
    border: none;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .toast-btn:hover {
    background: #27272a;
    transform: translateY(-1px);
  }

  .toast-btn:active {
    transform: translateY(0);
  }

  .toast-dismiss {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    padding: 0;
    margin-left: -4px;
    background: transparent;
    color: #a1a1aa;
    border: none;
    border-radius: 4px;
    font-size: 15px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .toast-dismiss:hover {
    background: rgba(0, 0, 0, 0.05);
    color: #71717a;
  }

  :global(.dark) .paste-toast {
    background: rgba(39, 39, 42, 0.92);
    border-color: rgba(255, 255, 255, 0.08);
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.25), 0 1px 3px rgba(0, 0, 0, 0.15);
  }

  :global(.dark) .toast-text {
    color: #a1a1aa;
  }

  :global(.dark) .toast-btn {
    background: #fafafa;
    color: #18181b;
  }

  :global(.dark) .toast-btn:hover {
    background: #e4e4e7;
  }

  :global(.dark) .toast-dismiss {
    color: #71717a;
  }

  :global(.dark) .toast-dismiss:hover {
    background: rgba(255, 255, 255, 0.08);
    color: #a1a1aa;
  }

  .editor-container :global(.cm-editor) {
    height: 100%;
  }

  /* Text selection with accent color for contrast */
  .editor-container :global(.cm-selectionBackground) {
    background: var(--accent) !important;
    opacity: 0.9;
  }

  .editor-container :global(.cm-focused .cm-selectionBackground) {
    background: var(--accent) !important;
  }

  .editor-container :global(.cm-content ::selection) {
    background: var(--accent);
    color: #ffffff;
  }

  .editor-container :global(.cm-line ::selection) {
    background: var(--accent);
    color: #ffffff;
  }

  /* Image preview tooltip */
  .editor-container :global(.cm-tooltip) {
    border: none;
    background: transparent;
  }

  .editor-container :global(.cm-image-preview) {
    padding: 8px;
    background: white;
    border-radius: 12px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
    max-width: 320px;
  }

  :global(.dark) .editor-container :global(.cm-image-preview) {
    background: #27272a;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
  }

  .editor-container :global(.cm-image-preview img) {
    display: block;
    max-width: 100%;
    height: auto;
    border-radius: 8px;
  }
</style>
