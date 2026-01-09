use arboard::Clipboard;
use html2md::parse_html;

#[cfg(target_os = "windows")]
use std::ffi::OsStr;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;

/// Read HTML from Windows clipboard
#[cfg(target_os = "windows")]
fn get_clipboard_html() -> Option<String> {
    use windows::Win32::System::DataExchange::*;
    use windows::Win32::System::Memory::*;
    use windows::Win32::Foundation::*;
    use windows::core::*;

    unsafe {
        if OpenClipboard(None).is_err() {
            return None;
        }

        // Register HTML format
        let format_name: Vec<u16> = OsStr::new("HTML Format")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let html_format = RegisterClipboardFormatW(PCWSTR(format_name.as_ptr()));

        let result = if html_format != 0 {
            let handle = GetClipboardData(html_format);
            if let Ok(h) = handle {
                let ptr = GlobalLock(HGLOBAL(h.0)) as *const u8;
                if !ptr.is_null() {
                    let size = GlobalSize(HGLOBAL(h.0));
                    let slice = std::slice::from_raw_parts(ptr, size);
                    // HTML Format has a header, find the actual HTML
                    let content = String::from_utf8_lossy(slice);
                    let html = extract_html_from_cf_html(&content);
                    let _ = GlobalUnlock(HGLOBAL(h.0));
                    html
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        let _ = CloseClipboard();
        result
    }
}

#[cfg(not(target_os = "windows"))]
fn get_clipboard_html() -> Option<String> {
    None
}

/// Extract actual HTML from CF_HTML format (which has headers)
fn extract_html_from_cf_html(cf_html: &str) -> Option<String> {
    // CF_HTML format has headers like:
    // Version:0.9
    // StartHTML:0000000105
    // EndHTML:0000000253
    // StartFragment:0000000141
    // EndFragment:0000000217
    // <html>...</html>

    // Try to find StartFragment/EndFragment for selection
    let start_frag = cf_html.find("StartFragment:")
        .and_then(|i| {
            let rest = &cf_html[i + 14..];
            rest.lines().next()?.trim().parse::<usize>().ok()
        });
    let end_frag = cf_html.find("EndFragment:")
        .and_then(|i| {
            let rest = &cf_html[i + 12..];
            rest.lines().next()?.trim().parse::<usize>().ok()
        });

    if let (Some(start), Some(end)) = (start_frag, end_frag) {
        if end > start && end <= cf_html.len() {
            return Some(cf_html[start..end].to_string());
        }
    }

    // Fallback: find <html> or first <
    if let Some(start) = cf_html.find("<html") {
        return Some(cf_html[start..].to_string());
    }
    if let Some(start) = cf_html.find('<') {
        return Some(cf_html[start..].to_string());
    }

    None
}

/// Read HTML from clipboard, convert to Markdown, and write back to clipboard
#[tauri::command]
pub fn clip_to_markdown() -> Result<String, String> {
    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;

    // Try to read HTML from clipboard first
    let html = get_clipboard_html();

    // Also get plain text as fallback
    let plain_text = clipboard.get_text().ok();

    let markdown = if let Some(html_content) = html {
        if !html_content.trim().is_empty() {
            // Convert HTML to Markdown
            let md = parse_html(&html_content);
            clean_markdown(&md)
        } else if let Some(text) = plain_text {
            // Fallback to plain text
            text
        } else {
            return Err("Clipboard is empty".to_string());
        }
    } else if let Some(text) = plain_text {
        // No HTML, use plain text
        text
    } else {
        return Err("Clipboard is empty".to_string());
    };

    // Write markdown back to clipboard
    clipboard
        .set_text(&markdown)
        .map_err(|e| e.to_string())?;

    Ok(markdown)
}

/// Clean up the converted markdown
fn clean_markdown(md: &str) -> String {
    let mut result = md.to_string();

    // Remove excessive blank lines (more than 2 consecutive)
    while result.contains("\n\n\n") {
        result = result.replace("\n\n\n", "\n\n");
    }

    // Trim leading/trailing whitespace from each line while preserving structure
    let lines: Vec<&str> = result.lines().collect();
    let cleaned: Vec<String> = lines
        .iter()
        .map(|line| {
            // Preserve indentation for code blocks and lists
            if line.starts_with("    ") || line.starts_with("\t") || line.starts_with("- ") || line.starts_with("* ") || line.starts_with("> ") {
                line.trim_end().to_string()
            } else {
                line.trim().to_string()
            }
        })
        .collect();

    result = cleaned.join("\n");

    // Trim overall
    result.trim().to_string()
}
