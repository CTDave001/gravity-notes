use crate::storage::{
    self, count_words, ensure_notes_dir, extract_preview, extract_title,
    generate_note_filename, is_note_empty, NoteMeta,
};
use std::fs;
use std::time::UNIX_EPOCH;
use tauri::AppHandle;

#[tauri::command]
pub async fn create_note(app: AppHandle) -> Result<NoteMeta, String> {
    let notes_dir = ensure_notes_dir(&app)?;
    let filename = generate_note_filename();
    let path = notes_dir.join(&filename);

    fs::write(&path, "").map_err(|e| e.to_string())?;

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
    let notes_dir = ensure_notes_dir(&app)?;
    let path = notes_dir.join(format!("{}.md", id));

    fs::write(&path, &content).map_err(|e| e.to_string())?;

    let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;
    let _modified = metadata
        .modified()
        .map_err(|e| e.to_string())?
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?;

    Ok(NoteMeta {
        id,
        path: path.to_string_lossy().to_string(),
        title: extract_title(&content),
        preview: extract_preview(&content),
        created_at: String::new(), // Will be filled by frontend
        modified_at: chrono::Local::now().to_rfc3339(),
        word_count: count_words(&content),
        char_count: content.chars().count(),
    })
}

#[tauri::command]
pub async fn delete_note(app: AppHandle, id: String) -> Result<(), String> {
    let notes_dir = storage::get_notes_dir(&app);
    let path = notes_dir.join(format!("{}.md", id));

    if path.exists() {
        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_note(app: AppHandle, id: String) -> Result<String, String> {
    let notes_dir = storage::get_notes_dir(&app);
    let path = notes_dir.join(format!("{}.md", id));

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

        if path.extension().map_or(false, |ext| ext == "md") {
            let content = fs::read_to_string(&path).unwrap_or_default();
            let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;

            let filename = path.file_stem().unwrap().to_string_lossy().to_string();
            let modified = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| {
                    chrono::DateTime::from_timestamp(d.as_secs() as i64, 0)
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_default()
                })
                .unwrap_or_default();

            let created = metadata
                .created()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| {
                    chrono::DateTime::from_timestamp(d.as_secs() as i64, 0)
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_default()
                })
                .unwrap_or_default();

            notes.push(NoteMeta {
                id: filename,
                path: path.to_string_lossy().to_string(),
                title: extract_title(&content),
                preview: extract_preview(&content),
                created_at: created,
                modified_at: modified,
                word_count: count_words(&content),
                char_count: content.chars().count(),
            });
        }
    }

    // Sort by modified date, most recent first
    notes.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));

    Ok(notes)
}

#[tauri::command]
pub async fn delete_if_empty(app: AppHandle, id: String) -> Result<bool, String> {
    let notes_dir = storage::get_notes_dir(&app);
    let path = notes_dir.join(format!("{}.md", id));

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
    let notes_dir = storage::get_notes_dir(&app);
    let mut deleted = 0;

    if !notes_dir.exists() {
        return Ok(0);
    }

    let entries = fs::read_dir(&notes_dir).map_err(|e| e.to_string())?;
    let now = std::time::SystemTime::now();

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "md") {
            let content = fs::read_to_string(&path).unwrap_or_default();

            if is_note_empty(&content) {
                if let Ok(metadata) = fs::metadata(&path) {
                    if let Ok(modified) = metadata.modified() {
                        if let Ok(age) = now.duration_since(modified) {
                            if age.as_secs() > max_age_minutes * 60 {
                                if fs::remove_file(&path).is_ok() {
                                    deleted += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(deleted)
}
