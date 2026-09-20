# 🚀 快速获取 APK 下载链接

## 方法一：一键构建（推荐）

### 只需 3 步：

#### 第 1 步：在 GitHub 创建仓库

1. 打开浏览器访问：[github.com/new](https://github.com/new)
2. 填写信息：
   - **Repository name**: `novel-reader`
   - **Description**: `Rust + Tauri 小说阅读应用`
   - 选择 **Public**（免费）
3. 点击 **Create repository**

#### 第 2 步：推送代码

在终端执行（复制粘贴即可）：

```bash
cd /workspace/novel_reader

git remote add origin https://github.com/YOUR_USERNAME/novel-reader.git

git branch -M main

git push -u origin main
```

> ⚠️ 将 `YOUR_USERNAME` 替换为你的 GitHub 用户名

#### 第 3 步：下载 APK

1. 打开：`https://github.com/YOUR_USERNAME/novel-reader/actions`
2. 等待构建完成（约 10-15 分钟）
3. 点击构建记录 → **android-apk** → 下载

## 📥 APK 下载链接格式

构建完成后，访问：

```
https://github.com/YOUR_USERNAME/novel-reader/actions/runs/RUN_ID/artifacts
```

其中 `RUN_ID` 是构建运行的 ID。

或者直接在仓库页面：
- 点击 **Actions** 标签
- 点击 **Build Android APK** 工作流
- 点击最新的运行记录
- 在页面底部找到 **android-apk** artifact
- 点击下载

## 🎯 快速命令

项目包含自动化脚本，一键完成所有操作：

```bash
# 推送到 GitHub 并触发构建
./QUICK_START.sh
```

## 📱 项目功能

✅ 网站小说搜索
✅ 智能内容解析
✅ 自动广告清理
✅ 内容重新排版
✅ 多种阅读主题
✅ 封面来源切换
✅ 阅读进度保存
✅ 书架管理

## 🔧 技术栈

- **Rust** - 高性能后端
- **Tauri 2.0** - 跨平台框架
- **Vue 3** - 现代化前端
- **GitHub Actions** - CI/CD 自动构建

## ❓ 常见问题

**Q: 构建需要多长时间？**
A: 首次约 10-15 分钟，后续约 5 分钟

**Q: APK 无法安装？**
A: 需要开启手机"未知来源安装"权限

**Q: 如何更新到最新版本？**
A: 删除旧版本，重新从 GitHub 下载最新 APK

**Q: 想发布正式版？**
A:
```bash
git tag v1.0.0
git push origin v1.0.0
```

## 📞 详细文档

- `GET_APK.md` - APK 下载详细指南
- `BUILD_APK_GUIDE.md` - 完整构建教程
- `README.md` - 项目使用说明
- `SPEC.md` - 技术规格说明

## 🌟 开始使用

现在就打开 GitHub 创建仓库，开始构建你的小说阅读器 APK 吧！

---

**构建成功后，所有用户都可以免费从 GitHub 下载安装包，无需任何服务器费用！**
