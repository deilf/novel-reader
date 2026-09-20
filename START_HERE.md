# 📱 APK 下载链接获取指南

## ⚡ 快速开始

### 第 1 步：创建 GitHub 仓库

打开浏览器，访问：
```
https://github.com/new
```

填写信息：
- **Repository name**: `novel-reader`
- 选择 **Public**
- 点击 **Create repository**

### 第 2 步：推送代码

在终端执行以下命令：

```bash
cd /workspace/novel_reader

git remote add origin https://github.com/YOUR_USERNAME/novel-reader.git

git push -u origin main
```

> ⚠️ 记得把 `YOUR_USERNAME` 改成你的 GitHub 用户名！

### 第 3 步：等待构建

打开这个链接查看构建状态：
```
https://github.com/YOUR_USERNAME/novel-reader/actions
```

等待 **10-15 分钟**，直到显示 ✅ 完成。

### 第 4 步：下载 APK

1. 点击 "Build Android APK" 工作流
2. 点击最新的运行记录
3. 向下滚动，找到 **android-apk**
4. 点击下载

## 📦 APK 下载链接

构建成功后，访问：
```
https://github.com/YOUR_USERNAME/novel-reader/actions/workflows/build.yml
```

然后：
- 选择最新的工作流运行
- 点击 **android-apk** artifact
- 下载并解压

## 🎯 快速命令

一键推送并触发构建：

```bash
cd /workspace/novel_reader
git remote add origin https://github.com/YOUR_USERNAME/novel-reader.git
git push -u origin main
```

## 📚 项目文档

- `APK_README.md` - APK 下载指南（你在这里）
- `GET_APK.md` - 详细下载教程
- `BUILD_APK_GUIDE.md` - 完整构建说明
- `README.md` - 项目使用手册
- `SPEC.md` - 技术规格文档

## 🔧 项目特性

✅ **小说搜索** - 输入任意网站 URL 搜索小说
✅ **智能解析** - 自动识别网页结构
✅ **广告清理** - 去除页面广告
✅ **内容排版** - 自定义字体、间距、主题
✅ **封面切换** - 原始/Google/百度图片
✅ **进度保存** - 自动保存阅读位置
✅ **书架管理** - 收藏喜欢的小说

## 🚀 一键脚本

项目提供了自动化脚本：

```bash
# 推送到 GitHub
./QUICK_START.sh

# 或手动推送
git remote add origin https://github.com/YOUR_USERNAME/novel-reader.git
git push -u origin main
```

## ❓ 常见问题

### Q: GitHub 用户名在哪里看？
A: 登录 GitHub 后，点击右上角头像，弹出的用户名就是。

### Q: 构建需要多久？
A: 首次约 10-15 分钟，后续约 5 分钟。

### Q: APK 无法安装？
A:
1. 检查手机系统是否为 Android 7.0+
2. 开启"未知来源安装"权限
   - 华为：设置 → 安全 → 未知来源
   - 小米：设置 → 隐私保护 → 未知来源
   - OPPO：设置 → 其他设置 → 未知来源

### Q: 如何更新应用？
A: 删除旧版本，从 GitHub 重新下载最新 APK。

### Q: 想发布正式版？
A:
```bash
git tag v1.0.0
git push origin v1.0.0
```
这会自动创建 Release 并上传安装包。

## 📞 遇到问题？

1. 查看 GitHub Actions 日志定位问题
2. 检查 BUILD_APK_GUIDE.md 文档
3. 提交 GitHub Issue 寻求帮助

---

## 🌟 现在就开始！

1. 打开 [github.com/new](https://github.com/new)
2. 创建 `novel-reader` 仓库
3. 执行推送命令
4. 等待构建完成
5. 下载 APK 安装使用

**整个过程不超过 20 分钟，就能拥有自己的小说阅读器！**
