# 调用用户信息报错 - 解决方案

## 🔍 问题分析

调用用户信息时出现报错，主要原因是：

### 1. HTTPS证书验证问题
League Client使用**自签名证书**，浏览器的fetch API会拒绝这些连接，导致以下错误：
- `certificate verification failed`
- `self-signed certificate`
- `SSL connection error`

### 2. 网络连接问题
- 防火墙阻止连接
- League Client未运行
- 端口不可达

## ✅ 解决方案

### 方案1：使用Tauri命令绕过证书验证（已实现）

我已经实现了一个新的Tauri命令 `call_lcp_api`，它在Rust后端使用 `reqwest` 客户端，配置了：
```rust
let client = reqwest::ClientBuilder::new()
    .danger_accept_invalid_certs(true)  // 忽略无效证书
    .danger_accept_invalid_hostnames(true)  // 忽略主机名验证
    .build()?;
```

### 方案2：详细的错误诊断

添加了网络诊断工具 `diagnoseLCPConnection`，可以：
- 测试端口连通性
- 分析错误类型
- 提供解决建议

## 🚀 使用方法

### 1. 自动使用（推荐）
启动Nidalee和League Client，应用会自动：
1. 连接到游戏客户端
2. 使用Tauri命令获取用户信息
3. 在仪表板显示结果

### 2. 手动调试
如果仍有问题，可以：
1. 打开开发者工具查看控制台
2. 点击用户信息卡片中的"🔍 测试连接"按钮
3. 查看详细诊断信息

## 📋 详细日志

现在获取用户信息时会输出详细日志：

```
🚀 自动连接成功: LeagueClient (12345) 端口: 54321
🎮 开始获取用户信息...
📡 开始获取召唤师信息...
🔐 使用Token: AbCdEfGh...
🌐 连接端口: 54321
🔗 API地址: https://127.0.0.1:54321
📞 调用 /lol-summoner/v1/current-summoner API...
🚀 使用Tauri命令调用LCP API...
📡 调用LCP API: /lol-summoner/v1/current-summoner
🔗 地址: https://127.0.0.1:54321/lol-summoner/v1/current-summoner
✅ LCP API调用成功
✅ Tauri LCP API调用成功，数据: {...}
🏆 开始获取排位信息...
📊 排位数据原始信息: {...}
🥇 单双排位信息: {...}
🎉 用户信息获取成功!
👨‍💼 完整用户信息: {...}
```

## 🔧 技术实现

### Rust后端 (src-tauri/src/main.rs)
```rust
#[tauri::command]
async fn call_lcp_api(
    state: tauri::State<'_, AppState>,
    path: String,
) -> Result<serde_json::Value, String> {
    // 使用reqwest忽略SSL证书验证
    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build()?;

    // 发送HTTPS请求到League Client
    let response = client.get(&url)
        .header("Authorization", format!("Basic {}", auth))
        .send().await?;

    Ok(response.json().await?)
}
```

### 前端调用 (src/stores/useGameStore.ts)
```typescript
// 使用Tauri命令而不是直接fetch
const rawData = await invoke('call_lcp_api', {
  path: '/lol-summoner/v1/current-summoner'
})
const summonerData = rawData as Summoner
```

## 🛠️ 故障排除

### 如果仍然报错：

#### 1. 检查League Client状态
```bash
# 确认League Client正在运行
tasklist | findstr "LeagueClient"
```

#### 2. 检查端口连通性
```bash
# 测试端口（替换为实际端口）
telnet 127.0.0.1 54321
```

#### 3. 查看详细错误
- 打开开发者工具 (F12)
- 查看Console标签页
- 寻找带有 ❌ 图标的错误信息

#### 4. 手动诊断
- 在用户信息卡片中点击"🔍 测试连接"
- 查看诊断报告和建议

### 常见错误解决：

| 错误信息 | 原因 | 解决方案 |
|---------|------|----------|
| `HTTP 404` | API路径错误 | 检查League Client版本兼容性 |
| `HTTP 401` | 认证失败 | Token可能已过期，重新连接 |
| `HTTP 500` | 服务器内部错误 | League Client可能有问题，重启客户端 |
| `ECONNREFUSED` | 连接被拒绝 | 确认League Client正在运行 |
| `timeout` | 连接超时 | 检查网络和防火墙设置 |

## ✅ 验证解决方案

成功解决后，你应该看到：
1. ✅ 控制台显示成功日志
2. 🎨 仪表板显示漂亮的用户信息卡片
3. 📊 包含头像、等级、排位等完整信息
4. 🔄 可以手动刷新获取最新数据

## 🎯 总结

这个解决方案通过以下方式彻底解决了调用用户信息的报错问题：

1. **绕过HTTPS证书验证** - 使用Rust后端处理SSL连接
2. **详细错误诊断** - 提供完整的问题分析工具
3. **友好的用户界面** - 美观的信息展示和状态反馈
4. **完善的日志系统** - 便于调试和问题定位

现在你可以放心使用这个功能了！🎮✨
