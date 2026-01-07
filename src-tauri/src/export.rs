use std::fs;
use std::path::PathBuf;

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
        _ => "md",
    };

    let output_path = destination.join(format!("{}.{}", filename, ext));
    fs::write(&output_path, final_content).map_err(|e| e.to_string())?;

    Ok(output_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn get_downloads_dir() -> Result<String, String> {
    dirs::download_dir()
        .map(|p| p.to_string_lossy().to_string())
        .ok_or_else(|| "Could not find downloads directory".to_string())
}
