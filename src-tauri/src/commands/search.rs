//! 搜索命令模块
//!
//! 提供小说搜索功能的Tauri命令

use crate::core::WebScraper;
use crate::models::SearchResult;
use log::{error, info};

/// 搜索小说
///
/// # 参数
/// * `website_url` - 网站基础URL
/// * `keyword` - 搜索关键词
///
/// # 返回
/// 搜索结果列表
#[tauri::command]
pub async fn search_novels(
    website_url: String,
    keyword: String,
) -> Result<Vec<SearchResult>, String> {
    info!(
        "执行搜索命令: 网站={}, 关键词={}",
        website_url, keyword
    );

    // 参数验证
    if website_url.is_empty() {
        return Err(String::from("网站URL不能为空"));
    }

    if keyword.is_empty() {
        return Err(String::from("搜索关键词不能为空"));
    }

    // 创建爬虫实例
    let scraper = WebScraper::new();

    // 执行搜索
    match scraper.search_novels(&website_url, &keyword).await {
        Ok(results) => {
            info!("搜索成功，找到 {} 个结果", results.len());
            Ok(results)
        }
        Err(e) => {
            error!("搜索失败: {}", e);
            Err(format!("搜索失败: {}", e))
        }
    }
}
