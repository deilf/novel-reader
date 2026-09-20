//! 工具函数模块
//!
//! 提供各种辅助功能函数

use regex::Regex;
use std::collections::HashMap;

/// URL编码
pub fn url_encode(input: &str) -> String {
    urlencoding::encode(input).to_string()
}

/// URL解码
pub fn url_decode(input: &str) -> String {
    urlencoding::decode(input)
        .map(|s| s.to_string())
        .unwrap_or_else(|_| input.to_string())
}

/// HTML实体解码
pub fn decode_html_entities(text: &str) -> String {
    let mut result = text.to_string();

    let entities: HashMap<&str, &str> = [
        ("&nbsp;", " "),
        ("&amp;", "&"),
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&quot;", "\""),
        ("&apos;", "'"),
        ("&copy;", "©"),
        ("&reg;", "®"),
        ("&trade;", "™"),
        ("&hellip;", "…"),
        ("&middot;", "·"),
        ("&lsquo;", """),
        ("&rsquo;", """),
        ("&ldquo;", """),
        ("&rdquo;", """),
    ]
    .iter()
    .cloned()
    .collect();

    for (entity, replacement) in entities {
        result = result.replace(entity, replacement);
    }

    // 处理数字形式的实体
    if let Ok(re) = Regex::new(r"&#(\d+);") {
        result = re
            .replace_all(&result, |caps: &regex::Captures| {
                if let Ok(num) = caps[1].parse::<u32>() {
                    char::from_u32(num).map(|c| c.to_string()).unwrap_or_default()
                } else {
                    String::new()
                }
            })
            .to_string();
    }

    result
}

/// 提取URL中的域名
pub fn extract_domain(url: &str) -> Option<String> {
    url::Url::parse(url)
        .ok()
        .and_then(|u| u.host_str().map(|s| s.to_string()))
}

/// 判断URL是否有效
pub fn is_valid_url(url: &str) -> bool {
    url::Url::parse(url)
        .map(|u| u.scheme().starts_with("http"))
        .unwrap_or(false)
}

/// 清理字符串中的空白字符
pub fn trim_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// 截断字符串
pub fn truncate(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len])
    }
}

/// 生成唯一ID
pub fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();

    format!("{:x}", timestamp)
}

/// 格式化文件大小
pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// 检测字符串编码
pub fn detect_encoding(bytes: &[u8]) -> &'static str {
    // BOM检测
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return "UTF-8";
    }
    if bytes.starts_with(&[0xFF, 0xFE]) {
        return "UTF-16LE";
    }
    if bytes.starts_with(&[0xFE, 0xFF]) {
        return "UTF-16BE";
    }

    // 尝试解码检测
    if std::str::from_utf8(bytes).is_ok() {
        return "UTF-8";
    }

    // 默认为GBK（中文网站常用）
    "GBK"
}

/// 简化文本（用于搜索索引）
pub fn simplify_text(text: &str) -> String {
    let mut result = text.to_lowercase();

    // 移除标点符号
    if let Ok(re) = Regex::new(r"[^\w\u4e00-\u9fff]") {
        result = re.replace_all(&result, "").to_string();
    }

    result
}

/// 字符串相似度（简单实现）
pub fn string_similarity(a: &str, b: &str) -> f64 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }

    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    let a_len = a_chars.len();
    let b_len = b_chars.len();

    // 计算编辑距离
    let mut matrix = vec![vec![0usize; b_len + 1]; a_len + 1];

    for i in 0..=a_len {
        matrix[i][0] = i;
    }
    for j in 0..=b_len {
        matrix[0][j] = j;
    }

    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = std::cmp::min(
                std::cmp::min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1),
                matrix[i - 1][j - 1] + cost,
            );
        }
    }

    let distance = matrix[a_len][b_len];
    let max_len = std::cmp::max(a_len, b_len);

    1.0 - (distance as f64 / max_len as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_encode_decode() {
        let original = "小说阅读器";
        let encoded = url_encode(original);
        let decoded = url_decode(&encoded);
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_html_entities() {
        let input = "&lt;div&gt;&amp;&quot;测试&quot;&lt;/div&gt;";
        let output = decode_html_entities(input);
        assert_eq!(output, "<div>&\"测试\"</div>");
    }

    #[test]
    fn test_extract_domain() {
        assert_eq!(
            extract_domain("https://example.com/book/123"),
            Some(String::from("example.com"))
        );
    }

    #[test]
    fn test_is_valid_url() {
        assert!(is_valid_url("https://example.com"));
        assert!(is_valid_url("http://example.com/path"));
        assert!(!is_valid_url("ftp://example.com"));
        assert!(!is_valid_url("not a url"));
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("Hello World", 5), "Hello...");
        assert_eq!(truncate("Hi", 10), "Hi");
    }

    #[test]
    fn test_similarity() {
        let similarity = string_similarity("小说", "小兑");
        assert!(similarity > 0.5);
    }
}
