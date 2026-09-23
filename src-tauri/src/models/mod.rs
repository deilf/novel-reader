//! 数据模型模块
//!
//! 定义应用中使用的数据结构和类型

use serde::{Deserialize, Serialize};

/// 网站配置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteConfig {
    /// 网站基础URL
    pub url: String,
    /// 网站名称
    pub name: String,
    /// 编码格式
    pub encoding: String,
    /// 搜索路径
    pub search_path: String,
    /// 搜索参数名
    pub search_key: String,
    /// 编码检测选择器
    pub encoding_selector: Option<String>,
}

impl Default for WebsiteConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            name: String::from("未知网站"),
            encoding: String::from("UTF-8"),
            search_path: String::from("/search"),
            search_key: String::from("keyword"),
            encoding_selector: None,
        }
    }
}

/// 搜索结果结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// 小说标题
    pub title: String,
    /// 作者名称
    pub author: String,
    /// 详情页URL
    pub url: String,
    /// 封面图URL
    pub cover_url: String,
    /// 小说简介
    pub description: String,
    /// 最新章节
    pub latest_chapter: String,
    /// 来源网站
    pub source: String,
}

impl Default for SearchResult {
    fn default() -> Self {
        Self {
            title: String::new(),
            author: String::from("未知作者"),
            url: String::new(),
            cover_url: String::new(),
            description: String::new(),
            latest_chapter: String::from("未知"),
            source: String::new(),
        }
    }
}

/// 小说详情结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelDetails {
    /// 小说标题
    pub title: String,
    /// 作者
    pub author: String,
    /// 封面URL
    pub cover_url: String,
    /// 简介
    pub description: String,
    /// 分类
    pub category: String,
    /// 状态
    pub status: String,
    /// 章节列表
    pub chapters: Vec<Chapter>,
    /// 来源URL
    pub source_url: String,
}

impl Default for NovelDetails {
    fn default() -> Self {
        Self {
            title: String::new(),
            author: String::from("未知作者"),
            cover_url: String::new(),
            description: String::new(),
            category: String::from("未知分类"),
            status: String::from("连载中"),
            chapters: Vec::new(),
            source_url: String::new(),
        }
    }
}

/// 章节信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    /// 章节标题
    pub title: String,
    /// 章节URL
    pub url: String,
    /// 章节序号
    pub index: u32,
    /// 字数统计
    pub word_count: Option<u32>,
}

impl Default for Chapter {
    fn default() -> Self {
        Self {
            title: String::new(),
            url: String::new(),
            index: 0,
            word_count: None,
        }
    }
}

/// 格式化后的章节内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattedChapter {
    /// 章节标题
    pub title: String,
    /// 格式化后的正文
    pub content: String,
    /// 上一章URL
    pub prev_url: Option<String>,
    /// 下一章URL
    pub next_url: Option<String>,
    /// 来源URL
    pub source_url: String,
    /// 字数
    pub word_count: u32,
}

impl Default for FormattedChapter {
    fn default() -> Self {
        Self {
            title: String::new(),
            content: String::new(),
            prev_url: None,
            next_url: None,
            source_url: String::new(),
            word_count: 0,
        }
    }
}

/// 格式化配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatConfig {
    /// 字体系列
    pub font_family: String,
    /// 字号（像素）
    pub font_size: u32,
    /// 行高倍数
    pub line_height: f32,
    /// 段间距（em）
    pub paragraph_spacing: f32,
    /// 文本对齐方式
    pub text_align: String,
    /// 首行缩进
    pub text_indent: bool,
    /// 水平边距
    pub margin_horizontal: u32,
    /// 垂直边距
    pub margin_vertical: u32,
    /// 背景颜色
    pub background_color: String,
    /// 文字颜色
    pub text_color: String,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            font_family: String::from("系统默认"),
            font_size: 18,
            line_height: 1.8,
            paragraph_spacing: 1.5,
            text_align: String::from("justify"),
            text_indent: true,
            margin_horizontal: 16,
            margin_vertical: 20,
            background_color: String::from("#f5f5f5"),
            text_color: String::from("#333333"),
        }
    }
}

/// 封面图片来源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoverSource {
    /// 原始封面
    Original,
    /// Google图片搜索
    GoogleImages,
    /// 百度图片搜索
    BaiduImages,
    /// 自定义API
    Custom(String),
}

impl Default for CoverSource {
    fn default() -> Self {
        Self::Original
    }
}

/// 阅读进度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingProgress {
    /// 小说URL
    pub novel_url: String,
    /// 小说标题
    pub novel_title: String,
    /// 当前章节URL
    pub chapter_url: String,
    /// 当前章节标题
    pub chapter_title: String,
    /// 章节索引
    pub chapter_index: u32,
    /// 滚动位置百分比
    pub scroll_position: f32,
    /// 最后阅读时间
    pub last_read_time: String,
}

