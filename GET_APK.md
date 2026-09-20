# 📱 APK 下载指南

## 🚀 一键构建（推荐）

### 步骤 1：在 GitHub 创建仓库

1. 访问 [github.com/new](https://github.com/new)
2. **Repository name**: `novel-reader`
3. 选择 **Public**
4. 点击 **Create repository**（不要勾选 README）

### 步骤 2：推送代码并自动构建

在终端执行：

```bash
cd /workspace/novel_reader

# 添加远程仓库（替换 YOUR_USERNAME）
git remote add origin https://github.com/YOUR_USERNAME/novel-reader.git

# 推送代码
git branch -M main
git push -u origin main
```

### 步骤 3：等待构建完成

1. 访问：`https://github.com/YOUR_USERNAME/novel-reader/actions`
2. 看到 "Build Android APK" 工作流正在运行
3. 等待 **10-15 分钟**

### 步骤 4：下载 APK

构建成功后：

1. 点击工作流运行记录
2. 向下滚动到 **Artifacts** 部分
3. 点击 **android-apk**
4. 解压下载的 ZIP 文件

## 🔗 构建完成后，直接访问：

```
https://github.com/YOUR_USERNAME/novel-reader/actions/workflows/build.yml
```

点击最新的工作流运行 → Artifacts → android-apk

## 📥 快捷命令

项目已包含自动化脚本：

```bash
# 方式 1: 推送代码
git remote add origin https://github.com/YOUR_USERNAME/novel-reader.git
git push -u origin main

# 方式 2: 使用快速构建脚本
./QUICK_START.sh
```

## ❓ 常见问题

### Q: GitHub 用户名是什么？
A: 登录 GitHub 后，点击头像，顶部显示的用户名就是。

### Q: 构建需要多长时间？
A: 首次构建约 10-15 分钟，后续构建约 5-10 分钟。

### Q: APK 安装失败？
A:
- 检查手机设置中是否开启"未知来源安装"
- 确认手机系统为 Android 7.0+
- 尝试重新下载 APK 文件

### Q: 想创建正式版 Release？
A:
```bash
git tag v1.0.0
git push origin v1.0.0
```

这会自动创建 GitHub Release 并上传所有平台的安装包。

## 📱 项目特性

- **输入网站 URL**：支持任意小说网站
- **智能搜索**：自动解析网站搜索功能
- **广告清理**：自动去除页面广告
- **内容排版**：自定义字体、间距、主题
- **封面切换**：原始/Google/百度图片

## 🔧 技术架构

- **后端**：Rust + Tauri 2.0
- **前端**：Vue 3 + TypeScript
- **构建**：GitHub Actions CI/CD

## 📞 获取帮助

如有问题：
1. 查看 GitHub Actions 日志
2. 提交 Issue 报告问题
3. 检查 BUILD_APK_GUIDE.md 文档

---

构建成功后，所有用户都可以通过 GitHub 下载 APK，无需任何编程知识！
