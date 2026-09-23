//! 书源存储命令模块
//!
//! 书源以 JSON 文件持久化（book_sources.json），支持导入（按 URL 去重合并）、
//! 列表、更新、删除。格式与 Legado 导出的书源 JSON 兼容。

use crate::models::BookSource;
use log::info;
use std::fs;
use std::path::PathBuf;

/// 获取应用数据目录
fn get_data_dir() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("novel_reader");
    path
}

/// 获取书源数据文件路径
fn get_book_sources_path() -> PathBuf {
    let mut path = get_data_dir();
    path.push("book_sources.json");
    path
}

/// 确保数据目录存在
fn ensure_data_dir() -> Result<(), String> {
    let data_dir = get_data_dir();
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).map_err(|e| format!("创建数据目录失败: {}", e))?;
    }
    Ok(())
}

/// 读取全部书源
fn read_all() -> Result<Vec<BookSource>, String> {
    let path = get_book_sources_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let json = fs::read_to_string(&path).map_err(|e| format!("读取书源文件失败: {}", e))?;
    serde_json::from_str(&json).map_err(|e| format!("解析书源文件失败: {}", e))
}

/// 写入全部书源
fn write_all(sources: &[BookSource]) -> Result<(), String> {
    ensure_data_dir()?;
    let json =
        serde_json::to_string_pretty(sources).map_err(|e| format!("序列化书源失败: {}", e))?;
    fs::write(get_book_sources_path(), json).map_err(|e| format!("写入书源文件失败: {}", e))
}

/// 导入书源（按 bookSourceUrl 去重合并，已存在则保留原启停状态、更新其它字段）
///
/// # 返回
/// 新增数量
#[tauri::command]
pub fn import_book_sources(sources: Vec<BookSource>) -> Result<usize, String> {
    info!("导入书源: {} 个", sources.len());

    if sources.is_empty() {
        return Ok(0);
    }

    let mut existing = read_all()?;
    let mut added = 0usize;

    for mut source in sources {
        let url = source.bookSourceUrl.trim().to_string();
        if url.is_empty() {
            continue;
        }
        source.bookSourceUrl = url.clone();

        if source.bookSourceName.trim().is_empty() {
            source.bookSourceName = url.clone();
        }

        match existing.iter_mut().find(|s| s.bookSourceUrl == url) {
            Some(old) => {
                let enabled = old.enabled;
                let custom_order = old.customOrder;
                *old = source;
                old.enabled = enabled;
                old.customOrder = custom_order;
            }
            None => {
                existing.push(source);
                added += 1;
            }
        }
    }

    write_all(&existing)?;
    info!("书源导入完成: 新增 {} 个，共 {} 个", added, existing.len());
    Ok(added)
}

/// 获取全部书源
#[tauri::command]
pub fn list_book_sources() -> Result<Vec<BookSource>, String> {
    let sources = read_all()?;
    info!("读取书源列表: {} 个", sources.len());
    Ok(sources)
}

/// 更新单个书源（整体替换）
#[tauri::command]
pub fn update_book_source(source: BookSource) -> Result<(), String> {
    if source.bookSourceUrl.trim().is_empty() {
        return Err("书源URL不能为空".to_string());
    }

    let url = source.bookSourceUrl.clone();
    let mut sources = read_all()?;
    let name = source.bookSourceName.clone();
    match sources.iter_mut().find(|s| s.bookSourceUrl == url) {
        Some(old) => *old = source,
        None => return Err("书源不存在".to_string()),
    }
    write_all(&sources)?;
    info!("书源已更新: {}", name);
    Ok(())
}

/// 删除书源
#[tauri::command]
pub fn delete_book_source(book_source_url: String) -> Result<(), String> {
    let mut sources = read_all()?;
    let before = sources.len();
    sources.retain(|s| s.bookSourceUrl != book_source_url);
    if sources.len() == before {
        return Err("书源不存在".to_string());
    }
    write_all(&sources)?;
    info!("书源已删除: {}", book_source_url);
    Ok(())
}

/// 切换书源启用状态
#[tauri::command]
pub fn toggle_book_source(book_source_url: String) -> Result<bool, String> {
    let mut sources = read_all()?;
    let target = sources
        .iter_mut()
        .find(|s| s.bookSourceUrl == book_source_url)
        .ok_or_else(|| "书源不存在".to_string())?;
    target.enabled = !target.enabled;
    let new_state = target.enabled;
    write_all(&sources)?;
    info!("书源启用状态切换: {} -> {}", book_source_url, new_state);
    Ok(new_state)
}

/// 清空全部书源
#[tauri::command]
pub fn clear_book_sources() -> Result<(), String> {
    write_all(&[])?;
    info!("已清空全部书源");
    Ok(())
}
