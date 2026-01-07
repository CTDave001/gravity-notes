import { invoke } from '@tauri-apps/api/core';
import type { NoteMeta } from './types';

export async function createNote(): Promise<NoteMeta> {
  return invoke('create_note');
}

export async function saveNote(id: string, content: string): Promise<NoteMeta> {
  return invoke('save_note', { id, content });
}

export async function deleteNote(id: string): Promise<void> {
  return invoke('delete_note', { id });
}

export async function getNote(id: string): Promise<string> {
  return invoke('get_note', { id });
}

export async function listNotes(): Promise<NoteMeta[]> {
  return invoke('list_notes');
}

export async function deleteIfEmpty(id: string): Promise<boolean> {
  return invoke('delete_if_empty', { id });
}

export async function cleanupEmptyNotes(maxAgeMinutes: number = 15): Promise<number> {
  return invoke('cleanup_empty_notes', { maxAgeMinutes });
}
