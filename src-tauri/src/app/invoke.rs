use crate::util::{
    check_file_or_append, get_download_message_with_lang, show_toast, MessageType,
};
use std::fs::{self, File};
use std::io::Write;
use std::str::FromStr;
use tauri::http::Method;
use tauri::{command, AppHandle, Manager, Url, WebviewWindow};
use tauri_plugin_http::reqwest::{ClientBuilder, Request};

#[derive(serde::Deserialize)]
pub struct DownloadFileParams {
    url: String,
    filename: String,
    language: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct BinaryDownloadParams {
    filename: String,
    binary: Vec<u8>,
    language: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct NotificationParams {
    title: String,
    body: String,
    icon: String,
}

#[command]
pub async fn download_file(app: AppHandle, params: DownloadFileParams) -> Result<(), String> {
    let window: WebviewWindow = app.get_webview_window("pake").unwrap();
    show_toast(
        &window,
        &get_download_message_with_lang(MessageType::Start, params.language.clone()),
    );

    let output_path = app.path().download_dir().unwrap().join(params.filename);
    let file_path = check_file_or_append(output_path.to_str().unwrap());
    let client = ClientBuilder::new().build().unwrap();

    let response = client
        .execute(Request::new(
            Method::GET,
            Url::from_str(&params.url).unwrap(),
        ))
        .await;

    match response {
        Ok(res) => {
            let bytes = res.bytes().await.unwrap();

            let mut file = File::create(file_path).unwrap();
            file.write_all(&bytes).unwrap();
            show_toast(
                &window,
                &get_download_message_with_lang(MessageType::Success, params.language.clone()),
            );
            Ok(())
        }
        Err(e) => {
            show_toast(
                &window,
                &get_download_message_with_lang(MessageType::Failure, params.language),
            );
            Err(e.to_string())
        }
    }
}

#[command]
pub async fn download_file_by_binary(
    app: AppHandle,
    params: BinaryDownloadParams,
) -> Result<(), String> {
    let window: WebviewWindow = app.get_webview_window("pake").unwrap();
    show_toast(
        &window,
        &get_download_message_with_lang(MessageType::Start, params.language.clone()),
    );
    let output_path = app.path().download_dir().unwrap().join(params.filename);
    let file_path = check_file_or_append(output_path.to_str().unwrap());
    let download_file_result = fs::write(file_path, &params.binary);
    match download_file_result {
        Ok(_) => {
            show_toast(
                &window,
                &get_download_message_with_lang(MessageType::Success, params.language.clone()),
            );
            Ok(())
        }
        Err(e) => {
            show_toast(
                &window,
                &get_download_message_with_lang(MessageType::Failure, params.language),
            );
            Err(e.to_string())
        }
    }
}

#[command]
pub fn send_notification(app: AppHandle, params: NotificationParams) -> Result<(), String> {
    use tauri_plugin_notification::NotificationExt;
    app.notification()
        .builder()
        .title(&params.title)
        .body(&params.body)
        .icon(&params.icon)
        .show()
        .unwrap();
    Ok(())
}

#[command]
pub async fn get_current_url(window: WebviewWindow) -> Result<String, String> {
    // Execute JavaScript to get the current URL
    // We'll use the title trick: temporarily set title to URL, read it, restore it
    let script = r#"
        (function() {
            try {
                const url = window.location.href;
                const oldTitle = document.title;
                document.title = '__PAKE_URL__:' + url;
                setTimeout(() => { document.title = oldTitle; }, 10);
                return url;
            } catch(e) {
                return null;
            }
        })();
    "#;

    window.eval(script).map_err(|e| format!("Failed to execute script: {}", e))?;

    // Give it a moment for the title to update
    tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;

    // Read the title
    if let Ok(title) = window.title() {
        if let Some(url) = title.strip_prefix("__PAKE_URL__:") {
            return Ok(url.to_string());
        }
    }

    Err("Failed to retrieve URL from window".to_string())
}
