/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}', './index.html'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        // Warm light mode palette
        warm: {
          bg: '#fffefb',
          sidebar: '#f8f7f4',
          surface: '#f3f2ef',
          border: 'rgba(120, 110, 90, 0.08)',
          text: '#1c1917',
          'text-secondary': '#78716c',
          'text-muted': '#a8a29e',
        },
        // Cool dark mode palette
        cool: {
          bg: '#0c0d10',
          sidebar: '#12131a',
          surface: '#1a1b24',
          border: 'rgba(140, 160, 200, 0.08)',
          text: '#e4e6eb',
          'text-secondary': '#9ca3af',
          'text-muted': '#6b7280',
        },
        surface: {
          50: '#fafafa',
          100: '#f4f4f5',
          200: '#e4e4e7',
          300: '#d4d4d8',
          400: '#a1a1aa',
          500: '#71717a',
          600: '#52525b',
          700: '#3f3f46',
          800: '#27272a',
          900: '#18181b',
          950: '#09090b',
        },
        accent: {
          DEFAULT: '#6366f1',
          hover: '#4f46e5',
          muted: 'rgba(99, 102, 241, 0.15)',
        }
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'Consolas', 'monospace'],
      },
      borderRadius: {
        'window': '10px',
        'card': '8px',
        'button': '6px',
        'input': '6px',
      },
      boxShadow: {
        'sidebar': '4px 0 24px rgba(0, 0, 0, 0.08)',
        'sidebar-dark': '4px 0 24px rgba(0, 0, 20, 0.4)',
        'soft': '0 2px 8px rgba(0, 0, 0, 0.05)',
        'soft-dark': '0 2px 8px rgba(0, 0, 0, 0.3)',
      },
      transitionDuration: {
        'sidebar': '200ms',
        'fast': '150ms',
      }
    },
  },
  plugins: [],
}
