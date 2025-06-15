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
    // 获取文件信息用于日志记录
    let metadata = match tokio::fs::metadata(&path).await {
        Ok(meta) => meta,
        Err(e) => return Err(format!("无法获取文件信息: {}", e)),
    };
    
    let file_size = metadata.len();
    println!("正在读取PDF文件: {} (大小: {:.2} MB)", path, file_size as f64 / 1024.0 / 1024.0);
    
    // 对于大文件给出提示但不限制
    if file_size > 100 * 1024 * 1024 { // 100MB
        println!("警告: 文件较大 ({:.2} MB)，加载可能需要较长时间", file_size as f64 / 1024.0 / 1024.0);
    }
    
    match tokio::fs::read(&path).await {
        Ok(data) => {
            println!("文件读取成功，数据大小: {} 字节", data.len());
            Ok(data)
        },
        Err(e) => {
            let error_msg = if e.kind() == std::io::ErrorKind::OutOfMemory {
                format!("内存不足，无法加载文件 ({:.2} MB)。请尝试关闭其他应用程序或使用较小的文件。", file_size as f64 / 1024.0 / 1024.0)
            } else {
                format!("文件读取失败: {}", e)
            };
            Err(error_msg)
        },
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
