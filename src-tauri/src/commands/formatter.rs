//! 格式化命令模块
//!
//! 提供内容格式化功能的Tauri命令

use crate::core::ContentFormatter;
use crate::models::FormatConfig;
use log::info;

/// 格式化小说内容
///
/// # 参数
/// * `title` - 章节标题
/// * `content` - 原始内容
/// * `config` - 格式化配置
///
/// # 返回
/// 格式化后的HTML内容
#[tauri::command]
pub fn format_content(
    title: String,
    content: String,
    config: FormatConfig,
) -> Result<String, String> {
    info!("执行内容格式化: {}", title);

    if content.is_empty() {
        return Err(String::from("内容不能为空"));
    }

    let formatter = ContentFormatter::new(config);
    let result = formatter.format(&title, &content);

    info!("内容格式化完成");
    Ok(result)
}
