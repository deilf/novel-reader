//! 内容格式化模块
//!
//! 提供小说内容的智能排版和格式化功能

use crate::models::FormatConfig;
use log::{debug, info};
use regex::Regex;

/// 章节格式化器
pub struct ContentFormatter {
    /// 格式化配置
    config: FormatConfig,
}

impl ContentFormatter {
    /// 创建新的格式化器
    pub fn new(config: FormatConfig) -> Self {
        Self { config }
    }

    /// 使用默认配置创建格式化器
    pub fn default_formatter() -> Self {
        Self {
            config: FormatConfig::default(),
        }
    }

    /// 格式化小说内容
    pub fn format(&self, title: &str, content: &str) -> String {
        info!("格式化章节: {}", title);

        // 清理内容
        let cleaned = self.clean_content(content);

        // 分段处理
        let paragraphs = self.process_paragraphs(&cleaned);

        // 生成HTML
        let html = self.generate_html(title, &paragraphs);

        debug!("格式化完成，段落数: {}", paragraphs.len());
        html
    }

    /// 清理内容
    fn clean_content(&self, content: &str) -> String {
        let mut result = content.to_string();

        // 移除多余的空白字符
        result = result
            .chars()
            .map(|c| {
                if c.is_whitespace() && c != '\n' && c != ' ' {
                    ' '
                } else {
                    c
                }
            })
            .collect();

        // 规范化换行符
        result = result.replace("\r\n", "\n").replace("\r", "\n");

        // 移除连续的空白行
        if let Ok(re) = Regex::new(r"\n{3,}") {
            result = re.replace_all(&result, "\n\n").to_string();
        }

        // 压缩连续空格
        if let Ok(re) = Regex::new(r"[ ]{2,}") {
            result = re.replace_all(&result, " ").to_string();
        }

        result.trim().to_string()
    }

    /// 处理段落
    fn process_paragraphs(&self, content: &str) -> Vec<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut paragraphs = Vec::new();
        let mut current_paragraph = String::new();

        for line in lines {
            let trimmed = line.trim();

            // 跳过空行（将在后续处理）
            if trimmed.is_empty() {
                if !current_paragraph.is_empty() {
                    paragraphs.push(current_paragraph.clone());
                    current_paragraph.clear();
                }
                continue;
            }

            // 跳过章节标题（通常很短）
            if trimmed.len() < 10 && (trimmed.starts_with("第") || trimmed.starts_with("章节")) {
                continue;
            }

            // 移除多余的空格
            let cleaned_line = trimmed.split_whitespace().collect::<Vec<_>>().join(" ");

            // 如果当前段落为空，直接添加
            if current_paragraph.is_empty() {
                current_paragraph = cleaned_line;
            } else if current_paragraph.len() + cleaned_line.len() < 200 {
                // 如果合并后长度合理，则合并
                current_paragraph.push_str(" ");
                current_paragraph.push_str(&cleaned_line);
            } else {
                // 保存当前段落，开始新段落
                paragraphs.push(current_paragraph.clone());
                current_paragraph = cleaned_line;
            }
        }

        // 添加最后一个段落
        if !current_paragraph.is_empty() {
            paragraphs.push(current_paragraph);
        }

