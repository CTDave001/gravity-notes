<script lang="ts">
  let {
    title = 'Gravity',
    showingList = true,
    onnotes,
    onsettings,
    onnewnote,
  }: {
    title?: string;
    showingList?: boolean;
    onnotes?: () => void;
    onsettings?: () => void;
    onnewnote?: () => void;
  } = $props();
</script>

<header class="mobile-header">
  <div class="leading">
    {#if !showingList}
      <button class="text-action" onclick={onnotes} aria-label="Back to notes">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M15 18l-6-6 6-6" />
        </svg>
        Notes
      </button>
    {:else}
      <span class="brand">Gravity</span>
    {/if}
  </div>

  <p class="title">{showingList ? 'Notes' : title}</p>

  <div class="actions">
    <button class="icon-action" onclick={onsettings} aria-label="Settings">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
        <path stroke-linecap="round" stroke-linejoin="round" d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.592c.55 0 1.02.398 1.11.94l.214 1.286c.063.377.318.69.66.858l.228.118c.337.183.738.24 1.094.1l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.245c.275.476.162 1.08-.273 1.419l-1.028.802c-.3.234-.437.613-.424.994v.408c-.013.381.125.76.424.994l1.028.802c.435.338.548.943.273 1.419l-1.296 2.245a1.125 1.125 0 01-1.37.49l-1.217-.456c-.356-.14-.757-.083-1.094.1l-.228.118c-.342.168-.597.481-.66.858l-.214 1.286c-.09.542-.56.94-1.11.94h-2.592c-.55 0-1.02-.398-1.11-.94l-.214-1.286c-.063-.377-.318-.69-.66-.858l-.228-.118c-.337-.183-.738-.24-1.094-.1l-1.217.456a1.125 1.125 0 01-1.37-.49l-1.296-2.245A1.125 1.125 0 013.788 14l1.028-.802c.3-.234.437-.613.424-.994v-.408c.013-.381-.125-.76-.424-.994L3.788 10a1.125 1.125 0 01-.273-1.419l1.296-2.245a1.125 1.125 0 011.37-.49l1.217.456c.356.14.757.083 1.094-.1l.228-.118c.342-.168.597-.481.66-.858l.214-1.286z"/>
        <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
      </svg>
    </button>
    <button class="icon-action new" onclick={onnewnote} aria-label="New note">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" d="M12 5v14M5 12h14" />
      </svg>
    </button>
  </div>
</header>

<style>
  .mobile-header {
    position: relative;
    display: grid;
    grid-template-columns: minmax(88px, 1fr) auto minmax(88px, 1fr);
    align-items: end;
    min-height: calc(52px + env(safe-area-inset-top));
    padding: env(safe-area-inset-top) 10px 6px;
    background: color-mix(in srgb, var(--bg-primary) 94%, transparent);
    border-bottom: 1px solid var(--border-color);
    backdrop-filter: blur(18px);
    flex-shrink: 0;
    z-index: 30;
  }

  .leading,
  .actions {
    display: flex;
    align-items: center;
    min-width: 0;
  }

  .actions {
    justify-content: flex-end;
    gap: 2px;
  }

  .brand {
    padding-left: 8px;
    color: var(--accent);
    font-size: 15px;
    font-weight: 700;
    letter-spacing: -.02em;
  }

  .title {
    max-width: 42vw;
    margin: 0 6px 11px;
    overflow: hidden;
    color: var(--text-primary);
    font-size: 16px;
    font-weight: 650;
    letter-spacing: -.02em;
    text-align: center;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 44px;
    min-height: 44px;
    padding: 0;
    color: var(--accent);
    background: transparent;
    border: 0;
    border-radius: 12px;
  }

  button:active {
    background: var(--accent-muted);
    transform: scale(.96);
  }

  .text-action {
    justify-content: flex-start;
    gap: 1px;
    padding: 0 8px 0 2px;
    font-size: 16px;
  }

  .text-action svg {
    width: 23px;
    height: 23px;
  }

  .icon-action svg {
    width: 21px;
    height: 21px;
  }

  .icon-action.new {
    min-width: 34px;
    min-height: 34px;
    margin: 5px;
    color: white;
    background: var(--accent);
    border-radius: 50%;
  }

  .icon-action.new svg {
    width: 19px;
    height: 19px;
  }
</style>
