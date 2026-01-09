use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Find a unique filename by adding (1), (2), etc. if file exists
fn get_unique_path(base_path: PathBuf) -> PathBuf {
    if !base_path.exists() {
        return base_path;
    }

    let stem = base_path.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let ext = base_path.extension().map(|e| e.to_string_lossy().to_string());
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
    let final_content = match format.as_str() {
        "txt" => {
            // Strip markdown formatting (basic)
            content
                .lines()
                .map(|line| line.trim_start_matches('#').trim())
                .collect::<Vec<_>>()
                .join("\n")
        }
        _ => content, // md keeps as-is
    };

    let ext = match format.as_str() {
        "txt" => "txt",
        "pdf" => "pdf",
        _ => "md",
    };

    let base_path = destination.join(format!("{}.{}", filename, ext));
    let output_path = get_unique_path(base_path);
    fs::write(&output_path, final_content).map_err(|e| e.to_string())?;

    Ok(output_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn export_pdf(
    content: Vec<u8>,
    filename: String,
    destination: PathBuf,
) -> Result<String, String> {
    let base_path = destination.join(format!("{}.pdf", filename));
    let output_path = get_unique_path(base_path);
    fs::write(&output_path, content).map_err(|e| e.to_string())?;
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
