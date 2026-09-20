//! 广告清理模块
//!
//! 提供智能广告识别和清理功能，移除网页中的各类广告元素

use log::{debug, info, warn};
use regex::Regex;
use scraper::{Html, Selector};

/// 广告模式定义
pub struct AdPattern {
    /// 广告名称
    pub name: String,
    /// CSS选择器
    pub selector: String,
    /// 正则匹配模式
    pub regex_pattern: Option<String>,
    /// 文本关键词
    pub text_keywords: Vec<String>,
    /// 匹配权重
    pub weight: i32,
}

impl AdPattern {
    pub fn new(name: &str, selector: &str) -> Self {
        Self {
            name: name.to_string(),
            selector: selector.to_string(),
            regex_pattern: None,
            text_keywords: Vec::new(),
            weight: 1,
        }
    }

    pub fn with_regex(mut self, pattern: &str) -> Self {
        self.regex_pattern = Some(pattern.to_string());
        self
    }

    pub fn with_keywords(mut self, keywords: Vec<&str>) -> Self {
        self.text_keywords = keywords.into_iter().map(|s| s.to_string()).collect();
        self.weight += 1;
        self
    }

    pub fn with_weight(mut self, weight: i32) -> Self {
        self.weight = weight;
        self
    }
}

/// 广告清理器
pub struct AdCleaner {
    /// 广告模式列表
    patterns: Vec<AdPattern>,
    /// 正则表达式缓存
    regex_cache: std::collections::HashMap<String, Regex>,
}

impl AdCleaner {
    /// 创建新的广告清理器
    pub fn new() -> Self {
        let patterns = Self::default_patterns();
        Self {
            patterns,
            regex_cache: std::collections::HashMap::new(),
        }
    }

    /// 默认广告模式
    fn default_patterns() -> Vec<AdPattern> {
        vec![
            // 常见广告容器选择器
            AdPattern::new("弹窗广告", ".popup, .modal, .dialog, [class*='popup'], [class*='modal']")
                .with_weight(3),

            AdPattern::new("浮动广告", "[style*='position: fixed'], [style*='position:absolute']")
                .with_keywords(vec!["fixed", "sticky"])
                .with_weight(3),

            AdPattern::new("横幅广告", "[class*='banner'], [id*='banner'], [class*='ad-'], [id*='ad-']")
                .with_weight(2),

            AdPattern::new("Google广告", "[id*='google_ads'], [id*='google-ads'], [class*='google-ad']")
                .with_regex(r"google.*ads?")
                .with_weight(5),

            AdPattern::new("百度广告", "[id*='baidu_ads'], [class*='baidu-ad']")
                .with_regex(r"baidu.*ads?")
                .with_weight(5),

            AdPattern::new("联盟广告", "[class*='union-ad'], [class*='affiliate']")
                .with_keywords(vec!["联盟", "推广", "广告"])
                .with_weight(2),

            AdPattern::new("推广内容", "[class*='promotion'], [class*='sponsor']")
                .with_keywords(vec!["推广", "赞助", "广告"])
                .with_weight(2),

            AdPattern::new("下载提示", "[class*='download-tip'], [class*='app-download']")
                .with_keywords(vec!["下载APP", "立即下载"])
                .with_weight(2),

            AdPattern::new("悬浮按钮", "[class*='float-'], [class*='sticky-btn']")
                .with_weight(2),

            AdPattern::new("视频广告", "[class*='video-ad'], [id*='video-ad']")
                .with_regex(r"video.*ad")
                .with_weight(4),

            AdPattern::new("侧边栏广告", ".sidebar-ad, .side-ad, [class*='side-banner']")
                .with_weight(2),

            AdPattern::new("内容推荐", "[class*='recommend']:not([class*='book-recommend'])")
                .with_keywords(vec!["相关推荐", "猜你喜欢"])
                .with_weight(1),

            AdPattern::new("登录提示", "[class*='login-tip'], [class*='login-popup']")
                .with_keywords(vec!["登录", "注册"])
                .with_weight(2),

            AdPattern::new("分享按钮", "[class*='share-box']:not([class*='book-share'])")
                .with_weight(1),

            AdPattern::new("评论框", ".comment-box, #comment, .comment-form")
                .with_weight(1),
        ]
    }

    /// 清理HTML中的广告
    pub fn clean_html(&mut self, html: &str) -> String {
        info!("开始清理广告内容");

        let document = Html::parse_document(html);
        let mut result = html.to_string();

        // 移除广告元素
        for pattern in &self.patterns {
            if let Ok(selector) = Selector::parse(&pattern.selector) {
                let elements: Vec<_> = document.select(&selector).collect();

                for element in elements {
                    let element_html = element.html();

                    // 检查是否包含广告关键词
                    if self.is_ad_element(&element) {
                        debug!("移除广告元素: {} - {}", pattern.name, element_html.chars().take(50).collect::<String>());
                        result = result.replace(&element_html, "");
                    }
                }
            }
        }

        // 清理内联广告代码
        result = self.clean_inline_ads(&result);

        // 清理危险脚本
        result = self.clean_dangerous_scripts(&result);

        // 清理空标签
        result = self.clean_empty_tags(&result);

        info!("广告清理完成");
        result
    }

