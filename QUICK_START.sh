#!/bin/bash

# 快速构建 APK 脚本
# 自动完成所有步骤，直接获取 APK 下载链接

echo "=========================================="
echo "  小说阅读器 APK 一键构建工具"
echo "=========================================="
echo ""

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 检查必要工具
echo -e "${YELLOW}检查环境...${NC}"

if ! command -v git &> /dev/null; then
    echo -e "${RED}错误: 未安装 Git${NC}"
    exit 1
fi

if ! command -v curl &> /dev/null; then
    echo -e "${RED}错误: 未安装 curl${NC}"
    exit 1
fi

echo -e "${GREEN}✓ 环境检查通过${NC}"
echo ""

# 检查远程仓库
echo -e "${YELLOW}步骤 1: 检查 GitHub 仓库配置${NC}"
git remote -v | grep origin > /dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "未配置远程仓库"
    echo ""
    echo -e "${YELLOW}请先在 GitHub 上创建仓库，然后运行：${NC}"
    echo ""
    echo "  git remote add origin https://github.com/YOUR_USERNAME/novel-reader.git"
    echo "  ./QUICK_START.sh"
    echo ""
    exit 1
fi

REMOTE_URL=$(git remote get-url origin)
echo -e "${GREEN}✓ 远程仓库: $REMOTE_URL${NC}"
echo ""

# 提取 GitHub 用户名和仓库名
if [[ $REMOTE_URL =~ github\.com[:/]([^/]+)/([^/]+) ]]; then
    GITHUB_USERNAME="${BASH_REMATCH[1]}"
    REPO_NAME="${BASH_REMATCH[2]}"
    REPO_NAME="${REPO_NAME%.git}"
    echo -e "${YELLOW}步骤 2: GitHub 信息${NC}"
    echo -e "${GREEN}✓ 用户名: $GITHUB_USERNAME${NC}"
    echo -e "${GREEN}✓ 仓库: $REPO_NAME${NC}"
    echo ""
else
    echo -e "${RED}错误: 无法解析 GitHub 仓库信息${NC}"
    exit 1
fi

# 推送代码
echo -e "${YELLOW}步骤 3: 推送代码到 GitHub${NC}"
git push -u origin $(git branch --show-current) 2>&1
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ 代码推送成功${NC}"
else
    echo -e "${RED}✗ 代码推送失败${NC}"
    echo "请检查 GitHub 认证是否正确"
    exit 1
fi
echo ""

# 等待构建触发
echo -e "${YELLOW}步骤 4: 触发构建${NC}"
echo -e "${GREEN}✓ 构建任务已触发${NC}"
echo ""

# 显示下载指南
echo "=========================================="
echo -e "${GREEN}  构建已启动！${NC}"
echo "=========================================="
echo ""
echo -e "${YELLOW}📱 APK 下载步骤：${NC}"
echo ""
echo "1. 打开以下链接查看构建状态："
echo -e "   ${GREEN}https://github.com/$GITHUB_USERNAME/$REPO_NAME/actions${NC}"
echo ""
echo "2. 等待构建完成（通常需要 10-15 分钟）"
echo ""
echo "3. 构建成功后，点击工作流运行记录"
echo ""
echo "4. 在页面底部找到 Artifacts 部分"
echo ""
echo "5. 点击 'android-apk' 下载 APK"
echo ""
echo "=========================================="
echo ""
echo -e "${YELLOW}💡 创建新版本（自动发布）：${NC}"
echo ""
echo "  git tag v0.1.0"
echo "  git push origin v0.1.0"
echo ""
echo "=========================================="
echo ""
echo -e "${GREEN}祝你使用愉快！🎉${NC}"
echo ""
