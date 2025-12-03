use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, WebviewWindow};

const LAST_URL_FILE: &str = "last_visited_url.txt";

/// Get the path to the file where we store the last visited URL
fn get_last_url_file_path(app: &AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .expect("Failed to get app data directory")
        .join(LAST_URL_FILE)
}

/// Save the current URL to persistent storage
pub fn save_last_url(app: &AppHandle, url: &str) {
    // Don't save special URLs
    if url.starts_with("about:")
        || url.starts_with("blob:")
        || url.starts_with("data:")
        || url.starts_with("tauri:")
    {
        return;
    }

    let file_path = get_last_url_file_path(app);

    // Create parent directory if it doesn't exist
    if let Some(parent) = file_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    // Write the URL to file
    if let Err(e) = fs::write(&file_path, url) {
        eprintln!("Failed to save last URL: {}", e);
    }

    #[cfg(debug_assertions)]
    println!("Saved last URL: {}", url);
}

/// Load the last visited URL from storage
pub fn load_last_url(app: &AppHandle) -> Option<String> {
    let file_path = get_last_url_file_path(app);

    if !file_path.exists() {
        return None;
    }

    match fs::read_to_string(&file_path) {
        Ok(url) => {
            #[cfg(debug_assertions)]
            println!("Loaded last URL: {}", url);
            Some(url)
        }
        Err(e) => {
            eprintln!("Failed to load last URL: {}", e);
            None
        }
    }
}

/// Restore the last visited URL
pub fn restore_last_url(window: &WebviewWindow, initial_url: &str) {
    let app = window.app_handle();
    if let Some(last_url) = load_last_url(&app) {
        // Only navigate if the last URL is different from the initial URL
        if last_url != initial_url && !last_url.starts_with("about:") {
            #[cfg(debug_assertions)]
            println!("Restoring last URL: {} (initial was: {})", last_url, initial_url);

            let window_clone = window.clone();
            tauri::async_runtime::spawn(async move {
                // Small delay to ensure the window is fully loaded
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                if let Err(e) = window_clone.eval(&format!(
                    "window.location.href = '{}'",
                    last_url.replace('\'', "\\'")
                )) {
                    eprintln!("Failed to restore last URL: {}", e);
                }
            });
        }
    }
}

