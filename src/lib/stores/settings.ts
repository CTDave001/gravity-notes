import { writable } from 'svelte/store';
import { Store } from '@tauri-apps/plugin-store';

export interface Settings {
  theme: 'system' | 'light' | 'dark';
  editorFontSize: number;
  editorFontFamily: 'sans' | 'mono';
  editorLineHeight: number;
  sidebarDefaultOpen: boolean;
  onboardingComplete: boolean;
}

const defaultSettings: Settings = {
  theme: 'system',
  editorFontSize: 14,
  editorFontFamily: 'sans',
  editorLineHeight: 1.65,
  sidebarDefaultOpen: true,
  onboardingComplete: false,
};

export const settings = writable<Settings>(defaultSettings);

let store: Store | null = null;

function normalizeSettings(saved?: Partial<Settings>): Settings {
  const theme = saved?.theme;
  const editorFontFamily = saved?.editorFontFamily;
  return {
    theme: theme === 'light' || theme === 'dark' || theme === 'system' ? theme : defaultSettings.theme,
    editorFontSize: Math.min(22, Math.max(12, Number(saved?.editorFontSize) || defaultSettings.editorFontSize)),
    editorFontFamily: editorFontFamily === 'mono' || editorFontFamily === 'sans'
      ? editorFontFamily
      : defaultSettings.editorFontFamily,
    editorLineHeight: Math.min(2, Math.max(1.35, Number(saved?.editorLineHeight) || defaultSettings.editorLineHeight)),
    sidebarDefaultOpen: typeof saved?.sidebarDefaultOpen === 'boolean'
      ? saved.sidebarDefaultOpen
      : defaultSettings.sidebarDefaultOpen,
    onboardingComplete: typeof saved?.onboardingComplete === 'boolean'
      ? saved.onboardingComplete
      : defaultSettings.onboardingComplete,
  };
}

export async function loadSettings(): Promise<Settings> {
  try {
    store = await Store.load('settings.json');
    const saved = await store.get<Partial<Settings>>('settings');
    const loaded = normalizeSettings(saved);
    settings.set(loaded);
    return loaded;
  } catch (error) {
    console.error('Failed to load settings; using defaults:', error);
    const fallback = { ...defaultSettings };
    settings.set(fallback);
    return fallback;
  }
}

export async function saveSettings(newSettings: Settings): Promise<void> {
  const normalized = normalizeSettings(newSettings);
  settings.set(normalized);
  store ??= await Store.load('settings.json');
  await store.set('settings', normalized);
  await store.save();
}
