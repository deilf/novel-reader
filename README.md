# 小说阅读器（Novel Reader）

基于 **Rust + Tauri 2 + Vue 3** 的跨平台（Android / Windows / macOS / Linux）小说阅读应用。内置与 **Legado（阅读）书源生态兼容**的规则引擎：导入 Legado 导出的书源 JSON 即可搜索、解析目录与正文，并支持本地 TXT / EPUB 阅读。

> 本项目为对开源阅读器 Legado_Max 的功能性重写（Rust/Tauri 技术栈重构），引擎为自研 TypeScript 实现，未直接复制其 Kotlin 源码。

---

## 功能特性

### 书源引擎（M1 · 兼容 Legado 书源）
- 导入 Legado 书源 JSON（粘贴 / 分享 URL 拉取 / 导出备份）
- 规则引擎支持：`{{$.css()}}` / `{{$.xpath()}}` / `{{$.regex()}}` / `{{$.json()}}` / `{{js.代码}}` / `{{java.表达式}}`、`@js:` / `@css:` 前缀、`||` 分段拼接、URL 模板变量（`{{key}}` / `{{page}}`）
- 搜索 → 详情+目录 → 正文全链路，支持 `nextContent` / `nextTocUrl` 翻页、正文净化
- **jsLib 公共脚本**（M5）：书源可携带公共 JS 库，规则代码直接调用其函数

### 书源管理（M4）
- 可视化**规则编辑器**：基本信息 / 搜索规则 / 目录规则 / 正文规则分区块表单编辑
- 编辑器内**搜索测试**：实时验证当前规则
- 导入：粘贴 JSON、**分享 URL 拉取**；导出：一键复制全部书源

### 书架与阅读体验（M2 / M5）
- 书架：加入/移除/续读定位（自动跳转上次章节）
- 阅读进度自动保存（章节级）
- 书签（章节级增删）
- 替换净化规则（正则/文本，作用域 正文/目录/全部）
- 简繁转换（opencc-js 懒加载）
- 阅读主题：日间 / 羊皮纸 / 夜间（本地持久化）；字号调节
- 顶部/底部安全区适配（Android 状态栏与手势条）

### 本地书籍（M3）
- 导入 TXT（章节正则切分、GBK/UTF-8/BOM 自动解码）
- 导入 EPUB（container.xml → OPF → spine 顺序解析，去样式/脚本/标签、实体解码）
- 本地阅读器：目录 / 上下章 / 字号

### 工程化
- GitHub Actions 自动构建 + 签名 Android APK（tag 发布 Release）
- Rust 单元/集成测试、前端规则引擎回归测试

---

## 技术栈

| 层 | 技术 |
| --- | --- |
| 桌面/移动壳 | Tauri 2（Android / Windows / macOS / Linux） |
| 前端 | Vue 3 + TypeScript + Vite + Pinia |
| Rust | reqwest（HTTP）、scraper（HTML 解析）、encoding_rs（编码）、zip + roxmltree（EPUB）、serde_json |
| 书源规则引擎 | 自研 TypeScript（WebView 内执行，兼容 Legado 语法） |
| 简繁转换 | opencc-js |

## 项目结构

```
novel-reader/
├── src/                          # 前端（Vue 3）
│   ├── App.vue                 # 五标签导航（发现/书架/本地书/净化/书源）
│   ├── components/             # 视图组件
│   │   ├── DiscoverView.vue    # 发现/搜索
│   │   ├── DetailView.vue      # 书籍详情 + 目录
│   │   ├── ReaderView.vue      # 在线阅读（净化/简繁/书签/进度/主题）
│   │   ├── BookshelfView.vue   # 书架
│   │   ├── LocalBooksView.vue  # 本地书籍列表/导入
│   │   ├── LocalReaderView.vue # 本地阅读
│   │   ├── ReplaceRulesView.vue# 替换净化规则管理
│   │   ├── SourceManager.vue   # 书源管理（导入/导出/启停）
│   │   └── SourceEditorView.vue# 书源规则编辑器（含测试）
│   └── lib/bookSource/         # 书源引擎
│       ├── engine.ts           # 规则引擎（css/xpath/regex/json/js/模板）
│       ├── runner.ts           # 搜索/详情/正文流程 + jsLib 加载
│       ├── http.ts             # Rust http_fetch 封装
│       ├── store.ts            # Rust 命令封装（书源/书签/替换/本地书）
│       ├── shelf.ts            # 书架/进度
│       ├── replace.ts          # 净化规则应用
│       └── traditional.ts      # 简繁转换
├── src-tauri/                   # Rust 后端
│   └── src/
│       ├── commands/           # Tauri 命令
│       │   ├── http.rs         # http_fetch（编码检测/UA/超时）
│       │   ├── book_source.rs  # 书源 CRUD
│       │   ├── bookmark.rs     # 书签
│       │   ├── replace_rule.rs # 替换规则
│       │   ├── local_book.rs   # TXT/EPUB 导入与章节
│       │   ├── storage.rs      # 书架/进度
│       │   └── ...
│       ├── core/               # scraper/cleaner/formatter
│       ├── models/             # 数据模型（Legado 兼容）
│       └── utils/data_dir.rs   # 数据目录
├── scripts/                     # 测试脚本（test-engine/test-m23/m4/m5）
└── .github/workflows/build.yml  # CI：构建+签名 APK
```

