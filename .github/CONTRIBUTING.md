# 贡献指南

感谢您有兴趣为小说阅读器项目做出贡献！

## 开发环境设置

### 前置要求

- Rust 1.70+
- Node.js 18+
- npm 或 pnpm
- Android SDK（用于 Android 构建）

### 克隆并安装依赖

```bash
# 克隆仓库
git clone <repository-url>
cd novel_reader

# 安装 Node.js 依赖
npm install

# 安装 Rust 依赖
cargo build
```

### 运行开发模式

```bash
npm run tauri dev
```

## 开发流程

### 1. 创建分支

```bash
git checkout -b feature/your-feature-name
# 或
git checkout -b fix/bug-description
```

### 2. 进行更改

在您的分支上进行更改。请确保：

- 遵循项目的代码风格
- 添加必要的测试
- 更新相关文档
- 保持提交信息清晰明了

### 3. 提交更改

我们使用 Conventional Commits 格式：

```
<type>(<scope>): <subject>

feat(search): 添加小说搜索功能
fix(reader): 修复阅读器闪退问题
docs(readme): 更新使用文档
```

类型包括：
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更改
- `style`: 代码格式（不影响功能）
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建/工具相关

### 4. 推送并创建 Pull Request

```bash
git push origin feature/your-feature-name
```

然后在 GitHub 上创建 Pull Request。

## 代码规范

### Rust 代码

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码
- 遵循 Rust 所有权和生命周期规则

### Vue/TypeScript 代码

- 遵循 Vue 3 组合式 API 规范
- 使用 TypeScript 严格模式
- 组件文件使用 PascalCase 命名

### Git 提交规范

- 提交信息使用中文或英文
- 主题不超过 72 个字符
- 描述要清楚为什么要做这个更改

## 测试

### 运行测试

```bash
# Rust 测试
cargo test

# 前端类型检查
npm run build
```

## Pull Request 审查流程

1. 自动检查必须通过（CI）
2. 至少需要 1 人审查
3. 所有审查意见解决后可以合并

## 报告问题

请使用 Issue 模板创建问题，并提供：

- 清晰的标题和描述
- 复现步骤
- 预期行为和实际行为
- 环境信息（操作系统、版本等）
- 相关截图（如适用）

## 许可证

通过贡献代码，您同意将您的作品按照项目许可证发布。
