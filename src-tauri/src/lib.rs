//! 小说阅读器库模块
//!
//! 提供小说搜索、内容解析、广告清理和格式化的核心功能

mod commands;
mod core;
mod models;
mod utils;

use tauri::Manager;

pub use commands::*;
pub use core::*;
pub use models::*;

/// 运行Tauri应用
pub fn run() {
    log::info!("初始化Tauri应用...");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            log::info!("应用设置完成");

            // 获取主窗口
            let window = app.get_webview_window("main").unwrap();

            // 设置窗口标题
            window.set_title("小说阅读器").unwrap();

            log::info!("主窗口初始化成功");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::search::search_novels,
            commands::fetch::get_chapter_content,
            commands::fetch::get_chapter_list,
            commands::fetch::get_novel_details,
            commands::cleaner::clean_ad_content,
            commands::formatter::format_content,
            commands::cover::change_cover_source,
            commands::storage::save_bookshelf,
            commands::storage::get_bookshelf,
            commands::storage::save_reading_progress,
            commands::storage::get_reading_progress,
        ])
        .run(tauri::generate_context!())
        .expect("运行Tauri应用时发生错误");
}
