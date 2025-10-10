import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useLiveClient } from './useLiveClient'
import { getChampionIdByName, getChampionIconUrl } from '@/lib'
import type { LiveClientPlayer } from '@/types/generated/LiveClientPlayer'
// import type { EnrichedLivePlayer } from '@/types/handle.d'

/**
 * LiveClient 数据管理组合式函数
 * 职责：管理 LiveClient 数据的获取、处理和状态管理
 */
export function useLiveClientManager() {
  // 获取 useLiveClient 实例
  const { identifyTeams } = useLiveClient()

  // 状态管理
  const players = ref<any[]>([])
  const myTeamPlayers = ref<any[]>([])
  const isLiveClientAvailable = ref(false)
  const hasFetchedGameData = ref(false)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // 事件监听管理
  let eventListeners: (() => void)[] = []

  /**
   * 检查 LiveClient 可用性
   */
  const checkLiveClientAvailability = async (): Promise<boolean> => {
    try {
      const available = await invoke<boolean>('is_liveclient_available')
      isLiveClientAvailable.value = available

      if (available) {
        console.log('[LiveClient] 服务可用')
        return true
      } else {
        console.log('[LiveClient] 服务不可用')
        return false
      }
    } catch (err) {
      console.error('[LiveClient] 检查可用性失败:', err)
      isLiveClientAvailable.value = false
      return false
    }
  }

  /**
   * 获取游戏数据
   */
  const fetchGameData = async (localPlayerName?: string) => {
    isLoading.value = true
    error.value = null

    try {
      const newPlayerList = await invoke<LiveClientPlayer[]>('get_live_player_list')
      console.log('[LiveClient] 获取到游戏数据:', newPlayerList)

      await processGameData(newPlayerList, localPlayerName)
      hasFetchedGameData.value = true
    } catch (err) {
      console.error('[LiveClient] 获取游戏数据失败:', err)
      error.value = err instanceof Error ? err.message : '获取游戏数据失败'
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 强制拉取玩家列表（不依赖可用性预检）
   * 用于 InProgress 刷新后的首帧快速填充
   */
  const getLivePlayers = async (localPlayerName?: string) => {
    try {
      const list = await invoke<LiveClientPlayer[]>('get_live_player_list')
      if (Array.isArray(list) && list.length > 0) {
        await processGameData(list, localPlayerName)
        hasFetchedGameData.value = true
        isLiveClientAvailable.value = true
        console.log('[LiveClient] getLivePlayers 首次抓取成功，已填充 UI')
      } else {
        console.log('[LiveClient] getLivePlayers 返回为空')
      }
    } catch (err) {
      console.warn('[LiveClient] getLivePlayers 调用失败:', err)
    }
  }

  /**
   * 处理游戏数据
   */
  const processGameData = async (newPlayerList: LiveClientPlayer[], localPlayerName?: string) => {
    if (!Array.isArray(newPlayerList) || newPlayerList.length === 0) {
      console.log('[LiveClient] 玩家列表为空')
      return
    }

    console.log('[LiveClient] 处理游戏数据:', newPlayerList)

    // 调试：检查每个玩家的 summonerName
    newPlayerList.forEach((player, index) => {
      console.log(`[LiveClient] 玩家 ${index}:`, {
        summonerName: player.summonerName,
        championName: player.championName,
        team: player.team,
        isBot: player.isBot
      })
    })

    // 使用 LiveClient 的队伍识别逻辑
    const { myTeam, enemyTeam } = identifyTeams(newPlayerList, localPlayerName)

    console.log('[LiveClient] 我方队伍:', myTeam)
    console.log('[LiveClient] 敌方队伍:', enemyTeam)

    // 转换我方队伍数据
    myTeamPlayers.value = myTeam.map((p: LiveClientPlayer): any => {
      const championId = getChampionIdByName(p.championName) || 0
      // 对于人机玩家，使用更友好的显示名称
      const displayName = p.isBot ? `${p.championName}（电脑）` : p.summonerName

      return {
        displayName: displayName,
        championName: p.championName,
        championId: championId,
        assignedPosition: p.position !== 'NONE' ? p.position : null,
        team: p.team,
        isBot: p.isBot,
        isLocal: p.summonerName === localPlayerName,
        championIcon: getChampionIconUrl(championId),
        // 添加其他可能需要的字段
        cellId: 0,
        puuid: null,
        summonerId: p.summonerName,
        championPickIntent: null,
        selectedSkinId: p.skinId,
        spell1Id: null,
        spell2Id: null,
        tagLine: null,
        profileIconId: null,
        tier: null,
        recentMatches: null
      }
    })

    // 转换敌方队伍数据
    players.value = enemyTeam.map((p: LiveClientPlayer): any => {
      const championId = getChampionIdByName(p.championName) || 0
      // 对于人机玩家，使用更友好的显示名称
      const displayName = p.isBot ? `${p.championName}（电脑）` : p.summonerName

      return {
        displayName: displayName,
        championName: p.championName,
        championId: championId,
        assignedPosition: p.position !== 'NONE' ? p.position : null,
        team: p.team,
        isBot: p.isBot,
        isLocal: false,
        championIcon: getChampionIconUrl(championId),
        // 添加其他可能需要的字段
        cellId: 0,
        puuid: null,
        summonerId: p.summonerName,
        championPickIntent: null,
        selectedSkinId: p.skinId,
        spell1Id: null,
        spell2Id: null,
        tagLine: null,
        profileIconId: null,
        tier: null,
        recentMatches: null
      }
    })
  }

  /**
   * 开始 LiveClient 事件监听
   * 优化：改为纯事件驱动，不再前端轮询
   */
  const startLiveClientAvailabilityCheck = async (localPlayerName?: string) => {
    console.log('[LiveClient] 启动事件驱动模式，等待后端推送数据')

    // 清理之前的事件监听器
    eventListeners.forEach((cleanup) => cleanup())
    eventListeners = []

    try {
      // 先做一次可用性检测与一次性抓取，尽快填充 UI
      const available = await checkLiveClientAvailability()
      if (available) {
        await fetchGameData(localPlayerName)
      } else {
        // 即便不可用也尝试一次强制抓取，部分环境下可用性探测可能滞后
        await getLivePlayers(localPlayerName)
      }

      // 监听 LiveClient 玩家列表更新
      const cleanup1 = await listen<LiveClientPlayer[]>('liveclient-player-list', async (event) => {
        console.log('[LiveClient] 收到玩家列表事件:', event.payload)
        if (event.payload && event.payload.length > 0) {
          await processGameData(event.payload, localPlayerName)
          hasFetchedGameData.value = true
        }
      })
      eventListeners.push(cleanup1)

      // 监听 LiveClient 可用性状态
      const cleanup2 = await listen<boolean>('liveclient-availability-changed', (event) => {
        console.log('[LiveClient] LiveClient 可用性变化:', event.payload)
        isLiveClientAvailable.value = event.payload
      })
      eventListeners.push(cleanup2)

      console.log('[LiveClient] 事件监听已启动')
    } catch (error) {
      console.error('[LiveClient] 启动事件监听失败:', error)
    }
  }

  /**
   * 停止 LiveClient 事件监听
   */
  const stopLiveClientAvailabilityCheck = () => {
    console.log('[LiveClient] 停止事件监听')
    eventListeners.forEach((cleanup) => cleanup())
    eventListeners = []
  }

  /**
   * 重置状态
   */
  const resetState = () => {
    players.value = []
    myTeamPlayers.value = []
    isLiveClientAvailable.value = false
    hasFetchedGameData.value = false
    error.value = null
    stopLiveClientAvailabilityCheck()
  }

  // 计算属性
  const hasData = computed(() => players.value.length > 0 || myTeamPlayers.value.length > 0)
  // 兼容：若外部未定义检查句柄，则内部定义为 null
  const liveClientCheckInterval: any = null
  const isChecking = computed(() => liveClientCheckInterval !== null)

  return {
    // 状态
    players,
    myTeamPlayers,
    isLiveClientAvailable,
    hasFetchedGameData,
    isLoading,
    error,
    hasData,
    isChecking,

    // 方法
    checkLiveClientAvailability,
    fetchGameData,
    getLivePlayers,
    startLiveClientAvailabilityCheck,
    stopLiveClientAvailabilityCheck,
    resetState
  }
}