## 快速开始

### 桌面开发

```bash
# 需要 Rust、Node 18+、系统 WebKitGTK 4.1（Linux）
npm install
npm run tauri dev        # 开发运行
npm run build            # 前端构建（vue-tsc 类型检查）
npm run test:engine      # 规则引擎回归测试
cargo test --manifest-path src-tauri/Cargo.toml   # Rust 测试
```

### Android APK

本地交叉编译需 NDK；更简单的方式是推送到 GitHub 触发 CI：

```bash
git push origin main     # 自动构建 + 签名，产物见 Actions artifact
git tag v1.0.0 && git push origin v1.0.0   # 额外发布 GitHub Release
```

签名密钥通过仓库 Secrets 注入（`KEYSTORE_BASE64` / `KEYSTORE_PASSWORD` / `KEY_ALIAS` / `KEY_PASSWORD`），密钥文件不进入仓库。

## 书源使用

1. 书源页 → 「导入书源」：粘贴 Legado 导出的书源 JSON（数组或单个对象），或粘贴书源分享 URL
2. 发现页搜索 → 选择书源结果 → 详情/目录 → 阅读
3. 书源页「编辑」可修改任意规则字段并即时测试搜索
4. 书源 JSON 字段与 Legado 导出格式完全兼容（`bookSourceUrl` / `searchUrl` / `ruleSearch` / `ruleToc` / `ruleContent` / `ruleBookInfo` / `jsLib` / `header` …）

## 数据存储

应用数据存于系统数据目录的 `novel_reader/` 下（JSON 文件）：

```
novel_reader/
├── book_sources.json      # 书源
├── bookmarks.json         # 书签
├── replace_rules.json     # 替换净化规则
├── local_books.json       # 本地书籍元数据
├── bookshelf.json         # 书架
├── reading_progress.json  # 阅读进度
└── books/<id>.chapters.txt  # 本地书归一化章节文件
```

## 测试

| 套件 | 命令 | 覆盖 |
| --- | --- | --- |
| `test:engine` | `npm run test:engine` | 规则引擎全链路（CSS/JSON 双路径、翻页、净化） |
| `test-m23` | `node --import tsx scripts/test-m23.mjs` | 替换净化、简繁转换 |
| `test-m4` | `node --import tsx scripts/test-m4.mjs` | 书源导入格式、URL 导入链路 |
| `test-m5` | `node --import tsx scripts/test-m5.mjs` | jsLib 注入、缓存、裸选择器语法 |
| Rust | `cargo test --manifest-path src-tauri/Cargo.toml` | TXT 切分、GBK/UTF-8、EPUB 解析（含真实文件集成测试） |

## 许可与合规

- 本项目为对 Legado_Max 的功能性重写：未直接复制其 Kotlin 源码，规则引擎为独立实现，书源数据格式与 Legado 导出格式互通。
- 书源由用户自行配置与维护；请仅使用你有权访问的内容源。

## Roadmap

- [x] M1 书源驱动引擎（搜索/目录/正文 + 规则引擎）
- [x] M2 书架/进度/书签/替换净化/简繁转换
- [x] M3 本地 TXT/EPUB
- [x] M4 书源规则编辑器 + URL 导入/导出
- [x] M5 jsLib 支持 + 阅读主题 + Android 安全区修复
- [ ] 登录/cookie 会话、探索页、书源市场
- [ ] 桌面端一键打包分发
