//! 存储命令模块
//!
//! 提供本地数据存储功能的Tauri命令

use crate::models::{BookshelfNovel, ReadingProgress};
use log::info;
use std::fs;
use std::path::PathBuf;

/// 获取应用数据目录
fn get_data_dir() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("novel_reader");
    path
}

/// 获取书架数据文件路径
fn get_bookshelf_path() -> PathBuf {
    let mut path = get_data_dir();
    path.push("bookshelf.json");
    path
}

/// 获取阅读进度文件路径
fn get_progress_path() -> PathBuf {
    let mut path = get_data_dir();
    path.push("reading_progress.json");
    path
}

/// 确保数据目录存在
fn ensure_data_dir() -> Result<(), String> {
    let data_dir = get_data_dir();
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .map_err(|e| format!("创建数据目录失败: {}", e))?;
    }
    Ok(())
}

/// 保存书架数据
///
/// # 参数
/// * `novels` - 书架中的小说列表
#[tauri::command]
pub fn save_bookshelf(novels: Vec<BookshelfNovel>) -> Result<(), String> {
    info!("保存书架数据: {} 本小说", novels.len());

    ensure_data_dir()?;

    let path = get_bookshelf_path();
    let json = serde_json::to_string_pretty(&novels)
        .map_err(|e| format!("序列化失败: {}", e))?;

    fs::write(&path, json)
        .map_err(|e| format!("写入文件失败: {}", e))?;

    info!("书架数据保存成功: {:?}", path);
    Ok(())
}

/// 获取书架数据
#[tauri::command]
pub fn get_bookshelf() -> Result<Vec<BookshelfNovel>, String> {
    info!("读取书架数据");

    let path = get_bookshelf_path();

    if !path.exists() {
        info!("书架文件不存在，返回空列表");
        return Ok(Vec::new());
    }

    let json = fs::read_to_string(&path)
        .map_err(|e| format!("读取文件失败: {}", e))?;

    let novels: Vec<BookshelfNovel> = serde_json::from_str(&json)
        .map_err(|e| format!("解析JSON失败: {}", e))?;

    info!("书架数据读取成功: {} 本小说", novels.len());
    Ok(novels)
}

/// 添加小说到书架
///
/// # 参数
/// * `novel` - 要添加的小说
#[tauri::command]
pub fn add_to_bookshelf(novel: BookshelfNovel) -> Result<(), String> {
    info!("添加小说到书架: {}", novel.title);

    let mut novels = get_bookshelf().unwrap_or_default();

    // 检查是否已存在
    if novels.iter().any(|n| n.url == novel.url) {
        return Err(String::from("该小说已在书架中"));
    }

    novels.push(novel);
    save_bookshelf(novels)?;

    info!("小说添加成功");
    Ok(())
}

/// 从书架移除小说
///
/// # 参数
/// * `novel_url` - 小说URL
#[tauri::command]
pub fn remove_from_bookshelf(novel_url: String) -> Result<(), String> {
    info!("从书架移除小说: {}", novel_url);

    let mut novels = get_bookshelf().unwrap_or_default();
    let original_len = novels.len();

    novels.retain(|n| n.url != novel_url);

    if novels.len() == original_len {
        return Err(String::from("书架中未找到该小说"));
    }

    save_bookshelf(novels)?;

    info!("小说移除成功");
    Ok(())
}

/// 保存阅读进度
///
/// # 参数
/// * `progress` - 阅读进度
#[tauri::command]
pub fn save_reading_progress(progress: ReadingProgress) -> Result<(), String> {
    info!(
        "保存阅读进度: {} - {}",
        progress.novel_title, progress.chapter_title
    );

    ensure_data_dir()?;

    let path = get_progress_path();

    // 读取现有进度
    let mut all_progress = if path.exists() {
        let json = fs::read_to_string(&path)
            .map_err(|e| format!("读取进度文件失败: {}", e))?;
        serde_json::from_str::<Vec<ReadingProgress>>(&json)
            .unwrap_or_default()
    } else {
        Vec::new()
    };

    // 更新或添加进度
    if let Some(existing) = all_progress.iter_mut().find(|p| p.novel_url == progress.novel_url) {
        *existing = progress;
    } else {
        all_progress.push(progress);
    }

    // 保存
    let json = serde_json::to_string_pretty(&all_progress)
        .map_err(|e| format!("序列化失败: {}", e))?;

    fs::write(&path, json)
        .map_err(|e| format!("写入进度文件失败: {}", e))?;

    info!("阅读进度保存成功");
    Ok(())
}

/// 获取阅读进度
///
/// # 参数
/// * `novel_url` - 小说URL
#[tauri::command]
pub fn get_reading_progress(novel_url: String) -> Result<Option<ReadingProgress>, String> {
    info!("获取阅读进度: {}", novel_url);

    let path = get_progress_path();

    if !path.exists() {
        return Ok(None);
    }

    let json = fs::read_to_string(&path)
        .map_err(|e| format!("读取进度文件失败: {}", e))?;

    let all_progress: Vec<ReadingProgress> = serde_json::from_str(&json)
        .map_err(|e| format!("解析进度文件失败: {}", e))?;

    let progress = all_progress.into_iter().find(|p| p.novel_url == novel_url);

    Ok(progress)
}

/// 获取所有阅读进度
#[tauri::command]
pub fn get_all_reading_progress() -> Result<Vec<ReadingProgress>, String> {
    info!("获取所有阅读进度");

    let path = get_progress_path();

    if !path.exists() {
        return Ok(Vec::new());
    }

    let json = fs::read_to_string(&path)
        .map_err(|e| format!("读取进度文件失败: {}", e))?;

    let progress: Vec<ReadingProgress> = serde_json::from_str(&json)
        .map_err(|e| format!("解析进度文件失败: {}", e))?;

    Ok(progress)
}

/// 清除阅读进度
///
/// # 参数
/// * `novel_url` - 小说URL
#[tauri::command]
pub fn clear_reading_progress(novel_url: String) -> Result<(), String> {
    info!("清除阅读进度: {}", novel_url);

    let path = get_progress_path();

    if !path.exists() {
        return Ok(());
    }

    let json = fs::read_to_string(&path)
        .map_err(|e| format!("读取进度文件失败: {}", e))?;

    let mut all_progress: Vec<ReadingProgress> = serde_json::from_str(&json)
        .map_err(|e| format!("解析进度文件失败: {}", e))?;

    all_progress.retain(|p| p.novel_url != novel_url);

    let json = serde_json::to_string_pretty(&all_progress)
        .map_err(|e| format!("序列化失败: {}", e))?;

    fs::write(&path, json)
        .map_err(|e| format!("写入进度文件失败: {}", e))?;

    info!("阅读进度清除成功");
    Ok(())
}
