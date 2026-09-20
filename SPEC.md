# 小说阅读器 - 技术规格说明书

## 1. 项目概述

**项目名称**：Rust Novel Reader（Rust小说阅读器）

**项目类型**：跨平台移动应用（Android为主）

**核心功能**：输入网站地址，在目标网站搜索小说，智能化解析内容，自动去除广告并重新排版，支持封面图来源切换。

**目标用户**：喜欢阅读网络小说的用户，需要干净、无广告阅读体验的读者。

## 2. 技术架构

### 2.1 技术栈

| 层级 | 技术选型 | 说明 |
|------|---------|------|
| 移动框架 | Tauri 2.0 | Rust原生开发，支持Android/iOS |
| 后端核心 | Rust | 高性能网页抓取和内容处理 |
| HTTP客户端 | reqwest | 异步HTTP请求库 |
| HTML解析 | scraper | HTML解析和DOM操作 |
| 正则匹配 | regex | 广告特征匹配和内容提取 |
| 前端界面 | Vue 3 + TypeScript | 现代化响应式界面 |
| 构建工具 | Vite | 快速前端构建 |

### 2.2 模块架构

```
novel_reader/
├── src-tauri/                 # Rust后端代码
│   ├── src/
│   │   ├── main.rs           # 应用入口
│   │   ├── lib.rs            # 库入口
│   │   ├── commands/         # Tauri命令模块
│   │   │   ├── mod.rs
│   │   │   ├── search.rs      # 小说搜索
│   │   │   ├── fetch.rs       # 内容获取
│   │   │   ├── parser.rs      # 内容解析
│   │   │   └── storage.rs     # 本地存储
│   │   ├── core/             # 核心业务逻辑
│   │   │   ├── mod.rs
│   │   │   ├── scraper.rs     # 爬虫核心
│   │   │   ├── cleaner.rs     # 广告清理
│   │   │   └── formatter.rs   # 内容格式化
│   │   ├── models/            # 数据模型
│   │   │   └── mod.rs
│   │   └── utils/             # 工具函数
│   │       └── mod.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                       # 前端源代码
│   ├── components/           # Vue组件
│   ├── views/                # 页面视图
│   ├── stores/               # 状态管理
│   ├── styles/               # 样式文件
│   ├── App.vue
│   └── main.ts
├── index.html
└── package.json
```

## 3. 功能模块详细设计

### 3.1 网站输入与搜索模块

**功能描述**：用户输入小说网站URL，系统自动识别网站类型并执行搜索。

**核心流程**：
1. 用户输入目标网站URL
2. 系统验证URL格式和可访问性
3. 自动识别网站编码（UTF-8/GBK/GB2312）
4. 构建搜索请求（GET/POST）
5. 获取搜索结果页面
6. 解析搜索结果列表

**数据模型**：
```rust
pub struct WebsiteConfig {
    pub url: String,              // 网站基础URL
    pub name: String,             // 网站名称
    pub encoding: String,        // 编码格式
    pub search_path: String,      // 搜索路径
    pub search_key: String,       // 搜索参数名
}

pub struct SearchResult {
    pub title: String,            // 小说标题
    pub author: String,           // 作者
    pub url: String,              // 详情页URL
    pub cover_url: String,       // 封面图URL
    pub description: String,      // 简介
    pub latest_chapter: String,   // 最新章节
}
```

### 3.2 内容解析模块

**功能描述**：深度解析小说页面，提取正文内容、章节列表等关键信息。

**解析策略**：
1. **DOM树分析**：定位内容容器节点
2. **文本密度算法**：计算每个节点的文本密度，识别正文区域
3. **特征匹配**：基于常见小说网站的CSS选择器模式
4. **智能推断**：对于未知网站，使用启发式算法推断内容区域

**章节解析**：
- 提取章节列表页面
- 解析每章标题和URL
- 支持多级目录结构
- 处理分页章节内容

### 3.3 广告清理模块

**功能描述**：智能识别并移除网页广告，还原纯净阅读内容。

**广告识别策略**：

