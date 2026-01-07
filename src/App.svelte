<script lang="ts">
  import './app.css';
  import CaptureWindow from './lib/windows/CaptureWindow.svelte';

  // For now, default to capture window
  // Later we'll use URL params to determine window type
  let windowType: 'capture' | 'main' = 'capture';
</script>

{#if windowType === 'capture'}
  <CaptureWindow />
{:else}
  <div>Main window coming soon...</div>
{/if}
