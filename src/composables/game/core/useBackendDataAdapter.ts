/**
 * 后端数据适配器 - 简化版
 * 🎉 由于后端已输出 camelCase 格式，且类型已通过 ts-rs 自动生成
 * 现在可以直接使用后端数据，无需复杂的格式转换
 */

/**
 * 直接使用后端的 TeamAnalysisData，无需转换
 * 后端类型已通过 ts-rs 自动同步到前端
 */
export function useTeamAnalysisData(backendData: TeamAnalysisData | null) {
  if (!backendData) {
    return null
  }

  // 🔥 直接返回后端数据，类型已完全匹配
  return backendData
}

/**
 * 为兼容现有代码，提供简单的数据访问辅助函数
 */
export function getPlayerSpells(player: PlayerAnalysisData): [number, number] {
  return [player.spell1Id || 0, player.spell2Id || 0]
}

/**
 * 获取玩家基本信息
 */
export function getPlayerBasicInfo(player: PlayerAnalysisData) {
  return {
    cellId: player.cellId,
    displayName: player.displayName || '未知',
    championId: player.championId,
    championName: player.championName,
    isLocal: player.isLocal,
    isBot: player.isBot,
    tier: player.tier
  }
}

/**
 * 🎉 新的简化适配器 - 直接使用后端数据
 * 由于类型已自动同步，无需复杂的格式转换
 */
export function useBackendDataAdapter() {
  return {
    // 直接使用后端数据，无需转换
    useTeamAnalysisData,
    getPlayerSpells,
    getPlayerBasicInfo
  }
}