| 广告类型 | 识别特征 | 清除策略 |
|---------|---------|---------|
| 浮动广告 | position:fixed/absolute, z-index高 | 移除整个元素 |
| 弹窗广告 | 触发式显示, overlay层 | 移除trigger和overlay |
| 横幅广告 | 固定位置, 图片/iframe | 移除img/iframe |
| 嵌入式广告 | div内含特定广告关键词 | 正则匹配移除 |
| 脚本广告 | script标签内的广告代码 | 移除危险脚本 |

**核心算法**：
```rust
pub struct AdPattern {
    pub name: String,              // 广告名称
    pub selector: String,          // CSS选择器
    pub regex_pattern: Option<Regex>, // 正则匹配
    pub text_keywords: Vec<String>, // 文本关键词
    pub weight: i32,              // 匹配权重
}

impl AdCleaner {
    // 多策略广告检测
    pub fn detect_ads(&self, element: &Element) -> Vec<AdMatch>;

    // 递归清理广告元素
    pub fn clean_recursive(&self, root: &mut Element);

    // 内容保护（不清理正文区域）
    pub fn protect_content(&self, content_area: &Element);
}
```

### 3.4 内容排版模块

**功能描述**：重新格式化小说内容，提供优秀的阅读体验。

**排版配置**：
```rust
pub struct FormatConfig {
    pub font_family: String,       // 字体系列
    pub font_size: u32,           // 字号(px)
    pub line_height: f32,          // 行高倍数
    pub paragraph_spacing: f32,    // 段间距(em)
    pub text_align: String,        // 文本对齐
    pub text_indent: bool,         // 首行缩进
    pub margin_horizontal: u32,   // 水平边距
    pub margin_vertical: u32,      // 垂直边距
}

pub struct FormattedChapter {
    pub title: String,             // 章节标题
    pub content: String,           // 格式化后的正文
    pub prev_url: Option<String>,  // 上一章
    pub next_url: Option<String>,  // 下一章
}
```

**格式化规则**：
1. 智能分段：基于段落标记和长度
2. 去除冗余空行：合并连续空行
3. 标点规范化：统一全角/半角标点
4. HTML清理：移除危险的HTML标签
5. 样式注入：添加阅读友好的CSS样式

### 3.5 封面图管理模块

**功能描述**：支持多来源封面图切换，提供灵活的封面获取策略。

**封面来源策略**：
```rust
pub enum CoverSource {
    Original,      // 原始封面
    GoogleImages,  // Google图片搜索
    BaiduImages,   // 百度图片搜索
    Custom(String), // 自定义API
}

pub struct CoverConfig {
    pub source: CoverSource,
    pub fallback_enabled: bool,   // 启用备用源
    pub cache_enabled: bool,      // 启用缓存
    pub size_preference: String,   // 图片尺寸偏好
}
```

**功能特性**：
- 一键切换封面来源
- 智能选择最优尺寸
- 本地缓存加速加载
- 支持手动输入封面URL

### 3.6 本地存储模块

**功能描述**：管理书架、阅读进度、用户设置等数据。

**存储内容**：
| 数据类型 | 存储方式 | 说明 |
|---------|---------|------|
| 书架数据 | SQLite | 收藏的小说列表 |
| 阅读进度 | SQLite | 每本书的阅读位置 |
| 用户设置 | JSON文件 | 阅读偏好配置 |
| 缓存内容 | 文件系统 | 已下载的章节内容 |

## 4. 前端界面设计

### 4.1 页面结构

```
┌─────────────────────────────┐
│        顶部导航栏            │
│  [网站输入框] [搜索按钮]    │
├─────────────────────────────┤
│                             │
│        搜索结果列表          │
│  ┌─────┬─────────────────┐ │
│  │封面 │ 书名：xxx        │ │
│  │图片 │ 作者：xxx        │ │
│  └─────┴─────────────────┘ │
│                             │
├─────────────────────────────┤
│        阅读器页面            │
│  章节标题                   │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━ │
│  正文内容...                │
│  正文内容...                │
│                             │
│  [上一章] [目录] [下一章]   │
└─────────────────────────────┘
```

