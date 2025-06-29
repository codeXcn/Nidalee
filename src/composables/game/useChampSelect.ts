import { invoke } from '@tauri-apps/api/core'

export function useChampSelect() {
  /**
   * 获取当前选人会话信息
   * @returns Promise<any> 选人会话数据
   */
  async function getChampSelectSession() {
    try {
      console.log('[🔍 ChampSelect] 📡 开始获取选人会话信息...')
      const session = await invoke('get_champ_select_session')
      console.log('[🔍 ChampSelect] ✅ 获取选人会话成功')
      return session
    } catch (error) {
      console.error('[🔍 ChampSelect] ❌ 获取选人会话失败:', error)
      throw new Error(`获取选人会话失败: ${error}`)
    }
  }

  /**
   * 选择英雄 (hover 或 lock)
   * @param actionId 操作ID
   * @param championId 英雄ID
   * @param completed 是否完成(锁定)
   */
  async function pickChampion(actionId: number, championId: number, completed: boolean = false) {
    try {
      console.log('[⭐ ChampSelect] 🎯 选择英雄操作:', {
        actionId,
        championId,
        completed
      })

      const result = await invoke('pick_champion', { actionId, championId, completed })
      console.log('pick_champion:', result)
      console.log(`[⭐ ChampSelect] ✅ ${completed ? '英雄已锁定' : '英雄已Hover'}`)
    } catch (error) {
      console.error('[⭐ ChampSelect] ❌ 选择英雄失败:', error)
      throw new Error(`选择英雄失败: ${error}`)
    }
  }

  /**
   * 禁用英雄
   * @param actionId 操作ID
   * @param championId 英雄ID
   */
  async function banChampion(actionId: number, championId: number) {
    try {
      console.log('[🚫 ChampSelect] 🎯 禁用英雄操作:', { actionId, championId })
      const result = await invoke('ban_champion', { actionId, championId })
      console.log('ban_champion:', result)
      console.log('[🚫 ChampSelect] ✅ 英雄已被禁用')
    } catch (error) {
      console.error('[🚫 ChampSelect] ❌ 禁用英雄失败:', error)
      throw new Error(`禁用英雄失败: ${error}`)
    }
  }

  /**
   * 检查并执行当前玩家的自动操作
   * @param session 选人会话数据
   * @param autoFunctions 自动功能配置
   * @param executedActions 已执行的操作记录
   * @returns 返回是否安排了操作执行
   */
  async function checkAndExecuteAutoActions(session: any, autoFunctions: any, executedActions: any) {
    console.log('[🤖 AutoChampSelect] ===== 检查自动选人操作 =====')
    console.log('[🤖 AutoChampSelect] 会话阶段:', session?.timer?.phase)
    console.log('[🤖 AutoChampSelect] 已执行操作:', executedActions)
    // session?.timer?.phase FINALIZATION
    if (session?.timer?.phase === 'FINALIZATION') {
      // 重置已执行的操作记录，为下一局游戏做准备
      console.log('[🤖 AutoChampSelect] 🔄 选人阶段结束，重置已执行操作记录')
      executedActions.banChampion = false
      executedActions.selectChampion = false
      return false
    }
    if (!session?.actions || session.localPlayerCellId === undefined) {
      console.log('[🤖 AutoChampSelect] ⚠️ 缺少必要的会话数据')
      return false
    }

    const localPlayerCellId = session.localPlayerCellId
    let hasScheduledAction = false
    // 检查当前阶段是否是 BAN_PICK
    if (session?.timer?.phase === 'BAN_PICK') {
    // 遍历所有 actions 查找当前玩家的操作
      for (const actionGroup of session.actions) {
        for (const action of actionGroup) {
          // 检查是否是当前玩家的操作
          if (action.actorCellId === localPlayerCellId) {
            console.log('[🤖 AutoChampSelect] 🎯 找到当前玩家的操作:', {
              id: action.id,
              type: action.type,
              championId: action.championId,
              completed: action.completed,
              isInProgress: action.isInProgress
            })

            // 根据 action 类型执行自动操作
            if (action.type === 'ban' && !executedActions.banChampion && !action.completed) {
              // 自动禁用英雄
              if (autoFunctions.banChampion.enabled && autoFunctions.banChampion.championId) {
                console.log('[🤖 AutoChampSelect] 🚫 安排自动禁用英雄')

                setTimeout(async () => {
                  try {
                    await banChampion(action.id, autoFunctions.banChampion.championId)
                    executedActions.banChampion = true
                    console.log('[🤖 AutoChampSelect] ✅ 自动禁用英雄成功')
                  } catch (error) {
                    console.error('[🤖 AutoChampSelect] ❌ 自动禁用英雄失败:', error)
                  }
                }, autoFunctions.banChampion.delay || 500)

                hasScheduledAction = true
              }
            } else if (action.type === 'pick' && !executedActions.selectChampion && !action.completed) {
              // 自动选择英雄
              if (autoFunctions.selectChampion.enabled && autoFunctions.selectChampion.championId) {
                console.log('[🤖 AutoChampSelect] ⭐ 安排自动选择英雄')

                setTimeout(async () => {
                  try {
                    // 先 hover
                    await pickChampion(action.id, autoFunctions.selectChampion.championId, false)
                    console.log('[🤖 AutoChampSelect] 👁️ 自动Hover成功')

                    // 延迟后锁定
                    setTimeout(async () => {
                      try {
                        await pickChampion(action.id, autoFunctions.selectChampion.championId, true)
                        executedActions.selectChampion = true
                        console.log('[🤖 AutoChampSelect] ✅ 自动锁定英雄成功')
                      } catch (error) {
                        console.error('[🤖 AutoChampSelect] ❌ 自动锁定英雄失败:', error)
                      }
                    }, 1000) // 锁定延迟1秒
                  } catch (error) {
                    console.error('[🤖 AutoChampSelect] ❌ 自动Hover英雄失败:', error)
                  }
                }, autoFunctions.selectChampion.delay || 500)

                hasScheduledAction = true
              }
            }
          }
        }
      }
    }
  

    console.log('[🤖 AutoChampSelect] ===== 自动选人操作检查完成 =====\n')
    return hasScheduledAction
  }

  return {
    getChampSelectSession,
    pickChampion,
    banChampion,
    checkAndExecuteAutoActions
  }
}
