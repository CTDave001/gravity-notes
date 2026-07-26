use crate::storage::{
    self, atomic_write, count_words, ensure_images_dir, ensure_notes_dir, extract_preview,
    extract_title, generate_image_filename, generate_note_filename, get_images_dir, get_note_path,
    is_note_empty, NoteMeta,
};
use std::fs::{self, OpenOptions};
use std::path::Path;
use std::time::UNIX_EPOCH;
use tauri::AppHandle;

#[tauri::command]
pub async fn create_note(app: AppHandle) -> Result<NoteMeta, String> {
    let notes_dir = ensure_notes_dir(&app)?;
    let (filename, path) = (0..100)
        .find_map(|_| {
            let filename = generate_note_filename();
            let path = notes_dir.join(&filename);
            match OpenOptions::new().create_new(true).write(true).open(&path) {
                Ok(_) => Some(Ok((filename, path))),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => None,
                Err(error) => Some(Err(error.to_string())),
            }
        })
        .transpose()?
        .ok_or_else(|| "Could not allocate a unique note filename".to_string())?;

    let id = filename.trim_end_matches(".md").to_string();
    let now = chrono::Local::now().to_rfc3339();

    Ok(NoteMeta {
        id,
        path: path.to_string_lossy().to_string(),
        title: "Untitled".to_string(),
        preview: String::new(),
        created_at: now.clone(),
        modified_at: now,
        word_count: 0,
        char_count: 0,
    })
}

#[tauri::command]
pub async fn save_note(app: AppHandle, id: String, content: String) -> Result<NoteMeta, String> {
    ensure_notes_dir(&app)?;
    let path = get_note_path(&app, &id)?;

    atomic_write(&path, content.as_bytes())?;

    let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;
    let created_at = system_time_to_rfc3339(metadata.created().ok());
    let modified_at = system_time_to_rfc3339(metadata.modified().ok());

    Ok(NoteMeta {
        id,
        path: path.to_string_lossy().to_string(),
        title: extract_title(&content),
        preview: extract_preview(&content),
        created_at,
        modified_at,
        word_count: count_words(&content),
        char_count: content.chars().count(),
    })
}

#[tauri::command]
pub async fn delete_note(app: AppHandle, id: String) -> Result<(), String> {
    let path = get_note_path(&app, &id)?;

    if path.exists() {
        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_note(app: AppHandle, id: String) -> Result<String, String> {
    let path = get_note_path(&app, &id)?;

    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_notes(app: AppHandle) -> Result<Vec<NoteMeta>, String> {
    let notes_dir = ensure_notes_dir(&app)?;
    let mut notes = Vec::new();

    let entries = fs::read_dir(&notes_dir).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.extension().is_some_and(|ext| ext == "md") {
            match note_meta_from_path(&path) {
                Ok(note) => notes.push(note),
                Err(error) => log::warn!("Skipping unreadable note {}: {}", path.display(), error),
            }
        }
    }

    // Sort by modified date, most recent first
    notes.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));

    Ok(notes)
}

#[tauri::command]
pub async fn delete_if_empty(app: AppHandle, id: String) -> Result<bool, String> {
    let path = get_note_path(&app, &id)?;

    if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        if is_note_empty(&content) {
            fs::remove_file(&path).map_err(|e| e.to_string())?;
            return Ok(true);
        }
    }
    Ok(false)
}

