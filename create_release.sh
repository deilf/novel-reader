#!/bin/bash

# GitHub Actions 自动构建触发脚本
# 用于自动触发 GitHub Actions 构建 APK

echo "=========================================="
echo "  小说阅读器 - GitHub Actions 构建脚本"
echo "=========================================="
echo ""

# 检查 Git 状态
echo "步骤 1: 检查 Git 状态..."
if [ ! -d ".git" ]; then
    echo "错误: 未找到 Git 仓库，请先执行 git init"
    exit 1
fi

# 检查远程仓库
echo "步骤 2: 检查远程仓库..."
git remote -v | grep origin > /dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "错误: 未配置远程仓库"
    echo "请先执行: git remote add origin https://github.com/YOUR_USERNAME/novel-reader.git"
    exit 1
fi

# 显示当前状态
echo "当前分支: $(git branch --show-current)"
echo "远程仓库: $(git remote get-url origin)"
echo ""

# 推送代码
echo "步骤 3: 推送代码到 GitHub..."
git push -u origin $(git branch --show-current)
if [ $? -ne 0 ]; then
    echo "错误: 推送失败"
    exit 1
fi

echo ""
echo "=========================================="
echo "  代码已成功推送到 GitHub！"
echo "=========================================="
echo ""
echo "接下来请访问以下链接查看构建状态："
echo ""
echo "1. 访问 GitHub 仓库页面"
echo "2. 点击 Actions 标签"
echo "3. 查看 'Build Android APK' 工作流"
echo "4. 等待构建完成（约 10-15 分钟）"
echo "5. 下载 android-apk artifact"
echo ""
echo "GitHub 仓库地址："
echo "$(git remote get-url origin | sed 's/\.git$//')"
echo ""
echo "=========================================="
echo ""
echo "提示：创建新版本只需执行以下命令："
echo "  git tag v0.2.0"
echo "  git push origin v0.2.0"
echo ""