        paragraphs
    }

    /// 生成HTML
    fn generate_html(&self, title: &str, paragraphs: &[String]) -> String {
        let mut html = String::new();

        // CSS样式
        let css = format!(
            r#"
            <style>
                .chapter-container {{
                    font-family: {},;
                    font-size: {}px;
                    line-height: {};
                    color: {};
                    background-color: {};
                    padding: {}px;
                    max-width: 100%;
                    margin: 0 auto;
                }}
                .chapter-title {{
                    font-size: {}px;
                    font-weight: bold;
                    text-align: center;
                    margin-bottom: 20px;
                    color: {};
                }}
                .chapter-content {{
                    text-align: {};
                }}
                .chapter-content p {{
                    margin-bottom: {}em;
                    text-indent: {};
                }}
                .chapter-content p:first-child {{
                    text-indent: 0;
                }}
            </style>
            "#,
            self.get_font_family(),
            self.config.font_size,
            self.config.line_height,
            self.config.text_color,
            self.config.background_color,
            self.config.margin_horizontal,
            self.config.font_size + 4,
            self.config.text_color,
            self.config.text_align,
            self.config.paragraph_spacing,
            if self.config.text_indent { "2em" } else { "0" }
        );

        html.push_str(&css);

        // 容器开始
        html.push_str(r#"<div class="chapter-container">"#);

        // 章节标题
        html.push_str(&format!(
            r#"<h1 class="chapter-title">{}</h1>"#,
            self.escape_html(title)
        ));

        // 内容区域
        html.push_str(r#"<div class="chapter-content">"#);

        for paragraph in paragraphs {
            let formatted = if self.config.text_indent {
                // 首行缩进（第一段除外）
                format!(
                    "<p style=\"text-indent: 2em;\">{}</p>",
                    self.escape_html(paragraph)
                )
            } else {
                format!("<p>{}</p>", self.escape_html(paragraph))
            };
            html.push_str(&formatted);
        }

        // 容器结束
        html.push_str("</div></div>");

        html
    }

    /// 获取字体系列
    fn get_font_family(&self) -> &str {
        match self.config.font_family.as_str() {
            "宋体" => "'SimSun', 'Songti SC', serif",
            "黑体" => "'SimHei', 'Heiti SC', sans-serif",
            "楷体" => "'KaiTi', 'Kaiti SC', cursive",
            "微软雅黑" => "'Microsoft YaHei', 'PingFang SC', sans-serif",
            _ => "'System', '-apple-system', 'PingFang SC', sans-serif",
        }
    }

    /// HTML转义
    fn escape_html(&self, text: &str) -> String {
        text.replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&#39;")
    }

    /// 生成纯文本
    pub fn format_plain_text(&self, title: &str, content: &str) -> String {
        let cleaned = self.clean_content(content);
        let paragraphs = self.process_paragraphs(&cleaned);

        let mut result = String::new();

        // 添加标题
        result.push_str(title);
        result.push_str("\n\n");

        // 添加段落
        for (index, paragraph) in paragraphs.iter().enumerate() {
            if self.config.text_indent && index > 0 {
                // 首行缩进用空格模拟
                result.push_str("    ");
            }
            result.push_str(paragraph);
            result.push_str("\n\n");
        }

        result.trim().to_string()
    }

    /// 更新配置
    pub fn update_config(&mut self, config: FormatConfig) {
        self.config = config;
    }

    /// 获取当前配置
    pub fn get_config(&self) -> &FormatConfig {
        &self.config
    }
}

impl Default for ContentFormatter {
    fn default() -> Self {
        Self::default_formatter()
    }
}

/// 文本统计工具
pub struct TextStats;

impl TextStats {
    /// 统计字数
    pub fn count_words(text: &str) -> u32 {
        text.chars()
            .filter(|c| !c.is_whitespace())
            .count() as u32
    }

    /// 统计章节数
    pub fn count_chapters(text: &str) -> u32 {
        let re = Regex::new(r"第[一二三四五六七八九十百千万\d]+章").unwrap();
        re.find_iter(text).count() as u32
    }

    /// 统计段落数
    pub fn count_paragraphs(text: &str) -> u32 {
        text.lines()
            .filter(|line| !line.trim().is_empty())
            .count() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        let formatter = ContentFormatter::default_formatter();
        let content = "这是第一章的内容。\n\n这是第二段的内容。";
        let html = formatter.format("第一章", content);

        assert!(html.contains("第一章"));
        assert!(html.contains("chapter-container"));
    }

    #[test]
    fn test_clean_content() {
        let formatter = ContentFormatter::default_formatter();
        let content = "   这是  内容   \n\n\n\n第二段   ";
        let cleaned = formatter.clean_content(content);

        assert!(cleaned.contains("这是"));
        assert!(cleaned.contains("内容"));
        assert!(!cleaned.contains("   "));
    }

    #[test]
    fn test_count_words() {
        assert_eq!(TextStats::count_words("这是一段测试文字"), 8);
        assert_eq!(TextStats::count_words("Hello World"), 10);
    }
}
