<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import './app.css';
  import CaptureWindow from './lib/windows/CaptureWindow.svelte';
  import MainWindow from './lib/windows/MainWindow.svelte';
  import NoteWindow from './lib/windows/NoteWindow.svelte';
  import { cleanupEmptyNotes } from './lib/api';

  // Svelte 5 state for window type
  let windowType: 'capture' | 'main' | 'note' = $state('main');

  // Media query for dark mode detection
  let darkModeQuery: MediaQueryList | null = null;

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
    applyDarkMode(event.matches);
  }

  onMount(async () => {
    // Determine window type from URL params
    windowType = getWindowTypeFromUrl();

    // Set up dark mode detection
    darkModeQuery = window.matchMedia('(prefers-color-scheme: dark)');
    applyDarkMode(darkModeQuery.matches);
    darkModeQuery.addEventListener('change', handleDarkModeChange);

    // Cleanup empty notes on main window load
    if (windowType === 'main') {
      try {
        const deleted = await cleanupEmptyNotes(15);
        if (deleted > 0) {
          console.log(`Cleaned up ${deleted} empty note(s) older than 15 minutes`);
        }
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
  });
</script>

{#if windowType === 'capture'}
  <CaptureWindow />
{:else if windowType === 'note'}
  <NoteWindow />
{:else}
  <MainWindow />
{/if}
