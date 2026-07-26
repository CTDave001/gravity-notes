<script lang="ts">
  import { tick } from 'svelte';
  import gravityMark from '../../assets/gravity-mark.png';
  import { focusTrap } from '../focusTrap';

  let {
    show = false,
    mobile = false,
    oncomplete,
  }: {
    show?: boolean;
    mobile?: boolean;
    oncomplete?: () => void;
  } = $props();

  let startButton: HTMLButtonElement | undefined = $state();
  let wasOpen = false;

  $effect(() => {
    if (show && !wasOpen) {
      void tick().then(() => startButton?.focus());
    }
    wasOpen = show;
  });
</script>

{#if show}
  <div class:mobile class="welcome-backdrop">
    <div class="ambient ambient-one" aria-hidden="true"></div>
    <div class="ambient ambient-two" aria-hidden="true"></div>

    <div
      class="welcome-dialog"
      role="dialog"
      aria-modal="true"
      aria-labelledby="welcome-title"
      tabindex="-1"
      use:focusTrap
    >
      <header class="brand">
        <img src={gravityMark} alt="" />
        <div>
          <strong>Gravity</strong>
          <span>Notes without the noise</span>
        </div>
      </header>

      <section class="hero">
        <p class="eyebrow">A quieter place to think</p>
        <h1 id="welcome-title">
          Keep the thought.
          <span>Lose the friction.</span>
        </h1>
        <p class="intro">
          {mobile
            ? 'Capture what matters, the moment it appears. Gravity stays simple, private, and ready—even offline.'
            : 'Capture what matters, the moment it appears. Gravity gives every thought a calm, private home without turning note-taking into a project.'}
        </p>
      </section>

      <section class="values" aria-label="What makes Gravity different">
        <article>
          <span class="value-icon" aria-hidden="true">
            <svg viewBox="0 0 24 24">
              <path d="M12 5v14M5 12h14"></path>
            </svg>
          </span>
          <div>
            <h2>Capture instantly</h2>
            <p>{mobile ? 'One tap to a blank page.' : 'A fresh note is always one shortcut away.'}</p>
          </div>
        </article>

        <article>
          <span class="value-icon" aria-hidden="true">
            <svg viewBox="0 0 24 24">
              <path d="M7 4.75h7.75L19.25 9v10.25H7z"></path>
              <path d="M14.5 4.75V9h4.75M10 13h6M10 16h4"></path>
            </svg>
          </span>
          <div>
            <h2>Yours by default</h2>
            <p>Plain Markdown. Easy to export. Never locked in.</p>
          </div>
        </article>

        <article>
          <span class="value-icon" aria-hidden="true">
            <svg viewBox="0 0 24 24">
              <path d="M12 3.75 19 6.5v5.25c0 4.25-2.65 7.3-7 8.5-4.35-1.2-7-4.25-7-8.5V6.5z"></path>
              <path d="m9 12 2 2 4-4"></path>
            </svg>
          </span>
          <div>
            <h2>Private and offline</h2>
            <p>No account, tracking, or connection required.</p>
          </div>
        </article>
      </section>

      <footer>
        <button bind:this={startButton} onclick={oncomplete}>
          <span>{mobile ? 'Create my first note' : 'Start writing'}</span>
          <svg viewBox="0 0 24 24" aria-hidden="true">
            <path d="M5 12h14M14 7l5 5-5 5"></path>
          </svg>
        </button>
        <p>
          <svg viewBox="0 0 24 24" aria-hidden="true">
            <rect x="6.5" y="10" width="11" height="9" rx="2"></rect>
            <path d="M9 10V7.5a3 3 0 0 1 6 0V10"></path>
          </svg>
          Your notes remain on this device unless you choose to export them.
        </p>
      </footer>
    </div>
  </div>
{/if}

<style>
  .welcome-backdrop {
    --welcome-ink: #111320;
    --welcome-muted: #656778;
    --welcome-soft: #8b8d9c;
    --welcome-line: rgb(24 27 47 / 0.09);
    --welcome-surface: rgb(255 255 255 / 0.84);
    --welcome-accent: #5b5ee8;
    --welcome-accent-hover: #4f51d8;
    position: fixed;
    inset: 0;
    z-index: 70;
    display: grid;
    place-items: center;
    overflow: auto;
    padding: 28px;
    background:
      radial-gradient(circle at 14% 12%, rgb(111 116 255 / 0.11), transparent 29rem),
      radial-gradient(circle at 92% 86%, rgb(169 139 255 / 0.1), transparent 27rem),
      #f7f7fa;
    color: var(--welcome-ink);
    font-family: -apple-system, BlinkMacSystemFont, "SF Pro Display", "Segoe UI", sans-serif;
  }

  .ambient {
    position: fixed;
    border-radius: 999px;
    pointer-events: none;
    filter: blur(2px);
  }

  .ambient-one {
    top: -190px;
    right: -130px;
    width: 460px;
    height: 460px;
    border: 1px solid rgb(91 94 232 / 0.09);
    box-shadow:
      0 0 0 70px rgb(91 94 232 / 0.025),
      0 0 0 150px rgb(91 94 232 / 0.018);
  }

  .ambient-two {
    bottom: -230px;
    left: -170px;
    width: 430px;
    height: 430px;
    border: 1px solid rgb(91 94 232 / 0.07);
    box-shadow: 0 0 0 90px rgb(91 94 232 / 0.02);
  }

  .welcome-dialog {
    position: relative;
    width: min(880px, 100%);
    padding: clamp(34px, 5vw, 64px);
    overflow: hidden;
    background: var(--welcome-surface);
    border: 1px solid rgb(255 255 255 / 0.92);
    border-radius: 32px;
    box-shadow:
      0 36px 100px rgb(42 43 82 / 0.13),
      inset 0 0 0 1px var(--welcome-line);
    backdrop-filter: blur(22px);
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 13px;
  }

  .brand img {
    width: 50px;
    height: 50px;
    border-radius: 14px;
    box-shadow:
      0 10px 24px rgb(22 25 65 / 0.2),
      0 0 0 1px rgb(255 255 255 / 0.18);
  }

  .brand div {
    display: grid;
    gap: 2px;
  }

  .brand strong {
    font-size: 17px;
    font-weight: 700;
    letter-spacing: -0.025em;
  }

  .brand span {
    color: var(--welcome-soft);
    font-size: 12px;
    font-weight: 500;
    letter-spacing: 0.01em;
  }

  .hero {
    max-width: 700px;
    margin-top: clamp(42px, 7vh, 72px);
  }

  .eyebrow {
    margin: 0 0 13px;
    color: var(--welcome-accent);
    font-size: 12px;
    font-weight: 700;
    letter-spacing: 0.115em;
    text-transform: uppercase;
  }

  h1 {
    max-width: 720px;
    margin: 0;
    font-size: clamp(42px, 6.5vw, 68px);
    font-weight: 720;
    letter-spacing: -0.058em;
    line-height: 0.99;
  }

  h1 span {
    display: block;
    color: #666a7d;
    font-weight: 520;
  }

  .intro {
    max-width: 610px;
    margin: 23px 0 0;
    color: var(--welcome-muted);
    font-size: 16px;
    line-height: 1.62;
  }

  .values {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 14px;
    margin-top: clamp(42px, 6vh, 62px);
  }

  .values article {
    min-width: 0;
    padding: 18px;
    background: rgb(248 248 252 / 0.7);
    border: 1px solid var(--welcome-line);
    border-radius: 18px;
  }

  .value-icon {
    display: grid;
    place-items: center;
    width: 34px;
    height: 34px;
    margin-bottom: 17px;
    color: var(--welcome-accent);
    background: rgb(91 94 232 / 0.09);
    border-radius: 10px;
  }

  .value-icon svg {
    width: 18px;
    height: 18px;
    fill: none;
    stroke: currentColor;
    stroke-linecap: round;
    stroke-linejoin: round;
    stroke-width: 1.7;
  }

  .values h2 {
    margin: 0 0 6px;
    font-size: 14px;
    font-weight: 680;
    letter-spacing: -0.018em;
  }

  .values p {
    margin: 0;
    color: var(--welcome-muted);
    font-size: 12px;
    line-height: 1.5;
  }

  footer {
    display: flex;
    align-items: center;
    gap: 24px;
    margin-top: 24px;
  }

  button {
    display: inline-flex;
    flex: 0 0 auto;
    align-items: center;
    justify-content: center;
    gap: 26px;
    min-height: 52px;
    padding: 0 21px 0 24px;
    color: white;
    background: var(--welcome-accent);
    border: 0;
    border-radius: 14px;
    box-shadow: 0 12px 24px rgb(91 94 232 / 0.21);
    cursor: pointer;
    font: 650 14px/1 -apple-system, BlinkMacSystemFont, "SF Pro Text", "Segoe UI", sans-serif;
    letter-spacing: -0.01em;
    transition: background 160ms ease, transform 160ms ease, box-shadow 160ms ease;
  }

  button svg {
    width: 18px;
    height: 18px;
    fill: none;
    stroke: currentColor;
    stroke-linecap: round;
    stroke-linejoin: round;
    stroke-width: 1.8;
  }

  button:hover {
    background: var(--welcome-accent-hover);
    box-shadow: 0 14px 28px rgb(91 94 232 / 0.26);
    transform: translateY(-1px);
  }

  button:active {
    transform: translateY(0);
  }

  button:focus-visible {
    outline: 3px solid rgb(91 94 232 / 0.3);
    outline-offset: 3px;
  }

  footer > p {
    display: flex;
    align-items: center;
    gap: 8px;
    max-width: 330px;
    margin: 0;
    color: var(--welcome-soft);
    font-size: 11px;
    line-height: 1.45;
  }

  footer > p svg {
    flex: 0 0 auto;
    width: 16px;
    height: 16px;
    fill: none;
    stroke: currentColor;
    stroke-linecap: round;
    stroke-linejoin: round;
    stroke-width: 1.6;
  }

  :global(.dark) .welcome-backdrop {
    --welcome-ink: #f5f5fb;
    --welcome-muted: #a8aaba;
    --welcome-soft: #7e8192;
    --welcome-line: rgb(224 226 255 / 0.09);
    --welcome-surface: rgb(17 18 27 / 0.88);
    --welcome-accent: #8588ff;
    --welcome-accent-hover: #9698ff;
    background:
      radial-gradient(circle at 14% 12%, rgb(104 109 255 / 0.13), transparent 29rem),
      radial-gradient(circle at 92% 86%, rgb(155 122 255 / 0.09), transparent 27rem),
      #0b0c12;
  }

  :global(.dark) .welcome-dialog {
    border-color: rgb(255 255 255 / 0.05);
    box-shadow:
      0 36px 110px rgb(0 0 0 / 0.46),
      inset 0 0 0 1px var(--welcome-line);
  }

  :global(.dark) h1 span {
    color: #999cad;
  }

  :global(.dark) .values article {
    background: rgb(255 255 255 / 0.024);
  }

  :global(.dark) .value-icon {
    background: rgb(133 136 255 / 0.11);
  }

  @media (max-width: 720px) {
    .welcome-backdrop {
      display: block;
      padding: 0;
      background:
        radial-gradient(circle at 88% 7%, rgb(111 116 255 / 0.13), transparent 21rem),
        #f9f9fb;
    }

    .welcome-dialog {
      display: flex;
      flex-direction: column;
      width: 100%;
      min-height: 100%;
      padding:
        max(28px, env(safe-area-inset-top))
        max(22px, env(safe-area-inset-right))
        max(22px, env(safe-area-inset-bottom))
        max(22px, env(safe-area-inset-left));
      background: transparent;
      border: 0;
      border-radius: 0;
      box-shadow: none;
      backdrop-filter: none;
    }

    .ambient-one {
      top: -235px;
      right: -260px;
    }

    .ambient-two {
      display: none;
    }

    .brand img {
      width: 46px;
      height: 46px;
      border-radius: 13px;
    }

    .hero {
      margin-top: clamp(40px, 7.5vh, 70px);
    }

    .eyebrow {
      margin-bottom: 14px;
      font-size: 11px;
    }

    h1 {
      font-size: clamp(39px, 11.5vw, 52px);
      line-height: 1.015;
    }

    .intro {
      margin-top: 19px;
      font-size: 15px;
      line-height: 1.58;
    }

    .values {
      grid-template-columns: 1fr;
      gap: 0;
      margin-top: clamp(34px, 5vh, 52px);
      border-top: 1px solid var(--welcome-line);
    }

    .values article {
      display: grid;
      grid-template-columns: 38px 1fr;
      gap: 13px;
      padding: 16px 0;
      background: transparent;
      border: 0;
      border-bottom: 1px solid var(--welcome-line);
      border-radius: 0;
    }

    .value-icon {
      width: 34px;
      height: 34px;
      margin: 0;
    }

    .values h2 {
      font-size: 14px;
    }

    .values p {
      font-size: 12px;
    }

    footer {
      display: grid;
      gap: 13px;
      margin-top: auto;
      padding-top: 26px;
    }

    button {
      width: 100%;
      min-height: 54px;
    }

    footer > p {
      justify-self: center;
      max-width: 310px;
      text-align: center;
    }

    :global(.dark) .welcome-backdrop {
      background:
        radial-gradient(circle at 88% 7%, rgb(104 109 255 / 0.15), transparent 21rem),
        #0b0c12;
    }

    :global(.dark) .values article {
      background: transparent;
    }
  }

  @media (max-width: 380px), (max-height: 730px) {
    .welcome-dialog {
      padding-top: max(20px, env(safe-area-inset-top));
    }

    .brand img {
      width: 40px;
      height: 40px;
      border-radius: 11px;
    }

    .brand strong {
      font-size: 16px;
    }

    .brand span {
      font-size: 11px;
    }

    .hero {
      margin-top: 30px;
    }

    h1 {
      font-size: clamp(34px, 10.5vw, 43px);
    }

    .intro {
      margin-top: 14px;
      font-size: 14px;
    }

    .values {
      margin-top: 24px;
    }

    .values article {
      padding: 12px 0;
    }

    footer {
      padding-top: 18px;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    button {
      transition: none;
    }
  }
</style>