impl Default for ReadingProgress {
    fn default() -> Self {
        Self {
            novel_url: String::new(),
            novel_title: String::new(),
            chapter_url: String::new(),
            chapter_title: String::new(),
            chapter_index: 0,
            scroll_position: 0.0,
            last_read_time: chrono_now(),
        }
    }
}

/// 书架小说
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookshelfNovel {
    /// 小说标题
    pub title: String,
    /// 作者
    pub author: String,
    /// 封面URL
    pub cover_url: String,
    /// 详情页URL
    pub url: String,
    /// 来源网站
    pub source: String,
    /// 添加时间
    pub added_time: String,
    /// 最新章节
    pub latest_chapter: String,
    /// 阅读进度
    pub progress: Option<ReadingProgress>,
}

impl Default for BookshelfNovel {
    fn default() -> Self {
        Self {
            title: String::new(),
            author: String::from("未知作者"),
            cover_url: String::new(),
            url: String::new(),
            source: String::new(),
            added_time: chrono_now(),
            latest_chapter: String::from("未知"),
            progress: None,
        }
    }
}

/// 应用设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// 封面来源
    pub cover_source: CoverSource,
    /// 主题模式
    pub theme: String,
    /// 格式化配置
    pub format_config: FormatConfig,
    /// 启用缓存
    pub cache_enabled: bool,
    /// 缓存大小限制（MB）
    pub cache_size_limit: u32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            cover_source: CoverSource::Original,
            theme: String::from("light"),
            format_config: FormatConfig::default(),
            cache_enabled: true,
            cache_size_limit: 500,
        }
    }
}

/// 获取当前时间字符串
fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", duration.as_secs())
}

// ==================== 书源模型（兼容 Legado 格式） ====================

/// 搜索规则
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SearchRule {
    pub bookList: Option<String>,
    pub name: Option<String>,
    pub author: Option<String>,
    pub coverUrl: Option<String>,
    pub bookUrl: Option<String>,
    pub intro: Option<String>,
    pub kind: Option<String>,
    pub lastChapter: Option<String>,
}

/// 书籍信息规则
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BookInfoRule {
    pub name: Option<String>,
    pub author: Option<String>,
    pub coverUrl: Option<String>,
    pub intro: Option<String>,
    pub kind: Option<String>,
    pub lastChapter: Option<String>,
    pub init: Option<String>,
}

/// 目录规则
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TocRule {
    pub chapterList: Option<String>,
    pub chapterName: Option<String>,
    pub chapterUrl: Option<String>,
    pub nextTocUrl: Option<String>,
}

/// 正文规则
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ContentRule {
    pub content: Option<String>,
    pub nextContent: Option<String>,
}

/// 书源（字段与 Legado 导出的书源 JSON 保持一致）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BookSource {
    pub bookSourceUrl: String,
    pub bookSourceName: String,
    pub bookSourceGroup: Option<String>,
    pub bookSourceType: i32,
    pub bookUrlPattern: Option<String>,
    pub customOrder: i32,
    pub enabled: bool,
    pub enabledExplore: bool,
    pub jsLib: Option<String>,
    pub enabledCookieJar: Option<bool>,
    pub concurrentRate: Option<String>,
    pub header: Option<String>,
    pub loginUrl: Option<String>,
    pub loginUi: Option<String>,
    pub loginCheckJs: Option<String>,
    pub coverDecodeJs: Option<String>,
    pub bookSourceComment: Option<String>,
    pub variableComment: Option<String>,
    pub lastUpdateTime: i64,
    pub respondTime: i64,
    pub weight: i32,
    pub exploreUrl: Option<String>,
    pub exploreScreen: Option<String>,
    pub ruleExplore: SearchRule,
    pub searchUrl: Option<String>,
    pub ruleSearch: SearchRule,
    pub ruleBookInfo: BookInfoRule,
    pub ruleToc: TocRule,
    pub ruleContent: ContentRule,
    pub ruleReview: SearchRule,
    pub eventListener: bool,
    pub customButton: bool,
    pub nextPageLazyLoad: bool,
    pub homepageModules: Option<String>,
}

/// 通用 HTTP 请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    pub url: String,
    pub method: Option<String>,
    pub headers: Option<std::collections::HashMap<String, String>>,
    pub body: Option<String>,
    pub timeout_ms: Option<u64>,
}

/// 通用 HTTP 响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,
    pub final_url: String,
    pub content_type: String,
    pub body: String,
}