### 4.2 核心组件

| 组件名称 | 功能描述 | 状态管理 |
|---------|---------|---------|
| SearchBar | 网站URL输入和搜索触发 | searchStore |
| NovelList | 搜索结果展示列表 | searchStore |
| NovelCard | 单本小说信息卡片 | - |
| ChapterList | 章节列表弹窗 | readerStore |
| Reader | 阅读器主界面 | readerStore |
| ReaderToolbar | 阅读器工具栏 | readerStore |
| SettingsPanel | 设置面板 | settingsStore |
| CoverSwitcher | 封面切换器 | coverStore |

### 4.3 交互流程

**搜索流程**：
1. 用户输入URL → 2. 点击搜索 → 3. 显示加载状态 → 4. 解析结果 → 5. 展示列表

**阅读流程**：
1. 点击小说卡片 → 2. 加载详情 → 3. 显示章节列表 → 4. 选择章节 → 5. 加载内容 → 6. 渲染阅读器

## 5. 数据流设计

### 5.1 状态管理（Pinia）

```typescript
// 搜索状态
interface SearchState {
  websiteUrl: string;
  searchResults: SearchResult[];
  loading: boolean;
  error: string | null;
}

// 阅读状态
interface ReaderState {
  currentNovel: Novel | null;
  currentChapter: Chapter | null;
  progress: ReadingProgress;
  formatConfig: FormatConfig;
}

// 设置状态
interface SettingsState {
  coverSource: CoverSource;
  theme: 'light' | 'dark';
  fontSize: number;
  lineHeight: number;
}
```

### 5.2 Tauri命令接口

```rust
#[tauri::command]
async fn search_novels(url: String, keyword: String) -> Result<Vec<SearchResult>, String>;

#[tauri::command]
async fn get_chapter_content(url: String) -> Result<FormattedChapter, String>;

#[tauri::command]
async fn get_chapter_list(url: String) -> Result<Vec<Chapter>, String>;

#[tauri::command]
async fn change_cover_source(novel_title: String, source: String) -> Result<String, String>;

#[tauri::command]
async fn save_to_bookshelf(novel: Novel) -> Result<(), String>;

#[tauri::command]
async fn get_bookshelf() -> Result<Vec<Novel>, String>;
```

## 6. 性能优化策略

### 6.1 网络优化
- 异步HTTP请求（reqwest异步runtime）
- 请求超时控制（默认30秒）
- 自动重试机制（最多3次）
- 连接池复用

### 6.2 内容处理优化
- 多线程并行解析（ rayon库）
- 增量解析（处理大数据页面）
- 流式处理（避免全量加载内存）

### 6.3 前端优化
- 虚拟滚动（长列表优化）
- 图片懒加载
- 本地缓存（已读章节）

## 7. 错误处理机制

### 7.1 网络错误
| 错误类型 | 处理策略 |
|---------|---------|
| 连接超时 | 显示友好提示，允许重试 |
| 404错误 | 提示页面不存在 |
| 500错误 | 提示服务器错误 |
| 解析错误 | 记录日志，返回部分数据 |

### 7.2 内容错误
| 错误类型 | 处理策略 |
|---------|---------|
| 编码错误 | 自动尝试多种编码 |
| 广告清理失败 | 保留原始内容，标记警告 |
| 内容为空 | 提示内容获取失败 |

## 8. 安全考虑

### 8.1 隐私保护
- 不收集用户阅读数据
- 本地存储加密（敏感信息）
- 网络请求不携带追踪参数

### 8.2 内容安全
- 移除危险脚本（XSS防护）
- 过滤恶意链接
- 不执行外部JavaScript

### 8.3 权限控制
- 仅请求必要权限
- Android权限最小化
- 用户授权明确提示

## 9. 未来扩展方向

1. **书源市场**：用户分享和导入书源规则
2. **云同步**：跨设备阅读进度同步
3. **离线阅读**：完整下载小说到本地
4. **朗读功能**：文字转语音（TTS）
5. **社区功能**：书评、笔记分享