    /// 检查元素是否为广告
    fn is_ad_element(&self, element: &scraper::ElementRef) -> bool {
        let html = element.html().to_lowercase();
        let text = element.text().collect::<String>().to_lowercase();

        // 检查文本关键词
        let ad_keywords = vec![
            "广告", "ad", "ads", "推广", "sponsor", "promotion",
            "下载app", "立即下载", "看视频", "看广告",
            "google adsense", "百度联盟", "adsense",
            "注册送", "首充", "优惠", "打折",
        ];

        for keyword in ad_keywords.iter() {
            if text.contains(keyword) || html.contains(keyword) {
                return true;
            }
        }

        // 检查特定属性
        let element_html = element.html();
        if element_html.contains("data-ad") || element_html.contains("ad-") {
            return true;
        }

        false
    }

    /// 清理内联广告代码
    fn clean_inline_ads(&self, html: &str) -> String {
        let mut result = html.to_string();

        // 移除带有广告标识的script标签内容
        let ad_script_patterns = [
            r"(<script[^>]*>[\s\S]*?(google|adsense|baidu|analytics)[\s\S]*?</script>)",
            r"(<script[^>]*ad[\s\S]*?</script>)",
            r"(<style[^>]*>[\s\S]*?(ad|promo)[\s\S]*?</style>)",
        ];

        for pattern in ad_script_patterns.iter() {
            if let Ok(re) = Regex::new(pattern) {
                result = re.replace_all(&result, "").to_string();
            }
        }

        // 移除onclick广告触发器
        if let Ok(re) = Regex::new(r#"(onclick|onmouseover|onload)=["'][^"']*(?:ad|ads|promo|click)["']"#) {
            result = re.replace_all(&result, "").to_string();
        }

        result
    }

    /// 清理危险脚本
    fn clean_dangerous_scripts(&self, html: &str) -> String {
        let mut result = html.to_string();

        // 移除所有script标签
        if let Ok(re) = Regex::new(r"<script[^>]*>[\s\S]*?</script>") {
            result = re.replace_all(&result, "").to_string();
        }

        // 移除内联事件处理器
        if let Ok(re) = Regex::new(r"\s+on\w+=[\"'][^\"']*[\"']") {
            result = re.replace_all(&result, "").to_string();
        }

        // 移除危险标签
        let dangerous_tags = vec!["iframe", "object", "embed", "applet", "form"];
        for tag in dangerous_tags {
            if let Ok(re) = Regex::new(&format!(r"<{}[^>]*>[\s\S]*?</{}>", tag, tag)) {
                result = re.replace_all(&result, "").to_string();
            }
        }

        result
    }

    /// 清理空标签
    fn clean_empty_tags(&self, html: &str) -> String {
        let mut result = html.to_string();

        // 移除空段落
        if let Ok(re) = Regex::new(r"<p[^>]*>\s*</p>") {
            result = re.replace_all(&result, "").to_string();
        }

        // 移除空div
        if let Ok(re) = Regex::new(r"<div[^>]*>\s*</div>") {
            result = re.replace_all(&result, "").to_string();
        }

        // 移除只有空白字符的标签
        if let Ok(re) = Regex::new(r"<(\w+)[^>]*>\s*</\1>") {
            result = re.replace_all(&result, "").to_string();
        }

        result
    }

    /// 清理文本中的广告内容
    pub fn clean_text(&mut self, text: &str) -> String {
        let mut result = text.to_string();

        // 移除常见的广告文本模式
        let ad_text_patterns = vec![
            r"(?:看完整内容|请到|访问|打开链接)\s*[:：]\s*https?://\S+",
            r"(?:APP下载|手机客户端)\s*[:：]?\s*\S+",
            r"(?:微信公众号|关注公众号)\s*[:：]?\s*\S+",
            r"(?:加入|关注)\s*(?:QQ群|微信群|telegram)\s*[:：]?\s*\S+",
            r"(?:赞助商|广告)\s*[:：]\s*.+",
        ];

        for pattern in ad_text_patterns.iter() {
            if let Ok(re) = Regex::new(pattern) {
                result = re.replace_all(&result, "").to_string();
            }
        }

        // 清理多余空白
        result = result
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n\n");

        result
    }

    /// 添加自定义广告模式
    pub fn add_pattern(&mut self, pattern: AdPattern) {
        self.patterns.push(pattern);
    }

    /// 移除指定名称的广告模式
    pub fn remove_pattern(&mut self, name: &str) {
        self.patterns.retain(|p| p.name != name);
    }
}

impl Default for AdCleaner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_html() {
        let mut cleaner = AdCleaner::new();
        let html = r#"
            <div class="ad-banner">这是广告</div>
            <div class="content">这是正文内容</div>
            <script>alert('ad');</script>
        "#;

        let cleaned = cleaner.clean_html(html);
        assert!(cleaned.contains("这是正文内容"));
        assert!(!cleaned.contains("这是广告"));
    }

    #[test]
    fn test_clean_text() {
        let mut cleaner = AdCleaner::new();
        let text = "第一章\n\n这是正文内容\n\nAPP下载: example.com\n\n第二章";
        let cleaned = cleaner.clean_text(text);
        assert!(cleaned.contains("这是正文内容"));
        assert!(!cleaned.contains("APP下载"));
    }
}
