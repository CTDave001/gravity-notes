use crate::storage::atomic_write;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

fn validate_filename(filename: &str) -> Result<&str, String> {
    let trimmed = filename.trim();
    let mut components = Path::new(trimmed).components();
    if trimmed.is_empty()
        || !matches!(components.next(), Some(Component::Normal(_)))
        || components.next().is_some()
        || trimmed.chars().any(char::is_control)
        || trimmed.contains(['/', '\\', ':', '*', '?', '"', '<', '>', '|'])
    {
        return Err("Invalid export filename".to_string());
    }
    Ok(trimmed)
}

fn validate_destination(destination: &Path) -> Result<(), String> {
    if destination.is_dir() {
        Ok(())
    } else {
        Err("Export destination is not a directory".to_string())
    }
}

/// Find a unique filename by adding (1), (2), etc. if file exists
fn get_unique_path(base_path: PathBuf) -> PathBuf {
    if !base_path.exists() {
        return base_path;
    }

    let stem = base_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let ext = base_path
        .extension()
        .map(|e| e.to_string_lossy().to_string());
    let parent = base_path.parent().unwrap_or(&base_path);

    let mut counter = 1;
    loop {
        let new_name = match &ext {
            Some(e) => format!("{} ({}).{}", stem, counter, e),
            None => format!("{} ({})", stem, counter),
        };
        let new_path = parent.join(new_name);
        if !new_path.exists() {
            return new_path;
        }
        counter += 1;
    }
}

#[tauri::command]
pub async fn export_note_file(
    content: String,
    filename: String,
    format: String,
    destination: PathBuf,
) -> Result<String, String> {
    validate_destination(&destination)?;
    let filename = validate_filename(&filename)?;
    let final_content = match format.as_str() {
        "txt" => {
            // Strip markdown formatting (basic)
            content
                .lines()
                .map(|line| line.trim_start_matches('#').trim())
                .collect::<Vec<_>>()
                .join("\n")
        }
        "md" => content,
        _ => return Err("Unsupported export format".to_string()),
    };

    let ext = match format.as_str() {
        "txt" => "txt",
        "md" => "md",
        _ => unreachable!(),
    };

    let base_path = destination.join(format!("{}.{}", filename, ext));
    let output_path = get_unique_path(base_path);
    atomic_write(&output_path, final_content.as_bytes())?;

    Ok(output_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn export_pdf(
    content: Vec<u8>,
    filename: String,
    destination: PathBuf,
) -> Result<String, String> {
    validate_destination(&destination)?;
    let filename = validate_filename(&filename)?;
    let base_path = destination.join(format!("{}.pdf", filename));
    let output_path = get_unique_path(base_path);
    atomic_write(&output_path, &content)?;
    Ok(output_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn get_downloads_dir() -> Result<String, String> {
    dirs::download_dir()
        .map(|p| p.to_string_lossy().to_string())
        .ok_or_else(|| "Could not find downloads directory".to_string())
}

#[tauri::command]
pub async fn reveal_in_folder(path: String) -> Result<(), String> {
    let path = PathBuf::from(&path);
    if !path.is_file() {
        return Err("Cannot reveal a file that does not exist".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .args(["/select,", &path.to_string_lossy()])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .args(["-R", &path.to_string_lossy()])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(path.parent().unwrap_or(&path))
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_filename;

    #[test]
    fn export_filename_rejects_paths_and_invalid_characters() {
        assert!(validate_filename("my note").is_ok());
        assert!(validate_filename("../note").is_err());
        assert!(validate_filename("folder/note").is_err());
        assert!(validate_filename("note?.md").is_err());
        assert!(validate_filename("").is_err());
    }
}
