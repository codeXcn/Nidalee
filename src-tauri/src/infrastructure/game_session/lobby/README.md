# Lobby & Chat 模块

## 📋 概述

该模块提供房间（Lobby）管理和聊天功能，包括获取房间信息、发送聊天消息等。

## 🏗️ 模块结构

```
lobby/
├── service.rs    # 核心业务逻辑
├── commands.rs   # Tauri 命令接口
├── mod.rs        # 模块导出
└── README.md     # 本文档
```

## 🔧 Service 层 API

### 1. 获取房间信息
```rust
pub async fn get_lobby_info(client: &Client) -> Result<LobbyInfo, String>
```

### 2. 发送聊天消息
```rust
pub async fn send_chat_message(
    client: &Client,
    chat_id: &str,
    message: &str,
) -> Result<(), String>
```

### 3. 发送格式化消息
```rust
pub async fn send_formatted_message(
    client: &Client,
    chat_id: &str,
    prefix: &str,
    content: &str,
) -> Result<(), String>
```

### 4. 提取聊天室 ID
```rust
pub fn get_chat_id_from_lobby(lobby: &LobbyInfo) -> Option<String>
```

---

## 📡 Tauri 命令

### 1. `get_current_lobby`
获取当前房间信息

```typescript
const lobby = await invoke<LobbyInfo>('get_current_lobby')
```

### 2. `send_lobby_chat_message`
发送聊天消息

```typescript
await invoke('send_lobby_chat_message', {
  chatId: lobby.multiUserChatId,
  message: '大家好！'
})
```

### 3. `send_lobby_formatted_message`
发送带前缀的格式化消息

```typescript
await invoke('send_lobby_formatted_message', {
  chatId: lobby.multiUserChatId,
  prefix: 'Nidalee',
  content: '推荐选择亚索'
})
// 发送: "[Nidalee] 推荐选择亚索"
```

---

## 🎯 前端使用（TypeScript）

### 基础用法

```typescript
import { invoke } from '@tauri-apps/api/core'
import type { LobbyInfo } from '@/types/generated/LobbyInfo'

// 获取房间信息
const lobby = await invoke<LobbyInfo>('get_current_lobby')

// 发送消息
await invoke('send_lobby_chat_message', {
  chatId: lobby.multiUserChatId,
  message: '你好，队友们！'
})
```

### 使用 Composable（推荐）

```vue
<script setup lang="ts">
import { useLobbyChat } from '@/composables/lobby'

const {
  currentLobby,
  canSendMessage,
  refreshLobby,
  sendMessage,
  sendNidaleeMessage,
  sendChampionRecommendation
} = useLobbyChat()

// 刷新房间信息
await refreshLobby()

// 发送普通消息
await sendMessage('大家好！')

// 发送助手消息
await sendNidaleeMessage('推荐ban掉亚索')

// 发送英雄推荐
await sendChampionRecommendation('亚索', '当前版本强势，适合你的打法')
</script>
```

---

## 💡 实际应用场景

### 场景1：自动欢迎消息

```typescript
// 进入房间后自动打招呼
const lobby = await getCurrentLobby()
if (lobby) {
  await sendFormattedMessage(
    lobby.multiUserChatId,
    'Nidalee',
    '大家好，祝游戏愉快！使用 Nidalee 助手为你服务 🎮'
  )
}
```

### 场景2：分享英雄推荐

```typescript
// 分析完成后分享推荐
const { sendChampionRecommendation } = useLobbyChat()

await refreshLobby()
await sendChampionRecommendation(
  '亚索',
  '当前版本胜率 53%，适合你的打法风格'
)
```

### 场景3：战术提示

```typescript
const { sendTacticTip } = useLobbyChat()

await refreshLobby()
await sendTacticTip('推荐前期稳健发育，15分钟后团战')
```

### 场景4：Ban 位建议

```typescript
await sendNidaleeMessage('建议Ban：亚索、劫、卡萨丁')
```

---

## 🔍 LobbyInfo 关键字段

```typescript
interface LobbyInfo {
  canStartActivity: boolean           // 是否可以开始游戏
  gameConfig: any                     // 游戏配置
  members: LobbyMember[]              // 成员列表
  localMember: LobbyMember            // 本地玩家
  multiUserChatId: string             // 💬 聊天室ID（发消息用）
  multiUserChatPassword: string       // JWT 令牌
  partyId: string                     // 队伍ID
  partyType: string                   // 队伍类型（open/closed）
  invitations: any[]                  // 邀请列表
  restrictions: any[]                 // 限制
  warnings: any[]                     // 警告
}
```

---

## ⚠️ 注意事项

1. **聊天室 ID**：必须从 `LobbyInfo.multiUserChatId` 获取
2. **房间状态**：只能在房间中发送消息
3. **权限检查**：确保 `canSendMessage` 为 `true`
4. **消息长度**：建议限制在 500 字符以内
5. **频率控制**：避免短时间内发送大量消息（可能被限流）

---

## 🚀 扩展建议

未来可以添加的功能：

1. **监听聊天消息**：订阅 WebSocket 接收消息
2. **消息历史**：获取聊天记录
3. **表情和图片**：支持富文本消息
4. **私聊功能**：支持一对一聊天
5. **消息撤回**：删除已发送的消息

---

## 📝 相关类型定义

TypeScript 类型已通过 `ts-rs` 自动生成：
- `src/types/generated/LobbyInfo.ts`
- `src/types/generated/LobbyMember.ts`

