//! 书签命令模块（bookmarks.json）

use crate::models::Bookmark;
use crate::utils::data_dir::{ensure_data_dir, get_data_dir};
use log::info;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_bookmarks_path() -> PathBuf {
    get_data_dir().join("bookmarks.json")
}

fn read_all() -> Result<Vec<Bookmark>, String> {
    let path = get_bookmarks_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let json = fs::read_to_string(&path).map_err(|e| format!("读取书签文件失败: {}", e))?;
    serde_json::from_str(&json).map_err(|e| format!("解析书签文件失败: {}", e))
}

fn write_all(bookmarks: &[Bookmark]) -> Result<(), String> {
    ensure_data_dir()?;
    let json =
        serde_json::to_string_pretty(bookmarks).map_err(|e| format!("序列化书签失败: {}", e))?;
    fs::write(get_bookmarks_path(), json).map_err(|e| format!("写入书签文件失败: {}", e))
}

/// 添加书签（同一章节已存在则返回 false）
#[tauri::command]
pub fn add_bookmark(bookmark: Bookmark) -> Result<bool, String> {
    let mut all = read_all()?;
    if all
        .iter()
        .any(|b| b.novel_url == bookmark.novel_url && b.chapter_url == bookmark.chapter_url)
    {
        return Ok(false);
    }
    let mut bm = bookmark;
    if bm.created_at <= 0 {
        bm.created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
    }
    all.push(bm);
    write_all(&all)?;
    info!("书签已添加");
    Ok(true)
}

/// 获取某本书的全部书签
#[tauri::command]
pub fn list_bookmarks(novel_url: String) -> Result<Vec<Bookmark>, String> {
    let all = read_all()?;
    Ok(all
        .into_iter()
        .filter(|b| b.novel_url == novel_url)
        .collect())
}

/// 删除书签
#[tauri::command]
pub fn remove_bookmark(novel_url: String, chapter_url: String) -> Result<(), String> {
    let mut all = read_all()?;
    let before = all.len();
    all.retain(|b| !(b.novel_url == novel_url && b.chapter_url == chapter_url));
    if all.len() == before {
        return Err("书签不存在".to_string());
    }
    write_all(&all)?;
    Ok(())
}
