import { useActivityStore } from '@/stores/core/activityStore'

/**
 * 活动记录工具组合式函数
 * 职责：提供统一的活动记录接口，简化活动记录的使用
 */
export function useActivityLogger() {
  const activityStore = useActivityStore()

  // 连接相关活动
  const logConnection = {
    connected: () => activityStore.addActivity('success', '客户端连接成功', 'connection'),
    disconnected: () => activityStore.addActivity('error', '客户端连接断开', 'connection'),
    reconnected: () => activityStore.addActivity('info', '客户端重连成功', 'connection'),
    timeout: () => activityStore.addActivity('warning', '客户端连接超时，正在重试...', 'connection'),
    failed: (reason?: string) =>
      activityStore.addActivity('error', `连接失败${reason ? `: ${reason}` : ''}`, 'connection')
  }

  // 游戏相关活动
  const logGame = {
    queueing: () => activityStore.addActivity('info', '进入队列匹配中', 'game'),
    readyCheck: () => activityStore.addActivity('success', '找到对局，等待接受', 'game'),
    champSelect: () => activityStore.addActivity('info', '进入英雄选择阶段', 'game'),
    gameStart: () => activityStore.addActivity('success', '游戏开始', 'game'),
    gameEnd: (result?: 'win' | 'lose') => {
      const message = result ? `游戏结束 - ${result === 'win' ? '胜利' : '失败'}` : '游戏结束'
      activityStore.addActivity('info', message, 'game')
    },
    backToLobby: () => activityStore.addActivity('info', '返回客户端主界面', 'game')
  }

  // 自动功能相关活动
  const logAutoFunction = {
    acceptMatch: {
      success: () => activityStore.addActivity('success', '自动接受对局成功', 'auto'),
      failed: (reason = '超时') => activityStore.addActivity('error', `自动接受对局失败：${reason}`, 'auto')
    },
    selectChampion: {
      success: (championName: string) => activityStore.addActivity('success', `自动选择英雄：${championName}`, 'auto'),
      failed: (championName: string, reason = '不可用') =>
        activityStore.addActivity('error', `自动选择英雄失败：${championName} ${reason}`, 'auto')
    },
    banChampion: {
      success: (championName: string) => activityStore.addActivity('success', `自动禁用英雄：${championName}`, 'auto'),
      noConfig: () => activityStore.addActivity('warning', '自动禁用英雄：未设置禁用英雄', 'auto')
    },
    configRunes: {
      success: (runeName: string) => activityStore.addActivity('success', `自动配置符文：${runeName}`, 'auto'),
      failed: (reason = '符文页面已满') => activityStore.addActivity('error', `自动符文配置失败：${reason}`, 'auto')
    }
  }

  // 数据更新相关活动
  const logData = {
    summonerUpdated: () => activityStore.addActivity('info', '召唤师信息已更新', 'data'),
    rankUpdated: () => activityStore.addActivity('info', '排位信息已刷新', 'data'),
    matchHistoryUpdated: (count?: number) => {
      const message = count ? `对局历史记录已更新，新增 ${count} 场对局` : '对局历史记录已更新'
      activityStore.addActivity('success', message, 'data')
    },
    championDataUpdated: () => activityStore.addActivity('info', '英雄数据已同步', 'data'),
    skinDataUpdated: () => activityStore.addActivity('info', '皮肤数据已更新', 'data')
  }

  // 设置相关活动
  const logSettings = {
    setCareerBackground: (backgroundName: string) =>
      activityStore.addActivity('success', `设置主页背景：${backgroundName}`, 'settings'),
    autoFunctionEnabled: (functionName: string) =>
      activityStore.addActivity('success', `已开启${functionName}`, 'settings'),
    autoFunctionDisabled: (functionName: string) =>
      activityStore.addActivity('info', `已关闭${functionName}`, 'settings'),
    championSet: (championName: string) =>
      activityStore.addActivity('success', `设置自动选择英雄：${championName}`, 'settings'),
    themeChanged: (themeName: string) => activityStore.addActivity('info', `切换至${themeName}主题`, 'settings'),
    colorChanged: (colorName: string) =>
      activityStore.addActivity('info', `主题颜色已更改为：${colorName}`, 'settings'),
    settingsSaved: () => activityStore.addActivity('info', '应用设置已保存', 'settings'),
    settingsReset: () => activityStore.addActivity('info', '配置已重置为默认值', 'settings')
  }

  // 错误相关活动
  const logError = {
    apiError: (error: string) => activityStore.addActivity('error', `API 请求失败：${error}`, 'error'),
    networkTimeout: () => activityStore.addActivity('warning', '数据加载超时，请检查网络连接', 'error'),
    permissionDenied: () => activityStore.addActivity('error', '自动功能执行异常：权限不足', 'error'),
    versionMismatch: () => activityStore.addActivity('warning', '检测到游戏版本更新，部分功能可能受影响', 'error'),
    rateLimited: () => activityStore.addActivity('warning', '操作过于频繁，请稍后再试', 'error'),
    diskSpaceError: () => activityStore.addActivity('error', '保存设置失败：磁盘空间不足', 'error')
  }

  // 通用活动记录
  const log = {
    info: (
      message: string,
      category: 'system' | 'connection' | 'game' | 'auto' | 'data' | 'settings' | 'error' = 'system'
    ) => activityStore.addActivity('info', message, category),
    success: (
      message: string,
      category: 'system' | 'connection' | 'game' | 'auto' | 'data' | 'settings' | 'error' = 'system'
    ) => activityStore.addActivity('success', message, category),
    warning: (
      message: string,
      category: 'system' | 'connection' | 'game' | 'auto' | 'data' | 'settings' | 'error' = 'system'
    ) => activityStore.addActivity('warning', message, category),
    error: (
      message: string,
      category: 'system' | 'connection' | 'game' | 'auto' | 'data' | 'settings' | 'error' = 'system'
    ) => activityStore.addActivity('error', message, category)
  }

  return {
    log,
    logConnection,
    logGame,
    logAutoFunction,
    logData,
    logSettings,
    logError
  }
}
