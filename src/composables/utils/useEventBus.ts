/**
 * 事件总线组合式函数
 * 提供 Vue 组件中使用事件总线的便捷方法
 */

import { onUnmounted, type Ref } from 'vue'
import { eventBus, GameEvents, type EventCallback } from '@/lib/eventBus'

export function useEventBus() {
  const subscriptions: Array<{ event: string; callback: EventCallback }> = []

  /**
   * 订阅事件
   * @param event 事件名称
   * @param callback 回调函数
   */
  const subscribe = (event: string, callback: EventCallback) => {
    eventBus.on(event, callback)
    subscriptions.push({ event, callback })
  }

  /**
   * 订阅一次性事件
   * @param event 事件名称
   * @param callback 回调函数
   */
  const _subscribeOnce = (event: string, callback: EventCallback) => {
    eventBus.once(event, callback)
  }

  /**
   * 发布事件
   * @param event 事件名称
   * @param data 事件数据
   */
  const emit = (event: string, data?: any) => {
    eventBus.emit(event, data)
  }

  /**
   * 取消订阅事件
   * @param event 事件名称
   * @param callback 回调函数
   */
  const unsubscribe = (event: string, callback: EventCallback) => {
    eventBus.off(event, callback)
    const index = subscriptions.findIndex(sub => sub.event === event && sub.callback === callback)
    if (index > -1) {
      subscriptions.splice(index, 1)
    }
  }

  /**
   * 取消所有订阅
   */
  const unsubscribeAll = () => {
    subscriptions.forEach(({ event, callback }) => {
      eventBus.off(event, callback)
    })
    subscriptions.length = 0
  }

  // 组件卸载时自动清理订阅
  onUnmounted(() => {
    unsubscribeAll()
  })

  return {
    subscribe,
    emit,
    unsubscribe,
    unsubscribeAll,
    GameEvents
  }
}

/**
 * 响应式事件订阅
 * 将事件数据绑定到响应式变量
 * @param event 事件名称
 * @param initialValue 初始值
 */
export function useReactiveEvent<T>(event: string, initialValue: T): Ref<T> {
  const { subscribe } = useEventBus()
  const value = ref<T>(initialValue)

  subscribe(event, (data: T) => {
    value.value = data
  })

  return value
}

/**
 * 游戏事件专用组合式函数
 */
export function useGameEvents() {
  const { subscribe, emit } = useEventBus()

  return {
    // 游戏阶段事件
    onGamePhaseChanged: (callback: (phase: string | null) => void) =>
      subscribe(GameEvents.GAME_PHASE_CHANGED, callback),

    onGameStarted: (callback: () => void) =>
      subscribe(GameEvents.GAME_STARTED, callback),

    onGameEnded: (callback: () => void) =>
      subscribe(GameEvents.GAME_ENDED, callback),

    // LiveClient 事件
    onLiveClientAvailable: (callback: () => void) =>
      subscribe(GameEvents.LIVECLIENT_AVAILABLE, callback),

    onLiveClientDataFetched: (callback: (data: any) => void) =>
      subscribe(GameEvents.LIVECLIENT_DATA_FETCHED, callback),

    onLiveClientError: (callback: (error: any) => void) =>
      subscribe(GameEvents.LIVECLIENT_ERROR, callback),

    // 数据更新事件
    onPlayerDataUpdated: (callback: (data: any) => void) =>
      subscribe(GameEvents.PLAYER_DATA_UPDATED, callback),

    onMatchHistoryUpdated: (callback: (data: any) => void) =>
      subscribe(GameEvents.MATCH_HISTORY_UPDATED, callback),

    onChampionDataLoaded: (callback: (data: any) => void) =>
      subscribe(GameEvents.CHAMPION_DATA_LOADED, callback),

    // 连接状态事件
    onConnectionStateChanged: (callback: (state: any) => void) =>
      subscribe(GameEvents.CONNECTION_STATE_CHANGED, callback),

    onLcuConnected: (callback: () => void) =>
      subscribe(GameEvents.LCU_CONNECTED, callback),

    onLcuDisconnected: (callback: () => void) =>
      subscribe(GameEvents.LCU_DISCONNECTED, callback),

    // 性能事件
    onPerformanceMetric: (callback: (metric: any) => void) =>
      subscribe(GameEvents.PERFORMANCE_METRIC, callback),

    onSlowOperation: (callback: (operation: any) => void) =>
      subscribe(GameEvents.SLOW_OPERATION, callback),

    // 发布事件
    emitGamePhaseChanged: (phase: string | null) =>
      emit(GameEvents.GAME_PHASE_CHANGED, phase),

    emitLiveClientAvailable: () =>
      emit(GameEvents.LIVECLIENT_AVAILABLE),

    emitLiveClientDataFetched: (data: any) =>
      emit(GameEvents.LIVECLIENT_DATA_FETCHED, data),

    emitLiveClientError: (error: any) =>
      emit(GameEvents.LIVECLIENT_ERROR, error),

    emitPlayerDataUpdated: (data: any) =>
      emit(GameEvents.PLAYER_DATA_UPDATED, data),

    emitMatchHistoryUpdated: (data: any) =>
      emit(GameEvents.MATCH_HISTORY_UPDATED, data),

    emitChampionDataLoaded: (data: any) =>
      emit(GameEvents.CHAMPION_DATA_LOADED, data),

    emitConnectionStateChanged: (state: any) =>
      emit(GameEvents.CONNECTION_STATE_CHANGED, state),

    emitPerformanceMetric: (metric: any) =>
      emit(GameEvents.PERFORMANCE_METRIC, metric),

    emitSlowOperation: (operation: any) =>
      emit(GameEvents.SLOW_OPERATION, operation)
  }
}
