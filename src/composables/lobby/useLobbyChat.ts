/**
 * 房间聊天组合函数
 * 提供房间聊天相关的功能
 */
import { invoke } from '@tauri-apps/api/core'
import type { LobbyInfo } from '@/types/generated/LobbyInfo'

/**
 * 获取当前房间信息
 */
export async function getCurrentLobby(): Promise<LobbyInfo | null> {
  try {
    const lobby = await invoke<LobbyInfo>('get_current_lobby')
    return lobby
  } catch (error) {
    console.error('[LobbyChat] 获取房间信息失败:', error)
    return null
  }
}

/**
 * 发送房间聊天消息
 * @param chatId - 聊天室 ID（从 lobby.multiUserChatId 获取）
 * @param message - 消息内容
 */
export async function sendLobbyChatMessage(chatId: string, message: string): Promise<boolean> {
  try {
    await invoke('send_lobby_chat_message', { chatId, message })
    console.log('[LobbyChat] 消息已发送:', message)
    return true
  } catch (error) {
    console.error('[LobbyChat] 发送消息失败:', error)
    return false
  }
}

/**
 * 发送格式化消息（带前缀）
 * @param chatId - 聊天室 ID
 * @param prefix - 消息前缀（如 "Nidalee"）
 * @param content - 消息内容
 * @example
 * sendFormattedMessage(chatId, "Nidalee", "推荐选择亚索")
 * // 发送: "[Nidalee] 推荐选择亚索"
 */
export async function sendFormattedMessage(
  chatId: string,
  prefix: string,
  content: string
): Promise<boolean> {
  try {
    await invoke('send_lobby_formatted_message', { chatId, prefix, content })
    console.log(`[LobbyChat] 格式化消息已发送: [${prefix}] ${content}`)
    return true
  } catch (error) {
    console.error('[LobbyChat] 发送格式化消息失败:', error)
    return false
  }
}

/**
 * 房间聊天 Composable
 */
export function useLobbyChat() {
  const currentLobby = ref<LobbyInfo | null>(null)
  const chatId = computed(() => currentLobby.value?.multiUserChatId || '')
  const canSendMessage = computed(() => !!chatId.value)

  /**
   * 刷新房间信息
   */
  const refreshLobby = async () => {
    currentLobby.value = await getCurrentLobby()
    return currentLobby.value
  }

  /**
   * 发送消息（自动使用当前房间）
   */
  const sendMessage = async (message: string) => {
    if (!canSendMessage.value) {
      console.warn('[LobbyChat] 无法发送消息：未在房间中')
      return false
    }
    return sendLobbyChatMessage(chatId.value, message)
  }

  /**
   * 发送 Nidalee 助手消息
   */
  const sendNidaleeMessage = async (content: string) => {
    if (!canSendMessage.value) {
      console.warn('[LobbyChat] 无法发送消息：未在房间中')
      return false
    }
    return sendFormattedMessage(chatId.value, 'Nidalee', content)
  }

  /**
   * 发送英雄推荐消息
   */
  const sendChampionRecommendation = async (championName: string, reason: string) => {
    const content = `推荐选择 ${championName}：${reason}`
    return sendNidaleeMessage(content)
  }

  /**
   * 发送战术提示
   */
  const sendTacticTip = async (tip: string) => {
    return sendNidaleeMessage(`💡 ${tip}`)
  }

  return {
    // 状态
    currentLobby,
    chatId,
    canSendMessage,

    // 方法
    refreshLobby,
    sendMessage,
    sendNidaleeMessage,
    sendChampionRecommendation,
    sendTacticTip
  }
}

