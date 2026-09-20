//! 内容获取命令模块
//!
//! 提供小说章节内容获取功能的Tauri命令

use crate::core::{AdCleaner, ContentFormatter, WebScraper};
use crate::models::{Chapter, FormatConfig, FormattedChapter};
use log::{error, info};

/// 获取小说详情和章节列表
///
/// # 参数
/// * `url` - 小说详情页URL
///
/// # 返回
/// (标题, 作者, 章节列表, 简介)
#[tauri::command]
pub async fn get_novel_details(url: String) -> Result<(String, String, Vec<Chapter>, String), String> {
    info!("获取小说详情: {}", url);

    if url.is_empty() {
        return Err(String::from("URL不能为空"));
    }

    let scraper = WebScraper::new();

    match scraper.get_novel_details(&url).await {
        Ok((title, author, chapters, description)) => {
            info!(
                "小说详情获取成功: {} ({}), {} 个章节",
                title,
                author,
                chapters.len()
            );
            Ok((title, author, chapters, description))
        }
        Err(e) => {
            error!("获取小说详情失败: {}", e);
            Err(format!("获取小说详情失败: {}", e))
        }
    }
}

/// 获取章节列表
///
/// # 参数
/// * `url` - 小说详情页URL
///
/// # 返回
/// 章节列表
#[tauri::command]
pub async fn get_chapter_list(url: String) -> Result<Vec<Chapter>, String> {
    info!("获取章节列表: {}", url);

    if url.is_empty() {
        return Err(String::from("URL不能为空"));
    }

    let scraper = WebScraper::new();

    match scraper.get_novel_details(&url).await {
        Ok((_, _, chapters, _)) => {
            info!("章节列表获取成功: {} 个章节", chapters.len());
            Ok(chapters)
        }
        Err(e) => {
            error!("获取章节列表失败: {}", e);
            Err(format!("获取章节列表失败: {}", e))
        }
    }
}

/// 获取章节内容
///
/// # 参数
/// * `url` - 章节URL
/// * `format_config` - 格式化配置（可选）
///
/// # 返回
/// 格式化后的章节内容
#[tauri::command]
pub async fn get_chapter_content(
    url: String,
    format_config: Option<FormatConfig>,
) -> Result<FormattedChapter, String> {
    info!("获取章节内容: {}", url);

    if url.is_empty() {
        return Err(String::from("URL不能为空"));
    }

    let scraper = WebScraper::new();

    // 获取原始内容
    let (title, content) = match scraper.get_chapter_content(&url).await {
        Ok(result) => result,
        Err(e) => {
            error!("获取章节内容失败: {}", e);
            return Err(format!("获取章节内容失败: {}", e));
        }
    };

    // 清理广告
    let mut cleaner = AdCleaner::new();
    let cleaned_content = cleaner.clean_text(&content);

    // 格式化内容
    let formatter = if let Some(config) = format_config {
        ContentFormatter::new(config)
    } else {
        ContentFormatter::default_formatter()
    };

    let formatted_content = formatter.format(&title, &cleaned_content);
    let word_count = formatted_content.chars().filter(|c| !c.is_whitespace()).count() as u32;

    let result = FormattedChapter {
        title,
        content: formatted_content,
        prev_url: None,
        next_url: None,
        source_url: url,
        word_count,
    };

    info!("章节内容获取成功: {} 字", word_count);
    Ok(result)
}
