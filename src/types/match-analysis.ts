/**
 * 对局分析模块专用类型定义
 * 🎉 简化版：大部分类型已通过 ts-rs 自动生成，这里只保留 UI 特定的类型
 */

export type GamePhase = 'None' | 'Lobby' | 'Matchmaking' | 'ChampSelect' | 'InProgress' | 'EndOfGame'

/**
 * UI 专用的玩家数据扩展
 * 基于全局的 PlayerAnalysisData，添加 UI 便利属性
 */
export interface UIPlayerData extends PlayerAnalysisData {
  // UI 便利属性
  spells: [number, number] // 召唤师技能数组 [spell1Id, spell2Id]
}

/**
 * 带 displayName 的战绩统计（用于 UI 显示）
 */
export interface EnrichedPlayerMatchStats extends PlayerMatchStats {
  displayName: string
}

/**
 * 敌方英雄选择信息（用于 UI 显示）
 */
export interface EnemyChampionPick {
  cellId: number
  championId: number | null
  championPickIntent?: number | null
}

/**
 * UI 中的团队数据格式（简化版，用于 Store 和组件）
 */
export interface TeamData {
  players: UIPlayerData[]
  localPlayerCellId: number
}

/**
 * 对局分析状态（使用全局类型）
 */
export interface MatchAnalysisState {
  phase: GamePhase
  isConnected: boolean
  isLoading: boolean

  // 🔥 直接使用后端的 TeamAnalysisData
  teamAnalysisData: TeamAnalysisData | null

  // UI 专用数据
  myTeamStats: EnrichedPlayerMatchStats[]
  enemyTeamStats: EnrichedPlayerMatchStats[]
  enemyChampionPicks: EnemyChampionPick[]
}
