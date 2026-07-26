use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RuntimeInfo {
    pub platform: &'static str,
    pub mobile: bool,
}

#[tauri::command]
pub fn runtime_info() -> RuntimeInfo {
    RuntimeInfo {
        platform: std::env::consts::OS,
        mobile: cfg!(mobile),
    }
}
