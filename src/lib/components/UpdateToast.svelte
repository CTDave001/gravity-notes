<script lang="ts">
  let {
    show = false,
    version = '',
    installing = false,
    error = '',
    oninstall,
    ondismiss,
  }: {
    show?: boolean;
    version?: string;
    installing?: boolean;
    error?: string;
    oninstall?: () => void;
    ondismiss?: () => void;
  } = $props();
</script>

{#if show}
  <aside class="update-toast" aria-live="polite" aria-label="Application update">
    <div class="update-mark" aria-hidden="true">↑</div>
    <div class="update-copy">
      <strong>{error ? 'Update failed' : `Gravity ${version} is ready`}</strong>
      <span>{error || (installing ? 'Downloading and verifying…' : 'Install the signed update now.')}</span>
    </div>
    {#if !installing}
      <div class="update-actions">
        <button class="later" onclick={ondismiss}>{error ? 'Close' : 'Later'}</button>
        {#if !error}<button class="install" onclick={oninstall}>Install</button>{/if}
      </div>
    {/if}
  </aside>
{/if}

<style>
  .update-toast {
    position: fixed;
    right: 18px;
    bottom: 18px;
    z-index: 55;
    display: flex;
    align-items: center;
    gap: 11px;
    width: min(420px, calc(100vw - 36px));
    padding: 12px;
    color: var(--text-primary);
    background: color-mix(in srgb, var(--bg-primary) 94%, transparent);
    border: 1px solid var(--border-color);
    border-radius: 12px;
    box-shadow: 0 16px 50px rgb(0 0 0 / .25);
    backdrop-filter: blur(10px);
  }

  .update-mark {
    display: grid;
    place-items: center;
    width: 34px;
    height: 34px;
    flex: 0 0 auto;
    color: white;
    background: var(--accent);
    border-radius: 9px;
    font-weight: 700;
  }

  .update-copy {
    min-width: 0;
    flex: 1;
  }

  strong, span { display: block; }
  strong { font-size: 13px; }
  span { margin-top: 2px; color: var(--text-muted); font-size: 11px; }
  .update-actions { display: flex; gap: 5px; }

  button {
    padding: 7px 9px;
    border: 0;
    border-radius: 7px;
    cursor: pointer;
    font-size: 12px;
    font-weight: 600;
  }

  .later { color: var(--text-secondary); background: var(--bg-surface); }
  .install { color: white; background: var(--accent); }
  button:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
</style>
