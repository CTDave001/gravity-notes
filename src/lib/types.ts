export interface NoteMeta {
  id: string;
  path: string;
  title: string;
  preview: string;
  created_at: string;
  modified_at: string;
  word_count: number;
  char_count: number;
}

export interface EditorStats {
  wordCount: number;
  charCount: number;
  line: number;
  column: number;
}

export type SaveStatus = 'saved' | 'saving' | 'error';

export type ResizeDirection =
  | 'East'
  | 'North'
  | 'NorthEast'
  | 'NorthWest'
  | 'South'
  | 'SouthEast'
  | 'SouthWest'
  | 'West';