#[tauri::command]
pub async fn cleanup_empty_notes(app: AppHandle, max_age_minutes: u64) -> Result<u32, String> {
    let notes_dir = storage::get_notes_dir(&app)?;
    let mut deleted = 0;

    if !notes_dir.exists() {
        return Ok(0);
    }

    let entries = fs::read_dir(&notes_dir).map_err(|e| e.to_string())?;
    let now = std::time::SystemTime::now();

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.extension().is_some_and(|ext| ext == "md") {
            let content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(error) => {
                    log::warn!(
                        "Skipping cleanup for unreadable note {}: {}",
                        path.display(),
                        error
                    );
                    continue;
                }
            };

            if is_note_empty(&content) {
                if let Ok(metadata) = fs::metadata(&path) {
                    if let Ok(modified) = metadata.modified() {
                        if let Ok(age) = now.duration_since(modified) {
                            if age.as_secs() > max_age_minutes.saturating_mul(60)
                                && fs::remove_file(&path).is_ok()
                            {
                                deleted += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(deleted)
}

#[tauri::command]
pub async fn save_image(
    app: AppHandle,
    data: Vec<u8>,
    extension: String,
) -> Result<String, String> {
    if data.len() > 25 * 1024 * 1024 {
        return Err("Images must be 25 MB or smaller".to_string());
    }
    save_image_data(&app, &data, &extension)
}

#[tauri::command]
pub async fn import_image(app: AppHandle, path: String) -> Result<String, String> {
    let source = Path::new(&path);
    let extension = source
        .extension()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "Image has no valid extension".to_string())?;
    validate_image_extension(extension)?;

    let metadata = fs::metadata(source).map_err(|error| error.to_string())?;
    if !metadata.is_file() {
        return Err("Image path is not a file".to_string());
    }
    if metadata.len() > 25 * 1024 * 1024 {
        return Err("Images must be 25 MB or smaller".to_string());
    }

    let data = fs::read(source).map_err(|error| error.to_string())?;
    save_image_data(&app, &data, extension)
}

fn validate_image_extension(extension: &str) -> Result<String, String> {
    let extension = extension.to_ascii_lowercase();
    if matches!(
        extension.as_str(),
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg"
    ) {
        Ok(extension)
    } else {
        Err("Unsupported image extension".to_string())
    }
}

fn save_image_data(app: &AppHandle, data: &[u8], extension: &str) -> Result<String, String> {
    let images_dir = ensure_images_dir(app)?;
    let extension = validate_image_extension(extension)?;
    let filename = generate_image_filename(&extension);
    let path = images_dir.join(&filename);

    atomic_write(&path, data)?;

    // Return the filename (not full path) for use in markdown
    Ok(filename)
}

#[tauri::command]
pub async fn get_images_path(app: AppHandle) -> Result<String, String> {
    let images_dir = get_images_dir(&app)?;
    Ok(images_dir.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn get_notes_path(app: AppHandle) -> Result<String, String> {
    let notes_dir = ensure_notes_dir(&app)?;
    Ok(notes_dir.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn search_notes(app: AppHandle, query: String) -> Result<Vec<NoteMeta>, String> {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return list_notes(app).await;
    }

    let notes_dir = ensure_notes_dir(&app)?;
    let mut matches = Vec::new();
    for entry in fs::read_dir(notes_dir).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if !path.extension().is_some_and(|ext| ext == "md") {
            continue;
        }

        match fs::read_to_string(&path) {
            Ok(content) if content.to_lowercase().contains(&query) => {
                match note_meta_from_content(&path, &content) {
                    Ok(mut note) => {
                        if let Some(matching_line) = content
                            .lines()
                            .find(|line| line.to_lowercase().contains(&query))
                        {
                            let matching_line = matching_line.trim();
                            note.preview = if matching_line.chars().count() > 180 {
                                format!("{}…", matching_line.chars().take(179).collect::<String>())
                            } else {
                                matching_line.to_string()
                            };
                        }
                        matches.push(note);
                    }
                    Err(error) => {
                        log::warn!("Skipping unreadable note {}: {}", path.display(), error)
                    }
                }
            }
            Ok(_) => {}
            Err(error) => {
                log::warn!("Skipping unreadable note {}: {}", path.display(), error)
            }
        }
    }

    matches.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
    Ok(matches)
}

fn note_meta_from_path(path: &std::path::Path) -> Result<NoteMeta, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    note_meta_from_content(path, &content)
}

fn note_meta_from_content(path: &std::path::Path, content: &str) -> Result<NoteMeta, String> {
    let metadata = fs::metadata(path).map_err(|e| e.to_string())?;
    let id = path
        .file_stem()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "Invalid note filename".to_string())?
        .to_string();

    Ok(NoteMeta {
        id,
        path: path.to_string_lossy().to_string(),
        title: extract_title(content),
        preview: extract_preview(content),
        created_at: system_time_to_rfc3339(metadata.created().ok()),
        modified_at: system_time_to_rfc3339(metadata.modified().ok()),
        word_count: count_words(content),
        char_count: content.chars().count(),
    })
}

fn system_time_to_rfc3339(time: Option<std::time::SystemTime>) -> String {
    time.and_then(|value| value.duration_since(UNIX_EPOCH).ok())
        .and_then(|duration| {
            chrono::DateTime::from_timestamp(duration.as_secs() as i64, duration.subsec_nanos())
        })
        .map(|date| date.to_rfc3339())
        .unwrap_or_default()
}
