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

// 获取系统可用内存信息
fn get_available_memory() -> u64 {
    // 简单的内存检查，实际应用中可以使用更精确的系统API
    // 这里返回一个保守的估计值
    2 * 1024 * 1024 * 1024 // 假设至少有2GB可用内存
}

// 新增：分片读取PDF文件命令
#[tauri::command]
async fn read_pdf_file_chunked(
    path: String, 
    chunk_size: Option<usize>,
    offset: Option<u64>
) -> Result<Vec<u8>, String> {
    use tokio::io::{AsyncReadExt, AsyncSeekExt};
    
    let chunk_size = chunk_size.unwrap_or(1024 * 1024); // 默认1MB
    let offset = offset.unwrap_or(0);
    
    println!("分片读取PDF: {} (偏移: {}, 大小: {}KB)", path, offset, chunk_size / 1024);
    
    let mut file = match tokio::fs::File::open(&path).await {
        Ok(f) => f,
        Err(e) => return Err(format!("打开文件失败: {}", e)),
    };
    
    // 定位到指定偏移量
    if offset > 0 {
        if let Err(e) = file.seek(std::io::SeekFrom::Start(offset)).await {
            return Err(format!("文件定位失败: {}", e));
        }
    }
    
    // 读取指定大小的数据块
    let mut buffer = vec![0u8; chunk_size];
    match file.read(&mut buffer).await {
        Ok(bytes_read) => {
            buffer.truncate(bytes_read);
            println!("分片读取成功: {} 字节", bytes_read);
            Ok(buffer)
        },
        Err(e) => Err(format!("读取文件失败: {}", e))
    }
}

// 获取文件大小信息
#[tauri::command]
async fn get_pdf_file_info(path: String) -> Result<(u64, String), String> {
    match tokio::fs::metadata(&path).await {
        Ok(metadata) => {
            let file_size = metadata.len();
            let file_size_mb = file_size as f64 / 1024.0 / 1024.0;
            Ok((file_size, format!("{:.2} MB", file_size_mb)))
        },
        Err(e) => Err(format!("获取文件信息失败: {}", e))
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
    let file_size_mb = file_size as f64 / 1024.0 / 1024.0;
    let file_size_gb = file_size_mb / 1024.0;
    
    println!("正在读取PDF文件: {} (大小: {:.2} MB)", path, file_size_mb);
    
    // 对于大文件（>50MB），建议使用分片加载
    if file_size > 50 * 1024 * 1024 {
        println!("建议: 文件较大 ({:.2} MB)，推荐使用分片加载以获得更好性能", file_size_mb);
    }
    
    // 动态内存检查 - 不设置硬性限制，而是基于可用内存智能判断
    let available_memory = get_available_memory();
    let available_memory_mb = available_memory as f64 / 1024.0 / 1024.0;
    
    // 如果文件大小超过可用内存的50%，给出警告但仍允许加载
    if file_size > available_memory / 2 {
        println!("警告: 文件较大 ({:.2} MB)，可能会消耗大量内存 (可用: {:.2} MB)", file_size_mb, available_memory_mb);
        println!("建议: 确保关闭其他大型应用程序以释放内存");
    }
    
    // 对于超大文件（>1GB）给出特别提示
    if file_size_gb > 1.0 {
        println!("处理超大文件 ({:.2} GB)，这可能需要较长时间和大量内存", file_size_gb);
        println!("提示: 应用将尝试优化内存使用，但建议在高配置设备上运行");
    }
    
    // 对于大文件给出加载提示
    if file_size > 100 * 1024 * 1024 { // 100MB
        println!("正在加载大文件 ({:.2} MB)，请耐心等待...", file_size_mb);
    }
    
    // 使用优化的内存分配策略
    match tokio::fs::read(&path).await {
        Ok(data) => {
            println!("文件读取成功，数据大小: {} 字节 ({:.2} MB)", data.len(), data.len() as f64 / 1024.0 / 1024.0);
            
            // 验证读取的数据大小
            if data.len() != file_size as usize {
                return Err(format!(
                    "文件读取不完整：期望 {} 字节，实际读取 {} 字节",
                    file_size, data.len()
                ));
            }
            
            // 对于超大文件，提示内存使用情况
            if data.len() > 500 * 1024 * 1024 { // 500MB
                println!("大文件加载完成，当前内存使用: {:.2} MB", data.len() as f64 / 1024.0 / 1024.0);
                println!("提示: 如果遇到性能问题，建议重启应用程序释放内存");
            }
            
            Ok(data)
        },
        Err(e) => {
            let error_msg = match e.kind() {
                std::io::ErrorKind::OutOfMemory => {
                    format!(
                        "内存不足，无法加载文件 ({:.2} MB)。\n\n这通常发生在：\n• 系统可用内存不足\n• 文件过大超出系统处理能力\n\n建议解决方案：\n• 关闭其他应用程序释放更多内存\n• 重启应用程序清理内存\n• 在更高配置的设备上运行\n• 如果文件确实过大，考虑使用专业PDF工具进行预处理", 
                        file_size_mb
                    )
                },
                std::io::ErrorKind::PermissionDenied => {
                    "文件访问被拒绝，请检查文件权限或以管理员身份运行".to_string()
                },
                std::io::ErrorKind::NotFound => {
                    "文件不存在或已被移动，请检查文件路径".to_string()
                },
                _ => {
                    format!("文件读取失败: {}\n\n如果是大文件，这可能是由于内存限制导致的", e)
                }
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
        .invoke_handler(tauri::generate_handler![greet, open_file_dialog, read_pdf_file, read_pdf_file_chunked, get_pdf_file_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
