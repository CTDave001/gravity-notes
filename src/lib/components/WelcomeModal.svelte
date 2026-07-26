<script lang="ts">
  import { tick } from 'svelte';
  import { focusTrap } from '../focusTrap';

  let {
    show = false,
    oncomplete,
  }: {
    show?: boolean;
    oncomplete?: () => void;
  } = $props();

  let startButton: HTMLButtonElement | undefined = $state();
  let wasOpen = false;
  let isMac = $state(false);

  $effect(() => {
    if (show && !wasOpen) {
      isMac = navigator.userAgent.toUpperCase().includes('MAC');
      void tick().then(() => startButton?.focus());
    }
    wasOpen = show;
  });
</script>

{#if show}
  <div class="welcome-backdrop">
    <div class="welcome-dialog" role="dialog" aria-modal="true" aria-labelledby="welcome-title" tabindex="-1" use:focusTrap>
      <div class="mark" aria-hidden="true">G</div>
      <p class="eyebrow">Welcome to Gravity</p>
      <h1 id="welcome-title">Capture first. Organize later.</h1>
      <p class="intro">Your notes are plain Markdown files stored locally, with quick capture available from anywhere.</p>
      <div class="steps">
        <div><kbd>{isMac ? '⌘' : 'Ctrl'}</kbd><span>+</span><kbd>{isMac ? '⌥' : 'Alt'}</kbd><span>+</span><kbd>N</kbd><p>Open quick capture</p></div>
        <div><kbd>{isMac ? '⌘' : 'Ctrl'}</kbd><span>+</span><kbd>{isMac ? '⌥' : 'Alt'}</kbd><span>+</span><kbd>G</kbd><p>Bring Gravity forward</p></div>
        <div><kbd>F1</kbd><p>See Markdown and editor shortcuts</p></div>
      </div>
      <button bind:this={startButton} onclick={oncomplete}>Start writing</button>
      <small>Gravity lives in the system tray when its windows are closed.</small>
    </div>
  </div>
{/if}

<style>
  .welcome-backdrop {
    position: fixed;
    inset: 0;
    z-index: 70;
    display: grid;
    place-items: center;
    padding: 20px;
    background: rgb(5 6 10 / .7);
    backdrop-filter: blur(10px);
  }

  .welcome-dialog {
    width: min(520px, 100%);
    padding: 36px;
    text-align: center;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 20px;
    box-shadow: 0 30px 100px rgb(0 0 0 / .45);
  }

  .mark {
    display: grid;
    place-items: center;
    width: 48px;
    height: 48px;
    margin: 0 auto 18px;
    color: white;
    background: linear-gradient(145deg, #818cf8, #4f46e5);
    border-radius: 14px;
    box-shadow: 0 8px 28px rgb(99 102 241 / .35);
    font-size: 21px;
    font-weight: 700;
  }

  .eyebrow {
    margin: 0 0 7px;
    color: var(--accent);
    font-size: 11px;
    font-weight: 700;
    letter-spacing: .1em;
    text-transform: uppercase;
  }

  h1 {
    margin: 0;
    font-size: clamp(24px, 5vw, 32px);
    letter-spacing: -.035em;
  }

  .intro {
    max-width: 410px;
    margin: 12px auto 26px;
    color: var(--text-secondary);
    font-size: 14px;
    line-height: 1.55;
  }

  .steps {
    display: grid;
    gap: 8px;
    margin-bottom: 24px;
    text-align: left;
  }

  .steps > div {
    display: flex;
    align-items: center;
    gap: 5px;
    min-height: 42px;
    padding: 6px 10px;
    background: var(--bg-sidebar);
    border: 1px solid var(--border-color);
    border-radius: 9px;
  }

  .steps p {
    flex: 1;
    margin: 0 0 0 9px;
    color: var(--text-secondary);
    font-size: 13px;
  }

  .steps span {
    color: var(--text-muted);
    font-size: 10px;
  }

  kbd {
    padding: 4px 7px;
    color: var(--text-primary);
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 5px;
    box-shadow: 0 1px 0 var(--border-color);
    font: 11px var(--editor-font-family);
  }

  button {
    width: 100%;
    padding: 11px 18px;
    color: white;
    background: var(--accent);
    border: 0;
    border-radius: 9px;
    cursor: pointer;
    font-weight: 600;
  }

  button:hover { filter: brightness(1.08); }
  button:focus-visible { outline: 2px solid var(--accent); outline-offset: 3px; }

  small {
    display: block;
    margin-top: 13px;
    color: var(--text-muted);
    font-size: 11px;
  }

  @media (max-width: 520px) {
    .welcome-dialog { padding: 28px 20px; }
  }
</style>
