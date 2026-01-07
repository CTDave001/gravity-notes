import { writable } from 'svelte/store';
import { Store } from '@tauri-apps/plugin-store';

export interface Settings {
  theme: 'system' | 'light' | 'dark';
  editorFontSize: number;
  newNoteShortcut: string;
  mainWindowShortcut: string;
}

const defaultSettings: Settings = {
  theme: 'system',
  editorFontSize: 15,
  newNoteShortcut: 'Ctrl+Alt+N',
  mainWindowShortcut: 'Ctrl+Alt+G',
};

export const settings = writable<Settings>(defaultSettings);

let store: Store | null = null;

export async function loadSettings() {
  store = await Store.load('settings.json');
  const saved = await store.get<Settings>('settings');
  if (saved) {
    settings.set({ ...defaultSettings, ...saved });
  }
}

export async function saveSettings(newSettings: Settings) {
  settings.set(newSettings);
  if (store) {
    await store.set('settings', newSettings);
    await store.save();
  }
}
