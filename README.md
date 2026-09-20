# 小说阅读器 - Rust + Tauri

基于 Rust 和 Tauri 2.0 构建的跨平台小说阅读应用，支持输入网站地址搜索小说，智能去除广告并重新排版。

## 功能特性

- 输入任意小说网站 URL 进行搜索
- 智能解析网页内容，提取小说信息
- 自动去除页面广告，还原纯净阅读体验
- 多种排版主题（白天/夜间/护眼）
- 字体大小、行距、段间距可调节
- 封面图来源切换（原始/Google图片/百度图片）
- 阅读进度自动保存
- 书架管理功能

## 技术栈

- **后端**：Rust + Tauri 2.0
- **前端**：Vue 3 + TypeScript + Vite
- **HTTP 客户端**：reqwest
- **HTML 解析**：scraper
- **数据存储**：本地 JSON 文件

## 项目结构

```
novel_reader/
├── src-tauri/                 # Rust 后端代码
│   ├── src/
│   │   ├── main.rs          # 应用入口
│   │   ├── lib.rs           # 库入口
│   │   ├── commands/         # Tauri 命令
│   │   │   ├── search.rs    # 搜索功能
│   │   │   ├── fetch.rs     # 内容获取
│   │   │   ├── cleaner.rs   # 广告清理
│   │   │   ├── formatter.rs  # 内容格式化
│   │   │   ├── cover.rs     # 封面管理
│   │   │   └── storage.rs   # 本地存储
│   │   ├── core/            # 核心业务逻辑
│   │   │   ├── scraper.rs   # 网页爬虫
│   │   │   ├── cleaner.rs   # 广告清理器
│   │   │   └── formatter.rs  # 格式化器
│   │   ├── models/          # 数据模型
│   │   └── utils/           # 工具函数
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                      # Vue 前端代码
│   ├── App.vue              # 主组件
│   ├── main.ts             # 入口文件
│   └── styles/             # 样式文件
├── package.json
├── vite.config.ts
├── tsconfig.json
└── SPEC.md                  # 技术规格说明
```

## 环境要求

- Rust 1.70+
- Node.js 18+
- npm 或 pnpm
- Android SDK (用于 Android 构建)

## 开发环境搭建

### 1. 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. 安装 Node.js

推荐使用 nvm 安装：

```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18
nvm use 18
```

### 3. 安装 Android SDK

下载 Android Studio 或命令行工具：

```bash
# Linux
sudo apt install android-sdk

# 或手动下载
wget https://dl.google.com/android/repository/commandlinetools-linux-9477386_latest.zip
unzip commandlinetools-linux-9477386_latest.zip -d ~/android-sdk
export ANDROID_SDK_ROOT=~/android-sdk
```

## 构建步骤

### 1. 克隆项目

```bash
git clone <repository_url>
cd novel_reader
```

### 2. 安装前端依赖

```bash
npm install
```

### 3. 运行开发模式

```bash
npm run tauri dev
```

这将启动 Vite 开发服务器和 Tauri 应用。

### 4. 构建发布版本

#### Android APK

```bash
npm run tauri build
```

APK 文件将生成在 `src-tauri/target/release/bundle/apk/` 目录。

#### Windows/macOS/Linux

```bash
npm run build
```

可执行文件将生成在 `src-tauri/target/release/` 目录。

## 使用说明

### 搜索小说

1. 在顶部输入框输入小说网站 URL（如 `https://example.com`）
2. 在下方输入框输入要搜索的小说名称
3. 点击"搜索"按钮
4. 从搜索结果中选择想要阅读的小说

### 阅读小说

1. 点击小说卡片进入详情页
2. 在目录中选择想要阅读的章节
3. 进入阅读界面，开始阅读
4. 点击"上一章"/"下一章"切换章节
5. 点击"目录"查看所有章节

### 设置阅读偏好

1. 在阅读界面点击右上角设置图标
2. 调整字体大小、行高、段间距
3. 选择阅读主题（白天/夜间/护眼）
4. 开关首行缩进

### 切换封面来源

1. 在小说详情页点击右上角相机图标
2. 选择封面图片来源（原始/Google图片/百度图片）
3. 封面将自动更新

## 广告清理机制

应用采用多层次广告清理策略：

1. **CSS 选择器过滤**：移除常见的广告容器
2. **文本关键词检测**：识别含广告关键词的元素
3. **脚本清理**：移除广告脚本和追踪代码
4. **空标签清理**：清理无用的空标签

## 性能优化

- 异步 HTTP 请求，避免阻塞
- 内容解析使用流式处理
- 本地缓存已读章节
- 图片懒加载

## 常见问题

### Q: 搜索不到想要的小说？

A: 请确保：
1. 输入的网站 URL 正确且可访问
2. 网站允许被抓取（未进行反爬限制）
3. 尝试更换不同的关键词

### Q: 章节内容为空？

A: 部分网站可能使用了特殊的反爬机制，可以：
1. 等待一段时间后重试
2. 尝试其他小说网站
3. 查看是否为动态加载内容

### Q: 封面图片无法显示？

A: 可以尝试：
1. 切换封面图片来源
2. 检查网络连接
3. 手动输入封面 URL

## 开发指南

### 添加新的网站支持

在 `src-tauri/src/core/scraper.rs` 中的 `parse_search_results` 方法添加该网站的选择器模式。

### 自定义广告清理规则

在 `src-tauri/src/core/cleaner.rs` 的 `default_patterns` 方法中添加新的广告模式。

### 修改排版样式

在 `src-tauri/src/models/mod.rs` 的 `FormatConfig` 结构体中调整默认配置。

## 许可证

本项目仅供学习交流使用，请勿用于商业用途。

## 致谢

- 感谢 [Legado](https://github.com/gedoor/legado) 项目提供的灵感
- 使用 [Tauri](https://tauri.app/) 框架构建
- 使用 [Vue](https://vuejs.org/) 框架构建前端界面
