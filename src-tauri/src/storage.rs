use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteMeta {
    pub id: String,
    pub path: String,
    pub title: String,
    pub preview: String,
    pub created_at: String,
    pub modified_at: String,
    pub word_count: usize,
    pub char_count: usize,
}

pub fn get_notes_dir(app: &AppHandle) -> PathBuf {
    let app_data = app.path().app_data_dir().expect("Failed to get app data dir");
    app_data.join("notes")
}

pub fn ensure_notes_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let notes_dir = get_notes_dir(app);
    if !notes_dir.exists() {
        fs::create_dir_all(&notes_dir).map_err(|e| e.to_string())?;
    }
    Ok(notes_dir)
}

pub fn generate_note_filename() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d_%H-%M-%S-%3f.md").to_string()
}

pub fn extract_title(content: &str) -> String {
    content
        .lines()
        .find(|line| !line.trim().is_empty())
        .map(|line| {
            let trimmed = line.trim().trim_start_matches('#').trim();
            if trimmed.len() > 50 {
                format!("{}...", &trimmed[..47])
            } else {
                trimmed.to_string()
            }
        })
        .unwrap_or_else(|| "Untitled".to_string())
}

pub fn extract_preview(content: &str) -> String {
    let preview: String = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .take(15)
        .collect::<Vec<_>>()
        .join("\n");

    if preview.len() > 800 {
        format!("{}...", &preview[..797])
    } else {
        preview
    }
}

pub fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}

pub fn is_note_empty(content: &str) -> bool {
    content.trim().is_empty()
}

pub fn get_images_dir(app: &AppHandle) -> PathBuf {
    let app_data = app.path().app_data_dir().expect("Failed to get app data dir");
    app_data.join("images")
}

pub fn ensure_images_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let images_dir = get_images_dir(app);
    if !images_dir.exists() {
        fs::create_dir_all(&images_dir).map_err(|e| e.to_string())?;
    }
    Ok(images_dir)
}

pub fn generate_image_filename(extension: &str) -> String {
    let now = Local::now();
    now.format(&format!("%Y-%m-%d_%H-%M-%S-%3f.{}", extension)).to_string()
}
