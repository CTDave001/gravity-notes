<script lang="ts">
  import { onMount, onDestroy, type Component } from 'svelte';
  import './app.css';
  import { cleanupEmptyNotes } from './lib/api';
  import { getRuntimeInfo } from './lib/platform';
  import { loadSettings, settings, type Settings } from './lib/stores/settings';

  // Svelte 5 state for window type
  let windowType: 'capture' | 'main' | 'note' = $state('main');
  let settingsReady = $state(false);
  let WindowComponent: Component | null = $state(null);

  // Media query for dark mode detection
  let darkModeQuery: MediaQueryList | null = null;
  let unsubscribeSettings: (() => void) | null = null;
  let activeSettings: Settings | null = null;

  function getWindowTypeFromUrl(): 'capture' | 'main' | 'note' {
    const params = new URLSearchParams(window.location.search);
    const windowParam = params.get('window');
    if (windowParam === 'capture') return 'capture';
    if (windowParam === 'note') return 'note';
    return 'main';
  }

  function applyDarkMode(isDark: boolean) {
    if (isDark) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }

  function handleDarkModeChange(event: MediaQueryListEvent) {
    if (activeSettings?.theme === 'system') {
      applyDarkMode(event.matches);
    }
  }

  function applySettings(value: Settings) {
    activeSettings = value;
    const isDark = value.theme === 'dark' ||
      (value.theme === 'system' && (darkModeQuery?.matches ?? false));
    applyDarkMode(isDark);
    document.documentElement.style.setProperty('--editor-font-size', `${value.editorFontSize}px`);
    document.documentElement.style.setProperty('--editor-line-height', String(value.editorLineHeight));
    document.documentElement.style.setProperty(
      '--editor-font-family',
      value.editorFontFamily === 'mono'
        ? "'JetBrains Mono', 'Cascadia Code', 'Consolas', monospace"
        : "-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif",
    );
  }

  onMount(async () => {
    // Determine window type from URL params
    windowType = getWindowTypeFromUrl();

    const runtime = await getRuntimeInfo();
    document.documentElement.dataset.platform = runtime.platform;
    document.documentElement.classList.toggle('mobile', runtime.mobile);

    darkModeQuery = window.matchMedia('(prefers-color-scheme: dark)');
    darkModeQuery.addEventListener('change', handleDarkModeChange);
    await loadSettings();
    unsubscribeSettings = settings.subscribe(applySettings);
    settingsReady = true;

    if (windowType === 'capture') {
      WindowComponent = (await import('./lib/windows/CaptureWindow.svelte')).default;
    } else if (windowType === 'note') {
      WindowComponent = (await import('./lib/windows/NoteWindow.svelte')).default;
    } else {
      WindowComponent = (await import('./lib/windows/MainWindow.svelte')).default;
    }

    // Cleanup empty notes on main window load
    if (windowType === 'main') {
      try {
        await cleanupEmptyNotes(15);
      } catch (err) {
        console.error('Failed to cleanup empty notes:', err);
      }
    }
  });

  onDestroy(() => {
    // Remove dark mode listener
    if (darkModeQuery) {
      darkModeQuery.removeEventListener('change', handleDarkModeChange);
    }
    unsubscribeSettings?.();
  });
</script>

{#if !settingsReady || !WindowComponent}
  <div class="app-loading" aria-label="Loading Gravity"></div>
{:else}
  <WindowComponent />
{/if}

<style>
  .app-loading {
    width: 100vw;
    height: 100vh;
    background: var(--bg-primary);
  }
</style>
