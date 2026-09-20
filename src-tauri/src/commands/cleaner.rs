//! 广告清理命令模块
//!
//! 提供内容清理功能的Tauri命令

use crate::core::AdCleaner;
use log::info;

/// 清理广告内容
///
/// # 参数
/// * `content` - 需要清理的内容（HTML或纯文本）
/// * `is_html` - 内容是否为HTML格式
///
/// # 返回
/// 清理后的内容
#[tauri::command]
pub fn clean_ad_content(content: String, is_html: bool) -> Result<String, String> {
    info!("执行广告清理，HTML: {}", is_html);

    if content.is_empty() {
        return Ok(String::new());
    }

    let mut cleaner = AdCleaner::new();

    let result = if is_html {
        cleaner.clean_html(&content)
    } else {
        cleaner.clean_text(&content)
    };

    info!("广告清理完成");
    Ok(result)
}
