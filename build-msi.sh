#!/bin/bash

echo "🚀 开始构建 Nidalee MSI 安装程序..."

# 检查 Node.js 版本
echo "📋 检查环境..."
node_version=$(node --version)
echo "Node.js 版本: $node_version"

# 检查是否有 pnpm
if command -v pnpm &> /dev/null; then
    echo "✅ 发现 pnpm"
    package_manager="pnpm"
elif command -v npm &> /dev/null; then
    echo "✅ 使用 npm 作为包管理器"
    package_manager="npm"
else
    echo "❌ 未找到包管理器"
    exit 1
fi

# 安装依赖
echo "📦 安装依赖..."
$package_manager install

# 构建应用
echo "🏗️ 构建应用..."
$package_manager run build

# 生成 MSI
echo "📦 生成 MSI 安装程序..."
cd src-tauri
cargo tauri build --target x86_64-pc-windows-msvc

echo "✅ 构建完成！"
echo "📁 MSI 文件位置: src-tauri/target/release/bundle/msi/"

# 显示生成的文件
if [ -d "target/release/bundle/msi" ]; then
    echo "🎉 生成的文件:"
    ls -la target/release/bundle/msi/
else
    echo "⚠️ MSI 文件夹不存在，可能构建失败"
fi 