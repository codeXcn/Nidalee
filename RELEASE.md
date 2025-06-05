# 🚀 Nidalee 发布指南

## 自动化发布流程

本项目已配置 GitHub Actions CI/CD，支持自动打包 Windows、macOS、Linux 三平台安装包。

### 📦 支持的平台和格式

| 平台 | 格式 | 说明 |
|------|------|------|
| **Windows** | `.msi` | Windows 安装程序 |
| **macOS** | `.dmg` | macOS 磁盘映像（支持 Intel + Apple Silicon） |
| **Linux** | `.deb` | Debian/Ubuntu 软件包 |

### 🎯 如何发布新版本

#### 1. 更新版本号
编辑 `src-tauri/tauri.conf.json` 中的版本号：
```json
{
  "version": "1.0.1"
}
```

#### 2. 创建并推送 tag
```bash
# 提交所有更改
git add .
git commit -m "feat: 新功能或修复"

# 创建版本tag (必须以v开头)
git tag v1.0.1

# 推送tag到远程仓库
git push origin v1.0.1
```

#### 3. 自动构建
- GitHub Actions 将自动触发
- 同时构建 4 个版本：
  - Windows x64
  - macOS Intel (x86_64)
  - macOS Apple Silicon (ARM64)
  - Linux x64

#### 4. 查看发布
- 构建完成后，在 [GitHub Releases](../../releases) 页面查看
- 发布为草稿状态，可以编辑发布说明后手动发布

### 🔧 手动触发构建

如果需要手动触发构建，可以在 GitHub 网页上：
1. 进入 Actions 页面
2. 选择 "Release" workflow
3. 点击 "Run workflow"

### ⚠️ 注意事项

1. **版本号格式**：tag 必须以 `v` 开头，如 `v1.0.0`
2. **权限**：确保仓库有 `contents: write` 权限
3. **图标**：确保 `src-tauri/icons/` 目录包含所需图标文件
4. **代码签名**：
   - macOS 需要开发者证书（可选）
   - Windows 需要代码签名证书（可选）

### 📋 构建状态

可以在以下位置查看构建状态：
- [GitHub Actions](../../actions) - 构建日志和状态
- [GitHub Releases](../../releases) - 发布的安装包

### 🐛 故障排除

如果构建失败，常见问题：
1. **依赖问题**：检查 `package.json` 和 `Cargo.toml` 依赖
2. **权限问题**：确保仓库设置允许 Actions 写入
3. **版本号**：确保 tag 格式正确 (`v` + 语义化版本号)
4. **pnpm lock文件**：确保 `pnpm-lock.yaml` 已提交

---

## 🎮 手动测试构建

如果想在本地测试构建过程：

```bash
# 安装依赖
pnpm install

# 开发模式
pnpm tauri dev

# 构建生产版本
pnpm tauri build
```

### 🚀 发布示例

完整的发布流程示例：

```bash
# 1. 更新版本号 (编辑 src-tauri/tauri.conf.json)
# 2. 测试本地构建
pnpm tauri build

# 3. 提交更改
git add .
git commit -m "release: v1.0.1"

# 4. 创建tag并推送
git tag v1.0.1
git push origin main
git push origin v1.0.1

# 5. 在 GitHub 查看自动构建结果
```

---

> 💡 **提示**：推荐每次发布都更新版本号，这样用户可以清楚地知道自己使用的版本。 