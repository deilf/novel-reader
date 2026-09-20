# 一键构建 APK 指南

## 方法一：通过 GitHub Actions 构建（推荐）

### 第一步：推送代码到 GitHub

在终端中执行以下命令：

```bash
cd /workspace/novel_reader

# 1. 添加远程仓库（将 YOUR_USERNAME 替换为你的 GitHub 用户名）
git remote add origin https://github.com/YOUR_USERNAME/novel-reader.git

# 2. 推送代码到 main 分支
git branch -M main
git push -u origin main
```

### 第二步：等待构建完成

1. 访问 GitHub 仓库页面
2. 点击 **Actions** 标签
3. 看到 "Build Android APK" 工作流正在运行
4. 等待 10-15 分钟构建完成

### 第三步：下载 APK

构建成功后：

1. 点击工作流运行记录
2. 点击 **"android-apk"** artifact
3. 点击下载按钮
4. 解压下载的 ZIP 文件

### 第四步：安装 APK

1. 将 APK 文件传输到手机
2. 在手机上打开 APK 文件
3. 如果提示"禁止安装未知来源应用"：
   - 进入 **设置 → 安全**
   - 开启 **"未知来源"** 或 **"安装未知应用"**
4. 点击安装即可

## 方法二：本地构建 APK

如果你想在本地构建，需要以下环境：

### 环境要求

- Rust 1.70+
- Node.js 18+
- Android SDK

### 构建步骤

```bash
# 1. 克隆仓库
git clone https://github.com/YOUR_USERNAME/novel-reader.git
cd novel-reader

# 2. 安装依赖
npm install

# 3. 安装 Rust（如果未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 4. 设置 Android SDK 环境变量
export ANDROID_SDK_ROOT=~/Android/Sdk
export PATH=$PATH:$ANDROID_SDK_ROOT/cmdline-tools/latest/bin:$ANDROID_SDK_ROOT/platform-tools

# 5. 构建 APK
npm run tauri build

# 6. APK 文件位置
# src-tauri/target/release/bundle/apk/
```

## 方法三：使用 GitHub API 下载（无需手动操作）

构建完成后，可以直接通过以下链接下载：

```
https://github.com/YOUR_USERNAME/novel-reader/suites/ACTIONS_RUN_ID/artifacts
```

将 `YOUR_USERNAME` 替换为你的 GitHub 用户名，`ACTIONS_RUN_ID` 替换为实际的构建 ID。

## APK 下载后的安装

### 手机安装

1. 开启"未知来源安装"
   - **华为/荣耀**: 设置 → 安全 → 未知来源应用
   - **小米/红米**: 设置 → 隐私保护 → 特殊权限 → 未知来源
   - **OPPO/Realme**: 设置 → 其他设置 → 设备与隐私 → 未知来源
   - **vivo**: 设置 → 安全 → 未知来源

2. 安装 APK
   - 直接点击 APK 文件
   - 或使用文件管理器打开

### 常见问题

#### Q: APK 安装失败怎么办？
A:
- 检查手机存储空间是否充足
- 确保下载的是完整的 APK 文件
- 尝试清除应用商店缓存

#### Q: 应用闪退怎么办？
A:
- 可能是手机系统不兼容
- 检查 APK 是否下载完整
- 查看 GitHub Actions 日志是否有错误

#### Q: 如何获取最新版本？
A:
- 删除旧版本
- 重新从 GitHub 下载最新 APK
- 或创建新的 tag 触发自动构建

## 版本说明

当前版本：v0.1.0

包含功能：
- ✅ 网站小说搜索
- ✅ 智能内容解析
- ✅ 广告自动清理
- ✅ 内容重新排版
- ✅ 多种阅读主题
- ✅ 封面来源切换
- ✅ 阅读进度保存
- ✅ 书架管理功能

## 技术支持

如果遇到问题：
1. 查看 GitHub Actions 日志
2. 提交 GitHub Issue
3. 检查手机系统版本要求（Android 7.0+）

## 许可证

本应用仅供学习交流使用，请勿用于商业用途。
