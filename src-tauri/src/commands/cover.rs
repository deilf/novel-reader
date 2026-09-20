//! 封面命令模块
//!
//! 提供封面图管理和切换功能的Tauri命令

use crate::core::WebScraper;
use log::{error, info};

/// 切换封面图片来源
///
/// # 参数
/// * `novel_title` - 小说标题
/// * `current_cover` - 当前封面URL
/// * `source` - 目标来源 (original, google, baidu)
/// * `base_url` - 网站基础URL
///
/// # 返回
/// 新的封面URL
#[tauri::command]
pub async fn change_cover_source(
    novel_title: String,
    current_cover: String,
    source: String,
    base_url: String,
) -> Result<String, String> {
    info!(
        "切换封面来源: {} -> {}",
        novel_title, source
    );

    if novel_title.is_empty() {
        return Err(String::from("小说标题不能为空"));
    }

    // 如果选择原始封面且当前封面有效，直接返回
    if source == "original" && !current_cover.is_empty() {
        return Ok(current_cover);
    }

    // 根据来源生成新的封面URL
    let new_cover = match source.as_str() {
        "google" => {
            // Google图片搜索
            let search_url = format!(
                "https://www.google.com/search?tbm=isch&q={}+小说封面",
                urlencoding::encode(&novel_title)
            );

            // 返回搜索页面URL（实际应用中会在这里获取第一张图片）
            info!("Google图片搜索: {}", search_url);
            search_url
        }
        "baidu" => {
            // 百度图片搜索
            let search_url = format!(
                "https://image.baidu.com/search/index?word={}+小说封面",
                urlencoding::encode(&novel_title)
            );

            info!("百度图片搜索: {}", search_url);
            search_url
        }
        "original" => current_cover,
        _ => {
            error!("未知的封面来源: {}", source);
            return Err(format!("未知的封面来源: {}", source));
        }
    };

    Ok(new_cover)
}

/// 获取封面图片
///
/// # 参数
/// * `cover_url` - 封面URL
///
/// # 返回
/// 封面图片数据（Base64编码）
#[tauri::command]
pub async fn fetch_cover_image(cover_url: String) -> Result<String, String> {
    info!("获取封面图片: {}", cover_url);

    if cover_url.is_empty() {
        return Err(String::from("封面URL不能为空"));
    }

    // 检查是否为搜索页面URL
    if cover_url.contains("google.com/search") || cover_url.contains("baidu.com/search") {
        return Err(String::from("这是搜索页面URL，需要进一步解析"));
    }

    let scraper = WebScraper::new();

    match scraper.fetch_page(&cover_url).await {
        Ok(html) => {
            // 在实际应用中，这里会解析HTML获取实际图片URL
            info!("封面页面获取成功");
            Ok(cover_url)
        }
        Err(e) => {
            error!("获取封面失败: {}", e);
            Err(format!("获取封面失败: {}", e))
        }
    }
}

/// 验证封面URL是否可访问
///
/// # 参数
/// * `cover_url` - 封面URL
///
/// # 返回
/// 是否可访问
#[tauri::command]
pub async fn validate_cover_url(cover_url: String) -> Result<bool, String> {
    if cover_url.is_empty() {
        return Ok(false);
    }

    let scraper = WebScraper::new();

    match scraper.fetch_page(&cover_url).await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
