# GitHub 部署指南

## 🚀 快速开始

### 1. 创建 GitHub 仓库

1. 访问 [GitHub](https://github.com) 并登录
2. 点击右上角的 **"+"** 按钮，选择 **"New repository"**
3. 填写仓库信息：
   - **Repository name**: `novel-reader`
   - **Description**: 基于 Rust + Tauri 的小说阅读应用
   - **Public/Private**: 根据需要选择（公开仓库可免费使用 GitHub Actions）
   - **不要勾选** "Add a README file"（我们已经有了）
   - **不要勾选** ".gitignore"（我们已经有了）
4. 点击 **"Create repository"**

### 2. 推送代码到 GitHub

在项目目录中执行以下命令（将 `YOUR_USERNAME` 替换为您的 GitHub 用户名）：

```bash
# 添加远程仓库
git remote add origin https://github.com/YOUR_USERNAME/novel-reader.git

# 推送代码到 main 分支
git branch -M main
git push -u origin main
```

### 3. 配置 GitHub Secrets（可选）

如果您需要签名发布版本，需要添加签名密钥：

1. 进入仓库 **Settings** → **Secrets and variables** → **Actions**
2. 点击 **"New repository secret"**
3. 添加以下 Secret：
   - **Name**: `TAURI_KEY_PASSWORD`
   - **Value**: 您的证书密码

### 4. 触发构建

#### 自动构建

推送代码后，GitHub Actions 会自动运行：

- **推送代码**: 自动构建所有平台
- **创建 Tag**: 自动创建 Release 并上传构建产物
- **Pull Request**: 仅运行 CI 检查，不构建

#### 手动构建

1. 进入仓库的 **Actions** 页面
2. 选择 **"Build Android APK"** 工作流
3. 点击 **"Run workflow"**
4. 选择分支并运行

#### 创建 Release 构建

1. 创建新标签：
```bash
git tag v0.1.0
git push origin v0.1.0
```

2. GitHub Actions 将自动：
   - 构建 Android APK
   - 构建 Windows/macOS/Linux 应用
   - 创建 GitHub Release
   - 上传所有构建产物

### 5. 查看构建结果

1. 进入仓库 **Actions** 页面
2. 点击构建任务查看日志
3. 构建成功后：
   - **APK**: 在 **Artifacts** 中下载
   - **Release**: 在 **Releases** 页面查看

## 📱 Android 构建产物位置

构建完成后，APK 文件位于：
```
src-tauri/target/release/bundle/apk/
```

或者下载 GitHub Actions 的 artifact：
- **android-apk**: 正式版 APK
- **android-apk-debug**: 调试版 APK

## 🖥️ 跨平台构建产物

| 平台 | 文件类型 | 下载位置 |
|------|---------|---------|
| Android | `.apk`, `.aab` | Actions Artifacts |
| Windows | `.exe` | Release Assets |
| macOS | `.app`, `.dmg` | Release Assets |
| Linux | `.AppImage`, `.deb` | Release Assets |

## ⚙️ CI/CD 工作流说明

### build.yml

**触发条件**：
- 推送到 `main` 分支
- 创建 `v*` 标签
- Pull Request 到 `main`

**构建任务**：
1. Android APK 构建
2. Windows 可执行文件构建
3. macOS 应用构建
4. Linux 应用构建
5. 自动创建 Release（仅 tag 触发）

### ci.yml

**触发条件**：
- 所有 Push 和 Pull Request

**检查任务**：
1. Rust 代码格式化检查
2. Clippy 代码审查
3. Rust 单元测试
4. 前端构建检查
5. 安全审计

## 🔧 自定义配置

### 修改包名

编辑 `src-tauri/tauri.conf.json`：

```json
{
  "bundle": {
    "android": {
      "package": "com.yourname.novelreader"
    }
  }
}
```

### 修改应用图标

将图标文件放入 `src-tauri/icons/` 目录，覆盖现有文件。

### 修改应用名称

编辑 `src-tauri/tauri.conf.json`：

```json
{
  "productName": "你的应用名称"
}
```

## 📊 监控构建状态

在 README 中添加构建状态徽章：

```markdown
[![Build Status](https://github.com/YOUR_USERNAME/novel-reader/actions/workflows/build.yml/badge.svg)](https://github.com/YOUR_USERNAME/novel-reader/actions)
```

## 🐛 故障排除

### 构建失败

1. 查看 Actions 日志定位问题
2. 常见问题：
   - **Rust 依赖下载失败**: 重试构建
   - **Android SDK 缺失**: 检查 `setup-android` 步骤
   - **Node 依赖安装失败**: 检查 `npm ci` 步骤

### 签名问题

如果遇到签名错误：
1. 检查 `TAURI_KEY_PASSWORD` Secret 是否设置
2. 确保证书路径正确
3. 参考 [Tauri 签名文档](https://tauri.app/zh-cn/distribute/sign/)

### 权限问题

确保 GitHub Actions 有足够权限：
1. 进入仓库 **Settings** → **Actions** → **General**
2. 确认 "Workflow permissions" 设置为 "Read and write permissions"

## 📞 获取帮助

- 查看 [GitHub Actions 文档](https://docs.github.com/cn/actions)
- 查看 [Tauri 构建文档](https://tauri.app/zh-cn/distribute/)
- 提交 GitHub Issue 报告问题
