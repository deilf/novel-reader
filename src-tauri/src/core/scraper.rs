//! 网页爬取模块
//!
//! 提供网页内容获取、编码检测、HTML解析等功能

use crate::models::{Chapter, SearchResult, WebsiteConfig};
use log::{debug, error, info, warn};
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;
use url::Url;

/// 爬虫错误类型
#[derive(Error, Debug)]
pub enum ScraperError {
    #[error("网络请求失败: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("URL解析失败: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("编码检测失败")]
    EncodingError,

    #[error("解析失败: {0}")]
    ParseError(String),

    #[error("超时错误")]
    TimeoutError,

    #[error("网站不可访问: {0}")]
    UnreachableError(String),
}

/// 网页爬虫
pub struct WebScraper {
    client: Client,
    user_agent: String,
}

impl WebScraper {
    /// 创建新的爬虫实例
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .build()
            .expect("创建HTTP客户端失败");

        Self {
            client,
            user_agent: String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"),
        }
    }

    /// 检测网页编码
    pub fn detect_encoding(&self, html: &[u8], default_encoding: &str) -> String {
        // 首先尝试从meta标签检测
        if let Ok(text) = std::str::from_utf8(html) {
            // 检测UTF-8
            if text.contains("charset=utf-8") || text.contains("charset=\"utf-8\"") {
                return String::from("UTF-8");
            }

            // 检测GBK
            if text.contains("charset=gbk") || text.contains("charset=\"gbk\"") {
                return String::from("GBK");
            }

            // 检测GB2312
            if text.contains("charset=gb2312") || text.contains("charset=\"gb2312\"") {
                return String::from("GB2312");
            }
        }

        // 尝试自动检测编码
        let (decoded, encoding, _) = encoding_rs::Encoding::for_bom(html)
            .map(|(e, _)| {
                let (decoded, _, had_errors) = e.decode(html);
                (decoded.into_owned(), e.name().to_string(), had_errors)
            })
            .unwrap_or_else(|| {
                let encoding = encoding_rs::Encoding::for_label(default_encoding.as_bytes())
                    .unwrap_or(encoding_rs::UTF_8);
                let (decoded, _, _) = encoding.decode(html);
                (decoded.into_owned(), encoding.name().to_string(), false)
            });

        debug!("检测到编码: {}", encoding);
        encoding
    }

    /// 获取网页内容
    pub async fn fetch_page(&self, url: &str) -> Result<String, ScraperError> {
        info!("正在获取页面: {}", url);

        // 验证URL
        let parsed_url = Url::parse(url)?;
        if !parsed_url.scheme().starts_with("http") {
            return Err(ScraperError::UrlParseError(
                url::ParseError::EmptyHost,
            ));
        }

        // 发送请求
        let response = self.client
            .get(url)
            .header("User-Agent", &self.user_agent)
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
            .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
            .send()
            .await?;

        // 检查状态码
        let status = response.status();
        if !status.is_success() {
            warn!("页面返回非成功状态码: {} - {}", url, status);
            if status.as_u16() == 404 {
                return Err(ScraperError::UnreachableError(String::from("页面不存在")));
            }
        }

        // 获取字节内容
        let bytes = response.bytes().await?;
        let encoding = self.detect_encoding(&bytes, "UTF-8");

        // 解码内容
        let encoding_obj = encoding_rs::Encoding::for_label(encoding.as_bytes())
            .unwrap_or(encoding_rs::UTF_8);
        let (text, _, _) = encoding_obj.decode(&bytes);

        debug!("成功获取页面，长度: {} 字节，编码: {}", bytes.len(), encoding);

        Ok(text.into_owned())
    }

    /// 从网页中提取链接的绝对URL
    fn make_absolute_url(base_url: &str, href: &str) -> Option<String> {
        if href.starts_with("http://") || href.starts_with("https://") {
            Some(href.to_string())
        } else if href.starts_with("/") {
            // 绝对路径
            let base = Url::parse(base_url).ok()?;
            let base_path = base.path();
            let new_path = if href.starts_with("/") {
                href.to_string()
            } else {
                format!("/{}", href)
            };

            // 处理路径
            let combined = format!("{}://{}:{}", base.scheme(), base.host_str()?, base.port().unwrap_or(80));
            let host = base.host_str()?;

            if let Some(port) = base.port() {
                Some(format!("{}://{}:{}{}", base.scheme(), host, port, new_path))
            } else {
                Some(format!("{}://{}{}", base.scheme(), host, new_path))
            }
        } else {
            // 相对路径
            let base = Url::parse(base_url).ok()?;
            base.join(href).ok().map(|u| u.to_string())
        }
    }

    /// 搜索小说
    pub async fn search_novels(&self, website_url: &str, keyword: &str) -> Result<Vec<SearchResult>, ScraperError> {
        info!("在 {} 搜索小说: {}", website_url, keyword);

        let base_url = Url::parse(website_url)?;
        let mut search_url = base_url.join("/search")?;
        search_url
            .query_pairs_mut()
            .append_pair("keyword", keyword)
            .append_pair("searchkey", keyword)
            .append_pair("s", keyword);

        debug!("搜索URL: {}", search_url);

        // 获取搜索结果页面
        let html = self.fetch_page(&search_url.to_string()).await?;

        // 解析搜索结果
        let results = self.parse_search_results(&html, &search_url.to_string(), &base_url.host_str().unwrap_or("unknown"));

        info!("搜索到 {} 个结果", results.len());
        Ok(results)
    }

    /// 解析搜索结果
    fn parse_search_results(&self, html: &str, base_url: &str, source: &str) -> Vec<SearchResult> {
        let document = Html::parse_document(html);
        let mut results = Vec::new();

        // 尝试多种常见的选择器模式
        let selector_patterns = [
            // 通用小说列表选择器
            ".bookbox|.book-item|.novel-item",
            ".result-item|.search-result",
            "div[data-type='book']|li[data-id]",
            ".book-list .book-item",
            ".mainbody .book",
        ];

        for pattern in selector_patterns.iter() {
            if let Ok(selector) = Selector::parse(pattern) {
                let elements: Vec<_> = document.select(&selector).collect();
                if !elements.is_empty() {
                    debug!("使用选择器 {} 找到 {} 个元素", pattern, elements.len());

                    for element in elements {
                        if let Some(result) = self.extract_novel_info(&element, base_url, source) {
                            results.push(result);
                        }
                    }

                    if !results.is_empty() {
                        break;
                    }
                }
            }
        }

        // 如果标准选择器都没找到，尝试全文搜索
        if results.is_empty() {
            debug!("标准选择器未找到结果，尝试全文搜索");

            // 搜索标题链接
            let link_selector = Selector::parse("a[href*='/book/'], a[href*='/novel/'], a[href*='/info/']").unwrap();
            for link in document.select(&link_selector) {
                if let Some(href) = link.value().attr("href") {
                    let title = link.text().collect::<String>().trim().to_string();
                    if !title.is_empty() && title.len() > 2 && !title.contains("登录") && !title.contains("注册") {
                        let url = Self::make_absolute_url(base_url, href).unwrap_or_default();
                        if !url.is_empty() && url.contains("book") || url.contains("novel") || url.contains("info") {
                            results.push(SearchResult {
                                title,
                                author: String::from("未知作者"),
                                url,
                                cover_url: String::new(),
                                description: String::new(),
                                latest_chapter: String::from("未知"),
                                source: source.to_string(),
                            });
                        }
                    }
                }
            }
        }

        // 去重
        results.sort_by(|a, b| a.title.cmp(&b.title));
        results.dedup_by(|a, b| a.title == b.title && a.url == b.url);

        results
    }

    /// 从元素中提取小说信息
    fn extract_novel_info(&self, element: &scraper::ElementRef, base_url: &str, source: &str) -> Option<SearchResult> {
        let html = element.html();

        // 提取标题和链接
        let a_selector = Selector::parse("a").ok();
        let title = element
            .select(&Selector::parse(".bookname, .title, h3, h2, .name").ok()?)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| {
                a_selector
                    .as_ref()
                    .and_then(|sel| element.select(sel).next())
                    .map(|e| e.text().collect::<String>().trim().to_string())
                    .unwrap_or_default()
            });

        if title.is_empty() {
            return None;
        }

        // 提取URL
        let url = element
            .select(&Selector::parse("a").ok()?)
            .next()
            .and_then(|a| a.value().attr("href"))
            .and_then(|href| Self::make_absolute_url(base_url, href))
            .unwrap_or_default();

        if url.is_empty() {
            return None;
        }

        // 提取作者
        let author = element
            .select(&Selector::parse(".author, .writer, .book-author").ok()?)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| String::from("未知作者"));

        // 提取封面
        let cover_url = element
            .select(&Selector::parse("img").ok()?)
            .next()
            .and_then(|img| img.value().attr("src"))
            .and_then(|src| Self::make_absolute_url(base_url, src))
            .unwrap_or_default();

        // 提取简介
        let description = element
            .select(&Selector::parse(".desc, .intro, .description, p").ok()?)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        // 提取最新章节
        let latest_chapter = element
            .select(&Selector::parse(".new-chapter, .latest, .update").ok()?)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| String::from("未知"));

        Some(SearchResult {
            title,
            author,
            url,
            cover_url,
            description,
            latest_chapter,
            source: source.to_string(),
        })
    }

    /// 获取小说详情和章节列表
    pub async fn get_novel_details(&self, url: &str) -> Result<(String, String, Vec<Chapter>, String), ScraperError> {
        info!("获取小说详情: {}", url);

        let html = self.fetch_page(url).await?;
        let document = Html::parse_document(&html);

        // 提取标题
        let title = self.extract_text_by_selectors(&document, &[
            ".book-title", ".novel-title", "h1.title", ".bookname h1", ".info h1"
        ]).unwrap_or_else(|| String::from("未知小说"));

        // 提取作者
        let author = self.extract_text_by_selectors(&document, &[
            ".author", ".writer", ".book-author", ".author-name"
        ]).unwrap_or_else(|| String::from("未知作者"));

        // 提取封面
        let cover_url = Selector::parse("img")
            .ok()
            .and_then(|sel| document.select(&sel).next())
            .and_then(|img| img.value().attr("src"))
            .and_then(|src| Self::make_absolute_url(url, src))
            .unwrap_or_default();

        // 提取简介
        let description = self.extract_text_by_selectors(&document, &[
            ".desc", ".intro", ".description", ".book-desc"
        ]).unwrap_or_default();

        // 提取章节列表
        let chapters = self.extract_chapters(&document, url);

        debug!("小说: {}, 作者: {}, 章节数: {}", title, author, chapters.len());

        Ok((title, author, chapters, description))
    }

    /// 根据选择器提取文本
    fn extract_text_by_selectors(&self, document: &Html, selectors: &[&str]) -> Option<String> {
        for selector_str in selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    let text = element.text().collect::<String>().trim().to_string();
                    if !text.is_empty() {
                        return Some(text);
                    }
                }
            }
        }
        None
    }

    /// 提取章节列表
    fn extract_chapters(&self, document: &Html, base_url: &str) -> Vec<Chapter> {
        let mut chapters = Vec::new();

        // 尝试多种章节列表选择器
        let selector_patterns = [
            ".chapter-list li a",
            ".chapter a",
            ".list a",
            "ul#chapter-list a",
            ".chapter-list a",
            ".catalog a",
            "#chapterlist a",
            ".volume-chapter a",
        ];

        for pattern in selector_patterns {
            if let Ok(selector) = Selector::parse(pattern) {
                let elements: Vec<_> = document.select(&selector).collect();
                if elements.len() > 5 {
                    debug!("使用选择器 {} 找到 {} 个章节", pattern, elements.len());

                    for (index, element) in elements.iter().enumerate() {
                        let title = element.text().collect::<String>().trim().to_string();
                        let url = element
                            .value()
                            .attr("href")
                            .and_then(|href| Self::make_absolute_url(base_url, href))
                            .unwrap_or_default();

                        if !title.is_empty() && !url.is_empty() {
                            chapters.push(Chapter {
                                title,
                                url,
                                index: index as u32,
                                word_count: None,
                            });
                        }
                    }

                    if !chapters.is_empty() {
                        break;
                    }
                }
            }
        }

        chapters
    }

    /// 获取章节内容
    pub async fn get_chapter_content(&self, url: &str) -> Result<(String, String), ScraperError> {
        info!("获取章节内容: {}", url);

        let html = self.fetch_page(url).await?;
        let document = Html::parse_document(&html);

        // 提取章节标题
        let title = self.extract_text_by_selectors(&document, &[
            ".chapter-title", ".title h1", "h1.title", ".read-title", ".content h1"
        ]).unwrap_or_else(|| String::from("未知章节"));

        // 提取正文内容
        let content = self.extract_content(&document);

        debug!("章节标题: {}, 内容长度: {} 字", title, content.len());

        Ok((title, content))
    }

    /// 提取正文内容
    fn extract_content(&self, document: &Html) -> String {
        // 尝试多种内容区域选择器
        let selector_patterns = [
            ".content", "#content", ".novel-content", ".chapter-content",
            ".read-content", ".book-content", "#chapter-content", ".text-content",
            ".article-content", ".post-content", ".entry-content"
        ];

        for pattern in selector_patterns {
            if let Ok(selector) = Selector::parse(pattern) {
                if let Some(element) = document.select(&selector).next() {
                    let html_content = element.html();
                    // 清理HTML标签，保留段落结构
                    let text = self.clean_html_content(&html_content);
                    if text.len() > 100 {
                        debug!("使用选择器 {} 提取到内容，长度: {}", pattern, text.len());
                        return text;
                    }
                }
            }
        }

        // 如果标准选择器都失败，尝试通用方法
        let body_selector = Selector::parse("body").ok();
        if let Some(body) = body_selector.and_then(|s| document.select(&s).next()) {
            let html = body.html();
            return self.clean_html_content(&html);
        }

        String::new()
    }

    /// 清理HTML内容
    fn clean_html_content(&self, html: &str) -> String {
        let document = Html::parse_fragment(html);

        // 移除脚本和样式
        let scripts = Selector::parse("script, style, noscript, iframe, form").unwrap();
        let mut result = String::new();

        for element in document.root_element().children() {
            if let Some(element_ref) = scraper::ElementRef::wrap(element) {
                let tag_name = element_ref.value().name();
                if tag_name == "p" || tag_name == "div" || tag_name == "br" {
                    let text = element_ref.text().collect::<String>().trim().to_string();
                    if !text.is_empty() {
                        result.push_str(&text);
                        result.push_str("\n\n");
                    }
                }
            }
        }

        // 如果上面的方法失败，直接提取所有文本
        if result.is_empty() {
            result = document.root_element()
                .text()
                .collect::<Vec<_>>()
                .join("\n")
                .lines()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty() && s.len() > 10)
                .collect::<Vec<_>>()
                .join("\n\n");
        }

        result
    }
}

impl Default for WebScraper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_absolute_url() {
        let base = "https://example.com/book/123";
        let href1 = "/chapter/456";
        let href2 = "https://other.com/test";
        let href3 = "relative/path";

        assert_eq!(
            WebScraper::make_absolute_url(base, href1),
            Some(String::from("https://example.com/chapter/456"))
        );
        assert_eq!(
            WebScraper::make_absolute_url(base, href2),
            Some(String::from("https://other.com/test"))
        );
        assert!(
            WebScraper::make_absolute_url(base, href3).is_some()
        );
    }
}
