// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::sync::mpsc;
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn open_file_dialog(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let (tx, rx) = mpsc::channel();
    
    app.dialog()
        .file()
        .add_filter("PDF Files", &["pdf"])
        .pick_file(move |file_path| {
            let _ = tx.send(file_path);
        });
    
    match rx.recv() {
        Ok(Some(path)) => Ok(Some(path.to_string())),
        Ok(None) => Ok(None),
        Err(_) => Err("Failed to receive file path".to_string()),
    }
}

#[tauri::command]
async fn read_pdf_file(path: String) -> Result<Vec<u8>, String> {
    match tokio::fs::read(&path).await {
        Ok(data) => Ok(data),
        Err(e) => Err(format!("Failed to read file: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![greet, open_file_dialog, read_pdf_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
