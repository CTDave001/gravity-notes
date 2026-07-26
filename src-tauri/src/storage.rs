use chrono::Local;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
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

pub fn get_notes_dir(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map(|path| path.join("notes"))
        .map_err(|error| format!("Could not resolve the app data directory: {error}"))
}

pub fn ensure_notes_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let notes_dir = get_notes_dir(app)?;
    if !notes_dir.exists() {
        fs::create_dir_all(&notes_dir).map_err(|e| e.to_string())?;
    }
    Ok(notes_dir)
}

pub fn validate_note_id(id: &str) -> Result<(), String> {
    let path = Path::new(id);
    let mut components = path.components();
    let is_single_filename = matches!(components.next(), Some(Component::Normal(_)))
        && components.next().is_none()
        && path.file_name() == Some(OsStr::new(id));

    if id.is_empty() || id.contains(['/', '\\']) || !is_single_filename || id == "." || id == ".." {
        return Err("Invalid note ID".to_string());
    }

    Ok(())
}

pub fn get_note_path(app: &AppHandle, id: &str) -> Result<PathBuf, String> {
    validate_note_id(id)?;
    Ok(get_notes_dir(app)?.join(format!("{id}.md")))
}

pub fn atomic_write(path: &Path, content: &[u8]) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| "Cannot write a file without a parent directory".to_string())?;
    fs::create_dir_all(parent).map_err(|e| e.to_string())?;

    let file_name = path
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or_else(|| "Invalid output filename".to_string())?;
    let temp_name = format!(
        ".{file_name}.tmp-{}-{}",
        std::process::id(),
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
    );
    let temp_path = parent.join(temp_name);

    let write_result = (|| -> Result<(), String> {
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&temp_path)
            .map_err(|e| e.to_string())?;
        file.write_all(content).map_err(|e| e.to_string())?;
        file.sync_all().map_err(|e| e.to_string())?;
        fs::rename(&temp_path, path).map_err(|e| e.to_string())
    })();

    if write_result.is_err() {
        let _ = fs::remove_file(&temp_path);
    }

    write_result
}

pub fn generate_note_filename() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d_%H-%M-%S-%9f.md").to_string()
}

pub fn extract_title(content: &str) -> String {
    content
        .lines()
        .find(|line| !line.trim().is_empty())
        .map(|line| {
            let trimmed = line.trim().trim_start_matches('#').trim();
            let char_count: usize = trimmed.chars().count();
            if char_count > 50 {
                let truncated: String = trimmed.chars().take(47).collect();
                format!("{}...", truncated)
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

    let char_count: usize = preview.chars().count();
    if char_count > 800 {
        let truncated: String = preview.chars().take(797).collect();
        format!("{}...", truncated)
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

pub fn get_images_dir(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map(|path| path.join("images"))
        .map_err(|error| format!("Could not resolve the app data directory: {error}"))
}

pub fn ensure_images_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let images_dir = get_images_dir(app)?;
    if !images_dir.exists() {
        fs::create_dir_all(&images_dir).map_err(|e| e.to_string())?;
    }
    Ok(images_dir)
}

pub fn generate_image_filename(extension: &str) -> String {
    let now = Local::now();
    now.format(&format!("%Y-%m-%d_%H-%M-%S-%9f.{}", extension))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_and_preview_truncate_unicode_without_panicking() {
        let title = "🪐".repeat(60);
        let content = format!("# {title}\n\n{}", "界".repeat(900));

        let extracted_title = extract_title(&content);
        let extracted_preview = extract_preview(&content);

        assert_eq!(extracted_title.chars().count(), 50);
        assert_eq!(extracted_preview.chars().count(), 800);
        assert!(extracted_title.ends_with("..."));
        assert!(extracted_preview.ends_with("..."));
    }

    #[test]
    fn note_ids_must_be_single_file_stems() {
        for valid in [
            "2026-01-01_12-30-00-000",
            "A note with spaces",
            "notes & ideas",
        ] {
            assert!(validate_note_id(valid).is_ok(), "{valid}");
        }

        for invalid in [
            "",
            ".",
            "..",
            "../outside",
            r"..\outside",
            "/tmp/outside",
            r"C:\tmp\outside",
        ] {
            assert!(validate_note_id(invalid).is_err(), "{invalid}");
        }
    }

    #[test]
    fn atomic_write_replaces_complete_contents() {
        let path = std::env::temp_dir().join(format!(
            "gravity-atomic-write-test-{}-{}.md",
            std::process::id(),
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
        ));

        atomic_write(&path, b"first").unwrap();
        atomic_write(&path, b"second").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "second");

        let _ = fs::remove_file(path);
    }
}
